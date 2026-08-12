use std::collections::BTreeMap;
use std::path::Path;

use serde_json::Value;

use crate::AcceptanceError;
use crate::durable::sha256;
use crate::manifest_inventory::VerifiedManifest;
use crate::model::AuthorityBoundary;
use crate::qualification_evidence::EvidenceBinding as LegacyEvidence;
use crate::qualification_evidence::load_evidence as load_legacy_evidence;

use super::model::AggregateManifestBinding;
use super::model::AggregateQualificationPacket;
use super::model::CandidateBindingV2;
use super::model::PlatformGateBinding;
use super::model::PlatformPolicy;
use super::model::PrerequisiteReceiptBinding;
use super::model::QualificationAssessment;
use super::model::QualificationDecision;

pub(crate) const CANDIDATE_HEAD: &str = "09e9e9ff7fa6b6c1d129d0c7a858979823e13ae8";
pub(crate) const CANDIDATE_TREE: &str = "bc14150f75cee49515e9bf244e15c526eb74e79e";
const CANDIDATE_PARENT_1: &str = "8a84ec2d76cd576f8f07eebd39764692c8bdd134";
const CANDIDATE_PARENT_2: &str = "c4b287cf5791d7f4336b925f7dfdb55ee4c3b668";
const UPSTREAM_CUTOFF: &str = CANDIDATE_PARENT_2;
const CANDIDATE_BUNDLE_SHA256: &str =
    "c6dca268010e98f759e15cde4009d8ebf49b413181ba122c3013ea6b3158d9a0";
const CANDIDATE_BUNDLE_SIZE_BYTES: u64 = 177_075_328;
const CANDIDATE_BUNDLE_RELATIVE_PATH: &str = "candidate-09e9e9ff7f.bundle";

pub(crate) const AGGREGATE_ROOT: &str =
    "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-09e9e9ff7f-aggregate-qualification-v2";
pub(crate) const LEGACY_QUALIFICATION_ROOT: &str =
    "/Volumes/T5/hepta-vnext/artifacts/receipts/qualification-3110c5aba5-final-20260810T192902Z";
pub(crate) const LEGACY_PRODUCT_AUDIT_ROOT: &str =
    "/Volumes/T5/hepta-vnext/artifacts/audits/2026-08-09-frozen-product-2f704-live-build";

const RECEIPTS_PARENT: &str = "/Volumes/T5/hepta-vnext/artifacts/receipts";
const MAC_RECEIPT_ROOT: &str =
    "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-09e9e9ff7f-mac-exact-20260812T140226Z";
const MAC_RECEIPT_MANIFEST_SHA256: &str =
    "5caed37f7439f696c69b3b5c4f4979c7582fa0626d180a0f34129a85d6e6162b";
const LINUX_RECEIPT_ROOT: &str =
    "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-09e9e9ff7f-linux-exact-20260812T141602Z";
const NIX_RECEIPT_ROOT: &str = "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-09e9e9ff7f-nix-exact-v2-20260812T141602Z";
const GITHUB_RECEIPT_ROOT: &str = "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-09e9e9ff7f-github-external-20260812T143239Z";
const GITHUB_RECEIPT_MANIFEST_SHA256: &str =
    "821be15fa70583f4f56b9c41a0cd78af24767b21ce9007ee9c36dd2872dea7b3";
const CANONICAL_PATH_TRUST_RECEIPT_ROOT: &str = "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-09e9e9ff7f-canonical-path-trust-20260812T143920Z";
const CANONICAL_PATH_TRUST_MANIFEST_SHA256: &str =
    "319c08c585a3cff07be504e78446240422bb476846fc20ed6b46d104f4acb20b";
const UPSTREAM_CUTOFF_RECEIPT_ROOT: &str = "/Volumes/T5/hepta-vnext/artifacts/receipts/vnext-main-09e9e9ff7f-upstream-cutoff-observation-20260812T1456Z";
const UPSTREAM_CUTOFF_MANIFEST_SHA256: &str =
    "67653bf2ef6bd035401d26e6f80d8af23e2deb14f91e763364c7876057fd11de";
const WINDOWS_RECEIPT_PREFIX: &str = "vnext-main-09e9e9ff7f-windows-native-";

