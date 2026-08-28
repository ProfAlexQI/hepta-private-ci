use codex_hepta_contracts::QuotaVector;
use codex_hepta_contracts::RefreshWithSecretRefRequest;
use codex_hepta_contracts::RotateSecretRefRequest;
use codex_hepta_contracts::Sha256Digest;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

const MAX_ID_BYTES: usize = 512;
const MAX_REASON_BYTES: usize = 1_024;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum QualificationError {
    #[error("AuthBus P0.2 qualification input is invalid")]
    InvalidInput,
    #[error("AuthBus P0.2 operation was not found")]
    NotFound,
    #[error("AuthBus P0.2 idempotency or operation binding conflicts")]
    Conflict,
    #[error("AuthBus P0.2 token-family claim is already active")]
    ActiveClaim,
    #[error("AuthBus P0.2 operation revision is stale")]
    StaleRevision,
    #[error("AuthBus P0.2 writer boot or generation is stale")]
    StaleWriter,
    #[error("AuthBus P0.2 fence is stale or mismatched")]
    StaleFence,
    #[error("AuthBus P0.2 state transition is not permitted")]
    InvalidTransition,
    #[error("AuthBus P0.2 status observation is stale")]
    StaleObservation,
    #[error("AuthBus P0.2 status observation conflicts with durable evidence")]
    ObservationConflict,
    #[error("AuthBus P0.2 terminal state is immutable")]
    TerminalImmutable,
    #[error("AuthBus P0.2 outbox cursor conflicts")]
    CursorConflict,
    #[error("AuthBus P0.2 qualification database is full")]
    StorageFull,
    #[error("AuthBus P0.2 qualification storage is unavailable")]
    StorageUnavailable,
    #[error("AuthBus P0.2 qualification evidence is corrupt")]
    Corrupt,
    #[error("AuthBus P0.2 deterministic disk-full failpoint fired")]
    InjectedDiskFull,
}

pub type QualificationResult<T> = Result<T, QualificationError>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WriterIdentity {
    pub boot_id: String,
    pub generation: u64,
}

