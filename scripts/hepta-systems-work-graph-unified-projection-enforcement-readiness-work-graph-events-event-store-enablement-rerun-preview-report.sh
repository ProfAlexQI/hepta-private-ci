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

previous_report="$(
  capture_json_report \
    "hepta-work-graph-unified-projection-enforcement-readiness-work-graph-events-persistence-guard-rerun-preview-report" \
    "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-persistence-guard-rerun-preview-report.sh"
)"

application_report="$(
  capture_json_report \
    "hepta-work-graph-append-only-work-graph-events-event-store-enablement-application-preview-report" \
    "$ROOT/scripts/hepta-systems-work-graph-append-only-work-graph-events-event-store-enablement-application-preview-report.sh"
)"

rerun_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_enablement_rerun_preview.rs
)"
rerun_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-enablement-rerun-preview-report.sh
)"
rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-enablement-rerun-preview-gate.sh
)"
application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_work_graph_events_event_store_enablement_application_preview.rs
)"
application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-work-graph-events-event-store-enablement-application-preview-gate.sh
)"

jq -n \
  --argjson previous "$previous_report" \
  --argjson application "$application_report" \
  --argjson rerun_rust_module_present "$rerun_rust_module_present" \
  --argjson rerun_report_script_present "$rerun_report_script_present" \
  --argjson rerun_gate_script_present "$rerun_gate_script_present" \
  --argjson application_rust_module_present "$application_rust_module_present" \
  --argjson application_gate_script_present "$application_gate_script_present" \
  '
  def cleared_application_blocker($id):
    $id == "work_graph_events_event_store_enablement_readiness_rerun_missing";
  def source_blockers($source):
    [$application.blockers[]
      | select((cleared_application_blocker(.id) | not) and (.affected_source_surface_ids | index($source)))
      | .id];
  def rerun_decision($covered; $blockers):
    if ($covered | not) then "deny_work_graph_events_event_store_enablement_application_missing"
    elif ($blockers | index("append_only_work_graph_events_disabled")) then "deny_append_only_work_graph_events_disabled"
    elif ($blockers | index("replay_readback_execution_disabled")) then "deny_replay_readback_execution_disabled"
    elif ($blockers | index("canonical_adapter_projection_partial_or_gap")) then "deny_canonical_adapter_projection_partial_or_gap"
    elif ($blockers | index("runtime_canonical_adapter_enforcement_disabled")) then "deny_runtime_canonical_adapter_enforcement_disabled"
    else "allow_preview_only"
    end;
  def next_gate($decision):
    if $decision == "deny_work_graph_events_event_store_enablement_application_missing"
    then "hepta_work_graph_append_only_work_graph_events_event_store_enablement_application_preview_gate"
    else "hepta_work_graph_append_only_work_graph_events_event_store_activation_preview_gate"
    end;
  def residual_blocker($blocker): {
      id: $blocker.id,
      severity: $blocker.severity,
      category: $blocker.category,
      affected_source_surface_ids: $blocker.affected_source_surface_ids,
      required_before_projection_enforcement: true,
      recommended_fix: $blocker.recommended_fix
    };
  def rerun_stage($id; $observed; $before; $after; $blockers): {
      id: $id,
      observed_contract_count: $observed,
      ready_contract_count_before: $before,
      ready_contract_count_after: $after,
      hard_blocker_ids: $blockers,
      enforcement_enabled: false,
      next_gate: "hepta_work_graph_append_only_work_graph_events_event_store_activation_preview_gate"
    };
  ($previous.decision_deltas | map(
    . as $previous_decision
    | ($application.source_outcomes | map(select(
        .source_surface_id == $previous_decision.source_surface_id
        and .event_store_enablement_contract_ready_preview == true
        and .ready_for_event_store_enablement == false
      )) | length > 0) as $covered
    | (source_blockers($previous_decision.source_surface_id)) as $source_blocker_ids
    | (rerun_decision($covered; $source_blocker_ids)) as $decision
    | {
        source_surface_id: $previous_decision.source_surface_id,
        source_category: $previous_decision.source_category,
        previous_enforcement_decision: $previous_decision.work_graph_events_persistence_guard_rerun_enforcement_decision,
        work_graph_events_event_store_enablement_rerun_enforcement_decision: $decision,
        covered_by_event_store_enablement_application_preview: $covered,
        event_store_enablement_contract_ready: $covered,
        event_store_enablement_application_applied: false,
        append_only_work_graph_events_enabled: false,
        event_store_enabled: false,
        replay_readback_execution_enabled: false,
        runtime_canonical_adapter_enforcement_enabled: false,
        residual_source_blocker_ids: $source_blocker_ids,
        next_required_gate: next_gate($decision)
      }
    )) as $decision_deltas
  | ($previous.decision_deltas
      | map(select(.work_graph_events_persistence_guard_rerun_enforcement_decision == "deny_append_only_work_graph_events_disabled") | .source_surface_id)) as $cleared_sources
  | ($decision_deltas
      | map(select(.work_graph_events_event_store_enablement_rerun_enforcement_decision == "deny_work_graph_events_event_store_enablement_application_missing"))) as $missing_after
  | [{
      id: "work_graph_events_event_store_enablement_application_required_for_enforcement",
      cleared_source_surface_ids: $cleared_sources,
      source_count_before: ($cleared_sources | length),
      source_count_after: ($missing_after | length),
      closure_gate_id: "hepta_work_graph_append_only_work_graph_events_event_store_enablement_application_preview_gate"
    }] as $cleared_blockers
  | ($application.blockers | map(select(cleared_application_blocker(.id) | not) | residual_blocker(.))) as $residual_blockers
  | ($decision_deltas | map(select(.residual_source_blocker_ids | index("canonical_adapter_projection_partial_or_gap"))) | length) as $partial_gap_count
  | [
      rerun_stage("work_graph_events_event_store_enablement_contracts"; ($application.source_outcome_count); 0; ($decision_deltas | map(select(.covered_by_event_store_enablement_application_preview)) | length); ["work_graph_events_event_store_enablement_readiness_rerun_missing"]),
      rerun_stage("append_only_work_graph_events_persistence"; ($decision_deltas | length); 0; 0; ["append_only_work_graph_events_disabled"]),
      rerun_stage("event_store_activation_readiness"; ($decision_deltas | length); 0; 0; ["append_only_work_graph_events_disabled"]),
      rerun_stage("replay_readback_execution_readiness"; ($decision_deltas | length); 0; 0; ["replay_readback_execution_disabled"]),
      rerun_stage("canonical_adapter_partial_gap_closure"; $partial_gap_count; 0; 0; ["canonical_adapter_projection_partial_or_gap"]),
      rerun_stage("runtime_canonical_adapter_enforcement_dry_run"; ($decision_deltas | length); 0; 0; ["runtime_canonical_adapter_enforcement_disabled"])
    ] as $enforcement_stages
  | ($application.required_prior_gates + [$application.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_enablement_rerun_preview_gate",
      schema_version: "work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_enablement_rerun_preview_v1",
      preview_mode: "read_only_projection_enforcement_readiness_work_graph_events_event_store_enablement_rerun_no_enforcement",
      source_surface_count: ($previous.source_surface_count),
      event_store_enablement_outcome_count: ($application.source_outcome_count),
      event_store_enablement_application_covered_source_count: ($decision_deltas | map(select(.covered_by_event_store_enablement_application_preview)) | length),
      previous_ready_surface_count: ($previous.decision_deltas | map(select(.work_graph_events_persistence_guard_rerun_enforcement_decision == "allow_preview_only")) | length),
      event_store_enablement_contract_ready_source_count: ($decision_deltas | map(select(.event_store_enablement_contract_ready)) | length),
      previous_append_only_work_graph_events_primary_blocked_surface_count: ($cleared_sources | length),
      event_store_enablement_application_missing_surface_count_after: ($missing_after | length),
      append_only_work_graph_events_primary_blocked_surface_count: ($decision_deltas | map(select(.work_graph_events_event_store_enablement_rerun_enforcement_decision == "deny_append_only_work_graph_events_disabled")) | length),
      replay_readback_execution_blocked_surface_count: ($decision_deltas | map(select(.residual_source_blocker_ids | index("replay_readback_execution_disabled"))) | length),
      partial_or_gap_blocked_surface_count: $partial_gap_count,
      append_only_work_graph_events_enabled_source_count: 0,
      event_store_enabled_source_count: 0,
      replay_readback_enabled_source_count: 0,
      runtime_canonical_adapter_enforcement_enabled_source_count: 0,
      rerun_ready_surface_count: ($decision_deltas | map(select(.work_graph_events_event_store_enablement_rerun_enforcement_decision == "allow_preview_only")) | length),
      rerun_blocked_surface_count: ($decision_deltas | map(select(.work_graph_events_event_store_enablement_rerun_enforcement_decision != "allow_preview_only")) | length),
      decision_delta_count: ($decision_deltas | length),
      cleared_blocker_count: ($cleared_blockers | length),
      residual_blocker_count: ($residual_blockers | length),
      enforcement_stage_count: ($enforcement_stages | length),
      required_prior_gate_count: ($required_prior_gates | length),
      decision_deltas: $decision_deltas,
      cleared_blockers: $cleared_blockers,
      residual_blockers: $residual_blockers,
      enforcement_stages: $enforcement_stages,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_append_only_work_graph_events_event_store_activation_preview_gate",
      ready_for_event_store_activation_preview: true,
      ready_for_append_only_work_graph_events: false,
      ready_for_event_store_enablement: false,
      ready_for_replay_readback: false,
      ready_for_runtime_adapter_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        unified_projection_enforcement_readiness_work_graph_events_event_store_enablement_rerun: {
          rust_module_present: $rerun_rust_module_present,
          report_script_present: $rerun_report_script_present,
          gate_script_present: $rerun_gate_script_present
        },
        append_only_work_graph_events_event_store_enablement_application: {
          rust_module_present: $application_rust_module_present,
          gate_script_present: $application_gate_script_present,
          upstream_gate: ($application.gate == "hepta_work_graph_append_only_work_graph_events_event_store_enablement_application_preview_gate")
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
        approval_recorded: false,
        side_effect_lock_established: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
