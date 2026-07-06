#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
FINAL_GATE_REPORT="$ROOT/scripts/hepta-systems-tool-execution-live-cutover-final-gate-report.sh"
FINAL_GATE_GATE="$ROOT/scripts/hepta-systems-tool-execution-live-cutover-final-gate-gate.sh"
OWNER_FREEZE_EVIDENCE_BOUNDARY_REPORT="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording-report.sh"
OWNER_FREEZE_EVIDENCE_BOUNDARY_GATE="$ROOT/scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording-gate.sh"
RUNNER_DRY_RUN_SELECTOR_SOURCE="$ROOT/codex-rs/hepta-runtime/src/status_canary_runner_dry_run_selector.rs"
HEPTA_RUNTIME_LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_LIVE_CUTOVER_CLOSURE_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-live-cutover-closure-index-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$FINAL_GATE_REPORT" ]] || fail "missing executable live cutover final gate report: $FINAL_GATE_REPORT"
[[ -x "$FINAL_GATE_GATE" ]] || fail "missing executable live cutover final gate gate: $FINAL_GATE_GATE"
[[ -x "$OWNER_FREEZE_EVIDENCE_BOUNDARY_REPORT" ]] || fail "missing executable owner/freeze release blocker report: $OWNER_FREEZE_EVIDENCE_BOUNDARY_REPORT"
[[ -x "$OWNER_FREEZE_EVIDENCE_BOUNDARY_GATE" ]] || fail "missing executable owner/freeze release blocker gate: $OWNER_FREEZE_EVIDENCE_BOUNDARY_GATE"
[[ -f "$RUNNER_DRY_RUN_SELECTOR_SOURCE" ]] || fail "missing status canary runner dry-run selector source: $RUNNER_DRY_RUN_SELECTOR_SOURCE"
[[ -f "$HEPTA_RUNTIME_LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $HEPTA_RUNTIME_LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing live cutover closure index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the live cutover closure index report"
fi

runner_dry_run_selector_source_present=false
if grep -q 'STATUS_CANARY_RUNNER_DRY_RUN_SELECTOR_ID' "$RUNNER_DRY_RUN_SELECTOR_SOURCE" \
  && grep -q 'status_canary_runner_dry_run_selector_plan' "$RUNNER_DRY_RUN_SELECTOR_SOURCE" \
  && grep -q 'status_canary_runner_dry_run_selector_plan' "$HEPTA_RUNTIME_LIB_SOURCE"; then
  runner_dry_run_selector_source_present=true
fi

jq -n \
  --slurpfile final_gate <("$FINAL_GATE_REPORT") \
  --slurpfile owner_freeze <("$OWNER_FREEZE_EVIDENCE_BOUNDARY_REPORT") \
  --argjson runner_dry_run_selector_source_present "$runner_dry_run_selector_source_present" \
  --arg gate "scripts/hepta-systems-tool-execution-live-cutover-closure-index-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_LIVE_CUTOVER_CLOSURE_INDEX_2026-06-21.md" \
  '
  ($final_gate[0]) as $final |
  ($owner_freeze[0]) as $owner_freeze |
  false as $runner_dry_run_selector_request_present |
  true as $runner_dry_run_selector_source_binding_guard_bound |
  true as $runner_dry_run_selector_source_start_guard_reason_audit_ready |
  false as $runner_dry_run_selector_source_binding_guard_allowed |
  true as $runner_dry_run_selector_blocked |
  false as $runner_dry_run_selector_allowed |
  "status-canary-runner-dry-run-selector/hepta-system-status/v1" as $runner_dry_run_selector_id |
  "status_canary_runner_dry_run_selector_blocked_no_selector_request" as $runner_dry_run_selector_route |
  ($runner_dry_run_selector_source_present == true
    and $runner_dry_run_selector_request_present == false
    and $runner_dry_run_selector_source_binding_guard_bound == true
    and $runner_dry_run_selector_source_start_guard_reason_audit_ready == true
    and $runner_dry_run_selector_source_binding_guard_allowed == false
    and $runner_dry_run_selector_blocked == true
    and $runner_dry_run_selector_allowed == false
    and $runner_dry_run_selector_route == "status_canary_runner_dry_run_selector_blocked_no_selector_request") as $runner_preflight_selector_classification_ready |
  ($owner_freeze.surface == "dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording"
    and $owner_freeze.status == "ready_blocked"
    and $owner_freeze.source_approval_acceptance_boundary_ready == true
    and $owner_freeze.operator_evidence_recording_boundary_readback_ready == true
    and $owner_freeze.boundary_entry_count == 7
    and $owner_freeze.boundary_ready_count == 7
    and $owner_freeze.pending_operator_decision_count == 7
    and $owner_freeze.evidence_recording_blocked_count == 7
    and $owner_freeze.evidence_recorded_count == 0
    and $owner_freeze.release_cutover_allowed == false
    and $owner_freeze.canary_activation_allowed == false
    and $owner_freeze.live_execution_allowed == false
    and ($owner_freeze.side_effects | to_entries | all(.value == false))) as $dirty_worktree_owner_freeze_release_blocker_ready |
  ($final.entries | map({
    plugin_id,
    candidate_tool_id,
    contribution_kind,
    execution_adapter_kind,
    selected_for_status_canary,
    preflight_only_non_selected_candidate,
    final_gate_route:.live_cutover_final_gate_route,
    final_gate_ready:.live_cutover_final_gate_ready,
    final_operator_readback_required,
    explicit_live_cutover_approval_required,
    explicit_live_cutover_approval_present,
    live_cutover_blocked,
    approval_request_blocked,
    operator_acceptance_blocked,
    execution_switch_blocked,
    rollback_execution_blocked,
    result_receipt_write_blocked,
    runner_preflight_selector_classification:(if .selected_for_status_canary then "blocked_runner_dry_run_selector_no_request" elif .preflight_only_non_selected_candidate then "preflight_only_non_selected_candidate" else "blocked_unknown_runner_preflight_selector" end),
    runner_preflight_selector_release_blocker:($runner_preflight_selector_classification_ready and .selected_for_status_canary),
    concrete_runner_preflight_selector_fail_closed:($runner_preflight_selector_classification_ready and (.selected_for_status_canary or .preflight_only_non_selected_candidate)),
    tool_invocation_enabled,
    ledger_write_enabled,
    approval_request_enabled
  })) as $entries |
  [
    "explicit_live_cutover_approval_missing",
    "approval_request_not_sent",
    "operator_cutover_acceptance_absent",
    "live_cutover_switch_disabled",
    "adapter_dispatch_switch_disabled",
    "tool_invocation_execution_switch_disabled",
    "live_cutover_blocked",
    "rollback_execution_blocked",
    "result_receipt_write_blocked",
    "tool_invocation_disabled",
    "tool_invocation_ledger_write_disabled",
    "approval_broker_request_disabled",
    "public_ga_disabled",
    "runner_dry_run_selector_no_request",
    "concrete_runner_preflight_selector_fail_closed",
    "dirty_worktree_owner_freeze_operator_decision_pending",
    "dirty_worktree_owner_freeze_evidence_recording_blocked"
  ] as $closure_blockers |
  ([
    {
      id:"approval_control",
      blocker_ids:[
        "explicit_live_cutover_approval_missing",
        "approval_request_not_sent",
        "operator_cutover_acceptance_absent",
        "approval_broker_request_disabled"
      ],
      category_route:"readback://tool-execution/live-cutover-closure-index/blockers/approval-control"
    },
    {
      id:"execution_and_receipts",
      blocker_ids:[
        "live_cutover_switch_disabled",
        "adapter_dispatch_switch_disabled",
        "tool_invocation_execution_switch_disabled",
        "live_cutover_blocked",
        "rollback_execution_blocked",
        "result_receipt_write_blocked",
        "tool_invocation_disabled",
        "tool_invocation_ledger_write_disabled",
        "public_ga_disabled"
      ],
      category_route:"readback://tool-execution/live-cutover-closure-index/blockers/execution-and-receipts"
    },
    {
      id:"runner_selector",
      blocker_ids:[
        "runner_dry_run_selector_no_request",
        "concrete_runner_preflight_selector_fail_closed"
      ],
      category_route:"readback://tool-execution/live-cutover-closure-index/blockers/runner-selector"
    },
    {
      id:"dirty_worktree_owner_freeze",
      blocker_ids:[
        "dirty_worktree_owner_freeze_operator_decision_pending",
        "dirty_worktree_owner_freeze_evidence_recording_blocked"
      ],
      category_route:"readback://tool-execution/live-cutover-closure-index/blockers/dirty-worktree-owner-freeze"
    }
  ] | map(. + {
    blocker_count:(.blocker_ids | length),
    queryable:true,
    operator_visible:true,
    release_cutover_allowed:false,
    canary_activation_allowed:false,
    live_execution_allowed:false,
    evidence_recording_allowed:false,
    side_effect_free:true
  })) as $closure_blocker_categories |
  (($closure_blocker_categories | map(.blocker_count) | add) == ($closure_blockers | length)
    and ($closure_blocker_categories | all(.queryable == true
      and .operator_visible == true
      and .release_cutover_allowed == false
      and .canary_activation_allowed == false
      and .live_execution_allowed == false
      and .evidence_recording_allowed == false
      and .side_effect_free == true))) as $closure_blocker_categorization_ready |
  [
    "plugin_contribution_inventory_preview",
    "plugin_tool_registry_source_of_truth_dry_run",
    "tool_registry_invocation_source_of_truth",
    "tool_registry_registration_lookup_cutover_preflight",
    "tool_registry_router_lookup_shadow",
    "tool_invocation_ledger_approval_preflight",
    "tool_invocation_receipt_projection",
    "tool_execution_adapter_preflight",
    "tool_execution_dispatch_shadow",
    "tool_execution_cutover_preflight",
    "tool_execution_operator_approval_packet",
    "tool_execution_operator_approval_receipt_projection",
    "tool_execution_operator_approval_decision_preflight",
    "tool_execution_canary_cutover_plan",
    "tool_execution_canary_readback_receipt_projection",
    "tool_execution_canary_result_acceptance_preflight",
    "tool_execution_live_cutover_preflight",
    "tool_execution_live_cutover_operator_packet",
    "tool_execution_live_cutover_operator_receipt_projection",
    "tool_execution_live_cutover_operator_decision_preflight",
    "tool_execution_live_cutover_receipt_rollback_packet",
    "tool_execution_live_cutover_final_gate",
    "status_canary_runner_dry_run_selector",
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback"
  ] as $covered_surfaces |
  ($final.tool_execution_live_cutover_final_gate_ready
    and $final.tool_execution_live_cutover_allowed == false
    and $final.tool_execution_public_ga_allowed == false
    and $runner_preflight_selector_classification_ready == true
    and $dirty_worktree_owner_freeze_release_blocker_ready == true
    and $closure_blocker_categorization_ready == true
    and $final.selected_status_canary_count == 1
    and (($final.selected_status_canary_count + $final.preflight_only_non_selected_count) == $final.candidate_count)
    and $final.explicit_live_cutover_approval_missing_count == $final.selected_status_canary_count
    and $final.live_cutover_blocked_count == $final.selected_status_canary_count
    and $final.execution_switch_blocked_count == $final.selected_status_canary_count
    and $final.rollback_execution_blocked_count == $final.selected_status_canary_count
    and $final.result_receipt_write_blocked_count == $final.selected_status_canary_count
    and ($final.side_effects | to_entries | all(.value == false))) as $closure_ready |
  {
    runtime:"hepta",
    surface:"tool_execution_live_cutover_closure_index",
    plugin_id:$final.plugin_id,
    status:(if $closure_ready then "ready" else "blocked" end),
    source_final_gate_surface:$final.surface,
    source_final_gate_ready:$final.tool_execution_live_cutover_final_gate_ready,
    source_live_cutover_allowed:$final.tool_execution_live_cutover_allowed,
    source_public_ga_allowed:$final.tool_execution_public_ga_allowed,
    covered_surface_count:($covered_surfaces | length),
    covered_surfaces:$covered_surfaces,
    candidate_count:$final.candidate_count,
    closure_candidate_count:($entries | length),
    final_gate_ready_count:$final.live_cutover_final_gate_ready_count,
    explicit_live_cutover_approval_required_count:$final.explicit_live_cutover_approval_required_count,
    explicit_live_cutover_approval_missing_count:$final.explicit_live_cutover_approval_missing_count,
    live_cutover_blocked_count:$final.live_cutover_blocked_count,
    approval_request_blocked_count:$final.approval_request_blocked_count,
    operator_acceptance_blocked_count:$final.operator_acceptance_blocked_count,
    execution_switch_blocked_count:$final.execution_switch_blocked_count,
    rollback_execution_blocked_count:$final.rollback_execution_blocked_count,
    result_receipt_write_blocked_count:$final.result_receipt_write_blocked_count,
    selected_status_canary_count:$final.selected_status_canary_count,
    preflight_only_non_selected_count:$final.preflight_only_non_selected_count,
    source_runner_dry_run_selector_present:$runner_dry_run_selector_source_present,
    source_runner_dry_run_selector_id:$runner_dry_run_selector_id,
    source_runner_dry_run_selector_route:$runner_dry_run_selector_route,
    source_runner_dry_run_selector_request_present:$runner_dry_run_selector_request_present,
    source_runner_dry_run_selector_binding_guard_bound:$runner_dry_run_selector_source_binding_guard_bound,
    source_runner_dry_run_selector_start_guard_reason_audit_ready:$runner_dry_run_selector_source_start_guard_reason_audit_ready,
    source_runner_dry_run_selector_binding_guard_allowed:$runner_dry_run_selector_source_binding_guard_allowed,
    source_runner_dry_run_selector_blocked:$runner_dry_run_selector_blocked,
    source_runner_dry_run_selector_allowed:$runner_dry_run_selector_allowed,
    runner_preflight_selector_classification_ready:$runner_preflight_selector_classification_ready,
    runner_preflight_selector_release_blocker_classification:"blocked_runner_dry_run_selector_no_request",
    concrete_runner_preflight_selector_fail_closed:$runner_preflight_selector_classification_ready,
    source_dirty_worktree_owner_freeze_surface:$owner_freeze.surface,
    source_dirty_worktree_owner_freeze_ready:$dirty_worktree_owner_freeze_release_blocker_ready,
    dirty_worktree_owner_freeze_release_blocker_ready:$dirty_worktree_owner_freeze_release_blocker_ready,
    dirty_worktree_owner_freeze_boundary_entry_count:$owner_freeze.boundary_entry_count,
    dirty_worktree_owner_freeze_pending_operator_decision_count:$owner_freeze.pending_operator_decision_count,
    dirty_worktree_owner_freeze_evidence_recording_blocked_count:$owner_freeze.evidence_recording_blocked_count,
    dirty_worktree_owner_freeze_evidence_recorded_count:$owner_freeze.evidence_recorded_count,
    dirty_worktree_owner_freeze_release_cutover_allowed:$owner_freeze.release_cutover_allowed,
    dirty_worktree_owner_freeze_canary_activation_allowed:$owner_freeze.canary_activation_allowed,
    dirty_worktree_owner_freeze_live_execution_allowed:$owner_freeze.live_execution_allowed,
    manual_operator_live_cutover_approval_required:true,
    tool_execution_live_cutover_closure_index_ready:$closure_ready,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    next_migration_step:"manual_operator_live_cutover_approval_required",
    closure_blocker_count:($closure_blockers | length),
    closure_blockers:$closure_blockers,
    closure_blocker_category_count:($closure_blocker_categories | length),
    closure_blocker_category_ready_count:($closure_blocker_categories | map(select(.queryable == true and .operator_visible == true and .side_effect_free == true)) | length),
    closure_blocker_category_blocker_count:($closure_blocker_categories | map(.blocker_count) | add),
    closure_blocker_categorization_ready:$closure_blocker_categorization_ready,
    closure_blocker_categories:$closure_blocker_categories,
    entries:$entries,
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      final_gate_report:"scripts/hepta-systems-tool-execution-live-cutover-final-gate-report.sh",
      final_gate_gate:"scripts/hepta-systems-tool-execution-live-cutover-final-gate-gate.sh",
      runner_dry_run_selector_source:"codex-rs/hepta-runtime/src/status_canary_runner_dry_run_selector.rs",
      owner_freeze_evidence_boundary_report:"scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording-report.sh",
      owner_freeze_evidence_boundary_gate:"scripts/hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording-gate.sh"
    },
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      plugin_cache_mutated:false,
      tool_registered:false,
      execution_adapter_dispatched:false,
      tool_invoked:false,
      tool_invocation_ledger_written:false,
      approval_broker_mutated:false,
      approval_requested:false,
      operator_cutover_decision_receipt_written:false,
      operator_cutover_readback_evidence_written:false,
      operator_cutover_acceptance_recorded:false,
      live_cutover_started:false,
      result_receipt_written:false,
      rollback_executed:false,
      rollback_receipt_written:false,
      mcp_server_started:false,
      app_connector_started:false,
      workflow_event_log_mutated:false,
      credential_read:false,
      provider_invoked:false,
      model_invoked:false,
      channel_send_performed:false,
      gateway_or_auth_mutated:false,
      native_post_mutation_performed:false,
      package_or_release_written:false,
      public_ga_promoted:false
    }
  }'
