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
    "hepta-work-graph-append-only-work-graph-events-replay-readback-readback-preview-report" \
    "$ROOT/scripts/hepta-systems-work-graph-append-only-work-graph-events-replay-readback-readback-preview-report.sh"
)"

application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_work_graph_events_replay_readback_application_preview.rs
)"
application_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-work-graph-events-replay-readback-application-preview-report.sh
)"
application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-work-graph-events-replay-readback-application-preview-gate.sh
)"
readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_work_graph_events_replay_readback_readback_preview.rs
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-work-graph-events-replay-readback-readback-preview-gate.sh
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
    application_plan_id: ($plan.source_surface_id + "_append_only_work_graph_events_replay_readback_application"),
    readback_source_surface_id: $plan.source_surface_id,
    source_category: $plan.source_category,
    replay_readback_plan_id: $plan.replay_readback_plan_id,
    application_state: "work_graph_events_replay_readback_contract_ready_preview_after_application",
    readback_verified_by_preview: ($plan.readback_status == "readback_plan_ready"),
    replay_readback_contract_ready_preview: true,
    applies_to_runtime: false,
    persists_work_graph_events: false,
    writes_wal: false,
    writes_checkpoint: false,
    executes_replay: false,
    executes_readback: false,
    executes_rollback: false,
    mutates_idempotency_index: false,
    enforces_adapter_projection: false
  };
  def source_outcome($plan): {
    source_surface_id: $plan.readback_source_surface_id,
    source_category: $plan.source_category,
    application_plan_id: $plan.application_plan_id,
    post_application_replay_readback_state: $plan.application_state,
    replay_readback_contract_ready_preview: $plan.replay_readback_contract_ready_preview,
    ready_for_replay_readback_readiness_rerun_preview: true,
    ready_for_append_only_work_graph_events: false,
    ready_for_replay_readback_execution: false
  };
  def application_guard($id; $severity; $scope): {
    id: $id,
    severity: $severity,
    guard_scope: $scope,
    required_before_append_only_events: true,
    satisfied_by_preview: true
  };
  def blocker($id; $severity; $category; $sources; $plans; $fix): {
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
  | ($application_plans | map(.readback_source_surface_id)) as $all_sources
  | ($application_plans | map(.application_plan_id)) as $all_plan_ids
  | (($readback.blockers[] | select(.id == "canonical_adapter_projection_partial_or_gap") | .affected_source_surface_ids) // []) as $partial_gap_sources
  | [
      blocker("append_only_work_graph_events_disabled"; "high"; "append_only_fact_source"; $all_sources; $all_plan_ids; "keep WorkGraph event persistence disabled until replay/readback readiness rerun is verified"),
      blocker("replay_readback_execution_disabled"; "high"; "replay_readback_execution"; $all_sources; $all_plan_ids; "keep replay/readback execution disabled until append-only events are promoted behind operator review"),
      blocker("runtime_canonical_adapter_enforcement_disabled"; "high"; "runtime_adapter_enforcement"; $all_sources; $all_plan_ids; "keep canonical adapter enforcement disabled until append-only events replay/readback is promoted"),
      blocker("canonical_adapter_projection_partial_or_gap"; "high"; "projection_coverage"; $partial_gap_sources; $all_plan_ids; "close partial/gap adapter source mappings before authoritative event replay/readback"),
      blocker("work_graph_events_replay_readback_readiness_rerun_missing"; "medium"; "readiness_rerun"; $all_sources; $all_plan_ids; "rerun enforcement readiness after no-execution replay/readback application outcomes are available")
    ] as $blockers
  | [
      application_guard("no_work_graph_event_persistence"; "critical"; "event_store"),
      application_guard("no_wal_write"; "critical"; "wal"),
      application_guard("no_checkpoint_write"; "critical"; "checkpoint"),
      application_guard("no_replay_execution"; "critical"; "replay"),
      application_guard("no_readback_execution"; "critical"; "readback"),
      application_guard("no_rollback_execution"; "critical"; "rollback"),
      application_guard("no_idempotency_index_mutation"; "critical"; "idempotency"),
      application_guard("no_adapter_projection_enforcement"; "critical"; "adapter_projection"),
      application_guard("no_agent_spawn"; "high"; "agent_spawn"),
      application_guard("no_external_send_or_model_invocation"; "high"; "external_effects"),
      application_guard("no_append_only_events_promotion_without_rerun"; "high"; "readiness_rerun")
    ] as $application_guards
  | ($readback.required_prior_gates + ["hepta_work_graph_append_only_work_graph_events_replay_readback_readback_preview_gate"]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_append_only_work_graph_events_replay_readback_application_preview_gate",
      schema_version: "work_graph_append_only_work_graph_events_replay_readback_application_preview_v1",
      preview_mode: "read_only_append_only_work_graph_events_replay_readback_application_preview_no_mutation",
      readback_plan_count: $readback.readback_plan_count,
      application_plan_count: ($application_plans | length),
      source_outcome_count: ($source_outcomes | length),
      replay_readback_contract_ready_preview_count: ($source_outcomes | map(select(.replay_readback_contract_ready_preview == true)) | length),
      stage_application_count: $readback.stage_assertion_count,
      evidence_field_application_count: $readback.evidence_field_assertion_count,
      guard_application_count: $readback.guard_assertion_count,
      blocker_application_count: $readback.blocker_mapping_assertion_count,
      application_guard_count: ($application_guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      application_plans: $application_plans,
      source_outcomes: $source_outcomes,
      stage_applications: ($readback.stage_assertions | map({
        application_id: (.stage_id + "_stage_application"),
        stage_id: .stage_id,
        affected_source_surface_ids: .affected_source_surface_ids,
        required_contract_ref_ids: .required_contract_ref_ids,
        contract_ready_preview: .contract_ready_preview,
        persists_work_graph_events: false,
        executes_replay: false,
        executes_readback: false,
        executes_rollback: false
      })),
      evidence_field_applications: ($readback.evidence_field_assertions | map({
        application_id: (.source_surface_id + "_evidence_field_application"),
        source_surface_id: .source_surface_id,
        evidence_field_ids: .evidence_field_ids,
        evidence_contract_ready_preview: .evidence_contract_ready_preview,
        persists_evidence: false
      })),
      guard_applications: ($readback.guard_assertions | map({
        application_id: (.guard_id + "_guard_application"),
        guard_id: .guard_id,
        guard_scope: .guard_scope,
        required_before_replay_readback_execution: .required_before_replay_readback_execution,
        satisfied_by_preview: .satisfied_by_preview,
        mutates_runtime: false
      })),
      blocker_applications: ($readback.blocker_mapping_assertions | map({
        application_id: (.blocker_id + "_blocker_application"),
        blocker_id: .blocker_id,
        affected_source_surface_ids: .affected_source_surface_ids,
        affected_replay_readback_stage_ids: .affected_replay_readback_stage_ids,
        expected_blocker_state: "mapped_for_work_graph_events_replay_readback_rerun_preview",
        readback_verified_by_preview: true,
        clears_application_missing_blocker: (.blocker_id == "append_only_work_graph_events_replay_readback_readback_missing"),
        mutates_runtime: false
      })),
      application_guards: $application_guards,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_preview_gate",
      ready_for_replay_readback_readiness_rerun_preview: true,
      ready_for_append_only_work_graph_events: false,
      ready_for_replay_readback_execution: false,
      ready_for_runtime_adapter_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        append_only_work_graph_events_replay_readback_application: {
          rust_module_present: $application_rust_module_present,
          report_script_present: $application_report_script_present,
          gate_script_present: $application_gate_script_present
        },
        append_only_work_graph_events_replay_readback_readback: {
          rust_module_present: $readback_rust_module_present,
          gate_script_present: $readback_gate_script_present,
          upstream_gate: ($readback.gate == "hepta_work_graph_append_only_work_graph_events_replay_readback_readback_preview_gate" and $readback.status == "ready")
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        work_graph_events_persisted: false,
        wal_written: false,
        checkpoint_written: false,
        replay_executed: false,
        readback_executed: false,
        rollback_executed: false,
        idempotency_index_mutated: false,
        adapter_projection_enforced: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
