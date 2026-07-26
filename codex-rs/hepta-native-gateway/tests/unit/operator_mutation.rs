use std::fs;
use std::path::PathBuf;

use hepta_memory::DurableIntegrityKey;

use super::*;

struct TestArtifact(PathBuf);

impl Drop for TestArtifact {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

fn kernel() -> (tempfile::TempDir, RuntimeKernel) {
    let root = tempfile::tempdir().expect("outcome root");
    let kernel = RuntimeKernel::bootstrap_with_durable_outcomes(
        root.path().join("outcomes.sqlite3"),
        DurableIntegrityKey::from_bytes([9; 32]),
    )
    .expect("durable runtime");
    (root, kernel)
}

fn journal(root: &tempfile::TempDir) -> OperatorMutationJournal {
    OperatorMutationJournal::for_test_path(root.path().join("operator-mutations.json"))
}

#[test]
fn product_mutation_requires_separate_exact_plan_and_commit_authority() {
    let (root, kernel) = kernel();
    let journal = journal(&root);
    let key = [7_u8; 32];
    let mutation_id = "a".repeat(64);
    let note = "operator-approved product note";
    let request_binding = "b".repeat(64);
    let session_binding = "c".repeat(64);
    let plan_proof = proof(
        &key,
        PLAN_PROOF_DOMAIN,
        &[&mutation_id, note, &request_binding, &session_binding],
    );
    let plan_body = serde_json::json!({
        "mutation_id": mutation_id,
        "note": note,
        "proof": plan_proof,
    })
    .to_string();
    let plan = plan_with_key(
        &kernel,
        Some(&plan_body),
        &request_binding,
        &session_binding,
        &key,
        &journal,
    )
    .expect("plan exact mutation");
    assert!(!plan.mutation_authorized);
    assert!(!plan.filesystem_mutated);

    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let artifact =
        TestArtifact(workspace.join(format!("artifacts/.hepta-operator-note-{mutation_id}.json")));
    assert!(
        !artifact.0.exists(),
        "planning must not create the artifact"
    );

    let commit_proof = proof(
        &key,
        COMMIT_PROOF_DOMAIN,
        &[
            &mutation_id,
            note,
            &plan.plan_hash,
            &plan.candidate_binding_hash,
            &plan.request_binding_hash,
            &plan.session_binding_hash,
            &"d".repeat(64),
        ],
    );
    let commit_body = serde_json::json!({
        "mutation_id": mutation_id,
        "note": note,
        "plan_hash": plan.plan_hash,
        "candidate_binding_hash": plan.candidate_binding_hash,
        "plan_request_binding_hash": plan.request_binding_hash,
        "session_binding_hash": plan.session_binding_hash,
        "proof": commit_proof,
    })
    .to_string();
    assert!(
        commit_with_key(
            &kernel,
            Some(&commit_body),
            &"e".repeat(64),
            &session_binding,
            &key,
            &journal,
        )
        .is_err(),
        "commit proof must not move to another transport request binding"
    );
    let commit = commit_with_key(
        &kernel,
        Some(&commit_body),
        &"d".repeat(64),
        &session_binding,
        &key,
        &journal,
    )
    .expect("commit exact mutation");
    assert!(commit.execution_receipt.durable_intent_recorded);
    assert!(commit.execution_receipt.effect_plan_recorded);
    assert!(commit.execution_receipt.provider_effect_ack_hash.is_some());
    assert_eq!(commit.execution_receipt.terminal_status, "succeeded");
    let written = fs::read_to_string(&artifact.0).expect("operator note artifact");
    let value: serde_json::Value = serde_json::from_str(&written).expect("artifact JSON");
    assert_eq!(value["note"], note);

    assert!(
        plan_with_key(
            &kernel,
            Some(&plan_body),
            &request_binding,
            &session_binding,
            &key,
            &journal,
        )
        .is_err(),
        "a completed mutation id must not mint a fresh approval candidate"
    );
    assert!(
        commit_with_key(
            &kernel,
            Some(&commit_body),
            &"d".repeat(64),
            &session_binding,
            &key,
            &journal,
        )
        .is_err(),
        "consumed exact approval must not replay"
    );
}

#[test]
fn product_mutation_plan_and_candidate_are_bound_to_the_frozen_request_context() {
    let (root, kernel) = kernel();
    let journal = journal(&root);
    let key = [11_u8; 32];
    let mutation_id = "f".repeat(64);
    let note = "request-context-bound note";
    let plan_request_binding = "1".repeat(64);
    let plan_session_binding = "2".repeat(64);
    let changed_session_binding = "3".repeat(64);
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let _artifact =
        TestArtifact(workspace.join(format!("artifacts/.hepta-operator-note-{mutation_id}.json")));
    let plan_proof = proof(
        &key,
        PLAN_PROOF_DOMAIN,
        &[
            &mutation_id,
            note,
            &plan_request_binding,
            &plan_session_binding,
        ],
    );
    let plan_body = serde_json::json!({
        "mutation_id": mutation_id,
        "note": note,
        "proof": plan_proof,
    })
    .to_string();
    let plan = plan_with_key(
        &kernel,
        Some(&plan_body),
        &plan_request_binding,
        &plan_session_binding,
        &key,
        &journal,
    )
    .expect("plan exact mutation");
    let commit_proof = proof(
        &key,
        COMMIT_PROOF_DOMAIN,
        &[
            &mutation_id,
            note,
            &plan.plan_hash,
            &plan.candidate_binding_hash,
            &plan.request_binding_hash,
            &plan.session_binding_hash,
            &"5".repeat(64),
        ],
    );
    let commit_body = serde_json::json!({
        "mutation_id": mutation_id,
        "note": note,
        "plan_hash": plan.plan_hash,
        "candidate_binding_hash": plan.candidate_binding_hash,
        "plan_request_binding_hash": plan.request_binding_hash,
        "session_binding_hash": plan.session_binding_hash,
        "proof": commit_proof,
    })
    .to_string();

    assert!(
        commit_with_key(
            &kernel,
            Some(&commit_body),
            &"4".repeat(64),
            &changed_session_binding,
            &key,
            &journal,
        )
        .is_err(),
        "a plan minted for one frozen runtime session must not authorize another"
    );
    assert!(
        commit_with_key(
            &kernel,
            Some(&commit_body),
            &"5".repeat(64),
            &plan_session_binding,
            &key,
            &journal,
        )
        .is_ok(),
        "the exact originating frozen runtime session must remain executable once"
    );
}

#[test]
fn product_mutation_rejects_tampering_and_unbounded_input_before_candidate_creation() {
    let (root, kernel) = kernel();
    let journal = journal(&root);
    let key = [3_u8; 32];
    let mutation_id = "c".repeat(64);
    let valid_proof = proof(
        &key,
        PLAN_PROOF_DOMAIN,
        &[&mutation_id, "original", &"d".repeat(64), &"f".repeat(64)],
    );
    let authentic = serde_json::json!({
        "mutation_id": mutation_id,
        "note": "original",
        "proof": valid_proof,
    })
    .to_string();
    assert!(
        plan_with_key(
            &kernel,
            Some(&authentic),
            &"e".repeat(64),
            &"f".repeat(64),
            &key,
            &journal,
        )
        .is_err(),
        "plan proof must not move to another transport request binding"
    );
    let tampered = serde_json::json!({
        "mutation_id": mutation_id,
        "note": "tampered",
        "proof": valid_proof,
    })
    .to_string();
    assert!(
        plan_with_key(
            &kernel,
            Some(&tampered),
            &"d".repeat(64),
            &"f".repeat(64),
            &key,
            &journal,
        )
        .is_err()
    );
    assert!(
        kernel
            .approval_snapshot_for_session("native-gateway:operator-note:missing")
            .expect("empty snapshot")
            .pending
            .is_empty()
    );

    let oversized = "x".repeat(MAX_NOTE_BYTES + 1);
    let proof = proof(
        &key,
        PLAN_PROOF_DOMAIN,
        &[&mutation_id, &oversized, &"e".repeat(64), &"f".repeat(64)],
    );
    let body = serde_json::json!({
        "mutation_id": mutation_id,
        "note": oversized,
        "proof": proof,
    })
    .to_string();
    assert!(
        plan_with_key(
            &kernel,
            Some(&body),
            &"e".repeat(64),
            &"f".repeat(64),
            &key,
            &journal,
        )
        .is_err()
    );
}
