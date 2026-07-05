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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-projection-gap-closure-application.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

capture_json_report \
  "hepta-work-graph-projection-adapter-gap-closure-readback-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-projection-adapter-gap-closure-readback-preview-report.sh" \
  >"$tmpdir/gap_closure_readback.json"

application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_projection_adapter_gap_closure_application_preview.rs
)"
application_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-projection-adapter-gap-closure-application-preview-report.sh
)"
application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-projection-adapter-gap-closure-application-preview-gate.sh
)"
readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_projection_adapter_gap_closure_readback_preview.rs
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-projection-adapter-gap-closure-readback-preview-gate.sh
)"

jq -n \
  --slurpfile readback "$tmpdir/gap_closure_readback.json" \
  --argjson application_rust_module_present "$application_rust_module_present" \
  --argjson application_report_script_present "$application_report_script_present" \
  --argjson application_gate_script_present "$application_gate_script_present" \
  --argjson readback_rust_module_present "$readback_rust_module_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  '
  $readback[0] as $readback
  | def source_ids: [
      "update_plan_tool",
      "plan_mode_proposed_plan_blocks",
      "app_server_turn_plan_notification",
      "multi_agent_v2_mailbox_wait",
      "hepta_runtime_multi_agent_reducer",
      "hepta_runtime_task_board",
      "hepta_runtime_approval_broker"
    ];
  def application_plan($plan): {
      application_plan_id: $plan.closure_action_id,
      closure_action_id: $plan.closure_action_id,
      source_surface_id: $plan.source_surface_id,
      adapter_kind: $plan.adapter_kind,
      application_scope: $plan.readback_scope,
      expected_projected_collection_ids: $plan.expected_projected_collection_ids,
      expected_timeline_event_type_ids: $plan.expected_timeline_event_type_ids,
      required_evidence_fields: $plan.required_evidence_fields,
      application_state: "preview_application_defined_runtime_not_mutated",
      readback_verified_by_preview: true,
      applies_to_runtime: false,
      mutates_store: false,
      persists_timeline: false,
      enforces_projection: false
    };
  def source_plans($source; $plans):
      [$plans[] | select(.source_surface_id == $source)];
  def unique_ordered:
      reduce .[] as $item ([]; if index($item) then . else . + [$item] end);
  def source_outcome($source; $plans):
      (source_plans($source; $plans)) as $source_plans
      | {
          source_surface_id: $source,
          closure_action_ids: ($source_plans | map(.closure_action_id)),
          application_plan_ids: ($source_plans | map(.application_plan_id)),
          projected_collection_ids: ($source_plans | map(.expected_projected_collection_ids[]) | unique_ordered),
          timeline_event_type_ids: ($source_plans | map(.expected_timeline_event_type_ids[]) | unique_ordered),
          fixture_application_required: (($source_plans | map(select(.adapter_kind == "adapter_projection_fixture")) | length) > 0),
          store_projection_application_required: (($source_plans | map(select(.adapter_kind == "unified_store_projection")) | length) > 0),
          timeline_projection_application_required: (($source_plans | map(select(.adapter_kind == "observability_timeline_projection")) | length) > 0),
          post_application_coverage_state: "contract_ready_preview_after_application",
          ready_for_enforcement_readiness_rerun: true,
          ready_for_projection_enforcement: false,
          applies_to_runtime: false
        };
  def group($id; $priority; $sources; $plans; $expected): {
      id: $id,
      priority: $priority,
      source_surface_ids: $sources,
      closure_action_ids: ($sources | map(. as $source | ($plans[] | select(.source_surface_id == $source) | .closure_action_id))),
      application_plan_ids: ($sources | map(. as $source | ($plans[] | select(.source_surface_id == $source) | .application_plan_id))),
      expected_contract_ready_source_count_after_application: $expected,
      mutates_runtime: false,
      enforces_projection: false
    };
  def guard($id; $severity; $scope): {
      id: $id,
      severity: $severity,
      guard_scope: $scope,
      required_before_projection_enforcement: true,
      satisfied_by_preview: false
    };
  def blocker($id; $severity; $sources; $plans; $fix): {
      id: $id,
      severity: $severity,
      affected_source_surface_ids: $sources,
      affected_application_plan_ids: $plans,
      required_before_projection_enforcement: true,
      recommended_fix: $fix
    };
  ($readback.readback_plans | map(application_plan(.))) as $application_plans
  | (source_ids | map(source_outcome(. ; $application_plans))) as $source_outcomes
  | [
      group("planning_projection_adapter_gap_closure_application"; "P0"; ["update_plan_tool", "plan_mode_proposed_plan_blocks", "app_server_turn_plan_notification"]; $application_plans; 3),
      group("multi_agent_mailbox_projection_adapter_gap_closure_application"; "P0"; ["multi_agent_v2_mailbox_wait"]; $application_plans; 1),
      group("multi_agent_reducer_projection_adapter_gap_closure_application"; "P1"; ["hepta_runtime_multi_agent_reducer"]; $application_plans; 1),
      group("task_board_projection_adapter_gap_closure_application"; "P1"; ["hepta_runtime_task_board"]; $application_plans; 1),
      group("approval_broker_projection_adapter_gap_closure_application"; "P1"; ["hepta_runtime_approval_broker"]; $application_plans; 1)
    ] as $application_groups
  | [
      guard("runtime_attachment_disabled"; "critical"; "runtime"),
      guard("store_mutation_disabled"; "critical"; "unified_store"),
      guard("timeline_persistence_disabled"; "critical"; "timeline"),
      guard("projection_enforcement_disabled"; "critical"; "projection"),
      guard("task_result_enforcement_disabled"; "high"; "task_result"),
      guard("enforcement_readiness_rerun_required"; "high"; "readiness_rerun")
    ] as $application_guards
  | [
      blocker("gap_closure_application_is_preview_only"; "medium"; source_ids; ($application_plans | map(.application_plan_id)); "keep application as a no-mutation preview until enforcement-readiness rerun confirms contract-ready projection"),
      blocker("runtime_closure_application_disabled"; "high"; source_ids; ($application_plans | map(.application_plan_id)); "attach adapter closures to runtime only after operator review and store/timeline guards are satisfied"),
      blocker("append_only_store_enablement_disabled"; "high"; ($application_plans | map(select(.adapter_kind == "unified_store_projection") | .source_surface_id) | unique); ($application_plans | map(select(.adapter_kind == "unified_store_projection") | .application_plan_id)); "keep store writes disabled until append-only store enablement has its own promotion gate"),
      blocker("timeline_persistence_disabled"; "high"; ($application_plans | map(select(.adapter_kind == "observability_timeline_projection") | .source_surface_id) | unique); ($application_plans | map(select(.adapter_kind == "observability_timeline_projection") | .application_plan_id)); "keep timeline persistence disabled until redaction and event ordering are enforced"),
      blocker("terminal_task_result_enforcement_disabled"; "high"; ["hepta_runtime_multi_agent_reducer", "hepta_runtime_task_board"]; ($application_plans | map(select(.source_surface_id == "hepta_runtime_multi_agent_reducer" or .source_surface_id == "hepta_runtime_task_board") | .application_plan_id)); "enforce terminal TaskResult output before promoting reducer and task_board adapter closures"),
      blocker("enforcement_readiness_rerun_missing"; "high"; source_ids; ($application_plans | map(.application_plan_id)); "rerun unified projection enforcement-readiness against the application preview outcomes")
    ] as $blockers
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_projection_adapter_gap_closure_application_preview_gate",
      schema_version: "work_graph_projection_adapter_gap_closure_application_preview_v1",
      preview_mode: "read_only_projection_adapter_gap_closure_application_preview_no_runtime_mutation",
      source_gap_count: $readback.source_gap_count,
      readback_plan_count: $readback.readback_plan_count,
      application_plan_count: ($application_plans | length),
      fixture_application_count: ($application_plans | map(select(.adapter_kind == "adapter_projection_fixture")) | length),
      store_projection_application_count: ($application_plans | map(select(.adapter_kind == "unified_store_projection")) | length),
      timeline_projection_application_count: ($application_plans | map(select(.adapter_kind == "observability_timeline_projection")) | length),
      source_outcome_count: ($source_outcomes | length),
      source_contract_ready_preview_count: ($source_outcomes | map(select(.post_application_coverage_state == "contract_ready_preview_after_application")) | length),
      application_group_count: ($application_groups | length),
      projected_collection_reference_count: (($application_plans | map(select(.adapter_kind == "unified_store_projection") | (.expected_projected_collection_ids | length)) | add) // 0),
      timeline_event_type_reference_count: (($application_plans | map(select(.adapter_kind == "observability_timeline_projection") | (.expected_timeline_event_type_ids | length)) | add) // 0),
      application_guard_count: ($application_guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: (($readback.required_prior_gates + [$readback.gate]) | length),
      application_plans: $application_plans,
      source_outcomes: $source_outcomes,
      application_groups: $application_groups,
      application_guards: $application_guards,
      blockers: $blockers,
      required_prior_gates: ($readback.required_prior_gates + [$readback.gate]),
      recommended_next_gate: "hepta_work_graph_unified_projection_enforcement_readiness_rerun_preview_gate",
      ready_for_unified_projection_enforcement_readiness_rerun_preview: true,
      ready_for_projection_enforcement: false,
      ready_for_append_only_store_enablement: false,
      ready_for_live_execution: false,
      source_probes: {
        gap_closure_application: {
          rust_module_present: $application_rust_module_present,
          report_script_present: $application_report_script_present,
          gate_script_present: $application_gate_script_present
        },
        gap_closure_readback: {
          rust_module_present: $readback_rust_module_present,
          gate_script_present: $readback_gate_script_present,
          upstream_gate: ($readback.gate == "hepta_work_graph_projection_adapter_gap_closure_readback_preview_gate")
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        adapter_projection_enforced: false,
        closure_applied_to_runtime: false,
        append_only_store_enabled: false,
        scheduler_admission_enforced: false,
        task_result_enforcement_enabled: false,
        timeline_persisted: false,
        readback_performed: false,
        approval_recorded: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
