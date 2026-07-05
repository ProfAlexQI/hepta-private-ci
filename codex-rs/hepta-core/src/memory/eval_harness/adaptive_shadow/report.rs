use super::super::super::CONTEXT_MEMORY_ADAPTIVE_ALLOCATOR_EVAL_SHADOW_SCHEMA_VERSION;
use super::super::ContextMemoryEvalHarnessReport;
use super::super::ContextMemoryEvalMetric;
use super::adaptive_allocator_eval_required_fixture_kinds;
use super::comparison::ContextMemoryAdaptiveAllocatorEvalShadowComparisonVerdict;
use super::result::ContextMemoryAdaptiveAllocatorEvalArm;
use super::result::ContextMemoryAdaptiveAllocatorEvalShadowResult;
use serde::Deserialize;
use serde::Serialize;

/// Offline, behavior-neutral shadow comparison between the current budget
/// heuristic and the proposed adaptive allocator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryAdaptiveAllocatorEvalShadowReport {
    pub schema_version: u32,
    pub metrics: Vec<ContextMemoryEvalMetric>,
    pub shadow_results: Vec<ContextMemoryAdaptiveAllocatorEvalShadowResult>,
    pub comparison_verdict: ContextMemoryAdaptiveAllocatorEvalShadowComparisonVerdict,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub adaptive_allocator_runtime_activation: bool,
    pub source_aware_runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
}

impl Default for ContextMemoryAdaptiveAllocatorEvalShadowReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_ADAPTIVE_ALLOCATOR_EVAL_SHADOW_SCHEMA_VERSION,
            metrics: Vec::new(),
            shadow_results: Vec::new(),
            comparison_verdict: ContextMemoryAdaptiveAllocatorEvalShadowComparisonVerdict::default(
            ),
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

impl ContextMemoryAdaptiveAllocatorEvalShadowReport {
    pub fn seeded() -> Self {
        Self::from_seed(&ContextMemoryEvalHarnessReport::seeded())
    }

    pub fn from_seed(seed: &ContextMemoryEvalHarnessReport) -> Self {
        let mut shadow_results = Vec::new();
        for fixture in &seed.fixtures {
            shadow_results.push(
                ContextMemoryAdaptiveAllocatorEvalShadowResult::from_seed_fixture(
                    ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic,
                    fixture,
                ),
            );
            shadow_results.push(
                ContextMemoryAdaptiveAllocatorEvalShadowResult::from_seed_fixture(
                    ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive,
                    fixture,
                ),
            );
        }

        Self {
            schema_version: CONTEXT_MEMORY_ADAPTIVE_ALLOCATOR_EVAL_SHADOW_SCHEMA_VERSION,
            metrics: ContextMemoryEvalMetric::fixed_seed_metrics(),
            comparison_verdict:
                ContextMemoryAdaptiveAllocatorEvalShadowComparisonVerdict::from_shadow_results(
                    &shadow_results,
                ),
            shadow_results,
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            adaptive_allocator_runtime_activation: false,
            source_aware_runtime_activation: false,
            prompt_assembly_change: false,
            operator_activation_allowed: false,
        }
    }

