use super::super::CONTEXT_MEMORY_SELECTED_RECALL_SUMMARY_CANARY_EVAL_SCHEMA_VERSION;
use super::super::stable_receipt_hash;
use super::super::stable_receipt_hash_is_valid;
use serde::Deserialize;
use serde::Serialize;

pub const SELECTED_RECALL_SUMMARY_CANARY_TOKEN_SAVED_MIN_BASIS_POINTS: u32 = 1_000;
pub const SELECTED_RECALL_SUMMARY_CANARY_LATENCY_DELTA_MAX_MS: u32 = 250;
pub const SELECTED_RECALL_SUMMARY_CANARY_QUALITY_DELTA_MIN_BASIS_POINTS: i32 = 0;

/// Payload-light replay mode for the selected-recall summary canary.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemorySelectedRecallSummaryCanaryEvalMode {
    GoldenReplayShadow,
    #[default]
    Unknown,
}

impl ContextMemorySelectedRecallSummaryCanaryEvalMode {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Fixed metric set for selected-recall summary canary replay.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemorySelectedRecallSummaryCanaryEvalMetric {
    ShadowVsLive,
    TokenSaved,
    LatencyDelta,
    QualityDelta,
    RollbackReadback,
    PromptInputProof,
    ResponseDebugProof,
    RegressionBlocked,
    #[default]
    Unknown,
}

impl ContextMemorySelectedRecallSummaryCanaryEvalMetric {
    pub fn fixed_canary_metrics() -> Vec<Self> {
        vec![
            Self::ShadowVsLive,
            Self::TokenSaved,
            Self::LatencyDelta,
            Self::QualityDelta,
            Self::RollbackReadback,
            Self::PromptInputProof,
            Self::ResponseDebugProof,
            Self::RegressionBlocked,
        ]
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Stable fixture kind for the selected-recall summary canary replay.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind {
    SummaryBaseline,
    SummaryCandidate,
    RollbackReadback,
    RegressionGuard,
    #[default]
    Unknown,
}

impl ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SummaryBaseline => "summary_baseline",
            Self::SummaryCandidate => "summary_candidate",
            Self::RollbackReadback => "rollback_readback",
            Self::RegressionGuard => "regression_guard",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light result for one selected-recall summary canary replay fixture.
///
/// This stores only fixture class, pass/block posture, thresholds, proof
/// coverage, and side-effect flags. It intentionally does not carry prompt text,
/// recalled memory text, query payloads, source ids, replay keys, tool payloads,
/// session ids, trace ids, rollback hashes, or operator identity.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemorySelectedRecallSummaryCanaryEvalFixtureResult {
    pub fixture_kind: ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind,
    pub fixture_id_hash: String,
    pub gate_pass: bool,
    pub positive_fixture: bool,
    pub negative_fixture: bool,
    pub shadow_vs_live_pair: bool,
    pub rollback_readback_fixture: bool,
    pub prompt_input_proof_covered: bool,
    pub response_debug_proof_covered: bool,
    pub token_saved_basis_points: u32,
    pub latency_delta_ms: u32,
    pub quality_delta_basis_points: i32,
    pub regression_fixture: bool,
    pub regression_blocked: bool,
    pub production_route: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
}

impl ContextMemorySelectedRecallSummaryCanaryEvalFixtureResult {
    fn positive(
        fixture_kind: ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind,
        rollback_readback_fixture: bool,
        token_saved_basis_points: u32,
        latency_delta_ms: u32,
        quality_delta_basis_points: i32,
    ) -> Self {
        Self {
            fixture_kind,
            fixture_id_hash: fixture_id_hash(
                fixture_kind,
                "positive",
                token_saved_basis_points,
                latency_delta_ms,
                quality_delta_basis_points,
            ),
            gate_pass: true,
            positive_fixture: true,
            negative_fixture: false,
            shadow_vs_live_pair: true,
            rollback_readback_fixture,
            prompt_input_proof_covered: true,
            response_debug_proof_covered: true,
            token_saved_basis_points,
            latency_delta_ms,
            quality_delta_basis_points,
            regression_fixture: false,
            regression_blocked: false,
            production_route: false,
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            prompt_assembly_change: false,
            operator_activation_allowed: false,
        }
    }

