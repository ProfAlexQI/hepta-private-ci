use super::super::CONTEXT_MEMORY_RANKED_RECALL_SHADOW_EVAL_SCHEMA_VERSION;
use super::super::basis_points;
use super::super::stable_receipt_hash;
use super::super::stable_receipt_hash_is_valid;
use serde::Deserialize;
use serde::Serialize;

pub const RANKED_RECALL_SHADOW_RECALL_FLOOR_BASIS_POINTS: u32 = 7_000;
pub const RANKED_RECALL_SHADOW_PRECISION_FLOOR_BASIS_POINTS: u32 = 7_000;
pub const RANKED_RECALL_SHADOW_TOKEN_SAVED_MIN: usize = 300;
pub const RANKED_RECALL_SHADOW_TOKEN_SAVED_MIN_BASIS_POINTS: u32 = 1_000;
pub const RANKED_RECALL_SHADOW_LATENCY_MAX_MS: u32 = 100;
pub const RANKED_RECALL_SHADOW_REGRET_MAX_BASIS_POINTS: u32 = 0;

/// Payload-light replay mode for ranked recall evaluation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryRankedRecallShadowEvalMode {
    DeterministicShadow,
    #[default]
    Unknown,
}

impl ContextMemoryRankedRecallShadowEvalMode {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Fixed metric set for ranked recall shadow evaluation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryRankedRecallShadowEvalMetric {
    Recall,
    Precision,
    TokenSaved,
    Latency,
    Regret,
    #[default]
    Unknown,
}

impl ContextMemoryRankedRecallShadowEvalMetric {
    pub fn fixed_shadow_metrics() -> Vec<Self> {
        vec![
            Self::Recall,
            Self::Precision,
            Self::TokenSaved,
            Self::Latency,
            Self::Regret,
        ]
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Stable ranked-recall fixture kind.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryRankedRecallShadowEvalFixtureKind {
    QueryMatch,
    RecencyTieBreak,
    BudgetPressure,
    RegressionGuard,
    #[default]
    Unknown,
}

impl ContextMemoryRankedRecallShadowEvalFixtureKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QueryMatch => "query_match",
            Self::RecencyTieBreak => "recency_tie_break",
            Self::BudgetPressure => "budget_pressure",
            Self::RegressionGuard => "regression_guard",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light result for one ranked recall shadow fixture.
///
/// This stores only fixture class, aggregate ranking counts, thresholds, and
/// side-effect flags. It intentionally does not carry prompt text, transcript
/// text, memory text, query payloads, source ids, session ids, memory ids, raw
/// ranked items, rank explanations, or tool payloads.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryRankedRecallShadowEvalFixtureResult {
    pub fixture_kind: ContextMemoryRankedRecallShadowEvalFixtureKind,
    pub fixture_id_hash: String,
    pub gate_pass: bool,
    pub positive_fixture: bool,
    pub negative_fixture: bool,
    pub shadow_eval_fixture: bool,
    pub ranked_item_count: usize,
    pub expected_relevant_count: usize,
    pub recalled_relevant_count: usize,
    pub predicted_relevant_count: usize,
    pub false_positive_count: usize,
    pub recall_basis_points: u32,
    pub precision_basis_points: u32,
    pub baseline_token_cost: usize,
    pub ranked_token_cost: usize,
    pub token_saved: usize,
    pub token_saved_basis_points: u32,
    pub latency_ms: u32,
    pub latency_budget_ms: u32,
    pub regret_basis_points: u32,
    pub regression_fixture: bool,
    pub regression_blocked: bool,
    pub production_route: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
}

impl ContextMemoryRankedRecallShadowEvalFixtureResult {
    #[allow(clippy::too_many_arguments)]
    fn positive(
        fixture_kind: ContextMemoryRankedRecallShadowEvalFixtureKind,
        ranked_item_count: usize,
        expected_relevant_count: usize,
        recalled_relevant_count: usize,
        predicted_relevant_count: usize,
        false_positive_count: usize,
        baseline_token_cost: usize,
        ranked_token_cost: usize,
        latency_ms: u32,
    ) -> Self {
        Self::fixture(
            fixture_kind,
            "positive",
            true,
            false,
            ranked_item_count,
            expected_relevant_count,
            recalled_relevant_count,
            predicted_relevant_count,
            false_positive_count,
            baseline_token_cost,
            ranked_token_cost,
            latency_ms,
            RANKED_RECALL_SHADOW_LATENCY_MAX_MS,
            0,
            false,
            false,
        )
    }

