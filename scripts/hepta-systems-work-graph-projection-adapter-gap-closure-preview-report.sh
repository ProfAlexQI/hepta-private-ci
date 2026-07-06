#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

path_exists() {
  local path="$1"
  [[ -e "$path" ]]
}

bool_for() {
  if "$@"; then
    printf 'true\n'
  else
    printf 'false\n'
  fi
}

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-projection-gap-closure.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-unified-projection-audit-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-audit-preview-report.sh" \
  >"$tmpdir/unified_projection_audit.json"
capture_json_report \
  "hepta-work-graph-unified-projection-enforcement-readiness-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-preview-report.sh" \
  >"$tmpdir/enforcement_readiness.json"

gap_closure_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_projection_adapter_gap_closure_preview.rs
)"
gap_closure_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-projection-adapter-gap-closure-preview-report.sh
)"
gap_closure_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-projection-adapter-gap-closure-preview-gate.sh
)"

jq -n \
  --slurpfile audit "$tmpdir/unified_projection_audit.json" \
  --slurpfile readiness "$tmpdir/enforcement_readiness.json" \
  --argjson gap_closure_rust_module_present "$gap_closure_rust_module_present" \
  --argjson gap_closure_report_script_present "$gap_closure_report_script_present" \
  --argjson gap_closure_gate_script_present "$gap_closure_gate_script_present" \
  '
  $audit[0] as $audit
  | $readiness[0] as $readiness
  | def decision_for($id): [$readiness.source_decisions[] | select(.source_surface_id == $id)][0] // null;
  def store_collections($id):
    if $id == "plan_mode_proposed_plan_blocks" or $id == "app_server_turn_plan_notification" then ["nodes", "edges", "timelineEvents"]
    elif $id == "multi_agent_v2_mailbox_wait" then ["edges", "timelineEvents"]
    elif $id == "hepta_runtime_multi_agent_reducer" then ["nodes", "taskResults", "timelineEvents"]
    elif $id == "hepta_runtime_task_board" then ["nodes", "taskResults", "artifacts", "timelineEvents"]
    elif $id == "hepta_runtime_approval_broker" then ["nodes", "approvals", "timelineEvents"]
    else ["nodes", "timelineEvents"]
    end;
  def timeline_events($id):
    if $id == "update_plan_tool" or $id == "plan_mode_proposed_plan_blocks" then ["plan_step_observed"]
    elif $id == "hepta_runtime_multi_agent_reducer" then ["task_result_observed"]
    elif $id == "hepta_runtime_task_board" then ["scheduler_admission_decision_observed"]
    elif $id == "hepta_runtime_approval_broker" then ["approval_decision_observed"]
    else ["verification_gate_observed"]
    end;
  def fixture_action_id($id):
    if $id == "plan_mode_proposed_plan_blocks" then "close_plan_mode_adapter_projection_fixture"
    elif $id == "app_server_turn_plan_notification" then "close_app_server_plan_adapter_projection_fixture"
    elif $id == "multi_agent_v2_mailbox_wait" then "close_mailbox_wait_adapter_projection_fixture"
    elif $id == "hepta_runtime_multi_agent_reducer" then "close_multi_agent_reducer_adapter_projection_fixture"
    elif $id == "hepta_runtime_task_board" then "close_task_board_adapter_projection_fixture"
    else "close_adapter_projection_fixture"
    end;
  def store_action_id($id):
    if $id == "plan_mode_proposed_plan_blocks" then "close_plan_mode_unified_store_projection"
    elif $id == "app_server_turn_plan_notification" then "close_app_server_plan_unified_store_projection"
    elif $id == "multi_agent_v2_mailbox_wait" then "close_mailbox_wait_unified_store_projection"
    elif $id == "hepta_runtime_multi_agent_reducer" then "close_multi_agent_reducer_unified_store_projection"
    elif $id == "hepta_runtime_task_board" then "close_task_board_unified_store_projection"
    elif $id == "hepta_runtime_approval_broker" then "close_approval_broker_unified_store_projection"
    else "close_unified_store_projection"
    end;
  def timeline_action_id($id):
    if $id == "update_plan_tool" then "close_update_plan_timeline_projection"
    elif $id == "plan_mode_proposed_plan_blocks" then "close_plan_mode_timeline_projection"
    elif $id == "hepta_runtime_multi_agent_reducer" then "close_multi_agent_reducer_timeline_projection"
    elif $id == "hepta_runtime_task_board" then "close_task_board_timeline_projection"
    elif $id == "hepta_runtime_approval_broker" then "close_approval_broker_timeline_projection"
    else "close_timeline_projection"
    end;
  def task_result_action_id($id):
    if $id == "hepta_runtime_multi_agent_reducer" then "close_multi_agent_reducer_task_result_projection"
    else "close_task_result_projection"
    end;
  def action($id; $source; $kind; $collections; $events; $evidence; $blockers): {
    id: $id,
    source_surface_id: $source,
    adapter_kind: $kind,
    projected_collection_ids: $collections,
    timeline_event_type_ids: $events,
    required_evidence_fields: $evidence,
    closes_blocker_ids: $blockers,
    mutates_runtime: false,
    enforces_projection: false
  };
  def source_gap($source):
    (decision_for($source.source_surface_id)) as $decision
    | ($source.has_adapter_fixture | not) as $missing_fixture
    | ($source.has_unified_store_projection | not) as $missing_store
    | ($source.has_observability_timeline_projection | not) as $missing_timeline
    | (($source.requires_terminal_task_result and ($source.has_task_result_projection | not))) as $missing_task_result
    | {
        source_surface_id: $source.source_surface_id,
        source_category: $source.source_category,
        current_coverage_state: $source.coverage_state,
        enforcement_decision: ($decision.enforcement_decision // "deny_missing_unified_store_projection"),
        missing_adapter_fixture: $missing_fixture,
        missing_unified_store_adapter: $missing_store,
        missing_timeline_adapter: $missing_timeline,
        missing_task_result_adapter: $missing_task_result,
        closure_action_ids: (
          []
          + (if $missing_fixture then [fixture_action_id($source.source_surface_id)] else [] end)
          + (if $missing_store then [store_action_id($source.source_surface_id)] else [] end)
          + (if $missing_timeline then [timeline_action_id($source.source_surface_id)] else [] end)
          + (if $missing_task_result then [task_result_action_id($source.source_surface_id)] else [] end)
        ),
        current_blocker_ids: (($decision.source_blocker_ids // $source.blocker_ids)),
        expected_post_closure_state: "contract_ready_preview_after_gap_closure"
      };
  def closure_actions_for($gap):
    []
    + (if $gap.missing_adapter_fixture then [
      action(fixture_action_id($gap.source_surface_id); $gap.source_surface_id; "adapter_projection_fixture"; []; []; ["sourceSurfaceId", "nodeKind", "traceId", "fixtureHash"]; ["adapter_projection_fixture_missing"])
    ] else [] end)
    + (if $gap.missing_unified_store_adapter then [
      action(store_action_id($gap.source_surface_id); $gap.source_surface_id; "unified_store_projection"; store_collections($gap.source_surface_id); []; ["traceId", "nodeId", "sourceSurfaceId", "redactionState"]; ["unified_store_projection_missing"])
    ] else [] end)
    + (if $gap.missing_timeline_adapter then [
      action(timeline_action_id($gap.source_surface_id); $gap.source_surface_id; "observability_timeline_projection"; ["timelineEvents"]; timeline_events($gap.source_surface_id); ["traceId", "nodeId", "eventKind", "evidenceRefs"]; ["timeline_projection_missing"])
    ] else [] end)
    + (if $gap.missing_task_result_adapter then [
      action(task_result_action_id($gap.source_surface_id); $gap.source_surface_id; "task_result_projection"; ["taskResults"]; ["task_result_observed"]; ["taskId", "status", "summaryHash", "evidenceRefs", "traceId"]; ["task_result_projection_missing"])
    ] else [] end);
  def plan($id; $priority; $sources; $gaps; $coverage; $expected; $next): {
    id: $id,
    priority: $priority,
    source_surface_ids: $sources,
    closure_action_ids: ($sources | map(. as $source_id | ($gaps[] | select(.source_surface_id == $source_id) | .closure_action_ids[]))),
    closes_coverage_gap_ids: $coverage,
    expected_contract_ready_source_count_after_closure: $expected,
    next_gate: $next,
    mutates_runtime: false
  };
  def blocker($id; $severity; $affected; $fix): {
    id: $id,
    severity: $severity,
    affected_source_surface_ids: $affected,
    required_before_projection_enforcement: true,
    recommended_fix: $fix
  };
  def prior_gates: [
    "hepta_work_graph_contract_preview_gate",
    "hepta_work_graph_task_result_contract_preview_gate",
    "hepta_work_graph_scheduler_admission_controller_preview_gate",
    "hepta_work_graph_observability_timeline_preview_gate",
    "hepta_work_graph_role_manifest_contract_preview_gate",
    "hepta_work_graph_unified_state_store_preview_gate",
    "hepta_work_graph_adapter_projection_fixture_gate",
    "hepta_work_graph_unified_projection_audit_preview_gate",
    "hepta_work_graph_state_store_persistence_preview_gate",
    "hepta_work_graph_append_only_event_intake_preview_gate",
    "hepta_work_graph_replay_readback_preview_gate",
    "hepta_work_graph_idempotency_readback_adapter_preview_gate",
    "hepta_work_graph_unified_projection_enforcement_readiness_preview_gate"
  ];
  ($audit.source_surfaces | map(select((.has_unified_store_projection | not) or (.has_observability_timeline_projection | not) or (.requires_terminal_task_result and (.has_task_result_projection | not))) | source_gap(.))) as $source_gaps
  | ($source_gaps | map(closure_actions_for(.)) | add) as $closure_actions
  | [
    plan("planning_projection_adapter_gap_closure"; "P0"; ["update_plan_tool", "plan_mode_proposed_plan_blocks", "app_server_turn_plan_notification"]; $source_gaps; ["planning_identity_is_split_between_update_plan_and_plan_mode"]; 3; "hepta_work_graph_projection_adapter_gap_closure_readback_preview_gate"),
    plan("multi_agent_mailbox_projection_adapter_gap_closure"; "P0"; ["multi_agent_v2_mailbox_wait"]; $source_gaps; ["mailbox_wait_lacks_structured_task_result_join"]; 1; "hepta_work_graph_projection_adapter_gap_closure_readback_preview_gate"),
    plan("multi_agent_reducer_projection_adapter_gap_closure"; "P1"; ["hepta_runtime_multi_agent_reducer"]; $source_gaps; ["batch_and_worker_results_are_not_enforced_task_results"]; 1; "hepta_work_graph_terminal_task_result_wrapper_preview_gate"),
    plan("task_board_projection_adapter_gap_closure"; "P1"; ["hepta_runtime_task_board"]; $source_gaps; ["task_board_has_admission_shape_without_unified_store_projection"]; 1; "hepta_work_graph_scheduler_admission_controller_preview_gate"),
    plan("approval_broker_projection_adapter_gap_closure"; "P1"; ["hepta_runtime_approval_broker"]; $source_gaps; ["role_manifest_and_scheduler_admission_remain_preview_only"]; 1; "hepta_work_graph_state_store_persistence_preview_gate")
  ] as $closure_plans
  | [
    blocker("gap_closure_is_preview_only"; "medium"; ($source_gaps | map(.source_surface_id)); "keep closure as a read-only plan until the closure readback gate verifies the adapter shapes"),
    blocker("adapter_fixture_closure_not_applied"; "high"; ($source_gaps | map(select(.missing_adapter_fixture) | .source_surface_id)); "add fixture coverage for every newly closed source before treating reports as contract-ready"),
    blocker("unified_store_adapter_closure_not_applied"; "high"; ($source_gaps | map(select(.missing_unified_store_adapter) | .source_surface_id)); "add deterministic node, edge, taskResult, artifact, or approval collection mapping for each store gap"),
    blocker("timeline_adapter_closure_not_applied"; "high"; ($source_gaps | map(select(.missing_timeline_adapter) | .source_surface_id)); "add redacted timeline event mapping for each source that lacks observable trace events"),
    blocker("post_closure_enforcement_readiness_not_rerun"; "high"; ($source_gaps | map(.source_surface_id)); "rerun the unified projection enforcement-readiness gate after closure plans become adapter previews")
  ] as $blockers
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_projection_adapter_gap_closure_preview_gate",
      schema_version: "work_graph_projection_adapter_gap_closure_preview_v1",
      preview_mode: "read_only_projection_adapter_gap_closure_plan_no_runtime_attachment",
      source_gap_count: ($source_gaps | length),
      closure_action_count: ($closure_actions | length),
      store_adapter_closure_count: ($closure_actions | map(select(.adapter_kind == "unified_store_projection")) | length),
      timeline_adapter_closure_count: ($closure_actions | map(select(.adapter_kind == "observability_timeline_projection")) | length),
      adapter_fixture_closure_count: ($closure_actions | map(select(.adapter_kind == "adapter_projection_fixture")) | length),
      task_result_adapter_closure_count: ($closure_actions | map(select(.adapter_kind == "task_result_projection")) | length),
      closure_plan_count: ($closure_plans | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: (prior_gates | length),
      source_gaps: $source_gaps,
      closure_actions: $closure_actions,
      closure_plans: $closure_plans,
      blockers: $blockers,
      required_prior_gates: prior_gates,
      recommended_next_gate: "hepta_work_graph_projection_adapter_gap_closure_readback_preview_gate",
      ready_for_projection_adapter_gap_closure_readback_preview: true,
      ready_for_projection_enforcement: false,
      ready_for_append_only_store_enablement: false,
      ready_for_live_execution: false,
      source_probes: {
        gap_closure: {
          rust_module_present: $gap_closure_rust_module_present,
          report_script_present: $gap_closure_report_script_present,
          gate_script_present: $gap_closure_gate_script_present
        },
        upstream_reports: {
          unified_projection_audit: ($audit.gate == "hepta_work_graph_unified_projection_audit_preview_gate"),
          enforcement_readiness: ($readiness.gate == "hepta_work_graph_unified_projection_enforcement_readiness_preview_gate")
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        adapter_projection_enforced: false,
        closure_applied_to_runtime: false,
        append_only_store_enabled: false,
        scheduler_admission_enforced: false,
        task_result_enforcement_enabled: false,
        timeline_persisted: false,
        approval_recorded: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
