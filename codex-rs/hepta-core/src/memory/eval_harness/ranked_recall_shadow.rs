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
pub const RANKED_RECALL_SHADOW_HYBRID_SIGNAL_MIN_BASIS_POINTS: u32 = 6_000;
pub const RANKED_RECALL_SHADOW_HYBRID_SIGNAL_COUNT: usize = 5;
pub const RANKED_RECALL_SHADOW_RERANKING_DELTA_MIN_BASIS_POINTS: i32 = 400;
pub const RANKED_RECALL_SHADOW_LATENCY_DELTA_MAX_MS: i32 = 20;
pub const RANKED_RECALL_SHADOW_TOKEN_TRADEOFF_MIN_BASIS_POINTS: u32 = 1_000;
pub const RANKED_RECALL_SHADOW_ROUTING_DIFF_DELTA_MIN_BASIS_POINTS: i32 =
    RANKED_RECALL_SHADOW_RERANKING_DELTA_MIN_BASIS_POINTS;
pub const RANKED_RECALL_SHADOW_ROUTING_DIFF_LATENCY_DELTA_MAX_MS: i32 =
    RANKED_RECALL_SHADOW_LATENCY_DELTA_MAX_MS;
pub const RANKED_RECALL_SHADOW_ROUTING_DIFF_TOKEN_TRADEOFF_MIN_BASIS_POINTS: u32 =
    RANKED_RECALL_SHADOW_TOKEN_TRADEOFF_MIN_BASIS_POINTS;

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

/// Fixed payload-light hybrid ranking signals for ranked recall shadow eval.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryRankedRecallShadowHybridSignal {
    LexicalBm25,
    Recency,
    SourceAuthority,
    TemporalValidity,
    Feedback,
    #[default]
    Unknown,
}

