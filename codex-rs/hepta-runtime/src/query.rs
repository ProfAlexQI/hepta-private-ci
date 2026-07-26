use std::collections::BTreeMap;
use std::collections::BTreeSet;

use hepta_core::ContextRecallAvailability;
use hepta_core::ContextRecallBundle;
use hepta_core::ContextRecallItem;
use hepta_core::ContextRecallRequest;
use hepta_core::ContextRecallScore;
use hepta_core::ContextRecallSource;
use hepta_core::HeptaError;
use hepta_core::HeptaNeuron;
use hepta_core::IntelligenceTurnFrame;
use hepta_core::IntuitionActionMode;
use hepta_core::IntuitionBundle;
use hepta_core::IntuitionFeedbackOutcome;
use hepta_core::IntuitionFeedbackRecord;
use hepta_core::IntuitionRequest;
use hepta_core::LinkPolarity;
use hepta_core::MEMORY_NEURON_COMPRESSION_V2_POLICY;
use hepta_core::MemoryQuery;
use hepta_core::MemoryStore;
use hepta_core::MessageRole;
use hepta_core::ModelRef;
use hepta_core::NeuronActivation;
use hepta_core::NeuronCompressionReport;
use hepta_core::NeuronId;
use hepta_core::NeuronLink;
use hepta_core::NeuronLinkKind;
use hepta_core::SessionId;
use hepta_core::SkillPrior;
use hepta_core::TopicActivationScore;
use hepta_core::TopicGraphEdgeKind;
use hepta_core::TopicId;
use hepta_core::TopicRoutingDecision;
use hepta_core::TopicSession;
use hepta_core::TopicSessionStatus;
use hepta_core::TopicShiftEvent;
use hepta_core::TopicShiftKind;
use hepta_core::TranscriptEntry;
use hepta_core::TranscriptEntryKind;
use hepta_core::TranscriptQueryReport;
use hepta_core::TranscriptRange;
use hepta_core::TranscriptSpanRef;
use hepta_core::WorkflowPrior;
use hepta_intelligence::INTUITION_FEEDBACK_LEARNER_COUNT_KEY as FEEDBACK_LEARNER_COUNT_KEY;
use hepta_intelligence::IntuitionCalibrationFeedbackSummary;
use hepta_intelligence::IntuitionCalibrationTargetSummary;
use hepta_intelligence::IntuitionCapabilityView;
use hepta_intelligence::IntuitionPlan;
use hepta_intelligence::IntuitionPlanInput;
use hepta_intelligence::IntuitionPolicyBinding;
use hepta_intelligence::LearnedSemanticRouterEvidence;
use hepta_intelligence::MemoryKgAdapterClientReport;
use hepta_intelligence::MemoryKgAdapterConfigEnvReport;
use hepta_intelligence::MemoryKgAdapterDryRunReport;
use hepta_intelligence::MemoryKgAdapterStagingGateReport;
use hepta_intelligence::MemoryKgContextInjectionReadinessReport;
use hepta_intelligence::MemoryKgContextRecallBridgeReport;
use hepta_intelligence::MemoryKgPromptPreviewApprovalPacketReport;
use hepta_intelligence::MemoryKgPromptPreviewContextHandoffReport;
use hepta_intelligence::MemoryKgPromptPreviewOperatorEvidenceReport;
use hepta_intelligence::MemoryKgPromptPreviewPreflightReport;
use hepta_intelligence::MemoryKgPromptPreviewRedactionDiffReport;
use hepta_intelligence::MemoryKgPromptPreviewRollbackKillSwitchReport;
use hepta_intelligence::MemoryKgRecallEvaluationReport;
use hepta_intelligence::MemoryKgRecallPlanReport;
use hepta_intelligence::MemoryKgShadowRankComparisonReport;
use hepta_intelligence::MemoryKgShadowRankDriftReport;
use hepta_intelligence::MemoryKgShadowRankReport;
use hepta_intelligence::MemoryKgWriteCandidateReport;
use hepta_intelligence::NeuronActivationEvidenceCounts;
use hepta_intelligence::NeuronActivationInput;
use hepta_intelligence::SEMANTIC_ROUTER_LEARNED_KEY;
use hepta_intelligence::TopicAwareModelFeedbackOutcome;
use hepta_intelligence::TopicAwareModelFeedbackRecord;
use hepta_intelligence::TopicAwareModelFeedbackSummary;
use hepta_intelligence::TopicRouteShellPatch;
use hepta_intelligence::apply_intuition_feedback_to_topic_sessions;
use hepta_intelligence::compute_neuron_activations;
use hepta_intelligence::estimate_intuition_feedback_confidence;
use hepta_intelligence::evaluate_intelligence_semantic_expectations;
use hepta_intelligence::format_intuition_feedback_outcome;
use hepta_intelligence::intuition_calibration_feedback_summary;
use hepta_intelligence::intuition_calibration_skill_targets;
use hepta_intelligence::intuition_calibration_workflow_targets;
use hepta_intelligence::intuition_feedback_confidence_shift;
use hepta_intelligence::intuition_feedback_weight_delta;
use hepta_intelligence::is_learned_feedback_contrast_case;
use hepta_intelligence::learned_feedback_contrast_expected_signal_direction;
use hepta_intelligence::learned_feedback_contrast_focus;
use hepta_intelligence::memory_atom_pipeline_sample_report;
use hepta_intelligence::memory_kg_adapter_client_report;
use hepta_intelligence::memory_kg_adapter_config_env_report;
use hepta_intelligence::memory_kg_adapter_dry_run_report;
use hepta_intelligence::memory_kg_adapter_staging_gate_report;
use hepta_intelligence::memory_kg_context_injection_readiness_report;
use hepta_intelligence::memory_kg_context_recall_bridge_report;
use hepta_intelligence::memory_kg_prompt_preview_approval_packet_report;
use hepta_intelligence::memory_kg_prompt_preview_context_handoff_report;
use hepta_intelligence::memory_kg_prompt_preview_operator_evidence_report;
use hepta_intelligence::memory_kg_prompt_preview_preflight_report;
use hepta_intelligence::memory_kg_prompt_preview_redaction_diff_report;
use hepta_intelligence::memory_kg_prompt_preview_rollback_kill_switch_report;
use hepta_intelligence::memory_kg_recall_evaluation_report;
use hepta_intelligence::memory_kg_recall_plan_report;
use hepta_intelligence::memory_kg_shadow_rank_comparison_report;
use hepta_intelligence::memory_kg_shadow_rank_drift_report;
use hepta_intelligence::memory_kg_shadow_rank_report;
use hepta_intelligence::memory_kg_write_candidate_report;
use hepta_intelligence::neuron_lifecycle_health_summary;
use hepta_intelligence::plan_intuition;
use hepta_intelligence::reduce_intuition_feedback_neurons;
use hepta_intelligence::semantic_score_from_counts;
use hepta_intelligence::summarize_topic_aware_model_feedback;
use serde::Deserialize;
use serde::Serialize;

use crate::EventRecord;
use crate::MemorySnapshot;
use crate::RuntimeKernel;
use crate::SessionSnapshot;

pub const TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION: u32 = 1;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CoreTurnContextRecallSelectedSnippetEnvelope {
    pub version: u32,
    pub max_snippets: u32,
    pub max_snippet_chars: u32,
    pub selected_snippet_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub omitted_snippet_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub redacted_snippet_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub truncated_snippet_count: u32,
    pub snippets: Vec<CoreTurnContextRecallSelectedSnippet>,
    pub safety: CoreTurnContextRecallSelectedSnippetSafety,
}

impl CoreTurnContextRecallSelectedSnippetEnvelope {
    pub fn counts_match(&self) -> bool {
        self.selected_snippet_count == u32::try_from(self.snippets.len()).unwrap_or(u32::MAX)
            && self.redacted_snippet_count
                == u32::try_from(
                    self.snippets
                        .iter()
                        .filter(|snippet| snippet.redacted)
                        .count(),
                )
                .unwrap_or(u32::MAX)
            && self.truncated_snippet_count
                == u32::try_from(
                    self.snippets
                        .iter()
                        .filter(|snippet| snippet.truncated)
                        .count(),
                )
                .unwrap_or(u32::MAX)
    }

    pub fn bounds_match(&self) -> bool {
        self.selected_snippet_count <= self.max_snippets
            && self.snippets.len() <= usize::try_from(self.max_snippets).unwrap_or(usize::MAX)
            && self.snippets.iter().all(|snippet| {
                !snippet.text.is_empty()
                    && snippet.text.chars().count()
                        <= usize::try_from(self.max_snippet_chars).unwrap_or(usize::MAX)
                    && is_stable_manifest_replay_hash(&snippet.snippet_hash)
            })
    }

    pub fn safety_matches(&self) -> bool {
        let forbidden_exposure = self.safety.origin_identifiers_exposed
            || self.safety.raw_ranked_payload_exposed
            || self.safety.rank_explanation_exposed
            || self.safety.control_marker_exposed
            || self.safety.query_payload_exposed
            || self.safety.per_origin_list_exposed
            || self
                .snippets
                .iter()
                .any(|snippet| snippet.text.contains("[hepta-memory:"));
        self.safety.bounded == self.bounds_match()
            && self.safety.ready_for_shadow_handoff == (self.safety.bounded && !forbidden_exposure)
            && self.safety.ready_for_shadow_handoff
    }

