#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

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

unified_projection_audit_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_unified_projection_audit_preview.rs
)"
unified_projection_audit_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-audit-preview-report.sh
)"
unified_projection_audit_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-audit-preview-gate.sh
)"
adapter_projection_fixture_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_adapter_projection_fixture.rs
)"
multi_agent_v2_surface_present="$(
  bool_for path_exists codex-rs/core/src/tools/handlers/multi_agents_v2.rs
)"
agent_jobs_surface_present="$(
  bool_for path_exists codex-rs/core/src/tools/handlers/agent_jobs.rs
)"
task_board_surface_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/task_board.rs
)"
worker_tasks_surface_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/worker_tasks.rs
)"
scheduler_store_surface_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/scheduler_store.rs
)"

jq -n \
  --argjson unified_projection_audit_rust_module_present "$unified_projection_audit_rust_module_present" \
  --argjson unified_projection_audit_report_script_present "$unified_projection_audit_report_script_present" \
  --argjson unified_projection_audit_gate_script_present "$unified_projection_audit_gate_script_present" \
  --argjson adapter_projection_fixture_rust_module_present "$adapter_projection_fixture_rust_module_present" \
  --argjson multi_agent_v2_surface_present "$multi_agent_v2_surface_present" \
  --argjson agent_jobs_surface_present "$agent_jobs_surface_present" \
  --argjson task_board_surface_present "$task_board_surface_present" \
  --argjson worker_tasks_surface_present "$worker_tasks_surface_present" \
  --argjson scheduler_store_surface_present "$scheduler_store_surface_present" \
  '
  def prior_gates: [
    "hepta_work_graph_contract_preview_gate",
    "hepta_work_graph_task_result_contract_preview_gate",
    "hepta_work_graph_scheduler_admission_controller_preview_gate",
    "hepta_work_graph_observability_timeline_preview_gate",
    "hepta_work_graph_role_manifest_contract_preview_gate",
    "hepta_work_graph_unified_state_store_preview_gate",
    "hepta_work_graph_adapter_projection_fixture_gate"
  ];
  def surface($id; $category; $kinds; $collections; $events; $fixture; $store; $task_result; $admission; $timeline; $role; $terminal; $state; $blockers; $next): {
    source_surface_id: $id,
    source_category: $category,
    projected_node_kinds: $kinds,
    projected_collection_ids: $collections,
    timeline_event_type_ids: $events,
    has_adapter_fixture: $fixture,
    has_unified_store_projection: $store,
    has_task_result_projection: $task_result,
    has_scheduler_admission_projection: $admission,
    has_observability_timeline_projection: $timeline,
    has_role_manifest_projection: $role,
    requires_terminal_task_result: $terminal,
    coverage_state: $state,
    blocker_ids: $blockers,
    next_projection_step: $next
  };
  def gap($id; $severity; $surfaces; $blockers; $fix): {
    id: $id,
    severity: $severity,
    source_surface_ids: $surfaces,
    blocker_ids: $blockers,
    recommended_fix: $fix
  };
  def next_cut($id; $priority; $gate; $purpose): {
    id: $id,
    priority: $priority,
    gate: $gate,
    purpose: $purpose,
    must_remain_side_effect_free: true
  };
  [
    surface("update_plan_tool"; "planning"; ["plan_step"]; ["edges", "nodes", "timelineEvents"]; []; true; true; false; false; false; false; false; "partial_projection_preview"; ["plan_step_store_projection_not_enforced", "timeline_projection_missing"]; "add_timeline_adapter_projection"),
    surface("plan_mode_proposed_plan_blocks"; "planning"; ["plan_step"]; ["nodes", "timelineEvents"]; []; true; false; false; false; false; false; false; "partial_projection_preview"; ["plan_mode_store_projection_missing", "timeline_projection_missing", "unified_store_projection_missing"]; "add_unified_store_adapter_projection"),
    surface("app_server_turn_plan_notification"; "planning"; ["plan_step"]; ["nodes", "timelineEvents"]; ["plan_step_observed"]; true; false; false; false; true; false; false; "timeline_only_preview"; ["turn_plan_notification_timeline_adapter_not_enforced", "unified_store_projection_missing"]; "add_unified_store_adapter_projection"),
    surface("multi_agent_v2_thread_spawn"; "multi_agent"; ["agent_task"]; ["edges", "nodes", "taskResults", "timelineEvents"]; ["agent_task_spawned"]; true; true; true; true; true; true; true; "contract_ready_preview"; ["agent_task_admission_not_enforced", "agent_task_store_projection_not_enforced", "agent_task_timeline_adapter_not_enforced", "multi_agent_v2_role_manifest_not_enforced", "thread_spawn_edge_missing_terminal_task_result"]; "keep_read_only_until_append_only_store_exists"),
    surface("multi_agent_v2_mailbox_wait"; "multi_agent"; ["agent_task"]; ["nodes", "timelineEvents"]; ["mailbox_progress_observed"]; true; false; false; false; true; false; false; "timeline_only_preview"; ["mailbox_progress_timeline_adapter_not_enforced", "unified_store_projection_missing"]; "add_unified_store_adapter_projection"),
    surface("hepta_runtime_multi_agent_reducer"; "multi_agent"; ["agent_task"]; ["nodes", "taskResults", "timelineEvents"]; []; true; false; true; false; false; false; true; "partial_projection_preview"; ["reducer_output_missing_task_result_wrapper", "timeline_projection_missing", "unified_store_projection_missing"]; "add_unified_store_adapter_projection"),
    surface("agent_jobs_batch_workers"; "batch_agent_jobs"; ["worker_task"]; ["nodes", "taskResults", "timelineEvents"]; ["agent_task_spawned", "task_result_observed"]; true; true; true; true; true; true; true; "contract_ready_preview"; ["agent_job_result_json_is_not_task_result_schema", "agent_job_store_projection_not_enforced", "agent_job_timeline_adapter_not_enforced", "agent_jobs_role_manifest_not_enforced", "agent_job_item_admission_not_enforced"]; "keep_read_only_until_append_only_store_exists"),
    surface("hepta_runtime_task_board"; "runtime_scheduler"; ["worker_task"]; ["nodes", "timelineEvents"]; []; true; false; false; true; false; false; false; "partial_projection_preview"; ["task_board_admission_not_enforced", "timeline_projection_missing", "unified_store_projection_missing"]; "add_unified_store_adapter_projection"),
    surface("hepta_runtime_worker_tasks"; "runtime_scheduler"; ["worker_task"]; ["artifacts", "nodes", "taskResults", "timelineEvents"]; ["artifact_produced", "task_result_observed", "tool_invocation_observed"]; true; true; true; true; true; true; true; "contract_ready_preview"; ["worker_task_admission_not_enforced", "worker_task_missing_verifier_and_reducer_projection", "worker_task_role_manifest_not_enforced", "worker_task_store_projection_not_enforced", "worker_task_timeline_adapter_not_enforced"]; "keep_read_only_until_append_only_store_exists"),
    surface("hepta_runtime_scheduler_store"; "runtime_scheduler"; ["scheduler_run"]; ["edges", "nodes", "taskResults", "timelineEvents"]; ["scheduler_admission_decision_observed"]; true; true; true; true; true; false; true; "contract_ready_preview"; ["scheduler_run_admission_not_enforced", "scheduler_run_missing_task_result_projection", "scheduler_store_projection_not_enforced", "scheduler_timeline_adapter_not_enforced"]; "keep_read_only_until_append_only_store_exists"),
    surface("hepta_runtime_approval_broker"; "operator_control"; ["human_approval"]; ["approvals", "nodes", "timelineEvents"]; []; true; false; false; false; false; false; false; "partial_projection_preview"; ["timeline_projection_missing", "unified_store_projection_missing"]; "add_unified_store_adapter_projection"),
    surface("hepta_runtime_agent_harness"; "external_handoff"; ["external_handoff"]; ["artifacts", "nodes", "taskResults", "timelineEvents"]; ["artifact_produced", "external_handoff_observed", "verification_gate_observed"]; true; true; true; false; true; true; true; "contract_ready_preview"; ["agent_harness_ledger_missing_task_result_projection", "agent_harness_role_manifest_not_enforced", "agent_harness_store_projection_not_enforced", "agent_harness_timeline_adapter_not_enforced"]; "keep_read_only_until_append_only_store_exists")
  ] as $source_surfaces
  | [
    gap("planning_identity_is_split_between_update_plan_and_plan_mode"; "high"; ["update_plan_tool", "plan_mode_proposed_plan_blocks", "app_server_turn_plan_notification"]; ["plan_step_store_projection_not_enforced", "plan_mode_store_projection_missing"]; "project both checklist updates and Plan Mode proposals into the same plan_step node namespace"),
    gap("mailbox_wait_lacks_structured_task_result_join"; "high"; ["multi_agent_v2_mailbox_wait", "multi_agent_v2_thread_spawn"]; ["mailbox_progress_timeline_adapter_not_enforced", "thread_spawn_edge_missing_terminal_task_result"]; "return WorkGraph mailbox event refs and terminal TaskResult refs from wait_agent"),
    gap("task_board_has_admission_shape_without_unified_store_projection"; "high"; ["hepta_runtime_task_board"]; ["task_board_admission_not_enforced", "unified_store_projection_missing"]; "add a task_board store adapter before it can be a schedulable source of truth"),
    gap("batch_and_worker_results_are_not_enforced_task_results"; "high"; ["agent_jobs_batch_workers", "hepta_runtime_worker_tasks"]; ["agent_job_result_json_is_not_task_result_schema", "worker_task_missing_verifier_and_reducer_projection"]; "wrap agent_jobs and worker_tasks completions in the TaskResult contract before terminal promotion"),
    gap("role_manifest_and_scheduler_admission_remain_preview_only"; "medium"; ["multi_agent_v2_thread_spawn", "agent_jobs_batch_workers", "hepta_runtime_worker_tasks", "hepta_runtime_scheduler_store"]; ["multi_agent_v2_role_manifest_not_enforced", "agent_task_admission_not_enforced", "scheduler_run_admission_not_enforced"]; "make role manifest and scheduler admission gates authoritative after the durable store exists")
  ] as $coverage_gaps
  | [
    next_cut("p0_projection_report_gate"; "P0"; "hepta_work_graph_unified_projection_audit_preview_gate"; "keep a single read-only audit view over planning, subagent, batch, worker, and scheduler surfaces"),
    next_cut("p1_append_only_store_events"; "P1"; "hepta_work_graph_state_store_persistence_preview_gate"; "promote projected nodes, edges, TaskResults, artifacts, approvals, and timeline events into append-only records"),
    next_cut("p2_scheduler_admission_cutover"; "P2"; "hepta_work_graph_scheduler_admission_controller_preview_gate"; "make dependency, lease, budget, role, approval, and side-effect checks the spawn and promotion authority"),
    next_cut("p3_structured_multi_agent_results"; "P3"; "hepta_work_graph_task_result_contract_preview_gate"; "make wait_agent, subagent completion notifications, reducers, and agent_jobs results return structured refs")
  ] as $next_cuts
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_unified_projection_audit_preview_gate",
      schema_version: "work_graph_unified_projection_audit_preview_v1",
      preview_mode: "read_only_cross_surface_projection_audit_no_persistence",
      source_surface_count: ($source_surfaces | length),
      source_category_count: ($source_surfaces | map(.source_category) | unique | length),
      projected_node_kind_count: ($source_surfaces | map(.projected_node_kinds[]) | unique | length),
      projected_collection_count: ($source_surfaces | map(.projected_collection_ids[]) | unique | length),
      required_prior_gate_count: (prior_gates | length),
      coverage_gap_count: ($coverage_gaps | length),
      next_cut_count: ($next_cuts | length),
      source_surfaces: $source_surfaces,
      required_prior_gates: prior_gates,
      coverage_gaps: $coverage_gaps,
      next_cuts: $next_cuts,
      recommended_next_gate: "hepta_work_graph_state_store_persistence_preview_gate",
      ready_for_state_store_persistence_preview: true,
      ready_for_store_persistence: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        unified_projection_audit: {
          rust_module_present: $unified_projection_audit_rust_module_present,
          report_script_present: $unified_projection_audit_report_script_present,
          gate_script_present: $unified_projection_audit_gate_script_present
        },
        adapter_projection_fixture: {
          rust_module_present: $adapter_projection_fixture_rust_module_present
        },
        source_surfaces: {
          multi_agent_v2_present: $multi_agent_v2_surface_present,
          agent_jobs_present: $agent_jobs_surface_present,
          task_board_present: $task_board_surface_present,
          worker_tasks_present: $worker_tasks_surface_present,
          scheduler_store_present: $scheduler_store_surface_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        store_persistence_enabled: false,
        runtime_mutation_performed: false,
        scheduler_admission_enforced: false,
        task_result_enforcement_enabled: false,
        role_manifest_enforcement_enabled: false,
        approval_recorded: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
