use super::*;

#[tokio::test]
async fn search_filters_results_and_honors_limit() {
    let store = InMemoryStore::default();
    let matching_a = memory_record("memory-1", MemoryScope::Session, "doctor snapshot ok");
    let non_matching = memory_record("memory-2", MemoryScope::LongTerm, "approval ledger");
    let matching_b = memory_record("memory-3", MemoryScope::LongTerm, "snapshot rollback");

    store
        .put(matching_a.clone())
        .await
        .expect("put should succeed");
    store.put(non_matching).await.expect("put should succeed");
    store
        .put(matching_b.clone())
        .await
        .expect("put should succeed");

    let hits = store
        .search(MemoryQuery {
            text: "snapshot".into(),
            limit: 1,
        })
        .await
        .expect("search should succeed");

    assert_eq!(hits, vec![matching_a]);
}

#[tokio::test]
async fn search_report_suppresses_tombstone_and_conflict_control_records() {
    let store = InMemoryStore::default();
    let matching = memory_record(
        "memory-1",
        MemoryScope::LongTerm,
        "stale rollout plan current decision",
    );
    let tombstone = memory_record(
        "memory-2",
        MemoryScope::LongTerm,
        "[hepta-memory:tombstone] stale rollout plan retired decision",
    );
    let conflict = memory_record(
        "memory-3",
        MemoryScope::Session,
        "[hepta-memory:conflict] stale rollout plan conflicting summary",
    );
    let non_matching = memory_record("memory-4", MemoryScope::Session, "approval ledger");
    let query = MemoryQuery {
        text: "stale rollout plan".into(),
        limit: 10,
    };

    store
        .put(matching.clone())
        .await
        .expect("put should succeed");
    store.put(tombstone).await.expect("put should succeed");
    store.put(conflict).await.expect("put should succeed");
    store.put(non_matching).await.expect("put should succeed");

    let report = store
        .search_report(query.clone())
        .expect("search report should succeed");
    let hits = store
        .search(query.clone())
        .await
        .expect("search should succeed");
    let snapshot_report = store
        .snapshot()
        .expect("snapshot should load")
        .search_report(&query);
    let serialized = serde_json::to_string(&report).expect("report should serialize");

    assert_eq!(report, snapshot_report);
    assert_eq!(report.matched_count, 1);
    assert_eq!(report.returned_count, 1);
    assert_eq!(report.omitted_control_count, 2);
    assert!(!report.truncated);
    assert_eq!(report.hits, vec![matching.clone()]);
    assert_eq!(hits, vec![matching]);
    assert!(!serialized.contains(MEMORY_RECALL_TOMBSTONE_MARKER));
    assert!(!serialized.contains(MEMORY_RECALL_CONFLICT_MARKER));
}

#[tokio::test]
async fn search_report_tracks_total_matches_and_truncation() {
    let store = InMemoryStore::default();
    let matching_a = memory_record("memory-1", MemoryScope::Session, "doctor snapshot ok");
    let non_matching = memory_record("memory-2", MemoryScope::LongTerm, "approval ledger");
    let matching_b = memory_record("memory-3", MemoryScope::LongTerm, "snapshot rollback");

    store
        .put(matching_a.clone())
        .await
        .expect("put should succeed");
    store.put(non_matching).await.expect("put should succeed");
    store.put(matching_b).await.expect("put should succeed");

    let report = store
        .search_report(MemoryQuery {
            text: "snapshot".into(),
            limit: 1,
        })
        .expect("search report should succeed");

    assert_eq!(report.query.text, "snapshot");
    assert_eq!(report.query.limit, 1);
    assert_eq!(report.matched_count, 2);
    assert_eq!(report.returned_count, 1);
    assert!(report.truncated);
    assert_eq!(report.hits, vec![matching_a]);
    assert!(!report.is_empty());
}

