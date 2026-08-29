use std::fmt;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use ed25519_dalek::Signature;
use ed25519_dalek::Verifier as _;
use ed25519_dalek::VerifyingKey;
use serde::Deserialize;
use serde::Serialize;

use crate::AgentId;
use crate::Sha256Digest;

pub const RUNTIME_BOOTSTRAP_SCHEMA_VERSION: u32 = 1;
pub const RUNTIME_BOOTSTRAP_TRUST_SCHEMA_VERSION: u32 = 1;
pub const RUNTIME_BOOTSTRAP_RESERVATION_SCHEMA_VERSION: u32 = 1;
pub const RUNTIME_BOOTSTRAP_NAMESPACE: &str = "hepta.runtime-authority-bootstrap.v1";
pub const RUNTIME_BOOTSTRAP_SIGNATURE_ALGORITHM: &str = "ed25519";
pub const RUNTIME_BOOTSTRAP_MAX_DOCUMENT_BYTES: u64 = 64 * 1024;
pub const RUNTIME_BOOTSTRAP_MAX_LIFETIME_SECONDS: u64 = 300;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeBootstrapError {
    Invalid(&'static str),
    InvalidOwned(String),
    Binding(&'static str),
    NotYetValid { not_before: u64 },
    Expired { expires_at: u64 },
    SignatureRejected(String),
    Decode(String),
}

impl fmt::Display for RuntimeBootstrapError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(reason) => write!(formatter, "invalid runtime bootstrap: {reason}"),
            Self::InvalidOwned(reason) => write!(formatter, "invalid runtime bootstrap: {reason}"),
            Self::Binding(field) => {
                write!(formatter, "runtime bootstrap binding mismatch: {field}")
            }
            Self::NotYetValid { not_before } => {
                write!(
                    formatter,
                    "runtime bootstrap is not valid before {not_before}"
                )
            }
            Self::Expired { expires_at } => {
                write!(formatter, "runtime bootstrap expired at {expires_at}")
            }
            Self::SignatureRejected(reason) => {
                write!(formatter, "runtime bootstrap signature rejected: {reason}")
            }
            Self::Decode(reason) => write!(formatter, "runtime bootstrap decode failed: {reason}"),
        }
    }
}

impl std::error::Error for RuntimeBootstrapError {}

#[derive(Clone, Debug)]
pub struct RuntimeBootstrapEnvelopeFields {
    pub subject_agent_id: AgentId,
    pub release_id: String,
    pub source_commit: String,
    pub source_tree: String,
    pub binary_sha256: Sha256Digest,
    pub runtime_profile: String,
    pub runtime_profile_sha256: Sha256Digest,
    pub authority_grant_sha256: Sha256Digest,
    pub product_graph_sha256: Sha256Digest,
    pub authority_epoch: u64,
    pub owner_epoch: u64,
    pub generation: u64,
    pub fencing_token_sha256: Sha256Digest,
    pub signer_key_id: String,
    pub signer_epoch: u64,
    pub issued_at_unix_seconds: u64,
    pub not_before_unix_seconds: u64,
    pub expires_at_unix_seconds: u64,
    pub nonce_sha256: Sha256Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeBootstrapEnvelope {
    schema_version: u32,
    bootstrap_id: String,
    subject_agent_id: AgentId,
    release_id: String,
    source_commit: String,
    source_tree: String,
    binary_sha256: Sha256Digest,
    runtime_profile: String,
    runtime_profile_sha256: Sha256Digest,
    authority_grant_sha256: Sha256Digest,
    product_graph_sha256: Sha256Digest,
    authority_epoch: u64,
    owner_epoch: u64,
    generation: u64,
    fencing_token_sha256: Sha256Digest,
    signer_key_id: String,
    signer_epoch: u64,
    issued_at_unix_seconds: u64,
    not_before_unix_seconds: u64,
    expires_at_unix_seconds: u64,
    nonce_sha256: Sha256Digest,
}

impl RuntimeBootstrapEnvelope {
    pub fn new(fields: RuntimeBootstrapEnvelopeFields) -> Result<Self, RuntimeBootstrapError> {
        let envelope = Self {
            schema_version: RUNTIME_BOOTSTRAP_SCHEMA_VERSION,
            bootstrap_id: format!("runtime-bootstrap:v1:{}", fields.nonce_sha256.as_str()),
            subject_agent_id: fields.subject_agent_id,
            release_id: fields.release_id,
            source_commit: fields.source_commit,
            source_tree: fields.source_tree,
            binary_sha256: fields.binary_sha256,
            runtime_profile: fields.runtime_profile,
            runtime_profile_sha256: fields.runtime_profile_sha256,
            authority_grant_sha256: fields.authority_grant_sha256,
            product_graph_sha256: fields.product_graph_sha256,
            authority_epoch: fields.authority_epoch,
            owner_epoch: fields.owner_epoch,
            generation: fields.generation,
            fencing_token_sha256: fields.fencing_token_sha256,
            signer_key_id: fields.signer_key_id,
            signer_epoch: fields.signer_epoch,
            issued_at_unix_seconds: fields.issued_at_unix_seconds,
            not_before_unix_seconds: fields.not_before_unix_seconds,
            expires_at_unix_seconds: fields.expires_at_unix_seconds,
            nonce_sha256: fields.nonce_sha256,
        };
        envelope.validate_shape()?;
        Ok(envelope)
    }

