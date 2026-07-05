use super::super::super::basis_points;
use super::super::super::stable_receipt_hash;
use super::super::super::stable_receipt_hash_is_valid;
use super::super::ContextMemoryEvalFixtureKind;
use super::super::ContextMemoryEvalFixtureResult;
use serde::Deserialize;
use serde::Serialize;

/// Offline shadow arm used to compare the existing budget heuristic against the
/// proposed adaptive allocator without changing runtime behavior.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryAdaptiveAllocatorEvalArm {
    CurrentHeuristic,
    ProposedAdaptive,
    #[default]
    Unknown,
}

impl ContextMemoryAdaptiveAllocatorEvalArm {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CurrentHeuristic => "current_heuristic",
            Self::ProposedAdaptive => "proposed_adaptive",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light verdict for the offline adaptive-allocator shadow comparison.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryAdaptiveAllocatorEvalShadowVerdict {
    ShadowThresholdPass,
    Blocked,
    #[default]
    Unknown,
}

impl ContextMemoryAdaptiveAllocatorEvalShadowVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ShadowThresholdPass => "shadow_threshold_pass",
            Self::Blocked => "blocked",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light eval result for one allocator shadow arm and fixture.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryAdaptiveAllocatorEvalShadowResult {
    pub arm: ContextMemoryAdaptiveAllocatorEvalArm,
    pub fixture_kind: ContextMemoryEvalFixtureKind,
    pub fixture_id_hash: String,
    pub scenario_count: usize,
    pub critical_fact_count: usize,
    pub recalled_critical_fact_count: usize,
    pub missing_critical_fact_count: usize,
    pub predicted_relevant_count: usize,
    pub false_positive_count: usize,
    pub recall_coverage_basis_points: u32,
    pub precision_basis_points: u32,
    pub observed_latency_ms: u32,
    pub latency_budget_ms: u32,
    pub token_cost: usize,
    pub token_saved: usize,
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

impl ContextMemoryAdaptiveAllocatorEvalShadowResult {
    pub(super) fn from_seed_fixture(
        arm: ContextMemoryAdaptiveAllocatorEvalArm,
        fixture: &ContextMemoryEvalFixtureResult,
    ) -> Self {
        let (token_cost, token_saved, observed_latency_ms) = match arm {
            ContextMemoryAdaptiveAllocatorEvalArm::CurrentHeuristic
            | ContextMemoryAdaptiveAllocatorEvalArm::Unknown => (
                fixture.token_cost,
                fixture.token_saved,
                fixture.observed_latency_ms,
            ),
            ContextMemoryAdaptiveAllocatorEvalArm::ProposedAdaptive => {
                let token_cost = (fixture.token_cost.saturating_mul(4) / 5).max(1);
                let token_reduction = fixture.token_cost.saturating_sub(token_cost);
                let latency_reduction_ms = (fixture.observed_latency_ms / 10).max(1);
                (
                    token_cost,
                    fixture.token_saved.saturating_add(token_reduction),
                    fixture
                        .observed_latency_ms
                        .saturating_sub(latency_reduction_ms)
                        .max(1),
                )
            }
        };

        Self {
            arm,
            fixture_kind: fixture.fixture_kind,
            fixture_id_hash: stable_receipt_hash(&[
                "context_memory_adaptive_allocator_eval_shadow",
                arm.as_str(),
                fixture.fixture_kind.as_str(),
                &fixture.fixture_id_hash,
            ]),
            scenario_count: fixture.scenario_count,
            critical_fact_count: fixture.critical_fact_count,
            recalled_critical_fact_count: fixture.recalled_critical_fact_count,
            missing_critical_fact_count: fixture.missing_critical_fact_count,
            predicted_relevant_count: fixture.predicted_relevant_count,
            false_positive_count: fixture.false_positive_count,
            recall_coverage_basis_points: fixture.recall_coverage_basis_points,
            precision_basis_points: fixture.precision_basis_points,
            observed_latency_ms,
            latency_budget_ms: fixture.latency_budget_ms,
            token_cost,
            token_saved,
            safety_leak_count: fixture.safety_leak_count,
            answer_quality_regression_count: fixture.answer_quality_regression_count,
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
        !self.arm.is_unknown()
            && !self.fixture_kind.is_unknown()
            && stable_receipt_hash_is_valid(&self.fixture_id_hash)
            && self.scenario_count > 0
            && self.critical_fact_count > 0
            && self.recalled_critical_fact_count <= self.critical_fact_count
            && self.missing_critical_fact_count
                == self
                    .critical_fact_count
                    .saturating_sub(self.recalled_critical_fact_count)
            && self.false_positive_count <= self.predicted_relevant_count
            && self.predicted_relevant_count >= self.recalled_critical_fact_count
            && self.recall_coverage_basis_points
                == basis_points(self.recalled_critical_fact_count, self.critical_fact_count)
            && self.precision_basis_points
                == basis_points(
                    self.predicted_relevant_count
                        .saturating_sub(self.false_positive_count),
                    self.predicted_relevant_count,
                )
            && self.observed_latency_ms > 0
            && self.observed_latency_ms <= self.latency_budget_ms
            && self.latency_budget_ms > 0
            && self.token_cost > 0
            && self.token_saved <= self.token_cost
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
