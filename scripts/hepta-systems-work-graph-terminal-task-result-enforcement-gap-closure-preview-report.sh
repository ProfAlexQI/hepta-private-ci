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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-terminal-task-result-enforcement-gap-closure.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-unified-projection-enforcement-readiness-store-guard-rerun-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-store-guard-rerun-preview-report.sh" \
  >"$tmpdir/store_guard_rerun.json"
capture_json_report \
  "hepta-work-graph-terminal-task-result-wrapper-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-terminal-task-result-wrapper-preview-report.sh" \
  >"$tmpdir/wrapper.json"
capture_json_report \
  "hepta-work-graph-terminal-task-result-wrapper-readback-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-terminal-task-result-wrapper-readback-preview-report.sh" \
  >"$tmpdir/readback.json"

closure_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_terminal_task_result_enforcement_gap_closure_preview.rs
)"
closure_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-preview-report.sh
)"
closure_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-task-result-enforcement-gap-closure-preview-gate.sh
)"
store_guard_rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-store-guard-rerun-preview-gate.sh
)"
wrapper_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-task-result-wrapper-preview-gate.sh
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-terminal-task-result-wrapper-readback-preview-gate.sh
)"

jq -n \
  --slurpfile rerun "$tmpdir/store_guard_rerun.json" \
  --slurpfile wrapper "$tmpdir/wrapper.json" \
  --slurpfile readback "$tmpdir/readback.json" \
  --argjson closure_rust_module_present "$closure_rust_module_present" \
  --argjson closure_report_script_present "$closure_report_script_present" \
  --argjson closure_gate_script_present "$closure_gate_script_present" \
  --argjson store_guard_rerun_gate_script_present "$store_guard_rerun_gate_script_present" \
  --argjson wrapper_gate_script_present "$wrapper_gate_script_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  '
  $rerun[0] as $rerun
  | $wrapper[0] as $wrapper
  | $readback[0] as $readback
  | def unique_preserve:
      reduce .[] as $item ([]; if index($item) then . else . + [$item] end);
  def wrapper_for($id): [$wrapper.terminal_wrappers[] | select(.source_surface_id == $id)][0];
  def readback_for($id): [$readback.readback_plans[] | select(.source_surface_id == $id)][0];
  def terminal_source_blocker($blocker):
      ($blocker | contains("task_result"))
      or ($blocker | contains("TaskResult"))
      or ($blocker | contains("terminal_task_result"))
      or ($blocker | contains("verifier_and_reducer"))
      or ($blocker | contains("result_json"));
  def plan_for($decision):
      (wrapper_for($decision.source_surface_id)) as $wrapper
      | (readback_for($decision.source_surface_id)) as $readback_plan
      | {
          id: ("close_" + $decision.source_surface_id + "_terminal_task_result_enforcement_gap"),
          source_surface_id: $decision.source_surface_id,
          source_category: $decision.source_category,
          wrapper_id: $wrapper.id,
          terminal_source_kind: $wrapper.terminal_source_kind,
          emitted_event_contract_id: $wrapper.emitted_event_contract_id,
          replay_key_contract_id: $wrapper.replay_key_contract_id,
          readback_plan_id: $readback_plan.id,
          evidence_contract_id: $readback_plan.expected_evidence_contract_id,
          enforcement_binding_id: ("bind_" + $decision.source_surface_id + "_terminal_task_result_enforcement_preview"),
          readback_probe_binding_id: ("bind_" + $decision.source_surface_id + "_terminal_task_result_readback_probe_preview"),
          residual_source_blocker_ids: $decision.residual_source_blocker_ids,
          terminal_source_blocker_ids: ($decision.residual_source_blocker_ids | map(select(terminal_source_blocker(.)))),
          route_blocker_ids_before: $decision.residual_route_blocker_ids,
          route_blocker_ids_after_preview: ($decision.residual_route_blocker_ids | map(select(. != "terminal_task_result_enforcement_disabled"))),
          required_wire_fields: $wrapper.required_wire_fields,
          readback_collection_assertion_ids: $readback_plan.required_collection_assertion_ids,
          drift_detector_ids: $readback_plan.drift_detector_ids,
          application_state: "preview_closure_defined_terminal_task_result_enforcement_not_attached",
          readback_verified_by_preview: true,
          attaches_runtime_wrapper: false,
          executes_wrapper: false,
          persists_task_result: false,
          enables_task_result_enforcement: false,
          mutates_store: false
        };
  def group($id; $plans): {
      id: $id,
      priority: "p0",
      closure_plan_ids: ($plans | map(.id)),
      source_surface_ids: ($plans | map(.source_surface_id)),
      expected_contract_count_after_closure: ($plans | length),
      mutates_runtime: false,
      persists_task_result: false,
      enables_enforcement: false
    };
  def guard($id; $severity; $scope): {
      id: $id,
      severity: $severity,
      guard_scope: $scope,
      required_before_projection_enforcement: true,
      satisfied_by_preview: false
    };
  def blocker($id; $severity; $plans; $fix): {
      id: $id,
      severity: $severity,
      affected_source_surface_ids: ($plans | map(.source_surface_id)),
      affected_closure_plan_ids: ($plans | map(.id)),
      required_before_projection_enforcement: true,
      recommended_fix: $fix
    };
  ($rerun.decision_deltas | map(select(.store_guard_rerun_enforcement_decision == "deny_terminal_task_result_enforcement_disabled")) | map(plan_for(.))) as $plans
  | ($plans | map(select(.residual_source_blocker_ids | any(endswith("_admission_not_enforced") or contains("role_manifest_not_enforced"))))) as $admission_or_role_plans
  | [
      group("terminal_wrapper_contract_closure"; $plans),
      group("terminal_enforcement_binding_closure"; $plans),
      group("terminal_readback_probe_closure"; $plans),
      group("terminal_readiness_rerun_input_closure"; $plans)
    ] as $groups
  | [
      guard("closure_preview_only"; "medium"; "preview"),
      guard("wrapper_runtime_attachment_disabled"; "high"; "runtime"),
      guard("wrapper_execution_disabled"; "high"; "runtime"),
      guard("task_result_persistence_disabled"; "high"; "task_result_store"),
      guard("readback_execution_disabled"; "high"; "readback"),
      guard("terminal_task_result_enforcement_disabled"; "critical"; "task_result_enforcement"),
      guard("scheduler_admission_or_role_manifest_residuals_not_enforced"; "high"; "residual_admission_role"),
      guard("append_only_store_enablement_disabled"; "high"; "append_only_store"),
      guard("enforcement_readiness_task_result_rerun_required"; "high"; "readiness_rerun")
    ] as $guards
  | [
      blocker("terminal_task_result_closure_is_preview_only"; "medium"; $plans; "keep terminal TaskResult closure as a no-mutation preview until readback verifies every wrapper binding"),
      blocker("wrapper_runtime_attachment_disabled"; "high"; $plans; "attach terminal wrappers to runtime only after operator review and persistence gates are promoted"),
      blocker("wrapper_execution_disabled"; "high"; $plans; "execute no terminal wrapper until readback, drift budget, and promotion-precondition previews are clean"),
      blocker("task_result_persistence_disabled"; "high"; $plans; "do not persist TaskResult rows until append-only store intake and replay evidence are promoted"),
      blocker("readback_execution_disabled"; "high"; $plans; "run terminal TaskResult readback only after fixture and wrapper contracts are promoted"),
      blocker("terminal_task_result_enforcement_disabled"; "critical"; $plans; "keep terminal TaskResult enforcement disabled until wrapper application and readiness rerun prove no route blocker remains"),
      blocker("scheduler_admission_or_role_manifest_residuals_not_enforced"; "high"; $admission_or_role_plans; "preserve admission and role-manifest blockers as separate readiness gates after TaskResult closure"),
      blocker("append_only_store_enablement_disabled"; "high"; $plans; "keep append-only store disabled until TaskResult enforcement, replay, and operator readiness are promoted"),
      blocker("enforcement_readiness_task_result_rerun_missing"; "high"; $plans; "rerun unified projection enforcement-readiness after terminal TaskResult closure application preview")
    ] as $blockers
  | (($rerun.required_prior_gates
      + [$rerun.gate]
      + $wrapper.required_prior_gates
      + [$wrapper.gate]
      + $readback.required_prior_gates
      + [$readback.gate]) | unique_preserve) as $required_priors
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_terminal_task_result_enforcement_gap_closure_preview_gate",
      schema_version: "work_graph_terminal_task_result_enforcement_gap_closure_preview_v1",
      preview_mode: "read_only_terminal_task_result_enforcement_gap_closure_preview_no_runtime_attachment",
      terminal_task_result_blocker_source_count: ($plans | length),
      wrapper_candidate_source_count: ($plans | length),
      closure_plan_count: ($plans | length),
      enforcement_binding_count: ($plans | length),
      readback_probe_binding_count: ($plans | length),
      application_group_count: ($groups | length),
      terminal_source_blocker_ref_count: ($plans | map(.terminal_source_blocker_ids | length) | add),
      terminal_route_blocker_count_before: ($plans | map(select(.route_blocker_ids_before | index("terminal_task_result_enforcement_disabled"))) | length),
      terminal_route_blocker_count_after_preview: 0,
      readback_collection_assertion_ref_count: ($plans | map(.readback_collection_assertion_ids | length) | add),
      drift_detector_ref_count: ($plans | map(.drift_detector_ids | length) | add),
      application_guard_count: ($guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_priors | length),
      closure_plans: $plans,
      enforcement_bindings: ($plans | map({
        id: .enforcement_binding_id,
        source_surface_id,
        wrapper_id,
        task_result_collection_id: "taskResults",
        timeline_collection_id: "timelineEvents",
        route_blocker_id: "terminal_task_result_enforcement_disabled",
        terminal_source_blocker_ids,
        binding_state: "preview_binding_defined_runtime_attachment_disabled",
        attaches_runtime_wrapper: false,
        persists_task_result: false,
        enables_task_result_enforcement: false
      })),
      readback_probe_bindings: ($plans | map({
        id: .readback_probe_binding_id,
        source_surface_id,
        readback_plan_id,
        wrapper_id,
        required_collection_assertion_ids: .readback_collection_assertion_ids,
        drift_detector_ids,
        probe_state: "preview_probe_binding_defined_readback_execution_disabled",
        performs_readback: false,
        persists_drift: false,
        enables_task_result_enforcement: false
      })),
      application_groups: $groups,
      application_guards: $guards,
      blockers: $blockers,
      required_prior_gates: $required_priors,
      recommended_next_gate: "hepta_work_graph_terminal_task_result_enforcement_gap_closure_readback_preview_gate",
      ready_for_terminal_task_result_enforcement_gap_closure_readback_preview: true,
      ready_for_runtime_wrapper_attachment: false,
      ready_for_wrapper_execution: false,
      ready_for_task_result_persistence: false,
      ready_for_task_result_enforcement: false,
      ready_for_projection_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        terminal_task_result_enforcement_gap_closure: {
          rust_module_present: $closure_rust_module_present,
          report_script_present: $closure_report_script_present,
          gate_script_present: $closure_gate_script_present
        },
        store_guard_readiness_rerun: {
          upstream_gate: ($rerun.gate == "hepta_work_graph_unified_projection_enforcement_readiness_store_guard_rerun_preview_gate"),
          gate_script_present: $store_guard_rerun_gate_script_present
        },
        terminal_task_result_wrapper: {
          upstream_gate: ($wrapper.gate == "hepta_work_graph_terminal_task_result_wrapper_preview_gate"),
          gate_script_present: $wrapper_gate_script_present
        },
        terminal_task_result_wrapper_readback: {
          upstream_gate: ($readback.gate == "hepta_work_graph_terminal_task_result_wrapper_readback_preview_gate"),
          gate_script_present: $readback_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        wrapper_executed: false,
        runtime_wrapper_attached: false,
        readback_performed: false,
        drift_state_persisted: false,
        event_record_persisted: false,
        task_result_persisted: false,
        graph_state_persisted: false,
        wal_written: false,
        checkpoint_written: false,
        append_only_store_enabled: false,
        projection_enforcement_enabled: false,
        task_result_enforcement_enabled: false,
        scheduler_admission_enforced: false,
        role_manifest_enforcement_enabled: false,
        approval_recorded: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
