use serde::Deserialize;
use serde::Serialize;

use super::super::CONTEXT_MEMORY_SHADOW_CANARY_PROMOTION_READINESS_SCHEMA_VERSION;
use super::super::ContextMemoryShadowQualityTrendSnapshotReport;
use super::super::ContextMemoryShadowQualityTrendWindowVerdict;

const SHADOW_CANARY_PROMOTION_REQUIRED_STABLE_WINDOW_COUNT: usize = 1;
const SHADOW_CANARY_PROMOTION_REHEARSAL_COUNT: usize = 3;
const SHADOW_CANARY_PROMOTION_OPERATOR_PACKET_LINE_COUNT: usize = 6;

/// Payload-light mode for the shadow canary promotion readiness rehearsal.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryShadowCanaryPromotionMode {
    ShadowOnly,
    #[default]
    Unknown,
}

impl ContextMemoryShadowCanaryPromotionMode {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Controlled decision for a shadow-only canary promotion precheck.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryShadowCanaryPromotionDecision {
    ReadyShadowOnly,
    BlockedRegression,
    #[default]
    Unknown,
}

impl ContextMemoryShadowCanaryPromotionDecision {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Controlled rehearsal verdict for rollback, kill-switch, and soak checks.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryShadowCanaryRehearsalVerdict {
    Covered,
    Blocked,
    #[default]
    Unknown,
}

impl ContextMemoryShadowCanaryRehearsalVerdict {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light promotion-readiness rehearsal derived from the trend snapshot.
///
/// This report is intentionally one step short of any live canary route. It
/// exposes only aggregate stable-window counts, blocker counts, rehearsal
/// coverage counters, controlled verdict enums, and side-effect booleans. It
/// must not carry prompt/query/transcript/memory text, ranked payloads, graph
/// payloads, ids, hashes, operator identity, or activation commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryShadowCanaryPromotionReadinessReport {
    pub schema_version: u32,
    pub mode: ContextMemoryShadowCanaryPromotionMode,
    pub source_trend_snapshot_pass: bool,
    pub source_trend_window_verdict: ContextMemoryShadowQualityTrendWindowVerdict,
    pub required_stable_window_count: usize,
    pub observed_stable_window_count: usize,
    pub required_pass_streak: usize,
    pub observed_pass_streak: usize,
    pub promotion_decision: ContextMemoryShadowCanaryPromotionDecision,
    pub promotion_blocker_count: usize,
    pub regression_window_blocking_count: usize,
    pub rollback_rehearsal_verdict: ContextMemoryShadowCanaryRehearsalVerdict,
    pub rollback_rehearsal_count: usize,
    pub rollback_rehearsal_pass_count: usize,
    pub rollback_rehearsal_blocking_count: usize,
    pub kill_switch_rehearsal_verdict: ContextMemoryShadowCanaryRehearsalVerdict,
    pub kill_switch_rehearsal_count: usize,
    pub kill_switch_rehearsal_pass_count: usize,
    pub soak_readback_verdict: ContextMemoryShadowCanaryRehearsalVerdict,
    pub soak_readback_window_count: usize,
    pub soak_readback_pass_count: usize,
    pub operator_packet_line_count: usize,
    pub operator_packet_redacted: bool,
    pub operator_approval_required: bool,
    pub history_persistence_write: bool,
    pub production_route: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
    pub canary_promotion_route_opened: bool,
    pub rollback_write: bool,
}

impl Default for ContextMemoryShadowCanaryPromotionReadinessReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_SHADOW_CANARY_PROMOTION_READINESS_SCHEMA_VERSION,
            mode: ContextMemoryShadowCanaryPromotionMode::Unknown,
            source_trend_snapshot_pass: false,
            source_trend_window_verdict: ContextMemoryShadowQualityTrendWindowVerdict::Unknown,
            required_stable_window_count: 0,
            observed_stable_window_count: 0,
            required_pass_streak: 0,
            observed_pass_streak: 0,
            promotion_decision: ContextMemoryShadowCanaryPromotionDecision::Unknown,
            promotion_blocker_count: 0,
            regression_window_blocking_count: 0,
            rollback_rehearsal_verdict: ContextMemoryShadowCanaryRehearsalVerdict::Unknown,
            rollback_rehearsal_count: 0,
            rollback_rehearsal_pass_count: 0,
            rollback_rehearsal_blocking_count: 0,
            kill_switch_rehearsal_verdict: ContextMemoryShadowCanaryRehearsalVerdict::Unknown,
            kill_switch_rehearsal_count: 0,
            kill_switch_rehearsal_pass_count: 0,
            soak_readback_verdict: ContextMemoryShadowCanaryRehearsalVerdict::Unknown,
            soak_readback_window_count: 0,
            soak_readback_pass_count: 0,
            operator_packet_line_count: 0,
            operator_packet_redacted: false,
            operator_approval_required: false,
            history_persistence_write: false,
            production_route: false,
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            prompt_assembly_change: false,
            operator_activation_allowed: false,
            canary_promotion_route_opened: false,
            rollback_write: false,
        }
    }
}

