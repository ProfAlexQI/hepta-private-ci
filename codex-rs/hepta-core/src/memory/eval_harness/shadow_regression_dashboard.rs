use serde::Deserialize;
use serde::Serialize;

use super::super::CONTEXT_MEMORY_SHADOW_REGRESSION_DASHBOARD_SCHEMA_VERSION;
use super::super::ContextMemoryRankedRecallShadowEvalReport;
use super::super::ContextMemoryRecallQualityGateReport;
use super::super::ContextMemoryTemporalGraphShadowEvalReport;
use super::super::MemoryProviderReport;

const SHADOW_REGRESSION_DASHBOARD_INPUT_REPORT_COUNT: usize = 4;

/// Payload-light aggregation mode for memory shadow regression dashboards.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryShadowRegressionDashboardMode {
    ShadowOnly,
    #[default]
    Unknown,
}

impl ContextMemoryShadowRegressionDashboardMode {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light rollup across recall/provider/temporal shadow gates.
///
/// The dashboard carries only aggregate counts, threshold observations, pass
/// counts, and side-effect booleans. It intentionally does not carry prompt
/// text, query text, transcript or memory payloads, ranked payloads, graph
/// payloads, source ids, session ids, memory ids, trace ids, or operator
/// identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryShadowRegressionDashboardReport {
    pub schema_version: u32,
    pub mode: ContextMemoryShadowRegressionDashboardMode,
    pub input_report_count: usize,
    pub input_report_pass_count: usize,
    pub regression_blocking_count: usize,
    pub ranked_recall_fixture_count: usize,
    pub ranked_recall_fixture_pass_count: usize,
    pub ranked_recall_regression_blocked_count: usize,
    pub ranked_recall_min_positive_recall_basis_points: u32,
    pub ranked_recall_min_positive_precision_basis_points: u32,
    pub ranked_recall_total_positive_token_saved: usize,
    pub ranked_recall_max_positive_latency_ms: u32,
    pub ranked_recall_max_positive_regret_basis_points: u32,
    pub ranked_recall_comparison_summary_pass: bool,
    pub ranked_recall_hybrid_signal_count: usize,
    pub ranked_recall_positive_hybrid_signal_pass_count: usize,
    pub ranked_recall_hybrid_regression_blocked_count: usize,
    pub ranked_recall_min_positive_hybrid_score_basis_points: u32,
    pub ranked_recall_calibrated_reranking_fixture_count: usize,
    pub ranked_recall_calibrated_reranking_win_count: usize,
    pub ranked_recall_calibrated_reranking_loss_count: usize,
    pub ranked_recall_min_positive_reranking_delta_basis_points: i32,
    pub ranked_recall_max_positive_latency_delta_ms: i32,
    pub ranked_recall_min_positive_token_tradeoff_basis_points: u32,
    pub ranked_recall_reranking_regression_blocked_count: usize,
    pub ranked_recall_routing_diff_fixture_count: usize,
    pub ranked_recall_routing_diff_shadow_only_count: usize,
    pub ranked_recall_routing_diff_win_count: usize,
    pub ranked_recall_routing_diff_loss_count: usize,
    pub ranked_recall_min_positive_routing_diff_delta_basis_points: i32,
    pub ranked_recall_max_positive_routing_diff_latency_delta_ms: i32,
    pub ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points: u32,
    pub ranked_recall_routing_diff_regression_blocked_count: usize,
    pub temporal_graph_fixture_count: usize,
    pub temporal_graph_fixture_pass_count: usize,
    pub temporal_graph_regression_blocked_count: usize,
    pub temporal_graph_min_positive_node_coverage_basis_points: u32,
    pub temporal_graph_min_positive_edge_coverage_basis_points: u32,
    pub temporal_graph_min_positive_validity_window_coverage_basis_points: u32,
    pub temporal_graph_min_positive_supersedes_coverage_basis_points: u32,
    pub temporal_graph_max_positive_latency_ms: u32,
    pub temporal_graph_max_positive_regret_basis_points: u32,
    pub recall_quality_fixture_count: usize,
    pub recall_quality_fixture_pass_count: usize,
    pub recall_quality_blocking_reason_count: usize,
    pub recall_quality_missing_critical_fact_regression_count: usize,
    pub recall_quality_recall_regression_count: usize,
    pub recall_quality_precision_regression_count: usize,
    pub recall_quality_observed_recall_basis_points: u32,
    pub recall_quality_observed_precision_basis_points: u32,
    pub provider_boundary_pass: bool,
    pub provider_payload_light: bool,
    pub provider_selected_item_count: usize,
    pub provider_ranked_item_count: usize,
    pub provider_estimated_token_count: usize,
    pub operator_approval_required: bool,
    pub production_route: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
}

