use std::error::Error;
use std::fmt;

use hepta_contracts::ContentHash;
use hepta_contracts::ReceiptId;

/// Typed failure returned by an outcome store.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OutcomeStoreError {
    /// The store mutex was poisoned by a panicking holder.
    StorePoisoned,
    /// Durable storage could not complete an I/O or SQLite operation.
    Persistence {
        /// Stable operation label.
        operation: &'static str,
        /// Backend error detail.
        detail: String,
    },
    /// Durable bytes or their recovered contract relationships are invalid.
    Corrupt {
        /// Fail-closed corruption detail.
        detail: String,
    },
    /// One receipt identity was presented with a different receipt hash.
    ReceiptHashConflict {
        /// Reused receipt identity.
        receipt: ReceiptId,
        /// Receipt hash held by the store.
        existing: ContentHash,
        /// Different receipt hash supplied by the caller.
        attempted: ContentHash,
    },
    /// One receipt identity and hash were reused for a different envelope.
    ReceiptEnvelopeConflict {
        /// Reused receipt identity.
        receipt: ReceiptId,
    },
    /// The exact evidence envelope was presented with a different hash.
    EvidenceHashConflict {
        /// Receipt identity whose evidence changed.
        receipt: ReceiptId,
        /// Evidence hash held by the store.
        existing: ContentHash,
        /// Different evidence hash supplied by the caller.
        attempted: ContentHash,
    },
    /// One evidence hash was presented with a different opaque envelope.
    EvidenceEnvelopeConflict {
        /// Receipt identity whose evidence envelope changed.
        receipt: ReceiptId,
        /// Hash claimed for both different envelopes.
        evidence_hash: ContentHash,
    },
    /// Both the opaque evidence envelope and its claimed hash changed.
    EvidenceEnvelopeAndHashConflict {
        /// Receipt identity whose evidence changed.
        receipt: ReceiptId,
        /// Evidence hash held by the store.
        existing_hash: ContentHash,
        /// Different evidence hash supplied by the caller.
        attempted_hash: ContentHash,
    },
    /// One attempt was reused for a different terminal receipt.
    AttemptAlreadyFinalized {
        /// Reused execution-attempt identity.
        attempt_id: String,
        /// Receipt that first finalized the attempt.
        existing_receipt: ReceiptId,
        /// Different receipt supplied by the caller.
        attempted_receipt: ReceiptId,
    },
    /// One receipt identity was reused across execution attempts.
    ReceiptAttemptConflict {
        /// Reused receipt identity.
        receipt: ReceiptId,
        /// Attempt first bound to the receipt.
        existing_attempt: String,
        /// Different attempt supplied by the caller.
        attempted_attempt: String,
    },
    /// Another unresolved pre-dispatch intent already owns the durable executor.
    ExecutionIntentOutstanding {
        /// Attempt whose unresolved intent blocks further provider dispatch.
        existing_attempt: String,
        /// Different attempt that tried to enter provider dispatch.
        attempted_attempt: String,
    },
    /// One execution attempt was presented with different pre-dispatch material.
    ExecutionIntentConflict {
        /// Reused execution-attempt identity.
        attempt_id: String,
    },
    /// One idempotency key was reused by a different execution attempt.
    ExecutionIdempotencyConflict {
        /// Reused deterministic provider idempotency key.
        idempotency_key: String,
        /// Attempt first bound to the key.
        existing_attempt: String,
        /// Different attempt that tried to reuse the key.
        attempted_attempt: String,
    },
    /// An execution intent was staged after its attempt already became terminal.
    ExecutionIntentAfterFinalization {
        /// Already-finalized execution-attempt identity.
        attempt_id: String,
    },
    /// An execution intent cannot resolve before its terminal outcome is durable.
    ExecutionIntentOutcomeMissing {
        /// Unresolved execution-attempt identity.
        attempt_id: String,
    },
    /// One effect acknowledgement was presented with conflicting exact material.
    ExecutionEffectAckConflict {
        /// Reused execution-attempt identity.
        attempt_id: String,
    },
    /// One effect acknowledgement has no unresolved execution intent.
    ExecutionEffectAckIntentMissing {
        /// Execution-attempt identity without a staged plan.
        attempt_id: String,
    },
    /// One effect acknowledgement targets an intent without an effect plan.
    ExecutionEffectAckPlanMissing {
        /// Execution-attempt identity without a planned provider effect.
        attempt_id: String,
    },
    /// One effect acknowledgement disagrees with its staged plan.
    ExecutionEffectAckBindingMismatch {
        /// Execution-attempt identity whose ACK binding changed.
        attempt_id: String,
    },
}

