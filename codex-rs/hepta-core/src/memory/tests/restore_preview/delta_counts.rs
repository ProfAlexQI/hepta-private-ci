use super::*;

#[test]
fn restore_delta_counts_roundtrip_through_json() {
    let counts = RestoreDeltaCounts {
        added_count: 2,
        removed_count: 1,
        updated_count: 3,
        unchanged_count: 4,
    };

    let json = serde_json::to_string(&counts).expect("restore delta counts should serialize");
    let parsed: RestoreDeltaCounts =
        serde_json::from_str(&json).expect("restore delta counts should deserialize");

    assert_eq!(parsed, counts);
    assert_eq!(parsed.change_count(), 6);
    assert!(!parsed.is_empty());
}

#[test]
fn restore_delta_counts_deserialize_from_sparse_json() {
    let parsed: RestoreDeltaCounts = serde_json::from_str("{}")
        .expect("sparse restore delta counts should deserialize with defaults");

    assert_eq!(parsed, RestoreDeltaCounts::default());
    assert_eq!(parsed.change_count(), 0);
    assert!(parsed.is_empty());
}
