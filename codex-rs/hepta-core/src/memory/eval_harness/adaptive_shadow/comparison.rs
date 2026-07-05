use super::super::ContextMemoryEvalFixtureKind;
use super::adaptive_allocator_eval_required_fixture_kinds;
use super::result::ContextMemoryAdaptiveAllocatorEvalArm;
use super::result::ContextMemoryAdaptiveAllocatorEvalShadowResult;
use super::result::ContextMemoryAdaptiveAllocatorEvalShadowVerdict;
use serde::Deserialize;
use serde::Serialize;

/// Payload-light aggregate verdict for the adaptive-allocator shadow arms.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryAdaptiveAllocatorEvalShadowComparisonVerdict {
    pub verdict: ContextMemoryAdaptiveAllocatorEvalShadowVerdict,
    pub current_result_count: usize,
    pub proposed_result_count: usize,
    pub current_missing_critical_fact_count: usize,
    pub proposed_missing_critical_fact_count: usize,
    pub current_token_cost: usize,
    pub proposed_token_cost: usize,
    pub current_token_saved: usize,
    pub proposed_token_saved: usize,
    pub current_latency_ms: u32,
    pub proposed_latency_ms: u32,
    pub missing_critical_fact_regression_count: usize,
    pub recall_regression_count: usize,
    pub precision_regression_count: usize,
    pub latency_regression_count: usize,
    pub token_cost_regression_count: usize,
    pub token_saved_regression_count: usize,
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

impl ContextMemoryAdaptiveAllocatorEvalShadowComparisonVerdict {
    pub(super) fn from_shadow_results(
        shadow_results: &[ContextMemoryAdaptiveAllocatorEvalShadowResult],
    ) -> Self {
        let current_result_count = shadow_results
            .iter()
            .filter(|result| result.arm == ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic)
            .count();
        let proposed_result_count = shadow_results
            .iter()
            .filter(|result| result.arm == ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive)
            .count();
        let current_missing_critical_fact_count = total_missing_critical_fact_count_for_shadow_arm(
            shadow_results,
            ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic,
        );
        let proposed_missing_critical_fact_count = total_missing_critical_fact_count_for_shadow_arm(
            shadow_results,
            ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive,
        );
        let current_token_cost = total_token_cost_for_shadow_arm(
            shadow_results,
            ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic,
        );
        let proposed_token_cost = total_token_cost_for_shadow_arm(
            shadow_results,
            ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive,
        );
        let current_token_saved = total_token_saved_for_shadow_arm(
            shadow_results,
            ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic,
        );
        let proposed_token_saved = total_token_saved_for_shadow_arm(
            shadow_results,
            ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive,
        );
        let current_latency_ms = total_latency_ms_for_shadow_arm(
            shadow_results,
            ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic,
        );
        let proposed_latency_ms = total_latency_ms_for_shadow_arm(
            shadow_results,
            ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive,
        );
        let mut missing_critical_fact_regression_count = 0;
        let mut recall_regression_count = 0;
        let mut precision_regression_count = 0;
        let mut latency_regression_count = 0;
        let mut token_cost_regression_count = 0;
        let mut token_saved_regression_count = 0;

        for fixture_kind in adaptive_allocator_eval_required_fixture_kinds() {
            let current = shadow_result_from_slice(
                shadow_results,
                ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic,
                fixture_kind,
            );
            let proposed = shadow_result_from_slice(
                shadow_results,
                ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive,
                fixture_kind,
            );
            let (Some(current), Some(proposed)) = (current, proposed) else {
                continue;
            };

            missing_critical_fact_regression_count += usize::from(
                proposed.missing_critical_fact_count > current.missing_critical_fact_count,
            );
            recall_regression_count += usize::from(
                proposed.recall_coverage_basis_points < current.recall_coverage_basis_points,
            );
            precision_regression_count +=
                usize::from(proposed.precision_basis_points < current.precision_basis_points);
            latency_regression_count +=
                usize::from(proposed.observed_latency_ms > current.observed_latency_ms);
            token_cost_regression_count += usize::from(proposed.token_cost > current.token_cost);
            token_saved_regression_count += usize::from(proposed.token_saved < current.token_saved);
        }

        let safety_leak_count = shadow_results
            .iter()
            .map(|result| result.safety_leak_count)
            .sum();
        let answer_quality_regression_count = shadow_results
            .iter()
            .map(|result| result.answer_quality_regression_count)
            .sum();
        let production_write = shadow_results.iter().any(|result| result.production_write);
        let graph_write = shadow_results.iter().any(|result| result.graph_write);
        let runtime_activation = shadow_results
            .iter()
            .any(|result| result.runtime_activation);
        let adaptive_allocator_runtime_activation = shadow_results
            .iter()
            .any(|result| result.adaptive_allocator_runtime_activation);
        let source_aware_runtime_activation = shadow_results
            .iter()
            .any(|result| result.source_aware_runtime_activation);
        let prompt_assembly_change = shadow_results
            .iter()
            .any(|result| result.prompt_assembly_change);
        let operator_activation_allowed = shadow_results
            .iter()
            .any(|result| result.operator_activation_allowed);

        let mut comparison = Self {
            verdict: ContextMemoryAdaptiveAllocatorEvalShadowVerdict::Blocked,
            current_result_count,
            proposed_result_count,
            current_missing_critical_fact_count,
            proposed_missing_critical_fact_count,
            current_token_cost,
            proposed_token_cost,
            current_token_saved,
            proposed_token_saved,
            current_latency_ms,
            proposed_latency_ms,
            missing_critical_fact_regression_count,
            recall_regression_count,
            precision_regression_count,
            latency_regression_count,
            token_cost_regression_count,
            token_saved_regression_count,
            safety_leak_count,
            answer_quality_regression_count,
            production_write,
            graph_write,
            runtime_activation,
            adaptive_allocator_runtime_activation,
            source_aware_runtime_activation,
            prompt_assembly_change,
            operator_activation_allowed,
        };
        if comparison.has_shadow_threshold_pass_shape() {
            comparison.verdict =
                ContextMemoryAdaptiveAllocatorEvalShadowVerdict::ShadowThresholdPass;
        }
        comparison
    }

