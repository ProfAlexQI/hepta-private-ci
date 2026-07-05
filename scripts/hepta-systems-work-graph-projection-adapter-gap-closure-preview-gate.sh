#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-projection-adapter-gap-closure-preview-report.sh"

report="$(capture_json_report "hepta-work-graph-projection-adapter-gap-closure-preview-report" "$REPORT_SCRIPT")"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "blocked"
  and .gate == "hepta_work_graph_projection_adapter_gap_closure_preview_gate"
  and .schema_version == "work_graph_projection_adapter_gap_closure_preview_v1"
  and .preview_mode == "read_only_projection_adapter_gap_closure_plan_no_runtime_attachment"
  and .source_gap_count == 7
  and (.source_gaps | length) == .source_gap_count
  and (.source_gaps | map(.source_surface_id) == [
    "update_plan_tool",
    "plan_mode_proposed_plan_blocks",
    "app_server_turn_plan_notification",
    "multi_agent_v2_mailbox_wait",
    "hepta_runtime_multi_agent_reducer",
    "hepta_runtime_task_board",
    "hepta_runtime_approval_broker"
  ])
  and (.source_gaps | all(.expected_post_closure_state == "contract_ready_preview_after_gap_closure"))
' >/dev/null <<<"$report"

jq -e '
  .closure_action_count == 11
  and (.closure_actions | length) == .closure_action_count
  and .store_adapter_closure_count == 6
  and .timeline_adapter_closure_count == 5
  and .adapter_fixture_closure_count == 0
  and .task_result_adapter_closure_count == 0
  and (.closure_actions | all(.mutates_runtime == false and .enforces_projection == false))
  and (.closure_actions | map(select(.adapter_kind == "unified_store_projection" and .source_surface_id == "hepta_runtime_task_board")) | length) == 1
  and (.closure_actions | map(select(.adapter_kind == "observability_timeline_projection" and .source_surface_id == "update_plan_tool")) | length) == 1
  and (.closure_actions | map(select(.adapter_kind == "adapter_projection_fixture")) | length) == 0
' >/dev/null <<<"$report"

jq -e '
  .closure_plan_count == 5
  and (.closure_plans | map(.id) == [
    "planning_projection_adapter_gap_closure",
    "multi_agent_mailbox_projection_adapter_gap_closure",
    "multi_agent_reducer_projection_adapter_gap_closure",
    "task_board_projection_adapter_gap_closure",
    "approval_broker_projection_adapter_gap_closure"
  ])
  and (.closure_plans | all(.mutates_runtime == false))
  and (.closure_plans | map(select(.id == "planning_projection_adapter_gap_closure" and (.closure_action_ids | length == 4) and .expected_contract_ready_source_count_after_closure == 3)) | length) == 1
  and (.closure_plans | map(select(.id == "task_board_projection_adapter_gap_closure" and (.closure_action_ids | length == 2))) | length) == 1
' >/dev/null <<<"$report"

jq -e '
  .blocker_count == 5
  and (.blockers | map(.id) == [
    "gap_closure_is_preview_only",
    "adapter_fixture_closure_not_applied",
    "unified_store_adapter_closure_not_applied",
    "timeline_adapter_closure_not_applied",
    "post_closure_enforcement_readiness_not_rerun"
  ])
  and (.blockers | map(select(.severity == "high")) | length) == 4
  and (.blockers | all(.required_before_projection_enforcement == true))
  and (.blockers | map(select(.id == "adapter_fixture_closure_not_applied" and (.affected_source_surface_ids | length == 0))) | length) == 1
  and (.blockers | map(select(.id == "unified_store_adapter_closure_not_applied" and (.affected_source_surface_ids | length == 6))) | length) == 1
  and (.blockers | map(select(.id == "timeline_adapter_closure_not_applied" and (.affected_source_surface_ids | length == 5))) | length) == 1
  and (.blockers | map(select(.id == "post_closure_enforcement_readiness_not_rerun" and (.affected_source_surface_ids | length == 7))) | length) == 1
' >/dev/null <<<"$report"

jq -e '
  .required_prior_gate_count == 13
  and (.required_prior_gates[-1] == "hepta_work_graph_unified_projection_enforcement_readiness_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_projection_adapter_gap_closure_readback_preview_gate"
  and .ready_for_projection_adapter_gap_closure_readback_preview == true
  and .ready_for_projection_enforcement == false
  and .ready_for_append_only_store_enablement == false
  and .ready_for_live_execution == false
  and .source_probes.gap_closure.rust_module_present == true
  and .source_probes.gap_closure.report_script_present == true
  and .source_probes.gap_closure.gate_script_present == true
  and (.source_probes.upstream_reports | to_entries | all(.value == true))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_projection_adapter_gap_closure --lib

echo "Hepta WorkGraph projection adapter gap closure preview gate passed"
