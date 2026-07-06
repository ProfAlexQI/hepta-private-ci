#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-canonical-projection-readiness-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-canonical-projection-readiness-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_canonical_projection_readiness_gate"
  and .schema_version == "work_graph_canonical_projection_readiness_v1"
  and .preview_mode == "read_only_receipt15_canonical_projection_rollup_no_tail_extension"
  and .receipt15_source_surface_count == 7
  and .receipt15_ready_surface_count == 7
  and .receipt15_blocked_surface_count == 0
  and .receipt15_required_prior_gate_count == 180
  and .contract_count == 7
  and .contract_ready_count == 7
  and .blocker_count == 0
  and (.contracts | map(.id) == [
    "receipt15_terminal_no_cutover_proof",
    "adapter_projection_fixture",
    "task_result_contract",
    "scheduler_admission_controller",
    "role_manifest_contract",
    "append_only_event_intake",
    "observability_timeline"
  ])
  and (.contracts | all(.ready == true and .enforcement_enabled == false and .persistence_enabled == false))
  and .recommended_next_gate == "hepta_work_graph_task_result_envelope_report_only_validator_gate"
  and .ready_for_task_result_envelope_report_only_validator == true
  and .ready_for_scheduler_admission_dry_run_enforcement == true
  and .ready_for_append_only_event_store_shadow_path == true
  and .ready_for_live_execution == false
  and .source_probes.canonical_projection_readiness.rust_module_present == true
  and .source_probes.canonical_projection_readiness.report_script_present == true
  and .source_probes.canonical_projection_readiness.gate_script_present == true
  and .source_probes.receipt15_rerun.ready_surface_count == 7
  and .source_probes.receipt15_rerun.blocked_surface_count == 0
  and .source_probes.receipt15_rerun.required_prior_gate_count == 180
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_canonical_projection_readiness --lib

echo "Hepta WorkGraph canonical projection readiness gate passed"
