#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-controlled-live-operator-readiness-dashboard-report.sh"
KILL_SWITCH_BOUNDARY_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-kill-switch-rehearsal-boundary-readback-report.sh"
CLOSURE_INDEX_GATE="$ROOT/scripts/hepta-systems-tool-execution-live-cutover-closure-index-gate.sh"
EVIDENCE_PLAN_GATE="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-collection-plan-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_OPERATOR_READINESS_DASHBOARD_2026-06-27.md"

fail() {
  printf 'hepta-systems-controlled-live-operator-readiness-dashboard-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Phase 6 dashboard report: $REPORT"
[[ -x "$KILL_SWITCH_BOUNDARY_REPORT" ]] || fail "missing executable Phase 5n kill-switch rehearsal boundary report: $KILL_SWITCH_BOUNDARY_REPORT"
[[ -x "$CLOSURE_INDEX_GATE" ]] || fail "missing executable live cutover closure index gate: $CLOSURE_INDEX_GATE"
[[ -x "$EVIDENCE_PLAN_GATE" ]] || fail "missing executable required evidence collection plan gate: $EVIDENCE_PLAN_GATE"
[[ -f "$DOC" ]] || fail "missing Phase 6 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the controlled-live operator readiness dashboard report"
fi

grep -q 'Controlled Live Operator Readiness Dashboard' "$DOC" \
  || fail "architecture note must document Controlled Live Operator Readiness Dashboard"
grep -q 'consumer dashboard without suffix expansion' "$DOC" \
  || fail "architecture note must document consumer dashboard without suffix expansion"
grep -q 'not a new current-reality matrix capability row' "$DOC" \
  || fail "architecture note must document that Phase 6 is not a matrix capability row"
grep -q 'Status Canary Evidence Acceptance Packet' "$DOC" \
  || fail "architecture note must document Status Canary Evidence Acceptance Packet"
grep -q 'Status Canary Evidence Source Adapter' "$DOC" \
  || fail "architecture note must document Status Canary Evidence Source Adapter"
grep -q 'per-source metadata contract route' "$DOC" \
  || fail "architecture note must document status canary source adapter metadata contracts"
grep -q 'Status Canary Evidence Source Reason Packet' "$DOC" \
  || fail "architecture note must document Status Canary Evidence Source Reason Packet"
grep -q 'contract audit' "$DOC" \
  || fail "architecture note must document status canary source contract audit"
grep -q 'Status Canary Evidence Source Readback' "$DOC" \
  || fail "architecture note must document Status Canary Evidence Source Readback"
grep -q 'Status Canary Evidence Source Validator' "$DOC" \
  || fail "architecture note must document Status Canary Evidence Source Validator"
grep -q 'evidence-packet reason-audit overlay' "$DOC" \
  || fail "architecture note must document start guard evidence-packet reason-audit overlay"
grep -q 'runner-chain start-guard reason-audit carry-through' "$DOC" \
  || fail "architecture note must document runner-chain start-guard reason-audit carry-through"
grep -q 'Status Canary Runner Dry-Run Selector' "$DOC" \
  || fail "architecture note must document Status Canary Runner Dry-Run Selector"
grep -q '17-blocker release classification' "$DOC" \
  || fail "architecture note must document closure-index 17-blocker release classification"
grep -q 'no approval request, approval acceptance, approval recording, evidence recording, evidence persistence, blocker waiver, credential read, transport mutation, packet persistence, attachment persistence, readback persistence, ledger write, event-log write, SQLite write, provider invocation, model invocation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed dashboard boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_operator_readiness_dashboard"
  and .status == "ready_blocked"
  and .gate == "controlled_live_operator_readiness_dashboard_gate"
  and .schema_version == "controlled_live_operator_readiness_dashboard_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_single_render_cache_boundary_ready == true
  and .source_matrix_ready == true
  and .source_matrix_capability_count == 104
  and .source_matrix_capability_ready_count == 104
  and .source_matrix_live_enabled_count == 0
  and .source_matrix_all_live_paths_blocked == true
  and .source_matrix_next_migration_step == "close_controlled_live_evidence_before_status_canary_start"
  and .source_kill_switch_boundary_readback_ready == true
  and .source_kill_switch_boundary_entry_count == 7
  and .source_kill_switch_boundary_ready_count == 7
  and .source_live_cutover_closure_index_surface == "tool_execution_live_cutover_closure_index"
  and .source_live_cutover_closure_index_ready == true
  and .source_live_cutover_final_gate_ready_count == 2
  and .source_live_cutover_closure_blocker_count == 17
  and .source_live_cutover_closure_blocker_category_count == 4
  and .source_live_cutover_closure_blocker_category_ready_count == 4
  and .source_live_cutover_closure_blocker_category_blocker_count == 17
  and .source_live_cutover_closure_blocker_categorization_ready == true
  and (.source_live_cutover_closure_blocker_categories | length) == 4
  and any(.source_live_cutover_closure_blocker_categories[]; .id == "approval_control" and .blocker_count == 4)
  and any(.source_live_cutover_closure_blocker_categories[]; .id == "execution_and_receipts" and .blocker_count == 9)
  and any(.source_live_cutover_closure_blocker_categories[]; .id == "runner_selector" and .blocker_count == 2 and (.blocker_ids | index("concrete_runner_preflight_selector_fail_closed")) != null)
  and any(.source_live_cutover_closure_blocker_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2 and (.blocker_ids | index("dirty_worktree_owner_freeze_operator_decision_pending")) != null)
  and .source_required_evidence_collection_plan_ready == true
  and .status_canary_final_guard_present == true
  and .status_canary_tool_id == "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp"
  and .preflight_only_connector_tool_id == "preview:connector:hepta-system@hepta-local:hepta_system_local_app"
  and .status_canary_candidate_count == 2
  and .selected_status_canary_count == 1
  and .preflight_only_non_selected_count == 1
  and .status_canary_final_gate_ready_count == 2
  and .status_canary_final_guard_live_blocked_count == 1
  and .status_canary_final_guard_approval_missing_count == 1
  and .status_canary_final_guard_live_enabled == false
  and .status_canary_final_guard_tool_invocation_enabled == false
  and .status_canary_final_guard_ledger_write_enabled == false
  and .status_canary_evidence_packet_ready == true
  and .status_canary_evidence_packet_id == "status-canary-evidence-packet/hepta-system-status/v1"
  and .status_canary_evidence_packet_item_count == 7
  and .status_canary_evidence_packet_missing_count == 7
  and .status_canary_evidence_packet_recorded_count == 0
  and .status_canary_evidence_packet_waived_count == 0
  and .status_canary_evidence_packet_expired_count == 0
  and .status_canary_evidence_packet_invalid_count == 0
  and .status_canary_evidence_packet_decision_reason_audit_count == 0
  and .status_canary_evidence_packet_decision_reason_audit_ready_count == 0
  and .status_canary_evidence_packet_decision_reason_audit_rejected_count == 0
  and .status_canary_evidence_packet_complete == false
  and .status_canary_start_blocked_by_evidence_packet == true
  and .status_canary_start_allowed_by_evidence_packet == false
  and .status_canary_evidence_packet_guard_route == "status_canary_evidence_packet_blocked_missing_evidence"
  and .status_canary_evidence_acceptance_packet_ready == true
  and .status_canary_evidence_acceptance_packet_id == "status-canary-evidence-acceptance-packet/hepta-system-status/v1"
  and .status_canary_evidence_acceptance_packet_route == "status_canary_evidence_acceptance_packet_ready_no_decision_requests"
  and .status_canary_evidence_acceptance_request_count == 0
  and .status_canary_evidence_acceptance_known_request_count == 0
  and .status_canary_evidence_acceptance_unknown_request_count == 0
  and .status_canary_evidence_acceptance_duplicate_request_count == 0
  and .status_canary_evidence_acceptance_request_source_validator_bound_count == 0
  and .status_canary_evidence_acceptance_request_source_validator_contract_audit_ready_count == 0
  and .status_canary_evidence_acceptance_request_reason_audit_count == 0
  and .status_canary_evidence_acceptance_request_reason_audit_ready_count == 0
  and .status_canary_evidence_acceptance_request_reason_audit_rejected_count == 0
  and .status_canary_evidence_acceptance_accepted_decision_count == 0
  and .status_canary_evidence_acceptance_rejected_decision_count == 0
  and .status_canary_evidence_acceptance_generated_override_count == 0
  and .status_canary_evidence_acceptance_generated_override_reason_audit_ready_count == 0
  and .status_canary_evidence_source_adapter_ready == true
  and .status_canary_evidence_source_adapter_id == "status-canary-evidence-source-adapter/hepta-system-status/v1"
  and .status_canary_evidence_source_adapter_route == "status_canary_evidence_source_adapter_ready_no_inputs"
  and .status_canary_evidence_source_adapter_count == 7
  and .status_canary_evidence_source_adapter_input_count == 0
  and .status_canary_evidence_source_adapter_generated_fixture_count == 0
  and .status_canary_evidence_source_adapter_missing_input_count == 7
  and .status_canary_evidence_source_adapter_metadata_contract_count == 7
  and .status_canary_evidence_source_adapter_metadata_contract_ready_count == 7
  and .status_canary_evidence_source_adapter_input_contract_field_count == 21
  and .status_canary_evidence_source_adapter_readback_fixture_contract_field_count == 70
  and .status_canary_evidence_source_adapter_required_field_validator_count == 7
  and .status_canary_evidence_source_adapter_required_field_validator_ready_count == 7
  and .status_canary_evidence_source_adapter_required_field_rejected_count == 0
  and .status_canary_evidence_source_adapter_missing_required_field_count == 0
  and .status_canary_evidence_source_reason_packet_ready == true
  and .status_canary_evidence_source_reason_packet_id == "status-canary-evidence-source-reason-packet/hepta-system-status/v1"
  and .status_canary_evidence_source_reason_packet_route == "status_canary_evidence_source_reason_packet_ready_no_adapter_inputs"
  and .status_canary_evidence_source_reason_packet_source_count == 7
  and .status_canary_evidence_source_decision_reason_count == 28
  and .status_canary_evidence_source_decision_reason_ready_count == 28
  and .status_canary_evidence_source_decision_required_field_count == 84
  and .status_canary_evidence_source_missing_required_field_reason_count == 84
  and .status_canary_evidence_source_adapter_input_missing_reason_count == 28
  and .status_canary_evidence_source_adapter_input_other_decision_reason_count == 0
  and .status_canary_evidence_source_adapter_rejection_reason_count == 0
  and .status_canary_evidence_source_fixture_generation_allowed_count == 0
  and .status_canary_evidence_source_fixture_generation_blocked_count == 28
  and .status_canary_evidence_source_readback_ready == true
  and .status_canary_evidence_source_readback_id == "status-canary-evidence-source-readback/hepta-system-status/v1"
  and .status_canary_evidence_source_readback_route == "status_canary_evidence_source_readback_ready_no_fixtures"
  and .status_canary_evidence_source_readback_fixture_count == 0
  and .status_canary_evidence_source_readback_observation_count == 0
  and .status_canary_evidence_source_readback_missing_observation_count == 7
  and .status_canary_evidence_source_readback_contract_audit_count == 7
  and .status_canary_evidence_source_readback_contract_audit_ready_count == 7
  and .status_canary_evidence_source_readback_fixture_contract_audit_ready_count == 0
  and .status_canary_evidence_source_readback_reason_packet_bound == true
  and .status_canary_evidence_source_readback_reason_packet_ready == true
  and .status_canary_evidence_source_readback_reason_packet_route == "status_canary_evidence_source_reason_packet_ready_no_adapter_inputs"
  and .status_canary_evidence_source_readback_fixture_reason_audit_count == 0
  and .status_canary_evidence_source_readback_fixture_reason_audit_ready_count == 0
  and .status_canary_evidence_source_readback_fixture_reason_audit_rejected_count == 0
  and .status_canary_evidence_source_validator_ready == true
  and .status_canary_evidence_source_validator_id == "status-canary-evidence-source-validator/hepta-system-status/v1"
  and .status_canary_evidence_source_validator_route == "status_canary_evidence_source_validator_ready_no_observations"
  and .status_canary_evidence_source_validator_contract_audit_count == 0
  and .status_canary_evidence_source_validator_contract_audit_ready_count == 0
  and .status_canary_evidence_source_validator_contract_audit_rejected_count == 0
  and .status_canary_evidence_source_validator_reason_audit_count == 0
  and .status_canary_evidence_source_validator_reason_audit_ready_count == 0
  and .status_canary_evidence_source_validator_reason_audit_rejected_count == 0
  and .status_canary_evidence_source_observation_count == 0
  and .status_canary_evidence_source_missing_count == 7
  and .status_canary_evidence_source_validated_count == 0
  and .status_canary_evidence_source_rejected_count == 0
  and .status_canary_evidence_source_generated_request_count == 0
  and .status_canary_start_guard_ready == true
  and .status_canary_start_guard_id == "status-canary-start-guard/hepta-system-status/v1"
  and .status_canary_start_guard_route == "status_canary_start_blocked_missing_evidence_packet"
  and .status_canary_start_guard_switch_enabled == false
  and .status_canary_start_guard_evidence_packet_reason_audit_count == 0
  and .status_canary_start_guard_evidence_packet_reason_audit_ready_count == 0
  and .status_canary_start_guard_evidence_packet_reason_audit_rejected_count == 0
  and .status_canary_start_guard_evidence_packet_reason_audit_ready == true
  and .status_canary_start_guard_blocked == true
  and .status_canary_start_guard_allowed == false
  and .status_canary_start_request_gate_ready == true
  and .status_canary_start_request_gate_id == "status-canary-start-request-gate/hepta-system-status/v1"
  and .status_canary_start_request_gate_route == "status_canary_start_request_blocked_no_request"
  and .status_canary_start_request_present == false
  and .status_canary_start_request_requested_tool_id == "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp"
  and .status_canary_start_request_selected_status_canary == true
  and .status_canary_start_request_preflight_only_connector == false
  and .status_canary_start_request_source_start_guard_reason_audit_ready == true
  and .status_canary_start_request_blocked == true
  and .status_canary_start_request_allowed == false
  and .status_canary_runner_adapter_ready == true
  and .status_canary_runner_adapter_id == "status-canary-runner-adapter/hepta-system-status/v1"
  and .status_canary_runner_adapter_route == "status_canary_runner_adapter_blocked_no_runner_request"
  and .status_canary_runner_adapter_request_present == false
  and .status_canary_runner_adapter_source_gate_bound == true
  and .status_canary_runner_adapter_source_start_guard_reason_audit_ready == true
  and .status_canary_runner_adapter_source_start_request_allowed == false
  and .status_canary_runner_adapter_blocked == true
  and .status_canary_runner_adapter_allowed == false
  and .status_canary_runner_start_surface_ready == true
  and .status_canary_runner_start_surface_id == "status-canary-runner-start-surface/hepta-system-status/v1"
  and .status_canary_runner_start_surface_route == "status_canary_runner_start_surface_blocked_no_start_request"
  and .status_canary_runner_start_request_present == false
  and .status_canary_runner_start_surface_source_adapter_bound == true
  and .status_canary_runner_start_surface_source_start_guard_reason_audit_ready == true
  and .status_canary_runner_start_surface_source_adapter_allowed == false
  and .status_canary_runner_start_surface_blocked == true
  and .status_canary_runner_start_surface_allowed == false
  and .status_canary_runner_entry_boundary_ready == true
  and .status_canary_runner_entry_boundary_id == "status-canary-runner-entry-boundary/hepta-system-status/v1"
  and .status_canary_runner_entry_boundary_route == "status_canary_runner_entry_boundary_blocked_no_entry_request"
  and .status_canary_runner_entry_request_present == false
  and .status_canary_runner_entry_boundary_source_start_surface_bound == true
  and .status_canary_runner_entry_boundary_source_start_guard_reason_audit_ready == true
  and .status_canary_runner_entry_boundary_source_start_surface_allowed == false
  and .status_canary_runner_entry_boundary_blocked == true
  and .status_canary_runner_entry_boundary_allowed == false
  and .status_canary_runner_entry_adapter_ready == true
  and .status_canary_runner_entry_adapter_id == "status-canary-runner-entry-adapter/hepta-system-status/v1"
  and .status_canary_runner_entry_adapter_route == "status_canary_runner_entry_adapter_blocked_no_adapter_request"
  and .status_canary_runner_entry_adapter_request_present == false
  and .status_canary_runner_entry_adapter_source_boundary_bound == true
  and .status_canary_runner_entry_adapter_source_start_guard_reason_audit_ready == true
  and .status_canary_runner_entry_adapter_source_boundary_allowed == false
  and .status_canary_runner_entry_adapter_blocked == true
  and .status_canary_runner_entry_adapter_allowed == false
  and .status_canary_runner_binding_guard_ready == true
  and .status_canary_runner_binding_guard_id == "status-canary-runner-binding-guard/hepta-system-status/v1"
  and .status_canary_runner_binding_guard_route == "status_canary_runner_binding_guard_blocked_no_binding_request"
  and .status_canary_runner_binding_request_present == false
  and .status_canary_runner_binding_guard_source_entry_adapter_bound == true
  and .status_canary_runner_binding_guard_source_start_guard_reason_audit_ready == true
  and .status_canary_runner_binding_guard_source_entry_adapter_allowed == false
  and .status_canary_runner_binding_guard_blocked == true
  and .status_canary_runner_binding_guard_allowed == false
  and .status_canary_runner_dry_run_selector_ready == true
  and .status_canary_runner_dry_run_selector_id == "status-canary-runner-dry-run-selector/hepta-system-status/v1"
  and .status_canary_runner_dry_run_selector_route == "status_canary_runner_dry_run_selector_blocked_no_selector_request"
  and .status_canary_runner_dry_run_selector_request_present == false
  and .status_canary_runner_dry_run_selector_source_binding_guard_bound == true
  and .status_canary_runner_dry_run_selector_source_start_guard_reason_audit_ready == true
  and .status_canary_runner_dry_run_selector_source_binding_guard_allowed == false
  and .status_canary_runner_dry_run_selector_blocked == true
  and .status_canary_runner_dry_run_selector_allowed == false
  and .status_canary_evidence_closure_entry_count == 7
  and .status_canary_evidence_closure_ready_count == 7
  and .status_canary_evidence_closure_missing_count == 7
  and .status_canary_evidence_closure_recorded_count == 0
  and .status_canary_evidence_closure_waived_count == 0
  and .status_canary_evidence_closure_actionable_precondition_count == 7
  and .lib_export_present == true
  and .capability_row_count == 104
  and .capability_ready_count == 104
  and .live_enabled_count == 0
  and .all_live_paths_blocked == true
  and .blocker_entry_count == 7
  and .operator_visible_blocker_count == 7
  and .missing_evidence_blocker_count == 7
  and .accepted_blocker_count == 0
  and .waived_blocker_count == 0
  and .evidence_recorded_count == 0
  and .approval_request_sent == false
  and .approval_accepted == false
  and .credential_read_allowed == false
  and .transport_mutation_allowed == false
  and .persistence_allowed == false
  and .live_execution_allowed == false
  and .dashboard_ready == true
  and (.entries | length) == 7
  and (.entries | all(.operator_visible == true and .queryable == true and .diffable == true and .operator_status == "blocked_missing_evidence" and .evidence_state == "missing" and (.dashboard_key | length) > 0 and (.dashboard_route | startswith("readback://controlled-live/operator-dashboard/")) and (.source_readback_route | length) > 0 and .acceptance_allowed == false and .waiver_allowed == false and .evidence_recording_allowed == false and .credential_read_allowed == false and .transport_mutation_allowed == false and .persistence_allowed == false and .live_mutation_allowed == false))
  and (.status_canary_evidence_closure_entries | length) == 7
  and (.status_canary_evidence_closure_entries | all(.selected_status_canary_tool_id == "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp" and .preflight_only_connector_tool_id == "preview:connector:hepta-system@hepta-local:hepta_system_local_app" and .operator_visible == true and .action_required == true and .canary_start_blocked == true and .evidence_state == "missing" and .evidence_recorded == false and .evidence_waived == false and .evidence_expired == false and .evidence_invalid == false and .evidence_recording_allowed == false and .waiver_allowed == false and .credential_read_allowed == false and .transport_mutation_allowed == false and .persistence_allowed == false and .live_mutation_allowed == false and (.closure_key | startswith("controlled_live.status_canary.evidence_closure.")) and (.closure_route | startswith("readback://controlled-live/status-canary/evidence-closure/"))))
  and any(.status_canary_evidence_closure_entries[]; .source_blocker_id == "dirty_worktree_boundary" and .action_kind == "clean_worktree_snapshot_required")
  and any(.status_canary_evidence_closure_entries[]; .source_blocker_id == "operator_live_approval_missing" and .action_kind == "operator_live_approval_packet_required")
  and any(.status_canary_evidence_closure_entries[]; .source_blocker_id == "fresh_soak_readback_missing" and .action_kind == "fresh_status_canary_soak_readback_required")
  and any(.status_canary_evidence_closure_entries[]; .source_blocker_id == "credential_boundary_attestation_missing" and .action_kind == "credential_boundary_attestation_required")
  and any(.status_canary_evidence_closure_entries[]; .source_blocker_id == "gateway_native_telegram_post_boundary_approval_missing" and .action_kind == "transport_boundary_approval_required")
  and any(.status_canary_evidence_closure_entries[]; .source_blocker_id == "rollback_rehearsal_missing" and .action_kind == "rollback_rehearsal_packet_required")
  and any(.status_canary_evidence_closure_entries[]; .source_blocker_id == "kill_switch_rehearsal_missing" and .action_kind == "kill_switch_rehearsal_packet_required")
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .dashboard_route == "readback://controlled-live/operator-dashboard/dirty-worktree-boundary")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .dashboard_route == "readback://controlled-live/operator-dashboard/operator-live-approval-missing")
  and any(.entries[]; .source_blocker_id == "fresh_soak_readback_missing" and .dashboard_route == "readback://controlled-live/operator-dashboard/fresh-soak-readback-missing")
  and any(.entries[]; .source_blocker_id == "credential_boundary_attestation_missing" and .dashboard_route == "readback://controlled-live/operator-dashboard/credential-boundary-attestation-missing")
  and any(.entries[]; .source_blocker_id == "gateway_native_telegram_post_boundary_approval_missing" and .dashboard_route == "readback://controlled-live/operator-dashboard/gateway-native-telegram-post-boundary-approval-missing")
  and any(.entries[]; .source_blocker_id == "rollback_rehearsal_missing" and .dashboard_route == "readback://controlled-live/operator-dashboard/rollback-rehearsal-missing")
  and any(.entries[]; .source_blocker_id == "kill_switch_rehearsal_missing" and .dashboard_route == "readback://controlled-live/operator-dashboard/kill-switch-rehearsal-missing")
  and (.next_actions | index("close_controlled_live_evidence_before_status_canary_start")) != null
  and (.next_actions | index("keep_status_canary_final_guard_read_only")) != null
  and (.next_actions | index("keep_connector_candidate_preflight_only")) != null
  and .next_migration_step == "close_controlled_live_evidence_before_status_canary_start"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$KILL_SWITCH_BOUNDARY_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback"
  and .status == "ready_blocked"
  and .kill_switch_rehearsal_boundary_readback_ready == true
  and .kill_switch_rehearsal_boundary_entry_count == 7
  and .kill_switch_rehearsal_boundary_ready_count == 7
  and .kill_switch_rehearsal_boundary_closed_count == 7
  and .kill_switch_rehearsal_execution_blocked_count == 7
  and .kill_switch_mutation_blocked_count == 7
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$CLOSURE_INDEX_GATE" >/dev/null
"$EVIDENCE_PLAN_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime controlled_live_operator_readiness_dashboard --lib
)

printf 'hepta-systems-controlled-live-operator-readiness-dashboard-gate: PASS: controlled-live readiness is collapsed into an operator dashboard without suffix expansion or live mutation\n'
