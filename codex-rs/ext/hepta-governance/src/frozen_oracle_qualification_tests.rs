use std::collections::BTreeSet;
use std::process::Command;
use std::sync::Arc;

use codex_extension_api::ExtensionData;
use codex_extension_api::ToolCallOutcome;
use codex_extension_api::ToolCallSource;
use codex_extension_api::ToolPayload;
use codex_extension_api::ToolPolicyContributor;
use codex_extension_api::ToolPolicyDecision;
use codex_extension_api::ToolPolicyInput;
use codex_extension_api::ToolPolicyTerminalInput;
use codex_hepta_contracts::ActionId;
use codex_hepta_contracts::GovernanceMode;
use codex_hepta_contracts::ReceiptId;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_evidence::AppendDisposition;
use codex_hepta_evidence::FrozenOracleConformanceStatus;
use codex_hepta_evidence::HeptaEvidenceStore;
use codex_hepta_evidence::VerifiedFrozenOracleCorpus;
use codex_hepta_evidence::pinned_frozen_oracle_corpus_bytes;
use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use tempfile::TempDir;

use super::GovernanceState;
use super::HeptaGovernanceExtension;

const GENERATOR_SOURCE_BYTES: &[u8] =
    include_bytes!("../fixtures/frozen_oracle_conformance_2f704_generator.rs.txt");
const CORPUS_SHA256: &str = "6fbf5ef6eba851554f1c342fc6e262ff17c51dc58033ed2ca5d86dcafb7d804f";
const ORACLE_COMMIT: &str = "2f704dc7c1172cefca908852456beccf4d02a5d1";
const ORACLE_TREE: &str = "7be9a382b2610790838eef874cb4d381b5025490";
const ORACLE_MANIFEST_SHA256: &str =
    "2c82d45303e912b92a7b9ac31da4661197e59a5ca415d3c70375b49169691377";
const PAYLOAD_KINDS: [&str; 3] = ["function", "tool_search", "custom"];
const RECEIPT_PHASES: [&str; 2] = ["admission_only", "admission_and_authorization"];
const SOURCE_KINDS: [&str; 3] = ["direct", "direct_plaintext_message", "code_mode"];
const OUTCOME_KINDS: [&str; 7] = [
    "completed_success",
    "completed_reported_failure",
    "blocked",
    "failed_before_handler",
    "failed_after_handler",
    "aborted",
    "indeterminate",
];
const HOST_ACCEPTED_VALUES: [bool; 2] = [false, true];
const CASE_COUNT: usize = 252;

const COMPILED_ARTIFACT_DIR: Option<&str> =
    option_env!("HEPTA_FROZEN_ORACLE_QUALIFICATION_ARTIFACT_DIR");
const COMPILED_CANDIDATE_COMMIT: Option<&str> = option_env!("HEPTA_FROZEN_ORACLE_CANDIDATE_COMMIT");
const COMPILED_CANDIDATE_TREE: Option<&str> = option_env!("HEPTA_FROZEN_ORACLE_CANDIDATE_TREE");
const COMPILED_QUALIFICATION_NONCE: Option<&str> =
    option_env!("HEPTA_FROZEN_ORACLE_QUALIFICATION_NONCE_SHA256");
const COMPILED_EXECUTION_ORIGIN: Option<&str> = option_env!("HEPTA_FROZEN_ORACLE_EXECUTION_ORIGIN");
const COMPILED_SOURCE_ROOT: Option<&str> = option_env!("HEPTA_FROZEN_ORACLE_SOURCE_ROOT");
const COMPILED_SOURCE_MANIFEST_PATH: Option<&str> =
    option_env!("HEPTA_FROZEN_ORACLE_SOURCE_MANIFEST_PATH");
const COMPILED_SOURCE_MANIFEST_SHA256: Option<&str> =
    option_env!("HEPTA_FROZEN_ORACLE_SOURCE_MANIFEST_SHA256");

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CaseKey {
    payload_kind: String,
    receipt_phase: String,
    source_kind: String,
    outcome_kind: String,
    host_accepted: bool,
}

struct ComparedCase {
    ordinal: u64,
    key: CaseKey,
    sample_id_sha256: Sha256Digest,
    candidate_output_sha256: Sha256Digest,
    canonical_oracle_output_sha256: Sha256Digest,
    candidate_projection: serde_json::Value,
    canonical_oracle_projection: serde_json::Value,
}

