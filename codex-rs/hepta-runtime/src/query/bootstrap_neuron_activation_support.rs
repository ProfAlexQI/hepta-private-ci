use hepta_core::NeuronId;
use hepta_core::NeuronLinkKind;
use hepta_core::TranscriptSpanRef;

use super::BootstrapNeuronSeed;
use super::compute_bootstrap_inhibition_score;
use super::compute_bootstrap_propagated_score;
use super::infer_bootstrap_neuron_inhibition_link;
use super::infer_bootstrap_neuron_propagation_link;
use super::merge_bootstrap_topic_session_transcript_evidence;
use super::record_bootstrap_neuron_link;

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
