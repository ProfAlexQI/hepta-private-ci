//! P0.4 source-only typed orchestration for one Hepta Intelligence mutation.
//!
//! This module models the only legal ordering between source witness,
//! grounding, durable intent, memory/KG commit, projection publication,
//! outbox settlement, and terminalization.  It is deliberately not wired to
//! a runtime caller or SQLite journal in this tranche.  The transition
//! receipts are deterministic, causal, replay-safe, and authority-negative.

use codex_hepta_contracts::Sha256Digest;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::framing::frame_part;

pub(crate) const INTELLIGENCE_MUTATION_STATE_SCHEMA_VERSION: u32 = 1;
pub(crate) const INTELLIGENCE_MUTATION_STATE_NAMESPACE: &str =
    "intelligence_mutation_source_model_v1";
pub(crate) const INTELLIGENCE_MUTATION_STATE_RUNTIME_WIRED: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_STATE_SQLITE_PERSISTENCE: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_STATE_EXTERNAL_EFFECTS: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_STATE_PRODUCTION_AUTHORITY: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_STATE_OPERATOR_ACCEPTANCE: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_STATE_PROMOTION: bool = false;

const MAX_ID_BYTES: usize = 256;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IntelligenceMutationPhase {
    Planned,
    SourceWitnessed,
    GroundingValidated,
    DurableIntentAppended,
    MemoryFactsCommitted,
    ProjectionPublished,
    OutboxSettled,
    Terminal,
    Indeterminate,
    ReconciledApplied,
    ReconciledNotApplied,
    Quarantined,
}

impl IntelligenceMutationPhase {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::SourceWitnessed => "source_witnessed",
            Self::GroundingValidated => "grounding_validated",
            Self::DurableIntentAppended => "durable_intent_appended",
            Self::MemoryFactsCommitted => "memory_facts_committed",
            Self::ProjectionPublished => "projection_published",
            Self::OutboxSettled => "outbox_settled",
            Self::Terminal => "terminal",
            Self::Indeterminate => "indeterminate",
            Self::ReconciledApplied => "reconciled_applied",
            Self::ReconciledNotApplied => "reconciled_not_applied",
            Self::Quarantined => "quarantined",
        }
    }

    pub(crate) const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Terminal
                | Self::ReconciledApplied
                | Self::ReconciledNotApplied
                | Self::Quarantined
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct IntelligenceMutationBinding {
    pub(crate) operation_id: String,
    pub(crate) lease_id: String,
    pub(crate) lease_epoch: u64,
    pub(crate) expected_revision: Option<u64>,
    pub(crate) starting_projection_generation: u64,
    pub(crate) causal_root_sha256: Sha256Digest,
}

