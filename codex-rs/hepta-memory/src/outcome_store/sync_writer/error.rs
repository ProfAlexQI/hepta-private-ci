use std::error::Error;
use std::fmt;

use crate::OutcomeStoreError;
use crate::outcome_store::durable::effect_ack::RECORD_EFFECT_ACK_COMMIT_OPERATION;
use crate::outcome_store::durable::execution_intent::RESOLVE_EXECUTION_INTENT_COMMIT_OPERATION;
use crate::outcome_store::durable::execution_intent::STAGE_EXECUTION_INTENT_COMMIT_OPERATION;
use crate::outcome_store::durable::intent::ACKNOWLEDGE_INTENT_COMMIT_OPERATION;
use crate::outcome_store::durable::intent::STAGE_INTENT_COMMIT_OPERATION;
use crate::outcome_store::durable::provider_completion::STAGE_PROVIDER_COMPLETION_COMMIT_OPERATION;

/// Fail-closed error returned by [`super::SyncDurableOutcomeWriter`].
///
/// This reports persistence transport and commit state only. It does not
/// authenticate a receipt, evidence envelope, caller, or execution authority.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DurableOutcomeWriterError {
    /// The dedicated worker thread could not be created or initialize Tokio.
    WorkerStartup { detail: String },
    /// Durable recovery did not finish before the startup deadline.
    StartupTimeout { timeout_ms: u64 },
    /// The worker was unavailable before accepting a command, or while
    /// completing an accepted read-only command.
    WorkerUnavailable,
    /// The bounded command queue had no capacity; nothing was accepted.
    QueueFull { capacity: usize },
    /// The accepted command did not acknowledge before the deadline.
    ///
    /// The commit outcome is ambiguous and must only be reconciled by replaying
    /// the exact same attempt, receipt, and evidence.
    AcknowledgementTimeout { attempt_id: String, timeout_ms: u64 },
    /// A read-only attempt lookup did not acknowledge before the deadline.
    ReadAcknowledgementTimeout { attempt_id: String, timeout_ms: u64 },
    /// The worker or SQLite commit boundary disappeared after accepting work.
    ///
    /// The caller must not mint a replacement receipt for this attempt.
    CommitAmbiguous { attempt_id: String, detail: String },
    /// Exact material is durable, but the staged commit failed deterministically.
    ///
    /// Reconciliation must reuse the pending intent; callers must not mint a
    /// replacement receipt.
    PendingIntent {
        attempt_id: String,
        source: OutcomeStoreError,
    },
    /// The durable store rejected the command before an ambiguous commit edge.
    Backend { source: OutcomeStoreError },
    /// Internal writer configuration is invalid.
    InvalidConfiguration { detail: String },
}

impl fmt::Display for DurableOutcomeWriterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorkerStartup { detail } => {
                write!(formatter, "durable outcome writer startup failed: {detail}")
            }
            Self::StartupTimeout { timeout_ms } => write!(
                formatter,
                "durable outcome writer recovery exceeded its {timeout_ms}ms startup deadline"
            ),
            Self::WorkerUnavailable => formatter.write_str("durable outcome worker is unavailable"),
            Self::QueueFull { capacity } => {
                write!(
                    formatter,
                    "durable outcome writer queue is full at capacity {capacity}"
                )
            }
            Self::AcknowledgementTimeout {
                attempt_id,
                timeout_ms,
            } => write!(
                formatter,
                "durable outcome attempt {attempt_id} was not acknowledged within {timeout_ms}ms"
            ),
            Self::ReadAcknowledgementTimeout {
                attempt_id,
                timeout_ms,
            } => write!(
                formatter,
                "durable outcome attempt lookup {attempt_id} was not acknowledged within {timeout_ms}ms"
            ),
            Self::CommitAmbiguous { attempt_id, detail } => write!(
                formatter,
                "durable outcome attempt {attempt_id} has an ambiguous commit: {detail}"
            ),
            Self::PendingIntent { attempt_id, source } => write!(
                formatter,
                "durable outcome attempt {attempt_id} remains a pending exact intent: {source}"
            ),
            Self::Backend { source } => {
                write!(formatter, "durable outcome backend failed: {source}")
            }
            Self::InvalidConfiguration { detail } => {
                write!(
                    formatter,
                    "durable outcome writer configuration is invalid: {detail}"
                )
            }
        }
    }
}

impl Error for DurableOutcomeWriterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::PendingIntent { source, .. } | Self::Backend { source } => Some(source),
            _ => None,
        }
    }
}

pub(super) fn map_stage_error(
    attempt_id: String,
    source: OutcomeStoreError,
) -> DurableOutcomeWriterError {
    match source {
        OutcomeStoreError::Persistence {
            operation: STAGE_INTENT_COMMIT_OPERATION,
            detail,
        } => DurableOutcomeWriterError::CommitAmbiguous { attempt_id, detail },
        source => DurableOutcomeWriterError::Backend { source },
    }
}

pub(super) fn map_execution_stage_error(
    attempt_id: String,
    source: OutcomeStoreError,
) -> DurableOutcomeWriterError {
    match source {
        OutcomeStoreError::Persistence {
            operation: STAGE_EXECUTION_INTENT_COMMIT_OPERATION,
            detail,
        } => DurableOutcomeWriterError::CommitAmbiguous { attempt_id, detail },
        source => DurableOutcomeWriterError::Backend { source },
    }
}

pub(super) fn map_execution_resolution_error(
    attempt_id: String,
    source: OutcomeStoreError,
) -> DurableOutcomeWriterError {
    let detail = match source {
        OutcomeStoreError::Persistence {
            operation: RESOLVE_EXECUTION_INTENT_COMMIT_OPERATION,
            detail,
        } => detail,
        source => source.to_string(),
    };
    DurableOutcomeWriterError::CommitAmbiguous { attempt_id, detail }
}

pub(super) fn map_effect_ack_record_error(
    attempt_id: String,
    source: OutcomeStoreError,
) -> DurableOutcomeWriterError {
    match source {
        OutcomeStoreError::Persistence {
            operation: RECORD_EFFECT_ACK_COMMIT_OPERATION,
            detail,
        } => DurableOutcomeWriterError::CommitAmbiguous { attempt_id, detail },
        source => DurableOutcomeWriterError::Backend { source },
    }
}

pub(super) fn map_provider_completion_stage_error(
    attempt_id: String,
    source: OutcomeStoreError,
) -> DurableOutcomeWriterError {
    match source {
        OutcomeStoreError::Persistence {
            operation: STAGE_PROVIDER_COMPLETION_COMMIT_OPERATION,
            detail,
        } => DurableOutcomeWriterError::CommitAmbiguous { attempt_id, detail },
        source => DurableOutcomeWriterError::Backend { source },
    }
}

pub(super) fn map_acknowledgement_error(
    attempt_id: String,
    source: OutcomeStoreError,
) -> DurableOutcomeWriterError {
    let detail = match source {
        OutcomeStoreError::Persistence {
            operation: ACKNOWLEDGE_INTENT_COMMIT_OPERATION,
            detail,
        } => detail,
        source => source.to_string(),
    };
    DurableOutcomeWriterError::CommitAmbiguous { attempt_id, detail }
}
