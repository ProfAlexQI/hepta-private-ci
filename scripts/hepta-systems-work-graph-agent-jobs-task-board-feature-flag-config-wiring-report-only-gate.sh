#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-feature-flag-config-wiring-report-only-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-feature-flag-config-wiring-report-only-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only_v1"
  and .preview_mode == "feature_flag_config_wiring_report_only_no_config_write"
  and .config_contract_count == 2
  and .config_digest_preview_count == 2
  and .source_binding_count == 2
  and .safety_check_count == 4
  and (.config_contracts | map(.flag_id) == [
    "work_graph_agent_jobs_non_blocking_canary",
    "work_graph_task_board_non_blocking_canary"
  ])
  and (.config_contracts | all(
    .owner == "hepta-backend"
    and .config_surface_id == "work_graph_agent_jobs_task_board_canary_flags"
    and .default_enabled == false
    and .current_enabled == false
    and .canary_stage_id == "shadow_0ppm_report_only"
    and .traffic_ppm == 0
    and .readback_required == true
    and .rollback_replay_required == true
    and .operator_packet_required == true
    and .config_digest_required == true
    and .config_written == false
    and .live_mutation_allowed == false
  ))
  and (.config_digest_previews | all(
    .digest_algorithm == "sha256"
    and .digest_source == "canonical_config_contract_preview"
    and .redaction_policy == "redact_operator_packet_and_runtime_payloads"
    and .deterministic_input_count == 9
    and .digest_required_before_enablement == true
    and .digest_persisted == false
  ))
  and (.source_bindings | all(
    .metadata_field == "workGraphReportOnly"
    and .field_present == true
    and .default_off_observed == true
    and .zero_traffic_observed == true
    and .readback_observed == true
    and .rollback_replay_observed == true
    and .live_cutover_observed == false
  ))
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate",
    "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate",
    "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate",
    "hepta_work_graph_trace_guardrail_span_report_only_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  ]
  and .required_prior_gate_count == 5
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_operator_packet_report_only_gate"
  and .ready_for_operator_packet_report_only == true
  and .ready_for_feature_flag_config_write == false
  and .ready_for_feature_flag_enablement == false
  and .ready_for_live_cutover == false
  and .source_probes.config_wiring_module_present == true
  and .source_probes.feature_flag_gate_present == true
  and .source_probes.feature_flag_gate_points_here == true
  and .source_probes.agent_jobs_metadata_flag_fields_present == true
  and .source_probes.agent_jobs_metadata_safety_fields_present == true
  and .source_probes.task_board_metadata_flag_fields_present == true
  and .source_probes.task_board_metadata_safety_fields_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_feature_flag_config_wiring_report_only --lib

echo "Hepta WorkGraph agent_jobs + task_board feature-flag config wiring report-only gate passed"
