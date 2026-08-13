use std::fs;
use std::path::Path;
use std::path::PathBuf;

use serde_json::Value;
use serde_json::json;
use tempfile::TempDir;

use crate::durable::canonical_json;
use crate::durable::sha256;
use crate::model::AuthorityBoundary;

use super::builder::build_for_test;
use super::builder::plan_for_test;
use super::builder::verify_for_test;
use super::evidence::BUILD_SPEC_SCHEMA;
use super::evidence::GATES;
use super::evidence::PREREQUISITES;
use super::evidence::ValidationPolicy;
use super::evidence::assess_packet;
use super::evidence::exact_platform_policy;
use super::evidence::validate_spec;
use super::model::AggregateBuildSpecV3;
use super::model::ArtifactAssertionV3;
use super::model::CandidateBindingV3;
use super::model::CandidateBundleBindingV3;
use super::model::EvidenceArtifactBindingV3;
use super::model::EvidenceArtifactFormatV3;
use super::model::ManifestLayerBindingV3;
use super::model::ManifestRootKindV3;
use super::model::ModeManifestBindingV3;
use super::model::ModeManifestFormatV3;
use super::model::PlatformGateInputV3;
use super::model::PrerequisiteInputV3;
use super::model::ReceiptEvidenceBindingV3;
use super::model::SemanticClaimBindingV3;
use super::model::SemanticClaimV3;

struct Fixture {
    _temporary: TempDir,
    candidate: CandidateBindingV3,
    receipts_parent: PathBuf,
    spec: AggregateBuildSpecV3,
}

impl Fixture {
    fn pass() -> Self {
        let temporary = tempfile::tempdir().expect("tempdir");
        private_dir(temporary.path());
        let receipts_parent = temporary.path().join("receipts");
        fs::create_dir(&receipts_parent).expect("receipt parent");
        private_dir(&receipts_parent);

        let bundle_bytes = b"synthetic candidate bundle";
        let candidate = CandidateBindingV3 {
            bundle: CandidateBundleBindingV3 {
                prerequisite_id: "portable-inputs".to_string(),
                relative_path: "candidate.bundle".to_string(),
                sha256: sha256(bundle_bytes),
                size_bytes: bundle_bytes.len() as u64,
            },
            head: "1111111111111111111111111111111111111111".to_string(),
            integration_merge: "2222222222222222222222222222222222222222".to_string(),
            parents: vec!["3333333333333333333333333333333333333333".to_string()],
            tree: "4444444444444444444444444444444444444444".to_string(),
            upstream_cutoff: "5555555555555555555555555555555555555555".to_string(),
        };

        let mut gates = Vec::new();
        for gate in GATES {
            let root = receipts_parent.join(gate);
            let status = gate_status(&candidate, "PASS", true, true, false, false, 1);
            let receipt = direct_receipt(&root, "synthetic_gate_v1", status, None);
            gates.push(PlatformGateInputV3 {
                gate: gate.to_string(),
                receipt,
                required: true,
            });
        }

        let mut prerequisites = Vec::new();
        for id in PREREQUISITES {
            let root = receipts_parent.join(id);
            let status = prerequisite_status(&candidate);
            let extra = (id == "portable-inputs").then_some((
                candidate.bundle.relative_path.as_str(),
                bundle_bytes.as_slice(),
            ));
            let receipt = direct_receipt(&root, "synthetic_prerequisite_v1", status, extra);
            prerequisites.push(PrerequisiteInputV3 {
                id: id.to_string(),
                receipt,
                required: true,
            });
        }

        let spec = AggregateBuildSpecV3 {
            automatic_transition: false,
            authority: AuthorityBoundary::all_closed(),
            candidate: candidate.clone(),
            platform_gates: gates,
            platform_policy: exact_platform_policy(),
            prerequisite_receipts: prerequisites,
            schema: BUILD_SPEC_SCHEMA.to_string(),
            schema_version: 3,
        };
        Self {
            _temporary: temporary,
            candidate,
            receipts_parent,
            spec,
        }
    }