impl Default for ContextMemoryShadowRegressionDashboardReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_SHADOW_REGRESSION_DASHBOARD_SCHEMA_VERSION,
            mode: ContextMemoryShadowRegressionDashboardMode::Unknown,
            input_report_count: 0,
            input_report_pass_count: 0,
            regression_blocking_count: 0,
            ranked_recall_fixture_count: 0,
            ranked_recall_fixture_pass_count: 0,
            ranked_recall_regression_blocked_count: 0,
            ranked_recall_min_positive_recall_basis_points: 0,
            ranked_recall_min_positive_precision_basis_points: 0,
            ranked_recall_total_positive_token_saved: 0,
            ranked_recall_max_positive_latency_ms: 0,
            ranked_recall_max_positive_regret_basis_points: 0,
            ranked_recall_comparison_summary_pass: false,
            ranked_recall_hybrid_signal_count: 0,
            ranked_recall_positive_hybrid_signal_pass_count: 0,
            ranked_recall_hybrid_regression_blocked_count: 0,
            ranked_recall_min_positive_hybrid_score_basis_points: 0,
            ranked_recall_calibrated_reranking_fixture_count: 0,
            ranked_recall_calibrated_reranking_win_count: 0,
            ranked_recall_calibrated_reranking_loss_count: 0,
            ranked_recall_min_positive_reranking_delta_basis_points: 0,
            ranked_recall_max_positive_latency_delta_ms: 0,
            ranked_recall_min_positive_token_tradeoff_basis_points: 0,
            ranked_recall_reranking_regression_blocked_count: 0,
            ranked_recall_routing_diff_fixture_count: 0,
            ranked_recall_routing_diff_shadow_only_count: 0,
            ranked_recall_routing_diff_win_count: 0,
            ranked_recall_routing_diff_loss_count: 0,
            ranked_recall_min_positive_routing_diff_delta_basis_points: 0,
            ranked_recall_max_positive_routing_diff_latency_delta_ms: 0,
            ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points: 0,
            ranked_recall_routing_diff_regression_blocked_count: 0,
            temporal_graph_fixture_count: 0,
            temporal_graph_fixture_pass_count: 0,
            temporal_graph_regression_blocked_count: 0,
            temporal_graph_min_positive_node_coverage_basis_points: 0,
            temporal_graph_min_positive_edge_coverage_basis_points: 0,
            temporal_graph_min_positive_validity_window_coverage_basis_points: 0,
            temporal_graph_min_positive_supersedes_coverage_basis_points: 0,
            temporal_graph_max_positive_latency_ms: 0,
            temporal_graph_max_positive_regret_basis_points: 0,
            recall_quality_fixture_count: 0,
            recall_quality_fixture_pass_count: 0,
            recall_quality_blocking_reason_count: 0,
            recall_quality_missing_critical_fact_regression_count: 0,
            recall_quality_recall_regression_count: 0,
            recall_quality_precision_regression_count: 0,
            recall_quality_observed_recall_basis_points: 0,
            recall_quality_observed_precision_basis_points: 0,
            provider_boundary_pass: false,
            provider_payload_light: false,
            provider_selected_item_count: 0,
            provider_ranked_item_count: 0,
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

