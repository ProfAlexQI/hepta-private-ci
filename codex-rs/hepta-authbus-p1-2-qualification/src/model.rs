use codex_hepta_authbus_p1_qualification::P11Error;
use codex_hepta_authbus_p1_qualification::P11EvidenceState;
use codex_hepta_authbus_p1_qualification::P11IdentityVerificationReceipt;
use codex_hepta_authbus_p1_qualification::P11KeyPurpose;
use codex_hepta_authbus_p1_qualification::P11ManualDecision;
use codex_hepta_authbus_p1_qualification::P11ManualEvidenceReceipt;
use codex_hepta_authbus_p1_qualification::P11OperationEvidenceBinding;
use codex_hepta_authbus_p1_qualification::P11ProviderStatusReceipt;
use codex_hepta_authbus_p1_qualification::P11SignedIdentityEvidence;
use codex_hepta_authbus_p1_qualification::P11SignedManualEvidence;
use codex_hepta_authbus_p1_qualification::P11SignedProviderStatusEvidence;
use codex_hepta_authbus_p1_qualification::P11VerificationKeyRecord;
use codex_hepta_contracts::Sha256Digest;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

pub const AUTHBUS_P1_2_SCHEMA_VERSION: u32 = 1;
const MAX_TEXT_BYTES: usize = 512;
const MAX_GC_ROWS: u64 = 10_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum P12WriteDisposition {
    Applied,
    AlreadyPresent,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum P12Error {
    #[error("AuthBus P1.2 input is invalid")]
    InvalidInput,
    #[error("AuthBus P1.2 durable storage is unavailable")]
    StorageUnavailable,
    #[error("AuthBus P1.2 durable state is corrupt")]
    CorruptState,
    #[error("AuthBus P1.2 writer generation is stale")]
    StaleWriter,
    #[error("AuthBus P1.2 durable policy conflicts with the opened store")]
    PolicyConflict,
    #[error("AuthBus P1.2 key registration conflicts")]
    KeyConflict,
    #[error("AuthBus P1.2 key was not found")]
    UnknownKey,
    #[error("AuthBus P1.2 key epoch is stale")]
    StaleKeyEpoch,
    #[error("AuthBus P1.2 key registry is full")]
    KeyCapacity,
    #[error("AuthBus P1.2 nonce was replayed")]
    NonceReplay,
    #[error("AuthBus P1.2 nonce ledger is full")]
    NonceCapacity,
    #[error("AuthBus P1.2 operation registration conflicts")]
    OperationConflict,
    #[error("AuthBus P1.2 operation ledger is full")]
    OperationCapacity,
    #[error("AuthBus P1.2 operation was not found")]
    UnknownOperation,
    #[error("AuthBus P1.2 evidence binding mismatches")]
    BindingMismatch,
    #[error("AuthBus P1.2 evidence revision or time is stale")]
    StaleObservation,
    #[error("AuthBus P1.2 evidence conflicts with a durable observation")]
    EvidenceConflict,
    #[error("AuthBus P1.2 terminal evidence is immutable")]
    TerminalImmutable,
    #[error("AuthBus P1.2 operation requires independent manual evidence")]
    ManualEvidenceRequired,
    #[error("AuthBus P1.2 manual evidence is not permitted in this state")]
    InvalidManualTransition,
    #[error("AuthBus P1.2 garbage-collection cursor conflicts")]
    GcConflict,
    #[error("AuthBus P1.2 deterministic pre-commit failpoint fired")]
    InjectedFailure,
}

pub type P12Result<T> = Result<T, P12Error>;

impl From<P11Error> for P12Error {
    fn from(_: P11Error) -> Self {
        Self::InvalidInput
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct P12WriterIdentity {
    pub boot_id: String,
    pub generation: u64,
}

impl P12WriterIdentity {
    pub fn validate(&self) -> P12Result<()> {
        validate_text(&self.boot_id)?;
        validate_positive_i64(self.generation)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct P12Policy {
    pub max_key_entries: u64,
    pub max_nonce_entries: u64,
    pub max_operation_entries: u64,
    pub evidence_retention_seconds: u64,
    pub terminal_retention_seconds: u64,
    pub key_retention_seconds: u64,
}

impl Default for P12Policy {
    fn default() -> Self {
        Self {
            max_key_entries: 4_096,
            max_nonce_entries: 4_096,
            max_operation_entries: 4_096,
            evidence_retention_seconds: 86_400,
            terminal_retention_seconds: 604_800,
            key_retention_seconds: 604_800,
        }
    }
}

impl P12Policy {
    pub fn validate(self) -> P12Result<()> {
        for value in [
            self.max_key_entries,
            self.max_nonce_entries,
            self.max_operation_entries,
            self.evidence_retention_seconds,
            self.terminal_retention_seconds,
            self.key_retention_seconds,
        ] {
            validate_positive_i64(value)?;
        }
        if self.terminal_retention_seconds < self.evidence_retention_seconds {
            return Err(P12Error::InvalidInput);
        }
        Ok(())
    }

    pub fn digest(self) -> P12Result<Sha256Digest> {
        self.validate()?;
        digest_serializable("hepta.authbus.p1.2.policy.v1", &self)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum P12Failpoint {
    KeyBeforeCommit,
    NonceBeforeCommit,
    OperationBeforeCommit,
    StatusBeforeCommit,
    ManualBeforeCommit,
    GcBeforeCommit,
    StorageUnavailableBeforeCommit,
}

impl P12Failpoint {
    pub(crate) const fn bit(self) -> u64 {
        match self {
            Self::KeyBeforeCommit => 1 << 0,
            Self::NonceBeforeCommit => 1 << 1,
            Self::OperationBeforeCommit => 1 << 2,
            Self::StatusBeforeCommit => 1 << 3,
            Self::ManualBeforeCommit => 1 << 4,
            Self::GcBeforeCommit => 1 << 5,
            Self::StorageUnavailableBeforeCommit => 1 << 6,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct P12NonceClaim {
    pub schema_version: u32,
    pub nonce_key_sha256: Sha256Digest,
    pub evidence_sha256: Sha256Digest,
    pub binding_sha256: Sha256Digest,
    pub issuer_id: String,
    pub key_id: String,
    pub key_epoch: u64,
    pub subject_sha256: Sha256Digest,
    pub nonce_sha256: Sha256Digest,
    pub launch_nonce_sha256: Sha256Digest,
    pub audience: String,
    pub expires_at_unix_seconds: u64,
    pub claimed_at_unix_seconds: u64,
    #[serde(default)]
    pub authority: bool,
}

impl P12NonceClaim {
    pub fn from_verified(
        evidence: &P11SignedIdentityEvidence,
        receipt: &P11IdentityVerificationReceipt,
        claimed_at_unix_seconds: u64,
    ) -> P12Result<Self> {
        if receipt.authority || claimed_at_unix_seconds == 0 {
            return Err(P12Error::InvalidInput);
        }
        let evidence_sha256 = evidence.evidence_digest()?;
        let binding_sha256 = evidence
            .binding
            .digest()
            .map_err(|_| P12Error::InvalidInput)?;
        if receipt.evidence_sha256 != evidence_sha256
            || receipt.binding_sha256 != binding_sha256
            || receipt.issuer_id != evidence.issuer_id
            || receipt.key_id != evidence.key_id
            || receipt.key_epoch != evidence.key_epoch
            || receipt.subject_sha256 != evidence.binding.subject_digest
            || receipt.nonce_sha256 != evidence.binding.nonce_sha256
            || receipt.launch_nonce_sha256 != evidence.binding.launch_nonce_sha256
            || receipt.expires_at_unix_seconds != evidence.binding.expires_at_unix_seconds
            || claimed_at_unix_seconds >= receipt.expires_at_unix_seconds
        {
            return Err(P12Error::BindingMismatch);
        }

        let nonce_key_sha256 = identity_nonce_key(evidence, &binding_sha256);
        let claim = Self {
            schema_version: AUTHBUS_P1_2_SCHEMA_VERSION,
            nonce_key_sha256,
            evidence_sha256,
            binding_sha256,
            issuer_id: evidence.issuer_id.clone(),
            key_id: evidence.key_id.clone(),
            key_epoch: evidence.key_epoch,
            subject_sha256: receipt.subject_sha256.clone(),
            nonce_sha256: receipt.nonce_sha256.clone(),
            launch_nonce_sha256: receipt.launch_nonce_sha256.clone(),
            audience: evidence.binding.audience.clone(),
            expires_at_unix_seconds: receipt.expires_at_unix_seconds,
            claimed_at_unix_seconds,
            authority: false,
        };
        claim.validate()?;
        Ok(claim)
    }

    pub fn validate(&self) -> P12Result<()> {
        if self.schema_version != AUTHBUS_P1_2_SCHEMA_VERSION || self.authority {
            return Err(P12Error::InvalidInput);
        }
        for value in [&self.issuer_id, &self.key_id, &self.audience] {
            validate_text(value)?;
        }
        for digest in [
            &self.nonce_key_sha256,
            &self.evidence_sha256,
            &self.binding_sha256,
            &self.subject_sha256,
            &self.nonce_sha256,
            &self.launch_nonce_sha256,
        ] {
            validate_digest(digest)?;
        }
        validate_positive_i64(self.key_epoch)?;
        validate_positive_i64(self.claimed_at_unix_seconds)?;
        validate_positive_i64(self.expires_at_unix_seconds)?;
        if self.claimed_at_unix_seconds >= self.expires_at_unix_seconds {
            return Err(P12Error::InvalidInput);
        }
        Ok(())
    }

    pub fn digest(&self) -> P12Result<Sha256Digest> {
        self.validate()?;
        digest_serializable("hepta.authbus.p1.2.nonce-claim.v1", self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct P12ProviderObservation {
    pub schema_version: u32,
    pub evidence_sha256: Sha256Digest,
    pub issuer_id: String,
    pub key_id: String,
    pub key_epoch: u64,
    pub operation_id: String,
    pub provider_id: String,
    pub profile_id: String,
    pub token_family_id: String,
    pub status_binding_sha256: Sha256Digest,
    pub fence: codex_hepta_authbus_p1_qualification::P11Fence,
    pub status_revision: u64,
    pub observed_at_unix_seconds: u64,
    pub state: P11EvidenceState,
    pub outcome: codex_hepta_authbus_p1_qualification::P11ProviderEvidenceOutcome,
    #[serde(default)]
    pub authority: bool,
}

impl P12ProviderObservation {
    pub fn from_verified(
        evidence: &P11SignedProviderStatusEvidence,
        receipt: &P11ProviderStatusReceipt,
    ) -> P12Result<Self> {
        let evidence_sha256 = evidence.evidence_digest()?;
        if receipt.authority
            || receipt.evidence_sha256 != evidence_sha256
            || receipt.operation_id != evidence.operation_id
            || receipt.status_revision != evidence.status_revision
            || receipt.observed_at_unix_seconds != evidence.observed_at_unix_seconds
            || receipt.state != evidence.outcome.target_state()
        {
            return Err(P12Error::BindingMismatch);
        }
        let observation = Self {
            schema_version: AUTHBUS_P1_2_SCHEMA_VERSION,
            evidence_sha256,
            issuer_id: evidence.issuer_id.clone(),
            key_id: evidence.key_id.clone(),
            key_epoch: evidence.key_epoch,
            operation_id: evidence.operation_id.clone(),
            provider_id: evidence.provider_id.clone(),
            profile_id: evidence.profile_id.clone(),
            token_family_id: evidence.token_family_id.clone(),
            status_binding_sha256: evidence.status_binding_sha256.clone(),
            fence: evidence.fence.clone(),
            status_revision: evidence.status_revision,
            observed_at_unix_seconds: evidence.observed_at_unix_seconds,
            state: receipt.state,
            outcome: evidence.outcome.clone(),
            authority: false,
        };
        observation.validate()?;
        Ok(observation)
    }

    pub fn validate(&self) -> P12Result<()> {
        if self.schema_version != AUTHBUS_P1_2_SCHEMA_VERSION || self.authority {
            return Err(P12Error::InvalidInput);
        }
        for value in [
            &self.issuer_id,
            &self.key_id,
            &self.operation_id,
            &self.provider_id,
            &self.profile_id,
            &self.token_family_id,
        ] {
            validate_text(value)?;
        }
        validate_digest(&self.evidence_sha256)?;
        validate_digest(&self.status_binding_sha256)?;
        validate_positive_i64(self.key_epoch)?;
        self.fence.validate()?;
        self.outcome.validate()?;
        validate_positive_i64(self.status_revision)?;
        validate_positive_i64(self.observed_at_unix_seconds)?;
        if self.state != self.outcome.target_state() {
            return Err(P12Error::InvalidInput);
        }
        Ok(())
    }

    pub fn digest(&self) -> P12Result<Sha256Digest> {
        self.validate()?;
        digest_serializable("hepta.authbus.p1.2.provider-observation.v1", self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct P12ManualObservation {
    pub schema_version: u32,
    pub evidence_sha256: Sha256Digest,
    pub issuer_id: String,
    pub key_id: String,
    pub key_epoch: u64,
    pub operation_id: String,
    pub operator_id: String,
    pub case_id: String,
    pub status_binding_sha256: Sha256Digest,
    pub fence: codex_hepta_authbus_p1_qualification::P11Fence,
    pub manual_revision: u64,
    pub observed_at_unix_seconds: u64,
    pub decision: P11ManualDecision,
    pub reason_sha256: Sha256Digest,
    pub state: P11EvidenceState,
    #[serde(default)]
    pub authority: bool,
}

impl P12ManualObservation {
    pub fn from_verified(
        evidence: &P11SignedManualEvidence,
        receipt: &P11ManualEvidenceReceipt,
    ) -> P12Result<Self> {
        let evidence_sha256 = evidence.evidence_digest()?;
        let expected_state = manual_state(evidence.decision);
        if receipt.authority
            || receipt.evidence_sha256 != evidence_sha256
            || receipt.operation_id != evidence.operation_id
            || receipt.manual_revision != evidence.manual_revision
            || receipt.observed_at_unix_seconds != evidence.observed_at_unix_seconds
            || receipt.state != expected_state
        {
            return Err(P12Error::BindingMismatch);
        }
        let observation = Self {
            schema_version: AUTHBUS_P1_2_SCHEMA_VERSION,
            evidence_sha256,
            issuer_id: evidence.issuer_id.clone(),
            key_id: evidence.key_id.clone(),
            key_epoch: evidence.key_epoch,
            operation_id: evidence.operation_id.clone(),
            operator_id: evidence.operator_id.clone(),
            case_id: evidence.case_id.clone(),
            status_binding_sha256: evidence.status_binding_sha256.clone(),
            fence: evidence.fence.clone(),
            manual_revision: evidence.manual_revision,
            observed_at_unix_seconds: evidence.observed_at_unix_seconds,
            decision: evidence.decision,
            reason_sha256: evidence.reason_sha256.clone(),
            state: receipt.state,
            authority: false,
        };
        observation.validate()?;
        Ok(observation)
    }

    pub fn validate(&self) -> P12Result<()> {
        if self.schema_version != AUTHBUS_P1_2_SCHEMA_VERSION || self.authority {
            return Err(P12Error::InvalidInput);
        }
        for value in [
            &self.issuer_id,
            &self.key_id,
            &self.operation_id,
            &self.operator_id,
            &self.case_id,
        ] {
            validate_text(value)?;
        }
        for digest in [
            &self.evidence_sha256,
            &self.status_binding_sha256,
            &self.reason_sha256,
        ] {
            validate_digest(digest)?;
        }
        self.fence.validate()?;
        validate_positive_i64(self.key_epoch)?;
        validate_positive_i64(self.manual_revision)?;
        validate_positive_i64(self.observed_at_unix_seconds)?;
        if self.state != manual_state(self.decision) {
            return Err(P12Error::InvalidInput);
        }
        Ok(())
    }

    pub fn digest(&self) -> P12Result<Sha256Digest> {
        self.validate()?;
        digest_serializable("hepta.authbus.p1.2.manual-observation.v1", self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P12OperationSnapshot {
    pub binding: P11OperationEvidenceBinding,
    pub state: P11EvidenceState,
    pub last_status_revision: Option<u64>,
    pub last_manual_revision: Option<u64>,
    pub last_status_sha256: Option<Sha256Digest>,
    pub last_manual_sha256: Option<Sha256Digest>,
    pub last_observed_at_unix_seconds: Option<u64>,
    pub record_revision: u64,
    pub terminal_retain_until_unix_seconds: Option<u64>,
    pub authority: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct P12GcRequest {
    pub expected_revision: u64,
    pub now_unix_seconds: u64,
    pub max_rows: u64,
}

impl P12GcRequest {
    pub fn validate(self) -> P12Result<()> {
        validate_positive_i64(self.now_unix_seconds)?;
        if self.max_rows == 0 || self.max_rows > MAX_GC_ROWS {
            return Err(P12Error::InvalidInput);
        }
        validate_i64(self.expected_revision)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P12GcReport {
    pub before_revision: u64,
    pub after_revision: u64,
    pub nonce_rows_deleted: u64,
    pub status_rows_deleted: u64,
    pub manual_rows_deleted: u64,
    pub key_rows_deleted: u64,
    pub receipt_rows_deleted: u64,
    pub terminal_operations_deleted: u64,
    pub authority: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P12IntegrityReport {
    pub key_rows: u64,
    pub nonce_rows: u64,
    pub operation_rows: u64,
    pub status_rows: u64,
    pub manual_rows: u64,
    pub tombstone_rows: u64,
    pub receipt_rows: u64,
    pub gc_revision: u64,
    pub authority: bool,
}

pub(crate) fn manual_state(decision: P11ManualDecision) -> P11EvidenceState {
    match decision {
        P11ManualDecision::ResumeLookupOnly => P11EvidenceState::LookupOnly,
        P11ManualDecision::KeepManualRequired => P11EvidenceState::ManualRequired,
        P11ManualDecision::Quarantine => P11EvidenceState::Quarantined,
    }
}

pub(crate) fn key_purpose_name(purpose: P11KeyPurpose) -> &'static str {
    match purpose {
        P11KeyPurpose::IdentityIssuer => "IDENTITY_ISSUER",
        P11KeyPurpose::ProviderStatusIssuer => "PROVIDER_STATUS_ISSUER",
        P11KeyPurpose::OperatorEvidenceIssuer => "OPERATOR_EVIDENCE_ISSUER",
    }
}

pub(crate) fn parse_key_purpose(value: &str) -> P12Result<P11KeyPurpose> {
    match value {
        "IDENTITY_ISSUER" => Ok(P11KeyPurpose::IdentityIssuer),
        "PROVIDER_STATUS_ISSUER" => Ok(P11KeyPurpose::ProviderStatusIssuer),
        "OPERATOR_EVIDENCE_ISSUER" => Ok(P11KeyPurpose::OperatorEvidenceIssuer),
        _ => Err(P12Error::CorruptState),
    }
}

pub(crate) fn evidence_state_name(state: P11EvidenceState) -> &'static str {
    match state {
        P11EvidenceState::Pending => "PENDING",
        P11EvidenceState::Unknown => "UNKNOWN",
        P11EvidenceState::Indeterminate => "INDETERMINATE",
        P11EvidenceState::LookupOnly => "LOOKUP_ONLY",
        P11EvidenceState::ManualRequired => "MANUAL_REQUIRED",
        P11EvidenceState::Completed => "COMPLETED",
        P11EvidenceState::NoEffect => "NO_EFFECT",
        P11EvidenceState::Quarantined => "QUARANTINED",
    }
}

pub(crate) fn parse_evidence_state(value: &str) -> P12Result<P11EvidenceState> {
    match value {
        "PENDING" => Ok(P11EvidenceState::Pending),
        "UNKNOWN" => Ok(P11EvidenceState::Unknown),
        "INDETERMINATE" => Ok(P11EvidenceState::Indeterminate),
        "LOOKUP_ONLY" => Ok(P11EvidenceState::LookupOnly),
        "MANUAL_REQUIRED" => Ok(P11EvidenceState::ManualRequired),
        "COMPLETED" => Ok(P11EvidenceState::Completed),
        "NO_EFFECT" => Ok(P11EvidenceState::NoEffect),
        "QUARANTINED" => Ok(P11EvidenceState::Quarantined),
        _ => Err(P12Error::CorruptState),
    }
}

pub(crate) fn validate_text(value: &str) -> P12Result<()> {
    if value.trim().is_empty() || value.len() > MAX_TEXT_BYTES || value.as_bytes().contains(&0) {
        return Err(P12Error::InvalidInput);
    }
    Ok(())
}

pub(crate) fn validate_digest(value: &Sha256Digest) -> P12Result<()> {
    Sha256Digest::parse(value.as_str().to_owned())
        .map(|_| ())
        .map_err(|_| P12Error::InvalidInput)
}

pub(crate) fn validate_i64(value: u64) -> P12Result<()> {
    i64::try_from(value)
        .map(|_| ())
        .map_err(|_| P12Error::InvalidInput)
}

pub(crate) fn validate_positive_i64(value: u64) -> P12Result<()> {
    if value == 0 {
        return Err(P12Error::InvalidInput);
    }
    validate_i64(value)
}

pub(crate) fn digest_serializable<T: Serialize>(
    domain: &str,
    value: &T,
) -> P12Result<Sha256Digest> {
    let bytes = serde_json::to_vec(value).map_err(|_| P12Error::InvalidInput)?;
    Ok(length_delimited_digest(domain, &[bytes.as_slice()]))
}

pub(crate) fn length_delimited_digest(domain: &str, fields: &[&[u8]]) -> Sha256Digest {
    let mut bytes = Vec::new();
    push_text(&mut bytes, domain);
    for field in fields {
        push_bytes(&mut bytes, field);
    }
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

fn identity_nonce_key(
    evidence: &P11SignedIdentityEvidence,
    binding_sha256: &Sha256Digest,
) -> Sha256Digest {
    let binding = &evidence.binding;
    let mut bytes = Vec::new();
    push_text(&mut bytes, &evidence.issuer_id);
    push_text(&mut bytes, &evidence.key_id);
    push_u64(&mut bytes, evidence.key_epoch);
    push_digest(&mut bytes, &binding.subject_digest);
    push_text(&mut bytes, &binding.audience);
    push_digest(&mut bytes, &binding.nonce_sha256);
    push_digest(&mut bytes, &binding.launch_nonce_sha256);
    push_digest(&mut bytes, binding_sha256);
    length_delimited_digest(
        "hepta.authbus.p1.1.identity-nonce-key.v1",
        &[bytes.as_slice()],
    )
}

pub(crate) fn validate_key_record(record: &P11VerificationKeyRecord) -> P12Result<()> {
    record.validate().map_err(Into::into)
}
