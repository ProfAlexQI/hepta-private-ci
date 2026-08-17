use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Component;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use super::ClosedAuthorityV1;
use super::DecisionReceiptBindingV1;
use super::FullMatrixVerdictV1;
use super::GateIdV1;
use super::GateVerificationStateV1;
use super::GateVerificationV1;
use super::PhaseAVerdictV1;
use super::PlatformProfileV1;
use super::PlatformReceiptBindingV1;
use super::ReceiptLayerBindingV1;
use super::ReceiptLayerIdV1;
use super::SuccessorContractV1;
use super::SuccessorVerificationV1;
use super::VerifiedDecisionReceiptV1;
use super::VerifiedReceiptIdentityV1;
use super::profiles;
use crate::AcceptanceError;
use crate::durable::MAX_SMALL_FILE_BYTES;
use crate::durable::canonical_json;
use crate::durable::secure_read;
use crate::durable::sha256;
use crate::manifest_inventory::VerifiedManifest;
use crate::manifest_inventory::digest_shape;
use crate::manifest_inventory::parse_manifest;
use crate::manifest_inventory::validate_relative_path;

/// Read-only Phase A verification for the exact MNL successor boundary.
///
/// This function revalidates present prerequisite receipts plus each re-emitted
/// platform wrapper, its projected provenance, and its pinned content. It does
/// not re-open the canonical original live roots and is not platform PASS. It
/// cannot emit an aggregate, challenge, signature, acceptance, ref transition,
/// or cutover action. Linux has no frozen successor identity, so every
/// successful Phase A verification is still BLOCKED.
pub fn verify_current_receipts(
    contract: &SuccessorContractV1,
    receipts_parent: &Path,
) -> Result<SuccessorVerificationV1, AcceptanceError> {
    profiles::validate_contract(contract)?;
    if receipts_parent != Path::new(profiles::RECEIPTS_PARENT)
        || contract.receipts_parent != profiles::RECEIPTS_PARENT
    {
        return Err(invalid(
            "successor verification requires the exact canonical receipts parent",
        ));
    }

    let strategy_receipt =
        verify_decision_receipt(&contract.strategy_receipt, receipts_parent, |bytes| {
            validate_strategy_decision(bytes, contract)
        })?;
    let development_freeze_receipt = verify_decision_receipt(
        &contract.development_freeze_receipt,
        receipts_parent,
        |bytes| validate_development_freeze_decision(bytes, contract),
    )?;

    let mac = verify_platform_identity(&contract.present_platform_receipts[0], receipts_parent)?;
    let nix = verify_platform_identity(&contract.present_platform_receipts[1], receipts_parent)?;
    let gates = vec![
        mac,
        GateVerificationV1 {
            gate: GateIdV1::LinuxX8664,
            profile: PlatformProfileV1::LinuxSuccessorIdentityPendingV1,
            receipt: None,
            state: GateVerificationStateV1::ProfileIdentityUnpinned,
        },
        nix,
        deferred_gate(
            GateIdV1::WindowsX8664Native,
            PlatformProfileV1::WindowsNativeDeferredMnlSuccessorV1,
        ),
        deferred_gate(
            GateIdV1::GithubActions,
            PlatformProfileV1::GithubHostedDeferredMnlSuccessorV1,
        ),
    ];

    Ok(SuccessorVerificationV1 {
        authority: ClosedAuthorityV1::exact(),
        blockers: vec!["gate:linux-x86_64:PROFILE_IDENTITY_UNPINNED".to_string()],
        contract_sha256: sha256(&canonical_json(contract)?),
        development_freeze_receipt,
        full_matrix_verdict: FullMatrixVerdictV1::NotClaimed,
        gates,
        phase_a_verdict: PhaseAVerdictV1::Blocked,
        present_wrapper_content_reverified: true,
        product_candidate: contract.product_candidate.clone(),
        ready_for_successor_builder: false,
        required_gate_count: profiles::REQUIRED_GATE_COUNT,
        // Content identity verification is deliberately not platform PASS.
        required_pass_count: 0,
        schema: profiles::VERIFICATION_SCHEMA.to_string(),
        strategy_receipt,
        tooling_integration_base: contract.tooling_integration_base.clone(),
    })
}

