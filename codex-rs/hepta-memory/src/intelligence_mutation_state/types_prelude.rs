use codex_hepta_contracts::Sha256Digest;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::framing::frame_part;

pub(crate) const INTELLIGENCE_MUTATION_STATE_SCHEMA_VERSION: u32 = 2;
pub(crate) const INTELLIGENCE_MUTATION_STATE_NAMESPACE: &str =
    "intelligence_mutation_source_model_v2";
pub(crate) const INTELLIGENCE_MUTATION_STATE_IMPLEMENTED: bool = true;
pub(crate) const INTELLIGENCE_MUTATION_STATE_RUNTIME_WIRED: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_STATE_QUALIFIED: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_STATE_SQLITE_PERSISTENCE: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_STATE_EXTERNAL_EFFECTS: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_STATE_PRODUCTION_AUTHORITY: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_STATE_OPERATOR_ACCEPTANCE: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_STATE_PROMOTION: bool = false;
pub(crate) const INTELLIGENCE_MUTATION_STATE_CALLERS_RATCHET: bool = false;

const MAX_ID_BYTES: usize = 512;
const MAX_TRANSITIONS: usize = 64;
const OPERATION_ID_DOMAIN: &[u8] = b"hepta:intelligence:mutation-operation:v2";
const CAUSAL_ROOT_DOMAIN: &[u8] = b"hepta:intelligence:mutation-causal-root:v2";
const REQUEST_DOMAIN: &[u8] = b"hepta:intelligence:mutation-request:v2";
const TRANSITION_DOMAIN: &[u8] = b"hepta:intelligence:mutation-transition:v2";

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
    RejectedPreCommit,
    CancelledPreCommit,
    Indeterminate,
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
            Self::RejectedPreCommit => "rejected_pre_commit",
            Self::CancelledPreCommit => "cancelled_pre_commit",
            Self::Indeterminate => "indeterminate",
            Self::ReconciledNotApplied => "reconciled_not_applied",
            Self::Quarantined => "quarantined",
        }
    }

    pub(crate) const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Terminal
                | Self::RejectedPreCommit
                | Self::CancelledPreCommit
                | Self::ReconciledNotApplied
                | Self::Quarantined
        )
    }

    const fn durable_rank(self) -> Option<u8> {
        match self {
            Self::DurableIntentAppended => Some(0),
            Self::MemoryFactsCommitted => Some(1),
            Self::ProjectionPublished => Some(2),
            Self::OutboxSettled => Some(3),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IntelligenceMutationIntentDisposition {
    None,
    Pending,
    SettledApplied,
    SettledNotApplied,
    Quarantined,
}

impl IntelligenceMutationIntentDisposition {
    const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Pending => "pending",
            Self::SettledApplied => "settled_applied",
            Self::SettledNotApplied => "settled_not_applied",
            Self::Quarantined => "quarantined",
        }
    }
}
