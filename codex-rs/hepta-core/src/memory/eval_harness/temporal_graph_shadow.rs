use serde::Deserialize;
use serde::Serialize;

use super::super::CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_EVAL_SCHEMA_VERSION;
use super::super::basis_points;
use super::super::stable_receipt_hash;
use super::super::stable_receipt_hash_is_valid;

pub const TEMPORAL_GRAPH_SHADOW_NODE_COVERAGE_FLOOR_BASIS_POINTS: u32 = 10_000;
pub const TEMPORAL_GRAPH_SHADOW_EDGE_COVERAGE_FLOOR_BASIS_POINTS: u32 = 10_000;
pub const TEMPORAL_GRAPH_SHADOW_VALIDITY_WINDOW_FLOOR_BASIS_POINTS: u32 = 10_000;
pub const TEMPORAL_GRAPH_SHADOW_SUPERSEDES_FLOOR_BASIS_POINTS: u32 = 10_000;
pub const TEMPORAL_GRAPH_SHADOW_LATENCY_MAX_MS: u32 = 100;
pub const TEMPORAL_GRAPH_SHADOW_REGRET_MAX_BASIS_POINTS: u32 = 0;

/// Payload-light replay mode for temporal graph shadow evaluation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryTemporalGraphShadowEvalMode {
    DeterministicShadow,
    #[default]
    Unknown,
}

