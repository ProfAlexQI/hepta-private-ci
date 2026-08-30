//! Qualification-only core contracts for the minimal Hepta local inference controller.
//!
//! The crate deliberately contains no model backend, network client, raw prompt field,
//! production listener authority, Memory/KG writer, or remote inference path.

mod adapter;
mod capability;
mod controller;
mod hashing;
mod identity;
mod model;
mod principal;
mod private_protocol;
mod private_security;
mod protocol;
mod scheduler;
mod security;
mod worker;

use std::fmt;

pub use adapter::AdapterAdmission;
pub use adapter::AdapterCapabilities;
pub use adapter::AdapterId;
pub use adapter::AdapterRegistry;
pub use adapter::CapabilityEvidence;
pub use adapter::DispatchRequirements;
pub use adapter::ExactAdapterTuple;
pub use adapter::FallbackPolicy;
pub use adapter::PolicyProfile;
pub use adapter::QualifiedController;
pub use capability::CAPABILITY_KEY_BYTES;
pub use capability::CapabilityKey;
pub use capability::PRIVATE_AUTH_TAG_BYTES;
pub use capability::RequestGrant;
pub use capability::RequestGrantFence;
pub use capability::WorkerAuthenticationTag;
pub use capability::WorkerHandshakeFence;
pub use controller::AcceptedEvent;
pub use controller::Controller;
pub use controller::ControllerConfig;
pub use controller::ControllerSnapshot;
pub use controller::EventFence;
pub use controller::LifecycleState;
pub use controller::StateEvent;
pub use controller::TerminalReceipt;
pub use identity::AgentId;
pub use identity::Digest;
pub use identity::RequestId;
pub use identity::RequestIdentity;
pub use identity::ResourceBudgetId;
pub use identity::TaskId;
pub use identity::TenantId;
pub use identity::WorkspaceId;
pub use model::AuthoritySnapshot;
pub use model::InferenceRequest;
pub use principal::OperatorPrincipal;
pub use principal::PeerProcessIdentity;
pub use principal::PublicPrincipal;
pub use principal::RequestOwnership;
pub use principal::RequestOwnershipLedger;
pub use private_protocol::MAX_PRIVATE_FRAME_BYTES;
pub use private_protocol::MAX_PRIVATE_TOKEN_BYTES;
pub use private_protocol::OperatorPrivateMessage;
pub use private_protocol::PRIVATE_PROTOCOL_VERSION;
pub use private_protocol::WorkerPrivateMessage;
pub use private_security::GrantConsumption;
pub use private_security::GrantDisposition;
pub use private_security::GrantLedgerSnapshot;
pub use private_security::OperatorAuthenticationTag;
pub use private_security::OperatorCapabilityKey;
pub use private_security::OperatorHandshakeFence;
pub use private_security::RequestGrantLedger;
pub use private_security::SessionNonce;
pub use private_security::generate_request_capability_key_os;
pub use protocol::ClientMessage;
pub use protocol::MAX_FRAME_BYTES;
pub use protocol::PROTOCOL_VERSION;
pub use protocol::ServerMessage;
pub use protocol::token_fence;
pub use scheduler::DeterministicScheduler;
pub use scheduler::ReservationKey;
pub use scheduler::ReservationLedger;
pub use scheduler::ReservationLimits;
pub use scheduler::ReservationPhase;
pub use scheduler::ReservationRequest;
pub use scheduler::ReservationSnapshot;
pub use scheduler::ScheduledRequest;
pub use scheduler::SchedulerConfig;
pub use security::MessageRole;
pub use worker::AbiByteSlice;
pub use worker::AbiOwnedBuffer;
pub use worker::BackendAbiContract;
pub use worker::BackendOperation;
pub use worker::GgufModelManifest;
pub use worker::HEPTA_BACKEND_ABI_NAME;
pub use worker::HEPTA_BACKEND_ABI_VERSION;
pub use worker::LLAMA_CPP_PINNED_COMMIT;
pub use worker::MAX_NATIVE_WORKER_ACTIVE;
pub use worker::MAX_SHARED_REGION_BYTES;
pub use worker::NativeWorkerRegistry;
pub use worker::NativeWorkerRequest;
pub use worker::REQUIRED_BACKEND_OPERATIONS;
pub use worker::SharedRegionDescriptor;
pub use worker::WorkerError;
pub use worker::WorkerFault;
pub use worker::WorkerFaultReceipt;
pub use worker::WorkerHealth;
pub use worker::WorkerQualificationDisposition;
pub use worker::WorkerSnapshot;
pub use worker::WorkerSupervisor;
pub use worker::WorkerTransport;

