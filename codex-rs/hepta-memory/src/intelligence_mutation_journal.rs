//! P0.4b append-only SQLite journal for the typed Intelligence mutation model.
//!
//! The journal is qualification-only and opt-in. It persists operation
//! bindings and immutable transition receipts, then reconstructs the typed
//! state machine from genesis on every reopen verification. It does not wire a
//! runtime caller, invoke a model or tool, change the default cognitive-store
//! open path, or grant production/external-effect authority.

use codex_hepta_contracts::Sha256Digest;
use codex_hepta_paths::HeptaAgentLayout;
use serde::Deserialize;
use serde::Serialize;
use sqlx::Row;
use sqlx::Sqlite;
use sqlx::Transaction;

use crate::CognitiveStore;
use crate::CognitiveStoreError;
use crate::cognitive_store::unavailable;

use super::intelligence_mutation_state::IntelligenceMutationAction;
use super::intelligence_mutation_state::IntelligenceMutationApplyDisposition;
use super::intelligence_mutation_state::IntelligenceMutationBinding;
use super::intelligence_mutation_state::IntelligenceMutationPhase;
use super::intelligence_mutation_state::IntelligenceMutationState;
use super::intelligence_mutation_state::IntelligenceMutationStateError;
use super::intelligence_mutation_state::IntelligenceMutationTransitionReceipt;
use super::intelligence_mutation_state::IntelligenceMutationTransitionRequest;

mod digest;
mod replay;
mod schema;

#[cfg(test)]
mod tests;

pub(crate) const INTELLIGENCE_MUTATION_JOURNAL_SCHEMA_VERSION: u32 = 1;
pub(crate) const INTELLIGENCE_MUTATION_JOURNAL_NAMESPACE: &str =
    "intelligence_mutation_sqlite_journal_v1";
pub(crate) const INTELLIGENCE_MUTATION_JOURNAL_RUNTIME_WIRED: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_JOURNAL_DEFAULT_OPEN_WIRED: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_JOURNAL_EXTERNAL_EFFECTS: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_JOURNAL_PRODUCTION_AUTHORITY: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_JOURNAL_OPERATOR_ACCEPTANCE: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_JOURNAL_PROMOTION: bool = false;

const COMPONENT_MIGRATION_VERSION: i64 = 12;
const COMPONENT_MIGRATION_DESCRIPTION: &str = "intelligence mutation transition journal";
const COMPONENT_MIGRATION_SQL: &str =
    include_str!("../mutation-migrations/0012_intelligence_mutation_journal.sql");