const PACKET_SCHEMA: &str = "hepta_vnext_aggregate_qualification_packet_v2";
const PACKET_RELATIVE_PATH: &str = "qualification-packet.json";
const ASSESSMENT_SCHEMA: &str = "hepta_operator_acceptance_qualification_assessment_v2";
const MANIFEST_ROOT_KIND: &str = "sha256_of_sha256sums_bytes";
const GATES: [&str; 5] = [
    "macos-aarch64",
    "linux-x86_64",
    "nix-x86_64-linux",
    "windows-x86_64-native",
    "github-actions",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AggregateEvidence {
    pub aggregate_manifest: AggregateManifestBinding,
    pub assessment: QualificationAssessment,
    pub packet: AggregateQualificationPacket,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ObservedGate {
    candidate_executed: bool,
    candidate_failure: bool,
    executed_steps: u64,
    excluded_from_pass: bool,
    pass: bool,
    status: &'static str,
}

pub(crate) fn load_aggregate_evidence(
    aggregate_root: &Path,
    externally_pinned_manifest_sha256: &str,
    legacy_product_audit_root: &Path,
) -> Result<AggregateEvidence, AcceptanceError> {
    if aggregate_root != Path::new(AGGREGATE_ROOT)
        || legacy_product_audit_root != Path::new(LEGACY_PRODUCT_AUDIT_ROOT)
    {
        return Err(invalid(
            "V2 requires the exact 09e9 aggregate and legacy product-audit roots",
        ));
    }
    if !digest_shape(externally_pinned_manifest_sha256) {
        return Err(invalid(
            "externally pinned aggregate manifest digest is malformed",
        ));
    }
    let aggregate =
        VerifiedManifest::load_digest_pinned(aggregate_root, externally_pinned_manifest_sha256)?;
    if aggregate.entry_count() == 0 || aggregate.entry_count() > 8 {
        return Err(invalid(
            "aggregate packet inventory must contain 1..=8 files",
        ));
    }
    let packet_bytes = aggregate.bytes(PACKET_RELATIVE_PATH)?;
    let packet: AggregateQualificationPacket = aggregate.json_canonical(PACKET_RELATIVE_PATH)?;
    let legacy = load_legacy_evidence(
        Path::new(LEGACY_QUALIFICATION_ROOT),
        legacy_product_audit_root,
    )?;
    validate_packet(&packet, &legacy)?;
    validate_platform_receipts(&packet)?;

    let mut assessment = assess_packet(&packet);
    assessment.aggregate_manifest_sha256 = externally_pinned_manifest_sha256.to_string();
    Ok(AggregateEvidence {
        aggregate_manifest: AggregateManifestBinding {
            manifest_entry_count: aggregate.entry_count(),
            manifest_root_kind: MANIFEST_ROOT_KIND.to_string(),
            manifest_sha256: externally_pinned_manifest_sha256.to_string(),
            packet_relative_path: PACKET_RELATIVE_PATH.to_string(),
            packet_sha256: sha256(&packet_bytes),
            receipt_root: path_string(&aggregate.root)?,
        },
        assessment,
        packet,
    })
}

pub(crate) fn assess_packet(packet: &AggregateQualificationPacket) -> QualificationAssessment {
    let blockers = computed_blockers(&packet.platform_receipts);
    let github_excluded_from_pass = packet
        .platform_receipts
        .iter()
        .find(|gate| gate.gate == "github-actions")
        .is_some_and(|gate| gate.excluded_from_pass && !gate.pass);
    QualificationAssessment {
        aggregate_manifest_sha256: String::new(),
        blockers: blockers.clone(),
        candidate_head: packet.candidate.head.clone(),
        candidate_tree: packet.candidate.tree.clone(),
        github_excluded_from_pass,
        ready_for_challenge: blockers.is_empty(),
        schema: ASSESSMENT_SCHEMA.to_string(),
    }
}

fn validate_packet(
    packet: &AggregateQualificationPacket,
    legacy: &LegacyEvidence,
) -> Result<(), AcceptanceError> {
    if packet.schema != PACKET_SCHEMA
        || packet.schema_version != 2
        || packet.automatic_transition
        || packet.authority != AuthorityBoundary::all_closed()
        || packet.candidate != exact_candidate()
        || packet.legacy_frozen_product != legacy.frozen_product
        || packet.legacy_oracle != legacy.oracle
        || packet.platform_policy != exact_platform_policy()
        || packet.platform_receipts.len() != GATES.len()
        || packet.prerequisite_receipts.len() != 2
    {
        return Err(invalid(
            "aggregate packet differs from the exact head-specific V2 boundary",
        ));
    }

    for (gate, expected_id) in packet.platform_receipts.iter().zip(GATES) {
        if gate.gate != expected_id || !gate.required {
            return Err(invalid(
                "platform receipts must be complete, required, and canonically ordered",
            ));
        }
        validate_gate_shape(gate)?;
    }
    if packet.prerequisite_receipts[0].id != "canonical-path-trust"
        || !packet.prerequisite_receipts[0].pass
        || packet.prerequisite_receipts[1].id != "upstream-cutoff-observation"
        || !packet.prerequisite_receipts[1].pass
    {
        return Err(invalid(
            "path/trust and cutoff prerequisites must be present and canonically ordered",
        ));
    }

    let blockers = computed_blockers(&packet.platform_receipts);
    let pass_gate_count = packet
        .platform_receipts
        .iter()
        .filter(|gate| gate.pass)
        .count();
    let expected_decision = QualificationDecision {
        blockers: blockers.clone(),
        complete_gate_count: GATES.len(),
        pass_gate_count,
        verdict: if blockers.is_empty() {
            "PASS"
        } else {
            "BLOCKED"
        }
        .to_string(),
    };
    if packet.decision != expected_decision {
        return Err(invalid(
            "aggregate decision does not equal the fail-closed gate calculation",
        ));
    }
    Ok(())
}

fn validate_gate_shape(gate: &PlatformGateBinding) -> Result<(), AcceptanceError> {
    match gate.status.as_str() {
        "PASS"
            if gate.pass
                && gate.candidate_executed
                && gate.executed_steps > 0
                && !gate.candidate_failure
                && !gate.excluded_from_pass => {}
        "BLOCKED_EXTERNAL"
            if gate.gate == "github-actions"
                && !gate.pass
                && !gate.candidate_executed
                && gate.executed_steps == 0
                && !gate.candidate_failure
                && gate.excluded_from_pass => {}
        _ => {
            return Err(invalid(format!(
                "platform gate has an invalid status/execution boundary: {}",
                gate.gate
            )));
        }
    }
    Ok(())
}

fn validate_platform_receipts(
    packet: &AggregateQualificationPacket,
) -> Result<(), AcceptanceError> {
    let mut seen_roots = Vec::new();
    for gate in &packet.platform_receipts {
        let root = Path::new(&gate.receipt.receipt_root);
        validate_receipt_root(&gate.gate, root)?;
        match gate.gate.as_str() {
            "macos-aarch64" if gate.receipt.manifest_sha256 != MAC_RECEIPT_MANIFEST_SHA256 => {
                return Err(invalid(
                    "macOS receipt manifest differs from its source pin",
                ));
            }
            "github-actions" if gate.receipt.manifest_sha256 != GITHUB_RECEIPT_MANIFEST_SHA256 => {
                return Err(invalid(
                    "GitHub receipt manifest differs from its source pin",
                ));
            }
            _ => {}
        }
        if root.join("SUPERSEDED.txt").exists() {
            return Err(invalid(format!(
                "platform receipt is superseded: {}",
                gate.gate
            )));
        }
        if seen_roots.contains(&gate.receipt.receipt_root) {
            return Err(invalid("platform receipt roots must be distinct"));
        }
        seen_roots.push(gate.receipt.receipt_root.clone());
        let manifest = VerifiedManifest::load(
            root,
            &gate.receipt.manifest_sha256,
            gate.receipt.manifest_entry_count,
        )?;
        manifest.require_hash(
            &gate.receipt.status_artifact_relative_path,
            &gate.receipt.status_artifact_sha256,
        )?;
        let observed = observe_gate(
            &gate.gate,
            &manifest,
            &gate.receipt.status_artifact_relative_path,
        )?;
        let expected = ObservedGate {
            candidate_executed: gate.candidate_executed,
            candidate_failure: gate.candidate_failure,
            executed_steps: gate.executed_steps,
            excluded_from_pass: gate.excluded_from_pass,
            pass: gate.pass,
            status: if gate.status == "PASS" {
                "PASS"
            } else {
                "BLOCKED_EXTERNAL"
            },
        };
        if observed != expected {
            return Err(invalid(format!(
                "platform packet claims differ from receipt evidence: {}",
                gate.gate
            )));
        }
        if gate.gate == packet.candidate.bundle_receipt_gate {
            let bundle = manifest
                .entry(&packet.candidate.bundle_relative_path)
                .ok_or_else(|| invalid("exact candidate bundle is absent from its receipt"))?;
            if bundle.sha256 != packet.candidate.bundle_sha256
                || bundle.size_bytes != packet.candidate.bundle_size_bytes
            {
                return Err(invalid("exact candidate bundle differs from the 09e9 pin"));
            }
        }
    }
    validate_prerequisite_receipts(&packet.prerequisite_receipts)?;
    Ok(())
}

fn validate_prerequisite_receipts(
    prerequisites: &[PrerequisiteReceiptBinding],
) -> Result<(), AcceptanceError> {
    if prerequisites.len() != 2 {
        return Err(invalid("V2 prerequisite receipt set is incomplete"));
    }
    validate_path_trust_prerequisite(&prerequisites[0])?;
    validate_cutoff_prerequisite(&prerequisites[1])
}

fn validate_path_trust_prerequisite(
    prerequisite: &PrerequisiteReceiptBinding,
) -> Result<(), AcceptanceError> {
    if prerequisite.id != "canonical-path-trust"
        || !prerequisite.pass
        || prerequisite.receipt.receipt_root != CANONICAL_PATH_TRUST_RECEIPT_ROOT
        || prerequisite.receipt.manifest_sha256 != CANONICAL_PATH_TRUST_MANIFEST_SHA256
        || prerequisite.receipt.status_artifact_relative_path != "status.txt"
    {
        return Err(invalid("canonical path/trust prerequisite binding differs"));
    }
    let manifest = VerifiedManifest::load(
        Path::new(&prerequisite.receipt.receipt_root),
        &prerequisite.receipt.manifest_sha256,
        prerequisite.receipt.manifest_entry_count,
    )?;
    manifest.require_hash(
        &prerequisite.receipt.status_artifact_relative_path,
        &prerequisite.receipt.status_artifact_sha256,
    )?;
    let values = parse_key_values(&manifest.bytes("status.txt")?)?;
    for (key, expected) in [
        ("schema", "hepta_vnext_canonical_path_trust_v1"),
        ("status", "pass"),
        ("candidate_head", CANDIDATE_HEAD),
        ("candidate_tree", CANDIDATE_TREE),
        ("canonical_local_ref_exact", "true"),
        ("canonical_remote_ref_exact", "true"),
        ("three_integration_remote_refs_exact", "true"),
        ("archive_remote_ref_exact", "true"),
        ("worktree_clean", "true"),
        ("t5_uuid_exact", "true"),
        ("t5_owners_enabled", "true"),
        ("codex_main_integration_trusted", "true"),
        ("codex_ui_main_trusted", "true"),
        ("old_hepta_codex_trust_entry", "false"),
        ("agent_instructions_point_to_t5", "true"),
        ("old_workspace_paths_frozen", "true"),
        ("ui_main_independent_clean", "true"),
        ("default_main_changed", "false"),
        ("production_changed", "false"),
    ] {
        require_value(&values, key, expected)?;
    }
    Ok(())
}

fn validate_cutoff_prerequisite(
    prerequisite: &PrerequisiteReceiptBinding,
) -> Result<(), AcceptanceError> {
    if prerequisite.id != "upstream-cutoff-observation"
        || !prerequisite.pass
        || prerequisite.receipt.receipt_root != UPSTREAM_CUTOFF_RECEIPT_ROOT
        || prerequisite.receipt.manifest_sha256 != UPSTREAM_CUTOFF_MANIFEST_SHA256
        || prerequisite.receipt.status_artifact_relative_path != "upstream-cutoff.txt"
    {
        return Err(invalid("upstream cutoff prerequisite binding differs"));
    }
    let manifest = VerifiedManifest::load(
        Path::new(&prerequisite.receipt.receipt_root),
        &prerequisite.receipt.manifest_sha256,
        prerequisite.receipt.manifest_entry_count,
    )?;
    manifest.require_hash(
        &prerequisite.receipt.status_artifact_relative_path,
        &prerequisite.receipt.status_artifact_sha256,
    )?;
    let values = parse_key_values(&manifest.bytes("upstream-cutoff.txt")?)?;
    for (key, expected) in [
        ("schema", "hepta_vnext_upstream_cutoff_observation_v1"),
        ("candidate_head", CANDIDATE_HEAD),
        ("candidate_tree", CANDIDATE_TREE),
        ("frozen_upstream_cutoff", UPSTREAM_CUTOFF),
        (
            "observed_upstream_main",
            "9dd22890f5ff47e4af128c20e32b9758a61d78d2",
        ),
        ("post_cutoff_commit_count", "4"),
        ("post_cutoff_changed_file_count", "23"),
        ("candidate_changed", "false"),
        ("qualification_invalidated", "false"),
        (
            "policy",
            "post_cutoff_commits_enter_next_development_cycle_backlog",
        ),
    ] {
        require_value(&values, key, expected)?;
    }
    Ok(())
}

fn observe_gate(
    gate: &str,
    manifest: &VerifiedManifest,
    status_relative_path: &str,
) -> Result<ObservedGate, AcceptanceError> {
    match gate {
        "macos-aarch64" => observe_mac(manifest, status_relative_path),
        "linux-x86_64" => observe_linux(manifest, status_relative_path),
        "nix-x86_64-linux" => observe_nix(manifest, status_relative_path),
        "windows-x86_64-native" => observe_windows(manifest, status_relative_path),
        "github-actions" => observe_github(manifest, status_relative_path),
        _ => Err(invalid("unknown platform gate")),
    }
}

fn observe_mac(
    manifest: &VerifiedManifest,
    status_relative_path: &str,
) -> Result<ObservedGate, AcceptanceError> {
    if status_relative_path != "qualification-status.txt" {
        return Err(invalid("macOS status artifact path differs"));
    }
    let values = parse_key_values(&manifest.bytes(status_relative_path)?)?;
    for (key, expected) in [
        ("schema", "hepta_vnext_main_mac_validation_v4"),
        ("status", "pass"),
        ("candidate_commit", CANDIDATE_HEAD),
        ("candidate_tree", CANDIDATE_TREE),
        ("mac_binary_built_from_exact_head", "true"),
        ("mac_binary_inherited", "false"),
        ("target_release_inherited", "false"),
        ("isolated_canary", "pass"),
        ("isolated_canary_authority_all_closed", "true"),
        ("isolated_canary_production_service_changed", "false"),
        ("candidate_operator_acceptance", "false"),
        ("enforce", "false"),
        ("promotion", "false"),
        ("outbound", "false"),
        ("retirement", "false"),
        ("automatic_transition", "false"),
        ("default_branch_changed", "false"),
        ("production_cutover", "false"),
    ] {
        require_value(&values, key, expected)?;
    }
    Ok(pass_observation(count_tsv_rows(manifest, "steps.tsv")?))
}

fn observe_linux(
    manifest: &VerifiedManifest,
    status_relative_path: &str,
) -> Result<ObservedGate, AcceptanceError> {
    if status_relative_path != "result.txt" {
        return Err(invalid("Linux status artifact path differs"));
    }
    let values = parse_key_values(&manifest.bytes(status_relative_path)?)?;
    for (key, expected) in [
        ("linux_exact_rc", "0"),
        ("qualification", "true"),
        ("verdict", "PASS"),
        ("candidate_pass", "true"),
        ("candidate_fail", "false"),
        ("candidate_execution_started", "true"),
        ("candidate_execution_completed", "true"),
        ("postflight_verified", "true"),
        ("candidate_head", CANDIDATE_HEAD),
        ("candidate_tree", CANDIDATE_TREE),
        ("source_identity", "match"),
        ("worktree_clean", "true"),
        ("production_changed", "false"),
    ] {
        require_value(&values, key, expected)?;
    }
    Ok(pass_observation(count_tsv_rows(manifest, "steps.tsv")?))
}

fn observe_nix(
    manifest: &VerifiedManifest,
    status_relative_path: &str,
) -> Result<ObservedGate, AcceptanceError> {
    if status_relative_path != "result.txt" {
        return Err(invalid("Nix status artifact path differs"));
    }
    let values = parse_key_values(&manifest.bytes(status_relative_path)?)?;
    for (key, expected) in [
        ("schema", "hepta_vnext_nix_qualification_v2_umbrella"),
        ("candidate_head", CANDIDATE_HEAD),
        ("candidate_tree", CANDIDATE_TREE),
        ("status", "PASS"),
        ("verdict", "PASS"),
        ("candidate_pass", "true"),
        ("candidate_fail", "false"),
        ("harness_pass", "true"),
        ("harness_fail", "false"),
        ("container_exit_code", "0"),
        ("container_oom_killed", "false"),
        ("metadata_rc", "0"),
        ("flake_check_rc", "0"),
        ("build_rc", "0"),
        ("output_verify_rc", "0"),
        ("remote_manifest_strict_check", "true"),
        ("t5_copy_strict_check", "true"),
        ("source_identity_postflight", "true"),
        ("worktree_clean_postflight", "true"),
        ("fresh_source", "true"),
        ("fresh_nix_store", "true"),
        ("defaults_or_services_changed", "false"),
    ] {
        require_value(&values, key, expected)?;
    }
    Ok(pass_observation(count_zero_exit_codes(
        manifest,
        "remote/exit-codes.txt",
    )?))
}

fn observe_windows(
    manifest: &VerifiedManifest,
    status_relative_path: &str,
) -> Result<ObservedGate, AcceptanceError> {
    if status_relative_path != "result.txt" {
        return Err(invalid("native Windows status artifact path differs"));
    }
    let values = parse_key_values(&manifest.bytes(status_relative_path)?)?;
    for (key, expected) in [
        ("schema", "hepta_vnext_windows_native_qualification_v2"),
        ("candidate_head", CANDIDATE_HEAD),
        ("candidate_tree", CANDIDATE_TREE),
        ("status", "PASS"),
        ("verdict", "PASS"),
        ("candidate_pass", "true"),
        ("candidate_fail", "false"),
        ("candidate_execution_started", "true"),
        ("candidate_execution_completed", "true"),
        ("source_identity", "match"),
        ("worktree_clean", "true"),
        ("production_changed", "false"),
    ] {
        require_value(&values, key, expected)?;
    }
    Ok(pass_observation(count_tsv_rows(manifest, "steps.tsv")?))
}

fn observe_github(
    manifest: &VerifiedManifest,
    status_relative_path: &str,
) -> Result<ObservedGate, AcceptanceError> {
    if status_relative_path != "github-external-gate.json" {
        return Err(invalid("GitHub status artifact path differs"));
    }
    let value: Value = serde_json::from_slice(&manifest.bytes(status_relative_path)?)
        .map_err(|error| invalid(format!("invalid GitHub gate receipt: {error}")))?;
    require_json_string(
        &value,
        "/schema",
        "hepta-vnext-github-external-gate-receipt-v1",
    )?;
    require_json_string(&value, "/candidate/commit", CANDIDATE_HEAD)?;
    require_json_string(&value, "/candidate/tree", CANDIDATE_TREE)?;
    require_json_string(&value, "/decision/verdict", "BLOCKED_EXTERNAL")?;
    for (pointer, expected) in [
        ("/decision/candidate_executed_on_github", false),
        ("/decision/candidate_executed_on_github_windows", false),
        ("/decision/candidate_failure", false),
        ("/decision/github_pass", false),
        ("/decision/github_windows_pass", false),
        ("/decision/native_windows_result_is_separate", true),
    ] {
        require_json_bool(&value, pointer, expected)?;
    }
    let runs = value
        .pointer("/runs")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid("GitHub receipt runs are absent"))?;
    if runs.is_empty() {
        return Err(invalid("GitHub receipt has no candidate-specific runs"));
    }
    let mut jobs = 0_u64;
    let mut zero_step_jobs = 0_u64;
    for run in runs {
        require_json_string(run, "/head_sha", CANDIDATE_HEAD)?;
        let run_jobs = json_u64(run, "/jobs")?;
        let run_zero = json_u64(run, "/zero_step_jobs")?;
        if run_jobs == 0 || run_zero != run_jobs || json_u64(run, "/artifacts")? != 0 {
            return Err(invalid(
                "GitHub BLOCKED_EXTERNAL receipt contains execution or artifacts",
            ));
        }
        jobs = jobs
            .checked_add(run_jobs)
            .ok_or_else(|| invalid("GitHub job count overflow"))?;
        zero_step_jobs = zero_step_jobs
            .checked_add(run_zero)
            .ok_or_else(|| invalid("GitHub zero-step job count overflow"))?;
    }
    if jobs == 0 || jobs != zero_step_jobs {
        return Err(invalid("GitHub zero-step classification is inconsistent"));
    }
    Ok(ObservedGate {
        candidate_executed: false,
        candidate_failure: false,
        executed_steps: 0,
        excluded_from_pass: true,
        pass: false,
        status: "BLOCKED_EXTERNAL",
    })
}

fn pass_observation(executed_steps: u64) -> ObservedGate {
    ObservedGate {
        candidate_executed: true,
        candidate_failure: false,
        executed_steps,
        excluded_from_pass: false,
        pass: true,
        status: "PASS",
    }
}

fn validate_receipt_root(gate: &str, root: &Path) -> Result<(), AcceptanceError> {
    let expected = match gate {
        "macos-aarch64" => Some(MAC_RECEIPT_ROOT),
        "linux-x86_64" => Some(LINUX_RECEIPT_ROOT),
        "nix-x86_64-linux" => Some(NIX_RECEIPT_ROOT),
        "github-actions" => Some(GITHUB_RECEIPT_ROOT),
        "windows-x86_64-native" => None,
        _ => return Err(invalid("unknown receipt root gate")),
    };
    if let Some(expected) = expected {
        if root != Path::new(expected) {
            return Err(invalid(format!("receipt root differs for {gate}")));
        }
    } else {
        let parent = root.parent();
        let basename = root.file_name().and_then(|value| value.to_str());
        if parent != Some(Path::new(RECEIPTS_PARENT))
            || !basename.is_some_and(|value| value.starts_with(WINDOWS_RECEIPT_PREFIX))
        {
            return Err(invalid(
                "native Windows receipt root differs from its exact prefix",
            ));
        }
    }
    Ok(())
}

fn exact_candidate() -> CandidateBindingV2 {
    CandidateBindingV2 {
        bundle_receipt_gate: "linux-x86_64".to_string(),
        bundle_relative_path: CANDIDATE_BUNDLE_RELATIVE_PATH.to_string(),
        bundle_sha256: CANDIDATE_BUNDLE_SHA256.to_string(),
        bundle_size_bytes: CANDIDATE_BUNDLE_SIZE_BYTES,
        head: CANDIDATE_HEAD.to_string(),
        parent_1: CANDIDATE_PARENT_1.to_string(),
        parent_2: CANDIDATE_PARENT_2.to_string(),
        tree: CANDIDATE_TREE.to_string(),
        upstream_cutoff: UPSTREAM_CUTOFF.to_string(),
    }
}

fn exact_platform_policy() -> PlatformPolicy {
    PlatformPolicy {
        blocked_external_satisfies_required_gate: false,
        native_windows_substitutes_for_github: false,
        require_all_required_gates_pass: true,
        required_gates: GATES.iter().map(|gate| (*gate).to_string()).collect(),
        zero_step_execution_satisfies_pass: false,
    }
}

fn computed_blockers(receipts: &[PlatformGateBinding]) -> Vec<String> {
    receipts
        .iter()
        .filter(|gate| gate.required && !gate.pass)
        .map(|gate| gate.gate.clone())
        .collect()
}

fn parse_key_values(bytes: &[u8]) -> Result<BTreeMap<String, String>, AcceptanceError> {
    let text = std::str::from_utf8(bytes).map_err(|_| invalid("receipt is not UTF-8"))?;
    if !text.ends_with('\n') {
        return Err(invalid("key/value receipt is not newline terminated"));
    }
    let mut values = BTreeMap::new();
    for line in text.lines() {
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| invalid("key/value receipt contains a malformed line"))?;
        if key.is_empty()
            || value.is_empty()
            || values.insert(key.to_string(), value.to_string()).is_some()
        {
            return Err(invalid(
                "key/value receipt contains an invalid or duplicate field",
            ));
        }
    }
    Ok(values)
}

