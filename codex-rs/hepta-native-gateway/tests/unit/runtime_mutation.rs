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

#[test]
fn mutation_body_requires_exact_confirmation_and_idempotency() {
    let key = "a".repeat(64);
    assert_eq!(
        body_admitted(Some(&format!(
            r#"{{"confirm":true,"idempotency_key":"{key}"}}"#
        ))),
        Some(key)
    );
    assert!(body_admitted(Some(r#"{"confirm":false,"idempotency_key":"bad"}"#)).is_none());
    assert!(body_admitted(Some(r#"{"confirm":true,"idempotency_key":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","extra":1}"#)).is_none());
}

#[test]
fn mutation_canary_records_intent_effect_ack_and_terminal_receipt() {
    let outcome_root = tempfile::tempdir().expect("outcome root");
    let kernel = RuntimeKernel::bootstrap_with_durable_outcomes(
        outcome_root.path().join("outcomes.sqlite3"),
        DurableIntegrityKey::from_bytes([7; 32]),
    )
    .expect("durable runtime");
    let nonce = format!("{}:{:?}", std::process::id(), std::time::SystemTime::now());
    let key = format!("{:x}", Sha256::digest(nonce.as_bytes()));
    let request = format!("{:x}", Sha256::digest(format!("request:{key}").as_bytes()));
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let artifact =
        TestArtifact(workspace.join(format!("artifacts/.hepta-runtime-mutation-{key}.json")));
    let receipt = execute(&kernel, &request, &key).expect("mutation receipt");
    assert!(receipt.execution_receipt.durable_intent_recorded);
    assert!(receipt.execution_receipt.effect_plan_recorded);
    assert!(receipt.execution_receipt.provider_effect_ack_hash.is_some());
    assert_eq!(receipt.execution_receipt.terminal_status, "succeeded");
    assert_eq!(fs::read_to_string(&artifact.0).expect("artifact"), request);
}
