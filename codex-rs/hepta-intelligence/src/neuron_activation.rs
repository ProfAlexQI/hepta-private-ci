use std::collections::BTreeMap;
use std::collections::BTreeSet;

use hepta_core::HeptaNeuron;
use hepta_core::NeuronActivation;
use hepta_core::NeuronId;
use hepta_core::NeuronLinkKind;
use hepta_core::TopicActivationScore;
use hepta_core::TopicSession;
use hepta_core::TranscriptSpanRef;

use self::links::detect_inhibition_marker;
use self::links::infer_inhibition_link;
use self::links::infer_propagation_link;

mod links;

const MAX_ACTIVATION_TRANSCRIPT_SPAN_REFS: usize = 8;

/// Recall evidence counts used when routing did not provide a topic score.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NeuronActivationEvidenceCounts {
    pub recent_entry_count: usize,
    pub transcript_matched_count: usize,
    pub durable_memory_hit_count: usize,
    pub summary_hit_count: usize,
}

/// Immutable, already-hydrated cognitive state for one activation pass.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NeuronActivationInput<'a> {
    pub query_text: Option<&'a str>,
    pub topic_sessions: &'a [TopicSession],
    pub neurons: &'a [HeptaNeuron],
    pub active_topic_session_ids: &'a [String],
    pub activation_scores: &'a [TopicActivationScore],
    pub evidence_counts: NeuronActivationEvidenceCounts,
    pub limit: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DirectSeed<'a> {
    topic_session: &'a TopicSession,
    neuron: &'a HeptaNeuron,
    direct_score: f32,
}

#[derive(Debug, Clone, Default, PartialEq)]
struct ActivationSources {
    propagated_score: f32,
    inhibition_score: f32,
    source_topic_session_ids: Vec<String>,
    source_neuron_ids: Vec<NeuronId>,
    source_transcript_spans: Vec<TranscriptSpanRef>,
    source_link_kinds: Vec<NeuronLinkKind>,
    source_link_reasons: Vec<String>,
}

/// Computes explainable direct activation, graph propagation, and inhibition
/// without reading or mutating runtime state.
pub fn compute_neuron_activations(input: NeuronActivationInput<'_>) -> Vec<NeuronActivation> {
    if input.limit == 0 {
        return Vec::new();
    }

    let inhibition_marker = detect_inhibition_marker(input.query_text);
    let direct_seeds = collect_direct_seeds(input);

    direct_seeds
        .iter()
        .take(input.limit)
        .map(|seed| build_activation(seed, &direct_seeds, inhibition_marker))
        .collect()
}

