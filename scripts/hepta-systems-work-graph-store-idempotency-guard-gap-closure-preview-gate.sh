#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-store-idempotency-guard-gap-closure-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-store-idempotency-guard-gap-closure-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "blocked"
  and .gate == "hepta_work_graph_store_idempotency_guard_gap_closure_preview_gate"
  and .schema_version == "work_graph_store_idempotency_guard_gap_closure_preview_v1"
  and .preview_mode == "read_only_store_idempotency_guard_gap_closure_no_index_write"
  and .rerun_store_guard_gap_count == 5
  and .idempotency_adapter_count == 5
  and .existing_state_store_guard_count == 7
  and .existing_guard_gap_count == 5
  and .closure_plan_count == 5
  and .candidate_guard_count == 5
  and .guard_binding_count == 5
  and .guard_probe_binding_count == 5
  and .expected_collection_ref_count == 14
  and .readback_probe_contract_ref_count == 14
  and .task_result_guard_dependency_count == 2
  and (.closure_plans | length) == .closure_plan_count
  and (.candidate_guards | length) == .candidate_guard_count
  and (.guard_bindings | length) == .guard_binding_count
  and (.guard_probe_bindings | length) == .guard_probe_binding_count
' >/dev/null <<<"$report"

jq -e '
  (.closure_plans | map(.source_surface_id) == [
    "plan_mode_proposed_plan_blocks",
    "app_server_turn_plan_notification",
    "multi_agent_v2_mailbox_wait",
    "hepta_runtime_multi_agent_reducer",
    "hepta_runtime_task_board"
  ])
  and (.closure_plans | all(
    .rerun_enforcement_decision == "deny_missing_store_idempotency_guard"
    and .closure_state == "candidate_guard_preview_only_runtime_not_attached"
    and .runtime_guard_attached == false
    and .mutates_idempotency_index == false
    and .enables_store_write == false
    and (.key_fields | length) > 0
    and (.expected_collection_ids | length) > 0
    and (.readback_probe_contract_ids | length) > 0
  ))
  and (.closure_plans | map(select(.requires_task_result_wrapper == true) | .source_surface_id) == [
    "hepta_runtime_multi_agent_reducer",
    "hepta_runtime_task_board"
  ])
  and (.candidate_guards | map(.id) == [
    "plan_mode_proposed_plan_blocks_store_idempotency_guard",
    "app_server_turn_plan_notification_store_idempotency_guard",
    "multi_agent_v2_mailbox_wait_store_idempotency_guard",
    "hepta_runtime_multi_agent_reducer_store_idempotency_guard",
    "hepta_runtime_task_board_store_idempotency_guard"
  ])
  and (.candidate_guards | all(
    .required_before_append_only_intake == true
    and .mutates_idempotency_index == false
    and (.key_fields | length) > 0
    and (.key_formula | startswith("sha256("))
  ))
' >/dev/null <<<"$report"

jq -e '
  (.guard_bindings | all(
    .existing_state_store_guard_present == false
    and .adapter_replay_key_contract_present == true
    and .no_runtime_application == true
  ))
  and (.guard_bindings | map(.readback_probe_count) == [3, 3, 2, 2, 4])
  and (.guard_probe_bindings | all(.performs_readback == false and .mutates_store == false))
  and (.guard_probe_bindings | map(.target_collection_ids | length) == [3, 3, 2, 2, 4])
  and (.guard_probe_bindings | map(.drift_detector_ids) | all(. == ["detect_identity_drift", "detect_ordering_drift", "detect_hash_drift"]))
' >/dev/null <<<"$report"

jq -e '
  .blocker_count == 6
  and (.blockers | map({id, count: (.affected_source_surface_ids | length)}) == [
    {"id": "runtime_guard_application_disabled", "count": 5},
    {"id": "state_store_guard_persistence_disabled", "count": 5},
    {"id": "append_only_store_enablement_disabled", "count": 5},
    {"id": "task_result_enforcement_disabled", "count": 2},
    {"id": "readback_execution_disabled", "count": 5},
    {"id": "operator_review_required", "count": 5}
  ])
  and (.blockers | all(.required_before_projection_enforcement == true))
' >/dev/null <<<"$report"

jq -e '
  .required_prior_gate_count == 17
  and (.required_prior_gates[-1] == "hepta_work_graph_unified_projection_enforcement_readiness_rerun_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_store_idempotency_guard_gap_closure_readback_preview_gate"
  and .ready_for_store_idempotency_guard_gap_closure_readback_preview == true
  and .ready_for_runtime_guard_application == false
  and .ready_for_append_only_store_enablement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.store_idempotency_guard_gap_closure.rust_module_present == true
  and .source_probes.store_idempotency_guard_gap_closure.report_script_present == true
  and .source_probes.store_idempotency_guard_gap_closure.gate_script_present == true
  and .source_probes.readiness_rerun.upstream_gate == true
  and .source_probes.readiness_rerun.gate_script_present == true
  and .source_probes.idempotency_readback_adapter.upstream_gate == true
  and .source_probes.idempotency_readback_adapter.gate_script_present == true
  and .source_probes.state_store_persistence.upstream_gate == true
  and .source_probes.state_store_persistence.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_store_idempotency_guard_gap_closure --lib

echo "Hepta WorkGraph store idempotency guard gap closure preview gate passed"
