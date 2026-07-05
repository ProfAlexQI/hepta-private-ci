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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-append-only-store-precondition.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-unified-projection-enforcement-readiness-terminal-task-result-rerun-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-terminal-task-result-rerun-preview-report.sh" \
  >"$tmpdir/terminal_rerun.json"
capture_json_report \
  "hepta-work-graph-append-only-event-intake-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-append-only-event-intake-preview-report.sh" \
  >"$tmpdir/append_intake.json"
capture_json_report \
  "hepta-work-graph-state-store-persistence-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-state-store-persistence-preview-report.sh" \
  >"$tmpdir/state_store.json"
capture_json_report \
  "hepta-work-graph-store-idempotency-guard-gap-closure-application-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-store-idempotency-guard-gap-closure-application-preview-report.sh" \
  >"$tmpdir/store_guard_application.json"
capture_json_report \
  "hepta-work-graph-terminal-task-result-enforcement-gap-closure-application-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-application-preview-report.sh" \
  >"$tmpdir/terminal_task_result_application.json"

precondition_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_store_enablement_precondition_preview.rs
)"
precondition_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-enablement-precondition-preview-report.sh
)"
precondition_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-enablement-precondition-preview-gate.sh
)"
terminal_rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-terminal-task-result-rerun-preview-gate.sh
)"
append_intake_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-event-intake-preview-gate.sh
)"