impl ContextMemoryShadowCanaryPromotionReadinessReport {
    pub fn from_trend_snapshot(
        trend_snapshot: &ContextMemoryShadowQualityTrendSnapshotReport,
    ) -> Self {
        let source_trend_snapshot_pass =
            trend_snapshot.has_shadow_quality_trend_snapshot_integrity();
        let side_effects_disabled = !trend_snapshot.history_persistence_write
            && !trend_snapshot.production_route
            && !trend_snapshot.production_write
            && !trend_snapshot.graph_write
            && !trend_snapshot.runtime_activation
            && !trend_snapshot.prompt_assembly_change
            && !trend_snapshot.operator_activation_allowed;
        let stable_window_ready = source_trend_snapshot_pass
            && trend_snapshot.trend_window_verdict
                == ContextMemoryShadowQualityTrendWindowVerdict::StableWindow
            && trend_snapshot.observed_pass_streak == trend_snapshot.required_pass_streak
            && trend_snapshot.regression_window_blocking_count == 0
            && trend_snapshot.operator_approval_required
            && side_effects_disabled;
        let observed_stable_window_count = if stable_window_ready {
            SHADOW_CANARY_PROMOTION_REQUIRED_STABLE_WINDOW_COUNT
        } else {
            0
        };
        let promotion_blocker_count = if stable_window_ready {
            0
        } else {
            trend_snapshot.regression_window_blocking_count.max(1)
        };
        let rehearsal_pass_count = if stable_window_ready {
            SHADOW_CANARY_PROMOTION_REHEARSAL_COUNT
        } else {
            0
        };
        let rehearsal_blocking_count =
            SHADOW_CANARY_PROMOTION_REHEARSAL_COUNT.saturating_sub(rehearsal_pass_count);
        let rehearsal_verdict = if stable_window_ready {
            ContextMemoryShadowCanaryRehearsalVerdict::Covered
        } else {
            ContextMemoryShadowCanaryRehearsalVerdict::Blocked
        };

        Self {
            schema_version: CONTEXT_MEMORY_SHADOW_CANARY_PROMOTION_READINESS_SCHEMA_VERSION,
            mode: ContextMemoryShadowCanaryPromotionMode::ShadowOnly,
            source_trend_snapshot_pass,
            source_trend_window_verdict: trend_snapshot.trend_window_verdict,
            required_stable_window_count: SHADOW_CANARY_PROMOTION_REQUIRED_STABLE_WINDOW_COUNT,
            observed_stable_window_count,
            required_pass_streak: trend_snapshot.required_pass_streak,
            observed_pass_streak: trend_snapshot.observed_pass_streak,
            promotion_decision: if stable_window_ready {
                ContextMemoryShadowCanaryPromotionDecision::ReadyShadowOnly
            } else {
                ContextMemoryShadowCanaryPromotionDecision::BlockedRegression
            },
            promotion_blocker_count,
            regression_window_blocking_count: trend_snapshot.regression_window_blocking_count,
            rollback_rehearsal_verdict: rehearsal_verdict,
            rollback_rehearsal_count: SHADOW_CANARY_PROMOTION_REHEARSAL_COUNT,
            rollback_rehearsal_pass_count: rehearsal_pass_count,
            rollback_rehearsal_blocking_count: rehearsal_blocking_count,
            kill_switch_rehearsal_verdict: rehearsal_verdict,
            kill_switch_rehearsal_count: SHADOW_CANARY_PROMOTION_REHEARSAL_COUNT,
            kill_switch_rehearsal_pass_count: rehearsal_pass_count,
            soak_readback_verdict: rehearsal_verdict,
            soak_readback_window_count: SHADOW_CANARY_PROMOTION_REHEARSAL_COUNT,
            soak_readback_pass_count: rehearsal_pass_count,
            operator_packet_line_count: SHADOW_CANARY_PROMOTION_OPERATOR_PACKET_LINE_COUNT,
            operator_packet_redacted: true,
            operator_approval_required: trend_snapshot.operator_approval_required,
            history_persistence_write: trend_snapshot.history_persistence_write,
            production_route: trend_snapshot.production_route,
            production_write: trend_snapshot.production_write,
            graph_write: trend_snapshot.graph_write,
            runtime_activation: trend_snapshot.runtime_activation,
            prompt_assembly_change: trend_snapshot.prompt_assembly_change,
            operator_activation_allowed: trend_snapshot.operator_activation_allowed,
            canary_promotion_route_opened: false,
            rollback_write: false,
        }
    }