struct CandidateIdentity {
    candidate_commit: String,
    candidate_tree: String,
    qualification_nonce_sha256: Sha256Digest,
    provenance: serde_json::Value,
}

fn canonical_json(value: &serde_json::Value) -> Vec<u8> {
    fn sort(value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Array(items) => {
                for item in items {
                    sort(item);
                }
            }
            serde_json::Value::Object(map) => {
                let mut entries = std::mem::take(map).into_iter().collect::<Vec<_>>();
                entries.sort_by(|(left, _), (right, _)| left.cmp(right));
                for (_, item) in &mut entries {
                    sort(item);
                }
                map.extend(entries);
            }
            _ => {}
        }
    }

    let mut value = value.clone();
    sort(&mut value);
    serde_json::to_vec(&value).expect("canonical frozen-oracle extension callback JSON")
}

fn field<'a>(value: &'a serde_json::Value, name: &str) -> &'a serde_json::Value {
    value
        .get(name)
        .unwrap_or_else(|| panic!("frozen-oracle corpus is missing {name}"))
}

fn string_field<'a>(value: &'a serde_json::Value, name: &str) -> &'a str {
    field(value, name)
        .as_str()
        .unwrap_or_else(|| panic!("frozen-oracle corpus field {name} is not a string"))
}

fn assert_keys(value: &serde_json::Value, expected: &[&str], label: &str) {
    let actual = value
        .as_object()
        .unwrap_or_else(|| panic!("{label} is not an object"))
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actual,
        expected.iter().copied().collect(),
        "{label} keys changed"
    );
}

fn payload(payload_kind: &str) -> ToolPayload {
    match payload_kind {
        "function" => ToolPayload::Function {
            arguments: r#"{"command":"echo canonical"}"#.to_string(),
        },
        "tool_search" => ToolPayload::ToolSearch {
            arguments: serde_json::from_value(serde_json::json!({
                "query": "hepta canonical payload",
                "limit": 3,
            }))
            .expect("tool-search arguments"),
        },
        "custom" => ToolPayload::Custom {
            input: "canonical custom input".to_string(),
        },
        other => panic!("unknown payload kind {other}"),
    }
}

fn source(source_kind: &str) -> ToolCallSource {
    match source_kind {
        "direct" => ToolCallSource::Direct,
        "direct_plaintext_message" => ToolCallSource::DirectPlaintextMessage,
        "code_mode" => ToolCallSource::CodeMode {
            cell_id: "cell-oracle".to_string(),
            runtime_tool_call_id: "runtime-oracle".to_string(),
        },
        other => panic!("unknown source kind {other}"),
    }
}

fn outcome(outcome_kind: &str) -> ToolCallOutcome {
    match outcome_kind {
        "completed_success" => ToolCallOutcome::Completed { success: true },
        "completed_reported_failure" => ToolCallOutcome::Completed { success: false },
        "blocked" => ToolCallOutcome::Blocked,
        "failed_before_handler" => ToolCallOutcome::Failed {
            handler_executed: false,
        },
        "failed_after_handler" => ToolCallOutcome::Failed {
            handler_executed: true,
        },
        "aborted" => ToolCallOutcome::Aborted,
        "indeterminate" => ToolCallOutcome::Indeterminate {
            reason_code: "canonical_oracle_indeterminate",
        },
        other => panic!("unknown outcome kind {other}"),
    }
}

fn decision_projection(decision: ToolPolicyDecision) -> serde_json::Value {
    match decision {
        ToolPolicyDecision::Allow => serde_json::json!({"decision": "allow"}),
        ToolPolicyDecision::Block {
            reason_code,
            message,
        } => serde_json::json!({
            "decision": "block",
            "reason_code": reason_code,
            "message": message,
        }),
    }
}

