use hepta_core::*;

#[test]
fn memory_query_roundtrips_through_json() {
    let query = MemoryQuery {
        text: "doctor snapshot".into(),
        limit: 5,
    };

    let json = serde_json::to_string(&query).expect("memory query should serialize");
    let parsed: MemoryQuery = serde_json::from_str(&json).expect("memory query should deserialize");

    assert_eq!(parsed, query);
}

#[test]
fn memory_query_report_tracks_counts_and_truncation() {
    let report = MemoryQueryReport::from_hits(
        MemoryQuery {
            text: "snapshot".into(),
            limit: 1,
        },
        2,
        vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::Session,
            content: "snapshot ok".into(),
        }],
    );

    assert_eq!(report.query.text, "snapshot");
    assert_eq!(report.query.limit, 1);
    assert_eq!(report.matched_count, 2);
    assert_eq!(report.returned_count, 1);
    assert!(report.truncated);
    assert_eq!(report.hits.len(), 1);
    assert!(!report.is_empty());
}

#[test]
fn query_report_coverage_roundtrips_through_json() {
    let coverage = QueryReportCoverage {
        returned_count: 1,
        matched_count: 3,
    };

    let json = serde_json::to_string(&coverage).expect("coverage should serialize");
    let parsed: QueryReportCoverage =
        serde_json::from_str(&json).expect("coverage should deserialize");

    assert_eq!(parsed, coverage);
    assert_eq!(parsed.omitted_count(), 2);
    assert!(parsed.is_truncated());
    assert!(!parsed.is_complete());
    assert!(!parsed.is_empty());
}

#[test]
fn query_report_limit_pressure_deserializes_from_sparse_json() {
    let parsed: QueryReportLimitPressure =
        serde_json::from_str("{}").expect("sparse limit pressure should deserialize with defaults");

    assert_eq!(parsed, QueryReportLimitPressure::default());
    assert!(parsed.is_complete());
    assert!(parsed.is_empty());
}

#[test]
fn memory_query_report_exposes_coverage_and_limit_pressure() {
    let report = MemoryQueryReport::from_hits(
        MemoryQuery {
            text: "snapshot".into(),
            limit: 1,
        },
        3,
        vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::Session,
            content: "snapshot ok".into(),
        }],
    );

    assert!(report.has_hits());
    assert_eq!(report.omitted_count(), 2);
    assert!(!report.is_complete());
    assert_eq!(
        report.coverage(),
        QueryReportCoverage {
            returned_count: 1,
            matched_count: 3,
        }
    );
    assert_eq!(
        report.limit_pressure(),
        QueryReportLimitPressure {
            truncated: true,
            omitted_count: 2,
        }
    );
    assert_eq!(report.omitted_control_count, 0);
    assert!(!report.limit_pressure().is_complete());
    assert!(!report.limit_pressure().is_empty());
}

#[test]
fn memory_query_report_roundtrips_through_json() {
    let report = MemoryQueryReport::from_hits(
        MemoryQuery {
            text: "contract".into(),
            limit: 2,
        },
        1,
        vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "contract ready".into(),
        }],
    );

    let json = serde_json::to_string(&report).expect("memory query report should serialize");
    let parsed: MemoryQueryReport =
        serde_json::from_str(&json).expect("memory query report should deserialize");

    assert_eq!(parsed, report);
}

#[test]
fn memory_query_report_surfaces_control_omission_count_without_hits() {
    let report = MemoryQueryReport::from_hits_with_omitted_control_count(
        MemoryQuery {
            text: "snapshot".into(),
            limit: 1,
        },
        1,
        vec![MemoryRecord {
            id: "memory-1".into(),
            scope: MemoryScope::LongTerm,
            content: "snapshot ok".into(),
        }],
        2,
    );

    let json = serde_json::to_string(&report).expect("report should serialize");
    let parsed: MemoryQueryReport = serde_json::from_str(&json).expect("report should deserialize");

    assert_eq!(parsed, report);
    assert_eq!(report.omitted_control_count, 2);
    assert!(json.contains("omitted_control_count"));
}

#[test]
fn query_report_limit_pressure_from_coverage_tracks_completion_state() {
    let truncated = QueryReportLimitPressure::from_coverage(&QueryReportCoverage {
        returned_count: 1,
        matched_count: 3,
    });
    let complete = QueryReportLimitPressure::from_coverage(&QueryReportCoverage {
        returned_count: 2,
        matched_count: 2,
    });

    assert_eq!(
        truncated,
        QueryReportLimitPressure {
            truncated: true,
            omitted_count: 2,
        }
    );
    assert!(!truncated.is_complete());
    assert!(!truncated.is_empty());

    assert_eq!(complete, QueryReportLimitPressure::default());
    assert!(complete.is_complete());
    assert!(complete.is_empty());
}
