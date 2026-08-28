#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "observation", rename_all = "snake_case")]
pub(crate) enum IntelligenceMutationReconciliationObservation {
    NotApplied {
        outcome_sha256: Sha256Digest,
    },
    MemoryFactsCommitted {
        write_receipt_sha256: Sha256Digest,
    },
    ProjectionPublished {
        write_receipt_sha256: Sha256Digest,
        expected_previous_generation: u64,
        new_generation: u64,
        projection_receipt_sha256: Sha256Digest,
    },
    OutboxSettled {
        write_receipt_sha256: Sha256Digest,
        expected_previous_generation: u64,
        new_generation: u64,
        projection_receipt_sha256: Sha256Digest,
        outcome_sha256: Sha256Digest,
    },
}

impl IntelligenceMutationReconciliationObservation {
    const fn kind(&self) -> &'static str {
        match self {
            Self::NotApplied { .. } => "not_applied",
            Self::MemoryFactsCommitted { .. } => "memory_facts_committed",
            Self::ProjectionPublished { .. } => "projection_published",
            Self::OutboxSettled { .. } => "outbox_settled",
        }
    }

    const fn observed_rank(&self) -> Option<u8> {
        match self {
            Self::NotApplied { .. } => None,
            Self::MemoryFactsCommitted { .. } => Some(1),
            Self::ProjectionPublished { .. } => Some(2),
            Self::OutboxSettled { .. } => Some(3),
        }
    }

    fn validate(&self) -> Result<(), IntelligenceMutationStateError> {
        match self {
            Self::NotApplied { outcome_sha256 } => {
                validate_digest(outcome_sha256, "reconciliation outcome digest")
            }
            Self::MemoryFactsCommitted {
                write_receipt_sha256,
            } => validate_digest(write_receipt_sha256, "observed write receipt digest"),
            Self::ProjectionPublished {
                write_receipt_sha256,
                expected_previous_generation,
                new_generation,
                projection_receipt_sha256,
            } => {
                validate_digest(write_receipt_sha256, "observed write receipt digest")?;
                validate_projection_step(*expected_previous_generation, *new_generation)?;
                validate_digest(
                    projection_receipt_sha256,
                    "observed projection receipt digest",
                )
            }
            Self::OutboxSettled {
                write_receipt_sha256,
                expected_previous_generation,
                new_generation,
                projection_receipt_sha256,
                outcome_sha256,
            } => {
                validate_digest(write_receipt_sha256, "observed write receipt digest")?;
                validate_projection_step(*expected_previous_generation, *new_generation)?;
                validate_digest(
                    projection_receipt_sha256,
                    "observed projection receipt digest",
                )?;
                validate_digest(outcome_sha256, "observed outbox outcome digest")
            }
        }
    }
}
