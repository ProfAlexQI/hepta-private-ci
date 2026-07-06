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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-canonical-adapter-inventory-rerun.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$tmpdir/cache"
fi

capture_json_report \
  "hepta-work-graph-unified-projection-enforcement-readiness-runtime-wal-write-boundary-execution-rerun-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-wal-write-boundary-execution-rerun-preview-report.sh" \
  >"$tmpdir/previous.json"
capture_json_report \
  "hepta-work-graph-canonical-adapter-inventory-application-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-canonical-adapter-inventory-application-preview-report.sh" \
  >"$tmpdir/application.json"

rerun_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview.rs
)"
rerun_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-canonical-adapter-inventory-rerun-preview-report.sh
)"
rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-canonical-adapter-inventory-rerun-preview-gate.sh
)"
application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_canonical_adapter_inventory_application_preview.rs
)"
application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-canonical-adapter-inventory-application-preview-gate.sh
)"
previous_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-wal-write-boundary-execution-rerun-preview-gate.sh
)"

jq -n \
  --slurpfile previous "$tmpdir/previous.json" \
  --slurpfile application "$tmpdir/application.json" \
  --argjson rerun_rust_module_present "$rerun_rust_module_present" \
  --argjson rerun_report_script_present "$rerun_report_script_present" \
  --argjson rerun_gate_script_present "$rerun_gate_script_present" \
  --argjson application_rust_module_present "$application_rust_module_present" \
  --argjson application_gate_script_present "$application_gate_script_present" \
  --argjson previous_gate_script_present "$previous_gate_script_present" \
  '
  $previous[0] as $previous
  | $application[0] as $application
  | def unique_order: reduce .[] as $item ([]; if index($item) then . else . + [$item] end);
  def push_unique($values; $value): if ($values | index($value)) then $values else ($values + [$value]) end;
  def cleared_application_blocker($id):
      $id == "canonical_adapter_inventory_readiness_rerun_missing";
  def source_blocker_ids($source):
      [$application.blockers[]
        | select((.affected_source_surface_ids | index($source)) and (cleared_application_blocker(.id) | not))
        | .id] | unique_order;
  def application_covered($source):
      any($application.source_outcomes[]?;
        .source_surface_id == $source
        and .canonical_adapter_inventory_contract_ready_preview == true
        and .applies_to_runtime == false);
  def rerun_decision_for($contract_ready; $source_blockers):
      if ($contract_ready | not) then "deny_canonical_adapter_inventory_application_missing"
      elif ($source_blockers | index("append_only_work_graph_events_disabled")) then "deny_append_only_work_graph_events_disabled"
      elif ($source_blockers | index("canonical_adapter_projection_partial_or_gap")) then "deny_canonical_adapter_projection_partial_or_gap"
      elif ($source_blockers | index("runtime_canonical_adapter_enforcement_disabled")) then "deny_runtime_canonical_adapter_enforcement_disabled"
      else "allow_preview_only"
      end;
  def next_gate_for($decision):
      if $decision == "deny_canonical_adapter_inventory_application_missing" then "hepta_work_graph_canonical_adapter_inventory_application_preview_gate"
      else "hepta_work_graph_append_only_work_graph_events_shadow_write_preview_gate"
      end;
  def rerun_decision($decision):
      application_covered($decision.source_surface_id) as $covered
      | source_blocker_ids($decision.source_surface_id) as $source_blockers
      | rerun_decision_for($covered; $source_blockers) as $rerun_decision
      | {
          source_surface_id: $decision.source_surface_id,
          source_category: $decision.source_category,
          previous_enforcement_decision: $decision.runtime_wal_write_boundary_execution_rerun_enforcement_decision,
          canonical_adapter_inventory_rerun_enforcement_decision: $rerun_decision,
          covered_by_canonical_adapter_inventory_application_preview: $covered,
          canonical_adapter_inventory_contract_ready: $covered,
          canonical_adapter_inventory_applied: false,
          append_only_work_graph_events_enabled: false,
          runtime_canonical_adapter_enforcement_enabled: false,
          scheduler_admission_enforcement_ready: false,
          task_result_enforcement_ready: false,
          role_manifest_enforcement_ready: false,
          residual_source_blocker_ids: $source_blockers,
          next_required_gate: next_gate_for($rerun_decision)
        };
  def residual_blocker($blocker): {
      id: $blocker.id,
      severity: $blocker.severity,
      category: $blocker.category,
      affected_source_surface_ids: $blocker.affected_source_surface_ids,
      required_before_projection_enforcement: true,
      recommended_fix: $blocker.recommended_fix
    };
  def stage($id; $observed; $before; $after; $blockers): {
      id: $id,
      observed_contract_count: $observed,
      ready_contract_count_before: $before,
      ready_contract_count_after: $after,
      hard_blocker_ids: $blockers,
      enforcement_enabled: false,
      next_gate: "hepta_work_graph_append_only_work_graph_events_shadow_write_preview_gate"
    };
  def residual_sources($decisions; $ids):
      reduce $decisions[] as $decision ([]; if any($ids[]; . as $id | $decision.residual_source_blocker_ids | index($id)) then push_unique(.; $decision.source_surface_id) else . end);
  ($previous.decision_deltas | map(rerun_decision(.))) as $decisions
  | ($previous.decision_deltas | map(.source_surface_id)) as $before_application_sources
  | ($decisions | map(select(.canonical_adapter_inventory_rerun_enforcement_decision == "deny_canonical_adapter_inventory_application_missing") | .source_surface_id)) as $after_application_sources
  | ($application.blockers | map(select(cleared_application_blocker(.id) | not) | residual_blocker(.))) as $residual_blockers
  | (residual_sources($decisions; ["canonical_adapter_projection_partial_or_gap"])) as $partial_gap_sources
  | ([
      stage("canonical_adapter_inventory_contracts"; ($application.source_outcomes | length); 0; ($decisions | map(select(.covered_by_canonical_adapter_inventory_application_preview)) | length); ["canonical_adapter_inventory_readiness_rerun_missing"]),
      stage("append_only_work_graph_events_shadow_write"; ($decisions | length); 0; 0; ["append_only_work_graph_events_disabled"]),
      stage("canonical_adapter_partial_gap_closure"; ($partial_gap_sources | length); 0; 0; ["canonical_adapter_projection_partial_or_gap"]),
      stage("runtime_canonical_adapter_enforcement_dry_run"; ($decisions | length); 0; 0; ["runtime_canonical_adapter_enforcement_disabled"]),
      stage("projection_enforcement_dry_run"; ($decisions | length); 0; 0; ["append_only_work_graph_events_disabled","canonical_adapter_projection_partial_or_gap","runtime_canonical_adapter_enforcement_disabled"])
    ]) as $stages
  | ($application.required_prior_gates + [$application.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview_gate",
      schema_version: "work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview_v1",
      preview_mode: "read_only_projection_enforcement_readiness_canonical_adapter_inventory_rerun_no_enforcement",
      source_surface_count: ($decisions | length),
      canonical_adapter_inventory_outcome_count: ($application.source_outcomes | length),
      canonical_adapter_inventory_application_covered_source_count: ($decisions | map(select(.covered_by_canonical_adapter_inventory_application_preview)) | length),
      previous_ready_surface_count: ($previous.decision_deltas | map(select(.runtime_wal_write_boundary_execution_rerun_enforcement_decision == "allow_preview_only")) | length),
      canonical_adapter_inventory_contract_ready_source_count: ($decisions | map(select(.canonical_adapter_inventory_contract_ready)) | length),
      previous_canonical_inventory_primary_blocked_surface_count: ($before_application_sources | length),
      canonical_inventory_primary_blocked_surface_count_after: ($after_application_sources | length),
      append_only_work_graph_events_primary_blocked_surface_count: ($decisions | map(select(.canonical_adapter_inventory_rerun_enforcement_decision == "deny_append_only_work_graph_events_disabled")) | length),
      partial_or_gap_blocked_surface_count: ($partial_gap_sources | length),
      append_only_work_graph_events_enabled_source_count: ($decisions | map(select(.append_only_work_graph_events_enabled)) | length),
      runtime_canonical_adapter_enforcement_enabled_source_count: ($decisions | map(select(.runtime_canonical_adapter_enforcement_enabled)) | length),
      rerun_ready_surface_count: ($decisions | map(select(.canonical_adapter_inventory_rerun_enforcement_decision == "allow_preview_only")) | length),
      rerun_blocked_surface_count: ($decisions | map(select(.canonical_adapter_inventory_rerun_enforcement_decision != "allow_preview_only")) | length),
      decision_delta_count: ($decisions | length),
      cleared_blocker_count: 1,
      residual_blocker_count: ($residual_blockers | length),
      enforcement_stage_count: ($stages | length),
      required_prior_gate_count: ($required_prior_gates | length),
      decision_deltas: $decisions,
      cleared_blockers: [{
        id: "canonical_adapter_inventory_application_required_for_enforcement",
        cleared_source_surface_ids: $before_application_sources,
        source_count_before: ($before_application_sources | length),
        source_count_after: ($after_application_sources | length),
        closure_gate_id: "hepta_work_graph_canonical_adapter_inventory_application_preview_gate"
      }],
      residual_blockers: $residual_blockers,
      enforcement_stages: $stages,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_append_only_work_graph_events_shadow_write_preview_gate",
      ready_for_append_only_work_graph_events_preview: true,
      ready_for_append_only_work_graph_events: false,
      ready_for_runtime_adapter_enforcement: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_task_result_enforcement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        canonical_adapter_inventory_readiness_rerun: {
          rust_module_present: $rerun_rust_module_present,
          report_script_present: $rerun_report_script_present,
          gate_script_present: $rerun_gate_script_present
        },
        canonical_adapter_inventory_application: {
          rust_module_present: $application_rust_module_present,
          gate_script_present: $application_gate_script_present,
          upstream_gate: ($application.gate == "hepta_work_graph_canonical_adapter_inventory_application_preview_gate")
        },
        runtime_wal_write_boundary_execution_readiness_rerun: {
          upstream_gate: ($previous.gate == "hepta_work_graph_unified_projection_enforcement_readiness_runtime_wal_write_boundary_execution_rerun_preview_gate"),
          gate_script_present: $previous_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        work_graph_events_persisted: false,
        adapter_projection_enforced: false,
        runtime_mutation_performed: false,
        scheduler_admission_enforced: false,
        task_result_enforcement_enabled: false,
        role_manifest_enforcement_enabled: false,
        approval_recorded: false,
        side_effect_lock_established: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }
  '
