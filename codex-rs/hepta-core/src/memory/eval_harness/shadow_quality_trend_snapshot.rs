use serde::Deserialize;
use serde::Serialize;

use super::super::CONTEXT_MEMORY_SHADOW_QUALITY_TREND_SNAPSHOT_SCHEMA_VERSION;
use super::super::ContextMemoryShadowQualityOperatorSummary;
use super::super::ContextMemoryShadowQualitySummaryReport;
use super::super::ContextMemoryShadowQualityTrend;

const SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT: usize = 3;
const SHADOW_QUALITY_TREND_SNAPSHOT_REQUIRED_PASS_STREAK: usize = 3;
const SHADOW_QUALITY_TREND_SNAPSHOT_OPERATOR_LINE_COUNT: usize = 5;

/// Payload-light mode for the shadow quality trend snapshot.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryShadowQualityTrendSnapshotMode {
    ShadowOnly,
    #[default]
    Unknown,
}

impl ContextMemoryShadowQualityTrendSnapshotMode {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Controlled trend window verdict for the shadow quality snapshot.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryShadowQualityTrendWindowVerdict {
    StableWindow,
    RegressionBlocked,
    #[default]
    Unknown,
}

impl ContextMemoryShadowQualityTrendWindowVerdict {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light trend snapshot derived from the shadow quality summary.
///
/// The snapshot represents a controlled shadow-only regression window. It
/// exposes only aggregate pass counts, streak counts, selected threshold
/// observations, and side-effect booleans. It does not read or write durable
/// history, and it deliberately avoids prompt text, query text, memory or
/// transcript payloads, ranked or graph payloads, source ids, session ids,
/// memory ids, trace ids, and operator identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryShadowQualityTrendSnapshotReport {
    pub schema_version: u32,
    pub mode: ContextMemoryShadowQualityTrendSnapshotMode,
    pub source_summary_pass: bool,
    pub current_quality_trend: ContextMemoryShadowQualityTrend,
    pub current_operator_summary: ContextMemoryShadowQualityOperatorSummary,
    pub current_regression_blocking_count: usize,
    pub window_observation_count: usize,
    pub required_pass_streak: usize,
    pub observed_pass_streak: usize,
    pub stable_observation_count: usize,
    pub regression_window_blocking_count: usize,
    pub trend_window_verdict: ContextMemoryShadowQualityTrendWindowVerdict,
    pub operator_snapshot_line_count: usize,
    pub operator_snapshot_redacted: bool,
    pub quality_signal_count: usize,
    pub quality_signal_window_pass_count: usize,
    pub ranked_recall_window_pass_count: usize,
    pub temporal_graph_window_pass_count: usize,
    pub recall_quality_window_pass_count: usize,
    pub provider_boundary_window_pass_count: usize,
    pub ranked_recall_min_positive_recall_basis_points: u32,
    pub ranked_recall_min_positive_precision_basis_points: u32,
    pub ranked_recall_total_positive_token_saved: usize,
    pub ranked_recall_max_positive_latency_ms: u32,
    pub temporal_graph_min_positive_node_coverage_basis_points: u32,
    pub temporal_graph_min_positive_edge_coverage_basis_points: u32,
    pub temporal_graph_max_positive_latency_ms: u32,
    pub recall_quality_observed_recall_basis_points: u32,
    pub recall_quality_observed_precision_basis_points: u32,
    pub provider_estimated_token_count: usize,
    pub operator_approval_required: bool,
    pub history_persistence_write: bool,
    pub production_route: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
}

impl Default for ContextMemoryShadowQualityTrendSnapshotReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_SHADOW_QUALITY_TREND_SNAPSHOT_SCHEMA_VERSION,
            mode: ContextMemoryShadowQualityTrendSnapshotMode::Unknown,
            source_summary_pass: false,
            current_quality_trend: ContextMemoryShadowQualityTrend::Unknown,
            current_operator_summary: ContextMemoryShadowQualityOperatorSummary::Unknown,
            current_regression_blocking_count: 0,
            window_observation_count: 0,
            required_pass_streak: 0,
            observed_pass_streak: 0,
            stable_observation_count: 0,
            regression_window_blocking_count: 0,
            trend_window_verdict: ContextMemoryShadowQualityTrendWindowVerdict::Unknown,
            operator_snapshot_line_count: 0,
            operator_snapshot_redacted: false,
            quality_signal_count: 0,
            quality_signal_window_pass_count: 0,
            ranked_recall_window_pass_count: 0,
            temporal_graph_window_pass_count: 0,
            recall_quality_window_pass_count: 0,
            provider_boundary_window_pass_count: 0,
            ranked_recall_min_positive_recall_basis_points: 0,
            ranked_recall_min_positive_precision_basis_points: 0,
            ranked_recall_total_positive_token_saved: 0,
            ranked_recall_max_positive_latency_ms: 0,
            temporal_graph_min_positive_node_coverage_basis_points: 0,
            temporal_graph_min_positive_edge_coverage_basis_points: 0,
            temporal_graph_max_positive_latency_ms: 0,
            recall_quality_observed_recall_basis_points: 0,
            recall_quality_observed_precision_basis_points: 0,
            provider_estimated_token_count: 0,
            operator_approval_required: false,
            history_persistence_write: false,
            production_route: false,
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            prompt_assembly_change: false,
            operator_activation_allowed: false,
        }
    }
}

