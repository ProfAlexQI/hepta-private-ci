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

tmpdir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-operator-review-side-effect-lock-readback.XXXXXX")"
trap 'rm -rf "$tmpdir"' EXIT

if [[ -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$tmpdir/cache"
fi

capture_json_report \
  "hepta-work-graph-append-only-store-operator-review-side-effect-lock-preview-report" \
  "$ROOT/scripts/hepta-systems-work-graph-append-only-store-operator-review-side-effect-lock-preview-report.sh" \
  >"$tmpdir/preview.json"

readback_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_append_only_store_operator_review_side_effect_lock_readback_preview.rs
)"
readback_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-operator-review-side-effect-lock-readback-preview-report.sh
)"
readback_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-operator-review-side-effect-lock-readback-preview-gate.sh
)"
preview_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-append-only-store-operator-review-side-effect-lock-preview-gate.sh
)"

jq -n \
  --slurpfile preview "$tmpdir/preview.json" \
  --argjson readback_rust_module_present "$readback_rust_module_present" \
  --argjson readback_report_script_present "$readback_report_script_present" \
  --argjson readback_gate_script_present "$readback_gate_script_present" \
  --argjson preview_gate_script_present "$preview_gate_script_present" \
  '
  $preview[0] as $preview
  | def readback_plan_id($source): "operator_review_side_effect_lock_readback_plan__" + $source;
  def affected_readback_plan_ids($sources; $plans):
      [$plans[] as $plan | select($sources | index($plan.source_surface_id)) | $plan.id];
  def drift_detector($id; $fields): {
      id: $id,
      compared_field_ids: $fields,
      severity: "high",
      blocks_application_preview: true,
      performs_readback: false
    };
  ($preview.operator_review_packets | map(. as $packet
      | ($preview.side_effect_lock_plans[] | select(.source_surface_id == $packet.source_surface_id)) as $lock
      | ($preview.approval_evidence_boundaries[] | select(.source_surface_id == $packet.source_surface_id)) as $approval
      | ($preview.readback_boundaries[] | select(.source_surface_id == $packet.source_surface_id)) as $boundary
      | {
          id: readback_plan_id($packet.source_surface_id),
          source_surface_id: $packet.source_surface_id,
          source_category: $packet.source_category,
          operator_review_packet_id: $packet.packet_id,
          side_effect_lock_plan_id: $lock.lock_plan_id,
          approval_evidence_boundary_id: $approval.boundary_id,
          readback_boundary_id: $boundary.boundary_id,
          required_evidence_fields: $packet.evidence_field_ids,
          lock_scope_ids: $lock.lock_scope_ids,
          readback_state: "asserted_from_operator_review_preview_no_execution",
          required_before_application: true,
          performs_readback: false,
          records_operator_review: false,
          records_approval: false,
          establishes_side_effect_lock: false,
          mutates_store: false,
          writes_wal: false
        })) as $readback_plans
  | ($preview.operator_review_packets | map({
      id: ("operator_review_packet_readback_assertion__" + .source_surface_id),
      source_surface_id: .source_surface_id,
      packet_id: .packet_id,
      expected_packet_state: "readback_verified_no_mutation",
      required_section_ids: .required_section_ids,
      required_evidence_field_ids: .evidence_field_ids,
      records_operator_review: false,
      records_approval: false,
      applies_to_runtime: false
    })) as $packet_assertions
  | ($preview.side_effect_lock_plans | map({
      id: ("side_effect_lock_readback_assertion__" + .source_surface_id),
      source_surface_id: .source_surface_id,
      lock_plan_id: .lock_plan_id,
      expected_lock_state: "readback_verified_not_established",
      lock_scope_ids: .lock_scope_ids,
      prevents_runtime_mutation: true,
      side_effects_allowed: false,
      lock_established: false
    })) as $lock_assertions
  | ($preview.approval_evidence_boundaries | map({
      id: ("approval_evidence_boundary_readback_assertion__" + .source_surface_id),
      source_surface_id: .source_surface_id,
      boundary_id: .boundary_id,
      expected_boundary_state: "readback_verified_not_recorded",
      required_evidence_field_ids: .required_evidence_field_ids,
      records_operator_review: false,
      records_approval: false,
      persists_receipt: false
    })) as $approval_assertions
  | ($preview.readback_boundaries | map({
      id: ("operator_review_readback_boundary_assertion__" + .source_surface_id),
      source_surface_id: .source_surface_id,
      boundary_id: .boundary_id,
      readback_probe_id: .readback_probe_id,
      expected_readback_state: "readback_contract_declared_not_executed",
      performs_readback: false,
      rollback_executed: false,
      writes_checkpoint: false
    })) as $boundary_assertions
  | ($readback_plans | map({
      id: ("operator_review_evidence_field_readback_assertion__" + .source_surface_id),
      source_surface_id: .source_surface_id,
      required_evidence_fields: .required_evidence_fields,
      required_field_count: (.required_evidence_fields | length),
      expected_evidence_state: "evidence_fields_declared_not_persisted",
      performs_readback: false,
      persists_evidence: false
    })) as $evidence_assertions
  | ($preview.operator_review_groups | map({
      id: ("operator_review_group_readback_assertion__" + .id),
      group_id: .id,
      source_category: .source_category,
      affected_source_surface_ids: .affected_source_surface_ids,
      operator_review_packet_ids: .operator_review_packet_ids,
      side_effect_lock_plan_ids: .side_effect_lock_plan_ids,
      expected_review_packet_count: .expected_review_packet_count,
      expected_group_state: "readback_verified_no_mutation"
    })) as $group_assertions
  | ($preview.guards | map({
      id: ("operator_review_guard_readback_assertion__" + .id),
      guard_id: .id,
      severity: .severity,
      guard_scope: .scope,
      expected_guard_state: "guard_declared_and_runtime_mutation_prevented",
      prevents_runtime_mutation: true,
      mutates_runtime: false
    })) as $guard_assertions
  | ($preview.blockers + [{
      id: "operator_review_side_effect_lock_application_missing",
      severity: "high",
      affected_source_surface_ids: ($readback_plans | map(.source_surface_id)),
      blocks_operator_review: true,
      blocks_side_effect_lock: true,
      blocks_runtime_write_boundary: false,
      recommended_fix: "apply readback-verified operator review packets and side-effect lock plans before readiness rerun"
    }]) as $blockers
  | ($blockers | map({
      id: ("operator_review_blocker_mapping_readback_assertion__" + .id),
      blocker_id: .id,
      severity: .severity,
      affected_source_surface_ids: .affected_source_surface_ids,
      affected_readback_plan_ids: affected_readback_plan_ids(.affected_source_surface_ids; $readback_plans),
      expected_blocker_state: "blocker_mapping_readback_verified_no_mutation",
      blocks_operator_review: .blocks_operator_review,
      blocks_side_effect_lock: .blocks_side_effect_lock,
      blocks_runtime_write_boundary: .blocks_runtime_write_boundary,
      performs_readback: false,
      mutates_runtime: false
    })) as $blocker_assertions
  | ([
      drift_detector("operator_review_packet_alignment"; ["packet_id", "required_section_ids"]),
      drift_detector("side_effect_lock_scope_alignment"; ["lock_plan_id", "lock_scope_ids"]),
      drift_detector("approval_evidence_boundary_alignment"; ["boundary_id", "required_evidence_field_ids"]),
      drift_detector("readback_boundary_alignment"; ["readback_probe_id", "readback_state"]),
      drift_detector("guard_no_mutation_alignment"; ["guard_id", "mutates_runtime"]),
      drift_detector("blocker_mapping_alignment"; ["blocker_id", "affected_readback_plan_ids"]),
      drift_detector("side_effect_boundary_alignment"; ["side_effects", "operator_review_recorded"])
    ]) as $drift_detectors
  | ($preview.required_prior_gates + [$preview.gate]) as $required_prior_gates
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_append_only_store_operator_review_side_effect_lock_readback_preview_gate",
      schema_version: "work_graph_append_only_store_operator_review_side_effect_lock_readback_preview_v1",
      preview_mode: "read_only_append_only_store_operator_review_side_effect_lock_readback_no_execution",
      operator_review_packet_count: ($preview.operator_review_packets | length),
      side_effect_lock_plan_count: ($preview.side_effect_lock_plans | length),
      approval_evidence_boundary_count: ($preview.approval_evidence_boundaries | length),
      readback_boundary_count: ($preview.readback_boundaries | length),
      readback_plan_count: ($readback_plans | length),
      packet_assertion_count: ($packet_assertions | length),
      side_effect_lock_assertion_count: ($lock_assertions | length),
      approval_boundary_assertion_count: ($approval_assertions | length),
      readback_boundary_assertion_count: ($boundary_assertions | length),
      evidence_field_assertion_count: ($evidence_assertions | length),
      guard_assertion_count: ($guard_assertions | length),
      blocker_mapping_assertion_count: ($blocker_assertions | length),
      evidence_field_ref_count: ($evidence_assertions | map(.required_field_count) | add),
      lock_scope_ref_count: ($readback_plans | map(.lock_scope_ids | length) | add),
      group_source_ref_count: ($preview.operator_review_groups | map(.affected_source_surface_ids | length) | add),
      drift_detector_count: ($drift_detectors | length),
      blocker_count: ($blockers | length),
      required_prior_gate_count: ($required_prior_gates | length),
      readback_plans: $readback_plans,
      packet_assertions: $packet_assertions,
      side_effect_lock_assertions: $lock_assertions,
      approval_boundary_assertions: $approval_assertions,
      readback_boundary_assertions: $boundary_assertions,
      evidence_field_assertions: $evidence_assertions,
      group_assertions: $group_assertions,
      guard_assertions: $guard_assertions,
      blocker_mapping_assertions: $blocker_assertions,
      drift_detectors: $drift_detectors,
      blockers: $blockers,
      required_prior_gates: $required_prior_gates,
      recommended_next_gate: "hepta_work_graph_append_only_store_operator_review_side_effect_lock_application_preview_gate",
      ready_for_operator_review_side_effect_lock_application_preview: true,
      ready_for_readback_execution: false,
      ready_for_operator_review_recording: false,
      ready_for_side_effect_lock_establishment: false,
      ready_for_runtime_write_boundary_preview: false,
      ready_for_append_only_store_enablement: false,
      ready_for_projection_enforcement: false,
      ready_for_live_execution: false,
      source_probes: {
        operator_review_side_effect_lock_readback: {
          rust_module_present: $readback_rust_module_present,
          report_script_present: $readback_report_script_present,
          gate_script_present: $readback_gate_script_present
        },
        operator_review_side_effect_lock_preview: {
          upstream_gate: ($preview.gate == "hepta_work_graph_append_only_store_operator_review_side_effect_lock_preview_gate"),
          gate_script_present: $preview_gate_script_present,
          recommended_next_matches: ($preview.recommended_next_gate == "hepta_work_graph_append_only_store_operator_review_side_effect_lock_readback_preview_gate")
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
        role_manifest_enforced: false,
        task_result_enforcement_enabled: false,
        task_result_persisted: false,
        approval_recorded: false,
        operator_review_recorded: false,
        side_effect_lock_established: false,
        readback_executed: false,
        rollback_executed: false,
        runtime_mutation_performed: false,
        external_send_performed: false,
        model_invoked: false,
        agent_spawn_performed: false
      }
    }
  '
