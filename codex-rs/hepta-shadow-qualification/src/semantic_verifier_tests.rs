use serde_json::Value;

use super::oracle::FrozenOracle;
use super::semantic_verifier::SemanticVerifier;
use super::verification_primitives::canonical_json;
use crate::QualificationError;

const ACTION_ID: &str = "tool:v1:8f6bcbfaa4fa03f850776b5ec940744f96bf50b6d78b17a04b436e196ea300d8";
const ADMISSION_ID: &str =
    "decision:v1:657ce242e8205905bdc85678268fe3cb6ebaa2abdca12d6f588b87284e33e543";
const AUTHORIZATION_ID: &str =
    "decision:v1:bc77cf37cb529e1149ef433d1ab174ed4d096c9ca7bffaeeda5412a2e34243ae";
const RECEIPT_ID: &str =
    "receipt:v1:1ece195bbd3ee5519e25e28f123001fa786e44bed46dbea7144db6fe93f7b8b5";

#[test]
fn normalizes_dynamic_identity_and_matches_oracle_bytes() -> Result<(), QualificationError> {
    let oracle = FrozenOracle::load_embedded()?;
    let receipt = dynamic_receipt(&oracle)?;
    let verified = SemanticVerifier::verify(&oracle, &receipt)?;
    assert_eq!(
        verified.normalized_receipt_sha256(),
        oracle.expected_normalized_receipt_sha256()
    );
    assert_eq!(
        verified.oracle_sample_id_sha256(),
        oracle.sample_id_sha256()
    );
    assert_eq!(verified.source_receipt_sha256().len(), 64);
    Ok(())
}

#[test]
fn rejects_semantic_or_identity_divergence() -> Result<(), QualificationError> {
    let oracle = FrozenOracle::load_embedded()?;
    let mut semantic: Value = serde_json::from_slice(&dynamic_receipt(&oracle)?)
        .map_err(|error| QualificationError::Serialization(error.to_string()))?;
    semantic["host_accepted"] = Value::Bool(false);
    assert!(SemanticVerifier::verify(&oracle, &canonical_json(&semantic)?).is_err());

    let mut identity: Value = serde_json::from_slice(&dynamic_receipt(&oracle)?)
        .map_err(|error| QualificationError::Serialization(error.to_string()))?;
    identity["admission"]["action"]["call_id"] = Value::String("changed".to_string());
    assert!(SemanticVerifier::verify(&oracle, &canonical_json(&identity)?).is_err());
    Ok(())
}

fn dynamic_receipt(oracle: &FrozenOracle) -> Result<Vec<u8>, QualificationError> {
    let mut value: Value = serde_json::from_slice(oracle.expected_normalized_receipt())
        .map_err(|error| QualificationError::Serialization(error.to_string()))?;
    value["action_id"] = Value::String(ACTION_ID.to_string());
    value["receipt_id"] = Value::String(RECEIPT_ID.to_string());
    for (record, decision_id) in [
        ("admission", ADMISSION_ID),
        ("authorization", AUTHORIZATION_ID),
    ] {
        value[record]["decision_id"] = Value::String(decision_id.to_string());
        value[record]["action"]["action_id"] = Value::String(ACTION_ID.to_string());
        value[record]["action"]["thread_id"] = Value::String("thread-live-1".to_string());
        value[record]["action"]["turn_id"] = Value::String("turn-live-1".to_string());
        value[record]["action"]["call_id"] = Value::String("call-live-1".to_string());
    }
    canonical_json(&value)
}
