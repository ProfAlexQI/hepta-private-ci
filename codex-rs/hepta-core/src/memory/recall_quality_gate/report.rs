use serde::Deserialize;
use serde::Serialize;

use super::super::CONTEXT_MEMORY_RECALL_QUALITY_GATE_SCHEMA_VERSION;
use super::super::ContextMemoryAdaptiveAllocatorEvalArm;
use super::super::ContextMemoryAdaptiveAllocatorEvalShadowReport;
use super::super::ContextMemoryEvalMetric;
use super::super::basis_points;
use super::super::eval_harness::adaptive_allocator_eval_required_fixture_kinds;
use super::ContextMemoryRecallQualityGateVerdict;
use super::fixture::ContextMemoryRecallQualityFixtureGateReport;

/// Offline, behavior-neutral gate for recall quality thresholds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryRecallQualityGateReport {
    pub schema_version: u32,
    pub verdict: ContextMemoryRecallQualityGateVerdict,
    pub metric_count: usize,
    pub fixture_count: usize,
    pub fixture_matrix: Vec<ContextMemoryRecallQualityFixtureGateReport>,
    pub fixture_gate_pass_count: usize,
    pub fixture_blocked_count: usize,
    pub blocking_reason_count: usize,
    pub critical_fact_count: usize,
    pub recalled_critical_fact_count: usize,
    pub missing_critical_fact_count: usize,
    pub missing_critical_fact_regression_count: usize,
    pub predicted_relevant_count: usize,
    pub false_positive_count: usize,
    pub recall_coverage_floor_basis_points: u32,
    pub observed_recall_coverage_basis_points: u32,
    pub recall_regression_count: usize,
    pub precision_floor_basis_points: u32,
    pub observed_precision_basis_points: u32,
    pub precision_regression_count: usize,
    pub missing_critical_fact_limit: usize,
    pub safety_leak_count: usize,
    pub answer_quality_regression_count: usize,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub adaptive_allocator_runtime_activation: bool,
    pub source_aware_runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
}

impl Default for ContextMemoryRecallQualityGateReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_RECALL_QUALITY_GATE_SCHEMA_VERSION,
            verdict: ContextMemoryRecallQualityGateVerdict::Unknown,
            metric_count: 0,
            fixture_count: 0,
            fixture_matrix: Vec::new(),
            fixture_gate_pass_count: 0,
            fixture_blocked_count: 0,
            blocking_reason_count: 0,
            critical_fact_count: 0,
            recalled_critical_fact_count: 0,
            missing_critical_fact_count: 0,
            missing_critical_fact_regression_count: 0,
            predicted_relevant_count: 0,
            false_positive_count: 0,
            recall_coverage_floor_basis_points: 7_000,
            observed_recall_coverage_basis_points: 0,
            recall_regression_count: 0,
            precision_floor_basis_points: 7_000,
            observed_precision_basis_points: 0,
            precision_regression_count: 0,
            missing_critical_fact_limit: 2,
            safety_leak_count: 0,
            answer_quality_regression_count: 0,
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            adaptive_allocator_runtime_activation: false,
            source_aware_runtime_activation: false,
            prompt_assembly_change: false,
            operator_activation_allowed: false,
        }
    }
}

impl ContextMemoryRecallQualityGateReport {
    pub fn seeded() -> Self {
        Self::from_shadow(&ContextMemoryAdaptiveAllocatorEvalShadowReport::seeded())
    }

