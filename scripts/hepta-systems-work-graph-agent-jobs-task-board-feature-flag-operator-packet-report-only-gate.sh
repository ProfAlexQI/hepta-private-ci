#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-operator-packet-report-only-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-operator-packet-report-only-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_v1"
  and .preview_mode == "feature_flag_operator_packet_report_only_no_approval_no_send_no_persistence"
  and .operator_packet_section_count == 5
  and .review_item_count == 2
  and .evidence_ref_count == 5
  and .blocked_action_count == 6
  and (.operator_packet_sections | all(.required == true))
  and (.operator_packet_sections | map(.source_gate) == [
    "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_gate",
    "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate",
    "hepta_work_graph_trace_guardrail_span_report_only_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  ])
  and (.review_items | map(.flag_id) == [
    "work_graph_agent_jobs_non_blocking_canary",
    "work_graph_task_board_non_blocking_canary"
  ])
  and (.review_items | all(
    .review_surface_id == "work_graph_agent_jobs_task_board_feature_flag_operator_packet"
    and .decision_state == "pending_operator_review"
    and .required_before_enablement == true
    and .config_write_authorized == false
    and .canary_traffic_authorized == false
    and .live_cutover_authorized == false
  ))
  and (.evidence_refs | all(
    .required == true
    and .redacted == true
    and .persisted == false
  ))
  and (.blocked_actions | all(.blocked == true))
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_gate",
    "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate",
    "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate",
    "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate",
    "hepta_work_graph_trace_guardrail_span_report_only_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  ]
  and .required_prior_gate_count == 6
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_non_send_readback_gate"
  and .operator_packet_visible == true
  and .operator_packet_sent == false
  and .operator_packet_recorded == false
  and .operator_packet_persisted == false
  and .operator_packet_authorizes_config_write == false
  and .operator_packet_authorizes_canary_traffic == false
  and .operator_packet_authorizes_live_cutover == false
  and .ready_for_operator_packet_non_send_readback == true
  and .ready_for_feature_flag_config_write == false
  and .ready_for_feature_flag_enablement == false
  and .ready_for_live_cutover == false
  and .source_probes.operator_packet_module_present == true
  and .source_probes.config_wiring_gate_present == true
  and .source_probes.config_wiring_gate_points_here == true
  and .source_probes.config_wiring_ready_for_operator_packet == true
  and .source_probes.agent_jobs_flag_metadata_present == true
  and .source_probes.task_board_flag_metadata_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only --lib

echo "Hepta WorkGraph agent_jobs + task_board feature-flag operator packet report-only gate passed"
