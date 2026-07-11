use hepta_core::ContextBudget;
use hepta_core::ContextRecallBundle;
use hepta_core::ContextRecallItem;
use hepta_core::ContextRecallRequest;
use hepta_core::ContextRecallScore;
use hepta_core::ContextRecallSource;
use hepta_core::MemoryQueryReport;
use hepta_core::MemoryRecord;
use hepta_core::MemoryScope;
use hepta_core::TopicSession;
use hepta_core::TranscriptEntry;
use hepta_core::TranscriptQueryReport;
use hepta_core::TranscriptSpan;
use hepta_core::TranscriptSpanRef;

use super::LOW_RECENCY_RANKED_ITEM_RECENCY_THRESHOLD;
use super::LOW_TRUST_RANKED_ITEM_CONFIDENCE_THRESHOLD;
use super::RuntimeContextRecallSlice;

#[derive(Debug)]
pub(super) struct ContextRecallBuildInputs {
    pub(super) request: ContextRecallRequest,
    pub(super) recent_entries: Vec<TranscriptEntry>,
    pub(super) total_recent_entry_count: usize,
    pub(super) transcript_report: TranscriptQueryReport,
    pub(super) memory_report: MemoryQueryReport,
    pub(super) active_topic_sessions: Vec<TopicSession>,
}

pub(super) fn prepare_recent_entries(
    mut recent_entries: Vec<TranscriptEntry>,
    recent_window_limit: usize,
) -> (Vec<TranscriptEntry>, usize) {
    let total_recent_entry_count = recent_entries.len();
    if recent_entries.len() > recent_window_limit {
        recent_entries = recent_entries.split_off(recent_entries.len() - recent_window_limit);
    }

    (recent_entries, total_recent_entry_count)
}

pub(super) fn build_slice(inputs: ContextRecallBuildInputs) -> RuntimeContextRecallSlice {
    let ContextRecallBuildInputs {
        request,
        recent_entries,
        total_recent_entry_count,
        transcript_report,
        memory_report,
        active_topic_sessions,
    } = inputs;

    let recent_entry_count = recent_entries.len();
    let transcript_matched_count = transcript_report.matched_count;
    let transcript_returned_count = transcript_report.returned_count;
    let transcript_truncated = transcript_report.truncated;
    let transcript_hits = transcript_report.hits;
    let memory_matched_count = memory_report.matched_count;
    let memory_control_omitted_count = memory_report.omitted_control_count;
    let memory_truncated = memory_report.truncated;
    let (durable_memory_hits, summary_hits) = partition_memory_hits(memory_report.hits);
    let durable_memory_hit_count = durable_memory_hits.len();
    let summary_hit_count = summary_hits.len();
    let active_topic_session_count = active_topic_sessions.len();
    let budget = ContextBudget::from_request(&request);
    let (ranked_items, omitted_by_budget) = select_ranked_items_for_budget(
        build_ranked_items(
            &recent_entries,
            &transcript_hits,
            &durable_memory_hits,
            &summary_hits,
            &active_topic_sessions,
        ),
        &budget,
    );
    let bundle = ContextRecallBundle {
        request,
        recent_entries,
        transcript_hits,
        durable_memory_hits,
        summary_hits,
        active_topic_sessions,
        active_neurons: Vec::new(),
        budget,
        ranked_items,
        omitted_by_budget,
        truncated: transcript_truncated || memory_truncated || omitted_by_budget > 0,
    };
    let transcript_evidence = transcript_evidence(&bundle);
    let low_trust_ranked_item_count = low_trust_ranked_item_count(&bundle.ranked_items);
    let low_recency_ranked_item_count = low_recency_ranked_item_count(&bundle.ranked_items);

    RuntimeContextRecallSlice {
        bundle,
        recent_entry_count,
        total_recent_entry_count,
        transcript_matched_count,
        transcript_returned_count,
        memory_matched_count,
        durable_memory_hit_count,
        summary_hit_count,
        memory_control_omitted_count,
        active_topic_session_count,
        transcript_evidence,
        low_trust_ranked_item_count,
        low_recency_ranked_item_count,
    }
}

fn score(
    recency: f32,
    relevance: f32,
    durability: f32,
    topic_activation: f32,
    neuron_activation: f32,
    confidence: f32,
    reason: &str,
) -> ContextRecallScore {
    let final_score = ((recency * 0.22)
        + (relevance * 0.28)
        + (durability * 0.16)
        + (topic_activation * 0.14)
        + (neuron_activation * 0.10)
        + (confidence * 0.10))
        .clamp(0.0, 1.0);
    ContextRecallScore {
        recency,
        relevance,
        durability,
        topic_activation,
        neuron_activation,
        confidence,
        final_score,
        reason: Some(reason.to_string()),
    }
}