    pub fn from_shadow(shadow: &ContextMemoryAdaptiveAllocatorEvalShadowReport) -> Self {
        let fixture_matrix: Vec<_> = adaptive_allocator_eval_required_fixture_kinds()
            .into_iter()
            .filter_map(|fixture_kind| {
                let current = shadow.shadow_result(
                    ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic,
                    fixture_kind,
                )?;
                let proposed = shadow.shadow_result(
                    ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive,
                    fixture_kind,
                )?;
                Some(
                    ContextMemoryRecallQualityFixtureGateReport::from_shadow_pair(
                        current, proposed,
                    ),
                )
            })
            .collect();
        let critical_fact_count = fixture_matrix
            .iter()
            .map(|fixture| fixture.proposed_critical_fact_count)
            .sum();
        let recalled_critical_fact_count = fixture_matrix
            .iter()
            .map(|fixture| fixture.proposed_recalled_critical_fact_count)
            .sum();
        let missing_critical_fact_count = fixture_matrix
            .iter()
            .map(|fixture| fixture.proposed_missing_critical_fact_count)
            .sum();
        let missing_critical_fact_regression_count = fixture_matrix
            .iter()
            .filter(|fixture| fixture.missing_critical_fact_regression)
            .count();
        let predicted_relevant_count = fixture_matrix
            .iter()
            .map(|fixture| fixture.proposed_predicted_relevant_count)
            .sum();
        let false_positive_count = fixture_matrix
            .iter()
            .map(|fixture| fixture.proposed_false_positive_count)
            .sum();
        let recall_regression_count = fixture_matrix
            .iter()
            .filter(|fixture| fixture.recall_regression)
            .count();
        let precision_regression_count = fixture_matrix
            .iter()
            .filter(|fixture| fixture.precision_regression)
            .count();
        let safety_leak_count = fixture_matrix
            .iter()
            .map(|fixture| fixture.safety_leak_count)
            .sum();
        let answer_quality_regression_count = fixture_matrix
            .iter()
            .map(|fixture| fixture.answer_quality_regression_count)
            .sum();
        let fixture_gate_pass_count = fixture_matrix
            .iter()
            .filter(|fixture| fixture.verdict == ContextMemoryRecallQualityGateVerdict::GatePass)
            .count();
        let fixture_blocked_count = fixture_matrix.len().saturating_sub(fixture_gate_pass_count);
        let blocking_reason_count = fixture_matrix
            .iter()
            .map(|fixture| fixture.blocking_reasons.len())
            .sum();
        let production_write = fixture_matrix
            .iter()
            .any(|fixture| fixture.production_write);
        let graph_write = fixture_matrix.iter().any(|fixture| fixture.graph_write);
        let runtime_activation = fixture_matrix
            .iter()
            .any(|fixture| fixture.runtime_activation);
        let adaptive_allocator_runtime_activation = fixture_matrix
            .iter()
            .any(|fixture| fixture.adaptive_allocator_runtime_activation);
        let source_aware_runtime_activation = fixture_matrix
            .iter()
            .any(|fixture| fixture.source_aware_runtime_activation);
        let prompt_assembly_change = fixture_matrix
            .iter()
            .any(|fixture| fixture.prompt_assembly_change);
        let operator_activation_allowed = fixture_matrix
            .iter()
            .any(|fixture| fixture.operator_activation_allowed);
        let mut report = Self {
            schema_version: CONTEXT_MEMORY_RECALL_QUALITY_GATE_SCHEMA_VERSION,
            verdict: ContextMemoryRecallQualityGateVerdict::Blocked,
            metric_count: shadow.metrics.len(),
            fixture_count: fixture_matrix.len(),
            fixture_matrix,
            fixture_gate_pass_count,
            fixture_blocked_count,
            blocking_reason_count,
            critical_fact_count,
            recalled_critical_fact_count,
            missing_critical_fact_count,
            missing_critical_fact_regression_count,
            predicted_relevant_count,
            false_positive_count,
            observed_recall_coverage_basis_points: basis_points(
                recalled_critical_fact_count,
                critical_fact_count,
            ),
            recall_regression_count,
            observed_precision_basis_points: basis_points(
                predicted_relevant_count.saturating_sub(false_positive_count),
                predicted_relevant_count,
            ),
            precision_regression_count,
            safety_leak_count,
            answer_quality_regression_count,
            production_write,
            graph_write,
            runtime_activation,
            adaptive_allocator_runtime_activation,
            source_aware_runtime_activation,
            prompt_assembly_change,
            operator_activation_allowed,
            ..Self::default()
        };
        if report.has_quality_gate_pass_shape() {
            report.verdict = ContextMemoryRecallQualityGateVerdict::GatePass;
        }
        report
    }

