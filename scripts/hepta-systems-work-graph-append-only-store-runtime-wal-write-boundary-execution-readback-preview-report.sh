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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-runtime-wal-write-boundary-execution-readback.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$tmpdir/cache"
fi

capture_json_report \
  "hepta-work-graph-append-only-store-runtime-wal-write-boundary-execution-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-preview-report.sh" \
  >"$tmpdir/preview.json"

readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_preview.rs
)"
readback_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-readback-preview-report.sh
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-readback-preview-gate.sh
)"
preview_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-runtime-wal-write-boundary-execution-preview-gate.sh
)"

jq -n \
  --slurpfile preview "$tmpdir/preview.json" \
  --argjson readback_rust_module_present "$readback_rust_module_present" \
  --argjson readback_report_script_present "$readback_report_script_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  --argjson preview_gate_script_present "$preview_gate_script_present" \
  '
  $preview[0] as $preview
  | def readback_plan_id($source): "append_only_store_runtime_wal_write_boundary_execution_readback_plan__" + $source;
  def affected_readback_plan_ids($sources; $plans):
      [$plans[] as $plan | select($sources | index($plan.source_surface_id)) | $plan.id];
  def drift_detector($id; $fields): {
      id: $id,
      compared_field_ids: $fields,
      severity: "high",
      blocks_application_preview: true,
      performs_readback: false
    };
  ($preview.wal_write_boundary_execution_plans | map({
      id: readback_plan_id(.source_surface_id),
      source_surface_id: .source_surface_id,
      source_category: .source_category,
      wal_write_boundary_execution_plan_id: .wal_write_boundary_execution_plan_id,
      required_wal_write_boundary_execution_stage_ids: .required_wal_write_boundary_execution_stage_ids,
      residual_source_blocker_ids: .residual_source_blocker_ids,
      required_evidence_field_ids: .expected_evidence_field_ids,
      readback_state: "readback_verified_from_wal_write_boundary_execution_preview_no_execution",
      required_before_application: true,
      performs_readback: false,
      writes_wal: false,
      writes_checkpoint: false,
      mutates_idempotency_index: false,
      executes_replay: false,
      executes_rollback: false,
      mutates_runtime: false
    })) as $readback_plans
  | ($preview.wal_write_boundary_execution_stage_plans | map({
      id: ("wal_write_boundary_execution_stage_readback_assertion__" + .id),
      stage_id: .id,
      category: .category,
      affected_source_surface_ids: .affected_source_surface_ids,
      required_contract_ref_ids: .required_contract_ref_ids,
      expected_runtime_state: "readback_verified_contract_ready_runtime_disabled",
      contract_ready_preview: .contract_ready_preview,
      runtime_enabled_after_readback: false,
      declared_writes_wal: .writes_wal,
      declared_writes_checkpoint: .writes_checkpoint,
      declared_mutates_idempotency_index: .mutates_idempotency_index,
      declared_executes_replay: .executes_replay,
      declared_executes_readback: .executes_readback,
      declared_executes_rollback: .executes_rollback,
      performs_readback: false,
      mutates_runtime: false
    })) as $stage_assertions
  | ($readback_plans | map({
      id: ("wal_write_boundary_execution_evidence_field_readback_assertion__" + .source_surface_id),
      source_surface_id: .source_surface_id,
      required_evidence_field_ids: .required_evidence_field_ids,
      required_field_count: (.required_evidence_field_ids | length),
      expected_evidence_state: "evidence_fields_declared_not_persisted",
      performs_readback: false,
      persists_evidence: false
    })) as $evidence_assertions
  | ($preview.guards | map({
      id: ("wal_write_boundary_execution_guard_readback_assertion__" + .id),
      guard_id: .id,
      severity: .severity,
      guard_scope: .guard_scope,
      expected_guard_state: "guard_declared_and_runtime_mutation_prevented",
      required_before_wal_write_boundary_execution: .required_before_wal_write_boundary_execution,
      satisfied_by_readback: false,
      mutates_runtime: false
    })) as $guard_assertions
  | ($preview.blockers + [{
      id: "wal_write_boundary_execution_application_missing",
      severity: "high",
      category: "application_preview",
      affected_source_surface_ids: ($readback_plans | map(.source_surface_id)),
      affected_wal_write_boundary_execution_stage_ids: ($readback_plans[0].required_wal_write_boundary_execution_stage_ids // []),
      blocks_wal_write_boundary_execution: true,
      recommended_fix: "apply readback-verified WAL write-boundary execution plans before any idempotency index mutation, WAL replay, rollback/readback, or projection enforcement promotion"
    }]) as $blockers
  | ($blockers | map({
      id: ("wal_write_boundary_execution_blocker_mapping_readback_assertion__" + .id),
      blocker_id: .id,
      severity: .severity,
      category: .category,
      affected_source_surface_ids: .affected_source_surface_ids,
      affected_wal_write_boundary_execution_stage_ids: .affected_wal_write_boundary_execution_stage_ids,
      affected_readback_plan_ids: affected_readback_plan_ids(.affected_source_surface_ids; $readback_plans),
      expected_blocker_state: "blocker_mapping_readback_verified_no_mutation",
      blocks_wal_write_boundary_execution: true,
      performs_readback: false,
      mutates_runtime: false
    })) as $blocker_assertions
  | ([
      drift_detector("wal_write_boundary_execution_plan_alignment"; ["wal_write_boundary_execution_plan_id", "required_wal_write_boundary_execution_stage_ids"]),
      drift_detector("wal_write_boundary_execution_stage_contract_alignment"; ["stage_id", "required_contract_ref_ids"]),
      drift_detector("wal_write_boundary_execution_evidence_field_alignment"; ["source_surface_id", "required_evidence_field_ids"]),
      drift_detector("wal_write_boundary_execution_guard_no_mutation_alignment"; ["guard_id", "mutates_runtime"]),
      drift_detector("wal_write_boundary_execution_blocker_mapping_alignment"; ["blocker_id", "affected_readback_plan_ids"]),
      drift_detector("wal_write_boundary_execution_side_effect_alignment"; ["side_effects", "idempotency_index_mutated", "runtime_mutation_performed"]),
      drift_detector("wal_write_boundary_execution_upstream_gate_alignment"; ["upstream_wal_write_boundary_execution_preview_gate", "recommended_next_gate"])
    ]) as $drift_detectors
  | ($preview.required_prior_gates + [$preview.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_preview_gate",
      schema_version: "work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_preview_v1",
      preview_mode: "read_only_append_only_store_runtime_wal_write_boundary_execution_readback_no_execution",
      upstream_wal_write_boundary_execution_preview_gate: $preview.gate,
      source_surface_count: $preview.source_surface_count,
      wal_write_boundary_execution_plan_count: ($preview.wal_write_boundary_execution_plans | length),
      readback_plan_count: ($readback_plans | length),
      stage_assertion_count: ($stage_assertions | length),
      evidence_field_assertion_count: ($evidence_assertions | length),
      guard_assertion_count: ($guard_assertions | length),
      blocker_mapping_assertion_count: ($blocker_assertions | length),
      drift_detector_count: ($drift_detectors | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      wal_write_boundary_execution_stage_source_ref_count: ($stage_assertions | map(.affected_source_surface_ids | length) | add),
      wal_write_boundary_execution_stage_contract_ref_count: ($stage_assertions | map(.required_contract_ref_ids | length) | add),
      wal_write_boundary_execution_plan_stage_ref_count: ($readback_plans | map(.required_wal_write_boundary_execution_stage_ids | length) | add),
      wal_write_boundary_execution_plan_evidence_field_ref_count: ($evidence_assertions | map(.required_field_count) | add),
      blocker_mapping_source_ref_count: ($blocker_assertions | map(.affected_source_surface_ids | length) | add),
      blocker_mapping_stage_ref_count: ($blocker_assertions | map(.affected_wal_write_boundary_execution_stage_ids | length) | add),
      readback_plans: $readback_plans,
      stage_assertions: $stage_assertions,
      evidence_field_assertions: $evidence_assertions,
      guard_assertions: $guard_assertions,
      blocker_mapping_assertions: $blocker_assertions,
      drift_detectors: $drift_detectors,
      blockers: ($blockers | map(. + {
        affected_readback_plan_ids: affected_readback_plan_ids(.affected_source_surface_ids; $readback_plans),
        blocks_wal_write_boundary_execution: true
      })),
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_application_preview_gate",
      ready_for_runtime_wal_write_boundary_execution_application_preview: true,
      ready_for_readback_execution: false,
      ready_for_replay_execution: false,
      ready_for_wal_write: false,
      ready_for_checkpoint_write: false,
      ready_for_wal_write_boundary_execution: false,
      ready_for_rollback_execution: false,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        wal_write_boundary_execution_readback: {
          rust_module_present: $readback_rust_module_present,
          report_script_present: $readback_report_script_present,
          gate_script_present: $readback_gate_script_present
        },
        wal_write_boundary_execution_preview: {
          upstream_gate: ($preview.gate == "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_preview_gate"),
          gate_script_present: $preview_gate_script_present,
          recommended_next_matches: ($preview.recommended_next_gate == "hepta_work_graph_append_only_store_runtime_wal_write_boundary_execution_readback_preview_gate")
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        wal_written: false,
        checkpoint_written: false,
        durable_store_switch_enabled: false,
        idempotency_index_mutated: false,
        append_only_store_enabled: false,
        projection_enforcement_enabled: false,
        scheduler_admission_enforced: false,
        approval_recorded: false,
        operator_review_recorded: false,
        side_effect_lock_established: false,
        task_result_enforcement_enabled: false,
        task_result_persisted: false,
        role_manifest_enforcement_enabled: false,
        readback_executed: false,
        replay_executed: false,
        rollback_executed: false,
        runtime_mutation_performed: false,
        external_send_performed: false,
        model_invoked: false,
        agent_spawn_performed: false
      }
    }
  '