fn verify_decision_receipt<F>(
    binding: &DecisionReceiptBindingV1,
    receipts_parent: &Path,
    validate: F,
) -> Result<VerifiedDecisionReceiptV1, AcceptanceError>
where
    F: FnOnce(&[u8]) -> Result<(), AcceptanceError>,
{
    validate_root_name(&binding.receipt_root_name)?;
    let root = receipts_parent.join(&binding.receipt_root_name);
    let manifest = VerifiedManifest::load_named(
        &root,
        &binding.manifest_relative_path,
        &binding.manifest_sha256,
        binding.manifest_entry_count,
    )?;
    let manifest_bytes = secure_read(
        &root.join(&binding.manifest_relative_path),
        MAX_SMALL_FILE_BYTES,
    )?;
    if manifest_bytes.len() as u64 != binding.manifest_size_bytes
        || sha256(&manifest_bytes) != binding.manifest_sha256
    {
        return Err(invalid(
            "decision receipt manifest differs from its exact byte identity",
        ));
    }
    manifest.require_hash(&binding.decision.relative_path, &binding.decision.sha256)?;
    let entry = manifest
        .entry(&binding.decision.relative_path)
        .ok_or_else(|| invalid("decision artifact is absent from its manifest"))?;
    if entry.size_bytes != binding.decision.size_bytes {
        return Err(invalid(
            "decision artifact size differs from its exact byte identity",
        ));
    }
    let decision = manifest.bytes(&binding.decision.relative_path)?;
    validate(&decision)?;
    manifest.reverify()?;
    Ok(VerifiedDecisionReceiptV1 {
        decision_sha256: sha256(&decision),
        manifest_sha256: binding.manifest_sha256.clone(),
        receipt_root: path_text(&root, "decision receipt root")?,
    })
}

#[cfg(test)]
pub(super) fn verify_strategy_receipt_for_test(
    binding: &DecisionReceiptBindingV1,
    receipts_parent: &Path,
    contract: &SuccessorContractV1,
) -> Result<VerifiedDecisionReceiptV1, AcceptanceError> {
    verify_decision_receipt(binding, receipts_parent, |bytes| {
        validate_strategy_decision(bytes, contract)
    })
}

fn verify_platform_identity(
    binding: &PlatformReceiptBindingV1,
    receipts_parent: &Path,
) -> Result<GateVerificationV1, AcceptanceError> {
    validate_root_name(&binding.receipt_root_name)?;
    let root = receipts_parent.join(&binding.receipt_root_name);
    let outer = load_layer(&root, &binding.outer)?;
    let mut visible_modes = BTreeSet::from([binding.outer.mode_manifest_relative_path.clone()]);
    if let Some(inner) = &binding.inner {
        let visible = Path::new(&inner.root_relative_path).join(&inner.mode_manifest_relative_path);
        visible_modes.insert(path_text(&visible, "nested mode manifest")?);
    }
    verify_mode_manifest(&binding.outer, &outer, &visible_modes)?;
    reject_terminal_markers(&outer)?;

    let inner = binding
        .inner
        .as_ref()
        .map(|inner_binding| {
            let inner = load_layer(&root, inner_binding)?;
            verify_mode_manifest(
                inner_binding,
                &inner,
                &BTreeSet::from([inner_binding.mode_manifest_relative_path.clone()]),
            )?;
            let nested_manifest = Path::new(&inner_binding.root_relative_path)
                .join(&inner_binding.manifest_relative_path);
            let nested_mode = Path::new(&inner_binding.root_relative_path)
                .join(&inner_binding.mode_manifest_relative_path);
            outer.require_hash(
                &path_text(&nested_manifest, "nested receipt manifest")?,
                &inner_binding.manifest_sha256,
            )?;
            outer.require_hash(
                &path_text(&nested_mode, "nested mode manifest")?,
                &inner_binding.mode_manifest_sha256,
            )?;
            reject_terminal_markers(&inner)?;
            Ok::<VerifiedManifest, AcceptanceError>(inner)
        })
        .transpose()?;

    let terminal_manifest = match binding.terminal_layer {
        ReceiptLayerIdV1::Outer => &outer,
        ReceiptLayerIdV1::Inner => inner
            .as_ref()
            .ok_or_else(|| invalid("terminal artifact requires an absent inner receipt layer"))?,
    };
    terminal_manifest.require_hash(&binding.terminal.relative_path, &binding.terminal.sha256)?;
    let terminal_entry = terminal_manifest
        .entry(&binding.terminal.relative_path)
        .ok_or_else(|| invalid("platform terminal artifact is absent"))?;
    if terminal_entry.size_bytes != binding.terminal.size_bytes {
        return Err(invalid(
            "platform terminal artifact size differs from its exact identity",
        ));
    }
    let terminal = terminal_manifest.bytes(&binding.terminal.relative_path)?;
    validate_terminal_identity(binding, &terminal)?;
    verify_reemission_provenance(binding, &outer, terminal_manifest, &terminal)?;
    outer.reverify()?;
    if let Some(inner) = &inner {
        inner.reverify()?;
    }

    Ok(GateVerificationV1 {
        gate: binding.gate,
        profile: binding.profile,
        receipt: Some(VerifiedReceiptIdentityV1 {
            inner_manifest_sha256: binding
                .inner
                .as_ref()
                .map(|layer| layer.manifest_sha256.clone()),
            outer_manifest_sha256: binding.outer.manifest_sha256.clone(),
            receipt_root: path_text(&root, "platform receipt root")?,
        }),
        state: GateVerificationStateV1::ContentIdentityVerified,
    })
}

