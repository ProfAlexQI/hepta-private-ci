use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde_json::json;
use tempfile::TempDir;

use crate::durable::canonical_json;
use crate::durable::sha256;
use crate::manifest_inventory::LegacyExtendedMetadataPolicy;
use crate::manifest_inventory::VerifiedManifest;
use crate::manifest_inventory::load_legacy_manifest_with_policy;
use crate::model::AuthorityBoundary;

use super::builder::build_for_test;
use super::builder::modes_bytes as aggregate_modes_bytes;
use super::builder::plan_for_test;
use super::builder::publish_exclusive;
use super::builder::verify_for_test;
use super::evidence::BUILD_SPEC_SCHEMA;
use super::evidence::StepPolicy;
use super::evidence::ValidationPolicy;
use super::evidence::admit_formal_platform_pass_for_test;
use super::evidence::compiled_visible_mode_paths;
use super::evidence::exact_candidate;
use super::evidence::exact_platform_policy;
use super::evidence::parse_key_values;
use super::evidence::parse_mode_rows;
use super::evidence::parse_step_tsv;
use super::evidence::reject_conflicting_markers_for_test;
use super::evidence::reject_reserved_kv_fields;
use super::evidence::require_windows_step_results;
use super::evidence::valid_utc_timestamp_for_test;
use super::evidence::validate_github_prepared_profile_for_test;
use super::evidence::validate_kv_execution_fields;
use super::evidence::validate_linux_v5_trust_policy_for_test;
use super::evidence::validate_output_relative_name;
use super::evidence::validate_receipt_for_test;
use super::evidence::validate_reemitted_wrapper_for_test;
use super::evidence::validate_spec;
use super::evidence::validate_windows_nonce_paths_for_test;
use super::evidence::validate_windows_resource_floors_for_test;
use super::evidence::verify_frozen_original_provenance_for_test;
use super::evidence::verify_linux_watchdog_observations_for_test;
use super::evidence::verify_mode_manifest;
use super::evidence::verify_nix_exact_bindings_for_test;
use super::evidence::wrapper_attestation_bytes_for_test;
use super::model::AggregateBuildSpecV3;
use super::model::AggregateQualificationPacketV3;
use super::model::ArtifactBindingV3;
use super::model::EvidenceProfileV3;
use super::model::ManifestLayerBindingV3;
use super::model::ManifestLayerIdV3;
use super::model::ManifestRootKindV3;
use super::model::ModeManifestBindingV3;
use super::model::ModeManifestFormatV3;
use super::model::ObservedGateV3;
use super::model::OriginalReceiptBindingV3;
use super::model::PlatformGateInputV3;
use super::model::PrerequisiteInputV3;
use super::model::ReceiptEvidenceBindingV3;
use super::model::ReceiptProvenanceV3;
use super::profiles;
use super::run_cli_v3;

#[test]
fn compiled_gate_and_prerequisite_profiles_are_exact() {
    assert_eq!(
        profiles::gate_profile("macos-aarch64"),
        Some(EvidenceProfileV3::MacExactV6)
    );
    assert_eq!(
        profiles::gate_profile("github-actions"),
        Some(EvidenceProfileV3::GithubHostedExactV2)
    );
    assert_eq!(profiles::gate_profile("shadow-mac"), None);
    assert_eq!(
        profiles::prerequisite_profile("portable-inputs"),
        Some(EvidenceProfileV3::PortableInputsV1)
    );
    assert_eq!(profiles::prerequisite_profile("shadow"), None);
    assert!(
        profiles::frozen_windows_driver_identity().is_none(),
        "Windows r5 must stay programmatically unpinned until a host-locked successor is frozen"
    );
    assert!(
        profiles::frozen_linux_driver_identity().is_none(),
        "Linux v5 must stay programmatically unpinned until its WIP contract becomes compatible"
    );
    for profile in [
        EvidenceProfileV3::GithubHostedExactV2,
        EvidenceProfileV3::LinuxExactV5,
        EvidenceProfileV3::WindowsNativeV6,
    ] {
        assert!(
            profiles::frozen_receipt_identity(profile).is_none(),
            "unfinished platform receipts must not acquire placeholder identities"
        );
    }
}

#[test]
fn linux_v5_trust_policy_compiles_only_the_narrow_existing_public_signer_scope() {
    let policy = canonical_json(&json!({
        "acceptance_profile_revision": 7,
        "allowed_signers_sha256": profiles::LINUX_OPERATOR_ALLOWED_SIGNERS_SHA256,
        "authorization_scope": profiles::LINUX_OPERATOR_AUTHORIZATION_SCOPE,
        "authorized_action": profiles::LINUX_OPERATOR_ACTION,
        "candidate_head": super::evidence::CANDIDATE_HEAD,
        "candidate_nix_process_pause_authority": false,
        "candidate_tree": super::evidence::CANDIDATE_TREE,
        "challenge_maximum_lifetime_seconds": 900,
        "challenge_schema": "hepta_vnext_linux_operator_challenge_v2",
        "delete_authority": false,
        "driver_revision": 5,
        "execution_authorization_schema": "hepta_vnext_linux_execution_authorization_v1",
        "fresh_authorization_nonce_required": true,
        "fresh_challenge_required": true,
        "independent_workload_pause_restore_authority": true,
        "key_fingerprint": profiles::LINUX_OPERATOR_KEY_FINGERPRINT,
        "nix_container_volume_source_mutation_authority": false,
        "parent_trust_policy_sha256": profiles::LINUX_PARENT_TRUST_POLICY_SHA256,
        "principal": profiles::LINUX_OPERATOR_PRINCIPAL,
        "production_authority": false,
        "promotion_authority": false,
        "qualification_host": "desktop-ts",
        "runner_pause_restore_authority": true,
        "schema": profiles::LINUX_TRUST_POLICY_SCHEMA,
        "schema_version": 1,
        "signature_algorithm": "sshsig-ed25519",
        "signature_namespace": profiles::LINUX_OPERATOR_SIGNATURE_NAMESPACE,
        "single_use": true,
        "trust_policy_scope": "candidate_52ec_linux_v5_runner_and_independent_workload_lifecycle_only",
        "trust_root_id": profiles::LINUX_TRUST_ROOT_ID,
        "trust_root_revision": 2,
        "unregister_authority": false
    }))
    .expect("canonical Linux trust policy");
    validate_linux_v5_trust_policy_for_test(
        &policy,
        profiles::LINUX_OPERATOR_ALLOWED_SIGNERS.as_bytes(),
    )
    .expect("narrow Linux v5 trust policy");

    let mut broadened: serde_json::Value = serde_json::from_slice(&policy).expect("policy JSON");
    broadened["production_authority"] = json!(true);
    let broadened = canonical_json(&broadened).expect("canonical broadened policy");
    assert!(
        validate_linux_v5_trust_policy_for_test(
            &broadened,
            profiles::LINUX_OPERATOR_ALLOWED_SIGNERS.as_bytes(),
        )
        .is_err()
    );
}

#[test]
fn nested_profiles_separate_outer_relay_from_inner_authority() {
    for profile in [
        EvidenceProfileV3::LinuxExactV5,
        EvidenceProfileV3::NixExactV3,
        EvidenceProfileV3::WindowsNativeV6,
    ] {
        assert_eq!(
            profiles::expected_layers(profile),
            &[ManifestLayerIdV3::Outer, ManifestLayerIdV3::InnerReceipt,]
        );
        assert_eq!(
            profiles::authoritative_artifact(profile)
                .expect("compiled authority")
                .0,
            ManifestLayerIdV3::InnerReceipt
        );
        assert!(profiles::outer_verification_artifact(profile).is_some());
        let outer_mode = if profile == EvidenceProfileV3::WindowsNativeV6 {
            "MODES.tsv"
        } else {
            "OUTER-MODES.tsv"
        };
        let inner_inventory = if profile == EvidenceProfileV3::WindowsNativeV6 {
            "FILES.tsv"
        } else {
            "MODES.tsv"
        };
        assert_eq!(
            compiled_visible_mode_paths(profile, ManifestLayerIdV3::Outer)
                .expect("outer visible mode paths"),
            [outer_mode.to_string(), format!("receipt/{inner_inventory}"),]
                .into_iter()
                .collect()
        );
        assert_eq!(
            compiled_visible_mode_paths(profile, ManifestLayerIdV3::InnerReceipt)
                .expect("inner visible mode paths"),
            [inner_inventory.to_string()].into_iter().collect()
        );
    }
}

#[test]
fn github_v2_semantics_are_compiled_but_final_identity_stays_unpinned() {
    let candidate = exact_candidate();
    let temporary = private_tempdir();
    let receipts = temporary.path().join("receipts");
    fs::create_dir(&receipts).expect("receipts");
    private_dir(&receipts);
    let spec = AggregateBuildSpecV3 {
        automatic_transition: false,
        authority: AuthorityBoundary::all_closed(),
        candidate: candidate.clone(),
        platform_gates: vec![
            gate("macos-aarch64", EvidenceProfileV3::MacExactV6),
            gate("linux-x86_64", EvidenceProfileV3::LinuxExactV5),
            gate("nix-x86_64-linux", EvidenceProfileV3::NixExactV3),
            gate("windows-x86_64-native", EvidenceProfileV3::WindowsNativeV6),
            gate("github-actions", EvidenceProfileV3::GithubHostedExactV2),
        ],
        platform_policy: exact_platform_policy(),
        prerequisite_receipts: Vec::new(),
        profile_set: profiles::PROFILE_SET.to_string(),
        schema: BUILD_SPEC_SCHEMA.to_string(),
        schema_version: 3,
    };
    let error = validate_spec(
        &spec,
        ValidationPolicy {
            expected_candidate: &candidate,
            receipts_parent: &receipts,
        },
    )
    .expect_err("missing exact prerequisites/receipts must fail closed");
    assert!(error.to_string().contains("boundary"));
    assert!(profiles::is_unpinned(
        EvidenceProfileV3::GithubHostedExactV2
    ));
    assert!(profiles::authoritative_artifact(EvidenceProfileV3::GithubHostedExactV2).is_some());
    let prepared = profiles::frozen_github_prepared_profile_identity();
    let profile_path = Path::new(prepared.prepared_root).join("PROFILE.json");
    let profile_bytes = fs::read(&profile_path).expect("sealed prepared GitHub profile");
    validate_github_prepared_profile_for_test(&profile_bytes, &candidate)
        .expect("compiled GitHub v2 prepared-profile semantics");
    let mut tampered = profile_bytes;
    let offset = tampered
        .iter()
        .position(|byte| *byte == b'P')
        .expect("profile has a mutable byte");
    tampered[offset] = b'X';
    assert!(validate_github_prepared_profile_for_test(&tampered, &candidate).is_err());
}

#[test]
fn formal_aggregate_admits_only_terminal_pass_platform_receipts() {
    let pass = ObservedGateV3 {
        candidate_executed: true,
        candidate_failure: false,
        executed_steps: 5,
        harness_failure: false,
        pass: true,
        production_changed: Some(false),
        qualification: true,
        refs_changed: Some(false),
        status: "PASS".to_string(),
    };
    admit_formal_platform_pass_for_test(&pass).expect("exact PASS is admissible");

    let mut blocked = pass.clone();
    blocked.status = "BLOCKED_HARNESS".to_string();
    blocked.pass = false;
    blocked.qualification = false;
    blocked.candidate_executed = false;
    blocked.executed_steps = 0;
    blocked.harness_failure = true;
    assert!(
        admit_formal_platform_pass_for_test(&blocked)
            .expect_err("BLOCKED is diagnostic only")
            .to_string()
            .contains("NON_PASS_RECEIPT_NOT_AGGREGATE_INPUT")
    );

    let mut failed = pass.clone();
    failed.status = "FAIL_CANDIDATE".to_string();
    failed.pass = false;
    failed.qualification = false;
    failed.candidate_failure = true;
    assert!(admit_formal_platform_pass_for_test(&failed).is_err());

    let mut mutated = pass;
    mutated.refs_changed = Some(true);
    assert!(admit_formal_platform_pass_for_test(&mutated).is_err());
}

#[cfg(unix)]
#[test]
fn minimal_synthetic_receipt_graph_cannot_qualify_revision_7() {
    let temporary = private_tempdir();
    let receipts = temporary.path().join("receipts");
    fs::create_dir(&receipts).expect("receipts");
    private_dir(&receipts);
    let receipts = receipts.canonicalize().expect("canonical receipts root");

    let mut candidate = exact_candidate();
    let bundle_bytes = b"synthetic candidate bundle";
    candidate.bundle.relative_path = "candidate-test.bundle".to_string();
    candidate.bundle.sha256 = sha256(bundle_bytes);
    candidate.bundle.size_bytes = bundle_bytes.len() as u64;

    let mac = synthetic_mac_receipt(&receipts, &candidate);
    let linux = synthetic_linux_receipt(&receipts, &candidate);
    let nix = synthetic_nix_receipt(&receipts, &candidate);
    let windows = synthetic_windows_receipt(&receipts, &candidate);
    let portable = synthetic_portable_receipt(&receipts, &candidate, bundle_bytes);
    let path_trust = synthetic_path_trust_receipt(&receipts, &candidate);
    let upstream = synthetic_upstream_receipt(&receipts, &candidate);

    let spec = AggregateBuildSpecV3 {
        automatic_transition: false,
        authority: AuthorityBoundary::all_closed(),
        candidate: candidate.clone(),
        platform_gates: vec![
            gate("macos-aarch64", EvidenceProfileV3::MacExactV6),
            gate("linux-x86_64", EvidenceProfileV3::LinuxExactV5),
            gate("nix-x86_64-linux", EvidenceProfileV3::NixExactV3),
            gate("windows-x86_64-native", EvidenceProfileV3::WindowsNativeV6),
            gate("github-actions", EvidenceProfileV3::GithubHostedExactV2),
        ],
        platform_policy: exact_platform_policy(),
        prerequisite_receipts: vec![
            prerequisite(
                "portable-inputs",
                EvidenceProfileV3::PortableInputsV1,
                portable,
            ),
            prerequisite(
                "canonical-path-trust",
                EvidenceProfileV3::CanonicalPathTrustV2,
                path_trust,
            ),
            prerequisite(
                "upstream-cutoff-observation",
                EvidenceProfileV3::UpstreamCutoffObservationV1,
                upstream,
            ),
        ],
        profile_set: profiles::PROFILE_SET.to_string(),
        schema: BUILD_SPEC_SCHEMA.to_string(),
        schema_version: 3,
    };
    let error = validate_spec(
        &spec,
        ValidationPolicy {
            expected_candidate: &candidate,
            receipts_parent: &receipts,
        },
    )
    .expect_err("minimal synthetic evidence must fail closed");
    assert!(
        error
            .to_string()
            .contains("pinned platform profile requires a receipt")
    );

    let _ = (mac, linux, nix, windows);
}

