use crate::CANDIDATE_HEAD;
use crate::CANDIDATE_TREE;
use crate::QualificationError;
use crate::invalid;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

const ATTEMPT_IDENTITY_SCHEMA: &str = "hepta_linux_exact_v8_attempt_identity_v1";

/// The complete immutable identity shared by every v8 journal record and artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AttemptIdentityV8 {
    pub attempt_nonce: String,
    pub barrier_generation: u64,
    pub candidate_head: String,
    pub candidate_tree: String,
    pub driver_manifest_sha256: String,
    pub profile_manifest_sha256: String,
    pub parameter_manifest_sha256: String,
    pub machine_id_sha256: String,
    pub runner_snapshot_sha256: String,
    pub restore_plan_sha256: String,
}

impl AttemptIdentityV8 {
    pub fn validate(&self) -> Result<(), QualificationError> {
        validate_lower_hex("attempt_nonce", &self.attempt_nonce, 64)?;
        if self.attempt_nonce.bytes().all(|byte| byte == b'0') {
            return Err(invalid("attempt_nonce must not be the all-zero sentinel"));
        }
        if self.barrier_generation == 0 {
            return Err(invalid("barrier_generation must be non-zero"));
        }
        validate_lower_hex("candidate_head", &self.candidate_head, 40)?;
        validate_lower_hex("candidate_tree", &self.candidate_tree, 40)?;
        if self.candidate_head != CANDIDATE_HEAD {
            return Err(invalid("candidate_head is not the frozen 52ec candidate"));
        }
        if self.candidate_tree != CANDIDATE_TREE {
            return Err(invalid(
                "candidate_tree is not the frozen 52ec candidate tree",
            ));
        }
        validate_sha256("driver_manifest_sha256", &self.driver_manifest_sha256)?;
        validate_sha256("profile_manifest_sha256", &self.profile_manifest_sha256)?;
        validate_sha256("parameter_manifest_sha256", &self.parameter_manifest_sha256)?;
        validate_sha256("machine_id_sha256", &self.machine_id_sha256)?;
        validate_sha256("runner_snapshot_sha256", &self.runner_snapshot_sha256)?;
        validate_sha256("restore_plan_sha256", &self.restore_plan_sha256)?;
        Ok(())
    }

    /// Returns the deterministic signing representation.
    ///
    /// The sequence is schema, nonce, generation, candidate head, candidate tree,
    /// driver manifest, profile manifest, parameter manifest, machine identity,
    /// runner snapshot, and restore plan. Text and byte fields are length-prefixed;
    /// the generation is an unsigned big-endian integer.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, QualificationError> {
        self.validate()?;
        let mut bytes = Vec::new();
        append_text(&mut bytes, "schema", ATTEMPT_IDENTITY_SCHEMA);
        append_text(&mut bytes, "attempt_nonce", &self.attempt_nonce);
        append_u64(&mut bytes, "barrier_generation", self.barrier_generation);
        append_text(&mut bytes, "candidate_head", &self.candidate_head);
        append_text(&mut bytes, "candidate_tree", &self.candidate_tree);
        append_text(
            &mut bytes,
            "driver_manifest_sha256",
            &self.driver_manifest_sha256,
        );
        append_text(
            &mut bytes,
            "profile_manifest_sha256",
            &self.profile_manifest_sha256,
        );
        append_text(
            &mut bytes,
            "parameter_manifest_sha256",
            &self.parameter_manifest_sha256,
        );
        append_text(&mut bytes, "machine_id_sha256", &self.machine_id_sha256);
        append_text(
            &mut bytes,
            "runner_snapshot_sha256",
            &self.runner_snapshot_sha256,
        );
        append_text(&mut bytes, "restore_plan_sha256", &self.restore_plan_sha256);
        Ok(bytes)
    }

    pub fn sha256(&self) -> Result<String, QualificationError> {
        Ok(sha256_hex(&self.canonical_bytes()?))
    }
}

pub(crate) fn validate_sha256(name: &str, value: &str) -> Result<(), QualificationError> {
    validate_lower_hex(name, value, 64)?;
    if value.bytes().all(|byte| byte == b'0') {
        return Err(invalid(format!("{name} must not be the all-zero sentinel")));
    }
    Ok(())
}

pub(crate) fn validate_lower_hex(
    name: &str,
    value: &str,
    expected_len: usize,
) -> Result<(), QualificationError> {
    if value.len() != expected_len
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid(format!(
            "{name} must be exactly {expected_len} lowercase hexadecimal characters"
        )));
    }
    Ok(())
}

pub(crate) fn append_text(bytes: &mut Vec<u8>, name: &str, value: &str) {
    append_bytes(bytes, name, value.as_bytes());
}

pub(crate) fn append_u64(bytes: &mut Vec<u8>, name: &str, value: u64) {
    append_bytes(bytes, name, &value.to_be_bytes());
}

pub(crate) fn append_bytes(bytes: &mut Vec<u8>, name: &str, value: &[u8]) {
    let name_len = name.len() as u64;
    let value_len = value.len() as u64;
    bytes.extend_from_slice(&name_len.to_be_bytes());
    bytes.extend_from_slice(name.as_bytes());
    bytes.extend_from_slice(&value_len.to_be_bytes());
    bytes.extend_from_slice(value);
}

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
#[path = "identity_tests.rs"]
mod tests;