/// Exercises only the extension callback semantic surface.
///
/// This directly calls the contributor, bypassing ToolRegistry, host
/// attempt-state transitions, and real dispatch reachability. Those product
/// path properties are not part of this conformance gate.
async fn extension_callback_semantic_projection(
    evidence: &Arc<HeptaEvidenceStore>,
    ordinal: u64,
    key: &CaseKey,
) -> serde_json::Value {
    let session = ExtensionData::new("session-oracle");
    let thread = ExtensionData::new("thread-oracle");
    let turn = ExtensionData::new("turn-oracle");
    thread.insert(GovernanceState::enabled(
        GovernanceMode::Shadow,
        Ok(Arc::clone(evidence)),
    ));
    let extension = HeptaGovernanceExtension {
        enabled: |_: &()| true,
        mode: GovernanceMode::Shadow,
        state_db: None,
        evidence: tokio::sync::OnceCell::new(),
    };
    let payload = payload(&key.payload_kind);
    let source = source(&key.source_kind);
    let call_id = format!("frozen-oracle-case-{ordinal:03}");
    let attempt_id = format!("frozen-oracle-attempt-{ordinal:03}");
    let tool_name = "exec_command".into();
    let input = || ToolPolicyInput {
        session_store: &session,
        thread_store: &thread,
        turn_store: &turn,
        attempt_id: &attempt_id,
        turn_id: "turn-oracle",
        call_id: &call_id,
        tool_name: &tool_name,
        source: source.clone(),
        payload: &payload,
    };
    let admission = extension
        .admit(input())
        .await
        .expect("extension admission callback");
    let authorization = match key.receipt_phase.as_str() {
        "admission_only" => None,
        "admission_and_authorization" => Some(
            extension
                .authorize(input())
                .await
                .expect("extension authorization callback"),
        ),
        other => panic!("unknown receipt phase {other}"),
    };
    extension
        .on_terminal(ToolPolicyTerminalInput {
            session_store: &session,
            thread_store: &thread,
            turn_store: &turn,
            attempt_id: &attempt_id,
            turn_id: "turn-oracle",
            call_id: &call_id,
            tool_name: &tool_name,
            source,
            outcome: outcome(&key.outcome_kind),
            host_accepted: key.host_accepted,
        })
        .await
        .expect("extension terminal callback in Shadow mode");
    let action_id = ActionId::for_tool_call("thread-oracle", "turn-oracle", &call_id);
    let stored = evidence
        .get_action_evidence(&action_id)
        .await
        .expect("read durable extension callback evidence");
    let durable_receipt = stored.receipt.as_ref().map(|stored_receipt| {
        let receipt_id = ReceiptId::for_action(&action_id);
        assert_eq!(stored_receipt.receipt.receipt_id, receipt_id);
        serde_json::to_value(&stored_receipt.receipt).expect("serialize durable governance receipt")
    });
    serde_json::json!({
        "admission_decision": decision_projection(admission),
        "authorization_decision": authorization.map(decision_projection),
        "terminal_callback": "ok",
        "durable_admission": stored.admission,
        "durable_authorization": stored.authorization,
        "durable_receipt": durable_receipt,
    })
}

fn expected_keys() -> BTreeSet<CaseKey> {
    let mut expected = BTreeSet::new();
    for payload_kind in PAYLOAD_KINDS {
        for receipt_phase in RECEIPT_PHASES {
            for source_kind in SOURCE_KINDS {
                for outcome_kind in OUTCOME_KINDS {
                    for host_accepted in HOST_ACCEPTED_VALUES {
                        expected.insert(CaseKey {
                            payload_kind: payload_kind.to_string(),
                            receipt_phase: receipt_phase.to_string(),
                            source_kind: source_kind.to_string(),
                            outcome_kind: outcome_kind.to_string(),
                            host_accepted,
                        });
                    }
                }
            }
        }
    }
    assert_eq!(expected.len(), CASE_COUNT);
    expected
}

fn exact_env(name: &str, compiled: Option<&str>) -> String {
    let runtime = std::env::var(name)
        .unwrap_or_else(|error| panic!("exact qualification run requires {name}: {error}"));
    let compiled =
        compiled.unwrap_or_else(|| panic!("exact qualification binary lacks compile-time {name}"));
    assert_eq!(
        runtime, compiled,
        "compile-time and runtime values differ for {name}"
    );
    runtime
}

fn command_output(program: &str, args: &[&str]) -> String {
    let output = Command::new(program)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("run {program}: {error}"));
    assert!(
        output.status.success(),
        "{program} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("command output is UTF-8")
        .trim()
        .to_string()
}

