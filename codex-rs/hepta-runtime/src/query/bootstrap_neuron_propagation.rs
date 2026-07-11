use super::BootstrapNeuronSeed;
use super::bootstrap_neuron_id;
use super::bootstrap_topic_graph_edge;
use super::bootstrap_topic_graph_edge_relation;
use super::infer_bootstrap_propagation_link;
use hepta_core::NeuronId;
use hepta_core::NeuronLinkKind;
use hepta_core::TopicGraphEdgeKind;
use hepta_core::TopicSession;

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