    fn negative_regression(fixture_kind: ContextMemoryRankedRecallShadowEvalFixtureKind) -> Self {
        Self::fixture(
            fixture_kind,
            "negative",
            false,
            true,
            6,
            4,
            2,
            6,
            4,
            1_200,
            1_280,
            125,
            RANKED_RECALL_SHADOW_LATENCY_MAX_MS,
            500,
            true,
            true,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn fixture(
        fixture_kind: ContextMemoryRankedRecallShadowEvalFixtureKind,
        fixture_class: &str,
        positive_fixture: bool,
        negative_fixture: bool,
        ranked_item_count: usize,
        expected_relevant_count: usize,
        recalled_relevant_count: usize,
        predicted_relevant_count: usize,
        false_positive_count: usize,
        baseline_token_cost: usize,
        ranked_token_cost: usize,
        latency_ms: u32,
        latency_budget_ms: u32,
        regret_basis_points: u32,
        regression_fixture: bool,
        regression_blocked: bool,
    ) -> Self {
        let token_saved = baseline_token_cost.saturating_sub(ranked_token_cost);
        Self {
            fixture_kind,
            fixture_id_hash: fixture_id_hash(
                fixture_kind,
                fixture_class,
                expected_relevant_count,
                recalled_relevant_count,
                predicted_relevant_count,
                token_saved,
                latency_ms,
                regret_basis_points,
            ),
            gate_pass: true,
            positive_fixture,
            negative_fixture,
            shadow_eval_fixture: true,
            ranked_item_count,
            expected_relevant_count,
            recalled_relevant_count,
            predicted_relevant_count,
            false_positive_count,
            recall_basis_points: basis_points(recalled_relevant_count, expected_relevant_count),
            precision_basis_points: basis_points(recalled_relevant_count, predicted_relevant_count),
            baseline_token_cost,
            ranked_token_cost,
            token_saved,
            token_saved_basis_points: basis_points(token_saved, baseline_token_cost),
            latency_ms,
            latency_budget_ms,
            regret_basis_points,
            regression_fixture,
            regression_blocked,
            production_route: false,
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            prompt_assembly_change: false,
            operator_activation_allowed: false,
        }
    }

    pub fn has_ranked_recall_fixture_integrity(
        &self,
        recall_floor_basis_points: u32,
        precision_floor_basis_points: u32,
        token_saved_min: usize,
        token_saved_min_basis_points: u32,
        latency_max_ms: u32,
        regret_max_basis_points: u32,
    ) -> bool {
        !self.fixture_kind.is_unknown()
            && stable_receipt_hash_is_valid(&self.fixture_id_hash)
            && self.gate_pass
            && self.positive_fixture != self.negative_fixture
            && self.shadow_eval_fixture
            && self.ranked_item_count > 0
            && self.expected_relevant_count > 0
            && self.predicted_relevant_count > 0
            && self.recalled_relevant_count <= self.expected_relevant_count
            && self.recalled_relevant_count <= self.predicted_relevant_count
            && self.false_positive_count
                == self
                    .predicted_relevant_count
                    .saturating_sub(self.recalled_relevant_count)
            && self.recall_basis_points
                == basis_points(self.recalled_relevant_count, self.expected_relevant_count)
            && self.precision_basis_points
                == basis_points(self.recalled_relevant_count, self.predicted_relevant_count)
            && self.token_saved
                == self
                    .baseline_token_cost
                    .saturating_sub(self.ranked_token_cost)
            && self.token_saved_basis_points
                == basis_points(self.token_saved, self.baseline_token_cost)
            && self.latency_budget_ms <= latency_max_ms
            && !self.production_route
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
            && if self.positive_fixture {
                !self.regression_fixture
                    && !self.regression_blocked
                    && self.recall_basis_points >= recall_floor_basis_points
                    && self.precision_basis_points >= precision_floor_basis_points
                    && self.token_saved >= token_saved_min
                    && self.token_saved_basis_points >= token_saved_min_basis_points
                    && self.latency_ms <= self.latency_budget_ms
                    && self.latency_ms <= latency_max_ms
                    && self.regret_basis_points <= regret_max_basis_points
            } else {
                self.regression_fixture
                    && self.regression_blocked
                    && (self.recall_basis_points < recall_floor_basis_points
                        || self.precision_basis_points < precision_floor_basis_points
                        || self.token_saved < token_saved_min
                        || self.latency_ms > latency_max_ms
                        || self.regret_basis_points > regret_max_basis_points)
            }
    }
}

#[allow(clippy::too_many_arguments)]
fn fixture_id_hash(
    fixture_kind: ContextMemoryRankedRecallShadowEvalFixtureKind,
    fixture_class: &str,
    expected_relevant_count: usize,
    recalled_relevant_count: usize,
    predicted_relevant_count: usize,
    token_saved: usize,
    latency_ms: u32,
    regret_basis_points: u32,
) -> String {
    stable_receipt_hash(&[
        "context_memory_ranked_recall_shadow_eval",
        fixture_kind.as_str(),
        fixture_class,
        &expected_relevant_count.to_string(),
        &recalled_relevant_count.to_string(),
        &predicted_relevant_count.to_string(),
        &token_saved.to_string(),
        &latency_ms.to_string(),
        &regret_basis_points.to_string(),
    ])
}

/// Offline, behavior-neutral eval replay for ranked recall quality.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryRankedRecallShadowEvalReport {
    pub schema_version: u32,
    pub mode: ContextMemoryRankedRecallShadowEvalMode,
    pub metrics: Vec<ContextMemoryRankedRecallShadowEvalMetric>,
    pub fixtures: Vec<ContextMemoryRankedRecallShadowEvalFixtureResult>,
    pub recall_floor_basis_points: u32,
    pub precision_floor_basis_points: u32,
    pub token_saved_min: usize,
    pub token_saved_min_basis_points: u32,
    pub latency_max_ms: u32,
    pub regret_max_basis_points: u32,
    pub operator_approval_required: bool,
    pub production_route: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
}

impl Default for ContextMemoryRankedRecallShadowEvalReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_RANKED_RECALL_SHADOW_EVAL_SCHEMA_VERSION,
            mode: ContextMemoryRankedRecallShadowEvalMode::DeterministicShadow,
            metrics: Vec::new(),
            fixtures: Vec::new(),
            recall_floor_basis_points: RANKED_RECALL_SHADOW_RECALL_FLOOR_BASIS_POINTS,
            precision_floor_basis_points: RANKED_RECALL_SHADOW_PRECISION_FLOOR_BASIS_POINTS,
            token_saved_min: RANKED_RECALL_SHADOW_TOKEN_SAVED_MIN,
            token_saved_min_basis_points: RANKED_RECALL_SHADOW_TOKEN_SAVED_MIN_BASIS_POINTS,
            latency_max_ms: RANKED_RECALL_SHADOW_LATENCY_MAX_MS,
            regret_max_basis_points: RANKED_RECALL_SHADOW_REGRET_MAX_BASIS_POINTS,
            operator_approval_required: true,
            production_route: false,
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            prompt_assembly_change: false,
            operator_activation_allowed: false,
        }
    }
}