fn candidate_identity(exact_artifact_mode: bool) -> CandidateIdentity {
    if !exact_artifact_mode {
        return CandidateIdentity {
            candidate_commit: "1111111111111111111111111111111111111111".to_string(),
            candidate_tree: "2222222222222222222222222222222222222222".to_string(),
            qualification_nonce_sha256: Sha256Digest::for_bytes(
                b"fixture-frozen-oracle-qualification-nonce-v2",
            ),
            provenance: serde_json::json!({
                "identity_status": "identity_claim",
                "identity_claim_kind": "synthetic_test_fixture",
                "exact_verified": false,
                "external_identity_upgrade_required": true,
                "unverified_claim_digests": {
                    "source_manifest_sha256": serde_json::Value::Null,
                    "binary_sha256": serde_json::Value::Null,
                    "driver_sha256": serde_json::Value::Null,
                    "command_sha256": serde_json::Value::Null,
                },
            }),
        };
    }

    let candidate_commit = exact_env(
        "HEPTA_FROZEN_ORACLE_CANDIDATE_COMMIT",
        COMPILED_CANDIDATE_COMMIT,
    );
    let candidate_tree = exact_env(
        "HEPTA_FROZEN_ORACLE_CANDIDATE_TREE",
        COMPILED_CANDIDATE_TREE,
    );
    let qualification_nonce = exact_env(
        "HEPTA_FROZEN_ORACLE_QUALIFICATION_NONCE_SHA256",
        COMPILED_QUALIFICATION_NONCE,
    );
    let qualification_nonce_sha256 =
        Sha256Digest::parse(qualification_nonce).expect("qualification nonce SHA-256");
    let execution_origin = exact_env(
        "HEPTA_FROZEN_ORACLE_EXECUTION_ORIGIN",
        COMPILED_EXECUTION_ORIGIN,
    );
    let source_root = std::path::PathBuf::from(exact_env(
        "HEPTA_FROZEN_ORACLE_SOURCE_ROOT",
        COMPILED_SOURCE_ROOT,
    ));
    assert!(
        source_root.is_absolute(),
        "exact source root must be absolute"
    );
    let source_manifest_path = std::path::PathBuf::from(exact_env(
        "HEPTA_FROZEN_ORACLE_SOURCE_MANIFEST_PATH",
        COMPILED_SOURCE_MANIFEST_PATH,
    ));
    assert!(
        source_manifest_path.is_absolute(),
        "source manifest path must be absolute"
    );
    let expected_manifest_sha256 = exact_env(
        "HEPTA_FROZEN_ORACLE_SOURCE_MANIFEST_SHA256",
        COMPILED_SOURCE_MANIFEST_SHA256,
    );
    Sha256Digest::parse(expected_manifest_sha256.clone()).expect("source manifest SHA-256");
    let manifest_bytes =
        std::fs::read(&source_manifest_path).expect("read outer source manifest bytes");
    let actual_manifest_sha256 = Sha256Digest::for_bytes(&manifest_bytes);
    assert_eq!(actual_manifest_sha256.as_str(), expected_manifest_sha256);
    let manifest: serde_json::Value =
        serde_json::from_slice(&manifest_bytes).expect("source manifest JSON");
    assert_eq!(
        string_field(&manifest, "candidate_commit"),
        candidate_commit
    );
    assert_eq!(string_field(&manifest, "candidate_tree"), candidate_tree);

    let git_observation = match execution_origin.as_str() {
        "working_tree" => {
            let root = source_root.to_str().expect("UTF-8 source root");
            let head = command_output("git", &["-C", root, "rev-parse", "HEAD"]);
            let tree = command_output("git", &["-C", root, "rev-parse", "HEAD^{tree}"]);
            let status = command_output(
                "git",
                &["-C", root, "status", "--porcelain", "--untracked-files=all"],
            );
            assert_eq!(head, candidate_commit);
            assert_eq!(tree, candidate_tree);
            assert!(status.is_empty(), "exact working tree is not clean");
            serde_json::json!({
                "head": head,
                "tree": tree,
                "clean": true,
            })
        }
        "clean_archive" => {
            assert!(
                !source_root.join(".git").exists(),
                "clean-archive execution must not depend on mutable Git metadata"
            );
            serde_json::json!({
                "head": serde_json::Value::Null,
                "tree": serde_json::Value::Null,
                "clean": true,
                "clean_archive_manifest_claim_observed": true,
            })
        }
        other => panic!("unsupported execution origin {other}"),
    };
    let binary_path = std::env::current_exe().expect("current qualification binary path");
    let binary_sha256 = Sha256Digest::for_bytes(
        &std::fs::read(&binary_path).expect("read current qualification binary"),
    );
    CandidateIdentity {
        candidate_commit: candidate_commit.clone(),
        candidate_tree: candidate_tree.clone(),
        qualification_nonce_sha256,
        provenance: serde_json::json!({
            "identity_status": "identity_claim",
            "exact_verified": false,
            "execution_origin": execution_origin,
            "compile_time_runtime_identity_equal": true,
            "candidate_commit": candidate_commit,
            "candidate_tree": candidate_tree,
            "source_root": source_root,
            "source_manifest": {
                "path": source_manifest_path,
                "sha256": actual_manifest_sha256.as_str(),
                "payload": manifest,
            },
            "git_observation": git_observation,
            "binary_provenance": {
                "path": binary_path,
                "sha256": binary_sha256.as_str(),
                "outer_verification_required": true,
            },
            "unverified_claim_digests": {
                "source_manifest_sha256": actual_manifest_sha256.as_str(),
                "binary_sha256": binary_sha256.as_str(),
                "driver_sha256": serde_json::Value::Null,
                "command_sha256": serde_json::Value::Null,
            },
            "external_identity_upgrade_requirements": {
                "rehash_full_source_manifest": true,
                "rebuild_in_isolated_environment": true,
                "bind_binary_driver_and_command": true,
                "independent_verifier_required": true,
                "status": "not_performed_in_process",
            },
        }),
    }
}

