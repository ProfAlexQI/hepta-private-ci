#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-current-reality-capability-matrix-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CURRENT_REALITY_CAPABILITY_MATRIX_2026-06-27.md"

fail() {
  printf 'hepta-systems-current-reality-capability-matrix-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable current reality capability matrix report: $REPORT"
[[ -f "$DOC" ]] || fail "missing current reality capability matrix architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the current reality capability matrix report"
fi

grep -q 'Current Reality Capability Matrix' "$DOC" \
  || fail "architecture note must document Current Reality Capability Matrix"
grep -q 'memory/filesystem drift' "$DOC" \
  || fail "architecture note must document memory/filesystem drift"
grep -q 'does not open live execution' "$DOC" \
  || fail "architecture note must document no live execution"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "current_reality_capability_matrix"
  and .status == "ready"
  and .matrix_date == "2026-06-27"
  and .local_capability_count == 104
  and .local_capability_ready_count == 104
  and .local_capability_blocked_count == 0
  and .live_enabled_count == 0
  and .all_live_paths_blocked == true
  and .plugin_fixture_shape_ready == true
  and .plugin_manifest_present == true
  and .plugin_manifest_summary.skill_path_present == true
  and .plugin_manifest_summary.mcp_servers_path_present == true
  and .plugin_manifest_summary.apps_path_present == true
  and .plugin_manifest_summary.skill_count == 1
  and .plugin_manifest_summary.mcp_server_count == 1
  and .plugin_manifest_summary.app_count == 1
  and .plugin_manifest_summary.tool_schema_count == 2
  and .plugin_manifest_summary.permission_count == 2
  and .plugin_manifest_summary.activation_event_count == 2
  and .plugin_manifest_summary.tool_policy_count == 2
  and .memory_drift_entry_count == 5
  and .missing_memory_checkpoint_count == 0
  and .resolved_memory_checkpoint_count == 5
  and .memory_filesystem_drift_tracked == true
  and .dirty_worktree_boundary_tracked == true
  and .git_status_entry_count > 0
  and (.capabilities | length) == 104
  and (.capabilities | all(.ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "plugins_contribution_point_abi" and .ready == true))
  and (.capabilities | any(.id == "plugins_lifecycle_state_machine" and .ready == true))
  and (.capabilities | any(.id == "tools_invocation_source_of_truth" and .ready == true))
  and (.capabilities | any(.id == "tools_read_only_dispatch_preflight" and .ready == true))
  and (.capabilities | any(.id == "workflow_workgraph_durable_identity" and .ready == true))
  and (.capabilities | any(.id == "workflow_current_readback_receipt_tail" and .ready == true))
  and (.capabilities | any(.id == "workflow_temporal_lite_durable_store_adapter" and .ready == true))
  and (.capabilities | any(.id == "workflow_durable_store_test_only_append_fixture" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_append_only_event_store_test_implementation" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_append_only_event_store_minimal_local_persistence" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_lease_idempotency_index_local_persistence_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_work_graph_projection_local_persistence_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_replay_alignment_rollback_consistency_local_persistence_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_replay_alignment_recovery_window_local_persistence_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_lease_idempotency_index_feature_gated_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_work_graph_projection_feature_gated_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_replay_alignment_checkpoint_consistency_feature_gated_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_replay_alignment_rollback_consistency_feature_gated_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_replay_alignment_recovery_window_feature_gated_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "workflow_temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_gate_recursion_cost_boundary_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_gate_recursion_lean_contract_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_workgraph_legacy_gate_recursion_inventory_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_tool_registry_shadow_registration_lookup_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_signature_trust_install_cache_boundary_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_operator_evidence_acceptance_packet_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_install_cache_noop_preflight_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_tool_registry_registration_denial_receipt_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_tool_invocation_policy_approval_ledger_boundary_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_tool_invocation_dry_run_receipt_ledger_preview_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_packet_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_execution_open_preconditions_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_boundary_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_systems_matrix_report_single_render_cache_boundary_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "current_reality_matrix_compact_cache_boundary_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_without_git_mutation" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_without_git_mutation" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_without_git_mutation" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_without_git_mutation" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_without_recording" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_owner_freeze_classification_operator_approval_acceptance_boundary_readback_without_acceptance" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_system_status_read_only_e2e" and .ready == true))
  and (.capabilities | any(.id == "hepta_system_status_internal_read_only_invocation" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "hepta_system_status_operator_approval_protocol" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "controlled_canary_readiness_plan" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_inventory" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_grouping_freeze_plan" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_grouping_freeze_operator_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_actionable_clean_worktree_strategy" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_release_risk_snapshot" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "controlled_live_readiness_audit" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "controlled_live_readiness_denial_readback_index" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "controlled_live_operator_packet_preview" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "controlled_live_operator_packet_non_send_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "controlled_live_required_evidence_collection_plan" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "controlled_live_required_evidence_readback_index" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "controlled_live_required_evidence_gap_summary" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "controlled_live_required_evidence_gap_diff_view" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "controlled_live_required_evidence_gap_operator_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "controlled_live_required_evidence_gap_operator_packet_attachment" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback" and .ready == true and .live_enabled == false))
  and (.capabilities | any(.id == "controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback" and .ready == true and .live_enabled == false))
  and (.memory_drift_entries | all(.expected_from_memory == true))
  and (.memory_drift_entries | any(.id == "memory_plugin_lifecycle_state_machine" and .present == true))
  and (.memory_drift_entries | any(.id == "memory_plugin_lifecycle_phase_summary" and .present == true))
  and (.memory_drift_entries | any(.id == "memory_workflow_durable_store_adapter_absent" and .present == true))
  and (.memory_drift_entries | any(.id == "memory_workflow_durable_store_append_plan_absent" and .present == true))
  and (.memory_drift_entries | any(.id == "memory_workflow_durable_store_harness_absent" and .present == true))
  and (.blockers | index("workgraph_suffix_ladder_accretion")) != null
  and (.blockers | index("controlled_live_cutover_blocked_by_operator_approval_and_evidence")) != null
  and (.blockers | index("controlled_live_denial_readback_index_blocks_waiver_and_acceptance")) != null
  and (.blockers | index("controlled_live_operator_packet_preview_blocks_approval_request")) != null
  and (.blockers | index("controlled_live_operator_packet_non_send_readback_blocks_send_and_persistence")) != null
  and (.blockers | index("controlled_live_required_evidence_plan_blocks_recording_and_acceptance")) != null
  and (.blockers | index("controlled_live_required_evidence_readback_index_blocks_recording_and_acceptance")) != null
  and (.blockers | index("controlled_live_required_evidence_gap_summary_blocks_acceptance_and_recording")) != null
  and (.blockers | index("controlled_live_required_evidence_gap_diff_view_blocks_acceptance_and_recording")) != null
  and (.blockers | index("controlled_live_required_evidence_gap_operator_readback_blocks_acceptance_and_persistence")) != null
  and (.blockers | index("controlled_live_required_evidence_gap_operator_packet_attachment_blocks_acceptance_send_and_persistence")) != null
  and (.blockers | index("controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_blocks_send_persistence_and_approval_request")) != null
  and (.blockers | index("controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback_blocks_transport_mutation")) != null
  and (.blockers | index("controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary_readback_blocks_credential_access")) != null
  and (.blockers | index("controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback_blocks_rehearsal_execution")) != null
  and (.blockers | index("controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback_blocks_kill_switch_mutation")) != null
  and (.blockers | index("workflow_durable_store_test_only_append_fixture_blocks_runtime_writes")) != null
  and (.blockers | index("workflow_temporal_lite_append_only_event_store_test_implementation_blocks_runtime_writes_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_append_only_event_store_minimal_local_persistence_blocks_runtime_event_log_sqlite_store_workflow_replay_rollback_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_blocks_runtime_replay_projection_persistence_replay_execution_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback_blocks_checkpoint_rollback_anchor_writes_persistence_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_lease_idempotency_index_local_persistence_readback_blocks_lease_acquire_idempotency_write_persistence_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_blocks_event_log_sqlite_adapter_writes_persistence_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_work_graph_projection_local_persistence_readback_blocks_projection_persistence_writes_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback_blocks_replay_alignment_execution_persistence_writes_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback_blocks_checkpoint_consistency_execution_persistence_writes_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_replay_alignment_rollback_consistency_local_persistence_readback_blocks_rollback_consistency_execution_persistence_writes_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_replay_alignment_recovery_window_local_persistence_readback_blocks_recovery_window_execution_persistence_writes_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_blocks_recovery_receipt_execution_persistence_writes_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_blocks_replay_execution_persistence_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_blocks_checkpoint_rollback_writes_persistence_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_blocks_lease_acquire_idempotency_write_persistence_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_blocks_event_log_sqlite_adapter_writes_persistence_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_work_graph_projection_feature_gated_readback_blocks_projection_persistence_writes_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback_blocks_replay_alignment_execution_persistence_writes_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_replay_alignment_checkpoint_consistency_feature_gated_readback_blocks_checkpoint_consistency_execution_persistence_writes_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_replay_alignment_rollback_consistency_feature_gated_readback_blocks_rollback_consistency_execution_persistence_writes_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_replay_alignment_recovery_window_feature_gated_readback_blocks_recovery_window_execution_persistence_writes_and_live")) != null
  and (.blockers | index("workflow_temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback_blocks_recovery_receipt_execution_persistence_writes_and_live")) != null
  and (.blockers | index("hepta_systems_gate_recursion_cost_boundary_readback_blocks_matrix_cache_source_semantic_changes_gate_chain_invocation_and_live")) != null
  and (.blockers | index("hepta_systems_gate_recursion_lean_contract_readback_blocks_recursive_source_gate_chain_cache_persistence_source_semantics_and_live")) != null
  and (.blockers | index("hepta_systems_workgraph_legacy_gate_recursion_inventory_readback_blocks_legacy_workgraph_recursive_gate_chains_source_semantics_and_live")) != null
  and (.blockers | index("hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback_blocks_tool_write_ledger_approval_receipt_persistence_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_blocks_plugin_install_cache_activation_signature_trust_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_signature_trust_install_cache_boundary_readback_blocks_signature_trust_install_cache_evidence_acceptance_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_operator_evidence_acceptance_packet_readback_blocks_packet_send_evidence_acceptance_install_cache_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_install_cache_noop_preflight_readback_blocks_preflight_execution_cache_materialization_install_receipt_persistence_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback_blocks_idempotency_index_write_denial_receipt_persistence_install_cache_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback_blocks_rollback_uninstall_execution_plan_persistence_install_cache_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback_blocks_activation_permission_connector_start_tool_registration_ledger_receipt_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_tool_registry_registration_denial_receipt_readback_blocks_tool_registry_registration_lookup_invocation_ledger_receipt_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback_blocks_tool_invocation_noop_result_ledger_approval_receipt_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_tool_invocation_policy_approval_ledger_boundary_readback_blocks_policy_approval_ledger_receipt_persistence_invocation_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_blocks_feature_gate_open_dry_run_execution_tool_invocation_ledger_receipt_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_tool_invocation_dry_run_receipt_ledger_preview_readback_blocks_feature_gate_open_dry_run_execution_receipt_ledger_persistence_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_packet_readback_blocks_operator_packet_send_persistence_acceptance_recording_tool_invocation_ledger_receipt_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback_blocks_acceptance_recording_receipt_persistence_tool_invocation_runtime_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_tool_invocation_read_only_status_dry_run_execution_open_preconditions_readback_blocks_operator_evidence_acceptance_ledger_receipt_registration_invocation_connector_runtime_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback_blocks_evidence_packet_send_recording_acceptance_ledger_receipt_registration_invocation_connector_runtime_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_boundary_readback_blocks_acceptance_recording_evidence_recording_receipt_persistence_invocation_runtime_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_blocks_evidence_artifact_identity_acceptance_recording_ledger_receipt_registration_invocation_connector_runtime_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_blocks_acceptance_record_persistence_denial_receipt_idempotency_ledger_receipt_registration_invocation_runtime_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback_blocks_acceptance_record_store_binding_idempotency_ledger_receipt_runtime_rollback_kill_switch_evidence_feature_gate_and_live")) != null
  and (.blockers | index("hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback_blocks_shadow_execution_store_write_registry_lookup_invocation_runtime_and_live")) != null
  and (.blockers | index("hepta_systems_tool_registry_shadow_registration_lookup_readback_blocks_shadow_registry_lookup_registration_invocation_ledger_runtime_and_live")) != null
  and (.blockers | index("hepta_systems_matrix_report_single_render_cache_boundary_readback_blocks_cache_persistence_downstream_direct_matrix_render_and_live")) != null
  and (.blockers | index("current_reality_matrix_compact_cache_boundary_readback_blocks_cache_persistence_evidence_approval_decision_and_live")) != null
  and (.blockers | index("hepta_system_status_internal_read_only_invocation_blocks_external_network_credentials_mutation_and_live")) != null
  and (.blockers | index("hepta_system_status_operator_approval_protocol_blocks_auto_acceptance_broker_write_persistence_and_live")) != null
  and (.blockers | index("controlled_canary_readiness_plan_blocks_activation_transport_persistence_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_inventory_blocks_git_mutation_cleanup_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_grouping_freeze_plan_blocks_freeze_application_git_mutation_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_grouping_freeze_operator_readback_blocks_git_mutation_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_actionable_clean_worktree_strategy_blocks_strategy_application_git_mutation_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_blocks_send_persistence_git_mutation_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_blocks_send_persistence_git_mutation_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_blocks_git_mutation_cleanup_delete_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_blocks_decision_recording_git_mutation_cleanup_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback_blocks_packet_persistence_decision_recording_git_mutation_cleanup_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback_blocks_decision_recording_persistence_receipt_approval_git_mutation_cleanup_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback_blocks_approval_acceptance_receipt_decision_recording_evidence_git_mutation_cleanup_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback_blocks_evidence_recording_persistence_receipt_approval_decision_git_mutation_cleanup_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_release_risk_snapshot_blocks_release_cutover_git_mutation_cleanup_evidence_approval_decision_recording_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_blocks_test_probe_git_mutation_cleanup_evidence_approval_decision_recording_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_blocks_test_probe_git_mutation_cleanup_evidence_approval_decision_recording_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_blocks_owner_persistence_freeze_application_classification_persistence_git_mutation_cleanup_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_blocks_owner_assignment_freeze_classification_operator_packet_git_cleanup_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_blocks_packet_send_persistence_owner_assignment_git_cleanup_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation_blocks_git_mutation_cleanup_delete_owner_freeze_classification_packet_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_without_git_mutation_blocks_decision_recording_approval_evidence_git_cleanup_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_without_git_mutation_blocks_packet_readback_decision_recording_approval_evidence_git_cleanup_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_without_recording_blocks_decision_recording_persistence_receipt_approval_evidence_owner_freeze_classification_test_probe_git_cleanup_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_owner_freeze_classification_operator_approval_acceptance_boundary_readback_without_acceptance_blocks_approval_request_acceptance_recording_receipt_decision_evidence_owner_freeze_classification_test_probe_git_cleanup_release_and_live")) != null
  and (.blockers | index("dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording_blocks_evidence_recording_persistence_receipt_approval_decision_owner_freeze_classification_test_probe_git_cleanup_release_and_live")) != null
  and (.blockers | index("dirty_worktree_boundary")) != null
  and (.next_actions | index("close_controlled_live_evidence_before_status_canary_start")) != null
  and .next_migration_step == "close_controlled_live_evidence_before_status_canary_start"
  and .current_reality_capability_matrix_ready == true
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

printf 'hepta-systems-current-reality-capability-matrix-gate: PASS: current plugins/tools/workflow reality is tracked with live paths blocked\n'