fn load_layer(
    receipt_root: &Path,
    binding: &ReceiptLayerBindingV1,
) -> Result<VerifiedManifest, AcceptanceError> {
    let root = if binding.root_relative_path == "." {
        receipt_root.to_path_buf()
    } else {
        validate_relative_path(&binding.root_relative_path)?;
        receipt_root.join(&binding.root_relative_path)
    };
    VerifiedManifest::load_named(
        &root,
        &binding.manifest_relative_path,
        &binding.manifest_sha256,
        binding.manifest_entry_count,
    )
}

fn deferred_gate(gate: GateIdV1, profile: PlatformProfileV1) -> GateVerificationV1 {
    GateVerificationV1 {
        gate,
        profile,
        receipt: None,
        state: GateVerificationStateV1::DeferredDebt,
    }
}

fn validate_strategy_decision(
    bytes: &[u8],
    contract: &SuccessorContractV1,
) -> Result<(), AcceptanceError> {
    let fields = parse_sectioned_key_values(bytes, "strategy decision")?;
    for (key, expected) in [
        ("schema", "hepta_vnext_upstream_ui_strategy_v1"),
        (
            "backend_candidate.commit",
            contract.product_candidate.backend.head.as_str(),
        ),
        (
            "backend_candidate.tree",
            contract.product_candidate.backend.tree.as_str(),
        ),
        (
            "upstream.frozen_cutoff",
            "74004b5397b24662a87a5264a6ae80664168c7f3",
        ),
        (
            "upstream.live_main_observed",
            "86b1123ff6b5d089a146be4e603a324cf454223a",
        ),
        ("upstream.frozen_cutoff_to_live_main_ahead", "92"),
        ("upstream.decision", "freeze_74004_for_52ec_qualification"),
        (
            "upstream.deferred_action",
            "intake_live_upstream_delta_in_first_post_cutover_vnext_development_cycle",
        ),
        ("upstream.forbidden_claim", "upstream_plus_32"),
        (
            "ui_candidate.commit",
            contract.product_candidate.ui.repository.head.as_str(),
        ),
        ("ui_candidate.qualification_route_links", "22"),
        ("ui_candidate.route_directory_coverage", "26_of_26"),
        ("ui_candidate.backend_ui_merge_base", "none"),
        ("ui_upstream.decision", "bounded_patch_ledger_only"),
        ("ui_upstream.whole_tree_overwrite_allowed", "false"),
        ("integration_policy.decision", "dual_exact_head_binding"),
        (
            "integration_policy.backend_head",
            contract.product_candidate.backend.head.as_str(),
        ),
        (
            "integration_policy.ui_head",
            contract.product_candidate.ui.repository.head.as_str(),
        ),
        (
            "integration_policy.unrelated_history_merge_allowed",
            "false",
        ),
        ("integration_policy.aggregate_must_bind_both_heads", "true"),
        ("integration_policy.windows_and_github_ci_deferred", "true"),
        ("integration_policy.promotion_authority", "false"),
        ("integration_policy.qualification_pass_authority", "false"),
        ("integration_policy.production_authority", "false"),
    ] {
        require_field(&fields, key, expected, "strategy decision")?;
    }
    Ok(())
}

