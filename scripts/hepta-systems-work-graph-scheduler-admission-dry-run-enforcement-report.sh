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
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_scheduler_admission_dry_run_enforcement.rs
)"
report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-dry-run-enforcement-report.sh
)"
gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-dry-run-enforcement-gate.sh
)"
task_result_envelope_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-task-result-envelope-report-only-validator-gate.sh
)"
adapter_task_result_index_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-adapter-task-result-index-gate.sh
)"
terminal_envelope_readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-envelope-readback-gate.sh
)"
source_id_alignment_readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-source-id-alignment-readback-gate.sh
)"
task_result_contract_field_gap_readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-task-result-contract-field-gap-readback-gate.sh
)"

jq -n \
  --argjson rust_module_present "$rust_module_present" \
  --argjson report_script_present "$report_script_present" \
  --argjson gate_script_present "$gate_script_present" \
  --argjson task_result_envelope_gate_script_present "$task_result_envelope_gate_script_present" \
  --argjson adapter_task_result_index_gate_script_present "$adapter_task_result_index_gate_script_present" \
  --argjson terminal_envelope_readback_gate_script_present "$terminal_envelope_readback_gate_script_present" \
  --argjson source_id_alignment_readback_gate_script_present "$source_id_alignment_readback_gate_script_present" \
  --argjson task_result_contract_field_gap_readback_gate_script_present "$task_result_contract_field_gap_readback_gate_script_present" \
  '
  def check($id; $evidence): {
    id: $id,
    blocks_live_execution: true,
    explanation_required: true,
    required_evidence_fields: $evidence
  };
  def entrypoint($id; $source; $kind; $position; $fields; $checks; $explain): {
    id: $id,
    source_surface_id: $source,
    entrypoint_kind: $kind,
    admission_position: $position,
    required_input_fields: $fields,
    applied_check_ids: $checks,
    explanation_output_fields: $explain,
    dry_run_enforcement_enabled: true,
    live_blocking_enforcement_enabled: false
  };
  def decision($id; $outcome; $allow; $blocked; $reason): {
    id: $id,
    outcome: $outcome,
    allow_entrypoint_to_continue: $allow,
    live_execution_blocked: $blocked,
    reason: $reason
  };
  def explanation($entry; $decision; $allow; $text; $trace): {
    entrypoint_id: $entry,
    decision_id: $decision,
    allow: $allow,
    explanation: $text,
    trace_id: $trace
  };
  [
    check("dependencies_terminal_ready"; ["depends_on", "dependency_statuses", "trace_id"]),
    check("lane_lease_available_and_owned"; ["lane_id", "lease_state", "owner_agent_id"]),
    check("approval_authority_present_when_required"; ["approval_id", "authority_state", "expiry_state"]),
    check("idempotency_replay_window_clear"; ["idempotency_key_hash", "readback_evidence_id"]),
    check("budget_and_timeout_available"; ["budget_state", "timeout_budget_ms", "attempt_count"]),
    check("task_result_contract_preview_present"; ["schema_version", "validator_ids"]),
    check("side_effect_boundary_locked"; ["preview_mode", "side_effects"])
  ] as $checks
  | ($checks | map(.id)) as $check_ids
  | ["decision", "reason", "failedChecks", "requiredEvidence", "taskResultPreview", "traceId"] as $explain_fields
  | [
    entrypoint("spawn_agent"; "multi_agent_v2_thread_spawn"; "tool"; "before spawn_agent calls agent_control.spawn_agent_with_metadata"; ["task_name", "agent_type", "model", "service_tier", "trace_id"]; $check_ids; $explain_fields),
    entrypoint("spawn_agents_on_csv"; "agent_jobs_batch_workers"; "tool"; "before CSV fanout creates/runs agent job items"; ["job_id", "csv_path", "max_concurrency", "max_runtime_seconds", "trace_id"]; $check_ids; $explain_fields),
    entrypoint("task_board_claim"; "hepta_runtime_task_board"; "runtime"; "before task board claim acquires or refreshes a lease"; ["task_id", "depends_on", "claim_token", "lease_expires_at", "trace_id"]; $check_ids; $explain_fields),
    entrypoint("worker_task_run"; "hepta_runtime_worker_tasks"; "runtime"; "before worker task run starts command, tool, or agent work"; ["task_id", "attempt_count", "timeout_budget_ms", "side_effect_class", "trace_id"]; $check_ids; $explain_fields)
  ] as $entrypoints
  | [
    decision("allow_dry_run"; "allow"; true; false; "all checks are satisfied for dry-run continuation; no live authority is granted"),
    decision("deny_dependencies_not_ready"; "deny"; false; true; "one or more blocking dependencies are missing or not terminal-ready"),
    decision("deny_lease_unavailable"; "deny"; false; true; "lane lease is missing, expired, or owned by another worker"),
    decision("deny_approval_missing"; "deny"; false; true; "approval is required for this risk class and no valid approval is attached"),
    decision("deny_idempotency_conflict"; "deny"; false; true; "idempotency readback indicates a duplicate or replay conflict"),
    decision("deny_budget_exhausted"; "deny"; false; true; "attempt, token, command, wall-clock, or concurrency budget is exhausted"),
    decision("deny_task_result_preview_missing"; "deny"; false; true; "entrypoint cannot run without a TaskResultEnvelope preview path"),
    decision("deny_side_effect_boundary_open"; "deny"; false; true; "requested side-effect class is not covered by an allowed boundary")
  ] as $decisions
  | [
    explanation("spawn_agent"; "allow_dry_run"; true; "spawn_agent may continue in dry-run-admitted mode with trace-bound TaskResult preview"; "trace-admission-spawn-agent-preview-001"),
    explanation("spawn_agents_on_csv"; "allow_dry_run"; true; "spawn_agents_on_csv may fan out only after budget, lease, idempotency, and result envelope checks pass"; "trace-admission-agent-jobs-preview-001"),
    explanation("task_board_claim"; "deny_lease_unavailable"; false; "task_board claim would be denied when the lane lease is absent or stale"; "trace-admission-task-board-preview-001"),
    explanation("worker_task_run"; "deny_side_effect_boundary_open"; false; "worker task run would be denied when the side-effect boundary is not locked"; "trace-admission-worker-task-preview-001")
  ] as $explanations
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate",
      schema_version: "work_graph_scheduler_admission_dry_run_enforcement_v1",
      preview_mode: "dry_run_allow_deny_explanation_before_entrypoint_no_live_blocking",
      entrypoint_count: ($entrypoints | length),
      check_count: ($checks | length),
      decision_count: ($decisions | length),
      explanation_count: ($explanations | length),
      entrypoints: $entrypoints,
      checks: $checks,
      decisions: $decisions,
      explanations: $explanations,
      required_prior_gates: [
        "hepta_work_graph_task_result_envelope_report_only_validator_gate",
        "hepta_work_graph_adapter_task_result_index_gate",
        "hepta_work_graph_terminal_envelope_readback_gate",
        "hepta_work_graph_source_id_alignment_readback_gate",
        "hepta_work_graph_task_result_contract_field_gap_readback_gate"
      ],
      recommended_next_gate: "hepta_work_graph_append_only_event_store_shadow_path_gate",
      dry_run_enforcement_enabled: true,
      live_blocking_enforcement_enabled: false,
      ready_for_append_only_event_store_shadow_path: true,
      ready_for_live_execution: false,
      source_probes: {
        scheduler_admission_dry_run_enforcement: {
          rust_module_present: $rust_module_present,
          report_script_present: $report_script_present,
          gate_script_present: $gate_script_present
        },
        task_result_envelope_report_only_validator: {
          gate_script_present: $task_result_envelope_gate_script_present
        },
        adapter_task_result_index: {
          gate_script_present: $adapter_task_result_index_gate_script_present
        },
        terminal_envelope_readback: {
          gate_script_present: $terminal_envelope_readback_gate_script_present
        },
        source_id_alignment_readback: {
          gate_script_present: $source_id_alignment_readback_gate_script_present
        },
        task_result_contract_field_gap_readback: {
          gate_script_present: $task_result_contract_field_gap_readback_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        live_admission_enforcement_enabled: false,
        lease_acquired: false,
        work_started: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
