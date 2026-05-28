use std::collections::{BTreeMap, BTreeSet};

use hepta_core::{
    ApprovalRequirement, ContextRecallAvailability, ContextRecallBundle, ContextRecallItem,
    ContextRecallRequest, ContextRecallScore, ContextRecallSource, HeptaError, HeptaNeuron,
    IntelligenceTurnFrame, IntuitionActionMode, IntuitionBundle, IntuitionFeedbackOutcome,
    IntuitionFeedbackRecord, IntuitionRequest, LinkPolarity, MEMORY_NEURON_COMPRESSION_V2_POLICY,
    MemoryQuery, MemoryStore, MessageRole, ModelRef, NeuronActivation, NeuronCompressionReport,
    NeuronId, NeuronLink, NeuronLinkKind, RiskTier, SessionId, SkillActivationDecision, SkillPrior,
    TopicActivationScore, TopicGraphEdgeKind, TopicId, TopicLabel, TopicRoutingDecision,
    TopicSession, TopicSessionStatus, TopicShiftEvent, TopicShiftKind, TranscriptEntry,
    TranscriptEntryKind, TranscriptQueryReport, TranscriptRange, TranscriptSpanRef, WorkflowPrior,
};
use hepta_intelligence::{
    IntuitionCalibrationFeedbackSummary, IntuitionCalibrationTargetSummary,
    LearnedSemanticRouterEvidence, MemoryKgAdapterClientReport, MemoryKgAdapterConfigEnvReport,
    MemoryKgAdapterDryRunReport, MemoryKgAdapterStagingGateReport,
    MemoryKgContextInjectionReadinessReport, MemoryKgContextRecallBridgeReport,
    MemoryKgPromptPreviewApprovalPacketReport, MemoryKgPromptPreviewOperatorEvidenceReport,
    MemoryKgPromptPreviewRedactionDiffReport, MemoryKgRecallEvaluationReport,
    MemoryKgRecallPlanReport, MemoryKgShadowRankComparisonReport, MemoryKgShadowRankDriftReport,
    MemoryKgShadowRankReport, MemoryKgWriteCandidateReport, SEMANTIC_ROUTER_LAST_SIGNAL_KEY,
    SEMANTIC_ROUTER_LEARNED_KEY, SEMANTIC_ROUTER_NET_DELTA_KEY, TopicAwareModelFeedbackOutcome,
    TopicAwareModelFeedbackRecord, TopicAwareModelFeedbackSummary, TopicRouteShellPatch,
    compute_intuition_feedback_delta, evaluate_intelligence_semantic_expectations,
    format_intuition_feedback_outcome, intuition_calibration_feedback_summary,
    intuition_calibration_skill_targets, intuition_calibration_workflow_targets,
    intuition_feedback_confidence_shift, is_learned_feedback_contrast_case,
    learned_feedback_contrast_expected_signal_direction, learned_feedback_contrast_focus,
    learned_semantic_terms_for_feedback, memory_atom_pipeline_sample_report,
    memory_kg_adapter_client_report, memory_kg_adapter_config_env_report,
    memory_kg_adapter_dry_run_report, memory_kg_adapter_staging_gate_report,
    memory_kg_context_injection_readiness_report, memory_kg_context_recall_bridge_report,
    memory_kg_prompt_preview_approval_packet_report,
    memory_kg_prompt_preview_operator_evidence_report,
    memory_kg_prompt_preview_redaction_diff_report, memory_kg_recall_evaluation_report,
    memory_kg_recall_plan_report, memory_kg_shadow_rank_comparison_report,
    memory_kg_shadow_rank_drift_report, memory_kg_shadow_rank_report,
    memory_kg_write_candidate_report, neuron_lifecycle_health_summary, semantic_score_from_counts,
    summarize_topic_aware_model_feedback,
};
use serde::{Deserialize, Serialize};

use crate::events::{format_event_record, summarize_line};
use crate::{
    EventRecord, MemorySnapshot, RuntimeKernel, SessionSnapshot, ToolDescriptor, TopicGraphState,
    TurnRecord, current_unix_ms,
};

pub(crate) const PROVENANCE_RECALL_RECENT_WINDOW_LIMIT: usize = 6;
pub(crate) const PROVENANCE_RECALL_TRANSCRIPT_LIMIT: usize = 6;
pub(crate) const PROVENANCE_RECALL_MEMORY_LIMIT: usize = 6;
pub(crate) const PROVENANCE_INTUITION_TOPIC_LIMIT: usize = 3;
pub(crate) const PROVENANCE_INTUITION_NEURON_LIMIT: usize = 3;
pub(crate) const PROVENANCE_INTUITION_SKILL_LIMIT: usize = 3;
const FEEDBACK_LEARNER_COUNT_KEY: &str = "feedback.learning.count";
const FEEDBACK_LEARNER_NET_DELTA_KEY: &str = "feedback.learning.net_weight_delta";
const FEEDBACK_LEARNER_LAST_OUTCOME_KEY: &str = "feedback.learning.last_outcome";

#[cfg(test)]
use self::topic_graph::bootstrap_topic_graph_edge_count;
use self::topic_graph::{
    bootstrap_topic_graph_edge, bootstrap_topic_graph_edge_relation,
    bootstrap_topic_graph_edge_weight, bootstrap_topic_graph_relation_for_shift_kind,
    hydrate_topic_session_graph_edges, project_topic_sessions_with_graph_edges,
    upsert_bootstrap_topic_graph_edge,
};

mod topic_graph {
    use super::{TopicGraphState, topic_session_label_overlap};
    use hepta_core::{TopicGraphEdge, TopicGraphEdgeKind, TopicSession, TopicShiftKind};

    const LEGACY_BOOTSTRAP_TOPIC_GRAPH_EDGE_PREFIX: &str = "bootstrap.graph.edge:";

    fn bootstrap_topic_graph_edge_key(target_topic_session_id: &str) -> String {
        format!(
            "{}{}",
            LEGACY_BOOTSTRAP_TOPIC_GRAPH_EDGE_PREFIX, target_topic_session_id
        )
    }

    fn bootstrap_topic_graph_edge_kind_for_relation(relation: &str) -> TopicGraphEdgeKind {
        match relation {
            "co_activation" => TopicGraphEdgeKind::CoActivation,
            "split_component" => TopicGraphEdgeKind::SplitComponent,
            "merged_into" => TopicGraphEdgeKind::MergedInto,
            "has_component" => TopicGraphEdgeKind::HasComponent,
            "temporal_continuation" => TopicGraphEdgeKind::TemporalContinuation,
            "conflict" => TopicGraphEdgeKind::Conflict,
            _ => TopicGraphEdgeKind::SemanticSimilarity,
        }
    }

    fn bootstrap_topic_graph_relation_label(kind: TopicGraphEdgeKind) -> &'static str {
        match kind {
            TopicGraphEdgeKind::SemanticSimilarity => "semantic_similarity",
            TopicGraphEdgeKind::CoActivation => "co_activation",
            TopicGraphEdgeKind::SplitComponent => "split_component",
            TopicGraphEdgeKind::MergedInto => "merged_into",
            TopicGraphEdgeKind::HasComponent => "has_component",
            TopicGraphEdgeKind::TemporalContinuation => "temporal_continuation",
            TopicGraphEdgeKind::Conflict => "conflict",
        }
    }

    pub(super) fn bootstrap_topic_graph_edge_relation(edge: &TopicGraphEdge) -> &str {
        edge.relation
            .as_deref()
            .unwrap_or_else(|| bootstrap_topic_graph_relation_label(edge.kind))
    }

    fn parse_legacy_bootstrap_topic_graph_edge(key: &str, value: &str) -> Option<TopicGraphEdge> {
        let target_topic_session_id = key.strip_prefix(LEGACY_BOOTSTRAP_TOPIC_GRAPH_EDGE_PREFIX)?;
        let (relation, weight) = value.split_once('|')?;
        let weight = weight.parse::<f32>().ok()?.clamp(0.0, 1.0);

        Some(TopicGraphEdge {
            target_topic_session_id: target_topic_session_id.to_string(),
            kind: bootstrap_topic_graph_edge_kind_for_relation(relation),
            relation: Some(relation.to_string()),
            weight,
            evidence_count: 0,
            last_confirmed_unix_ms: None,
        })
    }

    fn standalone_topic_graph_edges_for_source(
        graph_state: &TopicGraphState,
        source_topic_session_id: &str,
    ) -> Vec<TopicGraphEdge> {
        let mut edges = graph_state
            .edges
            .iter()
            .filter(|record| record.source_topic_session_id == source_topic_session_id)
            .map(|record| record.edge.clone())
            .collect::<Vec<_>>();
        edges.sort_by(|left, right| {
            left.target_topic_session_id
                .cmp(&right.target_topic_session_id)
                .then_with(|| left.weight.total_cmp(&right.weight))
        });
        edges
    }

    pub(super) fn hydrate_topic_session_graph_edges(
        topic_session: &TopicSession,
        graph_state: &TopicGraphState,
    ) -> TopicSession {
        let mut projected = topic_session.clone();
        let standalone_edges =
            standalone_topic_graph_edges_for_source(graph_state, &topic_session.topic_session_id);
        if !standalone_edges.is_empty() {
            projected.graph_edges = standalone_edges;
            return projected;
        }

        if !projected.graph_edges.is_empty() {
            return projected;
        }

        projected.graph_edges = topic_session
            .entities
            .iter()
            .filter_map(|(key, value)| parse_legacy_bootstrap_topic_graph_edge(key, value))
            .collect();
        projected
    }

    pub(super) fn project_topic_sessions_with_graph_edges(
        sessions: &[TopicSession],
        graph_state: &TopicGraphState,
    ) -> Vec<TopicSession> {
        sessions
            .iter()
            .map(|topic_session| hydrate_topic_session_graph_edges(topic_session, graph_state))
            .collect()
    }

    #[cfg(test)]
    pub(super) fn bootstrap_topic_graph_edge_count(topic_session: &TopicSession) -> usize {
        topic_session.graph_edges.len().max(
            topic_session
                .entities
                .keys()
                .filter(|key| key.starts_with(LEGACY_BOOTSTRAP_TOPIC_GRAPH_EDGE_PREFIX))
                .count(),
        )
    }

    pub(super) fn bootstrap_topic_graph_edge(
        source_topic_session: &TopicSession,
        target_topic_session_id: &str,
    ) -> Option<TopicGraphEdge> {
        source_topic_session
            .graph_edges
            .iter()
            .find(|edge| edge.target_topic_session_id == target_topic_session_id)
            .cloned()
            .or_else(|| {
                let key = bootstrap_topic_graph_edge_key(target_topic_session_id);
                source_topic_session
                    .entities
                    .get(&key)
                    .and_then(|value| parse_legacy_bootstrap_topic_graph_edge(&key, value))
            })
    }

    pub(super) fn upsert_bootstrap_topic_graph_edge(
        graph_state: &mut TopicGraphState,
        source_topic_session_id: &str,
        target_topic_session_id: &str,
        kind: TopicGraphEdgeKind,
        weight: f32,
        now: u64,
    ) {
        let new_weight = weight.clamp(0.0, 1.0);
        if let Some(existing) = graph_state.edges.iter_mut().find(|record| {
            record.source_topic_session_id == source_topic_session_id
                && record.edge.target_topic_session_id == target_topic_session_id
        }) {
            if existing.edge.weight <= new_weight {
                existing.edge.kind = kind;
                existing.edge.relation =
                    Some(bootstrap_topic_graph_relation_label(kind).to_string());
                existing.edge.weight = new_weight;
            }
            existing.edge.evidence_count = existing.edge.evidence_count.saturating_add(1);
            existing.edge.last_confirmed_unix_ms = Some(now);
            return;
        }

        graph_state.edges.push(crate::RuntimeTopicGraphEdgeRecord {
            source_topic_session_id: source_topic_session_id.to_string(),
            edge: TopicGraphEdge {
                target_topic_session_id: target_topic_session_id.to_string(),
                kind,
                relation: Some(bootstrap_topic_graph_relation_label(kind).to_string()),
                weight: new_weight,
                evidence_count: 1,
                last_confirmed_unix_ms: Some(now),
            },
        });
    }

    pub(super) fn bootstrap_topic_graph_relation_for_shift_kind(
        shift_kind: TopicShiftKind,
    ) -> TopicGraphEdgeKind {
        if matches!(shift_kind, TopicShiftKind::Split) {
            TopicGraphEdgeKind::SplitComponent
        } else {
            TopicGraphEdgeKind::CoActivation
        }
    }

    pub(super) fn bootstrap_topic_graph_edge_weight(
        left: &TopicSession,
        right: &TopicSession,
        shift_kind: TopicShiftKind,
    ) -> f32 {
        let overlap = topic_session_label_overlap(left, right);
        let base = match shift_kind {
            TopicShiftKind::Split => 0.78,
            TopicShiftKind::Merged => 0.82,
            _ => 0.64,
        };
        (base + overlap * 0.14).min(0.92)
    }
}

mod bootstrap_neuron_propagation {
    use super::{
        BootstrapNeuronSeed, bootstrap_neuron_id, bootstrap_topic_graph_edge,
        bootstrap_topic_graph_edge_relation, infer_bootstrap_propagation_link,
    };
    use hepta_core::{NeuronId, NeuronLinkKind, TopicGraphEdgeKind, TopicSession};

    pub(super) fn record_source_link(
        source_topic_session: &TopicSession,
        link_kind: NeuronLinkKind,
        link_reason: &str,
        source_topic_session_ids: &mut Vec<String>,
        source_neuron_ids: &mut Vec<NeuronId>,
        source_link_kinds: &mut Vec<NeuronLinkKind>,
        source_link_reasons: &mut Vec<String>,
    ) {
        if source_topic_session_ids
            .iter()
            .all(|session_id| session_id != &source_topic_session.topic_session_id)
        {
            source_topic_session_ids.push(source_topic_session.topic_session_id.clone());
        }

        let source_neuron_id = bootstrap_neuron_id(&source_topic_session.topic_id);
        if !source_neuron_ids.contains(&source_neuron_id) {
            source_neuron_ids.push(source_neuron_id);
        }

        source_link_kinds.push(link_kind);
        source_link_reasons.push(format!(
            "{} via {}",
            link_reason, source_topic_session.topic_session_id,
        ));
    }

    pub(super) fn infer_link(
        source: &BootstrapNeuronSeed,
        target: &BootstrapNeuronSeed,
        co_active: bool,
    ) -> Option<(NeuronLinkKind, f32, String)> {
        stored_topic_graph_link(&source.topic_session, &target.topic_session)
            .or_else(|| compressed_neuron_link(source, target))
            .or_else(|| reciprocal_compressed_neuron_link(source, target))
            .or_else(|| {
                infer_bootstrap_propagation_link(
                    &source.topic_session,
                    &target.topic_session,
                    co_active,
                )
            })
    }

    fn stored_topic_graph_link(
        source: &TopicSession,
        target: &TopicSession,
    ) -> Option<(NeuronLinkKind, f32, String)> {
        let edge = bootstrap_topic_graph_edge(source, &target.topic_session_id)?;
        let (kind, reason) = match edge.kind {
            TopicGraphEdgeKind::CoActivation => (
                NeuronLinkKind::WorkflowAdjacency,
                format!(
                    "bootstrap stored co-activation edge into '{}' strength {:.2}",
                    target.topic_label.0, edge.weight,
                ),
            ),
            TopicGraphEdgeKind::SplitComponent => (
                NeuronLinkKind::TemporalContinuation,
                format!(
                    "bootstrap stored split-component edge into '{}' strength {:.2}",
                    target.topic_label.0, edge.weight,
                ),
            ),
            TopicGraphEdgeKind::MergedInto | TopicGraphEdgeKind::HasComponent => (
                NeuronLinkKind::CausalDependency,
                format!(
                    "bootstrap stored merge-component edge into '{}' strength {:.2}",
                    target.topic_label.0, edge.weight,
                ),
            ),
            _ => (
                NeuronLinkKind::SemanticSimilarity,
                format!(
                    "bootstrap stored {} edge into '{}' strength {:.2}",
                    bootstrap_topic_graph_edge_relation(&edge),
                    target.topic_label.0,
                    edge.weight,
                ),
            ),
        };
        Some((kind, edge.weight.min(0.46), reason))
    }

    fn compressed_neuron_link(
        source: &BootstrapNeuronSeed,
        target: &BootstrapNeuronSeed,
    ) -> Option<(NeuronLinkKind, f32, String)> {
        let link = source
            .neuron
            .links
            .iter()
            .find(|link| link.target_neuron_id == target.neuron.neuron_id)
            .filter(|link| {
                !matches!(
                    link.kind,
                    NeuronLinkKind::Conflict | NeuronLinkKind::Inhibition
                )
            })?;
        let relation = link.relation.as_deref().unwrap_or("compressed_neuron_link");
        Some((
            link.kind,
            link.strength.min(0.46),
            format!(
                "compressed neuron link '{}' into '{}' strength {:.2}",
                relation, target.neuron.topic_label.0, link.strength,
            ),
        ))
    }

    fn reciprocal_compressed_neuron_link(
        source: &BootstrapNeuronSeed,
        target: &BootstrapNeuronSeed,
    ) -> Option<(NeuronLinkKind, f32, String)> {
        let link = target
            .neuron
            .links
            .iter()
            .find(|link| link.target_neuron_id == source.neuron.neuron_id)
            .filter(|link| {
                !matches!(
                    link.kind,
                    NeuronLinkKind::Conflict | NeuronLinkKind::Inhibition
                )
            })?;
        let relation = link.relation.as_deref().unwrap_or("compressed_neuron_link");
        Some((
            link.kind,
            link.strength.min(0.46),
            format!(
                "compressed reciprocal neuron link '{}' into '{}' strength {:.2}",
                relation, target.neuron.topic_label.0, link.strength,
            ),
        ))
    }
}

mod bootstrap_neuron_activation_support {
    use hepta_core::{NeuronId, NeuronLinkKind, TranscriptSpanRef};

    use super::{
        BootstrapNeuronSeed, compute_bootstrap_inhibition_score,
        compute_bootstrap_propagated_score, infer_bootstrap_neuron_inhibition_link,
        infer_bootstrap_neuron_propagation_link, merge_bootstrap_topic_session_transcript_evidence,
        record_bootstrap_neuron_link,
    };

    #[derive(Debug, Clone, Default, PartialEq)]
    pub(super) struct BootstrapNeuronActivationSources {
        pub(super) propagated_score: f32,
        pub(super) inhibition_score: f32,
        pub(super) source_topic_session_ids: Vec<String>,
        pub(super) source_neuron_ids: Vec<NeuronId>,
        pub(super) source_transcript_spans: Vec<TranscriptSpanRef>,
        pub(super) source_link_kinds: Vec<NeuronLinkKind>,
        pub(super) source_link_reasons: Vec<String>,
    }

    pub(super) fn collect(
        seed: &BootstrapNeuronSeed,
        direct_seeds: &[BootstrapNeuronSeed],
        inhibition_marker: Option<&'static str>,
    ) -> BootstrapNeuronActivationSources {
        let mut sources = BootstrapNeuronActivationSources {
            source_topic_session_ids: vec![seed.topic_session.topic_session_id.clone()],
            ..Default::default()
        };

        merge_bootstrap_topic_session_transcript_evidence(
            &mut sources.source_transcript_spans,
            &seed.neuron.important_transcript_spans,
        );

        if inhibition_marker.is_none() {
            collect_propagation(seed, direct_seeds, &mut sources);
        }

        if let Some(marker) = inhibition_marker {
            collect_inhibition(seed, direct_seeds, marker, &mut sources);
        }

        sources
    }

    fn collect_propagation(
        seed: &BootstrapNeuronSeed,
        direct_seeds: &[BootstrapNeuronSeed],
        sources: &mut BootstrapNeuronActivationSources,
    ) {
        for source_seed in direct_seeds {
            if source_seed.topic_session.topic_session_id == seed.topic_session.topic_session_id {
                continue;
            }

            if let Some((link_kind, link_strength, link_reason)) =
                infer_bootstrap_neuron_propagation_link(source_seed, seed, true)
            {
                let contribution =
                    compute_bootstrap_propagated_score(source_seed.direct_score, link_strength);
                if contribution <= 0.0 {
                    continue;
                }

                sources.propagated_score += contribution;
                record_link(source_seed, link_kind, &link_reason, sources);
            }
        }
    }

    fn collect_inhibition(
        seed: &BootstrapNeuronSeed,
        direct_seeds: &[BootstrapNeuronSeed],
        marker: &'static str,
        sources: &mut BootstrapNeuronActivationSources,
    ) {
        let Some(source_seed) = direct_seeds.first() else {
            return;
        };
        if source_seed.topic_session.topic_session_id == seed.topic_session.topic_session_id {
            return;
        }

        if let Some((link_strength, link_reason)) =
            infer_bootstrap_neuron_inhibition_link(source_seed, seed, marker)
        {
            sources.inhibition_score =
                compute_bootstrap_inhibition_score(source_seed.direct_score, link_strength)
                    .min((seed.direct_score * 0.55).max(0.08));
            if sources.inhibition_score > 0.0 {
                record_link(
                    source_seed,
                    NeuronLinkKind::Inhibition,
                    &link_reason,
                    sources,
                );
            }
        }
    }

    fn record_link(
        source_seed: &BootstrapNeuronSeed,
        link_kind: NeuronLinkKind,
        link_reason: &str,
        sources: &mut BootstrapNeuronActivationSources,
    ) {
        merge_bootstrap_topic_session_transcript_evidence(
            &mut sources.source_transcript_spans,
            &source_seed.neuron.important_transcript_spans,
        );
        record_bootstrap_neuron_link(
            &source_seed.topic_session,
            link_kind,
            link_reason,
            &mut sources.source_topic_session_ids,
            &mut sources.source_neuron_ids,
            &mut sources.source_link_kinds,
            &mut sources.source_link_reasons,
        );
    }
}

mod bootstrap_neuron_activation_summary {
    use hepta_core::NeuronActivation;

    use super::{
        BootstrapNeuronSeed, bootstrap_neuron_activation_support::BootstrapNeuronActivationSources,
    };

    pub(super) fn build(
        seed: &BootstrapNeuronSeed,
        sources: BootstrapNeuronActivationSources,
    ) -> NeuronActivation {
        let direct_score = seed.direct_score;
        let propagated_score = clamp_propagated_score(direct_score, sources.propagated_score);
        let inhibition_score = sources.inhibition_score;
        let final_score = (direct_score + propagated_score - inhibition_score).clamp(0.0, 1.0);
        let reason = build_reason(
            seed,
            propagated_score,
            inhibition_score,
            sources.source_neuron_ids.len(),
        );

        NeuronActivation {
            neuron_id: seed.neuron.neuron_id.clone(),
            topic_id: seed.neuron.topic_id.clone(),
            direct_score,
            propagated_score,
            inhibition_score,
            final_score,
            source_topic_session_ids: sources.source_topic_session_ids,
            source_neuron_ids: sources.source_neuron_ids,
            source_transcript_spans: sources.source_transcript_spans,
            source_link_kinds: sources.source_link_kinds,
            source_link_reasons: sources.source_link_reasons,
            reason: Some(reason),
        }
    }

    fn clamp_propagated_score(direct_score: f32, propagated_score: f32) -> f32 {
        propagated_score.min((direct_score * 0.35).max(0.08))
    }