fn build_ranked_items(
    recent_entries: &[TranscriptEntry],
    transcript_hits: &[TranscriptSpan],
    durable_memory_hits: &[MemoryRecord],
    summary_hits: &[MemoryRecord],
    active_topic_sessions: &[TopicSession],
) -> Vec<ContextRecallItem> {
    let mut items = Vec::new();

    for entry in recent_entries {
        items.push(ContextRecallItem {
            source: ContextRecallSource::RecentWindow,
            source_id: entry.entry_id.clone(),
            summary: entry.content.chars().take(160).collect(),
            score: score(1.0, 0.72, 0.32, 0.0, 0.0, 0.80, "recent transcript window"),
            source_transcript_spans: vec![TranscriptSpanRef {
                session_id: entry.session_id.clone(),
                range: hepta_core::TranscriptRange {
                    start_sequence: entry.sequence,
                    end_sequence: entry.sequence,
                },
                reason: Some("ranked_recent_window".into()),
            }],
            source_memory_ids: Vec::new(),
            topic_session_ids: Vec::new(),
            neuron_ids: Vec::new(),
        });
    }

    for hit in transcript_hits {
        items.push(ContextRecallItem {
            source: ContextRecallSource::Transcript,
            source_id: format!(
                "{}:{}-{}",
                hit.session_id.0, hit.range.start_sequence, hit.range.end_sequence
            ),
            summary: hit
                .excerpt
                .clone()
                .unwrap_or_else(|| format!("{} transcript entrie(s)", hit.entry_count)),
            score: score(0.78, 1.0, 0.45, 0.0, 0.0, 0.82, "query transcript hit"),
            source_transcript_spans: vec![TranscriptSpanRef {
                session_id: hit.session_id.clone(),
                range: hit.range.clone(),
                reason: Some("ranked_transcript_hit".into()),
            }],
            source_memory_ids: Vec::new(),
            topic_session_ids: Vec::new(),
            neuron_ids: Vec::new(),
        });
    }

    for memory in durable_memory_hits {
        items.push(ContextRecallItem {
            source: ContextRecallSource::DurableMemory,
            source_id: memory.id.clone(),
            summary: memory.content.chars().take(160).collect(),
            score: score(
                0.58,
                0.78,
                1.0,
                0.0,
                0.0,
                0.74,
                "durable promoted memory hit",
            ),
            source_transcript_spans: Vec::new(),
            source_memory_ids: vec![memory.id.clone()],
            topic_session_ids: Vec::new(),
            neuron_ids: Vec::new(),
        });
    }

    for memory in summary_hits {
        items.push(ContextRecallItem {
            source: ContextRecallSource::SummaryMemory,
            source_id: memory.id.clone(),
            summary: memory.content.chars().take(160).collect(),
            score: score(
                0.52,
                0.70,
                0.64,
                0.0,
                0.0,
                0.68,
                "session summary memory hit",
            ),
            source_transcript_spans: Vec::new(),
            source_memory_ids: vec![memory.id.clone()],
            topic_session_ids: Vec::new(),
            neuron_ids: Vec::new(),
        });
    }

    for topic_session in active_topic_sessions {
        items.push(ContextRecallItem {
            source: ContextRecallSource::ActiveTopicSession,
            source_id: topic_session.topic_session_id.clone(),
            summary: topic_session.topic_label.0.clone(),
            score: score(
                0.74,
                0.72,
                0.72,
                1.0,
                0.0,
                0.76,
                "active topic session state",
            ),
            source_transcript_spans: topic_session.linked_transcript_spans.clone(),
            source_memory_ids: topic_session.durable_memory_refs.clone(),
            topic_session_ids: vec![topic_session.topic_session_id.clone()],
            neuron_ids: Vec::new(),
        });
    }

    items
}