    pub fn has_shadow_canary_promotion_readiness_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_SHADOW_CANARY_PROMOTION_READINESS_SCHEMA_VERSION
            && self.mode == ContextMemoryShadowCanaryPromotionMode::ShadowOnly
            && !self.mode.is_unknown()
            && self.source_trend_snapshot_pass
            && self.source_trend_window_verdict
                == ContextMemoryShadowQualityTrendWindowVerdict::StableWindow
            && !self.source_trend_window_verdict.is_unknown()
            && self.required_stable_window_count
                == SHADOW_CANARY_PROMOTION_REQUIRED_STABLE_WINDOW_COUNT
            && self.observed_stable_window_count
                == SHADOW_CANARY_PROMOTION_REQUIRED_STABLE_WINDOW_COUNT
            && self.required_pass_streak == SHADOW_CANARY_PROMOTION_REHEARSAL_COUNT
            && self.observed_pass_streak == SHADOW_CANARY_PROMOTION_REHEARSAL_COUNT
            && self.promotion_decision
                == ContextMemoryShadowCanaryPromotionDecision::ReadyShadowOnly
            && !self.promotion_decision.is_unknown()
            && self.promotion_blocker_count == 0
            && self.regression_window_blocking_count == 0
            && self.rollback_rehearsal_verdict == ContextMemoryShadowCanaryRehearsalVerdict::Covered
            && !self.rollback_rehearsal_verdict.is_unknown()
            && self.rollback_rehearsal_count == SHADOW_CANARY_PROMOTION_REHEARSAL_COUNT
            && self.rollback_rehearsal_pass_count == SHADOW_CANARY_PROMOTION_REHEARSAL_COUNT
            && self.rollback_rehearsal_blocking_count == 0
            && self.kill_switch_rehearsal_verdict
                == ContextMemoryShadowCanaryRehearsalVerdict::Covered
            && !self.kill_switch_rehearsal_verdict.is_unknown()
            && self.kill_switch_rehearsal_count == SHADOW_CANARY_PROMOTION_REHEARSAL_COUNT
            && self.kill_switch_rehearsal_pass_count == SHADOW_CANARY_PROMOTION_REHEARSAL_COUNT
            && self.soak_readback_verdict == ContextMemoryShadowCanaryRehearsalVerdict::Covered
            && !self.soak_readback_verdict.is_unknown()
            && self.soak_readback_window_count == SHADOW_CANARY_PROMOTION_REHEARSAL_COUNT
            && self.soak_readback_pass_count == SHADOW_CANARY_PROMOTION_REHEARSAL_COUNT
            && self.operator_packet_line_count == SHADOW_CANARY_PROMOTION_OPERATOR_PACKET_LINE_COUNT
            && self.operator_packet_redacted
            && self.operator_approval_required
            && !self.history_persistence_write
            && !self.production_route
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
            && !self.canary_promotion_route_opened
            && !self.rollback_write
    }
}