#[cfg(unix)]
#[test]
fn windows_pass_shaped_receipt_omitting_v6_execution_artifacts_is_rejected() {
    let temporary = private_tempdir();
    let receipts = temporary.path().join("receipts");
    fs::create_dir(&receipts).expect("receipts");
    private_dir(&receipts);
    let receipts = receipts.canonicalize().expect("canonical receipts");
    let candidate = exact_candidate();
    let receipt = synthetic_windows_receipt(&receipts, &candidate);
    let error = validate_receipt_for_test(&receipt, &receipts, &candidate)
        .expect_err("invented PASS booleans cannot replace the v6 execution closure");
    assert!(error.to_string().contains("PROFILE_IDENTITY_UNPINNED"));

    let required = profiles::required_artifacts(EvidenceProfileV3::WindowsNativeV6)
        .iter()
        .map(|artifact| artifact.path)
        .collect::<BTreeSet<_>>();
    for path in [
        "resource-preflight.json",
        "toolchain-exactness.txt",
        "tool-inventory.json",
        "git-capture-self-test.txt",
        "environment-sanitization.txt",
        "native-capture-self-test.txt",
        "preflight.txt",
        "postflight.txt",
        "test-suite-counts.tsv",
        "candidate-execution-started.txt",
        "candidate-execution-completed.txt",
        "input-verification.tsv",
        "driver-verification.tsv",
    ] {
        assert!(
            required.contains(path),
            "compiled Windows roster omits {path}"
        );
    }
}

#[cfg(unix)]
#[test]
fn complete_self_sealed_forged_receipt_cannot_select_its_own_identity() {
    let temporary = private_tempdir();
    let receipts = temporary.path().join("receipts");
    fs::create_dir(&receipts).expect("receipts");
    private_dir(&receipts);
    let receipts = receipts.canonicalize().expect("canonical receipts");
    let candidate = exact_candidate();
    let root = new_receipt_root(&receipts, "complete-forged-path-trust");
    let status = format!(
        "schema=hepta_vnext_canonical_path_trust_v2\nstatus=pass\nreceipt_attempt=4\ncandidate_head={}\ncandidate_tree={}\ncandidate_parent={}\ncanonical_worktree=/Volumes/T5/hepta-vnext/worktrees/main-integration\ncanonical_branch=hepta/vnext-main-integration-20260811\ncanonical_local_ref_exact=true\nworktree_clean=true\nt5_uuid_exact=true\nt5_owners_enabled=true\nopenclaw_agent_workspaces_aligned=true\ncodex_configs_checked=6\ncodex_lane_scoped_trust_aligned=true\nexplicit_old_workspace_trust_entries=false\nagent_instructions_point_to_t5=true\nagent_identities_point_to_t5=true\nold_workspace_paths_frozen=true\nremote_candidate_ref=09e9e9ff7fa6b6c1d129d0c7a858979823e13ae8\nremote_candidate_switch_deferred=true\nlocal_main_head=fe848052ceed06ed431e20893f15516fd349ffe5\nremote_main_head=1577a50e37c6332ab267dea9d838dab8b8c07536\nremote_default_branch=main\ncandidate_archive_branch_exact=true\ncandidate_archive_tag_exact=true\nhosted_qualification_ref_exact=true\ndefault_main_changed=false\nproduction_changed=false\n",
        candidate.head, candidate.tree, candidate.parents[0],
    );
    for artifact in profiles::required_artifacts(EvidenceProfileV3::CanonicalPathTrustV2) {
        let bytes = if artifact.path == "status.txt" {
            status.as_bytes()
        } else {
            b"self-authored forged evidence\n"
        };
        write_fixture_file(&root.join(artifact.path), bytes);
    }
    let mut receipt = direct_binding(
        root.clone(),
        EvidenceProfileV3::CanonicalPathTrustV2,
        "SHA256SUMS",
        "MODES.tsv",
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    );
    receipt.required_artifacts = artifact_bindings(&root, receipt.profile);
    let error = validate_receipt_for_test(&receipt, &receipts, &candidate)
        .expect_err("a complete self-sealed forged roster must not select its own identity");
    assert!(error.to_string().contains("compiled exact identity"));
}

#[cfg(target_os = "macos")]
#[test]
fn mac_reemitted_wrapper_preserves_real_original_provenance_and_detects_races() {
    use std::os::unix::fs::PermissionsExt;

    let temporary = private_tempdir();
    let receipts = temporary.path().join("receipts");
    fs::create_dir(&receipts).expect("receipts");
    private_dir(&receipts);
    let receipts = receipts.canonicalize().expect("canonical receipts");
    let original_root = Path::new(
        "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-mac-exact-attempt-2-20260813T052744Z",
    );
    let receipts_parent = Path::new("/Volumes/T5/hepta-vnext/artifacts/receipts");
    let frozen = profiles::frozen_original_identity(EvidenceProfileV3::MacExactV6)
        .expect("Mac original identity");
    let original = load_legacy_manifest_with_policy(
        original_root,
        frozen.manifest_relative_path,
        frozen.manifest_sha256,
        frozen.entry_count,
        LegacyExtendedMetadataPolicy::MacAttempt2,
    )
    .expect("genuine Mac original with intended xattr, ACL, and hardlinks");
    assert_eq!(
        original
            .hardlink_topology
            .iter()
            .filter(|byte| **byte == b'\n')
            .count(),
        2
    );
    assert_eq!(
        original
            .extended_metadata_inventory
            .iter()
            .filter(|byte| **byte == b'\n')
            .count(),
        4
    );
    verify_frozen_original_provenance_for_test(&original, frozen)
        .expect("Mac compiled non-content provenance pins");
    for mutate in [
        |identity: &mut profiles::FrozenOriginalIdentityV3| {
            identity.metadata_inventory.sha256 =
                "0000000000000000000000000000000000000000000000000000000000000000";
        },
        |identity: &mut profiles::FrozenOriginalIdentityV3| {
            identity.hardlink_topology.row_count += 1;
        },
        |identity: &mut profiles::FrozenOriginalIdentityV3| {
            identity.extended_metadata_inventory.size_bytes += 1;
        },
    ] {
        let mut tampered = frozen;
        mutate(&mut tampered);
        assert!(verify_frozen_original_provenance_for_test(&original, tampered).is_err());
    }

    let wrapper = receipts.join("mac-wrapper");
    fs::create_dir(&wrapper).expect("wrapper");
    private_dir(&wrapper);
    let original_tree = wrapper.join("provenance/original-tree");
    let canonical_relative = "qualification-status.txt";
    fs::copy(
        original_root.join(canonical_relative),
        wrapper.join(canonical_relative),
    )
    .expect("canonical semantic copy");
    strip_macos_extended_metadata(&wrapper.join(canonical_relative));
    fs::set_permissions(
        wrapper.join(canonical_relative),
        fs::Permissions::from_mode(0o400),
    )
    .expect("canonical semantic mode");
    for (relative, entry) in &original.entries {
        let destination = original_tree.join(relative);
        fs::create_dir_all(destination.parent().expect("archive parent")).expect("archive dirs");
        private_dir(destination.parent().expect("archive parent"));
        fs::copy(original_root.join(relative), &destination).expect("archive copy");
        strip_macos_extended_metadata(&destination);
        fs::set_permissions(&destination, fs::Permissions::from_mode(0o400))
            .expect("archive file mode");
        assert_eq!(
            sha256(&fs::read(&destination).expect("archive bytes")),
            entry.sha256
        );
    }
    let provenance = wrapper.join("provenance");
    fs::set_permissions(&provenance, fs::Permissions::from_mode(0o700)).expect("provenance mode");
    let topology = write_bound_artifact(
        &wrapper,
        "provenance/hardlink-topology.tsv",
        &original.hardlink_topology,
    );
    let metadata = write_bound_artifact(
        &wrapper,
        "provenance/original-metadata.tsv",
        &original.metadata_inventory,
    );
    let extended = write_bound_artifact(
        &wrapper,
        "provenance/original-extended-metadata.tsv",
        &original.extended_metadata_inventory,
    );
    let reemitter = write_bound_artifact(
        &wrapper,
        "provenance/reemitter",
        b"synthetic frozen reemitter\n",
    );
    let mut projection_rows = original
        .entries
        .iter()
        .map(|(relative, entry)| {
            format!(
                "archive\t./{relative}\t{}\t{}\t./provenance/original-tree/{relative}\n",
                entry.sha256, entry.size_bytes
            )
        })
        .collect::<Vec<_>>();
    let canonical = original
        .entries
        .get(canonical_relative)
        .expect("canonical original entry");
    projection_rows.push(format!(
        "canonical\t./{canonical_relative}\t{}\t{}\t./{canonical_relative}\n",
        canonical.sha256, canonical.size_bytes,
    ));
    projection_rows.sort();
    let projection = write_bound_artifact(
        &wrapper,
        "provenance/projection-map.tsv",
        projection_rows.concat().as_bytes(),
    );
    let original_binding = OriginalReceiptBindingV3 {
        manifest_entry_count: frozen.entry_count,
        manifest_relative_path: frozen.manifest_relative_path.to_string(),
        manifest_sha256: frozen.manifest_sha256.to_string(),
        receipt_root: frozen.receipt_root.to_string(),
    };
    let attestation_bytes = wrapper_attestation_bytes_for_test(
        &original_binding,
        &sha256(&original.inventory),
        &topology,
        &extended,
        &metadata,
        &projection,
        &reemitter,
    )
    .expect("attestation");
    let attestation = write_bound_artifact(
        &wrapper,
        "provenance/reemission-attestation.json",
        &attestation_bytes,
    );
    let layer = seal_fixture_layer(
        &wrapper,
        ManifestLayerIdV3::Outer,
        ".",
        "SHA256SUMS",
        "MODES.tsv",
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    );
    let binding = ReceiptEvidenceBindingV3 {
        manifest_layers: vec![layer],
        profile: EvidenceProfileV3::MacExactV6,
        provenance: ReceiptProvenanceV3::ReemittedWrapper {
            attestation,
            hardlink_topology: topology,
            original: original_binding,
            original_extended_metadata_inventory: extended,
            original_metadata_inventory: metadata,
            original_tree_relative_path: "provenance/original-tree".to_string(),
            projection_map: projection,
            reemitter,
        },
        receipt_root: path_text(&wrapper),
        required_artifacts: Vec::new(),
    };
    validate_reemitted_wrapper_for_test(&binding, receipts_parent, frozen, || Ok(()))
        .expect("valid genuine-Mac provenance wrapper");

    let mut direct = binding.clone();
    direct.provenance = ReceiptProvenanceV3::Direct;
    let direct_error =
        validate_reemitted_wrapper_for_test(&direct, receipts_parent, frozen, || Ok(()))
            .expect_err("Mac profile must reject direct provenance");
    assert!(
        direct_error
            .to_string()
            .contains("requires provenance-preserving")
    );

    let mut wrong = frozen;
    wrong.manifest_sha256 = "0".repeat(64).leak();
    assert!(
        validate_reemitted_wrapper_for_test(&binding, receipts_parent, wrong, || Ok(())).is_err()
    );

    let race = validate_reemitted_wrapper_for_test(&binding, receipts_parent, frozen, || {
        Err(crate::AcceptanceError::Invalid(
            "deterministic original-race hook".to_string(),
        ))
    })
    .expect_err("original mutation/race hook must abort provenance validation");
    assert!(race.to_string().contains("original-race"));
}

#[cfg(unix)]
#[test]
fn portable_original_non_content_provenance_matches_compiled_identity() {
    let frozen = profiles::frozen_original_identity(EvidenceProfileV3::PortableInputsV1)
        .expect("portable original identity");
    let original = load_legacy_manifest_with_policy(
        Path::new(frozen.receipt_root),
        frozen.manifest_relative_path,
        frozen.manifest_sha256,
        frozen.entry_count,
        LegacyExtendedMetadataPolicy::PortableInputs,
    )
    .expect("frozen portable original");
    for (bytes, identity) in [
        (&original.metadata_inventory, frozen.metadata_inventory),
        (&original.hardlink_topology, frozen.hardlink_topology),
        (
            &original.extended_metadata_inventory,
            frozen.extended_metadata_inventory,
        ),
    ] {
        assert_eq!(bytes.len(), identity.size_bytes);
        assert_eq!(
            bytes.iter().filter(|byte| **byte == b'\n').count(),
            identity.row_count
        );
        assert_eq!(sha256(bytes), identity.sha256);
    }
    original.reverify().expect("portable original reverify");
}

