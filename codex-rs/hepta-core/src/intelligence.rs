//! Core contracts for Hepta Intelligence.
//!
//! These types describe the routing, topic-session, neuron, and intuition
//! surfaces that sit on top of transcript truth. They stay storage-agnostic so
//! the runtime can evolve implementation strategy without churning the kernel
//! contract layer.

use std::collections::BTreeMap;

use crate::memory::IntelligenceTurnFrame;
use crate::memory::TranscriptRange;
use crate::runtime_types::SessionId;
use crate::tools::RiskTier;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TopicId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TopicLabel(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NeuronId(pub String);

/// Lightweight reference to transcript evidence without embedding the full span.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptSpanRef {
    pub session_id: SessionId,
    pub range: TranscriptRange,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Canonical relation type for a cross-topic topic-session edge.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopicGraphEdgeKind {
    #[default]
    SemanticSimilarity,
    CoActivation,
    SplitComponent,
    MergedInto,
    HasComponent,
    TemporalContinuation,
    Conflict,
}

/// Lightweight graph edge between topic sessions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicGraphEdge {
    pub target_topic_session_id: String,
    #[serde(default)]
    pub kind: TopicGraphEdgeKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    pub weight: f32,
    #[serde(default)]
    pub evidence_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_confirmed_unix_ms: Option<u64>,
}

/// Stable state for one internal topic session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicSession {
    pub topic_session_id: String,
    pub topic_id: TopicId,
    pub topic_label: TopicLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_embedding: Option<Vec<f32>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked_surface_session_ids: Vec<SessionId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked_transcript_spans: Vec<TranscriptSpanRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub open_loops: Vec<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub entities: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub graph_edges: Vec<TopicGraphEdge>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub durable_memory_refs: Vec<String>,
    pub status: TopicSessionStatus,
    pub created_at_unix_ms: u64,
    pub last_active_unix_ms: u64,
}

/// Lifecycle state for a topic session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopicSessionStatus {
    Active,
    Dormant,
    Merged,
    Archived,
}

/// Activation score for one topic during routing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicActivationScore {
    pub topic_id: TopicId,
    pub topic_label: TopicLabel,
    pub score: f32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub matched_terms: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Shift classification emitted by topic routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopicShiftKind {
    Stayed,
    Shifted,
    CoActivated,
    Created,
    Revived,
    Merged,
    Split,
}

/// Explainable event describing why routing changed topic state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopicShiftEvent {
    pub kind: TopicShiftKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_topic_id: Option<TopicId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_topic_id: Option<TopicId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Runtime-facing result of routing one surface turn into internal topic state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicRoutingDecision {
    #[serde(default = "default_topic_router_id")]
    pub router_id: String,
    #[serde(default)]
    pub learned_signal_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub learned_router_signals: Vec<String>,
    pub surface_session_id: SessionId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_topic_id: Option<TopicId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_transcript_spans: Vec<TranscriptSpanRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub active_topic_session_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub created_topic_session_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub revived_topic_session_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activation_scores: Vec<TopicActivationScore>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_event: Option<TopicShiftEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

fn default_topic_router_id() -> String {
    "semantic-router:bootstrap-v1".to_string()
}

impl TopicRoutingDecision {
    pub fn is_multi_topic(&self) -> bool {
        self.activation_scores.len() > 1
    }
}

/// Pluggable topic-routing contract.
///
/// Runtime implementations may keep a bootstrap heuristic router, a semantic
/// embedding/classifier router, and a replay router for golden tests behind the
/// same inspection-friendly interface.
pub trait TopicRouter {
    fn router_id(&self) -> &'static str;
    fn route(&self, frame: &IntelligenceTurnFrame) -> TopicRoutingDecision;
}

/// Skill-level prior attached to a topic or neuron.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillPrior {
    pub skill_id: String,
    pub score: f32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_topic_ids: Vec<TopicId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_neuron_ids: Vec<NeuronId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Workflow-level prior attached to a topic or neuron.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowPrior {
    pub workflow_id: String,
    pub score: f32,
    #[serde(default)]
    pub exists_in_registry: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_capability: Option<String>,
    #[serde(default)]
    pub requires_confirmation: bool,
    #[serde(default)]
    pub action_mode: IntuitionActionMode,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_topic_ids: Vec<TopicId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_neuron_ids: Vec<NeuronId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// User or runtime feedback about an intuition suggestion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntuitionFeedbackOutcome {
    Accepted,
    Rejected,
    Ignored,
    Corrected,
    ExecutedSuccess,
    ExecutedFailed,
    UserOverride,
    ToolFailed,
    UnsafeBlocked,
}

/// Durable calibration signal for future intuition predictions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntuitionFeedbackRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision_id: Option<String>,
    pub surface_session_id: SessionId,
    pub user_intent: String,
    pub outcome: IntuitionFeedbackOutcome,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skill_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_topic_ids: Vec<TopicId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_neuron_ids: Vec<NeuronId>,
    pub weight_delta: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_outcome: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_correction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence_before: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence_after: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub created_at_unix_ms: u64,
}

