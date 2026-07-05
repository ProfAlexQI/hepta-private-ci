use super::super::CONTEXT_MEMORY_EVAL_HARNESS_SCHEMA_VERSION;
use super::super::basis_points;
use super::super::stable_receipt_hash;
use super::super::stable_receipt_hash_is_valid;
use serde::Deserialize;
use serde::Serialize;

/// Fixed metric set for the offline context-memory eval harness seed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryEvalMetric {
    RecallCoverage,
    MissingCriticalFact,
    Precision,
    Latency,
    TokenCost,
    TokenSaved,
    SafetyLeak,
    AnswerQualityRegression,
    #[default]
    Unknown,
}

impl ContextMemoryEvalMetric {
    pub fn fixed_seed_metrics() -> Vec<Self> {
        vec![
            Self::RecallCoverage,
            Self::MissingCriticalFact,
            Self::Precision,
            Self::Latency,
            Self::TokenCost,
            Self::TokenSaved,
            Self::SafetyLeak,
            Self::AnswerQualityRegression,
        ]
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Offline fixture kind for eval harness seed reports.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryEvalFixtureKind {
    SyntheticLongSession,
    RedactedTrace,
    #[default]
    Unknown,
}

impl ContextMemoryEvalFixtureKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SyntheticLongSession => "synthetic_long_session",
            Self::RedactedTrace => "redacted_trace",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light result for one offline eval fixture.
///
/// This intentionally stores only counts, ratios, budgets, and a stable fixture
/// hash. It does not store prompt text, transcript text, memory text, trace
/// identifiers, source ids, or answer payloads.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryEvalFixtureResult {
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
    pub synthetic: bool,
    pub redacted: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
}

impl ContextMemoryEvalFixtureResult {
    fn seeded(
        fixture_kind: ContextMemoryEvalFixtureKind,
        scenario_count: usize,
        critical_fact_count: usize,
        recalled_critical_fact_count: usize,
        predicted_relevant_count: usize,
        false_positive_count: usize,
        observed_latency_ms: u32,
        latency_budget_ms: u32,
        token_cost: usize,
        token_saved: usize,
    ) -> Self {
        let missing_critical_fact_count =
            critical_fact_count.saturating_sub(recalled_critical_fact_count);
        let true_positive_count = predicted_relevant_count.saturating_sub(false_positive_count);
        Self {
            fixture_kind,
            fixture_id_hash: stable_receipt_hash(&[
                "context_memory_eval_harness_seed",
                fixture_kind.as_str(),
                &scenario_count.to_string(),
                &critical_fact_count.to_string(),
                &recalled_critical_fact_count.to_string(),
                &predicted_relevant_count.to_string(),
                &false_positive_count.to_string(),
            ]),
            scenario_count,
            critical_fact_count,
            recalled_critical_fact_count,
            missing_critical_fact_count,
            predicted_relevant_count,
            false_positive_count,
            recall_coverage_basis_points: basis_points(
                recalled_critical_fact_count,
                critical_fact_count,
            ),
            precision_basis_points: basis_points(true_positive_count, predicted_relevant_count),
            observed_latency_ms,
            latency_budget_ms,
            token_cost,
            token_saved,
            safety_leak_count: 0,
            answer_quality_regression_count: 0,
            synthetic: fixture_kind == ContextMemoryEvalFixtureKind::SyntheticLongSession,
            redacted: fixture_kind == ContextMemoryEvalFixtureKind::RedactedTrace,
            production_write: false,
            graph_write: false,
            runtime_activation: false,
        }
    }

    pub fn has_eval_integrity(&self) -> bool {
        !self.fixture_kind.is_unknown()
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
            && self.observed_latency_ms <= self.latency_budget_ms
            && self.latency_budget_ms > 0
            && self.token_cost > 0
            && self.token_saved <= self.token_cost
            && self.safety_leak_count == 0
            && self.answer_quality_regression_count == 0
            && match self.fixture_kind {
                ContextMemoryEvalFixtureKind::SyntheticLongSession => {
                    self.synthetic && !self.redacted
                }
                ContextMemoryEvalFixtureKind::RedactedTrace => self.redacted && !self.synthetic,
                ContextMemoryEvalFixtureKind::Unknown => false,
            }
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
    }
}

/// Offline, behavior-neutral eval harness seed for context memory quality.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryEvalHarnessReport {
    pub schema_version: u32,
    pub metrics: Vec<ContextMemoryEvalMetric>,
    pub fixtures: Vec<ContextMemoryEvalFixtureResult>,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub operator_activation_allowed: bool,
}

impl Default for ContextMemoryEvalHarnessReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_EVAL_HARNESS_SCHEMA_VERSION,
            metrics: Vec::new(),
            fixtures: Vec::new(),
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            operator_activation_allowed: false,
        }
    }
}

impl ContextMemoryEvalHarnessReport {
    pub fn seeded() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_EVAL_HARNESS_SCHEMA_VERSION,
            metrics: ContextMemoryEvalMetric::fixed_seed_metrics(),
            fixtures: vec![
                ContextMemoryEvalFixtureResult::seeded(
                    ContextMemoryEvalFixtureKind::SyntheticLongSession,
                    3,
                    5,
                    4,
                    5,
                    1,
                    42,
                    100,
                    1_800,
                    620,
                ),
                ContextMemoryEvalFixtureResult::seeded(
                    ContextMemoryEvalFixtureKind::RedactedTrace,
                    2,
                    4,
                    3,
                    4,
                    1,
                    64,
                    150,
                    1_400,
                    480,
                ),
            ],
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            operator_activation_allowed: false,
        }
    }

    pub fn has_eval_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_EVAL_HARNESS_SCHEMA_VERSION
            && self.metrics == ContextMemoryEvalMetric::fixed_seed_metrics()
            && self.fixtures.len() >= 2
            && self.fixtures.iter().any(|fixture| {
                fixture.fixture_kind == ContextMemoryEvalFixtureKind::SyntheticLongSession
            })
            && self
                .fixtures
                .iter()
                .any(|fixture| fixture.fixture_kind == ContextMemoryEvalFixtureKind::RedactedTrace)
            && self
                .fixtures
                .iter()
                .all(ContextMemoryEvalFixtureResult::has_eval_integrity)
            && self.metrics.iter().all(|metric| !metric.is_unknown())
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.operator_activation_allowed
    }

    pub fn fixture_count(&self) -> usize {
        self.fixtures.len()
    }

    pub fn total_missing_critical_fact_count(&self) -> usize {
        self.fixtures
            .iter()
            .map(|fixture| fixture.missing_critical_fact_count)
            .sum()
    }

    pub fn total_token_saved(&self) -> usize {
        self.fixtures
            .iter()
            .map(|fixture| fixture.token_saved)
            .sum()
    }

    pub fn safety_leak_count(&self) -> usize {
        self.fixtures
            .iter()
            .map(|fixture| fixture.safety_leak_count)
            .sum()
    }

    pub fn answer_quality_regression_count(&self) -> usize {
        self.fixtures
            .iter()
            .map(|fixture| fixture.answer_quality_regression_count)
            .sum()
    }
}
