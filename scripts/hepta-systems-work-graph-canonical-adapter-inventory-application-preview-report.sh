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

readback_report="$("$ROOT/scripts/hepta-systems-work-graph-canonical-adapter-inventory-readback-preview-report.sh")"

application_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_canonical_adapter_inventory_application_preview.rs
)"
application_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-canonical-adapter-inventory-application-preview-report.sh
)"
application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-canonical-adapter-inventory-application-preview-gate.sh
)"
readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_canonical_adapter_inventory_readback_preview.rs
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-canonical-adapter-inventory-readback-preview-gate.sh
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
    application_plan_id: ($plan.source_surface_id + "_canonical_adapter_inventory_application"),
    readback_source_surface_id: $plan.source_surface_id,
    source_category: $plan.source_category,
    canonical_inventory_state: $plan.canonical_inventory_state,
    application_state: "canonical_adapter_inventory_contract_ready_preview_after_application",
    readback_verified_by_preview: ($plan.readback_status == "readback_plan_ready"),
    canonical_adapter_inventory_contract_ready_preview: true,
    applies_to_runtime: false,
    persists_work_graph_events: false,
    enforces_adapter_projection: false,
    mutates_scheduler_admission: false,
    mutates_task_result_enforcement: false,
    mutates_role_manifest_enforcement: false
  };
  def source_outcome($plan): {
    source_surface_id: $plan.readback_source_surface_id,
    source_category: $plan.source_category,
    application_plan_id: $plan.application_plan_id,
    post_application_canonical_inventory_state: $plan.application_state,
    canonical_adapter_inventory_contract_ready_preview: $plan.canonical_adapter_inventory_contract_ready_preview,
    ready_for_canonical_adapter_inventory_readiness_rerun_preview: true,
    ready_for_append_only_work_graph_events: false,
    applies_to_runtime: false
  };
  def guard($id): {
    id: $id,
    severity: "high",
    required_before_runtime_enforcement: true,
    satisfied_by_preview: true
  };
  def blocker($id; $severity; $category; $sources; $plans; $fix): {
    id: $id,
    severity: $severity,
    category: $category,
    affected_source_surface_ids: $sources,
    affected_application_plan_ids: $plans,
    required_before_runtime_enforcement: true,
    recommended_fix: $fix
  };
  ($readback.readback_plans | map(application_plan(.))) as $application_plans
  | ($application_plans | map(source_outcome(.))) as $source_outcomes
  | ($application_plans | map(.readback_source_surface_id)) as $all_sources
  | ($application_plans | map(.application_plan_id)) as $all_plan_ids
  | ($application_plans | map(select(.canonical_inventory_state != "canonical_contract_ready_preview") | .readback_source_surface_id)) as $partial_gap_sources
  | [
      blocker("append_only_work_graph_events_disabled"; "high"; "append_only_fact_source"; $all_sources; $all_plan_ids; "shadow-write canonical WorkGraph events with replay/readback before enforcement"),
      blocker("runtime_canonical_adapter_enforcement_disabled"; "high"; "runtime_adapter_enforcement"; $all_sources; $all_plan_ids; "keep canonical adapters preview-only until append-only events and operator-review boundaries are promoted"),
      blocker("canonical_adapter_projection_partial_or_gap"; "high"; "projection_coverage"; $partial_gap_sources; $all_plan_ids; "close partial/gap source adapters before making the canonical projection authoritative"),
      blocker("canonical_adapter_inventory_readiness_rerun_missing"; "medium"; "readiness_rerun"; $all_sources; $all_plan_ids; "rerun enforcement readiness after no-mutation application outcomes are available")
    ] as $blockers
  | ($readback.required_prior_gates + ["hepta_work_graph_canonical_adapter_inventory_readback_preview_gate"]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_canonical_adapter_inventory_application_preview_gate",
      schema_version: "work_graph_canonical_adapter_inventory_application_preview_v1",
      preview_mode: "read_only_canonical_adapter_inventory_application_preview_no_mutation",
      readback_plan_count: $readback.readback_plan_count,
      application_plan_count: ($application_plans | length),
      source_outcome_count: ($source_outcomes | length),
      canonical_adapter_inventory_contract_ready_preview_count: ($source_outcomes | map(select(.canonical_adapter_inventory_contract_ready_preview == true)) | length),
      identity_application_count: $readback.identity_assertion_count,
      edge_kind_application_count: $readback.edge_kind_assertion_count,
      collection_binding_application_count: $readback.collection_binding_assertion_count,
      timeline_event_application_count: $readback.timeline_event_assertion_count,
      blocker_application_count: $readback.blocker_mapping_assertion_count,
      application_guard_count: 8,
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      application_plans: $application_plans,
      source_outcomes: $source_outcomes,
      identity_applications: ($readback.identity_assertions | map({
        application_id: (.source_surface_id + "_identity_application"),
        source_surface_id: .source_surface_id,
        canonical_node_kind: .canonical_node_kind,
        required_identity_fields: .required_identity_fields,
        deterministic_identity_ready_preview: .deterministic_identity_required,
        persists_identity: false
      })),
      edge_kind_applications: ($readback.edge_kind_assertions | map({
        application_id: (.source_surface_id + "_edge_kind_application"),
        source_surface_id: .source_surface_id,
        canonical_edge_kinds: .canonical_edge_kinds,
        edge_namespace: .edge_namespace,
        persists_edges: false
      })),
      collection_binding_applications: ($readback.collection_binding_assertions | map({
        application_id: (.source_surface_id + "_collection_binding_application"),
        source_surface_id: .source_surface_id,
        canonical_collection_ids: .canonical_collection_ids,
        persists_store_projection: false
      })),
      timeline_event_applications: ($readback.timeline_event_assertions | map({
        application_id: (.source_surface_id + "_timeline_event_application"),
        source_surface_id: .source_surface_id,
        timeline_event_type_ids: .timeline_event_type_ids,
        persists_timeline: false
      })),
      blocker_applications: ($readback.blocker_mapping_assertions | map({
        application_id: (.blocker_id + "_application"),
        blocker_id: .blocker_id,
        affected_source_surface_ids: .affected_source_surface_ids,
        expected_blocker_state: "mapped_for_canonical_adapter_inventory_rerun_preview",
        readback_verified_by_preview: true,
        clears_application_missing_blocker: (.blocker_id == "canonical_adapter_inventory_readback_missing"),
        mutates_runtime: false
      })),
      application_guards: [
        guard("no_work_graph_event_persistence"),
        guard("no_adapter_projection_enforcement"),
        guard("no_scheduler_admission_enforcement"),
        guard("no_task_result_enforcement"),
        guard("no_role_manifest_enforcement"),
        guard("no_agent_spawn"),
        guard("no_external_send"),
        guard("no_model_invocation")
      ],
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview_gate",
      ready_for_canonical_adapter_inventory_readiness_rerun_preview: true,
      ready_for_append_only_work_graph_events: false,
      ready_for_runtime_adapter_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        canonical_adapter_inventory_application: {
          rust_module_present: $application_rust_module_present,
          report_script_present: $application_report_script_present,
          gate_script_present: $application_gate_script_present
        },
        canonical_adapter_inventory_readback: {
          rust_module_present: $readback_rust_module_present,
          gate_script_present: $readback_gate_script_present,
          upstream_gate: ($readback.gate == "hepta_work_graph_canonical_adapter_inventory_readback_preview_gate" and $readback.status == "ready")
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
    }'