impl WriterIdentity {
    pub fn new(boot_id: impl Into<String>, generation: u64) -> QualificationResult<Self> {
        let value = Self {
            boot_id: boot_id.into(),
            generation,
        };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> QualificationResult<()> {
        validate_identifier(&self.boot_id)?;
        if self.generation == 0 {
            return Err(QualificationError::InvalidInput);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct QualificationFence {
    pub authority_epoch: u64,
    pub owner_epoch: u64,
    pub generation: u64,
    pub fencing_token_sha256: Sha256Digest,
}

impl QualificationFence {
    pub fn validate(&self) -> QualificationResult<()> {
        if self.authority_epoch == 0 || self.owner_epoch == 0 || self.generation == 0 {
            return Err(QualificationError::InvalidInput);
        }
        Ok(())
    }

    fn from_refresh(request: &RefreshWithSecretRefRequest) -> Self {
        Self {
            authority_epoch: request.authority_epoch,
            owner_epoch: request.owner_epoch,
            generation: request.generation,
            fencing_token_sha256: Sha256Digest::for_bytes(request.fencing_token.as_bytes()),
        }
    }

    fn from_rotate(request: &RotateSecretRefRequest) -> Self {
        Self {
            authority_epoch: request.authority_epoch,
            owner_epoch: request.owner_epoch,
            generation: request.generation,
            fencing_token_sha256: Sha256Digest::for_bytes(request.fencing_token.as_bytes()),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct QualificationQuota {
    pub rpm: u64,
    pub tpm: u64,
    pub concurrency: u64,
    pub day_budget: u64,
    pub context: u64,
}

impl From<QuotaVector> for QualificationQuota {
    fn from(value: QuotaVector) -> Self {
        Self {
            rpm: value.rpm,
            tpm: value.tpm,
            concurrency: value.concurrency,
            day_budget: value.day_budget,
            context: value.context,
        }
    }
}

impl QualificationQuota {
    pub fn validate_reservation(self) -> QualificationResult<()> {
        if self.rpm == 0
            && self.tpm == 0
            && self.concurrency == 0
            && self.day_budget == 0
            && self.context == 0
        {
            return Err(QualificationError::InvalidInput);
        }
        Ok(())
    }

    pub fn fits_within(self, reservation: Self) -> bool {
        self.rpm <= reservation.rpm
            && self.tpm <= reservation.tpm
            && self.concurrency <= reservation.concurrency
            && self.day_budget <= reservation.day_budget
            && self.context <= reservation.context
    }

    pub fn terminal_usage(self) -> Self {
        Self {
            concurrency: 0,
            ..self
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct QualificationPermit {
    pub permit_id: String,
    pub command_id: String,
    pub idempotency_key: String,
    pub resource_id: String,
    pub resource_sha256: Sha256Digest,
    pub reserved: QualificationQuota,
    pub fence: QualificationFence,
    pub issued_at_ms: u64,
    pub expires_at_ms: u64,
    pub authority: bool,
}

impl QualificationPermit {
    pub fn validate(&self, created_at_ms: u64) -> QualificationResult<()> {
        validate_identifier(&self.permit_id)?;
        validate_identifier(&self.command_id)?;
        validate_identifier(&self.idempotency_key)?;
        validate_identifier(&self.resource_id)?;
        self.reserved.validate_reservation()?;
        self.fence.validate()?;
        if self.authority
            || self.issued_at_ms == 0
            || self.expires_at_ms <= self.issued_at_ms
            || created_at_ms < self.issued_at_ms
            || created_at_ms >= self.expires_at_ms
        {
            return Err(QualificationError::InvalidInput);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QualificationOperationKind {
    Refresh,
    Rotate,
}

impl QualificationOperationKind {
    pub(crate) fn as_db(self) -> &'static str {
        match self {
            Self::Refresh => "REFRESH",
            Self::Rotate => "ROTATE",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DurableOperationIntent {
    pub kind: QualificationOperationKind,
    pub operation_id: String,
    pub operation_key: String,
    pub effect_key: String,
    pub command_id: String,
    pub run_id: String,
    pub idempotency_key: String,
    pub provider_id: String,
    pub profile_id: String,
    pub token_family_id: String,
    pub request_json: String,
    pub request_sha256: Sha256Digest,
    pub payload_sha256: Sha256Digest,
    pub policy_sha256: Sha256Digest,
    pub scope_sha256: Sha256Digest,
    pub purpose_sha256: Sha256Digest,
    pub fence: QualificationFence,
    pub created_at_ms: u64,
}

impl DurableOperationIntent {
    pub fn from_refresh(
        request: &RefreshWithSecretRefRequest,
        created_at_ms: u64,
    ) -> QualificationResult<Self> {
        request
            .validate()
            .map_err(|_| QualificationError::InvalidInput)?;
        let request_json =
            serde_json::to_string(request).map_err(|_| QualificationError::InvalidInput)?;
        let request_sha256 = request
            .digest()
            .map_err(|_| QualificationError::InvalidInput)?;
        Self::from_parts(
            QualificationOperationKind::Refresh,
            request.operation_id.clone(),
            request.refresh_operation_key.clone(),
            request.command_id.clone(),
            request.run_id.clone(),
            request.idempotency_key.clone(),
            request.provider_id.clone(),
            request.profile_id.clone(),
            request.token_family_id.clone(),
            request_json,
            request_sha256,
            request.payload_digest.clone(),
            request.policy_digest.clone(),
            request.scope_digest.clone(),
            request.purpose_digest.clone(),
            QualificationFence::from_refresh(request),
            created_at_ms,
        )
    }

    pub fn from_rotate(
        request: &RotateSecretRefRequest,
        created_at_ms: u64,
    ) -> QualificationResult<Self> {
        request
            .validate()
            .map_err(|_| QualificationError::InvalidInput)?;
        let request_json =
            serde_json::to_string(request).map_err(|_| QualificationError::InvalidInput)?;
        let request_sha256 = request
            .digest()
            .map_err(|_| QualificationError::InvalidInput)?;
        Self::from_parts(
            QualificationOperationKind::Rotate,
            request.operation_id.clone(),
            request.refresh_operation_key.clone(),
            request.command_id.clone(),
            request.run_id.clone(),
            request.idempotency_key.clone(),
            request.provider_id.clone(),
            request.profile_id.clone(),
            request.token_family_id.clone(),
            request_json,
            request_sha256,
            request.payload_digest.clone(),
            request.policy_digest.clone(),
            request.scope_digest.clone(),
            request.purpose_digest.clone(),
            QualificationFence::from_rotate(request),
            created_at_ms,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn from_parts(
        kind: QualificationOperationKind,
        operation_id: String,
        operation_key: String,
        command_id: String,
        run_id: String,
        idempotency_key: String,
        provider_id: String,
        profile_id: String,
        token_family_id: String,
        request_json: String,
        request_sha256: Sha256Digest,
        payload_sha256: Sha256Digest,
        policy_sha256: Sha256Digest,
        scope_sha256: Sha256Digest,
        purpose_sha256: Sha256Digest,
        fence: QualificationFence,
        created_at_ms: u64,
    ) -> QualificationResult<Self> {
        let operation_key_sha256 = Sha256Digest::for_bytes(operation_key.as_bytes());
        let value = Self {
            kind,
            operation_id,
            operation_key,
            effect_key: format!("provider-effect:v1:{operation_key_sha256}"),
            command_id,
            run_id,
            idempotency_key,
            provider_id,
            profile_id,
            token_family_id,
            request_json,
            request_sha256,
            payload_sha256,
            policy_sha256,
            scope_sha256,
            purpose_sha256,
            fence,
            created_at_ms,
        };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> QualificationResult<()> {
        validate_identifier(&self.operation_id)?;
        validate_identifier(&self.operation_key)?;
        validate_identifier(&self.effect_key)?;
        validate_identifier(&self.command_id)?;
        validate_identifier(&self.run_id)?;
        validate_identifier(&self.idempotency_key)?;
        validate_identifier(&self.provider_id)?;
        validate_identifier(&self.profile_id)?;
        validate_identifier(&self.token_family_id)?;
        self.fence.validate()?;
        if self.request_json.is_empty()
            || self.request_json.len() > 32 * 1_024
            || self.created_at_ms == 0
        {
            return Err(QualificationError::InvalidInput);
        }
        Ok(())
    }

    pub fn claim_key_sha256(&self) -> Sha256Digest {
        digest_length_delimited(
            "hepta.authbus.p0.2.claim-key.v1",
            &[
                self.provider_id.as_bytes(),
                self.profile_id.as_bytes(),
                self.token_family_id.as_bytes(),
            ],
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct QualificationAdmission {
    pub intent: DurableOperationIntent,
    pub permit: QualificationPermit,
}

impl QualificationAdmission {
    pub fn new(
        intent: DurableOperationIntent,
        permit: QualificationPermit,
    ) -> QualificationResult<Self> {
        let value = Self { intent, permit };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> QualificationResult<()> {
        self.intent.validate()?;
        self.permit.validate(self.intent.created_at_ms)?;
        if self.intent.command_id != self.permit.command_id
            || self.intent.idempotency_key != self.permit.idempotency_key
            || self.intent.fence != self.permit.fence
        {
            return Err(QualificationError::InvalidInput);
        }
        Ok(())
    }

    pub fn intent_sha256(&self) -> QualificationResult<Sha256Digest> {
        digest_serializable("hepta.authbus.p0.2.admission.v1", self)
    }

    pub fn status_binding_sha256(&self) -> QualificationResult<Sha256Digest> {
        #[derive(Serialize)]
        struct Binding<'a> {
            operation_id: &'a str,
            operation_key: &'a str,
            effect_key: &'a str,
            admission_sha256: Sha256Digest,
            fence: &'a QualificationFence,
        }
        let admission_sha256 = self.intent_sha256()?;
        digest_serializable(
            "hepta.authbus.p0.2.status-binding.v1",
            &Binding {
                operation_id: &self.intent.operation_id,
                operation_key: &self.intent.operation_key,
                effect_key: &self.intent.effect_key,
                admission_sha256,
                fence: &self.intent.fence,
            },
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OperationState {
    IntentDurable,
    AttemptStarted,
    Accepted,
    Unknown,
    Indeterminate,
    Completed,
    Rejected,
    Quarantined,
    ManualRequired,
}

impl OperationState {
    pub(crate) fn as_db(self) -> &'static str {
        match self {
            Self::IntentDurable => "INTENT_DURABLE",
            Self::AttemptStarted => "ATTEMPT_STARTED",
            Self::Accepted => "ACCEPTED",
            Self::Unknown => "UNKNOWN",
            Self::Indeterminate => "INDETERMINATE",
            Self::Completed => "COMPLETED",
            Self::Rejected => "REJECTED",
            Self::Quarantined => "QUARANTINED",
            Self::ManualRequired => "MANUAL_REQUIRED",
        }
    }

    pub(crate) fn from_db(value: &str) -> QualificationResult<Self> {
        match value {
            "INTENT_DURABLE" => Ok(Self::IntentDurable),
            "ATTEMPT_STARTED" => Ok(Self::AttemptStarted),
            "ACCEPTED" => Ok(Self::Accepted),
            "UNKNOWN" => Ok(Self::Unknown),
            "INDETERMINATE" => Ok(Self::Indeterminate),
            "COMPLETED" => Ok(Self::Completed),
            "REJECTED" => Ok(Self::Rejected),
            "QUARANTINED" => Ok(Self::Quarantined),
            "MANUAL_REQUIRED" => Ok(Self::ManualRequired),
            _ => Err(QualificationError::Corrupt),
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(Self::Completed | Self::Rejected | Self::Quarantined, self)
    }

    pub fn requires_lookup_only(self) -> bool {
        matches!(
            Self::AttemptStarted | Self::Accepted | Self::Unknown | Self::Indeterminate,
            self
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdmissionDisposition {
    Inserted,
    AlreadyPresent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WriteDisposition {
    Applied,
    AlreadyPresent,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DispatchTicket {
    pub operation_id: String,
    pub attempt: u32,
    pub operation_revision: u64,
    pub writer: WriterIdentity,
    pub fence: QualificationFence,
    pub witness_sequence: u64,
    pub witness_sha256: Sha256Digest,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DispatchObservation {
    Accepted {
        provider_operation_sha256: Sha256Digest,
        observed_at_ms: u64,
    },
    Completed {
        provider_operation_sha256: Sha256Digest,
        actual: QualificationQuota,
        observed_at_ms: u64,
    },
    VerifiedNoEffect {
        terminal: VerifiedNoEffectTerminal,
        provider_observation_sha256: Sha256Digest,
        observed_at_ms: u64,
    },
    Unknown {
        reason_code: String,
        observed_at_ms: u64,
    },
}

impl DispatchObservation {
    pub(crate) fn validate(&self, reserved: QualificationQuota) -> QualificationResult<()> {
        let observed_at_ms = match self {
            Self::Accepted { observed_at_ms, .. }
            | Self::Completed { observed_at_ms, .. }
            | Self::VerifiedNoEffect { observed_at_ms, .. }
            | Self::Unknown { observed_at_ms, .. } => *observed_at_ms,
        };
        if observed_at_ms == 0 {
            return Err(QualificationError::InvalidInput);
        }
        match self {
            Self::Completed { actual, .. } if !actual.fits_within(reserved) => {
                Err(QualificationError::InvalidInput)
            }
            Self::Unknown { reason_code, .. } => validate_reason(reason_code),
            _ => Ok(()),
        }
    }

    pub(crate) fn target_state(&self) -> OperationState {
        match self {
            Self::Accepted { .. } => OperationState::Accepted,
            Self::Completed { .. } => OperationState::Completed,
            Self::VerifiedNoEffect { terminal, .. } => match terminal {
                VerifiedNoEffectTerminal::Rejected => OperationState::Rejected,
                VerifiedNoEffectTerminal::Quarantined => OperationState::Quarantined,
            },
            Self::Unknown { .. } => OperationState::Unknown,
        }
    }

    pub(crate) fn observed_at_ms(&self) -> u64 {
        match self {
            Self::Accepted { observed_at_ms, .. }
            | Self::Completed { observed_at_ms, .. }
            | Self::VerifiedNoEffect { observed_at_ms, .. }
            | Self::Unknown { observed_at_ms, .. } => *observed_at_ms,
        }
    }

    pub(crate) fn event_kind(&self) -> &'static str {
        match self {
            Self::Accepted { .. } => "DISPATCH_ACCEPTED",
            Self::Completed { .. } => "DISPATCH_COMPLETED",
            Self::VerifiedNoEffect { terminal, .. } => match terminal {
                VerifiedNoEffectTerminal::Rejected => "DISPATCH_REJECTED_NO_EFFECT",
                VerifiedNoEffectTerminal::Quarantined => "DISPATCH_QUARANTINED_NO_EFFECT",
            },
            Self::Unknown { .. } => "DISPATCH_UNKNOWN",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerifiedNoEffectTerminal {
    Rejected,
    Quarantined,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LookupOutcome {
    Completed {
        provider_operation_sha256: Sha256Digest,
        actual: QualificationQuota,
    },
    VerifiedNoEffect {
        terminal: VerifiedNoEffectTerminal,
        provider_observation_sha256: Sha256Digest,
    },
    Unknown {
        reason_code: String,
    },
    Indeterminate {
        reason_code: String,
    },
}

impl LookupOutcome {
    pub(crate) fn validate(&self, reserved: QualificationQuota) -> QualificationResult<()> {
        match self {
            Self::Completed { actual, .. } if !actual.fits_within(reserved) => {
                Err(QualificationError::InvalidInput)
            }
            Self::Unknown { reason_code } | Self::Indeterminate { reason_code } => {
                validate_reason(reason_code)
            }
            _ => Ok(()),
        }
    }

    pub(crate) fn target_state(&self) -> OperationState {
        match self {
            Self::Completed { .. } => OperationState::Completed,
            Self::VerifiedNoEffect { terminal, .. } => match terminal {
                VerifiedNoEffectTerminal::Rejected => OperationState::Rejected,
                VerifiedNoEffectTerminal::Quarantined => OperationState::Quarantined,
            },
            Self::Unknown { .. } => OperationState::Unknown,
            Self::Indeterminate { .. } => OperationState::Indeterminate,
        }
    }

    pub(crate) fn event_kind(&self) -> &'static str {
        match self {
            Self::Completed { .. } => "LOOKUP_COMPLETED",
            Self::VerifiedNoEffect { terminal, .. } => match terminal {
                VerifiedNoEffectTerminal::Rejected => "LOOKUP_REJECTED_NO_EFFECT",
                VerifiedNoEffectTerminal::Quarantined => "LOOKUP_QUARANTINED_NO_EFFECT",
            },
            Self::Unknown { .. } => "LOOKUP_UNKNOWN",
            Self::Indeterminate { .. } => "LOOKUP_INDETERMINATE",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StatusObservation {
    pub operation_id: String,
    pub status_revision: u64,
    pub observed_at_ms: u64,
    pub binding_sha256: Sha256Digest,
    pub fence: QualificationFence,
    pub outcome: LookupOutcome,
}

impl StatusObservation {
    pub fn validate(&self, reserved: QualificationQuota) -> QualificationResult<()> {
        validate_identifier(&self.operation_id)?;
        self.fence.validate()?;
        self.outcome.validate(reserved)?;
        if self.status_revision == 0 || self.observed_at_ms == 0 {
            return Err(QualificationError::InvalidInput);
        }
        Ok(())
    }

    pub fn digest(&self) -> QualificationResult<Sha256Digest> {
        digest_serializable("hepta.authbus.p0.2.status-observation.v1", self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecoveryAction {
    DispatchEligible {
        operation_id: String,
        revision: u64,
    },
    LookupOnly {
        operation_id: String,
        attempt: u32,
        revision: u64,
    },
    Terminal {
        operation_id: String,
        state: OperationState,
        revision: u64,
    },
    SafeStop {
        operation_id: String,
        state: OperationState,
        revision: u64,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationSnapshot {
    pub operation_id: String,
    pub state: OperationState,
    pub revision: u64,
    pub attempt: u32,
    pub last_status_revision: Option<u64>,
    pub last_observed_at_ms: Option<u64>,
    pub writer: WriterIdentity,
    pub fence: QualificationFence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuotaReservationState {
    Held,
    Completed,
    Released,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuotaSnapshot {
    pub operation_id: String,
    pub reserved: QualificationQuota,
    pub used: QualificationQuota,
    pub state: QuotaReservationState,
    pub revision: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutboxRecord {
    pub sequence: u64,
    pub outbox_id: String,
    pub operation_id: String,
    pub operation_revision: u64,
    pub event_kind: String,
    pub idempotency_key: String,
    pub payload_sha256: Sha256Digest,
    pub payload_json: String,
    pub ack_sha256: Option<Sha256Digest>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QualificationFailpoint {
    AdmissionBeforeCommit,
    DispatchAttemptBeforeCommit,
    DispatchMarkerBeforeCommit,
    StatusObservationBeforeCommit,
    OutboxAckBeforeCommit,
}

impl QualificationFailpoint {
    pub(crate) fn bit(self) -> u64 {
        match self {
            Self::AdmissionBeforeCommit => 1 << 0,
            Self::DispatchAttemptBeforeCommit => 1 << 1,
            Self::DispatchMarkerBeforeCommit => 1 << 2,
            Self::StatusObservationBeforeCommit => 1 << 3,
            Self::OutboxAckBeforeCommit => 1 << 4,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegrityReport {
    pub operations: u64,
    pub active_claims: u64,
    pub held_reservations: u64,
    pub pending_outbox: u64,
    pub fsync_receipts: u64,
}

pub(crate) fn digest_serializable<T: Serialize>(
    domain: &str,
    value: &T,
) -> QualificationResult<Sha256Digest> {
    let json = serde_json::to_vec(value).map_err(|_| QualificationError::InvalidInput)?;
    Ok(digest_length_delimited(domain, &[json.as_slice()]))
}

pub(crate) fn digest_length_delimited(domain: &str, fields: &[&[u8]]) -> Sha256Digest {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(domain.as_bytes());
    bytes.push(0);
    for field in fields {
        bytes.extend_from_slice(&(field.len() as u64).to_be_bytes());
        bytes.extend_from_slice(field);
    }
    Sha256Digest::for_bytes(&bytes)
}

pub(crate) fn validate_identifier(value: &str) -> QualificationResult<()> {
    if value.is_empty() || value.len() > MAX_ID_BYTES || value.chars().any(char::is_control) {
        return Err(QualificationError::InvalidInput);
    }
    Ok(())
}

fn validate_reason(value: &str) -> QualificationResult<()> {
    if value.is_empty() || value.len() > MAX_REASON_BYTES || value.chars().any(char::is_control) {
        return Err(QualificationError::InvalidInput);
    }
    Ok(())
}
