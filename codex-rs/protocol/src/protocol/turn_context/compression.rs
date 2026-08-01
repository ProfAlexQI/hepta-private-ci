use super::common::TURN_CONTEXT_ADAPTIVE_BUDGET_ALLOCATION_SCHEMA_VERSION;
use super::common::TURN_CONTEXT_COMPRESSION_CANDIDATE_SCHEMA_VERSION;
use super::common::TURN_CONTEXT_COMPRESSION_STAGE_SCHEMA_VERSION;
use super::common::TurnContextTier;
use super::common::compression_candidate_source_id_is_payload_light;
use super::common::is_stable_manifest_replay_hash;
use super::common::is_zero_u32;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextCompressionStageKind {
    Summary,
    Rewrite,
    Defragment,
    Prune,
    #[default]
    Unknown,
}

impl TurnContextCompressionStageKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Summary => "summary",
            Self::Rewrite => "rewrite",
            Self::Defragment => "defragment",
            Self::Prune => "prune",
            Self::Unknown => "unknown",
        }
    }

    pub fn schema_version(self) -> Option<u32> {
        (!self.is_unknown()).then_some(TURN_CONTEXT_COMPRESSION_STAGE_SCHEMA_VERSION)
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextCompressionLossCheckStatus {
    MarkerBoundaryOnly,
    SemanticLossCheckPassed,
    SemanticLossCheckFailed,
    NotEvaluated,
    #[default]
    Unknown,
}

impl TurnContextCompressionLossCheckStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MarkerBoundaryOnly => "marker_boundary_only",
            Self::SemanticLossCheckPassed => "semantic_loss_check_passed",
            Self::SemanticLossCheckFailed => "semantic_loss_check_failed",
            Self::NotEvaluated => "not_evaluated",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextCompressionProtectedTierInvariant {
    Preserved,
    #[default]
    Unknown,
}

impl TurnContextCompressionProtectedTierInvariant {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextCompressionStage {
    pub kind: TurnContextCompressionStageKind,
    pub input_tokens: u32,
    pub output_tokens: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub affected_entries: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub loss_check_status: Option<TurnContextCompressionLossCheckStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub rollback_source_text_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub protected_tier_invariant: Option<TurnContextCompressionProtectedTierInvariant>,
}

impl TurnContextCompressionStage {
    pub fn tokens_saved(&self) -> u32 {
        self.input_tokens.saturating_sub(self.output_tokens)
    }

    pub fn has_payload_light_integrity(&self) -> bool {
        !self.kind.is_unknown()
            && self.output_tokens <= self.input_tokens
            && (self.input_tokens == 0 || self.affected_entries > 0)
            && self
                .loss_check_status
                .is_none_or(|status| !status.is_unknown())
            && self
                .rollback_source_text_hash
                .as_deref()
                .is_none_or(is_stable_manifest_replay_hash)
            && self
                .protected_tier_invariant
                .is_none_or(|invariant| !invariant.is_unknown())
    }
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextCompressionCandidateReason {
    BudgetPressureDryRun,
    #[default]
    Unknown,
}

impl TurnContextCompressionCandidateReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BudgetPressureDryRun => "budget_pressure_dry_run",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, JsonSchema, TS, Hash,
)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case")]
pub enum TurnContextBudgetAllocationAction {
    Keep,
    Drop,
    Compress,
    #[default]
    Unknown,
}

impl TurnContextBudgetAllocationAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Keep => "keep",
            Self::Drop => "drop",
            Self::Compress => "compress",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextAdaptiveBudgetAllocation {
    pub tier: TurnContextTier,
    pub source_id: String,
    pub budget_class: String,
    pub input_tokens: u32,
    pub reserve_tokens: u32,
    pub proposed_budget_tokens: u32,
    pub overflow_tokens: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub omit_priority: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub compression_kind: Option<TurnContextCompressionStageKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub estimated_compressed_tokens: Option<u32>,
    pub current_heuristic_action: TurnContextBudgetAllocationAction,
    pub proposed_action: TurnContextBudgetAllocationAction,
    pub would_drop: bool,
    pub would_compress: bool,
}

impl TurnContextAdaptiveBudgetAllocation {
    pub fn schema_version(&self) -> Option<u32> {
        self.has_payload_light_integrity()
            .then_some(TURN_CONTEXT_ADAPTIVE_BUDGET_ALLOCATION_SCHEMA_VERSION)
    }

    pub fn has_payload_light_integrity(&self) -> bool {
        !self.tier.is_unknown()
            && compression_candidate_source_id_is_payload_light(&self.source_id)
            && !self.budget_class.is_empty()
            && self.proposed_budget_tokens <= self.input_tokens
            && self.reserve_tokens <= self.input_tokens
            && self.overflow_tokens
                == self
                    .input_tokens
                    .saturating_sub(self.proposed_budget_tokens)
            && self
                .estimated_compressed_tokens
                .is_none_or(|tokens| tokens <= self.input_tokens)
            && self.compression_kind.is_none_or(|kind| !kind.is_unknown())
            && !self.current_heuristic_action.is_unknown()
            && !self.proposed_action.is_unknown()
            && self.would_drop == (self.proposed_action == TurnContextBudgetAllocationAction::Drop)
            && self.would_compress
                == (self.proposed_action == TurnContextBudgetAllocationAction::Compress)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, TS)]
pub struct TurnContextCompressionCandidate {
    pub kind: TurnContextCompressionStageKind,
    pub tier: TurnContextTier,
    pub source_id: String,
    pub input_tokens: u32,
    pub estimated_output_tokens: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub affected_entries: u32,
    pub not_executed_reason: TurnContextCompressionCandidateReason,
}

impl TurnContextCompressionCandidate {
    pub fn schema_version(&self) -> Option<u32> {
        self.has_payload_light_integrity()
            .then_some(TURN_CONTEXT_COMPRESSION_CANDIDATE_SCHEMA_VERSION)
    }

    pub fn estimated_tokens_saved(&self) -> u32 {
        self.input_tokens
            .saturating_sub(self.estimated_output_tokens)
    }

    pub fn has_payload_light_integrity(&self) -> bool {
        !self.kind.is_unknown()
            && !self.tier.is_unknown()
            && compression_candidate_source_id_is_payload_light(&self.source_id)
            && self.estimated_output_tokens <= self.input_tokens
            && (self.input_tokens == 0 || self.affected_entries > 0)
            && !self.not_executed_reason.is_unknown()
    }
}
