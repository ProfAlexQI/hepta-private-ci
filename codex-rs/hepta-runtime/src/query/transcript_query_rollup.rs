use std::collections::BTreeMap;

use hepta_core::TranscriptQueryReport;

use super::RuntimeTranscriptQueryOverview;
use super::RuntimeTranscriptQuerySessionTally;

pub(super) fn build(report: TranscriptQueryReport) -> RuntimeTranscriptQueryOverview {
    let returned_entries = report.hits.iter().map(|span| span.entry_count).sum();
    let sessions = tally_sessions(&report);
    let matched_sessions = sessions.len();

    RuntimeTranscriptQueryOverview {
        report,
        returned_entries,
        matched_sessions,
        sessions,
    }
}

fn tally_sessions(report: &TranscriptQueryReport) -> Vec<RuntimeTranscriptQuerySessionTally> {
    let mut sessions = BTreeMap::<String, RuntimeTranscriptQuerySessionTally>::new();

    for span in &report.hits {
        let entry = sessions
            .entry(span.session_id.0.clone())
            .or_insert_with(|| RuntimeTranscriptQuerySessionTally {
                session_id: span.session_id.0.clone(),
                hit_count: 0,
                entry_count: 0,
            });
        entry.hit_count += 1;
        entry.entry_count += span.entry_count;
    }

    let mut sessions = sessions.into_values().collect::<Vec<_>>();
    sessions.sort_by(|left, right| {
        right
            .hit_count
            .cmp(&left.hit_count)
            .then_with(|| right.entry_count.cmp(&left.entry_count))
            .then_with(|| left.session_id.cmp(&right.session_id))
    });
    sessions
}
