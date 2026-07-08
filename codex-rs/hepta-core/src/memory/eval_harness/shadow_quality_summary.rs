use serde::Deserialize;
use serde::Serialize;

use super::super::CONTEXT_MEMORY_SHADOW_QUALITY_SUMMARY_SCHEMA_VERSION;
use super::super::ContextMemoryShadowRegressionDashboardReport;

const SHADOW_QUALITY_SUMMARY_SIGNAL_COUNT: usize = 4;
const SHADOW_QUALITY_SUMMARY_OPERATOR_LINE_COUNT: usize = 4;

/// Payload-light mode for the shadow quality summary.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryShadowQualitySummaryMode {
    ShadowOnly,
    #[default]
    Unknown,
}

impl ContextMemoryShadowQualitySummaryMode {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Controlled trend verdict for the shadow quality summary.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryShadowQualityTrend {
    StablePass,
    RegressionBlocked,
    #[default]
    Unknown,
}

impl ContextMemoryShadowQualityTrend {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Controlled operator-facing summary posture.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryShadowQualityOperatorSummary {
    ReadyShadowOnly,
    BlockedRegression,
    #[default]
    Unknown,
}

impl ContextMemoryShadowQualityOperatorSummary {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light quality summary derived from the shadow regression dashboard.
///
/// The summary exposes only controlled trend enums, aggregate signal counts,
/// selected threshold observations, and side-effect booleans. It deliberately
/// avoids prompt text, query text, memory/transcript/answer payloads, ranked or
/// graph payloads, source ids, session ids, trace ids, and operator identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryShadowQualitySummaryReport {
    pub schema_version: u32,
    pub mode: ContextMemoryShadowQualitySummaryMode,
    pub quality_trend: ContextMemoryShadowQualityTrend,
    pub operator_summary: ContextMemoryShadowQualityOperatorSummary,
    pub source_dashboard_pass: bool,
    pub source_input_report_count: usize,
    pub source_input_report_pass_count: usize,
    pub quality_signal_count: usize,
    pub quality_signal_pass_count: usize,
    pub regression_blocking_count: usize,
    pub operator_summary_line_count: usize,
    pub operator_summary_redacted: bool,
    pub ranked_recall_signal_pass: bool,
    pub ranked_recall_min_positive_recall_basis_points: u32,
    pub ranked_recall_min_positive_precision_basis_points: u32,
    pub ranked_recall_total_positive_token_saved: usize,
    pub ranked_recall_max_positive_latency_ms: u32,
    pub ranked_recall_comparison_summary_pass: bool,
    pub ranked_recall_hybrid_signal_count: usize,
    pub ranked_recall_positive_hybrid_signal_pass_count: usize,
    pub ranked_recall_hybrid_regression_blocked_count: usize,
    pub ranked_recall_min_positive_hybrid_score_basis_points: u32,
    pub ranked_recall_calibrated_reranking_win_count: usize,
    pub ranked_recall_calibrated_reranking_loss_count: usize,
    pub ranked_recall_min_positive_reranking_delta_basis_points: i32,
    pub ranked_recall_max_positive_latency_delta_ms: i32,
    pub ranked_recall_min_positive_token_tradeoff_basis_points: u32,
    pub ranked_recall_reranking_regression_blocked_count: usize,
    pub ranked_recall_routing_diff_shadow_only_count: usize,
    pub ranked_recall_routing_diff_win_count: usize,
    pub ranked_recall_routing_diff_loss_count: usize,
    pub ranked_recall_min_positive_routing_diff_delta_basis_points: i32,
    pub ranked_recall_max_positive_routing_diff_latency_delta_ms: i32,
    pub ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points: u32,
    pub ranked_recall_routing_diff_regression_blocked_count: usize,
    pub temporal_graph_signal_pass: bool,
    pub temporal_graph_min_positive_node_coverage_basis_points: u32,
    pub temporal_graph_min_positive_edge_coverage_basis_points: u32,
    pub temporal_graph_max_positive_latency_ms: u32,
    pub recall_quality_signal_pass: bool,
    pub recall_quality_observed_recall_basis_points: u32,
    pub recall_quality_observed_precision_basis_points: u32,
    pub provider_boundary_signal_pass: bool,
    pub provider_estimated_token_count: usize,
    pub operator_approval_required: bool,
    pub production_route: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
}

impl Default for ContextMemoryShadowQualitySummaryReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_SHADOW_QUALITY_SUMMARY_SCHEMA_VERSION,
            mode: ContextMemoryShadowQualitySummaryMode::Unknown,
            quality_trend: ContextMemoryShadowQualityTrend::Unknown,
            operator_summary: ContextMemoryShadowQualityOperatorSummary::Unknown,
            source_dashboard_pass: false,
            source_input_report_count: 0,
            source_input_report_pass_count: 0,
            quality_signal_count: 0,
            quality_signal_pass_count: 0,
            regression_blocking_count: 0,
            operator_summary_line_count: 0,
            operator_summary_redacted: false,
            ranked_recall_signal_pass: false,
            ranked_recall_min_positive_recall_basis_points: 0,
            ranked_recall_min_positive_precision_basis_points: 0,
            ranked_recall_total_positive_token_saved: 0,
            ranked_recall_max_positive_latency_ms: 0,
            ranked_recall_comparison_summary_pass: false,
            ranked_recall_hybrid_signal_count: 0,
            ranked_recall_positive_hybrid_signal_pass_count: 0,
            ranked_recall_hybrid_regression_blocked_count: 0,
            ranked_recall_min_positive_hybrid_score_basis_points: 0,
            ranked_recall_calibrated_reranking_win_count: 0,
            ranked_recall_calibrated_reranking_loss_count: 0,
            ranked_recall_min_positive_reranking_delta_basis_points: 0,
            ranked_recall_max_positive_latency_delta_ms: 0,
            ranked_recall_min_positive_token_tradeoff_basis_points: 0,
            ranked_recall_reranking_regression_blocked_count: 0,
            ranked_recall_routing_diff_shadow_only_count: 0,
            ranked_recall_routing_diff_win_count: 0,
            ranked_recall_routing_diff_loss_count: 0,
            ranked_recall_min_positive_routing_diff_delta_basis_points: 0,
            ranked_recall_max_positive_routing_diff_latency_delta_ms: 0,
            ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points: 0,
            ranked_recall_routing_diff_regression_blocked_count: 0,
            temporal_graph_signal_pass: false,
            temporal_graph_min_positive_node_coverage_basis_points: 0,
            temporal_graph_min_positive_edge_coverage_basis_points: 0,
            temporal_graph_max_positive_latency_ms: 0,
            recall_quality_signal_pass: false,
            recall_quality_observed_recall_basis_points: 0,
            recall_quality_observed_precision_basis_points: 0,
            provider_boundary_signal_pass: false,
            provider_estimated_token_count: 0,
            operator_approval_required: false,
            production_route: false,
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            prompt_assembly_change: false,
            operator_activation_allowed: false,
        }
    }
}