    fn build_reason(
        seed: &BootstrapNeuronSeed,
        propagated_score: f32,
        inhibition_score: f32,
        source_neuron_count: usize,
    ) -> String {
        let topic_session = &seed.topic_session;
        let neuron = &seed.neuron;
        let prior_count = neuron.skill_priors.len() + neuron.workflow_priors.len();

        if inhibition_score > 0.0 {
            format!(
                "bootstrap direct activation via routed topic session '{}' for compressed neuron '{}' with {} open loops, {} durable refs, and {} prior(s), then inhibitory suppression {:.2} from {} linked neuron(s)",
                topic_session.topic_session_id,
                neuron.topic_label.0,
                neuron.open_loops.len(),
                neuron.promoted_memory_refs.len(),
                prior_count,
                inhibition_score,
                source_neuron_count,
            )
        } else if propagated_score > 0.0 {
            format!(
                "bootstrap direct activation via routed topic session '{}' for compressed neuron '{}' with {} open loops, {} durable refs, and {} prior(s), plus propagated activation {:.2} from {} linked neuron(s)",
                topic_session.topic_session_id,
                neuron.topic_label.0,
                neuron.open_loops.len(),
                neuron.promoted_memory_refs.len(),
                prior_count,
                propagated_score,
                source_neuron_count,
            )
        } else {
            format!(
                "bootstrap direct activation via routed topic session '{}' for compressed neuron '{}' with {} open loops, {} durable refs, and {} prior(s); no additional propagated activation fired yet",
                topic_session.topic_session_id,
                neuron.topic_label.0,
                neuron.open_loops.len(),
                neuron.promoted_memory_refs.len(),
                prior_count,
            )
        }
    }
}

mod neuron_activation_overview_support {
    use hepta_core::{HeptaNeuron, NeuronActivation, TopicActivationScore, TopicSession};

    use super::{
        build_bootstrap_neuron_activation, collect_bootstrap_direct_seeds,
        detect_bootstrap_inhibition_marker,
    };

    pub(super) fn build_bootstrap_activations(
        query_text: Option<&str>,
        topic_sessions: &[TopicSession],
        compressed_neurons: &[HeptaNeuron],
        active_topic_session_ids: &[String],
        activation_scores: &[TopicActivationScore],
        recent_entry_count: usize,
        transcript_matched_count: usize,
        durable_memory_hit_count: usize,
        summary_hit_count: usize,
        neuron_limit: usize,
    ) -> Vec<NeuronActivation> {
        if neuron_limit == 0 {
            return Vec::new();
        }

        let inhibition_marker = detect_bootstrap_inhibition_marker(query_text);
        let direct_seeds = collect_bootstrap_direct_seeds(
            topic_sessions,
            compressed_neurons,
            active_topic_session_ids,
            activation_scores,
            recent_entry_count,
            transcript_matched_count,
            durable_memory_hit_count,
            summary_hit_count,
        );

        direct_seeds
            .iter()
            .take(neuron_limit)
            .map(|seed| build_bootstrap_neuron_activation(seed, &direct_seeds, inhibition_marker))
            .collect()
    }
}

mod event_digest_rollup {
    use std::collections::BTreeMap;

    use super::{EventRecord, RuntimeEventDigest, RuntimeEventKindTally, RuntimeEventSessionTally};

    pub(super) fn build(events: Vec<EventRecord>) -> RuntimeEventDigest {
        let kinds = tally_kinds(&events);
        let sessions = tally_sessions(&events);

        RuntimeEventDigest {
            events,
            kinds,
            sessions,
        }
    }

    fn tally_kinds(events: &[EventRecord]) -> Vec<RuntimeEventKindTally> {
        let mut kinds = BTreeMap::<String, usize>::new();
        for record in events {
            *kinds.entry(format!("{:?}", record.event.kind)).or_default() += 1;
        }

        let mut kinds = kinds
            .into_iter()
            .map(|(kind, count)| RuntimeEventKindTally { kind, count })
            .collect::<Vec<_>>();
        kinds.sort_by(|left, right| {
            right
                .count
                .cmp(&left.count)
                .then_with(|| left.kind.cmp(&right.kind))
        });
        kinds
    }

    fn tally_sessions(events: &[EventRecord]) -> Vec<RuntimeEventSessionTally> {
        let mut sessions = BTreeMap::<Option<String>, RuntimeEventSessionTally>::new();

        for record in events {
            let session_id = record.event.session_id.as_ref().map(|id| id.0.clone());
            let entry =
                sessions
                    .entry(session_id.clone())
                    .or_insert_with(|| RuntimeEventSessionTally {
                        session_id: session_id.clone(),
                        count: 0,
                        latest_event: record.clone(),
                    });
            entry.count += 1;
            entry.latest_event = record.clone();
        }

        let mut sessions = sessions.into_values().collect::<Vec<_>>();
        sessions.sort_by(|left, right| {
            right
                .count
                .cmp(&left.count)
                .then_with(|| session_label(&left.session_id).cmp(session_label(&right.session_id)))
        });
        sessions
    }

    fn session_label(session_id: &Option<String>) -> &str {
        session_id.as_deref().unwrap_or("global")
    }
}

mod session_activity_rollup {
    use super::{RuntimeSessionActivityOverview, RuntimeSessionActivitySlice};

    pub(super) fn build(
        sessions: Vec<RuntimeSessionActivitySlice>,
    ) -> RuntimeSessionActivityOverview {
        let active_sessions = count_matching(&sessions, |activity| activity.session.is_active);
        let archived_sessions = count_matching(&sessions, |activity| {
            activity.session.archived_at_unix_ms.is_some()
        });
        let sessions_with_history =
            count_matching(&sessions, |activity| !activity.history.is_empty());
        let sessions_with_events =
            count_matching(&sessions, |activity| !activity.events.is_empty());
        let sessions_with_topic_state = count_matching(&sessions, |activity| {
            activity.session.topic_session_count > 0 || activity.session.topic_graph_edge_count > 0
        });
        let total_topic_sessions = sessions
            .iter()
            .map(|activity| activity.session.topic_session_count)
            .sum();
        let total_topic_graph_edges = sessions
            .iter()
            .map(|activity| activity.session.topic_graph_edge_count)
            .sum();

        RuntimeSessionActivityOverview {
            sessions,
            active_sessions,
            archived_sessions,
            sessions_with_history,
            sessions_with_events,
            sessions_with_topic_state,
            total_topic_sessions,
            total_topic_graph_edges,
        }
    }

    fn count_matching(
        sessions: &[RuntimeSessionActivitySlice],
        predicate: impl Fn(&RuntimeSessionActivitySlice) -> bool,
    ) -> usize {
        sessions
            .iter()
            .filter(|activity| predicate(activity))
            .count()
    }
}

mod transcript_query_rollup {
    use std::collections::BTreeMap;

    use hepta_core::TranscriptQueryReport;

    use super::{RuntimeTranscriptQueryOverview, RuntimeTranscriptQuerySessionTally};

    pub(super) fn build(report: TranscriptQueryReport) -> RuntimeTranscriptQueryOverview {
        let returned_entries = report.hits.iter().map(|span| span.entry_count).sum();
        let sessions = tally_sessions(&report);
        let matched_sessions = sessions.len();

        RuntimeTranscriptQueryOverview {
            report,
            returned_entries,
            matched_sessions,
            sessions,
        }
    }

    fn tally_sessions(report: &TranscriptQueryReport) -> Vec<RuntimeTranscriptQuerySessionTally> {
        let mut sessions = BTreeMap::<String, RuntimeTranscriptQuerySessionTally>::new();

        for span in &report.hits {
            let entry = sessions
                .entry(span.session_id.0.clone())
                .or_insert_with(|| RuntimeTranscriptQuerySessionTally {
                    session_id: span.session_id.0.clone(),
                    hit_count: 0,
                    entry_count: 0,
                });
            entry.hit_count += 1;
            entry.entry_count += span.entry_count;
        }

        let mut sessions = sessions.into_values().collect::<Vec<_>>();
        sessions.sort_by(|left, right| {
            right
                .hit_count
                .cmp(&left.hit_count)
                .then_with(|| right.entry_count.cmp(&left.entry_count))
                .then_with(|| left.session_id.cmp(&right.session_id))
        });
        sessions
    }
}

mod transcript_query_support {
    use hepta_core::{
        SessionId, TranscriptEntry, TranscriptQuery, TranscriptQueryReport, TranscriptSpan,
    };

    pub(super) fn request(session_id: Option<&str>, query: &str, limit: usize) -> TranscriptQuery {
        TranscriptQuery {
            session_id: session_id.map(|id| SessionId(id.to_string())),
            text: query.to_string(),
            limit,
        }
    }

    pub(super) fn empty_report(session_id: &str, limit: usize) -> TranscriptQueryReport {
        TranscriptQueryReport::from_hits(request(Some(session_id), "", limit), 0, Vec::new())
    }

    pub(super) fn fallback_legacy_report(
        transcript_query: TranscriptQuery,
        entries: Vec<TranscriptEntry>,
    ) -> TranscriptQueryReport {
        let matched = entries
            .into_iter()
            .filter(|entry| entry.matches_query(&transcript_query))
            .map(TranscriptSpan::from_entry)
            .collect::<Vec<_>>();
        let matched_count = matched.len();
        let mut hits = matched;
        hits.truncate(transcript_query.limit);

        TranscriptQueryReport::from_hits(transcript_query, matched_count, hits)
    }
}

mod context_recall_support {
    use hepta_core::{
        ContextBudget, ContextRecallBundle, ContextRecallItem, ContextRecallRequest,
        ContextRecallScore, ContextRecallSource, MemoryQueryReport, MemoryRecord, MemoryScope,
        TopicSession, TranscriptEntry, TranscriptQueryReport, TranscriptSpan, TranscriptSpanRef,
    };

    use super::RuntimeContextRecallSlice;

    #[derive(Debug)]
    pub(super) struct ContextRecallBuildInputs {
        pub(super) request: ContextRecallRequest,
        pub(super) recent_entries: Vec<TranscriptEntry>,
        pub(super) total_recent_entry_count: usize,
        pub(super) transcript_report: TranscriptQueryReport,
        pub(super) memory_report: MemoryQueryReport,
        pub(super) active_topic_sessions: Vec<TopicSession>,
    }

    pub(super) fn prepare_recent_entries(
        mut recent_entries: Vec<TranscriptEntry>,
        recent_window_limit: usize,
    ) -> (Vec<TranscriptEntry>, usize) {
        let total_recent_entry_count = recent_entries.len();
        if recent_entries.len() > recent_window_limit {
            recent_entries = recent_entries.split_off(recent_entries.len() - recent_window_limit);
        }

        (recent_entries, total_recent_entry_count)
    }

    pub(super) fn build_slice(inputs: ContextRecallBuildInputs) -> RuntimeContextRecallSlice {
        let ContextRecallBuildInputs {
            request,
            recent_entries,
            total_recent_entry_count,
            transcript_report,
            memory_report,
            active_topic_sessions,
        } = inputs;

        let recent_entry_count = recent_entries.len();
        let transcript_matched_count = transcript_report.matched_count;
        let transcript_returned_count = transcript_report.returned_count;
        let transcript_truncated = transcript_report.truncated;
        let transcript_hits = transcript_report.hits;
        let memory_matched_count = memory_report.matched_count;
        let memory_truncated = memory_report.truncated;
        let (durable_memory_hits, summary_hits) = partition_memory_hits(memory_report.hits);
        let durable_memory_hit_count = durable_memory_hits.len();
        let summary_hit_count = summary_hits.len();
        let active_topic_session_count = active_topic_sessions.len();
        let budget = ContextBudget::from_request(&request);
        let mut ranked_items = build_ranked_items(
            &recent_entries,
            &transcript_hits,
            &durable_memory_hits,
            &summary_hits,
            &active_topic_sessions,
        );
        ranked_items.sort_by(|left, right| {
            right
                .score
                .final_score
                .total_cmp(&left.score.final_score)
                .then_with(|| left.source_id.cmp(&right.source_id))
        });
        let omitted_by_budget = ranked_items.len().saturating_sub(budget.max_items);
        ranked_items.truncate(budget.max_items);
        let bundle = ContextRecallBundle {
            request,
            recent_entries,
            transcript_hits,
            durable_memory_hits,
            summary_hits,
            active_topic_sessions,
            active_neurons: Vec::new(),
            budget,
            ranked_items,
            omitted_by_budget,
            truncated: transcript_truncated || memory_truncated,
        };
        let transcript_evidence = transcript_evidence(&bundle);

        RuntimeContextRecallSlice {
            bundle,
            recent_entry_count,
            total_recent_entry_count,
            transcript_matched_count,
            transcript_returned_count,
            memory_matched_count,
            durable_memory_hit_count,
            summary_hit_count,
            active_topic_session_count,
            transcript_evidence,
        }
    }

    fn score(
        recency: f32,
        relevance: f32,
        durability: f32,
        topic_activation: f32,
        neuron_activation: f32,
        confidence: f32,
        reason: &str,
    ) -> ContextRecallScore {
        let final_score = ((recency * 0.22)
            + (relevance * 0.28)
            + (durability * 0.16)
            + (topic_activation * 0.14)
            + (neuron_activation * 0.10)
            + (confidence * 0.10))
            .clamp(0.0, 1.0);
        ContextRecallScore {
            recency,
            relevance,
            durability,
            topic_activation,
            neuron_activation,
            confidence,
            final_score,
            reason: Some(reason.to_string()),
        }
    }

    fn build_ranked_items(
        recent_entries: &[TranscriptEntry],
        transcript_hits: &[TranscriptSpan],
        durable_memory_hits: &[MemoryRecord],
        summary_hits: &[MemoryRecord],
        active_topic_sessions: &[TopicSession],
    ) -> Vec<ContextRecallItem> {
        let mut items = Vec::new();

        for entry in recent_entries {
            items.push(ContextRecallItem {
                source: ContextRecallSource::RecentWindow,
                source_id: entry.entry_id.clone(),
                summary: entry.content.chars().take(160).collect(),
                score: score(1.0, 0.72, 0.32, 0.0, 0.0, 0.80, "recent transcript window"),
                source_transcript_spans: vec![TranscriptSpanRef {
                    session_id: entry.session_id.clone(),
                    range: hepta_core::TranscriptRange {
                        start_sequence: entry.sequence,
                        end_sequence: entry.sequence,
                    },
                    reason: Some("ranked_recent_window".into()),
                }],
                source_memory_ids: Vec::new(),
                topic_session_ids: Vec::new(),
                neuron_ids: Vec::new(),
            });
        }

        for hit in transcript_hits {
            items.push(ContextRecallItem {
                source: ContextRecallSource::Transcript,
                source_id: format!(
                    "{}:{}-{}",
                    hit.session_id.0, hit.range.start_sequence, hit.range.end_sequence
                ),
                summary: hit
                    .excerpt
                    .clone()
                    .unwrap_or_else(|| format!("{} transcript entrie(s)", hit.entry_count)),
                score: score(0.78, 1.0, 0.45, 0.0, 0.0, 0.82, "query transcript hit"),
                source_transcript_spans: vec![TranscriptSpanRef {
                    session_id: hit.session_id.clone(),
                    range: hit.range.clone(),
                    reason: Some("ranked_transcript_hit".into()),
                }],
                source_memory_ids: Vec::new(),
                topic_session_ids: Vec::new(),
                neuron_ids: Vec::new(),
            });
        }

        for memory in durable_memory_hits {
            items.push(ContextRecallItem {
                source: ContextRecallSource::DurableMemory,
                source_id: memory.id.clone(),
                summary: memory.content.chars().take(160).collect(),
                score: score(
                    0.58,
                    0.78,
                    1.0,
                    0.0,
                    0.0,
                    0.74,
                    "durable promoted memory hit",
                ),
                source_transcript_spans: Vec::new(),
                source_memory_ids: vec![memory.id.clone()],
                topic_session_ids: Vec::new(),
                neuron_ids: Vec::new(),
            });
        }

        for memory in summary_hits {
            items.push(ContextRecallItem {
                source: ContextRecallSource::SummaryMemory,
                source_id: memory.id.clone(),
                summary: memory.content.chars().take(160).collect(),
                score: score(
                    0.52,
                    0.70,
                    0.64,
                    0.0,
                    0.0,
                    0.68,
                    "session summary memory hit",
                ),
                source_transcript_spans: Vec::new(),
                source_memory_ids: vec![memory.id.clone()],
                topic_session_ids: Vec::new(),
                neuron_ids: Vec::new(),
            });
        }

        for topic_session in active_topic_sessions {
            items.push(ContextRecallItem {
                source: ContextRecallSource::ActiveTopicSession,
                source_id: topic_session.topic_session_id.clone(),
                summary: topic_session.topic_label.0.clone(),
                score: score(
                    0.74,
                    0.72,
                    0.72,
                    1.0,
                    0.0,
                    0.76,
                    "active topic session state",
                ),
                source_transcript_spans: topic_session.linked_transcript_spans.clone(),
                source_memory_ids: topic_session.durable_memory_refs.clone(),
                topic_session_ids: vec![topic_session.topic_session_id.clone()],
                neuron_ids: Vec::new(),
            });
        }

        items
    }

    fn partition_memory_hits(hits: Vec<MemoryRecord>) -> (Vec<MemoryRecord>, Vec<MemoryRecord>) {
        let mut durable_memory_hits = Vec::new();
        let mut summary_hits = Vec::new();

        for hit in hits {
            match hit.scope {
                MemoryScope::LongTerm => durable_memory_hits.push(hit),
                MemoryScope::Session => summary_hits.push(hit),
            }
        }

        (durable_memory_hits, summary_hits)
    }

    fn transcript_evidence(bundle: &ContextRecallBundle) -> Vec<TranscriptSpanRef> {
        let mut transcript_evidence_bundle = bundle.clone();
        transcript_evidence_bundle.active_topic_sessions.clear();
        transcript_evidence_bundle.source_transcript_spans()
    }

    #[cfg(test)]
    mod tests {
        use hepta_core::{
            MemoryRecord, MemoryScope, MessageRole, SessionId, TranscriptEntry, TranscriptEntryKind,
        };

        use super::{partition_memory_hits, prepare_recent_entries};

        #[test]
        fn prepare_recent_entries_preserves_total_count_when_truncating() {
            let entries = vec![
                transcript_entry(1, "first"),
                transcript_entry(2, "second"),
                transcript_entry(3, "third"),
            ];

            let (recent_entries, total_recent_entry_count) = prepare_recent_entries(entries, 2);

            assert_eq!(total_recent_entry_count, 3);
            assert_eq!(recent_entries.len(), 2);
            assert_eq!(recent_entries[0].sequence, 2);
            assert_eq!(recent_entries[1].sequence, 3);
        }

        #[test]
        fn partition_memory_hits_preserves_scope_order() {
            let hits = vec![
                memory_record("session-1", MemoryScope::Session),
                memory_record("long-1", MemoryScope::LongTerm),
                memory_record("session-2", MemoryScope::Session),
                memory_record("long-2", MemoryScope::LongTerm),
            ];

            let (durable_memory_hits, summary_hits) = partition_memory_hits(hits);

            assert_eq!(
                durable_memory_hits
                    .iter()
                    .map(|record| record.id.as_str())
                    .collect::<Vec<_>>(),
                vec!["long-1", "long-2"]
            );
            assert_eq!(
                summary_hits
                    .iter()
                    .map(|record| record.id.as_str())
                    .collect::<Vec<_>>(),
                vec!["session-1", "session-2"]
            );
        }

        fn transcript_entry(sequence: u64, content: &str) -> TranscriptEntry {
            TranscriptEntry {
                entry_id: format!("entry-{sequence}"),
                session_id: SessionId("alpha".into()),
                sequence,
                kind: TranscriptEntryKind::Message,
                role: Some(MessageRole::User),
                content: content.to_string(),
                created_at_unix_ms: 0,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            }
        }

        fn memory_record(id: &str, scope: MemoryScope) -> MemoryRecord {
            MemoryRecord {
                id: id.to_string(),
                scope,
                content: format!("memory {id}"),
            }
        }
    }
}

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
    bundle.ranked_items.sort_by(|left, right| {
        right
            .score
            .final_score
            .total_cmp(&left.score.final_score)
            .then_with(|| left.source_id.cmp(&right.source_id))
    });
    let total_ranked = bundle.ranked_items.len();
    if total_ranked > bundle.budget.max_items {
        bundle.omitted_by_budget = bundle
            .omitted_by_budget
            .saturating_add(total_ranked - bundle.budget.max_items);
        bundle.ranked_items.truncate(bundle.budget.max_items);
        bundle.truncated = true;
    }
}

mod provenance_overview_rollup {
    use hepta_core::{TopicSession, TopicSessionStatus};

    use super::RuntimeProvenanceOverview;

    pub(super) struct ProvenanceOverviewInputs {
        pub session_id: String,
        pub last_user_intent_summary: Option<String>,
        pub topic_sessions: Vec<TopicSession>,
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
    pub active_topic_session_count: usize,
    pub transcript_evidence: Vec<TranscriptSpanRef>,
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

#[derive(Debug, Clone, PartialEq)]
struct BootstrapNeuronSeed {
    topic_session: TopicSession,
    neuron: HeptaNeuron,
    direct_score: f32,
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
        let weight_delta = match outcome {
            IntuitionFeedbackOutcome::Accepted => 0.12,
            IntuitionFeedbackOutcome::ExecutedSuccess => 0.18,
            IntuitionFeedbackOutcome::Corrected => 0.04,
            IntuitionFeedbackOutcome::Ignored => -0.03,
            IntuitionFeedbackOutcome::Rejected => -0.18,
            IntuitionFeedbackOutcome::ExecutedFailed => -0.22,
            IntuitionFeedbackOutcome::UserOverride => -0.10,
            IntuitionFeedbackOutcome::ToolFailed => -0.08,
            IntuitionFeedbackOutcome::UnsafeBlocked => -0.16,
        };
        let created_at_unix_ms = current_unix_ms()?;
        let confidence_before = self.estimate_intuition_feedback_confidence(
            session_id,
            skill_id,
            workflow_id,
            &source_topic_ids,
            &source_neuron_ids,
        )?;
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
        self.apply_intuition_feedback_learning(session_id, &record)?;
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

    fn estimate_intuition_feedback_confidence(
        &self,
        session_id: &str,
        skill_id: Option<&str>,
        workflow_id: Option<&str>,
        source_topic_ids: &[TopicId],
        source_neuron_ids: &[NeuronId],
    ) -> Result<f32, HeptaError> {
        let records = self.intuition_feedback_for_session(session_id)?;
        let topic_id = source_topic_ids.first();
        let neuron_id = source_neuron_ids.first();
        let delta =
            compute_intuition_feedback_delta(&records, topic_id, neuron_id, skill_id, workflow_id);

        Ok((0.50 + delta).clamp(0.0, 1.0))
    }

    fn apply_intuition_feedback_learning(
        &self,
        session_id: &str,
        record: &IntuitionFeedbackRecord,
    ) -> Result<(), HeptaError> {
        let learned_terms = learned_semantic_terms_for_feedback(record);
        self.apply_feedback_learning_to_topic_sessions(session_id, record, &learned_terms)?;
        self.apply_feedback_learning_to_neurons(session_id, record, &learned_terms)?;
        Ok(())
    }

