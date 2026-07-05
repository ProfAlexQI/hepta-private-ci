#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-idempotency-readback-adapter-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-idempotency-readback-adapter-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_idempotency_readback_adapter_preview_gate"
  and .schema_version == "work_graph_idempotency_readback_adapter_preview_v1"
  and .preview_mode == "read_only_idempotency_readback_adapter_preview_no_attachment"
  and .source_adapter_count == 5
  and (.source_adapters | length) == .source_adapter_count
  and (.source_adapters | map(.source_surface_id) == [
    "plan_mode_proposed_plan_blocks",
    "app_server_turn_plan_notification",
    "multi_agent_v2_mailbox_wait",
    "hepta_runtime_multi_agent_reducer",
    "hepta_runtime_task_board"
  ])
  and (.source_adapters | all(.attaches_runtime_adapter == false and .performs_readback == false and .mutates_store == false))
  and (.source_adapters | map(select(.requires_task_result_wrapper == true)) | length) == 2
  and .replay_key_contract_count == 5
  and (.replay_key_contracts | length) == .replay_key_contract_count
  and (.replay_key_contracts | map(.id) == [
    "plan_mode_plan_step_replay_key",
    "app_server_turn_plan_notification_replay_key",
    "multi_agent_mailbox_delivery_replay_key",
    "multi_agent_reducer_task_result_replay_key",
    "task_board_worker_task_replay_key"
  ])
  and (.replay_key_contracts | all(.mutates_idempotency_index == false and (.key_formula | startswith("sha256(")) and (.key_fields | length) >= 4))
  and .readback_probe_contract_count == 14
  and (.readback_probe_contracts | length) == .readback_probe_contract_count
  and (.readback_probe_contracts | map(.collection_id) | unique == [
    "artifacts",
    "edges",
    "nodes",
    "taskResults",
    "timelineEvents"
  ])
  and (.readback_probe_contracts | all(.performs_readback == false and .mutates_store == false and (.drift_detector_ids | length) == 3))
  and .gap_closure_count == 5
  and (.gap_closures | length) == .gap_closure_count
  and (.gap_closures | map(.closes_replay_readback_gap_id) == [
    "gap_plan_mode_proposed_plan_blocks_replay_key",
    "gap_app_server_turn_plan_notification_replay_key",
    "gap_multi_agent_mailbox_delivery_replay_key",
    "gap_multi_agent_reducer_task_result_replay_key",
    "gap_task_board_worker_task_replay_key"
  ])
  and (.gap_closures | all(.closure_state == "preview_contract_defined_runtime_not_attached" and .required_before_replay_execution == true))
  and .blocker_count == 4
  and (.blockers | length) == .blocker_count
  and (.blockers | map(.id) == [
    "runtime_adapter_attachment_disabled",
    "terminal_task_result_wrapper_not_enforced",
    "append_only_store_still_disabled",
    "replay_execution_disabled_by_design"
  ])
  and (.blockers | map(select(.severity == "high")) | length) == 2
  and (.blockers | all(.required_before_replay_execution == true))
  and .required_prior_gate_count == 11
  and (.required_prior_gates == [
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
    "hepta_work_graph_replay_readback_preview_gate"
  ])
  and .recommended_next_gate == "hepta_work_graph_terminal_task_result_wrapper_preview_gate"
  and .ready_for_task_result_wrapper_preview == true
  and .ready_for_replay_execution == false
  and .ready_for_store_enablement == false
  and .ready_for_live_execution == false
  and .source_probes.idempotency_readback_adapter.rust_module_present == true
  and .source_probes.idempotency_readback_adapter.report_script_present == true
  and .source_probes.idempotency_readback_adapter.gate_script_present == true
  and .source_probes.replay_readback.rust_module_present == true
  and .source_probes.replay_readback.report_script_present == true
  and .source_probes.replay_readback.gate_script_present == true
  and .source_probes.append_only_event_intake.rust_module_present == true
  and .source_probes.append_only_event_intake.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_idempotency_readback_adapter --lib

echo "Hepta WorkGraph idempotency/readback adapter preview gate passed"
