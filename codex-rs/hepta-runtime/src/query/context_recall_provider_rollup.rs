use hepta_core::ContextRecallSource;

use super::RuntimeContextRecallProviderRollup;
use super::RuntimeContextRecallSelectionSummary;
use super::RuntimeContextRecallSlice;

pub(super) fn build(slice: &RuntimeContextRecallSlice) -> RuntimeContextRecallProviderRollup {
    let mut returned_sources = [false; ContextRecallSource::COUNT];
    let mut ranked_sources = [false; ContextRecallSource::COUNT];

    if slice.recent_entry_count > 0 {
        returned_sources[ContextRecallSource::RecentWindow.registry_index()] = true;
    }
    if slice.transcript_returned_count > 0 {
        returned_sources[ContextRecallSource::Transcript.registry_index()] = true;
    }
    if slice.durable_memory_hit_count > 0 {
        returned_sources[ContextRecallSource::DurableMemory.registry_index()] = true;
    }
    if slice.summary_hit_count > 0 {
        returned_sources[ContextRecallSource::SummaryMemory.registry_index()] = true;
    }
    if slice.active_topic_session_count > 0 {
        returned_sources[ContextRecallSource::ActiveTopicSession.registry_index()] = true;
    }
    if !slice.bundle.active_neurons.is_empty() {
        returned_sources[ContextRecallSource::ActiveNeuron.registry_index()] = true;
    }

    for item in &slice.bundle.ranked_items {
        returned_sources[item.source.registry_index()] = true;
        ranked_sources[item.source.registry_index()] = true;
    }

    let returned_source_count = count_sources(&returned_sources);
    let selected_source_count = count_sources(&ranked_sources);
    let ranked_source_count = selected_source_count;
    let source_diversity_target = to_u32(slice.bundle.budget.min_source_diversity);
    let source_diversity_met =
        source_diversity_target == 0 || selected_source_count >= source_diversity_target;

    RuntimeContextRecallProviderRollup {
        recall_selection: RuntimeContextRecallSelectionSummary {
            returned_source_count,
            selected_source_count,
            ranked_source_count,
            returned_unselected_source_count: returned_source_count
                .saturating_sub(selected_source_count),
            source_diversity_met,
            source_diversity_target,
            max_per_source: to_u32(slice.bundle.budget.max_per_source),
            ranked_item_count: to_u32(slice.bundle.ranked_items.len()),
            omitted_by_budget_count: to_u32(slice.bundle.omitted_by_budget),
            memory_control_omitted_count: to_u32(slice.memory_control_omitted_count),
            low_trust_ranked_item_count: to_u32(slice.low_trust_ranked_item_count),
            low_recency_ranked_item_count: to_u32(slice.low_recency_ranked_item_count),
        },
    }
}

fn count_sources(sources: &[bool; ContextRecallSource::COUNT]) -> u32 {
    to_u32(sources.iter().filter(|selected| **selected).count())
}

fn to_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}