pub(super) fn select_ranked_items_for_budget(
    mut items: Vec<ContextRecallItem>,
    budget: &ContextBudget,
) -> (Vec<ContextRecallItem>, usize) {
    sort_ranked_items(&mut items);
    let total_ranked = items.len();
    if budget.max_items == 0 || items.is_empty() {
        return (Vec::new(), total_ranked);
    }

    let max_items = budget.max_items;
    let max_per_source = budget.max_per_source.max(1);
    let diversity_target = budget.min_source_diversity.min(max_items);
    let mut selected_indices = vec![false; items.len()];
    let mut source_counts = Vec::new();
    let mut selected = Vec::new();

    if diversity_target > 0 {
        for (index, item) in items.iter().enumerate() {
            if selected.len() >= diversity_target {
                break;
            }
            let count = source_count(&source_counts, item.source);
            if count > 0 || count >= max_per_source {
                continue;
            }

            selected_indices[index] = true;
            increment_source_count(&mut source_counts, item.source);
            selected.push(item.clone());
        }
    }

    for (index, item) in items.iter().enumerate() {
        if selected.len() >= max_items {
            break;
        }
        if selected_indices[index] || source_count(&source_counts, item.source) >= max_per_source {
            continue;
        }

        selected_indices[index] = true;
        increment_source_count(&mut source_counts, item.source);
        selected.push(item.clone());
    }

    sort_ranked_items(&mut selected);
    let omitted_by_budget = total_ranked.saturating_sub(selected.len());

    (selected, omitted_by_budget)
}

pub(super) fn low_trust_ranked_item_count(items: &[ContextRecallItem]) -> usize {
    items
        .iter()
        .filter(|item| item.score.confidence < LOW_TRUST_RANKED_ITEM_CONFIDENCE_THRESHOLD)
        .count()
}

pub(super) fn low_recency_ranked_item_count(items: &[ContextRecallItem]) -> usize {
    items
        .iter()
        .filter(|item| item.score.recency < LOW_RECENCY_RANKED_ITEM_RECENCY_THRESHOLD)
        .count()
}

fn sort_ranked_items(items: &mut [ContextRecallItem]) {
    items.sort_by(|left, right| {
        right
            .score
            .final_score
            .total_cmp(&left.score.final_score)
            .then_with(|| left.source_id.cmp(&right.source_id))
    });
}

fn source_count(
    source_counts: &[(ContextRecallSource, usize)],
    source: ContextRecallSource,
) -> usize {
    source_counts
        .iter()
        .find_map(|(candidate, count)| (*candidate == source).then_some(*count))
        .unwrap_or_default()
}

fn increment_source_count(
    source_counts: &mut Vec<(ContextRecallSource, usize)>,
    source: ContextRecallSource,
) {
    if let Some((_, count)) = source_counts
        .iter_mut()
        .find(|(candidate, _)| *candidate == source)
    {
        *count += 1;
        return;
    }

    source_counts.push((source, 1));
}

fn partition_memory_hits(hits: Vec<MemoryRecord>) -> (Vec<MemoryRecord>, Vec<MemoryRecord>) {
    let mut durable_memory_hits = Vec::new();
    let mut summary_hits = Vec::new();

    for hit in hits {
        match hit.scope {
            MemoryScope::LongTerm => durable_memory_hits.push(hit),
            MemoryScope::Session => summary_hits.push(hit),
        }
    }

    (durable_memory_hits, summary_hits)
}

fn transcript_evidence(bundle: &ContextRecallBundle) -> Vec<TranscriptSpanRef> {
    let mut transcript_evidence_bundle = bundle.clone();
    transcript_evidence_bundle.active_topic_sessions.clear();
    transcript_evidence_bundle.source_transcript_spans()
}

#[cfg(test)]
mod tests {
    use hepta_core::ContextBudget;
    use hepta_core::ContextRecallItem;
    use hepta_core::ContextRecallScore;
    use hepta_core::ContextRecallSource;
    use hepta_core::MemoryRecord;
    use hepta_core::MemoryScope;
    use hepta_core::MessageRole;
    use hepta_core::SessionId;
    use hepta_core::TranscriptEntry;
    use hepta_core::TranscriptEntryKind;

    use super::low_recency_ranked_item_count;
    use super::low_trust_ranked_item_count;
    use super::partition_memory_hits;
    use super::prepare_recent_entries;
    use super::select_ranked_items_for_budget;

    #[test]
    fn prepare_recent_entries_preserves_total_count_when_truncating() {
        let entries = vec![
            transcript_entry(1, "first"),
            transcript_entry(2, "second"),
            transcript_entry(3, "third"),
        ];

        let (recent_entries, total_recent_entry_count) = prepare_recent_entries(entries, 2);

        assert_eq!(total_recent_entry_count, 3);
        assert_eq!(recent_entries.len(), 2);
        assert_eq!(recent_entries[0].sequence, 2);
        assert_eq!(recent_entries[1].sequence, 3);
    }

    #[test]
    fn partition_memory_hits_preserves_scope_order() {
        let hits = vec![
            memory_record("session-1", MemoryScope::Session),
            memory_record("long-1", MemoryScope::LongTerm),
            memory_record("session-2", MemoryScope::Session),
            memory_record("long-2", MemoryScope::LongTerm),
        ];

        let (durable_memory_hits, summary_hits) = partition_memory_hits(hits);

        assert_eq!(
            durable_memory_hits
                .iter()
                .map(|record| record.id.as_str())
                .collect::<Vec<_>>(),
            vec!["long-1", "long-2"]
        );
        assert_eq!(
            summary_hits
                .iter()
                .map(|record| record.id.as_str())
                .collect::<Vec<_>>(),
            vec!["session-1", "session-2"]
        );
    }

