use serde::Serialize;

pub const CURRENT_REALITY_CAPABILITY_CATALOG_SCHEMA_VERSION: &str =
    "current_reality_capability_catalog_v1";
pub const CURRENT_REALITY_CAPABILITY_CATALOG_ID_SHA256: &str =
    "aa9a8ffc24bd806bc1c5205e0a32c7f1008a49e7401d4d32718161cbf0161df4";

pub const CURRENT_REALITY_WORK_GRAPH_DURABLE_IDENTITY_ALIAS: &str =
    "workflow_workgraph_durable_identity";
pub const CURRENT_REALITY_WORK_GRAPH_DURABLE_IDENTITY_SUCCESSOR: &str =
    "hepta-systems-work-graph-current-state-inventory";
pub const CURRENT_REALITY_WORK_GRAPH_RECEIPT_TAIL_ALIAS: &str =
    "workflow_current_readback_receipt_tail";
pub const CURRENT_REALITY_WORK_GRAPH_RECEIPT_TAIL_SUCCESSOR: &str = "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-preview";

pub const CURRENT_REALITY_CAPABILITY_IDS: [&str; 104] = [
    "plugins_contribution_point_abi",
    "plugins_loader_binding_fixture",
    "plugins_tool_contribution_inventory",
    "plugins_lifecycle_state_machine",
    "tools_invocation_source_of_truth",
    "tools_read_only_dispatch_preflight",
    "workflow_workgraph_durable_identity",
    "workflow_current_readback_receipt_tail",
    "workflow_temporal_lite_durable_store_adapter",
    "workflow_durable_store_test_only_append_fixture",
    "workflow_temporal_lite_append_only_event_store_test_implementation",
    "workflow_temporal_lite_append_only_event_store_minimal_local_persistence",
    "workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback",
    "workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback",
    "workflow_temporal_lite_lease_idempotency_index_local_persistence_readback",
    "workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback",
    "workflow_temporal_lite_work_graph_projection_local_persistence_readback",
    "workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback",
    "workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback",
    "workflow_temporal_lite_replay_alignment_rollback_consistency_local_persistence_readback",
    "workflow_temporal_lite_replay_alignment_recovery_window_local_persistence_readback",
    "workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback",
    "workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback",
    "workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback",
    "workflow_temporal_lite_lease_idempotency_index_feature_gated_readback",
    "workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback",
    "workflow_temporal_lite_work_graph_projection_feature_gated_readback",
    "workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback",
    "workflow_temporal_lite_replay_alignment_checkpoint_consistency_feature_gated_readback",
    "workflow_temporal_lite_replay_alignment_rollback_consistency_feature_gated_readback",
    "workflow_temporal_lite_replay_alignment_recovery_window_feature_gated_readback",
    "workflow_temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback",
    "hepta_systems_gate_recursion_cost_boundary_readback",
    "hepta_systems_gate_recursion_lean_contract_readback",
    "hepta_systems_workgraph_legacy_gate_recursion_inventory_readback",
    "hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback",
    "hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback",
    "hepta_systems_plugin_signature_trust_install_cache_boundary_readback",
    "hepta_systems_plugin_operator_evidence_acceptance_packet_readback",
    "hepta_systems_plugin_install_cache_noop_preflight_readback",
    "hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback",
    "hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback",
    "hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback",
    "hepta_systems_plugin_tool_registry_registration_denial_receipt_readback",
    "hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback",
    "hepta_systems_plugin_tool_invocation_policy_approval_ledger_boundary_readback",
    "hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback",
    "hepta_systems_plugin_tool_invocation_dry_run_receipt_ledger_preview_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_packet_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_execution_open_preconditions_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_boundary_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback",
    "hepta_systems_tool_registry_shadow_registration_lookup_readback",
    "hepta_systems_matrix_report_single_render_cache_boundary_readback",
    "current_reality_matrix_compact_cache_boundary_readback",
    "dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_without_git_mutation",
    "dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_without_git_mutation",
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send",
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation",
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_without_git_mutation",
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_without_git_mutation",
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_without_recording",
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_approval_acceptance_boundary_readback_without_acceptance",
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording",
    "hepta_system_status_read_only_e2e",
    "hepta_system_status_internal_read_only_invocation",
    "hepta_system_status_operator_approval_protocol",
    "controlled_canary_readiness_plan",
    "dirty_worktree_release_boundary_inventory",
    "dirty_worktree_release_boundary_grouping_freeze_plan",
    "dirty_worktree_release_boundary_grouping_freeze_operator_readback",
    "dirty_worktree_release_boundary_actionable_clean_worktree_strategy",
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet",
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback",
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback",
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist",
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback",
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback",
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback",
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback",
    "dirty_worktree_release_boundary_release_risk_snapshot",
    "dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal",
    "dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback",
    "controlled_live_readiness_audit",
    "controlled_live_readiness_denial_readback_index",
    "controlled_live_operator_packet_preview",
    "controlled_live_operator_packet_non_send_readback",
    "controlled_live_required_evidence_collection_plan",
    "controlled_live_required_evidence_readback_index",
    "controlled_live_required_evidence_gap_summary",
    "controlled_live_required_evidence_gap_diff_view",
    "controlled_live_required_evidence_gap_operator_readback",
    "controlled_live_required_evidence_gap_operator_packet_attachment",
    "controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback",
    "controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback",
    "controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary_readback",
    "controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback",
    "controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback",
    "current_compact_capability_summary",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentRealityCapabilityLayer {
    Plugin,
    Tooling,
    Workflow,
    Systems,
    Worktree,
    SystemStatus,
    ControlledLive,
    CurrentSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CurrentRealityCapabilitySource {
    TypedObservation { observation_id: &'static str },
    CompatibilityAlias { successor_report_id: &'static str },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CurrentRealityCapabilityDescriptor {
    pub id: &'static str,
    pub layer: CurrentRealityCapabilityLayer,
    pub source: CurrentRealityCapabilitySource,
}

pub fn current_reality_capability_catalog() -> Vec<CurrentRealityCapabilityDescriptor> {
    CURRENT_REALITY_CAPABILITY_IDS
        .iter()
        .copied()
        .map(|id| CurrentRealityCapabilityDescriptor {
            id,
            layer: capability_layer(id),
            source: capability_source(id),
        })
        .collect()
}

/// Count the stable rows in the typed current-reality capability catalog.
pub const fn current_reality_capability_registry_count() -> usize {
    CURRENT_REALITY_CAPABILITY_IDS.len()
}

fn capability_layer(id: &'static str) -> CurrentRealityCapabilityLayer {
    if id.starts_with("plugins_") || id.starts_with("hepta_systems_plugin_") {
        CurrentRealityCapabilityLayer::Plugin
    } else if id.starts_with("tools_") || id.starts_with("hepta_systems_tool_") {
        CurrentRealityCapabilityLayer::Tooling
    } else if id.starts_with("workflow_") {
        CurrentRealityCapabilityLayer::Workflow
    } else if id.starts_with("dirty_worktree_") {
        CurrentRealityCapabilityLayer::Worktree
    } else if id.starts_with("hepta_system_status_") {
        CurrentRealityCapabilityLayer::SystemStatus
    } else if id.starts_with("controlled_") {
        CurrentRealityCapabilityLayer::ControlledLive
    } else if id.starts_with("current_") {
        CurrentRealityCapabilityLayer::CurrentSummary
    } else {
        CurrentRealityCapabilityLayer::Systems
    }
}

fn capability_source(id: &'static str) -> CurrentRealityCapabilitySource {
    match id {
        CURRENT_REALITY_WORK_GRAPH_DURABLE_IDENTITY_ALIAS => {
            CurrentRealityCapabilitySource::CompatibilityAlias {
                successor_report_id: CURRENT_REALITY_WORK_GRAPH_DURABLE_IDENTITY_SUCCESSOR,
            }
        }
        CURRENT_REALITY_WORK_GRAPH_RECEIPT_TAIL_ALIAS => {
            CurrentRealityCapabilitySource::CompatibilityAlias {
                successor_report_id: CURRENT_REALITY_WORK_GRAPH_RECEIPT_TAIL_SUCCESSOR,
            }
        }
        _ => CurrentRealityCapabilitySource::TypedObservation { observation_id: id },
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn typed_catalog_preserves_the_stable_104_id_contract() {
        let catalog = current_reality_capability_catalog();
        let unique = catalog.iter().map(|row| row.id).collect::<BTreeSet<_>>();

        assert_eq!(catalog.len(), 104);
        assert_eq!(unique.len(), 104);
        assert_eq!(current_reality_capability_registry_count(), 104);
        assert_eq!(catalog[0].id, "plugins_contribution_point_abi");
        assert_eq!(catalog[103].id, "current_compact_capability_summary");
        assert_eq!(
            CURRENT_REALITY_CAPABILITY_CATALOG_ID_SHA256,
            "aa9a8ffc24bd806bc1c5205e0a32c7f1008a49e7401d4d32718161cbf0161df4"
        );
    }

    #[test]
    fn retired_work_graph_rows_are_aliases_to_current_typed_successors() {
        let catalog = current_reality_capability_catalog();
        let durable = catalog
            .iter()
            .find(|row| row.id == CURRENT_REALITY_WORK_GRAPH_DURABLE_IDENTITY_ALIAS)
            .expect("durable identity alias");
        let receipt = catalog
            .iter()
            .find(|row| row.id == CURRENT_REALITY_WORK_GRAPH_RECEIPT_TAIL_ALIAS)
            .expect("receipt tail alias");

        assert_eq!(
            durable.source,
            CurrentRealityCapabilitySource::CompatibilityAlias {
                successor_report_id: CURRENT_REALITY_WORK_GRAPH_DURABLE_IDENTITY_SUCCESSOR,
            }
        );
        assert_eq!(
            receipt.source,
            CurrentRealityCapabilitySource::CompatibilityAlias {
                successor_report_id: CURRENT_REALITY_WORK_GRAPH_RECEIPT_TAIL_SUCCESSOR,
            }
        );
    }
}
