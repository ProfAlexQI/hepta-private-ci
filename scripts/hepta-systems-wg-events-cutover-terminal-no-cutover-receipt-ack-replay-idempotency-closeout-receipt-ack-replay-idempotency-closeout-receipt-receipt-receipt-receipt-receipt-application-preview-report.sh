#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

path_exists() {
  local path="$1"
  [[ -e "$path" ]]
}

bool_for() {
  if "$@"; then
    printf 'true\n'
  else
    printf 'false\n'
  fi
}

readback_report="$(
  capture_json_report \
    "hepta-work-graph-append-only-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-replay-idempotency-closeout-receipt-receipt-receipt-receipt-readback-preview-report" \
    "$ROOT/scripts/hepta-systems-wg-events-cutover-terminal-no-cutover-receipt-ack-replay-idempotency-closeout-receipt-ack-replay-idempotency-closeout-receipt-receipt-receipt-receipt-receipt-readback-preview-report.sh"
)"

application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_application_preview.rs
)"
application_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-wg-events-cutover-terminal-no-cutover-receipt-ack-replay-idempotency-closeout-receipt-ack-replay-idempotency-closeout-receipt-receipt-receipt-receipt-receipt-application-preview-report.sh
)"
application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-wg-events-cutover-terminal-no-cutover-receipt-ack-replay-idempotency-closeout-receipt-ack-replay-idempotency-closeout-receipt-receipt-receipt-receipt-receipt-application-preview-gate.sh
)"
readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_readback_preview.rs
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-wg-events-cutover-terminal-no-cutover-receipt-ack-replay-idempotency-closeout-receipt-ack-replay-idempotency-closeout-receipt-receipt-receipt-receipt-receipt-readback-preview-gate.sh
)"

