#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-append-only-store-runtime-durable-store-switch-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-append-only-store-runtime-durable-store-switch-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "blocked"
  and .gate == "hepta_work_graph_append_only_store_runtime_durable_store_switch_preview_gate"
  and .schema_version == "work_graph_append_only_store_runtime_durable_store_switch_preview_v1"
  and .preview_mode == "read_only_append_only_store_runtime_durable_store_switch_preview_no_store_mutation"
  and .upstream_runtime_write_boundary_rerun_gate == "hepta_work_graph_unified_projection_enforcement_readiness_runtime_write_boundary_rerun_preview_gate"
  and .source_surface_count == 12
  and .durable_store_switch_source_count == 12
  and .durable_store_switch_plan_count == 12
  and .durable_store_switch_stage_count == 5
  and .durable_store_switch_stage_source_ref_count == 60
  and .durable_store_switch_stage_contract_ref_count == 28
  and .durable_store_switch_plan_stage_ref_count == 60
  and .durable_store_switch_plan_evidence_field_ref_count == 108
  and .durable_store_residual_source_count == 12
  and .wal_boundary_residual_source_count == 12
  and .idempotency_residual_source_count == 12
  and .rollback_readback_residual_source_count == 12
  and .guard_count == 8
  and .blocker_count == 6
  and .required_prior_gate_count == 56
' >/dev/null <<<"$report"

