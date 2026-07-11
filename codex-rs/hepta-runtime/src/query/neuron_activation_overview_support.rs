use hepta_core::HeptaNeuron;
use hepta_core::NeuronActivation;
use hepta_core::TopicActivationScore;
use hepta_core::TopicSession;

use super::build_bootstrap_neuron_activation;
use super::collect_bootstrap_direct_seeds;
use super::detect_bootstrap_inhibition_marker;

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