    fn negative_regression(
        fixture_kind: ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind,
    ) -> Self {
        Self {
            fixture_kind,
            fixture_id_hash: fixture_id_hash(fixture_kind, "negative", 0, 0, 0),
            gate_pass: true,
            positive_fixture: false,
            negative_fixture: true,
            shadow_vs_live_pair: false,
            rollback_readback_fixture: false,
            prompt_input_proof_covered: true,
            response_debug_proof_covered: true,
            token_saved_basis_points: 0,
            latency_delta_ms: 0,
            quality_delta_basis_points: 0,
            regression_fixture: true,
            regression_blocked: true,
            production_route: false,
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            prompt_assembly_change: false,
            operator_activation_allowed: false,
        }
    }

    pub fn has_canary_fixture_integrity(
        &self,
        token_saved_min_basis_points: u32,
        latency_delta_max_ms: u32,
        quality_delta_min_basis_points: i32,
    ) -> bool {
        !self.fixture_kind.is_unknown()
            && stable_receipt_hash_is_valid(&self.fixture_id_hash)
            && self.gate_pass
            && self.positive_fixture != self.negative_fixture
            && self.prompt_input_proof_covered
            && self.response_debug_proof_covered
            && !self.production_route
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
            && if self.positive_fixture {
                self.shadow_vs_live_pair
                    && !self.regression_fixture
                    && !self.regression_blocked
                    && self.token_saved_basis_points >= token_saved_min_basis_points
                    && self.latency_delta_ms <= latency_delta_max_ms
                    && self.quality_delta_basis_points >= quality_delta_min_basis_points
            } else {
                !self.shadow_vs_live_pair
                    && !self.rollback_readback_fixture
                    && self.regression_fixture
                    && self.regression_blocked
            }
    }
}

fn fixture_id_hash(
    fixture_kind: ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind,
    fixture_class: &str,
    token_saved_basis_points: u32,
    latency_delta_ms: u32,
    quality_delta_basis_points: i32,
) -> String {
    stable_receipt_hash(&[
        "context_memory_selected_recall_summary_canary_eval",
        fixture_kind.as_str(),
        fixture_class,
        &token_saved_basis_points.to_string(),
        &latency_delta_ms.to_string(),
        &quality_delta_basis_points.to_string(),
    ])
}

/// Offline, behavior-neutral eval replay for the selected-recall summary canary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemorySelectedRecallSummaryCanaryEvalReport {
    pub schema_version: u32,
    pub mode: ContextMemorySelectedRecallSummaryCanaryEvalMode,
    pub metrics: Vec<ContextMemorySelectedRecallSummaryCanaryEvalMetric>,
    pub fixtures: Vec<ContextMemorySelectedRecallSummaryCanaryEvalFixtureResult>,
    pub token_saved_min_basis_points: u32,
    pub latency_delta_max_ms: u32,
    pub quality_delta_min_basis_points: i32,
    pub operator_approval_required: bool,
    pub production_route: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
}

impl Default for ContextMemorySelectedRecallSummaryCanaryEvalReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_SELECTED_RECALL_SUMMARY_CANARY_EVAL_SCHEMA_VERSION,
            mode: ContextMemorySelectedRecallSummaryCanaryEvalMode::GoldenReplayShadow,
            metrics: Vec::new(),
            fixtures: Vec::new(),
            token_saved_min_basis_points:
                SELECTED_RECALL_SUMMARY_CANARY_TOKEN_SAVED_MIN_BASIS_POINTS,
            latency_delta_max_ms: SELECTED_RECALL_SUMMARY_CANARY_LATENCY_DELTA_MAX_MS,
            quality_delta_min_basis_points:
                SELECTED_RECALL_SUMMARY_CANARY_QUALITY_DELTA_MIN_BASIS_POINTS,
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

impl ContextMemorySelectedRecallSummaryCanaryEvalReport {
    pub fn seeded() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_SELECTED_RECALL_SUMMARY_CANARY_EVAL_SCHEMA_VERSION,
            mode: ContextMemorySelectedRecallSummaryCanaryEvalMode::GoldenReplayShadow,
            metrics: ContextMemorySelectedRecallSummaryCanaryEvalMetric::fixed_canary_metrics(),
            fixtures: vec![
                ContextMemorySelectedRecallSummaryCanaryEvalFixtureResult::positive(
                    ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind::SummaryBaseline,
                    false,
                    1_000,
                    125,
                    0,
                ),
                ContextMemorySelectedRecallSummaryCanaryEvalFixtureResult::positive(
                    ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind::SummaryCandidate,
                    false,
                    1_800,
                    150,
                    25,
                ),
                ContextMemorySelectedRecallSummaryCanaryEvalFixtureResult::positive(
                    ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind::RollbackReadback,
                    true,
                    1_250,
                    100,
                    0,
                ),
                ContextMemorySelectedRecallSummaryCanaryEvalFixtureResult::negative_regression(
                    ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind::RegressionGuard,
                ),
            ],
            token_saved_min_basis_points:
                SELECTED_RECALL_SUMMARY_CANARY_TOKEN_SAVED_MIN_BASIS_POINTS,
            latency_delta_max_ms: SELECTED_RECALL_SUMMARY_CANARY_LATENCY_DELTA_MAX_MS,
            quality_delta_min_basis_points:
                SELECTED_RECALL_SUMMARY_CANARY_QUALITY_DELTA_MIN_BASIS_POINTS,
            operator_approval_required: true,
            production_route: false,
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            prompt_assembly_change: false,
            operator_activation_allowed: false,
        }
    }

    pub fn has_canary_eval_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_SELECTED_RECALL_SUMMARY_CANARY_EVAL_SCHEMA_VERSION
            && self.mode == ContextMemorySelectedRecallSummaryCanaryEvalMode::GoldenReplayShadow
            && !self.mode.is_unknown()
            && self.metrics
                == ContextMemorySelectedRecallSummaryCanaryEvalMetric::fixed_canary_metrics()
            && self.metrics.iter().all(|metric| !metric.is_unknown())
            && self.fixture_count() == 4
            && self.fixture_pass_count() == 4
            && self.fixture_blocked_count() == 0
            && self.positive_fixture_count() == 3
            && self.negative_fixture_count() == 1
            && self.shadow_vs_live_pair_count() == 3
            && self.rollback_readback_fixture_count() == 1
            && self.regression_blocked_count() == 1
            && self.prompt_input_proof_covered()
            && self.response_debug_proof_covered()
            && self.passes_thresholds()
            && self.fixtures.iter().all(|fixture| {
                fixture.has_canary_fixture_integrity(
                    self.token_saved_min_basis_points,
                    self.latency_delta_max_ms,
                    self.quality_delta_min_basis_points,
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

    fn passes_thresholds(&self) -> bool {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .all(|fixture| {
                fixture.token_saved_basis_points >= self.token_saved_min_basis_points
                    && fixture.latency_delta_ms <= self.latency_delta_max_ms
                    && fixture.quality_delta_basis_points >= self.quality_delta_min_basis_points
            })
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

    pub fn fixture_blocked_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| !fixture.gate_pass)
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

    pub fn shadow_vs_live_pair_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.shadow_vs_live_pair)
            .count()
    }

    pub fn rollback_readback_fixture_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.rollback_readback_fixture)
            .count()
    }

    pub fn regression_blocked_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.regression_fixture && fixture.regression_blocked)
            .count()
    }

    pub fn prompt_input_proof_covered(&self) -> bool {
        self.fixtures
            .iter()
            .any(|fixture| fixture.prompt_input_proof_covered)
    }

    pub fn response_debug_proof_covered(&self) -> bool {
        self.fixtures
            .iter()
            .any(|fixture| fixture.response_debug_proof_covered)
    }

    pub fn fixture(
        &self,
        fixture_kind: ContextMemorySelectedRecallSummaryCanaryEvalFixtureKind,
    ) -> Option<&ContextMemorySelectedRecallSummaryCanaryEvalFixtureResult> {
        self.fixtures
            .iter()
            .find(|fixture| fixture.fixture_kind == fixture_kind)
    }
}