fn validate_development_freeze_decision(
    bytes: &[u8],
    contract: &SuccessorContractV1,
) -> Result<(), AcceptanceError> {
    let fields = parse_sectioned_key_values(bytes, "development freeze decision")?;
    for (key, expected) in [
        ("schema", "hepta_vnext_development_tree_freeze_decision_v1"),
        ("decision_status", "CONFIRMED"),
        (
            "legacy_source_policy",
            "PERMANENTLY_FROZEN_FOR_NEW_DEVELOPMENT",
        ),
        ("legacy_source_allowed_roles", "ROLLBACK_AND_FORENSICS_ONLY"),
        (
            "backend_new_development_root",
            "/Volumes/T5/hepta-vnext/worktrees/main-integration",
        ),
        (
            "backend_head",
            contract.product_candidate.backend.head.as_str(),
        ),
        (
            "backend_tree",
            contract.product_candidate.backend.tree.as_str(),
        ),
        (
            "ui_new_development_root",
            "/Volumes/T5/hepta-vnext/worktrees/ui-main",
        ),
        (
            "ui_head",
            contract.product_candidate.ui.repository.head.as_str(),
        ),
        (
            "ui_tree",
            contract.product_candidate.ui.repository.tree.as_str(),
        ),
        ("default_refs_changed_by_this_decision", "false"),
        ("production_changed_by_this_decision", "false"),
        ("data_deleted_by_this_decision", "false"),
        ("physical_retirement_authorized", "false"),
        ("promotion_authority", "false"),
        ("automatic_transition", "false"),
        (
            "policy",
            "all_new_backend_development_enters_main_integration_and_all_new_ui_development_enters_ui_main",
        ),
    ] {
        require_field(&fields, key, expected, "development freeze decision")?;
    }
    Ok(())
}

