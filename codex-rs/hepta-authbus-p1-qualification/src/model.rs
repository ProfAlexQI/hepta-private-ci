use codex_hepta_contracts::IdentityBinding;
use codex_hepta_contracts::IdentityPeerEvidence;
use codex_hepta_contracts::Sha256Digest;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

use crate::AUTHBUS_P1_1_AUTHORITY;

pub const AUTHBUS_P1_1_SCHEMA_VERSION: u32 = 1;
const MAX_TEXT_BYTES: usize = 512;
const ED25519_PUBLIC_KEY_BYTES: usize = 32;
const ED25519_SIGNATURE_BYTES: usize = 64;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum P11KeyPurpose {
    IdentityIssuer,
    ProviderStatusIssuer,
    OperatorEvidenceIssuer,
}

impl P11KeyPurpose {
    pub const fn usage_domain(self) -> &'static str {
        match self {
            Self::IdentityIssuer => "identity-issuer",
            Self::ProviderStatusIssuer => "provider-status-issuer",
            Self::OperatorEvidenceIssuer => "operator-evidence-issuer",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum P11WriteDisposition {
    Applied,
    AlreadyPresent,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum P11Error {
    #[error("AuthBus P1.1 input is invalid")]
    InvalidInput,
    #[error("AuthBus P1.1 underlying B1 contract is invalid")]
    ContractInvalid,
    #[error("AuthBus P1.1 verification key was not found")]
    UnknownKey,
    #[error("AuthBus P1.1 verification key conflicts with existing registration")]
    KeyConflict,
    #[error("AuthBus P1.1 verification key has the wrong purpose")]
    KeyPurposeMismatch,
    #[error("AuthBus P1.1 verification key epoch is stale")]
    StaleKeyEpoch,
    #[error("AuthBus P1.1 verification key is not yet valid")]
    KeyNotYetValid,
    #[error("AuthBus P1.1 verification key has expired")]
    KeyExpired,
    #[error("AuthBus P1.1 verification key is revoked")]
    KeyRevoked,
    #[error("AuthBus P1.1 signature is invalid")]
    SignatureInvalid,
    #[error("AuthBus P1.1 audience binding mismatches")]
    AudienceMismatch,
    #[error("AuthBus P1.1 service or policy binding mismatches")]
    BindingMismatch,
    #[error("AuthBus P1.1 evidence is not yet valid")]
    NotYetValid,
    #[error("AuthBus P1.1 evidence has expired")]
    Expired,
    #[error("AuthBus P1.1 evidence TTL exceeds policy")]
    TtlExceeded,
    #[error("AuthBus P1.1 issued-at or observation time is in the future")]
    FutureEvidence,
    #[error("AuthBus P1.1 evidence is older than policy permits")]
    EvidenceTooOld,
    #[error("AuthBus P1.1 launch nonce was replayed")]
    NonceReplay,
    #[error("AuthBus P1.1 nonce replay cache is full")]
    NonceCapacity,
    #[error("AuthBus P1.1 operation ledger is full")]
    OperationCapacity,
    #[error("AuthBus P1.1 operation was not registered")]
    UnknownOperation,
    #[error("AuthBus P1.1 operation registration conflicts")]
    OperationConflict,
    #[error("AuthBus P1.1 status observation is stale")]
    StaleObservation,
    #[error("AuthBus P1.1 evidence conflicts with durable observation")]
    EvidenceConflict,
    #[error("AuthBus P1.1 terminal evidence is immutable")]
    TerminalImmutable,
    #[error("AuthBus P1.1 operation requires independent manual evidence")]
    ManualEvidenceRequired,
    #[error("AuthBus P1.1 manual evidence is not permitted in this state")]
    InvalidManualTransition,
}

pub type P11Result<T> = Result<T, P11Error>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct P11VerificationKeyRecord {
    pub schema_version: u32,
    pub issuer_id: String,
    pub key_id: String,
    pub key_epoch: u64,
    pub purpose: P11KeyPurpose,
    pub usage_domain: String,
    pub public_key: Vec<u8>,
    pub public_key_sha256: Sha256Digest,
    pub backend_binding_sha256: Sha256Digest,
    pub valid_from_unix_seconds: u64,
    pub valid_until_unix_seconds: u64,
    pub revoked_at_unix_seconds: Option<u64>,
    #[serde(default)]
    pub authority: bool,
}

impl P11VerificationKeyRecord {
    pub fn validate(&self) -> P11Result<()> {
        if self.schema_version != AUTHBUS_P1_1_SCHEMA_VERSION {
            return Err(P11Error::InvalidInput);
        }
        assert_negative_authority(self.authority)?;
        validate_text(&self.issuer_id)?;
        validate_text(&self.key_id)?;
        validate_text(&self.usage_domain)?;
        validate_digest(&self.public_key_sha256)?;
        validate_digest(&self.backend_binding_sha256)?;
        if self.key_epoch == 0
            || self.public_key.len() != ED25519_PUBLIC_KEY_BYTES
            || self.valid_from_unix_seconds == 0
            || self.valid_until_unix_seconds <= self.valid_from_unix_seconds
            || self.usage_domain != self.purpose.usage_domain()
            || self.public_key_sha256 != Sha256Digest::for_bytes(&self.public_key)
        {
            return Err(P11Error::InvalidInput);
        }
        if self
            .revoked_at_unix_seconds
            .is_some_and(|revoked_at| revoked_at < self.valid_from_unix_seconds)
        {
            return Err(P11Error::InvalidInput);
        }
        Ok(())
    }

    pub fn registration_digest(&self) -> P11Result<Sha256Digest> {
        self.validate()?;
        digest_serializable("hepta.authbus.p1.1.key-registration.v1", self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct P11Fence {
    pub authority_epoch: u64,
    pub owner_epoch: u64,
    pub generation: u64,
    pub fencing_token_sha256: Sha256Digest,
}

impl P11Fence {
    pub fn validate(&self) -> P11Result<()> {
        validate_digest(&self.fencing_token_sha256)?;
        if self.authority_epoch == 0 || self.owner_epoch == 0 || self.generation == 0 {
            return Err(P11Error::InvalidInput);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P11IdentityVerificationContext {
    pub expected_audience: String,
    pub expected_service_identity_sha256: Sha256Digest,
    pub expected_policy_sha256: Sha256Digest,
    pub expected_peer: IdentityPeerEvidence,
    pub now_unix_seconds: u64,
}

impl P11IdentityVerificationContext {
    pub fn validate(&self) -> P11Result<()> {
        validate_text(&self.expected_audience)?;
        validate_digest(&self.expected_service_identity_sha256)?;
        validate_digest(&self.expected_policy_sha256)?;
        if self.now_unix_seconds == 0 {
            return Err(P11Error::InvalidInput);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct P11SignedIdentityEvidence {
    pub schema_version: u32,
    pub issuer_id: String,
    pub key_id: String,
    pub key_epoch: u64,
    pub binding: IdentityBinding,
    pub signature: Vec<u8>,
    #[serde(default)]
    pub authority: bool,
}

impl P11SignedIdentityEvidence {
    fn validate_unsigned(&self) -> P11Result<()> {
        if self.schema_version != AUTHBUS_P1_1_SCHEMA_VERSION || self.authority {
            return Err(P11Error::InvalidInput);
        }
        validate_text(&self.issuer_id)?;
        validate_text(&self.key_id)?;
        if self.key_epoch == 0
            || self.key_id != self.binding.key_id
            || self.key_epoch != self.binding.epoch
        {
            return Err(P11Error::InvalidInput);
        }
        self.binding
            .validate()
            .map_err(|_| P11Error::ContractInvalid)
    }

    pub fn signing_bytes(&self) -> P11Result<Vec<u8>> {
        self.validate_unsigned()?;
        let binding = self
            .binding
            .canonical_bytes()
            .map_err(|_| P11Error::ContractInvalid)?;
        let mut bytes = Vec::new();
        push_text(&mut bytes, "hepta.authbus.p1.1.signed-identity.v1");
        push_text(&mut bytes, &self.issuer_id);
        push_text(&mut bytes, &self.key_id);
        push_u64(&mut bytes, self.key_epoch);
        push_bytes(&mut bytes, &binding);
        Ok(bytes)
    }

    pub fn evidence_digest(&self) -> P11Result<Sha256Digest> {
        validate_signature(&self.signature)?;
        let mut bytes = self.signing_bytes()?;
        push_bytes(&mut bytes, &self.signature);
        Ok(Sha256Digest::for_bytes(&bytes))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P11IdentityVerificationReceipt {
    pub evidence_sha256: Sha256Digest,
    pub binding_sha256: Sha256Digest,
    pub issuer_id: String,
    pub key_id: String,
    pub key_epoch: u64,
    pub subject_sha256: Sha256Digest,
    pub nonce_sha256: Sha256Digest,
    pub launch_nonce_sha256: Sha256Digest,
    pub expires_at_unix_seconds: u64,
    pub authority: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct P11OperationEvidenceBinding {
    pub schema_version: u32,
    pub operation_id: String,
    pub provider_id: String,
    pub profile_id: String,
    pub token_family_id: String,
    pub status_binding_sha256: Sha256Digest,
    pub fence: P11Fence,
    #[serde(default)]
    pub authority: bool,
}

impl P11OperationEvidenceBinding {
    pub fn validate(&self) -> P11Result<()> {
        if self.schema_version != AUTHBUS_P1_1_SCHEMA_VERSION || self.authority {
            return Err(P11Error::InvalidInput);
        }
        for value in [
            self.operation_id.as_str(),
            self.provider_id.as_str(),
            self.profile_id.as_str(),
            self.token_family_id.as_str(),
        ] {
            validate_text(value)?;
        }
        validate_digest(&self.status_binding_sha256)?;
        self.fence.validate()
    }

    pub fn digest(&self) -> P11Result<Sha256Digest> {
        self.validate()?;
        digest_serializable("hepta.authbus.p1.1.operation-binding.v1", self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum P11ProviderEvidenceOutcome {
    Completed {
        result_sha256: Sha256Digest,
    },
    VerifiedNoEffect {
        provider_receipt_sha256: Sha256Digest,
    },
    Unknown {
        reason_sha256: Sha256Digest,
    },
    Indeterminate {
        reason_sha256: Sha256Digest,
    },
    Quarantined {
        reason_sha256: Sha256Digest,
    },
    ManualRequired {
        reason_sha256: Sha256Digest,
    },
}

impl P11ProviderEvidenceOutcome {
    pub fn validate(&self) -> P11Result<()> {
        match self {
            Self::Completed { result_sha256 } => validate_digest(result_sha256),
            Self::VerifiedNoEffect {
                provider_receipt_sha256,
            } => validate_digest(provider_receipt_sha256),
            Self::Unknown { reason_sha256 }
            | Self::Indeterminate { reason_sha256 }
            | Self::Quarantined { reason_sha256 }
            | Self::ManualRequired { reason_sha256 } => validate_digest(reason_sha256),
        }
    }

    pub fn target_state(&self) -> P11EvidenceState {
        match self {
            Self::Completed { .. } => P11EvidenceState::Completed,
            Self::VerifiedNoEffect { .. } => P11EvidenceState::NoEffect,
            Self::Unknown { .. } => P11EvidenceState::Unknown,
            Self::Indeterminate { .. } => P11EvidenceState::Indeterminate,
            Self::Quarantined { .. } => P11EvidenceState::Quarantined,
            Self::ManualRequired { .. } => P11EvidenceState::ManualRequired,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum P11EvidenceState {
    Pending,
    Unknown,
    Indeterminate,
    LookupOnly,
    ManualRequired,
    Completed,
    NoEffect,
    Quarantined,
}

impl P11EvidenceState {
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::NoEffect | Self::Quarantined)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct P11SignedProviderStatusEvidence {
    pub schema_version: u32,
    pub issuer_id: String,
    pub key_id: String,
    pub key_epoch: u64,
    pub operation_id: String,
    pub provider_id: String,
    pub profile_id: String,
    pub token_family_id: String,
    pub status_binding_sha256: Sha256Digest,
    pub fence: P11Fence,
    pub status_revision: u64,
    pub observed_at_unix_seconds: u64,
    pub outcome: P11ProviderEvidenceOutcome,
    pub signature: Vec<u8>,
    #[serde(default)]
    pub authority: bool,
}

impl P11SignedProviderStatusEvidence {
    fn validate_unsigned(&self) -> P11Result<()> {
        if self.schema_version != AUTHBUS_P1_1_SCHEMA_VERSION || self.authority {
            return Err(P11Error::InvalidInput);
        }
        for value in [
            self.issuer_id.as_str(),
            self.key_id.as_str(),
            self.operation_id.as_str(),
            self.provider_id.as_str(),
            self.profile_id.as_str(),
            self.token_family_id.as_str(),
        ] {
            validate_text(value)?;
        }
        validate_digest(&self.status_binding_sha256)?;
        self.fence.validate()?;
        self.outcome.validate()?;
        if self.key_epoch == 0 || self.status_revision == 0 || self.observed_at_unix_seconds == 0 {
            return Err(P11Error::InvalidInput);
        }
        Ok(())
    }

    pub fn signing_bytes(&self) -> P11Result<Vec<u8>> {
        self.validate_unsigned()?;
        let outcome = serde_json::to_vec(&self.outcome).map_err(|_| P11Error::InvalidInput)?;
        let mut bytes = Vec::new();
        push_text(&mut bytes, "hepta.authbus.p1.1.signed-provider-status.v1");
        push_text(&mut bytes, &self.issuer_id);
        push_text(&mut bytes, &self.key_id);
        push_u64(&mut bytes, self.key_epoch);
        push_text(&mut bytes, &self.operation_id);
        push_text(&mut bytes, &self.provider_id);
        push_text(&mut bytes, &self.profile_id);
        push_text(&mut bytes, &self.token_family_id);
        push_digest(&mut bytes, &self.status_binding_sha256);
        push_fence(&mut bytes, &self.fence);
        push_u64(&mut bytes, self.status_revision);
        push_u64(&mut bytes, self.observed_at_unix_seconds);
        push_bytes(&mut bytes, &outcome);
        Ok(bytes)
    }

    pub fn evidence_digest(&self) -> P11Result<Sha256Digest> {
        validate_signature(&self.signature)?;
        let mut bytes = self.signing_bytes()?;
        push_bytes(&mut bytes, &self.signature);
        Ok(Sha256Digest::for_bytes(&bytes))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P11ProviderStatusReceipt {
    pub evidence_sha256: Sha256Digest,
    pub operation_id: String,
    pub status_revision: u64,
    pub observed_at_unix_seconds: u64,
    pub state: P11EvidenceState,
    pub authority: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum P11ProviderEvidenceDisposition {
    Applied(P11ProviderStatusReceipt),
    AlreadyPresent(P11ProviderStatusReceipt),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum P11ManualDecision {
    ResumeLookupOnly,
    KeepManualRequired,
    Quarantine,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct P11SignedManualEvidence {
    pub schema_version: u32,
    pub issuer_id: String,
    pub key_id: String,
    pub key_epoch: u64,
    pub operator_id: String,
    pub case_id: String,
    pub operation_id: String,
    pub status_binding_sha256: Sha256Digest,
    pub fence: P11Fence,
    pub manual_revision: u64,
    pub observed_at_unix_seconds: u64,
    pub decision: P11ManualDecision,
    pub reason_sha256: Sha256Digest,
    pub signature: Vec<u8>,
    #[serde(default)]
    pub authority: bool,
}

impl P11SignedManualEvidence {
    fn validate_unsigned(&self) -> P11Result<()> {
        if self.schema_version != AUTHBUS_P1_1_SCHEMA_VERSION || self.authority {
            return Err(P11Error::InvalidInput);
        }
        for value in [
            self.issuer_id.as_str(),
            self.key_id.as_str(),
            self.operator_id.as_str(),
            self.case_id.as_str(),
            self.operation_id.as_str(),
        ] {
            validate_text(value)?;
        }
        validate_digest(&self.status_binding_sha256)?;
        validate_digest(&self.reason_sha256)?;
        self.fence.validate()?;
        if self.key_epoch == 0 || self.manual_revision == 0 || self.observed_at_unix_seconds == 0 {
            return Err(P11Error::InvalidInput);
        }
        Ok(())
    }

    pub fn signing_bytes(&self) -> P11Result<Vec<u8>> {
        self.validate_unsigned()?;
        let mut bytes = Vec::new();
        push_text(&mut bytes, "hepta.authbus.p1.1.signed-manual-evidence.v1");
        push_text(&mut bytes, &self.issuer_id);
        push_text(&mut bytes, &self.key_id);
        push_u64(&mut bytes, self.key_epoch);
        push_text(&mut bytes, &self.operator_id);
        push_text(&mut bytes, &self.case_id);
        push_text(&mut bytes, &self.operation_id);
        push_digest(&mut bytes, &self.status_binding_sha256);
        push_fence(&mut bytes, &self.fence);
        push_u64(&mut bytes, self.manual_revision);
        push_u64(&mut bytes, self.observed_at_unix_seconds);
        push_text(
            &mut bytes,
            match self.decision {
                P11ManualDecision::ResumeLookupOnly => "RESUME_LOOKUP_ONLY",
                P11ManualDecision::KeepManualRequired => "KEEP_MANUAL_REQUIRED",
                P11ManualDecision::Quarantine => "QUARANTINE",
            },
        );
        push_digest(&mut bytes, &self.reason_sha256);
        Ok(bytes)
    }

    pub fn evidence_digest(&self) -> P11Result<Sha256Digest> {
        validate_signature(&self.signature)?;
        let mut bytes = self.signing_bytes()?;
        push_bytes(&mut bytes, &self.signature);
        Ok(Sha256Digest::for_bytes(&bytes))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P11ManualEvidenceReceipt {
    pub evidence_sha256: Sha256Digest,
    pub operation_id: String,
    pub manual_revision: u64,
    pub observed_at_unix_seconds: u64,
    pub state: P11EvidenceState,
    pub authority: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum P11ManualEvidenceDisposition {
    Applied(P11ManualEvidenceReceipt),
    AlreadyPresent(P11ManualEvidenceReceipt),
}

pub fn p11_identity_subject_digest(
    tenant_id: &str,
    workspace_id: &str,
    agent_id: &str,
    service_id: &str,
    node_id: &str,
    generation: u64,
) -> P11Result<Sha256Digest> {
    #[derive(Serialize)]
    struct IdentitySubject<'a> {
        tenant_id: &'a str,
        workspace_id: &'a str,
        agent_id: &'a str,
        service_id: &'a str,
        node_id: &'a str,
        generation: u64,
    }

    for value in [tenant_id, workspace_id, agent_id, service_id, node_id] {
        validate_text(value)?;
    }
    if generation == 0 {
        return Err(P11Error::InvalidInput);
    }
    let json = serde_json::to_vec(&IdentitySubject {
        tenant_id,
        workspace_id,
        agent_id,
        service_id,
        node_id,
        generation,
    })
    .map_err(|_| P11Error::InvalidInput)?;
    Ok(contract_style_domain_digest(
        "basil-identity-subject",
        &json,
    ))
}

pub(crate) fn validate_text(value: &str) -> P11Result<()> {
    if value.trim().is_empty() || value.len() > MAX_TEXT_BYTES || value.as_bytes().contains(&0) {
        return Err(P11Error::InvalidInput);
    }
    Ok(())
}

pub(crate) fn validate_digest(value: &Sha256Digest) -> P11Result<()> {
    Sha256Digest::parse(value.as_str().to_owned())
        .map(|_| ())
        .map_err(|_| P11Error::InvalidInput)
}

pub(crate) fn validate_signature(value: &[u8]) -> P11Result<()> {
    if value.len() != ED25519_SIGNATURE_BYTES {
        return Err(P11Error::InvalidInput);
    }
    Ok(())
}

pub(crate) fn digest_serializable<T: Serialize>(
    domain: &str,
    value: &T,
) -> P11Result<Sha256Digest> {
    let json = serde_json::to_vec(value).map_err(|_| P11Error::InvalidInput)?;
    Ok(length_delimited_digest(domain, &[json.as_slice()]))
}

pub(crate) fn length_delimited_digest(domain: &str, fields: &[&[u8]]) -> Sha256Digest {
    let mut bytes = Vec::new();
    push_text(&mut bytes, domain);
    for field in fields {
        push_bytes(&mut bytes, field);
    }
    Sha256Digest::for_bytes(&bytes)
}

fn contract_style_domain_digest(domain: &str, value: &[u8]) -> Sha256Digest {
    let mut bytes = Vec::new();
    push_bytes(&mut bytes, domain.as_bytes());
    push_bytes(&mut bytes, value);
    Sha256Digest::for_bytes(&bytes)
}

pub(crate) fn push_text(bytes: &mut Vec<u8>, value: &str) {
    push_bytes(bytes, value.as_bytes());
}

pub(crate) fn push_bytes(bytes: &mut Vec<u8>, value: &[u8]) {
    bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
    bytes.extend_from_slice(value);
}

pub(crate) fn push_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

pub(crate) fn push_digest(bytes: &mut Vec<u8>, value: &Sha256Digest) {
    push_text(bytes, value.as_str());
}

pub(crate) fn push_fence(bytes: &mut Vec<u8>, value: &P11Fence) {
    push_u64(bytes, value.authority_epoch);
    push_u64(bytes, value.owner_epoch);
    push_u64(bytes, value.generation);
    push_digest(bytes, &value.fencing_token_sha256);
}

pub(crate) fn assert_negative_authority(value: bool) -> P11Result<()> {
    if value || AUTHBUS_P1_1_AUTHORITY {
        return Err(P11Error::InvalidInput);
    }
    Ok(())
}
