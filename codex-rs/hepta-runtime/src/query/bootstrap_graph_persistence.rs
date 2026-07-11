use std::collections::BTreeSet;

use hepta_core::TopicGraphEdgeKind;
use hepta_core::TopicSession;
use hepta_core::TopicShiftKind;

use super::BootstrapTopicCandidateRoute;
use super::TopicGraphState;
use super::bootstrap_topic_graph_edge_weight;
use super::bootstrap_topic_graph_relation_for_shift_kind;
use super::upsert_bootstrap_topic_graph_edge;

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
            sessions
                .iter()
                .position(|topic_session| topic_session.topic_session_id == route.topic_session_id)
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