    fn apply_feedback_learning_to_topic_sessions(
        &self,
        session_id: &str,
        record: &IntuitionFeedbackRecord,
        learned_terms: &[String],
    ) -> Result<(), HeptaError> {
        let source_topic_ids = record
            .source_topic_ids
            .iter()
            .map(|topic_id| topic_id.0.as_str())
            .collect::<BTreeSet<_>>();
        let mut guard = self
            .topic_session_state
            .lock()
            .map_err(|_| HeptaError("topic session state mutex poisoned".into()))?;

        for topic_session in guard.sessions.iter_mut().filter(|topic_session| {
            topic_session
                .linked_surface_session_ids
                .iter()
                .any(|linked| linked.0 == session_id)
                && (source_topic_ids.is_empty()
                    || source_topic_ids.contains(topic_session.topic_id.0.as_str())
                    || matches!(topic_session.status, TopicSessionStatus::Active))
        }) {
            bootstrap_planner::merge_bootstrap_topic_session_semantic_hints(
                &mut topic_session.entities,
                learned_terms,
            );
            increment_entity_usize(&mut topic_session.entities, FEEDBACK_LEARNER_COUNT_KEY, 1);
            accumulate_entity_f32(
                &mut topic_session.entities,
                FEEDBACK_LEARNER_NET_DELTA_KEY,
                record.weight_delta,
            );
            accumulate_entity_f32(
                &mut topic_session.entities,
                SEMANTIC_ROUTER_NET_DELTA_KEY,
                record.weight_delta,
            );
            topic_session.entities.insert(
                FEEDBACK_LEARNER_LAST_OUTCOME_KEY.into(),
                format_intuition_feedback_outcome(record.outcome).into(),
            );
            topic_session
                .entities
                .insert(SEMANTIC_ROUTER_LEARNED_KEY.into(), "true".into());
            if let Some(term) = learned_terms.first() {
                topic_session
                    .entities
                    .insert(SEMANTIC_ROUTER_LAST_SIGNAL_KEY.into(), term.clone());
            }
        }

        Ok(())
    }