fn collect_direct_seeds(input: NeuronActivationInput<'_>) -> Vec<DirectSeed<'_>> {
    let mut topic_session_by_id = input
        .topic_sessions
        .iter()
        .map(|topic_session| (topic_session.topic_session_id.as_str(), topic_session))
        .collect::<BTreeMap<_, _>>();
    let neuron_by_topic_id = input
        .neurons
        .iter()
        .map(|neuron| (neuron.topic_id.0.as_str(), neuron))
        .collect::<BTreeMap<_, _>>();

    input
        .active_topic_session_ids
        .iter()
        .filter_map(|active_id| topic_session_by_id.remove(active_id.as_str()))
        .filter_map(|topic_session| {
            let neuron = neuron_by_topic_id.get(topic_session.topic_id.0.as_str())?;
            let direct_score = input
                .activation_scores
                .iter()
                .find(|score| score.topic_id == topic_session.topic_id)
                .map(|score| score.score)
                .unwrap_or_else(|| direct_score_from_evidence(input.evidence_counts));

            (direct_score > 0.0).then_some(DirectSeed {
                topic_session,
                neuron,
                direct_score,
            })
        })
        .collect()
}

fn direct_score_from_evidence(counts: NeuronActivationEvidenceCounts) -> f32 {
    let recent_score = (counts.recent_entry_count.min(2) as f32) * 0.10;
    let transcript_score = (counts.transcript_matched_count.min(2) as f32) * 0.25;
    let durable_score = (counts.durable_memory_hit_count.min(2) as f32) * 0.20;
    let summary_score = (counts.summary_hit_count.min(2) as f32) * 0.10;

    (recent_score + transcript_score + durable_score + summary_score).min(1.0)
}

fn build_activation(
    seed: &DirectSeed<'_>,
    direct_seeds: &[DirectSeed<'_>],
    inhibition_marker: Option<&'static str>,
) -> NeuronActivation {
    let sources = collect_activation_sources(seed, direct_seeds, inhibition_marker);
    let direct_score = seed.direct_score;
    let propagated_score = sources
        .propagated_score
        .min((direct_score * 0.35).max(0.08));
    let inhibition_score = sources.inhibition_score;
    let final_score = (direct_score + propagated_score - inhibition_score).clamp(0.0, 1.0);
    let reason = build_activation_reason(
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

fn collect_activation_sources(
    seed: &DirectSeed<'_>,
    direct_seeds: &[DirectSeed<'_>],
    inhibition_marker: Option<&'static str>,
) -> ActivationSources {
    let mut sources = ActivationSources {
        source_topic_session_ids: vec![seed.topic_session.topic_session_id.clone()],
        ..Default::default()
    };
    merge_transcript_evidence(
        &mut sources.source_transcript_spans,
        &seed.neuron.important_transcript_spans,
    );

    if inhibition_marker.is_none() {
        for source_seed in direct_seeds {
            if source_seed.topic_session.topic_session_id == seed.topic_session.topic_session_id {
                continue;
            }

            if let Some((link_kind, link_strength, link_reason)) =
                infer_propagation_link(source_seed, seed)
            {
                let contribution = (source_seed.direct_score * link_strength * 0.20).min(0.18);
                if contribution <= 0.0 {
                    continue;
                }

                sources.propagated_score += contribution;
                record_source_link(source_seed, link_kind, &link_reason, &mut sources);
            }
        }
    }

    if let Some(marker) = inhibition_marker
        && let Some(source_seed) = direct_seeds.first()
        && source_seed.topic_session.topic_session_id != seed.topic_session.topic_session_id
        && let Some((link_strength, link_reason)) = infer_inhibition_link(source_seed, seed, marker)
    {
        sources.inhibition_score = (source_seed.direct_score * link_strength * 0.26)
            .min(0.22)
            .min((seed.direct_score * 0.55).max(0.08));
        if sources.inhibition_score > 0.0 {
            record_source_link(
                source_seed,
                NeuronLinkKind::Inhibition,
                &link_reason,
                &mut sources,
            );
        }
    }

    sources
}

fn record_source_link(
    source: &DirectSeed<'_>,
    link_kind: NeuronLinkKind,
    link_reason: &str,
    sources: &mut ActivationSources,
) {
    merge_transcript_evidence(
        &mut sources.source_transcript_spans,
        &source.neuron.important_transcript_spans,
    );

    if sources
        .source_topic_session_ids
        .iter()
        .all(|session_id| session_id != &source.topic_session.topic_session_id)
    {
        sources
            .source_topic_session_ids
            .push(source.topic_session.topic_session_id.clone());
    }

    let source_neuron_id = source.neuron.neuron_id.clone();
    if !sources.source_neuron_ids.contains(&source_neuron_id) {
        sources.source_neuron_ids.push(source_neuron_id);
    }

    sources.source_link_kinds.push(link_kind);
    sources.source_link_reasons.push(format!(
        "{} via {}",
        link_reason, source.topic_session.topic_session_id,
    ));
}

fn build_activation_reason(
    seed: &DirectSeed<'_>,
    propagated_score: f32,
    inhibition_score: f32,
    source_neuron_count: usize,
) -> String {
    let prior_count = seed.neuron.skill_priors.len() + seed.neuron.workflow_priors.len();

    if inhibition_score > 0.0 {
        format!(
            "bootstrap direct activation via routed topic session '{}' for compressed neuron '{}' with {} open loops, {} durable refs, and {} prior(s), then inhibitory suppression {:.2} from {} linked neuron(s)",
            seed.topic_session.topic_session_id,
            seed.neuron.topic_label.0,
            seed.neuron.open_loops.len(),
            seed.neuron.promoted_memory_refs.len(),
            prior_count,
            inhibition_score,
            source_neuron_count,
        )
    } else if propagated_score > 0.0 {
        format!(
            "bootstrap direct activation via routed topic session '{}' for compressed neuron '{}' with {} open loops, {} durable refs, and {} prior(s), plus propagated activation {:.2} from {} linked neuron(s)",
            seed.topic_session.topic_session_id,
            seed.neuron.topic_label.0,
            seed.neuron.open_loops.len(),
            seed.neuron.promoted_memory_refs.len(),
            prior_count,
            propagated_score,
            source_neuron_count,
        )
    } else {
        format!(
            "bootstrap direct activation via routed topic session '{}' for compressed neuron '{}' with {} open loops, {} durable refs, and {} prior(s); no additional propagated activation fired yet",
            seed.topic_session.topic_session_id,
            seed.neuron.topic_label.0,
            seed.neuron.open_loops.len(),
            seed.neuron.promoted_memory_refs.len(),
            prior_count,
        )
    }
}

fn merge_transcript_evidence(
    linked_transcript_spans: &mut Vec<TranscriptSpanRef>,
    transcript_evidence: &[TranscriptSpanRef],
) {
    for incoming in transcript_evidence {
        if let Some(existing) = linked_transcript_spans.iter_mut().find(|existing| {
            existing.session_id == incoming.session_id && existing.range == incoming.range
        }) {
            existing.reason = merge_transcript_span_reasons(
                existing.reason.as_deref(),
                incoming.reason.as_deref(),
            );
        } else {
            linked_transcript_spans.push(incoming.clone());
        }
    }

    linked_transcript_spans.sort_by(|left, right| {
        right
            .range
            .end_sequence
            .cmp(&left.range.end_sequence)
            .then_with(|| right.range.start_sequence.cmp(&left.range.start_sequence))
            .then_with(|| left.session_id.0.cmp(&right.session_id.0))
    });
    linked_transcript_spans.truncate(MAX_ACTIVATION_TRANSCRIPT_SPAN_REFS);
}

fn merge_transcript_span_reasons(existing: Option<&str>, incoming: Option<&str>) -> Option<String> {
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

#[cfg(test)]
mod tests;