    pub fn has_shadow_threshold_integrity(&self) -> bool {
        self.verdict == ContextMemoryAdaptiveAllocatorEvalShadowVerdict::ShadowThresholdPass
            && self.has_shadow_threshold_pass_shape()
    }

    fn has_shadow_threshold_pass_shape(&self) -> bool {
        self.current_result_count >= adaptive_allocator_eval_required_fixture_kinds().len()
            && self.proposed_result_count >= adaptive_allocator_eval_required_fixture_kinds().len()
            && self.proposed_missing_critical_fact_count <= self.current_missing_critical_fact_count
            && self.proposed_token_cost <= self.current_token_cost
            && self.proposed_token_saved >= self.current_token_saved
            && self.proposed_latency_ms <= self.current_latency_ms
            && self.missing_critical_fact_regression_count == 0
            && self.recall_regression_count == 0
            && self.precision_regression_count == 0
            && self.latency_regression_count == 0
            && self.token_cost_regression_count == 0
            && self.token_saved_regression_count == 0
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
}

fn shadow_result_from_slice(
    shadow_results: &[ContextMemoryAdaptiveAllocatorEvalShadowResult],
    arm: ContextMemoryAdaptiveAllocatorEvalArm,
    fixture_kind: ContextMemoryEvalFixtureKind,
) -> Option<&ContextMemoryAdaptiveAllocatorEvalShadowResult> {
    shadow_results
        .iter()
        .find(|result| result.arm == arm && result.fixture_kind == fixture_kind)
}

fn total_missing_critical_fact_count_for_shadow_arm(
    shadow_results: &[ContextMemoryAdaptiveAllocatorEvalShadowResult],
    arm: ContextMemoryAdaptiveAllocatorEvalArm,
) -> usize {
    shadow_results
        .iter()
        .filter(|result| result.arm == arm)
        .map(|result| result.missing_critical_fact_count)
        .sum()
}

fn total_token_cost_for_shadow_arm(
    shadow_results: &[ContextMemoryAdaptiveAllocatorEvalShadowResult],
    arm: ContextMemoryAdaptiveAllocatorEvalArm,
) -> usize {
    shadow_results
        .iter()
        .filter(|result| result.arm == arm)
        .map(|result| result.token_cost)
        .sum()
}

fn total_token_saved_for_shadow_arm(
    shadow_results: &[ContextMemoryAdaptiveAllocatorEvalShadowResult],
    arm: ContextMemoryAdaptiveAllocatorEvalArm,
) -> usize {
    shadow_results
        .iter()
        .filter(|result| result.arm == arm)
        .map(|result| result.token_saved)
        .sum()
}

fn total_latency_ms_for_shadow_arm(
    shadow_results: &[ContextMemoryAdaptiveAllocatorEvalShadowResult],
    arm: ContextMemoryAdaptiveAllocatorEvalArm,
) -> u32 {
    shadow_results
        .iter()
        .filter(|result| result.arm == arm)
        .fold(0_u32, |latency, result| {
            latency.saturating_add(result.observed_latency_ms)
        })
}
