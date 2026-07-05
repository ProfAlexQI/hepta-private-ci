use hepta_core::MemoryRecord;
use hepta_core::TranscriptEntry;
use hepta_core::TranscriptQuery;
use hepta_core::TranscriptSpan;

pub(super) fn transcript_query_hits(
    entries: &[TranscriptEntry],
    query: &TranscriptQuery,
) -> (usize, Vec<TranscriptSpan>) {
    let mut matched_entries = entries
        .iter()
        .filter(|entry| entry.matches_query(query))
        .cloned()
        .collect::<Vec<_>>();
    let matched_count = matched_entries.len();
    matched_entries.sort_by(|left, right| {
        right
            .created_at_unix_ms
            .cmp(&left.created_at_unix_ms)
            .then_with(|| right.sequence.cmp(&left.sequence))
            .then_with(|| left.session_id.0.cmp(&right.session_id.0))
            .then_with(|| left.entry_id.cmp(&right.entry_id))
    });
    let hits = matched_entries
        .into_iter()
        .take(query.limit)
        .map(TranscriptSpan::from_entry)
        .collect();

    (matched_count, hits)
}

pub(crate) const MEMORY_RECALL_TOMBSTONE_MARKER: &str = "[hepta-memory:tombstone]";
pub(crate) const MEMORY_RECALL_CONFLICT_MARKER: &str = "[hepta-memory:conflict]";

fn memory_record_is_recall_control(record: &MemoryRecord) -> bool {
    record.content.contains(MEMORY_RECALL_TOMBSTONE_MARKER)
        || record.content.contains(MEMORY_RECALL_CONFLICT_MARKER)
}

pub(crate) fn memory_records_matching_recall_query(
    records: &[MemoryRecord],
    query_text: &str,
) -> (Vec<MemoryRecord>, usize) {
    let mut matches = Vec::new();
    let mut omitted_control_count = 0;

    for record in records {
        if !record.content.contains(query_text) {
            continue;
        }
        if memory_record_is_recall_control(record) {
            omitted_control_count += 1;
            continue;
        }
        matches.push(record.clone());
    }

    (matches, omitted_control_count)
}
