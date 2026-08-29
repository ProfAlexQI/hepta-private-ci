//! Qualification-only core contracts for the minimal Hepta local inference controller.
//!
//! The crate deliberately contains no model backend, network client, raw prompt field,
//! production listener authority, Memory/KG writer, or remote inference path.

mod controller;
mod identity;
mod model;
mod protocol;

use std::fmt;

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
pub use protocol::ClientMessage;
pub use protocol::MAX_FRAME_BYTES;
pub use protocol::PROTOCOL_VERSION;
pub use protocol::ServerMessage;
pub use protocol::token_fence;

pub type Result<T> = std::result::Result<T, InferError>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InferError {
    AuthorityEscalation,
    DeadlineExpired,
    DuplicateRequest,
    EmptyOutputLimit,
    EmptyPrompt,
    EmptyToken,
    GenerationOverflow,
    InvalidControllerConfig,
    InvalidDigest,
    InvalidGeneration,
    InvalidIdentity(&'static str),
    InvalidTransition,
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
    RequestNotTerminal,
    SequenceOverflow,
    StaleBackendGeneration,
    StaleCancelGeneration,
    StaleOrNonMonotonicSequence,
    StaleRequestGeneration,
    TenantQueueFull,
    TerminalState,
    UnknownModelTuple,
    UnknownRequest,
}

impl InferError {
    pub const fn code(&self) -> &'static str {
        match self {
            Self::AuthorityEscalation => "INF_AUTHORITY_ESCALATION",
            Self::DeadlineExpired => "INF_DEADLINE_EXPIRED",
            Self::DuplicateRequest => "INF_DUPLICATE_REQUEST",
            Self::EmptyOutputLimit => "INF_EMPTY_OUTPUT_LIMIT",
            Self::EmptyPrompt => "INF_EMPTY_PROMPT",
            Self::EmptyToken => "INF_EMPTY_TOKEN",
            Self::GenerationOverflow => "INF_GENERATION_OVERFLOW",
            Self::InvalidControllerConfig => "INF_INVALID_CONTROLLER_CONFIG",
            Self::InvalidDigest => "INF_INVALID_DIGEST",
            Self::InvalidGeneration => "INF_INVALID_GENERATION",
            Self::InvalidIdentity(_) => "INF_INVALID_IDENTITY",
            Self::InvalidTransition => "INF_INVALID_TRANSITION",
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
            Self::RequestNotTerminal => "INF_REQUEST_NOT_TERMINAL",
            Self::SequenceOverflow => "INF_SEQUENCE_OVERFLOW",
            Self::StaleBackendGeneration => "INF_STALE_BACKEND_GENERATION",
            Self::StaleCancelGeneration => "INF_STALE_CANCEL_GENERATION",
            Self::StaleOrNonMonotonicSequence => "INF_STALE_OR_NON_MONOTONIC_SEQUENCE",
            Self::StaleRequestGeneration => "INF_STALE_REQUEST_GENERATION",
            Self::TenantQueueFull => "INF_TENANT_QUEUE_FULL",
            Self::TerminalState => "INF_TERMINAL_STATE",
            Self::UnknownModelTuple => "INF_UNKNOWN_MODEL_TUPLE",
            Self::UnknownRequest => "INF_UNKNOWN_REQUEST",
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