impl ContextMemoryShadowQualitySummaryReport {
    pub fn from_dashboard(dashboard: &ContextMemoryShadowRegressionDashboardReport) -> Self {
        let ranked_recall_signal_pass = dashboard.ranked_recall_fixture_count == 4
            && dashboard.ranked_recall_fixture_pass_count == dashboard.ranked_recall_fixture_count
            && dashboard.ranked_recall_regression_blocked_count == 1
            && dashboard.ranked_recall_min_positive_recall_basis_points >= 8_000
            && dashboard.ranked_recall_min_positive_precision_basis_points >= 8_000
            && dashboard.ranked_recall_total_positive_token_saved >= 2_140
            && dashboard.ranked_recall_max_positive_latency_ms <= 55
            && dashboard.ranked_recall_max_positive_regret_basis_points == 0
            && dashboard.ranked_recall_comparison_summary_pass
            && dashboard.ranked_recall_hybrid_signal_count == 5
            && dashboard.ranked_recall_positive_hybrid_signal_pass_count == 15
            && dashboard.ranked_recall_hybrid_regression_blocked_count == 1
            && dashboard.ranked_recall_min_positive_hybrid_score_basis_points >= 7_800
            && dashboard.ranked_recall_calibrated_reranking_win_count == 3
            && dashboard.ranked_recall_calibrated_reranking_loss_count == 1
            && dashboard.ranked_recall_min_positive_reranking_delta_basis_points >= 640
            && dashboard.ranked_recall_max_positive_latency_delta_ms <= 10
            && dashboard.ranked_recall_min_positive_token_tradeoff_basis_points >= 3_000
            && dashboard.ranked_recall_reranking_regression_blocked_count == 1
            && dashboard.ranked_recall_routing_diff_shadow_only_count
                == dashboard.ranked_recall_fixture_count
            && dashboard.ranked_recall_routing_diff_win_count == 3
            && dashboard.ranked_recall_routing_diff_loss_count == 1
            && dashboard.ranked_recall_min_positive_routing_diff_delta_basis_points >= 640
            && dashboard.ranked_recall_max_positive_routing_diff_latency_delta_ms <= 10
            && dashboard.ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points
                >= 3_000
            && dashboard.ranked_recall_routing_diff_regression_blocked_count == 1;
        let temporal_graph_signal_pass = dashboard.temporal_graph_fixture_count == 4
            && dashboard.temporal_graph_fixture_pass_count
                == dashboard.temporal_graph_fixture_count
            && dashboard.temporal_graph_regression_blocked_count == 1
            && dashboard.temporal_graph_min_positive_node_coverage_basis_points >= 10_000
            && dashboard.temporal_graph_min_positive_edge_coverage_basis_points >= 10_000
            && dashboard.temporal_graph_min_positive_validity_window_coverage_basis_points
                >= 10_000
            && dashboard.temporal_graph_min_positive_supersedes_coverage_basis_points >= 10_000
            && dashboard.temporal_graph_max_positive_latency_ms <= 47
            && dashboard.temporal_graph_max_positive_regret_basis_points == 0;
        let recall_quality_signal_pass = dashboard.recall_quality_fixture_count == 2
            && dashboard.recall_quality_fixture_pass_count
                == dashboard.recall_quality_fixture_count
            && dashboard.recall_quality_blocking_reason_count == 0
            && dashboard.recall_quality_missing_critical_fact_regression_count == 0
            && dashboard.recall_quality_recall_regression_count == 0
            && dashboard.recall_quality_precision_regression_count == 0
            && dashboard.recall_quality_observed_recall_basis_points >= 7_000
            && dashboard.recall_quality_observed_precision_basis_points >= 7_000;
        let provider_boundary_signal_pass =
            dashboard.provider_boundary_pass && dashboard.provider_payload_light;
        let source_dashboard_pass = dashboard.has_shadow_regression_dashboard_integrity();
        let quality_signal_pass_count = [
            ranked_recall_signal_pass,
            temporal_graph_signal_pass,
            recall_quality_signal_pass,
            provider_boundary_signal_pass,
        ]
        .into_iter()
        .filter(|passed| *passed)
        .count();
        let signal_blocking_count =
            SHADOW_QUALITY_SUMMARY_SIGNAL_COUNT.saturating_sub(quality_signal_pass_count);
        let source_dashboard_blocking_count = usize::from(!source_dashboard_pass);
        let regression_blocking_count = dashboard
            .regression_blocking_count
            .saturating_add(signal_blocking_count)
            .saturating_add(source_dashboard_blocking_count);
        let side_effects_disabled = !dashboard.production_route
            && !dashboard.production_write
            && !dashboard.graph_write
            && !dashboard.runtime_activation
            && !dashboard.prompt_assembly_change
            && !dashboard.operator_activation_allowed;
        let stable_shadow_summary = source_dashboard_pass
            && quality_signal_pass_count == SHADOW_QUALITY_SUMMARY_SIGNAL_COUNT
            && regression_blocking_count == 0
            && dashboard.operator_approval_required
            && side_effects_disabled;

        Self {
            schema_version: CONTEXT_MEMORY_SHADOW_QUALITY_SUMMARY_SCHEMA_VERSION,
            mode: ContextMemoryShadowQualitySummaryMode::ShadowOnly,
            quality_trend: if stable_shadow_summary {
                ContextMemoryShadowQualityTrend::StablePass
            } else {
                ContextMemoryShadowQualityTrend::RegressionBlocked
            },
            operator_summary: if stable_shadow_summary {
                ContextMemoryShadowQualityOperatorSummary::ReadyShadowOnly
            } else {
                ContextMemoryShadowQualityOperatorSummary::BlockedRegression
            },
            source_dashboard_pass,
            source_input_report_count: dashboard.input_report_count,
            source_input_report_pass_count: dashboard.input_report_pass_count,
            quality_signal_count: SHADOW_QUALITY_SUMMARY_SIGNAL_COUNT,
            quality_signal_pass_count,
            regression_blocking_count,
            operator_summary_line_count: SHADOW_QUALITY_SUMMARY_OPERATOR_LINE_COUNT,
            operator_summary_redacted: true,
            ranked_recall_signal_pass,
            ranked_recall_min_positive_recall_basis_points: dashboard
                .ranked_recall_min_positive_recall_basis_points,
            ranked_recall_min_positive_precision_basis_points: dashboard
                .ranked_recall_min_positive_precision_basis_points,
            ranked_recall_total_positive_token_saved: dashboard
                .ranked_recall_total_positive_token_saved,
            ranked_recall_max_positive_latency_ms: dashboard.ranked_recall_max_positive_latency_ms,
            ranked_recall_comparison_summary_pass: dashboard.ranked_recall_comparison_summary_pass,
            ranked_recall_hybrid_signal_count: dashboard.ranked_recall_hybrid_signal_count,
            ranked_recall_positive_hybrid_signal_pass_count: dashboard
                .ranked_recall_positive_hybrid_signal_pass_count,
            ranked_recall_hybrid_regression_blocked_count: dashboard
                .ranked_recall_hybrid_regression_blocked_count,
            ranked_recall_min_positive_hybrid_score_basis_points: dashboard
                .ranked_recall_min_positive_hybrid_score_basis_points,
            ranked_recall_calibrated_reranking_win_count: dashboard
                .ranked_recall_calibrated_reranking_win_count,
            ranked_recall_calibrated_reranking_loss_count: dashboard
                .ranked_recall_calibrated_reranking_loss_count,
            ranked_recall_min_positive_reranking_delta_basis_points: dashboard
                .ranked_recall_min_positive_reranking_delta_basis_points,
            ranked_recall_max_positive_latency_delta_ms: dashboard
                .ranked_recall_max_positive_latency_delta_ms,
            ranked_recall_min_positive_token_tradeoff_basis_points: dashboard
                .ranked_recall_min_positive_token_tradeoff_basis_points,
            ranked_recall_reranking_regression_blocked_count: dashboard
                .ranked_recall_reranking_regression_blocked_count,
            ranked_recall_routing_diff_shadow_only_count: dashboard
                .ranked_recall_routing_diff_shadow_only_count,
            ranked_recall_routing_diff_win_count: dashboard.ranked_recall_routing_diff_win_count,
            ranked_recall_routing_diff_loss_count: dashboard.ranked_recall_routing_diff_loss_count,
            ranked_recall_min_positive_routing_diff_delta_basis_points: dashboard
                .ranked_recall_min_positive_routing_diff_delta_basis_points,
            ranked_recall_max_positive_routing_diff_latency_delta_ms: dashboard
                .ranked_recall_max_positive_routing_diff_latency_delta_ms,
            ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points: dashboard
                .ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points,
            ranked_recall_routing_diff_regression_blocked_count: dashboard
                .ranked_recall_routing_diff_regression_blocked_count,
            temporal_graph_signal_pass,
            temporal_graph_min_positive_node_coverage_basis_points: dashboard
                .temporal_graph_min_positive_node_coverage_basis_points,
            temporal_graph_min_positive_edge_coverage_basis_points: dashboard
                .temporal_graph_min_positive_edge_coverage_basis_points,
            temporal_graph_max_positive_latency_ms: dashboard
                .temporal_graph_max_positive_latency_ms,
            recall_quality_signal_pass,
            recall_quality_observed_recall_basis_points: dashboard
                .recall_quality_observed_recall_basis_points,
            recall_quality_observed_precision_basis_points: dashboard
                .recall_quality_observed_precision_basis_points,
            provider_boundary_signal_pass,
            provider_estimated_token_count: dashboard.provider_estimated_token_count,
            operator_approval_required: dashboard.operator_approval_required,
            production_route: dashboard.production_route,
            production_write: dashboard.production_write,
            graph_write: dashboard.graph_write,
            runtime_activation: dashboard.runtime_activation,
            prompt_assembly_change: dashboard.prompt_assembly_change,
            operator_activation_allowed: dashboard.operator_activation_allowed,
        }
    }

