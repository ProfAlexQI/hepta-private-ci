#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-terminal-envelope-readback-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-terminal-envelope-readback-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_terminal_envelope_readback_gate"
  and .schema_version == "work_graph_terminal_envelope_readback_v1"
  and .preview_mode == "read_only_terminal_envelope_readback_no_live_enforcement"
  and .terminal_source_count == 6
  and .terminal_envelope_adapter_count == 6
  and .terminal_envelope_preview_count == 6
  and .readback_consistent_source_count == 6
  and .missing_terminal_envelope_adapter_count == 0
  and .missing_terminal_envelope_preview_count == 0
  and .missing_terminal_envelope_wire_field_count == 0
  and .task_result_contract_required_field_gap_count == 0
  and .task_result_contract_terminal_field_gap_count == 0
  and (.terminal_sources | map(.source_surface_id) == [
    "multi_agent_v2_thread_spawn",
    "hepta_runtime_multi_agent_reducer",
    "agent_jobs_batch_workers",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_scheduler_store",
    "hepta_runtime_agent_harness"
  ])
' >/dev/null <<<"$report"

jq -e '
  (.terminal_sources | all(
    .task_result_envelope_adapter_state == "present_report_only"
    and .task_result_envelope_preview_state == "present_report_only"
    and (.missing_envelope_wire_fields | length) == 0
    and .readback_decision == "terminal_envelope_readback_consistent_report_only"
    and .live_enforcement_enabled == false
    and .next_readback_step == "source_id_alignment_readback"
  ))
  and ((.blockers | map(.id) | index("terminal_task_result_envelope_readback_missing")) | not)
  and ((.blockers | map(.id) | index("task_result_contract_required_fields_partial")) | not)
  and (.blockers | map(.id) | index("terminal_envelope_live_enforcement_disabled"))
  and (.blockers | all(.blocks_live_execution == true))
  and (.required_prior_gates == [
    "hepta_work_graph_task_result_envelope_report_only_validator_gate",
    "hepta_work_graph_adapter_task_result_index_gate"
  ])
  and .recommended_next_gate == "hepta_work_graph_source_id_alignment_readback_gate"
  and .ready_for_source_id_alignment_readback == true
  and .ready_for_task_result_enforcement == false
  and .ready_for_live_execution == false
' >/dev/null <<<"$report"

jq -e '
  .source_probes.terminal_envelope_readback.rust_module_present == true
  and .source_probes.terminal_envelope_readback.report_script_present == true
  and .source_probes.terminal_envelope_readback.gate_script_present == true
  and .source_probes.priors.task_result_envelope_gate == "hepta_work_graph_task_result_envelope_report_only_validator_gate"
  and .source_probes.priors.adapter_task_result_index_gate == "hepta_work_graph_adapter_task_result_index_gate"
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_terminal_envelope_readback --lib

echo "Hepta WorkGraph terminal envelope readback gate passed"
