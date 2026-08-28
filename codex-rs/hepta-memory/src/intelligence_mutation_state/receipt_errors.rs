#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub(crate) enum IntelligenceMutationStateError {
    #[error("invalid intelligence mutation state: {0}")]
    Invalid(String),
    #[error("corrupt intelligence mutation state: {0}")]
    Corrupt(String),
    #[error("mutation binding changed after planning")]
    BindingDrift,
    #[error("transition sequence mismatch: expected {expected}, received {received}")]
    SequenceMismatch { expected: u64, received: u64 },
    #[error("transition sequence overflow")]
    SequenceOverflow,
    #[error("transition history exceeds the source-only maximum of {maximum}")]
    TransitionLimit { maximum: usize },
    #[error("replayed sequence does not match the original transition")]
    ReplayConflict,
    #[error("transition causal parent does not match the current state")]
    CausalParentMismatch,
    #[error("invalid transition from {phase:?} with action `{action}`")]
    InvalidTransition {
        phase: IntelligenceMutationPhase,
        action: String,
    },
    #[error("state {0:?} is terminal")]
    TerminalState(IntelligenceMutationPhase),
    #[error("memory facts would be written more than once")]
    DoubleWrite,
    #[error("projection would be published more than once")]
    DoubleProjectionPublish,
    #[error("projection generation overflow")]
    ProjectionGenerationOverflow,
    #[error("stale projection generation: expected {expected}, received {received}")]
    StaleProjectionGeneration { expected: u64, received: u64 },
    #[error("invalid indeterminate reconciliation: {0}")]
    InvalidReconciliation(String),
    #[error("reconciliation regressed from {origin:?} to observation `{observation}`")]
    ReconciliationRegression {
        origin: IntelligenceMutationPhase,
        observation: &'static str,
    },
    #[error("conflicting reconciliation evidence: {0}")]
    ReconciliationConflict(String),
    #[error("reconciliation count overflow")]
    ReconciliationOverflow,
    #[error("transition receipt {sequence} does not match deterministic replay")]
    ReceiptMismatch { sequence: u64 },
    #[error("transition receipt contains a positive authority claim")]
    AuthorityEscalation,
    #[error("transition receipt schema or namespace mismatch")]
    ReceiptSchemaMismatch,
    #[error("transition receipt digest mismatch")]
    ReceiptDigestMismatch,
}