jq -n \
  --argjson readback "$readback_report" \
  --argjson application_rust_module_present "$application_rust_module_present" \
  --argjson application_report_script_present "$application_report_script_present" \
  --argjson application_gate_script_present "$application_gate_script_present" \
  --argjson readback_rust_module_present "$readback_rust_module_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  '
  def application_plan($plan): {
      application_plan_id: ($plan.source_surface_id + "_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_application"),
      readback_source_surface_id: $plan.source_surface_id,
      source_category: $plan.source_category,
      event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_plan_id: $plan.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_plan_id,
      application_state: "work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_ready_preview_after_application",
      readback_verified_by_preview: ($plan.readback_status == "readback_plan_ready"),
      event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview: true,
      applies_to_runtime: false,
      persists_work_graph_events: false,
      enables_event_store: false,
      writes_wal: false,
      writes_checkpoint: false,
      executes_replay: false,
      executes_readback: false,
      enforces_adapter_projection: false,
      mutates_runtime: false
    };
  def source_outcome($plan): {
      source_surface_id: $plan.readback_source_surface_id,
      source_category: $plan.source_category,
      application_plan_id: $plan.application_plan_id,
      post_application_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_state: $plan.application_state,
      event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview: $plan.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview,
      ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_rerun_preview: true,
      ready_for_append_only_work_graph_events: false,
      ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt: false
    };
  def stage_application($assertion): {
      application_id: ($assertion.stage_id + "_stage_application"),
      stage_id: $assertion.stage_id,
      affected_source_surface_ids: $assertion.affected_source_surface_ids,
      required_contract_ref_ids: $assertion.required_contract_ref_ids,
      contract_ready_preview: $assertion.contract_ready_preview,
      persists_work_graph_events: false,
      enables_event_store: false,
      executes_replay: false,
      executes_readback: false
    };
  def evidence_field_application($assertion): {
      application_id: ($assertion.source_surface_id + "_evidence_field_application"),
      source_surface_id: $assertion.source_surface_id,
      evidence_field_ids: $assertion.evidence_field_ids,
      evidence_contract_ready_preview: $assertion.evidence_contract_ready_preview,
      persists_evidence: false
    };
  def guard_application($assertion): {
      application_id: ($assertion.guard_id + "_guard_application"),
      guard_id: $assertion.guard_id,
      guard_scope: $assertion.guard_scope,
      required_before_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt: $assertion.required_before_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt,
      satisfied_by_preview: $assertion.satisfied_by_preview,
      mutates_runtime: false
    };
  def blocker_application($assertion): {
      application_id: ($assertion.blocker_id + "_blocker_application"),
      blocker_id: $assertion.blocker_id,
      affected_source_surface_ids: $assertion.affected_source_surface_ids,
      affected_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_stage_ids: $assertion.affected_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_stage_ids,
      expected_blocker_state: "mapped_for_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_rerun_preview",
      readback_verified_by_preview: true,
      clears_readback_missing_blocker: ($assertion.blocker_id == "append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_readback_missing"),
      mutates_runtime: false
    };
  def application_guard($id; $severity; $scope): {
      id: $id,
      severity: $severity,
      guard_scope: $scope,
      required_before_append_only_events: true,
      satisfied_by_preview: true
    };
  def application_blocker($id; $severity; $category; $sources; $plans; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_source_surface_ids: $sources,
      affected_application_plan_ids: $plans,
      required_before_append_only_events: true,
      recommended_fix: $fix
    };
  ($readback.readback_plans | map(application_plan(.))) as $application_plans
  | ($application_plans | map(source_outcome(.))) as $source_outcomes
  | ($readback.stage_assertions | map(stage_application(.))) as $stage_applications
  | ($readback.evidence_field_assertions | map(evidence_field_application(.))) as $evidence_field_applications
  | ($readback.guard_assertions | map(guard_application(.))) as $guard_applications
  | ($readback.blocker_mapping_assertions | map(blocker_application(.))) as $blocker_applications
  | [
      application_guard("no_work_graph_event_persistence"; "critical"; "event_store"),
      application_guard("no_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt"; "critical"; "event_store"),
      application_guard("no_wal_write"; "critical"; "wal"),
      application_guard("no_checkpoint_write"; "critical"; "checkpoint"),
      application_guard("no_replay_execution"; "critical"; "replay"),
      application_guard("no_readback_execution"; "critical"; "readback"),
      application_guard("no_adapter_projection_enforcement"; "critical"; "adapter_projection"),
      application_guard("no_git_mutation"; "critical"; "git"),
      application_guard("no_agent_spawn"; "high"; "agent_spawn"),
      application_guard("no_external_send_or_model_invocation"; "high"; "external_effects"),
      application_guard("no_append_only_events_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_without_rerun"; "high"; "readiness_rerun")
    ] as $application_guards
  | ($application_plans | map(.readback_source_surface_id)) as $all_sources
  | ($application_plans | map(.application_plan_id)) as $plan_ids
  | [
      application_blocker("work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_rerun_missing"; "medium"; "readiness_rerun"; $all_sources; $plan_ids; "rerun enforcement readiness after terminal no-cutover receipt acknowledgement replay idempotency closeout outcomes are available")
    ] as $blockers
  | ($readback.required_prior_gates + [$readback.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_application_preview_gate",
      schema_version: "work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_application_preview_v1",
      preview_mode: "read_only_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_application_preview_no_mutation",
      readback_plan_count: $readback.readback_plan_count,
      application_plan_count: ($application_plans | length),
      source_outcome_count: ($source_outcomes | length),
      event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview_count: ($source_outcomes | map(select(.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview)) | length),
      stage_application_count: ($stage_applications | length),
      evidence_field_application_count: ($evidence_field_applications | length),
      guard_application_count: ($guard_applications | length),
      blocker_application_count: ($blocker_applications | length),
      application_guard_count: ($application_guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      application_plans: $application_plans,
      source_outcomes: $source_outcomes,
      stage_applications: $stage_applications,
      evidence_field_applications: $evidence_field_applications,
      guard_applications: $guard_applications,
      blocker_applications: $blocker_applications,
      application_guards: $application_guards,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_rerun_preview_gate",
      ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_rerun_preview: true,
      ready_for_append_only_work_graph_events: false,
      ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt: false,
      ready_for_replay_readback_execution: false,
      ready_for_runtime_adapter_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_application: {
          rust_module_present: $application_rust_module_present,
          report_script_present: $application_report_script_present,
          gate_script_present: $application_gate_script_present
        },
        append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_readback: {
          rust_module_present: $readback_rust_module_present,
          gate_script_present: $readback_gate_script_present,
          upstream_gate: ($readback.gate == "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_readback_preview_gate")
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        work_graph_events_persisted: false,
        event_store_enabled: false,
        wal_written: false,
        checkpoint_written: false,
        replay_executed: false,
        readback_executed: false,
        adapter_projection_enforced: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