#[cfg(unix)]
#[test]
fn reemitted_wrapper_rejects_self_resealed_projection_provenance_and_original_races() {
    use std::os::unix::fs::PermissionsExt;

    let valid = synthetic_reemitted_wrapper_fixture();
    validate_synthetic_reemitted_wrapper(&valid).expect("valid compact provenance wrapper");

    let mut extra = synthetic_reemitted_wrapper_fixture();
    write_fixture_file(
        &extra.wrapper_root.join("provenance/unaudited-payload"),
        b"self-sealed but outside the exact namespace\n",
    );
    reseal_synthetic_wrapper(&mut extra);
    assert!(validate_synthetic_reemitted_wrapper(&extra).is_err());

    let mut incomplete = synthetic_reemitted_wrapper_fixture();
    rewrite_projection(&mut incomplete, |line| {
        (!line.starts_with("canonical\t")).then_some(line.to_string())
    });
    reseal_synthetic_wrapper(&mut incomplete);
    assert!(validate_synthetic_reemitted_wrapper(&incomplete).is_err());

    let mut rewritten = synthetic_reemitted_wrapper_fixture();
    let rewritten_bytes = b"rewritten authority projection\n";
    rewrite_fixture_file(
        &rewritten.wrapper_root.join("artifact.txt"),
        rewritten_bytes,
    );
    rewrite_projection(&mut rewritten, |line| {
        if line.starts_with("canonical\t./artifact.txt\t") {
            Some(format!(
                "canonical\t./artifact.txt\t{}\t{}\t./artifact.txt",
                sha256(rewritten_bytes),
                rewritten_bytes.len()
            ))
        } else {
            Some(line.to_string())
        }
    });
    reseal_synthetic_wrapper(&mut rewritten);
    assert!(validate_synthetic_reemitted_wrapper(&rewritten).is_err());

    let mut archive_drop = synthetic_reemitted_wrapper_fixture();
    fs::remove_file(
        archive_drop
            .wrapper_root
            .join("provenance/original-tree/nested/data.txt"),
    )
    .expect("drop archive copy");
    rewrite_projection(&mut archive_drop, |line| {
        (!line.starts_with("archive\t./nested/data.txt\t")).then_some(line.to_string())
    });
    reseal_synthetic_wrapper(&mut archive_drop);
    assert!(validate_synthetic_reemitted_wrapper(&archive_drop).is_err());

    for relative in [
        "provenance/original-metadata.tsv",
        "provenance/hardlink-topology.tsv",
        "provenance/original-extended-metadata.tsv",
    ] {
        let mut metadata = synthetic_reemitted_wrapper_fixture();
        rewrite_provenance_artifact(&mut metadata, relative, b"self-resealed forgery\n");
        reseal_synthetic_wrapper(&mut metadata);
        assert!(
            validate_synthetic_reemitted_wrapper(&metadata).is_err(),
            "self-resealed provenance forgery must fail: {relative}"
        );
    }

    let race = synthetic_reemitted_wrapper_fixture();
    let raced_path = race.original_root.join("artifact.txt");
    let error =
        validate_reemitted_wrapper_for_test(&race.binding, &race.receipts, race.frozen, || {
            fs::set_permissions(&raced_path, fs::Permissions::from_mode(0o500))?;
            Ok(())
        })
        .expect_err("a real original metadata race must fail final reverification");
    assert!(error.to_string().contains("original receipt changed"));
}

#[cfg(target_os = "macos")]
fn strip_macos_extended_metadata(path: &Path) {
    let xattr = std::process::Command::new("/usr/bin/xattr")
        .args(["-c"])
        .arg(path)
        .status()
        .expect("invoke xattr -c");
    assert!(xattr.success(), "strip copied xattrs");
    let acl = std::process::Command::new("/bin/chmod")
        .args(["-N"])
        .arg(path)
        .status()
        .expect("invoke chmod -N");
    assert!(acl.success(), "strip copied ACL");
}

#[cfg(unix)]
#[test]
fn frozen_path_trust_attempt_4_and_upstream_receipts_validate_directly() {
    let candidate = exact_candidate();
    let receipts = Path::new("/Volumes/T5/hepta-vnext/artifacts/receipts");
    for profile in [
        EvidenceProfileV3::CanonicalPathTrustV2,
        EvidenceProfileV3::UpstreamCutoffObservationV1,
    ] {
        let receipt = frozen_direct_binding(profile);
        validate_receipt_for_test(&receipt, receipts, &candidate)
            .unwrap_or_else(|error| panic!("frozen {profile:?} receipt: {error}"));
    }
}

#[cfg(unix)]
#[test]
fn frozen_mac_portable_and_nix_rev7_wrappers_validate_with_live_provenance() {
    let candidate = exact_candidate();
    let receipts = Path::new("/Volumes/T5/hepta-vnext/artifacts/receipts");
    for profile in [
        EvidenceProfileV3::MacExactV6,
        EvidenceProfileV3::PortableInputsV1,
        EvidenceProfileV3::NixExactV3,
    ] {
        let receipt = frozen_reemitted_binding(profile);
        validate_receipt_for_test(&receipt, receipts, &candidate)
            .unwrap_or_else(|error| panic!("frozen {profile:?} rev7 wrapper: {error}"));
    }
}

#[cfg(unix)]
#[test]
fn frozen_nix_raw_status_is_qualification_pass_not_container_lifecycle_state() {
    let root = Path::new(
        "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-nix-exact-reemitted-rev7-prepared-20260813T185012Z",
    );
    let mut inner = parse_key_values(
        &fs::read(root.join("receipt/result.txt")).expect("read frozen Nix result"),
    )
    .expect("parse frozen Nix result");
    let outer = parse_key_values(
        &fs::read(root.join("LOCAL-VERIFICATION.txt")).expect("read frozen Nix verification"),
    )
    .expect("parse frozen Nix verification");

    verify_nix_exact_bindings_for_test(&inner, &outer)
        .expect("the sealed raw qualification status is PASS");

    inner.insert("raw_status".to_string(), "exited".to_string());
    let error = verify_nix_exact_bindings_for_test(&inner, &outer)
        .expect_err("container lifecycle text must not substitute for raw qualification PASS");
    assert!(error.to_string().contains("raw_status"));
}

#[cfg(unix)]
#[test]
fn canonical_path_trust_attempt_3_cannot_substitute_for_frozen_attempt_4() {
    let candidate = exact_candidate();
    let receipts = Path::new("/Volumes/T5/hepta-vnext/artifacts/receipts");
    let mut binding = frozen_direct_binding(EvidenceProfileV3::CanonicalPathTrustV2);
    binding.receipt_root = "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-canonical-path-trust-v3-attempt-3-20260813T113220Z".to_string();
    let error = validate_receipt_for_test(&binding, receipts, &candidate)
        .expect_err("PathTrust attempt-3 must not replace frozen attempt-4");
    assert!(error.to_string().contains("compiled exact identity"));

    let attempt_3 = Path::new(
        "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-52ec4b3868-canonical-path-trust-v3-attempt-3-20260813T113220Z",
    );
    let marker_error = reject_conflicting_markers_for_test(
        attempt_3,
        "SHA256SUMS",
        "9d22240b2a34c998ef439ea4ff205a43b65a4c3b37abb90ad17a5ff269b17541",
        16,
    )
    .expect_err("attempt-3 PASS is contradicted by prior-partial BLOCKED_HARNESS");
    assert!(
        marker_error
            .to_string()
            .contains("conflicting terminal marker")
    );
}