fn require_value(
    values: &BTreeMap<String, String>,
    key: &str,
    expected: &str,
) -> Result<(), AcceptanceError> {
    if values.get(key).map(String::as_str) != Some(expected) {
        return Err(invalid(format!("receipt field differs: {key}")));
    }
    Ok(())
}

fn count_tsv_rows(manifest: &VerifiedManifest, relative: &str) -> Result<u64, AcceptanceError> {
    let bytes = manifest.bytes(relative)?;
    let text = std::str::from_utf8(&bytes).map_err(|_| invalid("steps.tsv is not UTF-8"))?;
    if !text.ends_with('\n') {
        return Err(invalid("steps.tsv is not newline terminated"));
    }
    let mut rows = 0_u64;
    for line in text.lines() {
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() < 2 || fields[0].is_empty() || fields[1].parse::<i32>().is_err() {
            return Err(invalid("steps.tsv contains a malformed row"));
        }
        rows = rows
            .checked_add(1)
            .ok_or_else(|| invalid("step count overflow"))?;
    }
    if rows == 0 {
        return Err(invalid("steps.tsv contains no executed steps"));
    }
    Ok(rows)
}

fn count_zero_exit_codes(
    manifest: &VerifiedManifest,
    relative: &str,
) -> Result<u64, AcceptanceError> {
    let values = parse_key_values(&manifest.bytes(relative)?)?;
    if values.is_empty() || values.values().any(|value| value != "0") {
        return Err(invalid("Nix exit-code receipt is absent or nonzero"));
    }
    u64::try_from(values.len()).map_err(|_| invalid("Nix step count overflow"))
}

