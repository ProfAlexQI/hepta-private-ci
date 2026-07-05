#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-store-idempotency-guard-gap-closure-application-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-store-idempotency-guard-gap-closure-application-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_store_idempotency_guard_gap_closure_application_preview_gate"
  and .schema_version == "work_graph_store_idempotency_guard_gap_closure_application_preview_v1"
  and .preview_mode == "read_only_store_idempotency_guard_gap_closure_application_preview_no_runtime_mutation"
  and .readback_plan_count == 5
  and .application_plan_count == 5
  and .source_outcome_count == 5
  and .source_store_guard_contract_ready_preview_count == 5
  and .application_group_count == 3
  and .expected_collection_ref_count == 14
  and .readback_probe_contract_ref_count == 14
  and .readback_evidence_field_ref_count == 39
  and .task_result_guard_dependency_count == 2
  and .application_guard_count == 7
  and .blocker_count == 7
  and (.application_plans | length) == .application_plan_count
  and (.source_outcomes | length) == .source_outcome_count
  and (.application_groups | length) == .application_group_count
  and (.application_guards | length) == .application_guard_count
  and (.blockers | length) == .blocker_count
' >/dev/null <<<"$report"

jq -e '
  (.application_plans | map(.source_surface_id) == [
    "plan_mode_proposed_plan_blocks",
    "app_server_turn_plan_notification",
    "multi_agent_v2_mailbox_wait",
    "hepta_runtime_multi_agent_reducer",
    "hepta_runtime_task_board"
  ])
  and (.application_plans | all(
    .application_scope == "store_idempotency_guard_runtime_binding"
    and .application_state == "preview_application_defined_runtime_guard_not_attached"
    and .readback_verified_by_preview == true
    and .applies_to_runtime == false
    and .mutates_idempotency_index == false
    and .persists_state_store_guard == false
    and .enables_append_only_store == false
    and .enforces_projection == false
    and (.expected_key_fields | length) > 0
    and (.expected_collection_ids | length) > 0
    and (.readback_probe_contract_ids | length) > 0
    and (.readback_evidence_fields | length) > 0
  ))
  and (.application_plans | map(select(.source_surface_id == "hepta_runtime_task_board") | .expected_collection_ids) == [["nodes", "taskResults", "artifacts", "timelineEvents"]])
' >/dev/null <<<"$report"

jq -e '
  (.source_outcomes | all(
    .post_application_store_guard_state == "store_guard_contract_ready_preview_after_application"
    and .store_idempotency_guard_ready_preview == true
    and .ready_for_enforcement_readiness_store_guard_rerun == true
    and .ready_for_projection_enforcement == false
    and .applies_to_runtime == false
  ))
  and (.application_groups | map({id, count: (.application_plan_ids | length)}) == [
    {"id": "planning_store_idempotency_guard_application", "count": 2},
    {"id": "multi_agent_store_idempotency_guard_application", "count": 2},
    {"id": "task_board_store_idempotency_guard_application", "count": 1}
  ])
  and (.application_groups | all(
    .priority == "p0"
    and .mutates_runtime == false
    and .mutates_idempotency_index == false
    and .enables_append_only_store == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.application_guards | map(.id) == [
    "runtime_guard_attachment_disabled",
    "idempotency_index_mutation_disabled",
    "state_store_guard_persistence_disabled",
    "append_only_store_enablement_disabled",
    "task_result_enforcement_disabled",
    "operator_review_required",
    "enforcement_readiness_store_guard_rerun_required"
  ])
  and (.application_guards | all(.required_before_projection_enforcement == true and .satisfied_by_preview == false))
  and (.blockers | map({id, count: (.affected_source_surface_ids | length)}) == [
    {"id": "store_guard_application_is_preview_only", "count": 5},
    {"id": "runtime_guard_application_disabled", "count": 5},
    {"id": "idempotency_index_mutation_disabled", "count": 5},
    {"id": "state_store_guard_persistence_disabled", "count": 5},
    {"id": "append_only_store_enablement_disabled", "count": 5},
    {"id": "terminal_task_result_enforcement_disabled", "count": 2},
    {"id": "enforcement_readiness_store_guard_rerun_missing", "count": 5}
  ])
  and (.blockers | all(.required_before_projection_enforcement == true))
' >/dev/null <<<"$report"

jq -e '
  .required_prior_gate_count == 19
  and (.required_prior_gates[-1] == "hepta_work_graph_store_idempotency_guard_gap_closure_readback_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview_gate"
  and .ready_for_unified_projection_enforcement_readiness_store_guard_rerun_preview == true
  and .ready_for_runtime_guard_application == false
  and .ready_for_append_only_store_enablement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.store_idempotency_guard_gap_closure_application.rust_module_present == true
  and .source_probes.store_idempotency_guard_gap_closure_application.report_script_present == true
  and .source_probes.store_idempotency_guard_gap_closure_application.gate_script_present == true
  and .source_probes.store_idempotency_guard_gap_closure_readback.upstream_gate == true
  and .source_probes.store_idempotency_guard_gap_closure_readback.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_store_idempotency_guard_gap_closure_application --lib

echo "Hepta WorkGraph store idempotency guard gap closure application preview gate passed"
