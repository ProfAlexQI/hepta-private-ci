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

task_result_contract_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_task_result_contract.rs
)"
task_result_contract_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-task-result-contract-preview-report.sh
)"
task_result_contract_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-task-result-contract-preview-gate.sh
)"
contract_preview_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_contract_preview.rs
)"
contract_preview_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-contract-preview-report.sh
)"
contract_preview_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-contract-preview-gate.sh
)"

jq -n \
  --argjson task_result_contract_rust_module_present "$task_result_contract_rust_module_present" \
  --argjson task_result_contract_report_script_present "$task_result_contract_report_script_present" \
  --argjson task_result_contract_gate_script_present "$task_result_contract_gate_script_present" \
  --argjson contract_preview_rust_module_present "$contract_preview_rust_module_present" \
  --argjson contract_preview_report_script_present "$contract_preview_report_script_present" \
  --argjson contract_preview_gate_script_present "$contract_preview_gate_script_present" \
  '
  def field($wire; $rust; $kind; $required; $terminal_required; $purpose): {
    wire_name: $wire,
    rust_name: $rust,
    field_kind: $kind,
    required: $required,
    terminal_required: $terminal_required,
    purpose: $purpose
  };
  def status($id; $terminal; $promotion; $evidence): {
    id: $id,
    terminal: $terminal,
    promotion_allowed: $promotion,
    requires_evidence: $evidence
  };
  def validator($id; $reason): {
    id: $id,
    required: true,
    reason: $reason
  };
  def adapter($source; $status_field; $node_kind; $covered; $blockers): {
    source_surface_id: $source,
    source_status_field: $status_field,
    projected_result_node_kind: $node_kind,
    covered_wire_fields: $covered,
    blocker_ids: $blockers,
    enforcement_enabled: false
  };
  [
    field("taskId"; "task_id"; "stable_id"; true; true; "stable task, agent, worker, or scheduler result identity"),
    field("status"; "status"; "enum"; true; true; "normalized lifecycle outcome for graph promotion"),
    field("summary"; "summary"; "string"; true; true; "operator-facing result summary with no secret payload"),
    field("artifacts"; "artifacts"; "array"; true; false; "artifact ids, content hashes, path hints, or external handoff references"),
    field("evidence"; "evidence"; "array"; true; true; "readback, command, gate, mailbox, or reducer evidence references"),
    field("risks"; "risks"; "array"; true; false; "known risks, blocked states, redaction notes, and review requirements"),
    field("nextActions"; "next_actions"; "array"; true; false; "follow-up actions for parent agents, operators, or schedulers"),
    field("verifier"; "verifier"; "object"; true; true; "verification gate identity, status, and report hash"),
    field("reducer"; "reducer"; "object"; true; false; "multi-agent reducer mode, decision, and consensus evidence"),
    field("usage"; "usage"; "object"; true; false; "model, tool, command, budget, and token usage accounting"),
    field("traceId"; "trace_id"; "stable_id"; true; true; "join key across plan, spawn, mailbox, tools, artifacts, gates, and result")
  ] as $required_fields
  | [
    status("queued"; false; false; false),
    status("running"; false; false; false),
    status("succeeded"; true; true; true),
    status("failed"; true; true; true),
    status("cancelled"; true; true; true),
    status("blocked"; true; false; true),
    status("superseded"; true; true; true)
  ] as $statuses
  | [
    validator("required_wire_fields_present"; "every TaskResult must include the canonical wire fields before it can be accepted"),
    validator("terminal_status_requires_summary_evidence_and_trace"; "terminal TaskResults must include summary, evidence, verifier, and traceId"),
    validator("artifact_reference_requires_identity_and_hash_or_path"; "artifact entries must be joinable without embedding raw payloads"),
    validator("risk_entry_requires_severity_reason_and_owner"; "risks must be actionable by parent agents, schedulers, or operators"),
    validator("verifier_reducer_and_usage_are_structured"; "gate, reducer, and budget information cannot be free-form text only"),
    validator("terminal_promotion_requires_no_secret_payload"; "summaries and evidence references must not expose raw secrets or private payloads"),
    validator("adapter_projection_is_preview_only"; "existing agent, worker, and scheduler result stores are only projected, not enforced")
  ] as $validators
  | [
    adapter("agent_jobs_batch_workers"; "AgentJobItem.status"; "worker_task"; ["taskId", "status", "summary", "evidence", "nextActions", "traceId"]; ["agent_job_result_json_is_not_task_result_schema"]),
    adapter("hepta_runtime_worker_tasks"; "WorkerTaskRecord.status"; "worker_task"; ["taskId", "status", "summary", "artifacts", "evidence", "risks", "nextActions", "usage", "traceId"]; ["worker_task_missing_verifier_and_reducer_projection"]),
    adapter("hepta_runtime_multi_agent_reducer"; "AgentRuntimeRunReport.reducer_passed"; "agent_task"; ["taskId", "status", "summary", "evidence", "risks", "reducer", "traceId"]; ["reducer_output_missing_task_result_wrapper"]),
    adapter("multi_agent_v2_thread_spawn"; "thread_spawn_edge.status"; "agent_task"; ["taskId", "status", "summary", "evidence", "traceId"]; ["thread_spawn_edge_missing_terminal_task_result"]),
    adapter("hepta_runtime_scheduler_store"; "SchedulerRunRecord.status"; "scheduler_run"; ["taskId", "status", "summary", "evidence", "risks", "nextActions", "traceId"]; ["scheduler_run_missing_task_result_projection"]),
    adapter("hepta_runtime_agent_harness"; "AgentHarnessRunRecord.status"; "external_handoff"; ["taskId", "status", "summary", "artifacts", "evidence", "risks", "traceId"]; ["agent_harness_ledger_missing_task_result_projection"])
  ] as $adapter_previews
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_task_result_contract_preview_gate",
      schema_version: "work_graph_task_result_contract_preview_v1",
      preview_mode: "validator_first_schema_preview_no_enforcement",
      required_field_count: ($required_fields | length),
      status_count: ($statuses | length),
      terminal_status_count: ($statuses | map(select(.terminal == true)) | length),
      validator_count: ($validators | length),
      adapter_preview_count: ($adapter_previews | length),
      required_fields: $required_fields,
      statuses: $statuses,
      validators: $validators,
      adapter_previews: $adapter_previews,
      recommended_next_gate: "hepta_work_graph_scheduler_admission_controller_preview_gate",
      ready_for_scheduler_admission_preview: true,
      ready_for_task_result_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        task_result_contract: {
          rust_module_present: $task_result_contract_rust_module_present,
          report_script_present: $task_result_contract_report_script_present,
          gate_script_present: $task_result_contract_gate_script_present
        },
        work_graph_contract_preview: {
          rust_module_present: $contract_preview_rust_module_present,
          report_script_present: $contract_preview_report_script_present,
          gate_script_present: $contract_preview_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        runtime_mutation_performed: false,
        scheduler_cutover_performed: false,
        task_result_enforcement_enabled: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