impl ContextMemoryRankedRecallShadowHybridSignal {
    pub fn fixed_shadow_signals() -> Vec<Self> {
        vec![
            Self::LexicalBm25,
            Self::Recency,
            Self::SourceAuthority,
            Self::TemporalValidity,
            Self::Feedback,
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
    pub calibrated_reranking_fixture: bool,
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
    pub lexical_bm25_score_basis_points: u32,
    pub recency_score_basis_points: u32,
    pub source_authority_score_basis_points: u32,
    pub temporal_validity_score_basis_points: u32,
    pub feedback_score_basis_points: u32,
    pub hybrid_score_basis_points: u32,
    pub hybrid_signal_pass_count: usize,
    pub baseline_rank_window_score_basis_points: u32,
    pub hybrid_rank_window_score_basis_points: u32,
    pub reranking_delta_basis_points: i32,
    pub reranking_win: bool,
    pub reranking_loss: bool,
    pub latency_delta_ms: i32,
    pub token_tradeoff_basis_points: u32,
    pub routing_diff_fixture: bool,
    pub routing_diff_shadow_only: bool,
    pub production_selection_score_basis_points: u32,
    pub hybrid_calibrated_selection_score_basis_points: u32,
    pub routing_diff_delta_basis_points: i32,
    pub routing_diff_win: bool,
    pub routing_diff_loss: bool,
    pub routing_diff_latency_delta_ms: i32,
    pub routing_diff_token_tradeoff_basis_points: u32,
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
        let token_saved_basis_points = basis_points(token_saved, baseline_token_cost);
        let hybrid_scores = hybrid_signal_scores(fixture_kind, positive_fixture);
        let hybrid_score_basis_points = hybrid_score_basis_points(hybrid_scores);
        let hybrid_signal_pass_count = hybrid_signal_pass_count(
            hybrid_scores,
            RANKED_RECALL_SHADOW_HYBRID_SIGNAL_MIN_BASIS_POINTS,
        );
        let (
            baseline_rank_window_score_basis_points,
            hybrid_rank_window_score_basis_points,
            latency_delta_ms,
        ) = calibrated_reranking_delta(fixture_kind, positive_fixture);
        let reranking_delta_basis_points = hybrid_rank_window_score_basis_points as i32
            - baseline_rank_window_score_basis_points as i32;
        let reranking_win = positive_fixture
            && reranking_delta_basis_points
                >= RANKED_RECALL_SHADOW_RERANKING_DELTA_MIN_BASIS_POINTS;
        let reranking_loss = negative_fixture
            && reranking_delta_basis_points < RANKED_RECALL_SHADOW_RERANKING_DELTA_MIN_BASIS_POINTS;
        let production_selection_score_basis_points = baseline_rank_window_score_basis_points;
        let hybrid_calibrated_selection_score_basis_points = hybrid_rank_window_score_basis_points;
        let routing_diff_delta_basis_points = hybrid_calibrated_selection_score_basis_points as i32
            - production_selection_score_basis_points as i32;
        let routing_diff_win = reranking_win;
        let routing_diff_loss = reranking_loss;
        let routing_diff_latency_delta_ms = latency_delta_ms;
        let routing_diff_token_tradeoff_basis_points = token_saved_basis_points;
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
                hybrid_scores,
                hybrid_score_basis_points,
                hybrid_signal_pass_count,
                baseline_rank_window_score_basis_points,
                hybrid_rank_window_score_basis_points,
                reranking_delta_basis_points,
                latency_delta_ms,
                token_saved_basis_points,
                reranking_win,
                reranking_loss,
                production_selection_score_basis_points,
                hybrid_calibrated_selection_score_basis_points,
                routing_diff_delta_basis_points,
                routing_diff_win,
                routing_diff_loss,
                routing_diff_latency_delta_ms,
                routing_diff_token_tradeoff_basis_points,
            ),
            gate_pass: true,
            positive_fixture,
            negative_fixture,
            shadow_eval_fixture: true,
            calibrated_reranking_fixture: true,
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
            token_saved_basis_points,
            latency_ms,
            latency_budget_ms,
            regret_basis_points,
            lexical_bm25_score_basis_points: hybrid_scores.0,
            recency_score_basis_points: hybrid_scores.1,
            source_authority_score_basis_points: hybrid_scores.2,
            temporal_validity_score_basis_points: hybrid_scores.3,
            feedback_score_basis_points: hybrid_scores.4,
            hybrid_score_basis_points,
            hybrid_signal_pass_count,
            baseline_rank_window_score_basis_points,
            hybrid_rank_window_score_basis_points,
            reranking_delta_basis_points,
            reranking_win,
            reranking_loss,
            latency_delta_ms,
            token_tradeoff_basis_points: token_saved_basis_points,
            routing_diff_fixture: true,
            routing_diff_shadow_only: true,
            production_selection_score_basis_points,
            hybrid_calibrated_selection_score_basis_points,
            routing_diff_delta_basis_points,
            routing_diff_win,
            routing_diff_loss,
            routing_diff_latency_delta_ms,
            routing_diff_token_tradeoff_basis_points,
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

    #[allow(clippy::too_many_arguments)]
    pub fn has_ranked_recall_fixture_integrity(
        &self,
        recall_floor_basis_points: u32,
        precision_floor_basis_points: u32,
        token_saved_min: usize,
        token_saved_min_basis_points: u32,
        latency_max_ms: u32,
        regret_max_basis_points: u32,
        hybrid_signal_min_basis_points: u32,
        reranking_delta_min_basis_points: i32,
        latency_delta_max_ms: i32,
        token_tradeoff_min_basis_points: u32,
        routing_diff_delta_min_basis_points: i32,
        routing_diff_latency_delta_max_ms: i32,
        routing_diff_token_tradeoff_min_basis_points: u32,
    ) -> bool {
        !self.fixture_kind.is_unknown()
            && stable_receipt_hash_is_valid(&self.fixture_id_hash)
            && self.gate_pass
            && self.positive_fixture != self.negative_fixture
            && self.shadow_eval_fixture
            && self.calibrated_reranking_fixture
            && self.routing_diff_fixture
            && self.routing_diff_shadow_only
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
            && self.hybrid_score_basis_points
                == hybrid_score_basis_points((
                    self.lexical_bm25_score_basis_points,
                    self.recency_score_basis_points,
                    self.source_authority_score_basis_points,
                    self.temporal_validity_score_basis_points,
                    self.feedback_score_basis_points,
                ))
            && self.hybrid_signal_pass_count
                == hybrid_signal_pass_count(
                    (
                        self.lexical_bm25_score_basis_points,
                        self.recency_score_basis_points,
                        self.source_authority_score_basis_points,
                        self.temporal_validity_score_basis_points,
                        self.feedback_score_basis_points,
                    ),
                    hybrid_signal_min_basis_points,
                )
            && self.reranking_delta_basis_points
                == self.hybrid_rank_window_score_basis_points as i32
                    - self.baseline_rank_window_score_basis_points as i32
            && self.token_tradeoff_basis_points == self.token_saved_basis_points
            && self.production_selection_score_basis_points
                == self.baseline_rank_window_score_basis_points
            && self.hybrid_calibrated_selection_score_basis_points
                == self.hybrid_rank_window_score_basis_points
            && self.routing_diff_delta_basis_points
                == self.hybrid_calibrated_selection_score_basis_points as i32
                    - self.production_selection_score_basis_points as i32
            && self.routing_diff_delta_basis_points == self.reranking_delta_basis_points
            && self.routing_diff_win == self.reranking_win
            && self.routing_diff_loss == self.reranking_loss
            && self.routing_diff_latency_delta_ms == self.latency_delta_ms
            && self.routing_diff_token_tradeoff_basis_points == self.token_tradeoff_basis_points
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
                    && self.lexical_bm25_score_basis_points >= hybrid_signal_min_basis_points
                    && self.recency_score_basis_points >= hybrid_signal_min_basis_points
                    && self.source_authority_score_basis_points >= hybrid_signal_min_basis_points
                    && self.temporal_validity_score_basis_points >= hybrid_signal_min_basis_points
                    && self.feedback_score_basis_points >= hybrid_signal_min_basis_points
                    && self.hybrid_score_basis_points >= hybrid_signal_min_basis_points
                    && self.hybrid_signal_pass_count == RANKED_RECALL_SHADOW_HYBRID_SIGNAL_COUNT
                    && self.reranking_win
                    && !self.reranking_loss
                    && self.reranking_delta_basis_points >= reranking_delta_min_basis_points
                    && self.latency_delta_ms <= latency_delta_max_ms
                    && self.token_tradeoff_basis_points >= token_tradeoff_min_basis_points
                    && self.routing_diff_win
                    && !self.routing_diff_loss
                    && self.routing_diff_delta_basis_points >= routing_diff_delta_min_basis_points
                    && self.routing_diff_latency_delta_ms <= routing_diff_latency_delta_max_ms
                    && self.routing_diff_token_tradeoff_basis_points
                        >= routing_diff_token_tradeoff_min_basis_points
            } else {
                self.regression_fixture
                    && self.regression_blocked
                    && !self.reranking_win
                    && self.reranking_loss
                    && !self.routing_diff_win
                    && self.routing_diff_loss
                    && (self.recall_basis_points < recall_floor_basis_points
                        || self.precision_basis_points < precision_floor_basis_points
                        || self.token_saved < token_saved_min
                        || self.latency_ms > latency_max_ms
                        || self.regret_basis_points > regret_max_basis_points
                        || self.hybrid_signal_pass_count < RANKED_RECALL_SHADOW_HYBRID_SIGNAL_COUNT
                        || self.reranking_delta_basis_points < reranking_delta_min_basis_points
                        || self.latency_delta_ms > latency_delta_max_ms
                        || self.token_tradeoff_basis_points < token_tradeoff_min_basis_points
                        || self.routing_diff_delta_basis_points
                            < routing_diff_delta_min_basis_points
                        || self.routing_diff_latency_delta_ms > routing_diff_latency_delta_max_ms
                        || self.routing_diff_token_tradeoff_basis_points
                            < routing_diff_token_tradeoff_min_basis_points)
            }
    }
}

