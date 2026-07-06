#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

path_exists() {
  local path="$1"
  [[ -e "$path" ]]
}

source_has() {
  local pattern="$1"
  local path="$2"
  rg -q "$pattern" "$path"
}

bool_for() {
  if "$@"; then
    printf 'true\n'
  else
    printf 'false\n'
  fi
}

entrypoint_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint.rs
)"
final_closeout_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-operator-review-request-precondition-terminal-no-request-final-closeout-gate.sh
)"
final_closeout_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout.rs
)"
scheduler_dry_run_present="$(
  bool_for source_has "WORK_GRAPH_SCHEDULER_ADMISSION_DRY_RUN_ENFORCEMENT_GATE" \
    codex-rs/hepta-runtime/src/work_graph_scheduler_admission_dry_run_enforcement.rs
)"
trace_guardrail_present="$(
  bool_for source_has "WORK_GRAPH_TRACE_GUARDRAIL_SPAN_REPORT_ONLY_GATE" \
    codex-rs/hepta-runtime/src/work_graph_trace_guardrail_span_report_only.rs
)"
entrypoint_emission_present="$(
  bool_for source_has "WORK_GRAPH_AGENT_JOBS_TASK_BOARD_REPORT_ONLY_ENTRYPOINT_EMISSION_GATE" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_report_only_entrypoint_emission.rs
)"

jq -n \
  --argjson entrypoint_module_present "$entrypoint_module_present" \
  --argjson final_closeout_gate_present "$final_closeout_gate_present" \
  --argjson final_closeout_points_here "$final_closeout_points_here" \
  --argjson scheduler_dry_run_present "$scheduler_dry_run_present" \
  --argjson trace_guardrail_present "$trace_guardrail_present" \
  --argjson entrypoint_emission_present "$entrypoint_emission_present" \
  '
  def binding($id; $surface; $entrypoint; $position): {
    id: $id,
    source_surface_id: $surface,
    entrypoint_id: $entrypoint,
    hook_position: $position,
    dry_run_decision: "deny_live_allow_report_only",
    applied_check_ids: [
      "dependencies_terminal_ready",
      "lane_lease_available_and_owned",
      "approval_authority_present_when_required",
      "idempotency_replay_window_clear",
      "budget_and_timeout_available",
      "task_result_contract_preview_present",
      "side_effect_boundary_locked",
      "trace_guardrail_span_present"
    ],
    required_trace_fields: [
      "traceId",
      "spanId",
      "parentSpanId",
      "guardrailId",
      "evidenceRef",
      "payloadHash"
    ],
    would_block_if_live: true,
    dry_run_allows_current_runtime_to_continue: true,
    live_blocking_enabled: false
  };
  def check($id; $source): {
    id: $id,
    source: $source,
    blocks_live_execution: true,
    dry_run_explanation_required: true
  };
  def decision($entrypoint; $reason; $trace): {
    entrypoint_id: $entrypoint,
    outcome: "deny_live_allow_report_only",
    reason: $reason,
    trace_id: $trace,
    allow_current_runtime_to_continue: true,
    block_live_execution: true
  };
  [
    binding("spawn_agent_blocking_guardrail_dry_run"; "multi_agent_v2_thread_spawn"; "spawn_agent"; "before agent_control.spawn_agent_with_metadata"),
    binding("spawn_agents_on_csv_blocking_guardrail_dry_run"; "agent_jobs_batch_workers"; "spawn_agents_on_csv"; "before CSV fanout creates or runs agent job items"),
    binding("task_board_claim_blocking_guardrail_dry_run"; "hepta_runtime_task_board"; "task_board_claim"; "before task board claim acquires or refreshes a lease"),
    binding("worker_task_run_blocking_guardrail_dry_run"; "hepta_runtime_worker_tasks"; "worker_task_run"; "before worker task starts command, tool, or agent work")
  ] as $entrypoint_bindings
  | [
    check("dependencies_terminal_ready"; "scheduler_admission"),
    check("lane_lease_available_and_owned"; "scheduler_admission"),
    check("approval_authority_present_when_required"; "scheduler_admission"),
    check("idempotency_replay_window_clear"; "scheduler_admission"),
    check("budget_and_timeout_available"; "scheduler_admission"),
    check("task_result_contract_preview_present"; "task_result_envelope"),
    check("side_effect_boundary_locked"; "scheduler_admission"),
    check("trace_guardrail_span_present"; "trace_guardrail")
  ] as $guardrail_checks
  | [
    decision("spawn_agent"; "spawn_agent would require blocking guardrail approval before live spawn"; "trace-blocking-dry-run-spawn-agent-001"),
    decision("spawn_agents_on_csv"; "CSV fanout would require lease, budget, idempotency, and TaskResult evidence before live execution"; "trace-blocking-dry-run-agent-jobs-001"),
    decision("task_board_claim"; "task board claim would require owned lease and dependency readback before live claim"; "trace-blocking-dry-run-task-board-001"),
    decision("worker_task_run"; "worker task run would require side-effect boundary and guardrail span before live work"; "trace-blocking-dry-run-worker-task-001")
  ] as $dry_run_decisions
  | [
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate",
    "hepta_work_graph_trace_guardrail_span_report_only_gate",
    "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate"
  ] as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_v1",
      preview_mode: "blocking_guardrail_dry_run_before_entrypoint_no_live_enforcement",
      source_scheduler_gate: "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate",
      source_scheduler_entrypoint_count: 4,
      source_scheduler_check_count: 7,
      source_trace_guardrail_gate: "hepta_work_graph_trace_guardrail_span_report_only_gate",
      source_trace_span_count: 9,
      source_blocking_guardrail_count: 6,
      source_entrypoint_emission_gate: "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate",
      source_emission_count: 2,
      source_final_closeout_gate: "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_gate",
      source_final_closeout_entry_count: 8,
      entrypoint_binding_count: ($entrypoint_bindings | length),
      guardrail_check_count: ($guardrail_checks | length),
      dry_run_decision_count: ($dry_run_decisions | length),
      required_prior_gate_count: ($required_prior_gates | length),
      entrypoint_bindings: $entrypoint_bindings,
      guardrail_checks: $guardrail_checks,
      dry_run_decisions: $dry_run_decisions,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate",
      scheduler_admission_dry_run_present: true,
      blocking_guardrail_dry_run_attached: true,
      pre_entrypoint_hook_contract_ready: true,
      live_blocking_enforcement_enabled: false,
      runtime_interception_enabled: false,
      work_graph_event_persistence_enabled: false,
      ready_for_work_graph_shadow_event_store_readback: true,
      ready_for_live_execution: false,
      source_probes: {
        entrypoint_module_present: $entrypoint_module_present,
        final_closeout_gate_present: $final_closeout_gate_present,
        final_closeout_points_here: $final_closeout_points_here,
        scheduler_dry_run_present: $scheduler_dry_run_present,
        trace_guardrail_present: $trace_guardrail_present,
        entrypoint_emission_present: $entrypoint_emission_present
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        scheduler_admission_enforced: false,
        guardrail_enforcement_enabled: false,
        live_blocking_hook_installed: false,
        lease_acquired: false,
        work_started: false,
        config_written: false,
        feature_flag_mutated: false,
        canary_traffic_routed: false,
        operator_review_requested: false,
        approval_recorded: false,
        replay_executed: false,
        rollback_executed: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }
  '
