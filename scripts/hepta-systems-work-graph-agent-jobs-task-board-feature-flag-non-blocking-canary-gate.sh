#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-non-blocking-canary-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-non-blocking-canary-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_v1"
  and .preview_mode == "feature_flag_non_blocking_canary_report_only_no_flag_mutation"
  and .feature_flag_count == 2
  and .rollout_stage_count == 1
  and .emission_binding_count == 2
  and .safety_check_count == 4
  and (.feature_flags | map(.id) == [
    "work_graph_agent_jobs_non_blocking_canary",
    "work_graph_task_board_non_blocking_canary"
  ])
  and (.feature_flags | all(
    .config_surface_id == "work_graph_agent_jobs_task_board_canary_flags"
    and .default_enabled == false
    and .current_enabled == false
    and .canary_stage_id == "shadow_0ppm_report_only"
    and .traffic_ppm == 0
    and (.required_before_enablement | index("canary_readback_replay_gate_green") != null)
    and (.required_before_enablement | index("operator_review_packet") != null)
    and (.required_before_enablement | index("rollback_replay_gate_green") != null)
    and .allows_live_blocking == false
    and .allows_persistence == false
  ))
  and (.rollout_stages == [{
    "id": "shadow_0ppm_report_only",
    "order": 0,
    "traffic_ppm": 0,
    "mode": "report_only_observation",
    "blocks_runtime_mutation": true,
    "requires_readback_gate": true,
    "requires_rollback_replay_gate": true
  }])
  and (.emission_bindings | all(
    .report_only_field == "workGraphReportOnly"
    and .feature_flag_field_attached == true
    and .readback_field_attached == true
    and .rollback_replay_field_attached == true
    and .live_cutover_field_attached == true
    and .live_cutover_enabled == false
  ))
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate",
    "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate",
    "hepta_work_graph_trace_guardrail_span_report_only_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  ]
  and .required_prior_gate_count == 4
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_gate"
  and .ready_for_feature_flag_config_wiring == true
  and .ready_for_feature_flag_enablement == false
  and .ready_for_live_cutover == false
  and .source_probes.feature_flag_module_present == true
  and .source_probes.canary_readback_gate_present == true
  and .source_probes.entrypoint_emission_gate_present == true
  and .source_probes.agent_jobs_feature_flag_field_present == true
  and .source_probes.agent_jobs_feature_flag_id_present == true
  and .source_probes.task_board_feature_flag_field_present == true
  and .source_probes.task_board_feature_flag_id_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary --lib

echo "Hepta WorkGraph agent_jobs + task_board feature-flag non-blocking canary gate passed"