    fn apply_feedback_learning_to_neurons(
        &self,
        session_id: &str,
        record: &IntuitionFeedbackRecord,
        learned_terms: &[String],
    ) -> Result<(), HeptaError> {
        let source_topic_ids = record
            .source_topic_ids
            .iter()
            .map(|topic_id| topic_id.0.as_str())
            .collect::<BTreeSet<_>>();
        let source_neuron_ids = record
            .source_neuron_ids
            .iter()
            .map(|neuron_id| neuron_id.0.as_str())
            .collect::<BTreeSet<_>>();
        let mut updated = Vec::new();

        for mut neuron in self.stored_neurons_for_session(session_id)? {
            let target_neuron = source_neuron_ids.contains(neuron.neuron_id.0.as_str());
            let target_topic = source_topic_ids.contains(neuron.topic_id.0.as_str());
            if !target_neuron && !target_topic {
                continue;
            }

            let confidence_delta = record.weight_delta * 0.35;
            let freshness_delta = record.weight_delta * 0.25;
            neuron.confidence = (neuron.confidence + confidence_delta).clamp(0.0, 1.0);
            neuron.freshness = (neuron.freshness + freshness_delta).clamp(0.0, 1.0);
            neuron.staleness_score = (neuron.staleness_score - freshness_delta).clamp(0.0, 1.0);
            neuron.neuron_revision = neuron.neuron_revision.saturating_add(1);
            neuron.last_refresh_reason = Some(format!(
                "feedback-learning:{}:{:+.2}",
                format_intuition_feedback_outcome(record.outcome),
                record.weight_delta,
            ));
            neuron.source_evidence_digest.get_or_insert_with(|| {
                format!(
                    "feedback:{}:{}",
                    session_id,
                    record.decision_id.as_deref().unwrap_or("untracked")
                )
            });
            increment_entity_usize(&mut neuron.entity_state, FEEDBACK_LEARNER_COUNT_KEY, 1);
            accumulate_entity_f32(
                &mut neuron.entity_state,
                FEEDBACK_LEARNER_NET_DELTA_KEY,
                record.weight_delta,
            );
            neuron
                .entity_state
                .insert(SEMANTIC_ROUTER_LEARNED_KEY.into(), "true".into());
            if !learned_terms.is_empty() {
                neuron.entity_state.insert(
                    SEMANTIC_ROUTER_LAST_SIGNAL_KEY.into(),
                    learned_terms
                        .iter()
                        .take(4)
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if record.weight_delta > 0.0 {
                let preference = format!(
                    "feedback-confirmed:{}",
                    record
                        .skill_id
                        .as_deref()
                        .or(record.workflow_id.as_deref())
                        .unwrap_or("intuition")
                );
                if !neuron.stable_preferences.contains(&preference) {
                    neuron.stable_preferences.push(preference);
                }
            } else if let Some(reason) = record.reason.as_deref() {
                let open_loop = format!("feedback-review:{}", reason);
                if !neuron.open_loops.contains(&open_loop) {
                    neuron.open_loops.push(open_loop);
                }
            }

            updated.push(neuron);
        }

        self.upsert_neurons_for_session(session_id, updated)
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
        user_entries.sort_by(|left, right| left.sequence.cmp(&right.sequence));

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

        let recall_bundle =
            self.recall_context(session_id, Some("hello adaptive memory"), 10, 10, 10, true)?;
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
                    "ranked_items={} source_count={} transcript_spans={} durable_memory_hits={} active_neurons={}",
                    recall_bundle.ranked_items.len(),
                    recall_source_count,
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

        let activations = neuron_activation_overview_support::build_bootstrap_activations(
            query_text,
            &topic_sessions.topic_sessions,
            &compressed_neurons,
            &routing.decision.active_topic_session_ids,
            &routing.decision.activation_scores,
            recent_entry_count,
            transcript_matched_count,
            durable_memory_hit_count,
            summary_hit_count,
            neuron_limit,
        );

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
                    .map(|decision| IntuitionToolPolicyBinding {
                        tool_name: tool.name.clone(),
                        requirement: decision.requirement,
                        reason: decision.reason,
                        matched_rule_id: decision.matched_rule_id,
                    })
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| HeptaError(err.0))?;
        let intuition_feedback = self.intuition_feedback_for_session(session_id)?;
        let workflow_priors = build_bootstrap_workflow_priors(
            user_intent,
            &routing.decision.activation_scores,
            &activation.activations,
            &compressed_neurons,
            &intuition_feedback,
            skill_limit,
        );
        let skill_decisions = build_bootstrap_skill_decisions(
            user_intent,
            &routing.decision.activation_scores,
            &activation.activations,
            &compressed_neurons,
            &workflow_priors,
            &intuition_feedback,
            &registered_tools,
            &policy_bindings,
            skill_limit,
        );
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

fn compute_bootstrap_propagated_score(source_direct_score: f32, link_strength: f32) -> f32 {
    (source_direct_score * link_strength * 0.20).min(0.18)
}

fn compute_bootstrap_inhibition_score(source_direct_score: f32, link_strength: f32) -> f32 {
    (source_direct_score * link_strength * 0.26).min(0.22)
}

fn collect_bootstrap_direct_seeds(
    topic_sessions: &[TopicSession],
    compressed_neurons: &[HeptaNeuron],
    active_topic_session_ids: &[String],
    activation_scores: &[TopicActivationScore],
    recent_entry_count: usize,
    transcript_matched_count: usize,
    durable_memory_hit_count: usize,
    summary_hit_count: usize,
) -> Vec<BootstrapNeuronSeed> {
    let mut topic_session_by_id = topic_sessions
        .iter()
        .cloned()
        .map(|topic_session| (topic_session.topic_session_id.clone(), topic_session))
        .collect::<BTreeMap<_, _>>();
    let neuron_by_topic_id = compressed_neurons
        .iter()
        .cloned()
        .map(|neuron| (neuron.topic_id.0.clone(), neuron))
        .collect::<BTreeMap<_, _>>();

    active_topic_session_ids
        .iter()
        .filter_map(|active_id| topic_session_by_id.remove(active_id))
        .filter_map(|topic_session| {
            let neuron = neuron_by_topic_id.get(&topic_session.topic_id.0)?.clone();
            let direct_score = activation_scores
                .iter()
                .find(|score| score.topic_id == topic_session.topic_id)
                .map(|score| score.score)
                .unwrap_or_else(|| {
                    compute_bootstrap_direct_score(
                        recent_entry_count,
                        transcript_matched_count,
                        durable_memory_hit_count,
                        summary_hit_count,
                    )
                });
            (direct_score > 0.0).then_some(BootstrapNeuronSeed {
                topic_session,
                neuron,
                direct_score,
            })
        })
        .collect()
}

fn build_bootstrap_neuron_activation(
    seed: &BootstrapNeuronSeed,
    direct_seeds: &[BootstrapNeuronSeed],
    inhibition_marker: Option<&'static str>,
) -> NeuronActivation {
    let sources =
        bootstrap_neuron_activation_support::collect(seed, direct_seeds, inhibition_marker);
    bootstrap_neuron_activation_summary::build(seed, sources)
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

fn increment_entity_usize(entities: &mut BTreeMap<String, String>, key: &str, amount: usize) {
    let next = entities
        .get(key)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0)
        .saturating_add(amount);
    entities.insert(key.to_string(), next.to_string());
}

fn accumulate_entity_f32(entities: &mut BTreeMap<String, String>, key: &str, amount: f32) {
    let next = entities
        .get(key)
        .and_then(|value| value.parse::<f32>().ok())
        .unwrap_or(0.0)
        + amount;
    entities.insert(key.to_string(), format!("{next:.4}"));
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

fn build_bootstrap_workflow_priors(
    user_intent: &str,
    topic_scores: &[TopicActivationScore],
    activations: &[NeuronActivation],
    compressed_neurons: &[HeptaNeuron],
    intuition_feedback: &[IntuitionFeedbackRecord],
    limit: usize,
) -> Vec<WorkflowPrior> {
    if limit == 0 {
        return Vec::new();
    }

    topic_scores
        .iter()
        .take(limit)
        .map(|score| {
            let neuron_score = activations
                .iter()
                .find(|activation| activation.topic_id == score.topic_id)
                .map(|activation| activation.final_score)
                .unwrap_or(score.score);
            let neuron = compressed_neurons
                .iter()
                .find(|neuron| neuron.topic_id == score.topic_id);
            let neuron_prior = neuron.and_then(|neuron| neuron.workflow_priors.first());
            let neuron_policy = neuron
                .map(|neuron| neuron.compression_policy_version.as_str())
                .filter(|policy| !policy.trim().is_empty())
                .unwrap_or("none");
            let ranked_workflow = rank_intuition_workflow_candidate(
                user_intent,
                score,
                neuron_prior,
                &default_intuition_workflow_registry(),
            );
            let base_score = neuron_prior
                .map(|prior| ((score.score + neuron_score + prior.score) / 3.0).clamp(0.0, 1.0))
                .unwrap_or_else(|| ((score.score + neuron_score) / 2.0).clamp(0.0, 1.0));
            let feedback_delta = compute_intuition_feedback_delta(
                intuition_feedback,
                Some(&score.topic_id),
                neuron.map(|neuron| &neuron.neuron_id),
                None,
                Some(&ranked_workflow.workflow_id),
            );

            WorkflowPrior {
                workflow_id: ranked_workflow.workflow_id,
                score: (base_score + ranked_workflow.rank_bonus + feedback_delta).clamp(0.0, 1.0),
                exists_in_registry: ranked_workflow.registry_binding.exists_in_registry,
                missing_capability: ranked_workflow.registry_binding.missing_capability,
                requires_confirmation: ranked_workflow.registry_binding.requires_confirmation,
                action_mode: ranked_workflow.registry_binding.action_mode,
                source_topic_ids: vec![score.topic_id.clone()],
                source_neuron_ids: neuron
                    .map(|neuron| vec![neuron.neuron_id.clone()])
                    .unwrap_or_default(),
                reason: Some(format!(
                    "workflow registry ranked a prior for topic '{}' (routing {:.2}, neuron {:.2}, registry_rank {:.2}, prior {}, feedback {:+.2}, neuron_policy {}, {})",
                    score.topic_label.0,
                    score.score,
                    neuron_score,
                    ranked_workflow.registry_affinity,
                    neuron_prior
                        .map(|prior| format!("{:.2}", prior.score))
                        .unwrap_or_else(|| "none".into()),
                    feedback_delta,
                    neuron_policy,
                    ranked_workflow.registry_binding.reason,
                )),
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WorkflowDescriptor {
    workflow_id: &'static str,
    label: &'static str,
    keywords: &'static [&'static str],
    requires_confirmation: bool,
    action_mode: IntuitionActionMode,
}

#[derive(Debug, Clone, PartialEq)]
struct RankedIntuitionWorkflowCandidate {
    workflow_id: String,
    registry_binding: IntuitionWorkflowRegistryBinding,
    registry_affinity: f32,
    rank_bonus: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IntuitionWorkflowRegistryBinding {
    exists_in_registry: bool,
    missing_capability: Option<String>,
    requires_confirmation: bool,
    action_mode: IntuitionActionMode,
    reason: String,
}

fn default_intuition_workflow_registry() -> Vec<WorkflowDescriptor> {
    vec![
        WorkflowDescriptor {
            workflow_id: "workflow:memory-review",
            label: "Memory and provenance review",
            keywords: &[
                "memory",
                "recall",
                "context",
                "provenance",
                "adaptive",
                "remember",
            ],
            requires_confirmation: false,
            action_mode: IntuitionActionMode::Prepare,
        },
        WorkflowDescriptor {
            workflow_id: "workflow:engineering-change",
            label: "Engineering implementation lane",
            keywords: &[
                "rust",
                "worker",
                "pipeline",
                "implementation",
                "code",
                "router",
                "neuron",
                "intelligence",
                "hepta",
                "lane",
                "agent",
            ],
            requires_confirmation: false,
            action_mode: IntuitionActionMode::Prepare,
        },
        WorkflowDescriptor {
            workflow_id: "workflow:file-inspection",
            label: "File inspection and evidence gathering",
            keywords: &[
                "read",
                "inspect",
                "open",
                "show",
                "cat",
                "file",
                "architecture",
            ],
            requires_confirmation: false,
            action_mode: IntuitionActionMode::Prepare,
        },
        WorkflowDescriptor {
            workflow_id: "workflow:file-change",
            label: "File mutation planning",
            keywords: &[
                "write",
                "create",
                "append",
                "overwrite",
                "save",
                "edit",
                "patch",
                "release notes",
            ],
            requires_confirmation: true,
            action_mode: IntuitionActionMode::SuggestOnly,
        },
        WorkflowDescriptor {
            workflow_id: "workflow:tool-smoke-test",
            label: "Low-risk tool smoke test",
            keywords: &["echo", "repeat", "smoke test", "test tool"],
            requires_confirmation: false,
            action_mode: IntuitionActionMode::Prepare,
        },
    ]
}

fn rank_intuition_workflow_candidate(
    user_intent: &str,
    score: &TopicActivationScore,
    neuron_prior: Option<&WorkflowPrior>,
    registry: &[WorkflowDescriptor],
) -> RankedIntuitionWorkflowCandidate {
    if let Some(prior) =
        neuron_prior.filter(|prior| !prior.workflow_id.starts_with("workflow-bootstrap:"))
    {
        let binding = bind_intuition_workflow_to_registry(&prior.workflow_id, registry);
        return RankedIntuitionWorkflowCandidate {
            workflow_id: prior.workflow_id.clone(),
            registry_affinity: if binding.exists_in_registry { 1.0 } else { 0.0 },
            rank_bonus: if binding.exists_in_registry {
                0.10
            } else {
                0.0
            },
            registry_binding: binding,
        };
    }

    let mut candidates = registry
        .iter()
        .map(|descriptor| {
            let affinity = score_workflow_descriptor_for_intent(descriptor, user_intent, score);
            (descriptor, affinity)
        })
        .filter(|(_, affinity)| *affinity > 0.0)
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        right
            .1
            .total_cmp(&left.1)
            .then_with(|| left.0.workflow_id.cmp(right.0.workflow_id))
    });

    if let Some((descriptor, affinity)) = candidates.first() {
        let binding = bind_intuition_workflow_to_registry(descriptor.workflow_id, registry);
        return RankedIntuitionWorkflowCandidate {
            workflow_id: descriptor.workflow_id.to_string(),
            registry_binding: binding,
            registry_affinity: *affinity,
            rank_bonus: (affinity * 0.10).min(0.10),
        };
    }

    let fallback_id = neuron_prior
        .map(|prior| prior.workflow_id.clone())
        .unwrap_or_else(|| format!("workflow-bootstrap:{}", score.topic_id.0));
    RankedIntuitionWorkflowCandidate {
        workflow_id: fallback_id.clone(),
        registry_binding: bind_intuition_workflow_to_registry(&fallback_id, registry),
        registry_affinity: 0.0,
        rank_bonus: 0.0,
    }
}

fn score_workflow_descriptor_for_intent(
    descriptor: &WorkflowDescriptor,
    user_intent: &str,
    score: &TopicActivationScore,
) -> f32 {
    let haystack = format!(
        "{} {} {}",
        user_intent.to_ascii_lowercase(),
        score.topic_label.0.to_ascii_lowercase(),
        score.matched_terms.join(" ").to_ascii_lowercase(),
    );
    let matched_count = descriptor
        .keywords
        .iter()
        .filter(|keyword| haystack.contains(**keyword))
        .count();
    if matched_count == 0 {
        0.0
    } else {
        ((matched_count as f32) * 0.18 + score.score * 0.20).min(1.0)
    }
}

fn bind_intuition_workflow_to_registry(
    workflow_id: &str,
    registry: &[WorkflowDescriptor],
) -> IntuitionWorkflowRegistryBinding {
    if let Some(descriptor) = registry
        .iter()
        .find(|descriptor| descriptor.workflow_id == workflow_id)
    {
        return IntuitionWorkflowRegistryBinding {
            exists_in_registry: true,
            missing_capability: None,
            requires_confirmation: descriptor.requires_confirmation,
            action_mode: descriptor.action_mode,
            reason: format!(
                "bound to workflow registry entry '{}' ({}, action={}, requires_confirmation={})",
                descriptor.workflow_id,
                descriptor.label,
                format_intuition_action_mode(descriptor.action_mode),
                descriptor.requires_confirmation,
            ),
        };
    }

    IntuitionWorkflowRegistryBinding {
        exists_in_registry: false,
        missing_capability: Some("workflow_registry_binding_pending".into()),
        requires_confirmation: true,
        action_mode: IntuitionActionMode::SuggestOnly,
        reason: format!(
            "no workflow registry entry matched '{}'; prior remains suggest-only",
            workflow_id,
        ),
    }
}

fn build_bootstrap_skill_decisions(
    user_intent: &str,
    topic_scores: &[TopicActivationScore],
    activations: &[NeuronActivation],
    compressed_neurons: &[HeptaNeuron],
    workflow_priors: &[WorkflowPrior],
    intuition_feedback: &[IntuitionFeedbackRecord],
    registered_tools: &[ToolDescriptor],
    policy_bindings: &[IntuitionToolPolicyBinding],
    limit: usize,
) -> Vec<SkillActivationDecision> {
    if limit == 0 {
        return Vec::new();
    }

    topic_scores
        .iter()
        .enumerate()
        .take(limit)
        .map(|(index, score)| {
            let matching_activation = activations
                .iter()
                .find(|activation| activation.topic_id == score.topic_id);
            let neuron_ids = matching_activation
                .map(|activation| vec![activation.neuron_id.clone()])
                .unwrap_or_default();
            let workflow_id = workflow_priors
                .iter()
                .find(|prior| prior.source_topic_ids.contains(&score.topic_id))
                .or_else(|| workflow_priors.get(index))
                .map(|prior| prior.workflow_id.clone());
            let neuron = compressed_neurons
                .iter()
                .find(|neuron| neuron.topic_id == score.topic_id);
            let skill_prior = neuron.and_then(|neuron| neuron.skill_priors.first());
            let neuron_policy = neuron
                .map(|neuron| neuron.compression_policy_version.as_str())
                .filter(|policy| !policy.trim().is_empty())
                .unwrap_or("none");
            let activation_score = matching_activation
                .map(|activation| ((score.score + activation.final_score) / 2.0).clamp(0.0, 1.0))
                .unwrap_or(score.score);
            let base_skill_score = skill_prior
                .map(|prior| ((activation_score + prior.score) / 2.0).clamp(0.0, 1.0))
                .unwrap_or(activation_score);
            let preferred_skill_id = skill_prior
                .map(|prior| prior.skill_id.clone())
                .unwrap_or_else(|| format!("skill-bootstrap:{}:followup", score.topic_id.0));
            let ranked_skill = rank_intuition_skill_candidate(
                &preferred_skill_id,
                user_intent,
                &score.topic_label,
                registered_tools,
                policy_bindings,
            );
            let skill_id = ranked_skill.skill_id;
            let registry_binding = ranked_skill.registry_binding;
            let feedback_delta = compute_intuition_feedback_delta(
                intuition_feedback,
                Some(&score.topic_id),
                neuron.map(|neuron| &neuron.neuron_id),
                Some(&skill_id),
                workflow_id.as_deref(),
            );
            let skill_score = (base_skill_score + ranked_skill.rank_bonus + feedback_delta)
                .clamp(0.0, 1.0);

            SkillActivationDecision {
                skill_id,
                workflow_id,
                score: skill_score,
                exists_in_registry: registry_binding.exists_in_registry,
                missing_capability: registry_binding.missing_capability,
                risk_tier: registry_binding.risk_tier,
                requires_confirmation: registry_binding.requires_confirmation,
                action_mode: registry_binding.action_mode,
                source_topic_ids: vec![score.topic_id.clone()],
                source_neuron_ids: neuron_ids,
                reason: Some(format!(
                    "policy-aware intuition ranked a follow-up skill for topic '{}' (routing {:.2}, activation {:.2}, skill {:.2}, registry_rank {:.2}, policy_rank {:.2}, feedback {:+.2}{}, neuron_policy {}, {})",
                    score.topic_label.0,
                    score.score,
                    activation_score,
                    skill_score,
                    ranked_skill.registry_affinity,
                    ranked_skill.policy_affinity,
                    feedback_delta,
                    skill_prior
                        .map(|prior| format!(", compressed neuron prior {:.2}", prior.score))
                        .unwrap_or_default(),
                    neuron_policy,
                    registry_binding.reason,
                )),
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IntuitionSkillRegistryBinding {
    exists_in_registry: bool,
    missing_capability: Option<String>,
    risk_tier: Option<RiskTier>,
    policy_requirement: ApprovalRequirement,
    requires_confirmation: bool,
    action_mode: IntuitionActionMode,
    reason: String,
}

#[derive(Debug, Clone, PartialEq)]
struct RankedIntuitionSkillCandidate {
    skill_id: String,
    registry_binding: IntuitionSkillRegistryBinding,
    registry_affinity: f32,
    policy_affinity: f32,
    rank_bonus: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IntuitionToolPolicyBinding {
    tool_name: String,
    requirement: ApprovalRequirement,
    reason: String,
    matched_rule_id: Option<String>,
}

fn rank_intuition_skill_candidate(
    preferred_skill_id: &str,
    user_intent: &str,
    topic_label: &TopicLabel,
    registered_tools: &[ToolDescriptor],
    policy_bindings: &[IntuitionToolPolicyBinding],
) -> RankedIntuitionSkillCandidate {
    if let Some(tool) = find_registered_tool_for_skill_id(preferred_skill_id, registered_tools) {
        let policy = find_policy_binding_for_tool(&tool.name, policy_bindings);
        let registry_binding =
            bind_intuition_skill_to_runtime_registry(&tool.name, registered_tools, policy);
        let policy_affinity = policy_requirement_affinity(registry_binding.policy_requirement());
        return RankedIntuitionSkillCandidate {
            skill_id: tool.name.clone(),
            registry_binding,
            registry_affinity: 1.0,
            policy_affinity,
            rank_bonus: (0.08 + policy_affinity * 0.04).min(0.12),
        };
    }

    let mut candidates = registered_tools
        .iter()
        .filter_map(|tool| {
            let registry_affinity =
                score_registered_tool_for_intent(tool, user_intent, topic_label);
            (registry_affinity > 0.0).then(|| {
                let policy = find_policy_binding_for_tool(&tool.name, policy_bindings);
                let policy_affinity = policy
                    .map(|policy| policy_requirement_affinity(policy.requirement))
                    .unwrap_or_else(|| {
                        policy_requirement_affinity(tool.default_approval_requirement)
                    });
                let safety_affinity = safety_affinity_for_tool(tool);
                let total = registry_affinity + policy_affinity * 0.18 + safety_affinity * 0.12;
                (tool, policy, registry_affinity, policy_affinity, total)
            })
        })
        .collect::<Vec<_>>();

    candidates.sort_by(|left, right| {
        right
            .4
            .total_cmp(&left.4)
            .then_with(|| left.0.name.cmp(&right.0.name))
    });

    if let Some((tool, policy, registry_affinity, policy_affinity, _)) = candidates.first() {
        let registry_binding =
            bind_intuition_skill_to_runtime_registry(&tool.name, registered_tools, *policy);
        return RankedIntuitionSkillCandidate {
            skill_id: tool.name.clone(),
            registry_binding,
            registry_affinity: *registry_affinity,
            policy_affinity: *policy_affinity,
            rank_bonus: (registry_affinity * 0.08 + policy_affinity * 0.04).min(0.12),
        };
    }

    RankedIntuitionSkillCandidate {
        skill_id: preferred_skill_id.to_string(),
        registry_binding: bind_intuition_skill_to_runtime_registry(
            preferred_skill_id,
            registered_tools,
            None,
        ),
        registry_affinity: 0.0,
        policy_affinity: 0.0,
        rank_bonus: 0.0,
    }
}

fn bind_intuition_skill_to_runtime_registry(
    skill_id: &str,
    registered_tools: &[ToolDescriptor],
    policy_binding: Option<&IntuitionToolPolicyBinding>,
) -> IntuitionSkillRegistryBinding {
    if let Some(tool) = find_registered_tool_for_skill_id(skill_id, registered_tools) {
        let requirement = policy_binding
            .map(|binding| binding.requirement)
            .unwrap_or(tool.default_approval_requirement);
        let requires_confirmation = requirement != ApprovalRequirement::None;
        let action_mode = if requires_confirmation {
            IntuitionActionMode::SuggestOnly
        } else if tool.execution_metadata.read_only && tool.execution_metadata.idempotent {
            IntuitionActionMode::Prepare
        } else {
            IntuitionActionMode::SuggestOnly
        };

        return IntuitionSkillRegistryBinding {
            exists_in_registry: true,
            missing_capability: None,
            risk_tier: Some(tool.risk_tier),
            policy_requirement: requirement,
            requires_confirmation,
            action_mode,
            reason: format!(
                "bound to runtime tool registry entry '{}' (risk={}, approval={}, policy_rule={}, policy_reason=\"{}\")",
                tool.name,
                format_skill_registry_risk_tier(tool.risk_tier),
                format_approval_requirement(requirement),
                policy_binding
                    .and_then(|binding| binding.matched_rule_id.as_deref())
                    .unwrap_or("default"),
                summarize_line(
                    policy_binding
                        .map(|binding| binding.reason.as_str())
                        .unwrap_or("tool default approval requirement"),
                    72,
                ),
            ),
        };
    }

    IntuitionSkillRegistryBinding {
        exists_in_registry: false,
        missing_capability: Some("bootstrap_skill_registry_binding_pending".into()),
        risk_tier: Some(RiskTier::Low),
        policy_requirement: ApprovalRequirement::Ask,
        requires_confirmation: true,
        action_mode: IntuitionActionMode::SuggestOnly,
        reason: format!(
            "no runtime registry entry matched skill '{}'; suggestion remains gated",
            skill_id,
        ),
    }
}

impl IntuitionSkillRegistryBinding {
    fn policy_requirement(&self) -> ApprovalRequirement {
        self.policy_requirement
    }
}

fn find_policy_binding_for_tool<'a>(
    tool_name: &str,
    policy_bindings: &'a [IntuitionToolPolicyBinding],
) -> Option<&'a IntuitionToolPolicyBinding> {
    policy_bindings
        .iter()
        .find(|binding| binding.tool_name == tool_name)
}

fn policy_requirement_affinity(requirement: ApprovalRequirement) -> f32 {
    match requirement {
        ApprovalRequirement::None => 1.0,
        ApprovalRequirement::Ask => 0.55,
        ApprovalRequirement::Deny => 0.05,
    }
}

fn safety_affinity_for_tool(tool: &ToolDescriptor) -> f32 {
    let mut score = 0.0_f32;
    if tool.execution_metadata.read_only {
        score += 0.45;
    }
    if tool.execution_metadata.idempotent {
        score += 0.35;
    }
    if !tool.execution_metadata.destructive {
        score += 0.20;
    }
    score.min(1.0)
}

fn find_registered_tool_for_skill_id<'a>(
    skill_id: &str,
    registered_tools: &'a [ToolDescriptor],
) -> Option<&'a ToolDescriptor> {
    let normalized = normalize_skill_tool_selector(skill_id);
    registered_tools.iter().find(|tool| tool.name == normalized)
}

fn normalize_skill_tool_selector(skill_id: &str) -> &str {
    skill_id
        .strip_prefix("tool:")
        .or_else(|| skill_id.strip_prefix("skill-tool:"))
        .or_else(|| skill_id.strip_prefix("runtime-tool:"))
        .unwrap_or(skill_id)
}

fn score_registered_tool_for_intent(
    tool: &ToolDescriptor,
    user_intent: &str,
    topic_label: &TopicLabel,
) -> f32 {
    let intent_haystack = format!(
        "{} {}",
        user_intent.to_ascii_lowercase(),
        topic_label.0.to_ascii_lowercase(),
    );

    match tool.name.as_str() {
        "read_file"
            if contains_any(
                &intent_haystack,
                &["read", "inspect", "open", "show", "cat"],
            ) =>
        {
            if intent_haystack.contains("file") || intent_haystack.contains("path") {
                1.0
            } else {
                0.62
            }
        }
        "write_file"
            if contains_any(
                &intent_haystack,
                &[
                    "write",
                    "save",
                    "create",
                    "append",
                    "overwrite",
                    "edit",
                    "patch",
                ],
            ) =>
        {
            if intent_haystack.contains("file") || intent_haystack.contains("path") {
                1.0
            } else {
                0.66
            }
        }
        "echo"
            if contains_any(
                &intent_haystack,
                &["echo", "repeat", "smoke test", "test tool"],
            ) =>
        {
            0.82
        }
        _ => 0.0,
    }
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

fn format_approval_requirement(requirement: ApprovalRequirement) -> &'static str {
    match requirement {
        ApprovalRequirement::None => "none",
        ApprovalRequirement::Ask => "ask",
        ApprovalRequirement::Deny => "deny",
    }
}

fn format_skill_registry_risk_tier(risk_tier: RiskTier) -> &'static str {
    match risk_tier {
        RiskTier::Low => "low",
        RiskTier::Medium => "medium",
        RiskTier::High => "high",
    }
}

fn format_intuition_action_mode(mode: IntuitionActionMode) -> &'static str {
    match mode {
        IntuitionActionMode::SuggestOnly => "suggest_only",
        IntuitionActionMode::Prepare => "prepare",
        IntuitionActionMode::ExecuteAllowed => "execute_allowed",
    }
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

fn record_bootstrap_neuron_link(
    source_topic_session: &TopicSession,
    link_kind: NeuronLinkKind,
    link_reason: &str,
    source_topic_session_ids: &mut Vec<String>,
    source_neuron_ids: &mut Vec<NeuronId>,
    source_link_kinds: &mut Vec<NeuronLinkKind>,
    source_link_reasons: &mut Vec<String>,
) {
    bootstrap_neuron_propagation::record_source_link(
        source_topic_session,
        link_kind,
        link_reason,
        source_topic_session_ids,
        source_neuron_ids,
        source_link_kinds,
        source_link_reasons,
    );
}

fn infer_bootstrap_propagation_link(
    source: &TopicSession,
    target: &TopicSession,
    co_active: bool,
) -> Option<(NeuronLinkKind, f32, String)> {
    if let Some(edge) = bootstrap_topic_graph_edge(source, &target.topic_session_id) {
        let (kind, reason) = match edge.kind {
            TopicGraphEdgeKind::CoActivation => (
                NeuronLinkKind::WorkflowAdjacency,
                format!(
                    "bootstrap stored co-activation edge into '{}' strength {:.2}",
                    target.topic_label.0, edge.weight,
                ),
            ),
            TopicGraphEdgeKind::SplitComponent => (
                NeuronLinkKind::TemporalContinuation,
                format!(
                    "bootstrap stored split-component edge into '{}' strength {:.2}",
                    target.topic_label.0, edge.weight,
                ),
            ),
            TopicGraphEdgeKind::MergedInto | TopicGraphEdgeKind::HasComponent => (
                NeuronLinkKind::CausalDependency,
                format!(
                    "bootstrap stored merge-component edge into '{}' strength {:.2}",
                    target.topic_label.0, edge.weight,
                ),
            ),
            _ => (
                NeuronLinkKind::SemanticSimilarity,
                format!(
                    "bootstrap stored {} edge into '{}' strength {:.2}",
                    bootstrap_topic_graph_edge_relation(&edge),
                    target.topic_label.0,
                    edge.weight,
                ),
            ),
        };
        return Some((kind, edge.weight.min(0.46), reason));
    }

    let overlap = topic_session_label_overlap(source, target);

    if co_active {
        let strength = (0.24 + overlap * 0.16).min(0.38);
        let reason = if overlap > 0.0 {
            format!(
                "bootstrap co-routed adjacency with semantic overlap {:.2}",
                overlap,
            )
        } else {
            "bootstrap co-routed adjacency from the same mixed turn".to_string()
        };
        return Some((NeuronLinkKind::WorkflowAdjacency, strength, reason));
    }

    (overlap >= 0.25).then(|| {
        (
            NeuronLinkKind::SemanticSimilarity,
            (0.22 + overlap * 0.24).min(0.42),
            format!("bootstrap semantic overlap {:.2}", overlap),
        )
    })
}

fn infer_bootstrap_inhibition_link(
    source: &TopicSession,
    target: &TopicSession,
    marker: &'static str,
) -> Option<(f32, String)> {
    let overlap = topic_session_label_overlap(source, target);
    let strength = (0.30 + overlap * 0.18).min(0.48);

    Some((
        strength,
        if overlap > 0.0 {
            format!(
                "bootstrap contrast '{}' suppressed secondary topic with overlap {:.2}",
                marker, overlap,
            )
        } else {
            format!(
                "bootstrap contrast '{}' suppressed secondary topic from the same routed turn",
                marker,
            )
        },
    ))
}

fn infer_bootstrap_neuron_propagation_link(
    source: &BootstrapNeuronSeed,
    target: &BootstrapNeuronSeed,
    co_active: bool,
) -> Option<(NeuronLinkKind, f32, String)> {
    bootstrap_neuron_propagation::infer_link(source, target, co_active)
}

fn infer_bootstrap_neuron_inhibition_link(
    source: &BootstrapNeuronSeed,
    target: &BootstrapNeuronSeed,
    marker: &'static str,
) -> Option<(f32, String)> {
    if let Some(link) = source
        .neuron
        .links
        .iter()
        .find(|link| link.target_neuron_id == target.neuron.neuron_id)
        .filter(|link| {
            matches!(
                link.kind,
                NeuronLinkKind::Conflict | NeuronLinkKind::Inhibition
            )
        })
    {
        let relation = link
            .relation
            .as_deref()
            .unwrap_or("compressed_neuron_conflict");
        return Some((
            link.strength.min(0.48),
            format!(
                "bootstrap contrast '{}' followed compressed neuron inhibition '{}' into '{}' strength {:.2}",
                marker, relation, target.neuron.topic_label.0, link.strength,
            ),
        ));
    }

    infer_bootstrap_inhibition_link(&source.topic_session, &target.topic_session, marker)
}

fn detect_bootstrap_inhibition_marker(query_text: Option<&str>) -> Option<&'static str> {
    let lower = query_text?.to_ascii_lowercase();

    [" but not ", " instead of ", " rather than ", " except "]
        .into_iter()
        .find(|marker| lower.contains(marker))
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

mod bootstrap_planner {
    use std::collections::{BTreeMap, BTreeSet};

    use hepta_core::{TopicLabel, TopicSession, TopicSessionStatus};

    use super::{
        BOOTSTRAP_SEMANTIC_HINT_PREFIX, BootstrapTopicCandidateRoute,
        BootstrapTopicGraphRouteCandidate, BootstrapTopicRoutePlan, MAX_BOOTSTRAP_SEMANTIC_HINTS,
        SessionSnapshot, allocate_bootstrap_topic_id, allocate_bootstrap_topic_session_id,
        bootstrap_topic_graph_edge, bootstrap_topic_graph_edge_relation,
        detect_bootstrap_merge_marker, detect_bootstrap_split_marker, slugify_identifier,
        topic_label_for_session,
    };

    pub(super) fn plan_bootstrap_topic_routes(
        existing_sessions: &[TopicSession],
        session_indices: &[usize],
        session_id: &str,
        query_text: Option<&str>,
        session: &SessionSnapshot,
        effective_limit: usize,
        topic_score: f32,
        learned_route_planning_signals: Vec<hepta_intelligence::LearnedSemanticRouterSignal>,
        semantic_router_id: Option<&str>,
    ) -> BootstrapTopicRoutePlan {
        let candidate_labels =
            bootstrap_candidate_topic_labels(query_text, session, effective_limit);
        let merge_marker = detect_bootstrap_merge_marker(query_text);
        let split_marker = detect_bootstrap_split_marker(query_text);
        let implicit_routes = (candidate_labels.len() == 1)
            .then(|| {
                infer_bootstrap_implicit_topic_routes(
                    existing_sessions,
                    session_indices,
                    query_text,
                    effective_limit,
                    topic_score,
                )
            })
            .unwrap_or_default();

        let mut materializer = RuntimeBootstrapTopicRouteMaterializer {
            existing_sessions,
            session_indices,
            session_id,
            topic_score,
            effective_limit,
        };
        let router_registry = hepta_intelligence::SemanticRouterRegistry::new();
        let router = semantic_router_id
            .map(|router_id| router_registry.select(Some(router_id)))
            .unwrap_or_else(|| {
                router_registry
                    .select_for_learned_signal_count(learned_route_planning_signals.len())
            });
        let router_input = hepta_intelligence::BootstrapSemanticRouterInput {
            implicit_routes,
            candidate_labels,
            merge_marker,
            split_marker,
            limit: effective_limit,
            learned_signals: learned_route_planning_signals,
        };
        let planner_outcome = router.route(router_input, &mut materializer);

        BootstrapTopicRoutePlan {
            routes: planner_outcome.routes,
            selected_existing_indices: planner_outcome.selected_existing_indices,
            merged_source_indices: planner_outcome.merged_source_indices,
            merge_marker,
            split_marker,
        }
    }

    struct RuntimeBootstrapTopicRouteMaterializer<'a> {
        existing_sessions: &'a [TopicSession],
        session_indices: &'a [usize],
        session_id: &'a str,
        topic_score: f32,
        effective_limit: usize,
    }

    impl hepta_intelligence::BootstrapTopicRouteMaterializer
        for RuntimeBootstrapTopicRouteMaterializer<'_>
    {
        fn build_candidate_route(
            &mut self,
            selected_existing_indices: &BTreeSet<usize>,
            candidate_label: &str,
            has_prior_routes: bool,
        ) -> BootstrapTopicCandidateRoute {
            build_bootstrap_topic_candidate_route(
                self.existing_sessions,
                self.session_indices,
                selected_existing_indices,
                self.session_id,
                candidate_label,
                self.topic_score,
                has_prior_routes,
            )
        }

        fn build_merged_route(
            &mut self,
            routes: &[BootstrapTopicCandidateRoute],
            marker: &'static str,
        ) -> BootstrapTopicCandidateRoute {
            build_bootstrap_merged_topic_route(
                self.existing_sessions,
                self.session_id,
                routes,
                self.topic_score,
                marker,
            )
        }

        fn infer_graph_routes(
            &mut self,
            selected_existing_indices: &BTreeSet<usize>,
            routes: &[BootstrapTopicCandidateRoute],
        ) -> Vec<BootstrapTopicCandidateRoute> {
            bootstrap_topic_graph_routing::infer_bootstrap_topic_graph_routes(
                self.existing_sessions,
                self.session_indices,
                selected_existing_indices,
                routes,
                self.effective_limit,
            )
        }
    }

    pub(super) fn extract_semantic_terms(value: &str, limit: usize) -> Vec<String> {
        hepta_intelligence::extract_semantic_terms(value, limit)
    }

    fn bootstrap_candidate_topic_labels(
        query_text: Option<&str>,
        session: &SessionSnapshot,
        limit: usize,
    ) -> Vec<String> {
        hepta_intelligence::bootstrap_candidate_topic_labels(
            query_text,
            &topic_label_for_session(session),
            limit,
        )
    }

    fn infer_bootstrap_implicit_topic_routes(
        existing_sessions: &[TopicSession],
        session_indices: &[usize],
        query_text: Option<&str>,
        limit: usize,
        topic_score: f32,
    ) -> Vec<BootstrapTopicCandidateRoute> {
        let query = match query_text.map(str::trim).filter(|query| !query.is_empty()) {
            Some(query) => query,
            None => return Vec::new(),
        };
        if limit <= 1 {
            return Vec::new();
        }

        let query_terms = extract_semantic_terms(query, 12);
        if query_terms.len() < 4 {
            return Vec::new();
        }

        let matches = hepta_intelligence::select_bootstrap_implicit_topic_match_candidates(
            session_indices.iter().copied().map(|index| {
                let topic_session = &existing_sessions[index];
                let features = bootstrap_candidate_matching::compute_topic_match_features(
                    query,
                    topic_session,
                );
                hepta_intelligence::BootstrapImplicitTopicMatchCandidate {
                    index,
                    score: features.score,
                    matched_terms: features.matched_terms,
                    was_active: matches!(topic_session.status, TopicSessionStatus::Active),
                    last_active_unix_ms: topic_session.last_active_unix_ms,
                }
            }),
            limit,
            0.52,
            2,
        );

        matches
            .into_iter()
            .map(|match_candidate| {
                let index = match_candidate.index;
                let score = match_candidate.score;
                let overlap_terms = match_candidate.matched_terms;
                let topic_session = &existing_sessions[index];
                let was_active = matches!(topic_session.status, TopicSessionStatus::Active);
                let reason = if was_active {
                    format!(
                        "bootstrap router implicitly kept '{}' foregrounded from full-query semantic coverage {:.2}",
                        topic_session.topic_label.0, score,
                    )
                } else {
                    format!(
                        "bootstrap router implicitly revived '{}' from full-query semantic coverage {:.2}",
                        topic_session.topic_label.0, score,
                    )
                };

                BootstrapTopicCandidateRoute {
                    topic_id: topic_session.topic_id.clone(),
                    topic_label: topic_session.topic_label.clone(),
                    topic_session_id: topic_session.topic_session_id.clone(),
                    matched_terms: overlap_terms.into_iter().take(3).collect(),
                    semantic_hints:
                        bootstrap_candidate_matching::extract_bootstrap_semantic_hints_from_overlap(
                            query,
                            topic_session,
                            MAX_BOOTSTRAP_SEMANTIC_HINTS,
                        ),
                    topic_score: topic_score.max(score),
                    reason,
                    existing_index: Some(index),
                    was_active,
                    graph_routed: false,
                }
            })
            .collect()
    }

    fn build_bootstrap_merged_topic_route(
        existing_sessions: &[TopicSession],
        session_id: &str,
        source_routes: &[BootstrapTopicCandidateRoute],
        topic_score: f32,
        marker: &'static str,
    ) -> BootstrapTopicCandidateRoute {
        let mut labels = Vec::new();
        let mut seen = BTreeSet::new();
        let mut matched_terms = Vec::new();

        for route in source_routes {
            if seen.insert(route.topic_label.0.clone()) {
                labels.push(route.topic_label.0.clone());
            }
            for term in &route.matched_terms {
                if matched_terms.iter().all(|existing| existing != term) {
                    matched_terms.push(term.clone());
                    if matched_terms.len() >= 3 {
                        break;
                    }
                }
            }
        }

        let merged_label = labels.join(" + ");
        let merged_slug = slugify_identifier(&merged_label);
        if let Some((index, topic_session)) =
            existing_sessions
                .iter()
                .enumerate()
                .find(|(_, topic_session)| {
                    slugify_identifier(&topic_session.topic_label.0) == merged_slug
                })
        {
            return BootstrapTopicCandidateRoute {
                topic_id: topic_session.topic_id.clone(),
                topic_label: topic_session.topic_label.clone(),
                topic_session_id: topic_session.topic_session_id.clone(),
                matched_terms,
                semantic_hints: Vec::new(),
                topic_score,
                reason: format!(
                    "bootstrap router merged '{}' from {} source topic sessions via explicit merge signal '{}'",
                    topic_session.topic_label.0,
                    source_routes.len(),
                    marker.trim(),
                ),
                existing_index: Some(index),
                was_active: matches!(topic_session.status, TopicSessionStatus::Active),
                graph_routed: false,
            };
        }

        let topic_id =
            allocate_bootstrap_topic_id(session_id, &merged_slug, true, existing_sessions);
        let topic_session_id =
            allocate_bootstrap_topic_session_id(session_id, &merged_slug, true, existing_sessions);

        BootstrapTopicCandidateRoute {
            topic_id,
            topic_label: TopicLabel(merged_label.clone()),
            topic_session_id,
            matched_terms,
            semantic_hints: Vec::new(),
            topic_score,
            reason: format!(
                "bootstrap router merged '{}' from {} source topic sessions via explicit merge signal '{}'",
                merged_label,
                source_routes.len(),
                marker.trim(),
            ),
            existing_index: None,
            was_active: false,
            graph_routed: false,
        }
    }

    fn build_bootstrap_topic_candidate_route(
        existing_sessions: &[TopicSession],
        session_indices: &[usize],
        selected_existing_indices: &BTreeSet<usize>,
        session_id: &str,
        candidate_label: &str,
        topic_score: f32,
        has_prior_routes: bool,
    ) -> BootstrapTopicCandidateRoute {
        let match_candidates = session_indices
            .iter()
            .copied()
            .filter(|index| !selected_existing_indices.contains(index))
            .filter_map(|index| {
                let features = bootstrap_candidate_matching::compute_topic_match_features(
                    candidate_label,
                    &existing_sessions[index],
                );
                (features.score > 0.0)
                    .then_some(hepta_intelligence::BootstrapTopicMatchCandidate { index, features })
            });

        if let Some(match_candidate) =
            hepta_intelligence::select_bootstrap_topic_match_candidate(match_candidates, 0.55)
        {
            let selected_index = match_candidate.index;
            let features = match_candidate.features;
            let selected = &existing_sessions[selected_index];
            let was_active = matches!(selected.status, TopicSessionStatus::Active);
            let reason = if was_active {
                format!(
                    "bootstrap router kept '{}' foregrounded with semantic term-overlap {:.2}",
                    selected.topic_label.0, features.score,
                )
            } else {
                format!(
                    "bootstrap router revived '{}' with semantic term-overlap {:.2}",
                    selected.topic_label.0, features.score,
                )
            };

            return BootstrapTopicCandidateRoute {
                topic_id: selected.topic_id.clone(),
                topic_label: selected.topic_label.clone(),
                topic_session_id: selected.topic_session_id.clone(),
                matched_terms: features.matched_terms,
                semantic_hints:
                    bootstrap_candidate_matching::extract_bootstrap_semantic_hints_for_match(
                        candidate_label,
                        selected,
                        MAX_BOOTSTRAP_SEMANTIC_HINTS,
                    ),
                topic_score: topic_score.max(features.score),
                reason,
                existing_index: Some(selected_index),
                was_active,
                graph_routed: false,
            };
        }

        let candidate_slug = slugify_identifier(candidate_label);
        let has_existing_sessions = !session_indices.is_empty() || has_prior_routes;
        let topic_id = allocate_bootstrap_topic_id(
            session_id,
            &candidate_slug,
            has_existing_sessions,
            existing_sessions,
        );
        let topic_session_id = allocate_bootstrap_topic_session_id(
            session_id,
            &candidate_slug,
            has_existing_sessions,
            existing_sessions,
        );

        BootstrapTopicCandidateRoute {
            topic_id,
            topic_label: TopicLabel(candidate_label.to_string()),
            topic_session_id,
            matched_terms: extract_semantic_terms(candidate_label, 3),
            semantic_hints: Vec::new(),
            topic_score,
            reason: format!(
                "bootstrap router created '{}' because no matching topic session was found for session '{}'",
                candidate_label, session_id,
            ),
            existing_index: None,
            was_active: false,
            graph_routed: false,
        }
    }

    mod bootstrap_topic_graph_routing {
        use std::collections::{BTreeMap, BTreeSet};

        use hepta_core::TopicSession;

        use super::{
            BootstrapTopicCandidateRoute, BootstrapTopicGraphRouteCandidate,
            bootstrap_topic_graph_edge, bootstrap_topic_graph_edge_relation,
        };

        pub(super) fn infer_bootstrap_topic_graph_routes(
            existing_sessions: &[TopicSession],
            session_indices: &[usize],
            selected_existing_indices: &BTreeSet<usize>,
            routes: &[BootstrapTopicCandidateRoute],
            limit: usize,
        ) -> Vec<BootstrapTopicCandidateRoute> {
            if routes.is_empty() || routes.len() >= limit {
                return Vec::new();
            }

            let candidates = collect_bootstrap_topic_graph_route_candidates(
                existing_sessions,
                session_indices,
                selected_existing_indices,
                routes,
            );

            let ranked_target_indices = hepta_intelligence::rank_bootstrap_graph_route_candidates(
                candidates.iter().map(|candidate| {
                    hepta_intelligence::BootstrapGraphRouteRankCandidate {
                        target_index: candidate.target_index,
                        strength: candidate.strength,
                        last_active_unix_ms: existing_sessions[candidate.target_index]
                            .last_active_unix_ms,
                    }
                }),
                limit.saturating_sub(routes.len()),
            );
            let candidates_by_target_index = candidates
                .into_iter()
                .map(|candidate| (candidate.target_index, candidate))
                .collect::<BTreeMap<_, _>>();

            ranked_target_indices
                .into_iter()
                .filter_map(|target_index| candidates_by_target_index.get(&target_index).cloned())
                .map(|candidate| {
                    let topic_session = &existing_sessions[candidate.target_index];
                    BootstrapTopicCandidateRoute::from_graph_link(
                        topic_session,
                        candidate.target_index,
                        candidate.source_score,
                        hepta_intelligence::BootstrapTopicGraphLink {
                            strength: candidate.strength,
                            matched_terms: candidate.matched_terms,
                            reason: candidate.reason,
                        },
                    )
                })
                .collect()
        }

        fn collect_bootstrap_topic_graph_route_candidates(
            existing_sessions: &[TopicSession],
            session_indices: &[usize],
            selected_existing_indices: &BTreeSet<usize>,
            routes: &[BootstrapTopicCandidateRoute],
        ) -> Vec<BootstrapTopicGraphRouteCandidate> {
            session_indices
                .iter()
                .copied()
                .filter(|index| !selected_existing_indices.contains(index))
                .filter_map(|target_index| {
                    infer_bootstrap_topic_graph_route_candidate(
                        existing_sessions,
                        routes,
                        target_index,
                    )
                })
                .collect()
        }

        fn infer_bootstrap_topic_graph_route_candidate(
            existing_sessions: &[TopicSession],
            routes: &[BootstrapTopicCandidateRoute],
            target_index: usize,
        ) -> Option<BootstrapTopicGraphRouteCandidate> {
            let target = &existing_sessions[target_index];
            let best = routes
                .iter()
                .filter_map(|route| {
                    infer_bootstrap_topic_graph_link_for_target(existing_sessions, route, target)
                })
                .max_by(|left, right| left.1.total_cmp(&right.1));

            best.map(|(source_score, strength, matched_terms, reason)| {
                BootstrapTopicGraphRouteCandidate {
                    target_index,
                    source_score,
                    strength,
                    matched_terms,
                    reason,
                }
            })
        }

        fn infer_bootstrap_topic_graph_link_for_target(
            existing_sessions: &[TopicSession],
            route: &BootstrapTopicCandidateRoute,
            target: &TopicSession,
        ) -> Option<(f32, f32, Vec<String>, String)> {
            let persisted = route.existing_index.and_then(|source_index| {
                infer_bootstrap_persisted_topic_graph_link(&existing_sessions[source_index], target)
            });
            let heuristic = infer_bootstrap_topic_graph_link(route, target);

            persisted
                .into_iter()
                .chain(heuristic)
                .map(|(strength, matched_terms, reason)| {
                    (route.topic_score, strength, matched_terms, reason)
                })
                .max_by(|left, right| left.1.total_cmp(&right.1))
        }

        fn infer_bootstrap_persisted_topic_graph_link(
            source_topic_session: &TopicSession,
            target: &TopicSession,
        ) -> Option<(f32, Vec<String>, String)> {
            let edge = bootstrap_topic_graph_edge(source_topic_session, &target.topic_session_id)?;
            let link = hepta_intelligence::infer_bootstrap_persisted_topic_graph_link(
                &source_topic_session.topic_label.0,
                &target.topic_label.0,
                edge.kind,
                bootstrap_topic_graph_edge_relation(&edge),
                edge.weight,
            );

            Some((link.strength, link.matched_terms, link.reason))
        }

        fn infer_bootstrap_topic_graph_link(
            source_route: &BootstrapTopicCandidateRoute,
            target: &TopicSession,
        ) -> Option<(f32, Vec<String>, String)> {
            let link = hepta_intelligence::infer_bootstrap_heuristic_topic_graph_link(
                &source_route.topic_label.0,
                source_route.was_active,
                &source_route.reason,
                &target.topic_label.0,
                target.status,
            )?;

            Some((link.strength, link.matched_terms, link.reason))
        }
    }

    mod bootstrap_candidate_matching {
        use super::{TopicSession, slugify_identifier};

        pub(super) fn extract_bootstrap_semantic_hints_for_match(
            candidate_label: &str,
            topic_session: &TopicSession,
            limit: usize,
        ) -> Vec<String> {
            hepta_intelligence::extract_bootstrap_semantic_hints_for_match(
                candidate_label,
                topic_session,
                limit,
            )
        }

        pub(super) fn extract_bootstrap_semantic_hints_from_overlap(
            candidate_label: &str,
            topic_session: &TopicSession,
            limit: usize,
        ) -> Vec<String> {
            hepta_intelligence::extract_bootstrap_semantic_hints_from_overlap(
                candidate_label,
                topic_session,
                limit,
            )
        }

        pub(super) fn compute_topic_match_features(
            candidate_label: &str,
            topic_session: &TopicSession,
        ) -> hepta_intelligence::BootstrapTopicMatchFeatures {
            let candidate_slug = slugify_identifier(candidate_label);
            let topic_label_slug = slugify_identifier(&topic_session.topic_label.0);
            let features = hepta_intelligence::compute_bootstrap_topic_match_features(
                candidate_label,
                &candidate_slug,
                topic_session,
                &topic_label_slug,
            );

            features
        }
    }

    fn bootstrap_semantic_hint_key(term: &str) -> String {
        format!(
            "{}{}",
            BOOTSTRAP_SEMANTIC_HINT_PREFIX,
            slugify_identifier(term)
        )
    }

    pub(super) fn merge_bootstrap_topic_session_semantic_hints(
        entities: &mut BTreeMap<String, String>,
        semantic_hints: &[String],
    ) {
        for hint in semantic_hints {
            entities.insert(bootstrap_semantic_hint_key(hint), hint.clone());
        }

        let semantic_hint_keys = entities
            .keys()
            .filter(|key| key.starts_with(BOOTSTRAP_SEMANTIC_HINT_PREFIX))
            .cloned()
            .collect::<Vec<_>>();

        if semantic_hint_keys.len() <= MAX_BOOTSTRAP_SEMANTIC_HINTS {
            return;
        }

        for key in semantic_hint_keys
            .into_iter()
            .skip(MAX_BOOTSTRAP_SEMANTIC_HINTS)
        {
            entities.remove(&key);
        }
    }
}

mod bootstrap_graph_persistence {
    use std::collections::BTreeSet;

    use hepta_core::{TopicGraphEdgeKind, TopicSession, TopicShiftKind};

    use super::{
        BootstrapTopicCandidateRoute, TopicGraphState, bootstrap_topic_graph_edge_weight,
        bootstrap_topic_graph_relation_for_shift_kind, upsert_bootstrap_topic_graph_edge,
    };

    pub(super) fn persist_bootstrap_topic_graph_semantics(
        topic_graph_state: &mut TopicGraphState,
        sessions: &mut [TopicSession],
        routes: &[BootstrapTopicCandidateRoute],
        merged_source_indices: &BTreeSet<usize>,
        shift_kind: TopicShiftKind,
        now: u64,
    ) {
        let route_indices = collect_bootstrap_topic_graph_route_indices(sessions, routes);

        persist_bootstrap_merged_topic_graph_edges(
            topic_graph_state,
            sessions,
            &route_indices,
            merged_source_indices,
            shift_kind,
            now,
        );
        persist_bootstrap_route_pair_graph_edges(
            topic_graph_state,
            sessions,
            &route_indices,
            shift_kind,
            now,
        );
    }

    fn collect_bootstrap_topic_graph_route_indices(
        sessions: &[TopicSession],
        routes: &[BootstrapTopicCandidateRoute],
    ) -> Vec<usize> {
        routes
            .iter()
            .filter_map(|route| {
                sessions.iter().position(|topic_session| {
                    topic_session.topic_session_id == route.topic_session_id
                })
            })
            .collect()
    }

    fn persist_bootstrap_merged_topic_graph_edges(
        topic_graph_state: &mut TopicGraphState,
        sessions: &[TopicSession],
        route_indices: &[usize],
        merged_source_indices: &BTreeSet<usize>,
        shift_kind: TopicShiftKind,
        now: u64,
    ) {
        if !matches!(shift_kind, TopicShiftKind::Merged) {
            return;
        }

        let Some(&merged_index) = route_indices.first() else {
            return;
        };
        let merged_topic_session_id = sessions[merged_index].topic_session_id.clone();

        for source_index in merged_source_indices {
            if *source_index >= sessions.len() || *source_index == merged_index {
                continue;
            }

            let source_topic_session_id = sessions[*source_index].topic_session_id.clone();
            upsert_bootstrap_topic_graph_edge(
                topic_graph_state,
                &source_topic_session_id,
                &merged_topic_session_id,
                TopicGraphEdgeKind::MergedInto,
                0.92,
                now,
            );
            upsert_bootstrap_topic_graph_edge(
                topic_graph_state,
                &merged_topic_session_id,
                &source_topic_session_id,
                TopicGraphEdgeKind::HasComponent,
                0.92,
                now,
            );
        }
    }

    fn persist_bootstrap_route_pair_graph_edges(
        topic_graph_state: &mut TopicGraphState,
        sessions: &[TopicSession],
        route_indices: &[usize],
        shift_kind: TopicShiftKind,
        now: u64,
    ) {
        if route_indices.len() < 2 {
            return;
        }

        let relation = bootstrap_topic_graph_relation_for_shift_kind(shift_kind);

        for left in 0..route_indices.len() {
            for right in (left + 1)..route_indices.len() {
                let left_index = route_indices[left];
                let right_index = route_indices[right];
                if left_index == right_index {
                    continue;
                }

                let weight = bootstrap_topic_graph_edge_weight(
                    &sessions[left_index],
                    &sessions[right_index],
                    shift_kind,
                );
                let left_topic_session_id = sessions[left_index].topic_session_id.clone();
                let right_topic_session_id = sessions[right_index].topic_session_id.clone();
                upsert_bootstrap_topic_graph_edge(
                    topic_graph_state,
                    &left_topic_session_id,
                    &right_topic_session_id,
                    relation,
                    weight,
                    now,
                );
                upsert_bootstrap_topic_graph_edge(
                    topic_graph_state,
                    &right_topic_session_id,
                    &left_topic_session_id,
                    relation,
                    weight,
                    now,
                );
            }
        }
    }
}

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

mod bootstrap_route_persistence {
    use std::collections::{BTreeMap, BTreeSet};

    use hepta_core::{SessionId, TopicSession, TopicSessionStatus, TranscriptSpanRef};

    use super::{
        BootstrapTopicCandidateRoute, BootstrapTopicRoutePersistInputs, bootstrap_memory_refs,
        bootstrap_open_loops, bootstrap_planner::merge_bootstrap_topic_session_semantic_hints,
        merge_bootstrap_topic_session_transcript_evidence,
    };

    pub(super) fn prepare_bootstrap_topic_route_persist_inputs(
        session_id: &str,
        recent_entry_count: usize,
        durable_memory_hit_count: usize,
        transcript_evidence: &[TranscriptSpanRef],
        now: u64,
    ) -> BootstrapTopicRoutePersistInputs {
        BootstrapTopicRoutePersistInputs {
            linked_surface_session_id: SessionId(session_id.to_string()),
            linked_transcript_spans: transcript_evidence.to_vec(),
            open_loops: bootstrap_open_loops(recent_entry_count),
            durable_memory_refs: bootstrap_memory_refs(durable_memory_hit_count),
            now,
        }
    }

    pub(super) fn apply_bootstrap_topic_session_status_transitions(
        sessions: &mut [TopicSession],
        session_indices: &[usize],
        selected_existing_indices: &BTreeSet<usize>,
        merged_source_indices: &BTreeSet<usize>,
    ) {
        for index in session_indices {
            if merged_source_indices.contains(index) {
                sessions[*index].status = TopicSessionStatus::Merged;
            } else if !selected_existing_indices.contains(index)
                && matches!(sessions[*index].status, TopicSessionStatus::Active)
            {
                sessions[*index].status = TopicSessionStatus::Dormant;
            }
        }
    }

    pub(super) fn refresh_existing_bootstrap_topic_sessions(
        sessions: &mut [TopicSession],
        routes: &[BootstrapTopicCandidateRoute],
        persist_inputs: &BootstrapTopicRoutePersistInputs,
    ) {
        for route in routes {
            let Some(existing_index) = route.existing_index else {
                continue;
            };

            let existing = &mut sessions[existing_index];
            existing.status = TopicSessionStatus::Active;
            existing.last_active_unix_ms = persist_inputs.now;
            existing.open_loops = persist_inputs.open_loops.clone();
            existing.durable_memory_refs = persist_inputs.durable_memory_refs.clone();
            merge_bootstrap_topic_session_semantic_hints(
                &mut existing.entities,
                &route.semantic_hints,
            );
            merge_bootstrap_topic_session_transcript_evidence(
                &mut existing.linked_transcript_spans,
                &persist_inputs.linked_transcript_spans,
            );

            if existing
                .linked_surface_session_ids
                .iter()
                .all(|linked| linked.0 != persist_inputs.linked_surface_session_id.0)
            {
                existing
                    .linked_surface_session_ids
                    .push(persist_inputs.linked_surface_session_id.clone());
            }
        }
    }

    pub(super) fn materialize_new_bootstrap_topic_sessions(
        sessions: &mut Vec<TopicSession>,
        routes: &[BootstrapTopicCandidateRoute],
        persist_inputs: &BootstrapTopicRoutePersistInputs,
    ) {
        for route in routes {
            if route.existing_index.is_some() {
                continue;
            }

            let mut entities = BTreeMap::new();
            merge_bootstrap_topic_session_semantic_hints(&mut entities, &route.semantic_hints);

            sessions.push(TopicSession {
                topic_session_id: route.topic_session_id.clone(),
                topic_id: route.topic_id.clone(),
                topic_label: route.topic_label.clone(),
                topic_embedding: None,
                linked_surface_session_ids: vec![persist_inputs.linked_surface_session_id.clone()],
                linked_transcript_spans: persist_inputs.linked_transcript_spans.clone(),
                open_loops: persist_inputs.open_loops.clone(),
                entities,
                graph_edges: Vec::new(),
                durable_memory_refs: persist_inputs.durable_memory_refs.clone(),
                status: TopicSessionStatus::Active,
                created_at_unix_ms: persist_inputs.now,
                last_active_unix_ms: persist_inputs.now,
            });
        }
    }
}

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

mod bootstrap_route_stage {
    use std::collections::BTreeSet;

    use hepta_core::{
        TopicActivationScore, TopicId, TopicSession, TopicSessionStatus, TopicShiftEvent,
    };
    use hepta_intelligence::{
        BootstrapTopicRouteOutcomeDraftInput, build_bootstrap_topic_route_outcome_draft,
    };

    use super::{
        BootstrapTopicCandidateRoute, BootstrapTopicRouteOutcome, BootstrapTopicRoutePlan,
        SessionSnapshot, apply_topic_route_shell_patch, bootstrap_planner,
    };

    #[derive(Debug, Clone, PartialEq)]
    pub(super) struct BootstrapTopicRouteReadStage {
        pub(super) routes: Vec<BootstrapTopicCandidateRoute>,
        pub(super) session_indices: Vec<usize>,
        pub(super) previously_active_topic_ids: Vec<TopicId>,
        pub(super) selected_existing_indices: BTreeSet<usize>,
        pub(super) merged_source_indices: BTreeSet<usize>,
        pub(super) merge_marker: Option<&'static str>,
        pub(super) split_marker: Option<&'static str>,
        pub(super) has_evidence: bool,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub(super) struct BootstrapTopicRouteApplyStage {
        pub(super) routes: Vec<BootstrapTopicCandidateRoute>,
        pub(super) session_indices: Vec<usize>,
        pub(super) selected_existing_indices: BTreeSet<usize>,
        pub(super) merged_source_indices: BTreeSet<usize>,
        pub(super) outcome: BootstrapTopicRouteOutcome,
    }

    pub(super) fn prepare_bootstrap_topic_route_read_stage(
        projected_sessions: &[TopicSession],
        session_id: &str,
        query_text: Option<&str>,
        session: &SessionSnapshot,
        recent_entry_count: usize,
        transcript_matched_count: usize,
        durable_memory_hit_count: usize,
        summary_hit_count: usize,
        topic_score: f32,
        topic_limit: usize,
        learned_route_planning_signals: Vec<hepta_intelligence::LearnedSemanticRouterSignal>,
        semantic_router_id: Option<&str>,
    ) -> BootstrapTopicRouteReadStage {
        let effective_limit = topic_limit.max(1);
        let has_evidence = recent_entry_count > 0
            || transcript_matched_count > 0
            || durable_memory_hit_count > 0
            || summary_hit_count > 0;
        let session_indices = projected_sessions
            .iter()
            .enumerate()
            .filter(|(_, topic_session)| {
                topic_session
                    .linked_surface_session_ids
                    .iter()
                    .any(|linked| linked.0 == session_id)
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let previously_active_topic_ids = session_indices
            .iter()
            .copied()
            .filter(|index| {
                matches!(
                    projected_sessions[*index].status,
                    TopicSessionStatus::Active
                )
            })
            .map(|index| projected_sessions[index].topic_id.clone())
            .collect::<Vec<_>>();
        let BootstrapTopicRoutePlan {
            routes,
            selected_existing_indices,
            merged_source_indices,
            merge_marker,
            split_marker,
        } = bootstrap_planner::plan_bootstrap_topic_routes(
            projected_sessions,
            &session_indices,
            session_id,
            query_text,
            session,
            effective_limit,
            topic_score,
            learned_route_planning_signals,
            semantic_router_id,
        );

        BootstrapTopicRouteReadStage {
            routes,
            session_indices,
            previously_active_topic_ids,
            selected_existing_indices,
            merged_source_indices,
            merge_marker,
            split_marker,
            has_evidence,
        }
    }

    pub(super) fn build_bootstrap_topic_route_apply_stage(
        read_stage: BootstrapTopicRouteReadStage,
        session_id: &str,
        fallback_topic_label: String,
        recent_entry_count: usize,
        transcript_matched_count: usize,
        durable_memory_hit_count: usize,
        summary_hit_count: usize,
    ) -> BootstrapTopicRouteApplyStage {
        let BootstrapTopicRouteReadStage {
            routes,
            session_indices,
            previously_active_topic_ids,
            selected_existing_indices,
            merged_source_indices,
            merge_marker,
            split_marker,
            has_evidence,
        } = read_stage;

        let active_topic_session_ids = routes
            .iter()
            .map(|route| route.topic_session_id.clone())
            .collect::<Vec<_>>();
        let created_topic_session_ids = routes
            .iter()
            .filter(|route| route.existing_index.is_none())
            .map(|route| route.topic_session_id.clone())
            .collect::<Vec<_>>();
        let revived_topic_session_ids = routes
            .iter()
            .filter(|route| route.existing_index.is_some() && !route.was_active)
            .map(|route| route.topic_session_id.clone())
            .collect::<Vec<_>>();
        let activation_scores = routes
            .iter()
            .map(|route| TopicActivationScore {
                topic_id: route.topic_id.clone(),
                topic_label: route.topic_label.clone(),
                score: route.topic_score,
                matched_terms: route.matched_terms.clone(),
                reason: Some(route.reason.clone()),
            })
            .collect::<Vec<_>>();
        let outcome = build_bootstrap_topic_route_outcome(
            session_id,
            &routes,
            &session_indices,
            &previously_active_topic_ids,
            &merged_source_indices,
            merge_marker,
            split_marker,
            activation_scores,
            active_topic_session_ids,
            created_topic_session_ids,
            revived_topic_session_ids,
            fallback_topic_label,
            has_evidence,
            recent_entry_count,
            transcript_matched_count,
            durable_memory_hit_count,
            summary_hit_count,
        );

        BootstrapTopicRouteApplyStage {
            routes,
            session_indices,
            selected_existing_indices,
            merged_source_indices,
            outcome,
        }
    }

    fn build_bootstrap_topic_route_outcome(
        session_id: &str,
        routes: &[BootstrapTopicCandidateRoute],
        session_indices: &[usize],
        previously_active_topic_ids: &[TopicId],
        merged_source_indices: &BTreeSet<usize>,
        merge_marker: Option<&'static str>,
        split_marker: Option<&'static str>,
        activation_scores: Vec<TopicActivationScore>,
        active_topic_session_ids: Vec<String>,
        created_topic_session_ids: Vec<String>,
        revived_topic_session_ids: Vec<String>,
        fallback_topic_label: String,
        has_evidence: bool,
        recent_entry_count: usize,
        transcript_matched_count: usize,
        durable_memory_hit_count: usize,
        summary_hit_count: usize,
    ) -> BootstrapTopicRouteOutcome {
        let route_outcome_draft =
            build_bootstrap_topic_route_outcome_draft(BootstrapTopicRouteOutcomeDraftInput {
                session_id,
                routes,
                session_indices,
                previously_active_topic_ids,
                merged_source_indices,
                merge_marker,
                split_marker,
                activation_scores: &activation_scores,
                active_topic_session_ids: &active_topic_session_ids,
                created_topic_session_ids: &created_topic_session_ids,
                revived_topic_session_ids: &revived_topic_session_ids,
                fallback_topic_label: &fallback_topic_label,
                has_evidence,
                recent_entry_count,
                transcript_matched_count,
                durable_memory_hit_count,
                summary_hit_count,
            });

        let mut outcome = BootstrapTopicRouteOutcome {
            primary_topic_id: None,
            active_topic_session_ids,
            created_topic_session_ids: route_outcome_draft.output_created_topic_session_ids,
            revived_topic_session_ids,
            activation_scores,
            shift_event: TopicShiftEvent {
                kind: route_outcome_draft.shift_kind,
                from_topic_id: route_outcome_draft.shift_from_topic_id,
                to_topic_id: None,
                reason: None,
            },
            explanation: String::new(),
        };
        apply_topic_route_shell_patch(&mut outcome, &route_outcome_draft.route_shell_patch);
        outcome
    }
}

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
    use hepta_core::{EventKind, MessageRole};

    use super::*;

    #[test]
    fn topic_route_shell_patch_application_updates_route_shell_fields() {
        let topic_id = TopicId("topic:patched".into());
        let mut route = BootstrapTopicRouteOutcome {
            primary_topic_id: None,
            active_topic_session_ids: Vec::new(),
            created_topic_session_ids: Vec::new(),
            revived_topic_session_ids: Vec::new(),
            activation_scores: Vec::new(),
            shift_event: TopicShiftEvent {
                kind: TopicShiftKind::Created,
                from_topic_id: None,
                to_topic_id: None,
                reason: Some("bootstrap".into()),
            },
            explanation: "bootstrap route".into(),
        };
        let patch = TopicRouteShellPatch::from_primary_topic(Some(topic_id.clone()))
            .with_shift_reason("patched shift reason")
            .with_explanation_replacement("replacement route")
            .with_explanation_suffix("patched by test");

        apply_topic_route_shell_patch(&mut route, &patch);

        assert_eq!(route.primary_topic_id, Some(topic_id.clone()));
        assert_eq!(route.shift_event.to_topic_id, Some(topic_id));
        assert_eq!(
            route.shift_event.reason.as_deref(),
            Some("patched shift reason")
        );
        assert!(route.explanation.starts_with("replacement route"));
        assert!(route.explanation.ends_with("; patched by test"));
    }

    #[tokio::test]
    async fn recent_session_window_and_query_transcript_follow_recorded_turns() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("hello transcript layer")
            .await
            .expect("demo turn should succeed");

        let recent = runtime
            .recent_session_window("session-main", 4)
            .expect("recent window should load");
        assert!(recent.iter().any(|entry| {
            entry.role == Some(MessageRole::User) && entry.content == "hello transcript layer"
        }));
        assert!(
            recent
                .iter()
                .any(|entry| entry.role == Some(MessageRole::Assistant))
        );

        let report = runtime
            .query_transcript(Some("session-main"), "hello transcript layer", 10)
            .expect("transcript query should succeed");
        assert!(report.matched_count >= 1);
        assert!(report.hits.iter().any(|span| {
            span.entries
                .iter()
                .any(|entry| entry.content == "hello transcript layer")
        }));
    }

    #[tokio::test]
    async fn fresh_runtime_transcript_recall_and_activity_surfaces_are_empty_but_valid() {
        let runtime = RuntimeKernel::new();

        let recent = runtime
            .recent_session_window("session-main", 4)
            .expect("recent window should load for fresh runtime");
        assert!(recent.is_empty());

        let recall = runtime
            .context_recall_slice("session-main", Some("fresh recall"), 4, 4, 4, true)
            .expect("context recall slice should succeed for fresh runtime");
        assert_eq!(recall.recent_entry_count, 0);
        assert_eq!(recall.total_recent_entry_count, 0);
        assert_eq!(recall.transcript_matched_count, 0);
        assert_eq!(recall.transcript_returned_count, 0);
        assert_eq!(recall.memory_matched_count, 0);
        assert_eq!(recall.durable_memory_hit_count, 0);
        assert_eq!(recall.summary_hit_count, 0);
        assert!(recall.bundle.recent_entries.is_empty());
        assert!(recall.bundle.transcript_hits.is_empty());
        assert!(recall.bundle.durable_memory_hits.is_empty());
        assert!(recall.bundle.summary_hits.is_empty());

        let activity = runtime
            .activity_slice(None, 3, 3)
            .expect("activity slice should succeed for fresh runtime");
        assert!(activity.history.is_empty());
        assert!(!activity.events.is_empty());
    }

    #[tokio::test]
    async fn transcript_query_overview_rolls_up_returned_hits_by_session() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("shared transcript needle")
            .await
            .expect("alpha turn should succeed");
        runtime
            .run_demo_turn_in_session("beta", "shared transcript needle")
            .await
            .expect("beta turn should succeed");

        let overview = runtime
            .transcript_query_overview(None, "shared transcript needle", 10)
            .expect("transcript query overview should succeed");

        assert_eq!(overview.matched_sessions, 2);
        assert_eq!(overview.sessions.len(), 2);
        assert!(overview.returned_entries >= overview.report.returned_count);
        assert!(overview.sessions.iter().any(|session| {
            session.session_id == "alpha" && session.hit_count >= 1 && session.entry_count >= 1
        }));
        assert!(overview.sessions.iter().any(|session| {
            session.session_id == "beta" && session.hit_count >= 1 && session.entry_count >= 1
        }));
    }

    #[tokio::test]
    async fn recall_context_blends_recent_transcript_and_memory_hits() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("demo turn should succeed");

        let bundle = runtime
            .recall_context("session-main", Some("hello adaptive memory"), 6, 6, 6, true)
            .expect("context recall should succeed");

        assert!(!bundle.recent_entries.is_empty());
        assert!(!bundle.transcript_hits.is_empty());
        assert!(
            bundle
                .durable_memory_hits
                .iter()
                .any(|record| record.content.contains("hello adaptive memory"))
        );
        assert!(!bundle.is_empty());
    }

    #[tokio::test]
    async fn context_recall_slice_preserves_transcript_match_counts() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let recall = runtime
            .context_recall_slice("alpha", Some("hello adaptive memory"), 4, 4, 4, true)
            .expect("context recall slice should succeed");

        assert_eq!(recall.recent_entry_count, 2);
        assert_eq!(recall.bundle.recent_entries.len(), 2);
        assert_eq!(recall.transcript_matched_count, 2);
        assert_eq!(recall.transcript_returned_count, 2);
        assert_eq!(recall.bundle.transcript_hits.len(), 2);
        assert_eq!(recall.durable_memory_hit_count, 1);
        assert_eq!(recall.bundle.durable_memory_hits.len(), 1);
        assert_eq!(recall.summary_hit_count, 0);
        assert!(!recall.bundle.truncated);
    }

    #[tokio::test]
    async fn activate_neurons_bootstraps_direct_activation_from_recall_evidence() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .rename_active_session("Alpha workspace")
            .expect("rename should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let activations = runtime
            .activate_neurons("alpha", Some("hello adaptive memory"), 4, 4, 4, 2)
            .expect("activate neurons should succeed");

        assert_eq!(activations.len(), 1);
        let activation = &activations[0];
        assert_eq!(activation.neuron_id.0, "neuron-alpha");
        assert_eq!(activation.topic_id.0, "topic-alpha");
        assert!(activation.direct_score > 0.0);
        assert_eq!(activation.propagated_score, 0.0);
        assert_eq!(activation.inhibition_score, 0.0);
        assert_eq!(activation.final_score, activation.direct_score);
        assert_eq!(
            activation.source_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert!(activation.source_neuron_ids.is_empty());
        assert!(
            activation
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains("via routed topic session")
        );
        assert!(
            activation
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains("compressed neuron")
        );
        assert!(
            activation
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains("no additional propagated activation fired yet")
        );
    }

    #[tokio::test]
    async fn neuron_activation_overview_respects_zero_limit() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .neuron_activation_overview("session-main", Some("hello adaptive memory"), 4, 4, 4, 0)
            .expect("neuron activation overview should succeed");

        assert_eq!(overview.recent_entry_count, 2);
        assert_eq!(overview.transcript_matched_count, 2);
        assert_eq!(overview.durable_memory_hit_count, 1);
        assert_eq!(overview.active_topic_session_count, 0);
        assert_eq!(overview.routed_topic_count, 0);
        assert!(overview.activations.is_empty());
    }

    #[tokio::test]
    async fn neuron_activation_overview_uses_topic_routing_state() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .neuron_activation_overview("alpha", Some("hello adaptive memory"), 4, 4, 4, 2)
            .expect("neuron activation overview should succeed");

        assert_eq!(overview.active_topic_session_count, 1);
        assert_eq!(overview.routed_topic_count, 1);
        assert_eq!(overview.activations.len(), 1);
        assert_eq!(
            overview.activations[0].source_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert!(!overview.activations[0].source_transcript_spans.is_empty());
        assert!(
            overview.activations[0]
                .source_transcript_spans
                .iter()
                .any(|span| {
                    span.session_id.0 == "alpha"
                        && span
                            .reason
                            .as_deref()
                            .is_some_and(|reason| reason.contains("query_match"))
                })
        );
    }

    #[tokio::test]
    async fn intuition_overview_returns_provenance_aware_bundle() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .intuition_overview("alpha", "hello adaptive memory", 4, 4, 4, 2, 2, 2)
            .expect("intuition overview should succeed");

        assert_eq!(overview.session_id, "alpha");
        assert_eq!(overview.user_intent, "hello adaptive memory");
        assert_eq!(overview.active_topic_session_count, 1);
        assert_eq!(overview.routed_topic_count, 1);
        assert_eq!(overview.returned_neuron_activation_count, 1);
        assert_eq!(overview.bundle.request.surface_session_id.0, "alpha");
        assert_eq!(overview.bundle.request.user_intent, "hello adaptive memory");
        assert_eq!(overview.bundle.topic_activation_scores.len(), 1);
        assert_eq!(overview.bundle.neuron_activations.len(), 1);
        assert_eq!(
            overview.bundle.foreground_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert!(!overview.bundle.source_transcript_spans.is_empty());
        assert!(overview.bundle.source_transcript_spans.iter().any(|span| {
            span.session_id.0 == "alpha"
                && span
                    .reason
                    .as_deref()
                    .is_some_and(|reason| reason.contains("query_match"))
        }));
        assert_eq!(overview.bundle.workflow_priors.len(), 1);
        assert_eq!(
            overview.bundle.workflow_priors[0].workflow_id,
            "workflow:memory-review"
        );
        assert!(overview.bundle.workflow_priors[0].exists_in_registry);
        assert_eq!(overview.bundle.workflow_priors[0].missing_capability, None);
        assert!(!overview.bundle.workflow_priors[0].requires_confirmation);
        assert_eq!(
            overview.bundle.workflow_priors[0].action_mode,
            IntuitionActionMode::Prepare
        );
        assert!(
            overview.bundle.workflow_priors[0]
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains("workflow registry ranked")
        );
        assert_eq!(overview.bundle.skill_decisions.len(), 1);
        assert_eq!(
            overview.bundle.skill_decisions[0].skill_id,
            "skill-bootstrap:topic-alpha:followup"
        );
        assert!(
            overview.bundle.skill_decisions[0]
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains("compressed neuron prior")
        );
        assert_eq!(
            overview.bundle.skill_decisions[0].source_topic_ids,
            vec![TopicId("topic-alpha".into())]
        );
        assert!(
            overview
                .bundle
                .explanation
                .as_deref()
                .unwrap_or_default()
                .contains("bootstrap intuition synthesized")
        );
        assert!(!overview.bundle.truncated);
    }

    #[tokio::test]
    async fn intuition_overview_reuses_single_routing_state_for_neuron_activation() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let overview = runtime
            .intuition_overview(
                "alpha",
                "hello adaptive memory and rust worker pipeline",
                8,
                8,
                8,
                2,
                1,
                2,
            )
            .expect("intuition overview should succeed");

        assert_eq!(overview.routed_topic_count, 2);
        assert_eq!(overview.bundle.foreground_topic_session_ids.len(), 2);
        assert_eq!(overview.bundle.neuron_activations.len(), 1);

        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic session overview should succeed");
        let active_topic_session_count = topic_sessions
            .iter()
            .filter(|topic_session| matches!(topic_session.status, TopicSessionStatus::Active))
            .count();
        assert_eq!(active_topic_session_count, 2);
    }

    #[tokio::test]
    async fn intuition_overview_uses_durable_neuron_store_and_feedback_calibration() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("topic route should succeed");

        let (mut neuron, _) = runtime
            .compress_topic_to_neuron("alpha", "topic-alpha")
            .expect("topic compression should persist neuron");
        neuron.skill_priors[0].skill_id = "skill-custom:memory-review".into();
        neuron.workflow_priors[0].workflow_id = "workflow-custom:memory-review".into();
        runtime
            .upsert_neurons_for_session("alpha", vec![neuron.clone()])
            .expect("custom neuron prior should upsert");

        let stored = runtime
            .stored_neurons_for_session("alpha")
            .expect("stored neurons should be readable");
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].neuron_id, neuron.neuron_id);

