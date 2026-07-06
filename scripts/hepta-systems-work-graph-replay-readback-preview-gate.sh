#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-replay-readback-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-replay-readback-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_replay_readback_preview_gate"
  and .schema_version == "work_graph_replay_readback_preview_v1"
  and .preview_mode == "read_only_replay_readback_contract_preview_no_replay"
  and .replay_stage_count == 6
  and (.replay_stages | length) == .replay_stage_count
  and (.replay_stages | map(.id) == [
    "preview_load_wal_head",
    "preview_validate_wal_hash_chain",
    "preview_apply_idempotency_window",
    "preview_materialize_collections",
    "preview_compare_checkpoint",
    "preview_emit_readback_report"
  ])
  and (.replay_stages | all(.executes_replay == false and (.input_contract_ids | length) >= 2 and (.output_contract_ids | length) >= 1))
  and .readback_assertion_count == 6
  and (.readback_assertions | length) == .readback_assertion_count
  and (.readback_assertions | map(.collection_id) == [
    "nodes",
    "edges",
    "taskResults",
    "artifacts",
    "approvals",
    "timelineEvents"
  ])
  and (.readback_assertions | all(.mutates_store == false and (.promotion_gate | startswith("block_"))))
  and .drift_detector_count == 5
  and (.drift_detectors | length) == .drift_detector_count
  and (.drift_detectors | map(.id) == [
    "detect_identity_drift",
    "detect_ordering_drift",
    "detect_status_drift",
    "detect_hash_drift",
    "detect_redaction_drift"
  ])
  and (.drift_detectors | all(.blocks_promotion == true and (.compared_fields | length) >= 3))
  and (.drift_detectors | map(select(.severity == "critical")) | length) == 4
  and .recovery_preview_count == 5
  and (.recovery_previews | length) == .recovery_preview_count
  and (.recovery_previews | map(.id) == [
    "preview_quarantine_checkpoint",
    "preview_rebuild_projection_indexes",
    "preview_hold_terminal_promotion",
    "preview_request_redaction_review",
    "preview_require_operator_replay_approval"
  ])
  and (.recovery_previews | all(.requires_operator_approval == true and .executes_recovery == false))
  and .invariant_count == 6
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "replay_is_deterministic",
    "readback_is_required_before_promotion",
    "drift_blocks_promotion",
    "recovery_requires_operator_approval",
    "readback_evidence_is_redacted",
    "replay_readback_preview_has_no_side_effects"
  ])
  and (.invariants | all(.required == true))
  and .append_only_event_contract_count == 9
  and .append_only_source_route_count == 12
  and .event_replay_plan_count == 9
  and (.append_only_event_replay_plans | length) == .event_replay_plan_count
  and (.append_only_event_replay_plans | map(.event_contract_id) == [
    "plan_step_event_intake",
    "agent_spawn_event_intake",
    "mailbox_delivery_event_intake",
    "agent_job_item_event_intake",
    "worker_task_event_intake",
    "scheduler_run_event_intake",
    "artifact_event_intake",
    "approval_event_intake",
    "task_result_event_intake"
  ])
  and (.append_only_event_replay_plans | all(.executes_replay == false and .performs_readback == false and .mutates_store == false))
  and (.append_only_event_replay_plans | all((.deterministic_replay_key_fields | length) >= 3 and (.readback_assertion_ids | length) >= 2))
  and (.append_only_event_replay_plans | map(select(.event_contract_id == "task_result_event_intake" and (.blocking_reason_ids | index("terminal_task_result_enforcement_disabled")) and (.blocking_reason_ids | index("event_intake_idempotency_guard_missing")))) | length) == 1
  and .source_readback_gap_count == 5
  and (.source_readback_gaps | length) == .source_readback_gap_count
  and (.source_readback_gaps | map(.source_surface_id) == [
    "plan_mode_proposed_plan_blocks",
    "app_server_turn_plan_notification",
    "multi_agent_v2_mailbox_wait",
    "hepta_runtime_multi_agent_reducer",
    "hepta_runtime_task_board"
  ])
  and (.source_readback_gaps | all(.required_before_replay_execution == true and (.missing_capability | contains("idempotency"))))
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
    "hepta_work_graph_append_only_event_intake_preview_gate"
  ])
  and .recommended_next_gate == "hepta_work_graph_idempotency_readback_adapter_preview_gate"
  and .ready_for_promotion_precondition_preview == true
  and .ready_for_replay_execution == false
  and .ready_for_live_execution == false
  and .source_probes.replay_readback.rust_module_present == true
  and .source_probes.replay_readback.report_script_present == true
  and .source_probes.replay_readback.gate_script_present == true
  and .source_probes.append_only_event_intake.rust_module_present == true
  and .source_probes.append_only_event_intake.report_script_present == true
  and .source_probes.append_only_event_intake.gate_script_present == true
  and .source_probes.unified_projection_audit.rust_module_present == true
  and .source_probes.state_store_persistence.rust_module_present == true
  and .source_probes.state_store_persistence.report_script_present == true
  and .source_probes.state_store_persistence.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_replay_readback_preview --lib

echo "Hepta WorkGraph replay/readback preview gate passed"
