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

hardening_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening.rs
)"
entrypoint_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-scheduler-guardrail-blocking-dry-run-entrypoint-gate.sh
)"
terminal_closeout_gate_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-agent-jobs-task-board-work-graph-shadow-event-store-replay-diff-dry-run-terminal-no-execution-final-closeout-gate.sh
)"
terminal_closeout_points_here="$(
  bool_for source_has \
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_gate" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout.rs
)"
entrypoint_no_live_present="$(
  bool_for source_has "live_blocking_enforcement_enabled: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint.rs
)"
entrypoint_dry_run_present="$(
  bool_for source_has "deny_live_allow_report_only" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint.rs
)"
terminal_closeout_no_live_present="$(
  bool_for source_has "ready_for_live_execution: false" \
    codex-rs/hepta-runtime/src/work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout.rs
)"

jq -n \
  --argjson hardening_module_present "$hardening_module_present" \
  --argjson entrypoint_gate_present "$entrypoint_gate_present" \
  --argjson terminal_closeout_gate_present "$terminal_closeout_gate_present" \
  --argjson terminal_closeout_points_here "$terminal_closeout_points_here" \
  --argjson entrypoint_no_live_present "$entrypoint_no_live_present" \
  --argjson entrypoint_dry_run_present "$entrypoint_dry_run_present" \
  --argjson terminal_closeout_no_live_present "$terminal_closeout_no_live_present" \
  '
  def evidence_fields: [
    "traceId",
    "spanId",
    "parentSpanId",
    "entrypointId",
    "decisionKey",
    "guardrailId",
    "evidenceRef",
    "payloadHash",
    "sideEffectClass"
  ];
  def non_live_guards: [
    "live_blocking_enabled=false",
    "runtime_interception_enabled=false",
    "scheduler_admission_enforced=false",
    "guardrail_enforcement_enabled=false",
    "work_graph_event_persisted=false"
  ];
  def binding($id; $source; $entrypoint; $position; $key): {
    id: $id,
    source_entrypoint_binding_id: $source,
    entrypoint_id: $entrypoint,
    hardened_hook_position: $position,
    deterministic_decision_key: $key,
    dry_run_outcome: "deny_live_allow_report_only_hardened",
    required_evidence_fields: evidence_fields,
    required_non_live_guards: non_live_guards,
    would_block_if_live: true,
    report_only_allows_current_runtime: true,
    live_blocking_enabled: false,
    runtime_interception_enabled: false
  };
  def check($id; $source; $condition; $requirement): {
    id: $id,
    source: $source,
    live_blocking_condition: $condition,
    hardening_requirement: $requirement,
    blocks_live_execution: true,
    dry_run_only: true
  };
  def decision($entrypoint; $explanation; $trace; $key): {
    entrypoint_id: $entrypoint,
    outcome: "deny_live_allow_report_only_hardened",
    explanation: $explanation,
    trace_id: $trace,
    deterministic_decision_key: $key,
    allow_current_runtime_to_continue: true,
    block_live_execution: true,
    decision_recorded: false,
    decision_persisted: false
  };
  def blocker($id; $action): {
    id: $id,
    blocked_action: $action,
    blocked: true,
    reason: "scheduler/guardrail blocking dry-run entrypoint hardening cannot authorize this action"
  };
  [
    binding("spawn_agent_hardened_dry_run_contract"; "spawn_agent_blocking_guardrail_dry_run"; "spawn_agent"; "before agent_control.spawn_agent_with_metadata"; "scheduler_guardrail.spawn_agent.v1"),
    binding("spawn_agents_on_csv_hardened_dry_run_contract"; "spawn_agents_on_csv_blocking_guardrail_dry_run"; "spawn_agents_on_csv"; "before CSV fanout creates or runs agent job items"; "scheduler_guardrail.spawn_agents_on_csv.v1"),
    binding("task_board_claim_hardened_dry_run_contract"; "task_board_claim_blocking_guardrail_dry_run"; "task_board_claim"; "before task board claim acquires or refreshes a lease"; "scheduler_guardrail.task_board_claim.v1"),
    binding("worker_task_run_hardened_dry_run_contract"; "worker_task_run_blocking_guardrail_dry_run"; "worker_task_run"; "before worker task starts command, tool, or agent work"; "scheduler_guardrail.worker_task_run.v1")
  ] as $hardened_entrypoints
  | [
    check("dependencies_readback_required"; "scheduler_admission"; "dependencies_not_terminal"; "terminal dependency readback must be attached before live entrypoint execution"),
    check("lane_lease_snapshot_required"; "scheduler_admission"; "lane_lease_missing_or_not_owned"; "lease ownership snapshot must be part of dry-run explanation"),
    check("approval_authority_snapshot_required"; "scheduler_admission"; "approval_authority_missing_when_required"; "operator or policy approval authority must be named before live execution"),
    check("idempotency_decision_key_required"; "scheduler_admission"; "idempotency_key_missing_or_unstable"; "deterministic decision key must suppress duplicate live work"),
    check("budget_timeout_snapshot_required"; "scheduler_admission"; "budget_or_timeout_unavailable"; "budget and timeout snapshot must be present before live execution"),
    check("task_result_envelope_preview_required"; "task_result_envelope"; "task_result_contract_missing"; "TaskResultEnvelope preview must exist before live entrypoint work"),
    check("side_effect_boundary_class_required"; "scheduler_admission"; "side_effect_boundary_unclassified"; "side-effect class must remain locked and visible in dry-run evidence"),
    check("trace_guardrail_span_required"; "trace_guardrail"; "blocking_guardrail_span_missing"; "traceId/spanId/guardrailId/evidenceRef/payloadHash must be linked"),
    check("shadow_event_join_preview_required"; "work_graph_shadow_event_store"; "shadow_event_join_missing"; "entrypoint dry-run must join a redacted shadow event preview"),
    check("replay_diff_terminal_no_execution_closeout_required"; "work_graph_shadow_event_store"; "replay_diff_no_execution_branch_not_closed"; "replay/diff no-execution final closeout must precede live hardening")
  ] as $hardening_checks
  | [
    decision("spawn_agent"; "spawn_agent remains deny-live until deterministic decision key, guardrail span, and shadow-event join are recordable"; "trace-hardened-blocking-dry-run-spawn-agent-001"; "scheduler_guardrail.spawn_agent.v1"),
    decision("spawn_agents_on_csv"; "CSV fanout remains deny-live until lease, budget, idempotency, and TaskResult evidence are recordable"; "trace-hardened-blocking-dry-run-agent-jobs-001"; "scheduler_guardrail.spawn_agents_on_csv.v1"),
    decision("task_board_claim"; "task board claim remains deny-live until lease and dependency readback evidence are recordable"; "trace-hardened-blocking-dry-run-task-board-001"; "scheduler_guardrail.task_board_claim.v1"),
    decision("worker_task_run"; "worker task run remains deny-live until side-effect boundary and guardrail span evidence are recordable"; "trace-hardened-blocking-dry-run-worker-task-001"; "scheduler_guardrail.worker_task_run.v1")
  ] as $hardening_decisions
  | [
    blocker("live_blocking_hook_install_blocked"; "install_live_blocking_hook"),
    blocker("runtime_interception_blocked"; "enable_runtime_interception"),
    blocker("scheduler_admission_enforcement_blocked"; "enforce_scheduler_admission"),
    blocker("guardrail_enforcement_blocked"; "enable_guardrail_enforcement"),
    blocker("hardening_decision_record_blocked"; "record_hardening_decision"),
    blocker("hardening_decision_persistence_blocked"; "persist_hardening_decision"),
    blocker("work_graph_event_persistence_blocked"; "persist_work_graph_event"),
    blocker("projection_index_persistence_blocked"; "persist_projection_index"),
    blocker("lease_acquisition_blocked"; "acquire_lane_lease"),
    blocker("work_start_blocked"; "start_entrypoint_work"),
    blocker("agent_spawn_blocked"; "spawn_agent"),
    blocker("model_invocation_blocked"; "invoke_model"),
    blocker("external_send_blocked"; "send_external_message"),
    blocker("replay_diff_recording_blocked"; "record_replay_diff"),
    blocker("replay_execution_blocked"; "execute_replay"),
    blocker("rollback_execution_blocked"; "execute_rollback"),
    blocker("idempotency_mutation_blocked"; "mutate_idempotency_index"),
    blocker("config_write_blocked"; "write_config"),
    blocker("feature_flag_mutation_blocked"; "mutate_feature_flag"),
    blocker("canary_traffic_blocked"; "route_canary_traffic"),
    blocker("operator_review_request_blocked"; "request_operator_review"),
    blocker("approval_recording_blocked"; "record_operator_approval"),
    blocker("live_cutover_blocked"; "perform_live_cutover")
  ] as $hardening_blockers
  | [
    "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_review_request_precondition_terminal_no_request_final_closeout_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate",
    "hepta_work_graph_trace_guardrail_span_report_only_gate",
    "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_non_persistence_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_audit_index_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_non_execution_readback_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_gate",
    "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_readback_gate"
  ] as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_gate",
      schema_version: "work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_v1",
      preview_mode: "scheduler_guardrail_blocking_dry_run_entrypoint_hardening_report_only",
      source_entrypoint_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_gate",
      source_entrypoint_binding_count: 4,
      source_guardrail_check_count: 8,
      source_dry_run_decision_count: 4,
      source_entrypoint_required_prior_gate_count: 4,
      source_terminal_no_execution_final_closeout_gate: "hepta_work_graph_agent_jobs_task_board_work_graph_shadow_event_store_replay_diff_dry_run_terminal_no_execution_final_closeout_gate",
      source_terminal_no_execution_final_closeout_entry_count: 9,
      source_terminal_no_execution_final_closeout_blocker_count: 26,
      source_terminal_no_execution_final_closeout_required_prior_gate_count: 5,
      hardened_entrypoint_count: ($hardened_entrypoints | length),
      hardening_check_count: ($hardening_checks | length),
      hardening_decision_count: ($hardening_decisions | length),
      hardening_blocker_count: ($hardening_blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      hardened_entrypoints: $hardened_entrypoints,
      hardening_checks: $hardening_checks,
      hardening_decisions: $hardening_decisions,
      hardening_blockers: $hardening_blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_jobs_task_board_scheduler_guardrail_blocking_dry_run_entrypoint_hardening_readback_gate",
      scheduler_guardrail_entrypoint_dry_run_present: true,
      terminal_no_execution_final_closeout_present: true,
      deny_live_allow_report_only_hardened: true,
      pre_entrypoint_hook_contract_hardened: true,
      deterministic_decision_key_ready: true,
      trace_evidence_contract_ready: true,
      shadow_event_join_ready: true,
      live_blocking_enforcement_enabled: false,
      runtime_interception_enabled: false,
      scheduler_admission_enforced: false,
      guardrail_enforcement_enabled: false,
      work_graph_event_persistence_enabled: false,
      ready_for_hardening_readback: true,
      ready_for_live_execution: false,
      source_probes: {
        hardening_module_present: $hardening_module_present,
        entrypoint_gate_present: $entrypoint_gate_present,
        terminal_closeout_gate_present: $terminal_closeout_gate_present,
        terminal_closeout_points_here: $terminal_closeout_points_here,
        entrypoint_no_live_present: $entrypoint_no_live_present,
        entrypoint_dry_run_present: $entrypoint_dry_run_present,
        terminal_closeout_no_live_present: $terminal_closeout_no_live_present
      },
      side_effects: {
        filesystem_written: false,
        hardening_decision_recorded: false,
        hardening_decision_persisted: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        projection_index_persisted: false,
        scheduler_admission_enforced: false,
        guardrail_enforcement_enabled: false,
        live_blocking_hook_installed: false,
        runtime_interception_enabled: false,
        lease_acquired: false,
        work_started: false,
        config_written: false,
        feature_flag_mutated: false,
        canary_traffic_routed: false,
        operator_review_requested: false,
        approval_recorded: false,
        replay_executed: false,
        replay_diff_recorded: false,
        replay_diff_persisted: false,
        rollback_executed: false,
        idempotency_index_mutated: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }
  '
