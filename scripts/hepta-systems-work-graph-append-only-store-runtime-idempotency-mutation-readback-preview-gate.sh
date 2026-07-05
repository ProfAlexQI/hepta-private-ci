#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-append-only-store-runtime-idempotency-mutation-readback-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-append-only-store-runtime-idempotency-mutation-readback-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_append_only_store_runtime_idempotency_mutation_readback_preview_gate"
  and .schema_version == "work_graph_append_only_store_runtime_idempotency_mutation_readback_preview_v1"
  and .preview_mode == "read_only_append_only_store_runtime_idempotency_mutation_readback_no_execution"
  and .upstream_idempotency_mutation_preview_gate == "hepta_work_graph_append_only_store_runtime_idempotency_mutation_preview_gate"
  and .source_surface_count == 12
  and .idempotency_mutation_plan_count == 12
  and .readback_plan_count == 12
  and .stage_assertion_count == 5
  and .evidence_field_assertion_count == 12
  and .guard_assertion_count == 8
  and .blocker_mapping_assertion_count == 6
  and .drift_detector_count == 7
  and .blocker_count == 6
  and .required_prior_gate_count == 61
  and .idempotency_mutation_stage_source_ref_count == 60
  and .idempotency_mutation_stage_contract_ref_count == 28
  and .idempotency_mutation_plan_stage_ref_count == 60
  and .idempotency_mutation_plan_evidence_field_ref_count == 108
  and .blocker_mapping_source_ref_count == 72
  and .blocker_mapping_stage_ref_count == 14
' >/dev/null <<<"$report"

jq -e '
  (.readback_plans | map(.source_surface_id) == [
    "update_plan_tool",
    "plan_mode_proposed_plan_blocks",
    "app_server_turn_plan_notification",
    "multi_agent_v2_thread_spawn",
    "multi_agent_v2_mailbox_wait",
    "hepta_runtime_multi_agent_reducer",
    "agent_jobs_batch_workers",
    "hepta_runtime_task_board",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_scheduler_store",
    "hepta_runtime_approval_broker",
    "hepta_runtime_agent_harness"
  ])
  and (.readback_plans | all(
    .readback_state == "readback_verified_from_idempotency_mutation_preview_no_execution"
    and .required_before_application == true
    and .performs_readback == false
    and .writes_wal == false
    and .writes_checkpoint == false
    and .mutates_idempotency_index == false
    and .executes_replay == false
    and .executes_rollback == false
    and .mutates_runtime == false
    and (.required_idempotency_mutation_stage_ids | length) == 5
    and (.required_evidence_field_ids | length) == 9
  ))
  and (.readback_plans | map(select(.source_surface_id == "update_plan_tool" and .source_category == "planning")) | length) == 1
  and (.readback_plans | map(select(.source_surface_id == "hepta_runtime_agent_harness" and .source_category == "external_handoff")) | length) == 1
' >/dev/null <<<"$report"

jq -e '
  (.stage_assertions | map({id: .stage_id, category, source_count: (.affected_source_surface_ids | length), contract_count: (.required_contract_ref_ids | length)}) == [
    {"id": "idempotency_mutation_policy_contract", "category": "idempotency_policy", "source_count": 12, "contract_count": 6},
    {"id": "idempotency_collision_replay_evidence_contract", "category": "collision_replay_evidence", "source_count": 12, "contract_count": 6},
    {"id": "idempotency_index_no_mutation_guard", "category": "preview_no_mutation", "source_count": 12, "contract_count": 6},
    {"id": "rollback_readback_prerequisite_contract", "category": "rollback_readback_prerequisite", "source_count": 12, "contract_count": 5},
    {"id": "idempotency_blocker_mapping", "category": "blocker_mapping", "source_count": 12, "contract_count": 5}
  ])
  and (.stage_assertions | all(
    .expected_runtime_state == "readback_verified_contract_ready_runtime_disabled"
    and .contract_ready_preview == true
    and .runtime_enabled_after_readback == false
    and .performs_readback == false
    and .mutates_runtime == false
  ))
  and (.stage_assertions | map(select(.stage_id == "idempotency_mutation_policy_contract" and .declared_mutates_idempotency_index == true and .declared_writes_wal == false and .declared_writes_checkpoint == false)) | length) == 1
  and (.stage_assertions | map(select(.stage_id == "idempotency_collision_replay_evidence_contract" and .declared_writes_wal == true and .declared_writes_checkpoint == true and .declared_executes_replay == true)) | length) == 1
  and (.stage_assertions | map(select(.stage_id == "rollback_readback_prerequisite_contract" and .declared_executes_readback == true and .declared_executes_rollback == true and .declared_writes_checkpoint == true)) | length) == 1
  and (.stage_assertions | map(select(.stage_id == "idempotency_index_no_mutation_guard" and .declared_mutates_idempotency_index == false and .declared_writes_wal == false and .declared_writes_checkpoint == false and .declared_executes_replay == false and .declared_executes_readback == false and .declared_executes_rollback == false)) | length) == 1
  and (.stage_assertions | map(select(.stage_id == "idempotency_blocker_mapping" and .declared_mutates_idempotency_index == false and .declared_writes_wal == false and .declared_writes_checkpoint == false and .declared_executes_replay == false and .declared_executes_readback == false and .declared_executes_rollback == false)) | length) == 1
