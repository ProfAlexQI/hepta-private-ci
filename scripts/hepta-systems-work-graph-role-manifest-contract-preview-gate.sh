#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-role-manifest-contract-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-role-manifest-contract-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_role_manifest_contract_preview_gate"
  and .schema_version == "work_graph_role_manifest_contract_preview_v1"
  and .preview_mode == "read_only_role_manifest_contract_preview_no_enforcement"
  and .required_field_count == 12
  and (.required_fields | length) == .required_field_count
  and (.required_fields | map(.wire_name) == [
    "roleId",
    "roleKind",
    "agentPath",
    "capabilities",
    "toolPermissions",
    "outputSchemaRef",
    "verifierRef",
    "budget",
    "concurrency",
    "lane",
    "approvalPolicy",
    "tracePolicy"
  ])
  and (.required_fields | all(.required == true))
  and .capability_count == 7
  and (.capabilities | length) == .capability_count
  and (.capabilities | map(.id) == [
    "planning",
    "agent_delegation",
    "code_editing",
    "verification",
    "research",
    "scheduler_control",
    "external_handoff_proposal"
  ])
  and (.capabilities | map(select(.requires_verifier == true)) | length) == .capability_count
  and .permission_mode_count == 5
  and (.permission_modes | length) == .permission_mode_count
  and (.permission_modes | map(.id) == [
    "deny",
    "preview",
    "read_only",
    "write_scoped",
    "approval_required"
  ])
  and (.permission_modes[] | select(.id == "write_scoped") | .can_mutate_runtime == true and .requires_approval == true)
  and .invariant_count == 6
  and (.invariants | length) == .invariant_count
  and (.invariants | map(.id) == [
    "capability_requires_permission_mode",
    "mutation_requires_approval_and_lane",
    "terminal_output_requires_schema_and_verifier",
    "budget_and_concurrency_are_required",
    "trace_policy_is_required",
    "preview_gate_does_not_change_permissions"
  ])
  and (.invariants | all(.required == true))
  and .adapter_preview_count == 4
  and (.adapter_previews | length) == .adapter_preview_count
  and (.adapter_previews | map(.source_surface_id) == [
    "multi_agent_v2_thread_spawn",
    "agent_jobs_batch_workers",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_agent_harness"
  ])
  and (.adapter_previews | all(.enforcement_enabled == false))
  and (.adapter_previews | all((.covered_wire_fields | index("roleId")) and (.covered_wire_fields | index("budget"))))
  and .recommended_next_gate == "hepta_work_graph_unified_state_store_preview_gate"
  and .ready_for_unified_state_store_preview == true
  and .ready_for_role_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.role_manifest_contract.rust_module_present == true
  and .source_probes.role_manifest_contract.report_script_present == true
  and .source_probes.role_manifest_contract.gate_script_present == true
  and .source_probes.observability_timeline.rust_module_present == true
  and .source_probes.observability_timeline.report_script_present == true
  and .source_probes.observability_timeline.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_role_manifest_contract --lib

echo "Hepta WorkGraph role manifest contract preview gate passed"
