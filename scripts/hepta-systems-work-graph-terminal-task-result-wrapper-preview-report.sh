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

terminal_wrapper_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_terminal_task_result_wrapper_preview.rs
)"
terminal_wrapper_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-task-result-wrapper-preview-report.sh
)"
terminal_wrapper_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-task-result-wrapper-preview-gate.sh
)"
idempotency_adapter_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_idempotency_readback_adapter_preview.rs
)"
idempotency_adapter_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-idempotency-readback-adapter-preview-gate.sh
)"
task_result_contract_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_task_result_contract.rs
)"
append_only_event_intake_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-event-intake-preview-gate.sh
)"

jq -n \
  --argjson terminal_wrapper_rust_module_present "$terminal_wrapper_rust_module_present" \
  --argjson terminal_wrapper_report_script_present "$terminal_wrapper_report_script_present" \
  --argjson terminal_wrapper_gate_script_present "$terminal_wrapper_gate_script_present" \
  --argjson idempotency_adapter_rust_module_present "$idempotency_adapter_rust_module_present" \
  --argjson idempotency_adapter_gate_script_present "$idempotency_adapter_gate_script_present" \
  --argjson task_result_contract_rust_module_present "$task_result_contract_rust_module_present" \
  --argjson append_only_event_intake_gate_script_present "$append_only_event_intake_gate_script_present" \
  '
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
    "hepta_work_graph_idempotency_readback_adapter_preview_gate"
  ];
  def fields: [
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
  ];
  def terminal_fields: ["taskId", "status", "summary", "evidence", "verifier", "traceId"];
  def mappings: [
    {source_status: "success", canonical_status: "succeeded", terminal: true, promotion_allowed: true},
    {source_status: "ok", canonical_status: "succeeded", terminal: true, promotion_allowed: true},
    {source_status: "failed", canonical_status: "failed", terminal: true, promotion_allowed: true},
    {source_status: "error", canonical_status: "failed", terminal: true, promotion_allowed: true},
    {source_status: "cancelled", canonical_status: "cancelled", terminal: true, promotion_allowed: true},
    {source_status: "blocked", canonical_status: "blocked", terminal: true, promotion_allowed: false},
    {source_status: "superseded", canonical_status: "superseded", terminal: true, promotion_allowed: true}
  ];
  def wrapper($id; $source; $kind; $field; $node; $key; $evidence; $redaction): {
    id: $id,
    source_surface_id: $source,
    terminal_source_kind: $kind,
    source_terminal_field: $field,
    emitted_event_contract_id: "task_result_event_intake",
    task_result_node_kind: $node,
    replay_key_contract_id: $key,
    required_wire_fields: fields,
    terminal_required_fields: terminal_fields,
    evidence_contract_ids: $evidence,
    canonical_status_mappings: mappings,
    wrapper_state: "preview_contract_defined_wrapper_execution_disabled",
    redaction_policy: $redaction,
    attaches_runtime_adapter: false,
    executes_wrapper: false,
    persists_task_result: false,
    enforces_task_result: false,
    mutates_store: false
  };
  def evidence($id; $source; $evidence_fields; $verifier_fields): {
    id: $id,
    source_surface_id: $source,
    evidence_ref_fields: $evidence_fields,
    verifier_ref_fields: $verifier_fields,
    redaction_policy: "store ids, hashes, schema refs, and verifier refs only",
    stores_raw_payload: false,
    performs_readback: false,
    mutates_store: false
  };
  def blocker($id; $severity; $sources; $fix): {
    id: $id,
    severity: $severity,
    affected_source_surface_ids: $sources,
    required_before_task_result_enforcement: true,
    recommended_fix: $fix
  };
  [
    wrapper("multi_agent_thread_spawn_terminal_task_result_wrapper"; "multi_agent_v2_thread_spawn"; "multi_agent_spawn"; "thread_spawn_edge.status"; "agent_task"; "multi_agent_spawn_projection_idempotency"; ["thread_spawn_completion_evidence"]; "hash spawn prompt and expose only parent/child thread ids, role id, agent path, and evidence refs"),
    wrapper("multi_agent_mailbox_wait_terminal_task_result_wrapper"; "multi_agent_v2_mailbox_wait"; "mailbox_delivery"; "mailbox_wait.deliveryState"; "agent_task"; "multi_agent_mailbox_delivery_replay_key"; ["mailbox_wait_delivery_evidence"]; "hash mailbox payload and expose only mailbox seq, agent path, delivery state, and evidence refs"),
    wrapper("multi_agent_reducer_terminal_task_result_wrapper"; "hepta_runtime_multi_agent_reducer"; "multi_agent_reducer"; "AgentRuntimeRunReport.reducer_passed"; "agent_task"; "multi_agent_reducer_task_result_replay_key"; ["reducer_consensus_evidence"]; "hash subagent outputs and expose only reducer strategy, decision, status, and evidence refs"),
    wrapper("agent_job_item_terminal_task_result_wrapper"; "agent_jobs_batch_workers"; "batch_agent_job_item"; "AgentJobItem.status"; "worker_task"; "agent_job_result_projection_idempotency"; ["agent_job_result_schema_evidence"]; "hash worker JSON result and expose only schema ref, job id, item id, attempt, and evidence refs"),
    wrapper("worker_task_terminal_task_result_wrapper"; "hepta_runtime_worker_tasks"; "worker_task"; "WorkerTaskRecord.status"; "worker_task"; "worker_task_projection_idempotency"; ["worker_task_artifact_gate_evidence"]; "hash command output and expose only artifact refs, verifier refs, lane, attempt, and status"),
    wrapper("task_board_terminal_task_result_wrapper"; "hepta_runtime_task_board"; "task_board_worker_task"; "TaskBoardRecord.status"; "worker_task"; "task_board_worker_task_replay_key"; ["task_board_lease_readback_evidence"]; "hash task board payload and expose only lease state, artifact hash, lane, and evidence refs"),
    wrapper("scheduler_run_terminal_task_result_wrapper"; "hepta_runtime_scheduler_store"; "scheduler_run"; "SchedulerRunRecord.status"; "scheduler_run"; "scheduler_run_projection_idempotency"; ["scheduler_admission_decision_evidence"]; "hash scheduler decision inputs and expose only lease id, admission decision, status, and evidence refs"),
    wrapper("agent_harness_terminal_task_result_wrapper"; "hepta_runtime_agent_harness"; "agent_harness_handoff"; "AgentHarnessRunRecord.status"; "external_handoff"; "agent_harness_handoff_projection_idempotency"; ["agent_harness_handoff_evidence"]; "hash harness payload and expose only handoff refs, artifact refs, verifier refs, and redaction state")
  ] as $terminal_wrappers
  | ($terminal_wrappers | map(.source_surface_id)) as $sources
  | [
    evidence("thread_spawn_completion_evidence"; "multi_agent_v2_thread_spawn"; ["parentThreadId", "childThreadId", "agentPath", "roleId"]; ["roleManifestGate", "spawnProjectionGate"]),
    evidence("mailbox_wait_delivery_evidence"; "multi_agent_v2_mailbox_wait"; ["mailboxSeq", "agentPath", "deliveryState", "timelineEventRef"]; ["mailboxReadbackProbe", "deliveryStateGate"]),
    evidence("reducer_consensus_evidence"; "hepta_runtime_multi_agent_reducer"; ["reducerStrategy", "decisionHash", "participantCount", "evidenceHash"]; ["reducerVerifier", "taskResultContractGate"]),
    evidence("agent_job_result_schema_evidence"; "agent_jobs_batch_workers"; ["jobId", "itemId", "attempt", "resultSchemaRef"]; ["agentJobSchemaGate", "taskResultContractGate"]),
    evidence("worker_task_artifact_gate_evidence"; "hepta_runtime_worker_tasks"; ["workerTaskId", "attempt", "artifactHash", "gateReportHash"]; ["workerTaskVerifier", "artifactRedactionGate"]),
    evidence("task_board_lease_readback_evidence"; "hepta_runtime_task_board"; ["workerTaskId", "lane", "leaseState", "artifactHash"]; ["taskBoardLeaseGate", "idempotencyReadbackAdapterGate"]),
    evidence("scheduler_admission_decision_evidence"; "hepta_runtime_scheduler_store"; ["schedulerRunId", "leaseId", "admissionDecision", "decisionHash"]; ["schedulerAdmissionGate", "roleManifestGate"]),
    evidence("agent_harness_handoff_evidence"; "hepta_runtime_agent_harness"; ["harnessRunId", "handoffRef", "artifactHash", "redactionState"]; ["agentHarnessVerifier", "artifactRedactionGate"])
  ] as $evidence_contracts
  | [
    blocker("wrapper_fixture_execution_disabled"; "high"; $sources; "run fixture-only wrapping checks before any runtime adapter executes these wrappers"),
    blocker("terminal_task_result_enforcement_disabled"; "high"; $sources; "keep TaskResult validation preview-only until fixtures prove every terminal source maps canonical fields"),
    blocker("append_only_store_still_disabled"; "medium"; $sources; "do not persist TaskResult records until replay/readback fixtures are deterministic"),
    blocker("scheduler_admission_consumes_preview_only"; "medium"; ["hepta_runtime_scheduler_store", "hepta_runtime_task_board", "hepta_runtime_worker_tasks"]; "scheduler admission must keep reading preview contracts until TaskResult enforcement is explicitly enabled")
  ] as $blockers
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_terminal_task_result_wrapper_preview_gate",
      schema_version: "work_graph_terminal_task_result_wrapper_preview_v1",
      preview_mode: "read_only_terminal_task_result_wrapper_preview_no_execution",
      terminal_wrapper_count: ($terminal_wrappers | length),
      terminal_source_count: ($sources | length),
      canonical_wire_field_count: (fields | length),
      terminal_required_field_count: (terminal_fields | length),
      evidence_contract_count: ($evidence_contracts | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: (prior_gates | length),
      terminal_wrappers: $terminal_wrappers,
      canonical_wire_fields: fields,
      terminal_required_fields: terminal_fields,
      evidence_contracts: $evidence_contracts,
      blockers: $blockers,
      required_prior_gates: prior_gates,
      recommended_next_gate: "hepta_work_graph_terminal_task_result_wrapper_fixture_preview_gate",
      ready_for_wrapper_fixture_preview: true,
      ready_for_task_result_enforcement: false,
      ready_for_store_enablement: false,
      ready_for_live_execution: false,
      source_probes: {
        terminal_task_result_wrapper: {
          rust_module_present: $terminal_wrapper_rust_module_present,
          report_script_present: $terminal_wrapper_report_script_present,
          gate_script_present: $terminal_wrapper_gate_script_present
        },
        idempotency_readback_adapter: {
          rust_module_present: $idempotency_adapter_rust_module_present,
          gate_script_present: $idempotency_adapter_gate_script_present
        },
        task_result_contract: {
          rust_module_present: $task_result_contract_rust_module_present
        },
        append_only_event_intake: {
          gate_script_present: $append_only_event_intake_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        event_record_persisted: false,
        task_result_persisted: false,
        graph_state_persisted: false,
        wal_written: false,
        checkpoint_written: false,
        wrapper_executed: false,
        runtime_adapter_attached: false,
        task_result_enforcement_enabled: false,
        scheduler_admission_enforced: false,
        readback_performed: false,
        replay_executed: false,
        approval_recorded: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
