use super::*;

#[test]
fn context_recall_omission_counts_roundtrip_through_json() {
    let counts = ContextRecallOmissionCounts {
        recent_entry_count: 2,
        transcript_hit_count: 1,
        memory_hit_count: 3,
        query_hit_count: 4,
        total_item_count: 6,
    };

    let json = serde_json::to_string(&counts).expect("omission counts should serialize");
    let parsed: ContextRecallOmissionCounts =
        serde_json::from_str(&json).expect("omission counts should deserialize");

    assert_eq!(parsed, counts);
    assert!(parsed.has_omissions());
    assert!(!parsed.is_empty());
}

#[test]
fn context_recall_omission_counts_deserialize_from_sparse_json() {
    let parsed: ContextRecallOmissionCounts = serde_json::from_str("{}")
        .expect("sparse omission counts should deserialize with defaults");

    assert_eq!(parsed, ContextRecallOmissionCounts::default());
    assert!(!parsed.has_omissions());
    assert!(parsed.is_empty());
}