    pub fn validate_shape(&self) -> Result<(), RuntimeBootstrapError> {
        if self.schema_version != RUNTIME_BOOTSTRAP_SCHEMA_VERSION {
            return Err(RuntimeBootstrapError::Invalid("unsupported schema version"));
        }
        validate_identifier(&self.release_id, "release id", 128)?;
        validate_git_oid(&self.source_commit, "source commit")?;
        validate_git_oid(&self.source_tree, "source tree")?;
        validate_identifier(&self.signer_key_id, "signer key id", 128)?;
        if !matches!(
            self.runtime_profile.as_str(),
            "snapshot_read_only" | "agent_local" | "qualification_cognitive_write"
        ) {
            return Err(RuntimeBootstrapError::Invalid("unknown runtime profile"));
        }
        if self.authority_epoch == 0
            || self.owner_epoch == 0
            || self.generation == 0
            || self.signer_epoch == 0
        {
            return Err(RuntimeBootstrapError::Invalid(
                "epochs and generation must be non-zero",
            ));
        }
        validate_window(
            self.issued_at_unix_seconds,
            self.not_before_unix_seconds,
            self.expires_at_unix_seconds,
        )?;
        let expected_id = format!("runtime-bootstrap:v1:{}", self.nonce_sha256.as_str());
        if self.bootstrap_id != expected_id {
            return Err(RuntimeBootstrapError::Invalid(
                "bootstrap id drifted from nonce",
            ));
        }
        Ok(())
    }

    pub fn signing_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        frame(&mut bytes, RUNTIME_BOOTSTRAP_NAMESPACE.as_bytes());
        frame(&mut bytes, &self.schema_version.to_be_bytes());
        frame(&mut bytes, self.bootstrap_id.as_bytes());
        frame(&mut bytes, self.subject_agent_id.as_str().as_bytes());
        frame(&mut bytes, self.release_id.as_bytes());
        frame(&mut bytes, self.source_commit.as_bytes());
        frame(&mut bytes, self.source_tree.as_bytes());
        frame(&mut bytes, self.binary_sha256.as_str().as_bytes());
        frame(&mut bytes, self.runtime_profile.as_bytes());
        frame(&mut bytes, self.runtime_profile_sha256.as_str().as_bytes());
        frame(&mut bytes, self.authority_grant_sha256.as_str().as_bytes());
        frame(&mut bytes, self.product_graph_sha256.as_str().as_bytes());
        frame(&mut bytes, &self.authority_epoch.to_be_bytes());
        frame(&mut bytes, &self.owner_epoch.to_be_bytes());
        frame(&mut bytes, &self.generation.to_be_bytes());
        frame(&mut bytes, self.fencing_token_sha256.as_str().as_bytes());
        frame(&mut bytes, self.signer_key_id.as_bytes());
        frame(&mut bytes, &self.signer_epoch.to_be_bytes());
        frame(&mut bytes, &self.issued_at_unix_seconds.to_be_bytes());
        frame(&mut bytes, &self.not_before_unix_seconds.to_be_bytes());
        frame(&mut bytes, &self.expires_at_unix_seconds.to_be_bytes());
        frame(&mut bytes, self.nonce_sha256.as_str().as_bytes());
        bytes
    }

    pub fn digest(&self) -> Sha256Digest {
        Sha256Digest::for_bytes(&self.signing_bytes())
    }

