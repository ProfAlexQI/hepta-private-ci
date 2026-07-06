#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

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
  "$ROOT/scripts/hepta-systems-work-graph-append-only-work-graph-events-shadow-write-readback-preview-report.sh"
)"

application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_work_graph_events_shadow_write_application_preview.rs
)"
application_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-work-graph-events-shadow-write-application-preview-report.sh
)"
application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-work-graph-events-shadow-write-application-preview-gate.sh
)"
readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_work_graph_events_shadow_write_readback_preview.rs
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-work-graph-events-shadow-write-readback-preview-gate.sh
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
    application_plan_id: ($plan.source_surface_id + "_append_only_work_graph_events_shadow_write_application"),
    readback_source_surface_id: $plan.source_surface_id,
    source_category: $plan.source_category,
    shadow_write_plan_id: $plan.shadow_write_plan_id,
    application_state: "work_graph_events_shadow_write_contract_ready_preview_after_application",
    readback_verified_by_preview: ($plan.readback_status == "readback_plan_ready"),
    shadow_write_contract_ready_preview: true,
    applies_to_runtime: false,
    persists_work_graph_events: false,
    writes_wal: false,
    writes_checkpoint: false,
    executes_replay: false,
    executes_readback: false,
    mutates_idempotency_index: false,
    enforces_adapter_projection: false,
    mutates_scheduler_admission: false,
    mutates_task_result_enforcement: false,
    mutates_role_manifest_enforcement: false
  };
  def source_outcome($plan): {
    source_surface_id: $plan.readback_source_surface_id,
    source_category: $plan.source_category,
    application_plan_id: $plan.application_plan_id,
    post_application_shadow_write_state: $plan.application_state,
    shadow_write_contract_ready_preview: $plan.shadow_write_contract_ready_preview,
    ready_for_shadow_write_readiness_rerun_preview: true,
    ready_for_append_only_work_graph_events: false,
    applies_to_runtime: false
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
      blocker("append_only_work_graph_events_disabled"; "high"; "append_only_fact_source"; $all_sources; $all_plan_ids; "keep WorkGraph event persistence disabled until shadow-write readiness rerun is verified"),
      blocker("runtime_canonical_adapter_enforcement_disabled"; "high"; "runtime_adapter_enforcement"; $all_sources; $all_plan_ids; "keep canonical adapter enforcement disabled until append-only events are promoted"),
      blocker("canonical_adapter_projection_partial_or_gap"; "high"; "projection_coverage"; $partial_gap_sources; $all_plan_ids; "close partial/gap adapter source mappings before authoritative event projection"),
      blocker("replay_readback_execution_disabled"; "high"; "replay_readback"; $all_sources; $all_plan_ids; "keep replay/readback disabled until shadow-write evidence is promoted behind operator review"),
      blocker("work_graph_events_shadow_write_readiness_rerun_missing"; "medium"; "readiness_rerun"; $all_sources; $all_plan_ids; "rerun enforcement readiness after no-mutation shadow-write application outcomes are available")
    ] as $blockers
  | [
      application_guard("no_work_graph_event_persistence"; "critical"; "event_store"),
      application_guard("no_wal_write"; "critical"; "wal"),
      application_guard("no_checkpoint_write"; "critical"; "checkpoint"),
      application_guard("no_replay_execution"; "critical"; "replay"),
      application_guard("no_readback_execution"; "critical"; "readback"),
      application_guard("no_idempotency_index_mutation"; "critical"; "idempotency"),
      application_guard("no_adapter_projection_enforcement"; "critical"; "adapter_projection"),
      application_guard("no_scheduler_admission_enforcement"; "high"; "scheduler_admission"),
      application_guard("no_agent_spawn"; "high"; "agent_spawn"),
      application_guard("no_external_send_or_model_invocation"; "high"; "external_effects")
    ] as $application_guards
  | ($readback.required_prior_gates + ["hepta_work_graph_append_only_work_graph_events_shadow_write_readback_preview_gate"]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_append_only_work_graph_events_shadow_write_application_preview_gate",
      schema_version: "work_graph_append_only_work_graph_events_shadow_write_application_preview_v1",
      preview_mode: "read_only_append_only_work_graph_events_shadow_write_application_preview_no_mutation",
      readback_plan_count: $readback.readback_plan_count,
      application_plan_count: ($application_plans | length),
      source_outcome_count: ($source_outcomes | length),
      shadow_write_contract_ready_preview_count: ($source_outcomes | map(select(.shadow_write_contract_ready_preview == true)) | length),
      event_schema_application_count: $readback.event_schema_assertion_count,
      stage_application_count: $readback.stage_assertion_count,
      source_mapping_application_count: $readback.source_mapping_assertion_count,
      event_binding_application_count: $readback.event_binding_assertion_count,
      idempotency_key_application_count: $readback.idempotency_key_assertion_count,
      guard_application_count: $readback.guard_assertion_count,
      blocker_application_count: $readback.blocker_mapping_assertion_count,
      application_guard_count: ($application_guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      application_plans: $application_plans,
      source_outcomes: $source_outcomes,
      event_schema_applications: ($readback.event_schema_assertions | map({
        application_id: (.event_schema_id + "_event_schema_application"),
        event_schema_id: .event_schema_id,
        category: .category,
        required_field_ids: .required_field_ids,
        shadow_write_only: .shadow_write_only,
        persists_event_schema: false
      })),
      stage_applications: ($readback.stage_assertions | map({
        application_id: (.stage_id + "_stage_application"),
        stage_id: .stage_id,
        affected_source_surface_ids: .affected_source_surface_ids,
        required_contract_ref_ids: .required_contract_ref_ids,
        contract_ready_preview: .contract_ready_preview,
        persists_work_graph_events: false,
        executes_replay: false,
        executes_readback: false
      })),
      source_mapping_applications: ($readback.source_mapping_assertions | map({
        application_id: (.source_surface_id + "_source_mapping_application"),
        source_surface_id: .source_surface_id,
        canonical_node_kind: .canonical_node_kind,
        canonical_collection_ids: .canonical_collection_ids,
        timeline_event_type_ids: .timeline_event_type_ids,
        source_mapping_ready_preview: .source_mapping_ready_preview,
        persists_mapping: false
      })),
      event_binding_applications: ($readback.event_binding_assertions | map({
        application_id: (.source_surface_id + "_" + .event_schema_id + "_event_binding_application"),
        source_surface_id: .source_surface_id,
        event_schema_id: .event_schema_id,
        binding_ready_preview: .binding_ready_preview,
        persists_event_binding: false
      })),
      idempotency_key_applications: ($readback.idempotency_key_assertions | map({
        application_id: (.source_surface_id + "_idempotency_key_application"),
        source_surface_id: .source_surface_id,
        idempotency_key_field_ids: .idempotency_key_field_ids,
        idempotency_key_ready_preview: .idempotency_key_ready_preview,
        mutates_idempotency_index: false
      })),
      guard_applications: ($readback.guard_assertions | map({
        application_id: (.guard_id + "_guard_application"),
        guard_id: .guard_id,
        guard_scope: .guard_scope,
        required_before_shadow_write: .required_before_shadow_write,
        satisfied_by_preview: .satisfied_by_preview,
        mutates_runtime: false
      })),
      blocker_applications: ($readback.blocker_mapping_assertions | map({
        application_id: (.blocker_id + "_blocker_application"),
        blocker_id: .blocker_id,
        affected_source_surface_ids: .affected_source_surface_ids,
        affected_shadow_write_stage_ids: .affected_shadow_write_stage_ids,
        expected_blocker_state: "mapped_for_work_graph_events_shadow_write_rerun_preview",
        readback_verified_by_preview: true,
        clears_application_missing_blocker: (.blocker_id == "append_only_work_graph_events_shadow_write_readback_missing"),
        mutates_runtime: false
      })),
      application_guards: $application_guards,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_shadow_write_rerun_preview_gate",
      ready_for_shadow_write_readiness_rerun_preview: true,
      ready_for_append_only_work_graph_events: false,
      ready_for_replay_readback: false,
      ready_for_runtime_adapter_enforcement: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_task_result_enforcement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        append_only_work_graph_events_shadow_write_application: {
          rust_module_present: $application_rust_module_present,
          report_script_present: $application_report_script_present,
          gate_script_present: $application_gate_script_present
        },
        append_only_work_graph_events_shadow_write_readback: {
          rust_module_present: $readback_rust_module_present,
          gate_script_present: $readback_gate_script_present,
          upstream_gate: ($readback.gate == "hepta_work_graph_append_only_work_graph_events_shadow_write_readback_preview_gate" and $readback.status == "ready")
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
        idempotency_index_mutated: false,
        adapter_projection_enforced: false,
        scheduler_admission_enforced: false,
        task_result_enforcement_enabled: false,
        role_manifest_enforcement_enabled: false,
        runtime_mutation_performed: false,
        agent_spawn_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
