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
    "hepta-work-graph-unified-projection-enforcement-readiness-work-graph-events-replay-readback-rerun-preview-report" \
    "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-replay-readback-rerun-preview-report.sh"
)"

preview_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_work_graph_events_persistence_guard_preview.rs
)"
preview_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-work-graph-events-persistence-guard-preview-report.sh
)"
preview_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-work-graph-events-persistence-guard-preview-gate.sh
)"
upstream_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_preview.rs
)"
upstream_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-replay-readback-rerun-preview-gate.sh
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
    "work_graph_events_persistence_guard_contract",
    "work_graph_events_event_store_enablement_contract",
    "work_graph_events_replay_readback_execution_prerequisite",
    "work_graph_events_adapter_enforcement_guard",
    "work_graph_events_no_persistence_guard",
    "work_graph_events_persistence_guard_blocker_mapping"
  ];
  def evidence_fields: [
    "source_surface_id",
    "source_category",
    "replay_readback_rerun_decision_ref",
    "persistence_guard_contract_id",
    "event_store_enablement_contract_id",
    "replay_readback_prerequisite_contract_id",
    "adapter_enforcement_guard_contract_id",
    "no_persistence_guard_id",
    "residual_source_blocker_ids",
    "next_required_gate"
  ];
  def plan_id($source):
    $source + "_append_only_work_graph_events_persistence_guard";
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
      expected_runtime_state: "preview_only_no_event_persistence",
      prerequisite_gate_ids: ["hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_preview_gate"],
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
      required_before_event_store_enablement: true,
      satisfied_by_preview: false
    };
  def blocker($id; $severity; $category; $sources; $stages; $plans; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_source_surface_ids: $sources,
      affected_persistence_guard_stage_ids: $stages,
      affected_persistence_guard_plan_ids: $plans,
      required_before_event_store_enablement: true,
      recommended_fix: $fix
    };
  ($upstream.decision_deltas | map(select(.residual_source_blocker_ids | index("append_only_work_graph_events_disabled")))) as $source_decisions
  | ($source_decisions | map(.source_surface_id)) as $all_sources
  | ($source_decisions | map({
      source_surface_id: .source_surface_id,
      source_category: .source_category,
      persistence_guard_plan_id: plan_id(.source_surface_id),
      previous_enforcement_decision: .work_graph_events_replay_readback_rerun_enforcement_decision,
      persistence_guard_state: "work_graph_events_persistence_guard_contract_ready_preview",
      required_persistence_guard_stage_ids: stage_ids,
      expected_evidence_field_ids: evidence_fields,
      residual_source_blocker_ids: .residual_source_blocker_ids,
      persistence_guard_contract_ready_preview: true,
      event_store_enablement_contract_ready_preview: true,
      replay_readback_prerequisite_ready_preview: true,
      adapter_enforcement_guard_ready_preview: true,
      no_persistence_guard_ready_preview: true,
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
  | ($plans | map(.persistence_guard_plan_id)) as $plan_ids
  | [
      stage("work_graph_events_persistence_guard_contract"; "critical"; "persistence_guard"; $all_sources; ["event_persistence_guard_contract_ready","source_surface_persistence_boundary_ready","redacted_evidence_persistence_boundary_ready","idempotency_key_persistence_boundary_ready","event_store_disable_switch_ready"]),
      stage("work_graph_events_event_store_enablement_contract"; "critical"; "event_store_enablement"; $all_sources; ["append_only_event_store_enablement_contract_ready","event_schema_registry_enablement_contract_ready","event_sequence_allocator_contract_ready","event_store_replay_cursor_contract_ready","event_store_operator_disable_contract_ready"]),
      stage("work_graph_events_replay_readback_execution_prerequisite"; "critical"; "replay_readback"; $all_sources; ["replay_execution_prerequisite_contract_ready","readback_probe_prerequisite_contract_ready","rollback_anchor_prerequisite_contract_ready","duplicate_suppression_prerequisite_contract_ready","timeline_order_prerequisite_contract_ready"]),
      stage("work_graph_events_adapter_enforcement_guard"; "high"; "adapter_enforcement"; $all_sources; ["canonical_adapter_enforcement_guard_ready","scheduler_admission_enforcement_guard_ready","terminal_task_result_enforcement_guard_ready","role_manifest_enforcement_guard_ready","projection_partial_gap_guard_ready"]),
      stage("work_graph_events_no_persistence_guard"; "critical"; "no_persistence_guard"; $all_sources; ["no_event_store_write_guard_ready","no_wal_write_guard_ready","no_checkpoint_write_guard_ready","no_replay_execution_guard_ready","no_readback_execution_guard_ready","no_adapter_enforcement_guard_ready"]),
      stage("work_graph_events_persistence_guard_blocker_mapping"; "high"; "blocker_mapping"; $all_sources; ["append_only_events_disabled_blocker_mapping_ready","replay_readback_disabled_blocker_mapping_ready","adapter_enforcement_disabled_blocker_mapping_ready","partial_gap_blocker_mapping_ready","readback_missing_blocker_mapping_ready"])
    ] as $stages
  | [
      guard("work_graph_events_persistence_disabled"; "critical"; "event_store"),
      guard("event_store_enablement_disabled"; "critical"; "event_store"),
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
      blocker("append_only_work_graph_events_disabled"; "critical"; "event_store_enablement"; sources_for("append_only_work_graph_events_disabled"); stage_ids; $plan_ids; "keep event persistence disabled until persistence guard readback and event-store enablement are promoted"),
      blocker("replay_readback_execution_disabled"; "critical"; "replay_readback"; sources_for("replay_readback_execution_disabled"); ["work_graph_events_replay_readback_execution_prerequisite"]; $plan_ids; "keep replay/readback execution disabled until event-store persistence and rollback anchors are promoted"),
      blocker("runtime_canonical_adapter_enforcement_disabled"; "high"; "adapter_enforcement"; sources_for("runtime_canonical_adapter_enforcement_disabled"); ["work_graph_events_adapter_enforcement_guard"]; $plan_ids; "keep runtime adapter enforcement disabled until append-only events are persisted and read back"),
      blocker("canonical_adapter_projection_partial_or_gap"; "high"; "projection_coverage"; sources_for("canonical_adapter_projection_partial_or_gap"); ["work_graph_events_adapter_enforcement_guard"]; $plan_ids; "close partial/gap adapter projections before authoritative event persistence"),
      blocker("append_only_work_graph_events_persistence_guard_readback_missing"; "medium"; "readback_preview"; $all_sources; stage_ids; $plan_ids; "run persistence guard readback before applying no-persistence outcomes")
    ] as $blockers
  | ($upstream.required_prior_gates + [$upstream.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_append_only_work_graph_events_persistence_guard_preview_gate",
      schema_version: "work_graph_append_only_work_graph_events_persistence_guard_preview_v1",
      preview_mode: "read_only_append_only_work_graph_events_persistence_guard_preview_no_persistence",
      upstream_replay_readback_rerun_gate: "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_preview_gate",
      source_surface_count: $upstream.source_surface_count,
      persistence_guard_plan_count: ($plans | length),
      persistence_guard_stage_count: ($stages | length),
      persistence_guard_stage_source_ref_count: ($stages | map(.affected_source_surface_ids | length) | add),
      persistence_guard_stage_contract_ref_count: ($stages | map(.required_contract_ref_ids | length) | add),
      persistence_guard_plan_stage_ref_count: ($plans | map(.required_persistence_guard_stage_ids | length) | add),
      persistence_guard_plan_evidence_field_ref_count: ($plans | map(.expected_evidence_field_ids | length) | add),
      append_only_work_graph_events_primary_blocked_source_count: (sources_for("append_only_work_graph_events_disabled") | length),
      replay_readback_execution_blocked_source_count: (sources_for("replay_readback_execution_disabled") | length),
      partial_or_gap_blocked_source_count: (sources_for("canonical_adapter_projection_partial_or_gap") | length),
      guard_count: ($guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      persistence_guard_plans: $plans,
      persistence_guard_stage_plans: $stages,
      guards: $guards,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_append_only_work_graph_events_persistence_guard_readback_preview_gate",
      ready_for_persistence_guard_readback_preview: true,
      ready_for_append_only_work_graph_events: false,
      ready_for_event_store_enablement: false,
      ready_for_replay_readback_execution: false,
      ready_for_runtime_adapter_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        append_only_work_graph_events_persistence_guard_preview: {
          rust_module_present: $preview_rust_module_present,
          report_script_present: $preview_report_script_present,
          gate_script_present: $preview_gate_script_present
        },
        work_graph_events_replay_readback_rerun: {
          rust_module_present: $upstream_rust_module_present,
          gate_script_present: $upstream_gate_script_present,
          upstream_gate: ($upstream.gate == "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_preview_gate")
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
