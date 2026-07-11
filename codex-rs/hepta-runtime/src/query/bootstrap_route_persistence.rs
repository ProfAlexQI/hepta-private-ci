use std::collections::BTreeMap;
use std::collections::BTreeSet;

use hepta_core::SessionId;
use hepta_core::TopicSession;
use hepta_core::TopicSessionStatus;
use hepta_core::TranscriptSpanRef;

use super::BootstrapTopicCandidateRoute;
use super::BootstrapTopicRoutePersistInputs;
use super::bootstrap_memory_refs;
use super::bootstrap_open_loops;
use super::bootstrap_planner::merge_bootstrap_topic_session_semantic_hints;
use super::merge_bootstrap_topic_session_transcript_evidence;

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
        merge_bootstrap_topic_session_semantic_hints(&mut existing.entities, &route.semantic_hints);
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