    pub fn has_eval_shadow_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_ADAPTIVE_ALLOCATOR_EVAL_SHADOW_SCHEMA_VERSION
            && self.metrics == ContextMemoryEvalMetric::fixed_seed_metrics()
            && self
                .shadow_results
                .iter()
                .all(ContextMemoryAdaptiveAllocatorEvalShadowResult::has_eval_shadow_integrity)
            && self.has_required_shadow_pairs()
            && self.passes_shadow_thresholds()
            && self.comparison_verdict
                == ContextMemoryAdaptiveAllocatorEvalShadowComparisonVerdict::from_shadow_results(
                    &self.shadow_results,
                )
            && self.comparison_verdict.has_shadow_threshold_integrity()
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.adaptive_allocator_runtime_activation
            && !self.source_aware_runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
    }

    fn has_required_shadow_pairs(&self) -> bool {
        adaptive_allocator_eval_required_fixture_kinds()
            .into_iter()
            .all(|fixture_kind| {
                self.shadow_result(
                    ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic,
                    fixture_kind,
                )
                .is_some()
                    && self
                        .shadow_result(
                            ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive,
                            fixture_kind,
                        )
                        .is_some()
            })
    }

    fn passes_shadow_thresholds(&self) -> bool {
        adaptive_allocator_eval_required_fixture_kinds()
            .into_iter()
            .all(|fixture_kind| {
                let Some(current) = self.shadow_result(
                    ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic,
                    fixture_kind,
                ) else {
                    return false;
                };
                let Some(proposed) = self.shadow_result(
                    ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive,
                    fixture_kind,
                ) else {
                    return false;
                };

                proposed.missing_critical_fact_count <= current.missing_critical_fact_count
                    && proposed.recall_coverage_basis_points >= current.recall_coverage_basis_points
                    && proposed.precision_basis_points >= current.precision_basis_points
                    && proposed.observed_latency_ms <= current.observed_latency_ms
                    && proposed.token_cost <= current.token_cost
                    && proposed.token_saved >= current.token_saved
                    && proposed.safety_leak_count == 0
                    && proposed.answer_quality_regression_count == 0
                    && !proposed.production_write
                    && !proposed.graph_write
                    && !proposed.runtime_activation
                    && !proposed.adaptive_allocator_runtime_activation
                    && !proposed.source_aware_runtime_activation
                    && !proposed.prompt_assembly_change
                    && !proposed.operator_activation_allowed
            })
    }

    pub fn shadow_result(
        &self,
        arm: ContextMemoryAdaptiveAllocatorEvalArm,
        fixture_kind: super::super::ContextMemoryEvalFixtureKind,
    ) -> Option<&ContextMemoryAdaptiveAllocatorEvalShadowResult> {
        self.shadow_results
            .iter()
            .find(|result| result.arm == arm && result.fixture_kind == fixture_kind)
    }

    pub fn result_count_for_arm(&self, arm: ContextMemoryAdaptiveAllocatorEvalArm) -> usize {
        self.shadow_results
            .iter()
            .filter(|result| result.arm == arm)
            .count()
    }

    pub fn total_missing_critical_fact_count_for_arm(
        &self,
        arm: ContextMemoryAdaptiveAllocatorEvalArm,
    ) -> usize {
        self.shadow_results
            .iter()
            .filter(|result| result.arm == arm)
            .map(|result| result.missing_critical_fact_count)
            .sum()
    }

    pub fn total_token_cost_for_arm(&self, arm: ContextMemoryAdaptiveAllocatorEvalArm) -> usize {
        self.shadow_results
            .iter()
            .filter(|result| result.arm == arm)
            .map(|result| result.token_cost)
            .sum()
    }

    pub fn total_token_saved_for_arm(&self, arm: ContextMemoryAdaptiveAllocatorEvalArm) -> usize {
        self.shadow_results
            .iter()
            .filter(|result| result.arm == arm)
            .map(|result| result.token_saved)
            .sum()
    }

    pub fn total_latency_ms_for_arm(&self, arm: ContextMemoryAdaptiveAllocatorEvalArm) -> u32 {
        self.shadow_results
            .iter()
            .filter(|result| result.arm == arm)
            .fold(0_u32, |latency, result| {
                latency.saturating_add(result.observed_latency_ms)
            })
    }

    pub fn safety_leak_count(&self) -> usize {
        self.shadow_results
            .iter()
            .map(|result| result.safety_leak_count)
            .sum()
    }

    pub fn answer_quality_regression_count(&self) -> usize {
        self.shadow_results
            .iter()
            .map(|result| result.answer_quality_regression_count)
            .sum()
    }
}
