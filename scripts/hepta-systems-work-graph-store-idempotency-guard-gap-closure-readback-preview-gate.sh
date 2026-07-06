#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-store-idempotency-guard-gap-closure-readback-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-store-idempotency-guard-gap-closure-readback-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_store_idempotency_guard_gap_closure_readback_preview_gate"
  and .schema_version == "work_graph_store_idempotency_guard_gap_closure_readback_preview_v1"
  and .preview_mode == "read_only_store_idempotency_guard_gap_closure_readback_no_execution"
  and .closure_plan_count == 5
  and .candidate_guard_count == 5
  and .guard_binding_count == 5
  and .guard_probe_binding_count == 5
  and .readback_plan_count == 5
  and .key_formula_assertion_count == 5
  and .collision_policy_assertion_count == 5
  and .probe_binding_assertion_count == 5
  and .collection_ref_assertion_count == 5
  and .expected_collection_ref_count == 14
  and .readback_probe_contract_ref_count == 14
  and .readback_evidence_field_ref_count == 39
  and .drift_detector_count == 5
  and (.readback_plans | length) == .readback_plan_count
  and (.key_formula_assertions | length) == .key_formula_assertion_count
  and (.collision_policy_assertions | length) == .collision_policy_assertion_count
  and (.probe_binding_assertions | length) == .probe_binding_assertion_count
  and (.collection_ref_assertions | length) == .collection_ref_assertion_count
' >/dev/null <<<"$report"

jq -e '
  (.readback_plans | map(.source_surface_id) == [
    "plan_mode_proposed_plan_blocks",
    "app_server_turn_plan_notification",
    "multi_agent_v2_mailbox_wait",
    "hepta_runtime_multi_agent_reducer",
    "hepta_runtime_task_board"
  ])
  and (.readback_plans | all(
    .required_before_runtime_guard_application == true
    and .readback_state == "readback_assertions_defined_execution_disabled"
    and .performs_readback == false
    and .mutates_store == false
    and (.expected_key_fields | length) > 0
    and (.expected_collection_ids | length) > 0
    and (.readback_probe_contract_ids | length) > 0
  ))
  and (.key_formula_assertions | all(
    .requires_sha256_formula == true
    and (.key_formula | startswith("sha256("))
    and (.key_fields | length) > 0
    and .mutates_idempotency_index == false
  ))
  and (.collision_policy_assertions | all(
    .required_before_append_only_intake == true
    and .expected_collision_state == "collision_blocks_duplicate_projection_preview_only"
    and .mutates_idempotency_index == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.probe_binding_assertions | map(.readback_probe_contract_ids | length) == [3, 3, 2, 2, 4])
  and (.collection_ref_assertions | map(.required_collection_count) == [3, 3, 2, 2, 4])
  and (.probe_binding_assertions | all(
    .expected_probe_binding_state == "probe_contract_shape_defined_readback_disabled"
    and .performs_readback == false
    and .mutates_store == false
    and (.drift_detector_ids == ["detect_identity_drift", "detect_ordering_drift", "detect_hash_drift"])
  ))
  and (.collection_ref_assertions | all(
    .expected_guard_binding_state == "candidate_guard_defined_state_store_binding_not_applied"
    and .mutates_store == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.drift_detectors | map(.id) == [
    "store_guard_key_formula_drift",
    "store_guard_collision_policy_drift",
    "store_guard_probe_contract_drift",
    "store_guard_collection_ref_drift",
    "store_guard_redaction_policy_drift"
  ])
  and (.drift_detectors | all(.blocks_runtime_guard_application == true and .performs_readback == false))
  and .blocker_count == 6
  and (.blockers | map({id, count: (.affected_source_surface_ids | length)}) == [
    {"id": "readback_execution_disabled", "count": 5},
    {"id": "runtime_guard_application_disabled", "count": 5},
    {"id": "state_store_guard_persistence_disabled", "count": 5},
    {"id": "append_only_store_enablement_disabled", "count": 5},
    {"id": "task_result_enforcement_disabled", "count": 2},
    {"id": "operator_review_required", "count": 5}
  ])
  and (.blockers | all(.required_before_projection_enforcement == true))
' >/dev/null <<<"$report"

jq -e '
  .required_prior_gate_count == 18
  and (.required_prior_gates[-1] == "hepta_work_graph_store_idempotency_guard_gap_closure_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_store_idempotency_guard_gap_closure_application_preview_gate"
  and .ready_for_store_idempotency_guard_gap_closure_application_preview == true
  and .ready_for_runtime_guard_application == false
  and .ready_for_append_only_store_enablement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.store_idempotency_guard_gap_closure_readback.rust_module_present == true
  and .source_probes.store_idempotency_guard_gap_closure_readback.report_script_present == true
  and .source_probes.store_idempotency_guard_gap_closure_readback.gate_script_present == true
  and .source_probes.store_idempotency_guard_gap_closure.upstream_gate == true
  and .source_probes.store_idempotency_guard_gap_closure.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_store_idempotency_guard_gap_closure_readback --lib

echo "Hepta WorkGraph store idempotency guard gap closure readback preview gate passed"
