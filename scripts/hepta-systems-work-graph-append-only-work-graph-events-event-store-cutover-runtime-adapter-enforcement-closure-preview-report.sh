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

upstream_report="$(
  capture_json_report \
    "hepta-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-adapter-projection-gap-closure-rerun-preview-report" \
    "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-adapter-projection-gap-closure-rerun-preview-report.sh"
)"

preview_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_preview.rs
)"
preview_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-runtime-adapter-enforcement-closure-preview-report.sh
)"
preview_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-work-graph-events-event-store-cutover-runtime-adapter-enforcement-closure-preview-gate.sh
)"
upstream_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_adapter_projection_gap_closure_rerun_preview.rs
)"
upstream_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-adapter-projection-gap-closure-rerun-preview-gate.sh
)"

jq -n \
  --argjson upstream "$upstream_report" \
  --argjson preview_rust_module_present "$preview_rust_module_present" \
  --argjson preview_report_script_present "$preview_report_script_present" \
  --argjson preview_gate_script_present "$preview_gate_script_present" \
  --argjson upstream_rust_module_present "$upstream_rust_module_present" \
  --argjson upstream_gate_script_present "$upstream_gate_script_present" \
  '
  def unique_order: reduce .[] as $item ([]; if index($item) then . else . + [$item] end);
  def stage_ids: [
    "work_graph_events_runtime_adapter_enforcement_closure_packet",
    "work_graph_events_scheduler_admission_no_enforcement_guard",
    "work_graph_events_task_result_role_manifest_prerequisite",
    "work_graph_events_event_store_cutover_no_enforcement_proof",
    "work_graph_events_replay_readback_prerequisite",
    "work_graph_events_runtime_adapter_enforcement_closure_blocker_mapping"
  ];
  def evidence_fields: [
    "source_surface_id",
    "source_category",
    "event_store_cutover_adapter_projection_gap_closure_rerun_decision_ref",
    "runtime_adapter_enforcement_closure_packet_id",
    "scheduler_admission_no_enforcement_guard_id",
    "task_result_role_manifest_prerequisite_contract_id",
    "event_store_cutover_no_enforcement_proof_id",
    "replay_readback_prerequisite_contract_id",
    "residual_source_blocker_ids",
    "next_required_gate"
  ];
  def plan_id($source):
    $source + "_append_only_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure";
  def sources_for($blocker_id):
    [$upstream.decision_deltas[]
      | select(.residual_source_blocker_ids | index($blocker_id))
      | .source_surface_id] | unique_order;
  def stage($id; $priority; $category; $sources; $contracts): {
      id: $id,
      priority: $priority,
      category: $category,
      affected_source_surface_ids: $sources,
      required_contract_ref_ids: $contracts,
      expected_runtime_state: "preview_only_no_event_store_cutover_runtime_adapter_enforcement_closure",
      prerequisite_gate_ids: ["hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_adapter_projection_gap_closure_rerun_preview_gate"],
      contract_ready_preview: true,
      persists_work_graph_events_after_preview: false,
      enables_event_store_after_preview: false,
      writes_wal_after_preview: false,
      writes_checkpoint_after_preview: false,
      executes_replay_after_preview: false,
      executes_readback_after_preview: false,
      enforces_adapter_projection_after_preview: false,
      mutates_runtime_after_preview: false
    };
  def guard($id; $severity; $scope): {
      id: $id,
      severity: $severity,
      guard_scope: $scope,
      required_before_event_store_cutover_runtime_adapter_enforcement_closure: true,
      satisfied_by_preview: false
    };
  def blocker($id; $severity; $category; $sources; $stages; $plans; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_source_surface_ids: $sources,
      affected_event_store_cutover_runtime_adapter_enforcement_closure_stage_ids: $stages,
      affected_event_store_cutover_runtime_adapter_enforcement_closure_plan_ids: $plans,
      required_before_event_store_cutover_runtime_adapter_enforcement_closure: true,
      recommended_fix: $fix
    };
  ($upstream.decision_deltas | map(select(.residual_source_blocker_ids | index("runtime_canonical_adapter_enforcement_disabled")))) as $source_decisions
  | ($source_decisions | map(.source_surface_id)) as $all_sources
  | ($source_decisions | map({
      source_surface_id: .source_surface_id,
      source_category: .source_category,
      event_store_cutover_runtime_adapter_enforcement_closure_plan_id: plan_id(.source_surface_id),
      previous_enforcement_decision: .work_graph_events_event_store_cutover_adapter_projection_gap_closure_rerun_enforcement_decision,
      event_store_cutover_runtime_adapter_enforcement_closure_state: "work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_packet_ready_preview",
      required_event_store_cutover_runtime_adapter_enforcement_closure_stage_ids: stage_ids,
      expected_evidence_field_ids: evidence_fields,
      residual_source_blocker_ids: .residual_source_blocker_ids,
      event_store_cutover_runtime_adapter_enforcement_closure_contract_ready_preview: true,
      append_only_event_store_persistence_guard_ready_preview: true,
      operator_review_no_approval_guard_ready_preview: true,
      replay_readback_prerequisite_ready_preview: true,
      adapter_enforcement_prerequisite_ready_preview: true,
      applies_to_runtime: false,
      persists_work_graph_events: false,
      enables_event_store: false,
      writes_wal: false,
      writes_checkpoint: false,
      executes_replay: false,
      executes_readback: false,
      enforces_adapter_projection: false,
      mutates_runtime: false
    })) as $plans
  | ($plans | map(.event_store_cutover_runtime_adapter_enforcement_closure_plan_id)) as $plan_ids
  | [
      stage("work_graph_events_runtime_adapter_enforcement_closure_packet"; "critical"; "event_store_cutover_runtime_adapter_enforcement_closure"; $all_sources; ["runtime_adapter_enforcement_closure_contract_ready","canonical_adapter_coverage_closure_ready","runtime_adapter_enforcement_switch_disabled_ready","event_schema_registry_runtime_adapter_enforcement_closure_ready","event_store_no_enforcement_contract_ready"]),
      stage("work_graph_events_scheduler_admission_no_enforcement_guard"; "critical"; "scheduler_admission_no_enforcement_guard"; $all_sources; ["scheduler_admission_no_enforcement_guard_ready","scheduler_admission_disabled_guard_ready","admission_controller_dry_run_guard_ready","worker_task_admission_bypass_guard_ready","no_scheduler_mutation_guard_ready"]),
      stage("work_graph_events_task_result_role_manifest_prerequisite"; "critical"; "task_result_role_manifest_prerequisite"; $all_sources; ["canonical_adapter_enforcement_prerequisite_ready","terminal_task_result_prerequisite_ready","task_result_reduction_prerequisite_ready","role_manifest_prerequisite_ready","tool_permission_manifest_prerequisite_ready","role_terminal_contract_prerequisite_ready"]),
      stage("work_graph_events_event_store_cutover_no_enforcement_proof"; "critical"; "event_store_cutover_no_enforcement_proof"; $all_sources; ["event_store_cutover_no_enforcement_proof_ready","runtime_adapter_enforcement_disabled_guard_ready","append_only_events_disabled_guard_ready","event_store_cutover_switch_disabled_ready","no_runtime_mutation_guard_ready"]),
      stage("work_graph_events_replay_readback_prerequisite"; "critical"; "replay_readback"; $all_sources; ["replay_cursor_prerequisite_contract_ready","readback_probe_prerequisite_contract_ready","rollback_anchor_prerequisite_contract_ready","duplicate_suppression_prerequisite_contract_ready","timeline_order_prerequisite_contract_ready"]),
      stage("work_graph_events_runtime_adapter_enforcement_closure_blocker_mapping"; "high"; "blocker_mapping"; $all_sources; ["append_only_events_disabled_blocker_mapping_ready","replay_readback_disabled_blocker_mapping_ready","adapter_enforcement_disabled_blocker_mapping_ready","runtime_adapter_enforcement_closure_blocker_mapping_ready","runtime_adapter_enforcement_closure_readback_missing_blocker_mapping_ready"])
    ] as $stages
  | [
      guard("work_graph_events_persistence_disabled"; "critical"; "event_store"),
      guard("event_store_cutover_runtime_adapter_enforcement_closure_disabled"; "critical"; "event_store"),
      guard("wal_write_disabled"; "critical"; "wal"),
      guard("checkpoint_write_disabled"; "critical"; "checkpoint"),
      guard("replay_execution_disabled"; "critical"; "replay"),
      guard("readback_execution_disabled"; "critical"; "readback"),
      guard("adapter_projection_enforcement_disabled"; "critical"; "adapter_projection"),
      guard("idempotency_index_mutation_disabled"; "critical"; "idempotency"),
      guard("approval_recording_disabled"; "high"; "operator_review"),
      guard("side_effect_lock_not_established"; "critical"; "side_effect_lock"),
      guard("no_agent_spawn_or_external_effect"; "high"; "external_effects")
    ] as $guards
  | [
      blocker("append_only_work_graph_events_disabled"; "critical"; "event_store_cutover_runtime_adapter_enforcement_closure"; sources_for("append_only_work_graph_events_disabled"); stage_ids; $plan_ids; "keep event persistence disabled until event-store cutover runtime-adapter enforcement closure readback and cutover_runtime_adapter_enforcement_closure readiness are promoted"),
      blocker("replay_readback_execution_disabled"; "critical"; "replay_readback"; sources_for("replay_readback_execution_disabled"); ["work_graph_events_replay_readback_prerequisite"]; $plan_ids; "keep replay/readback execution disabled until event-store cutover runtime-adapter enforcement closure and rollback anchors are promoted"),
      blocker("runtime_canonical_adapter_enforcement_disabled"; "high"; "adapter_enforcement"; sources_for("runtime_canonical_adapter_enforcement_disabled"); ["work_graph_events_runtime_adapter_enforcement_closure_blocker_mapping"]; $plan_ids; "close runtime adapter enforcement prerequisites through a no-enforcement proof before replay/readback or event-store cutover can promote"),
      blocker("append_only_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_readback_missing"; "medium"; "readback_preview"; $all_sources; stage_ids; $plan_ids; "run event-store cutover runtime-adapter enforcement closure readback before applying no-persistence outcomes")
    ] as $blockers
  | ($upstream.required_prior_gates + [$upstream.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_append_only_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_preview_gate",
      schema_version: "work_graph_append_only_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_preview_v1",
      preview_mode: "read_only_append_only_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_preview_no_persistence",
      upstream_event_store_cutover_adapter_projection_gap_closure_rerun_gate: "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_adapter_projection_gap_closure_rerun_preview_gate",
      source_surface_count: ($source_decisions | length),
      event_store_cutover_runtime_adapter_enforcement_closure_plan_count: ($plans | length),
      event_store_cutover_runtime_adapter_enforcement_closure_stage_count: ($stages | length),
      event_store_cutover_runtime_adapter_enforcement_closure_stage_source_ref_count: ($stages | map(.affected_source_surface_ids | length) | add),
      event_store_cutover_runtime_adapter_enforcement_closure_stage_contract_ref_count: ($stages | map(.required_contract_ref_ids | length) | add),
      event_store_cutover_runtime_adapter_enforcement_closure_plan_stage_ref_count: ($plans | map(.required_event_store_cutover_runtime_adapter_enforcement_closure_stage_ids | length) | add),
      event_store_cutover_runtime_adapter_enforcement_closure_plan_evidence_field_ref_count: ($plans | map(.expected_evidence_field_ids | length) | add),
      append_only_work_graph_events_primary_blocked_source_count: (sources_for("append_only_work_graph_events_disabled") | length),
      replay_readback_execution_blocked_source_count: (sources_for("replay_readback_execution_disabled") | length),
      runtime_adapter_enforcement_blocked_source_count: (sources_for("runtime_canonical_adapter_enforcement_disabled") | length),
      guard_count: ($guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      event_store_cutover_runtime_adapter_enforcement_closure_plans: $plans,
      event_store_cutover_runtime_adapter_enforcement_closure_stage_plans: $stages,
      guards: $guards,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_append_only_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_readback_preview_gate",
      ready_for_event_store_cutover_runtime_adapter_enforcement_closure_readback_preview: true,
      ready_for_append_only_work_graph_events: false,
      ready_for_event_store_cutover_runtime_adapter_enforcement_closure: false,
      ready_for_replay_readback_execution: false,
      ready_for_runtime_adapter_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        append_only_work_graph_events_event_store_cutover_runtime_adapter_enforcement_closure_preview: {
          rust_module_present: $preview_rust_module_present,
          report_script_present: $preview_report_script_present,
          gate_script_present: $preview_gate_script_present
        },
        work_graph_events_event_store_cutover_adapter_projection_gap_closure_rerun: {
          rust_module_present: $upstream_rust_module_present,
          gate_script_present: $upstream_gate_script_present,
          upstream_gate: ($upstream.gate == "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_adapter_projection_gap_closure_rerun_preview_gate")
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
        runtime_adapter_enforcement_closure_established: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