fn hybrid_signal_scores(
    fixture_kind: ContextMemoryRankedRecallShadowEvalFixtureKind,
    positive_fixture: bool,
) -> (u32, u32, u32, u32, u32) {
    if !positive_fixture {
        return (4_200, 5_000, 4_800, 4_000, 3_500);
    }

    match fixture_kind {
        ContextMemoryRankedRecallShadowEvalFixtureKind::QueryMatch => {
            (9_200, 7_600, 8_100, 7_800, 7_000)
        }
        ContextMemoryRankedRecallShadowEvalFixtureKind::RecencyTieBreak => {
            (7_200, 9_400, 7_600, 7_800, 7_000)
        }
        ContextMemoryRankedRecallShadowEvalFixtureKind::BudgetPressure => {
            (8_600, 7_300, 9_000, 7_600, 7_200)
        }
        ContextMemoryRankedRecallShadowEvalFixtureKind::RegressionGuard
        | ContextMemoryRankedRecallShadowEvalFixtureKind::Unknown => {
            (4_200, 5_000, 4_800, 4_000, 3_500)
        }
    }
}

fn hybrid_score_basis_points(scores: (u32, u32, u32, u32, u32)) -> u32 {
    (scores.0 + scores.1 + scores.2 + scores.3 + scores.4)
        / RANKED_RECALL_SHADOW_HYBRID_SIGNAL_COUNT as u32
}