        let before = runtime
            .intuition_overview("alpha", "hello adaptive memory", 4, 4, 4, 1, 1, 1)
            .expect("intuition overview should succeed");
        assert_eq!(
            before.bundle.workflow_priors[0].workflow_id,
            "workflow-custom:memory-review"
        );
        assert!(
            before.bundle.workflow_priors[0]
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains(MEMORY_NEURON_COMPRESSION_V2_POLICY)
        );
        assert_eq!(
            before.bundle.skill_decisions[0].skill_id,
            "skill-custom:memory-review"
        );
        assert!(
            before.bundle.skill_decisions[0]
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains(MEMORY_NEURON_COMPRESSION_V2_POLICY)
        );
        assert_eq!(
            before.bundle.skill_decisions[0].workflow_id.as_deref(),
            Some("workflow-custom:memory-review")
        );
        let before_score = before.bundle.skill_decisions[0].score;

        runtime
            .record_intuition_feedback(
                "alpha",
                "hello adaptive memory",
                IntuitionFeedbackOutcome::Accepted,
                Some("skill-custom:memory-review"),
                Some("workflow-custom:memory-review"),
                before.bundle.skill_decisions[0].source_topic_ids.clone(),
                before.bundle.skill_decisions[0].source_neuron_ids.clone(),
                Some("user accepted custom intuition lane"),
            )
            .expect("feedback should record");

