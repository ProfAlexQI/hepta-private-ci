use codex_protocol::protocol::TurnContextCompressionStageKind;
use codex_protocol::protocol::TurnContextTier;

use super::catalog::OWNER_LANE_HEPTA_CONTEXT;
use super::context_source_registry_entries;
use super::entry::ContextSourcePrivacyClass;
use super::health::context_source_registry_health_report;
use super::source_aware_compression_kind;
use super::source_aware_omit_priority;

const REGISTRY_TSV: &str = include_str!("../../../../CONTEXT_SOURCE_REGISTRY.tsv");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SettingsDiffCoverageKind {
    ManifestHashDiff,
    ManifestHashDiffWithClear,
    LiveTurnItem,
    TurnScopedNoSteadyStateDiff,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SettingsDiffCoverage {
    source_id: &'static str,
    kind: SettingsDiffCoverageKind,
    reason: &'static str,
}

const SETTINGS_DIFF_COVERAGE: &[SettingsDiffCoverage] = &[
    SettingsDiffCoverage {
        source_id: "apps",
        kind: SettingsDiffCoverageKind::ManifestHashDiffWithClear,
        reason: "capability inventory is rebuilt from the current connector set and clears stale manifests",
    },
    SettingsDiffCoverage {
        source_id: "available_plugins",
        kind: SettingsDiffCoverageKind::ManifestHashDiffWithClear,
        reason: "plugin inventory is rebuilt from the current plugin manager view and clears stale manifests",
    },
    SettingsDiffCoverage {
        source_id: "available_skills",
        kind: SettingsDiffCoverageKind::ManifestHashDiffWithClear,
        reason: "skill inventory is rebuilt from the current turn skill outcome and clears stale manifests",
    },
    SettingsDiffCoverage {
        source_id: "collaboration_mode",
        kind: SettingsDiffCoverageKind::ManifestHashDiffWithClear,
        reason: "collaboration-mode guidance is session policy and must override or clear previous guidance",
    },
    SettingsDiffCoverage {
        source_id: "context",
        kind: SettingsDiffCoverageKind::TurnScopedNoSteadyStateDiff,
        reason: "fallback context fragments are turn-scoped raw context, not durable session settings",
    },
    SettingsDiffCoverage {
        source_id: "contextual_user",
        kind: SettingsDiffCoverageKind::TurnScopedNoSteadyStateDiff,
        reason: "fallback contextual-user fragments are turn-scoped raw context, not durable session settings",
    },
    SettingsDiffCoverage {
        source_id: "developer_instructions",
        kind: SettingsDiffCoverageKind::ManifestHashDiffWithClear,
        reason: "developer instructions are protected session policy and must override or clear stale guidance",
    },
    SettingsDiffCoverage {
        source_id: "environment",
        kind: SettingsDiffCoverageKind::ManifestHashDiff,
        reason: "environment context is rebuilt from current runtime state when enabled; disabled configs intentionally omit it",
    },
    SettingsDiffCoverage {
        source_id: "extension_contextual_user",
        kind: SettingsDiffCoverageKind::ManifestHashDiffWithClear,
        reason: "extension contextual-user prompt fragments are hash-diffed per extension slot and clear stale fragments",
    },
    SettingsDiffCoverage {
        source_id: "extension_developer_capabilities",
        kind: SettingsDiffCoverageKind::ManifestHashDiffWithClear,
        reason: "extension capability prompt fragments are hash-diffed per extension slot and clear stale fragments",
    },
    SettingsDiffCoverage {
        source_id: "extension_developer_policy",
        kind: SettingsDiffCoverageKind::ManifestHashDiffWithClear,
        reason: "extension developer-policy prompt fragments are hash-diffed per extension slot and clear stale fragments",
    },
    SettingsDiffCoverage {
        source_id: "extension_separate_developer",
        kind: SettingsDiffCoverageKind::ManifestHashDiffWithClear,
        reason: "separate extension developer prompt fragments are hash-diffed per extension slot and clear stale fragments",
    },
    SettingsDiffCoverage {
        source_id: "model_switch",
        kind: SettingsDiffCoverageKind::ManifestHashDiffWithClear,
        reason: "model-specific guidance is hash-diffed and clears when the current model has no extra guidance",
    },
    SettingsDiffCoverage {
        source_id: "multi_agent_usage_hint",
        kind: SettingsDiffCoverageKind::ManifestHashDiffWithClear,
        reason: "multi-agent usage hints are durable developer guidance and clear once absent from the current source",
    },
    SettingsDiffCoverage {
        source_id: "non_text_content",
        kind: SettingsDiffCoverageKind::TurnScopedNoSteadyStateDiff,
        reason: "non-text user content is carried by user input or raw turn items, not a steady-state settings diff",
    },
    SettingsDiffCoverage {
        source_id: "permissions",
        kind: SettingsDiffCoverageKind::ManifestHashDiff,
        reason: "permission guidance is rebuilt from current permission and exec policy state when enabled",
    },
    SettingsDiffCoverage {
        source_id: "personality",
        kind: SettingsDiffCoverageKind::ManifestHashDiff,
        reason: "personality guidance is hash-diffed when the personality feature and model guidance are active",
    },
    SettingsDiffCoverage {
        source_id: "realtime",
        kind: SettingsDiffCoverageKind::ManifestHashDiffWithClear,
        reason: "realtime state starts, refreshes, or ends from the durable turn-context baseline",
    },
    SettingsDiffCoverage {
        source_id: "selected_context_recall",
        kind: SettingsDiffCoverageKind::LiveTurnItem,
        reason: "selected recall snippets are guarded turn handoff items with history de-duplication, not session settings",
    },
    SettingsDiffCoverage {
        source_id: "user_instructions",
        kind: SettingsDiffCoverageKind::ManifestHashDiffWithClear,
        reason: "workspace/user instructions are protected user policy and must override or clear stale guidance",
    },
];

#[test]
fn context_source_registry_entries_are_sorted_and_complete() {
    assert_eq!(context_source_registry_entries().len(), 20);

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
fn model_visible_context_sources_have_explicit_settings_diff_coverage() {
    let registry_source_ids = context_source_registry_entries()
        .iter()
        .filter(|entry| {
            matches!(
                entry.privacy_class,
                ContextSourcePrivacyClass::PromptVisible
                    | ContextSourcePrivacyClass::BoundedRecallPayload
            )
        })
        .map(|entry| entry.source_id)
        .collect::<Vec<_>>();
    let coverage_source_ids = SETTINGS_DIFF_COVERAGE
        .iter()
        .map(|coverage| coverage.source_id)
        .collect::<Vec<_>>();

    assert_eq!(coverage_source_ids, registry_source_ids);

    let mut previous_source_id = "";
    for coverage in SETTINGS_DIFF_COVERAGE {
        assert!(
            previous_source_id < coverage.source_id,
            "coverage entries must be sorted by source_id: {previous_source_id} before {}",
            coverage.source_id
        );
        assert!(
            !coverage.reason.trim().is_empty(),
            "coverage entry {} must document its diff/no-diff reason",
            coverage.source_id
        );
        previous_source_id = coverage.source_id;
    }

    let session_state_diff_sources = SETTINGS_DIFF_COVERAGE
        .iter()
        .filter(|coverage| {
            matches!(
                coverage.kind,
                SettingsDiffCoverageKind::ManifestHashDiff
                    | SettingsDiffCoverageKind::ManifestHashDiffWithClear
            )
        })
        .map(|coverage| coverage.source_id)
        .collect::<Vec<_>>();
    assert_eq!(
        session_state_diff_sources,
        vec![
            "apps",
            "available_plugins",
            "available_skills",
            "collaboration_mode",
            "developer_instructions",
            "environment",
            "extension_contextual_user",
            "extension_developer_capabilities",
            "extension_developer_policy",
            "extension_separate_developer",
            "model_switch",
            "multi_agent_usage_hint",
            "permissions",
            "personality",
            "realtime",
            "user_instructions",
        ]
    );
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

    assert_eq!(report.source_count, 20);
    assert_eq!(report.descriptor_field_count, 14);
    assert_eq!(report.turn_ttl_count, 14);
    assert_eq!(report.session_ttl_count, 6);
    assert_eq!(report.prompt_hash_only_count, 14);
    assert_eq!(report.guarded_envelope_count, 1);
    assert_eq!(report.metadata_only_count, 5);
    assert_eq!(report.compression_candidate_count, 5);
    assert_eq!(report.operator_approval_required_count, 1);
    assert_eq!(report.live_activation_route_count, 0);
    assert_eq!(report.runtime_activation, "disabled");
}