fn default_activation_decay() -> f32 {
    1.0
}

/// Canonical relation type for a cross-topic neuron link.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NeuronLinkKind {
    #[default]
    SemanticSimilarity,
    EntityOverlap,
    WorkflowAdjacency,
    CausalDependency,
    TemporalContinuation,
    Conflict,
    Inhibition,
}

/// Whether a neuron link should amplify or suppress downstream activation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkPolarity {
    #[default]
    Excitatory,
    Inhibitory,
    Neutral,
}

/// Cross-topic association between two neurons.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeuronLink {
    pub target_neuron_id: NeuronId,
    #[serde(default)]
    pub kind: NeuronLinkKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    #[serde(default)]
    pub polarity: LinkPolarity,
    #[serde(default)]
    pub directional: bool,
    pub strength: f32,
    #[serde(default = "default_activation_decay")]
    pub activation_decay: f32,
    #[serde(default)]
    pub evidence_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_confirmed_unix_ms: Option<u64>,
}

/// Compressed durable topic unit used for routing and recall.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeptaNeuron {
    pub neuron_id: NeuronId,
    pub topic_id: TopicId,
    pub topic_label: TopicLabel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_embedding_centroid: Option<Vec<f32>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked_session_ids: Vec<SessionId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub linked_topic_session_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub important_transcript_spans: Vec<TranscriptSpanRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub promoted_memory_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub entity_state: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stable_preferences: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub open_loops: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skill_priors: Vec<SkillPrior>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workflow_priors: Vec<WorkflowPrior>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<NeuronLink>,
    #[serde(default = "default_neuron_revision")]
    pub neuron_revision: u64,
    #[serde(default = "default_compression_policy_version")]
    pub compression_policy_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_evidence_digest: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_refresh_reason: Option<String>,
    #[serde(default)]
    pub staleness_score: f32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub merged_from: Vec<NeuronId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub split_from: Vec<NeuronId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supersedes: Vec<NeuronId>,
    pub confidence: f32,
    pub freshness: f32,
    pub last_revalidated_unix_ms: u64,
}

fn default_neuron_revision() -> u64 {
    1
}

pub const MEMORY_NEURON_COMPRESSION_V2_POLICY: &str = "memory-neuron-compression-v2";

fn default_compression_policy_version() -> String {
    "bootstrap-v1".to_string()
}

/// Explainable activation emitted when the runtime lights up one or more neurons.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeuronActivation {
    pub neuron_id: NeuronId,
    pub topic_id: TopicId,
    pub direct_score: f32,
    pub propagated_score: f32,
    #[serde(default)]
    pub inhibition_score: f32,
    pub final_score: f32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_topic_session_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_neuron_ids: Vec<NeuronId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_transcript_spans: Vec<TranscriptSpanRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_link_kinds: Vec<NeuronLinkKind>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_link_reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl NeuronActivation {
    pub fn is_propagated_only(&self) -> bool {
        self.direct_score == 0.0 && self.propagated_score != 0.0
    }
}

/// Machine-readable result of compressing topic-session state into a neuron.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeuronCompressionReport {
    pub topic_id: TopicId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_neuron_id: Option<NeuronId>,
    #[serde(default = "default_compression_policy_version")]
    pub compression_policy_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_evidence_digest: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_topic_session_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub merged_neuron_ids: Vec<NeuronId>,
    #[serde(default)]
    pub linked_session_count: usize,
    pub important_span_count: usize,
    pub promoted_memory_count: usize,
    #[serde(default)]
    pub stable_preference_count: usize,
    #[serde(default)]
    pub open_loop_count: usize,
    #[serde(default)]
    pub skill_prior_count: usize,
    #[serde(default)]
    pub workflow_prior_count: usize,
    #[serde(default)]
    pub typed_link_count: usize,
    #[serde(default)]
    pub lineage_edge_count: usize,
    #[serde(default)]
    pub confidence: f32,
    #[serde(default)]
    pub freshness: f32,
    #[serde(default)]
    pub staleness_score: f32,
    #[serde(default)]
    pub provenance_complete: bool,
    #[serde(default)]
    pub intuition_ready: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Decision describing a routed skill suggestion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillActivationDecision {
    pub skill_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    pub score: f32,
    #[serde(default)]
    pub exists_in_registry: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_capability: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_tier: Option<RiskTier>,
    #[serde(default)]
    pub requires_confirmation: bool,
    #[serde(default)]
    pub action_mode: IntuitionActionMode,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_topic_ids: Vec<TopicId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_neuron_ids: Vec<NeuronId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// How far an intuition decision is allowed to move toward action.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntuitionActionMode {
    #[default]
    SuggestOnly,
    Prepare,
    ExecuteAllowed,
}