        let after = runtime
            .intuition_overview("alpha", "hello adaptive memory", 4, 4, 4, 1, 1, 1)
            .expect("intuition overview should succeed after feedback");
        assert_eq!(
            after.router_id,
            hepta_intelligence::SEMANTIC_ROUTER_LEARNED_ID
        );
        assert!(after.learned_router_signal_count > 0);
        assert!(after.bundle.skill_decisions[0].score > before_score);
        assert!(
            after.bundle.skill_decisions[0]
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains("feedback +")
        );
    }

    #[tokio::test]
    async fn intuition_calibration_overview_groups_feedback_by_skill_and_workflow() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("topic route should succeed");

        let before = runtime
            .intuition_overview("alpha", "hello adaptive memory", 4, 4, 4, 1, 1, 1)
            .expect("intuition overview should succeed");
        let skill = before.bundle.skill_decisions[0].clone();
        let workflow = before.bundle.workflow_priors[0].clone();

        runtime
            .record_intuition_feedback(
                "alpha",
                "hello adaptive memory",
                IntuitionFeedbackOutcome::ExecutedSuccess,
                Some(skill.skill_id.as_str()),
                Some(workflow.workflow_id.as_str()),
                skill.source_topic_ids.clone(),
                skill.source_neuron_ids.clone(),
                Some("execution succeeded"),
            )
            .expect("positive feedback should record");
        runtime
            .record_intuition_feedback(
                "alpha",
                "hello adaptive memory",
                IntuitionFeedbackOutcome::ToolFailed,
                Some(skill.skill_id.as_str()),
                Some(workflow.workflow_id.as_str()),
                skill.source_topic_ids.clone(),
                skill.source_neuron_ids.clone(),
                Some("tool failed once"),
            )
            .expect("negative feedback should record");

        let overview = runtime
            .intuition_calibration_overview("alpha")
            .expect("calibration overview should succeed");

        assert_eq!(overview.session_id, "alpha");
        assert_eq!(overview.feedback_record_count, 2);
        assert!(overview.closed_loop_ready);
        assert!(overview.learner_applied_update_count >= 2);
        assert!(overview.learned_topic_hint_count > 0);
        assert!(overview.learned_neuron_update_count > 0);
        assert!(overview.learning_findings.is_empty());
        assert_eq!(overview.positive_feedback_count, 1);
        assert_eq!(overview.negative_feedback_count, 1);
        assert_eq!(overview.neutral_feedback_count, 0);
        assert_eq!(
            overview.outcome_counts.get("executed_success").copied(),
            Some(1)
        );
        assert_eq!(overview.outcome_counts.get("tool_failed").copied(), Some(1));
        assert_eq!(overview.skill_targets.len(), 1);
        assert_eq!(overview.workflow_targets.len(), 1);
        assert_eq!(overview.skill_targets[0].target_id, skill.skill_id);
        assert_eq!(overview.workflow_targets[0].target_id, workflow.workflow_id);
        assert_eq!(overview.skill_targets[0].feedback_count, 2);
        assert_eq!(
            overview.skill_targets[0].source_topic_ids,
            vec!["topic-alpha"]
        );
        assert_eq!(
            overview.skill_targets[0].source_neuron_ids,
            vec!["neuron-alpha"]
        );
        assert!(overview.skill_targets[0].net_weight_delta > 0.0);
        assert!(overview.skill_targets[0].confidence_shift_count > 0);
        assert_eq!(overview.recent_feedback.len(), 2);
    }

    #[tokio::test]
    async fn neuron_lookup_revalidates_stored_neuron_when_topic_evidence_changes() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("initial route should succeed");