fn validate_terminal_identity(
    binding: &PlatformReceiptBindingV1,
    bytes: &[u8],
) -> Result<(), AcceptanceError> {
    let fields = parse_sectioned_key_values(bytes, "platform terminal artifact")?;
    match binding.profile {
        PlatformProfileV1::MacFrozenRev7MnlSuccessorV1 => {
            for (key, expected) in [
                ("schema", "hepta_vnext_main_mac_validation_v6"),
                ("status", "pass"),
                ("candidate_commit", profiles::BACKEND_CANDIDATE_HEAD),
                ("candidate_tree", profiles::BACKEND_CANDIDATE_TREE),
                ("worktree_clean", "true"),
                ("exact_phases_all_pass", "true"),
                ("top_level_evidence_mode_sealed", "true"),
                ("operator_acceptance", "false"),
                ("candidate_operator_acceptance", "false"),
                ("cross_platform_qualification", "false"),
                ("promotion", "false"),
                ("enforce", "false"),
                ("outbound", "false"),
                ("retirement", "false"),
                ("automatic_transition", "false"),
                ("default_branch_changed", "false"),
                ("production_cutover", "false"),
            ] {
                require_field(&fields, key, expected, "Mac terminal artifact")?;
            }
        }
        PlatformProfileV1::NixFrozenRev7MnlSuccessorV1 => {
            for (key, expected) in [
                ("schema", "hepta_vnext_nix_exact_v3_result_v1"),
                ("status", "PASS"),
                ("verdict", "PASS"),
                ("qualification", "true"),
                ("candidate_pass", "true"),
                ("candidate_fail", "false"),
                ("harness_fail", "false"),
                ("interrupted", "false"),
                ("candidate_head", profiles::BACKEND_CANDIDATE_HEAD),
                ("candidate_tree", profiles::BACKEND_CANDIDATE_TREE),
                ("production_changed", "false"),
                ("refs_changed", "false"),
                ("data_deleted", "false"),
                ("promotion_authority", "false"),
            ] {
                require_field(&fields, key, expected, "Nix terminal artifact")?;
            }
        }
        _ => {
            return Err(invalid(
                "only exact frozen Mac and Nix identities are present in Phase A",
            ));
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ReemissionAttestationV1 {
    authority_projection_byte_identical: bool,
    dealiased: bool,
    hardlink_topology_sha256: String,
    original_extended_metadata_inventory_sha256: String,
    original_inventory_sha256: String,
    original_manifest_entry_count: usize,
    original_manifest_relative_path: String,
    original_manifest_sha256: String,
    original_metadata_inventory_sha256: String,
    original_receipt_root: String,
    original_reverified_after: bool,
    original_reverified_before: bool,
    post_original_inventory_sha256: String,
    pre_original_inventory_sha256: String,
    projection_map_sha256: String,
    reemitter_sha256: String,
    schema: String,
}

struct ExpectedProvenance {
    extended_metadata_sha256: &'static str,
    hardlink_topology_sha256: &'static str,
    manifest_entry_count: usize,
    manifest_relative_path: &'static str,
    manifest_sha256: &'static str,
    metadata_sha256: &'static str,
    original_inventory_sha256: &'static str,
    original_receipt_root: &'static str,
}

fn verify_reemission_provenance(
    binding: &PlatformReceiptBindingV1,
    outer: &VerifiedManifest,
    terminal_manifest: &VerifiedManifest,
    terminal: &[u8],
) -> Result<(), AcceptanceError> {
    let expected = expected_provenance(binding.profile)?;
    let attestation: ReemissionAttestationV1 =
        outer.json_canonical("provenance/reemission-attestation.json")?;
    if attestation.schema != "hepta_vnext_provenance_reemission_v1"
        || !attestation.authority_projection_byte_identical
        || !attestation.dealiased
        || !attestation.original_reverified_before
        || !attestation.original_reverified_after
        || attestation.hardlink_topology_sha256 != expected.hardlink_topology_sha256
        || attestation.original_extended_metadata_inventory_sha256
            != expected.extended_metadata_sha256
        || attestation.original_inventory_sha256 != expected.original_inventory_sha256
        || attestation.original_manifest_entry_count != expected.manifest_entry_count
        || attestation.original_manifest_relative_path != expected.manifest_relative_path
        || attestation.original_manifest_sha256 != expected.manifest_sha256
        || attestation.original_metadata_inventory_sha256 != expected.metadata_sha256
        || attestation.original_receipt_root != expected.original_receipt_root
        || attestation.pre_original_inventory_sha256 != expected.original_inventory_sha256
        || attestation.post_original_inventory_sha256 != expected.original_inventory_sha256
    {
        return Err(invalid(
            "reemission provenance differs from the independently frozen original identity",
        ));
    }
    for digest in [
        &attestation.projection_map_sha256,
        &attestation.reemitter_sha256,
    ] {
        if !digest_shape(digest) {
            return Err(invalid("reemission provenance contains a malformed digest"));
        }
    }
    for (relative, expected_sha256) in [
        (
            "provenance/hardlink-topology.tsv",
            attestation.hardlink_topology_sha256.as_str(),
        ),
        (
            "provenance/original-extended-metadata.tsv",
            attestation
                .original_extended_metadata_inventory_sha256
                .as_str(),
        ),
        (
            "provenance/original-metadata.tsv",
            attestation.original_metadata_inventory_sha256.as_str(),
        ),
        (
            "provenance/projection-map.tsv",
            attestation.projection_map_sha256.as_str(),
        ),
        (
            "provenance/reemitter",
            attestation.reemitter_sha256.as_str(),
        ),
    ] {
        outer.require_hash(relative, expected_sha256)?;
    }

    let projected_manifest_path =
        Path::new("provenance/original-tree").join(expected.manifest_relative_path);
    let projected_manifest_path = path_text(&projected_manifest_path, "projected manifest")?;
    outer.require_hash(&projected_manifest_path, expected.manifest_sha256)?;
    let projected_manifest_bytes = outer.bytes(&projected_manifest_path)?;
    let original_entries = parse_manifest(&projected_manifest_bytes)?;
    if original_entries.len() != expected.manifest_entry_count {
        return Err(invalid(
            "projected original manifest entry count differs from its frozen identity",
        ));
    }

    let mut inventory_rows = BTreeMap::new();
    for (relative, expected_sha256) in &original_entries {
        let projected = Path::new("provenance/original-tree").join(relative);
        let projected = path_text(&projected, "projected original artifact")?;
        outer.require_hash(&projected, expected_sha256)?;
        let entry = outer
            .entry(&projected)
            .ok_or_else(|| invalid("projected original artifact is absent"))?;
        inventory_rows.insert(
            relative.clone(),
            format!("{}\t{}\t./{}\n", entry.sha256, entry.size_bytes, relative),
        );
    }
    let manifest_entry = outer
        .entry(&projected_manifest_path)
        .ok_or_else(|| invalid("projected original manifest is absent"))?;
    inventory_rows.insert(
        expected.manifest_relative_path.to_string(),
        format!(
            "{}\t{}\t./{}\n",
            manifest_entry.sha256, manifest_entry.size_bytes, expected.manifest_relative_path
        ),
    );
    let inventory_bytes = inventory_rows.values().cloned().collect::<String>();
    if sha256(inventory_bytes.as_bytes()) != expected.original_inventory_sha256 {
        return Err(invalid(
            "projected original inventory differs from its frozen provenance digest",
        ));
    }

    let projected_terminal = match binding.profile {
        PlatformProfileV1::MacFrozenRev7MnlSuccessorV1 => {
            "provenance/original-tree/qualification-status.txt"
        }
        PlatformProfileV1::NixFrozenRev7MnlSuccessorV1 => {
            "provenance/original-tree/receipt/result.txt"
        }
        _ => return Err(invalid("unexpected Phase A provenance profile")),
    };
    if outer.bytes(projected_terminal)? != terminal
        || terminal_manifest
            .entry(&binding.terminal.relative_path)
            .is_none()
    {
        return Err(invalid(
            "terminal artifact is not byte-identical to its projected frozen original",
        ));
    }
    Ok(())
}

fn expected_provenance(profile: PlatformProfileV1) -> Result<ExpectedProvenance, AcceptanceError> {
    match profile {
        PlatformProfileV1::MacFrozenRev7MnlSuccessorV1 => Ok(ExpectedProvenance {
            extended_metadata_sha256: "7f2a66f797b273e3332e0d9f26e8290672f025433b587df9cd0a560875b89f76",
            hardlink_topology_sha256: "9efaf5f7b046c35ce88af776e723204407514629f96cb8a58bc26e255525c886",
            manifest_entry_count: 114,
            manifest_relative_path: "SHA256SUMS",
            manifest_sha256: "824b5158028fd2d171c7f9b427bc33455705e4d994432926a11d199c02313ca0",
            metadata_sha256: "dbb5dacee129e66dba8f8be7f51979db79ab95d9c02e7323c7750c388aeef4d0",
            original_inventory_sha256: "46d1da21cb5366ca34bf7f3ccc3def63dfcdf4590252d8ba57ff41b2457c28fe",
            original_receipt_root: "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-mac-exact-attempt-2-20260813T052744Z",
        }),
        PlatformProfileV1::NixFrozenRev7MnlSuccessorV1 => Ok(ExpectedProvenance {
            extended_metadata_sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            hardlink_topology_sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            manifest_entry_count: 116,
            manifest_relative_path: "OUTER-SHA256SUMS",
            manifest_sha256: "55a041bbdaf4bf31d676f20c80fe07737dc5d33f0d7d3dfaed26639e57db93a2",
            metadata_sha256: "1f5b61ffb693f6eca838abe782668aa291d0d8ab91d21ae36e22514a5a6c7cbd",
            original_inventory_sha256: "5e72993a81c3e6e543942f5a1104c715692e5d99a1fd7974d95476fe14b5bcd5",
            original_receipt_root: "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-nix-exact-v3-attempt-3-20260813T065739Z/attempt-52ec08130755",
        }),
        _ => Err(invalid("Phase A provenance profile is not frozen")),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ModeKind {
    Directory,
    File,
}

struct ModeRow {
    kind: ModeKind,
    mode: u32,
    size: Option<u64>,
}

fn verify_mode_manifest(
    binding: &ReceiptLayerBindingV1,
    manifest: &VerifiedManifest,
    allowed_mode_paths: &BTreeSet<String>,
) -> Result<(), AcceptanceError> {
    if !allowed_mode_paths.contains(&binding.mode_manifest_relative_path)
        || allowed_mode_paths
            .iter()
            .any(|relative| manifest.entry(relative).is_none())
    {
        return Err(invalid(
            "compiled mode inventory is absent from its visible sealed layer",
        ));
    }
    manifest.require_hash(
        &binding.mode_manifest_relative_path,
        &binding.mode_manifest_sha256,
    )?;
    let rows = parse_mode_rows(&manifest.bytes(&binding.mode_manifest_relative_path)?)?;
    let expected_files = manifest
        .entry_paths()
        .map(str::to_string)
        .chain(std::iter::once(
            manifest.manifest_relative_path().to_string(),
        ))
        .collect::<BTreeSet<_>>();
    let actual_files = rows
        .iter()
        .filter(|(_, row)| row.kind == ModeKind::File)
        .map(|(path, _)| path.clone())
        .collect::<BTreeSet<_>>();
    if actual_files != expected_files {
        return Err(invalid(
            "mode manifest file rows do not exactly cover the sealed layer",
        ));
    }
    let expected_directories = manifest
        .directory_paths()
        .map(|path| if path.is_empty() { "." } else { path }.to_string())
        .collect::<BTreeSet<_>>();
    let actual_directories = rows
        .iter()
        .filter(|(_, row)| row.kind == ModeKind::Directory)
        .map(|(path, _)| path.clone())
        .collect::<BTreeSet<_>>();
    if actual_directories != expected_directories {
        return Err(invalid(
            "mode manifest directory rows do not exactly cover the sealed layer",
        ));
    }
    for (relative, row) in rows {
        let target = if relative == "." {
            manifest.root.clone()
        } else {
            manifest.root.join(&relative)
        };
        let metadata = std::fs::symlink_metadata(&target)?;
        if metadata.file_type().is_symlink()
            || (row.kind == ModeKind::File && !metadata.is_file())
            || (row.kind == ModeKind::Directory && !metadata.is_dir())
            || row.size.is_some_and(|expected| metadata.len() != expected)
        {
            return Err(invalid(
                "mode manifest type or size differs from the receipt",
            ));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            let actual = metadata.mode() & 0o7777;
            if actual & 0o7000 != 0 || actual != row.mode {
                return Err(invalid(
                    "mode manifest mode or special bits differ from the receipt",
                ));
            }
        }
    }
    Ok(())
}

fn parse_mode_rows(bytes: &[u8]) -> Result<BTreeMap<String, ModeRow>, AcceptanceError> {
    let text = std::str::from_utf8(bytes).map_err(|_| invalid("mode manifest is not UTF-8"))?;
    if text.is_empty() || !text.ends_with('\n') || text.contains('\r') {
        return Err(invalid(
            "mode manifest must be newline-terminated UTF-8 without carriage returns",
        ));
    }
    let mut rows = BTreeMap::new();
    let mut previous: Option<String> = None;
    for line in text.lines() {
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 4 {
            return Err(invalid("typed mode/size/path row is malformed"));
        }
        let (kind, size) = match (fields[0], fields[2]) {
            ("Regular File", value) => (
                ModeKind::File,
                Some(
                    value
                        .parse::<u64>()
                        .map_err(|_| invalid("mode manifest size is malformed"))?,
                ),
            ),
            ("Directory", "-") => (ModeKind::Directory, None),
            _ => return Err(invalid("mode manifest type or size is malformed")),
        };
        if size.is_some_and(|value| value.to_string() != fields[2]) {
            return Err(invalid("mode manifest size is not canonical decimal"));
        }
        let mode = u32::from_str_radix(fields[1], 8)
            .ok()
            .filter(|value| *value <= 0o7777 && value & 0o7000 == 0)
            .ok_or_else(|| invalid("mode manifest mode is malformed"))?;
        if format!("{mode:o}") != fields[1] {
            return Err(invalid("mode manifest mode is not canonical octal"));
        }
        let relative = if fields[3] == "." {
            "."
        } else {
            fields[3]
                .strip_prefix("./")
                .ok_or_else(|| invalid("mode manifest path lacks the exact ./ prefix"))?
        };
        if relative != "." {
            validate_relative_path(relative)?;
        }
        if previous.as_deref().is_some_and(|value| value >= relative) {
            return Err(invalid(
                "mode manifest paths must be unique and strictly sorted",
            ));
        }
        previous = Some(relative.to_string());
        if rows
            .insert(relative.to_string(), ModeRow { kind, mode, size })
            .is_some()
        {
            return Err(invalid("mode manifest contains a duplicate path"));
        }
    }
    Ok(rows)
}

fn reject_terminal_markers(manifest: &VerifiedManifest) -> Result<(), AcceptanceError> {
    for path in manifest.entry_paths().chain(manifest.directory_paths()) {
        let name = Path::new(path)
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        if ["failed", "failure", "blocked", "interrupted", "superseded"]
            .iter()
            .any(|prefix| name == *prefix || name.starts_with(&format!("{prefix}.")))
        {
            return Err(invalid(
                "frozen receipt identity contains a conflicting terminal marker",
            ));
        }
    }
    Ok(())
}

fn parse_sectioned_key_values(
    bytes: &[u8],
    label: &str,
) -> Result<BTreeMap<String, String>, AcceptanceError> {
    let text = std::str::from_utf8(bytes).map_err(|_| invalid(format!("{label} is not UTF-8")))?;
    if text.is_empty() || !text.ends_with('\n') || text.contains('\r') {
        return Err(invalid(format!(
            "{label} must be newline-terminated UTF-8 without carriage returns"
        )));
    }
    let mut section: Option<&str> = None;
    let mut fields = BTreeMap::new();
    for line in text.lines() {
        if line.is_empty() {
            continue;
        }
        if let Some(name) = line
            .strip_prefix('[')
            .and_then(|line| line.strip_suffix(']'))
        {
            validate_identifier(name, label)?;
            section = Some(name);
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| invalid(format!("{label} contains a malformed line")))?;
        validate_identifier(key, label)?;
        if value.is_empty() {
            return Err(invalid(format!("{label} contains an empty value")));
        }
        let qualified = match section {
            Some(section) => format!("{section}.{key}"),
            None => key.to_string(),
        };
        if fields.insert(qualified, value.to_string()).is_some() {
            return Err(invalid(format!("{label} contains a duplicate field")));
        }
    }
    Ok(fields)
}

fn require_field(
    fields: &BTreeMap<String, String>,
    key: &str,
    expected: &str,
    label: &str,
) -> Result<(), AcceptanceError> {
    if fields.get(key).map(String::as_str) != Some(expected) {
        return Err(invalid(format!(
            "{label} field differs from the exact successor boundary: {key}"
        )));
    }
    Ok(())
}

fn validate_identifier(value: &str, label: &str) -> Result<(), AcceptanceError> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
    {
        return Err(invalid(format!("{label} identifier is malformed")));
    }
    Ok(())
}

fn validate_root_name(value: &str) -> Result<(), AcceptanceError> {
    let path = Path::new(value);
    if value.is_empty()
        || value.starts_with('.')
        || path.is_absolute()
        || path.parent() != Some(Path::new(""))
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(invalid("receipt root must be one safe child name"));
    }
    Ok(())
}

fn path_text(path: &Path, label: &str) -> Result<String, AcceptanceError> {
    path.to_str()
        .map(str::to_string)
        .ok_or_else(|| invalid(format!("{label} is not UTF-8")))
}

fn invalid(message: impl Into<String>) -> AcceptanceError {
    AcceptanceError::Invalid(message.into())
}