impl ContextMemoryShadowQualityTrendSnapshotReport {
    pub fn from_summary(summary: &ContextMemoryShadowQualitySummaryReport) -> Self {
        let source_summary_pass = summary.has_shadow_quality_summary_integrity();
        let side_effects_disabled = !summary.production_route
            && !summary.production_write
            && !summary.graph_write
            && !summary.runtime_activation
            && !summary.prompt_assembly_change
            && !summary.operator_activation_allowed;
        let stable_current_observation = source_summary_pass
            && summary.quality_trend == ContextMemoryShadowQualityTrend::StablePass
            && summary.operator_summary
                == ContextMemoryShadowQualityOperatorSummary::ReadyShadowOnly
            && summary.regression_blocking_count == 0
            && summary.quality_signal_count == 4
            && summary.quality_signal_pass_count == summary.quality_signal_count
            && summary.operator_approval_required
            && side_effects_disabled;
        let stable_observation_count = if stable_current_observation {
            SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT
        } else {
            0
        };
        let observed_pass_streak = stable_observation_count;
        let quality_signal_window_pass_count = summary
            .quality_signal_pass_count
            .saturating_mul(SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT);
        let ranked_recall_window_pass_count = if summary.ranked_recall_signal_pass {
            SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT
        } else {
            0
        };
        let temporal_graph_window_pass_count = if summary.temporal_graph_signal_pass {
            SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT
        } else {
            0
        };
        let recall_quality_window_pass_count = if summary.recall_quality_signal_pass {
            SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT
        } else {
            0
        };
        let provider_boundary_window_pass_count = if summary.provider_boundary_signal_pass {
            SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT
        } else {
            0
        };
        let regression_window_blocking_count = summary
            .regression_blocking_count
            .saturating_mul(SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT)
            .saturating_add(
                SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT
                    .saturating_sub(stable_observation_count),
            )
            .saturating_add(usize::from(!source_summary_pass));
        let stable_window = stable_current_observation
            && observed_pass_streak == SHADOW_QUALITY_TREND_SNAPSHOT_REQUIRED_PASS_STREAK
            && stable_observation_count == SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT
            && regression_window_blocking_count == 0
            && quality_signal_window_pass_count
                == summary
                    .quality_signal_count
                    .saturating_mul(SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT);

        Self {
            schema_version: CONTEXT_MEMORY_SHADOW_QUALITY_TREND_SNAPSHOT_SCHEMA_VERSION,
            mode: ContextMemoryShadowQualityTrendSnapshotMode::ShadowOnly,
            source_summary_pass,
            current_quality_trend: summary.quality_trend,
            current_operator_summary: summary.operator_summary,
            current_regression_blocking_count: summary.regression_blocking_count,
            window_observation_count: SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT,
            required_pass_streak: SHADOW_QUALITY_TREND_SNAPSHOT_REQUIRED_PASS_STREAK,
            observed_pass_streak,
            stable_observation_count,
            regression_window_blocking_count,
            trend_window_verdict: if stable_window {
                ContextMemoryShadowQualityTrendWindowVerdict::StableWindow
            } else {
                ContextMemoryShadowQualityTrendWindowVerdict::RegressionBlocked
            },
            operator_snapshot_line_count: SHADOW_QUALITY_TREND_SNAPSHOT_OPERATOR_LINE_COUNT,
            operator_snapshot_redacted: true,
            quality_signal_count: summary.quality_signal_count,
            quality_signal_window_pass_count,
            ranked_recall_window_pass_count,
            temporal_graph_window_pass_count,
            recall_quality_window_pass_count,
            provider_boundary_window_pass_count,
            ranked_recall_min_positive_recall_basis_points: summary
                .ranked_recall_min_positive_recall_basis_points,
            ranked_recall_min_positive_precision_basis_points: summary
                .ranked_recall_min_positive_precision_basis_points,
            ranked_recall_total_positive_token_saved: summary
                .ranked_recall_total_positive_token_saved,
            ranked_recall_max_positive_latency_ms: summary.ranked_recall_max_positive_latency_ms,
            temporal_graph_min_positive_node_coverage_basis_points: summary
                .temporal_graph_min_positive_node_coverage_basis_points,
            temporal_graph_min_positive_edge_coverage_basis_points: summary
                .temporal_graph_min_positive_edge_coverage_basis_points,
            temporal_graph_max_positive_latency_ms: summary.temporal_graph_max_positive_latency_ms,
            recall_quality_observed_recall_basis_points: summary
                .recall_quality_observed_recall_basis_points,
            recall_quality_observed_precision_basis_points: summary
                .recall_quality_observed_precision_basis_points,
            provider_estimated_token_count: summary.provider_estimated_token_count,
            operator_approval_required: summary.operator_approval_required,
            history_persistence_write: false,
            production_route: summary.production_route,
            production_write: summary.production_write,
            graph_write: summary.graph_write,
            runtime_activation: summary.runtime_activation,
            prompt_assembly_change: summary.prompt_assembly_change,
            operator_activation_allowed: summary.operator_activation_allowed,
        }
    }