#[tokio::test]
async fn search_coverage_and_limit_pressure_match_report_helpers() {
    let store = InMemoryStore::default();
    let matching_a = memory_record("memory-1", MemoryScope::Session, "doctor snapshot ok");
    let matching_b = memory_record("memory-2", MemoryScope::LongTerm, "snapshot rollback");
    let query = MemoryQuery {
        text: "snapshot".into(),
        limit: 1,
    };

    store
        .put(matching_a.clone())
        .await
        .expect("put should succeed");
    store
        .put(matching_b.clone())
        .await
        .expect("put should succeed");

    let snapshot = store.snapshot().expect("snapshot should load");
    let report = store
        .search_report(query.clone())
        .expect("search report should succeed");

    assert_eq!(snapshot.search_report(&query), report);
    assert_eq!(
        snapshot.search_coverage(&query),
        QueryReportCoverage {
            returned_count: 1,
            matched_count: 2,
        }
    );
    assert_eq!(
        store
            .search_coverage(query.clone())
            .expect("search coverage should succeed"),
        report.coverage()
    );
    assert_eq!(
        snapshot.search_limit_pressure(&query),
        QueryReportLimitPressure {
            truncated: true,
            omitted_count: 1,
        }
    );
    assert_eq!(
        store
            .search_limit_pressure(query)
            .expect("search limit pressure should succeed"),
        report.limit_pressure()
    );
}

#[tokio::test]
async fn search_report_with_zero_limit_preserves_match_counts_and_full_omission() {
    let store = InMemoryStore::default();
    let query = MemoryQuery {
        text: "snapshot".into(),
        limit: 0,
    };

    store
        .put(memory_record(
            "memory-1",
            MemoryScope::Session,
            "doctor snapshot ok",
        ))
        .await
        .expect("put should succeed");
    store
        .put(memory_record(
            "memory-2",
            MemoryScope::LongTerm,
            "snapshot rollback",
        ))
        .await
        .expect("put should succeed");

    let report = store
        .search_report(query.clone())
        .expect("search report should succeed");

    assert_eq!(report.query, query);
    assert_eq!(report.matched_count, 2);
    assert_eq!(report.returned_count, 0);
    assert!(report.truncated);
    assert!(report.hits.is_empty());
    assert_eq!(report.omitted_count(), 2);
    assert!(!report.is_complete());
    assert_eq!(
        report.coverage(),
        QueryReportCoverage {
            returned_count: 0,
            matched_count: 2,
        }
    );
    assert_eq!(
        report.limit_pressure(),
        QueryReportLimitPressure {
            truncated: true,
            omitted_count: 2,
        }
    );
}

#[tokio::test]
async fn search_report_hits_match_async_search_results() {
    let store = InMemoryStore::default();
    let matching = memory_record("memory-1", MemoryScope::LongTerm, "manifest payload");
    let query = MemoryQuery {
        text: "manifest".into(),
        limit: 5,
    };

    store
        .put(matching.clone())
        .await
        .expect("put should succeed");

    let report = store
        .search_report(query.clone())
        .expect("search report should succeed");
    let hits = store.search(query).await.expect("search should succeed");

    assert_eq!(report.matched_count, 1);
    assert_eq!(report.returned_count, 1);
    assert!(!report.truncated);
    assert_eq!(report.hits, hits);
    assert_eq!(report.hits, vec![matching]);
}

#[tokio::test]
async fn memory_report_store_trait_matches_inherent_search_report() {
    assert_memory_report_store::<InMemoryStore>();

    let store = InMemoryStore::default();
    let record = memory_record("memory-1", MemoryScope::LongTerm, "manifest payload");
    let query = MemoryQuery {
        text: "manifest".into(),
        limit: 5,
    };

    store.put(record.clone()).await.expect("put should succeed");

    let inherent = store
        .search_report(query.clone())
        .expect("inherent search report should succeed");
    let via_trait = <InMemoryStore as MemoryReportStore>::search_report(&store, query)
        .await
        .expect("trait search report should succeed");

    assert_eq!(via_trait, inherent);
    assert_eq!(via_trait.hits, vec![record]);
    assert_eq!(via_trait.returned_count, 1);
}

