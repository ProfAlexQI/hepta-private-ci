use hepta_core::SessionId;
use hepta_core::TranscriptEntry;
use hepta_core::TranscriptQuery;
use hepta_core::TranscriptQueryReport;
use hepta_core::TranscriptSpan;

pub(super) fn request(session_id: Option<&str>, query: &str, limit: usize) -> TranscriptQuery {
    TranscriptQuery {
        session_id: session_id.map(|id| SessionId(id.to_string())),
        text: query.to_string(),
        limit,
    }
}

pub(super) fn empty_report(session_id: &str, limit: usize) -> TranscriptQueryReport {
    TranscriptQueryReport::from_hits(request(Some(session_id), "", limit), 0, Vec::new())
}

pub(super) fn fallback_legacy_report(
    transcript_query: TranscriptQuery,
    entries: Vec<TranscriptEntry>,
) -> TranscriptQueryReport {
    let matched = entries
        .into_iter()
        .filter(|entry| entry.matches_query(&transcript_query))
        .map(TranscriptSpan::from_entry)
        .collect::<Vec<_>>();
    let matched_count = matched.len();
    let mut hits = matched;
    hits.truncate(transcript_query.limit);

    TranscriptQueryReport::from_hits(transcript_query, matched_count, hits)
}