    pub fn has_quality_gate_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_RECALL_QUALITY_GATE_SCHEMA_VERSION
            && !self.verdict.is_unknown()
            && self.verdict == ContextMemoryRecallQualityGateVerdict::GatePass
            && self.has_quality_gate_pass_shape()
    }

    fn has_quality_gate_pass_shape(&self) -> bool {
        self.metric_count == ContextMemoryEvalMetric::fixed_seed_metrics().len()
            && self.fixture_count >= adaptive_allocator_eval_required_fixture_kinds().len()
            && self.fixture_count == self.fixture_matrix.len()
            && self.has_required_fixture_matrix()
            && self
                .fixture_matrix
                .iter()
                .all(ContextMemoryRecallQualityFixtureGateReport::has_fixture_gate_integrity)
            && self.fixture_gate_pass_count == self.fixture_matrix.len()
            && self.fixture_blocked_count == 0
            && self.blocking_reason_count == 0
            && self.fixture_matrix_totals_match()
            && self.critical_fact_count > 0
            && self.recalled_critical_fact_count <= self.critical_fact_count
            && self.missing_critical_fact_count
                == self
                    .critical_fact_count
                    .saturating_sub(self.recalled_critical_fact_count)
            && self.false_positive_count <= self.predicted_relevant_count
            && self.predicted_relevant_count >= self.recalled_critical_fact_count
            && self.observed_recall_coverage_basis_points
                == basis_points(self.recalled_critical_fact_count, self.critical_fact_count)
            && self.observed_precision_basis_points
                == basis_points(
                    self.predicted_relevant_count
                        .saturating_sub(self.false_positive_count),
                    self.predicted_relevant_count,
                )
            && self.observed_recall_coverage_basis_points >= self.recall_coverage_floor_basis_points
            && self.missing_critical_fact_regression_count == 0
            && self.observed_precision_basis_points >= self.precision_floor_basis_points
            && self.recall_regression_count == 0
            && self.precision_regression_count == 0
            && self.missing_critical_fact_count <= self.missing_critical_fact_limit
            && self.safety_leak_count == 0
            && self.answer_quality_regression_count == 0
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.adaptive_allocator_runtime_activation
            && !self.source_aware_runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
    }

    fn has_required_fixture_matrix(&self) -> bool {
        adaptive_allocator_eval_required_fixture_kinds()
            .into_iter()
            .all(|fixture_kind| {
                self.fixture_matrix
                    .iter()
                    .filter(|fixture| fixture.fixture_kind == fixture_kind)
                    .count()
                    == 1
            })
    }

    fn fixture_matrix_totals_match(&self) -> bool {
        self.critical_fact_count
            == self
                .fixture_matrix
                .iter()
                .map(|fixture| fixture.proposed_critical_fact_count)
                .sum::<usize>()
            && self.recalled_critical_fact_count
                == self
                    .fixture_matrix
                    .iter()
                    .map(|fixture| fixture.proposed_recalled_critical_fact_count)
                    .sum::<usize>()
            && self.missing_critical_fact_count
                == self
                    .fixture_matrix
                    .iter()
                    .map(|fixture| fixture.proposed_missing_critical_fact_count)
                    .sum::<usize>()
            && self.predicted_relevant_count
                == self
                    .fixture_matrix
                    .iter()
                    .map(|fixture| fixture.proposed_predicted_relevant_count)
                    .sum::<usize>()
            && self.false_positive_count
                == self
                    .fixture_matrix
                    .iter()
                    .map(|fixture| fixture.proposed_false_positive_count)
                    .sum::<usize>()
            && self.missing_critical_fact_regression_count
                == self
                    .fixture_matrix
                    .iter()
                    .filter(|fixture| fixture.missing_critical_fact_regression)
                    .count()
            && self.recall_regression_count
                == self
                    .fixture_matrix
                    .iter()
                    .filter(|fixture| fixture.recall_regression)
                    .count()
            && self.precision_regression_count
                == self
                    .fixture_matrix
                    .iter()
                    .filter(|fixture| fixture.precision_regression)
                    .count()
            && self.safety_leak_count
                == self
                    .fixture_matrix
                    .iter()
                    .map(|fixture| fixture.safety_leak_count)
                    .sum::<usize>()
            && self.answer_quality_regression_count
                == self
                    .fixture_matrix
                    .iter()
                    .map(|fixture| fixture.answer_quality_regression_count)
                    .sum::<usize>()
            && self.blocking_reason_count
                == self
                    .fixture_matrix
                    .iter()
                    .map(|fixture| fixture.blocking_reasons.len())
                    .sum::<usize>()
    }
}
