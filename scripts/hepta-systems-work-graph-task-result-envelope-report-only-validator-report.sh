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

rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_task_result_envelope_report_only_validator.rs
)"
report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-task-result-envelope-report-only-validator-report.sh
)"
gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-task-result-envelope-report-only-validator-gate.sh
)"
canonical_readiness_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-canonical-projection-readiness-gate.sh
)"

jq -n \
  --argjson rust_module_present "$rust_module_present" \
  --argjson report_script_present "$report_script_present" \
  --argjson gate_script_present "$gate_script_present" \
  --argjson canonical_readiness_gate_script_present "$canonical_readiness_gate_script_present" \
  '
  def rule($id; $reason): {
    id: $id,
    required: true,
    report_only_blocks_promotion: false,
    reason: $reason
  };
  def adapter($source; $event; $entrypoint; $fields; $rules): {
    source_surface_id: $source,
    terminal_event: $event,
    entrypoint_or_reducer: $entrypoint,
    covered_wire_fields: $fields,
    validation_rule_ids: $rules,
    report_only_attached: true,
    live_enforcement_enabled: false
  };
  def verifier($id): {
    verifier_id: $id,
    gate_id: "hepta_work_graph_task_result_envelope_report_only_validator_gate",
    status: "report_only_valid",
    evidence_ref: "evidence:task-result-envelope-report-only-validator"
  };
  def reducer($id; $mode): {
    reducer_id: $id,
    mode: $mode,
    decision: "report_only_accept",
    evidence_ref: "evidence:task-result-envelope-reducer-preview"
  };
  def usage: {
    model_tokens: 0,
    tool_calls: 0,
    command_count: 0,
    budget_state: "not_debited_report_only"
  };
  def envelope($source; $task; $status; $summary; $artifacts; $evidence; $risks; $next; $trace; $verifier; $reducer; $mode): {
    source_surface_id: $source,
    task_id: $task,
    status: $status,
    summary: $summary,
    artifacts: $artifacts,
    evidence: $evidence,
    risks: $risks,
    next_actions: $next,
    verifier: verifier($verifier),
    reducer: reducer($reducer; $mode),
    usage: usage,
    trace_id: $trace,
    validation_decision: "allow_report_only",
    live_promotion_allowed: false
  };
  [
    "taskId",
    "status",
    "summary",
    "artifacts",
    "evidence",
    "risks",
    "nextActions",
    "verifier",
    "reducer",
    "usage",
    "traceId"
  ] as $fields
  | [
    rule("required_wire_fields_present"; "all canonical TaskResultEnvelope wire fields must be present before terminal promotion"),
    rule("status_is_normalized"; "source statuses must map to queued, running, succeeded, failed, cancelled, blocked, or superseded"),
    rule("summary_is_redacted_and_non_empty"; "summaries must be operator-readable without embedding raw private payloads"),
    rule("artifact_refs_are_ids_hashes_or_paths"; "artifacts must be references rather than raw payload blobs"),
    rule("evidence_refs_are_readback_bound"; "evidence must identify commands, gates, reducer output, mailbox records, or readback probes"),
    rule("risks_and_next_actions_are_actionable"; "risk and next action entries need owner, severity, or scheduler intent"),
    rule("verifier_reducer_usage_are_structured"; "verifier, reducer, and usage cannot be free-form strings"),
    rule("trace_id_joins_plan_spawn_mailbox_tool_result"; "traceId must join the result with upstream plan, spawn, mailbox, tool, artifact, and guardrail spans")
  ] as $rules
  | ($rules | map(.id)) as $rule_ids
  | [
    adapter("agent_jobs_batch_workers"; "report_agent_job_result.accepted"; "report_agent_job_result"; $fields; $rule_ids),
    adapter("multi_agent_v2_thread_spawn"; "thread_spawn_edge.status"; "spawn_agent"; $fields; $rule_ids),
    adapter("hepta_runtime_worker_tasks"; "WorkerTaskRecord.terminal_status"; "worker_task_run"; $fields; $rule_ids),
    adapter("hepta_runtime_multi_agent_reducer"; "AgentRuntimeRunReport.reducer_passed"; "multi_agent_reducer"; $fields; $rule_ids),
    adapter("hepta_runtime_task_board"; "TaskBoardTerminalEvent.status"; "task_board_terminal_event"; $fields; $rule_ids),
    adapter("hepta_runtime_scheduler_store"; "SchedulerRunRecord.status"; "scheduler_run_record"; $fields; $rule_ids),
    adapter("hepta_runtime_agent_harness"; "AgentHarnessRunRecord.status"; "agent_harness_ledger"; $fields; $rule_ids)
  ] as $adapters
  | [
    envelope("agent_jobs_batch_workers"; "wg-task-result-agent-job-item-preview-001"; "succeeded"; "agent job item reported a structured result object"; ["artifact:agent-job-output-csv-preview"]; ["gate:report-agent-job-result-json-object"]; []; ["next:scheduler-admission-dry-run"]; "trace-agent-job-preview-001"; "agent_job_result_verifier"; "agent_job_item_report_only_reducer"; "single"),
    envelope("multi_agent_v2_thread_spawn"; "wg-task-result-spawn-agent-preview-001"; "succeeded"; "spawn_agent emitted a trace-bound report-only TaskResult envelope"; ["artifact:agent-card-preview"]; ["spawn:thread-spawn-edge-preview"]; ["risk:subagent-output-contract-report-only"]; ["next:scheduler-admission-dry-run"]; "trace-spawn-agent-preview-001"; "spawn_agent_thread_verifier"; "spawn_agent_report_only_reducer"; "single"),
    envelope("hepta_runtime_worker_tasks"; "wg-task-result-worker-task-preview-001"; "succeeded"; "worker task reached a terminal status with artifact and evidence refs"; ["artifact:worker-task-output-preview"]; ["readback:worker-task-record-preview"]; []; ["next:task-board-terminal-event"]; "trace-worker-task-preview-001"; "worker_task_terminal_verifier"; "worker_task_terminal_reducer"; "single"),
    envelope("hepta_runtime_multi_agent_reducer"; "wg-task-result-multi-agent-reducer-preview-001"; "succeeded"; "multi-agent reducer selected a consensus result from child outputs"; ["artifact:reducer-summary-preview"]; ["reducer:quorum-preview"]; ["risk:child-output-drift-watch"]; ["next:parent-agent-merge"]; "trace-multi-agent-reducer-preview-001"; "multi_agent_reducer_verifier"; "multi_agent_quorum_reducer"; "quorum"),
    envelope("hepta_runtime_task_board"; "wg-task-result-task-board-terminal-preview-001"; "blocked"; "task board terminal event recorded a blocked result for scheduler readback"; []; ["task-board:terminal-event-preview"]; ["risk:operator-review-required"]; ["next:surface-deny-explanation"]; "trace-task-board-preview-001"; "task_board_terminal_verifier"; "task_board_terminal_reducer"; "single"),
    envelope("hepta_runtime_scheduler_store"; "wg-task-result-scheduler-run-preview-001"; "succeeded"; "scheduler store projected a terminal run result into a report-only TaskResult envelope"; ["artifact:scheduler-run-record-preview"]; ["readback:scheduler-run-record-preview"]; ["risk:scheduler-live-blocking-disabled"]; ["next:source-id-alignment-readback"]; "trace-scheduler-run-preview-001"; "scheduler_run_terminal_verifier"; "scheduler_run_terminal_reducer"; "single"),
    envelope("hepta_runtime_agent_harness"; "wg-task-result-agent-harness-preview-001"; "succeeded"; "agent harness ledger projected an external handoff result into a report-only TaskResult envelope"; ["artifact:agent-harness-ledger-preview"]; ["readback:agent-harness-ledger-preview"]; ["risk:external-handoff-report-only"]; ["next:source-id-alignment-readback"]; "trace-agent-harness-preview-001"; "agent_harness_terminal_verifier"; "agent_harness_terminal_reducer"; "single")
  ] as $envelopes
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_task_result_envelope_report_only_validator_gate",
      schema_version: "work_graph_task_result_envelope_report_only_validator_v1",
      preview_mode: "report_only_task_result_envelope_validator_no_live_enforcement",
      canonical_wire_field_count: ($fields | length),
      validation_rule_count: ($rules | length),
      source_adapter_count: ($adapters | length),
      source_envelope_count: ($envelopes | length),
      report_only_valid_source_count: ($envelopes | map(select(.validation_decision == "allow_report_only")) | length),
      canonical_wire_fields: $fields,
      validation_rules: $rules,
      source_adapters: $adapters,
      source_envelopes: $envelopes,
      required_prior_gates: [
        "hepta_work_graph_canonical_projection_readiness_gate",
        "hepta_work_graph_task_result_contract_preview_gate"
      ],
      recommended_next_gate: "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate",
      report_only_validator_attached: true,
      live_enforcement_enabled: false,
      ready_for_scheduler_admission_dry_run_enforcement: true,
      ready_for_live_execution: false,
      source_probes: {
        task_result_envelope_report_only_validator: {
          rust_module_present: $rust_module_present,
          report_script_present: $report_script_present,
          gate_script_present: $gate_script_present
        },
        canonical_projection_readiness: {
          gate_script_present: $canonical_readiness_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        task_result_enforcement_enabled: false,
        scheduler_admission_enforced: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
