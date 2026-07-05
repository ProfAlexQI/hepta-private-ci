#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

REPORT_SCRIPT="$ROOT/scripts/hepta-systems-wg-events-cutover-terminal-no-cutover-receipt-ack-replay-idempotency-closeout-receipt-ack-replay-idempotency-closeout-receipt-receipt-receipt-receipt-receipt-receipt-application-preview-report.sh"

report="$(
  capture_json_report \
    "hepta-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-replay-idempotency-closeout-receipt-receipt-receipt-receipt-application-preview-report" \
    "$REPORT_SCRIPT"
)"
printf '%s\n' "$report"

jq -e '
  .product == "Hepta"
  and .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_application_preview_gate"
  and .schema_version == "work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_application_preview_v1"
  and .preview_mode == "read_only_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_application_preview_no_mutation"
  and .readback_plan_count == 7
  and .application_plan_count == 7
  and .source_outcome_count == 7
  and .event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview_count == 7
  and .stage_application_count == 6
  and .evidence_field_application_count == 7
  and .guard_application_count == 11
  and .blocker_application_count == 1
  and .application_guard_count == 11
  and .blocker_count == 1
  and .required_prior_gate_count == 143
' >/dev/null <<<"$report"

jq -e '
  (.application_plans | all(
    .application_state == "work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_ready_preview_after_application"
    and .readback_verified_by_preview == true
    and .event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview == true
    and .applies_to_runtime == false
    and .persists_work_graph_events == false
    and .enables_event_store == false
    and .writes_wal == false
    and .writes_checkpoint == false
    and .executes_replay == false
    and .executes_readback == false
    and .enforces_adapter_projection == false
    and .mutates_runtime == false
  ))
  and (.source_outcomes | all(
    .event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview == true
    and .ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_rerun_preview == true
    and .ready_for_append_only_work_graph_events == false
    and .ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt == false
  ))
  and (.stage_applications | all(
    .contract_ready_preview == true
    and .persists_work_graph_events == false
    and .enables_event_store == false
    and .executes_replay == false
    and .executes_readback == false
  ))
' >/dev/null <<<"$report"

jq -e '
  (.evidence_field_applications | all(
    .evidence_contract_ready_preview == true
    and .persists_evidence == false
  ))
  and (.guard_applications | all(.mutates_runtime == false))
  and (.blocker_applications | all(
    .readback_verified_by_preview == true
    and .mutates_runtime == false
  ))
  and (.application_guards | all(
    .required_before_append_only_events == true
    and .satisfied_by_preview == true
  ))
  and (.blockers | map(.id) == [
    "work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_rerun_missing"
  ])
  and (.blockers | all(.required_before_append_only_events == true))
' >/dev/null <<<"$report"

jq -e '
  (.required_prior_gates | length == (unique | length))
  and (.required_prior_gates[-1] == "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_readback_preview_gate")
  and .recommended_next_gate == "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_rerun_preview_gate"
  and .ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_rerun_preview == true
  and .ready_for_append_only_work_graph_events == false
  and .ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt == false
  and .ready_for_replay_readback_execution == false
  and .ready_for_runtime_adapter_enforcement == false
  and .ready_for_live_execution == false
  and .source_probes.append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_application.rust_module_present == true
  and .source_probes.append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_application.report_script_present == true
  and .source_probes.append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_application.gate_script_present == true
  and .source_probes.append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_readback.rust_module_present == true
  and .source_probes.append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_readback.gate_script_present == true
  and .source_probes.append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_readback.upstream_gate == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

cargo test --manifest-path "$ROOT/codex-rs/Cargo.toml" -p hepta-runtime \
  work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_application_preview --lib

echo "Hepta WorkGraph append-only WorkGraph events event-store cutover terminal no-cutover receipt acknowledgement replay idempotency closeout application preview gate passed"