    pub fn has_shadow_quality_summary_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_SHADOW_QUALITY_SUMMARY_SCHEMA_VERSION
            && self.mode == ContextMemoryShadowQualitySummaryMode::ShadowOnly
            && !self.mode.is_unknown()
            && self.quality_trend == ContextMemoryShadowQualityTrend::StablePass
            && !self.quality_trend.is_unknown()
            && self.operator_summary == ContextMemoryShadowQualityOperatorSummary::ReadyShadowOnly
            && !self.operator_summary.is_unknown()
            && self.source_dashboard_pass
            && self.source_input_report_count == 4
            && self.source_input_report_pass_count == 4
            && self.quality_signal_count == SHADOW_QUALITY_SUMMARY_SIGNAL_COUNT
            && self.quality_signal_pass_count == SHADOW_QUALITY_SUMMARY_SIGNAL_COUNT
            && self.regression_blocking_count == 0
            && self.operator_summary_line_count == SHADOW_QUALITY_SUMMARY_OPERATOR_LINE_COUNT
            && self.operator_summary_redacted
            && self.ranked_recall_signal_pass
            && self.ranked_recall_min_positive_recall_basis_points >= 8_000
            && self.ranked_recall_min_positive_precision_basis_points >= 8_000
            && self.ranked_recall_total_positive_token_saved >= 2_140
            && self.ranked_recall_max_positive_latency_ms <= 55
            && self.ranked_recall_comparison_summary_pass
            && self.ranked_recall_hybrid_signal_count == 5
            && self.ranked_recall_positive_hybrid_signal_pass_count == 15
            && self.ranked_recall_hybrid_regression_blocked_count == 1
            && self.ranked_recall_min_positive_hybrid_score_basis_points >= 7_800
            && self.ranked_recall_calibrated_reranking_win_count == 3
            && self.ranked_recall_calibrated_reranking_loss_count == 1
            && self.ranked_recall_min_positive_reranking_delta_basis_points >= 640
            && self.ranked_recall_max_positive_latency_delta_ms <= 10
            && self.ranked_recall_min_positive_token_tradeoff_basis_points >= 3_000
            && self.ranked_recall_reranking_regression_blocked_count == 1
            && self.ranked_recall_routing_diff_shadow_only_count == 4
            && self.ranked_recall_routing_diff_win_count == 3
            && self.ranked_recall_routing_diff_loss_count == 1
            && self.ranked_recall_min_positive_routing_diff_delta_basis_points >= 640
            && self.ranked_recall_max_positive_routing_diff_latency_delta_ms <= 10
            && self.ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points >= 3_000
            && self.ranked_recall_routing_diff_regression_blocked_count == 1
            && self.temporal_graph_signal_pass
            && self.temporal_graph_min_positive_node_coverage_basis_points >= 10_000
            && self.temporal_graph_min_positive_edge_coverage_basis_points >= 10_000
            && self.temporal_graph_max_positive_latency_ms <= 47
            && self.recall_quality_signal_pass
            && self.recall_quality_observed_recall_basis_points >= 7_000
            && self.recall_quality_observed_precision_basis_points >= 7_000
            && self.provider_boundary_signal_pass
            && self.operator_approval_required
            && !self.production_route
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
    }
}
