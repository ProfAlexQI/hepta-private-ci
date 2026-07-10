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

rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_event_store_shadow_path.rs
)"
report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-event-store-shadow-path-report.sh
)"
gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-event-store-shadow-path-gate.sh
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
scheduler_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-dry-run-enforcement-gate.sh
)"
append_only_intake_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-event-intake-preview-gate.sh
)"
shadow_write_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-work-graph-events-shadow-write-preview-gate.sh
)"
shadow_write_readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-work-graph-events-shadow-write-readback-preview-gate.sh
)"

scheduler="$(
  capture_json_report \
    "hepta-work-graph-scheduler-admission-dry-run-enforcement-report" \
    "$ROOT/scripts/hepta-systems-work-graph-scheduler-admission-dry-run-enforcement-report.sh"
)"
field_gap="$(
  capture_json_report \
    "hepta-work-graph-task-result-contract-field-gap-readback-report" \
    "$ROOT/scripts/hepta-systems-work-graph-task-result-contract-field-gap-readback-report.sh"
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
  --argjson scheduler_gate_script_present "$scheduler_gate_script_present" \
  --argjson append_only_intake_gate_script_present "$append_only_intake_gate_script_present" \
  --argjson shadow_write_gate_script_present "$shadow_write_gate_script_present" \
  --argjson shadow_write_readback_gate_script_present "$shadow_write_readback_gate_script_present" \
  --argjson scheduler "$scheduler" \
  --argjson field_gap "$field_gap" \
  '
  def event($source; $kind; $id; $trace; $index; $readback; $diff): {
    source_surface_id: $source,
    event_kind: $kind,
    deterministic_event_id: $id,
    deterministic_id_inputs: [
      "sourceSurfaceId",
      "traceId",
      "eventKind",
      "sequenceKey",
      "payloadHash"
    ],
    redacted_payload_ref: "redacted:work-graph-shadow-payload",
    payload_hash: "sha256:shadow-payload-preview-hash",
    projection_index_key: $index,
    readback_evidence_ref: $readback,
    replay_diff_ref: $diff,
    trace_id: $trace,
    shadow_persisted: false,
    live_cutover_enabled: false
  };
  def index($id; $collection; $keys; $events): {
    id: $id,
    collection_id: $collection,
    key_fields: $keys,
    event_kind_refs: $events,
    deterministic_order: "traceId:eventId:sequenceKey",
    index_persisted: false
  };
  def readback($id; $target; $fields; $evidence): {
    id: $id,
    readback_target: $target,
    required_event_fields: $fields,
    evidence_ref: $evidence,
    readback_status: "readback_evidence_ready_not_executed",
    readback_executed: false
  };
  def replay_diff($id; $scope; $fields; $expected): {
    id: $id,
    replay_scope: $scope,
    compared_fields: $fields,
    expected_diff: $expected,
    replay_executed: false,
    diff_persisted: false
  };
  [
    event("update_plan_tool"; "PlanStepCreated"; "wg-event-shadow-plan-001"; "trace-shadow-plan-001"; "idx:trace-shadow-plan-001:plan"; "rb:shadow-plan-step-created-001"; "diff:shadow-plan-step-created-001"),
    event("multi_agent_v2_thread_spawn"; "AgentTaskSpawned"; "wg-event-shadow-spawn-001"; "trace-shadow-spawn-001"; "idx:trace-shadow-spawn-001:agent"; "rb:shadow-agent-task-spawned-001"; "diff:shadow-agent-task-spawned-001"),
    event("multi_agent_v2_mailbox_wait"; "MailboxEventLinked"; "wg-event-shadow-mailbox-001"; "trace-shadow-mailbox-001"; "idx:trace-shadow-mailbox-001:mailbox"; "rb:shadow-mailbox-event-linked-001"; "diff:shadow-mailbox-event-linked-001"),
    event("agent_jobs_batch_workers"; "TaskResultReported"; "wg-event-shadow-agent-job-result-001"; "trace-shadow-agent-job-001"; "idx:trace-shadow-agent-job-001:task-result"; "rb:shadow-agent-job-task-result-001"; "diff:shadow-agent-job-task-result-001"),
    event("hepta_runtime_worker_tasks"; "ArtifactProduced"; "wg-event-shadow-worker-artifact-001"; "trace-shadow-worker-001"; "idx:trace-shadow-worker-001:artifact"; "rb:shadow-worker-artifact-001"; "diff:shadow-worker-artifact-001"),
    event("hepta_runtime_task_board"; "TaskBoardTerminalEvent"; "wg-event-shadow-task-board-terminal-001"; "trace-shadow-task-board-001"; "idx:trace-shadow-task-board-001:terminal"; "rb:shadow-task-board-terminal-001"; "diff:shadow-task-board-terminal-001"),
    event("hepta_runtime_scheduler_store"; "SchedulerAdmissionEvaluated"; "wg-event-shadow-scheduler-admission-001"; "trace-shadow-scheduler-001"; "idx:trace-shadow-scheduler-001:admission"; "rb:shadow-scheduler-admission-001"; "diff:shadow-scheduler-admission-001"),
    event("hepta_runtime_approval_broker"; "GuardrailApprovalEvaluated"; "wg-event-shadow-guardrail-approval-001"; "trace-shadow-guardrail-001"; "idx:trace-shadow-guardrail-001:approval"; "rb:shadow-guardrail-approval-001"; "diff:shadow-guardrail-approval-001")
  ] as $events
  | [
    index("projection_by_trace_id"; "timelineEvents"; ["traceId", "eventId"]; ["PlanStepCreated", "AgentTaskSpawned", "MailboxEventLinked", "TaskResultReported"]),
    index("projection_by_task_id"; "taskResults"; ["taskId", "eventId"]; ["TaskResultReported", "TaskBoardTerminalEvent", "SchedulerAdmissionEvaluated"]),
    index("projection_by_source_surface"; "nodes"; ["sourceSurfaceId", "eventKind", "sequenceKey"]; ["PlanStepCreated", "AgentTaskSpawned", "ArtifactProduced", "GuardrailApprovalEvaluated"]),
    index("projection_by_parent_child_task"; "edges"; ["parentTaskId", "childTaskId", "eventId"]; ["AgentTaskSpawned", "MailboxEventLinked"]),
    index("projection_by_replay_diff"; "timelineEvents"; ["replayDiffRef", "payloadHash", "eventId"]; ["SchedulerAdmissionEvaluated", "TaskResultReported", "GuardrailApprovalEvaluated"])
  ] as $indexes
  | [
    readback("shadow_readback_event_id_lookup"; "event_id"; ["eventId", "eventKind", "traceId"]; "evidence:shadow-event-id-lookup"),
    readback("shadow_readback_payload_hash_check"; "payload_hash"; ["eventId", "payloadHash", "redactedPayloadRef"]; "evidence:shadow-payload-hash-check"),
    readback("shadow_readback_projection_index_lookup"; "projection_index"; ["projectionIndexKey", "collectionId", "deterministicOrder"]; "evidence:shadow-projection-index-lookup"),
    readback("shadow_readback_terminal_task_result_join"; "task_result_join"; ["taskId", "traceId", "verifierRef"]; "evidence:shadow-terminal-task-result-join"),
    readback("shadow_readback_scheduler_admission_join"; "scheduler_admission_join"; ["traceId", "admissionDecision", "failedChecks"]; "evidence:shadow-scheduler-admission-join")
  ] as $readback_evidence
  | [
    replay_diff("shadow_replay_noop_projection_diff"; "single_trace_projection"; ["eventId", "projectionIndexKey", "payloadHash"]; "no_diff_preview"),
    replay_diff("shadow_replay_duplicate_event_suppression_diff"; "idempotency_window"; ["deterministicEventId", "idempotencyKey", "payloadHash"]; "duplicate_suppressed_preview"),
    replay_diff("shadow_replay_projection_index_rebuild_diff"; "projection_index_rebuild"; ["collectionId", "keyFields", "eventKindRefs"]; "index_rebuild_matches_preview"),
    replay_diff("shadow_replay_redaction_hash_stability_diff"; "redaction_hash_stability"; ["redactedPayloadRef", "payloadHash", "evidenceRef"]; "hash_stable_preview")
  ] as $replay_diffs
  | [
    "hepta_work_graph_task_result_envelope_report_only_validator_gate",
    "hepta_work_graph_adapter_task_result_index_gate",
    "hepta_work_graph_terminal_envelope_readback_gate",
    "hepta_work_graph_source_id_alignment_readback_gate",
    "hepta_work_graph_task_result_contract_field_gap_readback_gate"
  ] as $scheduler_prior_gates
  | ($scheduler_prior_gates + [
      "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate",
      "hepta_work_graph_append_only_event_intake_preview_gate",
      "hepta_work_graph_append_only_work_graph_events_shadow_write_preview_gate",
      "hepta_work_graph_append_only_work_graph_events_shadow_write_readback_preview_gate"
    ]) as $required_prior_gates
  | ($scheduler.required_prior_gates == $scheduler_prior_gates
      and $scheduler.ready_for_append_only_event_store_shadow_path == true
      and $scheduler.live_blocking_enforcement_enabled == false) as $scheduler_prior_chain_ready
  | ($field_gap.ready_for_append_only_event_store_shadow_path == true
      and $field_gap.gap_source_count == 0
      and $field_gap.contract_required_field_gap_count == 0
      and $field_gap.contract_terminal_field_gap_count == 0
      and $field_gap.ready_for_task_result_enforcement == false) as $task_result_contract_field_gap_readback_ready
  | ($scheduler_prior_chain_ready
      and $task_result_contract_field_gap_readback_ready
      and ($events | length) > 0
      and ($indexes | length) > 0
      and ($readback_evidence | length) > 0
      and ($replay_diffs | length) > 0) as $append_only_shadow_path_readiness_complete
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_append_only_event_store_shadow_path_gate",
      schema_version: "work_graph_append_only_event_store_shadow_path_v1",
      preview_mode: "read_only_append_only_event_store_shadow_path_no_live_cutover",
      event_record_count: ($events | length),
      projection_index_count: ($indexes | length),
      readback_evidence_count: ($readback_evidence | length),
      replay_diff_count: ($replay_diffs | length),
      scheduler_prior_gate_count: ($scheduler_prior_gates | length),
      required_prior_gate_count: ($required_prior_gates | length),
      event_records: $events,
      projection_indexes: $indexes,
      readback_evidence: $readback_evidence,
      replay_diffs: $replay_diffs,
      scheduler_prior_gates: $scheduler_prior_gates,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_persistent_mailbox_handoff_event_mapping_gate",
      redacted_payload_policy_ready: true,
      deterministic_event_ids_ready: true,
      projection_index_ready: true,
      readback_evidence_ready: true,
      replay_diff_ready: true,
      scheduler_prior_chain_ready: $scheduler_prior_chain_ready,
      task_result_contract_field_gap_readback_ready: $task_result_contract_field_gap_readback_ready,
      append_only_shadow_path_readiness_complete: $append_only_shadow_path_readiness_complete,
      shadow_store_write_enabled: false,
      live_cutover_enabled: false,
      ready_for_persistent_mailbox_handoff: $append_only_shadow_path_readiness_complete,
      ready_for_live_execution: false,
      source_probes: {
        append_only_event_store_shadow_path: {
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
          gate_script_present: $task_result_contract_field_gap_readback_gate_script_present,
          report_gate: $field_gap.gate
        },
        scheduler_admission_dry_run_enforcement: {
          gate_script_present: $scheduler_gate_script_present,
          report_gate: $scheduler.gate
        },
        append_only_event_intake: {
          gate_script_present: $append_only_intake_gate_script_present
        },
        append_only_work_graph_events_shadow_write: {
          gate_script_present: $shadow_write_gate_script_present
        },
        append_only_work_graph_events_shadow_write_readback: {
          gate_script_present: $shadow_write_readback_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        event_store_enabled: false,
        shadow_event_persisted: false,
        projection_index_persisted: false,
        wal_written: false,
        checkpoint_written: false,
        readback_executed: false,
        replay_executed: false,
        replay_diff_persisted: false,
        idempotency_index_mutated: false,
        scheduler_admission_enforced: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
