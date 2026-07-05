use serde::Deserialize;
use serde::Serialize;

use crate::TranscriptSpanRef;
use crate::intelligence::HeptaNeuron;
use crate::intelligence::NeuronId;

use super::ContextBudget;
use super::ContextRecallBundle;

/// Source lane for one ranked recall item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextRecallSource {
    RecentWindow,
    Transcript,
    DurableMemory,
    SummaryMemory,
    ActiveTopicSession,
    ActiveNeuron,
    KnowledgeGraph,
}

/// Explainable score used to rank items inside an intelligence turn frame.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextRecallScore {
    pub recency: f32,
    pub relevance: f32,
    pub durability: f32,
    pub topic_activation: f32,
    pub neuron_activation: f32,
    pub confidence: f32,
    pub final_score: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Payload-light ranked item produced by blended recall.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextRecallItem {
    pub source: ContextRecallSource,
    pub source_id: String,
    pub summary: String,
    pub score: ContextRecallScore,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_transcript_spans: Vec<TranscriptSpanRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_memory_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topic_session_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub neuron_ids: Vec<NeuronId>,
}

/// One inspectable frame from which routing, neuron activation, and intuition
/// should be projected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntelligenceTurnFrame {
    pub recall_bundle: ContextRecallBundle,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub active_neurons: Vec<HeptaNeuron>,
    pub budget: ContextBudget,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provenance_spans: Vec<TranscriptSpanRef>,
    #[serde(default)]
    pub omitted_by_budget: usize,
}