fn require_json_string(
    value: &Value,
    pointer: &str,
    expected: &str,
) -> Result<(), AcceptanceError> {
    if value.pointer(pointer).and_then(Value::as_str) != Some(expected) {
        return Err(invalid(format!("JSON receipt field differs: {pointer}")));
    }
    Ok(())
}

fn require_json_bool(value: &Value, pointer: &str, expected: bool) -> Result<(), AcceptanceError> {
    if value.pointer(pointer).and_then(Value::as_bool) != Some(expected) {
        return Err(invalid(format!("JSON receipt field differs: {pointer}")));
    }
    Ok(())
}

fn json_u64(value: &Value, pointer: &str) -> Result<u64, AcceptanceError> {
    value
        .pointer(pointer)
        .and_then(Value::as_u64)
        .ok_or_else(|| invalid(format!("JSON receipt integer is absent: {pointer}")))
}

fn path_string(path: &Path) -> Result<String, AcceptanceError> {
    path.to_str()
        .map(str::to_string)
        .ok_or_else(|| invalid("canonical evidence path is not UTF-8"))
}

fn digest_shape(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn invalid(message: impl Into<String>) -> AcceptanceError {
    AcceptanceError::Invalid(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::FrozenProductBinding;
    use crate::model::OracleBinding;
    use crate::v2::model::ReceiptManifestBinding;

    #[test]
    fn github_zero_step_is_a_blocker_not_a_pass() {
        let mut packet = packet_with_github_blocked();
        let legacy = legacy();
        validate_packet(&packet, &legacy).expect("valid blocked packet");
        let assessment = assess_packet(&packet);
        assert!(!assessment.ready_for_challenge);
        assert_eq!(assessment.blockers, vec!["github-actions"]);
        assert!(assessment.github_excluded_from_pass);

        let github = packet.platform_receipts.last_mut().unwrap();
        github.pass = true;
        github.status = "PASS".to_string();
        assert!(validate_packet(&packet, &legacy).is_err());
    }

    #[test]
    fn authority_and_policy_cannot_be_relaxed() {
        let legacy = legacy();
        let mut packet = packet_with_github_blocked();
        packet.authority.promotion = true;
        assert!(validate_packet(&packet, &legacy).is_err());

        let mut packet = packet_with_github_blocked();
        packet
            .platform_policy
            .blocked_external_satisfies_required_gate = true;
        assert!(validate_packet(&packet, &legacy).is_err());

        let mut packet = packet_with_github_blocked();
        packet.platform_policy.native_windows_substitutes_for_github = true;
        assert!(validate_packet(&packet, &legacy).is_err());
    }

    fn packet_with_github_blocked() -> AggregateQualificationPacket {
        let legacy = legacy();
        let mut gates = GATES
            .iter()
            .map(|gate| PlatformGateBinding {
                candidate_executed: true,
                candidate_failure: false,
                executed_steps: 1,
                excluded_from_pass: false,
                gate: (*gate).to_string(),
                pass: true,
                receipt: ReceiptManifestBinding {
                    manifest_entry_count: 1,
                    manifest_sha256: "a".repeat(64),
                    receipt_root: format!("/receipt/{gate}"),
                    status_artifact_relative_path: "result.txt".to_string(),
                    status_artifact_sha256: "b".repeat(64),
                },
                required: true,
                status: "PASS".to_string(),
            })
            .collect::<Vec<_>>();
        let github = gates.last_mut().unwrap();
        github.candidate_executed = false;
        github.executed_steps = 0;
        github.excluded_from_pass = true;
        github.pass = false;
        github.status = "BLOCKED_EXTERNAL".to_string();
        AggregateQualificationPacket {
            automatic_transition: false,
            authority: AuthorityBoundary::all_closed(),
            candidate: exact_candidate(),
            decision: QualificationDecision {
                blockers: vec!["github-actions".to_string()],
                complete_gate_count: GATES.len(),
                pass_gate_count: GATES.len() - 1,
                verdict: "BLOCKED".to_string(),
            },
            legacy_frozen_product: legacy.frozen_product,
            legacy_oracle: legacy.oracle,
            platform_policy: exact_platform_policy(),
            platform_receipts: gates,
            prerequisite_receipts: vec![
                PrerequisiteReceiptBinding {
                    id: "canonical-path-trust".to_string(),
                    pass: true,
                    receipt: ReceiptManifestBinding {
                        manifest_entry_count: 1,
                        manifest_sha256: "c".repeat(64),
                        receipt_root: "/receipt/path-trust".to_string(),
                        status_artifact_relative_path: "status.txt".to_string(),
                        status_artifact_sha256: "d".repeat(64),
                    },
                },
                PrerequisiteReceiptBinding {
                    id: "upstream-cutoff-observation".to_string(),
                    pass: true,
                    receipt: ReceiptManifestBinding {
                        manifest_entry_count: 1,
                        manifest_sha256: "e".repeat(64),
                        receipt_root: "/receipt/cutoff".to_string(),
                        status_artifact_relative_path: "upstream-cutoff.txt".to_string(),
                        status_artifact_sha256: "f".repeat(64),
                    },
                },
            ],
            schema: PACKET_SCHEMA.to_string(),
            schema_version: 2,
        }
    }

    fn legacy() -> LegacyEvidence {
        LegacyEvidence {
            candidate: crate::model::CandidateBinding {
                base: String::new(),
                bundle_sha256: String::new(),
                head: String::new(),
                tree: String::new(),
            },
            frozen_product: FrozenProductBinding {
                audit_manifest_entry_count: 6,
                audit_manifest_sha256: "1".repeat(64),
                audit_root: "/legacy".to_string(),
                binary_relative_path: "hepta".to_string(),
                binary_sha256: "2".repeat(64),
                binary_size_bytes: 1,
                platform: "macos".to_string(),
                source_commit: "commit".to_string(),
                source_tree: "tree".to_string(),
            },
            oracle: OracleBinding {
                commit: "commit".to_string(),
                corpus_sha256: "3".repeat(64),
                expected_normalized_receipt_sha256: "4".repeat(64),
                sample_id_sha256: "5".repeat(64),
                tree: "tree".to_string(),
            },
            qualification_receipt: crate::model::QualificationReceiptBinding {
                candidate_bundle_sha256: String::new(),
                git_tree_manifest_sha256: String::new(),
                manifest_entry_count: 0,
                manifest_root_kind: String::new(),
                manifest_sha256: String::new(),
                receipt_id: String::new(),
                receipt_root: String::new(),
                runs: Vec::new(),
                soak_summary_sha256: String::new(),
                status_sha256: String::new(),
                tracked_content_manifest_sha256: String::new(),
            },
        }
    }
}
