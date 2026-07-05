use serde::Deserialize;
use serde::Serialize;

use super::super::ContextMemoryAdaptiveAllocatorEvalShadowResult;
use super::super::ContextMemoryEvalFixtureKind;
use super::super::basis_points;
use super::ContextMemoryRecallQualityGateVerdict;

/// Controlled, payload-light blocker reason for one recall-quality fixture row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryRecallQualityGateBlockerReason {
    MissingCriticalFactRegression,
    RecallCoverageRegression,
    PrecisionRegression,
    SafetyLeak,
    AnswerQualityRegression,
    SideEffectFlagEnabled,
}

/// Payload-light per-fixture row for the offline recall-quality gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryRecallQualityFixtureGateReport {
    pub fixture_kind: ContextMemoryEvalFixtureKind,
    pub verdict: ContextMemoryRecallQualityGateVerdict,
    pub scenario_count: usize,
    pub proposed_critical_fact_count: usize,
    pub proposed_recalled_critical_fact_count: usize,
    pub proposed_missing_critical_fact_count: usize,
    pub proposed_predicted_relevant_count: usize,
    pub proposed_false_positive_count: usize,
    pub current_missing_critical_fact_count: usize,
    pub proposed_recall_coverage_basis_points: u32,
    pub current_recall_coverage_basis_points: u32,
    pub proposed_precision_basis_points: u32,
    pub current_precision_basis_points: u32,
    pub recall_coverage_floor_basis_points: u32,
    pub precision_floor_basis_points: u32,
    pub missing_critical_fact_limit: usize,
    pub missing_critical_fact_regression: bool,
    pub recall_regression: bool,
    pub precision_regression: bool,
    pub blocking_reasons: Vec<ContextMemoryRecallQualityGateBlockerReason>,
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