impl ContextMemoryTemporalGraphShadowEvalMode {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Fixed metric set for temporal graph shadow evaluation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryTemporalGraphShadowEvalMetric {
    NodeCoverage,
    EdgeCoverage,
    ValidityWindowCoverage,
    SupersedesCoverage,
    Latency,
    Regret,
    #[default]
    Unknown,
}

impl ContextMemoryTemporalGraphShadowEvalMetric {
    pub fn fixed_shadow_metrics() -> Vec<Self> {
        vec![
            Self::NodeCoverage,
            Self::EdgeCoverage,
            Self::ValidityWindowCoverage,
            Self::SupersedesCoverage,
            Self::Latency,
            Self::Regret,
        ]
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Stable fixture kind for temporal graph shadow evaluation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMemoryTemporalGraphShadowEvalFixtureKind {
    TopologyCoverage,
    ValidityWindowReplay,
    SupersedesReplay,
    RegressionGuard,
    #[default]
    Unknown,
}

impl ContextMemoryTemporalGraphShadowEvalFixtureKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TopologyCoverage => "topology_coverage",
            Self::ValidityWindowReplay => "validity_window_replay",
            Self::SupersedesReplay => "supersedes_replay",
            Self::RegressionGuard => "regression_guard",
            Self::Unknown => "unknown",
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Payload-light result for one temporal graph shadow fixture.
///
/// This stores only aggregate graph counts, fixed thresholds, and side-effect
/// flags. It intentionally does not carry entity text, fact text, transcript
/// text, memory text, source ids, session ids, memory ids, query payloads,
/// raw graph payloads, or operator identities.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTemporalGraphShadowEvalFixtureResult {
    pub fixture_kind: ContextMemoryTemporalGraphShadowEvalFixtureKind,
    pub fixture_id_hash: String,
    pub gate_pass: bool,
    pub positive_fixture: bool,
    pub negative_fixture: bool,
    pub shadow_eval_fixture: bool,
    pub temporal_fact_count: usize,
    pub graph_node_count: usize,
    pub graph_edge_count: usize,
    pub expected_node_count: usize,
    pub expected_edge_count: usize,
    pub expected_validity_window_edge_count: usize,
    pub observed_validity_window_edge_count: usize,
    pub expected_supersedes_edge_count: usize,
    pub observed_supersedes_edge_count: usize,
    pub node_coverage_basis_points: u32,
    pub edge_coverage_basis_points: u32,
    pub validity_window_coverage_basis_points: u32,
    pub supersedes_coverage_basis_points: u32,
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

impl ContextMemoryTemporalGraphShadowEvalFixtureResult {
    fn positive(
        fixture_kind: ContextMemoryTemporalGraphShadowEvalFixtureKind,
        temporal_fact_count: usize,
        graph_node_count: usize,
        graph_edge_count: usize,
        validity_window_edge_count: usize,
        supersedes_edge_count: usize,
        latency_ms: u32,
    ) -> Self {
        Self::fixture(
            fixture_kind,
            "positive",
            true,
            false,
            temporal_fact_count,
            graph_node_count,
            graph_edge_count,
            graph_node_count,
            graph_edge_count,
            validity_window_edge_count,
            validity_window_edge_count,
            supersedes_edge_count,
            supersedes_edge_count,
            latency_ms,
            TEMPORAL_GRAPH_SHADOW_LATENCY_MAX_MS,
            0,
            false,
            false,
        )
    }

    fn negative_regression(fixture_kind: ContextMemoryTemporalGraphShadowEvalFixtureKind) -> Self {
        Self::fixture(
            fixture_kind,
            "negative",
            false,
            true,
            3,
            2,
            4,
            3,
            7,
            3,
            2,
            1,
            0,
            125,
            TEMPORAL_GRAPH_SHADOW_LATENCY_MAX_MS,
            400,
            true,
            true,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn fixture(
        fixture_kind: ContextMemoryTemporalGraphShadowEvalFixtureKind,
        fixture_class: &str,
        positive_fixture: bool,
        negative_fixture: bool,
        temporal_fact_count: usize,
        graph_node_count: usize,
        graph_edge_count: usize,
        expected_node_count: usize,
        expected_edge_count: usize,
        expected_validity_window_edge_count: usize,
        observed_validity_window_edge_count: usize,
        expected_supersedes_edge_count: usize,
        observed_supersedes_edge_count: usize,
        latency_ms: u32,
        latency_budget_ms: u32,
        regret_basis_points: u32,
        regression_fixture: bool,
        regression_blocked: bool,
    ) -> Self {
        let node_coverage_basis_points =
            coverage_basis_points(graph_node_count, expected_node_count);
        let edge_coverage_basis_points =
            coverage_basis_points(graph_edge_count, expected_edge_count);
        let validity_window_coverage_basis_points = coverage_basis_points(
            observed_validity_window_edge_count,
            expected_validity_window_edge_count,
        );
        let supersedes_coverage_basis_points = coverage_basis_points(
            observed_supersedes_edge_count,
            expected_supersedes_edge_count,
        );

        Self {
            fixture_kind,
            fixture_id_hash: fixture_id_hash(
                fixture_kind,
                fixture_class,
                temporal_fact_count,
                graph_node_count,
                graph_edge_count,
                node_coverage_basis_points,
                edge_coverage_basis_points,
                latency_ms,
                regret_basis_points,
            ),
            gate_pass: true,
            positive_fixture,
            negative_fixture,
            shadow_eval_fixture: true,
            temporal_fact_count,
            graph_node_count,
            graph_edge_count,
            expected_node_count,
            expected_edge_count,
            expected_validity_window_edge_count,
            observed_validity_window_edge_count,
            expected_supersedes_edge_count,
            observed_supersedes_edge_count,
            node_coverage_basis_points,
            edge_coverage_basis_points,
            validity_window_coverage_basis_points,
            supersedes_coverage_basis_points,
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

    pub fn has_temporal_graph_fixture_integrity(
        &self,
        node_coverage_floor_basis_points: u32,
        edge_coverage_floor_basis_points: u32,
        validity_window_floor_basis_points: u32,
        supersedes_floor_basis_points: u32,
        latency_max_ms: u32,
        regret_max_basis_points: u32,
    ) -> bool {
        !self.fixture_kind.is_unknown()
            && stable_receipt_hash_is_valid(&self.fixture_id_hash)
            && self.gate_pass
            && self.positive_fixture != self.negative_fixture
            && self.shadow_eval_fixture
            && self.temporal_fact_count > 0
            && self.graph_node_count > 0
            && self.graph_edge_count > 0
            && self.expected_node_count > 0
            && self.expected_edge_count > 0
            && self.graph_node_count <= self.expected_node_count
            && self.graph_edge_count <= self.expected_edge_count
            && self.observed_validity_window_edge_count <= self.expected_validity_window_edge_count
            && self.observed_supersedes_edge_count <= self.expected_supersedes_edge_count
            && self.graph_edge_count
                >= self
                    .observed_validity_window_edge_count
                    .saturating_add(self.observed_supersedes_edge_count)
            && self.node_coverage_basis_points
                == coverage_basis_points(self.graph_node_count, self.expected_node_count)
            && self.edge_coverage_basis_points
                == coverage_basis_points(self.graph_edge_count, self.expected_edge_count)
            && self.validity_window_coverage_basis_points
                == coverage_basis_points(
                    self.observed_validity_window_edge_count,
                    self.expected_validity_window_edge_count,
                )
            && self.supersedes_coverage_basis_points
                == coverage_basis_points(
                    self.observed_supersedes_edge_count,
                    self.expected_supersedes_edge_count,
                )
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
                    && self.node_coverage_basis_points >= node_coverage_floor_basis_points
                    && self.edge_coverage_basis_points >= edge_coverage_floor_basis_points
                    && self.validity_window_coverage_basis_points
                        >= validity_window_floor_basis_points
                    && self.supersedes_coverage_basis_points >= supersedes_floor_basis_points
                    && self.latency_ms <= self.latency_budget_ms
                    && self.latency_ms <= latency_max_ms
                    && self.regret_basis_points <= regret_max_basis_points
            } else {
                self.regression_fixture
                    && self.regression_blocked
                    && (self.node_coverage_basis_points < node_coverage_floor_basis_points
                        || self.edge_coverage_basis_points < edge_coverage_floor_basis_points
                        || self.validity_window_coverage_basis_points
                            < validity_window_floor_basis_points
                        || self.supersedes_coverage_basis_points < supersedes_floor_basis_points
                        || self.latency_ms > latency_max_ms
                        || self.regret_basis_points > regret_max_basis_points)
            }
    }
}

#[allow(clippy::too_many_arguments)]
fn fixture_id_hash(
    fixture_kind: ContextMemoryTemporalGraphShadowEvalFixtureKind,
    fixture_class: &str,
    temporal_fact_count: usize,
    graph_node_count: usize,
    graph_edge_count: usize,
    node_coverage_basis_points: u32,
    edge_coverage_basis_points: u32,
    latency_ms: u32,
    regret_basis_points: u32,
) -> String {
    stable_receipt_hash(&[
        "context_memory_temporal_graph_shadow_eval",
        fixture_kind.as_str(),
        fixture_class,
        &temporal_fact_count.to_string(),
        &graph_node_count.to_string(),
        &graph_edge_count.to_string(),
        &node_coverage_basis_points.to_string(),
        &edge_coverage_basis_points.to_string(),
        &latency_ms.to_string(),
        &regret_basis_points.to_string(),
    ])
}

fn coverage_basis_points(numerator: usize, denominator: usize) -> u32 {
    if denominator == 0 {
        return if numerator == 0 { 10_000 } else { 0 };
    }

    basis_points(numerator, denominator)
}

/// Offline, behavior-neutral eval replay for temporal graph quality.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextMemoryTemporalGraphShadowEvalReport {
    pub schema_version: u32,
    pub mode: ContextMemoryTemporalGraphShadowEvalMode,
    pub metrics: Vec<ContextMemoryTemporalGraphShadowEvalMetric>,
    pub fixtures: Vec<ContextMemoryTemporalGraphShadowEvalFixtureResult>,
    pub node_coverage_floor_basis_points: u32,
    pub edge_coverage_floor_basis_points: u32,
    pub validity_window_floor_basis_points: u32,
    pub supersedes_floor_basis_points: u32,
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

impl Default for ContextMemoryTemporalGraphShadowEvalReport {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_EVAL_SCHEMA_VERSION,
            mode: ContextMemoryTemporalGraphShadowEvalMode::DeterministicShadow,
            metrics: Vec::new(),
            fixtures: Vec::new(),
            node_coverage_floor_basis_points:
                TEMPORAL_GRAPH_SHADOW_NODE_COVERAGE_FLOOR_BASIS_POINTS,
            edge_coverage_floor_basis_points:
                TEMPORAL_GRAPH_SHADOW_EDGE_COVERAGE_FLOOR_BASIS_POINTS,
            validity_window_floor_basis_points:
                TEMPORAL_GRAPH_SHADOW_VALIDITY_WINDOW_FLOOR_BASIS_POINTS,
            supersedes_floor_basis_points: TEMPORAL_GRAPH_SHADOW_SUPERSEDES_FLOOR_BASIS_POINTS,
            latency_max_ms: TEMPORAL_GRAPH_SHADOW_LATENCY_MAX_MS,
            regret_max_basis_points: TEMPORAL_GRAPH_SHADOW_REGRET_MAX_BASIS_POINTS,
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

impl ContextMemoryTemporalGraphShadowEvalReport {
    pub fn seeded() -> Self {
        Self {
            schema_version: CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_EVAL_SCHEMA_VERSION,
            mode: ContextMemoryTemporalGraphShadowEvalMode::DeterministicShadow,
            metrics: ContextMemoryTemporalGraphShadowEvalMetric::fixed_shadow_metrics(),
            fixtures: vec![
                ContextMemoryTemporalGraphShadowEvalFixtureResult::positive(
                    ContextMemoryTemporalGraphShadowEvalFixtureKind::TopologyCoverage,
                    5,
                    5,
                    10,
                    5,
                    0,
                    38,
                ),
                ContextMemoryTemporalGraphShadowEvalFixtureResult::positive(
                    ContextMemoryTemporalGraphShadowEvalFixtureKind::ValidityWindowReplay,
                    3,
                    3,
                    6,
                    3,
                    0,
                    42,
                ),
                ContextMemoryTemporalGraphShadowEvalFixtureResult::positive(
                    ContextMemoryTemporalGraphShadowEvalFixtureKind::SupersedesReplay,
                    2,
                    2,
                    5,
                    2,
                    1,
                    47,
                ),
                ContextMemoryTemporalGraphShadowEvalFixtureResult::negative_regression(
                    ContextMemoryTemporalGraphShadowEvalFixtureKind::RegressionGuard,
                ),
            ],
            node_coverage_floor_basis_points:
                TEMPORAL_GRAPH_SHADOW_NODE_COVERAGE_FLOOR_BASIS_POINTS,
            edge_coverage_floor_basis_points:
                TEMPORAL_GRAPH_SHADOW_EDGE_COVERAGE_FLOOR_BASIS_POINTS,
            validity_window_floor_basis_points:
                TEMPORAL_GRAPH_SHADOW_VALIDITY_WINDOW_FLOOR_BASIS_POINTS,
            supersedes_floor_basis_points: TEMPORAL_GRAPH_SHADOW_SUPERSEDES_FLOOR_BASIS_POINTS,
            latency_max_ms: TEMPORAL_GRAPH_SHADOW_LATENCY_MAX_MS,
            regret_max_basis_points: TEMPORAL_GRAPH_SHADOW_REGRET_MAX_BASIS_POINTS,
            operator_approval_required: true,
            production_route: false,
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            prompt_assembly_change: false,
            operator_activation_allowed: false,
        }
    }

    pub fn has_temporal_graph_shadow_integrity(&self) -> bool {
        self.schema_version == CONTEXT_MEMORY_TEMPORAL_GRAPH_SHADOW_EVAL_SCHEMA_VERSION
            && self.mode == ContextMemoryTemporalGraphShadowEvalMode::DeterministicShadow
            && !self.mode.is_unknown()
            && self.metrics == ContextMemoryTemporalGraphShadowEvalMetric::fixed_shadow_metrics()
            && self.metrics.iter().all(|metric| !metric.is_unknown())
            && self.fixture_count() == 4
            && self.fixture_pass_count() == 4
            && self.positive_fixture_count() == 3
            && self.negative_fixture_count() == 1
            && self.regression_blocked_count() == 1
            && self.min_positive_node_coverage_basis_points()
                >= self.node_coverage_floor_basis_points
            && self.min_positive_edge_coverage_basis_points()
                >= self.edge_coverage_floor_basis_points
            && self.min_positive_validity_window_coverage_basis_points()
                >= self.validity_window_floor_basis_points
            && self.min_positive_supersedes_coverage_basis_points()
                >= self.supersedes_floor_basis_points
            && self.max_positive_latency_ms() <= self.latency_max_ms
            && self.max_positive_regret_basis_points() <= self.regret_max_basis_points
            && self.fixtures.iter().all(|fixture| {
                fixture.has_temporal_graph_fixture_integrity(
                    self.node_coverage_floor_basis_points,
                    self.edge_coverage_floor_basis_points,
                    self.validity_window_floor_basis_points,
                    self.supersedes_floor_basis_points,
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

    pub fn regression_blocked_count(&self) -> usize {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.regression_fixture && fixture.regression_blocked)
            .count()
    }

    pub fn min_positive_node_coverage_basis_points(&self) -> u32 {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.node_coverage_basis_points)
            .min()
            .unwrap_or(0)
    }

    pub fn min_positive_edge_coverage_basis_points(&self) -> u32 {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.edge_coverage_basis_points)
            .min()
            .unwrap_or(0)
    }

    pub fn min_positive_validity_window_coverage_basis_points(&self) -> u32 {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.validity_window_coverage_basis_points)
            .min()
            .unwrap_or(0)
    }

    pub fn min_positive_supersedes_coverage_basis_points(&self) -> u32 {
        self.fixtures
            .iter()
            .filter(|fixture| fixture.positive_fixture)
            .map(|fixture| fixture.supersedes_coverage_basis_points)
            .min()
            .unwrap_or(0)
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

    pub fn fixture(
        &self,
        fixture_kind: ContextMemoryTemporalGraphShadowEvalFixtureKind,
    ) -> Option<&ContextMemoryTemporalGraphShadowEvalFixtureResult> {
        self.fixtures
            .iter()
            .find(|fixture| fixture.fixture_kind == fixture_kind)
    }
}
