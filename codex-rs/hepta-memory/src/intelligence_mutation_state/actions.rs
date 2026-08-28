#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub(crate) enum IntelligenceMutationAction {
    WitnessSource {
        source_sha256: Sha256Digest,
    },
    ValidateGrounding {
        grounding_receipt_sha256: Sha256Digest,
    },
    RejectPreCommit {
        reason_sha256: Sha256Digest,
    },
    CancelPreCommit {
        reason_sha256: Sha256Digest,
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
    Reconcile {
        observation: IntelligenceMutationReconciliationObservation,
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
            Self::RejectPreCommit { .. } => "reject_pre_commit",
            Self::CancelPreCommit { .. } => "cancel_pre_commit",
            Self::AppendDurableIntent { .. } => "append_durable_intent",
            Self::CommitMemoryFacts { .. } => "commit_memory_facts",
            Self::PublishProjection { .. } => "publish_projection",
            Self::SettleOutbox { .. } => "settle_outbox",
            Self::Terminalize => "terminalize",
            Self::MarkIndeterminate { .. } => "mark_indeterminate",
            Self::Reconcile { .. } => "reconcile",
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
            Self::RejectPreCommit { reason_sha256 }
            | Self::CancelPreCommit { reason_sha256 }
            | Self::MarkIndeterminate { reason_sha256 }
            | Self::Quarantine { reason_sha256 } => {
                validate_digest(reason_sha256, "reason digest")
            }
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
                validate_projection_step(*expected_previous_generation, *new_generation)?;
                validate_digest(projection_receipt_sha256, "projection receipt digest")
            }
            Self::SettleOutbox { outcome_sha256 } => {
                validate_digest(outcome_sha256, "outbox outcome digest")
            }
            Self::Reconcile { observation } => observation.validate(),
            Self::Terminalize => Ok(()),
        }
    }
}
