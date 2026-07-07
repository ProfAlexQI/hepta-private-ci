use hepta_core::ContextRecallBundle;
use hepta_core::ContextRecallItem;
use hepta_core::ContextRecallScore;
use hepta_core::ContextRecallSource;
use hepta_core::MemoryRecord;
use hepta_core::MemoryScope;
use hepta_core::TranscriptEntry;
use hepta_core::TranscriptRange;
use hepta_core::TranscriptSpan;
use hepta_core::TranscriptSpanRef;

pub(super) fn ranked_recall_items(bundle: &ContextRecallBundle) -> (Vec<ContextRecallItem>, usize) {
    let query = bundle.request.normalized_query_text();
    let mut items = Vec::new();

    let mut recent_entries = bundle.recent_entries.iter().collect::<Vec<_>>();
    recent_entries.sort_by(|left, right| {
        right
            .created_at_unix_ms
            .cmp(&left.created_at_unix_ms)
            .then_with(|| right.sequence.cmp(&left.sequence))
            .then_with(|| left.session_id.0.cmp(&right.session_id.0))
            .then_with(|| left.entry_id.cmp(&right.entry_id))
    });
    let recent_total = recent_entries.len();
    for (index, entry) in recent_entries.into_iter().enumerate() {
        items.push(ranked_recent_entry(entry, query, index, recent_total));
    }

    let transcript_total = bundle.transcript_hits.len();
    for (index, span) in bundle.transcript_hits.iter().enumerate() {
        items.push(ranked_transcript_span(span, query, index, transcript_total));
    }

    let durable_total = bundle.durable_memory_hits.len();
    for (index, record) in bundle.durable_memory_hits.iter().enumerate() {
        items.push(ranked_memory_record(
            record,
            query,
            ContextRecallSource::DurableMemory,
            index,
            durable_total,
        ));
    }

    let summary_total = bundle.summary_hits.len();
    for (index, record) in bundle.summary_hits.iter().enumerate() {
        items.push(ranked_memory_record(
            record,
            query,
            ContextRecallSource::SummaryMemory,
            index,
            summary_total,
        ));
    }

    items.sort_by(|left, right| {
        right
            .score
            .final_score
            .total_cmp(&left.score.final_score)
            .then_with(|| left.source_id.cmp(&right.source_id))
    });

    let max_ranked_items = bundle.budget.max_items.max(1);
    let omitted_by_budget = items.len().saturating_sub(max_ranked_items);
    items.truncate(max_ranked_items);

    (items, omitted_by_budget)
}

fn ranked_recent_entry(
    entry: &TranscriptEntry,
    query: Option<&str>,
    index: usize,
    total: usize,
) -> ContextRecallItem {
    let relevance = relevance_score(&entry.content, query, 0.35);
    let score = score(
        recency_score(index, total),
        relevance,
        0.55,
        "recent_window",
    );
    ContextRecallItem {
        source: ContextRecallSource::RecentWindow,
        source_id: format!("recent:{}:{}", entry.session_id.0, entry.sequence),
        summary: payload_light_summary("recent_window", entry.content.len(), relevance),
        score,
        source_transcript_spans: vec![TranscriptSpanRef {
            session_id: entry.session_id.clone(),
            range: TranscriptRange {
                start_sequence: entry.sequence,
                end_sequence: entry.sequence,
            },
            reason: Some(reason_for_relevance("recent_window", relevance)),
        }],
        source_memory_ids: Vec::new(),
        topic_session_ids: Vec::new(),
        neuron_ids: Vec::new(),
    }
}

fn ranked_transcript_span(
    span: &TranscriptSpan,
    query: Option<&str>,
    index: usize,
    total: usize,
) -> ContextRecallItem {
    let matched_text = span
        .excerpt
        .as_deref()
        .or_else(|| span.entries.first().map(|entry| entry.content.as_str()))
        .unwrap_or_default();
    let relevance = relevance_score(matched_text, query, 1.0);
    let score = score(
        recency_score(index, total),
        relevance,
        0.65,
        "transcript_query_match",
    );
    ContextRecallItem {
        source: ContextRecallSource::Transcript,
        source_id: format!(
            "transcript:{}:{}-{}",
            span.session_id.0, span.range.start_sequence, span.range.end_sequence
        ),
        summary: payload_light_summary("transcript", matched_text.len(), relevance),
        score,
        source_transcript_spans: vec![TranscriptSpanRef {
            session_id: span.session_id.clone(),
            range: span.range.clone(),
            reason: Some(reason_for_relevance("query_match", relevance)),
        }],
        source_memory_ids: Vec::new(),
        topic_session_ids: Vec::new(),
        neuron_ids: Vec::new(),
    }
}

fn ranked_memory_record(
    record: &MemoryRecord,
    query: Option<&str>,
    source: ContextRecallSource,
    index: usize,
    total: usize,
) -> ContextRecallItem {
    let relevance = relevance_score(&record.content, query, 1.0);
    let durability = match record.scope {
        MemoryScope::LongTerm => 1.0,
        MemoryScope::Session => 0.8,
    };
    let label = match source {
        ContextRecallSource::DurableMemory => "durable_memory",
        ContextRecallSource::SummaryMemory => "summary_memory",
        _ => "memory",
    };
    let score = score(recency_score(index, total), relevance, durability, label);
    ContextRecallItem {
        source,
        source_id: format!("{label}:{}", record.id),
        summary: payload_light_summary(label, record.content.len(), relevance),
        score,
        source_transcript_spans: Vec::new(),
        source_memory_ids: vec![record.id.clone()],
        topic_session_ids: Vec::new(),
        neuron_ids: Vec::new(),
    }
}

fn score(recency: f32, relevance: f32, durability: f32, reason: &str) -> ContextRecallScore {
    let confidence = if relevance >= 1.0 {
        0.85
    } else if relevance >= 0.5 {
        0.65
    } else {
        0.45
    };
    let final_score =
        round_score(0.25 * recency + 0.35 * relevance + 0.25 * durability + 0.15 * confidence);

    ContextRecallScore {
        recency: round_score(recency),
        relevance: round_score(relevance),
        durability: round_score(durability),
        topic_activation: 0.0,
        neuron_activation: 0.0,
        confidence: round_score(confidence),
        final_score,
        reason: Some(reason.to_string()),
    }
}

fn recency_score(index: usize, total: usize) -> f32 {
    if total <= 1 {
        return 1.0;
    }
    1.0 - ((index as f32) / ((total - 1) as f32) * 0.35)
}

fn relevance_score(text: &str, query: Option<&str>, fallback: f32) -> f32 {
    let Some(query) = query else {
        return fallback;
    };
    if text.to_lowercase().contains(&query.to_lowercase()) {
        1.0
    } else {
        fallback
    }
}

fn payload_light_summary(label: &str, content_bytes: usize, relevance: f32) -> String {
    format!(
        "{label}; content_bytes={content_bytes}; query_match={}",
        relevance >= 1.0
    )
}

fn reason_for_relevance(prefix: &str, relevance: f32) -> String {
    if relevance >= 1.0 {
        format!("{prefix}, query_match")
    } else {
        prefix.to_string()
    }
}

fn round_score(value: f32) -> f32 {
    (value * 1000.0).round() / 1000.0
}