impl ContextMemoryRankedRecallShadowEvalReport {
    pub fn seeded() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_RANKED_RECALL_SHADOW_EVAL_SCHEMA_VERSION,
            mode: ContextMemoryRankedRecallShadowEvalMode::DeterministicShadow,
            metrics: ContextMemoryRankedRecallShadowEvalMetric::fixed_shadow_metrics(),
            fixtures: vec![
                ContextMemoryRankedRecallShadowEvalFixtureResult::positive(
                    ContextMemoryRankedRecallShadowEvalFixtureKind::QueryMatch,
                    5,
                    4,
                    4,
                    5,
                    1,
                    2_000,
                    1_300,
                    42,
                ),
                ContextMemoryRankedRecallShadowEvalFixtureResult::positive(
                    ContextMemoryRankedRecallShadowEvalFixtureKind::RecencyTieBreak,
                    4,
                    3,
                    3,
                    3,
                    0,
                    1_600,
                    1_120,
                    36,
                ),
                ContextMemoryRankedRecallShadowEvalFixtureResult::positive(
                    ContextMemoryRankedRecallShadowEvalFixtureKind::BudgetPressure,
                    6,
                    5,
                    4,
                    5,
                    1,
                    2_400,
                    1_440,
                    55,
                ),
                ContextMemoryRankedRecallShadowEvalFixtureResult::negative_regression(
                    ContextMemoryRankedRecallShadowEvalFixtureKind::RegressionGuard,
                ),
            ],
            recall_floor_basis_points: RANKED_RECALL_SHADOW_RECALL_FLOOR_BASIS_POINTS,
            precision_floor_basis_points: RANKED_RECALL_SHADOW_PRECISION_FLOOR_BASIS_POINTS,
            token_saved_min: RANKED_RECALL_SHADOW_TOKEN_SAVED_MIN,
            token_saved_min_basis_points: RANKED_RECALL_SHADOW_TOKEN_SAVED_MIN_BASIS_POINTS,
            latency_max_ms: RANKED_RECALL_SHADOW_LATENCY_MAX_MS,
            regret_max_basis_points: RANKED_RECALL_SHADOW_REGRET_MAX_BASIS_POINTS,
            operator_approval_required: true,
            production_route: false,
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            prompt_assembly_change: false,
            operator_activation_allowed: false,
        }
    }

    pub fn has_ranked_recall_shadow_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_RANKED_RECALL_SHADOW_EVAL_SCHEMA_VERSION
            && self.mode == ContextMemoryRankedRecallShadowEvalMode::DeterministicShadow
            && !self.mode.is_unknown()
            && self.metrics == ContextMemoryRankedRecallShadowEvalMetric::fixed_shadow_metrics()
            && self.metrics.iter().all(|metric| !metric.is_unknown())
            && self.fixture_count() == 4
            && self.fixture_pass_count() == 4
            && self.positive_fixture_count() == 3
            && self.negative_fixture_count() == 1
            && self.ranked_item_fixture_count() == 4
            && self.regression_blocked_count() == 1
            && self.min_positive_recall_basis_points() >= self.recall_floor_basis_points
            && self.min_positive_precision_basis_points() >= self.precision_floor_basis_points
            && self.total_positive_token_saved() >= self.token_saved_min * 3
            && self.max_positive_latency_ms() <= self.latency_max_ms
            && self.max_positive_regret_basis_points() <= self.regret_max_basis_points
            && self.fixtures.iter().all(|fixture| {
                fixture.has_ranked_recall_fixture_integrity(
                    self.recall_floor_basis_points,
                    self.precision_floor_basis_points,
                    self.token_saved_min,
                    self.token_saved_min_basis_points,
                    self.latency_max_ms,
                    self.regret_max_basis_points,
                )
            })
            && self.operator_approval_required
            && !self.production_route
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
    }

    pub fn fixture_count(&self) -> usize {
        self.fixtures.len()
    }

    pub fn fixture_pass_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.gate_pass)
            .count()
    }

    pub fn positive_fixture_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .count()
    }

    pub fn negative_fixture_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.negative_fixture)
            .count()
    }

    pub fn ranked_item_fixture_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.ranked_item_count > 0)
            .count()
    }

    pub fn regression_blocked_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.regression_fixture && fixture.regression_blocked)
            .count()
    }

    pub fn total_positive_token_saved(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.token_saved)
            .sum()
    }

    pub fn max_positive_latency_ms(&self) -> u32 {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.latency_ms)
            .max()
            .unwrap_or(0)
    }

    pub fn max_positive_regret_basis_points(&self) -> u32 {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.regret_basis_points)
            .max()
            .unwrap_or(0)
    }

    pub fn min_positive_recall_basis_points(&self) -> u32 {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.recall_basis_points)
            .min()
            .unwrap_or(0)
    }

    pub fn min_positive_precision_basis_points(&self) -> u32 {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.precision_basis_points)
            .min()
            .unwrap_or(0)
    }

    pub fn fixture(
        &self,
        fixture_kind: ContextMemoryRankedRecallShadowEvalFixtureKind,
    ) -> Option<&ContextMemoryRankedRecallShadowEvalFixtureResult> {
        self.fixtures
            .iter()
            .find(|fixture| fixture.fixture_kind == fixture_kind)
    }
}
