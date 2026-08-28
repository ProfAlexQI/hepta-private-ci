#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct IntelligenceMutationTransitionRequest {
    pub(crate) binding: IntelligenceMutationBinding,
    pub(crate) sequence: u64,
    pub(crate) causal_parent_sha256: Sha256Digest,
    pub(crate) action: IntelligenceMutationAction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IntelligenceMutationApplyDisposition {
    Applied,
    Replay,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct IntelligenceMutationTransitionReceipt {
    pub(crate) schema_version: u32,
    pub(crate) namespace: String,
    pub(crate) operation_id: Sha256Digest,
    pub(crate) sequence: u64,
    pub(crate) from_phase: IntelligenceMutationPhase,
    pub(crate) to_phase: IntelligenceMutationPhase,
    pub(crate) action: String,
    pub(crate) request_sha256: Sha256Digest,
    pub(crate) causal_parent_sha256: Sha256Digest,
    pub(crate) transition_sha256: Sha256Digest,
    pub(crate) intent_disposition: IntelligenceMutationIntentDisposition,
    pub(crate) memory_write_count: u8,
    pub(crate) projection_publish_count: u8,
    pub(crate) outbox_settled: bool,
    pub(crate) last_published_generation: u64,
    pub(crate) indeterminate_from: Option<IntelligenceMutationPhase>,
    pub(crate) last_recovery_origin: Option<IntelligenceMutationPhase>,
    pub(crate) reconciliation_count: u8,
    pub(crate) runtime_wired: bool,
    pub(crate) qualified: bool,
    pub(crate) sqlite_persistence: bool,
    pub(crate) external_effects: bool,
    pub(crate) production_authority: bool,
    pub(crate) operator_acceptance: bool,
    pub(crate) promotion: bool,
    pub(crate) callers_ratchet: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IntelligenceMutationApplyResult {
    pub(crate) disposition: IntelligenceMutationApplyDisposition,
    pub(crate) receipt: IntelligenceMutationTransitionReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct IntelligenceMutationHistoryEntry {
    request_sha256: Sha256Digest,
    receipt: IntelligenceMutationTransitionReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IntelligenceMutationState {
    binding: IntelligenceMutationBinding,
    phase: IntelligenceMutationPhase,
    next_sequence: u64,
    history: Vec<IntelligenceMutationHistoryEntry>,
    intent_disposition: IntelligenceMutationIntentDisposition,
    memory_write_count: u8,
    projection_publish_count: u8,
    outbox_settled: bool,
    last_published_generation: u64,
    indeterminate_from: Option<IntelligenceMutationPhase>,
    last_recovery_origin: Option<IntelligenceMutationPhase>,
    reconciliation_count: u8,
}