    fn validate(&self) -> Result<super::AggregateQualificationPacketV3, crate::AcceptanceError> {
        validate_spec(
            &self.spec,
            ValidationPolicy {
                expected_candidate: &self.candidate,
                receipts_parent: &self.receipts_parent,
            },
        )
    }
}

#[test]
fn all_pass_synthetic_graph_is_ready_and_head_topology_is_single_parent() {
    let fixture = Fixture::pass();
    let packet = fixture.validate().expect("all-pass packet");
    assert_eq!(packet.candidate.parents, fixture.candidate.parents);
    assert_eq!(packet.candidate.parents.len(), 1);
    assert_eq!(
        packet.candidate.upstream_cutoff,
        fixture.candidate.upstream_cutoff
    );
    assert_eq!(packet.decision.verdict, "PASS");
    assert_eq!(packet.decision.pass_gate_count, 5);
    assert_eq!(packet.decision.prerequisite_pass_count, 3);
    assert!(assess_packet(&packet, "").ready_for_challenge);
}

#[test]
fn unknown_fields_and_wrong_candidate_topology_are_rejected() {
    let fixture = Fixture::pass();
    let mut value = serde_json::to_value(&fixture.spec).expect("spec value");
    value
        .as_object_mut()
        .expect("spec object")
        .insert("unexpected".to_string(), Value::Bool(true));
    assert!(serde_json::from_value::<AggregateBuildSpecV3>(value).is_err());

    let mut wrong_parent = fixture.spec.clone();
    wrong_parent.candidate.parents = vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string()];
    assert!(
        validate_spec(
            &wrong_parent,
            ValidationPolicy {
                expected_candidate: &fixture.candidate,
                receipts_parent: &fixture.receipts_parent,
            },
        )
        .is_err()
    );
}

#[test]
fn missing_gate_prerequisite_and_duplicate_root_fail_closed() {
    let fixture = Fixture::pass();
    let mut missing_gate = fixture.spec.clone();
    missing_gate.platform_gates.pop();
    assert!(validate(&fixture, &missing_gate).is_err());

    let mut missing_prerequisite = fixture.spec.clone();
    missing_prerequisite.prerequisite_receipts.pop();
    assert!(validate(&fixture, &missing_prerequisite).is_err());

    let mut duplicate = fixture.spec.clone();
    duplicate.platform_gates[1].receipt.receipt_root =
        duplicate.platform_gates[0].receipt.receipt_root.clone();
    duplicate.platform_gates[1].receipt.manifest_layers =
        duplicate.platform_gates[0].receipt.manifest_layers.clone();
    duplicate.platform_gates[1].receipt.artifacts =
        duplicate.platform_gates[0].receipt.artifacts.clone();
    duplicate.platform_gates[1].receipt.semantic_claims =
        duplicate.platform_gates[0].receipt.semantic_claims.clone();
    assert!(validate(&fixture, &duplicate).is_err());
}

#[test]
fn github_blocked_external_is_valid_evidence_but_never_ready() {
    let mut fixture = Fixture::pass();
    replace_gate(
        &mut fixture,
        "github-actions",
        "BLOCKED_EXTERNAL",
        false,
        false,
        false,
        false,
        0,
        false,
    );
    let packet = fixture.validate().expect("blocked packet is assessable");
    assert_eq!(packet.decision.verdict, "BLOCKED");
    assert_eq!(packet.decision.pass_gate_count, 4);
    let assessment = assess_packet(&packet, "");
    assert!(!assessment.ready_for_challenge);
    assert_eq!(
        assessment.blockers,
        ["gate:github-actions:BLOCKED_EXTERNAL"]
    );
}

#[test]
fn zero_step_pass_and_harness_claim_mismatch_are_rejected() {
    let mut zero = Fixture::pass();
    replace_gate(
        &mut zero,
        "linux-x86_64",
        "PASS",
        true,
        true,
        false,
        false,
        0,
        false,
    );
    assert!(zero.validate().is_err());

    let mut harness = Fixture::pass();
    replace_gate(
        &mut harness,
        "nix-x86_64-linux",
        "PASS",
        true,
        true,
        false,
        true,
        1,
        false,
    );
    assert!(harness.validate().is_err());
}