impl ContextMemoryShadowRegressionDashboardReport {
    pub fn from_reports(
        ranked_recall: &ContextMemoryRankedRecallShadowEvalReport,
        temporal_graph: &ContextMemoryTemporalGraphShadowEvalReport,
        recall_quality: &ContextMemoryRecallQualityGateReport,
        provider: &MemoryProviderReport,
    ) -> Self {
        let ranked_recall_pass = ranked_recall.has_ranked_recall_shadow_integrity();
        let ranked_recall_comparison_summary_pass = ranked_recall.hybrid_signal_count() == 5
            && ranked_recall.positive_hybrid_signal_pass_count() == 15
            && ranked_recall.hybrid_regression_blocked_count() == 1
            && ranked_recall.min_positive_hybrid_score_basis_points() >= 7_800
            && ranked_recall.calibrated_reranking_fixture_count() == ranked_recall.fixture_count()
            && ranked_recall.calibrated_reranking_win_count()
                == ranked_recall.positive_fixture_count()
            && ranked_recall.calibrated_reranking_loss_count()
                == ranked_recall.negative_fixture_count()
            && ranked_recall.min_positive_reranking_delta_basis_points() >= 640
            && ranked_recall.max_positive_latency_delta_ms() <= 10
            && ranked_recall.min_positive_token_tradeoff_basis_points() >= 3_000
            && ranked_recall.reranking_regression_blocked_count() == 1
            && ranked_recall.routing_diff_fixture_count() == ranked_recall.fixture_count()
            && ranked_recall.routing_diff_shadow_only_count() == ranked_recall.fixture_count()
            && ranked_recall.routing_diff_win_count() == ranked_recall.positive_fixture_count()
            && ranked_recall.routing_diff_loss_count() == ranked_recall.negative_fixture_count()
            && ranked_recall.min_positive_routing_diff_delta_basis_points() >= 640
            && ranked_recall.max_positive_routing_diff_latency_delta_ms() <= 10
            && ranked_recall.min_positive_routing_diff_token_tradeoff_basis_points() >= 3_000
            && ranked_recall.routing_diff_regression_blocked_count() == 1;
        let temporal_graph_pass = temporal_graph.has_temporal_graph_shadow_integrity();
        let recall_quality_pass = recall_quality.has_quality_gate_integrity();
        let provider_boundary_pass = provider.has_provider_boundary_integrity();
        let provider_payload_light = provider.update_context.has_payload_light_boundary();
        let input_report_pass_count = [
            ranked_recall_pass,
            temporal_graph_pass,
            recall_quality_pass,
            provider_boundary_pass,
        ]
        .into_iter()
        .filter(|passed| *passed)
        .count();
        let regression_blocking_count = SHADOW_REGRESSION_DASHBOARD_INPUT_REPORT_COUNT
            .saturating_sub(input_report_pass_count)
            + recall_quality.blocking_reason_count;

        Self {
            schema_version: CONTEXT_MEMORY_SHADOW_REGRESSION_DASHBOARD_SCHEMA_VERSION,
            mode: ContextMemoryShadowRegressionDashboardMode::ShadowOnly,
            input_report_count: SHADOW_REGRESSION_DASHBOARD_INPUT_REPORT_COUNT,
            input_report_pass_count,
            regression_blocking_count,
            ranked_recall_fixture_count: ranked_recall.fixture_count(),
            ranked_recall_fixture_pass_count: ranked_recall.fixture_pass_count(),
            ranked_recall_regression_blocked_count: ranked_recall.regression_blocked_count(),
            ranked_recall_min_positive_recall_basis_points: ranked_recall
                .min_positive_recall_basis_points(),
            ranked_recall_min_positive_precision_basis_points: ranked_recall
                .min_positive_precision_basis_points(),
            ranked_recall_total_positive_token_saved: ranked_recall.total_positive_token_saved(),
            ranked_recall_max_positive_latency_ms: ranked_recall.max_positive_latency_ms(),
            ranked_recall_max_positive_regret_basis_points: ranked_recall
                .max_positive_regret_basis_points(),
            ranked_recall_comparison_summary_pass,
            ranked_recall_hybrid_signal_count: ranked_recall.hybrid_signal_count(),
            ranked_recall_positive_hybrid_signal_pass_count: ranked_recall
                .positive_hybrid_signal_pass_count(),
            ranked_recall_hybrid_regression_blocked_count: ranked_recall
                .hybrid_regression_blocked_count(),
            ranked_recall_min_positive_hybrid_score_basis_points: ranked_recall
                .min_positive_hybrid_score_basis_points(),
            ranked_recall_calibrated_reranking_fixture_count: ranked_recall
                .calibrated_reranking_fixture_count(),
            ranked_recall_calibrated_reranking_win_count: ranked_recall
                .calibrated_reranking_win_count(),
            ranked_recall_calibrated_reranking_loss_count: ranked_recall
                .calibrated_reranking_loss_count(),
            ranked_recall_min_positive_reranking_delta_basis_points: ranked_recall
                .min_positive_reranking_delta_basis_points(),
            ranked_recall_max_positive_latency_delta_ms: ranked_recall
                .max_positive_latency_delta_ms(),
            ranked_recall_min_positive_token_tradeoff_basis_points: ranked_recall
                .min_positive_token_tradeoff_basis_points(),
            ranked_recall_reranking_regression_blocked_count: ranked_recall
                .reranking_regression_blocked_count(),
            ranked_recall_routing_diff_fixture_count: ranked_recall.routing_diff_fixture_count(),
            ranked_recall_routing_diff_shadow_only_count: ranked_recall
                .routing_diff_shadow_only_count(),
            ranked_recall_routing_diff_win_count: ranked_recall.routing_diff_win_count(),
            ranked_recall_routing_diff_loss_count: ranked_recall.routing_diff_loss_count(),
            ranked_recall_min_positive_routing_diff_delta_basis_points: ranked_recall
                .min_positive_routing_diff_delta_basis_points(),
            ranked_recall_max_positive_routing_diff_latency_delta_ms: ranked_recall
                .max_positive_routing_diff_latency_delta_ms(),
            ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points: ranked_recall
                .min_positive_routing_diff_token_tradeoff_basis_points(),
            ranked_recall_routing_diff_regression_blocked_count: ranked_recall
                .routing_diff_regression_blocked_count(),
            temporal_graph_fixture_count: temporal_graph.fixture_count(),
            temporal_graph_fixture_pass_count: temporal_graph.fixture_pass_count(),
            temporal_graph_regression_blocked_count: temporal_graph.regression_blocked_count(),
            temporal_graph_min_positive_node_coverage_basis_points: temporal_graph
                .min_positive_node_coverage_basis_points(),
            temporal_graph_min_positive_edge_coverage_basis_points: temporal_graph
                .min_positive_edge_coverage_basis_points(),
            temporal_graph_min_positive_validity_window_coverage_basis_points: temporal_graph
                .min_positive_validity_window_coverage_basis_points(),
            temporal_graph_min_positive_supersedes_coverage_basis_points: temporal_graph
                .min_positive_supersedes_coverage_basis_points(),
            temporal_graph_max_positive_latency_ms: temporal_graph.max_positive_latency_ms(),
            temporal_graph_max_positive_regret_basis_points: temporal_graph
                .max_positive_regret_basis_points(),
            recall_quality_fixture_count: recall_quality.fixture_count,
            recall_quality_fixture_pass_count: recall_quality.fixture_gate_pass_count,
            recall_quality_blocking_reason_count: recall_quality.blocking_reason_count,
            recall_quality_missing_critical_fact_regression_count: recall_quality
                .missing_critical_fact_regression_count,
            recall_quality_recall_regression_count: recall_quality.recall_regression_count,
            recall_quality_precision_regression_count: recall_quality.precision_regression_count,
            recall_quality_observed_recall_basis_points: recall_quality
                .observed_recall_coverage_basis_points,
            recall_quality_observed_precision_basis_points: recall_quality
                .observed_precision_basis_points,
            provider_boundary_pass,
            provider_payload_light,
            provider_selected_item_count: provider.update_context.selected_item_count,
            provider_ranked_item_count: provider.update_context.ranked_item_count,
            provider_estimated_token_count: provider.update_context.estimated_token_count,
            operator_approval_required: ranked_recall.operator_approval_required
                && temporal_graph.operator_approval_required
                && provider.update_context.operator_approval_required,
            production_route: ranked_recall.production_route || temporal_graph.production_route,
            production_write: ranked_recall.production_write
                || temporal_graph.production_write
                || recall_quality.production_write
                || provider.update_context.write_performed,
            graph_write: ranked_recall.graph_write
                || temporal_graph.graph_write
                || recall_quality.graph_write,
            runtime_activation: ranked_recall.runtime_activation
                || temporal_graph.runtime_activation
                || recall_quality.runtime_activation
                || provider.update_context.runtime_activation,
            prompt_assembly_change: ranked_recall.prompt_assembly_change
                || temporal_graph.prompt_assembly_change
                || recall_quality.prompt_assembly_change,
            operator_activation_allowed: ranked_recall.operator_activation_allowed
                || temporal_graph.operator_activation_allowed
                || recall_quality.operator_activation_allowed,
        }
    }