    pub fn subject_agent_id(&self) -> &AgentId {
        &self.subject_agent_id
    }

    pub fn release_id(&self) -> &str {
        &self.release_id
    }

    pub fn source_commit(&self) -> &str {
        &self.source_commit
    }

    pub fn source_tree(&self) -> &str {
        &self.source_tree
    }

    pub fn binary_sha256(&self) -> &Sha256Digest {
        &self.binary_sha256
    }

    pub fn runtime_profile(&self) -> &str {
        &self.runtime_profile
    }

    pub fn runtime_profile_sha256(&self) -> &Sha256Digest {
        &self.runtime_profile_sha256
    }

    pub fn authority_grant_sha256(&self) -> &Sha256Digest {
        &self.authority_grant_sha256
    }

    pub fn product_graph_sha256(&self) -> &Sha256Digest {
        &self.product_graph_sha256
    }

    pub const fn authority_epoch(&self) -> u64 {
        self.authority_epoch
    }

    pub const fn owner_epoch(&self) -> u64 {
        self.owner_epoch
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }

    pub fn fencing_token_sha256(&self) -> &Sha256Digest {
        &self.fencing_token_sha256
    }

    pub fn signer_key_id(&self) -> &str {
        &self.signer_key_id
    }

    pub const fn signer_epoch(&self) -> u64 {
        self.signer_epoch
    }

    pub const fn expires_at_unix_seconds(&self) -> u64 {
        self.expires_at_unix_seconds
    }

