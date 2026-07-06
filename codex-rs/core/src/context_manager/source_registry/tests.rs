use codex_protocol::protocol::TurnContextCompressionStageKind;
use codex_protocol::protocol::TurnContextTier;

use super::catalog::OWNER_LANE_HEPTA_CONTEXT;
use super::context_source_registry_entries;
use super::health::context_source_registry_health_report;
use super::source_aware_compression_kind;
use super::source_aware_omit_priority;

const REGISTRY_TSV: &str = include_str!("../../../../CONTEXT_SOURCE_REGISTRY.tsv");

#[test]
fn context_source_registry_entries_are_sorted_and_complete() {
    assert_eq!(context_source_registry_entries().len(), 19);

    let mut previous_source_id = "";
    for entry in context_source_registry_entries() {
        assert!(
            previous_source_id < entry.source_id,
            "registry entries must be sorted by source_id: {previous_source_id} before {}",
            entry.source_id
        );
        assert_eq!(entry.owner_lane, OWNER_LANE_HEPTA_CONTEXT);
        assert_ne!(entry.tier, TurnContextTier::Unknown);
        previous_source_id = entry.source_id;
    }
}

#[test]
fn context_source_registry_matches_catalog_tsv() {
    let normalized_tsv = REGISTRY_TSV
        .lines()
        .filter(|line| !line.starts_with('#') && !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    let registry_rows = std::iter::once(
        "source_id\ttier\towner_lane\tprivacy_class\tbudget_class\tttl\tvolatility\ttrust_class\tredaction_policy\tquality_metric\tactivation_guard\trollback_policy\tomit_priority\tallowed_compression_actions"
            .to_string(),
    )
    .chain(
        context_source_registry_entries()
            .iter()
            .map(|entry| entry.as_tsv_row()),
    )
    .collect::<Vec<_>>()
    .join("\n");

    assert_eq!(registry_rows, normalized_tsv);
}

#[test]
fn context_source_registry_exposes_source_aware_budget_policy() {
    assert_eq!(
        source_aware_omit_priority(TurnContextTier::Tool, "extension_developer_capabilities"),
        Some(10)
    );
    assert_eq!(
        source_aware_omit_priority(TurnContextTier::Tool, "available_plugins"),
        Some(20)
    );
    assert_eq!(
        source_aware_omit_priority(TurnContextTier::Tool, "apps"),
        Some(30)
    );
    assert_eq!(
        source_aware_omit_priority(TurnContextTier::Tool, "available_skills"),
        Some(40)
    );
    assert_eq!(
        source_aware_omit_priority(
            TurnContextTier::RetrievedSnippets,
            "selected_context_recall"
        ),
        Some(50)
    );
    assert_eq!(
        source_aware_omit_priority(TurnContextTier::Developer, "available_plugins"),
        None
    );
    assert_eq!(
        source_aware_compression_kind(
            TurnContextTier::RetrievedSnippets,
            "selected_context_recall"
        ),
        Some(TurnContextCompressionStageKind::Summary)
    );
    assert_eq!(
        source_aware_compression_kind(TurnContextTier::Tool, "extension_developer_capabilities"),
        Some(TurnContextCompressionStageKind::Prune)
    );
    assert_eq!(
        source_aware_compression_kind(TurnContextTier::Tool, "available_plugins"),
        Some(TurnContextCompressionStageKind::Defragment)
    );
    assert_eq!(
        source_aware_compression_kind(TurnContextTier::Developer, "available_plugins"),
        None
    );
}

#[test]
fn context_source_registry_health_report_is_payload_light_and_non_activating() {
    let report = context_source_registry_health_report();

    assert_eq!(report.source_count, 19);
    assert_eq!(report.descriptor_field_count, 14);
    assert_eq!(report.turn_ttl_count, 14);
    assert_eq!(report.session_ttl_count, 5);
    assert_eq!(report.prompt_hash_only_count, 13);
    assert_eq!(report.guarded_envelope_count, 1);
    assert_eq!(report.metadata_only_count, 5);
    assert_eq!(report.compression_candidate_count, 5);
    assert_eq!(report.operator_approval_required_count, 1);
    assert_eq!(report.live_activation_route_count, 0);
    assert_eq!(report.runtime_activation, "disabled");
}