    pub fn has_shadow_regression_dashboard_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_SHADOW_REGRESSION_DASHBOARD_SCHEMA_VERSION
            && self.mode == ContextMemoryShadowRegressionDashboardMode::ShadowOnly
            && !self.mode.is_unknown()
            && self.input_report_count == SHADOW_REGRESSION_DASHBOARD_INPUT_REPORT_COUNT
            && self.input_report_pass_count == SHADOW_REGRESSION_DASHBOARD_INPUT_REPORT_COUNT
            && self.regression_blocking_count == 0
            && self.ranked_recall_fixture_count == 4
            && self.ranked_recall_fixture_pass_count == self.ranked_recall_fixture_count
            && self.ranked_recall_regression_blocked_count == 1
            && self.ranked_recall_min_positive_recall_basis_points >= 8_000
            && self.ranked_recall_min_positive_precision_basis_points >= 8_000
            && self.ranked_recall_total_positive_token_saved >= 2_140
            && self.ranked_recall_max_positive_latency_ms <= 55
            && self.ranked_recall_max_positive_regret_basis_points == 0
            && self.ranked_recall_comparison_summary_pass
            && self.ranked_recall_hybrid_signal_count == 5
            && self.ranked_recall_positive_hybrid_signal_pass_count == 15
            && self.ranked_recall_hybrid_regression_blocked_count == 1
            && self.ranked_recall_min_positive_hybrid_score_basis_points >= 7_800
            && self.ranked_recall_calibrated_reranking_fixture_count
                == self.ranked_recall_fixture_count
            && self.ranked_recall_calibrated_reranking_win_count == 3
            && self.ranked_recall_calibrated_reranking_loss_count == 1
            && self.ranked_recall_min_positive_reranking_delta_basis_points >= 640
            && self.ranked_recall_max_positive_latency_delta_ms <= 10
            && self.ranked_recall_min_positive_token_tradeoff_basis_points >= 3_000
            && self.ranked_recall_reranking_regression_blocked_count == 1
            && self.ranked_recall_routing_diff_fixture_count == self.ranked_recall_fixture_count
            && self.ranked_recall_routing_diff_shadow_only_count == self.ranked_recall_fixture_count
            && self.ranked_recall_routing_diff_win_count == 3
            && self.ranked_recall_routing_diff_loss_count == 1
            && self.ranked_recall_min_positive_routing_diff_delta_basis_points >= 640
            && self.ranked_recall_max_positive_routing_diff_latency_delta_ms <= 10
            && self.ranked_recall_min_positive_routing_diff_token_tradeoff_basis_points >= 3_000
            && self.ranked_recall_routing_diff_regression_blocked_count == 1
            && self.temporal_graph_fixture_count == 4
            && self.temporal_graph_fixture_pass_count == self.temporal_graph_fixture_count
            && self.temporal_graph_regression_blocked_count == 1
            && self.temporal_graph_min_positive_node_coverage_basis_points >= 10_000
            && self.temporal_graph_min_positive_edge_coverage_basis_points >= 10_000
            && self.temporal_graph_min_positive_validity_window_coverage_basis_points >= 10_000
            && self.temporal_graph_min_positive_supersedes_coverage_basis_points >= 10_000
            && self.temporal_graph_max_positive_latency_ms <= 47
            && self.temporal_graph_max_positive_regret_basis_points == 0
            && self.recall_quality_fixture_count == 2
            && self.recall_quality_fixture_pass_count == self.recall_quality_fixture_count
            && self.recall_quality_blocking_reason_count == 0
            && self.recall_quality_missing_critical_fact_regression_count == 0
            && self.recall_quality_recall_regression_count == 0
            && self.recall_quality_precision_regression_count == 0
            && self.recall_quality_observed_recall_basis_points >= 7_000
            && self.recall_quality_observed_precision_basis_points >= 7_000
            && self.provider_boundary_pass
            && self.provider_payload_light
            && self.operator_approval_required
            && !self.production_route
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
    }
}