jq -n \
  --slurpfile terminal "$tmpdir/terminal_rerun.json" \
  --slurpfile append "$tmpdir/append_intake.json" \
  --slurpfile state "$tmpdir/state_store.json" \
  --slurpfile store_guard "$tmpdir/store_guard_application.json" \
  --slurpfile terminal_application "$tmpdir/terminal_task_result_application.json" \
  --argjson precondition_rust_module_present "$precondition_rust_module_present" \
  --argjson precondition_report_script_present "$precondition_report_script_present" \
  --argjson precondition_gate_script_present "$precondition_gate_script_present" \
  --argjson terminal_rerun_gate_script_present "$terminal_rerun_gate_script_present" \
  --argjson append_intake_gate_script_present "$append_intake_gate_script_present" \
  '
  $terminal[0] as $terminal
  | $append[0] as $append
  | $state[0] as $state
  | $store_guard[0] as $store_guard
  | $terminal_application[0] as $terminal_application
  | def has_suffix($values; $suffix): any($values[]?; endswith($suffix));
  def has_contains($values; $needle): any($values[]?; contains($needle));
  def residual_sources($id): ([$terminal.residual_blockers[] | select(.id == $id)][0].affected_source_surface_ids // []);
  def unique_sources($left; $right): (($left + $right) | unique);
  def source_blockers($decision):
      ["durable_store_enablement_disabled",
       "wal_write_boundary_not_enabled",
       "idempotency_index_mutation_disabled",
       "rollback_readback_not_executed"]
      + (if $decision.covered_by_terminal_task_result_application_preview then ["operator_review_required"] else [] end)
      + (if has_suffix($decision.residual_source_blocker_ids; "_admission_not_enforced") then ["scheduler_admission_not_enforced"] else [] end)
      + (if has_contains($decision.residual_source_blocker_ids; "role_manifest_not_enforced") then ["role_manifest_not_enforced"] else [] end)
      + (if ($decision.residual_route_blocker_ids | index("append_only_store_disabled_by_design")) then ["append_only_store_enablement_disabled"] else [] end);
  def source_preconditions($decision):
      ["durable_store_enablement_switch",
       "wal_append_boundary_contract",
       "idempotency_mutation_policy",
       "rollback_readback_gate"]
      + (if $decision.covered_by_terminal_task_result_application_preview then ["operator_review_and_side_effect_lock"] else [] end)
      + (if has_suffix($decision.residual_source_blocker_ids; "_admission_not_enforced") then ["scheduler_admission_enforcement_precondition"] else [] end)
      + (if has_contains($decision.residual_source_blocker_ids; "role_manifest_not_enforced") then ["role_manifest_enforcement_precondition"] else [] end);
  def source_decision($decision):
      (source_blockers($decision)) as $blockers
      | {
          source_surface_id: $decision.source_surface_id,
          source_category: $decision.source_category,
          previous_readiness_decision: $decision.terminal_task_result_rerun_enforcement_decision,
          append_only_precondition_decision:
            (if ($blockers | index("scheduler_admission_not_enforced")) then "deny_scheduler_admission_not_enforced"
             elif ($blockers | index("role_manifest_not_enforced")) then "deny_role_manifest_not_enforced"
             else "deny_append_only_store_enablement_preconditions_missing"
             end),
          projection_contract_ready: $decision.projection_contract_ready,
          store_idempotency_guard_ready: $decision.store_idempotency_guard_ready,
          terminal_task_result_contract_ready: $decision.terminal_task_result_contract_ready,
          append_only_route_ready: $decision.append_only_route_ready,
          readback_probe_contract_ready: $decision.readback_probe_contract_ready,
          required_precondition_ids: source_preconditions($decision),
          blocker_ids: $blockers,
          ready_for_append_only_store_enablement: false,
          next_required_gate: "hepta_work_graph_append_only_store_enablement_precondition_readback_preview_gate"
        };
  def precondition($id; $category; $severity; $affected; $refs; $preview; $enablement; $blocker; $gate): {
      id: $id,
      category: $category,
      severity: $severity,
      affected_source_surface_ids: $affected,
      required_contract_refs: $refs,
      satisfied_by_preview_contracts: $preview,
      satisfied_for_enablement: $enablement,
      blocker_id: $blocker,
      recommended_closure_gate: $gate
    };
  def blocker($id; $severity; $category; $preconditions; $affected; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_precondition_ids: $preconditions,
      affected_source_surface_ids: $affected,
      required_before_append_only_store_enablement: true,
      recommended_fix: $fix
    };
  def stage($id; $observed; $preview; $enablement; $blockers): {
      id: $id,
      observed_contract_count: $observed,
      preview_ready_contract_count: $preview,
      enablement_ready_contract_count: $enablement,
      hard_blocker_ids: $blockers,
      enablement_enabled: false,
      next_gate: "hepta_work_graph_append_only_store_enablement_precondition_readback_preview_gate"
    };
  ($terminal.decision_deltas | map(source_decision(.))) as $source_decisions
  | ($source_decisions | map(.source_surface_id)) as $all_sources
  | ($terminal.decision_deltas | map(select(.covered_by_terminal_task_result_application_preview) | .source_surface_id)) as $terminal_sources
  | ($terminal.decision_deltas | map(select(has_suffix(.residual_source_blocker_ids; "_admission_not_enforced")) | .source_surface_id)) as $scheduler_sources
  | ($terminal.decision_deltas | map(select(has_contains(.residual_source_blocker_ids; "role_manifest_not_enforced")) | .source_surface_id)) as $role_sources
  | (residual_sources("projection_adapter_runtime_closure_application_disabled")) as $projection_runtime_sources
  | (residual_sources("store_guard_runtime_application_disabled")) as $store_runtime_sources
  | ($append.event_contracts | map(.id)) as $event_contract_refs
  | ($state.wal_operations | map(.id)) as $wal_operation_refs
  | (($state.checkpoint_contracts | map(.id)) + ($state.readback_probes | map(.id))) as $checkpoint_readback_refs
  | (($state.idempotency_guards | map(.id)) + ($store_guard.source_outcomes | map(.candidate_guard_id))) as $idempotency_refs
  | [
      precondition("durable_store_enablement_switch"; "durable_store_switch"; "critical"; $all_sources; $event_contract_refs; true; false; "durable_store_enablement_disabled"; "hepta_work_graph_append_only_store_enablement_switch_preview_gate"),
      precondition("wal_append_boundary_contract"; "wal_boundary"; "critical"; $all_sources; $wal_operation_refs; true; false; "wal_write_boundary_not_enabled"; "hepta_work_graph_append_only_store_wal_boundary_readback_preview_gate"),
      precondition("idempotency_mutation_policy"; "idempotency_mutation_policy"; "critical"; $all_sources; $idempotency_refs; true; false; "idempotency_index_mutation_disabled"; "hepta_work_graph_append_only_store_idempotency_mutation_policy_preview_gate"),
      precondition("rollback_readback_gate"; "rollback_readback_gate"; "critical"; $all_sources; $checkpoint_readback_refs; true; false; "rollback_readback_not_executed"; "hepta_work_graph_append_only_store_rollback_readback_preview_gate"),
      precondition("operator_review_and_side_effect_lock"; "operator_review"; "high"; $terminal_sources; ["operator_review_required","side_effect_lock_required","runtime_application_receipts_required"]; false; false; "operator_review_required"; "hepta_work_graph_append_only_store_operator_review_preview_gate"),
      precondition("scheduler_admission_enforcement_precondition"; "scheduler_admission"; "high"; $scheduler_sources; ["dependency_gate","lease_gate","budget_gate","approval_gate","idempotency_gate"]; false; false; "scheduler_admission_not_enforced"; "hepta_work_graph_scheduler_admission_controller_preview_gate"),
      precondition("role_manifest_enforcement_precondition"; "role_manifest"; "medium"; $role_sources; ["role_capabilities","tool_permissions","budget_limits","lane_boundaries"]; false; false; "role_manifest_not_enforced"; "hepta_work_graph_role_manifest_contract_preview_gate")
    ] as $preconditions
  | [
      blocker("durable_store_enablement_disabled"; "critical"; "durable_store_switch"; ["durable_store_enablement_switch"]; $all_sources; "keep append-only store disabled until the operator accepts durable write boundaries and rollback plan"),
      blocker("wal_write_boundary_not_enabled"; "critical"; "wal_boundary"; ["wal_append_boundary_contract"]; $all_sources; "promote WAL append contracts only after readback and replay fixtures prove deterministic recovery"),
      blocker("idempotency_index_mutation_disabled"; "critical"; "idempotency_mutation_policy"; ["idempotency_mutation_policy"]; $all_sources; "bind every source to a collision policy and mutation-safe idempotency index before writes"),
      blocker("rollback_readback_not_executed"; "critical"; "rollback_readback_gate"; ["rollback_readback_gate"]; $all_sources; "execute readback, replay, and rollback fixtures before any append-only store enablement"),
      blocker("operator_review_required"; "high"; "operator_review"; ["operator_review_and_side_effect_lock"]; $terminal_sources; "operator review must accept side-effect locks, terminal TaskResult persistence, and durable store switch"),
      blocker("scheduler_admission_not_enforced"; "high"; "scheduler_admission"; ["scheduler_admission_enforcement_precondition"]; $scheduler_sources; "scheduler admission must enforce dependency, lease, budget, approval, and idempotency gates before work start"),
      blocker("role_manifest_not_enforced"; "medium"; "role_manifest"; ["role_manifest_enforcement_precondition"]; $role_sources; "role manifests must bind capabilities, tools, budgets, reducers, and lane permissions before agent paths can append"),
      blocker("runtime_application_residuals_not_promoted"; "high"; "runtime_application"; ($preconditions | map(.id)); unique_sources($projection_runtime_sources; $store_runtime_sources); "projection adapter closures and store guards remain preview-only and cannot write into WorkGraph runtime state")
    ] as $blockers
  | [
      stage("contract_readiness_snapshot"; ($source_decisions | length); ($source_decisions | length); 0; ["durable_store_enablement_disabled"]),
      stage("append_only_event_intake_contracts"; ($append.event_contracts | length); ($append.event_contracts | length); 0; ["wal_write_boundary_not_enabled"]),
      stage("wal_and_idempotency_boundary"; (($state.wal_operations | length) + ((($state.idempotency_guards | map(.source_surface_id)) + ($store_guard.source_outcomes | map(.source_surface_id))) | unique | length)); (($state.wal_operations | length) + ((($state.idempotency_guards | map(.source_surface_id)) + ($store_guard.source_outcomes | map(.source_surface_id))) | unique | length)); 0; ["wal_write_boundary_not_enabled","idempotency_index_mutation_disabled"]),
      stage("rollback_readback_boundary"; ($state.readback_probes | length); ($state.readback_probes | length); 0; ["rollback_readback_not_executed"]),
      stage("admission_role_operator_policy"; (($scheduler_sources | length) + ($role_sources | length)); 0; 0; ["scheduler_admission_not_enforced","role_manifest_not_enforced","operator_review_required"])
    ] as $stages
  | (($terminal.required_prior_gates
      + (if ($terminal.required_prior_gates | index($terminal.gate)) then [] else [$terminal.gate] end)
      + (if ($terminal.required_prior_gates | index($append.gate)) then [] else [$append.gate] end))
    ) as $required_priors
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_append_only_store_enablement_precondition_preview_gate",
      schema_version: "work_graph_append_only_store_enablement_precondition_preview_v1",
      preview_mode: "read_only_append_only_store_enablement_precondition_preview_no_store_enablement",
      source_surface_count: ($source_decisions | length),
      append_only_disabled_source_count: ($source_decisions | map(select(.blocker_ids | index("append_only_store_enablement_disabled"))) | length),
      source_precondition_decision_count: ($source_decisions | length),
      append_only_store_precondition_ready_source_count: ($source_decisions | map(select(.ready_for_append_only_store_enablement)) | length),
      append_only_store_precondition_blocked_source_count: ($source_decisions | map(select(.ready_for_append_only_store_enablement | not)) | length),
      append_only_event_contract_count: ($append.event_contracts | length),
      append_only_event_route_count: ($append.source_routes | length),
      wal_operation_count: ($state.wal_operations | length),
      checkpoint_contract_count: ($state.checkpoint_contracts | length),
      existing_idempotency_guard_count: ($state.idempotency_guards | length),
      candidate_idempotency_guard_count: ($store_guard.source_outcomes | length),
      combined_idempotency_guard_source_count: ((($state.idempotency_guards | map(.source_surface_id)) + ($store_guard.source_outcomes | map(.source_surface_id))) | unique | length),
      readback_probe_count: ($state.readback_probes | length),
      terminal_task_result_contract_ready_source_count: ($terminal_application.source_outcomes | map(select(.terminal_task_result_contract_ready_preview)) | length),
      precondition_count: ($preconditions | length),
      blocker_count: ($blockers | length),
      enablement_stage_count: ($stages | length),
      required_prior_gate_count: ($required_priors | length),
      source_precondition_decisions: $source_decisions,
      preconditions: $preconditions,
      blockers: $blockers,
      enablement_stages: $stages,
      required_prior_gates: $required_priors,
      recommended_next_gate: "hepta_work_graph_append_only_store_enablement_precondition_readback_preview_gate",
      ready_for_precondition_readback_preview: true,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        append_only_store_enablement_precondition: {
          rust_module_present: $precondition_rust_module_present,
          report_script_present: $precondition_report_script_present,
          gate_script_present: $precondition_gate_script_present
        },
        upstream_readiness_terminal_task_result_rerun: {
          gate_script_present: $terminal_rerun_gate_script_present,
          upstream_gate: ($terminal.gate == "hepta_work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview_gate")
        },
        append_only_event_intake: {
          gate_script_present: $append_intake_gate_script_present,
          upstream_gate: ($append.gate == "hepta_work_graph_append_only_event_intake_preview_gate")
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        wal_written: false,
        checkpoint_written: false,
        idempotency_index_mutated: false,
        append_only_store_enabled: false,
        projection_enforcement_enabled: false,
        readback_executed: false,
        rollback_executed: false,
        scheduler_admission_enforced: false,
        role_manifest_enforcement_enabled: false,
        task_result_enforcement_enabled: false,
        runtime_wrapper_attached: false,
        approval_recorded: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