#[tokio::test]
async fn frozen_oracle_extension_callback_semantic_conformance_matches_pinned_oracle() {
    let corpus_bytes = pinned_frozen_oracle_corpus_bytes();
    let corpus_capability =
        VerifiedFrozenOracleCorpus::load(corpus_bytes).expect("pinned corpus capability");
    let corpus_sha256 = Sha256Digest::for_bytes(corpus_bytes);
    assert_eq!(corpus_sha256.as_str(), CORPUS_SHA256);
    let corpus: serde_json::Value =
        serde_json::from_slice(corpus_bytes).expect("frozen-oracle conformance corpus");
    assert_keys(
        &corpus,
        &[
            "canonical_encoding",
            "canonical_object",
            "cases",
            "formula",
            "generator",
            "identity_pattern",
            "live_product_shadow_collector_completed",
            "oracle_commit",
            "oracle_manifest_sha256",
            "oracle_tree",
            "schema",
            "scope",
        ],
        "corpus",
    );
    assert_eq!(
        string_field(&corpus, "schema"),
        "hepta_frozen_oracle_conformance_corpus_v2"
    );
    assert_eq!(
        string_field(&corpus, "scope"),
        "bounded_offline_frozen_oracle_qualification_conformance"
    );
    assert_eq!(
        field(&corpus, "live_product_shadow_collector_completed").as_bool(),
        Some(false)
    );
    assert_eq!(string_field(&corpus, "oracle_commit"), ORACLE_COMMIT);
    assert_eq!(string_field(&corpus, "oracle_tree"), ORACLE_TREE);
    assert_eq!(
        string_field(&corpus, "oracle_manifest_sha256"),
        ORACLE_MANIFEST_SHA256
    );
    assert_eq!(
        string_field(&corpus, "formula"),
        "3_payloads_x_2_receipt_phases_x_3_sources_x_7_outcomes_x_2_host_accepted"
    );
    let generator = field(&corpus, "generator");
    assert_keys(
        generator,
        &[
            "entrypoint",
            "lifecycle",
            "schema",
            "source_digest_computed_from_executing_bytes",
            "source_sha256",
            "version",
        ],
        "generator",
    );
    let tracked_generator_sha256 = Sha256Digest::for_bytes(GENERATOR_SOURCE_BYTES);
    assert_eq!(
        string_field(generator, "source_sha256"),
        tracked_generator_sha256.as_str()
    );
    assert_eq!(
        field(generator, "source_digest_computed_from_executing_bytes").as_bool(),
        Some(true)
    );

    let artifact_dir = std::env::var_os("HEPTA_FROZEN_ORACLE_QUALIFICATION_ARTIFACT_DIR");
    if let Some(runtime_artifact_dir) = artifact_dir.as_ref() {
        let runtime = runtime_artifact_dir.to_string_lossy();
        let compiled = COMPILED_ARTIFACT_DIR
            .expect("exact qualification binary lacks compile-time artifact directory");
        assert_eq!(runtime, compiled);
    }
    let temp = artifact_dir
        .is_none()
        .then(|| TempDir::new().expect("temporary conformance directory"));
    let home = if let Some(artifact_dir) = artifact_dir.as_ref() {
        let path = std::path::PathBuf::from(artifact_dir);
        if path.exists() {
            assert!(
                std::fs::read_dir(&path)
                    .expect("read exact artifact directory")
                    .next()
                    .is_none(),
                "exact artifact directory must be empty"
            );
        } else {
            std::fs::create_dir_all(&path).expect("create exact artifact directory");
        }
        path
    } else {
        temp.as_ref()
            .expect("temporary conformance directory")
            .path()
            .to_path_buf()
    };
    let identity = candidate_identity(artifact_dir.is_some());
    let sqlite = SqliteConfig::new_for_testing(
        AbsolutePathBuf::try_from(home.clone()).expect("absolute conformance home"),
    );
    let first = Arc::new(
        HeptaEvidenceStore::open(&sqlite)
            .await
            .expect("candidate lifecycle evidence store"),
    );
    let oracle_cases = field(&corpus, "cases")
        .as_array()
        .expect("frozen-oracle cases");
    assert_eq!(oracle_cases.len(), CASE_COUNT);
    let expected_key_set = expected_keys();
    let mut actual_key_set = BTreeSet::new();
    let mut sample_ids = BTreeSet::new();
    let mut candidate_outputs = BTreeSet::new();
    let mut oracle_outputs = BTreeSet::new();
    let mut durable_receipt_count = 0_usize;
    let mut missing_receipt_count = 0_usize;
    let mut compared = Vec::with_capacity(CASE_COUNT);
    for (index, oracle_case) in oracle_cases.iter().enumerate() {
        assert_keys(
            oracle_case,
            &[
                "dimensions",
                "expected_lifecycle_projection",
                "expected_output_sha256",
                "ordinal",
                "sample_id_sha256",
            ],
            "case",
        );
        let ordinal = field(oracle_case, "ordinal")
            .as_u64()
            .expect("case ordinal");
        assert_eq!(ordinal, index as u64 + 1);
        let dimensions = field(oracle_case, "dimensions");
        assert_keys(
            dimensions,
            &[
                "host_accepted",
                "outcome_kind",
                "payload_kind",
                "receipt_phase",
                "source_kind",
            ],
            "case dimensions",
        );
        let key = CaseKey {
            payload_kind: string_field(dimensions, "payload_kind").to_string(),
            receipt_phase: string_field(dimensions, "receipt_phase").to_string(),
            source_kind: string_field(dimensions, "source_kind").to_string(),
            outcome_kind: string_field(dimensions, "outcome_kind").to_string(),
            host_accepted: field(dimensions, "host_accepted")
                .as_bool()
                .expect("host_accepted dimension"),
        };
        assert!(actual_key_set.insert(key.clone()));
        let sample_label = format!(
            "hepta-frozen-oracle-conformance:v2:{}:{}:{}:{}:{}",
            key.payload_kind,
            key.receipt_phase,
            key.source_kind,
            key.outcome_kind,
            key.host_accepted
        );
        let sample_id_sha256 = Sha256Digest::for_bytes(sample_label.as_bytes());
        assert_eq!(
            sample_id_sha256.as_str(),
            string_field(oracle_case, "sample_id_sha256")
        );
        assert!(sample_ids.insert(sample_id_sha256.as_str().to_string()));
        let canonical_oracle_projection =
            field(oracle_case, "expected_lifecycle_projection").clone();
        let canonical_oracle_output_sha256 =
            Sha256Digest::for_bytes(&canonical_json(&canonical_oracle_projection));
        assert_eq!(
            canonical_oracle_output_sha256.as_str(),
            string_field(oracle_case, "expected_output_sha256")
        );
        assert!(oracle_outputs.insert(canonical_oracle_output_sha256.as_str().to_string()));
        let has_receipt = !canonical_oracle_projection["durable_receipt"].is_null();
        let execution_receipt_requires_authorized_acceptance = matches!(
            key.outcome_kind.as_str(),
            "completed_success" | "completed_reported_failure" | "failed_after_handler"
        );
        let expected_receipt = !execution_receipt_requires_authorized_acceptance
            || (key.host_accepted && key.receipt_phase == "admission_and_authorization");
        assert_eq!(has_receipt, expected_receipt);
        if has_receipt {
            durable_receipt_count += 1;
            assert_eq!(
                canonical_oracle_projection["durable_receipt"]["host_accepted"].as_bool(),
                Some(key.host_accepted)
            );
        } else {
            missing_receipt_count += 1;
        }
        assert_eq!(
            canonical_oracle_projection["durable_authorization"].is_null(),
            key.receipt_phase == "admission_only"
        );
        if key.outcome_kind == "aborted" {
            let outcome = &canonical_oracle_projection["durable_receipt"]["outcome"];
            if key.receipt_phase == "admission_only" {
                assert_eq!(outcome["outcome"], "aborted");
            } else {
                assert_eq!(outcome["outcome"], "indeterminate");
                assert_eq!(outcome["reason_code"], "cancelled_after_authorization");
            }
        }
        let candidate_projection =
            extension_callback_semantic_projection(&first, ordinal, &key).await;
        let candidate_output_sha256 =
            Sha256Digest::for_bytes(&canonical_json(&candidate_projection));
        assert!(candidate_outputs.insert(candidate_output_sha256.as_str().to_string()));
        compared.push(ComparedCase {
            ordinal,
            key,
            sample_id_sha256,
            candidate_output_sha256,
            canonical_oracle_output_sha256,
            candidate_projection,
            canonical_oracle_projection,
        });
    }
    assert_eq!(actual_key_set, expected_key_set);
    assert_eq!(sample_ids.len(), CASE_COUNT);
    assert_eq!(candidate_outputs.len(), CASE_COUNT);
    assert_eq!(oracle_outputs.len(), CASE_COUNT);
    assert_eq!(durable_receipt_count, 171);
    assert_eq!(missing_receipt_count, 81);

    let plan = corpus_capability
        .qualification_plan(
            identity.candidate_commit.clone(),
            identity.candidate_tree.clone(),
            identity.qualification_nonce_sha256.clone(),
        )
        .expect("capability-derived finite qualification plan");
    let registration = first
        .begin_frozen_oracle_qualification_run(&corpus_capability, &plan)
        .await
        .expect("begin finite qualification run");
    assert_eq!(registration.disposition(), AppendDisposition::Inserted);
    for case in &compared[..CASE_COUNT / 2] {
        first
            .append_frozen_oracle_conformance_observation(
                registration.qualification_run_id(),
                &corpus_capability,
                case.ordinal,
                &case.candidate_projection,
            )
            .await
            .expect("append first-half conformance observation");
    }
    drop(first);
    let restarted = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("restart finite qualification run");
    let midpoint = restarted
        .get_frozen_oracle_qualification_run_summary(
            registration.qualification_run_id(),
            &corpus_capability,
        )
        .await
        .expect("midpoint summary")
        .expect("registered qualification run");
    assert_eq!(midpoint.observation_count(), (CASE_COUNT / 2) as u64);
    assert!(midpoint.terminal().is_none());
    for case in &compared[CASE_COUNT / 2..] {
        restarted
            .append_frozen_oracle_conformance_observation(
                registration.qualification_run_id(),
                &corpus_capability,
                case.ordinal,
                &case.candidate_projection,
            )
            .await
            .expect("append second-half conformance observation");
    }
    let terminal = restarted
        .finish_frozen_oracle_qualification_run(
            registration.qualification_run_id(),
            &corpus_capability,
        )
        .await
        .expect("finish finite qualification run");
    let summary = restarted
        .get_frozen_oracle_qualification_run_summary(
            registration.qualification_run_id(),
            &corpus_capability,
        )
        .await
        .expect("terminal summary")
        .expect("registered qualification run");
    let all_matched = compared.iter().all(|case| {
        case.candidate_output_sha256 == case.canonical_oracle_output_sha256
            && case.candidate_projection == case.canonical_oracle_projection
    });
    assert_eq!(
        terminal.terminal().conformance_status(),
        if all_matched {
            FrozenOracleConformanceStatus::Conformant
        } else {
            FrozenOracleConformanceStatus::Diverged
        }
    );

    if artifact_dir.is_some() {
        let comparisons = compared
            .iter()
            .map(|case| {
                serde_json::json!({
                    "ordinal": case.ordinal,
                    "dimensions": {
                        "payload_kind": case.key.payload_kind,
                        "receipt_phase": case.key.receipt_phase,
                        "source_kind": case.key.source_kind,
                        "outcome_kind": case.key.outcome_kind,
                        "host_accepted": case.key.host_accepted,
                    },
                    "sample_id_sha256": case.sample_id_sha256,
                    "candidate_output_sha256": case.candidate_output_sha256,
                    "canonical_oracle_output_sha256": case.canonical_oracle_output_sha256,
                    "canonical_oracle_matched": case.candidate_output_sha256
                        == case.canonical_oracle_output_sha256,
                    "extension_callback_projection_equal": case.candidate_projection
                        == case.canonical_oracle_projection,
                })
            })
            .collect::<Vec<_>>();
        let result = serde_json::json!({
            "schema": "hepta_frozen_oracle_extension_callback_semantic_conformance_result_v3",
            "scope": "bounded_offline_extension_callback_semantic_conformance",
            "identity": identity.provenance,
            "extension_callback_semantic_conformance_complete": true,
            "canonical_oracle_matched": all_matched,
            "qualification_run_timestamps_do_not_close_live_soak": true,
            "product_reachability": {
                "included_in_conformance_gate": false,
                "execution_model": "direct_extension_callback_invocation_bypasses_tool_registry_and_attempt_state",
                "tool_registry_exercised": false,
                "attempt_state_exercised": false,
                "real_dispatch_path_exercised": false,
                "gate_status": "not_executed",
            },
            "metadata_semantics": {
                "sqlite_seq": "non_authoritative_operational_metadata",
                "recorded_at_ms": "non_authoritative_operational_metadata",
            },
            "binding": {
                "candidate_commit": identity.candidate_commit,
                "candidate_tree": identity.candidate_tree,
                "frozen_oracle_commit": ORACLE_COMMIT,
                "frozen_oracle_tree": ORACLE_TREE,
                "frozen_oracle_manifest_sha256": ORACLE_MANIFEST_SHA256,
                "canonical_oracle_corpus_sha256": CORPUS_SHA256,
                "tracked_generator_source_sha256": tracked_generator_sha256,
                "qualification_nonce_sha256": identity.qualification_nonce_sha256,
                "qualification_run_started_at_ms": summary.binding().qualification_run_started_at_ms(),
            },
            "sample_count": CASE_COUNT,
            "restart_boundary_after_sample": CASE_COUNT / 2,
            "durable_receipt_count": durable_receipt_count,
            "invalid_host_outcome_combinations_without_receipt": missing_receipt_count,
            "comparisons": comparisons,
            "summary": summary,
            "live_product_shadow_remaining": {
                "collector_implemented": false,
                "real_product_traffic_observed": false,
                "duration_soak_completed": false,
                "canonical_oracle_comparison_attached_to_live_events": false,
                "status": "not_executed",
            },
            "authority": {
                "promotion": false,
                "enforce": false,
                "outbound": false,
                "memory_mutation": false,
                "proof": false,
                "retirement": false,
            },
        });
        std::fs::write(
            home.join("frozen-oracle-extension-callback-semantic-conformance-result.json"),
            serde_json::to_vec_pretty(&result).expect("serialize qualification result"),
        )
        .expect("write qualification result");
    }

    assert!(
        all_matched,
        "extension callback semantics diverged from the bounded frozen oracle; product reachability and live Shadow were not exercised"
    );
    assert_eq!(terminal.terminal().observation_count(), CASE_COUNT as u64);
    assert_eq!(
        terminal.terminal().canonical_oracle_match_count(),
        CASE_COUNT as u64
    );
    assert_eq!(terminal.terminal().canonical_oracle_divergence_count(), 0);
    assert_eq!(
        terminal.terminal().conformance_status(),
        FrozenOracleConformanceStatus::Conformant
    );
}
