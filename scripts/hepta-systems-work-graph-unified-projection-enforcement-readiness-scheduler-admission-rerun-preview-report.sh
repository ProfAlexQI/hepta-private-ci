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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-enforcement-readiness-scheduler-admission-rerun.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-unified-projection-enforcement-readiness-append-only-store-rerun-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-append-only-store-rerun-preview-report.sh" \
  >"$tmpdir/append_only_store_rerun.json"
capture_json_report \
  "hepta-work-graph-scheduler-admission-enforcement-gap-closure-application-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-application-preview-report.sh" \
  >"$tmpdir/scheduler_admission_application.json"

scheduler_rerun_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview.rs
)"
scheduler_rerun_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-scheduler-admission-rerun-preview-report.sh
)"
scheduler_rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-scheduler-admission-rerun-preview-gate.sh
)"
scheduler_application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_scheduler_admission_enforcement_gap_closure_application_preview.rs
)"
scheduler_application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-scheduler-admission-enforcement-gap-closure-application-preview-gate.sh
)"
append_only_rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-append-only-store-rerun-preview-gate.sh
)"

jq -n \
  --slurpfile append_only "$tmpdir/append_only_store_rerun.json" \
  --slurpfile application "$tmpdir/scheduler_admission_application.json" \
  --argjson scheduler_rerun_rust_module_present "$scheduler_rerun_rust_module_present" \
  --argjson scheduler_rerun_report_script_present "$scheduler_rerun_report_script_present" \
  --argjson scheduler_rerun_gate_script_present "$scheduler_rerun_gate_script_present" \
  --argjson scheduler_application_rust_module_present "$scheduler_application_rust_module_present" \
  --argjson scheduler_application_gate_script_present "$scheduler_application_gate_script_present" \
  --argjson append_only_rerun_gate_script_present "$append_only_rerun_gate_script_present" \
  '
  $append_only[0] as $append_only
  | $application[0] as $application
  | def unique_order: reduce .[] as $item ([]; if index($item) then . else . + [$item] end);
  def has_suffix($values; $suffix): any($values[]?; endswith($suffix));
  def has_contains($values; $needle): any($values[]?; contains($needle));
  def push_unique($values; $value): if ($values | index($value)) then $values else ($values + [$value]) end;
  def union_sources($left; $right): reduce $right[] as $item ($left; push_unique(.; $item));
  def application_source_ids($plans): [$plans[] | .source_surface_id] | unique_order;
  def scheduler_decision_for($store; $timeline; $task_result; $route; $guard; $terminal_ready; $precondition_ready; $readback; $scheduler_ready; $route_blockers; $source_blockers):
      if ($store | not) then "deny_missing_unified_store_projection"
      elif ($timeline | not) then "deny_missing_timeline_projection"
      elif ($task_result | not) then "deny_missing_task_result_projection"
      elif ($route | not) then "deny_missing_append_only_route"
      elif ($guard | not) then "deny_missing_store_idempotency_guard"
      elif ($terminal_ready | not) then "deny_terminal_task_result_contract_missing"
      elif ($precondition_ready | not) then "deny_append_only_store_precondition_missing"
      elif ($readback | not) then "deny_missing_readback_probe"
      elif ($scheduler_ready | not) then "deny_scheduler_admission_not_enforced"
      elif has_contains($source_blockers; "role_manifest_not_enforced") then "deny_role_manifest_not_enforced"
      elif ($source_blockers | index("append_only_store_runtime_enablement_disabled")) then "deny_runtime_append_only_store_enablement_disabled"
      elif ($route_blockers | index("append_only_store_disabled_by_design")) then "deny_append_only_store_disabled"
      else "allow_preview_only"
      end;
  def next_gate_for($decision):
      if $decision == "deny_role_manifest_not_enforced" then "hepta_work_graph_role_manifest_contract_preview_gate"
      elif $decision == "deny_runtime_append_only_store_enablement_disabled" then "hepta_work_graph_append_only_store_runtime_enablement_preview_gate"
      elif $decision == "allow_preview_only" then "hepta_work_graph_projection_enforcement_dry_run_preview_gate"
      else "hepta_work_graph_role_manifest_enforcement_gap_closure_preview_gate"
      end;
  def scheduler_decision($decision):
      (any($application.source_outcomes[]?; .source_surface_id == $decision.source_surface_id and .scheduler_admission_contract_ready_preview == true and .applies_to_runtime == false)) as $covered
      | ((has_suffix($decision.residual_source_blocker_ids; "_admission_not_enforced") | not) or $covered) as $scheduler_ready
      | (($decision.append_only_store_rerun_enforcement_decision == "deny_scheduler_admission_not_enforced") and $scheduler_ready) as $primary_gap_closed
      | ($decision.residual_source_blocker_ids | map(select(endswith("_admission_not_enforced") | not))) as $source_without_admission_blocker
      | (if $covered
          then push_unique(push_unique(push_unique(push_unique(push_unique($source_without_admission_blocker; "scheduler_admission_runtime_application_disabled"); "lane_lease_acquisition_disabled"); "dependency_readback_not_executed"); "approval_recording_disabled"); "budget_consumption_disabled")
          else $source_without_admission_blocker
        end) as $source_blockers
      | (scheduler_decision_for($decision.unified_store_projection_ready; $decision.timeline_projection_ready; $decision.task_result_projection_ready; $decision.append_only_route_ready; $decision.store_idempotency_guard_ready; $decision.terminal_task_result_contract_ready; $decision.append_only_store_precondition_ready; $decision.readback_probe_contract_ready; $scheduler_ready; $decision.residual_route_blocker_ids; $source_blockers)) as $rerun_decision
      | {
          source_surface_id: $decision.source_surface_id,
          source_category: $decision.source_category,
          previous_projection_coverage_state: $decision.previous_projection_coverage_state,
          previous_store_guard_rerun_state: $decision.previous_store_guard_rerun_state,
          previous_terminal_task_result_rerun_state: $decision.previous_terminal_task_result_rerun_state,
          previous_append_only_store_rerun_state: $decision.append_only_store_rerun_state,
          scheduler_admission_rerun_state: (if $covered then "scheduler_admission_contract_ready_preview_after_application" else "scheduler_admission_not_required_for_source" end),
          covered_by_scheduler_admission_application_preview: $covered,
          previous_enforcement_decision: $decision.append_only_store_rerun_enforcement_decision,
          scheduler_admission_rerun_enforcement_decision: $rerun_decision,
          scheduler_admission_primary_gap_closed_by_application_preview: $primary_gap_closed,
          projection_contract_ready: $decision.projection_contract_ready,
          unified_store_projection_ready: $decision.unified_store_projection_ready,
          timeline_projection_ready: $decision.timeline_projection_ready,
          task_result_projection_ready: $decision.task_result_projection_ready,
          store_idempotency_guard_ready: $decision.store_idempotency_guard_ready,
          terminal_task_result_contract_ready: $decision.terminal_task_result_contract_ready,
          append_only_route_ready: $decision.append_only_route_ready,
          append_only_store_precondition_ready: $decision.append_only_store_precondition_ready,
          readback_probe_contract_ready: $decision.readback_probe_contract_ready,
          scheduler_admission_contract_ready: $scheduler_ready,
          scheduler_admission_enforcement_ready: false,
          role_manifest_enforcement_ready: false,
          runtime_append_only_store_enabled: false,
          residual_source_blocker_ids: $source_blockers,
          residual_route_blocker_ids: $decision.residual_route_blocker_ids,
          next_required_gate: next_gate_for($rerun_decision)
        };
  def residual_blocker($id; $severity; $affected; $fix): {
      id: $id,
      severity: $severity,
      affected_source_surface_ids: $affected,
      required_before_projection_enforcement: true,
      recommended_fix: $fix
    };
  def stage($id; $observed; $before; $after; $blockers; $next): {
      id: $id,
      observed_contract_count: $observed,
      ready_contract_count_before: $before,
      ready_contract_count_after: $after,
      hard_blocker_ids: $blockers,
      enforcement_enabled: false,
      next_gate: $next
    };
  ($append_only.decision_deltas | map(scheduler_decision(.))) as $decisions
  | ($append_only.decision_deltas | map(select(.append_only_store_rerun_enforcement_decision == "deny_scheduler_admission_not_enforced") | .source_surface_id)) as $before_scheduler_gap_sources
  | ($decisions | map(select(.scheduler_admission_rerun_enforcement_decision == "deny_scheduler_admission_not_enforced") | .source_surface_id)) as $after_scheduler_gap_sources
  | ($decisions | map(select(.previous_projection_coverage_state == "contract_ready_preview_after_application") | .source_surface_id)) as $projection_sources
  | ($decisions | map(select(.previous_store_guard_rerun_state == "store_guard_contract_ready_preview_after_application") | .source_surface_id)) as $store_sources
  | ($decisions | map(select(.previous_terminal_task_result_rerun_state == "terminal_task_result_contract_ready_preview_after_application") | .source_surface_id)) as $terminal_sources
  | (application_source_ids($application.application_plans)) as $scheduler_sources
  | ($decisions | map(select(.residual_source_blocker_ids | index("append_only_store_runtime_enablement_disabled")) | .source_surface_id)) as $runtime_append_only_sources
  | ($decisions | map(select(has_contains(.residual_source_blocker_ids; "role_manifest_not_enforced")) | .source_surface_id)) as $role_sources
  | (union_sources($terminal_sources; $scheduler_sources)) as $operator_sources
  | [
      {
        id: "scheduler_admission_not_enforced_for_enforcement",
        cleared_source_surface_ids: $before_scheduler_gap_sources,
        source_count_before: ($before_scheduler_gap_sources | length),
        source_count_after: ($after_scheduler_gap_sources | length),
        closure_gate_id: "hepta_work_graph_scheduler_admission_enforcement_gap_closure_application_preview_gate"
      }
    ] as $cleared_blockers
  | [
      residual_blocker("projection_adapter_runtime_closure_application_disabled"; "high"; $projection_sources; "keep projection adapter closures preview-only until runtime application gates are promoted"),
      residual_blocker("store_guard_runtime_application_disabled"; "high"; $store_sources; "attach store idempotency guards to runtime adapters only after persistence and operator-review gates are promoted"),
      residual_blocker("idempotency_index_mutation_disabled"; "critical"; $runtime_append_only_sources; "keep idempotency indexes immutable until mutation policy and replay evidence are enforced"),
      residual_blocker("state_store_guard_persistence_disabled"; "high"; $store_sources; "do not persist candidate guard rows until append-only store intake is promoted"),
      residual_blocker("terminal_task_result_runtime_application_disabled"; "high"; $terminal_sources; "attach terminal TaskResult wrappers to runtime only after persistence, replay, and operator-review gates are promoted"),
      residual_blocker("task_result_persistence_disabled"; "high"; $terminal_sources; "keep TaskResult rows preview-only until append-only store intake is promoted"),
      residual_blocker("append_only_store_runtime_enablement_disabled"; "critical"; $runtime_append_only_sources; "keep durable store enablement disabled until WAL, readback, rollback, and operator readiness gates are promoted"),
      residual_blocker("wal_write_boundary_not_enabled"; "critical"; $runtime_append_only_sources; "preserve no-WAL boundary until append-only event intake and replay receipts are promoted"),
      residual_blocker("rollback_readback_not_executed"; "critical"; $runtime_append_only_sources; "execute rollback and readback gates before any append-only store enablement"),
      residual_blocker("scheduler_admission_runtime_application_disabled"; "high"; $scheduler_sources; "keep scheduler admission runtime application disabled until role manifests, leases, budgets, approvals, and store writes are promoted"),
      residual_blocker("lane_lease_acquisition_disabled"; "critical"; $scheduler_sources; "do not acquire or mutate lane leases from the scheduler admission readiness rerun"),
      residual_blocker("dependency_readback_not_executed"; "high"; $scheduler_sources; "read back dependency terminal states before scheduler admission can become authoritative"),
      residual_blocker("approval_recording_disabled"; "critical"; $scheduler_sources; "approval evidence must be recorded by a later runtime boundary, not this rerun"),
      residual_blocker("budget_consumption_disabled"; "high"; $scheduler_sources; "budget checks remain contract-only and cannot consume resource or retry budget"),
      residual_blocker("role_manifest_not_enforced"; "medium"; $role_sources; "bind multi-agent, batch, worker, and handoff sources to role manifests with budgets and tool permissions"),
      residual_blocker("operator_review_required"; "high"; $operator_sources; "operator review must accept store enablement, scheduler admission, and role bindings before promotion"),
      residual_blocker("runtime_application_residuals_not_promoted"; "high"; $projection_sources; "promote projection adapter and store guard runtime applications only after readback and operator-review gates are satisfied")
    ] as $residual_blockers
  | [
      stage("unified_projection_contracts"; ($decisions | length); ($decisions | map(select(.projection_contract_ready)) | length); ($decisions | map(select(.projection_contract_ready)) | length); ["projection_adapter_runtime_closure_application_disabled"]; "hepta_work_graph_role_manifest_enforcement_gap_closure_preview_gate"),
      stage("store_idempotency_guard_contracts"; ($decisions | length); ($decisions | map(select(.store_idempotency_guard_ready)) | length); ($decisions | map(select(.store_idempotency_guard_ready)) | length); ["store_guard_runtime_application_disabled"]; "hepta_work_graph_role_manifest_enforcement_gap_closure_preview_gate"),
      stage("terminal_task_result_contracts"; ($terminal_sources | length); ($terminal_sources | length); ($terminal_sources | length); ["terminal_task_result_runtime_application_disabled","task_result_persistence_disabled"]; "hepta_work_graph_role_manifest_enforcement_gap_closure_preview_gate"),
      stage("append_only_store_preconditions"; ($decisions | length); ($decisions | map(select(.append_only_store_precondition_ready)) | length); ($decisions | map(select(.append_only_store_precondition_ready)) | length); ["append_only_store_runtime_enablement_disabled","wal_write_boundary_not_enabled","rollback_readback_not_executed"]; "hepta_work_graph_role_manifest_enforcement_gap_closure_preview_gate"),
      stage("append_only_store_runtime_enablement"; ($runtime_append_only_sources | length); 0; 0; ["append_only_store_runtime_enablement_disabled","wal_write_boundary_not_enabled","idempotency_index_mutation_disabled","rollback_readback_not_executed","operator_review_required"]; "hepta_work_graph_role_manifest_enforcement_gap_closure_preview_gate"),
      stage("scheduler_admission_contracts"; ($application.source_outcomes | length); 0; ($application.source_outcomes | map(select(.scheduler_admission_contract_ready_preview)) | length); ["scheduler_admission_runtime_application_disabled"]; "hepta_work_graph_role_manifest_enforcement_gap_closure_preview_gate"),
      stage("scheduler_admission_runtime_application"; ($scheduler_sources | length); 0; 0; ["scheduler_admission_runtime_application_disabled","lane_lease_acquisition_disabled","approval_recording_disabled","budget_consumption_disabled"]; "hepta_work_graph_role_manifest_enforcement_gap_closure_preview_gate"),
      stage("role_manifest_contracts"; ($role_sources | length); 0; 0; ["role_manifest_not_enforced"]; "hepta_work_graph_role_manifest_contract_preview_gate")
    ] as $stages
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview_gate",
      schema_version: "work_graph_unified_projection_enforcement_readiness_scheduler_admission_rerun_preview_v1",
      preview_mode: "read_only_projection_enforcement_readiness_scheduler_admission_rerun_no_enforcement",
      source_surface_count: ($decisions | length),
      scheduler_admission_application_outcome_count: ($application.source_outcomes | length),
      previous_contract_ready_surface_count: ($append_only.decision_deltas | map(select(.projection_contract_ready)) | length),
      scheduler_admission_rerun_contract_ready_surface_count: ($decisions | map(select(.projection_contract_ready)) | length),
      previous_scheduler_admission_primary_blocked_surface_count: ($before_scheduler_gap_sources | length),
      scheduler_admission_primary_blocked_surface_count_after: ($after_scheduler_gap_sources | length),
      scheduler_admission_application_source_count: ($scheduler_sources | length),
      scheduler_admission_contract_ready_surface_count: ($decisions | map(select(.scheduler_admission_contract_ready)) | length),
      scheduler_admission_runtime_residual_source_count: ($decisions | map(select(.residual_source_blocker_ids | index("scheduler_admission_runtime_application_disabled"))) | length),
      runtime_append_only_residual_source_count: ($runtime_append_only_sources | length),
      rerun_ready_surface_count: ($decisions | map(select(.scheduler_admission_rerun_enforcement_decision == "allow_preview_only")) | length),
      rerun_blocked_surface_count: ($decisions | map(select(.scheduler_admission_rerun_enforcement_decision != "allow_preview_only")) | length),
      decision_delta_count: ($decisions | length),
      cleared_blocker_count: ($cleared_blockers | length),
      residual_blocker_count: ($residual_blockers | length),
      enforcement_stage_count: ($stages | length),
      required_prior_gate_count: (($application.required_prior_gates + [$application.gate]) | length),
      decision_deltas: $decisions,
      cleared_blockers: $cleared_blockers,
      residual_blockers: $residual_blockers,
      enforcement_stages: $stages,
      required_prior_gates: ($application.required_prior_gates + [$application.gate]),
      recommended_next_gate: "hepta_work_graph_role_manifest_enforcement_gap_closure_preview_gate",
      ready_for_role_manifest_enforcement_gap_closure_preview: true,
      ready_for_projection_enforcement: false,
      ready_for_append_only_store_enablement: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        scheduler_admission_readiness_rerun: {
          rust_module_present: $scheduler_rerun_rust_module_present,
          report_script_present: $scheduler_rerun_report_script_present,
          gate_script_present: $scheduler_rerun_gate_script_present
        },
        scheduler_admission_application: {
          rust_module_present: $scheduler_application_rust_module_present,
          gate_script_present: $scheduler_application_gate_script_present,
          upstream_gate: ($application.gate == "hepta_work_graph_scheduler_admission_enforcement_gap_closure_application_preview_gate")
        },
        append_only_store_readiness_rerun: {
          upstream_gate: ($append_only.gate == "hepta_work_graph_unified_projection_enforcement_readiness_append_only_store_rerun_preview_gate"),
          gate_script_present: $append_only_rerun_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        wal_written: false,
        checkpoint_written: false,
        idempotency_index_mutated: false,
        store_guard_attached: false,
        append_only_store_enabled: false,
        projection_enforcement_enabled: false,
        scheduler_admission_enforced: false,
        lane_lease_acquired: false,
        work_started: false,
        budget_consumed: false,
        approval_recorded: false,
        task_result_enforcement_enabled: false,
        task_result_persisted: false,
        role_manifest_enforcement_enabled: false,
        timeline_persisted: false,
        readback_executed: false,
        closure_applied_to_runtime: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