fn hybrid_signal_pass_count(scores: (u32, u32, u32, u32, u32), min_basis_points: u32) -> usize {
    [scores.0, scores.1, scores.2, scores.3, scores.4]
        .into_iter()
        .filter(|score| *score >= min_basis_points)
        .count()
}

fn calibrated_reranking_delta(
    fixture_kind: ContextMemoryRankedRecallShadowEvalFixtureKind,
    positive_fixture: bool,
) -> (u32, u32, i32) {
    if !positive_fixture {
        return (6_500, 4_300, 35);
    }

    match fixture_kind {
        ContextMemoryRankedRecallShadowEvalFixtureKind::QueryMatch => (7_400, 8_140, 8),
        ContextMemoryRankedRecallShadowEvalFixtureKind::RecencyTieBreak => (7_100, 7_960, 6),
        ContextMemoryRankedRecallShadowEvalFixtureKind::BudgetPressure => (7_300, 7_940, 10),
        ContextMemoryRankedRecallShadowEvalFixtureKind::RegressionGuard
        | ContextMemoryRankedRecallShadowEvalFixtureKind::Unknown => (6_500, 4_300, 35),
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
    hybrid_scores: (u32, u32, u32, u32, u32),
    hybrid_score_basis_points: u32,
    hybrid_signal_pass_count: usize,
    baseline_rank_window_score_basis_points: u32,
    hybrid_rank_window_score_basis_points: u32,
    reranking_delta_basis_points: i32,
    latency_delta_ms: i32,
    token_tradeoff_basis_points: u32,
    reranking_win: bool,
    reranking_loss: bool,
    production_selection_score_basis_points: u32,
    hybrid_calibrated_selection_score_basis_points: u32,
    routing_diff_delta_basis_points: i32,
    routing_diff_win: bool,
    routing_diff_loss: bool,
    routing_diff_latency_delta_ms: i32,
    routing_diff_token_tradeoff_basis_points: u32,
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
        &hybrid_scores.0.to_string(),
        &hybrid_scores.1.to_string(),
        &hybrid_scores.2.to_string(),
        &hybrid_scores.3.to_string(),
        &hybrid_scores.4.to_string(),
        &hybrid_score_basis_points.to_string(),
        &hybrid_signal_pass_count.to_string(),
        &baseline_rank_window_score_basis_points.to_string(),
        &hybrid_rank_window_score_basis_points.to_string(),
        &reranking_delta_basis_points.to_string(),
        &latency_delta_ms.to_string(),
        &token_tradeoff_basis_points.to_string(),
        &reranking_win.to_string(),
        &reranking_loss.to_string(),
        &production_selection_score_basis_points.to_string(),
        &hybrid_calibrated_selection_score_basis_points.to_string(),
        &routing_diff_delta_basis_points.to_string(),
        &routing_diff_win.to_string(),
        &routing_diff_loss.to_string(),
        &routing_diff_latency_delta_ms.to_string(),
        &routing_diff_token_tradeoff_basis_points.to_string(),
    ])
}

