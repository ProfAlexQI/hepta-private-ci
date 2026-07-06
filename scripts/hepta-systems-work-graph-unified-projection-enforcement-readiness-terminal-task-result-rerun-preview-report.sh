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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-enforcement-readiness-terminal-task-result-rerun.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-unified-projection-enforcement-readiness-store-guard-rerun-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-store-guard-rerun-preview-report.sh" \
  >"$tmpdir/store_guard_rerun.json"
capture_json_report \
  "hepta-work-graph-terminal-task-result-enforcement-gap-closure-application-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-application-preview-report.sh" \
  >"$tmpdir/terminal_task_result_application.json"

terminal_rerun_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview.rs
)"
terminal_rerun_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-terminal-task-result-rerun-preview-report.sh
)"
terminal_rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-terminal-task-result-rerun-preview-gate.sh
)"
terminal_application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_terminal_task_result_enforcement_gap_closure_application_preview.rs
)"
terminal_application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-application-preview-gate.sh
)"

jq -n \
  --slurpfile store "$tmpdir/store_guard_rerun.json" \
  --slurpfile application "$tmpdir/terminal_task_result_application.json" \
  --argjson terminal_rerun_rust_module_present "$terminal_rerun_rust_module_present" \
  --argjson terminal_rerun_report_script_present "$terminal_rerun_report_script_present" \
  --argjson terminal_rerun_gate_script_present "$terminal_rerun_gate_script_present" \
  --argjson terminal_application_rust_module_present "$terminal_application_rust_module_present" \
  --argjson terminal_application_gate_script_present "$terminal_application_gate_script_present" \
  '
  $store[0] as $store
  | $application[0] as $application
  | def outcome_for($id): [$application.source_outcomes[] | select(.source_surface_id == $id)][0] // null;
  def plan_for($id): [$application.application_plans[] | select(.source_surface_id == $id)][0] // null;
  def has_suffix($values; $suffix): any($values[]?; endswith($suffix));
  def has_contains($values; $needle): any($values[]?; contains($needle));
  def terminal_decision_for($store; $timeline; $task_result; $route; $guard; $terminal; $readback; $route_blockers; $source_blockers):
      if ($store | not) then "deny_missing_unified_store_projection"
      elif ($timeline | not) then "deny_missing_timeline_projection"
      elif ($task_result | not) then "deny_missing_task_result_projection"
      elif ($route | not) then "deny_missing_append_only_route"
      elif ($guard | not) then "deny_missing_store_idempotency_guard"
      elif ($terminal | not) then "deny_terminal_task_result_contract_missing"
      elif ($readback | not) then "deny_missing_readback_probe"
      elif has_suffix($source_blockers; "_admission_not_enforced") then "deny_scheduler_admission_not_enforced"
      elif has_contains($source_blockers; "role_manifest_not_enforced") then "deny_role_manifest_not_enforced"
      elif ($route_blockers | index("append_only_store_disabled_by_design")) then "deny_append_only_store_disabled"
      else "allow_preview_only"
      end;
  def next_gate_for($decision):
      if $decision == "deny_scheduler_admission_not_enforced" then "hepta_work_graph_scheduler_admission_controller_preview_gate"
      elif $decision == "deny_role_manifest_not_enforced" then "hepta_work_graph_role_manifest_contract_preview_gate"
      elif $decision == "deny_append_only_store_disabled" then "hepta_work_graph_append_only_store_enablement_precondition_preview_gate"
      elif $decision == "allow_preview_only" then "hepta_work_graph_projection_enforcement_dry_run_preview_gate"
      else "hepta_work_graph_append_only_store_enablement_precondition_preview_gate"
      end;
  def terminal_decision($decision):
      (outcome_for($decision.source_surface_id)) as $outcome
      | (plan_for($decision.source_surface_id)) as $plan
      | ($outcome != null) as $covered
      | ($decision.residual_route_blocker_ids | index("terminal_task_result_enforcement_disabled")) as $previous_terminal_route_blocked
      | (($previous_terminal_route_blocked | not) or ($outcome.terminal_task_result_contract_ready_preview // false)) as $terminal_ready
      | (if $covered then ($decision.residual_route_blocker_ids | map(select(. != "terminal_task_result_enforcement_disabled"))) else $decision.residual_route_blocker_ids end) as $route_blockers
      | (($plan.terminal_source_blocker_ids // [])) as $terminal_source_blockers
      | (if $covered then ($decision.residual_source_blocker_ids | map(select(. as $blocker | ($terminal_source_blockers | index($blocker) | not)))) else $decision.residual_source_blocker_ids end) as $source_blockers
      | (terminal_decision_for($decision.unified_store_projection_ready; $decision.timeline_projection_ready; $decision.task_result_projection_ready; $decision.append_only_route_ready; $decision.store_idempotency_guard_ready; $terminal_ready; $decision.readback_probe_contract_ready; $route_blockers; $source_blockers)) as $rerun_decision
      | {
          source_surface_id: $decision.source_surface_id,
          source_category: $decision.source_category,
          previous_projection_coverage_state: $decision.previous_coverage_state,
          previous_store_guard_rerun_state: $decision.store_guard_rerun_state,
          terminal_task_result_rerun_state: (if $covered then "terminal_task_result_contract_ready_preview_after_application" elif $terminal_ready then "terminal_task_result_ready_before_application" else "terminal_task_result_missing" end),
          covered_by_terminal_task_result_application_preview: $covered,
          previous_enforcement_decision: $decision.store_guard_rerun_enforcement_decision,
          terminal_task_result_rerun_enforcement_decision: $rerun_decision,
          terminal_task_result_gap_closed_by_application_preview: ($covered and $previous_terminal_route_blocked and $terminal_ready),
          projection_contract_ready: $decision.projection_contract_ready,
          unified_store_projection_ready: $decision.unified_store_projection_ready,
          timeline_projection_ready: $decision.timeline_projection_ready,
          task_result_projection_ready: $decision.task_result_projection_ready,
          store_idempotency_guard_ready: $decision.store_idempotency_guard_ready,
          terminal_task_result_contract_ready: $terminal_ready,
          append_only_route_ready: $decision.append_only_route_ready,
          readback_probe_contract_ready: $decision.readback_probe_contract_ready,
          residual_source_blocker_ids: $source_blockers,
          residual_route_blocker_ids: $route_blockers,
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
  ($store.decision_deltas | map(terminal_decision(.))) as $decisions
  | ($store.decision_deltas | map(select(.residual_route_blocker_ids | index("terminal_task_result_enforcement_disabled")) | .source_surface_id)) as $before_terminal_gap_sources
  | ($decisions | map(select((.terminal_task_result_contract_ready | not) or (.residual_route_blocker_ids | index("terminal_task_result_enforcement_disabled"))) | .source_surface_id)) as $after_terminal_gap_sources
  | ($decisions | map(select(.covered_by_terminal_task_result_application_preview) | .source_surface_id)) as $terminal_application_sources
  | ($decisions | map(select(.previous_projection_coverage_state == "contract_ready_preview_after_application") | .source_surface_id)) as $projection_application_sources
  | ($decisions | map(select(.previous_store_guard_rerun_state == "store_guard_contract_ready_preview_after_application") | .source_surface_id)) as $store_application_sources
  | [
      {
        id: "terminal_task_result_enforcement_disabled_for_enforcement",
        cleared_source_surface_ids: $before_terminal_gap_sources,
        source_count_before: ($before_terminal_gap_sources | length),
        source_count_after: ($after_terminal_gap_sources | length),
        closure_gate_id: "hepta_work_graph_terminal_task_result_enforcement_gap_closure_application_preview_gate"
      }
    ] as $cleared_blockers
  | [
      residual_blocker("projection_adapter_runtime_closure_application_disabled"; "high"; $projection_application_sources; "keep projection adapter closures preview-only until store guards, terminal TaskResult, and operator-review gates are promoted"),
      residual_blocker("store_guard_runtime_application_disabled"; "high"; $store_application_sources; "attach store idempotency guards to runtime adapters only after persistence and operator-review gates are promoted"),
      residual_blocker("idempotency_index_mutation_disabled"; "high"; $store_application_sources; "keep idempotency indexes immutable until collision policy and replay evidence are enforced"),
      residual_blocker("state_store_guard_persistence_disabled"; "high"; $store_application_sources; "do not persist candidate guard rows until append-only store intake is promoted"),
      residual_blocker("terminal_task_result_runtime_application_disabled"; "high"; $terminal_application_sources; "attach terminal TaskResult wrappers to runtime only after persistence, replay, and operator-review gates are promoted"),
      residual_blocker("task_result_persistence_disabled"; "high"; $terminal_application_sources; "keep TaskResult rows preview-only until append-only store intake is promoted"),
      residual_blocker("scheduler_admission_not_enforced"; "high"; ($decisions | map(select(has_suffix(.residual_source_blocker_ids; "_admission_not_enforced")) | .source_surface_id)); "make dependency, lease, budget, approval, role, and idempotency checks authoritative before work start"),
      residual_blocker("role_manifest_not_enforced"; "medium"; ($decisions | map(select(has_contains(.residual_source_blocker_ids; "role_manifest_not_enforced")) | .source_surface_id)); "bind multi-agent, batch, worker, and handoff sources to role manifests with budgets and tool permissions"),
      residual_blocker("append_only_store_enablement_disabled"; "medium"; ($decisions | map(.source_surface_id)); "keep projection enforcement disabled until WAL, readback, replay, and operator readiness gates are promoted"),
      residual_blocker("operator_review_required"; "medium"; $terminal_application_sources; "operator review must accept terminal wrapper bindings, evidence contracts, and enforcement routing before promotion")
    ] as $residual_blockers
  | [
      stage("unified_projection_contracts"; ($decisions | length); $store.store_guard_rerun_contract_ready_surface_count; ($decisions | map(select(.projection_contract_ready)) | length); ["projection_adapter_runtime_closure_application_disabled"]; "hepta_work_graph_append_only_store_enablement_precondition_preview_gate"),
      stage("projection_adapter_runtime_application"; 7; 0; 0; ["projection_adapter_runtime_closure_application_disabled"]; "hepta_work_graph_append_only_store_enablement_precondition_preview_gate"),
      stage("store_idempotency_guard_contracts"; ($decisions | length); $store.store_guard_rerun_store_guard_ready_surface_count; ($decisions | map(select(.store_idempotency_guard_ready)) | length); ["store_guard_runtime_application_disabled"]; "hepta_work_graph_append_only_store_enablement_precondition_preview_gate"),
      stage("store_guard_runtime_application"; ($store_application_sources | length); 0; 0; ["store_guard_runtime_application_disabled","idempotency_index_mutation_disabled","state_store_guard_persistence_disabled"]; "hepta_work_graph_append_only_store_enablement_precondition_preview_gate"),
      stage("terminal_task_result_contracts"; ($terminal_application_sources | length); 0; ($decisions | map(select(.covered_by_terminal_task_result_application_preview and .terminal_task_result_contract_ready)) | length); ["terminal_task_result_runtime_application_disabled","task_result_persistence_disabled"]; "hepta_work_graph_append_only_store_enablement_precondition_preview_gate"),
      stage("terminal_task_result_runtime_application"; ($terminal_application_sources | length); 0; 0; ["terminal_task_result_runtime_application_disabled","operator_review_required"]; "hepta_work_graph_append_only_store_enablement_precondition_preview_gate"),
      stage("scheduler_admission_contracts"; ($decisions | map(select(has_suffix(.residual_source_blocker_ids; "_admission_not_enforced"))) | length); 0; 0; ["scheduler_admission_not_enforced"]; "hepta_work_graph_scheduler_admission_controller_preview_gate"),
      stage("role_manifest_contracts"; ($decisions | map(select(has_contains(.residual_source_blocker_ids; "role_manifest_not_enforced"))) | length); 0; 0; ["role_manifest_not_enforced"]; "hepta_work_graph_role_manifest_contract_preview_gate"),
      stage("append_only_store_enablement"; ($decisions | length); 0; 0; ["append_only_store_enablement_disabled"]; "hepta_work_graph_append_only_store_enablement_precondition_preview_gate")
    ] as $stages
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview_gate",
      schema_version: "work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview_v1",
      preview_mode: "read_only_projection_enforcement_readiness_terminal_task_result_rerun_no_enforcement",
      source_surface_count: ($decisions | length),
      terminal_task_result_application_outcome_count: ($application.source_outcomes | length),
      previous_contract_ready_surface_count: $store.store_guard_rerun_contract_ready_surface_count,
      terminal_task_result_rerun_contract_ready_surface_count: ($decisions | map(select(.projection_contract_ready)) | length),
      previous_terminal_task_result_ready_surface_count: ($store.decision_deltas | map(select(.residual_route_blocker_ids | index("terminal_task_result_enforcement_disabled") | not)) | length),
      terminal_task_result_rerun_ready_surface_count: ($decisions | map(select(.terminal_task_result_contract_ready)) | length),
      previous_terminal_task_result_gap_source_count: ($before_terminal_gap_sources | length),
      terminal_task_result_gap_source_count_after: ($after_terminal_gap_sources | length),
      terminal_task_result_application_source_count: ($terminal_application_sources | length),
      rerun_ready_surface_count: ($decisions | map(select(.terminal_task_result_rerun_enforcement_decision == "allow_preview_only")) | length),
      rerun_blocked_surface_count: ($decisions | map(select(.terminal_task_result_rerun_enforcement_decision != "allow_preview_only")) | length),
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
      recommended_next_gate: "hepta_work_graph_append_only_store_enablement_precondition_preview_gate",
      ready_for_append_only_store_enablement_precondition_preview: true,
      ready_for_projection_enforcement: false,
      ready_for_append_only_store_enablement: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_terminal_task_result_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        terminal_task_result_readiness_rerun: {
          rust_module_present: $terminal_rerun_rust_module_present,
          report_script_present: $terminal_rerun_report_script_present,
          gate_script_present: $terminal_rerun_gate_script_present
        },
        terminal_task_result_application: {
          rust_module_present: $terminal_application_rust_module_present,
          gate_script_present: $terminal_application_gate_script_present,
          upstream_gate: ($application.gate == "hepta_work_graph_terminal_task_result_enforcement_gap_closure_application_preview_gate")
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        wal_written: false,
        idempotency_index_mutated: false,
        store_guard_attached: false,
        append_only_store_enabled: false,
        projection_enforcement_enabled: false,
        scheduler_admission_enforced: false,
        task_result_enforcement_enabled: false,
        task_result_persisted: false,
        wrapper_executed: false,
        runtime_wrapper_attached: false,
        role_manifest_enforcement_enabled: false,
        timeline_persisted: false,
        closure_applied_to_runtime: false,
        approval_recorded: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