#[tokio::test]
async fn transcript_query_filters_by_session_tracks_counts_and_honors_limit() {
    let store = InMemoryStore::default();
    let matching_a = transcript_entry(
        "session-1",
        1,
        TranscriptEntryKind::Message,
        "approval requested",
    );
    let matching_b = transcript_entry(
        "session-1",
        2,
        TranscriptEntryKind::Approval,
        "approval granted",
    );
    let other_session = transcript_entry(
        "session-2",
        1,
        TranscriptEntryKind::Message,
        "approval elsewhere",
    );

    store
        .append(matching_a.clone())
        .await
        .expect("append should succeed");
    store
        .append(matching_b.clone())
        .await
        .expect("append should succeed");
    store
        .append(other_session)
        .await
        .expect("append should succeed");

    let report = store
        .query(TranscriptQuery {
            session_id: Some(SessionId("session-1".into())),
            text: "approval".into(),
            limit: 1,
        })
        .await
        .expect("query should succeed");

    assert_eq!(report.query.session_id, Some(SessionId("session-1".into())));
    assert_eq!(report.matched_count, 2);
    assert_eq!(report.returned_count, 1);
    assert!(report.truncated);
    assert_eq!(report.hits.len(), 1);
    assert_eq!(report.hits[0], TranscriptSpan::from_entry(matching_b));
}

#[tokio::test]
async fn transcript_search_report_returns_single_entry_spans() {
    let store = InMemoryStore::default();
    let transcript = transcript_entry(
        "session-1",
        7,
        TranscriptEntryKind::ToolResult,
        "manifest export complete",
    );

    store
        .append(transcript.clone())
        .await
        .expect("append should succeed");

    let report = store
        .transcript_search_report(TranscriptQuery {
            session_id: None,
            text: "manifest".into(),
            limit: 5,
        })
        .expect("search report should succeed");

    assert_eq!(report.matched_count, 1);
    assert_eq!(report.returned_count, 1);
    assert!(!report.truncated);
    assert_eq!(report.hits, vec![TranscriptSpan::from_entry(transcript)]);
}

#[tokio::test]
async fn transcript_search_coverage_and_limit_pressure_match_report_helpers() {
    let store = InMemoryStore::default();
    let matching_a = transcript_entry(
        "session-1",
        1,
        TranscriptEntryKind::Message,
        "approval requested",
    );
    let matching_b = transcript_entry(
        "session-1",
        2,
        TranscriptEntryKind::Approval,
        "approval granted",
    );
    let query = TranscriptQuery {
        session_id: Some(SessionId("session-1".into())),
        text: "approval".into(),
        limit: 1,
    };

    store
        .append(matching_a.clone())
        .await
        .expect("append should succeed");
    store
        .append(matching_b.clone())
        .await
        .expect("append should succeed");

    let snapshot = store.snapshot().expect("snapshot should load");
    let report = store
        .transcript_search_report(query.clone())
        .expect("transcript search report should succeed");

    assert_eq!(snapshot.transcript_search_report(&query), report);
    assert_eq!(
        snapshot.transcript_search_coverage(&query),
        QueryReportCoverage {
            returned_count: 1,
            matched_count: 2,
        }
    );
    assert_eq!(
        store
            .transcript_search_coverage(query.clone())
            .expect("transcript search coverage should succeed"),
        report.coverage()
    );
    assert_eq!(
        snapshot.transcript_search_limit_pressure(&query),
        QueryReportLimitPressure {
            truncated: true,
            omitted_count: 1,
        }
    );
    assert_eq!(
        store
            .transcript_search_limit_pressure(query)
            .expect("transcript search limit pressure should succeed"),
        report.limit_pressure()
    );
}

#[tokio::test]
async fn transcript_search_report_with_zero_limit_preserves_match_counts_and_omissions() {
    let store = InMemoryStore::default();
    let query = TranscriptQuery {
        session_id: Some(SessionId("session-1".into())),
        text: "approval".into(),
        limit: 0,
    };

    store
        .append(transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "approval requested",
        ))
        .await
        .expect("append should succeed");
    store
        .append(transcript_entry(
            "session-1",
            2,
            TranscriptEntryKind::Approval,
            "approval granted",
        ))
        .await
        .expect("append should succeed");

    let report = store
        .transcript_search_report(query.clone())
        .expect("transcript search report should succeed");

    assert_eq!(report.query, query);
    assert_eq!(report.matched_count, 2);
    assert_eq!(report.returned_count, 0);
    assert!(report.truncated);
    assert!(report.hits.is_empty());
    assert_eq!(report.omitted_count(), 2);
    assert!(!report.is_complete());
    assert_eq!(
        report.coverage(),
        QueryReportCoverage {
            returned_count: 0,
            matched_count: 2,
        }
    );
    assert_eq!(
        report.limit_pressure(),
        QueryReportLimitPressure {
            truncated: true,
            omitted_count: 2,
        }
    );
}