/// Request envelope for intuition prediction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntuitionRequest {
    pub surface_session_id: SessionId,
    pub user_intent: String,
    pub topic_limit: usize,
    pub neuron_limit: usize,
    pub skill_limit: usize,
}

/// Explainable bundle returned by intuition prediction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntuitionBundle {
    pub request: IntuitionRequest,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic_activation_scores: Vec<TopicActivationScore>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub neuron_activations: Vec<NeuronActivation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_transcript_spans: Vec<TranscriptSpanRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub foreground_topic_session_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skill_decisions: Vec<SkillActivationDecision>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workflow_priors: Vec<WorkflowPrior>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    pub truncated: bool,
}

impl IntuitionBundle {
    pub fn is_empty(&self) -> bool {
        self.topic_activation_scores.is_empty()
            && self.neuron_activations.is_empty()
            && self.skill_decisions.is_empty()
            && self.workflow_priors.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn sample_span_ref() -> TranscriptSpanRef {
        TranscriptSpanRef {
            session_id: SessionId("session-main".into()),
            range: TranscriptRange {
                start_sequence: 4,
                end_sequence: 9,
            },
            reason: Some("project continuity".into()),
        }
    }

    #[test]
    fn topic_session_roundtrips_through_json_with_typed_graph_edges() {
        let session = TopicSession {
            topic_session_id: "topic-session-1".into(),
            topic_id: TopicId("topic-hepta-intelligence".into()),
            topic_label: TopicLabel("Hepta Intelligence".into()),
            topic_embedding: Some(vec![0.12, 0.34, 0.56]),
            linked_surface_session_ids: vec![SessionId("surface-1".into())],
            linked_transcript_spans: vec![sample_span_ref()],
            open_loops: vec!["land typed graph store".into()],
            entities: BTreeMap::from([("repo".into(), "Hepta".into())]),
            graph_edges: vec![TopicGraphEdge {
                target_topic_session_id: "topic-session-2".into(),
                kind: TopicGraphEdgeKind::CoActivation,
                relation: Some("bootstrap_co_activation".into()),
                weight: 0.81,
                evidence_count: 3,
                last_confirmed_unix_ms: Some(1_713_713_200_000),
            }],
            durable_memory_refs: vec!["memory-42".into()],
            status: TopicSessionStatus::Active,
            created_at_unix_ms: 1_713_713_000_000,
            last_active_unix_ms: 1_713_713_200_000,
        };

        let json = serde_json::to_string(&session).expect("topic session should serialize");
        let parsed: TopicSession =
            serde_json::from_str(&json).expect("topic session should deserialize");

        assert_eq!(parsed, session);
        assert_eq!(parsed.graph_edges.len(), 1);
        assert_eq!(parsed.graph_edges[0].kind, TopicGraphEdgeKind::CoActivation);
    }

    #[test]
    fn topic_routing_decision_roundtrips_through_json() {
        let decision = TopicRoutingDecision {
            router_id: "semantic-router:learned-feedback-v1".into(),
            learned_signal_count: 2,
            learned_router_signals: vec![
                "feedback:accepted:topic-hepta-intelligence:+0.12".into(),
                "neuron:neuron-1:confidence:0.88".into(),
            ],
            surface_session_id: SessionId("surface-1".into()),
            primary_topic_id: Some(TopicId("topic-hepta-intelligence".into())),
            source_transcript_spans: vec![sample_span_ref()],
            active_topic_session_ids: vec!["topic-session-1".into(), "topic-session-2".into()],
            created_topic_session_ids: vec!["topic-session-2".into()],
            revived_topic_session_ids: vec!["topic-session-1".into()],
            activation_scores: vec![
                TopicActivationScore {
                    topic_id: TopicId("topic-hepta-intelligence".into()),
                    topic_label: TopicLabel("Hepta Intelligence".into()),
                    score: 0.92,
                    matched_terms: vec!["topic router".into(), "neuron".into()],
                    reason: Some("matched current architecture lane".into()),
                },
                TopicActivationScore {
                    topic_id: TopicId("topic-trnm".into()),
                    topic_label: TopicLabel("TRNM".into()),
                    score: 0.34,
                    matched_terms: vec!["worker".into()],
                    reason: None,
                },
            ],
            shift_event: Some(TopicShiftEvent {
                kind: TopicShiftKind::CoActivated,
                from_topic_id: Some(TopicId("topic-hepta-memory".into())),
                to_topic_id: Some(TopicId("topic-hepta-intelligence".into())),
                reason: Some("message mixed architecture and routing discussion".into()),
            }),
            explanation: Some(
                "revived architecture lane and opened a fresh topic-session for routing design"
                    .into(),
            ),
        };

        let json =
            serde_json::to_string(&decision).expect("topic routing decision should serialize");
        let parsed: TopicRoutingDecision =
            serde_json::from_str(&json).expect("topic routing decision should deserialize");

        assert_eq!(parsed, decision);
        assert!(parsed.is_multi_topic());
        assert_eq!(parsed.active_topic_session_ids.len(), 2);
        assert_eq!(parsed.source_transcript_spans.len(), 1);
    }

    #[test]
    fn hepta_neuron_roundtrips_through_json() {
        let neuron = HeptaNeuron {
            neuron_id: NeuronId("neuron-1".into()),
            topic_id: TopicId("topic-hepta-intelligence".into()),
            topic_label: TopicLabel("Hepta Intelligence".into()),
            topic_embedding_centroid: Some(vec![0.12, 0.34, 0.56]),
            linked_session_ids: vec![SessionId("surface-1".into())],
            linked_topic_session_ids: vec!["topic-session-1".into()],
            important_transcript_spans: vec![sample_span_ref()],
            promoted_memory_refs: vec!["memory-42".into()],
            entity_state: BTreeMap::from([("repo".into(), "Hepta".into())]),
            stable_preferences: vec!["prefer transcript truth".into()],
            open_loops: vec!["land topic router contract".into()],
            skill_priors: vec![SkillPrior {
                skill_id: "architecture-docs".into(),
                score: 0.81,
                source_topic_ids: vec![TopicId("topic-hepta-intelligence".into())],
                source_neuron_ids: vec![NeuronId("neuron-1".into())],
                reason: Some("frequently used for this topic".into()),
            }],
            workflow_priors: vec![WorkflowPrior {
                workflow_id: "hepta-intelligence-contract-lane".into(),
                score: 0.74,
                exists_in_registry: true,
                missing_capability: None,
                requires_confirmation: false,
                action_mode: IntuitionActionMode::Prepare,
                source_topic_ids: vec![TopicId("topic-hepta-intelligence".into())],
                source_neuron_ids: vec![NeuronId("neuron-1".into())],
                reason: Some("topic is still in contract-design phase".into()),
            }],
            links: vec![NeuronLink {
                target_neuron_id: NeuronId("neuron-2".into()),
                kind: NeuronLinkKind::SemanticSimilarity,
                relation: Some("adjacent_topic".into()),
                polarity: LinkPolarity::Excitatory,
                directional: false,
                strength: 0.41,
                activation_decay: 0.72,
                evidence_count: 3,
                last_confirmed_unix_ms: Some(1_713_713_100_000),
            }],
            neuron_revision: 1,
            compression_policy_version: "bootstrap-v1".into(),
            source_evidence_digest: Some("topic:topic-hepta-intelligence:sessions:1".into()),
            last_refresh_reason: Some("contract_test".into()),
            staleness_score: 0.33,
            merged_from: Vec::new(),
            split_from: Vec::new(),
            supersedes: Vec::new(),
            confidence: 0.88,
            freshness: 0.67,
            last_revalidated_unix_ms: 1_713_713_000_000,
        };

        let json = serde_json::to_string(&neuron).expect("hepta neuron should serialize");
        let parsed: HeptaNeuron =
            serde_json::from_str(&json).expect("hepta neuron should deserialize");

        assert_eq!(parsed, neuron);
        assert_eq!(parsed.important_transcript_spans.len(), 1);
        assert_eq!(parsed.skill_priors[0].skill_id, "architecture-docs");
    }

    #[test]
    fn intuition_bundle_roundtrips_through_json() {
        let bundle = IntuitionBundle {
            request: IntuitionRequest {
                surface_session_id: SessionId("surface-1".into()),
                user_intent: "continue Hepta Intelligence contracts".into(),
                topic_limit: 3,
                neuron_limit: 2,
                skill_limit: 2,
            },
            topic_activation_scores: vec![TopicActivationScore {
                topic_id: TopicId("topic-hepta-intelligence".into()),
                topic_label: TopicLabel("Hepta Intelligence".into()),
                score: 0.95,
                matched_terms: vec!["contracts".into(), "topic router".into()],
                reason: Some("turn directly references active architecture lane".into()),
            }],
            neuron_activations: vec![NeuronActivation {
                neuron_id: NeuronId("neuron-1".into()),
                topic_id: TopicId("topic-hepta-intelligence".into()),
                direct_score: 0.73,
                propagated_score: 0.16,
                inhibition_score: 0.05,
                final_score: 0.84,
                source_topic_session_ids: vec!["topic-session-1".into()],
                source_neuron_ids: vec![NeuronId("neuron-2".into())],
                source_transcript_spans: vec![sample_span_ref()],
                source_link_kinds: vec![NeuronLinkKind::SemanticSimilarity],
                source_link_reasons: vec!["semantic similarity propagated from adjacent architecture neuron".into()],
                reason: Some("fresh architecture neuron with open contract loop".into()),
            }],
            source_transcript_spans: vec![sample_span_ref()],
            foreground_topic_session_ids: vec!["topic-session-1".into()],
            skill_decisions: vec![SkillActivationDecision {
                skill_id: "rust-contract-authoring".into(),
                workflow_id: Some("hepta-intelligence-contract-lane".into()),
                score: 0.84,
                exists_in_registry: true,
                missing_capability: None,
                risk_tier: Some(RiskTier::Low),
                requires_confirmation: true,
                action_mode: IntuitionActionMode::SuggestOnly,
                source_topic_ids: vec![TopicId("topic-hepta-intelligence".into())],
                source_neuron_ids: vec![NeuronId("neuron-1".into())],
                reason: Some("best fit for landing kernel contracts".into()),
            }],
            workflow_priors: vec![WorkflowPrior {
                workflow_id: "hepta-intelligence-contract-lane".into(),
                score: 0.79,
                exists_in_registry: true,
                missing_capability: None,
                requires_confirmation: false,
                action_mode: IntuitionActionMode::Prepare,
                source_topic_ids: vec![TopicId("topic-hepta-intelligence".into())],
                source_neuron_ids: vec![NeuronId("neuron-1".into())],
                reason: Some("highest confidence active lane".into()),
            }],
            explanation: Some("foreground Hepta Intelligence contract lane and suggest Rust contract authoring workflow".into()),
            truncated: false,
        };

        let json = serde_json::to_string(&bundle).expect("intuition bundle should serialize");
        let parsed: IntuitionBundle =
            serde_json::from_str(&json).expect("intuition bundle should deserialize");

        assert_eq!(parsed, bundle);
        assert!(!parsed.is_empty());
        assert_eq!(
            parsed.skill_decisions[0].skill_id,
            "rust-contract-authoring"
        );
        assert_eq!(parsed.source_transcript_spans.len(), 1);
        assert_eq!(parsed.neuron_activations[0].final_score, 0.84);
        assert_eq!(
            parsed.neuron_activations[0].source_transcript_spans.len(),
            1
        );
        assert!(!parsed.neuron_activations[0].is_propagated_only());
    }

    #[test]
    fn neuron_link_defaults_support_legacy_deserialization() {
        let json = r#"{"target_neuron_id":"neuron-2","strength":0.4}"#;

        let parsed: NeuronLink =
            serde_json::from_str(json).expect("legacy neuron link should deserialize");

        assert_eq!(parsed.kind, NeuronLinkKind::SemanticSimilarity);
        assert_eq!(parsed.polarity, LinkPolarity::Excitatory);
        assert_eq!(parsed.activation_decay, 1.0);
        assert_eq!(parsed.evidence_count, 0);
        assert!(!parsed.directional);
    }

    #[test]
    fn topic_graph_edge_defaults_support_legacy_deserialization() {
        let json = r#"{"target_topic_session_id":"topic-session-2","weight":0.4}"#;

        let parsed: TopicGraphEdge =
            serde_json::from_str(json).expect("legacy topic graph edge should deserialize");

        assert_eq!(parsed.kind, TopicGraphEdgeKind::SemanticSimilarity);
        assert_eq!(parsed.weight, 0.4);
        assert_eq!(parsed.evidence_count, 0);
        assert!(parsed.relation.is_none());
        assert!(parsed.last_confirmed_unix_ms.is_none());
    }
}
