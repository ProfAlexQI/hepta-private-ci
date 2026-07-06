#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-contract-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-contract-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_contract_preview_gate"
  and .schema_version == "work_graph_contract_preview_v1"
  and .preview_mode == "read_only_contract_preview_no_state_store"
  and .node_type_count == 8
  and (.node_types | length) == .node_type_count
  and (.node_types | map(.id) == [
    "plan_step",
    "agent_task",
    "worker_task",
    "scheduler_run",
    "verification_gate",
    "artifact",
    "human_approval",
    "external_handoff"
  ])
  and (.node_types | all(
    (.required_identity_fields | length) >= 3
    and (.required_status_fields | index("status"))
    and (.required_evidence_fields | index("trace_id"))
  ))
  and .edge_type_count == 7
  and (.edge_types | length) == .edge_type_count
  and (.edge_types | map(.id) == [
    "depends_on",
    "spawned_by",
    "produces",
    "verifies",
    "blocks",
    "retries",
    "replaces"
  ])
  and (.edge_types | all((.from_node_kinds | length) >= 1 and (.to_node_kinds | length) >= 1 and (.invariant | length) > 0))
  and .invariant_count == 6
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "stable_node_identity_required",
    "source_surface_required",
    "trace_id_required",
    "task_result_not_optional_for_terminal_tasks",
    "admission_before_execution",
    "preview_gate_is_side_effect_free"
  ])
  and (.invariants | all(.required == true))
  and .adapter_preview_count == 6
  and (.adapter_previews | length) == .adapter_preview_count
  and (.adapter_previews | map(.source_surface_id) == [
    "update_plan_tool",
    "multi_agent_v2_thread_spawn",
    "agent_jobs_batch_workers",
    "hepta_runtime_task_board",
    "hepta_runtime_scheduler_store",
    "hepta_runtime_agent_harness"
  ])
  and (.adapter_previews | all(.live_mutation_enabled == false))
  and .recommended_next_gate == "hepta_work_graph_task_result_contract_preview_gate"
  and .ready_for_task_result_contract_preview == true
  and .ready_for_scheduler_admission_preview == false
  and .ready_for_live_execution == false
  and .source_probes.contract_preview_contract.rust_module_present == true
  and .source_probes.contract_preview_contract.report_script_present == true
  and .source_probes.contract_preview_contract.gate_script_present == true
  and .source_probes.current_state_inventory_contract.rust_module_present == true
  and .source_probes.current_state_inventory_contract.report_script_present == true
  and .source_probes.current_state_inventory_contract.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_contract_preview --lib

echo "Hepta WorkGraph contract preview gate passed"