pub type Result<T> = std::result::Result<T, InferError>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InferError {
    AdapterConfigInvalid,
    AdapterFallbackEnabled,
    AdapterPolicyUnknown,
    AdapterProviderCancelUnsupported,
    AdapterSemanticTextUnsupported,
    AdapterStrictSseUnsupported,
    AdapterToolCallUnsupported,
    AuthorityEscalation,
    InvalidCapability,
    InvalidCapabilityKey,
    DeadlineExpired,
    DuplicateRequest,
    EmptyOutputLimit,
    EmptyPrompt,
    EmptyToken,
    EntropyUnavailable,
    GrantLedgerFull,
    GrantExpiryInvalid,
    DuplicateGrant,
    UnknownGrant,
    GrantAlreadyConsumed,
    GrantRevoked,
    GrantExpired,
    GrantFenceMismatch,
    GenerationOverflow,
    InflightFull,
    InvalidControllerConfig,
    InvalidPrincipal,
    PrincipalBindingMismatch,
    RequestOwnershipMismatch,
    OwnershipLedgerFull,
    DuplicateOwnership,
    UnknownOwnership,
    InvalidDigest,
    InvalidGeneration,
    InvalidIdentity(&'static str),
    InvalidTransition,
    OutputTokenCountMismatch,
    OutputTokenLimitExceeded,
    ProtocolBound,
    ProtocolIndefinite,
    ProtocolNonCanonical,
    ProtocolShape,
    ProtocolTrailingData,
    ProtocolTruncated,
    ProtocolUtf8,
    ProtocolVersion,
    QueueFull,
    QueueInvariant,
    ReceiptSequenceNotReached,
    RequestNotTerminal,
    ResultDigestMismatch,
    RoleNotAuthorized,
    RunningFull,
    SchedulerConfigInvalid,
    SchedulerRequestInvalid,
    SchedulerDuplicateRequest,
    SchedulerQueueFull,
    SchedulerUnknownRequest,
    SchedulerArithmeticOverflow,
    ReservationConfigInvalid,
    ReservationRequestInvalid,
    DuplicateReservation,
    UnknownReservation,
    ReservationGlobalFull,
    ReservationTenantFull,
    ReservationBudgetFull,
    ReservationRunningGlobalFull,
    ReservationRunningTenantFull,
    ReservationRunningTupleFull,
    ReservationPhaseInvalid,
    ReservationArithmeticOverflow,
    ReservationInvariant,
    SequenceOverflow,
    StaleBackendGeneration,
    StaleCancelGeneration,
    StaleOrNonMonotonicSequence,
    StaleRequestGeneration,
    TenantInflightFull,
    TenantQueueFull,
    TenantRunningFull,
    TerminalState,
    UnknownModelTuple,
    UnknownRequest,
    WorkerCancellationRequired,
}

impl InferError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::AdapterConfigInvalid => "INF_ADAPTER_CONFIG_INVALID",
            Self::AdapterFallbackEnabled => "INF_ADAPTER_FALLBACK_ENABLED",
            Self::AdapterPolicyUnknown => "INF_ADAPTER_POLICY_UNKNOWN",
            Self::AdapterProviderCancelUnsupported => "INF_ADAPTER_PROVIDER_CANCEL_UNSUPPORTED",
            Self::AdapterSemanticTextUnsupported => "INF_ADAPTER_SEMANTIC_TEXT_UNSUPPORTED",
            Self::AdapterStrictSseUnsupported => "INF_ADAPTER_STRICT_SSE_UNSUPPORTED",
            Self::AdapterToolCallUnsupported => "INF_ADAPTER_TOOL_CALL_UNSUPPORTED",
            Self::AuthorityEscalation => "INF_AUTHORITY_ESCALATION",
            Self::InvalidCapability => "INF_INVALID_CAPABILITY",
            Self::InvalidCapabilityKey => "INF_INVALID_CAPABILITY_KEY",
            Self::DeadlineExpired => "INF_DEADLINE_EXPIRED",
            Self::DuplicateRequest => "INF_DUPLICATE_REQUEST",
            Self::EmptyOutputLimit => "INF_EMPTY_OUTPUT_LIMIT",
            Self::EmptyPrompt => "INF_EMPTY_PROMPT",
            Self::EmptyToken => "INF_EMPTY_TOKEN",
            Self::EntropyUnavailable => "INF_ENTROPY_UNAVAILABLE",
            Self::GrantLedgerFull => "INF_GRANT_LEDGER_FULL",
            Self::GrantExpiryInvalid => "INF_GRANT_EXPIRY_INVALID",
            Self::DuplicateGrant => "INF_DUPLICATE_GRANT",
            Self::UnknownGrant => "INF_UNKNOWN_GRANT",
            Self::GrantAlreadyConsumed => "INF_GRANT_ALREADY_CONSUMED",
            Self::GrantRevoked => "INF_GRANT_REVOKED",
            Self::GrantExpired => "INF_GRANT_EXPIRED",
            Self::GrantFenceMismatch => "INF_GRANT_FENCE_MISMATCH",
            Self::GenerationOverflow => "INF_GENERATION_OVERFLOW",
            Self::InflightFull => "INF_INFLIGHT_FULL",
            Self::InvalidControllerConfig => "INF_INVALID_CONTROLLER_CONFIG",
            Self::InvalidPrincipal => "INF_INVALID_PRINCIPAL",
            Self::PrincipalBindingMismatch => "INF_PRINCIPAL_BINDING_MISMATCH",
            Self::RequestOwnershipMismatch => "INF_REQUEST_OWNERSHIP_MISMATCH",
            Self::OwnershipLedgerFull => "INF_OWNERSHIP_LEDGER_FULL",
            Self::DuplicateOwnership => "INF_DUPLICATE_OWNERSHIP",
            Self::UnknownOwnership => "INF_UNKNOWN_OWNERSHIP",
            Self::InvalidDigest => "INF_INVALID_DIGEST",
            Self::InvalidGeneration => "INF_INVALID_GENERATION",
            Self::InvalidIdentity(_) => "INF_INVALID_IDENTITY",
            Self::InvalidTransition => "INF_INVALID_TRANSITION",
            Self::OutputTokenCountMismatch => "INF_OUTPUT_TOKEN_COUNT_MISMATCH",
            Self::OutputTokenLimitExceeded => "INF_OUTPUT_TOKEN_LIMIT_EXCEEDED",
            Self::ProtocolBound => "INF_PROTOCOL_BOUND",
            Self::ProtocolIndefinite => "INF_PROTOCOL_INDEFINITE",
            Self::ProtocolNonCanonical => "INF_PROTOCOL_NON_CANONICAL",
            Self::ProtocolShape => "INF_PROTOCOL_SHAPE",
            Self::ProtocolTrailingData => "INF_PROTOCOL_TRAILING_DATA",
            Self::ProtocolTruncated => "INF_PROTOCOL_TRUNCATED",
            Self::ProtocolUtf8 => "INF_PROTOCOL_UTF8",
            Self::ProtocolVersion => "INF_PROTOCOL_VERSION",
            Self::QueueFull => "INF_QUEUE_FULL",
            Self::QueueInvariant => "INF_QUEUE_INVARIANT",
            Self::ReceiptSequenceNotReached => "INF_RECEIPT_SEQUENCE_NOT_REACHED",
            Self::RequestNotTerminal => "INF_REQUEST_NOT_TERMINAL",
            Self::ResultDigestMismatch => "INF_RESULT_DIGEST_MISMATCH",
            Self::RoleNotAuthorized => "INF_ROLE_NOT_AUTHORIZED",
            Self::RunningFull => "INF_RUNNING_FULL",
            Self::SchedulerConfigInvalid => "INF_SCHEDULER_CONFIG_INVALID",
            Self::SchedulerRequestInvalid => "INF_SCHEDULER_REQUEST_INVALID",
            Self::SchedulerDuplicateRequest => "INF_SCHEDULER_DUPLICATE_REQUEST",
            Self::SchedulerQueueFull => "INF_SCHEDULER_QUEUE_FULL",
            Self::SchedulerUnknownRequest => "INF_SCHEDULER_UNKNOWN_REQUEST",
            Self::SchedulerArithmeticOverflow => "INF_SCHEDULER_ARITHMETIC_OVERFLOW",
            Self::ReservationConfigInvalid => "INF_RESERVATION_CONFIG_INVALID",
            Self::ReservationRequestInvalid => "INF_RESERVATION_REQUEST_INVALID",
            Self::DuplicateReservation => "INF_DUPLICATE_RESERVATION",
            Self::UnknownReservation => "INF_UNKNOWN_RESERVATION",
            Self::ReservationGlobalFull => "INF_RESERVATION_GLOBAL_FULL",
            Self::ReservationTenantFull => "INF_RESERVATION_TENANT_FULL",
            Self::ReservationBudgetFull => "INF_RESERVATION_BUDGET_FULL",
            Self::ReservationRunningGlobalFull => "INF_RESERVATION_RUNNING_GLOBAL_FULL",
            Self::ReservationRunningTenantFull => "INF_RESERVATION_RUNNING_TENANT_FULL",
            Self::ReservationRunningTupleFull => "INF_RESERVATION_RUNNING_TUPLE_FULL",
            Self::ReservationPhaseInvalid => "INF_RESERVATION_PHASE_INVALID",
            Self::ReservationArithmeticOverflow => "INF_RESERVATION_ARITHMETIC_OVERFLOW",
            Self::ReservationInvariant => "INF_RESERVATION_INVARIANT",
            Self::SequenceOverflow => "INF_SEQUENCE_OVERFLOW",
            Self::StaleBackendGeneration => "INF_STALE_BACKEND_GENERATION",
            Self::StaleCancelGeneration => "INF_STALE_CANCEL_GENERATION",
            Self::StaleOrNonMonotonicSequence => "INF_STALE_OR_NON_MONOTONIC_SEQUENCE",
            Self::StaleRequestGeneration => "INF_STALE_REQUEST_GENERATION",
            Self::TenantInflightFull => "INF_TENANT_INFLIGHT_FULL",
            Self::TenantQueueFull => "INF_TENANT_QUEUE_FULL",
            Self::TenantRunningFull => "INF_TENANT_RUNNING_FULL",
            Self::TerminalState => "INF_TERMINAL_STATE",
            Self::UnknownModelTuple => "INF_UNKNOWN_MODEL_TUPLE",
            Self::UnknownRequest => "INF_UNKNOWN_REQUEST",
            Self::WorkerCancellationRequired => "INF_WORKER_CANCELLATION_REQUIRED",
        }
    }
}

impl fmt::Display for InferError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity(label) => write!(formatter, "{}: {label}", self.code()),
            _ => formatter.write_str(self.code()),
        }
    }
}

impl std::error::Error for InferError {}

#[cfg(test)]
mod tests;
