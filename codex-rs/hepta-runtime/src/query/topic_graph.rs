use super::TopicGraphState;
use super::topic_session_label_overlap;
use hepta_core::TopicGraphEdge;
use hepta_core::TopicGraphEdgeKind;
use hepta_core::TopicSession;
use hepta_core::TopicShiftKind;

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
            existing.edge.relation = Some(bootstrap_topic_graph_relation_label(kind).to_string());
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
