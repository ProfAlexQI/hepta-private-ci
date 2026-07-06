#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-task-result-contract-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-task-result-contract-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_task_result_contract_preview_gate"
  and .schema_version == "work_graph_task_result_contract_preview_v1"
  and .preview_mode == "validator_first_schema_preview_no_enforcement"
  and .required_field_count == 11
  and (.required_fields | length) == .required_field_count
  and (.required_fields | map(.wire_name) == [
    "taskId",
    "status",
    "summary",
    "artifacts",
    "evidence",
    "risks",
    "nextActions",
    "verifier",
    "reducer",
    "usage",
    "traceId"
  ])
  and (.required_fields | all(.required == true))
  and (.required_fields | map(select(.terminal_required == true) | .wire_name) == [
    "taskId",
    "status",
    "summary",
    "evidence",
    "verifier",
    "traceId"
  ])
  and .status_count == 7
  and (.statuses | length) == .status_count
  and (.statuses | map(.id) == [
    "queued",
    "running",
    "succeeded",
    "failed",
    "cancelled",
    "blocked",
    "superseded"
  ])
  and .terminal_status_count == 5
  and (.statuses | map(select(.terminal == true) | .id) == [
    "succeeded",
    "failed",
    "cancelled",
    "blocked",
    "superseded"
  ])
  and (.statuses | all((.terminal == false and .requires_evidence == false) or (.terminal == true and .requires_evidence == true)))
  and (.statuses[] | select(.id == "blocked") | .promotion_allowed == false)
  and .validator_count == 7
  and (.validators | length) == .validator_count
  and (.validators | map(.id) == [
    "required_wire_fields_present",
    "terminal_status_requires_summary_evidence_and_trace",
    "artifact_reference_requires_identity_and_hash_or_path",
    "risk_entry_requires_severity_reason_and_owner",
    "verifier_reducer_and_usage_are_structured",
    "terminal_promotion_requires_no_secret_payload",
    "adapter_projection_is_preview_only"
  ])
  and (.validators | all(.required == true))
  and .adapter_preview_count == 6
  and (.adapter_previews | length) == .adapter_preview_count
  and (.adapter_previews | map(.source_surface_id) == [
    "agent_jobs_batch_workers",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_multi_agent_reducer",
    "multi_agent_v2_thread_spawn",
    "hepta_runtime_scheduler_store",
    "hepta_runtime_agent_harness"
  ])
  and (.adapter_previews | all(.enforcement_enabled == false))
  and (.adapter_previews | all(.covered_wire_fields == [
    "taskId",
    "status",
    "summary",
    "artifacts",
    "evidence",
    "risks",
    "nextActions",
    "verifier",
    "reducer",
    "usage",
    "traceId"
  ]))
  and (.adapter_previews | all(.blocker_ids | all(endswith("_report_only_not_enforced"))))
  and .recommended_next_gate == "hepta_work_graph_scheduler_admission_controller_preview_gate"
  and .ready_for_scheduler_admission_preview == true
  and .ready_for_task_result_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.task_result_contract.rust_module_present == true
  and .source_probes.task_result_contract.report_script_present == true
  and .source_probes.task_result_contract.gate_script_present == true
  and .source_probes.work_graph_contract_preview.rust_module_present == true
  and .source_probes.work_graph_contract_preview.report_script_present == true
  and .source_probes.work_graph_contract_preview.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_task_result_contract --lib

echo "Hepta WorkGraph TaskResult contract preview gate passed"
