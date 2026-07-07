#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-work-graph-agent-jobs-task-board-canary-readback-replay-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-agent-jobs-task-board-canary-readback-replay-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_agent_jobs_task_board_canary_readback_replay_gate"
  and .schema_version == "work_graph_agent_jobs_task_board_canary_readback_replay_v1"
  and .preview_mode == "canary_readback_replay_report_only_no_live_cutover"
  and .canary_entrypoint_count == 2
  and .readback_evidence_count == 2
  and .replay_diff_count == 2
  and .source_entrypoint_emission_required_prior_gate_count == 3
  and .source_entrypoint_emission_entrypoint_count == 2
  and .source_entrypoint_emission_emission_count == 2
  and .source_append_only_shadow_path_scheduler_prior_gate_count == 5
  and .source_append_only_shadow_path_required_prior_gate_count == 9
  and .source_task_result_envelope_source_adapter_count == 7
  and .source_task_result_envelope_source_envelope_count == 7
  and .source_scheduler_admission_entrypoint_count == 4
  and .source_scheduler_admission_required_prior_gate_count == 5
  and (.canary_entrypoints | map(.source_surface_id) == [
    "agent_jobs_batch_workers",
    "hepta_runtime_task_board"
  ])
  and (.canary_entrypoints | all(
    .report_only_field == "workGraphReportOnly"
    and .admission_decision == "allow_report_only_no_live_blocking"
    and (.task_result_preview | contains("TaskResultEnvelope"))
    and .live_blocking_enabled == false
    and .live_persistence_enabled == false
  ))
  and (.projection_indexes | length) == 2
  and (.projection_indexes | all(
    (.key_fields | index("taskId") != null)
    and (.key_fields | index("traceId") != null)
    and (.deterministic_id_rule | contains("sha256"))
    and (.redaction_rule | contains("no raw prompt"))
    and .persisted == false
  ))
  and (.readback_evidence | length) == 2
  and (.readback_evidence | all(
    .evidence_status == "preview_ready_not_persisted"
    and (.checks | length) >= 4
    and .evidence_persisted == false
  ))
  and (.replay_diffs | length) == 2
  and (.replay_diffs | all(
    .expected_diff == "deterministic report-only envelope matches readback projection"
    and .replay_executed == false
  ))
  and .required_prior_gates == [
    "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate",
    "hepta_work_graph_append_only_event_store_shadow_path_gate",
    "hepta_work_graph_task_result_envelope_report_only_validator_gate",
    "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  ]
  and .required_prior_gate_count == 4
  and .source_entrypoint_emission_gate == "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate"
  and .source_append_only_shadow_path_gate == "hepta_work_graph_append_only_event_store_shadow_path_gate"
  and .source_task_result_envelope_validator_gate == "hepta_work_graph_task_result_envelope_report_only_validator_gate"
  and .source_scheduler_admission_dry_run_gate == "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  and .recommended_next_gate == "hepta_work_graph_agent_jobs_task_board_feature_flag_non_blocking_canary_gate"
  and .feature_flag_required == true
  and .feature_flag_enabled == false
  and .source_entrypoint_emission_readiness_complete == true
  and .source_entrypoint_emission_no_live_confirmed == true
  and .source_append_only_shadow_path_readiness_complete == true
  and .source_append_only_shadow_path_no_persistence_confirmed == true
  and .source_task_result_envelope_validator_ready == true
  and .source_task_result_envelope_no_enforcement_confirmed == true
  and .source_scheduler_admission_dry_run_ready == true
  and .source_scheduler_admission_no_live_blocking_confirmed == true
  and .canary_readback_replay_prior_readbacks_complete == true
  and .canary_projection_readback_replay_preview_complete == true
  and .ready_for_non_blocking_canary == true
  and .ready_for_live_cutover == false
  and .source_probes.canary_readback_replay_module_present == true
  and .source_probes.entrypoint_emission_gate_present == true
  and .source_probes.shadow_path_gate_present == true
  and .source_probes.entrypoint_emission_report_gate == "hepta_work_graph_agent_jobs_task_board_report_only_entrypoint_emission_gate"
  and .source_probes.entrypoint_emission_readiness_complete == true
  and .source_probes.entrypoint_emission_ready_for_canary_readback_replay_gate == true
  and .source_probes.entrypoint_emission_side_effects_all_false == true
  and .source_probes.append_only_shadow_path_report_gate == "hepta_work_graph_append_only_event_store_shadow_path_gate"
  and .source_probes.append_only_shadow_path_readiness_complete == true
  and .source_probes.append_only_shadow_path_side_effects_all_false == true
  and .source_probes.task_result_envelope_validator_report_gate == "hepta_work_graph_task_result_envelope_report_only_validator_gate"
  and .source_probes.task_result_envelope_validator_ready == true
  and .source_probes.task_result_envelope_validator_side_effects_all_false == true
  and .source_probes.scheduler_admission_dry_run_report_gate == "hepta_work_graph_scheduler_admission_dry_run_enforcement_gate"
  and .source_probes.scheduler_admission_dry_run_ready == true
  and .source_probes.scheduler_admission_dry_run_side_effects_all_false == true
  and .source_probes.agent_jobs_report_only_hook_present == true
  and .source_probes.task_board_report_only_hook_present == true
  and .source_probes.agent_jobs_test_present == true
  and .source_probes.task_board_test_present == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_agent_jobs_task_board_canary_readback_replay --lib

echo "Hepta WorkGraph agent_jobs + task_board canary readback/replay gate passed"
