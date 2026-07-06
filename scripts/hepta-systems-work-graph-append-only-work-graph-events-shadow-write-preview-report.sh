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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-work-graph-events-shadow-write.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$tmpdir/cache"
fi

capture_json_report \
  "hepta-work-graph-unified-projection-enforcement-readiness-canonical-adapter-inventory-rerun-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-canonical-adapter-inventory-rerun-preview-report.sh" \
  >"$tmpdir/rerun.json"
capture_json_report \
  "hepta-work-graph-canonical-adapter-inventory-application-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-canonical-adapter-inventory-application-preview-report.sh" \
  >"$tmpdir/application.json"

shadow_write_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_work_graph_events_shadow_write_preview.rs
)"
shadow_write_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-work-graph-events-shadow-write-preview-report.sh
)"
shadow_write_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-work-graph-events-shadow-write-preview-gate.sh
)"
canonical_rerun_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-canonical-adapter-inventory-rerun-preview-gate.sh
)"
canonical_application_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-canonical-adapter-inventory-application-preview-gate.sh
)"

jq -n \
  --slurpfile rerun "$tmpdir/rerun.json" \
  --slurpfile application "$tmpdir/application.json" \
  --argjson shadow_write_rust_module_present "$shadow_write_rust_module_present" \
  --argjson shadow_write_report_script_present "$shadow_write_report_script_present" \
  --argjson shadow_write_gate_script_present "$shadow_write_gate_script_present" \
  --argjson canonical_rerun_gate_script_present "$canonical_rerun_gate_script_present" \
  --argjson canonical_application_gate_script_present "$canonical_application_gate_script_present" \
  '
  $rerun[0] as $rerun
  | $application[0] as $application
  | def stage_ids: [
      "work_graph_event_schema_contract",
      "work_graph_event_source_surface_mapping",
      "work_graph_event_idempotency_key_contract",
      "work_graph_event_replay_readback_guard",
      "work_graph_event_no_persistence_guard",
      "work_graph_event_blocker_mapping"
    ];
  def idempotency_key_fields: [
      "sourceSurfaceId",
      "traceId",
      "canonicalNodeId",
      "eventType",
      "sequenceKey"
    ];
  def event_schema_ids_for($source):
      if $source == "update_plan_tool" then ["PlanStepCreated"]
      elif $source == "plan_mode_proposed_plan_blocks" then ["PlanStepCreated"]
      elif $source == "app_server_turn_plan_notification" then ["PlanStepCreated","TimelineEventAppended"]
      elif $source == "multi_agent_v2_thread_spawn" then ["AgentTaskSpawned","MessageLinked","TimelineEventAppended"]
      elif $source == "multi_agent_v2_mailbox_wait" then ["MessageLinked","TimelineEventAppended"]
      elif $source == "hepta_runtime_multi_agent_reducer" then ["AgentTaskSpawned","TaskResultReported"]
      elif $source == "agent_jobs_batch_workers" then ["AgentTaskSpawned","TaskResultReported","TimelineEventAppended"]
      elif $source == "hepta_runtime_task_board" then ["LeaseAcquired","LeaseReleased","TaskResultReported"]
      elif $source == "hepta_runtime_worker_tasks" then ["TaskResultReported","ArtifactProduced","TimelineEventAppended"]
      elif $source == "hepta_runtime_scheduler_store" then ["GateEvaluated","LeaseAcquired"]
      elif $source == "hepta_runtime_approval_broker" then ["ApprovalRequired","ApprovalRecorded"]
      elif $source == "hepta_runtime_agent_harness" then ["ArtifactProduced","GateEvaluated","TimelineEventAppended"]
      else ["TimelineEventAppended"]
      end;
  def find_by_source($items; $source):
      ($items[]? | select(.source_surface_id == $source)) // {};
  def plan_id($source):
      $source + "_append_only_work_graph_events_shadow_write";
  def plan($decision):
      find_by_source($application.identity_applications; $decision.source_surface_id) as $identity
      | find_by_source($application.collection_binding_applications; $decision.source_surface_id) as $collection
      | find_by_source($application.timeline_event_applications; $decision.source_surface_id) as $timeline
      | {
          source_surface_id: $decision.source_surface_id,
          source_category: $decision.source_category,
          shadow_write_plan_id: plan_id($decision.source_surface_id),
          previous_enforcement_decision: $decision.canonical_adapter_inventory_rerun_enforcement_decision,
          shadow_write_state: "append_only_work_graph_events_shadow_write_contract_defined_preview_only",
          canonical_node_kind: ($identity.canonical_node_kind // "unknown"),
          required_identity_fields: ($identity.required_identity_fields // []),
          canonical_collection_ids: ($collection.canonical_collection_ids // []),
          timeline_event_type_ids: ($timeline.timeline_event_type_ids // []),
          event_schema_ids: event_schema_ids_for($decision.source_surface_id),
          required_shadow_write_stage_ids: stage_ids,
          idempotency_key_field_ids: idempotency_key_fields,
          residual_source_blocker_ids: $decision.residual_source_blocker_ids,
          canonical_adapter_inventory_contract_ready: $decision.canonical_adapter_inventory_contract_ready,
          shadow_write_contract_ready_preview: true,
          applies_to_runtime: false,
          persists_work_graph_events: false,
          writes_wal: false,
          writes_checkpoint: false,
          executes_replay: false,
          executes_readback: false,
          mutates_runtime: false
        };
  def event_schema($id; $category): {
      id: $id,
      category: $category,
      required_field_ids: [
        "eventId",
        "eventType",
        "sourceSurfaceId",
        "traceId",
        "canonicalNodeId",
        "idempotencyKey",
        "payloadHash",
        "redactedEvidenceRefs"
      ],
      idempotency_scope: "source_surface_trace_event_type_sequence",
      redaction_required: true,
      payload_hash_required: true,
      replay_readback_required: true,
      shadow_write_only: true,
      persists_event_after_preview: false
    };
  def stage($id; $category; $sources; $contracts): {
      id: $id,
      priority: "p0",
      category: $category,
      affected_source_surface_ids: $sources,
      required_contract_ref_ids: $contracts,
      expected_runtime_state: "contract_ready_preview_persistence_disabled",
      contract_ready_preview: true,
      persists_work_graph_events_after_preview: false,
      executes_replay_after_preview: false,
      executes_readback_after_preview: false,
      mutates_runtime_after_preview: false
    };
  def guard($id; $severity; $scope): {
      id: $id,
      severity: $severity,
      guard_scope: $scope,
      required_before_shadow_write: true,
      satisfied_by_preview: false
    };
  def stages_for_blocker($id):
      if $id == "append_only_work_graph_events_disabled" then stage_ids
      elif $id == "runtime_canonical_adapter_enforcement_disabled" then ["work_graph_event_replay_readback_guard","work_graph_event_no_persistence_guard"]
      elif $id == "canonical_adapter_projection_partial_or_gap" then ["work_graph_event_source_surface_mapping","work_graph_event_blocker_mapping"]
      else ["work_graph_event_blocker_mapping"]
      end;
  def category_for_blocker($id; $category):
      if $id == "append_only_work_graph_events_disabled" then "append_only_fact_source"
      elif $id == "runtime_canonical_adapter_enforcement_disabled" then "runtime_adapter_enforcement"
      elif $id == "canonical_adapter_projection_partial_or_gap" then "projection_coverage"
      else $category
      end;
  def blocker($id; $severity; $category; $sources; $stages; $fix): {
      id: $id,
      severity: $severity,
      category: $category,
      affected_source_surface_ids: $sources,
      affected_shadow_write_stage_ids: $stages,
      affected_shadow_write_plan_ids: ($sources | map(plan_id(.))),
      required_before_shadow_write: true,
      recommended_fix: $fix
    };
  ($rerun.decision_deltas
    | map(select(.canonical_adapter_inventory_rerun_enforcement_decision == "deny_append_only_work_graph_events_disabled") | plan(.))) as $plans
  | ($plans | map(.source_surface_id)) as $source_ids
  | [
      event_schema("PlanStepCreated"; "planning"),
      event_schema("AgentTaskSpawned"; "multi_agent"),
      event_schema("MessageLinked"; "multi_agent"),
      event_schema("TaskResultReported"; "task_result"),
      event_schema("ArtifactProduced"; "artifact"),
      event_schema("ApprovalRequired"; "operator_control"),
      event_schema("ApprovalRecorded"; "operator_control"),
      event_schema("LeaseAcquired"; "scheduler"),
      event_schema("LeaseReleased"; "scheduler"),
      event_schema("GateEvaluated"; "gate"),
      event_schema("TimelineEventAppended"; "timeline")
    ] as $event_schemas
  | [
      stage("work_graph_event_schema_contract"; "event_schema"; $source_ids; ["event_type_contract_ready","event_payload_contract_ready","event_redaction_contract_ready","event_hash_contract_ready","event_artifact_ref_contract_ready","event_version_contract_ready"]),
      stage("work_graph_event_source_surface_mapping"; "source_surface_mapping"; $source_ids; ["source_surface_id_mapping_ready","canonical_node_kind_mapping_ready","canonical_edge_kind_mapping_ready","canonical_collection_mapping_ready","timeline_event_mapping_ready","task_result_mapping_ready"]),
      stage("work_graph_event_idempotency_key_contract"; "idempotency_key"; $source_ids; ["source_surface_id_key_ready","trace_id_key_ready","canonical_node_id_key_ready","event_type_key_ready","sequence_key_ready"]),
      stage("work_graph_event_replay_readback_guard"; "replay_readback_guard"; $source_ids; ["shadow_replay_cursor_contract_ready","readback_probe_contract_ready","duplicate_suppression_contract_ready","timeline_ordering_contract_ready","rollback_anchor_contract_ready","event_integrity_digest_contract_ready"]),
      stage("work_graph_event_no_persistence_guard"; "preview_no_persistence_guard"; $source_ids; ["work_graph_events_no_persist_guard_ready","wal_no_write_guard_ready","checkpoint_no_write_guard_ready","durable_store_switch_disabled_guard_ready","scheduler_no_admission_guard_ready","runtime_no_mutation_guard_ready","external_send_noop_guard_ready"]),
      stage("work_graph_event_blocker_mapping"; "blocker_mapping"; $source_ids; ["append_only_events_blocker_mapping_ready","canonical_adapter_enforcement_blocker_mapping_ready","partial_gap_blocker_mapping_ready","readback_missing_blocker_mapping_ready"])
    ] as $stage_plans
  | [
      guard("work_graph_events_shadow_write_preview_only"; "medium"; "preview_boundary"),
      guard("work_graph_events_persistence_disabled"; "critical"; "event_store"),
      guard("wal_write_disabled"; "critical"; "wal"),
      guard("checkpoint_write_disabled"; "critical"; "checkpoint"),
      guard("replay_execution_disabled"; "critical"; "replay"),
      guard("readback_execution_disabled"; "critical"; "readback"),
      guard("adapter_projection_enforcement_disabled"; "critical"; "adapter_projection"),
      guard("scheduler_admission_enforcement_disabled"; "high"; "scheduler_admission"),
      guard("no_agent_spawn"; "high"; "agent_spawn"),
      guard("no_external_send_or_model_invocation"; "high"; "external_effects")
    ] as $guards
  | (($rerun.residual_blockers | map(blocker(
        .id;
        .severity;
        category_for_blocker(.id; .category);
        .affected_source_surface_ids;
        stages_for_blocker(.id);
        .recommended_fix
      ))) + [
        blocker(
          "append_only_work_graph_events_shadow_write_readback_missing";
          "high";
          "readback_preview";
          $source_ids;
          stage_ids;
          "read back shadow WorkGraph event contracts before any event-store persistence, replay/readback execution, or adapter enforcement"
        )
      ]) as $blockers
  | ($rerun.required_prior_gates + [$rerun.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "blocked",
      gate: "hepta_work_graph_append_only_work_graph_events_shadow_write_preview_gate",
      schema_version: "work_graph_append_only_work_graph_events_shadow_write_preview_v1",
      preview_mode: "read_only_append_only_work_graph_events_shadow_write_preview_no_persistence",
      upstream_canonical_adapter_inventory_rerun_gate: "hepta_work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview_gate",
      source_surface_count: ($rerun.source_surface_count),
      shadow_write_plan_count: ($plans | length),
      event_schema_count: ($event_schemas | length),
      source_event_binding_count: ($plans | map(.event_schema_ids | length) | add),
      shadow_write_stage_count: ($stage_plans | length),
      shadow_write_stage_source_ref_count: ($stage_plans | map(.affected_source_surface_ids | length) | add),
      shadow_write_stage_contract_ref_count: ($stage_plans | map(.required_contract_ref_ids | length) | add),
      shadow_write_plan_stage_ref_count: ($plans | map(.required_shadow_write_stage_ids | length) | add),
      shadow_write_plan_event_schema_ref_count: ($plans | map(.event_schema_ids | length) | add),
      idempotency_key_field_ref_count: ($plans | map(.idempotency_key_field_ids | length) | add),
      shadow_write_contract_ready_preview_count: ($plans | map(select(.shadow_write_contract_ready_preview)) | length),
      append_only_work_graph_events_primary_blocked_source_count: ($rerun.append_only_work_graph_events_primary_blocked_surface_count),
      partial_or_gap_blocked_source_count: ($rerun.partial_or_gap_blocked_surface_count),
      shadow_write_enabled_source_count: 0,
      guard_count: ($guards | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      shadow_write_plans: $plans,
      event_schemas: $event_schemas,
      stage_plans: $stage_plans,
      guards: $guards,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_append_only_work_graph_events_shadow_write_readback_preview_gate",
      ready_for_shadow_write_readback_preview: true,
      ready_for_shadow_write_application_preview: false,
      ready_for_append_only_work_graph_events: false,
      ready_for_replay_readback: false,
      ready_for_runtime_adapter_enforcement: false,
      ready_for_scheduler_admission_enforcement: false,
      ready_for_task_result_enforcement: false,
      ready_for_role_manifest_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        append_only_work_graph_events_shadow_write: {
          rust_module_present: $shadow_write_rust_module_present,
          report_script_present: $shadow_write_report_script_present,
          gate_script_present: $shadow_write_gate_script_present
        },
        canonical_adapter_inventory_rerun: {
          upstream_gate: ($rerun.gate == "hepta_work_graph_unified_projection_enforcement_readiness_canonical_adapter_inventory_rerun_preview_gate"),
          gate_script_present: $canonical_rerun_gate_script_present
        },
        canonical_adapter_inventory_application: {
          upstream_gate: ($application.gate == "hepta_work_graph_canonical_adapter_inventory_application_preview_gate"),
          gate_script_present: $canonical_application_gate_script_present
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
        adapter_projection_enforced: false,
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
