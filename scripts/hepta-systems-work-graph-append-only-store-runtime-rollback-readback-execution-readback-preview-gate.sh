#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-append-only-store-runtime-rollback-readback-execution-readback-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-append-only-store-runtime-rollback-readback-execution-readback-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_append_only_store_runtime_rollback_readback_execution_readback_preview_gate"
  and .schema_version == "work_graph_append_only_store_runtime_rollback_readback_execution_readback_preview_v1"
  and .preview_mode == "read_only_append_only_store_runtime_rollback_readback_execution_readback_no_execution"
  and .upstream_rollback_readback_execution_preview_gate == "hepta_work_graph_append_only_store_runtime_rollback_readback_execution_preview_gate"
  and .source_surface_count == 12
  and .rollback_readback_execution_plan_count == 12
  and .readback_plan_count == 12
  and .stage_assertion_count == 5
  and .evidence_field_assertion_count == 12
  and .guard_assertion_count == 8
  and .blocker_mapping_assertion_count == 5
  and .drift_detector_count == 7
  and .blocker_count == 5
  and .required_prior_gate_count == 65
  and .rollback_readback_execution_stage_source_ref_count == 60
  and .rollback_readback_execution_stage_contract_ref_count == 28
  and .rollback_readback_execution_plan_stage_ref_count == 60
  and .rollback_readback_execution_plan_evidence_field_ref_count == 108
  and .blocker_mapping_source_ref_count == 60
  and .blocker_mapping_stage_ref_count == 13
' >/dev/null <<<"$report"

jq -e '
  (.readback_plans | all(
    .readback_state == "readback_verified_from_rollback_readback_execution_preview_no_execution"
    and .required_before_application == true
    and .performs_readback == false
    and .writes_wal == false
    and .writes_checkpoint == false
    and .mutates_idempotency_index == false
    and .executes_replay == false
    and .executes_rollback == false
    and .mutates_runtime == false
    and (.required_rollback_readback_execution_stage_ids | length) == 5
    and (.required_evidence_field_ids | length) == 9
  ))
  and (.stage_assertions | all(
    .expected_runtime_state == "readback_verified_contract_ready_runtime_disabled"
    and .contract_ready_preview == true
    and .runtime_enabled_after_readback == false
    and .performs_readback == false
    and .mutates_runtime == false
  ))
  and (.evidence_field_assertions | all(
    .expected_evidence_state == "evidence_fields_declared_not_persisted"
    and .required_field_count == 9
    and .performs_readback == false
    and .persists_evidence == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.blockers | map({id, count: (.affected_source_surface_ids | length), stages: (.affected_rollback_readback_execution_stage_ids | length)}) == [
    {"id": "readback_execution_disabled", "count": 12, "stages": 1},
    {"id": "wal_write_boundary_not_enabled", "count": 12, "stages": 1},
    {"id": "rollback_readback_not_executed", "count": 12, "stages": 1},
    {"id": "rollback_readback_execution_readback_missing", "count": 12, "stages": 5},
    {"id": "rollback_readback_execution_application_missing", "count": 12, "stages": 5}
  ])
  and (.blockers | all(.blocks_rollback_readback_execution == true and (.affected_readback_plan_ids | length) == 12))
  and (.blocker_mapping_assertions | all(
    .expected_blocker_state == "blocker_mapping_readback_verified_no_mutation"
    and .blocks_rollback_readback_execution == true
    and .performs_readback == false
    and .mutates_runtime == false
  ))
  and (.drift_detectors | all(.blocks_application_preview == true and .performs_readback == false))
  and (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_append_only_store_runtime_rollback_readback_execution_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_append_only_store_runtime_rollback_readback_execution_application_preview_gate"
  and .ready_for_runtime_rollback_readback_execution_application_preview == true
  and .ready_for_readback_execution == false
  and .ready_for_replay_execution == false
  and .ready_for_wal_write == false
  and .ready_for_checkpoint_write == false
  and .ready_for_rollback_execution == false
  and .ready_for_live_execution == false
  and .source_probes.rollback_readback_execution_readback.rust_module_present == true
  and .source_probes.rollback_readback_execution_readback.report_script_present == true
  and .source_probes.rollback_readback_execution_readback.gate_script_present == true
  and .source_probes.rollback_readback_execution_preview.upstream_gate == true
  and .source_probes.rollback_readback_execution_preview.gate_script_present == true
  and .source_probes.rollback_readback_execution_preview.recommended_next_matches == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_append_only_store_runtime_rollback_readback_execution_readback --lib

echo "Hepta WorkGraph append-only store runtime rollback/readback execution readback preview gate passed"