jq -e '
  (.durable_store_switch_plans | map(.source_surface_id) == [
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
  and (.durable_store_switch_plans | all(
    .previous_enforcement_decision == "deny_runtime_durable_store_switch_disabled"
    and .durable_store_switch_state == "durable_store_switch_contract_defined_preview_only"
    and .required_durable_store_switch_stage_ids == [
      "runtime_durable_store_switch_contract",
      "wal_replay_prerequisite_contract",
      "operator_review_rollback_guard",
      "durable_store_switch_no_mutation_guard",
      "durable_store_switch_blocker_mapping"
    ]
    and .expected_evidence_field_ids == [
      "source_surface_id",
      "source_category",
      "runtime_write_boundary_rerun_decision_ref",
      "durable_store_switch_contract_id",
      "wal_replay_prerequisite_id",
      "operator_review_rollback_guard_id",
      "no_mutation_guard_ref",
      "residual_source_blocker_ids",
      "next_required_gate"
    ]
    and .durable_store_switch_contract_ready_preview == true
    and .applies_to_runtime == false
    and .writes_wal == false
    and .writes_checkpoint == false
    and .switches_durable_store == false
    and .mutates_idempotency_index == false
    and .executes_replay == false
    and .executes_readback == false
    and .executes_rollback == false
    and .mutates_runtime == false
  ))
  and (.durable_store_switch_plans | map(select(.residual_source_blocker_ids | index("durable_store_runtime_switch_disabled"))) | length) == 12
  and (.durable_store_switch_plans | map(select(.residual_source_blocker_ids | index("runtime_write_boundary_readiness_rerun_missing"))) | length) == 0
  and (.durable_store_switch_plans | map(select(.residual_source_blocker_ids | index("operator_review_required"))) | length) == 0
' >/dev/null <<<"$report"

jq -e '
  (.durable_store_switch_stage_plans | map({id, count: (.affected_source_surface_ids | length), contracts: (.required_contract_ref_ids | length)}) == [
    {"id": "runtime_durable_store_switch_contract", "count": 12, "contracts": 6},
    {"id": "wal_replay_prerequisite_contract", "count": 12, "contracts": 6},
    {"id": "operator_review_rollback_guard", "count": 12, "contracts": 5},
    {"id": "durable_store_switch_no_mutation_guard", "count": 12, "contracts": 6},
    {"id": "durable_store_switch_blocker_mapping", "count": 12, "contracts": 5}
  ])
  and (.durable_store_switch_stage_plans | all(
    .priority == "p0"
    and .expected_runtime_state == "contract_ready_preview_runtime_disabled"
    and .contract_ready_preview == true
    and .runtime_enabled_after_preview == false
    and (.prerequisite_gate_ids[-1] == "hepta_work_graph_unified_projection_enforcement_readiness_runtime_write_boundary_rerun_preview_gate")
  ))
  and (.durable_store_switch_stage_plans | map(select(.id == "runtime_durable_store_switch_contract" and .switches_durable_store == true and .writes_wal == false and .writes_checkpoint == false)) | length) == 1
  and (.durable_store_switch_stage_plans | map(select(.id == "wal_replay_prerequisite_contract" and .writes_wal == true and .writes_checkpoint == true and .executes_replay == true)) | length) == 1
  and (.durable_store_switch_stage_plans | map(select(.id == "operator_review_rollback_guard" and .executes_readback == true and .executes_rollback == true and .writes_checkpoint == true)) | length) == 1
  and (.durable_store_switch_stage_plans | map(select(.id == "durable_store_switch_no_mutation_guard" and .switches_durable_store == false and .writes_wal == false and .writes_checkpoint == false and .mutates_idempotency_index == false and .executes_replay == false and .executes_readback == false and .executes_rollback == false)) | length) == 1
  and (.durable_store_switch_stage_plans | map(select(.id == "durable_store_switch_blocker_mapping" and .switches_durable_store == false and .writes_wal == false and .writes_checkpoint == false and .mutates_idempotency_index == false and .executes_replay == false and .executes_readback == false and .executes_rollback == false)) | length) == 1
' >/dev/null <<<"$report"

jq -e '
  (.guards | map(.id) == [
    "durable_store_switch_preview_only",
    "durable_store_runtime_switch_disabled",
    "wal_write_boundary_disabled",
    "checkpoint_write_disabled",
    "replay_execution_disabled",
    "rollback_readback_execution_disabled",
    "idempotency_index_mutation_disabled",
    "runtime_mutation_disabled"
  ])
  and (.guards | all(.required_before_durable_store_switch == true and .satisfied_by_preview == false))
  and (.blockers | map({id, count: (.affected_source_surface_ids | length), stages: .affected_durable_store_switch_stage_ids}) == [
    {"id": "readback_execution_disabled", "count": 12, "stages": ["operator_review_rollback_guard"]},
    {"id": "durable_store_runtime_switch_disabled", "count": 12, "stages": ["runtime_durable_store_switch_contract"]},
    {"id": "wal_write_boundary_not_enabled", "count": 12, "stages": ["wal_replay_prerequisite_contract"]},
    {"id": "idempotency_index_mutation_disabled", "count": 12, "stages": ["durable_store_switch_blocker_mapping"]},
    {"id": "rollback_readback_not_executed", "count": 12, "stages": ["operator_review_rollback_guard"]},
    {"id": "durable_store_switch_readback_missing", "count": 12, "stages": [
      "runtime_durable_store_switch_contract",
      "wal_replay_prerequisite_contract",
      "operator_review_rollback_guard",
      "durable_store_switch_no_mutation_guard",
      "durable_store_switch_blocker_mapping"
    ]}
  ])
  and (.blockers | all(.required_before_durable_store_switch == true))
  and (.blockers | all((.affected_durable_store_switch_plan_ids | length) == (.affected_source_surface_ids | length)))
' >/dev/null <<<"$report"

jq -e '
  (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_unified_projection_enforcement_readiness_runtime_write_boundary_rerun_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_append_only_store_runtime_durable_store_switch_readback_preview_gate"
  and .ready_for_runtime_durable_store_switch_readback_preview == true
  and .ready_for_runtime_durable_store_switch_application_preview == false
  and .ready_for_wal_write == false
  and .ready_for_checkpoint_write == false
  and .ready_for_durable_store_switch == false
  and .ready_for_idempotency_mutation == false
  and .ready_for_rollback_readback_execution == false
  and .ready_for_append_only_store_enablement == false
  and .ready_for_projection_enforcement == false
  and .ready_for_scheduler_admission_enforcement == false
  and .ready_for_role_manifest_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.durable_store_switch_preview.rust_module_present == true
  and .source_probes.durable_store_switch_preview.report_script_present == true
  and .source_probes.durable_store_switch_preview.gate_script_present == true
  and .source_probes.runtime_write_boundary_rerun.upstream_gate == true
  and .source_probes.runtime_write_boundary_rerun.gate_script_present == true
  and .source_probes.runtime_write_boundary_rerun.recommended_next_matches == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_append_only_store_runtime_durable_store_switch --lib

echo "Hepta WorkGraph append-only store runtime durable-store switch preview gate passed"