    #[test]
    fn select_ranked_items_for_budget_prefers_source_diversity_before_overflow() {
        let budget = ContextBudget {
            max_items: 3,
            max_tokens_estimate: 768,
            min_source_diversity: 3,
            max_per_source: 2,
        };
        let items = vec![
            ranked_item(ContextRecallSource::RecentWindow, "recent-1", 0.99),
            ranked_item(ContextRecallSource::RecentWindow, "recent-2", 0.98),
            ranked_item(ContextRecallSource::Transcript, "transcript-1", 0.50),
            ranked_item(ContextRecallSource::DurableMemory, "memory-1", 0.40),
        ];

        let (selected, omitted_by_budget) = select_ranked_items_for_budget(items, &budget);

        assert_eq!(omitted_by_budget, 1);
        assert_eq!(
            selected
                .iter()
                .map(|item| item.source_id.as_str())
                .collect::<Vec<_>>(),
            vec!["recent-1", "transcript-1", "memory-1"]
        );
    }

    #[test]
    fn select_ranked_items_for_budget_enforces_max_per_source_when_filling() {
        let budget = ContextBudget {
            max_items: 4,
            max_tokens_estimate: 1024,
            min_source_diversity: 4,
            max_per_source: 2,
        };
        let items = vec![
            ranked_item(ContextRecallSource::RecentWindow, "recent-1", 0.99),
            ranked_item(ContextRecallSource::RecentWindow, "recent-2", 0.98),
            ranked_item(ContextRecallSource::RecentWindow, "recent-3", 0.97),
            ranked_item(ContextRecallSource::Transcript, "transcript-1", 0.50),
            ranked_item(ContextRecallSource::Transcript, "transcript-2", 0.40),
        ];

        let (selected, omitted_by_budget) = select_ranked_items_for_budget(items, &budget);

        assert_eq!(omitted_by_budget, 1);
        assert_eq!(
            selected
                .iter()
                .map(|item| item.source_id.as_str())
                .collect::<Vec<_>>(),
            vec!["recent-1", "recent-2", "transcript-1", "transcript-2"]
        );
    }

    #[test]
    fn ranked_item_quality_counts_track_low_trust_and_low_recency() {
        let normal = ranked_item(ContextRecallSource::RecentWindow, "normal", 0.80);
        let mut low_trust = ranked_item(ContextRecallSource::DurableMemory, "low-trust", 0.80);
        low_trust.score.confidence = 0.49;
        let mut low_recency = ranked_item(ContextRecallSource::SummaryMemory, "low-recency", 0.80);
        low_recency.score.recency = 0.49;
        let mut low_both = ranked_item(ContextRecallSource::Transcript, "low-both", 0.80);
        low_both.score.confidence = 0.49;
        low_both.score.recency = 0.49;

        let items = vec![normal, low_trust, low_recency, low_both];

        assert_eq!(low_trust_ranked_item_count(&items), 2);
        assert_eq!(low_recency_ranked_item_count(&items), 2);
    }

    fn transcript_entry(sequence: u64, content: &str) -> TranscriptEntry {
        TranscriptEntry {
            entry_id: format!("entry-{sequence}"),
            session_id: SessionId("alpha".into()),
            sequence,
            kind: TranscriptEntryKind::Message,
            role: Some(MessageRole::User),
            content: content.to_string(),
            created_at_unix_ms: 0,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        }
    }

    fn memory_record(id: &str, scope: MemoryScope) -> MemoryRecord {
        MemoryRecord {
            id: id.to_string(),
            scope,
            content: format!("memory {id}"),
        }
    }

    fn ranked_item(
        source: ContextRecallSource,
        source_id: &str,
        final_score: f32,
    ) -> ContextRecallItem {
        ContextRecallItem {
            source,
            source_id: source_id.to_string(),
            summary: source_id.to_string(),
            score: ContextRecallScore {
                recency: final_score,
                relevance: final_score,
                durability: final_score,
                topic_activation: 0.0,
                neuron_activation: 0.0,
                confidence: final_score,
                final_score,
                reason: Some("test fixture".into()),
            },
            source_transcript_spans: Vec::new(),
            source_memory_ids: Vec::new(),
            topic_session_ids: Vec::new(),
            neuron_ids: Vec::new(),
        }
    }
}