#[test]
fn strict_json_rejects_duplicates_and_preserves_exact_types() {
    assert!(super::strict_json::parse(br#"{"pass":false,"pass":true}"#).is_err());
    assert!(super::strict_json::parse(br#"{"nested":{"status":"FAIL","status":"PASS"}}"#).is_err());
    let value = super::strict_json::parse(br#"{"pass":"true"}"#).expect("strict JSON");
    assert!(value["pass"].as_bool().is_none());
}

#[test]
fn windows_revision_7_requires_whole_second_utc_default_floors_and_nonce_paths() {
    assert!(valid_utc_timestamp_for_test("2026-08-13T13:18:52Z"));
    assert!(!valid_utc_timestamp_for_test(
        "2026-08-13T13:18:52.1234567Z"
    ));

    validate_windows_resource_floors_for_test(
        25_769_803_776,
        25_769_803_776,
        1_610_612_736,
        1_610_612_736,
    )
    .expect("exact default floors qualify");
    assert!(
        validate_windows_resource_floors_for_test(25_769_803_776, 1, 1_610_612_736, 1_610_612_736,)
            .is_err()
    );
    assert!(
        validate_windows_resource_floors_for_test(
            25_769_803_775,
            25_769_803_776,
            1_610_612_736,
            1_610_612_736,
        )
        .is_err()
    );

    validate_windows_nonce_paths_for_test(
        "52ec0d66",
        r"C:\q\52ec-52ec0d66",
        r"C:\q\52ec-52ec0d66\s",
        r"C:\q\52ec-52ec0d66\vendor",
        r"C:\q\52ec-52ec0d66\target",
    )
    .expect("exact nonce-derived paths qualify");
    assert!(
        validate_windows_nonce_paths_for_test(
            "52ec0d66",
            r"D:\q\52ec-52ec0d66",
            r"D:\q\52ec-52ec0d66\s",
            r"D:\q\52ec-52ec0d66\vendor",
            r"D:\q\52ec-52ec0d66\target",
        )
        .is_err()
    );
    assert!(
        validate_windows_nonce_paths_for_test(
            "52ec0d66",
            r"C:\q\52ec-52ec0d66",
            r"C:\q\52ec-52ec0d66\source",
            r"C:\q\52ec-52ec0d66\vendor",
            r"C:\q\52ec-52ec0d66\target",
        )
        .is_err()
    );
}

#[test]
fn key_values_reject_duplicate_and_contradictory_fields() {
    assert!(parse_key_values(b"status=PASS\nstatus=FAIL\n").is_err());
    assert!(parse_key_values(b"pass=true\nother=1\n").is_ok());
    assert!(parse_key_values(b"pass=true\npass=false\n").is_err());
    assert!(parse_key_values(b"pass=true").is_err());
}

#[test]
fn reserved_core_aliases_cannot_shadow_compiled_claims() {
    let values = parse_key_values(
        b"schema=fixed\nstatus=PASS\npass=false\nsupporting_observation=present\n",
    )
    .expect("syntactically valid key/value evidence");
    assert!(reject_reserved_kv_fields(&values, &["schema", "status"]).is_err());

    let values = parse_key_values(b"schema=fixed\nstatus=PASS\nsupporting_observation=present\n")
        .expect("syntactically valid key/value evidence");
    reject_reserved_kv_fields(&values, &["schema", "status"])
        .expect("unreserved supporting evidence remains permitted");
}

#[test]
fn linux_and_nix_step_profiles_stop_at_the_first_failure() {
    let expected = ["one", "two", "three"];
    let first_failure = step_rows(&expected[..2], &[0, 101]);
    assert!(
        parse_step_tsv(
            first_failure.as_bytes(),
            "FAIL_CANDIDATE",
            &expected,
            StepPolicy::PrefixFirstFailure,
        )
        .is_ok()
    );

    let continued_after_failure = step_rows(&expected, &[0, 101, 0]);
    assert!(
        parse_step_tsv(
            continued_after_failure.as_bytes(),
            "FAIL_CANDIDATE",
            &expected,
            StepPolicy::PrefixFirstFailure,
        )
        .is_err()
    );

    let two_failures = step_rows(&expected, &[1, 0, 2]);
    assert!(
        parse_step_tsv(
            two_failures.as_bytes(),
            "FAIL_CANDIDATE",
            &expected,
            StepPolicy::PrefixFirstFailure,
        )
        .is_err()
    );

    let incomplete_pass = step_rows(&expected[..2], &[0, 0]);
    assert!(
        parse_step_tsv(
            incomplete_pass.as_bytes(),
            "PASS",
            &expected,
            StepPolicy::PrefixFirstFailure,
        )
        .is_err()
    );
}

#[test]
fn windows_candidate_failure_requires_full_roster_and_exact_json_results() {
    let expected = ["one", "two", "three"];
    let full_failure = windows_step_rows(&expected, &[0, 101, 1], "candidate");
    let parsed = parse_step_tsv(
        full_failure.as_bytes(),
        "FAIL_CANDIDATE",
        &expected,
        StepPolicy::WindowsFullCandidateRun,
    )
    .expect("Windows runs the complete candidate roster and permits multiple failures");
    require_windows_step_results(
        Some(&json!({"one": 0, "two": 101, "three": 1})),
        &parsed,
        "FAIL_CANDIDATE",
    )
    .expect("JSON results match the exact TSV return codes");
    assert!(
        require_windows_step_results(
            Some(&json!({"one": 0, "two": 101, "three": 0})),
            &parsed,
            "FAIL_CANDIDATE",
        )
        .is_err()
    );

    let partial = windows_step_rows(&expected[..2], &[0, 101], "candidate");
    assert!(
        parse_step_tsv(
            partial.as_bytes(),
            "FAIL_CANDIDATE",
            &expected,
            StepPolicy::WindowsFullCandidateRun,
        )
        .is_err()
    );
}

#[test]
fn windows_blocked_harness_may_have_a_partial_roster_but_no_result_claims() {
    let expected = ["one", "two", "three"];
    parse_step_tsv(
        b"",
        "BLOCKED_HARNESS",
        &expected,
        StepPolicy::WindowsFullCandidateRun,
    )
    .expect("a harness blocker may occur after the start marker but before its first step row");
    let partial = format!(
        "one\t0\tcandidate\t{TEST_TIMESTAMP}\t{TEST_TIMESTAMP}\tone.log\ntwo\t92\tharness\t{TEST_TIMESTAMP}\t{TEST_TIMESTAMP}\ttwo.log\n"
    );
    let parsed = parse_step_tsv(
        partial.as_bytes(),
        "BLOCKED_HARNESS",
        &expected,
        StepPolicy::WindowsFullCandidateRun,
    )
    .expect("the first harness failure terminates a partial Windows roster");
    require_windows_step_results(Some(&json!({})), &parsed, "BLOCKED_HARNESS")
        .expect("blocked result does not claim completed candidate step results");
    assert!(
        require_windows_step_results(
            Some(&json!({"one": 0, "two": 92})),
            &parsed,
            "BLOCKED_HARNESS",
        )
        .is_err()
    );

    let candidate_failure_mislabeled_as_harness =
        windows_step_rows(&expected[..2], &[0, 101], "candidate");
    assert!(
        parse_step_tsv(
            candidate_failure_mislabeled_as_harness.as_bytes(),
            "BLOCKED_HARNESS",
            &expected,
            StepPolicy::WindowsFullCandidateRun,
        )
        .is_err()
    );
}

#[test]
fn step_timestamps_must_be_real_ordered_utc_seconds() {
    let expected = ["one"];
    for invalid in [
        "one\t00\t2026-08-13T08:00:00Z\t2026-08-13T08:00:00Z\n",
        "one\t0\t2026-02-30T08:00:00Z\t2026-03-01T08:00:00Z\n",
        "one\t0\t2026-08-13T25:00:00Z\t2026-08-14T00:00:00Z\n",
        "one\t0\t2026-08-13T08:00:01Z\t2026-08-13T08:00:00Z\n",
    ] {
        assert!(
            parse_step_tsv(
                invalid.as_bytes(),
                "PASS",
                &expected,
                StepPolicy::PrefixFirstFailure,
            )
            .is_err()
        );
    }
}

#[test]
fn linux_status_cannot_claim_completion_after_first_failure() {
    let coherent = parse_key_values(
        b"harness_blocked=false\ndata_deleted=false\npromotion_authority=false\nharness_preflight_pass=true\ncandidate_execution_started=true\ncandidate_execution_completed=false\npostflight_verified=false\nsource_identity=match\nworktree_clean=true\n",
    )
    .expect("coherent Linux failure fields");
    validate_kv_execution_fields(EvidenceProfileV3::LinuxExactV5, &coherent, "FAIL_CANDIDATE")
        .expect("coherent first-failure shape");

    let contradictory = parse_key_values(
        b"harness_blocked=false\ndata_deleted=false\npromotion_authority=false\nharness_preflight_pass=true\ncandidate_execution_started=true\ncandidate_execution_completed=true\npostflight_verified=false\nsource_identity=match\nworktree_clean=true\n",
    )
    .expect("syntactically valid contradiction");
    assert!(
        validate_kv_execution_fields(
            EvidenceProfileV3::LinuxExactV5,
            &contradictory,
            "FAIL_CANDIDATE",
        )
        .is_err()
    );
}

#[test]
fn linux_watchdog_observations_are_exact_bound_and_cover_the_candidate_window() {
    let observations = b"2026-08-13T08:00:01Z\tsample=periodic\trequest_sequence=0\tlisteners=0\tworkers=0\tother_hepta_builds=0\tlock_held=true\n2026-08-13T08:00:02Z\tsample=boundary_candidate_start\trequest_sequence=1\tlisteners=0\tworkers=0\tother_hepta_builds=0\tlock_held=true\n2026-08-13T08:00:15Z\tsample=periodic\trequest_sequence=0\tlisteners=0\tworkers=0\tother_hepta_builds=0\tlock_held=true\n2026-08-13T08:00:16Z\tsample=boundary_candidate_complete\trequest_sequence=2\tlisteners=0\tworkers=0\tother_hepta_builds=0\tlock_held=true\n";
    let exact = parse_key_values(
        b"resource_watchdog_started_at=2026-08-13T08:00:00Z\nresource_watchdog_stopped_at=2026-08-13T08:00:17Z\ncandidate_window_started_at=2026-08-13T08:00:02Z\ncandidate_window_completed_at=2026-08-13T08:00:16Z\nresource_watchdog_row_count=4\nresource_watchdog_first_observed_at=2026-08-13T08:00:01Z\nresource_watchdog_last_observed_at=2026-08-13T08:00:16Z\n",
    )
    .expect("watchdog result fields");
    verify_linux_watchdog_observations_for_test(observations, &exact)
        .expect("exact watchdog observations");

    for malformed in [
        b"2026-08-13T08:00:01Z\tsample=periodic\trequest_sequence=0\tlisteners=1\tworkers=0\tother_hepta_builds=0\tlock_held=true\n".as_slice(),
        b"2026-08-13T08:00:01Z\tsample=periodic\trequest_sequence=3\tlisteners=0\tworkers=0\tother_hepta_builds=0\tlock_held=true\n".as_slice(),
        b"2026-08-13T08:00:01Z\tsample=periodic\trequest_sequence=0\tlisteners=0\tworkers=0\tother_hepta_builds=0\tlock_held=false\n".as_slice(),
        b"2026-08-13T08:00:01Z\tsample=periodic\trequest_sequence=0\tlisteners=0\tworkers=0\tother_hepta_builds=0\tlock_held=true\r\n".as_slice(),
        b"2026-08-13T08:00:16Z\tsample=boundary_candidate_complete\trequest_sequence=2\tlisteners=0\tworkers=0\tother_hepta_builds=0\tlock_held=true\n2026-08-13T08:00:02Z\tsample=boundary_candidate_start\trequest_sequence=1\tlisteners=0\tworkers=0\tother_hepta_builds=0\tlock_held=true\n".as_slice(),
    ] {
        assert!(verify_linux_watchdog_observations_for_test(malformed, &exact).is_err());
    }

    let wrong_count = parse_key_values(
        b"resource_watchdog_started_at=2026-08-13T08:00:00Z\nresource_watchdog_stopped_at=2026-08-13T08:00:17Z\ncandidate_window_started_at=2026-08-13T08:00:02Z\ncandidate_window_completed_at=2026-08-13T08:00:16Z\nresource_watchdog_row_count=1\nresource_watchdog_first_observed_at=2026-08-13T08:00:01Z\nresource_watchdog_last_observed_at=2026-08-13T08:00:16Z\n",
    )
    .expect("watchdog result fields with wrong count");
    assert!(verify_linux_watchdog_observations_for_test(observations, &wrong_count).is_err());

    let uncovered = parse_key_values(
        b"resource_watchdog_started_at=2026-08-13T08:00:00Z\nresource_watchdog_stopped_at=2026-08-13T08:00:17Z\ncandidate_window_started_at=2026-08-13T08:00:02Z\ncandidate_window_completed_at=2026-08-13T08:00:17Z\nresource_watchdog_row_count=2\nresource_watchdog_first_observed_at=2026-08-13T08:00:01Z\nresource_watchdog_last_observed_at=2026-08-13T08:00:16Z\n",
    )
    .expect("watchdog result fields with uncovered end");
    assert!(verify_linux_watchdog_observations_for_test(observations, &uncovered).is_err());

    let equal_boundary_rows = b"2026-08-13T08:00:02Z\tsample=periodic\trequest_sequence=0\tlisteners=0\tworkers=0\tother_hepta_builds=0\tlock_held=true\n2026-08-13T08:00:02Z\tsample=boundary_candidate_start\trequest_sequence=1\tlisteners=0\tworkers=0\tother_hepta_builds=0\tlock_held=true\n2026-08-13T08:00:17Z\tsample=boundary_candidate_complete\trequest_sequence=2\tlisteners=0\tworkers=0\tother_hepta_builds=0\tlock_held=true\n";
    let equal_boundary_result = parse_key_values(
        b"resource_watchdog_started_at=2026-08-13T08:00:01Z\nresource_watchdog_stopped_at=2026-08-13T08:00:18Z\ncandidate_window_started_at=2026-08-13T08:00:02Z\ncandidate_window_completed_at=2026-08-13T08:00:17Z\nresource_watchdog_row_count=3\nresource_watchdog_first_observed_at=2026-08-13T08:00:02Z\nresource_watchdog_last_observed_at=2026-08-13T08:00:17Z\n",
    )
    .expect("watchdog result fields with equal boundary rows");
    verify_linux_watchdog_observations_for_test(equal_boundary_rows, &equal_boundary_result)
        .expect("equal-second synchronous boundary rows are monotonic");
}

#[test]
fn mode_rows_reject_duplicates_special_bits_and_incomplete_syntax() {
    assert!(
        parse_mode_rows(
            b"Regular File\t400\t42\t./MODES.tsv\nRegular File\t400\t42\t./MODES.tsv\n",
            ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
        )
        .is_err()
    );
    assert!(
        parse_mode_rows(
            b"Regular File\t4400\t42\t./artifact\n",
            ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
        )
        .is_err()
    );
    assert!(
        parse_mode_rows(
            b"Regular File\t0400\t042\t./artifact\n",
            ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
        )
        .is_err()
    );
    assert!(
        parse_mode_rows(
            b"Directory\t700\t-\t.\nRegular File\t400\t42\t./MODES.tsv\n",
            ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
        )
        .is_ok()
    );
    assert!(
        parse_mode_rows(
            b"Directory\t-\t.\nRegular File\t42\t./FILES.tsv\n",
            ModeManifestFormatV3::WindowsNtfsTypeSizePathTsvV1,
        )
        .is_ok()
    );
}

#[cfg(unix)]
#[test]
fn mode_manifest_requires_full_root_file_and_directory_closure() {
    let (complete_temp, complete_binding, complete_manifest) = mode_fixture(true, true, true);
    let allowed = ["MODES.tsv".to_string()]
        .into_iter()
        .collect::<BTreeSet<_>>();
    verify_mode_manifest(&complete_binding, &complete_manifest, &allowed)
        .expect("complete typed mode closure");
    drop(complete_temp);

    let (_incomplete_temp, incomplete_binding, incomplete_manifest) =
        mode_fixture(false, true, false);
    assert!(verify_mode_manifest(&incomplete_binding, &incomplete_manifest, &allowed).is_err());

    let (_empty_temp, empty_binding, empty_manifest) = mode_fixture(true, false, false);
    assert!(verify_mode_manifest(&empty_binding, &empty_manifest, &allowed).is_err());
}

#[cfg(unix)]
#[test]
fn aggregate_modes_inventory_reaches_an_exact_self_size_fixed_point() {
    use std::os::unix::fs::PermissionsExt;

    let temporary = private_tempdir();
    let root = temporary.path().join("aggregate");
    fs::create_dir(&root).expect("aggregate root");
    private_dir(&root);
    let payloads = [
        ("aggregate-build-record.json", b"record".as_slice()),
        ("build-spec.json", b"spec".as_slice()),
        ("qualification-packet.json", b"packet".as_slice()),
    ];
    for (relative, bytes) in payloads {
        fs::write(root.join(relative), bytes).expect("aggregate payload");
        fs::set_permissions(root.join(relative), fs::Permissions::from_mode(0o400))
            .expect("aggregate payload mode");
    }

    let modes = aggregate_modes_bytes(&root).expect("fixed-point aggregate modes");
    fs::write(root.join("MODES.tsv"), &modes).expect("aggregate modes");
    fs::set_permissions(root.join("MODES.tsv"), fs::Permissions::from_mode(0o400))
        .expect("aggregate modes mode");
    assert_eq!(
        aggregate_modes_bytes(&root).expect("stable aggregate modes"),
        modes
    );

    let manifest_rows = [
        "MODES.tsv",
        "aggregate-build-record.json",
        "build-spec.json",
        "qualification-packet.json",
    ]
    .into_iter()
    .map(|relative| {
        let bytes = fs::read(root.join(relative)).expect("aggregate sealed file");
        format!("{}  ./{relative}\n", sha256(&bytes))
    })
    .collect::<Vec<_>>();
    let manifest_bytes = manifest_rows.concat().into_bytes();
    fs::write(root.join("SHA256SUMS"), &manifest_bytes).expect("aggregate hash manifest");
    fs::set_permissions(root.join("SHA256SUMS"), fs::Permissions::from_mode(0o400))
        .expect("aggregate hash manifest mode");

    let manifest_sha256 = sha256(&manifest_bytes);
    let manifest = VerifiedManifest::load(&root, &manifest_sha256, 4)
        .expect("aggregate hash and exact inventory");
    let binding = ManifestLayerBindingV3 {
        layer_id: ManifestLayerIdV3::Outer,
        manifest_entry_count: 4,
        manifest_relative_path: "SHA256SUMS".to_string(),
        manifest_root_kind: ManifestRootKindV3::Sha256ManifestFullInventoryV1,
        manifest_sha256,
        mode_manifest: ModeManifestBindingV3 {
            format: ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
            relative_path: "MODES.tsv".to_string(),
            sha256: sha256(&modes),
        },
        root_relative_path: ".".to_string(),
    };
    verify_mode_manifest(
        &binding,
        &manifest,
        &["MODES.tsv".to_string()].into_iter().collect(),
    )
    .expect("aggregate root, manifest, modes, sizes, and self entry close exactly");
}

#[test]
fn output_names_reject_staging_and_traversal() {
    assert!(validate_output_relative_name("aggregate-deadbeef").is_ok());
    assert!(validate_output_relative_name(".incoming-aggregate-deadbeef").is_err());
    assert!(validate_output_relative_name("../aggregate").is_err());
    assert!(validate_output_relative_name("a/b").is_err());
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn exclusive_publish_never_replaces_preexisting_destination() {
    let temporary = private_tempdir();
    let source = temporary.path().join(".incoming-synthetic");
    let destination = temporary.path().join("synthetic");
    fs::create_dir(&source).expect("source");
    fs::create_dir(&destination).expect("destination");
    private_dir(&source);
    private_dir(&destination);
    assert!(publish_exclusive(&source, &destination).is_err());
    assert!(source.is_dir());
    assert!(destination.is_dir());
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn concurrent_exclusive_publish_has_exactly_one_winner() {
    use std::sync::Arc;
    use std::sync::Barrier;

    let temporary = private_tempdir();
    let first = temporary.path().join(".incoming-first");
    let second = temporary.path().join(".incoming-second");
    let destination = temporary.path().join("aggregate");
    for source in [&first, &second] {
        fs::create_dir(source).expect("source");
        private_dir(source);
    }
    let barrier = Arc::new(Barrier::new(3));
    let contender = |source: std::path::PathBuf| {
        let barrier = Arc::clone(&barrier);
        let destination = destination.clone();
        std::thread::spawn(move || {
            barrier.wait();
            publish_exclusive(&source, &destination).is_ok()
        })
    };
    let left = contender(first);
    let right = contender(second);
    barrier.wait();
    let left = left.join().expect("first contender");
    let right = right.join().expect("second contender");
    assert_ne!(left, right);
    assert!(destination.is_dir());
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn crash_before_publish_leaves_only_unaccepted_incoming_tree() {
    let temporary = private_tempdir();
    let incoming = temporary.path().join(".incoming-aggregate-deadbeef");
    let final_root = temporary.path().join("aggregate-deadbeef");
    fs::create_dir(&incoming).expect("incoming");
    private_dir(&incoming);
    fs::write(incoming.join("partial"), b"crash residue").expect("partial output");
    assert!(!final_root.exists());
    assert!(validate_output_relative_name(".incoming-aggregate-deadbeef").is_err());
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn revision_7_builder_plan_and_execute_fail_closed_on_blockers() {
    let temporary = private_tempdir();
    let receipts = temporary.path().join("receipts");
    fs::create_dir(&receipts).expect("receipts");
    private_dir(&receipts);
    let receipts = receipts.canonicalize().expect("canonical receipts");
    let candidate = exact_candidate();
    let spec = builder_fixture_spec(&candidate);
    let spec_bytes = canonical_json(&spec).expect("canonical spec");
    let spec_path = temporary.path().join("build-spec.json");
    write_fixture_file(&spec_path, &spec_bytes);
    let spec_sha256 = sha256(&spec_bytes);
    let output = receipts.join("synthetic-aggregate-deadbeef");

    let plan = plan_for_test(
        &spec_path,
        &spec_sha256,
        &output,
        &receipts,
        &candidate,
        "synthetic-aggregate",
        builder_fixture_validator,
    )
    .expect("read-only plan");
    assert!(plan.execute_required);
    assert!(!plan.ready_for_challenge);
    assert_eq!(plan.blockers, vec!["GITHUB_ACTIONS_PROFILE_UNPINNED"]);
    assert_eq!(plan.would_create_files.len(), 5);
    assert!(!output.exists());
    assert!(
        !receipts
            .join(".incoming-synthetic-aggregate-deadbeef")
            .exists()
    );

    assert!(
        plan_for_test(
            &spec_path,
            &"0".repeat(64),
            &output,
            &receipts,
            &candidate,
            "synthetic-aggregate",
            builder_fixture_validator,
        )
        .is_err()
    );
    assert!(!output.exists());

    let error = build_for_test(
        &spec_path,
        &spec_sha256,
        &output,
        &receipts,
        &candidate,
        "synthetic-aggregate",
        builder_fixture_validator,
        || Ok(()),
    )
    .expect_err("formal aggregate build must reject any blocker");
    assert!(
        error
            .to_string()
            .contains("GITHUB_ACTIONS_PROFILE_UNPINNED")
    );
    assert!(!output.exists());
    assert!(
        !receipts
            .join(".incoming-synthetic-aggregate-deadbeef")
            .exists()
    );
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn revision_7_builder_rejects_source_drift_before_publication() {
    use std::os::unix::fs::PermissionsExt;

    let temporary = private_tempdir();
    let receipts = temporary.path().join("receipts");
    fs::create_dir(&receipts).expect("receipts");
    private_dir(&receipts);
    let receipts = receipts.canonicalize().expect("canonical receipts");
    let candidate = exact_candidate();
    let spec = builder_fixture_spec(&candidate);
    let spec_bytes = canonical_json(&spec).expect("canonical spec");
    let spec_path = temporary.path().join("build-spec.json");
    write_fixture_file(&spec_path, &spec_bytes);
    let spec_sha256 = sha256(&spec_bytes);
    let output = receipts.join("synthetic-aggregate-cafebabe");
    let drift_path = spec_path.clone();

    let error = build_for_test(
        &spec_path,
        &spec_sha256,
        &output,
        &receipts,
        &candidate,
        "synthetic-aggregate",
        builder_ready_fixture_validator,
        move || {
            fs::set_permissions(&drift_path, fs::Permissions::from_mode(0o600))?;
            fs::write(&drift_path, b"{}\n")?;
            Ok(())
        },
    )
    .expect_err("staged aggregate must not publish after source spec drift");
    assert!(error.to_string().contains("build spec changed"));
    assert!(!output.exists());
    let incoming = receipts.join(".incoming-synthetic-aggregate-cafebabe");
    assert!(incoming.is_dir());
    assert!(
        validate_output_relative_name(incoming.file_name().unwrap().to_str().unwrap()).is_err()
    );
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn revision_7_ready_builder_is_one_shot_and_tamper_evident() {
    use std::os::unix::fs::PermissionsExt;

    let temporary = private_tempdir();
    let receipts = temporary.path().join("receipts");
    fs::create_dir(&receipts).expect("receipts");
    private_dir(&receipts);
    let receipts = receipts.canonicalize().expect("canonical receipts");
    let candidate = exact_candidate();
    let spec = builder_fixture_spec(&candidate);
    let spec_bytes = canonical_json(&spec).expect("canonical spec");
    let spec_path = temporary.path().join("build-spec.json");
    write_fixture_file(&spec_path, &spec_bytes);
    let spec_sha256 = sha256(&spec_bytes);
    let output = receipts.join("synthetic-aggregate-feedface");
    let sealed = build_for_test(
        &spec_path,
        &spec_sha256,
        &output,
        &receipts,
        &candidate,
        "synthetic-aggregate",
        builder_ready_fixture_validator,
        || Ok(()),
    )
    .expect("ready one-shot build");
    assert!(sealed.assessment.ready_for_challenge);
    assert!(sealed.assessment.blockers.is_empty());
    verify_for_test(
        &output,
        &sealed.aggregate_manifest_sha256,
        &receipts,
        &candidate,
        "synthetic-aggregate",
        builder_ready_fixture_validator,
    )
    .expect("verify ready sealed aggregate");
    assert!(
        build_for_test(
            &spec_path,
            &spec_sha256,
            &output,
            &receipts,
            &candidate,
            "synthetic-aggregate",
            builder_ready_fixture_validator,
            || Ok(()),
        )
        .is_err()
    );
    fs::set_permissions(
        output.join("qualification-packet.json"),
        fs::Permissions::from_mode(0o600),
    )
    .expect("tamper aggregate mode");
    assert!(
        verify_for_test(
            &output,
            &sealed.aggregate_manifest_sha256,
            &receipts,
            &candidate,
            "synthetic-aggregate",
            builder_ready_fixture_validator,
        )
        .is_err()
    );
}

#[test]
fn cli_mutator_requires_exact_build_execute_shape() {
    let program = std::ffi::OsString::from("hepta-operator-acceptance-v3");
    assert!(run_cli_v3(vec![program.clone(), "build".into()]).is_err());
    assert!(
        run_cli_v3(vec![
            program.clone(),
            "build".into(),
            "spec".into(),
            "digest".into(),
            "output".into(),
        ])
        .is_err()
    );
    assert!(
        run_cli_v3(vec![
            program,
            "build".into(),
            "--execute".into(),
            "spec".into(),
            "digest".into(),
            "output".into(),
            "extra".into(),
        ])
        .is_err()
    );
}

#[test]
fn model_rejects_unknown_selector_like_fields() {
    let candidate = exact_candidate();
    let mut value = json!({
        "automatic_transition": false,
        "authority": AuthorityBoundary::all_closed(),
        "candidate": candidate,
        "platform_gates": [],
        "platform_policy": exact_platform_policy(),
        "prerequisite_receipts": [],
        "profile_set": profiles::PROFILE_SET,
        "schema": BUILD_SPEC_SCHEMA,
        "schema_version": 3
    });
    value["semantic_claims"] = json!([{"selector":"/pass"}]);
    assert!(serde_json::from_value::<AggregateBuildSpecV3>(value).is_err());

    let receipt = json!({
        "manifest_layers": [],
        "profile": "portable_inputs_v1",
        "receipt_root": "/tmp/receipt",
        "semantic_claims": [{"claim":"pass","selector":"/shadow_pass"}]
    });
    assert!(serde_json::from_value::<ReceiptEvidenceBindingV3>(receipt).is_err());
}

fn builder_fixture_spec(candidate: &super::model::CandidateBindingV3) -> AggregateBuildSpecV3 {
    AggregateBuildSpecV3 {
        automatic_transition: false,
        authority: AuthorityBoundary::all_closed(),
        candidate: candidate.clone(),
        platform_gates: Vec::new(),
        platform_policy: exact_platform_policy(),
        prerequisite_receipts: Vec::new(),
        profile_set: profiles::PROFILE_SET.to_string(),
        schema: BUILD_SPEC_SCHEMA.to_string(),
        schema_version: 3,
    }
}

fn builder_fixture_validator(
    spec: &AggregateBuildSpecV3,
    policy: ValidationPolicy<'_>,
) -> Result<AggregateQualificationPacketV3, crate::AcceptanceError> {
    if spec != &builder_fixture_spec(policy.expected_candidate) {
        return Err(crate::AcceptanceError::Invalid(
            "synthetic builder spec differs".to_string(),
        ));
    }
    Ok(AggregateQualificationPacketV3 {
        automatic_transition: false,
        authority: AuthorityBoundary::all_closed(),
        candidate: policy.expected_candidate.clone(),
        decision: super::model::QualificationDecisionV3 {
            blockers: vec!["GITHUB_ACTIONS_PROFILE_UNPINNED".to_string()],
            complete_gate_count: 4,
            pass_gate_count: 4,
            prerequisite_pass_count: 3,
            verdict: "NO_GO".to_string(),
        },
        platform_policy: exact_platform_policy(),
        platform_receipts: Vec::new(),
        prerequisite_receipts: Vec::new(),
        profile_set: profiles::PROFILE_SET.to_string(),
        schema: super::evidence::PACKET_SCHEMA.to_string(),
        schema_version: 3,
    })
}

fn builder_ready_fixture_validator(
    spec: &AggregateBuildSpecV3,
    policy: ValidationPolicy<'_>,
) -> Result<AggregateQualificationPacketV3, crate::AcceptanceError> {
    let mut packet = builder_fixture_validator(spec, policy)?;
    packet.decision.blockers.clear();
    packet.decision.verdict = "GO".to_string();
    Ok(packet)
}

fn gate(gate: &str, profile: EvidenceProfileV3) -> PlatformGateInputV3 {
    PlatformGateInputV3 {
        gate: gate.to_string(),
        profile,
        receipt: None,
        required: true,
    }
}

fn prerequisite(
    id: &str,
    profile: EvidenceProfileV3,
    receipt: ReceiptEvidenceBindingV3,
) -> PrerequisiteInputV3 {
    PrerequisiteInputV3 {
        id: id.to_string(),
        profile,
        receipt,
        required: true,
    }
}

#[cfg(unix)]
fn synthetic_mac_receipt(
    parent: &Path,
    candidate: &super::model::CandidateBindingV3,
) -> ReceiptEvidenceBindingV3 {
    let root = new_receipt_root(parent, "mac");
    let status = format!(
        "schema=hepta_vnext_main_mac_validation_v6\nstatus=pass\ncandidate_commit={}\ncandidate_tree={}\ncandidate_parent={}\nintegration_merge={}\nupstream_cutoff={}\nworktree_clean=true\nexact_phase_count=12\nexact_phases_all_pass=true\nproduction_state_snapshot=false\nproduction_canary=false\noperator_acceptance=false\ncandidate_operator_acceptance=false\ncross_platform_qualification=false\npromotion=false\nenforce=false\noutbound=false\nretirement=false\nautomatic_transition=false\ndefault_branch_changed=false\nproduction_cutover=false\n",
        candidate.head,
        candidate.tree,
        candidate.parents[0],
        candidate.integration_merge,
        candidate.upstream_cutoff,
    );
    write_fixture_file(&root.join("qualification-status.txt"), status.as_bytes());
    direct_binding(
        root,
        EvidenceProfileV3::MacExactV6,
        "SHA256SUMS",
        "MODES.tsv",
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    )
}

#[cfg(unix)]
fn synthetic_linux_receipt(
    parent: &Path,
    candidate: &super::model::CandidateBindingV3,
) -> ReceiptEvidenceBindingV3 {
    let root = new_receipt_root(parent, "linux");
    let inner = root.join("receipt");
    create_private_dir(&inner);
    let result = format!(
        "schema=hepta_vnext_linux_exact_result_v3\nverdict=PASS\nqualification=true\ncandidate_pass=true\ncandidate_fail=false\nharness_blocked=false\nharness_fail=false\nharness_preflight_pass=true\ncandidate_execution_started=true\ncandidate_execution_completed=true\npostflight_verified=true\nsource_identity=match\nworktree_clean=true\ncandidate_head={}\ncandidate_tree={}\ncandidate_parent={}\nupstream_cutoff={}\nproduction_changed=false\nrefs_changed=false\ndata_deleted=false\npromotion_authority=false\n",
        candidate.head, candidate.tree, candidate.parents[0], candidate.upstream_cutoff,
    );
    write_fixture_file(&inner.join("result.txt"), result.as_bytes());
    write_fixture_file(
        &inner.join("steps.tsv"),
        step_rows(&profiles::LINUX_STEPS, &[0; 43]).as_bytes(),
    );
    let inner_layer = seal_fixture_layer(
        &inner,
        ManifestLayerIdV3::InnerReceipt,
        "receipt",
        "SHA256SUMS",
        "MODES.tsv",
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    );
    let outer = nested_outer_status("hepta_vnext_linux_local_verification_v3", candidate, true);
    write_fixture_file(&root.join("LOCAL-VERIFICATION.txt"), outer.as_bytes());
    let outer_layer = seal_fixture_layer(
        &root,
        ManifestLayerIdV3::Outer,
        ".",
        "OUTER-SHA256SUMS",
        "OUTER-MODES.tsv",
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    );
    ReceiptEvidenceBindingV3 {
        manifest_layers: vec![outer_layer, inner_layer],
        profile: EvidenceProfileV3::LinuxExactV5,
        provenance: ReceiptProvenanceV3::Direct,
        receipt_root: path_text(&root),
        required_artifacts: Vec::new(),
    }
}

#[cfg(unix)]
fn synthetic_nix_receipt(
    parent: &Path,
    candidate: &super::model::CandidateBindingV3,
) -> ReceiptEvidenceBindingV3 {
    let root = new_receipt_root(parent, "nix");
    let inner = root.join("receipt");
    create_private_dir(&inner);
    let result = format!(
        "schema=hepta_vnext_nix_exact_v3_result_v1\nstatus=PASS\nverdict=PASS\nqualification=true\ncandidate_pass=true\ncandidate_fail=false\nharness_fail=false\ninterrupted=false\ncandidate_execution_started=true\ncandidate_execution_completed=true\nsource_postflight_verified=true\nresource_monitor_verified=true\npass_evidence_verified=true\nresource_binding_verified=true\nprobe_verified=true\ncandidate_head={}\ncandidate_tree={}\ncandidate_parent={}\nupstream_cutoff={}\nproduction_changed=false\nrefs_changed=false\ndata_deleted=false\npromotion_authority=false\n",
        candidate.head, candidate.tree, candidate.parents[0], candidate.upstream_cutoff,
    );
    write_fixture_file(&inner.join("result.txt"), result.as_bytes());
    write_fixture_file(
        &inner.join("steps.tsv"),
        step_rows(&profiles::NIX_STEPS, &[0; 5]).as_bytes(),
    );
    let inner_layer = seal_fixture_layer(
        &inner,
        ManifestLayerIdV3::InnerReceipt,
        "receipt",
        "SHA256SUMS",
        "MODES.tsv",
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    );
    let outer = nested_outer_status(
        "hepta_vnext_nix_exact_v3_local_verification_v1",
        candidate,
        false,
    );
    write_fixture_file(&root.join("LOCAL-VERIFICATION.txt"), outer.as_bytes());
    let outer_layer = seal_fixture_layer(
        &root,
        ManifestLayerIdV3::Outer,
        ".",
        "OUTER-SHA256SUMS",
        "OUTER-MODES.tsv",
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    );
    ReceiptEvidenceBindingV3 {
        manifest_layers: vec![outer_layer, inner_layer],
        profile: EvidenceProfileV3::NixExactV3,
        provenance: ReceiptProvenanceV3::Direct,
        receipt_root: path_text(&root),
        required_artifacts: Vec::new(),
    }
}

#[cfg(unix)]
fn synthetic_windows_receipt(
    parent: &Path,
    candidate: &super::model::CandidateBindingV3,
) -> ReceiptEvidenceBindingV3 {
    let root = new_receipt_root(parent, "windows");
    let inner = root.join("receipt");
    create_private_dir(&inner);
    let step_results = profiles::WINDOWS_STEPS
        .iter()
        .map(|name| ((*name).to_string(), json!(0)))
        .collect::<serde_json::Map<_, _>>();
    let mut result = serde_json::to_vec(&json!({
        "schema": "hepta_vnext_windows_native_qualification_v4",
        "status": "PASS",
        "verdict": "PASS",
        "qualification": true,
        "candidate_pass": true,
        "candidate_fail": false,
        "harness_fail": false,
        "harness_preflight_pass": true,
        "candidate_execution_started": true,
        "candidate_execution_completed": true,
        "postflight_verified": true,
        "candidate_head": candidate.head,
        "candidate_tree": candidate.tree,
        "candidate_parent": candidate.parents[0],
        "upstream_cutoff": candidate.upstream_cutoff,
        "source_identity": "match",
        "worktree_clean": true,
        "ordered_step_count": 5,
        "step_results": step_results,
        "production_changed": false,
        "refs_changed": false,
        "github_actions_runner_active_state_verified": true,
        "github_actions_runner_registration_state_verified": true
    }))
    .expect("Windows result JSON");
    result.push(b'\n');
    write_fixture_file(&inner.join("result.json"), &result);
    write_fixture_file(
        &inner.join("steps.tsv"),
        windows_step_rows(&profiles::WINDOWS_STEPS, &[0; 5], "candidate").as_bytes(),
    );
    for step in profiles::WINDOWS_STEPS {
        write_fixture_file(&inner.join(format!("{step}.log")), b"synthetic step log\n");
    }
    let inner_layer = seal_fixture_layer(
        &inner,
        ManifestLayerIdV3::InnerReceipt,
        "receipt",
        "SHA256SUMS",
        "FILES.tsv",
        ModeManifestFormatV3::WindowsNtfsTypeSizePathTsvV1,
    );
    let outer = format!(
        "schema=hepta_vnext_windows_native_outer_verification_v3\nstatus=PASS\ncandidate_head={}\nguest_receipt_status=PASS\nguest_receipt_sha256sums=PASS\nguest_receipt_exact_file_set=PASS\nguest_driver_payload_manifest=PASS\nguest_result_classification=PASS\ncandidate_execution_marker_consistent=PASS\nx230_vm_domain_uuid_state_evidence=PASS\nx230_original_domain_state_autostart_evidence=PASS\nx230_recovery_interface_mac_ipv4_evidence=PASS\nincoming_lstat_type_scan=PASS\ncopied_regular_file_set_exact_coverage=PASS\nouter_attempt_one_shot=true\ngithub_actions_runner_active_state_verified=true\ngithub_actions_runner_registration_state_verified=true\nproduction_changed=false\nrefs_changed=false\n",
        candidate.head,
    );
    write_fixture_file(&root.join("verification-result.txt"), outer.as_bytes());
    let outer_layer = seal_fixture_layer(
        &root,
        ManifestLayerIdV3::Outer,
        ".",
        "ATTEMPT.sha256",
        "MODES.tsv",
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    );
    ReceiptEvidenceBindingV3 {
        manifest_layers: vec![outer_layer, inner_layer],
        profile: EvidenceProfileV3::WindowsNativeV6,
        provenance: ReceiptProvenanceV3::Direct,
        receipt_root: path_text(&root),
        required_artifacts: Vec::new(),
    }
}

#[cfg(unix)]
fn synthetic_portable_receipt(
    parent: &Path,
    candidate: &super::model::CandidateBindingV3,
    bundle_bytes: &[u8],
) -> ReceiptEvidenceBindingV3 {
    let root = new_receipt_root(parent, "portable");
    write_fixture_file(&root.join(&candidate.bundle.relative_path), bundle_bytes);
    let complete = format!(
        "schema=hepta_vnext_portable_generation_v1\nstatus=pass\ncandidate_head={}\ncandidate_tree={}\ncandidate_parent={}\nsource_worktree_clean=true\ncandidate_fail=false\nrefs_changed=false\nproduction_changed=false\n",
        candidate.head, candidate.tree, candidate.parents[0],
    );
    write_fixture_file(&root.join("GENERATION-COMPLETE.txt"), complete.as_bytes());
    let binding = format!(
        "schema=hepta_vnext_portable_exact_inputs_v1\ncandidate_head={}\ncandidate_tree={}\ncandidate_parent={}\nintegration_merge={}\nupstream_cutoff={}\nsource_worktree_clean=true\nrefs_changed=false\nproduction_changed=false\n",
        candidate.head,
        candidate.tree,
        candidate.parents[0],
        candidate.integration_merge,
        candidate.upstream_cutoff,
    );
    write_fixture_file(&root.join("candidate-binding.txt"), binding.as_bytes());
    direct_binding(
        root,
        EvidenceProfileV3::PortableInputsV1,
        "INPUTS.sha256",
        "MODES.tsv",
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    )
}

#[cfg(unix)]
fn synthetic_path_trust_receipt(
    parent: &Path,
    candidate: &super::model::CandidateBindingV3,
) -> ReceiptEvidenceBindingV3 {
    let root = new_receipt_root(parent, "path-trust");
    let status = format!(
        "schema=hepta_vnext_canonical_path_trust_v2\nstatus=pass\ncandidate_head={}\ncandidate_tree={}\ncandidate_parent={}\nworktree_clean=true\ndefault_main_changed=false\nproduction_changed=false\n",
        candidate.head, candidate.tree, candidate.parents[0],
    );
    write_fixture_file(&root.join("status.txt"), status.as_bytes());
    direct_binding(
        root,
        EvidenceProfileV3::CanonicalPathTrustV2,
        "SHA256SUMS",
        "MODES.tsv",
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    )
}

#[cfg(unix)]
fn synthetic_upstream_receipt(
    parent: &Path,
    candidate: &super::model::CandidateBindingV3,
) -> ReceiptEvidenceBindingV3 {
    let root = new_receipt_root(parent, "upstream");
    let status = format!(
        "schema=hepta_vnext_upstream_cutoff_observation_v1\ncandidate_head={}\ncandidate_tree={}\nfrozen_upstream_cutoff={}\ncandidate_changed=false\nqualification_invalidated=false\n",
        candidate.head, candidate.tree, candidate.upstream_cutoff,
    );
    write_fixture_file(&root.join("upstream-cutoff.txt"), status.as_bytes());
    direct_binding(
        root,
        EvidenceProfileV3::UpstreamCutoffObservationV1,
        "SHA256SUMS",
        "MODES.tsv",
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    )
}

#[cfg(unix)]
fn nested_outer_status(
    schema: &str,
    candidate: &super::model::CandidateBindingV3,
    tool_binding: bool,
) -> String {
    format!(
        "schema={schema}\nstatus=PASS\ncandidate_head={}\ncandidate_tree={}\ncandidate_parent={}\nupstream_cutoff={}\ninner_recursive_hashes=pass\ninner_recursive_modes=pass\ninner_manifest_coverage=pass\ncandidate_binding=pass\n{}remote_roots_preserved=true\npromotion_authority=false\nproduction_changed=false\nrefs_changed=false\n",
        candidate.head,
        candidate.tree,
        candidate.parents[0],
        candidate.upstream_cutoff,
        if tool_binding {
            "tool_binding=pass\n"
        } else {
            ""
        },
    )
}

#[cfg(unix)]
fn direct_binding(
    root: std::path::PathBuf,
    profile: EvidenceProfileV3,
    manifest_name: &str,
    mode_name: &str,
    mode_format: ModeManifestFormatV3,
) -> ReceiptEvidenceBindingV3 {
    let layer = seal_fixture_layer(
        &root,
        ManifestLayerIdV3::Outer,
        ".",
        manifest_name,
        mode_name,
        mode_format,
    );
    ReceiptEvidenceBindingV3 {
        manifest_layers: vec![layer],
        profile,
        provenance: ReceiptProvenanceV3::Direct,
        receipt_root: path_text(&root),
        required_artifacts: Vec::new(),
    }
}

#[cfg(unix)]
fn artifact_bindings(root: &Path, profile: EvidenceProfileV3) -> Vec<ArtifactBindingV3> {
    profiles::required_artifacts(profile)
        .iter()
        .map(|artifact| {
            let layer = profiles::layer_profile(profile, artifact.layer)
                .expect("required artifact layer profile");
            let layer_root = if layer.root == "." {
                root.to_path_buf()
            } else {
                root.join(layer.root)
            };
            let bytes = fs::read(layer_root.join(artifact.path)).expect("required artifact bytes");
            ArtifactBindingV3 {
                layer_id: artifact.layer,
                relative_path: artifact.path.to_string(),
                sha256: sha256(&bytes),
                size_bytes: bytes.len() as u64,
            }
        })
        .collect()
}

#[cfg(unix)]
fn read_bound_artifact(root: &Path, relative: &str) -> ArtifactBindingV3 {
    let bytes = fs::read(root.join(relative)).expect("bound artifact bytes");
    ArtifactBindingV3 {
        layer_id: ManifestLayerIdV3::Outer,
        relative_path: relative.to_string(),
        sha256: sha256(&bytes),
        size_bytes: bytes.len() as u64,
    }
}

#[cfg(unix)]
fn write_bound_artifact(root: &Path, relative: &str, bytes: &[u8]) -> ArtifactBindingV3 {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("artifact parent");
        private_dir(parent);
    }
    write_fixture_file(&path, bytes);
    ArtifactBindingV3 {
        layer_id: ManifestLayerIdV3::Outer,
        relative_path: relative.to_string(),
        sha256: sha256(bytes),
        size_bytes: bytes.len() as u64,
    }
}

#[cfg(unix)]
struct SyntheticReemittedWrapperFixture {
    _temporary: TempDir,
    binding: ReceiptEvidenceBindingV3,
    frozen: profiles::FrozenOriginalIdentityV3,
    original_inventory_sha256: String,
    original_root: std::path::PathBuf,
    receipts: std::path::PathBuf,
    wrapper_root: std::path::PathBuf,
}

#[cfg(unix)]
fn synthetic_reemitted_wrapper_fixture() -> SyntheticReemittedWrapperFixture {
    use std::os::unix::fs::PermissionsExt;

    let temporary = private_tempdir();
    let receipts = temporary.path().join("receipts");
    fs::create_dir(&receipts).expect("synthetic receipts");
    private_dir(&receipts);
    let receipts = receipts
        .canonicalize()
        .expect("canonical synthetic receipts");
    let original_root = new_receipt_root(&receipts, "original");
    write_fixture_file(&original_root.join("artifact.txt"), b"original authority\n");
    let nested = original_root.join("nested");
    fs::create_dir(&nested).expect("synthetic original nested directory");
    private_dir(&nested);
    write_fixture_file(&nested.join("data.txt"), b"archived data\n");
    let manifest_bytes = [
        format!("{}  artifact.txt\n", sha256(b"original authority\n")),
        format!("{}  nested/data.txt\n", sha256(b"archived data\n")),
    ]
    .concat()
    .into_bytes();
    write_fixture_file(&original_root.join("INPUTS.sha256"), &manifest_bytes);
    let manifest_sha256 = sha256(&manifest_bytes);
    let original = load_legacy_manifest_with_policy(
        &original_root,
        "INPUTS.sha256",
        &manifest_sha256,
        2,
        LegacyExtendedMetadataPolicy::PortableInputs,
    )
    .expect("compact original provenance receipt");
    let inventory_identity = |bytes: &[u8]| profiles::FrozenInventoryIdentityV3 {
        row_count: bytes.iter().filter(|byte| **byte == b'\n').count(),
        sha256: sha256(bytes).leak(),
        size_bytes: bytes.len(),
    };
    let frozen = profiles::FrozenOriginalIdentityV3 {
        entry_count: 2,
        extended_metadata_inventory: inventory_identity(&original.extended_metadata_inventory),
        hardlink_topology: inventory_identity(&original.hardlink_topology),
        manifest_relative_path: "INPUTS.sha256",
        manifest_sha256: manifest_sha256.leak(),
        metadata_inventory: inventory_identity(&original.metadata_inventory),
        receipt_root: path_text(&original_root).leak(),
    };

    let wrapper_root = new_receipt_root(&receipts, "wrapper");
    let original_tree = wrapper_root.join("provenance/original-tree");
    fs::copy(
        original_root.join("artifact.txt"),
        wrapper_root.join("artifact.txt"),
    )
    .expect("compact canonical projection");
    fs::set_permissions(
        wrapper_root.join("artifact.txt"),
        fs::Permissions::from_mode(0o400),
    )
    .expect("compact canonical projection mode");
    for (relative, entry) in &original.entries {
        let destination = original_tree.join(relative);
        fs::create_dir_all(destination.parent().expect("archive parent"))
            .expect("compact archive directories");
        private_dir(destination.parent().expect("archive parent"));
        fs::copy(original_root.join(relative), &destination).expect("compact archive copy");
        fs::set_permissions(&destination, fs::Permissions::from_mode(0o400))
            .expect("compact archive mode");
        assert_eq!(
            sha256(&fs::read(&destination).expect("compact archive bytes")),
            entry.sha256
        );
    }
    private_dir(&wrapper_root.join("provenance"));
    let topology = write_bound_artifact(
        &wrapper_root,
        "provenance/hardlink-topology.tsv",
        &original.hardlink_topology,
    );
    let metadata = write_bound_artifact(
        &wrapper_root,
        "provenance/original-metadata.tsv",
        &original.metadata_inventory,
    );
    let extended = write_bound_artifact(
        &wrapper_root,
        "provenance/original-extended-metadata.tsv",
        &original.extended_metadata_inventory,
    );
    let reemitter = write_bound_artifact(
        &wrapper_root,
        "provenance/reemitter",
        b"compact test reemitter\n",
    );
    let mut rows = original
        .entries
        .iter()
        .map(|(relative, entry)| {
            format!(
                "archive\t./{relative}\t{}\t{}\t./provenance/original-tree/{relative}\n",
                entry.sha256, entry.size_bytes
            )
        })
        .collect::<Vec<_>>();
    let canonical = original
        .entries
        .get("artifact.txt")
        .expect("compact canonical original");
    rows.push(format!(
        "canonical\t./artifact.txt\t{}\t{}\t./artifact.txt\n",
        canonical.sha256, canonical.size_bytes
    ));
    rows.sort();
    let projection = write_bound_artifact(
        &wrapper_root,
        "provenance/projection-map.tsv",
        rows.concat().as_bytes(),
    );
    let original_binding = OriginalReceiptBindingV3 {
        manifest_entry_count: frozen.entry_count,
        manifest_relative_path: frozen.manifest_relative_path.to_string(),
        manifest_sha256: frozen.manifest_sha256.to_string(),
        receipt_root: frozen.receipt_root.to_string(),
    };
    let original_inventory_sha256 = sha256(&original.inventory);
    let attestation_bytes = wrapper_attestation_bytes_for_test(
        &original_binding,
        &original_inventory_sha256,
        &topology,
        &extended,
        &metadata,
        &projection,
        &reemitter,
    )
    .expect("compact wrapper attestation");
    let attestation = write_bound_artifact(
        &wrapper_root,
        "provenance/reemission-attestation.json",
        &attestation_bytes,
    );
    let layer = seal_fixture_layer(
        &wrapper_root,
        ManifestLayerIdV3::Outer,
        ".",
        "INPUTS.sha256",
        "MODES.tsv",
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    );
    let binding = ReceiptEvidenceBindingV3 {
        manifest_layers: vec![layer],
        profile: EvidenceProfileV3::PortableInputsV1,
        provenance: ReceiptProvenanceV3::ReemittedWrapper {
            attestation,
            hardlink_topology: topology,
            original: original_binding,
            original_extended_metadata_inventory: extended,
            original_metadata_inventory: metadata,
            original_tree_relative_path: "provenance/original-tree".to_string(),
            projection_map: projection,
            reemitter,
        },
        receipt_root: path_text(&wrapper_root),
        required_artifacts: Vec::new(),
    };
    SyntheticReemittedWrapperFixture {
        _temporary: temporary,
        binding,
        frozen,
        original_inventory_sha256,
        original_root,
        receipts,
        wrapper_root,
    }
}

#[cfg(unix)]
fn validate_synthetic_reemitted_wrapper(
    fixture: &SyntheticReemittedWrapperFixture,
) -> Result<(), crate::AcceptanceError> {
    validate_reemitted_wrapper_for_test(&fixture.binding, &fixture.receipts, fixture.frozen, || {
        Ok(())
    })
}

#[cfg(unix)]
fn rewrite_fixture_file(path: &Path, bytes: &[u8]) {
    use std::os::unix::fs::PermissionsExt;

    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).expect("writable fixture mode");
    fs::write(path, bytes).expect("rewrite fixture file");
    fs::set_permissions(path, fs::Permissions::from_mode(0o400)).expect("sealed fixture mode");
}

#[cfg(unix)]
fn rewrite_projection(
    fixture: &mut SyntheticReemittedWrapperFixture,
    mut transform: impl FnMut(&str) -> Option<String>,
) {
    let projection_path = fixture.wrapper_root.join("provenance/projection-map.tsv");
    let current = fs::read_to_string(&projection_path).expect("current projection map");
    let rewritten = current
        .lines()
        .filter_map(&mut transform)
        .map(|line| format!("{line}\n"))
        .collect::<String>();
    rewrite_provenance_artifact(
        fixture,
        "provenance/projection-map.tsv",
        rewritten.as_bytes(),
    );
}

#[cfg(unix)]
fn rewrite_provenance_artifact(
    fixture: &mut SyntheticReemittedWrapperFixture,
    relative: &str,
    bytes: &[u8],
) {
    let path = fixture.wrapper_root.join(relative);
    rewrite_fixture_file(&path, bytes);
    let replacement = ArtifactBindingV3 {
        layer_id: ManifestLayerIdV3::Outer,
        relative_path: relative.to_string(),
        sha256: sha256(bytes),
        size_bytes: bytes.len() as u64,
    };
    let ReceiptProvenanceV3::ReemittedWrapper {
        hardlink_topology,
        original_extended_metadata_inventory,
        original_metadata_inventory,
        projection_map,
        ..
    } = &mut fixture.binding.provenance
    else {
        panic!("synthetic fixture is not a wrapper")
    };
    match relative {
        "provenance/hardlink-topology.tsv" => *hardlink_topology = replacement,
        "provenance/original-extended-metadata.tsv" => {
            *original_extended_metadata_inventory = replacement;
        }
        "provenance/original-metadata.tsv" => *original_metadata_inventory = replacement,
        "provenance/projection-map.tsv" => *projection_map = replacement,
        _ => panic!("unsupported rewritten provenance artifact"),
    }
    refresh_synthetic_attestation(fixture);
}

#[cfg(unix)]
fn refresh_synthetic_attestation(fixture: &mut SyntheticReemittedWrapperFixture) {
    let ReceiptProvenanceV3::ReemittedWrapper {
        hardlink_topology,
        original,
        original_extended_metadata_inventory,
        original_metadata_inventory,
        projection_map,
        reemitter,
        ..
    } = &fixture.binding.provenance
    else {
        panic!("synthetic fixture is not a wrapper")
    };
    let bytes = wrapper_attestation_bytes_for_test(
        original,
        &fixture.original_inventory_sha256,
        hardlink_topology,
        original_extended_metadata_inventory,
        original_metadata_inventory,
        projection_map,
        reemitter,
    )
    .expect("refresh synthetic attestation");
    let relative = "provenance/reemission-attestation.json";
    rewrite_fixture_file(&fixture.wrapper_root.join(relative), &bytes);
    let replacement = ArtifactBindingV3 {
        layer_id: ManifestLayerIdV3::Outer,
        relative_path: relative.to_string(),
        sha256: sha256(&bytes),
        size_bytes: bytes.len() as u64,
    };
    let ReceiptProvenanceV3::ReemittedWrapper { attestation, .. } = &mut fixture.binding.provenance
    else {
        unreachable!()
    };
    *attestation = replacement;
}

#[cfg(unix)]
fn reseal_synthetic_wrapper(fixture: &mut SyntheticReemittedWrapperFixture) {
    fs::remove_file(fixture.wrapper_root.join("INPUTS.sha256"))
        .expect("remove prior wrapper manifest");
    fs::remove_file(fixture.wrapper_root.join("MODES.tsv")).expect("remove prior wrapper modes");
    fixture.binding.manifest_layers = vec![seal_fixture_layer(
        &fixture.wrapper_root,
        ManifestLayerIdV3::Outer,
        ".",
        "INPUTS.sha256",
        "MODES.tsv",
        ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
    )];
}

#[cfg(unix)]
fn frozen_direct_binding(profile: EvidenceProfileV3) -> ReceiptEvidenceBindingV3 {
    let identity = profiles::frozen_receipt_identity(profile).expect("frozen receipt identity");
    assert!(identity.inner.is_none());
    let root = Path::new(identity.receipt_root);
    let layer =
        profiles::layer_profile(profile, ManifestLayerIdV3::Outer).expect("compiled outer profile");
    ReceiptEvidenceBindingV3 {
        manifest_layers: vec![ManifestLayerBindingV3 {
            layer_id: ManifestLayerIdV3::Outer,
            manifest_entry_count: identity.outer.entry_count,
            manifest_relative_path: layer.manifest_path.to_string(),
            manifest_root_kind: ManifestRootKindV3::Sha256ManifestFullInventoryV1,
            manifest_sha256: identity.outer.manifest_sha256.to_string(),
            mode_manifest: ModeManifestBindingV3 {
                format: layer.mode_format,
                relative_path: layer.mode_path.to_string(),
                sha256: identity.outer.mode_sha256.to_string(),
            },
            root_relative_path: layer.root.to_string(),
        }],
        profile,
        provenance: ReceiptProvenanceV3::Direct,
        receipt_root: identity.receipt_root.to_string(),
        required_artifacts: artifact_bindings(root, profile),
    }
}

#[cfg(unix)]
fn frozen_reemitted_binding(profile: EvidenceProfileV3) -> ReceiptEvidenceBindingV3 {
    let identity = profiles::frozen_receipt_identity(profile).expect("frozen receipt identity");
    let original = profiles::frozen_original_identity(profile).expect("frozen original identity");
    let root = Path::new(identity.receipt_root);
    let outer =
        profiles::layer_profile(profile, ManifestLayerIdV3::Outer).expect("compiled outer profile");
    let bind_layer = |layer: profiles::LayerProfileV3, frozen: profiles::FrozenReceiptLayerV3| {
        ManifestLayerBindingV3 {
            layer_id: layer.id,
            manifest_entry_count: frozen.entry_count,
            manifest_relative_path: layer.manifest_path.to_string(),
            manifest_root_kind: ManifestRootKindV3::Sha256ManifestFullInventoryV1,
            manifest_sha256: frozen.manifest_sha256.to_string(),
            mode_manifest: ModeManifestBindingV3 {
                format: layer.mode_format,
                relative_path: layer.mode_path.to_string(),
                sha256: frozen.mode_sha256.to_string(),
            },
            root_relative_path: layer.root.to_string(),
        }
    };
    let mut manifest_layers = vec![bind_layer(outer, identity.outer)];
    if let Some(frozen_inner) = identity.inner {
        let inner = profiles::layer_profile(profile, ManifestLayerIdV3::InnerReceipt)
            .expect("compiled inner profile");
        manifest_layers.push(bind_layer(inner, frozen_inner));
    }
    ReceiptEvidenceBindingV3 {
        manifest_layers,
        profile,
        provenance: ReceiptProvenanceV3::ReemittedWrapper {
            attestation: read_bound_artifact(root, "provenance/reemission-attestation.json"),
            hardlink_topology: read_bound_artifact(root, "provenance/hardlink-topology.tsv"),
            original: OriginalReceiptBindingV3 {
                manifest_entry_count: original.entry_count,
                manifest_relative_path: original.manifest_relative_path.to_string(),
                manifest_sha256: original.manifest_sha256.to_string(),
                receipt_root: original.receipt_root.to_string(),
            },
            original_extended_metadata_inventory: read_bound_artifact(
                root,
                "provenance/original-extended-metadata.tsv",
            ),
            original_metadata_inventory: read_bound_artifact(
                root,
                "provenance/original-metadata.tsv",
            ),
            original_tree_relative_path: "provenance/original-tree".to_string(),
            projection_map: read_bound_artifact(root, "provenance/projection-map.tsv"),
            reemitter: read_bound_artifact(root, "provenance/reemitter"),
        },
        receipt_root: identity.receipt_root.to_string(),
        required_artifacts: artifact_bindings(root, profile),
    }
}

#[cfg(unix)]
fn new_receipt_root(parent: &Path, name: &str) -> std::path::PathBuf {
    let root = parent.join(name);
    create_private_dir(&root);
    root.canonicalize().expect("canonical receipt root")
}

#[cfg(unix)]
fn create_private_dir(path: &Path) {
    use std::os::unix::fs::PermissionsExt;

    fs::create_dir(path).expect("create private fixture directory");
    fs::set_permissions(path, fs::Permissions::from_mode(0o700))
        .expect("private fixture directory mode");
}

#[cfg(unix)]
fn write_fixture_file(path: &Path, bytes: &[u8]) {
    use std::os::unix::fs::PermissionsExt;

    fs::write(path, bytes).expect("write fixture file");
    fs::set_permissions(path, fs::Permissions::from_mode(0o400)).expect("fixture file mode");
}

#[cfg(unix)]
fn seal_fixture_layer(
    root: &Path,
    layer_id: ManifestLayerIdV3,
    root_relative_path: &str,
    manifest_name: &str,
    mode_name: &str,
    mode_format: ModeManifestFormatV3,
) -> ManifestLayerBindingV3 {
    let mut directories = Vec::new();
    let mut files = Vec::new();
    collect_fixture_paths(root, root, &mut directories, &mut files);
    assert!(
        !files
            .iter()
            .any(|path| path == manifest_name || path == mode_name)
    );
    files.push(mode_name.to_string());
    files.sort();
    directories.sort();

    let manifest_size = files
        .iter()
        .map(|path| 64 + 2 + path.len() + 1)
        .sum::<usize>();
    let mut mode_size = 0_usize;
    let mode_bytes = loop {
        let mut rows = Vec::new();
        for directory in &directories {
            let path = if directory.is_empty() {
                ".".to_string()
            } else {
                format!("./{directory}")
            };
            let row = match mode_format {
                ModeManifestFormatV3::TypedPosixModeSizePathTsvV2 => {
                    format!("Directory\t700\t-\t{path}\n")
                }
                ModeManifestFormatV3::WindowsNtfsTypeSizePathTsvV1 => {
                    format!("Directory\t-\t{path}\n")
                }
            };
            rows.push((directory.clone(), row));
        }
        let mut mode_files = files.clone();
        mode_files.push(manifest_name.to_string());
        for file in &mode_files {
            let size = if file == mode_name {
                mode_size as u64
            } else if file == manifest_name {
                manifest_size as u64
            } else {
                fs::metadata(root.join(file))
                    .expect("fixture metadata")
                    .len()
            };
            let row = match mode_format {
                ModeManifestFormatV3::TypedPosixModeSizePathTsvV2 => {
                    format!("Regular File\t400\t{size}\t./{file}\n")
                }
                ModeManifestFormatV3::WindowsNtfsTypeSizePathTsvV1 => {
                    format!("Regular File\t{size}\t./{file}\n")
                }
            };
            rows.push((file.clone(), row));
        }
        rows.sort_by(|left, right| left.0.cmp(&right.0));
        let bytes = rows
            .into_iter()
            .map(|(_, row)| row)
            .collect::<String>()
            .into_bytes();
        if bytes.len() == mode_size {
            break bytes;
        }
        mode_size = bytes.len();
    };
    write_fixture_file(&root.join(mode_name), &mode_bytes);

    let mut manifest_rows = Vec::new();
    for file in &files {
        let bytes = fs::read(root.join(file)).expect("fixture manifest input");
        manifest_rows.push(format!("{}  {file}\n", sha256(&bytes)));
    }
    let manifest_bytes = manifest_rows.concat().into_bytes();
    assert_eq!(manifest_bytes.len(), manifest_size);
    write_fixture_file(&root.join(manifest_name), &manifest_bytes);

    ManifestLayerBindingV3 {
        layer_id,
        manifest_entry_count: files.len(),
        manifest_relative_path: manifest_name.to_string(),
        manifest_root_kind: ManifestRootKindV3::Sha256ManifestFullInventoryV1,
        manifest_sha256: sha256(&manifest_bytes),
        mode_manifest: ModeManifestBindingV3 {
            format: mode_format,
            relative_path: mode_name.to_string(),
            sha256: sha256(&mode_bytes),
        },
        root_relative_path: root_relative_path.to_string(),
    }
}

#[cfg(unix)]
fn collect_fixture_paths(
    root: &Path,
    directory: &Path,
    directories: &mut Vec<String>,
    files: &mut Vec<String>,
) {
    directories.push(
        directory
            .strip_prefix(root)
            .expect("fixture directory under root")
            .to_str()
            .expect("UTF-8 fixture directory")
            .to_string(),
    );
    for entry in fs::read_dir(directory).expect("read fixture directory") {
        let path = entry.expect("fixture entry").path();
        if path.is_dir() {
            collect_fixture_paths(root, &path, directories, files);
        } else {
            files.push(
                path.strip_prefix(root)
                    .expect("fixture file under root")
                    .to_str()
                    .expect("UTF-8 fixture file")
                    .to_string(),
            );
        }
    }
}

#[cfg(unix)]
fn path_text(path: &Path) -> String {
    path.to_str().expect("UTF-8 fixture path").to_string()
}

fn private_tempdir() -> TempDir {
    let temporary = tempfile::tempdir().expect("tempdir");
    private_dir(temporary.path());
    temporary
}

fn private_dir(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("private dir");
    }
}

const TEST_TIMESTAMP: &str = "2026-08-13T08:00:00Z";

fn step_rows(names: &[&str], return_codes: &[u32]) -> String {
    names
        .iter()
        .zip(return_codes)
        .map(|(name, return_code)| {
            format!("{name}\t{return_code}\t{TEST_TIMESTAMP}\t{TEST_TIMESTAMP}\n")
        })
        .collect()
}

fn windows_step_rows(names: &[&str], return_codes: &[u32], domain: &str) -> String {
    names
        .iter()
        .zip(return_codes)
        .map(|(name, return_code)| {
            format!(
                "{name}\t{return_code}\t{domain}\t{TEST_TIMESTAMP}\t{TEST_TIMESTAMP}\t{name}.log\n"
            )
        })
        .collect()
}

#[cfg(unix)]
fn mode_fixture(
    include_nested_directory: bool,
    populate_nested_directory: bool,
    include_auxiliary_mode_evidence: bool,
) -> (TempDir, ManifestLayerBindingV3, VerifiedManifest) {
    use std::os::unix::fs::PermissionsExt;

    let temporary = private_tempdir();
    let root = temporary
        .path()
        .canonicalize()
        .expect("canonical mode root");
    let nested = root.join("nested");
    fs::create_dir(&nested).expect("nested directory");
    fs::set_permissions(&nested, fs::Permissions::from_mode(0o700)).expect("nested mode");
    fs::write(root.join("artifact"), b"sealed").expect("artifact");
    fs::set_permissions(root.join("artifact"), fs::Permissions::from_mode(0o400))
        .expect("artifact mode");
    if include_auxiliary_mode_evidence {
        fs::write(root.join("DRIVER-MODES.tsv"), b"driver").expect("driver mode evidence");
        fs::set_permissions(
            root.join("DRIVER-MODES.tsv"),
            fs::Permissions::from_mode(0o400),
        )
        .expect("driver mode evidence mode");
    }
    if populate_nested_directory {
        fs::write(nested.join("nested-artifact"), b"nested").expect("nested artifact");
        fs::set_permissions(
            nested.join("nested-artifact"),
            fs::Permissions::from_mode(0o400),
        )
        .expect("nested artifact mode");
    }

    let placeholder = "0".repeat(64);
    let mut manifest_rows = vec![
        format!("{placeholder}  MODES.tsv\n"),
        format!("{placeholder}  artifact\n"),
    ];
    if populate_nested_directory {
        manifest_rows.push(format!("{placeholder}  nested/nested-artifact\n"));
    }
    if include_auxiliary_mode_evidence {
        manifest_rows.push(format!("{placeholder}  DRIVER-MODES.tsv\n"));
    }
    manifest_rows.sort_by_key(|row| {
        row.split_once("  ")
            .expect("manifest fixture row")
            .1
            .to_string()
    });
    let manifest_size = manifest_rows.concat().len();
    let mut mode_size = 0_usize;
    let mode_bytes = loop {
        let mut rows = vec![
            "Directory\t700\t-\t.\n".to_string(),
            format!("Regular File\t400\t{mode_size}\t./MODES.tsv\n"),
            format!("Regular File\t400\t{manifest_size}\t./SHA256SUMS\n"),
            "Regular File\t400\t6\t./artifact\n".to_string(),
        ];
        if populate_nested_directory {
            rows.push("Regular File\t400\t6\t./nested/nested-artifact\n".to_string());
        }
        if include_auxiliary_mode_evidence {
            rows.push("Regular File\t400\t6\t./DRIVER-MODES.tsv\n".to_string());
        }
        if include_nested_directory {
            rows.push("Directory\t700\t-\t./nested\n".to_string());
        }
        rows.sort_by_key(|row| {
            row.rsplit('\t')
                .next()
                .expect("mode row path")
                .trim_end()
                .trim_start_matches("./")
                .to_string()
        });
        let bytes = rows.concat().into_bytes();
        if bytes.len() == mode_size {
            break bytes;
        }
        mode_size = bytes.len();
    };
    fs::write(root.join("MODES.tsv"), &mode_bytes).expect("mode manifest");
    fs::set_permissions(root.join("MODES.tsv"), fs::Permissions::from_mode(0o400))
        .expect("mode manifest mode");
    let mut manifest_rows = vec![
        format!("{}  MODES.tsv\n", sha256(&mode_bytes)),
        format!("{}  artifact\n", sha256(b"sealed")),
    ];
    if populate_nested_directory {
        manifest_rows.push(format!("{}  nested/nested-artifact\n", sha256(b"nested")));
    }
    if include_auxiliary_mode_evidence {
        manifest_rows.push(format!("{}  DRIVER-MODES.tsv\n", sha256(b"driver")));
    }
    manifest_rows.sort_by_key(|row| {
        row.split_once("  ")
            .expect("manifest fixture row")
            .1
            .to_string()
    });
    let manifest_bytes = manifest_rows.concat().into_bytes();
    assert_eq!(manifest_bytes.len(), manifest_size);
    fs::write(root.join("SHA256SUMS"), &manifest_bytes).expect("hash manifest");
    fs::set_permissions(root.join("SHA256SUMS"), fs::Permissions::from_mode(0o400))
        .expect("hash manifest mode");

    let manifest_sha256 = sha256(&manifest_bytes);
    let manifest_entry_count =
        2 + usize::from(populate_nested_directory) + usize::from(include_auxiliary_mode_evidence);
    let manifest =
        VerifiedManifest::load_named(&root, "SHA256SUMS", &manifest_sha256, manifest_entry_count)
            .expect("hash-sealed mode fixture");
    let binding = ManifestLayerBindingV3 {
        layer_id: ManifestLayerIdV3::Outer,
        manifest_entry_count,
        manifest_relative_path: "SHA256SUMS".to_string(),
        manifest_root_kind: ManifestRootKindV3::Sha256ManifestFullInventoryV1,
        manifest_sha256,
        mode_manifest: ModeManifestBindingV3 {
            format: ModeManifestFormatV3::TypedPosixModeSizePathTsvV2,
            relative_path: "MODES.tsv".to_string(),
            sha256: sha256(&mode_bytes),
        },
        root_relative_path: ".".to_string(),
    };
    (temporary, binding, manifest)
}

#[allow(dead_code)]
fn canonical_digest(spec: &AggregateBuildSpecV3) -> String {
    sha256(&canonical_json(spec).expect("canonical spec"))
}
