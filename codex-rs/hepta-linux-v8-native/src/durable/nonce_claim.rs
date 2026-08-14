use serde::Deserialize;
use serde::Serialize;
use sha2::Digest as _;

use crate::NativeErrorV8;
use crate::invalid;

use super::nonce_claim_relative_path_v8;
use super::validate_digest;

pub const NONCE_CLAIM_SCHEMA_V8: &str = "hepta_linux_v8_durable_nonce_claim_v1";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NonceClaimRecordV8 {
    pub attempt_identity_sha256: String,
    pub barrier_generation: u64,
    pub machine_id_sha256: String,
    pub namespace: String,
    pub nonce: String,
    pub schema: String,
    pub signature_sha256: String,
    pub statement_sha256: String,
}

impl NonceClaimRecordV8 {
    pub fn validate(&self) -> Result<(), NativeErrorV8> {
        if self.schema != NONCE_CLAIM_SCHEMA_V8 {
            return Err(invalid("nonce claim schema is not exact v8"));
        }
        if self.namespace.is_empty()
            || self.namespace.len() > 128
            || !self
                .namespace
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
        {
            return Err(invalid("nonce claim namespace is malformed"));
        }
        if self.barrier_generation == 0 {
            return Err(invalid("nonce claim generation must be non-zero"));
        }
        for (label, digest) in [
            ("nonce", &self.nonce),
            ("statement", &self.statement_sha256),
            ("signature", &self.signature_sha256),
            ("attempt", &self.attempt_identity_sha256),
            ("machine", &self.machine_id_sha256),
        ] {
            validate_digest(label, digest)?;
        }
        Ok(())
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, NativeErrorV8> {
        self.validate()?;
        let mut bytes = Vec::new();
        append(&mut bytes, b"schema", self.schema.as_bytes())?;
        append(&mut bytes, b"namespace", self.namespace.as_bytes())?;
        append(&mut bytes, b"nonce", self.nonce.as_bytes())?;
        append(
            &mut bytes,
            b"statement_sha256",
            self.statement_sha256.as_bytes(),
        )?;
        append(
            &mut bytes,
            b"signature_sha256",
            self.signature_sha256.as_bytes(),
        )?;
        append(
            &mut bytes,
            b"attempt_identity_sha256",
            self.attempt_identity_sha256.as_bytes(),
        )?;
        append(
            &mut bytes,
            b"barrier_generation",
            &self.barrier_generation.to_be_bytes(),
        )?;
        append(
            &mut bytes,
            b"machine_id_sha256",
            self.machine_id_sha256.as_bytes(),
        )?;
        Ok(bytes)
    }

    pub fn sha256(&self) -> Result<String, NativeErrorV8> {
        Ok(format!(
            "{:x}",
            sha2::Sha256::digest(self.canonical_bytes()?)
        ))
    }

    /// A single global nonce path deliberately omits namespace. Reusing one
    /// nonce in any authority family collides with the same durable claim.
    pub fn relative_path(&self) -> Result<String, NativeErrorV8> {
        self.validate()?;
        nonce_claim_relative_path_v8(&self.nonce)
    }
}

fn append(output: &mut Vec<u8>, name: &[u8], value: &[u8]) -> Result<(), NativeErrorV8> {
    let name_length = u64::try_from(name.len()).map_err(|_| invalid("field name overflow"))?;
    let value_length = u64::try_from(value.len()).map_err(|_| invalid("field value overflow"))?;
    output.extend_from_slice(&name_length.to_be_bytes());
    output.extend_from_slice(name);
    output.extend_from_slice(&value_length.to_be_bytes());
    output.extend_from_slice(value);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(character: char) -> String {
        character.to_string().repeat(64)
    }

    fn claim() -> NonceClaimRecordV8 {
        NonceClaimRecordV8 {
            attempt_identity_sha256: digest('1'),
            barrier_generation: 7,
            machine_id_sha256: digest('2'),
            namespace: "hepta-linux-v8-execution".to_string(),
            nonce: digest('3'),
            schema: NONCE_CLAIM_SCHEMA_V8.to_string(),
            signature_sha256: digest('4'),
            statement_sha256: digest('5'),
        }
    }

    #[test]
    fn every_claim_binding_changes_the_digest_but_not_the_global_nonce_path() {
        let baseline = claim();
        let baseline_digest = baseline.sha256().unwrap();
        let path = baseline.relative_path().unwrap();
        let mut variants = Vec::new();
        let mut changed = baseline.clone();
        changed.namespace = "hepta-linux-v8-break-glass".to_string();
        variants.push(changed);
        let mut changed = baseline.clone();
        changed.statement_sha256 = digest('6');
        variants.push(changed);
        let mut changed = baseline.clone();
        changed.signature_sha256 = digest('7');
        variants.push(changed);
        let mut changed = baseline.clone();
        changed.attempt_identity_sha256 = digest('8');
        variants.push(changed);
        let mut changed = baseline.clone();
        changed.barrier_generation += 1;
        variants.push(changed);
        let mut changed = baseline;
        changed.machine_id_sha256 = digest('9');
        variants.push(changed);

        assert!(
            variants
                .iter()
                .all(|changed| changed.sha256().unwrap() != baseline_digest)
        );
        assert!(
            variants
                .iter()
                .all(|changed| changed.relative_path().unwrap() == path)
        );
    }

    #[test]
    fn malformed_or_caller_completion_fields_are_rejected() {
        let mut value = serde_json::to_value(claim()).unwrap();
        value["release_complete"] = serde_json::json!(true);
        assert!(serde_json::from_value::<NonceClaimRecordV8>(value).is_err());

        let mut zero = claim();
        zero.nonce = "0".repeat(64);
        assert!(zero.validate().is_err());
    }
}
