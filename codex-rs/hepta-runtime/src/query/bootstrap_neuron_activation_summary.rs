use hepta_core::NeuronActivation;

use super::BootstrapNeuronSeed;
use super::bootstrap_neuron_activation_support::BootstrapNeuronActivationSources;

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
