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
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistent_mailbox_handoff_event_mapping.rs
)"
report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistent-mailbox-handoff-event-mapping-report.sh
)"
gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistent-mailbox-handoff-event-mapping-gate.sh
)"
shadow_path_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-event-store-shadow-path-gate.sh
)"

jq -n \
  --argjson rust_module_present "$rust_module_present" \
  --argjson report_script_present "$report_script_present" \
  --argjson gate_script_present "$gate_script_present" \
  --argjson shadow_path_gate_script_present "$shadow_path_gate_script_present" \
  '
  def mailbox_mapping($id; $kind; $ack; $deadline; $artifacts): {
    id: $id,
    source_surface_id: "multi_agent_v2_mailbox_wait",
    work_graph_event_kind: $kind,
    required_fields: [
      "traceId",
      "mailboxSeq",
      "agentPath",
      "parentTaskId",
      "childTaskId",
      "eventId"
    ],
    parent_child_task_ref_required: true,
    ack_ref_required: $ack,
    deadline_ref_required: $deadline,
    artifact_refs_required: $artifacts,
    maps_to_shadow_event_store: true,
    persists_mailbox_event: false
  };
  def handoff_mapping($id; $kind; $direction; $artifacts; $approval): {
    id: $id,
    source_surface_id: "hepta_runtime_agent_harness",
    work_graph_event_kind: $kind,
    required_fields: [
      "traceId",
      "handoffId",
      "parentTaskId",
      "childTaskId",
      "artifactRefs",
      "deadlineRef"
    ],
    handoff_direction: $direction,
    artifact_refs_required: $artifacts,
    approval_ref_required: $approval,
    maps_to_shadow_event_store: true,
    persists_handoff_event: false
  };
  def ack_deadline($id; $kind; $ack; $deadline): {
    id: $id,
    applies_to_event_kind: $kind,
    required_fields: [
      "traceId",
      "ackId",
      "deadlineUnixMs",
      "parentTaskId",
      "childTaskId"
    ],
    timeout_policy: "deadline_required_no_unbounded_wait",
    ack_state: $ack,
    deadline_state: $deadline,
    mutates_mailbox_state: false
  };
  def wait_target($id; $type; $fields; $success; $timeout; $result; $barrier): {
    id: $id,
    wait_agent_mode: "named_task_result_barrier_preview",
    wait_target_type: $type,
    required_fields: $fields,
    success_condition: $success,
    timeout_condition: $timeout,
    returns_task_result_ref: $result,
    returns_barrier_ref: $barrier,
    live_wait_behavior_enabled: false
  };
  [
    mailbox_mapping("mailbox_message_queued_to_work_graph_event"; "MailboxMessageQueued"; true; true; false),
    mailbox_mapping("mailbox_message_delivered_to_work_graph_event"; "MailboxMessageDelivered"; true; true; true),
    mailbox_mapping("mailbox_ack_observed_to_work_graph_event"; "MailboxAckObserved"; true; false; false),
    mailbox_mapping("mailbox_deadline_expired_to_work_graph_event"; "MailboxDeadlineExpired"; false; true; false)
  ] as $mailbox_mappings
  | [
    handoff_mapping("handoff_requested_to_work_graph_event"; "HandoffRequested"; "parent_to_child"; true; true),
    handoff_mapping("handoff_accepted_to_work_graph_event"; "HandoffAccepted"; "child_to_parent"; true; false),
    handoff_mapping("handoff_artifact_linked_to_work_graph_event"; "HandoffArtifactLinked"; "producer_to_consumer"; true; false),
    handoff_mapping("handoff_barrier_satisfied_to_work_graph_event"; "TaskBarrierSatisfied"; "barrier_to_waiter"; false; false)
  ] as $handoff_mappings
  | [
    ack_deadline("mailbox_delivery_ack_contract"; "MailboxMessageDelivered"; "ack_required_before_wait_success"; "deadline_carried_from_wait_budget"),
    ack_deadline("mailbox_wait_timeout_contract"; "MailboxDeadlineExpired"; "ack_absent_after_timeout"; "deadline_required_for_timeout_result"),
    ack_deadline("handoff_acceptance_ack_contract"; "HandoffAccepted"; "ack_required_before_parent_merge"; "deadline_carried_from_handoff_policy")
  ] as $ack_deadlines
  | [
    wait_target("wait_agent_named_task_target"; "named_task"; ["taskName", "taskId", "traceId", "parentTaskId"]; "named task reaches terminal TaskResultEnvelope"; "deadline expires before named task terminal result"; true; false),
    wait_target("wait_agent_task_result_target"; "task_result"; ["taskId", "expectedStatus", "traceId", "verifierRef"]; "TaskResultEnvelope status satisfies expected status"; "deadline expires before task result readback"; true; false),
    wait_target("wait_agent_mailbox_barrier_target"; "barrier"; ["barrierId", "parentTaskId", "childTaskIds", "traceId"]; "all child task barriers have acked or terminal results"; "deadline expires before barrier quorum"; false; true)
  ] as $wait_targets
  | ["hepta_work_graph_append_only_event_store_shadow_path_gate"] as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_persistent_mailbox_handoff_event_mapping_gate",
      schema_version: "work_graph_persistent_mailbox_handoff_event_mapping_v1",
      preview_mode: "report_only_persistent_mailbox_handoff_event_mapping_no_live_wait_change",
      mailbox_event_mapping_count: ($mailbox_mappings | length),
      handoff_event_mapping_count: ($handoff_mappings | length),
      ack_deadline_contract_count: ($ack_deadlines | length),
      wait_agent_target_count: ($wait_targets | length),
      required_prior_gate_count: ($required_prior_gates | length),
      mailbox_event_mappings: $mailbox_mappings,
      handoff_event_mappings: $handoff_mappings,
      ack_deadline_contracts: $ack_deadlines,
      wait_agent_targets: $wait_targets,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_agent_role_agent_card_manifest_report_only_gate",
      mailbox_events_map_to_work_graph_events: true,
      ack_deadline_parent_child_artifact_refs_ready: true,
      wait_agent_named_task_result_barrier_ready: true,
      persistent_mailbox_store_enabled: false,
      live_wait_agent_behavior_changed: false,
      ready_for_agent_role_agent_card_manifest: true,
      ready_for_live_execution: false,
      source_probes: {
        persistent_mailbox_handoff_event_mapping: {
          rust_module_present: $rust_module_present,
          report_script_present: $report_script_present,
          gate_script_present: $gate_script_present
        },
        append_only_event_store_shadow_path: {
          gate_script_present: $shadow_path_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        work_graph_event_persisted: false,
        mailbox_event_persisted: false,
        handoff_event_persisted: false,
        ack_recorded: false,
        deadline_recorded: false,
        wait_agent_runtime_changed: false,
        barrier_state_mutated: false,
        artifact_ref_persisted: false,
        scheduler_admission_enforced: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
