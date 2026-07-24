use std::collections::BTreeSet;

use hepta_core::NeuronLinkKind;
use hepta_core::TopicGraphEdge;
use hepta_core::TopicGraphEdgeKind;
use hepta_core::TopicSession;

use super::DirectSeed;
use crate::extract_semantic_terms;

pub(super) fn infer_propagation_link(
    source: &DirectSeed<'_>,
    target: &DirectSeed<'_>,
) -> Option<(NeuronLinkKind, f32, String)> {
    if let Some(edge) =
        topic_graph_edge(source.topic_session, &target.topic_session.topic_session_id)
    {
        let (kind, reason) = match edge.kind {
            TopicGraphEdgeKind::CoActivation => (
                NeuronLinkKind::WorkflowAdjacency,
                format!(
                    "bootstrap stored co-activation edge into '{}' strength {:.2}",
                    target.topic_session.topic_label.0, edge.weight,
                ),
            ),
            TopicGraphEdgeKind::SplitComponent => (
                NeuronLinkKind::TemporalContinuation,
                format!(
                    "bootstrap stored split-component edge into '{}' strength {:.2}",
                    target.topic_session.topic_label.0, edge.weight,
                ),
            ),
            TopicGraphEdgeKind::MergedInto | TopicGraphEdgeKind::HasComponent => (
                NeuronLinkKind::CausalDependency,
                format!(
                    "bootstrap stored merge-component edge into '{}' strength {:.2}",
                    target.topic_session.topic_label.0, edge.weight,
                ),
            ),
            TopicGraphEdgeKind::SemanticSimilarity
            | TopicGraphEdgeKind::TemporalContinuation
            | TopicGraphEdgeKind::Conflict => (
                NeuronLinkKind::SemanticSimilarity,
                format!(
                    "bootstrap stored {} edge into '{}' strength {:.2}",
                    topic_graph_edge_relation(edge),
                    target.topic_session.topic_label.0,
                    edge.weight,
                ),
            ),
        };
        return Some((kind, edge.weight.min(0.46), reason));
    }

    if let Some(link) = source
        .neuron
        .links
        .iter()
        .find(|link| link.target_neuron_id == target.neuron.neuron_id)
        .filter(|link| {
            !matches!(
                link.kind,
                NeuronLinkKind::Conflict | NeuronLinkKind::Inhibition
            )
        })
    {
        let relation = link.relation.as_deref().unwrap_or("compressed_neuron_link");
        return Some((
            link.kind,
            link.strength.min(0.46),
            format!(
                "compressed neuron link '{}' into '{}' strength {:.2}",
                relation, target.neuron.topic_label.0, link.strength,
            ),
        ));
    }

    if let Some(link) = target
        .neuron
        .links
        .iter()
        .find(|link| link.target_neuron_id == source.neuron.neuron_id)
        .filter(|link| {
            !matches!(
                link.kind,
                NeuronLinkKind::Conflict | NeuronLinkKind::Inhibition
            )
        })
    {
        let relation = link.relation.as_deref().unwrap_or("compressed_neuron_link");
        return Some((
            link.kind,
            link.strength.min(0.46),
            format!(
                "compressed reciprocal neuron link '{}' into '{}' strength {:.2}",
                relation, target.neuron.topic_label.0, link.strength,
            ),
        ));
    }

    let overlap = topic_session_label_overlap(source.topic_session, target.topic_session);
    let strength = (0.24 + overlap * 0.16).min(0.38);
    let reason = if overlap > 0.0 {
        format!(
            "bootstrap co-routed adjacency with semantic overlap {:.2}",
            overlap,
        )
    } else {
        "bootstrap co-routed adjacency from the same mixed turn".to_string()
    };
    Some((NeuronLinkKind::WorkflowAdjacency, strength, reason))
}

pub(super) fn infer_inhibition_link(
    source: &DirectSeed<'_>,
    target: &DirectSeed<'_>,
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

    let overlap = topic_session_label_overlap(source.topic_session, target.topic_session);
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

fn topic_graph_edge<'a>(
    source: &'a TopicSession,
    target_topic_session_id: &str,
) -> Option<&'a TopicGraphEdge> {
    source
        .graph_edges
        .iter()
        .find(|edge| edge.target_topic_session_id == target_topic_session_id)
}

fn topic_graph_edge_relation(edge: &TopicGraphEdge) -> &str {
    edge.relation.as_deref().unwrap_or(match edge.kind {
        TopicGraphEdgeKind::SemanticSimilarity => "semantic_similarity",
        TopicGraphEdgeKind::CoActivation => "co_activation",
        TopicGraphEdgeKind::SplitComponent => "split_component",
        TopicGraphEdgeKind::MergedInto => "merged_into",
        TopicGraphEdgeKind::HasComponent => "has_component",
        TopicGraphEdgeKind::TemporalContinuation => "temporal_continuation",
        TopicGraphEdgeKind::Conflict => "conflict",
    })
}

fn topic_session_label_overlap(left: &TopicSession, right: &TopicSession) -> f32 {
    let left_terms = extract_semantic_terms(&left.topic_label.0, 8);
    let right_terms = extract_semantic_terms(&right.topic_label.0, 8);
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

pub(super) fn detect_inhibition_marker(query_text: Option<&str>) -> Option<&'static str> {
    let lower = query_text?.to_ascii_lowercase();
    [" but not ", " instead of ", " rather than ", " except "]
        .into_iter()
        .find(|marker| lower.contains(marker))
}