const MAX_JOURNALED_OPERATIONS: usize = 10_000;
const MAX_TRANSITIONS_PER_OPERATION: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum IntelligenceMutationJournalFault {
    None,
    BeforeTransitionInsert,
    AfterTransitionInsertBeforeCommit,
    AfterCommitBeforeReturn,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IntelligenceMutationJournalDisposition {
    Applied,
    Replay,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct IntelligenceMutationJournalAppend {
    pub(crate) schema_version: u32,
    pub(crate) namespace: String,
    pub(crate) disposition: IntelligenceMutationJournalDisposition,
    pub(crate) receipt: IntelligenceMutationTransitionReceipt,
    pub(crate) sqlite_persistence: bool,
    pub(crate) runtime_wired: bool,
    pub(crate) default_open_wired: bool,
    pub(crate) external_effects: bool,
    pub(crate) production_authority: bool,
    pub(crate) operator_acceptance: bool,
    pub(crate) promotion: bool,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum IntelligenceMutationJournalError {
    #[error(transparent)]
    Store(#[from] CognitiveStoreError),
    #[error(transparent)]
    State(#[from] IntelligenceMutationStateError),
    #[error("injected intelligence mutation journal fault: {0}")]
    Injected(&'static str),
    #[error("mutation transition committed but acknowledgement is indeterminate: {0}")]
    Indeterminate(Sha256Digest),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum PersistedAction {
    WitnessSource {
        source_sha256: String,
    },
    ValidateGrounding {
        grounding_receipt_sha256: String,
    },
    AppendDurableIntent {
        intent_sha256: String,
    },
    CommitMemoryFacts {
        write_receipt_sha256: String,
    },
    PublishProjection {
        expected_previous_generation: u64,
        new_generation: u64,
        projection_receipt_sha256: String,
    },
    SettleOutbox {
        outcome_sha256: String,
    },
    Terminalize,
    MarkIndeterminate {
        reason_sha256: String,
    },
    ReconcileApplied {
        outcome_sha256: String,
    },
    ReconcileNotApplied {
        outcome_sha256: String,
    },
    Quarantine {
        reason_sha256: String,
    },
}

impl PersistedAction {
    pub(super) fn from_action(action: &IntelligenceMutationAction) -> Self {
        match action {
            IntelligenceMutationAction::WitnessSource { source_sha256 } => Self::WitnessSource {
                source_sha256: source_sha256.as_str().to_string(),
            },
            IntelligenceMutationAction::ValidateGrounding {
                grounding_receipt_sha256,
            } => Self::ValidateGrounding {
                grounding_receipt_sha256: grounding_receipt_sha256.as_str().to_string(),
            },
            IntelligenceMutationAction::AppendDurableIntent { intent_sha256 } => {
                Self::AppendDurableIntent {
                    intent_sha256: intent_sha256.as_str().to_string(),
                }
            }
            IntelligenceMutationAction::CommitMemoryFacts {
                write_receipt_sha256,
            } => Self::CommitMemoryFacts {
                write_receipt_sha256: write_receipt_sha256.as_str().to_string(),
            },
            IntelligenceMutationAction::PublishProjection {
                expected_previous_generation,
                new_generation,
                projection_receipt_sha256,
            } => Self::PublishProjection {
                expected_previous_generation: *expected_previous_generation,
                new_generation: *new_generation,
                projection_receipt_sha256: projection_receipt_sha256.as_str().to_string(),
            },
            IntelligenceMutationAction::SettleOutbox { outcome_sha256 } => Self::SettleOutbox {
                outcome_sha256: outcome_sha256.as_str().to_string(),
            },
            IntelligenceMutationAction::Terminalize => Self::Terminalize,
            IntelligenceMutationAction::MarkIndeterminate { reason_sha256 } => {
                Self::MarkIndeterminate {
                    reason_sha256: reason_sha256.as_str().to_string(),
                }
            }
            IntelligenceMutationAction::ReconcileApplied { outcome_sha256 } => {
                Self::ReconcileApplied {
                    outcome_sha256: outcome_sha256.as_str().to_string(),
                }
            }
            IntelligenceMutationAction::ReconcileNotApplied { outcome_sha256 } => {
                Self::ReconcileNotApplied {
                    outcome_sha256: outcome_sha256.as_str().to_string(),
                }
            }
            IntelligenceMutationAction::Quarantine { reason_sha256 } => Self::Quarantine {
                reason_sha256: reason_sha256.as_str().to_string(),
            },
        }
    }

    pub(super) fn into_action(self) -> Result<IntelligenceMutationAction, CognitiveStoreError> {
        Ok(match self {
            Self::WitnessSource { source_sha256 } => IntelligenceMutationAction::WitnessSource {
                source_sha256: parse_digest(source_sha256, "source digest")?,
            },
            Self::ValidateGrounding {
                grounding_receipt_sha256,
            } => IntelligenceMutationAction::ValidateGrounding {
                grounding_receipt_sha256: parse_digest(
                    grounding_receipt_sha256,
                    "grounding receipt digest",
                )?,
            },
            Self::AppendDurableIntent { intent_sha256 } => {
                IntelligenceMutationAction::AppendDurableIntent {
                    intent_sha256: parse_digest(intent_sha256, "intent digest")?,
                }
            }
            Self::CommitMemoryFacts {
                write_receipt_sha256,
            } => IntelligenceMutationAction::CommitMemoryFacts {
                write_receipt_sha256: parse_digest(
                    write_receipt_sha256,
                    "write receipt digest",
                )?,
            },
            Self::PublishProjection {
                expected_previous_generation,
                new_generation,
                projection_receipt_sha256,
            } => IntelligenceMutationAction::PublishProjection {
                expected_previous_generation,
                new_generation,
                projection_receipt_sha256: parse_digest(
                    projection_receipt_sha256,
                    "projection receipt digest",
                )?,
            },
            Self::SettleOutbox { outcome_sha256 } => IntelligenceMutationAction::SettleOutbox {
                outcome_sha256: parse_digest(outcome_sha256, "outcome digest")?,
            },
            Self::Terminalize => IntelligenceMutationAction::Terminalize,
            Self::MarkIndeterminate { reason_sha256 } => {
                IntelligenceMutationAction::MarkIndeterminate {
                    reason_sha256: parse_digest(reason_sha256, "reason digest")?,
                }
            }
            Self::ReconcileApplied { outcome_sha256 } => {
                IntelligenceMutationAction::ReconcileApplied {
                    outcome_sha256: parse_digest(outcome_sha256, "outcome digest")?,
                }
            }
            Self::ReconcileNotApplied { outcome_sha256 } => {
                IntelligenceMutationAction::ReconcileNotApplied {
                    outcome_sha256: parse_digest(outcome_sha256, "outcome digest")?,
                }
            }
            Self::Quarantine { reason_sha256 } => IntelligenceMutationAction::Quarantine {
                reason_sha256: parse_digest(reason_sha256, "reason digest")?,
            },
        })
    }

    pub(super) const fn kind(&self) -> &'static str {
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
}

impl CognitiveStore {
    /// Opens the normal store, applies the opt-in P0.4b component migration,
    /// and verifies every operation by exact replay from genesis.
    pub(crate) async fn open_with_intelligence_mutation_journal(
        layout: &HeptaAgentLayout,
    ) -> Result<Self, IntelligenceMutationJournalError> {
        let store = Self::open(layout).await?;
        store.ensure_intelligence_mutation_journal_schema().await?;
        store.verify_intelligence_mutation_journal().await?;
        Ok(store)
    }

    pub(crate) async fn ensure_intelligence_mutation_journal_schema(
        &self,
    ) -> Result<(), CognitiveStoreError> {
        schema::ensure(&self.pool).await
    }

    pub(crate) async fn verify_intelligence_mutation_journal(
        &self,
    ) -> Result<(), IntelligenceMutationJournalError> {
        schema::verify(&self.pool).await?;
        replay::verify_all(&self.pool, self.owner_agent_id.as_str()).await?;
        Ok(())
    }

    /// Appends one immutable operation binding, or returns Replay for an exact
    /// duplicate. A changed duplicate fails closed.
    pub(crate) async fn begin_intelligence_mutation_journal(
        &self,
        binding: &IntelligenceMutationBinding,
    ) -> Result<IntelligenceMutationJournalDisposition, IntelligenceMutationJournalError> {
        self.ensure_intelligence_mutation_journal_schema().await?;
        binding.validate()?;
        let binding_sha256 = digest::binding_digest(binding);
        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(unavailable)?;
        let existing = sqlx::query(
            "SELECT owner_agent_id, lease_id, lease_epoch, expected_revision,
                    starting_projection_generation, causal_root_sha256,
                    binding_sha256
             FROM cognitive_intelligence_mutation_operations
             WHERE operation_id = ?",
        )
        .bind(&binding.operation_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(unavailable)?;
        let disposition = if let Some(existing) = existing {
            replay::verify_operation_binding_row(
                &existing,
                self.owner_agent_id.as_str(),
                binding,
                &binding_sha256,
            )?;
            IntelligenceMutationJournalDisposition::Replay
        } else {
            sqlx::query(
                "INSERT INTO cognitive_intelligence_mutation_operations (
                    operation_id, owner_agent_id, lease_id, lease_epoch,
                    expected_revision, starting_projection_generation,
                    causal_root_sha256, binding_sha256, created_at_unix_seconds
                 ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, unixepoch())",
            )
            .bind(&binding.operation_id)
            .bind(self.owner_agent_id.as_str())
            .bind(&binding.lease_id)
            .bind(to_i64(binding.lease_epoch, "lease epoch")?)
            .bind(
                binding
                    .expected_revision
                    .map(|value| to_i64(value, "expected revision"))
                    .transpose()?,
            )
            .bind(to_i64(
                binding.starting_projection_generation,
                "starting projection generation",
            )?)
            .bind(binding.causal_root_sha256.as_str())
            .bind(binding_sha256.as_str())
            .execute(&mut *transaction)
            .await
            .map_err(unavailable)?;
            IntelligenceMutationJournalDisposition::Applied
        };
        transaction.commit().await.map_err(unavailable)?;
        Ok(disposition)
    }

    /// Atomically replays the current journal, applies one typed transition,
    /// and appends its immutable receipt. Exact retries return Replay.
    pub(crate) async fn append_intelligence_mutation_transition(
        &self,
        request: IntelligenceMutationTransitionRequest,
        fault: IntelligenceMutationJournalFault,
    ) -> Result<IntelligenceMutationJournalAppend, IntelligenceMutationJournalError> {
        self.ensure_intelligence_mutation_journal_schema().await?;
        let mut transaction = self
            .pool
            .begin_with("BEGIN IMMEDIATE")
            .await
            .map_err(unavailable)?;
        let mut state = replay::replay_operation_tx(
            &mut transaction,
            self.owner_agent_id.as_str(),
            &request.binding.operation_id,
        )
        .await?;
        let applied = state.apply(request.clone())?;
        if applied.disposition == IntelligenceMutationApplyDisposition::Replay {
            transaction.commit().await.map_err(unavailable)?;
            return Ok(journal_append(
                IntelligenceMutationJournalDisposition::Replay,
                applied.receipt,
            ));
        }
        if fault == IntelligenceMutationJournalFault::BeforeTransitionInsert {
            transaction.rollback().await.map_err(unavailable)?;
            return Err(IntelligenceMutationJournalError::Injected(
                "before transition insert",
            ));
        }
        insert_transition_tx(&mut transaction, &request, &applied.receipt).await?;
        if fault == IntelligenceMutationJournalFault::AfterTransitionInsertBeforeCommit {
            transaction.rollback().await.map_err(unavailable)?;
            return Err(IntelligenceMutationJournalError::Injected(
                "after transition insert before commit",
            ));
        }
        transaction.commit().await.map_err(unavailable)?;
        if fault == IntelligenceMutationJournalFault::AfterCommitBeforeReturn {
            return Err(IntelligenceMutationJournalError::Indeterminate(
                applied.receipt.transition_sha256,
            ));
        }
        Ok(journal_append(
            IntelligenceMutationJournalDisposition::Applied,
            applied.receipt,
        ))
    }

    pub(crate) async fn replay_intelligence_mutation_operation(
        &self,
        operation_id: &str,
    ) -> Result<IntelligenceMutationState, IntelligenceMutationJournalError> {
        self.ensure_intelligence_mutation_journal_schema().await?;
        Ok(replay::replay_operation_pool(
            &self.pool,
            self.owner_agent_id.as_str(),
            operation_id,
        )
        .await?)
    }
}

async fn insert_transition_tx(
    transaction: &mut Transaction<'_, Sqlite>,
    request: &IntelligenceMutationTransitionRequest,
    receipt: &IntelligenceMutationTransitionReceipt,
) -> Result<(), CognitiveStoreError> {
    let action = PersistedAction::from_action(&request.action);
    let action_json = serde_json::to_string(&action)
        .map_err(|error| CognitiveStoreError::Invalid(error.to_string()))?;
    sqlx::query(
        "INSERT INTO cognitive_intelligence_mutation_transitions (
            operation_id, sequence, from_phase, to_phase, action,
            action_payload_json, request_sha256, causal_parent_sha256,
            transition_sha256, durable_intent_appended,
            durable_intent_settled, memory_write_count,
            projection_publish_count, last_published_generation,
            recorded_at_unix_seconds
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, unixepoch())",
    )
    .bind(&receipt.operation_id)
    .bind(to_i64(receipt.sequence, "transition sequence")?)
    .bind(receipt.from_phase.as_str())
    .bind(receipt.to_phase.as_str())
    .bind(action.kind())
    .bind(action_json)
    .bind(receipt.request_sha256.as_str())
    .bind(
        receipt
            .causal_parent_sha256
            .as_ref()
            .map(Sha256Digest::as_str),
    )
    .bind(receipt.transition_sha256.as_str())
    .bind(bool_i64(receipt.durable_intent_appended))
    .bind(bool_i64(receipt.durable_intent_settled))
    .bind(i64::from(receipt.memory_write_count))
    .bind(i64::from(receipt.projection_publish_count))
    .bind(to_i64(
        receipt.last_published_generation,
        "last published generation",
    )?)
    .execute(&mut **transaction)
    .await
    .map_err(unavailable)?;
    Ok(())
}

fn journal_append(
    disposition: IntelligenceMutationJournalDisposition,
    receipt: IntelligenceMutationTransitionReceipt,
) -> IntelligenceMutationJournalAppend {
    IntelligenceMutationJournalAppend {
        schema_version: INTELLIGENCE_MUTATION_JOURNAL_SCHEMA_VERSION,
        namespace: INTELLIGENCE_MUTATION_JOURNAL_NAMESPACE.to_string(),
        disposition,
        receipt,
        sqlite_persistence: true,
        runtime_wired: INTELLIGENCE_MUTATION_JOURNAL_RUNTIME_WIRED,
        default_open_wired: INTELLIGENCE_MUTATION_JOURNAL_DEFAULT_OPEN_WIRED,
        external_effects: INTELLIGENCE_MUTATION_JOURNAL_EXTERNAL_EFFECTS,
        production_authority: INTELLIGENCE_MUTATION_JOURNAL_PRODUCTION_AUTHORITY,
        operator_acceptance: INTELLIGENCE_MUTATION_JOURNAL_OPERATOR_ACCEPTANCE,
        promotion: INTELLIGENCE_MUTATION_JOURNAL_PROMOTION,
    }
}

pub(super) fn parse_phase(value: &str) -> Result<IntelligenceMutationPhase, CognitiveStoreError> {
    Ok(match value {
        "planned" => IntelligenceMutationPhase::Planned,
        "source_witnessed" => IntelligenceMutationPhase::SourceWitnessed,
        "grounding_validated" => IntelligenceMutationPhase::GroundingValidated,
        "durable_intent_appended" => IntelligenceMutationPhase::DurableIntentAppended,
        "memory_facts_committed" => IntelligenceMutationPhase::MemoryFactsCommitted,
        "projection_published" => IntelligenceMutationPhase::ProjectionPublished,
        "outbox_settled" => IntelligenceMutationPhase::OutboxSettled,
        "terminal" => IntelligenceMutationPhase::Terminal,
        "indeterminate" => IntelligenceMutationPhase::Indeterminate,
        "reconciled_applied" => IntelligenceMutationPhase::ReconciledApplied,
        "reconciled_not_applied" => IntelligenceMutationPhase::ReconciledNotApplied,
        "quarantined" => IntelligenceMutationPhase::Quarantined,
        _ => {
            return Err(CognitiveStoreError::Corrupt(format!(
                "unknown intelligence mutation phase `{value}`"
            )));
        }
    })
}

pub(super) fn parse_digest(
    value: String,
    label: &str,
) -> Result<Sha256Digest, CognitiveStoreError> {
    Sha256Digest::parse(value)
        .map_err(|error| CognitiveStoreError::Corrupt(format!("invalid {label}: {error}")))
}

pub(super) fn to_i64(value: u64, label: &str) -> Result<i64, CognitiveStoreError> {
    i64::try_from(value)
        .map_err(|_| CognitiveStoreError::Invalid(format!("{label} exceeds i64")))
}

pub(super) fn from_i64(value: i64, label: &str) -> Result<u64, CognitiveStoreError> {
    u64::try_from(value)
        .map_err(|_| CognitiveStoreError::Corrupt(format!("negative {label}")))
}

pub(super) const fn bool_i64(value: bool) -> i64 {
    if value { 1 } else { 0 }
}
