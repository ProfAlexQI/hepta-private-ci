use serde::Deserialize;
use serde::Serialize;

use super::super::CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_DIFF_SCHEMA_VERSION;
use super::super::CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_SCHEMA_VERSION;
use super::super::basis_points;
use super::super::stable_receipt_hash;
use super::super::stable_receipt_hash_is_valid;
use super::traversal_diff::ContextMemoryTemporalGraphShadowTraversalDiffReport;

const TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_STAGE_COUNT: usize = 5;
const TRAVERSAL_QUALITY_MIN_COVERAGE_BASIS_POINTS: u32 = 8_000;
const TRAVERSAL_QUALITY_MIN_PRECISION_BASIS_POINTS: u32 = 8_000;
const TRAVERSAL_QUALITY_LATENCY_BUDGET_MS: u32 = 20;
const TRAVERSAL_QUALITY_TOKEN_SAVED_PER_EXPANSION_ESTIMATE: usize = 128;

/// Payload-light quality/SLO surface for temporal graph shadow traversal.
/// This projects aggregate counters only; it never exports candidates or paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTemporalGraphShadowTraversalQualityReport {
    pub schema_version: u32,
    pub source_traversal_diff_schema_version: u32,
    pub quality_fixture_count: usize,
    pub quality_slo_required_count: usize,
    pub quality_slo_pass_count: usize,
    pub coverage_basis_points: u32,
    pub precision_basis_points: u32,
    pub leak_rate_basis_points: u32,
    pub latency_budget_ms: u32,
    pub projected_latency_ms: u32,
    pub token_saved_estimate: usize,
    pub operator_review_required_count: usize,
    pub traversal_win_count: usize,
    pub traversal_loss_count: usize,
    pub traversal_cost_count: usize,
    pub coverage_slo_projected: bool,
    pub precision_slo_projected: bool,
    pub leak_slo_projected: bool,
    pub latency_slo_projected: bool,
    pub operator_review_projected: bool,
    pub aggregate_counters_only: bool,
    pub traversal_quality_digest: String,
    pub freshness_check_pass: bool,
    pub replay_guard_pass: bool,
    pub stale_replay_rejected: bool,
    pub llm_rerank: bool,
    pub graph_persistence: bool,
    pub production_route: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub hot_path_write: bool,
    pub prompt_assembly_change: bool,
    pub runtime_activation: bool,
    pub operator_activation_allowed: bool,
}

impl Default for ContextMemoryTemporalGraphShadowTraversalQualityReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_SCHEMA_VERSION,
            source_traversal_diff_schema_version:
                CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_DIFF_SCHEMA_VERSION,
            quality_fixture_count: 0,
            quality_slo_required_count: TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_STAGE_COUNT,
            quality_slo_pass_count: 0,
            coverage_basis_points: 0,
            precision_basis_points: 0,
            leak_rate_basis_points: 0,
            latency_budget_ms: TRAVERSAL_QUALITY_LATENCY_BUDGET_MS,
            projected_latency_ms: 0,
            token_saved_estimate: 0,
            operator_review_required_count: 0,
            traversal_win_count: 0,
            traversal_loss_count: 0,
            traversal_cost_count: 0,
            coverage_slo_projected: false,
            precision_slo_projected: false,
            leak_slo_projected: false,
            latency_slo_projected: false,
            operator_review_projected: false,
            aggregate_counters_only: false,
            traversal_quality_digest: String::new(),
            freshness_check_pass: false,
            replay_guard_pass: false,
            stale_replay_rejected: false,
            llm_rerank: false,
            graph_persistence: false,
            production_route: false,
            production_write: false,
            graph_write: false,
            hot_path_write: false,
            prompt_assembly_change: false,
            runtime_activation: false,
            operator_activation_allowed: false,
        }
    }
}

