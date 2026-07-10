#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-append-only-event-store-shadow-path-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-append-only-event-store-shadow-path-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_append_only_event_store_shadow_path_gate"
  and .schema_version == "work_graph_append_only_event_store_shadow_path_v1"
  and .preview_mode == "read_only_append_only_event_store_shadow_path_no_live_cutover"
  and .event_record_count == 8
  and (.event_records | map(.event_kind) == [
    "PlanStepCreated",
    "AgentTaskSpawned",
    "MailboxEventLinked",
    "TaskResultReported",
    "ArtifactProduced",
    "TaskBoardTerminalEvent",
    "SchedulerAdmissionEvaluated",
    "GuardrailApprovalEvaluated"
  ])
  and (.event_records | all(
    (.deterministic_event_id | startswith("wg-event-shadow-"))
    and (.deterministic_id_inputs == [
      "sourceSurfaceId",
      "traceId",
      "eventKind",
      "sequenceKey",
      "payloadHash"
    ])
    and (.redacted_payload_ref | startswith("redacted:"))
    and (.payload_hash | startswith("sha256:"))
    and (.projection_index_key | startswith("idx:"))
    and (.readback_evidence_ref | startswith("rb:"))
    and (.replay_diff_ref | startswith("diff:"))
    and .shadow_persisted == false
    and .live_cutover_enabled == false
  ))
  and .projection_index_count == 5
  and (.projection_indexes | map(.id) == [
    "projection_by_trace_id",
    "projection_by_task_id",
    "projection_by_source_surface",
    "projection_by_parent_child_task",
    "projection_by_replay_diff"
  ])
  and (.projection_indexes | all(.index_persisted == false and (.key_fields | length) > 0 and (.event_kind_refs | length) > 0))
  and .readback_evidence_count == 5
  and (.readback_evidence | map(.id) == [
    "shadow_readback_event_id_lookup",
    "shadow_readback_payload_hash_check",
    "shadow_readback_projection_index_lookup",
    "shadow_readback_terminal_task_result_join",
    "shadow_readback_scheduler_admission_join"
  ])
  and (.readback_evidence | all(.readback_status == "readback_evidence_ready_not_executed" and .readback_executed == false))
  and .replay_diff_count == 4
  and (.replay_diffs | map(.id) == [
    "shadow_replay_noop_projection_diff",
    "shadow_replay_duplicate_event_suppression_diff",
    "shadow_replay_projection_index_rebuild_diff",
    "shadow_replay_redaction_hash_stability_diff"
  ])
  and (.replay_diffs | all(.replay_executed == false and .diff_persisted == false))
  and .scheduler_prior_gates == [
    "hepta_work_graph_task_result_envelope_report_only_validator_gate",
    "hepta_work_graph_adapter_task_result_index_gate",
    "hepta_work_graph_terminal_envelope_readback_gate",
    "hepta_work_graph_source_id_alignment_readback_gate",
    "hepta_work_graph_task_result_contract_field_gap_readback_gate"
  ]
  and .scheduler_prior_gate_count == 5
  and .required_prior_gates == [
    "hepta_work_graph_task_result_envelope_report_only_validator_gate",
    "hepta_work_graph_adapter_task_result_index_gate",
    "hepta_work_graph_terminal_envelope_readback_gate",
    "hepta_work_graph_source_id_alignment_readback_gate",
    "hepta_work_graph_task_result_contract_field_gap_readback_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate",
    "hepta_work_graph_append_only_event_intake_preview_gate",
    "hepta_work_graph_append_only_work_graph_events_shadow_write_preview_gate",
    "hepta_work_graph_append_only_work_graph_events_shadow_write_readback_preview_gate"
  ]
  and .required_prior_gate_count == 9
  and .recommended_next_gate == "hepta_work_graph_persistent_mailbox_handoff_event_mapping_gate"
  and .redacted_payload_policy_ready == true
  and .deterministic_event_ids_ready == true
  and .projection_index_ready == true
  and .readback_evidence_ready == true
  and .replay_diff_ready == true
  and .scheduler_prior_chain_ready == true
  and .task_result_contract_field_gap_readback_ready == true
  and .append_only_shadow_path_readiness_complete == true
  and .shadow_store_write_enabled == false
  and .live_cutover_enabled == false
  and .ready_for_persistent_mailbox_handoff == true
  and .ready_for_live_execution == false
  and .source_probes.append_only_event_store_shadow_path.rust_module_present == true
  and .source_probes.append_only_event_store_shadow_path.report_script_present == true
  and .source_probes.append_only_event_store_shadow_path.gate_script_present == true
  and .source_probes.task_result_envelope_report_only_validator.gate_script_present == true
  and .source_probes.adapter_task_result_index.gate_script_present == true
  and .source_probes.terminal_envelope_readback.gate_script_present == true
  and .source_probes.source_id_alignment_readback.gate_script_present == true
  and .source_probes.task_result_contract_field_gap_readback.gate_script_present == true
  and .source_probes.task_result_contract_field_gap_readback.report_gate == "hepta_work_graph_task_result_contract_field_gap_readback_gate"
  and .source_probes.scheduler_admission_dry_run_enforcement.gate_script_present == true
  and .source_probes.scheduler_admission_dry_run_enforcement.report_gate == "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  and .source_probes.append_only_event_intake.gate_script_present == true
  and .source_probes.append_only_work_graph_events_shadow_write.gate_script_present == true
  and .source_probes.append_only_work_graph_events_shadow_write_readback.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_append_only_event_store_shadow_path --lib

echo "Hepta WorkGraph append-only event store shadow path gate passed"
