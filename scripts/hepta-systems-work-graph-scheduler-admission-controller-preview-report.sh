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

scheduler_admission_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_scheduler_admission_controller.rs
)"
scheduler_admission_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-controller-preview-report.sh
)"
scheduler_admission_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-controller-preview-gate.sh
)"
task_result_contract_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_task_result_contract.rs
)"
task_result_contract_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-task-result-contract-preview-report.sh
)"
task_result_contract_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-task-result-contract-preview-gate.sh
)"

jq -n \
  --argjson scheduler_admission_rust_module_present "$scheduler_admission_rust_module_present" \
  --argjson scheduler_admission_report_script_present "$scheduler_admission_report_script_present" \
  --argjson scheduler_admission_gate_script_present "$scheduler_admission_gate_script_present" \
  --argjson task_result_contract_rust_module_present "$task_result_contract_rust_module_present" \
  --argjson task_result_contract_report_script_present "$task_result_contract_report_script_present" \
  --argjson task_result_contract_gate_script_present "$task_result_contract_gate_script_present" \
  '
  def check($id; $evidence; $reason): {
    id: $id,
    required: true,
    blocks_execution: true,
    required_evidence_fields: $evidence,
    reason: $reason
  };
  def decision($id; $runnable; $terminal; $reason): {
    id: $id,
    runnable_in_preview: $runnable,
    terminal_denial: $terminal,
    reason: $reason
  };
  def adapter($source; $node_kind; $source_fields; $check_ids; $blockers): {
    source_surface_id: $source,
    target_node_kind: $node_kind,
    source_fields: $source_fields,
    applied_check_ids: $check_ids,
    enforcement_enabled: false,
    blocker_ids: $blockers
  };
  [
    check("dependencies_terminal_ready"; ["depends_on", "dependency_statuses", "trace_id"]; "work cannot become runnable until all blocking dependencies are terminal-ready"),
    check("lane_lease_available_and_owned"; ["lane_id", "lease_state", "owner_agent_id"]; "scheduler handoff must not run without a lane-owned lease boundary"),
    check("approval_authority_present_when_required"; ["approval_id", "authority_state", "expiry_state"]; "high-risk or external handoff work needs explicit non-expired approval evidence"),
    check("idempotency_replay_window_clear"; ["idempotency_key_hash", "readback_evidence_id"]; "retries must not duplicate already observed work or delivery effects"),
    check("budget_and_timeout_available"; ["budget_state", "timeout_budget_ms", "attempt_count"]; "work must have remaining attempt, wall-clock, and resource budget"),
    check("task_result_contract_preview_present"; ["schema_version", "validator_ids"]; "terminal promotion requires the TaskResult contract preview to be available first"),
    check("side_effect_boundary_locked"; ["preview_mode", "side_effects"]; "this gate explains allow or deny decisions without acquiring leases or starting work")
  ] as $checks
  | ($checks | map(.id)) as $check_ids
  | [
    decision("allow_preview_only"; true; false; "all preconditions are satisfied for a dry-run explanation, not live execution"),
    decision("deny_blocked_dependency"; false; true; "one or more dependencies are missing, blocked, failed, or not terminal-ready"),
    decision("deny_missing_lane_lease"; false; true; "no lane-owned lease can be proven for the target work item"),
    decision("deny_missing_required_approval"; false; true; "required operator approval is missing, expired, superseded, or out of scope"),
    decision("deny_idempotency_conflict"; false; true; "readback evidence or idempotency key indicates a duplicate effect risk"),
    decision("deny_budget_or_timeout_exhausted"; false; true; "attempt, resource, token, or wall-clock budget is exhausted"),
    decision("deny_task_result_contract_missing"; false; true; "terminal work cannot be admitted if the TaskResult preview contract is absent")
  ] as $decisions
  | [
    adapter("hepta_runtime_scheduler_store"; "scheduler_run"; ["job_id", "run_id", "status", "idempotency_key", "readback_evidence_id"]; $check_ids; ["scheduler_run_admission_not_enforced"]),
    adapter("hepta_runtime_task_board"; "worker_task"; ["task_id", "status", "depends_on", "claim_token", "lease_expires_at"]; $check_ids; ["task_board_admission_not_enforced"]),
    adapter("hepta_runtime_worker_tasks"; "worker_task"; ["task_id", "status", "depends_on", "attempt_count", "timeout_budget_ms"]; $check_ids; ["worker_task_admission_not_enforced"]),
    adapter("multi_agent_v2_thread_spawn"; "agent_task"; ["agent_path", "thread_id", "parent_thread_id", "role_id"]; $check_ids; ["agent_task_admission_not_enforced"]),
    adapter("agent_jobs_batch_workers"; "worker_task"; ["job_id", "item_id", "assigned_thread_id", "attempt_count", "max_runtime_seconds"]; $check_ids; ["agent_job_item_admission_not_enforced"])
  ] as $adapter_previews
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_scheduler_admission_controller_preview_gate",
      schema_version: "work_graph_scheduler_admission_controller_preview_v1",
      preview_mode: "dry_run_admission_explain_only_no_execution",
      check_count: ($checks | length),
      decision_count: ($decisions | length),
      adapter_preview_count: ($adapter_previews | length),
      checks: $checks,
      decisions: $decisions,
      adapter_previews: $adapter_previews,
      recommended_next_gate: "hepta_work_graph_observability_timeline_preview_gate",
      ready_for_observability_timeline_preview: true,
      ready_for_scheduler_cutover: false,
      ready_for_live_execution: false,
      source_probes: {
        scheduler_admission_controller: {
          rust_module_present: $scheduler_admission_rust_module_present,
          report_script_present: $scheduler_admission_report_script_present,
          gate_script_present: $scheduler_admission_gate_script_present
        },
        task_result_contract: {
          rust_module_present: $task_result_contract_rust_module_present,
          report_script_present: $task_result_contract_report_script_present,
          gate_script_present: $task_result_contract_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        runtime_mutation_performed: false,
        scheduler_cutover_performed: false,
        admission_enforcement_enabled: false,
        lease_acquired: false,
        work_started: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
