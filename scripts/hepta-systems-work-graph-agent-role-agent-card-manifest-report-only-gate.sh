#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-role-agent-card-manifest-report-only-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-role-agent-card-manifest-report-only-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_role_agent_card_manifest_report_only_gate"
  and .schema_version == "work_graph_agent_role_agent_card_manifest_report_only_v1"
  and .preview_mode == "report_only_agent_role_agent_card_manifest_no_enforcement"
  and .required_wire_field_count == 11
  and (.required_wire_fields == [
    "roleId",
    "agentCardId",
    "capabilities",
    "allowedTools",
    "budget",
    "sideEffectClass",
    "handoffDescription",
    "outputContract",
    "verifier",
    "reducer",
    "lane"
  ])
  and .agent_card_count == 5
  and (.agent_cards | map(.role_id) == [
    "planner_role",
    "builder_role",
    "reviewer_role",
    "scheduler_role",
    "handoff_role"
  ])
  and (.agent_cards | all(
    (.capability_ids | length) > 0
    and (.allowed_tools | length) > 0
    and (.budget_policy_id | length) > 0
    and (.side_effect_class | length) > 0
    and (.handoff_description | length) > 0
    and .output_contract == "TaskResultEnvelope"
    and (.verifier_id | length) > 0
    and (.reducer_id | length) > 0
    and .lane == "hepta-backend"
    and .manifest_report_only == true
    and .role_enforcement_enabled == false
  ))
  and .source_binding_count == 5
  and (.source_bindings | map(.source_surface_id) == [
    "multi_agent_v2_thread_spawn",
    "agent_jobs_batch_workers",
    "hepta_runtime_worker_tasks",
    "hepta_runtime_agent_harness",
    "hepta_runtime_task_board"
  ])
  and (.source_bindings | all(
    (.required_role_ids | length) > 0
    and .default_lane == "hepta-backend"
    and .output_contract == "TaskResultEnvelope"
    and .verifier_required == true
    and .manifest_attached_report_only == true
    and .enforcement_enabled == false
  ))
  and .required_prior_gates == [
    "hepta_work_graph_persistent_mailbox_handoff_event_mapping_gate",
    "hepta_work_graph_role_manifest_contract_preview_gate"
  ]
  and .required_prior_gate_count == 2
  and .recommended_next_gate == "hepta_work_graph_trace_guardrail_span_report_only_gate"
  and .capability_tool_budget_lane_ready == true
  and .side_effect_class_ready == true
  and .handoff_output_verifier_ready == true
  and .report_only_manifest_attached == true
  and .role_enforcement_enabled == false
  and .ready_for_trace_guardrail_span == true
  and .ready_for_live_execution == false
  and .source_probes.agent_role_agent_card_manifest_report_only.rust_module_present == true
  and .source_probes.agent_role_agent_card_manifest_report_only.report_script_present == true
  and .source_probes.agent_role_agent_card_manifest_report_only.gate_script_present == true
  and .source_probes.persistent_mailbox_handoff_event_mapping.gate_script_present == true
  and .source_probes.role_manifest_contract.gate_script_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_role_agent_card_manifest_report_only --lib

echo "Hepta WorkGraph AgentRole/AgentCard manifest report-only gate passed"