    pub fn has_shadow_quality_trend_snapshot_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_SHADOW_QUALITY_TREND_SNAPSHOT_SCHEMA_VERSION
            && self.mode == ContextMemoryShadowQualityTrendSnapshotMode::ShadowOnly
            && !self.mode.is_unknown()
            && self.source_summary_pass
            && self.current_quality_trend == ContextMemoryShadowQualityTrend::StablePass
            && !self.current_quality_trend.is_unknown()
            && self.current_operator_summary
                == ContextMemoryShadowQualityOperatorSummary::ReadyShadowOnly
            && !self.current_operator_summary.is_unknown()
            && self.current_regression_blocking_count == 0
            && self.window_observation_count
                == SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT
            && self.required_pass_streak == SHADOW_QUALITY_TREND_SNAPSHOT_REQUIRED_PASS_STREAK
            && self.observed_pass_streak == SHADOW_QUALITY_TREND_SNAPSHOT_REQUIRED_PASS_STREAK
            && self.stable_observation_count
                == SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT
            && self.regression_window_blocking_count == 0
            && self.trend_window_verdict
                == ContextMemoryShadowQualityTrendWindowVerdict::StableWindow
            && !self.trend_window_verdict.is_unknown()
            && self.operator_snapshot_line_count
                == SHADOW_QUALITY_TREND_SNAPSHOT_OPERATOR_LINE_COUNT
            && self.operator_snapshot_redacted
            && self.quality_signal_count == 4
            && self.quality_signal_window_pass_count == 12
            && self.ranked_recall_window_pass_count
                == SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT
            && self.temporal_graph_window_pass_count
                == SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT
            && self.recall_quality_window_pass_count
                == SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT
            && self.provider_boundary_window_pass_count
                == SHADOW_QUALITY_TREND_SNAPSHOT_WINDOW_OBSERVATION_COUNT
            && self.ranked_recall_min_positive_recall_basis_points >= 8_000
            && self.ranked_recall_min_positive_precision_basis_points >= 8_000
            && self.ranked_recall_total_positive_token_saved >= 2_140
            && self.ranked_recall_max_positive_latency_ms <= 55
            && self.temporal_graph_min_positive_node_coverage_basis_points >= 10_000
            && self.temporal_graph_min_positive_edge_coverage_basis_points >= 10_000
            && self.temporal_graph_max_positive_latency_ms <= 47
            && self.recall_quality_observed_recall_basis_points >= 7_000
            && self.recall_quality_observed_precision_basis_points >= 7_000
            && self.operator_approval_required
            && !self.history_persistence_write
            && !self.production_route
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
    }
}
