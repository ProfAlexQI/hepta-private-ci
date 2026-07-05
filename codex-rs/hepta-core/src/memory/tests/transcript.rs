use super::*;

#[test]
fn transcript_entry_roundtrips_through_json() {
    let entry = TranscriptEntry {
        entry_id: "entry-1".into(),
        session_id: SessionId("session-42".into()),
        sequence: 7,
        kind: TranscriptEntryKind::Summary,
        role: Some(MessageRole::Assistant),
        content: "condensed summary".into(),
        created_at_unix_ms: 77,
        tool_name: Some("write".into()),
        correlation_id: Some("corr-42".into()),
        summary_of_range: Some(TranscriptRange {
            start_sequence: 1,
            end_sequence: 6,
        }),
    };

    let json = serde_json::to_string(&entry).expect("transcript entry should serialize");
    let parsed: TranscriptEntry =
        serde_json::from_str(&json).expect("transcript entry should deserialize");

    assert_eq!(parsed, entry);
    assert!(
        parsed
            .summary_of_range
            .as_ref()
            .expect("summary range should be present")
            .contains(4)
    );
}

#[test]
fn transcript_query_report_tracks_counts_and_truncation() {
    let report = TranscriptQueryReport::from_hits(
        TranscriptQuery {
            session_id: Some(SessionId("session-42".into())),
            text: "approval".into(),
            limit: 1,
        },
        2,
        vec![TranscriptSpan {
            session_id: SessionId("session-42".into()),
            range: TranscriptRange {
                start_sequence: 3,
                end_sequence: 4,
            },
            entry_count: 2,
            excerpt: Some("approval was requested".into()),
            entries: vec![
                sample_transcript_entry(3, "please approve"),
                sample_transcript_entry(4, "approval granted"),
            ],
        }],
    );

    assert_eq!(report.query.text, "approval");
    assert_eq!(report.matched_count, 2);
    assert_eq!(report.returned_count, 1);
    assert!(report.truncated);
    assert!(!report.is_empty());
    assert_eq!(report.hits[0].entry_count, 2);
    assert!(!report.hits[0].is_empty());
}

#[test]
fn transcript_query_report_exposes_coverage_and_limit_pressure() {
    let report = TranscriptQueryReport::from_hits(
        TranscriptQuery {
            session_id: Some(SessionId("session-42".into())),
            text: "approval".into(),
            limit: 1,
        },
        2,
        vec![TranscriptSpan::from_entry(sample_transcript_entry(
            3,
            "approval granted",
        ))],
    );

    assert!(report.has_hits());
    assert_eq!(report.omitted_count(), 1);
    assert!(!report.is_complete());
    assert_eq!(
        report.coverage(),
        QueryReportCoverage {
            returned_count: 1,
            matched_count: 2,
        }
    );
    assert_eq!(
        report.limit_pressure(),
        QueryReportLimitPressure {
            truncated: true,
            omitted_count: 1,
        }
    );
    assert!(report.limit_pressure().truncated);
}

#[test]
fn transcript_entry_matches_query_with_optional_session_filter() {
    let entry = sample_transcript_entry(3, "approval granted");

    assert!(entry.matches_query(&TranscriptQuery {
        session_id: None,
        text: "approval".into(),
        limit: 5,
    }));
    assert!(entry.matches_query(&TranscriptQuery {
        session_id: Some(SessionId("session-42".into())),
        text: "granted".into(),
        limit: 5,
    }));
    assert!(!entry.matches_query(&TranscriptQuery {
        session_id: Some(SessionId("session-7".into())),
        text: "granted".into(),
        limit: 5,
    }));
    assert!(!entry.matches_query(&TranscriptQuery {
        session_id: None,
        text: "timeout".into(),
        limit: 5,
    }));
}

#[test]
fn transcript_span_from_entry_builds_single_entry_range() {
    let entry = sample_transcript_entry(11, "snapshot restored");

    let span = TranscriptSpan::from_entry(entry.clone());

    assert_eq!(span.session_id, entry.session_id);
    assert_eq!(span.range.start_sequence, 11);
    assert_eq!(span.range.end_sequence, 11);
    assert_eq!(span.entry_count, 1);
    assert_eq!(span.excerpt.as_deref(), Some("snapshot restored"));
    assert_eq!(span.entries, vec![entry]);
    assert!(!span.is_empty());
}