#[test]
fn manifest_digest_assertion_and_extra_file_drift_are_rejected() {
    let fixture = Fixture::pass();
    let mut bad_digest = fixture.spec.clone();
    bad_digest.platform_gates[0].receipt.manifest_layers[0].manifest_sha256 = "0".repeat(64);
    assert!(validate(&fixture, &bad_digest).is_err());

    let mut bad_assertion = fixture.spec.clone();
    bad_assertion.platform_gates[0].receipt.artifacts[0].assertions[0].expected =
        Value::String("wrong".to_string());
    assert!(validate(&fixture, &bad_assertion).is_err());

    let root = Path::new(&fixture.spec.platform_gates[0].receipt.receipt_root);
    write_private(&root.join("unsealed-extra.txt"), b"extra");
    assert!(fixture.validate().is_err());
}

#[cfg(unix)]
#[test]
fn symlink_and_hardlink_receipt_entries_are_rejected() {
    use std::os::unix::fs::symlink;

    let fixture = Fixture::pass();
    let root = Path::new(&fixture.spec.platform_gates[0].receipt.receipt_root);
    symlink(root.join("status.json"), root.join("linked-status.json")).expect("symlink");
    assert!(fixture.validate().is_err());

    fs::remove_file(root.join("linked-status.json")).expect("remove symlink");
    fs::hard_link(root.join("status.json"), root.join("hard-status.json")).expect("hardlink");
    assert!(fixture.validate().is_err());
}

#[test]
fn nested_attempt_requires_and_verifies_inner_receipt_manifest() {
    let mut fixture = Fixture::pass();
    let gate_index = fixture
        .spec
        .platform_gates
        .iter()
        .position(|gate| gate.gate == "windows-x86_64-native")
        .expect("windows gate");
    let old_root = PathBuf::from(&fixture.spec.platform_gates[gate_index].receipt.receipt_root);
    let nested_root = fixture.receipts_parent.join("windows-nested");
    let status = gate_status(&fixture.candidate, "PASS", true, true, false, false, 5);
    let receipt = nested_receipt(&nested_root, "synthetic_gate_v1", status);
    fixture.spec.platform_gates[gate_index].receipt = receipt;
    fs::remove_dir_all(old_root).expect("remove replaced root");
    fixture.validate().expect("nested attempt");

    let mut missing_layer = fixture.spec.clone();
    missing_layer.platform_gates[gate_index]
        .receipt
        .manifest_layers
        .pop();
    assert!(validate(&fixture, &missing_layer).is_err());

    write_private(&nested_root.join("receipt/drift.txt"), b"drift");
    assert!(fixture.validate().is_err());
}

#[test]
fn builder_is_plan_only_by_default_one_shot_and_self_verifying() {
    let fixture = Fixture::pass();
    let temporary_root = fixture.receipts_parent.parent().expect("temporary root");
    let spec_path = temporary_root.join("build-spec.json");
    let spec_bytes = canonical_json(&fixture.spec).expect("canonical spec");
    write_private(&spec_path, &spec_bytes);
    let spec_sha256 = sha256(&spec_bytes);
    let output = fixture.receipts_parent.join("synthetic-aggregate-deadbeef");

    let plan = plan_for_test(
        &spec_path,
        &spec_sha256,
        &output,
        &fixture.receipts_parent,
        &fixture.candidate,
        "synthetic-aggregate",
    )
    .expect("plan");
    assert!(plan.execute_required);
    assert!(plan.ready_for_challenge);
    assert!(!output.exists());

    let sealed = build_for_test(
        &spec_path,
        &spec_sha256,
        &output,
        &fixture.receipts_parent,
        &fixture.candidate,
        "synthetic-aggregate",
    )
    .expect("build synthetic aggregate");
    assert_eq!(sealed.aggregate_manifest_entry_count, 4);
    assert!(sealed.assessment.ready_for_challenge);
    verify_for_test(
        &output,
        &sealed.aggregate_manifest_sha256,
        &fixture.receipts_parent,
        &fixture.candidate,
        "synthetic-aggregate",
    )
    .expect("verify synthetic aggregate");

    assert!(
        build_for_test(
            &spec_path,
            &spec_sha256,
            &output,
            &fixture.receipts_parent,
            &fixture.candidate,
            "synthetic-aggregate",
        )
        .is_err()
    );

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(
            output.join("qualification-packet.json"),
            fs::Permissions::from_mode(0o600),
        )
        .expect("tamper mode");
    }
    assert!(
        verify_for_test(
            &output,
            &sealed.aggregate_manifest_sha256,
            &fixture.receipts_parent,
            &fixture.candidate,
            "synthetic-aggregate",
        )
        .is_err()
    );
}