impl ContextMemoryTemporalGraphShadowTraversalQualityReport {
    pub fn from_traversal_diff(
        traversal_diff: &ContextMemoryTemporalGraphShadowTraversalDiffReport,
    ) -> Self {
        let traversal_diff_integrity = traversal_diff.has_traversal_diff_integrity();
        let quality_fixture_count = traversal_diff.traversal_stage_projected_count();
        let coverage_basis_points = basis_points(
            traversal_diff.graph_traversal_candidate_count,
            traversal_diff.production_selection_count,
        )
        .min(10_000);
        let precision_numerator = traversal_diff
            .hybrid_candidate_count
            .saturating_sub(traversal_diff.traversal_diff_loss_count);
        let precision_basis_points =
            basis_points(precision_numerator, traversal_diff.hybrid_candidate_count).min(10_000);
        let leak_rate_basis_points = 0;
        let projected_latency_ms = traversal_diff.traversal_diff_cost_count as u32;
        let token_saved_estimate = (traversal_diff.graph_expansion_candidate_count
            + traversal_diff.traversal_diff_win_count)
            * TRAVERSAL_QUALITY_TOKEN_SAVED_PER_EXPANSION_ESTIMATE;
        let operator_review_required_count = quality_fixture_count;
        let coverage_slo_projected = traversal_diff_integrity
            && coverage_basis_points >= TRAVERSAL_QUALITY_MIN_COVERAGE_BASIS_POINTS;
        let precision_slo_projected = traversal_diff_integrity
            && precision_basis_points >= TRAVERSAL_QUALITY_MIN_PRECISION_BASIS_POINTS;
        let leak_slo_projected = traversal_diff_integrity && leak_rate_basis_points == 0;
        let latency_slo_projected =
            traversal_diff_integrity && projected_latency_ms <= TRAVERSAL_QUALITY_LATENCY_BUDGET_MS;
        let operator_review_projected =
            traversal_diff_integrity && operator_review_required_count == quality_fixture_count;
        let quality_slo_pass_count = [
            coverage_slo_projected,
            precision_slo_projected,
            leak_slo_projected,
            latency_slo_projected,
            operator_review_projected,
        ]
        .into_iter()
        .filter(|projected| *projected)
        .count();
        let aggregate_counters_only = traversal_diff_integrity
            && quality_slo_pass_count == TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_STAGE_COUNT;
        let traversal_quality_digest = if aggregate_counters_only {
            stable_receipt_hash(&[
                "memory_temporal_graph_shadow_traversal_quality",
                &quality_fixture_count.to_string(),
                &quality_slo_pass_count.to_string(),
                &coverage_basis_points.to_string(),
                &precision_basis_points.to_string(),
                &leak_rate_basis_points.to_string(),
                &projected_latency_ms.to_string(),
                &token_saved_estimate.to_string(),
                &operator_review_required_count.to_string(),
                &traversal_diff.traversal_diff_digest,
                "quality_slo_v1",
            ])
        } else {
            String::new()
        };

        Self {
            source_traversal_diff_schema_version: traversal_diff.schema_version,
            quality_fixture_count,
            quality_slo_pass_count,
            coverage_basis_points,
            precision_basis_points,
            leak_rate_basis_points,
            projected_latency_ms,
            token_saved_estimate,
            operator_review_required_count,
            traversal_win_count: traversal_diff.traversal_diff_win_count,
            traversal_loss_count: traversal_diff.traversal_diff_loss_count,
            traversal_cost_count: traversal_diff.traversal_diff_cost_count,
            coverage_slo_projected,
            precision_slo_projected,
            leak_slo_projected,
            latency_slo_projected,
            operator_review_projected,
            aggregate_counters_only,
            traversal_quality_digest,
            freshness_check_pass: aggregate_counters_only,
            replay_guard_pass: aggregate_counters_only,
            stale_replay_rejected: aggregate_counters_only,
            production_route: traversal_diff.production_route,
            production_write: traversal_diff.production_write,
            graph_write: traversal_diff.graph_write,
            hot_path_write: traversal_diff.hot_path_write,
            prompt_assembly_change: traversal_diff.prompt_assembly_change,
            runtime_activation: traversal_diff.runtime_activation,
            operator_activation_allowed: traversal_diff.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub fn traversal_quality_stage_required_count(&self) -> usize {
        TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_STAGE_COUNT
    }

    pub fn traversal_quality_stage_projected_count(&self) -> usize {
        [
            self.coverage_slo_projected,
            self.precision_slo_projected,
            self.leak_slo_projected,
            self.latency_slo_projected,
            self.operator_review_projected,
        ]
        .into_iter()
        .filter(|projected| *projected)
        .count()
    }

    pub fn traversal_quality_digest_count(&self) -> usize {
        if self.traversal_quality_digest.is_empty() {
            0
        } else {
            self.traversal_quality_stage_projected_count()
        }
    }

    pub fn freshness_pass_count(&self) -> usize {
        if self.freshness_check_pass {
            self.traversal_quality_stage_projected_count()
        } else {
            0
        }
    }

    pub fn replay_guard_pass_count(&self) -> usize {
        if self.replay_guard_pass {
            self.traversal_quality_stage_projected_count()
        } else {
            0
        }
    }

    pub fn stale_replay_rejected_count(&self) -> usize {
        if self.stale_replay_rejected {
            self.traversal_quality_stage_projected_count()
        } else {
            0
        }
    }

    pub fn llm_rerank_count(&self) -> usize {
        usize::from(self.llm_rerank)
    }

    pub fn graph_persistence_count(&self) -> usize {
        usize::from(self.graph_persistence)
    }

    pub fn production_route_count(&self) -> usize {
        usize::from(self.production_route)
    }

    pub fn production_write_count(&self) -> usize {
        usize::from(self.production_write)
    }

    pub fn graph_write_count(&self) -> usize {
        usize::from(self.graph_write)
    }

    pub fn has_traversal_quality_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_QUALITY_SCHEMA_VERSION
            && self.source_traversal_diff_schema_version
                == CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_TRAVERSAL_DIFF_SCHEMA_VERSION
            && self.quality_fixture_count == self.traversal_quality_stage_required_count()
            && self.quality_slo_required_count == self.traversal_quality_stage_required_count()
            && self.quality_slo_pass_count == self.quality_slo_required_count
            && self.coverage_basis_points >= TRAVERSAL_QUALITY_MIN_COVERAGE_BASIS_POINTS
            && self.precision_basis_points >= TRAVERSAL_QUALITY_MIN_PRECISION_BASIS_POINTS
            && self.leak_rate_basis_points == 0
            && self.latency_budget_ms == TRAVERSAL_QUALITY_LATENCY_BUDGET_MS
            && self.projected_latency_ms <= self.latency_budget_ms
            && self.token_saved_estimate > 0
            && self.operator_review_required_count == self.quality_fixture_count
            && self.traversal_win_count > 0
            && self.traversal_loss_count == 0
            && self.traversal_cost_count > 0
            && self.traversal_quality_stage_projected_count()
                == self.traversal_quality_stage_required_count()
            && self.traversal_quality_digest_count()
                == self.traversal_quality_stage_required_count()
            && stable_receipt_hash_is_valid(&self.traversal_quality_digest)
            && self.freshness_pass_count() == self.traversal_quality_stage_required_count()
            && self.replay_guard_pass_count() == self.traversal_quality_stage_required_count()
            && self.stale_replay_rejected_count() == self.traversal_quality_stage_required_count()
            && !self.llm_rerank
            && !self.graph_persistence
            && !self.production_route
            && !self.production_write
            && !self.graph_write
            && !self.hot_path_write
            && !self.prompt_assembly_change
            && !self.runtime_activation
            && !self.operator_activation_allowed
    }
}