    pub fn nonce_sha256(&self) -> &Sha256Digest {
        &self.nonce_sha256
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeBootstrapSignature {
    algorithm: String,
    signer_key_id: String,
    signer_epoch: u64,
    envelope_sha256: Sha256Digest,
    signature_base64: String,
}

impl RuntimeBootstrapSignature {
    pub fn new(
        signer_key_id: impl Into<String>,
        signer_epoch: u64,
        envelope_sha256: Sha256Digest,
        signature_base64: impl Into<String>,
    ) -> Result<Self, RuntimeBootstrapError> {
        let signature = Self {
            algorithm: RUNTIME_BOOTSTRAP_SIGNATURE_ALGORITHM.to_string(),
            signer_key_id: signer_key_id.into(),
            signer_epoch,
            envelope_sha256,
            signature_base64: signature_base64.into(),
        };
        signature.validate()?;
        Ok(signature)
    }

    fn validate(&self) -> Result<(), RuntimeBootstrapError> {
        if self.algorithm != RUNTIME_BOOTSTRAP_SIGNATURE_ALGORITHM || self.signer_epoch == 0 {
            return Err(RuntimeBootstrapError::Invalid(
                "signature metadata is invalid",
            ));
        }
        validate_identifier(&self.signer_key_id, "signature signer key id", 128)?;
        canonical_base64(&self.signature_base64, 64, "signature")?;
        Ok(())
    }

    pub fn signature_base64(&self) -> &str {
        &self.signature_base64
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeBootstrapDocument {
    pub envelope: RuntimeBootstrapEnvelope,
    pub signature: RuntimeBootstrapSignature,
}

impl RuntimeBootstrapDocument {
    pub fn new(
        envelope: RuntimeBootstrapEnvelope,
        signature: RuntimeBootstrapSignature,
    ) -> Result<Self, RuntimeBootstrapError> {
        let document = Self {
            envelope,
            signature,
        };
        document.validate_shape()?;
        Ok(document)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, RuntimeBootstrapError> {
        if bytes.is_empty() || bytes.len() as u64 > RUNTIME_BOOTSTRAP_MAX_DOCUMENT_BYTES {
            return Err(RuntimeBootstrapError::Invalid(
                "document size is out of bounds",
            ));
        }
        let document: Self = serde_json::from_slice(bytes)
            .map_err(|error| RuntimeBootstrapError::Decode(error.to_string()))?;
        document.validate_shape()?;
        Ok(document)
    }

    pub fn encode(&self) -> Result<Vec<u8>, RuntimeBootstrapError> {
        self.validate_shape()?;
        let mut bytes = serde_json::to_vec(self)
            .map_err(|error| RuntimeBootstrapError::Decode(error.to_string()))?;
        bytes.push(b'\n');
        if bytes.len() as u64 > RUNTIME_BOOTSTRAP_MAX_DOCUMENT_BYTES {
            return Err(RuntimeBootstrapError::Invalid(
                "encoded document exceeds bound",
            ));
        }
        Ok(bytes)
    }

    pub fn validate_shape(&self) -> Result<(), RuntimeBootstrapError> {
        self.envelope.validate_shape()?;
        self.signature.validate()?;
        if self.signature.signer_key_id != self.envelope.signer_key_id
            || self.signature.signer_epoch != self.envelope.signer_epoch
            || self.signature.envelope_sha256 != self.envelope.digest()
        {
            return Err(RuntimeBootstrapError::Invalid(
                "detached signature metadata drifted from envelope",
            ));
        }
        Ok(())
    }

    pub fn digest(&self) -> Sha256Digest {
        let mut bytes = Vec::new();
        frame(&mut bytes, b"hepta:runtime-bootstrap-document:v1");
        frame(&mut bytes, self.envelope.digest().as_str().as_bytes());
        frame(&mut bytes, self.signature.signature_base64.as_bytes());
        Sha256Digest::for_bytes(&bytes)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeBootstrapExpectation {
    pub subject_agent_id: AgentId,
    pub release_id: String,
    pub source_commit: String,
    pub source_tree: String,
    pub binary_sha256: Sha256Digest,
    pub runtime_profile: String,
    pub runtime_profile_sha256: Sha256Digest,
    pub authority_grant_sha256: Sha256Digest,
    pub product_graph_sha256: Sha256Digest,
    pub authority_epoch: u64,
    pub owner_epoch: u64,
    pub generation: u64,
    pub fencing_token_sha256: Sha256Digest,
    pub signer_key_id: String,
    pub signer_epoch: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedRuntimeBootstrap {
    document_sha256: Sha256Digest,
    nonce_sha256: Sha256Digest,
    generation: u64,
}

impl VerifiedRuntimeBootstrap {
    pub fn document_sha256(&self) -> &Sha256Digest {
        &self.document_sha256
    }

    pub fn nonce_sha256(&self) -> &Sha256Digest {
        &self.nonce_sha256
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }
}

pub trait RuntimeBootstrapSignatureVerifier: Send + Sync {
    fn verify(
        &self,
        envelope: &RuntimeBootstrapEnvelope,
        signature: &RuntimeBootstrapSignature,
    ) -> Result<(), String>;
}

pub fn verify_runtime_bootstrap<V>(
    document: &RuntimeBootstrapDocument,
    expected: &RuntimeBootstrapExpectation,
    observed_at_unix_seconds: u64,
    verifier: &V,
) -> Result<VerifiedRuntimeBootstrap, RuntimeBootstrapError>
where
    V: RuntimeBootstrapSignatureVerifier + ?Sized,
{
    document.validate_shape()?;
    if observed_at_unix_seconds < document.envelope.not_before_unix_seconds {
        return Err(RuntimeBootstrapError::NotYetValid {
            not_before: document.envelope.not_before_unix_seconds,
        });
    }
    if observed_at_unix_seconds >= document.envelope.expires_at_unix_seconds {
        return Err(RuntimeBootstrapError::Expired {
            expires_at: document.envelope.expires_at_unix_seconds,
        });
    }
    compare_expectation(&document.envelope, expected)?;
    verifier
        .verify(&document.envelope, &document.signature)
        .map_err(RuntimeBootstrapError::SignatureRejected)?;
    Ok(VerifiedRuntimeBootstrap {
        document_sha256: document.digest(),
        nonce_sha256: document.envelope.nonce_sha256.clone(),
        generation: document.envelope.generation,
    })
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeBootstrapTrustRoot {
    schema_version: u32,
    algorithm: String,
    signer_key_id: String,
    signer_epoch: u64,
    public_key_base64: String,
    public_key_sha256: Sha256Digest,
}

impl RuntimeBootstrapTrustRoot {
    pub fn new(
        signer_key_id: impl Into<String>,
        signer_epoch: u64,
        public_key: [u8; 32],
    ) -> Result<Self, RuntimeBootstrapError> {
        let trust = Self {
            schema_version: RUNTIME_BOOTSTRAP_TRUST_SCHEMA_VERSION,
            algorithm: RUNTIME_BOOTSTRAP_SIGNATURE_ALGORITHM.to_string(),
            signer_key_id: signer_key_id.into(),
            signer_epoch,
            public_key_base64: STANDARD.encode(public_key),
            public_key_sha256: Sha256Digest::for_bytes(&public_key),
        };
        trust.validate()?;
        Ok(trust)
    }

    pub fn validate(&self) -> Result<(), RuntimeBootstrapError> {
        if self.schema_version != RUNTIME_BOOTSTRAP_TRUST_SCHEMA_VERSION
            || self.algorithm != RUNTIME_BOOTSTRAP_SIGNATURE_ALGORITHM
            || self.signer_epoch == 0
        {
            return Err(RuntimeBootstrapError::Invalid(
                "trust root metadata is invalid",
            ));
        }
        validate_identifier(&self.signer_key_id, "trust signer key id", 128)?;
        let bytes = canonical_base64(&self.public_key_base64, 32, "public key")?;
        if self.public_key_sha256 != Sha256Digest::for_bytes(&bytes) {
            return Err(RuntimeBootstrapError::Invalid("public key digest mismatch"));
        }
        let array: [u8; 32] = bytes
            .try_into()
            .map_err(|_| RuntimeBootstrapError::Invalid("public key length is invalid"))?;
        let key = VerifyingKey::from_bytes(&array)
            .map_err(|_| RuntimeBootstrapError::Invalid("public key is malformed"))?;
        if key.is_weak() {
            return Err(RuntimeBootstrapError::Invalid("public key is weak"));
        }
        Ok(())
    }

    pub fn signer_key_id(&self) -> &str {
        &self.signer_key_id
    }

    pub const fn signer_epoch(&self) -> u64 {
        self.signer_epoch
    }

    pub fn public_key_bytes(&self) -> Result<[u8; 32], RuntimeBootstrapError> {
        self.validate()?;
        let bytes = canonical_base64(&self.public_key_base64, 32, "public key")?;
        bytes
            .try_into()
            .map_err(|_| RuntimeBootstrapError::Invalid("public key length is invalid"))
    }

    pub fn verifier(&self) -> Result<Ed25519RuntimeBootstrapVerifier, RuntimeBootstrapError> {
        Ed25519RuntimeBootstrapVerifier::new(
            self.signer_key_id.clone(),
            self.signer_epoch,
            self.public_key_bytes()?,
        )
    }
}

pub struct Ed25519RuntimeBootstrapVerifier {
    signer_key_id: String,
    signer_epoch: u64,
    verifying_key: VerifyingKey,
}

impl Ed25519RuntimeBootstrapVerifier {
    pub fn new(
        signer_key_id: impl Into<String>,
        signer_epoch: u64,
        public_key: [u8; 32],
    ) -> Result<Self, RuntimeBootstrapError> {
        let signer_key_id = signer_key_id.into();
        validate_identifier(&signer_key_id, "verifier signer key id", 128)?;
        if signer_epoch == 0 {
            return Err(RuntimeBootstrapError::Invalid(
                "verifier epoch must be non-zero",
            ));
        }
        let verifying_key = VerifyingKey::from_bytes(&public_key)
            .map_err(|_| RuntimeBootstrapError::Invalid("verifier public key is malformed"))?;
        if verifying_key.is_weak() {
            return Err(RuntimeBootstrapError::Invalid(
                "verifier public key is weak",
            ));
        }
        Ok(Self {
            signer_key_id,
            signer_epoch,
            verifying_key,
        })
    }
}

impl RuntimeBootstrapSignatureVerifier for Ed25519RuntimeBootstrapVerifier {
    fn verify(
        &self,
        envelope: &RuntimeBootstrapEnvelope,
        signature: &RuntimeBootstrapSignature,
    ) -> Result<(), String> {
        if signature.signer_key_id != self.signer_key_id
            || signature.signer_epoch != self.signer_epoch
            || envelope.signer_key_id != self.signer_key_id
            || envelope.signer_epoch != self.signer_epoch
        {
            return Err("signer identity or epoch mismatch".to_string());
        }
        let signature_bytes = canonical_base64(&signature.signature_base64, 64, "signature")
            .map_err(|error| error.to_string())?;
        let signature = Signature::from_slice(&signature_bytes)
            .map_err(|_| "signature bytes are malformed".to_string())?;
        self.verifying_key
            .verify(&envelope.signing_bytes(), &signature)
            .map_err(|_| "Ed25519 signature is invalid".to_string())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeBootstrapReservation {
    pub schema_version: u32,
    pub subject_agent_id: AgentId,
    pub generation: u64,
    pub envelope_sha256: Sha256Digest,
    pub nonce_sha256: Sha256Digest,
}

impl RuntimeBootstrapReservation {
    pub fn new(document: &RuntimeBootstrapDocument) -> Result<Self, RuntimeBootstrapError> {
        document.validate_shape()?;
        Ok(Self {
            schema_version: RUNTIME_BOOTSTRAP_RESERVATION_SCHEMA_VERSION,
            subject_agent_id: document.envelope.subject_agent_id.clone(),
            generation: document.envelope.generation,
            envelope_sha256: document.digest(),
            nonce_sha256: document.envelope.nonce_sha256.clone(),
        })
    }

    pub fn validate(&self) -> Result<(), RuntimeBootstrapError> {
        if self.schema_version != RUNTIME_BOOTSTRAP_RESERVATION_SCHEMA_VERSION
            || self.generation == 0
        {
            return Err(RuntimeBootstrapError::Invalid(
                "reservation metadata is invalid",
            ));
        }
        Ok(())
    }
}

pub fn runtime_bootstrap_document_file_name(generation: u64) -> String {
    format!("runtime-bootstrap-{generation:020}.json")
}

pub fn runtime_bootstrap_reservation_file_name(generation: u64) -> String {
    format!("runtime-bootstrap-reservation-{generation:020}.json")
}

pub fn runtime_bootstrap_claim_file_name(generation: u64) -> String {
    format!("runtime-bootstrap-claim-{generation:020}.json")
}

fn compare_expectation(
    envelope: &RuntimeBootstrapEnvelope,
    expected: &RuntimeBootstrapExpectation,
) -> Result<(), RuntimeBootstrapError> {
    macro_rules! compare {
        ($field:ident) => {
            if envelope.$field != expected.$field {
                return Err(RuntimeBootstrapError::Binding(stringify!($field)));
            }
        };
    }
    compare!(subject_agent_id);
    compare!(release_id);
    compare!(source_commit);
    compare!(source_tree);
    compare!(binary_sha256);
    compare!(runtime_profile);
    compare!(runtime_profile_sha256);
    compare!(authority_grant_sha256);
    compare!(product_graph_sha256);
    compare!(authority_epoch);
    compare!(owner_epoch);
    compare!(generation);
    compare!(fencing_token_sha256);
    compare!(signer_key_id);
    compare!(signer_epoch);
    Ok(())
}

fn validate_window(
    issued_at: u64,
    not_before: u64,
    expires_at: u64,
) -> Result<(), RuntimeBootstrapError> {
    if issued_at > not_before || not_before >= expires_at {
        return Err(RuntimeBootstrapError::Invalid(
            "validity window ordering is invalid",
        ));
    }
    if expires_at - not_before > RUNTIME_BOOTSTRAP_MAX_LIFETIME_SECONDS {
        return Err(RuntimeBootstrapError::Invalid(
            "validity window exceeds maximum",
        ));
    }
    Ok(())
}

fn validate_identifier(
    value: &str,
    label: &str,
    max_bytes: usize,
) -> Result<(), RuntimeBootstrapError> {
    if value.is_empty()
        || value.len() > max_bytes
        || !value.is_ascii()
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b':'))
    {
        return Err(RuntimeBootstrapError::InvalidOwned(format!(
            "{label} must be bounded canonical ASCII"
        )));
    }
    Ok(())
}

fn validate_git_oid(value: &str, label: &str) -> Result<(), RuntimeBootstrapError> {
    if value.len() != 40
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(RuntimeBootstrapError::InvalidOwned(format!(
            "{label} must be a lowercase 40-hex object id"
        )));
    }
    Ok(())
}

fn canonical_base64(
    value: &str,
    expected_len: usize,
    label: &str,
) -> Result<Vec<u8>, RuntimeBootstrapError> {
    let bytes = STANDARD
        .decode(value)
        .map_err(|_| RuntimeBootstrapError::InvalidOwned(format!("{label} is not base64")))?;
    if bytes.len() != expected_len || STANDARD.encode(&bytes) != value {
        return Err(RuntimeBootstrapError::InvalidOwned(format!(
            "{label} is not canonical or has the wrong length"
        )));
    }
    Ok(bytes)
}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

#[cfg(test)]
#[path = "runtime_bootstrap_tests.rs"]
mod tests;