        let (initial, _) = runtime
            .compress_topic_to_neuron("alpha", "topic-alpha")
            .expect("initial compression should persist neuron");
        assert_eq!(initial.neuron_revision, 1);
        let initial_digest = initial.source_evidence_digest.clone();

        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 6, 6, 6, 1)
            .expect("updated route should succeed");

        let overview = runtime
            .neuron_activation_overview("alpha", Some("hello adaptive memory"), 6, 6, 6, 1)
            .expect("activation should refresh stored neuron");
        assert_eq!(overview.activations.len(), 1);

        let stored = runtime
            .stored_neurons_for_session("alpha")
            .expect("stored neurons should be readable");
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].neuron_id, initial.neuron_id);
        assert_eq!(stored[0].neuron_revision, 2);
        assert_eq!(
            stored[0].last_refresh_reason.as_deref(),
            Some("bootstrap_revalidated_topic_session_evidence")
        );
        assert_ne!(stored[0].source_evidence_digest, initial_digest);
        assert!(
            stored[0].important_transcript_spans.len() > initial.important_transcript_spans.len()
        );
    }

    #[tokio::test]
    async fn intuition_overview_binds_file_intent_to_runtime_tool_registry() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("please read file architecture notes")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .intuition_overview("alpha", "read file architecture notes", 4, 4, 4, 1, 1, 1)
            .expect("intuition overview should succeed");

        let decision = overview
            .bundle
            .skill_decisions
            .first()
            .expect("registered skill decision should be returned");
        assert_eq!(decision.skill_id, "read_file");
        assert!(decision.exists_in_registry);
        assert_eq!(decision.missing_capability, None);
        assert_eq!(decision.risk_tier, Some(RiskTier::Medium));
        assert!(decision.requires_confirmation);
        assert_eq!(decision.action_mode, IntuitionActionMode::SuggestOnly);
        assert!(
            decision
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains("bound to runtime tool registry entry 'read_file'")
        );
    }

    #[tokio::test]
    async fn intuition_overview_applies_custom_policy_to_registered_skill_ranking() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .add_policy_rule(
                Some("alpha"),
                None,
                Some("read_file"),
                None,
                ApprovalRequirement::None,
                Some("alpha session may preflight read_file suggestions"),
            )
            .expect("policy rule should be accepted");
        runtime
            .run_demo_turn("please read file architecture notes")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .intuition_overview("alpha", "read file architecture notes", 4, 4, 4, 1, 1, 1)
            .expect("intuition overview should succeed");

        let decision = overview
            .bundle
            .skill_decisions
            .first()
            .expect("registered skill decision should be returned");
        assert_eq!(decision.skill_id, "read_file");
        assert!(decision.exists_in_registry);
        assert!(!decision.requires_confirmation);
        assert_eq!(decision.action_mode, IntuitionActionMode::Prepare);
        let reason = decision.reason.as_deref().unwrap_or_default();
        assert!(reason.contains("policy-aware intuition ranked"));
        assert!(reason.contains("approval=none"));
        assert!(reason.contains("alpha session may preflight read_file suggestions"));
    }

    #[tokio::test]
    async fn intuition_overview_keeps_denied_write_skill_suggest_only() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("create file release notes")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .intuition_overview("alpha", "create file release notes", 4, 4, 4, 1, 1, 1)
            .expect("intuition overview should succeed");

        let decision = overview
            .bundle
            .skill_decisions
            .first()
            .expect("registered skill decision should be returned");
        assert_eq!(decision.skill_id, "write_file");
        assert!(decision.exists_in_registry);
        assert_eq!(decision.risk_tier, Some(RiskTier::High));
        assert!(decision.requires_confirmation);
        assert_eq!(decision.action_mode, IntuitionActionMode::SuggestOnly);
        let reason = decision.reason.as_deref().unwrap_or_default();
        assert!(reason.contains("approval=deny"));
        assert!(reason.contains("denied by default"));
    }

    #[tokio::test]
    async fn intuition_overview_binds_workflow_priors_to_runtime_registry() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("rust worker pipeline needs semantic routing")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .intuition_overview(
                "alpha",
                "rust worker pipeline needs semantic routing",
                4,
                4,
                4,
                1,
                1,
                1,
            )
            .expect("intuition overview should succeed");

        let prior = overview
            .bundle
            .workflow_priors
            .first()
            .expect("workflow prior should be returned");
        assert_eq!(prior.workflow_id, "workflow:engineering-change");
        assert!(prior.exists_in_registry);
        assert_eq!(prior.missing_capability, None);
        assert!(!prior.requires_confirmation);
        assert_eq!(prior.action_mode, IntuitionActionMode::Prepare);
        let reason = prior.reason.as_deref().unwrap_or_default();
        assert!(reason.contains("workflow registry ranked"));
        assert!(reason.contains("bound to workflow registry entry"));
        assert_eq!(
            overview.bundle.skill_decisions[0].workflow_id.as_deref(),
            Some("workflow:engineering-change")
        );
    }

    #[tokio::test]
    async fn intuition_overview_marks_mutating_workflow_prior_as_gated() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("create file release notes")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .intuition_overview("alpha", "create file release notes", 4, 4, 4, 1, 1, 1)
            .expect("intuition overview should succeed");

        let prior = overview
            .bundle
            .workflow_priors
            .first()
            .expect("workflow prior should be returned");
        assert_eq!(prior.workflow_id, "workflow:file-change");
        assert!(prior.exists_in_registry);
        assert!(prior.requires_confirmation);
        assert_eq!(prior.action_mode, IntuitionActionMode::SuggestOnly);
        assert_eq!(overview.bundle.skill_decisions[0].skill_id, "write_file");
        assert_eq!(
            overview.bundle.skill_decisions[0].workflow_id.as_deref(),
            Some("workflow:file-change")
        );
    }

    #[tokio::test]
    async fn provenance_overview_summarizes_topic_recall_and_intuition_coverage() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("topic route should succeed");

        let overview = runtime
            .provenance_overview("alpha")
            .expect("provenance overview should succeed");

        assert_eq!(overview.session_id, "alpha");
        assert_eq!(
            overview.last_user_intent_summary.as_deref(),
            Some("hello adaptive memory")
        );
        assert_eq!(overview.total_topic_sessions, 1);
        assert_eq!(overview.active_topic_sessions, 1);
        assert_eq!(overview.active_topic_sessions_with_transcript_provenance, 1);
        assert_eq!(
            overview.active_topic_sessions_missing_transcript_provenance,
            0
        );
        assert!(overview.recall_transcript_evidence_spans > 0);
        assert_eq!(overview.recall_omitted_items, 0);
        assert!(overview.intuition_transcript_evidence_spans > 0);
        assert_eq!(overview.intuition_foreground_topic_sessions, 1);
    }

    #[tokio::test]
    async fn intelligence_eval_overview_replays_recent_user_turns_through_core_loop() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");

        let overview = runtime
            .intelligence_eval_overview("alpha", 2, 6, 6, 6, 2, 2, 2)
            .expect("eval overview should succeed");

        assert_eq!(overview.session_id, "alpha");
        assert_eq!(overview.evaluated_case_count, 2);
        assert_eq!(overview.failed_case_count, 0);
        assert_eq!(overview.passed_case_count, 2);
        assert!(overview.total_recall_ranked_items >= 2);
        assert!(overview.total_transcript_evidence_spans >= 2);
        assert_eq!(
            overview.total_active_neurons,
            overview
                .cases
                .iter()
                .map(|case| case.active_neuron_count)
                .sum::<usize>()
        );
        assert!(overview.total_routed_topics >= 2);
        assert!(overview.total_neuron_activations >= 2);
        assert!(overview.total_suggested_skills >= 2);
        assert!(overview.total_workflow_priors >= 2);
        assert!(overview.registered_workflow_prior_count >= 2);
        assert!(overview.prepared_workflow_prior_count >= 2);
        assert_eq!(overview.semantic_score, 100);
        assert_eq!(
            overview.total_semantic_expectations,
            overview.total_semantic_expectations_passed
        );
        assert!(overview.cases.iter().all(|case| case.passed));
        assert!(overview.cases.iter().all(|case| case.warnings.is_empty()));
        assert!(overview.cases.iter().all(|case| case.semantic_score == 100));
        assert!(
            overview
                .cases
                .iter()
                .all(|case| case.semantic_failures.is_empty())
        );
        assert!(
            overview
                .cases
                .iter()
                .all(|case| case.workflow_prior_count > 0)
        );
        assert!(
            overview
                .cases
                .iter()
                .all(|case| case.registered_workflow_prior_count > 0)
        );
        assert_eq!(overview.cases[0].query_text, "hello adaptive memory");
        assert_eq!(overview.cases[1].query_text, "rust worker pipeline");

        let forced_learned = runtime
            .intelligence_eval_overview_with_router(
                "alpha",
                1,
                6,
                6,
                6,
                2,
                2,
                2,
                Some(hepta_intelligence::SEMANTIC_ROUTER_LEARNED_ID),
            )
            .expect("forced router eval overview should succeed");
        assert_eq!(
            forced_learned.semantic_router_id,
            hepta_intelligence::SEMANTIC_ROUTER_LEARNED_ID
        );
        assert!(
            forced_learned
                .cases
                .iter()
                .all(|case| { case.router_id == hepta_intelligence::SEMANTIC_ROUTER_LEARNED_ID })
        );
    }

    #[tokio::test]
    async fn provenance_overview_materializes_fresh_active_session() {
        let runtime = RuntimeKernel::new();

        let overview = runtime
            .provenance_overview("session-main")
            .expect("provenance overview should succeed for fresh runtime");

        assert_eq!(overview.session_id, "session-main");
        assert_eq!(overview.last_user_intent_summary, None);
        assert_eq!(overview.total_topic_sessions, 0);
        assert_eq!(overview.active_topic_sessions, 0);
        assert_eq!(overview.active_topic_sessions_with_transcript_provenance, 0);
        assert_eq!(
            overview.active_topic_sessions_missing_transcript_provenance,
            0
        );
        assert_eq!(overview.recall_transcript_evidence_spans, 0);
        assert_eq!(overview.recall_omitted_items, 0);
        assert_eq!(overview.intuition_transcript_evidence_spans, 0);
        assert_eq!(overview.intuition_foreground_topic_sessions, 0);
    }

    #[tokio::test]
    async fn route_topics_bootstraps_primary_topic_from_session_evidence() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .rename_active_session("Alpha workspace")
            .expect("rename should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        let decision = runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 2)
            .expect("route topics should succeed");

        assert_eq!(decision.surface_session_id.0, "alpha");
        assert_eq!(
            decision
                .primary_topic_id
                .expect("primary topic should exist")
                .0,
            "topic-alpha"
        );
        assert_eq!(
            decision.active_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert!(!decision.source_transcript_spans.is_empty());
        assert!(decision.source_transcript_spans.iter().any(|span| {
            span.session_id.0 == "alpha"
                && span
                    .reason
                    .as_deref()
                    .is_some_and(|reason| reason.contains("query_match"))
        }));
        assert!(decision.created_topic_session_ids.is_empty());
        assert!(decision.revived_topic_session_ids.is_empty());
        assert_eq!(decision.activation_scores.len(), 1);
        assert_eq!(
            decision.activation_scores[0].topic_label.0,
            "hello adaptive memory"
        );
        assert!(
            decision.activation_scores[0]
                .matched_terms
                .iter()
                .any(|term| term == "hello")
        );
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::Stayed
        ));
        assert!(
            decision
                .explanation
                .as_deref()
                .unwrap_or_default()
                .contains("bootstrap topic routing")
        );
    }

    #[tokio::test]
    async fn topic_routing_overview_respects_zero_limit() {
        let runtime = RuntimeKernel::new();
        runtime
            .run_demo_turn("fresh topic route")
            .await
            .expect("turn should succeed");

        let overview = runtime
            .topic_routing_overview("session-main", Some("fresh topic route"), 2, 2, 2, 0)
            .expect("topic routing overview should succeed");

        assert_eq!(overview.session_id, "session-main");
        assert!(overview.decision.primary_topic_id.is_none());
        assert!(!overview.decision.source_transcript_spans.is_empty());
        assert!(overview.decision.active_topic_session_ids.is_empty());
        assert!(overview.decision.activation_scores.is_empty());
        assert!(
            overview
                .decision
                .explanation
                .as_deref()
                .unwrap_or_default()
                .contains("bootstrap topic routing")
        );
    }

    #[tokio::test]
    async fn topic_routing_overview_materializes_fresh_active_session() {
        let runtime = RuntimeKernel::new();

        let overview = runtime
            .topic_routing_overview("session-main", Some("fresh topic route"), 2, 2, 2, 1)
            .expect("topic routing overview should succeed for fresh runtime");

        assert_eq!(overview.session_id, "session-main");
        assert_eq!(overview.query_text.as_deref(), Some("fresh topic route"));
        assert_eq!(overview.recent_entry_count, 0);
        assert_eq!(overview.transcript_matched_count, 0);
        assert_eq!(overview.durable_memory_hit_count, 0);
        assert_eq!(overview.summary_hit_count, 0);
        assert_eq!(overview.decision.active_topic_session_ids.len(), 1);
        assert_eq!(overview.decision.activation_scores.len(), 1);
        assert_eq!(
            overview.decision.primary_topic_id,
            Some(TopicId("topic-session-main".into()))
        );
        assert!(
            overview
                .decision
                .explanation
                .as_deref()
                .unwrap_or_default()
                .contains("bootstrap topic routing")
        );
    }

    #[tokio::test]
    async fn route_topics_persists_bootstrap_topic_session_state() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("route topics should succeed");
        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");

        assert_eq!(topic_sessions.len(), 1);
        assert_eq!(
            topic_sessions[0].topic_session_id,
            "topic-session-bootstrap:alpha"
        );
        assert_eq!(topic_sessions[0].topic_id.0, "topic-alpha");
        assert_eq!(topic_sessions[0].topic_label.0, "hello adaptive memory");
        assert_eq!(topic_sessions[0].linked_surface_session_ids[0].0, "alpha");
        assert_eq!(topic_sessions[0].durable_memory_refs.len(), 1);
        assert_eq!(topic_sessions[0].open_loops.len(), 1);
        assert!(!topic_sessions[0].linked_transcript_spans.is_empty());
        assert!(
            topic_sessions[0]
                .linked_transcript_spans
                .iter()
                .any(|span| {
                    span.session_id.0 == "alpha"
                        && span
                            .reason
                            .as_deref()
                            .is_some_and(|reason| reason.contains("recent_window"))
                })
        );
        assert!(
            topic_sessions[0]
                .linked_transcript_spans
                .iter()
                .any(|span| {
                    span.session_id.0 == "alpha"
                        && span
                            .reason
                            .as_deref()
                            .is_some_and(|reason| reason.contains("query_match"))
                })
        );
    }

    #[tokio::test]
    async fn route_topics_updates_existing_bootstrap_topic_session_instead_of_duplicating() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("turn should succeed");

        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        let first_topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load after first route");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("second route should succeed");

        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");

        assert_eq!(topic_sessions.len(), 1);
        assert_eq!(
            topic_sessions[0].topic_session_id,
            "topic-session-bootstrap:alpha"
        );
        assert_eq!(
            topic_sessions[0].linked_transcript_spans,
            first_topic_sessions[0].linked_transcript_spans
        );
    }

    #[tokio::test]
    async fn route_topics_creates_new_topic_session_when_query_shifts() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");

        let decision = runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("shift route should succeed");
        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");

        assert_eq!(
            decision.active_topic_session_ids,
            vec!["topic-session-bootstrap:alpha:rust-worker-pipeline"]
        );
        assert_eq!(
            decision.created_topic_session_ids,
            vec!["topic-session-bootstrap:alpha:rust-worker-pipeline"]
        );
        assert!(decision.revived_topic_session_ids.is_empty());
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::Shifted
        ));
        assert_eq!(topic_sessions.len(), 2);
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha:rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Active)
                && topic_session.topic_id.0 == "topic-alpha-rust-worker-pipeline"
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha"
                && matches!(topic_session.status, TopicSessionStatus::Dormant)
        }));
    }

    #[tokio::test]
    async fn route_topics_revives_matching_dormant_topic_session() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("shift route should succeed");

        let decision = runtime
            .route_topics("alpha", Some("hello memory"), 6, 6, 6, 1)
            .expect("revive route should succeed");
        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");

        assert_eq!(
            decision.active_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert!(decision.created_topic_session_ids.is_empty());
        assert_eq!(
            decision.revived_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::Revived
        ));
        assert_eq!(topic_sessions.len(), 2);
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha"
                && matches!(topic_session.status, TopicSessionStatus::Active)
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha:rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Dormant)
        }));
    }

    #[tokio::test]
    async fn route_topics_coactivates_multiple_existing_topic_sessions_for_mixed_query() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let decision = runtime
            .route_topics(
                "alpha",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("mixed route should succeed");
        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");

        assert_eq!(
            decision.active_topic_session_ids,
            vec![
                "topic-session-bootstrap:alpha",
                "topic-session-bootstrap:alpha:rust-worker-pipeline"
            ]
        );
        assert!(decision.created_topic_session_ids.is_empty());
        assert_eq!(
            decision.revived_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert_eq!(decision.activation_scores.len(), 2);
        assert!(decision.is_multi_topic());
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::CoActivated
        ));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha"
                && matches!(topic_session.status, TopicSessionStatus::Active)
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha:rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Active)
        }));
    }

    #[tokio::test]
    async fn route_topics_detects_implicit_mixed_turn_without_explicit_delimiters() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let decision = runtime
            .route_topics(
                "alpha",
                Some("continue hello adaptive memory rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("implicit mixed route should succeed");

        assert_eq!(decision.active_topic_session_ids.len(), 2);
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| id == "topic-session-bootstrap:alpha")
        );
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| id == "topic-session-bootstrap:alpha:rust-worker-pipeline")
        );
        assert!(decision.created_topic_session_ids.is_empty());
        assert_eq!(
            decision.revived_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert_eq!(decision.activation_scores.len(), 2);
        assert!(decision.is_multi_topic());
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::CoActivated
        ));
        assert!(
            decision
                .activation_scores
                .iter()
                .any(|score| score.topic_id.0 == "topic-alpha")
        );
        assert!(
            decision
                .activation_scores
                .iter()
                .any(|score| score.topic_id.0 == "topic-alpha-rust-worker-pipeline")
        );
    }

    #[tokio::test]
    async fn route_topics_detects_semantic_mixed_turn_without_exact_label_overlap() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let decision = runtime
            .route_topics(
                "alpha",
                Some("continue adaptive recall while checking executor flow"),
                8,
                8,
                8,
                2,
            )
            .expect("semantic mixed route should succeed");

        assert_eq!(decision.active_topic_session_ids.len(), 2);
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| id == "topic-session-bootstrap:alpha")
        );
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| id == "topic-session-bootstrap:alpha:rust-worker-pipeline")
        );
        assert!(decision.created_topic_session_ids.is_empty());
        assert_eq!(
            decision.revived_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::CoActivated
        ));
        assert!(decision.activation_scores.iter().any(|score| {
            score.topic_id.0 == "topic-alpha"
                && score
                    .reason
                    .as_deref()
                    .is_some_and(|reason| reason.contains("semantic"))
        }));
        assert!(decision.activation_scores.iter().any(|score| {
            score.topic_id.0 == "topic-alpha-rust-worker-pipeline"
                && score
                    .reason
                    .as_deref()
                    .is_some_and(|reason| reason.contains("semantic"))
        }));
    }

    #[tokio::test]
    async fn route_topics_learns_open_ended_semantic_aliases_from_matched_evidence() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("first turn should succeed");

        let first_decision = runtime
            .route_topics("alpha", Some("rust worker pipeline"), 4, 4, 4, 1)
            .expect("first route should succeed");
        let worker_topic_session_id = first_decision.active_topic_session_ids[0].clone();
        let worker_topic_id = first_decision
            .primary_topic_id
            .expect("primary topic should exist");

        runtime
            .route_topics(
                "alpha",
                Some("rust worker pipeline queue backlog"),
                6,
                6,
                6,
                1,
            )
            .expect("alias-learning route should succeed");

        let learned_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load after alias learning");
        let worker_topic = learned_sessions
            .iter()
            .find(|topic_session| topic_session.topic_session_id == worker_topic_session_id)
            .expect("worker topic session should exist");
        assert!(worker_topic.entities.values().any(|value| value == "queue"));
        assert!(
            worker_topic
                .entities
                .values()
                .any(|value| value == "backlog")
        );

        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 6, 6, 6, 1)
            .expect("shift route should succeed");

        let revived = runtime
            .route_topics("alpha", Some("queue backlog"), 6, 6, 6, 1)
            .expect("learned alias revive route should succeed");

        assert_eq!(revived.primary_topic_id, Some(worker_topic_id));
        assert_eq!(revived.created_topic_session_ids, Vec::<String>::new());
        assert_eq!(
            revived.revived_topic_session_ids,
            vec![worker_topic_session_id]
        );
        assert!(matches!(
            revived.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::Revived
        ));
    }

    #[tokio::test]
    async fn route_topics_merges_multiple_topic_sessions_into_new_composite_topic() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let decision = runtime
            .route_topics(
                "alpha",
                Some("merge hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("merge route should succeed");
        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");

        assert_eq!(
            decision
                .primary_topic_id
                .expect("primary topic should exist")
                .0,
            "topic-alpha-hello-adaptive-memory-rust-worker-pipeline"
        );
        assert_eq!(
            decision.active_topic_session_ids,
            vec!["topic-session-bootstrap:alpha:hello-adaptive-memory-rust-worker-pipeline"]
        );
        assert_eq!(
            decision.created_topic_session_ids,
            vec!["topic-session-bootstrap:alpha:hello-adaptive-memory-rust-worker-pipeline"]
        );
        assert!(decision.revived_topic_session_ids.is_empty());
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::Merged
        ));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id
                == "topic-session-bootstrap:alpha:hello-adaptive-memory-rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Active)
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha"
                && matches!(topic_session.status, TopicSessionStatus::Merged)
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha:rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Merged)
        }));
    }

    #[tokio::test]
    async fn compress_topic_to_neuron_collects_provenance_and_component_links() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");
        runtime
            .route_topics(
                "alpha",
                Some("merge hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("merge route should succeed");

        let (neuron, report) = runtime
            .compress_topic_to_neuron(
                "alpha",
                "topic-alpha-hello-adaptive-memory-rust-worker-pipeline",
            )
            .expect("topic compression should succeed");

        assert_eq!(
            neuron.neuron_id.0,
            "neuron-alpha-hello-adaptive-memory-rust-worker-pipeline"
        );
        assert_eq!(report.created_neuron_id, Some(neuron.neuron_id.clone()));
        assert_eq!(
            neuron.compression_policy_version,
            MEMORY_NEURON_COMPRESSION_V2_POLICY
        );
        assert_eq!(
            report.compression_policy_version,
            MEMORY_NEURON_COMPRESSION_V2_POLICY
        );
        assert_eq!(report.source_evidence_digest, neuron.source_evidence_digest);
        assert_eq!(report.source_topic_session_ids.len(), 1);
        assert!(report.important_span_count >= 1);
        assert!(report.promoted_memory_count >= 1);
        assert!(
            report
                .merged_neuron_ids
                .iter()
                .any(|neuron_id| neuron_id.0 == "neuron-alpha")
        );
        assert!(
            report
                .merged_neuron_ids
                .iter()
                .any(|neuron_id| neuron_id.0 == "neuron-alpha-rust-worker-pipeline")
        );
        assert!(
            neuron
                .links
                .iter()
                .any(|link| link.target_neuron_id.0 == "neuron-alpha")
        );
        assert!(
            neuron
                .links
                .iter()
                .any(|link| { link.target_neuron_id.0 == "neuron-alpha-rust-worker-pipeline" })
        );
        assert_eq!(neuron.skill_priors.len(), 1);
        assert_eq!(neuron.workflow_priors.len(), 1);
        assert_eq!(report.skill_prior_count, 1);
        assert_eq!(report.workflow_prior_count, 1);
        assert!(report.typed_link_count >= 2);
        assert!(report.provenance_complete);
        assert!(report.intuition_ready);
        assert!(neuron.confidence > 0.0);
        assert!(neuron.freshness > 0.0);
    }

    #[tokio::test]
    async fn compress_active_topics_to_neurons_returns_unique_active_topics() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");
        runtime
            .route_topics(
                "alpha",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("mixed route should succeed");

        let neurons = runtime
            .compress_active_topics_to_neurons("alpha", 4)
            .expect("active neuron compression should succeed");

        assert_eq!(neurons.len(), 2);
        assert!(
            neurons
                .iter()
                .any(|neuron| neuron.neuron_id.0 == "neuron-alpha")
        );
        assert!(
            neurons
                .iter()
                .any(|neuron| neuron.neuron_id.0 == "neuron-alpha-rust-worker-pipeline")
        );
    }

    #[tokio::test]
    async fn neuron_lifecycle_overview_surfaces_stored_neuron_health() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("route should succeed");
        runtime
            .compress_active_topics_to_neurons("alpha", 4)
            .expect("compression should succeed");
        let mut neuron = runtime
            .stored_neurons_for_session("alpha")
            .expect("stored neuron should load")
            .pop()
            .expect("one neuron should exist");
        neuron.linked_session_ids.push(SessionId("beta".into()));
        runtime
            .upsert_neurons_for_session("alpha", vec![neuron])
            .expect("cross-session neuron should upsert");

        let overview = runtime
            .neuron_lifecycle_overview("alpha")
            .expect("lifecycle overview should succeed");

        assert_eq!(overview.session_id, "alpha");
        assert_eq!(overview.stored_neurons, 1);
        assert_eq!(overview.active_topic_sessions, 1);
        assert_eq!(overview.neurons_with_transcript_provenance, 1);
        assert_eq!(overview.neurons_with_memory_provenance, 1);
        assert_eq!(overview.neurons_with_evidence_digest, 1);
        assert_eq!(overview.v2_compressed_neurons, 1);
        assert_eq!(overview.neurons_with_skill_priors, 1);
        assert_eq!(overview.neurons_with_workflow_priors, 1);
        assert_eq!(overview.intuition_ready_neurons, 1);
        assert!(overview.neuron_upgrade_ready);
        assert_eq!(
            overview
                .compression_policy_versions
                .get(MEMORY_NEURON_COMPRESSION_V2_POLICY),
            Some(&1)
        );
        assert_eq!(overview.cross_session_stable_neurons, 1);
        assert_eq!(overview.cross_session_unstable_neurons, 0);
        assert!(overview.average_confidence > 0.0);
        assert!(overview.average_freshness > 0.0);
        assert_eq!(overview.stale_neurons, 0);
        assert_eq!(overview.low_confidence_neurons, 0);
        assert_eq!(overview.low_freshness_neurons, 0);
        assert!(overview.active_topics_without_neurons.is_empty());
        assert!(overview.healthy);
        assert!(overview.findings.is_empty());
    }

    #[tokio::test]
    async fn intelligence_phase2_gate_closes_memory_intelligence_next_phase() {
        let runtime = RuntimeKernel::new();

        let overview = runtime
            .intelligence_phase2_gate("phase2")
            .await
            .expect("phase2 gate should succeed");

        assert_eq!(overview.status, "complete");
        assert_eq!(overview.overall_percent, 100);
        assert!(overview.all_phase2_gates_ready);
        assert!(overview.blended_recall_ready);
        assert!(overview.provenance_memory_ready);
        assert!(overview.semantic_router_generalized);
        assert!(overview.neuron_compression_ready);
        assert!(overview.recall_source_count >= 4);
        assert!(overview.recall_ranked_items >= 4);
        assert!(overview.recall_transcript_evidence_spans > 0);
        assert!(overview.durable_memory_hits > 0);
        assert!(overview.active_neurons > 0);
        assert!(overview.provenance_topic_sessions_with_transcript > 0);
        assert!(overview.supported_semantic_router_count >= 3);
        assert!(overview.learned_router_signal_count > 0);
        assert!(overview.compressed_neuron_count > 0);
        assert!(overview.neurons_with_evidence_digest >= overview.compressed_neuron_count);
        assert_eq!(overview.gates.len(), 4);
        assert!(overview.gates.iter().all(|gate| gate.ready));
        assert!(overview.findings.is_empty());
    }

    #[tokio::test]
    async fn knowledge_graph_dry_run_overview_exposes_candidates_without_live_write() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_dry_run_overview();

        assert_eq!(report.status, "ready");
        assert!(report.candidate_count > 0);
        assert_eq!(report.memory_unit_count, report.candidate_count);
        assert_eq!(report.live_write_enabled_count, 0);
        assert_eq!(report.external_side_effect_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.all_candidates_have_provenance);
        assert!(report.checks.all_candidates_have_graph_payload);
        assert!(report.checks.all_plans_are_dry_run);
        assert!(report.checks.no_live_write_enabled);
        assert!(report.checks.no_external_side_effects);
    }

    #[tokio::test]
    async fn knowledge_graph_adapter_dry_run_overview_projects_external_targets() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_adapter_dry_run_overview();

        assert_eq!(report.status, "ready");
        assert!(report.candidate_count > 0);
        assert_eq!(report.adapter_count, 3);
        assert_eq!(
            report.projection_count,
            report.candidate_count * report.adapter_count
        );
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.external_write_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(
            report
                .projections
                .iter()
                .any(|plan| plan.adapter_id == "graphiti")
        );
        assert!(
            report
                .projections
                .iter()
                .any(|plan| plan.adapter_id == "neo4j")
        );
        assert!(
            report
                .projections
                .iter()
                .any(|plan| plan.adapter_id == "cocoindex")
        );
    }

    #[tokio::test]
    async fn knowledge_graph_adapter_staging_gate_overview_keeps_adapters_closed() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_adapter_staging_gate_overview();

        assert_eq!(report.status, "ready");
        assert!(report.candidate_count > 0);
        assert_eq!(report.adapter_count, 3);
        assert_eq!(
            report.staging_plan_count,
            report.candidate_count * report.adapter_count
        );
        assert_eq!(report.staging_ready_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.external_write_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.operator_review_required);
        assert!(report.checks.rollback_plan_required);
        assert!(report.checks.post_write_validation_required);
    }

    #[tokio::test]
    async fn knowledge_graph_adapter_client_overview_denies_disabled_clients() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_adapter_client_overview();

        assert_eq!(report.status, "ready");
        assert!(report.candidate_count > 0);
        assert_eq!(report.adapter_count, 3);
        assert_eq!(
            report.client_audit_count,
            report.candidate_count * report.adapter_count
        );
        assert_eq!(report.denied_client_count, report.client_audit_count);
        assert_eq!(report.network_call_attempted_count, 0);
        assert_eq!(report.external_write_attempted_count, 0);
        assert_eq!(report.live_write_attempted_count, 0);
        assert_eq!(report.persisted_record_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.all_supported_clients_present);
        assert!(report.checks.all_client_calls_denied_by_default);
    }

    #[tokio::test]
    async fn knowledge_graph_adapter_config_env_overview_reads_default_closed_snapshot() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_adapter_config_env_overview();

        assert_eq!(report.status, "ready");
        assert_eq!(report.adapter_count, 3);
        assert_eq!(report.config_read_count, 3);
        assert_eq!(report.feature_enabled_count, 0);
        assert_eq!(report.endpoint_configured_count, 0);
        assert_eq!(report.credentials_configured_count, 0);
        assert_eq!(report.fully_configured_count, 0);
        assert_eq!(report.live_write_requested_count, 0);
        assert_eq!(report.credential_value_captured_count, 0);
        assert_eq!(report.network_call_attempted_count, 0);
        assert_eq!(report.external_write_attempted_count, 0);
        assert_eq!(report.live_write_attempted_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.all_supported_adapters_read);
        assert!(report.checks.all_configs_closed_by_default);
    }

    #[tokio::test]
    async fn knowledge_graph_recall_plan_overview_stays_read_only() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_recall_plan_overview();

        assert_eq!(report.status, "ready");
        assert_eq!(report.query_count, 2);
        assert!(report.candidate_count > 0);
        assert!(report.entity_match_count > 0);
        assert!(report.relation_neighborhood_count > 0);
        assert!(report.timeline_slice_count > 0);
        assert!(report.evidence_path_count > 0);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.all_plans_are_read_only);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
    }

    #[tokio::test]
    async fn knowledge_graph_context_recall_bridge_overview_emits_kg_ranked_items() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_context_recall_bridge_overview();

        assert_eq!(report.status, "ready");
        assert_eq!(report.query_count, 2);
        assert!(report.kg_plan_count > 0);
        assert!(report.kg_evidence_path_count > 0);
        assert!(report.context_item_count > 0);
        assert!(report.transcript_span_count > 0);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert!(report.checks.ready());
        assert!(report.checks.all_items_have_kg_source);
        assert!(report.checks.transcript_provenance_preserved);
        assert!(report.checks.no_context_injection_performed);
    }

    #[tokio::test]
    async fn knowledge_graph_recall_evaluation_overview_keeps_quality_gate_report_only() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_recall_evaluation_overview();

        assert_eq!(report.status, "ready");
        assert_eq!(report.query_count, 2);
        assert!(report.evaluation_case_count > 0);
        assert_eq!(report.passed_case_count, report.evaluation_case_count);
        assert_eq!(report.failed_case_count, 0);
        assert_eq!(report.coverage_basis_points, 10_000);
        assert_eq!(report.precision_proxy_basis_points, 10_000);
        assert_eq!(report.score_stability_basis_points, 10_000);
        assert_eq!(report.duplicate_source_memory_id_count, 0);
        assert_eq!(report.score_order_violation_count, 0);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert!(report.checks.ready());
        assert!(report.checks.source_memory_ids_unique);
        assert!(report.checks.scores_stably_ordered);
        assert!(report.checks.no_context_injection_performed);
    }

    #[tokio::test]
    async fn knowledge_graph_context_injection_readiness_overview_blocks_injection() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_context_injection_readiness_overview();

        assert_eq!(report.status, "blocked");
        assert!(report.quality_gate_ready);
        assert_eq!(report.evaluation_case_count, report.passed_case_count);
        assert_eq!(report.failed_case_count, 0);
        assert_eq!(report.coverage_basis_points, 10_000);
        assert_eq!(report.precision_proxy_basis_points, 10_000);
        assert_eq!(report.score_stability_basis_points, 10_000);
        assert_eq!(report.quality_threshold_basis_points, 9_000);
        assert!(!report.operator_approved);
        assert!(!report.shadow_rank_enabled);
        assert!(!report.rollback_plan_ready);
        assert!(!report.kill_switch_ready);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.activation_blocked_without_operator_approval);
        assert!(report.checks.no_context_injection_performed);
    }

    #[tokio::test]
    async fn knowledge_graph_shadow_rank_overview_observes_without_injection() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_shadow_rank_overview();

        assert_eq!(report.status, "ready");
        assert_eq!(report.injection_readiness_status, "blocked");
        assert!(report.context_item_count > 0);
        assert_eq!(report.ranked_item_count, report.context_item_count);
        assert_eq!(report.observed_only_count, report.ranked_item_count);
        assert_eq!(report.would_enter_prompt_context_count, 0);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.injection_readiness_blocked);
        assert!(report.checks.all_items_observed_only);
        assert!(report.checks.no_items_enter_prompt_context);
        assert!(report.checks.scores_stably_ordered);
        assert!(report.checks.no_context_injection_performed);
    }

    #[tokio::test]
    async fn knowledge_graph_shadow_rank_comparison_overview_compares_without_injection() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_shadow_rank_comparison_overview();

        assert_eq!(report.status, "ready");
        assert_eq!(
            report.kg_shadow_rank_contract,
            hepta_intelligence::MEMORY_KG_SHADOW_RANK_V0_CONTRACT
        );
        assert_eq!(
            report.kg_context_injection_readiness_contract,
            hepta_intelligence::MEMORY_KG_CONTEXT_INJECTION_READINESS_V0_CONTRACT
        );
        assert!(report.kg_ranked_item_count > 0);
        assert_eq!(
            report.transcript_baseline_count,
            report.kg_ranked_item_count
        );
        assert_eq!(
            report.durable_memory_baseline_count,
            report.kg_ranked_item_count
        );
        assert_eq!(
            report.comparison_case_count,
            report.kg_ranked_item_count * 2
        );
        assert!(!report.prompt_preview_rendered);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.shadow_rank_ready);
        assert!(report.checks.no_kg_items_enter_prompt_context);
        assert!(report.checks.no_baseline_items_enter_prompt_context);
        assert!(report.checks.no_context_injection_performed);
    }

    #[tokio::test]
    async fn knowledge_graph_shadow_rank_drift_overview_gates_regression_without_injection() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_shadow_rank_drift_overview();

        assert_eq!(report.status, "ready");
        assert_eq!(report.verdict, "stable");
        assert_eq!(
            report.kg_shadow_rank_comparison_contract,
            hepta_intelligence::MEMORY_KG_SHADOW_RANK_COMPARISON_V0_CONTRACT
        );
        assert_eq!(
            report.kg_shadow_rank_contract,
            hepta_intelligence::MEMORY_KG_SHADOW_RANK_V0_CONTRACT
        );
        assert!(report.top_n_kg_rank_count > 0);
        assert_eq!(report.expected_drift_case_count, report.drift_case_count);
        assert_eq!(report.stable_case_count, report.drift_case_count);
        assert_eq!(report.drifted_case_count, 0);
        assert_eq!(report.transcript_case_count, report.top_n_kg_rank_count);
        assert_eq!(report.durable_memory_case_count, report.top_n_kg_rank_count);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.comparison_ready);
        assert!(report.checks.rank_order_stable);
        assert!(report.checks.score_delta_within_thresholds);
        assert!(report.checks.prompt_flags_stable);
        assert!(report.checks.no_context_injection_performed);
    }

    #[tokio::test]
    async fn knowledge_graph_prompt_preview_approval_packet_overview_blocks_prompt_preview() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_prompt_preview_approval_packet_overview();

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_APPROVAL_PACKET_V0_CONTRACT
        );
        assert_eq!(
            report.kg_shadow_rank_drift_contract,
            hepta_intelligence::MEMORY_KG_SHADOW_RANK_DRIFT_V0_CONTRACT
        );
        assert!(report.drift_case_count > 0);
        assert_eq!(report.approval_item_count, report.drift_case_count);
        assert_eq!(
            report.redacted_context_ref_count,
            report.approval_item_count
        );
        assert_eq!(report.drifted_case_count, 0);
        assert!(!report.operator_approval_recorded);
        assert!(!report.rollback_plan_ready);
        assert!(!report.kill_switch_ready);
        assert!(!report.approval_packet_accepted);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.drift_gate_stable);
        assert!(report.checks.operator_approval_required);
        assert!(report.checks.prompt_preview_disabled_by_default);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled_by_default);
        assert!(report.checks.no_context_injection_performed);
    }

    #[tokio::test]
    async fn knowledge_graph_prompt_preview_operator_evidence_overview_blocks_preview() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_prompt_preview_operator_evidence_overview();

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_OPERATOR_EVIDENCE_V0_CONTRACT
        );
        assert_eq!(
            report.approval_packet_contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_APPROVAL_PACKET_V0_CONTRACT
        );
        assert_eq!(report.approval_packet_status, "blocked");
        assert_eq!(report.required_evidence_count, 7);
        assert_eq!(
            report.missing_evidence_count,
            report.required_evidence_count
        );
        assert!(!report.operator_approval_evidence_present);
        assert!(!report.rollback_plan_evidence_present);
        assert!(!report.kill_switch_evidence_present);
        assert!(report.reviewer_identity_redacted);
        assert!(!report.signed_approval_digest_present);
        assert!(!report.bounded_preview_scope_present);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.approval_packet_not_accepted);
        assert!(report.checks.evidence_requirements_all_blocking);
        assert!(report.checks.operator_approval_evidence_required);
        assert!(report.checks.signed_approval_digest_required);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
    }

    #[tokio::test]
    async fn knowledge_graph_prompt_preview_redaction_diff_overview_suppresses_raw_diff() {
        let runtime = RuntimeKernel::new();

        let report = runtime.knowledge_graph_prompt_preview_redaction_diff_overview();

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT
        );
        assert_eq!(
            report.operator_evidence_contract,
            hepta_intelligence::MEMORY_KG_PROMPT_PREVIEW_OPERATOR_EVIDENCE_V0_CONTRACT
        );
        assert_eq!(report.operator_evidence_status, "blocked");
        assert_eq!(report.required_evidence_count, 7);
        assert_eq!(
            report.missing_evidence_count,
            report.required_evidence_count
        );
        assert_eq!(report.diff_item_count, report.required_evidence_count);
        assert_eq!(report.redacted_ref_count, report.diff_item_count);
        assert_eq!(report.raw_prompt_diff_count, 0);
        assert_eq!(report.prompt_text_included_count, 0);
        assert_eq!(report.payload_text_included_count, 0);
        assert!(report.redacted_diff_reported);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.operator_evidence_contract_linked);
        assert!(report.checks.operator_evidence_checks_ready);
        assert!(report.checks.operator_evidence_missing_requirements);
        assert!(report.checks.redacted_refs_present);
        assert!(report.checks.raw_prompt_diff_suppressed);
        assert!(report.checks.prompt_text_excluded);
        assert!(report.checks.payload_text_excluded);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
    }

    #[tokio::test]
    async fn route_topics_splits_merged_topic_back_into_component_topics() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");
        runtime
            .route_topics(
                "alpha",
                Some("merge hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("merge route should succeed");

        let decision = runtime
            .route_topics(
                "alpha",
                Some("split hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("split route should succeed");
        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");

        assert_eq!(
            decision.active_topic_session_ids,
            vec![
                "topic-session-bootstrap:alpha",
                "topic-session-bootstrap:alpha:rust-worker-pipeline"
            ]
        );
        assert!(decision.created_topic_session_ids.is_empty());
        assert_eq!(
            decision.revived_topic_session_ids,
            vec![
                "topic-session-bootstrap:alpha",
                "topic-session-bootstrap:alpha:rust-worker-pipeline"
            ]
        );
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::Split
        ));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha"
                && matches!(topic_session.status, TopicSessionStatus::Active)
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha:rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Active)
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id
                == "topic-session-bootstrap:alpha:hello-adaptive-memory-rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Dormant)
        }));
    }

    #[tokio::test]
    async fn route_topics_graph_expands_component_query_to_adjacent_composite_topic() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");
        runtime
            .route_topics(
                "alpha",
                Some("merge hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("merge route should succeed");

        let decision = runtime
            .route_topics("alpha", Some("hello adaptive memory"), 8, 8, 8, 2)
            .expect("graph-expanded route should succeed");
        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");

        assert_eq!(decision.active_topic_session_ids.len(), 2);
        assert_eq!(
            decision.revived_topic_session_ids,
            vec!["topic-session-bootstrap:alpha"]
        );
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| id == "topic-session-bootstrap:alpha")
        );
        assert!(decision.active_topic_session_ids.iter().any(|id| {
            id == "topic-session-bootstrap:alpha:hello-adaptive-memory-rust-worker-pipeline"
        }));
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::CoActivated
        ));
        assert!(decision.activation_scores.iter().any(|score| {
            score.topic_id.0 == "topic-alpha-hello-adaptive-memory-rust-worker-pipeline"
                && score.reason.as_deref().is_some_and(|reason| {
                    reason.contains("bootstrap topic graph expanded 'hello adaptive memory'")
                })
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha"
                && matches!(topic_session.status, TopicSessionStatus::Active)
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id
                == "topic-session-bootstrap:alpha:hello-adaptive-memory-rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Active)
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha:rust-worker-pipeline"
                && matches!(topic_session.status, TopicSessionStatus::Merged)
        }));
    }

    #[tokio::test]
    async fn route_topics_graph_expands_single_topic_query_via_stored_coactivation_edge() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");
        runtime
            .route_topics(
                "alpha",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("mixed route should succeed");

        let decision = runtime
            .route_topics("alpha", Some("hello adaptive memory"), 8, 8, 8, 2)
            .expect("graph-expanded route should succeed");
        let topic_sessions = runtime
            .topic_sessions_for_surface("alpha")
            .expect("topic sessions should load");
        let raw_topic_sessions = runtime
            .topic_session_state
            .lock()
            .expect("topic session state lock should succeed")
            .sessions
            .clone();
        let topic_graph_edges = runtime
            .topic_graph_state
            .lock()
            .expect("topic graph state lock should succeed")
            .edges
            .clone();

        assert_eq!(decision.active_topic_session_ids.len(), 2);
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| id == "topic-session-bootstrap:alpha")
        );
        assert!(
            decision
                .active_topic_session_ids
                .iter()
                .any(|id| id == "topic-session-bootstrap:alpha:rust-worker-pipeline")
        );
        assert!(decision.activation_scores.iter().any(|score| {
            score.topic_id.0 == "topic-alpha-rust-worker-pipeline"
                && score
                    .reason
                    .as_deref()
                    .is_some_and(|reason| reason.contains("stored co-activation edge"))
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha"
                && bootstrap_topic_graph_edge_count(topic_session) >= 1
        }));
        assert!(topic_sessions.iter().any(|topic_session| {
            topic_session.topic_session_id == "topic-session-bootstrap:alpha:rust-worker-pipeline"
                && bootstrap_topic_graph_edge_count(topic_session) >= 1
        }));
        assert!(
            raw_topic_sessions
                .iter()
                .all(|topic_session| topic_session.graph_edges.is_empty())
        );
        assert!(topic_graph_edges.iter().any(|record| {
            record.source_topic_session_id == "topic-session-bootstrap:alpha"
                && record.edge.target_topic_session_id
                    == "topic-session-bootstrap:alpha:rust-worker-pipeline"
        }));
    }

    #[tokio::test]
    async fn neuron_activation_overview_returns_multiple_activations_for_coactivated_topics() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let overview = runtime
            .neuron_activation_overview(
                "alpha",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                3,
            )
            .expect("neuron activation overview should succeed");

        assert_eq!(overview.active_topic_session_count, 2);
        assert_eq!(overview.routed_topic_count, 2);
        assert_eq!(overview.activations.len(), 2);
        assert_eq!(
            overview.activations[0].source_topic_session_ids,
            vec![
                "topic-session-bootstrap:alpha",
                "topic-session-bootstrap:alpha:rust-worker-pipeline"
            ]
        );
        assert_eq!(
            overview.activations[1].source_topic_session_ids,
            vec![
                "topic-session-bootstrap:alpha:rust-worker-pipeline",
                "topic-session-bootstrap:alpha"
            ]
        );
        assert!(overview.activations[0].propagated_score > 0.0);
        assert!(overview.activations[1].propagated_score > 0.0);
        assert_eq!(overview.activations[0].inhibition_score, 0.0);
        assert_eq!(overview.activations[1].inhibition_score, 0.0);
        assert_eq!(
            overview.activations[0].source_neuron_ids,
            vec![NeuronId("neuron-alpha-rust-worker-pipeline".into())]
        );
        assert_eq!(
            overview.activations[1].source_neuron_ids,
            vec![NeuronId("neuron-alpha".into())]
        );
        assert!(
            overview
                .activations
                .iter()
                .all(|activation| !activation.source_transcript_spans.is_empty())
        );
    }

    #[tokio::test]
    async fn neuron_activation_overview_prefers_stored_topic_graph_edges_for_propagation() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");
        runtime
            .route_topics(
                "alpha",
                Some("hello adaptive memory and rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("mixed route should succeed");

        let overview = runtime
            .neuron_activation_overview("alpha", Some("hello adaptive memory"), 8, 8, 8, 3)
            .expect("neuron activation overview should succeed");

        assert_eq!(overview.active_topic_session_count, 2);
        assert_eq!(overview.routed_topic_count, 2);
        assert_eq!(overview.activations.len(), 2);
        assert!(overview.activations.iter().all(|activation| {
            activation
                .source_link_reasons
                .iter()
                .any(|reason| reason.contains("stored co-activation edge"))
        }));
    }

    #[tokio::test]
    async fn neuron_activation_overview_applies_inhibitory_suppression_for_contrast_query() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("hello adaptive memory")
            .await
            .expect("first turn should succeed");
        runtime
            .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
            .expect("first route should succeed");
        runtime
            .run_demo_turn("rust worker pipeline")
            .await
            .expect("second turn should succeed");
        runtime
            .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
            .expect("second route should succeed");

        let decision = runtime
            .route_topics(
                "alpha",
                Some("hello adaptive memory but not rust worker pipeline"),
                8,
                8,
                8,
                2,
            )
            .expect("contrast route should succeed");
        assert!(matches!(
            decision.shift_event.expect("shift event should exist").kind,
            TopicShiftKind::CoActivated
        ));

        let overview = runtime
            .neuron_activation_overview(
                "alpha",
                Some("hello adaptive memory but not rust worker pipeline"),
                8,
                8,
                8,
                3,
            )
            .expect("neuron activation overview should succeed");

        assert_eq!(overview.activations.len(), 2);
        assert_eq!(overview.activations[0].topic_id.0, "topic-alpha");
        assert_eq!(overview.activations[0].propagated_score, 0.0);
        assert_eq!(overview.activations[0].inhibition_score, 0.0);
        assert_eq!(
            overview.activations[1].topic_id.0,
            "topic-alpha-rust-worker-pipeline"
        );
        assert_eq!(overview.activations[1].propagated_score, 0.0);
        assert!(overview.activations[1].inhibition_score > 0.0);
        assert!(overview.activations[1].final_score < overview.activations[1].direct_score);
        assert_eq!(
            overview.activations[1].source_neuron_ids,
            vec![NeuronId("neuron-alpha".into())]
        );
        assert_eq!(
            overview.activations[1].source_link_kinds,
            vec![NeuronLinkKind::Inhibition]
        );
        assert!(
            overview.activations[1]
                .reason
                .as_deref()
                .unwrap_or_default()
                .contains("inhibitory suppression")
        );
    }

    #[tokio::test]
    async fn session_activity_overview_counts_active_archived_and_populated_sessions() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("alpha planning")
            .await
            .expect("alpha turn should succeed");
        runtime
            .rename_active_session("Alpha workspace")
            .expect("rename should succeed");
        runtime
            .run_demo_turn_in_session("beta", "beta follow-up")
            .await
            .expect("beta turn should succeed");
        runtime
            .route_topics("alpha", Some("alpha planning"), 4, 4, 4, 1)
            .expect("alpha route should succeed");
        runtime
            .archive_session(Some("beta"))
            .expect("archive should succeed");

        let overview = runtime
            .session_activity_overview(1, 2)
            .expect("session activity overview should succeed");

        assert_eq!(overview.sessions.len(), 2);
        assert_eq!(overview.active_sessions, 1);
        assert_eq!(overview.archived_sessions, 1);
        assert_eq!(overview.sessions_with_history, 2);
        assert_eq!(overview.sessions_with_events, 2);
        assert_eq!(overview.sessions_with_topic_state, 1);
        assert_eq!(overview.total_topic_sessions, 1);
        assert_eq!(overview.total_topic_graph_edges, 0);
    }

    #[tokio::test]
    async fn event_digest_rolls_up_recent_events_by_kind_and_session() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_session("alpha")
            .expect("switch should succeed");
        runtime
            .run_demo_turn("alpha planning")
            .await
            .expect("alpha turn should succeed");
        runtime
            .rename_active_session("Alpha workspace")
            .expect("rename should succeed");
        runtime
            .run_demo_turn_in_session("beta", "beta follow-up")
            .await
            .expect("beta turn should succeed");

        let digest = runtime
            .event_digest(0)
            .expect("event digest should succeed");

        assert!(
            digest
                .kinds
                .iter()
                .any(|item| item.kind == "SessionRenamed" && item.count >= 1)
        );
        assert!(
            digest
                .sessions
                .iter()
                .any(|item| item.session_id.as_deref() == Some("bootstrap") && item.count >= 1)
        );
        let alpha = digest
            .sessions
            .iter()
            .find(|item| item.session_id.as_deref() == Some("alpha"))
            .expect("alpha session tally should exist");
        assert_eq!(alpha.latest_event.event.kind, EventKind::SessionRenamed);
        assert!(
            digest
                .events
                .iter()
                .any(|record| record.event.summary.contains("Alpha workspace"))
        );

        let sections = digest.summary_sections();
        assert_eq!(digest.recent_event_count(), digest.events.len());
        assert_eq!(digest.kind_count(), digest.kinds.len());
        assert_eq!(digest.session_scope_count(), digest.sessions.len());
        assert!(sections.iter().any(|line| line == "By kind:"));
        assert!(sections.iter().any(|line| line == "By session:"));
        assert!(sections.iter().any(|line| line == "Recent events:"));
        assert!(sections.iter().any(|line| line.contains("SessionRenamed")));
        assert!(sections.iter().any(|line| line.contains("Alpha workspace")));
    }
}