#[cfg(unix)]
#[test]
fn sealed_mode_manifest_is_required_and_verified_when_present() {
    use std::os::unix::fs::PermissionsExt;

    let mut fixture = Fixture::pass();
    let gate = fixture
        .spec
        .platform_gates
        .iter_mut()
        .find(|gate| gate.gate == "linux-x86_64")
        .expect("linux gate");
    let root = PathBuf::from(&gate.receipt.receipt_root);
    fs::remove_dir_all(&root).expect("remove old gate");
    let status = gate_status(&fixture.candidate, "PASS", true, true, false, false, 43);
    gate.receipt = direct_receipt_with_modes(&root, "synthetic_gate_v1", status);
    fixture.validate().expect("mode-bound receipt");

    let mut omitted = fixture.spec.clone();
    omitted
        .platform_gates
        .iter_mut()
        .find(|gate| gate.gate == "linux-x86_64")
        .expect("linux gate")
        .receipt
        .mode_manifest = None;
    assert!(validate(&fixture, &omitted).is_err());

    fs::set_permissions(root.join("status.json"), fs::Permissions::from_mode(0o400))
        .expect("tamper mode");
    assert!(fixture.validate().is_err());
}

fn validate(
    fixture: &Fixture,
    spec: &AggregateBuildSpecV3,
) -> Result<super::AggregateQualificationPacketV3, crate::AcceptanceError> {
    validate_spec(
        spec,
        ValidationPolicy {
            expected_candidate: &fixture.candidate,
            receipts_parent: &fixture.receipts_parent,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn replace_gate(
    fixture: &mut Fixture,
    gate: &str,
    status: &str,
    pass: bool,
    qualification: bool,
    candidate_failure: bool,
    harness_failure: bool,
    steps: u64,
    refs_changed: bool,
) {
    let input = fixture
        .spec
        .platform_gates
        .iter_mut()
        .find(|input| input.gate == gate)
        .expect("gate");
    let root = PathBuf::from(&input.receipt.receipt_root);
    fs::remove_dir_all(&root).expect("remove old gate");
    input.receipt = direct_receipt(
        &root,
        "synthetic_gate_v1",
        gate_status(
            &fixture.candidate,
            status,
            pass,
            qualification,
            candidate_failure,
            harness_failure,
            steps,
        )
        .with_refs_changed(refs_changed),
        None,
    );
}

trait StatusMutation {
    fn with_refs_changed(self, changed: bool) -> Value;
}

impl StatusMutation for Value {
    fn with_refs_changed(mut self, changed: bool) -> Value {
        self["refs_changed"] = Value::Bool(changed);
        self
    }
}

fn gate_status(
    candidate: &CandidateBindingV3,
    status: &str,
    pass: bool,
    qualification: bool,
    candidate_failure: bool,
    harness_failure: bool,
    steps: u64,
) -> Value {
    json!({
        "candidate_executed": steps > 0,
        "candidate_failure": candidate_failure,
        "candidate_head": candidate.head,
        "candidate_parent": candidate.parents[0],
        "candidate_tree": candidate.tree,
        "executed_steps": steps,
        "harness_failure": harness_failure,
        "pass": pass,
        "production_changed": false,
        "qualification": qualification,
        "refs_changed": false,
        "schema": "synthetic_gate_v1",
        "status": status,
        "upstream_cutoff": candidate.upstream_cutoff,
    })
}

fn prerequisite_status(candidate: &CandidateBindingV3) -> Value {
    json!({
        "candidate_head": candidate.head,
        "candidate_parent": candidate.parents[0],
        "candidate_tree": candidate.tree,
        "pass": true,
        "production_changed": false,
        "refs_changed": false,
        "schema": "synthetic_prerequisite_v1",
        "status": "PASS",
        "upstream_cutoff": candidate.upstream_cutoff,
    })
}

fn direct_receipt(
    root: &Path,
    schema: &str,
    status: Value,
    extra: Option<(&str, &[u8])>,
) -> ReceiptEvidenceBindingV3 {
    fs::create_dir(root).expect("receipt root");
    private_dir(root);
    let status_bytes = canonical_json(&status).expect("canonical status");
    write_private(&root.join("status.json"), &status_bytes);
    if let Some((relative, bytes)) = extra {
        write_private(&root.join(relative), bytes);
    }
    let manifest_bytes = seal_manifest(root, "SHA256SUMS");
    receipt_binding(
        root,
        schema,
        vec![ManifestLayerBindingV3 {
            manifest_entry_count: count_manifest_entries(&manifest_bytes),
            manifest_relative_path: "SHA256SUMS".to_string(),
            manifest_root_kind: ManifestRootKindV3::Sha256ManifestFullInventoryV1,
            manifest_sha256: sha256(&manifest_bytes),
            root_relative_path: ".".to_string(),
        }],
        &status,
        "status.json",
        &status_bytes,
    )
}

fn nested_receipt(root: &Path, schema: &str, status: Value) -> ReceiptEvidenceBindingV3 {
    fs::create_dir(root).expect("attempt root");
    private_dir(root);
    let inner = root.join("receipt");
    fs::create_dir(&inner).expect("inner root");
    private_dir(&inner);
    let status_bytes = canonical_json(&status).expect("canonical status");
    write_private(&inner.join("status.json"), &status_bytes);
    let inner_manifest = seal_manifest(&inner, "SHA256SUMS");
    let outer_manifest = seal_manifest(root, "ATTEMPT.sha256");
    receipt_binding(
        root,
        schema,
        vec![
            ManifestLayerBindingV3 {
                manifest_entry_count: count_manifest_entries(&outer_manifest),
                manifest_relative_path: "ATTEMPT.sha256".to_string(),
                manifest_root_kind: ManifestRootKindV3::Sha256ManifestFullInventoryV1,
                manifest_sha256: sha256(&outer_manifest),
                root_relative_path: ".".to_string(),
            },
            ManifestLayerBindingV3 {
                manifest_entry_count: count_manifest_entries(&inner_manifest),
                manifest_relative_path: "SHA256SUMS".to_string(),
                manifest_root_kind: ManifestRootKindV3::Sha256ManifestFullInventoryV1,
                manifest_sha256: sha256(&inner_manifest),
                root_relative_path: "receipt".to_string(),
            },
        ],
        &status,
        "receipt/status.json",
        &status_bytes,
    )
}

fn direct_receipt_with_modes(root: &Path, schema: &str, status: Value) -> ReceiptEvidenceBindingV3 {
    fs::create_dir(root).expect("receipt root");
    private_dir(root);
    let status_bytes = canonical_json(&status).expect("canonical status");
    write_private(&root.join("status.json"), &status_bytes);
    let mode_bytes = b"600\tstatus.json\n";
    write_private(&root.join("MODES.tsv"), mode_bytes);
    let manifest_bytes = seal_manifest(root, "SHA256SUMS");
    let mut receipt = receipt_binding(
        root,
        schema,
        vec![ManifestLayerBindingV3 {
            manifest_entry_count: count_manifest_entries(&manifest_bytes),
            manifest_relative_path: "SHA256SUMS".to_string(),
            manifest_root_kind: ManifestRootKindV3::Sha256ManifestFullInventoryV1,
            manifest_sha256: sha256(&manifest_bytes),
            root_relative_path: ".".to_string(),
        }],
        &status,
        "status.json",
        &status_bytes,
    );
    receipt.mode_manifest = Some(ModeManifestBindingV3 {
        format: ModeManifestFormatV3::PosixModePathTsvV1,
        relative_path: "MODES.tsv".to_string(),
        sha256: sha256(mode_bytes),
    });
    receipt
}

fn receipt_binding(
    root: &Path,
    schema: &str,
    layers: Vec<ManifestLayerBindingV3>,
    status: &Value,
    relative_path: &str,
    status_bytes: &[u8],
) -> ReceiptEvidenceBindingV3 {
    let mut assertions = status
        .as_object()
        .expect("status object")
        .iter()
        .map(|(key, expected)| ArtifactAssertionV3 {
            expected: expected.clone(),
            selector: format!("/{key}"),
        })
        .collect::<Vec<_>>();
    assertions.sort_by(|left, right| left.selector.cmp(&right.selector));
    let mut semantic_claims = vec![
        claim(SemanticClaimV3::CandidateHead, "/candidate_head"),
        claim(SemanticClaimV3::CandidateParent, "/candidate_parent"),
        claim(SemanticClaimV3::CandidateTree, "/candidate_tree"),
        claim(SemanticClaimV3::Pass, "/pass"),
        claim(SemanticClaimV3::ProductionChanged, "/production_changed"),
        claim(SemanticClaimV3::RefsChanged, "/refs_changed"),
        claim(SemanticClaimV3::Schema, "/schema"),
        claim(SemanticClaimV3::Status, "/status"),
        claim(SemanticClaimV3::UpstreamCutoff, "/upstream_cutoff"),
    ];
    if status.get("candidate_executed").is_some() {
        semantic_claims.extend([
            claim(SemanticClaimV3::CandidateExecuted, "/candidate_executed"),
            claim(SemanticClaimV3::CandidateFailure, "/candidate_failure"),
            claim(SemanticClaimV3::ExecutedSteps, "/executed_steps"),
            claim(SemanticClaimV3::HarnessFailure, "/harness_failure"),
            claim(SemanticClaimV3::Qualification, "/qualification"),
        ]);
    }
    semantic_claims.sort_by_key(|binding| binding.claim);
    ReceiptEvidenceBindingV3 {
        artifacts: vec![EvidenceArtifactBindingV3 {
            assertions,
            format: EvidenceArtifactFormatV3::CanonicalJsonV1,
            id: "status".to_string(),
            relative_path: relative_path.to_string(),
            sha256: sha256(status_bytes),
        }],
        expected_schema: schema.to_string(),
        manifest_layers: layers,
        mode_manifest: None,
        receipt_root: root.to_str().expect("UTF-8 root").to_string(),
        semantic_claims,
    }
}

fn claim(claim: SemanticClaimV3, selector: &str) -> SemanticClaimBindingV3 {
    SemanticClaimBindingV3 {
        artifact_id: "status".to_string(),
        claim,
        selector: selector.to_string(),
    }
}

fn seal_manifest(root: &Path, name: &str) -> Vec<u8> {
    let mut paths = Vec::new();
    collect_files(root, root, name, &mut paths);
    paths.sort();
    let lines = paths
        .iter()
        .map(|relative| {
            let bytes = fs::read(root.join(relative)).expect("manifest input");
            format!("{}  ./{relative}\n", sha256(&bytes))
        })
        .collect::<String>()
        .into_bytes();
    write_private(&root.join(name), &lines);
    lines
}

fn collect_files(root: &Path, directory: &Path, manifest_name: &str, output: &mut Vec<String>) {
    for entry in fs::read_dir(directory).expect("read receipt") {
        let entry = entry.expect("receipt entry");
        let path = entry.path();
        let relative = path
            .strip_prefix(root)
            .expect("relative")
            .to_str()
            .expect("UTF-8")
            .to_string();
        if path.is_dir() {
            collect_files(root, &path, manifest_name, output);
        } else if relative != manifest_name {
            output.push(relative);
        }
    }
}

fn count_manifest_entries(bytes: &[u8]) -> usize {
    bytes.iter().filter(|byte| **byte == b'\n').count()
}

fn write_private(path: &Path, bytes: &[u8]) {
    fs::write(path, bytes).expect("write fixture");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600)).expect("private file");
    }
}

fn private_dir(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("private dir");
    }
}