impl fmt::Display for OutcomeStoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StorePoisoned => formatter.write_str("outcome store mutex poisoned"),
            Self::Persistence { operation, detail } => {
                write!(formatter, "outcome store {operation} failed: {detail}")
            }
            Self::Corrupt { detail } => {
                write!(formatter, "outcome store is corrupt: {detail}")
            }
            Self::ReceiptHashConflict { receipt, .. } => {
                write!(
                    formatter,
                    "receipt identity {receipt} has a conflicting hash"
                )
            }
            Self::ReceiptEnvelopeConflict { receipt } => {
                write!(
                    formatter,
                    "receipt identity {receipt} has a conflicting envelope"
                )
            }
            Self::EvidenceHashConflict { receipt, .. } => {
                write!(
                    formatter,
                    "receipt identity {receipt} has a conflicting evidence hash"
                )
            }
            Self::EvidenceEnvelopeConflict { receipt, .. } => {
                write!(
                    formatter,
                    "receipt identity {receipt} has a conflicting evidence envelope"
                )
            }
            Self::EvidenceEnvelopeAndHashConflict { receipt, .. } => {
                write!(
                    formatter,
                    "receipt identity {receipt} has a conflicting evidence envelope and hash"
                )
            }
            Self::AttemptAlreadyFinalized { attempt_id, .. } => {
                write!(
                    formatter,
                    "execution attempt {attempt_id} is already finalized"
                )
            }
            Self::ReceiptAttemptConflict { receipt, .. } => {
                write!(
                    formatter,
                    "receipt identity {receipt} is bound to another attempt"
                )
            }
            Self::ExecutionIntentOutstanding {
                existing_attempt,
                attempted_attempt,
            } => write!(
                formatter,
                "execution attempt {attempted_attempt} is blocked by unresolved intent {existing_attempt}"
            ),
            Self::ExecutionIntentConflict { attempt_id } => write!(
                formatter,
                "execution attempt {attempt_id} has conflicting pre-dispatch material"
            ),
            Self::ExecutionIdempotencyConflict {
                idempotency_key,
                existing_attempt,
                attempted_attempt,
            } => write!(
                formatter,
                "execution idempotency key {idempotency_key} is bound to {existing_attempt}, not {attempted_attempt}"
            ),
            Self::ExecutionIntentAfterFinalization { attempt_id } => write!(
                formatter,
                "execution attempt {attempt_id} is already terminal and cannot be staged"
            ),
            Self::ExecutionIntentOutcomeMissing { attempt_id } => write!(
                formatter,
                "execution intent {attempt_id} cannot resolve before a durable terminal outcome"
            ),
            Self::ExecutionEffectAckConflict { attempt_id } => write!(
                formatter,
                "execution effect ACK {attempt_id} has conflicting exact material"
            ),
            Self::ExecutionEffectAckIntentMissing { attempt_id } => write!(
                formatter,
                "execution effect ACK {attempt_id} has no unresolved execution intent"
            ),
            Self::ExecutionEffectAckPlanMissing { attempt_id } => write!(
                formatter,
                "execution effect ACK {attempt_id} has no staged effect plan"
            ),
            Self::ExecutionEffectAckBindingMismatch { attempt_id } => write!(
                formatter,
                "execution effect ACK {attempt_id} disagrees with its staged effect plan"
            ),
        }
    }
}

impl Error for OutcomeStoreError {}