' >/dev/null <<<"$report"

jq -e '
  (.evidence_field_assertions | all(
    .expected_evidence_state == "evidence_fields_declared_not_persisted"
    and .required_field_count == 9
    and .performs_readback == false
    and .persists_evidence == false
  ))
  and (.guard_assertions | map(.guard_id) == [
    "idempotency_mutation_preview_only",
    "idempotency_index_mutation_disabled",
    "wal_write_boundary_disabled",
    "checkpoint_write_disabled",
    "replay_execution_disabled",
    "rollback_readback_execution_disabled",
    "durable_store_switch_disabled",
    "runtime_mutation_disabled"
  ])
  and (.guard_assertions | all(
    .expected_guard_state == "guard_declared_and_runtime_mutation_prevented"
    and .required_before_idempotency_mutation == true
    and .satisfied_by_readback == false
    and .mutates_runtime == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.blockers | map({id, count: (.affected_source_surface_ids | length), stage_count: (.affected_idempotency_mutation_stage_ids | length)}) == [
    {"id": "readback_execution_disabled", "count": 12, "stage_count": 1},
    {"id": "wal_write_boundary_not_enabled", "count": 12, "stage_count": 1},
    {"id": "idempotency_index_mutation_disabled", "count": 12, "stage_count": 1},
    {"id": "rollback_readback_not_executed", "count": 12, "stage_count": 1},
    {"id": "idempotency_mutation_readback_missing", "count": 12, "stage_count": 5},
    {"id": "idempotency_mutation_application_missing", "count": 12, "stage_count": 5}
  ])
  and (.blockers | all(.blocks_idempotency_mutation == true and (.affected_readback_plan_ids | length) == 12))
  and (.blocker_mapping_assertions | all(
    .expected_blocker_state == "blocker_mapping_readback_verified_no_mutation"
    and .blocks_idempotency_mutation == true
    and .performs_readback == false
    and .mutates_runtime == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.drift_detectors | map(.id) == [
    "idempotency_mutation_plan_alignment",
    "idempotency_mutation_stage_contract_alignment",
    "idempotency_mutation_evidence_field_alignment",
    "idempotency_mutation_guard_no_mutation_alignment",
    "idempotency_mutation_blocker_mapping_alignment",
    "idempotency_mutation_side_effect_alignment",
    "idempotency_mutation_upstream_gate_alignment"
  ])
  and (.drift_detectors | all(.blocks_application_preview == true and .performs_readback == false))
  and (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_append_only_store_runtime_idempotency_mutation_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_append_only_store_runtime_idempotency_mutation_application_preview_gate"
  and .ready_for_runtime_idempotency_mutation_application_preview == true
  and .ready_for_readback_execution == false
  and .ready_for_replay_execution == false
  and .ready_for_wal_write == false
  and .ready_for_checkpoint_write == false
  and .ready_for_idempotency_mutation == false
  and .ready_for_rollback_execution == false
  and .ready_for_append_only_store_enablement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.idempotency_mutation_readback.rust_module_present == true
  and .source_probes.idempotency_mutation_readback.report_script_present == true
  and .source_probes.idempotency_mutation_readback.gate_script_present == true
  and .source_probes.idempotency_mutation_preview.upstream_gate == true
  and .source_probes.idempotency_mutation_preview.gate_script_present == true
  and .source_probes.idempotency_mutation_preview.recommended_next_matches == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_append_only_store_runtime_idempotency_mutation_readback --lib

echo "Hepta WorkGraph append-only store runtime idempotency mutation readback preview gate passed"