/// Offline, behavior-neutral eval replay for ranked recall quality.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryRankedRecallShadowEvalReport {
    pub schema_version: u32,
    pub mode: ContextMemoryRankedRecallShadowEvalMode,
    pub metrics: Vec<ContextMemoryRankedRecallShadowEvalMetric>,
    pub hybrid_signals: Vec<ContextMemoryRankedRecallShadowHybridSignal>,
    pub fixtures: Vec<ContextMemoryRankedRecallShadowEvalFixtureResult>,
    pub recall_floor_basis_points: u32,
    pub precision_floor_basis_points: u32,
    pub token_saved_min: usize,
    pub token_saved_min_basis_points: u32,
    pub latency_max_ms: u32,
    pub regret_max_basis_points: u32,
    pub hybrid_signal_min_basis_points: u32,
    pub reranking_delta_min_basis_points: i32,
    pub latency_delta_max_ms: i32,
    pub token_tradeoff_min_basis_points: u32,
    pub routing_diff_delta_min_basis_points: i32,
    pub routing_diff_latency_delta_max_ms: i32,
    pub routing_diff_token_tradeoff_min_basis_points: u32,
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
            hybrid_signals: Vec::new(),
            fixtures: Vec::new(),
            recall_floor_basis_points: RANKED_RECALL_SHADOW_RECALL_FLOOR_BASIS_POINTS,
            precision_floor_basis_points: RANKED_RECALL_SHADOW_PRECISION_FLOOR_BASIS_POINTS,
            token_saved_min: RANKED_RECALL_SHADOW_TOKEN_SAVED_MIN,
            token_saved_min_basis_points: RANKED_RECALL_SHADOW_TOKEN_SAVED_MIN_BASIS_POINTS,
            latency_max_ms: RANKED_RECALL_SHADOW_LATENCY_MAX_MS,
            regret_max_basis_points: RANKED_RECALL_SHADOW_REGRET_MAX_BASIS_POINTS,
            hybrid_signal_min_basis_points: RANKED_RECALL_SHADOW_HYBRID_SIGNAL_MIN_BASIS_POINTS,
            reranking_delta_min_basis_points: RANKED_RECALL_SHADOW_RERANKING_DELTA_MIN_BASIS_POINTS,
            latency_delta_max_ms: RANKED_RECALL_SHADOW_LATENCY_DELTA_MAX_MS,
            token_tradeoff_min_basis_points: RANKED_RECALL_SHADOW_TOKEN_TRADEOFF_MIN_BASIS_POINTS,
            routing_diff_delta_min_basis_points:
                RANKED_RECALL_SHADOW_ROUTING_DIFF_DELTA_MIN_BASIS_POINTS,
            routing_diff_latency_delta_max_ms:
                RANKED_RECALL_SHADOW_ROUTING_DIFF_LATENCY_DELTA_MAX_MS,
            routing_diff_token_tradeoff_min_basis_points:
                RANKED_RECALL_SHADOW_ROUTING_DIFF_TOKEN_TRADEOFF_MIN_BASIS_POINTS,
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
            hybrid_signals: ContextMemoryRankedRecallShadowHybridSignal::fixed_shadow_signals(),
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
            hybrid_signal_min_basis_points: RANKED_RECALL_SHADOW_HYBRID_SIGNAL_MIN_BASIS_POINTS,
            reranking_delta_min_basis_points: RANKED_RECALL_SHADOW_RERANKING_DELTA_MIN_BASIS_POINTS,
            latency_delta_max_ms: RANKED_RECALL_SHADOW_LATENCY_DELTA_MAX_MS,
            token_tradeoff_min_basis_points: RANKED_RECALL_SHADOW_TOKEN_TRADEOFF_MIN_BASIS_POINTS,
            routing_diff_delta_min_basis_points:
                RANKED_RECALL_SHADOW_ROUTING_DIFF_DELTA_MIN_BASIS_POINTS,
            routing_diff_latency_delta_max_ms:
                RANKED_RECALL_SHADOW_ROUTING_DIFF_LATENCY_DELTA_MAX_MS,
            routing_diff_token_tradeoff_min_basis_points:
                RANKED_RECALL_SHADOW_ROUTING_DIFF_TOKEN_TRADEOFF_MIN_BASIS_POINTS,
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
            && self.hybrid_signals
                == ContextMemoryRankedRecallShadowHybridSignal::fixed_shadow_signals()
            && self
                .hybrid_signals
                .iter()
                .all(|signal| !signal.is_unknown())
            && self.hybrid_signal_count() == RANKED_RECALL_SHADOW_HYBRID_SIGNAL_COUNT
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
            && self.positive_hybrid_signal_pass_count()
                == self.positive_fixture_count() * RANKED_RECALL_SHADOW_HYBRID_SIGNAL_COUNT
            && self.hybrid_regression_blocked_count() == 1
            && self.min_positive_hybrid_score_basis_points() >= self.hybrid_signal_min_basis_points
            && self.calibrated_reranking_fixture_count() == self.fixture_count()
            && self.calibrated_reranking_win_count() == self.positive_fixture_count()
            && self.calibrated_reranking_loss_count() == self.negative_fixture_count()
            && self.min_positive_reranking_delta_basis_points()
                >= self.reranking_delta_min_basis_points
            && self.max_positive_latency_delta_ms() <= self.latency_delta_max_ms
            && self.min_positive_token_tradeoff_basis_points()
                >= self.token_tradeoff_min_basis_points
            && self.reranking_regression_blocked_count() == 1
            && self.routing_diff_fixture_count() == self.fixture_count()
            && self.routing_diff_shadow_only_count() == self.fixture_count()
            && self.routing_diff_win_count() == self.positive_fixture_count()
            && self.routing_diff_loss_count() == self.negative_fixture_count()
            && self.min_positive_routing_diff_delta_basis_points()
                >= self.routing_diff_delta_min_basis_points
            && self.max_positive_routing_diff_latency_delta_ms()
                <= self.routing_diff_latency_delta_max_ms
            && self.min_positive_routing_diff_token_tradeoff_basis_points()
                >= self.routing_diff_token_tradeoff_min_basis_points
            && self.routing_diff_regression_blocked_count() == 1
            && self.fixtures.iter().all(|fixture| {
                fixture.has_ranked_recall_fixture_integrity(
                    self.recall_floor_basis_points,
                    self.precision_floor_basis_points,
                    self.token_saved_min,
                    self.token_saved_min_basis_points,
                    self.latency_max_ms,
                    self.regret_max_basis_points,
                    self.hybrid_signal_min_basis_points,
                    self.reranking_delta_min_basis_points,
                    self.latency_delta_max_ms,
                    self.token_tradeoff_min_basis_points,
                    self.routing_diff_delta_min_basis_points,
                    self.routing_diff_latency_delta_max_ms,
                    self.routing_diff_token_tradeoff_min_basis_points,
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

    pub fn hybrid_signal_count(&self) -> usize {
        self.hybrid_signals.len()
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

    pub fn positive_hybrid_signal_pass_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.hybrid_signal_pass_count)
            .sum()
    }

    pub fn hybrid_regression_blocked_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| {
                fixture.regression_fixture
                    && fixture.regression_blocked
                    && fixture.hybrid_signal_pass_count < RANKED_RECALL_SHADOW_HYBRID_SIGNAL_COUNT
            })
            .count()
    }

    pub fn calibrated_reranking_fixture_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.calibrated_reranking_fixture)
            .count()
    }

    pub fn calibrated_reranking_win_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.reranking_win)
            .count()
    }

    pub fn calibrated_reranking_loss_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.reranking_loss)
            .count()
    }

    pub fn reranking_regression_blocked_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| {
                fixture.regression_fixture && fixture.regression_blocked && fixture.reranking_loss
            })
            .count()
    }

    pub fn routing_diff_fixture_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.routing_diff_fixture)
            .count()
    }

    pub fn routing_diff_shadow_only_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.routing_diff_shadow_only)
            .count()
    }

    pub fn routing_diff_win_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.routing_diff_win)
            .count()
    }

    pub fn routing_diff_loss_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.routing_diff_loss)
            .count()
    }

    pub fn routing_diff_regression_blocked_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| {
                fixture.regression_fixture
                    && fixture.regression_blocked
                    && fixture.routing_diff_loss
                    && fixture.routing_diff_shadow_only
            })
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

    pub fn min_positive_hybrid_score_basis_points(&self) -> u32 {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.hybrid_score_basis_points)
            .min()
            .unwrap_or(0)
    }

    pub fn min_positive_reranking_delta_basis_points(&self) -> i32 {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.reranking_delta_basis_points)
            .min()
            .unwrap_or(0)
    }

    pub fn max_positive_latency_delta_ms(&self) -> i32 {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.latency_delta_ms)
            .max()
            .unwrap_or(0)
    }

    pub fn min_positive_token_tradeoff_basis_points(&self) -> u32 {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.token_tradeoff_basis_points)
            .min()
            .unwrap_or(0)
    }

    pub fn min_positive_routing_diff_delta_basis_points(&self) -> i32 {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.routing_diff_delta_basis_points)
            .min()
            .unwrap_or(0)
    }

    pub fn max_positive_routing_diff_latency_delta_ms(&self) -> i32 {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.routing_diff_latency_delta_ms)
            .max()
            .unwrap_or(0)
    }

    pub fn min_positive_routing_diff_token_tradeoff_basis_points(&self) -> u32 {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.routing_diff_token_tradeoff_basis_points)
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