impl IntelligenceMutationBinding {
    pub(crate) fn validate(&self) -> Result<(), IntelligenceMutationStateError> {
        validate_id(&self.operation_id, "operation id")?;
        validate_id(&self.lease_id, "lease id")?;
        if self.lease_epoch == 0 {
            return Err(IntelligenceMutationStateError::Invalid(
                "lease epoch must be positive".to_string(),
            ));
        }
        if self.expected_revision == Some(0) {
            return Err(IntelligenceMutationStateError::Invalid(
                "expected revision must be positive when present".to_string(),
            ));
        }
        validate_digest(&self.causal_root_sha256, "causal root digest")
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub(crate) enum IntelligenceMutationAction {
    WitnessSource {
        source_sha256: Sha256Digest,
    },
    ValidateGrounding {
        grounding_receipt_sha256: Sha256Digest,
    },
    AppendDurableIntent {
        intent_sha256: Sha256Digest,
    },
    CommitMemoryFacts {
        write_receipt_sha256: Sha256Digest,
    },
    PublishProjection {
        expected_previous_generation: u64,
        new_generation: u64,
        projection_receipt_sha256: Sha256Digest,
    },
    SettleOutbox {
        outcome_sha256: Sha256Digest,
    },
    Terminalize,
    MarkIndeterminate {
        reason_sha256: Sha256Digest,
    },
    ReconcileApplied {
        outcome_sha256: Sha256Digest,
    },
    ReconcileNotApplied {
        outcome_sha256: Sha256Digest,
    },
    Quarantine {
        reason_sha256: Sha256Digest,
    },
}

impl IntelligenceMutationAction {
    const fn kind(&self) -> &'static str {
        match self {
            Self::WitnessSource { .. } => "witness_source",
            Self::ValidateGrounding { .. } => "validate_grounding",
            Self::AppendDurableIntent { .. } => "append_durable_intent",
            Self::CommitMemoryFacts { .. } => "commit_memory_facts",
            Self::PublishProjection { .. } => "publish_projection",
            Self::SettleOutbox { .. } => "settle_outbox",
            Self::Terminalize => "terminalize",
            Self::MarkIndeterminate { .. } => "mark_indeterminate",
            Self::ReconcileApplied { .. } => "reconcile_applied",
            Self::ReconcileNotApplied { .. } => "reconcile_not_applied",
            Self::Quarantine { .. } => "quarantine",
        }
    }

    fn validate(&self) -> Result<(), IntelligenceMutationStateError> {
        match self {
            Self::WitnessSource { source_sha256 } => {
                validate_digest(source_sha256, "source digest")
            }
            Self::ValidateGrounding {
                grounding_receipt_sha256,
            } => validate_digest(grounding_receipt_sha256, "grounding receipt digest"),
            Self::AppendDurableIntent { intent_sha256 } => {
                validate_digest(intent_sha256, "intent digest")
            }
            Self::CommitMemoryFacts {
                write_receipt_sha256,
            } => validate_digest(write_receipt_sha256, "write receipt digest"),
            Self::PublishProjection {
                expected_previous_generation,
                new_generation,
                projection_receipt_sha256,
            } => {
                if new_generation != &expected_previous_generation.saturating_add(1) {
                    return Err(IntelligenceMutationStateError::StaleProjectionGeneration {
                        expected: expected_previous_generation.saturating_add(1),
                        received: *new_generation,
                    });
                }
                validate_digest(projection_receipt_sha256, "projection receipt digest")
            }
            Self::SettleOutbox { outcome_sha256 }
            | Self::ReconcileApplied { outcome_sha256 }
            | Self::ReconcileNotApplied { outcome_sha256 } => {
                validate_digest(outcome_sha256, "outcome digest")
            }
            Self::MarkIndeterminate { reason_sha256 } | Self::Quarantine { reason_sha256 } => {
                validate_digest(reason_sha256, "reason digest")
            }
            Self::Terminalize => Ok(()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IntelligenceMutationTransitionRequest {
    pub(crate) binding: IntelligenceMutationBinding,
    pub(crate) sequence: u64,
    pub(crate) causal_parent_sha256: Option<Sha256Digest>,
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
    pub(crate) operation_id: String,
    pub(crate) sequence: u64,
    pub(crate) from_phase: IntelligenceMutationPhase,
    pub(crate) to_phase: IntelligenceMutationPhase,
    pub(crate) action: String,
    pub(crate) request_sha256: Sha256Digest,
    pub(crate) causal_parent_sha256: Option<Sha256Digest>,
    pub(crate) transition_sha256: Sha256Digest,
    pub(crate) durable_intent_appended: bool,
    pub(crate) durable_intent_settled: bool,
    pub(crate) memory_write_count: u8,
    pub(crate) projection_publish_count: u8,
    pub(crate) last_published_generation: u64,
    pub(crate) runtime_wired: bool,
    pub(crate) sqlite_persistence: bool,
    pub(crate) external_effects: bool,
    pub(crate) production_authority: bool,
    pub(crate) operator_acceptance: bool,
    pub(crate) promotion: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IntelligenceMutationApplyResult {
    pub(crate) disposition: IntelligenceMutationApplyDisposition,
    pub(crate) receipt: IntelligenceMutationTransitionReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct IntelligenceMutationState {
    binding: IntelligenceMutationBinding,
    phase: IntelligenceMutationPhase,
    next_sequence: u64,
    last_request_sha256: Option<Sha256Digest>,
    last_receipt: Option<IntelligenceMutationTransitionReceipt>,
    durable_intent_appended: bool,
    durable_intent_settled: bool,
    memory_write_count: u8,
    projection_publish_count: u8,
    last_published_generation: u64,
    indeterminate_from: Option<IntelligenceMutationPhase>,
}

impl IntelligenceMutationState {
    pub(crate) fn new(
        binding: IntelligenceMutationBinding,
    ) -> Result<Self, IntelligenceMutationStateError> {
        binding.validate()?;
        Ok(Self {
            last_published_generation: binding.starting_projection_generation,
            binding,
            phase: IntelligenceMutationPhase::Planned,
            next_sequence: 0,
            last_request_sha256: None,
            last_receipt: None,
            durable_intent_appended: false,
            durable_intent_settled: false,
            memory_write_count: 0,
            projection_publish_count: 0,
            indeterminate_from: None,
        })
    }

    pub(crate) fn phase(&self) -> IntelligenceMutationPhase {
        self.phase
    }

    pub(crate) fn next_sequence(&self) -> u64 {
        self.next_sequence
    }

    pub(crate) fn causal_parent_sha256(&self) -> Option<Sha256Digest> {
        self.last_receipt
            .as_ref()
            .map(|receipt| receipt.transition_sha256.clone())
    }

    pub(crate) fn binding(&self) -> &IntelligenceMutationBinding {
        &self.binding
    }

    pub(crate) fn apply(
        &mut self,
        request: IntelligenceMutationTransitionRequest,
    ) -> Result<IntelligenceMutationApplyResult, IntelligenceMutationStateError> {
        request.binding.validate()?;
        request.action.validate()?;
        if request.binding != self.binding {
            return Err(IntelligenceMutationStateError::BindingDrift);
        }
        let request_sha256 = request_digest(&request);
        if request.sequence < self.next_sequence {
            if request.sequence + 1 == self.next_sequence
                && self.last_request_sha256.as_ref() == Some(&request_sha256)
            {
                let receipt = self.last_receipt.clone().ok_or_else(|| {
                    IntelligenceMutationStateError::Corrupt(
                        "replay has no prior transition receipt".to_string(),
                    )
                })?;
                return Ok(IntelligenceMutationApplyResult {
                    disposition: IntelligenceMutationApplyDisposition::Replay,
                    receipt,
                });
            }
            return Err(IntelligenceMutationStateError::ReplayConflict);
        }
        if request.sequence != self.next_sequence {
            return Err(IntelligenceMutationStateError::SequenceMismatch {
                expected: self.next_sequence,
                received: request.sequence,
            });
        }
        if request.causal_parent_sha256 != self.causal_parent_sha256() {
            return Err(IntelligenceMutationStateError::CausalParentMismatch);
        }
        if self.phase.is_terminal() {
            return Err(IntelligenceMutationStateError::TerminalState(self.phase));
        }

        let from_phase = self.phase;
        let to_phase = self.apply_action(&request.action)?;
        self.phase = to_phase;
        let transition_sha256 = transition_digest(
            &self.binding,
            request.sequence,
            from_phase,
            to_phase,
            &request_sha256,
            request.causal_parent_sha256.as_ref(),
            self,
        );
        let receipt = IntelligenceMutationTransitionReceipt {
            schema_version: INTELLIGENCE_MUTATION_STATE_SCHEMA_VERSION,
            namespace: INTELLIGENCE_MUTATION_STATE_NAMESPACE.to_string(),
            operation_id: self.binding.operation_id.clone(),
            sequence: request.sequence,
            from_phase,
            to_phase,
            action: request.action.kind().to_string(),
            request_sha256: request_sha256.clone(),
            causal_parent_sha256: request.causal_parent_sha256,
            transition_sha256,
            durable_intent_appended: self.durable_intent_appended,
            durable_intent_settled: self.durable_intent_settled,
            memory_write_count: self.memory_write_count,
            projection_publish_count: self.projection_publish_count,
            last_published_generation: self.last_published_generation,
            runtime_wired: INTELLIGENCE_MUTATION_STATE_RUNTIME_WIRED,
            sqlite_persistence: INTELLIGENCE_MUTATION_STATE_SQLITE_PERSISTENCE,
            external_effects: INTELLIGENCE_MUTATION_STATE_EXTERNAL_EFFECTS,
            production_authority: INTELLIGENCE_MUTATION_STATE_PRODUCTION_AUTHORITY,
            operator_acceptance: INTELLIGENCE_MUTATION_STATE_OPERATOR_ACCEPTANCE,
            promotion: INTELLIGENCE_MUTATION_STATE_PROMOTION,
        };
        self.last_request_sha256 = Some(request_sha256);
        self.last_receipt = Some(receipt.clone());
        self.next_sequence = self.next_sequence.checked_add(1).ok_or_else(|| {
            IntelligenceMutationStateError::Corrupt("transition sequence overflow".to_string())
        })?;
        self.validate()?;
        Ok(IntelligenceMutationApplyResult {
            disposition: IntelligenceMutationApplyDisposition::Applied,
            receipt,
        })
    }

    pub(crate) fn validate(&self) -> Result<(), IntelligenceMutationStateError> {
        self.binding.validate()?;
        if self.memory_write_count > 1 {
            return Err(IntelligenceMutationStateError::Corrupt(
                "one mutation wrote memory facts more than once".to_string(),
            ));
        }
        if self.projection_publish_count > 1 {
            return Err(IntelligenceMutationStateError::Corrupt(
                "one mutation published a projection more than once".to_string(),
            ));
        }
        if self.projection_publish_count == 0
            && self.last_published_generation != self.binding.starting_projection_generation
        {
            return Err(IntelligenceMutationStateError::Corrupt(
                "projection generation changed without publication".to_string(),
            ));
        }
        if self.projection_publish_count == 1
            && self.last_published_generation
                != self
                    .binding
                    .starting_projection_generation
                    .saturating_add(1)
        {
            return Err(IntelligenceMutationStateError::Corrupt(
                "projection publication did not advance exactly once".to_string(),
            ));
        }
        if matches!(
            self.phase,
            IntelligenceMutationPhase::DurableIntentAppended
                | IntelligenceMutationPhase::MemoryFactsCommitted
                | IntelligenceMutationPhase::ProjectionPublished
                | IntelligenceMutationPhase::OutboxSettled
                | IntelligenceMutationPhase::Terminal
                | IntelligenceMutationPhase::Indeterminate
                | IntelligenceMutationPhase::ReconciledApplied
                | IntelligenceMutationPhase::ReconciledNotApplied
                | IntelligenceMutationPhase::Quarantined
        ) && !self.durable_intent_appended
        {
            return Err(IntelligenceMutationStateError::Corrupt(
                "post-intent phase has no durable intent".to_string(),
            ));
        }
        if matches!(
            self.phase,
            IntelligenceMutationPhase::ProjectionPublished
                | IntelligenceMutationPhase::OutboxSettled
                | IntelligenceMutationPhase::Terminal
        ) && (self.memory_write_count != 1 || self.projection_publish_count != 1)
        {
            return Err(IntelligenceMutationStateError::Corrupt(
                "projection phase lacks one committed write and publication".to_string(),
            ));
        }
        if matches!(
            self.phase,
            IntelligenceMutationPhase::OutboxSettled
                | IntelligenceMutationPhase::Terminal
                | IntelligenceMutationPhase::ReconciledApplied
                | IntelligenceMutationPhase::ReconciledNotApplied
                | IntelligenceMutationPhase::Quarantined
        ) && !self.durable_intent_settled
        {
            return Err(IntelligenceMutationStateError::Corrupt(
                "terminal or resolved phase has an unsettled durable intent".to_string(),
            ));
        }
        if self.phase == IntelligenceMutationPhase::Terminal
            && self.phase != IntelligenceMutationPhase::OutboxSettled
            && !self.durable_intent_settled
        {
            return Err(IntelligenceMutationStateError::Corrupt(
                "terminalization occurred before outbox settlement".to_string(),
            ));
        }
        if self.phase == IntelligenceMutationPhase::ReconciledApplied
            && self.memory_write_count != 1
        {
            return Err(IntelligenceMutationStateError::Corrupt(
                "applied reconciliation has no single observed write".to_string(),
            ));
        }
        if self.phase == IntelligenceMutationPhase::ReconciledNotApplied
            && self.memory_write_count != 0
        {
            return Err(IntelligenceMutationStateError::Corrupt(
                "not-applied reconciliation observed a committed write".to_string(),
            ));
        }
        Ok(())
    }

    fn apply_action(
        &mut self,
        action: &IntelligenceMutationAction,
    ) -> Result<IntelligenceMutationPhase, IntelligenceMutationStateError> {
        match (self.phase, action) {
            (
                IntelligenceMutationPhase::Planned,
                IntelligenceMutationAction::WitnessSource { .. },
            ) => Ok(IntelligenceMutationPhase::SourceWitnessed),
            (
                IntelligenceMutationPhase::SourceWitnessed,
                IntelligenceMutationAction::ValidateGrounding { .. },
            ) => Ok(IntelligenceMutationPhase::GroundingValidated),
            (
                IntelligenceMutationPhase::GroundingValidated,
                IntelligenceMutationAction::AppendDurableIntent { .. },
            ) => {
                self.durable_intent_appended = true;
                self.durable_intent_settled = false;
                Ok(IntelligenceMutationPhase::DurableIntentAppended)
            }
            (
                IntelligenceMutationPhase::DurableIntentAppended,
                IntelligenceMutationAction::CommitMemoryFacts { .. },
            ) => {
                self.memory_write_count =
                    self.memory_write_count.checked_add(1).ok_or_else(|| {
                        IntelligenceMutationStateError::Corrupt(
                            "memory write count overflow".to_string(),
                        )
                    })?;
                if self.memory_write_count != 1 {
                    return Err(IntelligenceMutationStateError::DoubleWrite);
                }
                Ok(IntelligenceMutationPhase::MemoryFactsCommitted)
            }
            (
                IntelligenceMutationPhase::MemoryFactsCommitted,
                IntelligenceMutationAction::PublishProjection {
                    expected_previous_generation,
                    new_generation,
                    ..
                },
            ) => {
                if *expected_previous_generation != self.last_published_generation {
                    return Err(IntelligenceMutationStateError::StaleProjectionGeneration {
                        expected: self.last_published_generation,
                        received: *expected_previous_generation,
                    });
                }
                let expected_new =
                    self.last_published_generation
                        .checked_add(1)
                        .ok_or_else(|| {
                            IntelligenceMutationStateError::Corrupt(
                                "projection generation overflow".to_string(),
                            )
                        })?;
                if *new_generation != expected_new {
                    return Err(IntelligenceMutationStateError::StaleProjectionGeneration {
                        expected: expected_new,
                        received: *new_generation,
                    });
                }
                self.projection_publish_count = self
                    .projection_publish_count
                    .checked_add(1)
                    .ok_or_else(|| {
                        IntelligenceMutationStateError::Corrupt(
                            "projection publish count overflow".to_string(),
                        )
                    })?;
                if self.projection_publish_count != 1 {
                    return Err(IntelligenceMutationStateError::DoubleProjectionPublish);
                }
                self.last_published_generation = *new_generation;
                Ok(IntelligenceMutationPhase::ProjectionPublished)
            }
            (
                IntelligenceMutationPhase::ProjectionPublished,
                IntelligenceMutationAction::SettleOutbox { .. },
            ) => {
                self.durable_intent_settled = true;
                Ok(IntelligenceMutationPhase::OutboxSettled)
            }
            (IntelligenceMutationPhase::OutboxSettled, IntelligenceMutationAction::Terminalize) => {
                if !self.durable_intent_settled {
                    return Err(IntelligenceMutationStateError::UnsettledIntent);
                }
                Ok(IntelligenceMutationPhase::Terminal)
            }
            (
                phase @ (IntelligenceMutationPhase::DurableIntentAppended
                | IntelligenceMutationPhase::MemoryFactsCommitted
                | IntelligenceMutationPhase::ProjectionPublished),
                IntelligenceMutationAction::MarkIndeterminate { .. },
            ) => {
                self.indeterminate_from = Some(phase);
                Ok(IntelligenceMutationPhase::Indeterminate)
            }
            (
                IntelligenceMutationPhase::Indeterminate,
                IntelligenceMutationAction::ReconcileApplied { .. },
            ) => {
                if self.indeterminate_from == Some(IntelligenceMutationPhase::DurableIntentAppended)
                    && self.memory_write_count == 0
                {
                    self.memory_write_count = 1;
                }
                if self.memory_write_count != 1 {
                    return Err(IntelligenceMutationStateError::InvalidReconciliation(
                        "applied outcome must observe exactly one memory write".to_string(),
                    ));
                }
                self.durable_intent_settled = true;
                Ok(IntelligenceMutationPhase::ReconciledApplied)
            }
            (
                IntelligenceMutationPhase::Indeterminate,
                IntelligenceMutationAction::ReconcileNotApplied { .. },
            ) => {
                if self.indeterminate_from != Some(IntelligenceMutationPhase::DurableIntentAppended)
                    || self.memory_write_count != 0
                {
                    return Err(IntelligenceMutationStateError::InvalidReconciliation(
                        "not-applied outcome is valid only before a committed memory write"
                            .to_string(),
                    ));
                }
                self.durable_intent_settled = true;
                Ok(IntelligenceMutationPhase::ReconciledNotApplied)
            }
            (
                IntelligenceMutationPhase::Indeterminate,
                IntelligenceMutationAction::Quarantine { .. },
            ) => {
                self.durable_intent_settled = true;
                Ok(IntelligenceMutationPhase::Quarantined)
            }
            (phase, action) => Err(IntelligenceMutationStateError::InvalidTransition {
                phase,
                action: action.kind().to_string(),
            }),
        }
    }
}

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
    #[error("stale projection generation: expected {expected}, received {received}")]
    StaleProjectionGeneration { expected: u64, received: u64 },
    #[error("durable intent has not been settled")]
    UnsettledIntent,
    #[error("invalid indeterminate reconciliation: {0}")]
    InvalidReconciliation(String),
}

fn request_digest(request: &IntelligenceMutationTransitionRequest) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(&mut hasher, b"hepta:intelligence:mutation-request:v1");
    frame_binding(&mut hasher, &request.binding);
    frame_part(&mut hasher, &request.sequence.to_be_bytes());
    frame_optional_digest(&mut hasher, request.causal_parent_sha256.as_ref());
    frame_action(&mut hasher, &request.action);
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn transition_digest(
    binding: &IntelligenceMutationBinding,
    sequence: u64,
    from_phase: IntelligenceMutationPhase,
    to_phase: IntelligenceMutationPhase,
    request_sha256: &Sha256Digest,
    causal_parent_sha256: Option<&Sha256Digest>,
    state: &IntelligenceMutationState,
) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(&mut hasher, b"hepta:intelligence:mutation-transition:v1");
    frame_binding(&mut hasher, binding);
    frame_part(&mut hasher, &sequence.to_be_bytes());
    frame_part(&mut hasher, from_phase.as_str().as_bytes());
    frame_part(&mut hasher, to_phase.as_str().as_bytes());
    frame_part(&mut hasher, request_sha256.as_str().as_bytes());
    frame_optional_digest(&mut hasher, causal_parent_sha256);
    frame_part(&mut hasher, &[u8::from(state.durable_intent_appended)]);
    frame_part(&mut hasher, &[u8::from(state.durable_intent_settled)]);
    frame_part(&mut hasher, &[state.memory_write_count]);
    frame_part(&mut hasher, &[state.projection_publish_count]);
    frame_part(&mut hasher, &state.last_published_generation.to_be_bytes());
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn frame_binding(hasher: &mut Sha256, binding: &IntelligenceMutationBinding) {
    frame_part(hasher, binding.operation_id.as_bytes());
    frame_part(hasher, binding.lease_id.as_bytes());
    frame_part(hasher, &binding.lease_epoch.to_be_bytes());
    match binding.expected_revision {
        Some(revision) => {
            frame_part(hasher, &[1]);
            frame_part(hasher, &revision.to_be_bytes());
        }
        None => frame_part(hasher, &[0]),
    }
    frame_part(
        hasher,
        &binding.starting_projection_generation.to_be_bytes(),
    );
    frame_part(hasher, binding.causal_root_sha256.as_str().as_bytes());
}

fn frame_optional_digest(hasher: &mut Sha256, digest: Option<&Sha256Digest>) {
    match digest {
        Some(digest) => {
            frame_part(hasher, &[1]);
            frame_part(hasher, digest.as_str().as_bytes());
        }
        None => frame_part(hasher, &[0]),
    }
}

fn frame_action(hasher: &mut Sha256, action: &IntelligenceMutationAction) {
    frame_part(hasher, action.kind().as_bytes());
    match action {
        IntelligenceMutationAction::WitnessSource { source_sha256 } => {
            frame_part(hasher, source_sha256.as_str().as_bytes());
        }
        IntelligenceMutationAction::ValidateGrounding {
            grounding_receipt_sha256,
        } => frame_part(hasher, grounding_receipt_sha256.as_str().as_bytes()),
        IntelligenceMutationAction::AppendDurableIntent { intent_sha256 } => {
            frame_part(hasher, intent_sha256.as_str().as_bytes());
        }
        IntelligenceMutationAction::CommitMemoryFacts {
            write_receipt_sha256,
        } => frame_part(hasher, write_receipt_sha256.as_str().as_bytes()),
        IntelligenceMutationAction::PublishProjection {
            expected_previous_generation,
            new_generation,
            projection_receipt_sha256,
        } => {
            frame_part(hasher, &expected_previous_generation.to_be_bytes());
            frame_part(hasher, &new_generation.to_be_bytes());
            frame_part(hasher, projection_receipt_sha256.as_str().as_bytes());
        }
        IntelligenceMutationAction::SettleOutbox { outcome_sha256 }
        | IntelligenceMutationAction::ReconcileApplied { outcome_sha256 }
        | IntelligenceMutationAction::ReconcileNotApplied { outcome_sha256 } => {
            frame_part(hasher, outcome_sha256.as_str().as_bytes());
        }
        IntelligenceMutationAction::MarkIndeterminate { reason_sha256 }
        | IntelligenceMutationAction::Quarantine { reason_sha256 } => {
            frame_part(hasher, reason_sha256.as_str().as_bytes());
        }
        IntelligenceMutationAction::Terminalize => {}
    }
}

fn validate_id(value: &str, label: &str) -> Result<(), IntelligenceMutationStateError> {
    if value.trim().is_empty() || value.len() > MAX_ID_BYTES || value.as_bytes().contains(&0) {
        return Err(IntelligenceMutationStateError::Invalid(format!(
            "{label} must contain 1..={MAX_ID_BYTES} non-NUL bytes"
        )));
    }
    Ok(())
}

fn validate_digest(
    value: &Sha256Digest,
    label: &str,
) -> Result<(), IntelligenceMutationStateError> {
    Sha256Digest::parse(value.as_str().to_string()).map_err(|error| {
        IntelligenceMutationStateError::Invalid(format!("invalid {label}: {error}"))
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(value: &str) -> Sha256Digest {
        Sha256Digest::for_bytes(value.as_bytes())
    }

    fn binding() -> IntelligenceMutationBinding {
        IntelligenceMutationBinding {
            operation_id: "operation:1".to_string(),
            lease_id: "lease:1".to_string(),
            lease_epoch: 7,
            expected_revision: Some(3),
            starting_projection_generation: 11,
            causal_root_sha256: digest("root"),
        }
    }

    fn request(
        state: &IntelligenceMutationState,
        action: IntelligenceMutationAction,
    ) -> IntelligenceMutationTransitionRequest {
        IntelligenceMutationTransitionRequest {
            binding: state.binding().clone(),
            sequence: state.next_sequence(),
            causal_parent_sha256: state.causal_parent_sha256(),
            action,
        }
    }

    fn apply(
        state: &mut IntelligenceMutationState,
        action: IntelligenceMutationAction,
    ) -> IntelligenceMutationTransitionReceipt {
        let request = request(state, action);
        state.apply(request).expect("transition").receipt
    }

    fn advance_to_intent(state: &mut IntelligenceMutationState) {
        apply(
            state,
            IntelligenceMutationAction::WitnessSource {
                source_sha256: digest("source"),
            },
        );
        apply(
            state,
            IntelligenceMutationAction::ValidateGrounding {
                grounding_receipt_sha256: digest("grounding"),
            },
        );
        apply(
            state,
            IntelligenceMutationAction::AppendDurableIntent {
                intent_sha256: digest("intent"),
            },
        );
    }

    #[test]
    fn normal_path_requires_outbox_settlement_before_terminal() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        advance_to_intent(&mut state);
        apply(
            &mut state,
            IntelligenceMutationAction::CommitMemoryFacts {
                write_receipt_sha256: digest("write"),
            },
        );
        apply(
            &mut state,
            IntelligenceMutationAction::PublishProjection {
                expected_previous_generation: 11,
                new_generation: 12,
                projection_receipt_sha256: digest("projection"),
            },
        );
        assert!(matches!(
            state.apply(request(&state, IntelligenceMutationAction::Terminalize)),
            Err(IntelligenceMutationStateError::InvalidTransition { .. })
        ));
        apply(
            &mut state,
            IntelligenceMutationAction::SettleOutbox {
                outcome_sha256: digest("settled"),
            },
        );
        let terminal = apply(&mut state, IntelligenceMutationAction::Terminalize);
        assert_eq!(state.phase(), IntelligenceMutationPhase::Terminal);
        assert!(terminal.durable_intent_settled);
        assert_eq!(terminal.memory_write_count, 1);
        assert_eq!(terminal.projection_publish_count, 1);
        assert!(!terminal.runtime_wired);
        assert!(!terminal.sqlite_persistence);
        assert!(!terminal.production_authority);
        state.validate().expect("valid terminal state");
    }

    #[test]
    fn exact_duplicate_replays_without_double_write() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        advance_to_intent(&mut state);
        let request = request(
            &state,
            IntelligenceMutationAction::CommitMemoryFacts {
                write_receipt_sha256: digest("write"),
            },
        );
        let applied = state.apply(request.clone()).expect("applied");
        let replay = state.apply(request).expect("replay");
        assert_eq!(
            applied.disposition,
            IntelligenceMutationApplyDisposition::Applied
        );
        assert_eq!(
            replay.disposition,
            IntelligenceMutationApplyDisposition::Replay
        );
        assert_eq!(applied.receipt, replay.receipt);
        assert_eq!(replay.receipt.memory_write_count, 1);
    }

    #[test]
    fn changed_replay_reorder_and_parent_drift_fail_closed() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        let first = request(
            &state,
            IntelligenceMutationAction::WitnessSource {
                source_sha256: digest("source"),
            },
        );
        state.apply(first).expect("first");

        let changed = IntelligenceMutationTransitionRequest {
            binding: state.binding().clone(),
            sequence: 0,
            causal_parent_sha256: None,
            action: IntelligenceMutationAction::WitnessSource {
                source_sha256: digest("other-source"),
            },
        };
        assert_eq!(
            state.apply(changed),
            Err(IntelligenceMutationStateError::ReplayConflict)
        );

        let reordered = IntelligenceMutationTransitionRequest {
            binding: state.binding().clone(),
            sequence: state.next_sequence() + 1,
            causal_parent_sha256: state.causal_parent_sha256(),
            action: IntelligenceMutationAction::ValidateGrounding {
                grounding_receipt_sha256: digest("grounding"),
            },
        };
        assert!(matches!(
            state.apply(reordered),
            Err(IntelligenceMutationStateError::SequenceMismatch { .. })
        ));

        let wrong_parent = IntelligenceMutationTransitionRequest {
            binding: state.binding().clone(),
            sequence: state.next_sequence(),
            causal_parent_sha256: Some(digest("wrong-parent")),
            action: IntelligenceMutationAction::ValidateGrounding {
                grounding_receipt_sha256: digest("grounding"),
            },
        };
        assert_eq!(
            state.apply(wrong_parent),
            Err(IntelligenceMutationStateError::CausalParentMismatch)
        );
    }

    #[test]
    fn binding_and_generation_drift_fail_closed() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        let mut drifted = state.binding().clone();
        drifted.lease_epoch += 1;
        let drifted_request = IntelligenceMutationTransitionRequest {
            binding: drifted,
            sequence: 0,
            causal_parent_sha256: None,
            action: IntelligenceMutationAction::WitnessSource {
                source_sha256: digest("source"),
            },
        };
        assert_eq!(
            state.apply(drifted_request),
            Err(IntelligenceMutationStateError::BindingDrift)
        );

        advance_to_intent(&mut state);
        apply(
            &mut state,
            IntelligenceMutationAction::CommitMemoryFacts {
                write_receipt_sha256: digest("write"),
            },
        );
        let stale = request(
            &state,
            IntelligenceMutationAction::PublishProjection {
                expected_previous_generation: 10,
                new_generation: 11,
                projection_receipt_sha256: digest("projection"),
            },
        );
        assert!(matches!(
            state.apply(stale),
            Err(IntelligenceMutationStateError::StaleProjectionGeneration { .. })
        ));
    }

    #[test]
    fn crash_before_write_reconciles_not_applied_without_stranded_intent() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        advance_to_intent(&mut state);
        apply(
            &mut state,
            IntelligenceMutationAction::MarkIndeterminate {
                reason_sha256: digest("crash-after-intent"),
            },
        );
        let resolved = apply(
            &mut state,
            IntelligenceMutationAction::ReconcileNotApplied {
                outcome_sha256: digest("not-applied"),
            },
        );
        assert_eq!(
            state.phase(),
            IntelligenceMutationPhase::ReconciledNotApplied
        );
        assert!(resolved.durable_intent_settled);
        assert_eq!(resolved.memory_write_count, 0);
    }

    #[test]
    fn crash_after_write_reconciles_applied_without_second_write() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        advance_to_intent(&mut state);
        apply(
            &mut state,
            IntelligenceMutationAction::CommitMemoryFacts {
                write_receipt_sha256: digest("write"),
            },
        );
        apply(
            &mut state,
            IntelligenceMutationAction::MarkIndeterminate {
                reason_sha256: digest("crash-after-write"),
            },
        );
        let resolved = apply(
            &mut state,
            IntelligenceMutationAction::ReconcileApplied {
                outcome_sha256: digest("applied"),
            },
        );
        assert_eq!(state.phase(), IntelligenceMutationPhase::ReconciledApplied);
        assert!(resolved.durable_intent_settled);
        assert_eq!(resolved.memory_write_count, 1);
    }

    #[test]
    fn not_applied_cannot_overwrite_an_observed_commit() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        advance_to_intent(&mut state);
        apply(
            &mut state,
            IntelligenceMutationAction::CommitMemoryFacts {
                write_receipt_sha256: digest("write"),
            },
        );
        apply(
            &mut state,
            IntelligenceMutationAction::MarkIndeterminate {
                reason_sha256: digest("crash"),
            },
        );
        let request = request(
            &state,
            IntelligenceMutationAction::ReconcileNotApplied {
                outcome_sha256: digest("not-applied"),
            },
        );
        assert!(matches!(
            state.apply(request),
            Err(IntelligenceMutationStateError::InvalidReconciliation(_))
        ));
    }

    #[test]
    fn quarantine_settles_an_indeterminate_intent() {
        let mut state = IntelligenceMutationState::new(binding()).expect("state");
        advance_to_intent(&mut state);
        apply(
            &mut state,
            IntelligenceMutationAction::MarkIndeterminate {
                reason_sha256: digest("unknown"),
            },
        );
        let receipt = apply(
            &mut state,
            IntelligenceMutationAction::Quarantine {
                reason_sha256: digest("operator-review-required"),
            },
        );
        assert_eq!(state.phase(), IntelligenceMutationPhase::Quarantined);
        assert!(receipt.durable_intent_settled);
    }

    #[test]
    fn identical_paths_have_identical_transition_digests() {
        fn run() -> Vec<Sha256Digest> {
            let mut state = IntelligenceMutationState::new(binding()).expect("state");
            [
                IntelligenceMutationAction::WitnessSource {
                    source_sha256: digest("source"),
                },
                IntelligenceMutationAction::ValidateGrounding {
                    grounding_receipt_sha256: digest("grounding"),
                },
                IntelligenceMutationAction::AppendDurableIntent {
                    intent_sha256: digest("intent"),
                },
                IntelligenceMutationAction::CommitMemoryFacts {
                    write_receipt_sha256: digest("write"),
                },
                IntelligenceMutationAction::PublishProjection {
                    expected_previous_generation: 11,
                    new_generation: 12,
                    projection_receipt_sha256: digest("projection"),
                },
                IntelligenceMutationAction::SettleOutbox {
                    outcome_sha256: digest("settled"),
                },
                IntelligenceMutationAction::Terminalize,
            ]
            .into_iter()
            .map(|action| apply(&mut state, action).transition_sha256)
            .collect()
        }
        assert_eq!(run(), run());
    }
}