    pub fn has_shadow_integrity(&self) -> bool {
        self.version == TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION
            && self.counts_match()
            && self.bounds_match()
            && self.safety_matches()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CoreTurnContextRecallSelectedSnippet {
    pub snippet_hash: String,
    pub text: String,
    pub estimated_tokens: u32,
    #[serde(default, skip_serializing_if = "is_false")]
    pub redacted: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub truncated: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CoreTurnContextRecallSelectedSnippetSafety {
    pub ready_for_shadow_handoff: bool,
    pub bounded: bool,
    pub origin_identifiers_exposed: bool,
    pub raw_ranked_payload_exposed: bool,
    pub rank_explanation_exposed: bool,
    pub control_marker_exposed: bool,
    pub query_payload_exposed: bool,
    pub per_origin_list_exposed: bool,
}
use crate::TopicGraphState;
use crate::TurnRecord;
use crate::current_unix_ms;
use crate::events::format_event_record;
use crate::events::summarize_line;

pub(crate) const PROVENANCE_RECALL_RECENT_WINDOW_LIMIT: usize = 6;
pub(crate) const PROVENANCE_RECALL_TRANSCRIPT_LIMIT: usize = 6;
pub(crate) const PROVENANCE_RECALL_MEMORY_LIMIT: usize = 6;
pub(crate) const PROVENANCE_INTUITION_TOPIC_LIMIT: usize = 3;
pub(crate) const PROVENANCE_INTUITION_NEURON_LIMIT: usize = 3;
pub(crate) const PROVENANCE_INTUITION_SKILL_LIMIT: usize = 3;
const LOW_TRUST_RANKED_ITEM_CONFIDENCE_THRESHOLD: f32 = 0.50;
const LOW_RECENCY_RANKED_ITEM_RECENCY_THRESHOLD: f32 = 0.50;
use self::topic_graph::bootstrap_topic_graph_edge;
#[cfg(test)]
use self::topic_graph::bootstrap_topic_graph_edge_count;
use self::topic_graph::bootstrap_topic_graph_edge_relation;
use self::topic_graph::bootstrap_topic_graph_edge_weight;
use self::topic_graph::bootstrap_topic_graph_relation_for_shift_kind;
use self::topic_graph::hydrate_topic_session_graph_edges;
use self::topic_graph::project_topic_sessions_with_graph_edges;
use self::topic_graph::upsert_bootstrap_topic_graph_edge;

mod context_recall_provider_rollup;
mod context_recall_selected_snippet_envelope;
mod context_recall_support;
mod context_recall_turn_handoff;
mod event_digest_rollup;
mod session_activity_rollup;
mod topic_graph;
mod transcript_query_rollup;
mod transcript_query_support;
fn revalidate_bootstrap_neuron_against_topic_sessions(
    stored: &HeptaNeuron,
    source_topic_sessions: &[TopicSession],
    all_topic_sessions: &[TopicSession],
) -> Result<Option<HeptaNeuron>, HeptaError> {
    if source_topic_sessions.is_empty() {
        return Ok(None);
    }

    let (mut candidate, _) = compress_bootstrap_topic_sessions_to_neuron(
        &stored.topic_id,
        source_topic_sessions,
        all_topic_sessions,
    )?;

    if !bootstrap_neuron_evidence_changed(stored, &candidate) {
        return Ok(None);
    }

    candidate.neuron_revision = stored.neuron_revision.saturating_add(1).max(2);
    candidate.last_refresh_reason = Some("bootstrap_revalidated_topic_session_evidence".into());
    candidate.merged_from =
        merge_bootstrap_neuron_id_sets(stored.merged_from.clone(), candidate.merged_from.clone());
    candidate.split_from =
        merge_bootstrap_neuron_id_sets(stored.split_from.clone(), candidate.split_from.clone());
    candidate.supersedes = stored.supersedes.clone();

    Ok(Some(candidate))
}

fn bootstrap_neuron_evidence_changed(stored: &HeptaNeuron, candidate: &HeptaNeuron) -> bool {
    stored.source_evidence_digest != candidate.source_evidence_digest
        || stored.linked_session_ids != candidate.linked_session_ids
        || stored.linked_topic_session_ids != candidate.linked_topic_session_ids
        || stored.important_transcript_spans != candidate.important_transcript_spans
        || stored.promoted_memory_refs != candidate.promoted_memory_refs
        || stored.open_loops != candidate.open_loops
        || stored.links != candidate.links
        || stored.merged_from != candidate.merged_from
}

fn merge_bootstrap_neuron_id_sets(mut left: Vec<NeuronId>, right: Vec<NeuronId>) -> Vec<NeuronId> {
    for neuron_id in right {
        if !left.contains(&neuron_id) {
            left.push(neuron_id);
        }
    }
    left.sort_by(|a, b| a.0.cmp(&b.0));
    left
}

fn attach_active_neurons_to_recall_bundle(
    bundle: &mut ContextRecallBundle,
    active_neurons: Vec<HeptaNeuron>,
) {
    if active_neurons.is_empty() {
        return;
    }

    for neuron in &active_neurons {
        let final_score = ((neuron.confidence * 0.55) + (neuron.freshness * 0.35)
            - (neuron.staleness_score * 0.10))
            .clamp(0.0, 1.0);
        bundle.ranked_items.push(ContextRecallItem {
            source: ContextRecallSource::ActiveNeuron,
            source_id: neuron.neuron_id.0.clone(),
            summary: neuron.topic_label.0.clone(),
            score: ContextRecallScore {
                recency: neuron.freshness,
                relevance: final_score,
                durability: neuron.confidence,
                topic_activation: 0.0,
                neuron_activation: final_score,
                confidence: neuron.confidence,
                final_score,
                reason: Some(format!(
                    "active neuron '{}' from {} topic session(s), policy={}, revision={}",
                    neuron.neuron_id.0,
                    neuron.linked_topic_session_ids.len(),
                    neuron.compression_policy_version,
                    neuron.neuron_revision,
                )),
            },
            source_transcript_spans: neuron.important_transcript_spans.clone(),
            source_memory_ids: neuron.promoted_memory_refs.clone(),
            topic_session_ids: neuron.linked_topic_session_ids.clone(),
            neuron_ids: vec![neuron.neuron_id.clone()],
        });
    }

    bundle.active_neurons = active_neurons;
    let (ranked_items, omitted_by_budget) = context_recall_support::select_ranked_items_for_budget(
        std::mem::take(&mut bundle.ranked_items),
        &bundle.budget,
    );
    bundle.ranked_items = ranked_items;
    if omitted_by_budget > 0 {
        bundle.omitted_by_budget = bundle.omitted_by_budget.saturating_add(omitted_by_budget);
        bundle.truncated = true;
    }
}

mod provenance_overview_rollup {
    use hepta_core::TopicSession;
    use hepta_core::TopicSessionStatus;

    use super::RuntimeProvenanceOverview;

    pub(super) struct ProvenanceOverviewInputs {
        pub session_id: String,
        pub last_user_intent_summary: Option<String>,
        pub topic_sessions: Vec<TopicSession>,
        pub recall_ranked_items: usize,
        pub recall_low_trust_ranked_items: usize,
        pub recall_low_recency_ranked_items: usize,
        pub recall_memory_control_omitted_items: usize,
        pub recall_transcript_evidence_spans: usize,
        pub recall_omitted_items: usize,
        pub intuition_transcript_evidence_spans: usize,
        pub intuition_foreground_topic_sessions: usize,
    }

    pub(super) fn build(input: ProvenanceOverviewInputs) -> RuntimeProvenanceOverview {
        let topic_coverage = tally_topic_sessions(&input.topic_sessions);

        RuntimeProvenanceOverview {
            session_id: input.session_id,
            last_user_intent_summary: input.last_user_intent_summary,
            total_topic_sessions: input.topic_sessions.len(),
            active_topic_sessions: topic_coverage.active_topic_sessions,
            active_topic_sessions_with_transcript_provenance: topic_coverage
                .active_topic_sessions_with_transcript_provenance,
            active_topic_sessions_missing_transcript_provenance: topic_coverage
                .active_topic_sessions
                .saturating_sub(topic_coverage.active_topic_sessions_with_transcript_provenance),
            recall_ranked_items: input.recall_ranked_items,
            recall_low_trust_ranked_items: input.recall_low_trust_ranked_items,
            recall_low_recency_ranked_items: input.recall_low_recency_ranked_items,
            recall_memory_control_omitted_items: input.recall_memory_control_omitted_items,
            recall_transcript_evidence_spans: input.recall_transcript_evidence_spans,
            recall_omitted_items: input.recall_omitted_items,
            intuition_transcript_evidence_spans: input.intuition_transcript_evidence_spans,
            intuition_foreground_topic_sessions: input.intuition_foreground_topic_sessions,
        }
    }

    struct TopicSessionCoverage {
        active_topic_sessions: usize,
        active_topic_sessions_with_transcript_provenance: usize,
    }

    fn tally_topic_sessions(topic_sessions: &[TopicSession]) -> TopicSessionCoverage {
        let active_topic_sessions = topic_sessions
            .iter()
            .filter(|topic_session| topic_session.status == TopicSessionStatus::Active)
            .count();
        let active_topic_sessions_with_transcript_provenance = topic_sessions
            .iter()
            .filter(|topic_session| {
                topic_session.status == TopicSessionStatus::Active
                    && !topic_session.linked_transcript_spans.is_empty()
            })
            .count();

        TopicSessionCoverage {
            active_topic_sessions,
            active_topic_sessions_with_transcript_provenance,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct RuntimeActivitySlice {
    pub history: Vec<TurnRecord>,
    pub events: Vec<EventRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuntimeSessionActivitySlice {
    pub session: SessionSnapshot,
    pub history: Vec<TurnRecord>,
    pub events: Vec<EventRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RuntimeSessionActivityOverview {
    pub sessions: Vec<RuntimeSessionActivitySlice>,
    pub active_sessions: usize,
    pub archived_sessions: usize,
    pub sessions_with_history: usize,
    pub sessions_with_events: usize,
    pub sessions_with_topic_state: usize,
    pub total_topic_sessions: usize,
    pub total_topic_graph_edges: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuntimeEventDigest {
    pub events: Vec<EventRecord>,
    pub kinds: Vec<RuntimeEventKindTally>,
    pub sessions: Vec<RuntimeEventSessionTally>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RuntimeContextRecallSlice {
    pub bundle: ContextRecallBundle,
    pub recent_entry_count: usize,
    pub total_recent_entry_count: usize,
    pub transcript_matched_count: usize,
    pub transcript_returned_count: usize,
    pub memory_matched_count: usize,
    pub durable_memory_hit_count: usize,
    pub summary_hit_count: usize,
    pub memory_control_omitted_count: usize,
    pub active_topic_session_count: usize,
    pub transcript_evidence: Vec<TranscriptSpanRef>,
    pub low_trust_ranked_item_count: usize,
    pub low_recency_ranked_item_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeContextRecallProviderRollup {
    pub recall_selection: RuntimeContextRecallSelectionSummary,
}

#[derive(Clone, PartialEq, Eq)]
pub struct RuntimeContextRecallTurnHandoff {
    pub provider_rollup: RuntimeContextRecallProviderRollup,
    pub selected_snippets: Option<CoreTurnContextRecallSelectedSnippetEnvelope>,
}

impl std::fmt::Debug for RuntimeContextRecallTurnHandoff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuntimeContextRecallTurnHandoff")
            .field("provider_rollup", &self.provider_rollup)
            .field(
                "selected_snippets_present",
                &self.selected_snippets.is_some(),
            )
            .field(
                "selected_snippet_count",
                &self
                    .selected_snippets
                    .as_ref()
                    .map(|envelope| envelope.selected_snippet_count),
            )
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeContextRecallSelectedSnippetEnvelope {
    pub version: u32,
    pub max_snippets: usize,
    pub max_snippet_chars: usize,
    pub selected_snippet_count: usize,
    pub omitted_snippet_count: usize,
    pub redacted_snippet_count: usize,
    pub truncated_snippet_count: usize,
    pub snippets: Vec<RuntimeContextRecallSelectedSnippet>,
    pub safety: RuntimeContextRecallSelectedSnippetSafety,
}

impl RuntimeContextRecallSelectedSnippetEnvelope {
    pub fn into_core_envelope(self) -> Option<CoreTurnContextRecallSelectedSnippetEnvelope> {
        let envelope = CoreTurnContextRecallSelectedSnippetEnvelope {
            version: self.version,
            max_snippets: u32::try_from(self.max_snippets).ok()?,
            max_snippet_chars: u32::try_from(self.max_snippet_chars).ok()?,
            selected_snippet_count: u32::try_from(self.selected_snippet_count).ok()?,
            omitted_snippet_count: u32::try_from(self.omitted_snippet_count).ok()?,
            redacted_snippet_count: u32::try_from(self.redacted_snippet_count).ok()?,
            truncated_snippet_count: u32::try_from(self.truncated_snippet_count).ok()?,
            snippets: self
                .snippets
                .into_iter()
                .map(RuntimeContextRecallSelectedSnippet::into_core)
                .collect(),
            safety: self.safety.into_core(),
        };

        envelope.has_shadow_integrity().then_some(envelope)
    }

    pub fn into_core_envelope_for_experimental_client(
        envelope: Option<Self>,
        experimental_api_enabled: bool,
    ) -> Option<CoreTurnContextRecallSelectedSnippetEnvelope> {
        if !experimental_api_enabled {
            return None;
        }

        envelope.and_then(Self::into_core_envelope)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeContextRecallSelectedSnippet {
    pub snippet_hash: String,
    pub text: String,
    pub estimated_tokens: u32,
    pub redacted: bool,
    pub truncated: bool,
}

impl RuntimeContextRecallSelectedSnippet {
    fn into_core(self) -> CoreTurnContextRecallSelectedSnippet {
        CoreTurnContextRecallSelectedSnippet {
            snippet_hash: self.snippet_hash,
            text: self.text,
            estimated_tokens: self.estimated_tokens,
            redacted: self.redacted,
            truncated: self.truncated,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeContextRecallSelectedSnippetSafety {
    pub ready_for_shadow_handoff: bool,
    pub bounded: bool,
    pub origin_identifiers_exposed: bool,
    pub raw_ranked_payload_exposed: bool,
    pub rank_explanation_exposed: bool,
    pub control_marker_exposed: bool,
    pub query_payload_exposed: bool,
    pub per_origin_list_exposed: bool,
}

impl RuntimeContextRecallSelectedSnippetSafety {
    fn into_core(self) -> CoreTurnContextRecallSelectedSnippetSafety {
        CoreTurnContextRecallSelectedSnippetSafety {
            ready_for_shadow_handoff: self.ready_for_shadow_handoff,
            bounded: self.bounded,
            origin_identifiers_exposed: self.origin_identifiers_exposed,
            raw_ranked_payload_exposed: self.raw_ranked_payload_exposed,
            rank_explanation_exposed: self.rank_explanation_exposed,
            control_marker_exposed: self.control_marker_exposed,
            query_payload_exposed: self.query_payload_exposed,
            per_origin_list_exposed: self.per_origin_list_exposed,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeContextRecallSelectionSummary {
    pub returned_source_count: u32,
    pub selected_source_count: u32,
    pub ranked_source_count: u32,
    pub returned_unselected_source_count: u32,
    pub source_diversity_met: bool,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub source_diversity_target: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub max_per_source: u32,
    pub ranked_item_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub omitted_by_budget_count: u32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub memory_control_omitted_count: u32,
    pub low_trust_ranked_item_count: u32,
    pub low_recency_ranked_item_count: u32,
}

impl RuntimeContextRecallSelectionSummary {
    pub fn has_count_integrity(&self) -> bool {
        self.selected_source_count <= self.returned_source_count
            && self.ranked_source_count <= self.selected_source_count
            && self.ranked_source_count <= self.ranked_item_count
            && (self.ranked_item_count == 0 || self.ranked_source_count > 0)
            && self.returned_unselected_source_count
                == self
                    .returned_source_count
                    .saturating_sub(self.selected_source_count)
            && (self.source_diversity_target == 0
                || self.source_diversity_met
                    == (self.selected_source_count >= self.source_diversity_target))
            && self.low_trust_ranked_item_count <= self.ranked_item_count
            && self.low_recency_ranked_item_count <= self.ranked_item_count
    }
}

fn is_zero_u32(value: &u32) -> bool {
    *value == 0
}

fn is_false(value: &bool) -> bool {
    !*value
}

fn is_stable_manifest_replay_hash(value: &str) -> bool {
    value.len() == 16 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RuntimeNeuronActivationOverview {
    pub session_id: String,
    pub query_text: Option<String>,
    pub recent_entry_count: usize,
    pub transcript_matched_count: usize,
    pub durable_memory_hit_count: usize,
    pub summary_hit_count: usize,
    pub active_topic_session_count: usize,
    pub routed_topic_count: usize,
    pub activations: Vec<NeuronActivation>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RuntimeIntuitionOverview {
    pub session_id: String,
    pub user_intent: String,
    pub router_id: String,
    pub learned_router_signal_count: usize,
    pub learned_router_signals: Vec<String>,
    pub recent_entry_count: usize,
    pub transcript_matched_count: usize,
    pub durable_memory_hit_count: usize,
    pub summary_hit_count: usize,
    pub active_topic_session_count: usize,
    pub routed_topic_count: usize,
    pub returned_neuron_activation_count: usize,
    pub bundle: IntuitionBundle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeProvenanceOverview {
    pub session_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_user_intent_summary: Option<String>,
    pub total_topic_sessions: usize,
    pub active_topic_sessions: usize,
    pub active_topic_sessions_with_transcript_provenance: usize,
    pub active_topic_sessions_missing_transcript_provenance: usize,
    #[serde(default)]
    pub recall_ranked_items: usize,
    #[serde(default)]
    pub recall_low_trust_ranked_items: usize,
    #[serde(default)]
    pub recall_low_recency_ranked_items: usize,
    #[serde(default)]
    pub recall_memory_control_omitted_items: usize,
    pub recall_transcript_evidence_spans: usize,
    pub recall_omitted_items: usize,
    pub intuition_transcript_evidence_spans: usize,
    pub intuition_foreground_topic_sessions: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeNeuronLifecycleOverview {
    pub session_id: String,
    pub total_topic_sessions: usize,
    pub active_topic_sessions: usize,
    pub stored_neurons: usize,
    pub neurons_with_transcript_provenance: usize,
    pub neurons_with_memory_provenance: usize,
    pub neurons_with_evidence_digest: usize,
    pub v2_compressed_neurons: usize,
    pub neurons_with_skill_priors: usize,
    pub neurons_with_workflow_priors: usize,
    pub neurons_with_typed_links: usize,
    pub intuition_ready_neurons: usize,
    pub lineage_neurons: usize,
    pub merged_neurons: usize,
    pub split_neurons: usize,
    pub superseded_neurons: usize,
    pub aging_neurons: usize,
    pub cross_session_stable_neurons: usize,
    pub cross_session_unstable_neurons: usize,
    pub merge_split_lineage_edges: usize,
    pub average_confidence: f32,
    pub average_freshness: f32,
    pub stale_neurons: usize,
    pub low_confidence_neurons: usize,
    pub low_freshness_neurons: usize,
    pub compression_policy_versions: BTreeMap<String, usize>,
    pub neuron_upgrade_ready: bool,
    pub active_topics_without_neurons: Vec<String>,
    pub findings: Vec<String>,
    pub healthy: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeIntuitionCalibrationOverview {
    pub session_id: String,
    pub feedback_record_count: usize,
    pub learner_applied_update_count: usize,
    pub learned_topic_hint_count: usize,
    pub learned_neuron_update_count: usize,
    pub closed_loop_ready: bool,
    pub positive_feedback_count: usize,
    pub negative_feedback_count: usize,
    pub neutral_feedback_count: usize,
    pub net_weight_delta: f32,
    pub average_weight_delta: f32,
    pub confidence_shift_count: usize,
    pub average_confidence_shift: f32,
    pub outcome_counts: BTreeMap<String, usize>,
    pub skill_targets: Vec<RuntimeIntuitionCalibrationTarget>,
    pub workflow_targets: Vec<RuntimeIntuitionCalibrationTarget>,
    pub learning_findings: Vec<String>,
    pub recent_feedback: Vec<RuntimeIntuitionCalibrationFeedback>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeIntuitionCalibrationTarget {
    pub target_kind: String,
    pub target_id: String,
    pub feedback_count: usize,
    pub positive_feedback_count: usize,
    pub negative_feedback_count: usize,
    pub neutral_feedback_count: usize,
    pub net_weight_delta: f32,
    pub average_weight_delta: f32,
    pub confidence_shift_count: usize,
    pub average_confidence_shift: f32,
    pub last_feedback_unix_ms: Option<u64>,
    pub outcome_counts: BTreeMap<String, usize>,
    pub source_topic_ids: Vec<String>,
    pub source_neuron_ids: Vec<String>,
    pub latest_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeIntuitionCalibrationFeedback {
    pub decision_id: Option<String>,
    pub user_intent: String,
    pub outcome: String,
    pub skill_id: Option<String>,
    pub workflow_id: Option<String>,
    pub weight_delta: f32,
    pub created_at_unix_ms: u64,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeIntelligenceEvalCase {
    pub case_id: String,
    pub transcript_sequence: u64,
    pub query_text: String,
    pub router_id: String,
    pub contrast_focus: Option<String>,
    pub contrast_expected_signal_direction: Option<String>,
    pub learned_router_signal_count: usize,
    pub learned_positive_signal_count: usize,
    pub learned_negative_signal_count: usize,
    pub recall_ranked_items: usize,
    pub recall_transcript_evidence_spans: usize,
    pub active_neuron_count: usize,
    pub routed_topic_count: usize,
    pub neuron_activation_count: usize,
    pub foreground_topic_session_count: usize,
    pub suggested_skill_count: usize,
    pub registered_skill_decision_count: usize,
    pub prepared_skill_decision_count: usize,
    pub gated_skill_decision_count: usize,
    pub workflow_prior_count: usize,
    pub registered_workflow_prior_count: usize,
    pub prepared_workflow_prior_count: usize,
    pub gated_workflow_prior_count: usize,
    pub semantic_expectation_count: usize,
    pub semantic_expectation_passed_count: usize,
    pub semantic_score: u8,
    pub semantic_failures: Vec<String>,
    pub passed: bool,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeIntelligenceEvalOverview {
    pub session_id: String,
    pub evaluated_case_count: usize,
    pub passed_case_count: usize,
    pub failed_case_count: usize,
    pub semantic_router_id: String,
    pub learned_router_case_count: usize,
    pub total_learned_router_signals: usize,
    pub total_learned_positive_signals: usize,
    pub total_learned_negative_signals: usize,
    pub contrast_focus_case_counts: BTreeMap<String, usize>,
    pub contrast_focus_passed_counts: BTreeMap<String, usize>,
    pub contrast_focus_signal_counts: BTreeMap<String, usize>,
    pub contrast_focus_positive_signal_counts: BTreeMap<String, usize>,
    pub contrast_focus_negative_signal_counts: BTreeMap<String, usize>,
    pub total_recall_ranked_items: usize,
    pub total_transcript_evidence_spans: usize,
    pub total_active_neurons: usize,
    pub total_routed_topics: usize,
    pub total_neuron_activations: usize,
    pub total_suggested_skills: usize,
    pub registered_skill_decision_count: usize,
    pub prepared_skill_decision_count: usize,
    pub gated_skill_decision_count: usize,
    pub total_workflow_priors: usize,
    pub registered_workflow_prior_count: usize,
    pub prepared_workflow_prior_count: usize,
    pub gated_workflow_prior_count: usize,
    pub feedback_record_count: usize,
    pub feedback_net_weight_delta: f32,
    pub calibrated_skill_target_count: usize,
    pub calibrated_workflow_target_count: usize,
    pub total_semantic_expectations: usize,
    pub total_semantic_expectations_passed: usize,
    pub semantic_score: u8,
    pub cases: Vec<RuntimeIntelligenceEvalCase>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeIntelligencePhase2Gate {
    pub id: String,
    pub title: String,
    pub ready: bool,
    pub evidence: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeIntelligencePhase2Overview {
    pub session_id: String,
    pub phase: String,
    pub status: String,
    pub overall_percent: u8,
    pub all_phase2_gates_ready: bool,
    pub blended_recall_ready: bool,
    pub provenance_memory_ready: bool,
    pub semantic_router_generalized: bool,
    pub neuron_compression_ready: bool,
    pub recall_ranked_items: usize,
    pub recall_source_count: usize,
    #[serde(default)]
    pub recall_low_trust_ranked_items: usize,
    #[serde(default)]
    pub recall_low_recency_ranked_items: usize,
    #[serde(default)]
    pub recall_memory_control_omitted_items: usize,
    pub recall_transcript_evidence_spans: usize,
    pub durable_memory_hits: usize,
    pub active_neurons: usize,
    pub provenance_active_topic_sessions: usize,
    pub provenance_topic_sessions_with_transcript: usize,
    pub supported_semantic_router_count: usize,
    pub learned_router_signal_count: usize,
    pub compressed_neuron_count: usize,
    pub neurons_with_evidence_digest: usize,
    pub gates: Vec<RuntimeIntelligencePhase2Gate>,
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RuntimeTopicRoutingOverview {
    pub session_id: String,
    pub query_text: Option<String>,
    pub router_id: String,
    pub learned_router_signal_count: usize,
    pub learned_router_signals: Vec<String>,
    pub recent_entry_count: usize,
    pub transcript_matched_count: usize,
    pub durable_memory_hit_count: usize,
    pub summary_hit_count: usize,
    pub decision: TopicRoutingDecision,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RuntimeTopicSessionOverview {
    pub session_id: String,
    pub topic_sessions: Vec<TopicSession>,
}

#[derive(Debug, Clone, PartialEq)]
struct BootstrapTopicRouteOutcome {
    primary_topic_id: Option<TopicId>,
    active_topic_session_ids: Vec<String>,
    created_topic_session_ids: Vec<String>,
    revived_topic_session_ids: Vec<String>,
    activation_scores: Vec<TopicActivationScore>,
    shift_event: TopicShiftEvent,
    explanation: String,
}

type BootstrapTopicCandidateRoute = hepta_intelligence::BootstrapTopicRouteCandidate;

#[derive(Debug, Clone, PartialEq)]
struct BootstrapTopicRoutePlan {
    routes: Vec<BootstrapTopicCandidateRoute>,
    selected_existing_indices: BTreeSet<usize>,
    merged_source_indices: BTreeSet<usize>,
    merge_marker: Option<&'static str>,
    split_marker: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
struct BootstrapTopicGraphRouteCandidate {
    target_index: usize,
    source_score: f32,
    strength: f32,
    matched_terms: Vec<String>,
    reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuntimeTranscriptQueryOverview {
    pub report: TranscriptQueryReport,
    pub returned_entries: usize,
    pub matched_sessions: usize,
    pub sessions: Vec<RuntimeTranscriptQuerySessionTally>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuntimeTranscriptQuerySessionTally {
    pub session_id: String,
    pub hit_count: usize,
    pub entry_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuntimeEventKindTally {
    pub kind: String,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuntimeEventSessionTally {
    pub session_id: Option<String>,
    pub count: usize,
    pub latest_event: EventRecord,
}

impl RuntimeEventDigest {
    pub(crate) fn recent_event_count(&self) -> usize {
        self.events.len()
    }

    pub(crate) fn kind_count(&self) -> usize {
        self.kinds.len()
    }

    pub(crate) fn session_scope_count(&self) -> usize {
        self.sessions.len()
    }

    pub(crate) fn summary_sections(&self) -> Vec<String> {
        let mut lines = vec!["By kind:".to_string()];

        if self.kinds.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(self.kinds.iter().map(RuntimeEventKindTally::summary_line));
        }

        lines.push("By session:".to_string());
        if self.sessions.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(
                self.sessions
                    .iter()
                    .map(RuntimeEventSessionTally::summary_line),
            );
        }

        lines.push("Recent events:".to_string());
        if self.events.is_empty() {
            lines.push("  - none".to_string());
        } else {
            lines.extend(self.events.iter().map(format_event_record));
        }

        lines
    }
}

impl RuntimeEventKindTally {
    fn summary_line(&self) -> String {
        format!("  - {}: {}", self.kind, self.count)
    }
}

impl RuntimeEventSessionTally {
    fn summary_line(&self) -> String {
        format!(
            "  - {}: {}, latest={:?}, summary=\"{}\"",
            self.session_id.as_deref().unwrap_or("global"),
            self.count,
            self.latest_event.event.kind,
            summarize_line(&self.latest_event.event.summary, 48)
        )
    }
}

impl RuntimeKernel {
    pub fn memory_snapshot(&self, limit: usize) -> Result<Vec<MemorySnapshot>, HeptaError> {
        let mut items = self
            .memory
            .list_memories()
            .map_err(|err| HeptaError(err.0))?;
        items.reverse();
        items.truncate(limit);
        Ok(items
            .into_iter()
            .map(|record| MemorySnapshot {
                id: record.id,
                scope: record.scope,
                content: record.content,
            })
            .collect())
    }

    pub async fn memory_search(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<MemorySnapshot>, HeptaError> {
        let hits = self
            .memory
            .search(MemoryQuery {
                text: query.to_string(),
                limit,
            })
            .await
            .map_err(|err| HeptaError(err.0))?;
        Ok(hits
            .into_iter()
            .map(|record| MemorySnapshot {
                id: record.id,
                scope: record.scope,
                content: record.content,
            })
            .collect())
    }

    pub fn history(
        &self,
        session_id: Option<&str>,
        limit: usize,
    ) -> Result<Vec<TurnRecord>, HeptaError> {
        let guard = self
            .history_state
            .lock()
            .map_err(|_| HeptaError("history state mutex poisoned".into()))?;
        let mut items = guard
            .iter()
            .filter(|item| session_id.map(|id| item.session_id == id).unwrap_or(true))
            .cloned()
            .collect::<Vec<_>>();
        items.reverse();
        items.truncate(limit);
        Ok(items)
    }

    pub fn recent_session_window(
        &self,
        session_id: &str,
        limit: usize,
    ) -> Result<Vec<TranscriptEntry>, HeptaError> {
        let mut items = self.transcript_entries_for_session(session_id)?;
        if items.len() > limit {
            items = items.split_off(items.len() - limit);
        }
        Ok(items)
    }

    pub fn activity_slice(
        &self,
        session_id: Option<&str>,
        history_limit: usize,
        event_limit: usize,
    ) -> Result<RuntimeActivitySlice, HeptaError> {
        Ok(RuntimeActivitySlice {
            history: self.history(session_id, history_limit)?,
            events: self.query_events(event_limit, None, session_id)?,
        })
    }

    pub fn session_activity_slices(
        &self,
        history_limit: usize,
        event_limit: usize,
    ) -> Result<Vec<RuntimeSessionActivitySlice>, HeptaError> {
        self.sessions()?
            .into_iter()
            .map(|session| {
                let session_id = session.session_id.clone();
                Ok(RuntimeSessionActivitySlice {
                    history: self.history(Some(&session_id), history_limit)?,
                    events: self.query_events(event_limit, None, Some(&session_id))?,
                    session,
                })
            })
            .collect()
    }

    pub fn session_activity_overview(
        &self,
        history_limit: usize,
        event_limit: usize,
    ) -> Result<RuntimeSessionActivityOverview, HeptaError> {
        let sessions = self.session_activity_slices(history_limit, event_limit)?;
        Ok(session_activity_rollup::build(sessions))
    }

    pub(crate) fn event_digest(&self, limit: usize) -> Result<RuntimeEventDigest, HeptaError> {
        let events = self.query_events(limit, None, None)?;
        Ok(event_digest_rollup::build(events))
    }

    pub fn query_transcript(
        &self,
        session_id: Option<&str>,
        query: &str,
        limit: usize,
    ) -> Result<TranscriptQueryReport, HeptaError> {
        let transcript_query = transcript_query_support::request(session_id, query, limit);

        let store_report = self
            .memory
            .transcript_search_report(transcript_query.clone())
            .map_err(|err| HeptaError(err.0))?;
        if store_report.matched_count > 0 {
            return Ok(store_report);
        }

        let legacy_session_id = transcript_query.session_id.as_ref().map(|id| id.0.as_str());
        let legacy_entries = self.legacy_transcript_entries(legacy_session_id)?;

        Ok(transcript_query_support::fallback_legacy_report(
            transcript_query,
            legacy_entries,
        ))
    }

    pub(crate) fn transcript_query_overview(
        &self,
        session_id: Option<&str>,
        query: &str,
        limit: usize,
    ) -> Result<RuntimeTranscriptQueryOverview, HeptaError> {
        let report = self.query_transcript(session_id, query, limit)?;
        Ok(transcript_query_rollup::build(report))
    }

    pub fn recall_context(
        &self,
        session_id: &str,
        query_text: Option<&str>,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        allow_cross_session: bool,
    ) -> Result<ContextRecallBundle, HeptaError> {
        let mut bundle = self
            .context_recall_slice(
                session_id,
                query_text,
                recent_window_limit,
                transcript_limit,
                memory_limit,
                allow_cross_session,
            )?
            .bundle;

        let active_topic_session_ids = bundle
            .active_topic_sessions
            .iter()
            .map(|topic_session| topic_session.topic_session_id.clone())
            .collect::<Vec<_>>();
        if !active_topic_session_ids.is_empty() {
            let topic_sessions = self.topic_session_overview(session_id)?.topic_sessions;
            let active_neurons = self.resolve_active_neurons_for_routing(
                session_id,
                &topic_sessions,
                &active_topic_session_ids,
                active_topic_session_ids.len(),
            )?;
            attach_active_neurons_to_recall_bundle(&mut bundle, active_neurons);
        }

        Ok(bundle)
    }

    pub fn intelligence_turn_frame(
        &self,
        session_id: &str,
        query_text: Option<&str>,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        allow_cross_session: bool,
    ) -> Result<IntelligenceTurnFrame, HeptaError> {
        let recall_bundle = self.recall_context(
            session_id,
            query_text,
            recent_window_limit,
            transcript_limit,
            memory_limit,
            allow_cross_session,
        )?;
        let active_neurons = recall_bundle.active_neurons.clone();
        let budget = recall_bundle.budget;
        let provenance_spans = recall_bundle.source_transcript_spans();
        let omitted_by_budget = recall_bundle.omitted_by_budget;

        Ok(IntelligenceTurnFrame {
            recall_bundle,
            active_neurons,
            budget,
            provenance_spans,
            omitted_by_budget,
        })
    }

    pub fn activate_neurons(
        &self,
        session_id: &str,
        query_text: Option<&str>,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        neuron_limit: usize,
    ) -> Result<Vec<NeuronActivation>, HeptaError> {
        Ok(self
            .neuron_activation_overview(
                session_id,
                query_text,
                recent_window_limit,
                transcript_limit,
                memory_limit,
                neuron_limit,
            )?
            .activations)
    }

    pub fn predict_intuition(
        &self,
        session_id: &str,
        user_intent: &str,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        topic_limit: usize,
        neuron_limit: usize,
        skill_limit: usize,
    ) -> Result<IntuitionBundle, HeptaError> {
        Ok(self
            .intuition_overview(
                session_id,
                user_intent,
                recent_window_limit,
                transcript_limit,
                memory_limit,
                topic_limit,
                neuron_limit,
                skill_limit,
            )?
            .bundle)
    }

    pub fn record_intuition_feedback(
        &self,
        session_id: &str,
        user_intent: &str,
        outcome: IntuitionFeedbackOutcome,
        skill_id: Option<&str>,
        workflow_id: Option<&str>,
        source_topic_ids: Vec<TopicId>,
        source_neuron_ids: Vec<NeuronId>,
        reason: Option<&str>,
    ) -> Result<IntuitionFeedbackRecord, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id cannot be empty".into()));
        }
        if user_intent.trim().is_empty() {
            return Err(HeptaError("user intent cannot be empty".into()));
        }
        let weight_delta = intuition_feedback_weight_delta(outcome);
        let created_at_unix_ms = current_unix_ms()?;
        let records = self.intuition_feedback_for_session(session_id)?;
        let confidence_before = estimate_intuition_feedback_confidence(
            &records,
            &source_topic_ids,
            &source_neuron_ids,
            skill_id,
            workflow_id,
        );
        let confidence_after = (confidence_before + weight_delta).clamp(0.0, 1.0);
        let record = IntuitionFeedbackRecord {
            decision_id: Some(format!(
                "intuition-feedback:{}:{}",
                session_id, created_at_unix_ms
            )),
            surface_session_id: SessionId(session_id.to_string()),
            user_intent: user_intent.trim().to_string(),
            outcome,
            skill_id: skill_id
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_string),
            workflow_id: workflow_id
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_string),
            source_topic_ids,
            source_neuron_ids,
            weight_delta,
            observed_outcome: None,
            latency_ms: None,
            cost: None,
            user_correction: None,
            confidence_before: Some(confidence_before),
            confidence_after: Some(confidence_after),
            reason: reason
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_string),
            created_at_unix_ms,
        };
        self.push_intuition_feedback_record(record.clone())?;
        {
            let mut guard = self
                .topic_session_state
                .lock()
                .map_err(|_| HeptaError("topic session state mutex poisoned".into()))?;
            apply_intuition_feedback_to_topic_sessions(session_id, &record, &mut guard.sessions);
        }
        let updated_neurons = reduce_intuition_feedback_neurons(
            session_id,
            &record,
            self.stored_neurons_for_session(session_id)?,
        );
        self.upsert_neurons_for_session(session_id, updated_neurons)?;
        Ok(record)
    }

    pub fn record_model_router_feedback(
        &self,
        session_id: &str,
        user_intent: &str,
        model: ModelRef,
        outcome: TopicAwareModelFeedbackOutcome,
        topic_ids: Vec<TopicId>,
        latency_ms: Option<u64>,
        cost: Option<f32>,
        safety_score: Option<f32>,
        user_acceptance: Option<f32>,
        reason: Option<&str>,
    ) -> Result<TopicAwareModelFeedbackRecord, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id cannot be empty".into()));
        }
        if user_intent.trim().is_empty() {
            return Err(HeptaError("user intent cannot be empty".into()));
        }
        if !self.providers.contains_model_ref(&model) {
            return Err(HeptaError(format!(
                "unknown model for model-router feedback: {}/{}",
                model.provider, model.model
            )));
        }
        validate_probability_metric("safety_score", safety_score)?;
        validate_probability_metric("user_acceptance", user_acceptance)?;

        let record = TopicAwareModelFeedbackRecord {
            session_id: session_id.to_string(),
            user_intent: user_intent.trim().to_string(),
            model,
            outcome,
            topic_ids,
            weight_delta: outcome.weight_delta(),
            latency_ms,
            cost,
            safety_score: safety_score.map(|value| value.clamp(0.0, 1.0)),
            user_acceptance: user_acceptance.map(|value| value.clamp(0.0, 1.0)),
            reason: reason
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_string),
            created_at_unix_ms: current_unix_ms()?,
        };
        self.push_model_router_feedback_record(record.clone())?;
        Ok(record)
    }

    pub fn model_router_feedback_records(
        &self,
        session_id: &str,
    ) -> Result<Vec<TopicAwareModelFeedbackRecord>, HeptaError> {
        self.model_router_feedback_for_session(session_id)
    }

    pub fn model_router_feedback_summary(
        &self,
        session_id: &str,
    ) -> Result<Vec<TopicAwareModelFeedbackSummary>, HeptaError> {
        let records = self.model_router_feedback_for_session(session_id)?;
        Ok(summarize_topic_aware_model_feedback(&records))
    }

    pub fn provenance_overview(
        &self,
        session_id: &str,
    ) -> Result<RuntimeProvenanceOverview, HeptaError> {
        let session = self.session_snapshot_for_id(session_id)?;
        let topic_sessions = self.topic_session_overview(session_id)?.topic_sessions;

        let recall = self.context_recall_slice(
            session_id,
            session.last_user_intent_summary.as_deref(),
            PROVENANCE_RECALL_RECENT_WINDOW_LIMIT,
            PROVENANCE_RECALL_TRANSCRIPT_LIMIT,
            PROVENANCE_RECALL_MEMORY_LIMIT,
            true,
        )?;
        let recall_inspection = recall.bundle.inspection(ContextRecallAvailability {
            total_recent_entry_count: recall.total_recent_entry_count,
            total_transcript_match_count: recall.transcript_matched_count,
            total_memory_match_count: recall.memory_matched_count,
        });

        let (intuition_transcript_evidence_spans, intuition_foreground_topic_sessions) = session
            .last_user_intent_summary
            .as_deref()
            .map(|intent| {
                self.intuition_overview(
                    session_id,
                    intent,
                    PROVENANCE_RECALL_RECENT_WINDOW_LIMIT,
                    PROVENANCE_RECALL_TRANSCRIPT_LIMIT,
                    PROVENANCE_RECALL_MEMORY_LIMIT,
                    PROVENANCE_INTUITION_TOPIC_LIMIT,
                    PROVENANCE_INTUITION_NEURON_LIMIT,
                    PROVENANCE_INTUITION_SKILL_LIMIT,
                )
                .map(|overview| {
                    (
                        overview.bundle.source_transcript_spans.len(),
                        overview.bundle.foreground_topic_session_ids.len(),
                    )
                })
            })
            .transpose()?
            .unwrap_or((0, 0));

        Ok(provenance_overview_rollup::build(
            provenance_overview_rollup::ProvenanceOverviewInputs {
                session_id: session_id.to_string(),
                last_user_intent_summary: session.last_user_intent_summary,
                topic_sessions,
                recall_ranked_items: recall.bundle.ranked_items.len(),
                recall_low_trust_ranked_items: recall.low_trust_ranked_item_count,
                recall_low_recency_ranked_items: recall.low_recency_ranked_item_count,
                recall_memory_control_omitted_items: recall.memory_control_omitted_count,
                recall_transcript_evidence_spans: recall_inspection.source_transcript_spans.len(),
                recall_omitted_items: recall_inspection.omitted_total_item_count(),
                intuition_transcript_evidence_spans,
                intuition_foreground_topic_sessions,
            },
        ))
    }

    pub fn neuron_lifecycle_overview(
        &self,
        session_id: &str,
    ) -> Result<RuntimeNeuronLifecycleOverview, HeptaError> {
        let topic_sessions = self.topic_session_overview(session_id)?.topic_sessions;
        let stored_neurons = self.stored_neurons_for_session(session_id)?;
        let lifecycle = neuron_lifecycle_health_summary(&topic_sessions, &stored_neurons);

        Ok(RuntimeNeuronLifecycleOverview {
            session_id: session_id.to_string(),
            total_topic_sessions: lifecycle.total_topic_sessions,
            active_topic_sessions: lifecycle.active_topic_sessions,
            stored_neurons: lifecycle.stored_neurons,
            neurons_with_transcript_provenance: lifecycle.neurons_with_transcript_provenance,
            neurons_with_memory_provenance: lifecycle.neurons_with_memory_provenance,
            neurons_with_evidence_digest: lifecycle.neurons_with_evidence_digest,
            v2_compressed_neurons: lifecycle.v2_compressed_neurons,
            neurons_with_skill_priors: lifecycle.neurons_with_skill_priors,
            neurons_with_workflow_priors: lifecycle.neurons_with_workflow_priors,
            neurons_with_typed_links: lifecycle.neurons_with_typed_links,
            intuition_ready_neurons: lifecycle.intuition_ready_neurons,
            lineage_neurons: lifecycle.lineage_neurons,
            merged_neurons: lifecycle.merged_neurons,
            split_neurons: lifecycle.split_neurons,
            superseded_neurons: lifecycle.superseded_neurons,
            aging_neurons: lifecycle.aging_neurons,
            cross_session_stable_neurons: lifecycle.cross_session_stable_neurons,
            cross_session_unstable_neurons: lifecycle.cross_session_unstable_neurons,
            merge_split_lineage_edges: lifecycle.merge_split_lineage_edges,
            average_confidence: lifecycle.average_confidence,
            average_freshness: lifecycle.average_freshness,
            stale_neurons: lifecycle.stale_neurons,
            low_confidence_neurons: lifecycle.low_confidence_neurons,
            low_freshness_neurons: lifecycle.low_freshness_neurons,
            compression_policy_versions: lifecycle.compression_policy_versions,
            neuron_upgrade_ready: lifecycle.neuron_upgrade_ready,
            active_topics_without_neurons: lifecycle.active_topics_without_neurons,
            findings: lifecycle.findings,
            healthy: lifecycle.healthy,
        })
    }

    pub fn intuition_calibration_overview(
        &self,
        session_id: &str,
    ) -> Result<RuntimeIntuitionCalibrationOverview, HeptaError> {
        self.session_snapshot_for_id(session_id)?;
        let mut records = self.intuition_feedback_for_session(session_id)?;
        records.sort_by(|left, right| {
            left.created_at_unix_ms
                .cmp(&right.created_at_unix_ms)
                .then_with(|| left.decision_id.cmp(&right.decision_id))
        });

        let mut outcome_counts = BTreeMap::new();
        let mut positive_feedback_count = 0usize;
        let mut negative_feedback_count = 0usize;
        let mut neutral_feedback_count = 0usize;
        let mut net_weight_delta = 0.0_f32;
        let mut confidence_shift_count = 0usize;
        let mut confidence_shift_total = 0.0_f32;

        for record in &records {
            let outcome = format_intuition_feedback_outcome(record.outcome).to_string();
            *outcome_counts.entry(outcome).or_insert(0) += 1;
            net_weight_delta += record.weight_delta;
            match record.weight_delta.total_cmp(&0.0) {
                std::cmp::Ordering::Greater => positive_feedback_count += 1,
                std::cmp::Ordering::Less => negative_feedback_count += 1,
                std::cmp::Ordering::Equal => neutral_feedback_count += 1,
            }
            if let Some(shift) = intuition_feedback_confidence_shift(record) {
                confidence_shift_count += 1;
                confidence_shift_total += shift;
            }
        }

        let feedback_record_count = records.len();
        let average_weight_delta = average_or_zero(net_weight_delta, feedback_record_count);
        let average_confidence_shift =
            average_or_zero(confidence_shift_total, confidence_shift_count);
        let topic_sessions = self.topic_session_overview(session_id)?.topic_sessions;
        let stored_neurons = self.stored_neurons_for_session(session_id)?;
        let learned_topic_hint_count = topic_sessions
            .iter()
            .filter(|topic_session| {
                topic_session
                    .entities
                    .get(SEMANTIC_ROUTER_LEARNED_KEY)
                    .is_some_and(|value| value == "true")
            })
            .flat_map(|topic_session| topic_session.entities.keys())
            .filter(|key| key.starts_with(BOOTSTRAP_SEMANTIC_HINT_PREFIX))
            .count();
        let learned_neuron_update_count = stored_neurons
            .iter()
            .filter(|neuron| neuron.entity_state.contains_key(FEEDBACK_LEARNER_COUNT_KEY))
            .count();
        let learner_applied_update_count = topic_sessions
            .iter()
            .map(|topic_session| {
                topic_session
                    .entities
                    .get(FEEDBACK_LEARNER_COUNT_KEY)
                    .and_then(|value| value.parse::<usize>().ok())
                    .unwrap_or(0)
            })
            .sum::<usize>()
            + stored_neurons
                .iter()
                .map(|neuron| {
                    neuron
                        .entity_state
                        .get(FEEDBACK_LEARNER_COUNT_KEY)
                        .and_then(|value| value.parse::<usize>().ok())
                        .unwrap_or(0)
                })
                .sum::<usize>();
        let closed_loop_ready = feedback_record_count > 0
            && learner_applied_update_count > 0
            && learned_topic_hint_count > 0
            && learned_neuron_update_count > 0;
        let mut learning_findings = Vec::new();
        if feedback_record_count == 0 {
            learning_findings.push("no feedback records available for calibration learning".into());
        }
        if feedback_record_count > 0 && learned_topic_hint_count == 0 {
            learning_findings.push("feedback has not produced learned topic semantic hints".into());
        }
        if feedback_record_count > 0 && learned_neuron_update_count == 0 {
            learning_findings
                .push("feedback has not updated durable neuron calibration state".into());
        }

        Ok(RuntimeIntuitionCalibrationOverview {
            session_id: session_id.to_string(),
            feedback_record_count,
            learner_applied_update_count,
            learned_topic_hint_count,
            learned_neuron_update_count,
            closed_loop_ready,
            positive_feedback_count,
            negative_feedback_count,
            neutral_feedback_count,
            net_weight_delta,
            average_weight_delta,
            confidence_shift_count,
            average_confidence_shift,
            outcome_counts,
            skill_targets: intuition_calibration_skill_targets(&records)
                .into_iter()
                .map(runtime_intuition_calibration_target)
                .collect(),
            workflow_targets: intuition_calibration_workflow_targets(&records)
                .into_iter()
                .map(runtime_intuition_calibration_target)
                .collect(),
            learning_findings,
            recent_feedback: records
                .iter()
                .rev()
                .take(8)
                .map(intuition_calibration_feedback_summary)
                .map(runtime_intuition_calibration_feedback)
                .collect(),
        })
    }

    pub fn intelligence_eval_overview(
        &self,
        session_id: &str,
        case_limit: usize,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        topic_limit: usize,
        neuron_limit: usize,
        skill_limit: usize,
    ) -> Result<RuntimeIntelligenceEvalOverview, HeptaError> {
        self.intelligence_eval_overview_with_router(
            session_id,
            case_limit,
            recent_window_limit,
            transcript_limit,
            memory_limit,
            topic_limit,
            neuron_limit,
            skill_limit,
            None,
        )
    }

    pub fn knowledge_graph_dry_run_overview(&self) -> MemoryKgWriteCandidateReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_write_candidate_report(&atom_report.atoms, true)
    }

    pub fn knowledge_graph_adapter_dry_run_overview(&self) -> MemoryKgAdapterDryRunReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_adapter_dry_run_report(&atom_report.atoms, true)
    }

    pub fn knowledge_graph_adapter_staging_gate_overview(
        &self,
    ) -> MemoryKgAdapterStagingGateReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_adapter_staging_gate_report(&atom_report.atoms, true)
    }

    pub fn knowledge_graph_adapter_client_overview(&self) -> MemoryKgAdapterClientReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_adapter_client_report(&atom_report.atoms, true)
    }

    pub fn knowledge_graph_adapter_config_env_overview(&self) -> MemoryKgAdapterConfigEnvReport {
        memory_kg_adapter_config_env_report(true)
    }

    pub fn knowledge_graph_recall_plan_overview(&self) -> MemoryKgRecallPlanReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_recall_plan_report(&atom_report.atoms, true)
    }

    pub fn knowledge_graph_context_recall_bridge_overview(
        &self,
    ) -> MemoryKgContextRecallBridgeReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_context_recall_bridge_report(&atom_report.atoms, true)
    }

    pub fn knowledge_graph_recall_evaluation_overview(&self) -> MemoryKgRecallEvaluationReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_recall_evaluation_report(&atom_report.atoms, true)
    }

    pub fn knowledge_graph_context_injection_readiness_overview(
        &self,
    ) -> MemoryKgContextInjectionReadinessReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_context_injection_readiness_report(&atom_report.atoms, true)
    }

    pub fn knowledge_graph_shadow_rank_overview(&self) -> MemoryKgShadowRankReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_shadow_rank_report(&atom_report.atoms, true)
    }

    pub fn knowledge_graph_shadow_rank_comparison_overview(
        &self,
    ) -> MemoryKgShadowRankComparisonReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_shadow_rank_comparison_report(&atom_report.atoms, true)
    }

    pub fn knowledge_graph_shadow_rank_drift_overview(&self) -> MemoryKgShadowRankDriftReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_shadow_rank_drift_report(&atom_report.atoms, true)
    }

    pub fn knowledge_graph_prompt_preview_approval_packet_overview(
        &self,
    ) -> MemoryKgPromptPreviewApprovalPacketReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_prompt_preview_approval_packet_report(&atom_report.atoms, true)
    }

    pub fn knowledge_graph_prompt_preview_operator_evidence_overview(
        &self,
    ) -> MemoryKgPromptPreviewOperatorEvidenceReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_prompt_preview_operator_evidence_report(&atom_report.atoms, true)
    }

    pub fn knowledge_graph_prompt_preview_redaction_diff_overview(
        &self,
    ) -> MemoryKgPromptPreviewRedactionDiffReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_prompt_preview_redaction_diff_report(&atom_report.atoms, true)
    }

    pub fn knowledge_graph_prompt_preview_rollback_kill_switch_overview(
        &self,
    ) -> MemoryKgPromptPreviewRollbackKillSwitchReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_prompt_preview_rollback_kill_switch_report(&atom_report.atoms, true)
    }

    pub fn knowledge_graph_prompt_preview_context_handoff_overview(
        &self,
    ) -> MemoryKgPromptPreviewContextHandoffReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_prompt_preview_context_handoff_report(&atom_report.atoms, true)
    }

    pub fn knowledge_graph_prompt_preview_preflight_overview(
        &self,
    ) -> MemoryKgPromptPreviewPreflightReport {
        let atom_report = memory_atom_pipeline_sample_report(true);
        memory_kg_prompt_preview_preflight_report(&atom_report.atoms, true)
    }

    pub fn intelligence_eval_overview_with_router(
        &self,
        session_id: &str,
        case_limit: usize,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        topic_limit: usize,
        neuron_limit: usize,
        skill_limit: usize,
        semantic_router_id: Option<&str>,
    ) -> Result<RuntimeIntelligenceEvalOverview, HeptaError> {
        let mut user_entries = self
            .transcript_entries_for_session(session_id)?
            .into_iter()
            .filter(|entry| {
                entry.kind == TranscriptEntryKind::Message
                    && entry.role == Some(MessageRole::User)
                    && !entry.content.trim().is_empty()
            })
            .collect::<Vec<_>>();
        user_entries.sort_by_key(|entry| entry.sequence);

        let case_limit = case_limit.max(1);
        let start = user_entries.len().saturating_sub(case_limit);
        let cases = user_entries
            .into_iter()
            .skip(start)
            .map(|entry| {
                self.evaluate_intelligence_replay_case(
                    session_id,
                    &entry,
                    recent_window_limit,
                    transcript_limit,
                    memory_limit,
                    topic_limit,
                    neuron_limit,
                    skill_limit,
                    semantic_router_id,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        let passed_case_count = cases.iter().filter(|case| case.passed).count();
        let total_semantic_expectations = cases
            .iter()
            .map(|case| case.semantic_expectation_count)
            .sum::<usize>();
        let total_semantic_expectations_passed = cases
            .iter()
            .map(|case| case.semantic_expectation_passed_count)
            .sum::<usize>();
        let semantic_score = semantic_score_from_counts(
            total_semantic_expectations_passed,
            total_semantic_expectations,
        );
        let learned_router_case_count = cases
            .iter()
            .filter(|case| case.learned_router_signal_count > 0)
            .count();
        let total_learned_router_signals = cases
            .iter()
            .map(|case| case.learned_router_signal_count)
            .sum::<usize>();
        let total_learned_positive_signals = cases
            .iter()
            .map(|case| case.learned_positive_signal_count)
            .sum::<usize>();
        let total_learned_negative_signals = cases
            .iter()
            .map(|case| case.learned_negative_signal_count)
            .sum::<usize>();
        let aggregate_contrast_focus_counts =
            |value_for_case: &dyn Fn(&RuntimeIntelligenceEvalCase) -> usize| {
                let mut counts = BTreeMap::<String, usize>::new();
                for case in &cases {
                    if let Some(focus) = case.contrast_focus.as_deref() {
                        *counts.entry(focus.to_string()).or_insert(0) += value_for_case(case);
                    }
                }
                counts
            };
        let contrast_focus_case_counts = aggregate_contrast_focus_counts(&|_| 1);
        let contrast_focus_passed_counts =
            aggregate_contrast_focus_counts(&|case| usize::from(case.passed));
        let contrast_focus_signal_counts =
            aggregate_contrast_focus_counts(&|case| case.learned_router_signal_count);
        let contrast_focus_positive_signal_counts =
            aggregate_contrast_focus_counts(&|case| case.learned_positive_signal_count);
        let contrast_focus_negative_signal_counts =
            aggregate_contrast_focus_counts(&|case| case.learned_negative_signal_count);
        let semantic_router_report = hepta_intelligence::SemanticRouterRegistry::new()
            .learned_composition_report_for_router_from_count(
                semantic_router_id,
                learned_router_case_count,
            );
        let semantic_router_id = semantic_router_report.router_id;
        let calibration = self.intuition_calibration_overview(session_id)?;
        Ok(RuntimeIntelligenceEvalOverview {
            session_id: session_id.to_string(),
            evaluated_case_count: cases.len(),
            passed_case_count,
            failed_case_count: cases.len().saturating_sub(passed_case_count),
            semantic_router_id,
            learned_router_case_count,
            total_learned_router_signals,
            total_learned_positive_signals,
            total_learned_negative_signals,
            contrast_focus_case_counts,
            contrast_focus_passed_counts,
            contrast_focus_signal_counts,
            contrast_focus_positive_signal_counts,
            contrast_focus_negative_signal_counts,
            total_recall_ranked_items: cases.iter().map(|case| case.recall_ranked_items).sum(),
            total_transcript_evidence_spans: cases
                .iter()
                .map(|case| case.recall_transcript_evidence_spans)
                .sum(),
            total_active_neurons: cases.iter().map(|case| case.active_neuron_count).sum(),
            total_routed_topics: cases.iter().map(|case| case.routed_topic_count).sum(),
            total_neuron_activations: cases.iter().map(|case| case.neuron_activation_count).sum(),
            total_suggested_skills: cases.iter().map(|case| case.suggested_skill_count).sum(),
            registered_skill_decision_count: cases
                .iter()
                .map(|case| case.registered_skill_decision_count)
                .sum(),
            prepared_skill_decision_count: cases
                .iter()
                .map(|case| case.prepared_skill_decision_count)
                .sum(),
            gated_skill_decision_count: cases
                .iter()
                .map(|case| case.gated_skill_decision_count)
                .sum(),
            total_workflow_priors: cases.iter().map(|case| case.workflow_prior_count).sum(),
            registered_workflow_prior_count: cases
                .iter()
                .map(|case| case.registered_workflow_prior_count)
                .sum(),
            prepared_workflow_prior_count: cases
                .iter()
                .map(|case| case.prepared_workflow_prior_count)
                .sum(),
            gated_workflow_prior_count: cases
                .iter()
                .map(|case| case.gated_workflow_prior_count)
                .sum(),
            feedback_record_count: calibration.feedback_record_count,
            feedback_net_weight_delta: calibration.net_weight_delta,
            calibrated_skill_target_count: calibration.skill_targets.len(),
            calibrated_workflow_target_count: calibration.workflow_targets.len(),
            total_semantic_expectations,
            total_semantic_expectations_passed,
            semantic_score,
            cases,
        })
    }

    fn evaluate_intelligence_replay_case(
        &self,
        session_id: &str,
        entry: &TranscriptEntry,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        topic_limit: usize,
        neuron_limit: usize,
        skill_limit: usize,
        semantic_router_id: Option<&str>,
    ) -> Result<RuntimeIntelligenceEvalCase, HeptaError> {
        let query_text = entry.content.trim().to_string();
        let turn_frame = self.intelligence_turn_frame(
            session_id,
            Some(&query_text),
            recent_window_limit,
            transcript_limit,
            memory_limit,
            true,
        )?;
        let intuition = self.intuition_overview_with_router(
            session_id,
            &query_text,
            recent_window_limit,
            transcript_limit,
            memory_limit,
            topic_limit,
            neuron_limit,
            skill_limit,
            semantic_router_id,
        )?;

        let mut warnings = Vec::new();
        let recall_ranked_items = turn_frame.recall_bundle.ranked_items.len();
        let recall_transcript_evidence_spans = turn_frame.provenance_spans.len();
        let active_neuron_count = turn_frame.active_neurons.len();
        let routed_topic_count = intuition.routed_topic_count;
        let neuron_activation_count = intuition.returned_neuron_activation_count;
        let foreground_topic_session_count = intuition.bundle.foreground_topic_session_ids.len();
        let suggested_skill_count = intuition.bundle.skill_decisions.len();
        let registered_skill_decision_count = intuition
            .bundle
            .skill_decisions
            .iter()
            .filter(|decision| decision.exists_in_registry)
            .count();
        let prepared_skill_decision_count = intuition
            .bundle
            .skill_decisions
            .iter()
            .filter(|decision| decision.action_mode == IntuitionActionMode::Prepare)
            .count();
        let gated_skill_decision_count = intuition
            .bundle
            .skill_decisions
            .iter()
            .filter(|decision| decision.requires_confirmation)
            .count();
        let workflow_prior_count = intuition.bundle.workflow_priors.len();
        let registered_workflow_prior_count = intuition
            .bundle
            .workflow_priors
            .iter()
            .filter(|prior| prior.exists_in_registry)
            .count();
        let prepared_workflow_prior_count = intuition
            .bundle
            .workflow_priors
            .iter()
            .filter(|prior| prior.action_mode == IntuitionActionMode::Prepare)
            .count();
        let gated_workflow_prior_count = intuition
            .bundle
            .workflow_priors
            .iter()
            .filter(|prior| prior.requires_confirmation)
            .count();
        let semantic_eval = evaluate_intelligence_semantic_expectations(
            &query_text,
            recall_ranked_items,
            recall_transcript_evidence_spans,
            routed_topic_count,
            neuron_activation_count,
            suggested_skill_count,
            workflow_prior_count,
            &intuition.bundle,
        );
        let learned_positive_signal_count = intuition
            .learned_router_signals
            .iter()
            .filter(|signal| learned_signal_summary_delta(signal).is_some_and(|delta| delta > 0.0))
            .count();
        let learned_negative_signal_count = intuition
            .learned_router_signals
            .iter()
            .filter(|signal| learned_signal_summary_delta(signal).is_some_and(|delta| delta < 0.0))
            .count();
        let contrast_focus = learned_feedback_contrast_focus(&query_text).map(str::to_string);
        let contrast_expected_signal_direction =
            learned_feedback_contrast_expected_signal_direction(&query_text).map(str::to_string);
        let mut semantic_expectation_count = semantic_eval.expectation_count;
        let mut semantic_failures = semantic_eval.failures;
        if is_learned_feedback_contrast_case(&query_text) {
            semantic_expectation_count += 1;
            if intuition.learned_router_signal_count == 0 {
                semantic_failures.push(
                    "learned-feedback contrast case produced no learned router signal".into(),
                );
            }
            if let Some(expected_direction) = contrast_expected_signal_direction.as_deref() {
                match expected_direction {
                    "positive" => {
                        semantic_expectation_count += 1;
                        if learned_positive_signal_count == 0 {
                            semantic_failures.push(format!(
                                "learned-feedback contrast focus {} produced no positive boost signal",
                                contrast_focus.as_deref().unwrap_or("unknown")
                            ));
                        }
                    }
                    "negative" => {
                        semantic_expectation_count += 1;
                        if learned_negative_signal_count == 0 {
                            semantic_failures.push(format!(
                                "learned-feedback contrast focus {} produced no negative suppression signal",
                                contrast_focus.as_deref().unwrap_or("unknown")
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
        let semantic_expectation_passed_count =
            semantic_expectation_count.saturating_sub(semantic_failures.len());
        let semantic_score = semantic_score_from_counts(
            semantic_expectation_passed_count,
            semantic_expectation_count,
        );

        if recall_ranked_items == 0 {
            warnings.push("recall returned no ranked items".into());
        }
        if recall_transcript_evidence_spans == 0 {
            warnings.push("recall returned no transcript provenance spans".into());
        }
        if routed_topic_count == 0 {
            warnings.push("topic router returned no routed topics".into());
        }
        if neuron_activation_count == 0 {
            warnings.push("neuron activation returned no active neurons".into());
        }
        if suggested_skill_count == 0 {
            warnings.push("intuition returned no skill decisions".into());
        }
        warnings.extend(
            semantic_failures
                .iter()
                .map(|failure| format!("semantic expectation failed: {failure}")),
        );

        Ok(RuntimeIntelligenceEvalCase {
            case_id: format!("{}:{}", session_id, entry.sequence),
            transcript_sequence: entry.sequence,
            query_text,
            router_id: intuition.router_id,
            contrast_focus,
            contrast_expected_signal_direction,
            learned_router_signal_count: intuition.learned_router_signal_count,
            learned_positive_signal_count,
            learned_negative_signal_count,
            recall_ranked_items,
            recall_transcript_evidence_spans,
            active_neuron_count,
            routed_topic_count,
            neuron_activation_count,
            foreground_topic_session_count,
            suggested_skill_count,
            registered_skill_decision_count,
            prepared_skill_decision_count,
            gated_skill_decision_count,
            workflow_prior_count,
            registered_workflow_prior_count,
            prepared_workflow_prior_count,
            gated_workflow_prior_count,
            semantic_expectation_count,
            semantic_expectation_passed_count,
            semantic_score,
            semantic_failures,
            passed: warnings.is_empty(),
            warnings,
        })
    }

    pub async fn intelligence_phase2_gate(
        &self,
        session_id: &str,
    ) -> Result<RuntimeIntelligencePhase2Overview, HeptaError> {
        let session_id = session_id.trim();
        if session_id.is_empty() {
            return Err(HeptaError("session id cannot be empty".into()));
        }

        self.switch_session(session_id)?;
        self.run_demo_turn_in_session(session_id, "hello adaptive memory")
            .await?;
        self.route_topics(session_id, Some("hello adaptive memory"), 8, 8, 8, 3)?;
        self.run_demo_turn_in_session(session_id, "rust worker pipeline")
            .await?;
        self.route_topics(session_id, Some("rust worker pipeline"), 8, 8, 8, 3)?;
        self.route_topics(
            session_id,
            Some("hello adaptive memory and rust worker pipeline"),
            10,
            10,
            10,
            4,
        )?;

        let compressed_neurons = self.compress_active_topics_to_neurons(session_id, 4)?;
        let source_topic_ids = compressed_neurons
            .iter()
            .map(|neuron| neuron.topic_id.clone())
            .collect::<Vec<_>>();
        let source_neuron_ids = compressed_neurons
            .iter()
            .map(|neuron| neuron.neuron_id.clone())
            .collect::<Vec<_>>();
        self.record_intuition_feedback(
            session_id,
            "hello adaptive memory",
            IntuitionFeedbackOutcome::ExecutedSuccess,
            Some("skill-custom:memory-review"),
            Some("workflow:memory-review"),
            source_topic_ids,
            source_neuron_ids,
            Some("phase2 gate positive feedback for blended recall provenance memory semantic router neuron compression"),
        )?;

        let recall =
            self.context_recall_slice(session_id, Some("hello adaptive memory"), 10, 10, 10, true)?;
        let recall_memory_control_omitted_items = recall.memory_control_omitted_count;
        let mut recall_bundle = recall.bundle;
        let active_topic_session_ids = recall_bundle
            .active_topic_sessions
            .iter()
            .map(|topic_session| topic_session.topic_session_id.clone())
            .collect::<Vec<_>>();
        if !active_topic_session_ids.is_empty() {
            let topic_sessions = self.topic_session_overview(session_id)?.topic_sessions;
            let active_neurons = self.resolve_active_neurons_for_routing(
                session_id,
                &topic_sessions,
                &active_topic_session_ids,
                active_topic_session_ids.len(),
            )?;
            attach_active_neurons_to_recall_bundle(&mut recall_bundle, active_neurons);
        }
        let recall_low_trust_ranked_items =
            context_recall_support::low_trust_ranked_item_count(&recall_bundle.ranked_items);
        let recall_low_recency_ranked_items =
            context_recall_support::low_recency_ranked_item_count(&recall_bundle.ranked_items);
        let recall_source_count = recall_bundle
            .ranked_items
            .iter()
            .map(|item| format!("{:?}", item.source))
            .collect::<BTreeSet<_>>()
            .len();
        let recall_transcript_evidence_spans = recall_bundle.source_transcript_spans().len();
        let provenance = self.provenance_overview(session_id)?;
        let learned_routing = self.topic_routing_overview_with_router(
            session_id,
            Some("hello adaptive memory"),
            10,
            10,
            10,
            4,
            Some(hepta_intelligence::SEMANTIC_ROUTER_LEARNED_ID),
        )?;
        let lifecycle = self.neuron_lifecycle_overview(session_id)?;
        let supported_semantic_router_count = hepta_intelligence::SemanticRouterRegistry::new()
            .supported_router_ids()
            .len();

        let blended_recall_ready = recall_bundle.ranked_items.len() >= 4
            && recall_source_count >= 4
            && !recall_bundle.recent_entries.is_empty()
            && !recall_bundle.transcript_hits.is_empty()
            && !recall_bundle.durable_memory_hits.is_empty()
            && !recall_bundle.active_neurons.is_empty()
            && recall_transcript_evidence_spans > 0;
        let provenance_memory_ready = provenance.active_topic_sessions > 0
            && provenance.active_topic_sessions_missing_transcript_provenance == 0
            && provenance.recall_transcript_evidence_spans > 0
            && provenance.intuition_transcript_evidence_spans > 0;
        let semantic_router_generalized = supported_semantic_router_count >= 3
            && learned_routing.router_id == hepta_intelligence::SEMANTIC_ROUTER_LEARNED_ID
            && learned_routing.learned_router_signal_count > 0;
        let neuron_compression_ready = !compressed_neurons.is_empty()
            && lifecycle.healthy
            && lifecycle.stored_neurons >= compressed_neurons.len()
            && lifecycle.neurons_with_transcript_provenance >= compressed_neurons.len()
            && lifecycle.neurons_with_evidence_digest >= compressed_neurons.len();

        let gates = vec![
            RuntimeIntelligencePhase2Gate {
                id: "blended_recall".into(),
                title: "Blended recall returns transcript, memory, topic, and neuron evidence"
                    .into(),
                ready: blended_recall_ready,
                evidence: format!(
                    "ranked_items={} source_count={} low_trust={} low_recency={} control_omitted={} transcript_spans={} durable_memory_hits={} active_neurons={}",
                    recall_bundle.ranked_items.len(),
                    recall_source_count,
                    recall_low_trust_ranked_items,
                    recall_low_recency_ranked_items,
                    recall_memory_control_omitted_items,
                    recall_transcript_evidence_spans,
                    recall_bundle.durable_memory_hits.len(),
                    recall_bundle.active_neurons.len()
                ),
            },
            RuntimeIntelligencePhase2Gate {
                id: "provenance_memory".into(),
                title: "Provenance memory keeps topic, recall, and intuition transcript evidence inspectable".into(),
                ready: provenance_memory_ready,
                evidence: format!(
                    "active_topic_sessions={} with_transcript={} recall_spans={} intuition_spans={}",
                    provenance.active_topic_sessions,
                    provenance.active_topic_sessions_with_transcript_provenance,
                    provenance.recall_transcript_evidence_spans,
                    provenance.intuition_transcript_evidence_spans
                ),
            },
            RuntimeIntelligencePhase2Gate {
                id: "semantic_router_generalization".into(),
                title: "Semantic router registry supports bootstrap, learned-feedback, and no-feedback modes".into(),
                ready: semantic_router_generalized,
                evidence: format!(
                    "supported_routers={} selected_router={} learned_signals={}",
                    supported_semantic_router_count,
                    learned_routing.router_id,
                    learned_routing.learned_router_signal_count
                ),
            },
            RuntimeIntelligencePhase2Gate {
                id: "neuron_compression".into(),
                title: "Neuron compression creates durable provenance-backed neurons and lifecycle stays healthy".into(),
                ready: neuron_compression_ready,
                evidence: format!(
                    "compressed={} stored={} transcript_backed={} digest_backed={} healthy={}",
                    compressed_neurons.len(),
                    lifecycle.stored_neurons,
                    lifecycle.neurons_with_transcript_provenance,
                    lifecycle.neurons_with_evidence_digest,
                    lifecycle.healthy
                ),
            },
        ];
        let ready_gate_count = gates.iter().filter(|gate| gate.ready).count();
        let all_phase2_gates_ready = ready_gate_count == gates.len();
        let overall_percent = ((ready_gate_count * 100) / gates.len().max(1)) as u8;
        let findings = gates
            .iter()
            .filter(|gate| !gate.ready)
            .map(|gate| format!("{} not ready: {}", gate.id, gate.evidence))
            .collect::<Vec<_>>();

        Ok(RuntimeIntelligencePhase2Overview {
            session_id: session_id.to_string(),
            phase: "memory-intelligence-phase2".into(),
            status: if all_phase2_gates_ready {
                "complete".into()
            } else {
                "blocked".into()
            },
            overall_percent,
            all_phase2_gates_ready,
            blended_recall_ready,
            provenance_memory_ready,
            semantic_router_generalized,
            neuron_compression_ready,
            recall_ranked_items: recall_bundle.ranked_items.len(),
            recall_source_count,
            recall_low_trust_ranked_items,
            recall_low_recency_ranked_items,
            recall_memory_control_omitted_items,
            recall_transcript_evidence_spans,
            durable_memory_hits: recall_bundle.durable_memory_hits.len(),
            active_neurons: recall_bundle.active_neurons.len(),
            provenance_active_topic_sessions: provenance.active_topic_sessions,
            provenance_topic_sessions_with_transcript: provenance
                .active_topic_sessions_with_transcript_provenance,
            supported_semantic_router_count,
            learned_router_signal_count: learned_routing.learned_router_signal_count,
            compressed_neuron_count: compressed_neurons.len(),
            neurons_with_evidence_digest: lifecycle.neurons_with_evidence_digest,
            gates,
            findings,
        })
    }

    pub fn route_topics(
        &self,
        session_id: &str,
        query_text: Option<&str>,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        topic_limit: usize,
    ) -> Result<TopicRoutingDecision, HeptaError> {
        Ok(self
            .topic_routing_overview(
                session_id,
                query_text,
                recent_window_limit,
                transcript_limit,
                memory_limit,
                topic_limit,
            )?
            .decision)
    }

    pub fn compress_topic_to_neuron(
        &self,
        session_id: &str,
        topic_id: &str,
    ) -> Result<(HeptaNeuron, NeuronCompressionReport), HeptaError> {
        let topic_id = TopicId(topic_id.trim().to_string());
        if topic_id.0.is_empty() {
            return Err(HeptaError("topic id cannot be empty".into()));
        }

        self.compress_topic_to_neuron_for_topic(session_id, &topic_id)
    }

    pub fn compress_active_topics_to_neurons(
        &self,
        session_id: &str,
        limit: usize,
    ) -> Result<Vec<HeptaNeuron>, HeptaError> {
        if limit == 0 {
            return Ok(Vec::new());
        }

        let topic_sessions = self.topic_sessions_for_surface(session_id)?;
        let active_topic_session_ids = topic_sessions
            .iter()
            .filter(|topic_session| matches!(topic_session.status, TopicSessionStatus::Active))
            .map(|topic_session| topic_session.topic_session_id.clone())
            .collect::<Vec<_>>();

        let neurons = compress_bootstrap_topic_session_ids_to_neurons(
            &topic_sessions,
            &active_topic_session_ids,
            limit,
        )?;
        self.upsert_neurons_for_session(session_id, neurons.clone())?;
        Ok(neurons)
    }

    pub fn topic_sessions_for_surface(
        &self,
        session_id: &str,
    ) -> Result<Vec<TopicSession>, HeptaError> {
        Ok(self.topic_session_overview(session_id)?.topic_sessions)
    }

    fn compress_topic_to_neuron_for_topic(
        &self,
        session_id: &str,
        topic_id: &TopicId,
    ) -> Result<(HeptaNeuron, NeuronCompressionReport), HeptaError> {
        let topic_sessions = self.topic_sessions_for_surface(session_id)?;
        let source_topic_sessions = topic_sessions
            .iter()
            .filter(|topic_session| topic_session.topic_id == *topic_id)
            .cloned()
            .collect::<Vec<_>>();

        if source_topic_sessions.is_empty() {
            return Err(HeptaError(format!(
                "no topic session found for '{}' in session '{}'",
                topic_id.0, session_id
            )));
        }

        let (neuron, report) = compress_bootstrap_topic_sessions_to_neuron(
            topic_id,
            &source_topic_sessions,
            &topic_sessions,
        )?;
        self.upsert_neurons_for_session(session_id, vec![neuron.clone()])?;
        Ok((neuron, report))
    }

    pub(crate) fn context_recall_slice(
        &self,
        session_id: &str,
        query_text: Option<&str>,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        allow_cross_session: bool,
    ) -> Result<RuntimeContextRecallSlice, HeptaError> {
        let request = ContextRecallRequest {
            session_id: SessionId(session_id.to_string()),
            query_text: query_text.map(str::to_string),
            recent_window_limit,
            transcript_limit,
            memory_limit,
            allow_cross_session,
        };

        let (recent_entries, total_recent_entry_count) =
            context_recall_support::prepare_recent_entries(
                self.transcript_entries_for_session(session_id)?,
                recent_window_limit,
            );

        let query_text = query_text.filter(|query| !query.is_empty());

        let transcript_report = match query_text {
            Some(query) => self.query_transcript(Some(session_id), query, transcript_limit)?,
            None => transcript_query_support::empty_report(session_id, transcript_limit),
        };

        let memory_report = self
            .memory
            .search_report(MemoryQuery {
                text: query_text.unwrap_or_default().to_string(),
                limit: memory_limit,
            })
            .map_err(|err| HeptaError(err.0))?;
        let active_topic_sessions = self
            .topic_session_overview(session_id)?
            .topic_sessions
            .into_iter()
            .filter(|topic_session| matches!(topic_session.status, TopicSessionStatus::Active))
            .collect::<Vec<_>>();

        Ok(context_recall_support::build_slice(
            context_recall_support::ContextRecallBuildInputs {
                request,
                recent_entries,
                total_recent_entry_count,
                transcript_report,
                memory_report,
                active_topic_sessions,
            },
        ))
    }

    pub fn context_recall_provider_rollup(
        &self,
        session_id: &str,
        query_text: Option<&str>,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        allow_cross_session: bool,
    ) -> Result<RuntimeContextRecallProviderRollup, HeptaError> {
        let recall = self.context_recall_slice(
            session_id,
            query_text,
            recent_window_limit,
            transcript_limit,
            memory_limit,
            allow_cross_session,
        )?;
        Ok(context_recall_provider_rollup::build(&recall))
    }

    pub fn context_recall_selected_snippet_envelope(
        &self,
        session_id: &str,
        query_text: Option<&str>,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        allow_cross_session: bool,
    ) -> Result<RuntimeContextRecallSelectedSnippetEnvelope, HeptaError> {
        let recall = self.context_recall_slice(
            session_id,
            query_text,
            recent_window_limit,
            transcript_limit,
            memory_limit,
            allow_cross_session,
        )?;
        Ok(context_recall_selected_snippet_envelope::build(
            &recall, query_text,
        ))
    }

    pub fn context_recall_turn_handoff(
        &self,
        session_id: &str,
        query_text: Option<&str>,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        allow_cross_session: bool,
        experimental_api_enabled: bool,
    ) -> Result<RuntimeContextRecallTurnHandoff, HeptaError> {
        let recall = self.context_recall_slice(
            session_id,
            query_text,
            recent_window_limit,
            transcript_limit,
            memory_limit,
            allow_cross_session,
        )?;
        Ok(context_recall_turn_handoff::build(
            &recall,
            query_text,
            experimental_api_enabled,
        ))
    }

    pub(crate) fn neuron_activation_overview(
        &self,
        session_id: &str,
        query_text: Option<&str>,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        neuron_limit: usize,
    ) -> Result<RuntimeNeuronActivationOverview, HeptaError> {
        let routing = self.topic_routing_overview(
            session_id,
            query_text,
            recent_window_limit,
            transcript_limit,
            memory_limit,
            neuron_limit,
        )?;
        let topic_sessions = self.topic_session_overview(session_id)?;
        self.neuron_activation_overview_from_routing(
            session_id,
            query_text,
            neuron_limit,
            &routing,
            &topic_sessions,
        )
    }

    fn neuron_activation_overview_from_routing(
        &self,
        session_id: &str,
        query_text: Option<&str>,
        neuron_limit: usize,
        routing: &RuntimeTopicRoutingOverview,
        topic_sessions: &RuntimeTopicSessionOverview,
    ) -> Result<RuntimeNeuronActivationOverview, HeptaError> {
        let recent_entry_count = routing.recent_entry_count;
        let transcript_matched_count = routing.transcript_matched_count;
        let durable_memory_hit_count = routing.durable_memory_hit_count;
        let summary_hit_count = routing.summary_hit_count;
        let active_topic_session_count = routing.decision.active_topic_session_ids.len();
        let routed_topic_count = routing.decision.activation_scores.len();
        let compressed_neurons = if neuron_limit > 0 {
            self.resolve_active_neurons_for_routing(
                session_id,
                &topic_sessions.topic_sessions,
                &routing.decision.active_topic_session_ids,
                neuron_limit,
            )?
        } else {
            Vec::new()
        };

        let activations = compute_neuron_activations(NeuronActivationInput {
            query_text,
            topic_sessions: &topic_sessions.topic_sessions,
            neurons: &compressed_neurons,
            active_topic_session_ids: &routing.decision.active_topic_session_ids,
            activation_scores: &routing.decision.activation_scores,
            evidence_counts: NeuronActivationEvidenceCounts {
                recent_entry_count,
                transcript_matched_count,
                durable_memory_hit_count,
                summary_hit_count,
            },
            limit: neuron_limit,
        });

        Ok(RuntimeNeuronActivationOverview {
            session_id: session_id.to_string(),
            query_text: query_text.map(str::to_string),
            recent_entry_count,
            transcript_matched_count,
            durable_memory_hit_count,
            summary_hit_count,
            active_topic_session_count,
            routed_topic_count,
            activations,
        })
    }

    fn resolve_active_neurons_for_routing(
        &self,
        session_id: &str,
        topic_sessions: &[TopicSession],
        active_topic_session_ids: &[String],
        limit: usize,
    ) -> Result<Vec<HeptaNeuron>, HeptaError> {
        if limit == 0 {
            return Ok(Vec::new());
        }

        let stored_neurons = self.stored_neurons_for_session(session_id)?;
        let topic_session_by_id = topic_sessions
            .iter()
            .map(|topic_session| (topic_session.topic_session_id.clone(), topic_session))
            .collect::<BTreeMap<_, _>>();
        let mut seen_topic_ids = BTreeSet::new();
        let mut resolved = Vec::new();
        let mut refreshed = Vec::new();

        for topic_session_id in active_topic_session_ids {
            let topic_session = topic_session_by_id.get(topic_session_id).ok_or_else(|| {
                HeptaError(format!(
                    "active topic session '{}' missing during neuron lookup",
                    topic_session_id
                ))
            })?;
            if !seen_topic_ids.insert(topic_session.topic_id.0.clone()) {
                continue;
            }

            if let Some(stored) = stored_neurons
                .iter()
                .find(|neuron| neuron.topic_id == topic_session.topic_id)
                .cloned()
            {
                let source_topic_sessions = topic_sessions
                    .iter()
                    .filter(|candidate| candidate.topic_id == topic_session.topic_id)
                    .cloned()
                    .collect::<Vec<_>>();
                if let Some(refreshed_neuron) = revalidate_bootstrap_neuron_against_topic_sessions(
                    &stored,
                    &source_topic_sessions,
                    topic_sessions,
                )? {
                    refreshed.push(refreshed_neuron.clone());
                    resolved.push(refreshed_neuron);
                } else {
                    resolved.push(stored);
                }
            } else {
                let source_topic_sessions = topic_sessions
                    .iter()
                    .filter(|candidate| candidate.topic_id == topic_session.topic_id)
                    .cloned()
                    .collect::<Vec<_>>();
                let (neuron, _) = compress_bootstrap_topic_sessions_to_neuron(
                    &topic_session.topic_id,
                    &source_topic_sessions,
                    topic_sessions,
                )?;
                refreshed.push(neuron.clone());
                resolved.push(neuron);
            }

            if resolved.len() >= limit {
                break;
            }
        }

        self.upsert_neurons_for_session(session_id, refreshed)?;
        Ok(resolved)
    }

    pub(crate) fn topic_routing_overview(
        &self,
        session_id: &str,
        query_text: Option<&str>,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        topic_limit: usize,
    ) -> Result<RuntimeTopicRoutingOverview, HeptaError> {
        self.topic_routing_overview_with_router(
            session_id,
            query_text,
            recent_window_limit,
            transcript_limit,
            memory_limit,
            topic_limit,
            None,
        )
    }

    fn topic_routing_overview_with_router(
        &self,
        session_id: &str,
        query_text: Option<&str>,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        topic_limit: usize,
        semantic_router_id: Option<&str>,
    ) -> Result<RuntimeTopicRoutingOverview, HeptaError> {
        let session = self.session_snapshot_for_id(session_id)?;
        let recall = self.context_recall_slice(
            session_id,
            query_text,
            recent_window_limit,
            transcript_limit,
            memory_limit,
            true,
        )?;
        let recent_entry_count = recall.recent_entry_count;
        let transcript_matched_count = recall.transcript_matched_count;
        let durable_memory_hit_count = recall.durable_memory_hit_count;
        let summary_hit_count = recall.summary_hit_count;
        let topic_score = compute_bootstrap_topic_score(
            recent_entry_count,
            recall.transcript_returned_count,
            durable_memory_hit_count,
            summary_hit_count,
        );
        let learned_route_planning_signals = if topic_limit > 0
            && semantic_router_id != Some(hepta_intelligence::SEMANTIC_ROUTER_BOOTSTRAP_ID)
            && semantic_router_id != Some(hepta_intelligence::SEMANTIC_ROUTER_NO_FEEDBACK_ID)
        {
            let topic_sessions = self.topic_session_overview(session_id)?.topic_sessions;
            let stored_neurons = self.stored_neurons_for_session(session_id)?;
            let feedback_records = self.intuition_feedback_for_session(session_id)?;
            LearnedSemanticRouterEvidence::new(&topic_sessions, &stored_neurons, &feedback_records)
                .collect_route_planning_signals(query_text)
        } else {
            Vec::new()
        };
        let mut route = self.bootstrap_route_topic_sessions(
            session_id,
            query_text,
            &session,
            recent_entry_count,
            transcript_matched_count,
            durable_memory_hit_count,
            summary_hit_count,
            &recall.transcript_evidence,
            topic_score,
            topic_limit,
            topic_limit > 0,
            learned_route_planning_signals,
            semantic_router_id,
        )?;
        let semantic_router_registry = hepta_intelligence::SemanticRouterRegistry::new();
        let semantic_router_report = if topic_limit > 0 {
            let topic_sessions = self.topic_session_overview(session_id)?.topic_sessions;
            let stored_neurons = self.stored_neurons_for_session(session_id)?;
            let feedback_records = self.intuition_feedback_for_session(session_id)?;
            let evidence = LearnedSemanticRouterEvidence::new(
                &topic_sessions,
                &stored_neurons,
                &feedback_records,
            );
            let report = semantic_router_registry.learned_run_report_for_router(
                semantic_router_id,
                query_text,
                &mut route.activation_scores,
                &evidence,
            );
            apply_topic_route_shell_patch(&mut route, &report.route_shell_patch());
            report.evidence.composition
        } else {
            semantic_router_registry.learned_composition_report_for_router(semantic_router_id, &[])
        };
        let router_id = semantic_router_report.router_id;
        let learned_router_signals = semantic_router_report.learned_router_signals;

        let decision = TopicRoutingDecision {
            router_id: router_id.clone(),
            learned_signal_count: learned_router_signals.len(),
            learned_router_signals: learned_router_signals.clone(),
            surface_session_id: SessionId(session_id.to_string()),
            primary_topic_id: if topic_limit > 0 {
                route.primary_topic_id.clone()
            } else {
                None
            },
            source_transcript_spans: recall.transcript_evidence.clone(),
            active_topic_session_ids: if topic_limit > 0 {
                route.active_topic_session_ids.clone()
            } else {
                Vec::new()
            },
            created_topic_session_ids: if topic_limit > 0 {
                route.created_topic_session_ids.clone()
            } else {
                Vec::new()
            },
            revived_topic_session_ids: if topic_limit > 0 {
                route.revived_topic_session_ids.clone()
            } else {
                Vec::new()
            },
            activation_scores: if topic_limit > 0 {
                route.activation_scores.clone()
            } else {
                Vec::new()
            },
            shift_event: Some(route.shift_event.clone()),
            explanation: Some(route.explanation.clone()),
        };

        Ok(RuntimeTopicRoutingOverview {
            session_id: session_id.to_string(),
            query_text: query_text.map(str::to_string),
            router_id,
            learned_router_signal_count: learned_router_signals.len(),
            learned_router_signals,
            recent_entry_count,
            transcript_matched_count,
            durable_memory_hit_count,
            summary_hit_count,
            decision,
        })
    }

    pub(crate) fn intuition_overview(
        &self,
        session_id: &str,
        user_intent: &str,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        topic_limit: usize,
        neuron_limit: usize,
        skill_limit: usize,
    ) -> Result<RuntimeIntuitionOverview, HeptaError> {
        self.intuition_overview_with_router(
            session_id,
            user_intent,
            recent_window_limit,
            transcript_limit,
            memory_limit,
            topic_limit,
            neuron_limit,
            skill_limit,
            None,
        )
    }

    fn intuition_overview_with_router(
        &self,
        session_id: &str,
        user_intent: &str,
        recent_window_limit: usize,
        transcript_limit: usize,
        memory_limit: usize,
        topic_limit: usize,
        neuron_limit: usize,
        skill_limit: usize,
        semantic_router_id: Option<&str>,
    ) -> Result<RuntimeIntuitionOverview, HeptaError> {
        let routing = self.topic_routing_overview_with_router(
            session_id,
            Some(user_intent),
            recent_window_limit,
            transcript_limit,
            memory_limit,
            topic_limit,
            semantic_router_id,
        )?;
        let topic_sessions = self.topic_session_overview(session_id)?;
        let activation = self.neuron_activation_overview_from_routing(
            session_id,
            Some(user_intent),
            neuron_limit,
            &routing,
            &topic_sessions,
        )?;
        let compressed_neurons = if neuron_limit > 0 {
            self.resolve_active_neurons_for_routing(
                session_id,
                &topic_sessions.topic_sessions,
                &routing.decision.active_topic_session_ids,
                neuron_limit,
            )?
        } else {
            Vec::new()
        };
        let active_model = self.model_selection()?.active;
        let registered_tools = self.tools.descriptors();
        let capabilities = registered_tools
            .iter()
            .map(|tool| IntuitionCapabilityView {
                name: tool.name.clone(),
                risk_tier: tool.risk_tier,
                execution_metadata: tool.execution_metadata,
                default_approval_requirement: tool.default_approval_requirement,
            })
            .collect::<Vec<_>>();
        let policy_bindings = registered_tools
            .iter()
            .map(|tool| {
                self.policy
                    .evaluate_with_match(hepta_core::PolicyEvaluationContext {
                        session_id: Some(SessionId(session_id.to_string())),
                        model: Some(active_model.clone()),
                        tool_name: tool.name.clone(),
                        risk_tier: tool.risk_tier,
                    })
                    .map(|decision| IntuitionPolicyBinding {
                        capability_name: tool.name.clone(),
                        requirement: decision.requirement,
                        reason: decision.reason,
                        matched_rule_id: decision.matched_rule_id,
                    })
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| HeptaError(err.0))?;
        let intuition_feedback = self.intuition_feedback_for_session(session_id)?;
        let IntuitionPlan {
            workflow_priors,
            skill_decisions,
        } = plan_intuition(IntuitionPlanInput {
            user_intent,
            topic_scores: &routing.decision.activation_scores,
            activations: &activation.activations,
            compressed_neurons: &compressed_neurons,
            intuition_feedback: &intuition_feedback,
            capabilities: &capabilities,
            policy_bindings: &policy_bindings,
            limit: skill_limit,
        });
        let suggested_skill_count = skill_decisions.len();
        let bundle = IntuitionBundle {
            request: IntuitionRequest {
                surface_session_id: SessionId(session_id.to_string()),
                user_intent: user_intent.to_string(),
                topic_limit,
                neuron_limit,
                skill_limit,
            },
            topic_activation_scores: routing.decision.activation_scores.clone(),
            neuron_activations: activation.activations.clone(),
            source_transcript_spans: aggregate_intuition_transcript_spans(
                &routing.decision,
                &activation.activations,
            ),
            foreground_topic_session_ids: routing.decision.active_topic_session_ids.clone(),
            skill_decisions,
            workflow_priors,
            explanation: Some(format!(
                "bootstrap intuition synthesized {} routed topic(s), {} neuron activation(s), and {} suggested skill(s) for '{}'",
                routing.decision.activation_scores.len(),
                activation.activations.len(),
                suggested_skill_count,
                user_intent,
            )),
            truncated: false,
        };

        Ok(RuntimeIntuitionOverview {
            session_id: session_id.to_string(),
            user_intent: user_intent.to_string(),
            router_id: routing.router_id,
            learned_router_signal_count: routing.learned_router_signal_count,
            learned_router_signals: routing.learned_router_signals,
            recent_entry_count: routing.recent_entry_count,
            transcript_matched_count: routing.transcript_matched_count,
            durable_memory_hit_count: routing.durable_memory_hit_count,
            summary_hit_count: routing.summary_hit_count,
            active_topic_session_count: routing.decision.active_topic_session_ids.len(),
            routed_topic_count: routing.decision.activation_scores.len(),
            returned_neuron_activation_count: activation.activations.len(),
            bundle,
        })
    }

    pub(crate) fn topic_session_overview(
        &self,
        session_id: &str,
    ) -> Result<RuntimeTopicSessionOverview, HeptaError> {
        let session_guard = self
            .topic_session_state
            .lock()
            .map_err(|_| HeptaError("topic session state mutex poisoned".into()))?;
        let graph_guard = self
            .topic_graph_state
            .lock()
            .map_err(|_| HeptaError("topic graph state mutex poisoned".into()))?;
        let mut sessions = session_guard
            .sessions
            .iter()
            .filter(|topic_session| {
                topic_session
                    .linked_surface_session_ids
                    .iter()
                    .any(|linked| linked.0 == session_id)
            })
            .map(|topic_session| hydrate_topic_session_graph_edges(topic_session, &graph_guard))
            .collect::<Vec<_>>();
        sessions.sort_by(|left, right| {
            right
                .last_active_unix_ms
                .cmp(&left.last_active_unix_ms)
                .then_with(|| left.topic_session_id.cmp(&right.topic_session_id))
        });

        Ok(RuntimeTopicSessionOverview {
            session_id: session_id.to_string(),
            topic_sessions: sessions,
        })
    }

    fn bootstrap_route_topic_sessions(
        &self,
        session_id: &str,
        query_text: Option<&str>,
        session: &SessionSnapshot,
        recent_entry_count: usize,
        transcript_matched_count: usize,
        durable_memory_hit_count: usize,
        summary_hit_count: usize,
        transcript_evidence: &[TranscriptSpanRef],
        topic_score: f32,
        topic_limit: usize,
        persist: bool,
        learned_route_planning_signals: Vec<hepta_intelligence::LearnedSemanticRouterSignal>,
        semantic_router_id: Option<&str>,
    ) -> Result<BootstrapTopicRouteOutcome, HeptaError> {
        let now = current_unix_ms()?;
        let mut guard = self
            .topic_session_state
            .lock()
            .map_err(|_| HeptaError("topic session state mutex poisoned".into()))?;
        let mut graph_guard = self
            .topic_graph_state
            .lock()
            .map_err(|_| HeptaError("topic graph state mutex poisoned".into()))?;
        let projected_sessions =
            project_topic_sessions_with_graph_edges(&guard.sessions, &graph_guard);
        let read_stage = bootstrap_route_stage::prepare_bootstrap_topic_route_read_stage(
            &projected_sessions,
            session_id,
            query_text,
            session,
            recent_entry_count,
            transcript_matched_count,
            durable_memory_hit_count,
            summary_hit_count,
            topic_score,
            topic_limit,
            learned_route_planning_signals,
            semantic_router_id,
        );

        let apply_stage = bootstrap_route_stage::build_bootstrap_topic_route_apply_stage(
            read_stage,
            session_id,
            topic_label_for_session(session),
            recent_entry_count,
            transcript_matched_count,
            durable_memory_hit_count,
            summary_hit_count,
        );

        if persist {
            persist_bootstrap_topic_routes(
                &mut guard.sessions,
                &mut graph_guard,
                &apply_stage.session_indices,
                &apply_stage.selected_existing_indices,
                &apply_stage.merged_source_indices,
                &apply_stage.routes,
                apply_stage.outcome.shift_event.kind,
                session_id,
                recent_entry_count,
                durable_memory_hit_count,
                transcript_evidence,
                now,
            );
        }

        Ok(apply_stage.outcome)
    }

    fn transcript_entries_for_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<TranscriptEntry>, HeptaError> {
        let mut items = self
            .memory
            .list_transcript_entries()
            .map_err(|err| HeptaError(err.0))?
            .into_iter()
            .filter(|entry| entry.session_id.0 == session_id)
            .collect::<Vec<_>>();
        items.sort_by_key(|entry| entry.sequence);

        if items.is_empty() {
            items = self.legacy_transcript_entries(Some(session_id))?;
        }

        Ok(items)
    }

    fn legacy_transcript_entries(
        &self,
        session_id: Option<&str>,
    ) -> Result<Vec<TranscriptEntry>, HeptaError> {
        let guard = self
            .history_state
            .lock()
            .map_err(|_| HeptaError("history state mutex poisoned".into()))?;
        let mut sequence = 0_u64;
        let mut entries = Vec::new();

        for turn in guard
            .iter()
            .filter(|turn| session_id.map(|id| turn.session_id == id).unwrap_or(true))
        {
            sequence += 1;
            entries.push(TranscriptEntry {
                entry_id: format!("{}-{}-legacy-user", turn.session_id, sequence),
                session_id: SessionId(turn.session_id.clone()),
                sequence,
                kind: TranscriptEntryKind::Message,
                role: Some(MessageRole::User),
                content: turn.input.clone(),
                created_at_unix_ms: 0,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            });

            sequence += 1;
            entries.push(TranscriptEntry {
                entry_id: format!("{}-{}-legacy-assistant", turn.session_id, sequence),
                session_id: SessionId(turn.session_id.clone()),
                sequence,
                kind: TranscriptEntryKind::Message,
                role: Some(MessageRole::Assistant),
                content: turn.final_text.clone(),
                created_at_unix_ms: 0,
                tool_name: turn.invoked_tool.clone(),
                correlation_id: None,
                summary_of_range: None,
            });

            if let Some(reason) = &turn.blocked_reason {
                sequence += 1;
                entries.push(TranscriptEntry {
                    entry_id: format!("{}-{}-legacy-event", turn.session_id, sequence),
                    session_id: SessionId(turn.session_id.clone()),
                    sequence,
                    kind: TranscriptEntryKind::Event,
                    role: None,
                    content: format!("blocked_reason:{}", reason),
                    created_at_unix_ms: 0,
                    tool_name: turn.invoked_tool.clone(),
                    correlation_id: None,
                    summary_of_range: Some(TranscriptRange {
                        start_sequence: sequence.saturating_sub(2),
                        end_sequence: sequence,
                    }),
                });
            }
        }

        Ok(entries)
    }
}

fn learned_signal_summary_delta(summary: &str) -> Option<f32> {
    summary.split(':').find_map(|part| {
        let trimmed = part.trim();
        if trimmed.starts_with('+') || trimmed.starts_with('-') {
            trimmed.parse::<f32>().ok()
        } else {
            None
        }
    })
}

fn compute_bootstrap_direct_score(
    recent_entry_count: usize,
    transcript_returned_count: usize,
    durable_memory_hit_count: usize,
    summary_hit_count: usize,
) -> f32 {
    let recent_score = (recent_entry_count.min(2) as f32) * 0.10;
    let transcript_score = (transcript_returned_count.min(2) as f32) * 0.25;
    let durable_score = (durable_memory_hit_count.min(2) as f32) * 0.20;
    let summary_score = (summary_hit_count.min(2) as f32) * 0.10;

    (recent_score + transcript_score + durable_score + summary_score).min(1.0)
}

fn compute_bootstrap_topic_score(
    recent_entry_count: usize,
    transcript_returned_count: usize,
    durable_memory_hit_count: usize,
    summary_hit_count: usize,
) -> f32 {
    let evidence_score = compute_bootstrap_direct_score(
        recent_entry_count,
        transcript_returned_count,
        durable_memory_hit_count,
        summary_hit_count,
    );

    if evidence_score > 0.0 {
        evidence_score
    } else {
        0.05
    }
}

fn aggregate_intuition_transcript_spans(
    decision: &TopicRoutingDecision,
    activations: &[NeuronActivation],
) -> Vec<TranscriptSpanRef> {
    let mut refs = Vec::new();
    for span in &decision.source_transcript_spans {
        upsert_bootstrap_transcript_span_ref(&mut refs, span.clone());
    }
    for activation in activations {
        for span in &activation.source_transcript_spans {
            upsert_bootstrap_transcript_span_ref(&mut refs, span.clone());
        }
    }
    refs.sort_by(|left, right| {
        right
            .range
            .end_sequence
            .cmp(&left.range.end_sequence)
            .then_with(|| right.range.start_sequence.cmp(&left.range.start_sequence))
            .then_with(|| left.session_id.0.cmp(&right.session_id.0))
    });
    refs.truncate(MAX_BOOTSTRAP_TRANSCRIPT_SPAN_REFS);
    refs
}

fn apply_topic_route_shell_patch(
    route: &mut BootstrapTopicRouteOutcome,
    patch: &TopicRouteShellPatch,
) {
    route.primary_topic_id = patch.primary_topic_id.clone();
    route.shift_event.to_topic_id = patch.shift_to_topic_id.clone();
    if let Some(shift_reason) = patch.shift_reason.as_deref() {
        route.shift_event.reason = Some(shift_reason.to_string());
    }
    if let Some(explanation) = patch.explanation_replacement.as_deref() {
        route.explanation = explanation.to_string();
    }
    if let Some(suffix) = patch.explanation_suffix.as_deref() {
        route.explanation = format!("{}; {}", route.explanation, suffix);
    }
}

fn runtime_intuition_calibration_target(
    summary: IntuitionCalibrationTargetSummary,
) -> RuntimeIntuitionCalibrationTarget {
    RuntimeIntuitionCalibrationTarget {
        target_kind: summary.target_kind,
        target_id: summary.target_id,
        feedback_count: summary.feedback_count,
        positive_feedback_count: summary.positive_feedback_count,
        negative_feedback_count: summary.negative_feedback_count,
        neutral_feedback_count: summary.neutral_feedback_count,
        net_weight_delta: summary.net_weight_delta,
        average_weight_delta: summary.average_weight_delta,
        confidence_shift_count: summary.confidence_shift_count,
        average_confidence_shift: summary.average_confidence_shift,
        last_feedback_unix_ms: summary.last_feedback_unix_ms,
        outcome_counts: summary.outcome_counts,
        source_topic_ids: summary.source_topic_ids,
        source_neuron_ids: summary.source_neuron_ids,
        latest_reason: summary.latest_reason,
    }
}

fn runtime_intuition_calibration_feedback(
    summary: IntuitionCalibrationFeedbackSummary,
) -> RuntimeIntuitionCalibrationFeedback {
    RuntimeIntuitionCalibrationFeedback {
        decision_id: summary.decision_id,
        user_intent: summary.user_intent,
        outcome: summary.outcome,
        skill_id: summary.skill_id,
        workflow_id: summary.workflow_id,
        weight_delta: summary.weight_delta,
        created_at_unix_ms: summary.created_at_unix_ms,
        reason: summary.reason,
    }
}

fn average_or_zero(total: f32, count: usize) -> f32 {
    if count == 0 {
        0.0
    } else {
        total / count as f32
    }
}

fn validate_probability_metric(name: &str, value: Option<f32>) -> Result<(), HeptaError> {
    if let Some(value) = value {
        if !(0.0..=1.0).contains(&value) {
            return Err(HeptaError(format!("{} must be between 0.0 and 1.0", name)));
        }
    }
    Ok(())
}

fn compress_bootstrap_topic_sessions_to_neuron(
    topic_id: &TopicId,
    source_topic_sessions: &[TopicSession],
    all_topic_sessions: &[TopicSession],
) -> Result<(HeptaNeuron, NeuronCompressionReport), HeptaError> {
    let now = current_unix_ms()?;
    let mut ordered_sessions = source_topic_sessions.to_vec();
    ordered_sessions.sort_by(|left, right| {
        right
            .last_active_unix_ms
            .cmp(&left.last_active_unix_ms)
            .then_with(|| left.topic_session_id.cmp(&right.topic_session_id))
    });

    let primary = ordered_sessions.first().cloned().ok_or_else(|| {
        HeptaError(format!(
            "no source topic session available for '{}'",
            topic_id.0
        ))
    })?;

    let mut linked_session_ids = Vec::new();
    let mut linked_topic_session_ids = Vec::new();
    let mut important_transcript_spans = Vec::new();
    let mut promoted_memory_refs = Vec::new();
    let mut entity_state = BTreeMap::new();
    let mut open_loops = Vec::new();
    let component_topic_sessions =
        collect_bootstrap_component_topic_sessions(&ordered_sessions, all_topic_sessions);

    for topic_session in &ordered_sessions {
        linked_topic_session_ids.push(topic_session.topic_session_id.clone());

        for linked_session_id in &topic_session.linked_surface_session_ids {
            if !linked_session_ids.contains(linked_session_id) {
                linked_session_ids.push(linked_session_id.clone());
            }
        }

        merge_bootstrap_topic_session_transcript_evidence(
            &mut important_transcript_spans,
            &topic_session.linked_transcript_spans,
        );

        for memory_ref in &topic_session.durable_memory_refs {
            if !promoted_memory_refs.contains(memory_ref) {
                promoted_memory_refs.push(memory_ref.clone());
            }
        }

        for open_loop in &topic_session.open_loops {
            if !open_loops.contains(open_loop) {
                open_loops.push(open_loop.clone());
            }
        }
    }

    for topic_session in &component_topic_sessions {
        merge_bootstrap_topic_session_transcript_evidence(
            &mut important_transcript_spans,
            &topic_session.linked_transcript_spans,
        );

        for memory_ref in &topic_session.durable_memory_refs {
            if !promoted_memory_refs.contains(memory_ref) {
                promoted_memory_refs.push(memory_ref.clone());
            }
        }

        for open_loop in &topic_session.open_loops {
            if !open_loops.contains(open_loop) {
                open_loops.push(open_loop.clone());
            }
        }
    }

    for topic_session in ordered_sessions.iter().rev() {
        for (key, value) in &topic_session.entities {
            entity_state.insert(key.clone(), value.clone());
        }
    }

    for topic_session in component_topic_sessions.iter().rev() {
        for (key, value) in &topic_session.entities {
            entity_state
                .entry(key.clone())
                .or_insert_with(|| value.clone());
        }
    }

    let links = build_bootstrap_neuron_links(topic_id, &ordered_sessions, all_topic_sessions);
    let merged_neuron_ids =
        collect_bootstrap_merged_neuron_ids(topic_id, &ordered_sessions, all_topic_sessions);
    let last_revalidated_unix_ms = ordered_sessions
        .iter()
        .map(|topic_session| topic_session.last_active_unix_ms)
        .max()
        .unwrap_or(now);
    let freshness = compute_bootstrap_neuron_freshness(last_revalidated_unix_ms, now);
    let confidence = compute_bootstrap_neuron_confidence(
        important_transcript_spans.len(),
        promoted_memory_refs.len(),
        open_loops.len(),
        links.len(),
        linked_session_ids.len(),
    );
    let stable_preferences = derive_bootstrap_stable_preferences(&entity_state);
    let workflow_score = ((confidence + freshness) / 2.0).clamp(0.0, 1.0);
    let skill_score = ((confidence * 0.65) + (freshness * 0.35)).clamp(0.0, 1.0);
    let source_evidence_digest = format!(
        "topic:{}:sessions:{}:spans:{}:memories:{}",
        topic_id.0,
        linked_topic_session_ids.len(),
        important_transcript_spans.len(),
        promoted_memory_refs.len(),
    );

    let neuron = HeptaNeuron {
        neuron_id: bootstrap_neuron_id(topic_id),
        topic_id: topic_id.clone(),
        topic_label: primary.topic_label.clone(),
        topic_embedding_centroid: compute_bootstrap_topic_embedding_centroid(&ordered_sessions),
        linked_session_ids,
        linked_topic_session_ids: linked_topic_session_ids.clone(),
        important_transcript_spans: important_transcript_spans.clone(),
        promoted_memory_refs: promoted_memory_refs.clone(),
        entity_state,
        stable_preferences,
        open_loops,
        skill_priors: vec![SkillPrior {
            skill_id: format!("skill-bootstrap:{}:followup", topic_id.0),
            score: skill_score,
            source_topic_ids: vec![topic_id.clone()],
            source_neuron_ids: vec![bootstrap_neuron_id(topic_id)],
            reason: Some(format!(
                "bootstrap compression derived a reusable follow-up prior for '{}' from {} topic session(s)",
                primary.topic_label.0,
                linked_topic_session_ids.len(),
            )),
        }],
        workflow_priors: vec![WorkflowPrior {
            workflow_id: format!("workflow-bootstrap:{}", topic_id.0),
            score: workflow_score,
            exists_in_registry: false,
            missing_capability: Some("workflow_registry_binding_pending".into()),
            requires_confirmation: true,
            action_mode: IntuitionActionMode::SuggestOnly,
            source_topic_ids: vec![topic_id.clone()],
            source_neuron_ids: vec![bootstrap_neuron_id(topic_id)],
            reason: Some(format!(
                "bootstrap compression retained workflow continuity for '{}' with confidence {:.2} and freshness {:.2}",
                primary.topic_label.0, confidence, freshness,
            )),
        }],
        links,
        neuron_revision: 1,
        compression_policy_version: MEMORY_NEURON_COMPRESSION_V2_POLICY.into(),
        source_evidence_digest: Some(source_evidence_digest.clone()),
        last_refresh_reason: Some("memory_neuron_compression_v2".into()),
        staleness_score: (1.0 - freshness).clamp(0.0, 1.0),
        merged_from: Vec::new(),
        split_from: Vec::new(),
        supersedes: Vec::new(),
        confidence,
        freshness,
        last_revalidated_unix_ms,
    };

    let report = NeuronCompressionReport {
        topic_id: topic_id.clone(),
        created_neuron_id: Some(neuron.neuron_id.clone()),
        compression_policy_version: neuron.compression_policy_version.clone(),
        source_evidence_digest: neuron.source_evidence_digest.clone(),
        source_topic_session_ids: linked_topic_session_ids.clone(),
        merged_neuron_ids: merged_neuron_ids.clone(),
        linked_session_count: neuron.linked_session_ids.len(),
        important_span_count: neuron.important_transcript_spans.len(),
        promoted_memory_count: neuron.promoted_memory_refs.len(),
        stable_preference_count: neuron.stable_preferences.len(),
        open_loop_count: neuron.open_loops.len(),
        skill_prior_count: neuron.skill_priors.len(),
        workflow_prior_count: neuron.workflow_priors.len(),
        typed_link_count: neuron.links.len(),
        lineage_edge_count: merged_neuron_ids.len()
            + neuron.split_from.len()
            + neuron.supersedes.len(),
        confidence: neuron.confidence,
        freshness: neuron.freshness,
        staleness_score: neuron.staleness_score,
        provenance_complete: !neuron.important_transcript_spans.is_empty()
            && !neuron.promoted_memory_refs.is_empty()
            && neuron.source_evidence_digest.is_some(),
        intuition_ready: !neuron.skill_priors.is_empty()
            && !neuron.workflow_priors.is_empty()
            && neuron.confidence > 0.0,
        reason: Some(format!(
            "memory neuron compression v2 folded {} topic session(s) into neuron '{}' with {} transcript span(s), {} memory ref(s), and {} typed link(s)",
            linked_topic_session_ids.len(),
            neuron.neuron_id.0,
            neuron.important_transcript_spans.len(),
            neuron.promoted_memory_refs.len(),
            neuron.links.len(),
        )),
    };

    Ok((neuron, report))
}

fn compress_bootstrap_topic_session_ids_to_neurons(
    topic_sessions: &[TopicSession],
    active_topic_session_ids: &[String],
    limit: usize,
) -> Result<Vec<HeptaNeuron>, HeptaError> {
    if limit == 0 {
        return Ok(Vec::new());
    }

    let topic_session_by_id = topic_sessions
        .iter()
        .cloned()
        .map(|topic_session| (topic_session.topic_session_id.clone(), topic_session))
        .collect::<BTreeMap<_, _>>();
    let mut seen_topic_ids = BTreeSet::new();
    let mut neurons = Vec::new();

    for topic_session_id in active_topic_session_ids {
        let topic_session = topic_session_by_id.get(topic_session_id).ok_or_else(|| {
            HeptaError(format!(
                "active topic session '{}' missing during neuron compression",
                topic_session_id
            ))
        })?;

        if !seen_topic_ids.insert(topic_session.topic_id.0.clone()) {
            continue;
        }

        let source_topic_sessions = topic_sessions
            .iter()
            .filter(|candidate| candidate.topic_id == topic_session.topic_id)
            .cloned()
            .collect::<Vec<_>>();
        let (neuron, _) = compress_bootstrap_topic_sessions_to_neuron(
            &topic_session.topic_id,
            &source_topic_sessions,
            topic_sessions,
        )?;
        neurons.push(neuron);

        if neurons.len() >= limit {
            break;
        }
    }

    Ok(neurons)
}

fn build_bootstrap_neuron_links(
    topic_id: &TopicId,
    source_topic_sessions: &[TopicSession],
    all_topic_sessions: &[TopicSession],
) -> Vec<NeuronLink> {
    let session_topic_lookup = all_topic_sessions
        .iter()
        .map(|topic_session| {
            (
                topic_session.topic_session_id.clone(),
                topic_session.topic_id.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let source_neuron_id = bootstrap_neuron_id(topic_id);
    let mut links = Vec::new();

    for topic_session in source_topic_sessions {
        for edge in &topic_session.graph_edges {
            let target_neuron_id = session_topic_lookup
                .get(&edge.target_topic_session_id)
                .map(bootstrap_neuron_id)
                .unwrap_or_else(|| {
                    NeuronId(format!(
                        "neuron-linked-{}",
                        slugify_identifier(&edge.target_topic_session_id)
                    ))
                });

            if target_neuron_id == source_neuron_id {
                continue;
            }

            let relation = edge.relation.clone();
            let kind = bootstrap_neuron_link_kind(edge.kind);
            let polarity = bootstrap_neuron_link_polarity(kind);
            let directional = matches!(
                edge.kind,
                TopicGraphEdgeKind::SplitComponent
                    | TopicGraphEdgeKind::MergedInto
                    | TopicGraphEdgeKind::HasComponent
                    | TopicGraphEdgeKind::TemporalContinuation
                    | TopicGraphEdgeKind::Conflict
            );
            let activation_decay = match kind {
                NeuronLinkKind::Conflict | NeuronLinkKind::Inhibition => 0.9,
                _ => 1.0,
            };
            let link = NeuronLink {
                target_neuron_id,
                kind,
                relation,
                polarity,
                directional,
                strength: edge.weight.clamp(0.0, 1.0),
                activation_decay,
                evidence_count: edge.evidence_count,
                last_confirmed_unix_ms: edge.last_confirmed_unix_ms,
            };

            upsert_bootstrap_neuron_link(&mut links, link);
        }
    }

    links.sort_by(|left, right| {
        right
            .strength
            .total_cmp(&left.strength)
            .then_with(|| left.target_neuron_id.0.cmp(&right.target_neuron_id.0))
            .then_with(|| format!("{:?}", left.kind).cmp(&format!("{:?}", right.kind)))
    });
    links
}

fn collect_bootstrap_component_topic_sessions(
    source_topic_sessions: &[TopicSession],
    all_topic_sessions: &[TopicSession],
) -> Vec<TopicSession> {
    let lookup = all_topic_sessions
        .iter()
        .map(|topic_session| {
            (
                topic_session.topic_session_id.clone(),
                topic_session.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut component_topic_sessions = Vec::new();

    for topic_session in source_topic_sessions {
        for edge in &topic_session.graph_edges {
            if !matches!(
                edge.kind,
                TopicGraphEdgeKind::HasComponent
                    | TopicGraphEdgeKind::MergedInto
                    | TopicGraphEdgeKind::SplitComponent
            ) {
                continue;
            }

            let Some(component_topic_session) = lookup.get(&edge.target_topic_session_id) else {
                continue;
            };
            if source_topic_sessions
                .iter()
                .any(|source| source.topic_session_id == component_topic_session.topic_session_id)
                || component_topic_sessions
                    .iter()
                    .any(|existing: &TopicSession| {
                        existing.topic_session_id == component_topic_session.topic_session_id
                    })
            {
                continue;
            }

            component_topic_sessions.push(component_topic_session.clone());
        }
    }

    component_topic_sessions
}

fn collect_bootstrap_merged_neuron_ids(
    topic_id: &TopicId,
    source_topic_sessions: &[TopicSession],
    all_topic_sessions: &[TopicSession],
) -> Vec<NeuronId> {
    let source_neuron_id = bootstrap_neuron_id(topic_id);
    let session_topic_lookup = all_topic_sessions
        .iter()
        .map(|topic_session| {
            (
                topic_session.topic_session_id.clone(),
                topic_session.topic_id.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut merged_neuron_ids = Vec::new();

    for topic_session in source_topic_sessions {
        for edge in &topic_session.graph_edges {
            if !matches!(
                edge.kind,
                TopicGraphEdgeKind::MergedInto | TopicGraphEdgeKind::HasComponent
            ) {
                continue;
            }

            let Some(target_topic_id) = session_topic_lookup.get(&edge.target_topic_session_id)
            else {
                continue;
            };
            let target_neuron_id = bootstrap_neuron_id(target_topic_id);
            if target_neuron_id == source_neuron_id || merged_neuron_ids.contains(&target_neuron_id)
            {
                continue;
            }
            merged_neuron_ids.push(target_neuron_id);
        }
    }

    merged_neuron_ids.sort_by(|left, right| left.0.cmp(&right.0));
    merged_neuron_ids
}

fn compute_bootstrap_topic_embedding_centroid(topic_sessions: &[TopicSession]) -> Option<Vec<f32>> {
    let first_len = topic_sessions
        .iter()
        .find_map(|topic_session| topic_session.topic_embedding.as_ref().map(Vec::len))?;
    let vectors = topic_sessions
        .iter()
        .filter_map(|topic_session| topic_session.topic_embedding.as_ref())
        .filter(|embedding| embedding.len() == first_len)
        .collect::<Vec<_>>();

    if vectors.is_empty() {
        return None;
    }

    let mut centroid = vec![0.0_f32; first_len];
    for embedding in &vectors {
        for (index, value) in embedding.iter().enumerate() {
            centroid[index] += *value;
        }
    }

    for value in &mut centroid {
        *value /= vectors.len() as f32;
    }

    Some(centroid)
}

fn compute_bootstrap_neuron_freshness(last_revalidated_unix_ms: u64, now_unix_ms: u64) -> f32 {
    let age_ms = now_unix_ms.saturating_sub(last_revalidated_unix_ms);
    match age_ms {
        0..=3_600_000 => 1.0,
        3_600_001..=86_400_000 => 0.88,
        86_400_001..=604_800_000 => 0.72,
        _ => 0.56,
    }
}

fn compute_bootstrap_neuron_confidence(
    important_span_count: usize,
    promoted_memory_count: usize,
    open_loop_count: usize,
    link_count: usize,
    linked_session_count: usize,
) -> f32 {
    let transcript_score = (important_span_count.min(6) as f32) * 0.06;
    let memory_score = (promoted_memory_count.min(4) as f32) * 0.07;
    let loop_score = (open_loop_count.min(4) as f32) * 0.05;
    let link_score = (link_count.min(4) as f32) * 0.05;
    let session_score = (linked_session_count.min(3) as f32) * 0.04;

    (0.38 + transcript_score + memory_score + loop_score + link_score + session_score)
        .clamp(0.0, 1.0)
}

fn derive_bootstrap_stable_preferences(entity_state: &BTreeMap<String, String>) -> Vec<String> {
    let mut stable_preferences = Vec::new();

    for (key, value) in entity_state {
        let normalized_key = key.to_ascii_lowercase();
        if !(normalized_key.contains("prefer")
            || normalized_key.contains("preference")
            || normalized_key.contains("default")
            || normalized_key.contains("style"))
        {
            continue;
        }

        let preference = format!("{}={}", key, value);
        if !stable_preferences.contains(&preference) {
            stable_preferences.push(preference);
        }
    }

    stable_preferences
}

fn bootstrap_neuron_link_kind(kind: TopicGraphEdgeKind) -> NeuronLinkKind {
    match kind {
        TopicGraphEdgeKind::SemanticSimilarity => NeuronLinkKind::SemanticSimilarity,
        TopicGraphEdgeKind::CoActivation => NeuronLinkKind::WorkflowAdjacency,
        TopicGraphEdgeKind::SplitComponent => NeuronLinkKind::TemporalContinuation,
        TopicGraphEdgeKind::MergedInto => NeuronLinkKind::CausalDependency,
        TopicGraphEdgeKind::HasComponent => NeuronLinkKind::EntityOverlap,
        TopicGraphEdgeKind::TemporalContinuation => NeuronLinkKind::TemporalContinuation,
        TopicGraphEdgeKind::Conflict => NeuronLinkKind::Conflict,
    }
}

fn bootstrap_neuron_link_polarity(kind: NeuronLinkKind) -> LinkPolarity {
    match kind {
        NeuronLinkKind::Conflict | NeuronLinkKind::Inhibition => LinkPolarity::Inhibitory,
        NeuronLinkKind::SemanticSimilarity
        | NeuronLinkKind::EntityOverlap
        | NeuronLinkKind::WorkflowAdjacency
        | NeuronLinkKind::CausalDependency
        | NeuronLinkKind::TemporalContinuation => LinkPolarity::Excitatory,
    }
}

fn upsert_bootstrap_neuron_link(links: &mut Vec<NeuronLink>, candidate: NeuronLink) {
    if let Some(existing) = links.iter_mut().find(|existing| {
        existing.target_neuron_id == candidate.target_neuron_id
            && existing.kind == candidate.kind
            && existing.relation == candidate.relation
    }) {
        existing.strength = existing.strength.max(candidate.strength);
        existing.evidence_count = existing.evidence_count.max(candidate.evidence_count);
        existing.last_confirmed_unix_ms = existing
            .last_confirmed_unix_ms
            .max(candidate.last_confirmed_unix_ms);
        existing.directional |= candidate.directional;
        existing.activation_decay = existing.activation_decay.min(candidate.activation_decay);
        return;
    }

    links.push(candidate);
}

fn detect_bootstrap_merge_marker(query_text: Option<&str>) -> Option<&'static str> {
    let lower = query_text?.to_ascii_lowercase();

    ["merge ", "combine ", "unify ", "consolidate "]
        .into_iter()
        .find(|marker| lower.contains(marker))
}

fn detect_bootstrap_split_marker(query_text: Option<&str>) -> Option<&'static str> {
    let lower = query_text?.to_ascii_lowercase();

    ["split ", "separate ", "break out ", "apart "]
        .into_iter()
        .find(|marker| lower.contains(marker))
}

fn topic_session_label_overlap(left: &TopicSession, right: &TopicSession) -> f32 {
    let left_terms = bootstrap_planner::extract_semantic_terms(&left.topic_label.0, 8);
    let right_terms = bootstrap_planner::extract_semantic_terms(&right.topic_label.0, 8);
    if left_terms.is_empty() || right_terms.is_empty() {
        return 0.0;
    }

    let right_terms = right_terms.into_iter().collect::<BTreeSet<_>>();
    let overlap = left_terms
        .iter()
        .filter(|term| right_terms.contains(term.as_str()))
        .count();

    (overlap as f32 / left_terms.len().max(right_terms.len()) as f32).min(1.0)
}

fn bootstrap_open_loops(recent_entry_count: usize) -> Vec<String> {
    (recent_entry_count > 0)
        .then(|| {
            vec!["bootstrap-topic-session-awaits-real-graph-propagation-and-inhibition".to_string()]
        })
        .unwrap_or_default()
}

const BOOTSTRAP_SEMANTIC_HINT_PREFIX: &str = "bootstrap.semantic_hint:";
const MAX_BOOTSTRAP_SEMANTIC_HINTS: usize = 8;

const MAX_BOOTSTRAP_TRANSCRIPT_SPAN_REFS: usize = 8;

fn upsert_bootstrap_transcript_span_ref(
    refs: &mut Vec<TranscriptSpanRef>,
    incoming: TranscriptSpanRef,
) {
    if let Some(existing) = refs.iter_mut().find(|existing| {
        existing.session_id == incoming.session_id && existing.range == incoming.range
    }) {
        existing.reason = merge_bootstrap_transcript_span_reasons(
            existing.reason.as_deref(),
            incoming.reason.as_deref(),
        );
        return;
    }

    refs.push(incoming);
}

fn merge_bootstrap_transcript_span_reasons(
    existing: Option<&str>,
    incoming: Option<&str>,
) -> Option<String> {
    let mut merged = Vec::new();
    let mut seen = BTreeSet::new();

    for reason in [existing, incoming].into_iter().flatten() {
        for part in reason
            .split(',')
            .map(str::trim)
            .filter(|part| !part.is_empty())
        {
            if seen.insert(part.to_string()) {
                merged.push(part.to_string());
            }
        }
    }

    (!merged.is_empty()).then(|| merged.join(", "))
}

mod bootstrap_graph_persistence;
mod bootstrap_planner;
fn allocate_bootstrap_topic_id(
    session_id: &str,
    candidate_slug: &str,
    has_existing_sessions: bool,
    existing_sessions: &[TopicSession],
) -> TopicId {
    if !has_existing_sessions {
        return TopicId(format!("topic-{}", slugify_identifier(session_id)));
    }

    let scope_slug = slugify_identifier(session_id);
    let candidate_slug = if candidate_slug.is_empty() {
        "topic".to_string()
    } else {
        candidate_slug.to_string()
    };
    let base = format!("topic-{}-{}", scope_slug, candidate_slug);

    allocate_unique_topic_id(base, existing_sessions)
}

fn allocate_bootstrap_topic_session_id(
    session_id: &str,
    candidate_slug: &str,
    has_existing_sessions: bool,
    existing_sessions: &[TopicSession],
) -> String {
    if !has_existing_sessions {
        return bootstrap_topic_session_id(session_id);
    }

    let candidate_slug = if candidate_slug.is_empty() {
        "topic".to_string()
    } else {
        candidate_slug.to_string()
    };
    let base = format!("topic-session-bootstrap:{}:{}", session_id, candidate_slug);

    if existing_sessions
        .iter()
        .all(|topic_session| topic_session.topic_session_id != base)
    {
        return base;
    }

    for suffix in 2.. {
        let candidate = format!("{}-{}", base, suffix);
        if existing_sessions
            .iter()
            .all(|topic_session| topic_session.topic_session_id != candidate)
        {
            return candidate;
        }
    }

    unreachable!("bootstrap topic session id space exhausted")
}

fn allocate_unique_topic_id(base: String, existing_sessions: &[TopicSession]) -> TopicId {
    if existing_sessions
        .iter()
        .all(|topic_session| topic_session.topic_id.0 != base)
    {
        return TopicId(base);
    }

    for suffix in 2.. {
        let candidate = format!("{}-{}", base, suffix);
        if existing_sessions
            .iter()
            .all(|topic_session| topic_session.topic_id.0 != candidate)
        {
            return TopicId(candidate);
        }
    }

    unreachable!("bootstrap topic id space exhausted")
}

fn persist_bootstrap_topic_routes(
    sessions: &mut Vec<TopicSession>,
    topic_graph_state: &mut TopicGraphState,
    session_indices: &[usize],
    selected_existing_indices: &BTreeSet<usize>,
    merged_source_indices: &BTreeSet<usize>,
    routes: &[BootstrapTopicCandidateRoute],
    shift_kind: TopicShiftKind,
    session_id: &str,
    recent_entry_count: usize,
    durable_memory_hit_count: usize,
    transcript_evidence: &[TranscriptSpanRef],
    now: u64,
) {
    let persist_inputs = bootstrap_route_persistence::prepare_bootstrap_topic_route_persist_inputs(
        session_id,
        recent_entry_count,
        durable_memory_hit_count,
        transcript_evidence,
        now,
    );

    bootstrap_route_persistence::apply_bootstrap_topic_session_status_transitions(
        sessions,
        session_indices,
        selected_existing_indices,
        merged_source_indices,
    );
    bootstrap_route_persistence::refresh_existing_bootstrap_topic_sessions(
        sessions,
        routes,
        &persist_inputs,
    );
    bootstrap_route_persistence::materialize_new_bootstrap_topic_sessions(
        sessions,
        routes,
        &persist_inputs,
    );

    bootstrap_graph_persistence::persist_bootstrap_topic_graph_semantics(
        topic_graph_state,
        sessions,
        routes,
        merged_source_indices,
        shift_kind,
        now,
    );
}

fn merge_bootstrap_topic_session_transcript_evidence(
    linked_transcript_spans: &mut Vec<TranscriptSpanRef>,
    transcript_evidence: &[TranscriptSpanRef],
) {
    for transcript_span in transcript_evidence {
        upsert_bootstrap_transcript_span_ref(linked_transcript_spans, transcript_span.clone());
    }

    linked_transcript_spans.sort_by(|left, right| {
        right
            .range
            .end_sequence
            .cmp(&left.range.end_sequence)
            .then_with(|| right.range.start_sequence.cmp(&left.range.start_sequence))
            .then_with(|| left.session_id.0.cmp(&right.session_id.0))
    });
    linked_transcript_spans.truncate(MAX_BOOTSTRAP_TRANSCRIPT_SPAN_REFS);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BootstrapTopicRoutePersistInputs {
    linked_surface_session_id: SessionId,
    linked_transcript_spans: Vec<TranscriptSpanRef>,
    open_loops: Vec<String>,
    durable_memory_refs: Vec<String>,
    now: u64,
}

mod bootstrap_route_persistence;
fn bootstrap_memory_refs(durable_memory_hit_count: usize) -> Vec<String> {
    (0..durable_memory_hit_count)
        .map(|index| format!("bootstrap-memory-ref-{}", index + 1))
        .collect()
}

fn topic_label_for_session(session: &SessionSnapshot) -> String {
    session
        .last_user_intent_summary
        .clone()
        .filter(|summary| !summary.trim().is_empty())
        .unwrap_or_else(|| session.title.clone())
}

mod bootstrap_route_stage;
fn bootstrap_topic_session_id(session_id: &str) -> String {
    format!("topic-session-bootstrap:{}", session_id)
}

fn bootstrap_neuron_id(topic_id: &TopicId) -> NeuronId {
    if let Some(stripped) = topic_id.0.strip_prefix("topic-") {
        NeuronId(format!("neuron-{}", stripped))
    } else {
        NeuronId(format!("neuron-{}", slugify_identifier(&topic_id.0)))
    }
}

fn slugify_identifier(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash && !slug.is_empty() {
            slug.push('-');
            last_was_dash = true;
        }
    }

    slug.trim_matches('-')
        .to_string()
        .chars()
        .collect::<String>()
        .if_empty_then("topic".to_string())
}

trait EmptyStringFallback {
    fn if_empty_then(self, fallback: String) -> String;
}

impl EmptyStringFallback for String {
    fn if_empty_then(self, fallback: String) -> String {
        if self.is_empty() { fallback } else { self }
    }
}

#[cfg(test)]
mod tests {
    include!("query/tests.rs");
}