impl Default for ContextMemoryRecallQualityFixtureGateReport {
    fn default() -> Self {
        Self {
            fixture_kind: ContextMemoryEvalFixtureKind::Unknown,
            verdict: ContextMemoryRecallQualityGateVerdict::Unknown,
            scenario_count: 0,
            proposed_critical_fact_count: 0,
            proposed_recalled_critical_fact_count: 0,
            proposed_missing_critical_fact_count: 0,
            proposed_predicted_relevant_count: 0,
            proposed_false_positive_count: 0,
            current_missing_critical_fact_count: 0,
            proposed_recall_coverage_basis_points: 0,
            current_recall_coverage_basis_points: 0,
            proposed_precision_basis_points: 0,
            current_precision_basis_points: 0,
            recall_coverage_floor_basis_points: 7_000,
            precision_floor_basis_points: 7_000,
            missing_critical_fact_limit: 2,
            missing_critical_fact_regression: false,
            recall_regression: false,
            precision_regression: false,
            blocking_reasons: Vec::new(),
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

impl ContextMemoryRecallQualityFixtureGateReport {
    pub(super) fn from_shadow_pair(
        current: &ContextMemoryAdaptiveAllocatorEvalShadowResult,
        proposed: &ContextMemoryAdaptiveAllocatorEvalShadowResult,
    ) -> Self {
        let missing_critical_fact_regression =
            proposed.missing_critical_fact_count > current.missing_critical_fact_count;
        let recall_regression =
            proposed.recall_coverage_basis_points < current.recall_coverage_basis_points;
        let precision_regression = proposed.precision_basis_points < current.precision_basis_points;
        let production_write = current.production_write || proposed.production_write;
        let graph_write = current.graph_write || proposed.graph_write;
        let runtime_activation = current.runtime_activation || proposed.runtime_activation;
        let adaptive_allocator_runtime_activation = current.adaptive_allocator_runtime_activation
            || proposed.adaptive_allocator_runtime_activation;
        let source_aware_runtime_activation =
            current.source_aware_runtime_activation || proposed.source_aware_runtime_activation;
        let prompt_assembly_change =
            current.prompt_assembly_change || proposed.prompt_assembly_change;
        let operator_activation_allowed =
            current.operator_activation_allowed || proposed.operator_activation_allowed;
        let mut report = Self {
            fixture_kind: proposed.fixture_kind,
            verdict: ContextMemoryRecallQualityGateVerdict::Blocked,
            scenario_count: proposed.scenario_count,
            proposed_critical_fact_count: proposed.critical_fact_count,
            proposed_recalled_critical_fact_count: proposed.recalled_critical_fact_count,
            proposed_missing_critical_fact_count: proposed.missing_critical_fact_count,
            proposed_predicted_relevant_count: proposed.predicted_relevant_count,
            proposed_false_positive_count: proposed.false_positive_count,
            current_missing_critical_fact_count: current.missing_critical_fact_count,
            proposed_recall_coverage_basis_points: proposed.recall_coverage_basis_points,
            current_recall_coverage_basis_points: current.recall_coverage_basis_points,
            proposed_precision_basis_points: proposed.precision_basis_points,
            current_precision_basis_points: current.precision_basis_points,
            missing_critical_fact_regression,
            recall_regression,
            precision_regression,
            safety_leak_count: current
                .safety_leak_count
                .saturating_add(proposed.safety_leak_count),
            answer_quality_regression_count: current
                .answer_quality_regression_count
                .saturating_add(proposed.answer_quality_regression_count),
            production_write,
            graph_write,
            runtime_activation,
            adaptive_allocator_runtime_activation,
            source_aware_runtime_activation,
            prompt_assembly_change,
            operator_activation_allowed,
            ..Self::default()
        };
        report.blocking_reasons = report.blocking_reasons_for_fixture();
        if report.has_fixture_gate_pass_shape() {
            report.verdict = ContextMemoryRecallQualityGateVerdict::GatePass;
        }
        report
    }

    pub fn has_fixture_gate_integrity(&self) -> bool {
        self.verdict == ContextMemoryRecallQualityGateVerdict::GatePass
            && self.has_fixture_gate_pass_shape()
    }

    fn has_fixture_gate_pass_shape(&self) -> bool {
        !self.fixture_kind.is_unknown()
            && self.scenario_count > 0
            && self.proposed_critical_fact_count > 0
            && self.proposed_recalled_critical_fact_count <= self.proposed_critical_fact_count
            && self.proposed_missing_critical_fact_count
                == self
                    .proposed_critical_fact_count
                    .saturating_sub(self.proposed_recalled_critical_fact_count)
            && self.proposed_false_positive_count <= self.proposed_predicted_relevant_count
            && self.proposed_predicted_relevant_count >= self.proposed_recalled_critical_fact_count
            && self.proposed_recall_coverage_basis_points
                == basis_points(
                    self.proposed_recalled_critical_fact_count,
                    self.proposed_critical_fact_count,
                )
            && self.proposed_precision_basis_points
                == basis_points(
                    self.proposed_predicted_relevant_count
                        .saturating_sub(self.proposed_false_positive_count),
                    self.proposed_predicted_relevant_count,
                )
            && self.proposed_missing_critical_fact_count <= self.current_missing_critical_fact_count
            && self.proposed_recall_coverage_basis_points
                >= self.current_recall_coverage_basis_points
            && self.proposed_precision_basis_points >= self.current_precision_basis_points
            && self.proposed_recall_coverage_basis_points >= self.recall_coverage_floor_basis_points
            && self.proposed_precision_basis_points >= self.precision_floor_basis_points
            && self.proposed_missing_critical_fact_count <= self.missing_critical_fact_limit
            && !self.missing_critical_fact_regression
            && !self.recall_regression
            && !self.precision_regression
            && self.blocking_reasons.is_empty()
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

    fn blocking_reasons_for_fixture(&self) -> Vec<ContextMemoryRecallQualityGateBlockerReason> {
        let mut reasons = Vec::new();
        if self.missing_critical_fact_regression {
            reasons
                .push(ContextMemoryRecallQualityGateBlockerReason::MissingCriticalFactRegression);
        }
        if self.recall_regression {
            reasons.push(ContextMemoryRecallQualityGateBlockerReason::RecallCoverageRegression);
        }
        if self.precision_regression {
            reasons.push(ContextMemoryRecallQualityGateBlockerReason::PrecisionRegression);
        }
        if self.safety_leak_count > 0 {
            reasons.push(ContextMemoryRecallQualityGateBlockerReason::SafetyLeak);
        }
        if self.answer_quality_regression_count > 0 {
            reasons.push(ContextMemoryRecallQualityGateBlockerReason::AnswerQualityRegression);
        }
        if self.has_side_effect_flag() {
            reasons.push(ContextMemoryRecallQualityGateBlockerReason::SideEffectFlagEnabled);
        }
        reasons
    }

    fn has_side_effect_flag(&self) -> bool {
        self.production_write
            || self.graph_write
            || self.runtime_activation
            || self.adaptive_allocator_runtime_activation
            || self.source_aware_runtime_activation
            || self.prompt_assembly_change
            || self.operator_activation_allowed
    }
}
