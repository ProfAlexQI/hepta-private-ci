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

promotion_blocker_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_promotion_blocker_preview.rs
)"
promotion_blocker_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-promotion-blocker-preview-report.sh
)"
promotion_blocker_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-promotion-blocker-preview-gate.sh
)"
canary_receipt_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_canary_readback_receipt_preview.rs
)"
canary_receipt_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-canary-readback-receipt-preview-gate.sh
)"
durable_identity_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_durable_identity_preview.rs
)"
durable_identity_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-durable-identity-preview-report.sh
)"
durable_identity_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-durable-identity-preview-gate.sh
)"

durable_identity_report="$(
  capture_json_report \
    "hepta-work-graph-durable-identity-preview-report" \
    "$ROOT/scripts/hepta-systems-work-graph-durable-identity-preview-report.sh"
)"

jq -n \
  --argjson promotion_blocker_rust_module_present "$promotion_blocker_rust_module_present" \
  --argjson promotion_blocker_report_script_present "$promotion_blocker_report_script_present" \
  --argjson promotion_blocker_gate_script_present "$promotion_blocker_gate_script_present" \
  --argjson canary_receipt_rust_module_present "$canary_receipt_rust_module_present" \
  --argjson canary_receipt_gate_script_present "$canary_receipt_gate_script_present" \
  --argjson durable_identity_rust_module_present "$durable_identity_rust_module_present" \
  --argjson durable_identity_report_script_present "$durable_identity_report_script_present" \
  --argjson durable_identity_gate_script_present "$durable_identity_gate_script_present" \
  --argjson durable_identity_report "$durable_identity_report" \
  '
  def durable_fields: [
    "workflow_id",
    "run_id",
    "step_id",
    "checkpoint",
    "replay_key",
    "rollback_anchor",
    "receipt_hash"
  ];
  def prior_gates: [
    "hepta_work_graph_contract_preview_gate",
    "hepta_work_graph_task_result_contract_preview_gate",
    "hepta_work_graph_scheduler_admission_controller_preview_gate",
    "hepta_work_graph_observability_timeline_preview_gate",
    "hepta_work_graph_role_manifest_contract_preview_gate",
    "hepta_work_graph_unified_state_store_preview_gate",
    "hepta_work_graph_adapter_projection_fixture_gate",
    "hepta_work_graph_state_store_persistence_preview_gate",
    "hepta_work_graph_replay_readback_preview_gate",
    "hepta_work_graph_promotion_precondition_preview_gate",
    "hepta_work_graph_activation_enforcement_blocker_preview_gate",
    "hepta_work_graph_shadow_adapter_readback_preview_gate",
    "hepta_work_graph_persistence_feature_flag_preview_gate",
    "hepta_work_graph_persistence_canary_dry_run_preview_gate",
    "hepta_work_graph_persistence_canary_readback_receipt_preview_gate",
    "hepta_work_graph_durable_identity_preview_gate"
  ];
  def targets: [
    "store_persistence_promotion",
    "wal_append_promotion",
    "checkpoint_write_promotion",
    "readback_receipt_persistence_promotion",
    "idempotency_index_promotion",
    "replay_execution_promotion",
    "external_release_publication_promotion"
  ];
  def persistent_targets: [
    "store_persistence_promotion",
    "wal_append_promotion",
    "checkpoint_write_promotion",
    "readback_receipt_persistence_promotion",
    "idempotency_index_promotion",
    "replay_execution_promotion"
  ];
  def blocker($id; $targets; $trigger; $fields): {
    id: $id,
    applies_to_target_ids: $targets,
    trigger: $trigger,
    required_evidence_fields: (durable_fields + $fields),
    blocks_persistence_promotion: true,
    blocks_live_execution: true
  };
  def durable_blocker($id; $targets; $trigger): {
    id: $id,
    applies_to_target_ids: $targets,
    trigger: $trigger,
    required_evidence_fields: durable_fields,
    blocks_persistence_promotion: true,
    blocks_live_execution: true
  };
  def denial($id; $channel; $reason; $blockers): {
    id: $id,
    target_channel: $channel,
    reason: $reason,
    applies_to_blocker_ids: $blockers,
    blocks_release: true
  };
  def acknowledgement($id; $audience; $fields): {
    id: $id,
    audience: $audience,
    required_fields: (durable_fields + $fields),
    currently_satisfied: false,
    external_delivery_enabled: false,
    approval_recorded: false
  };
  def quarantine($id; $trigger; $scope; $kill): {
    id: $id,
    trigger_blocker_id: $trigger,
    quarantine_scope: $scope,
    kill_switch_id: $kill,
    armed_in_preview: true
  };
  def invariant($id; $reason): {
    id: $id,
    required: true,
    reason: $reason
  };
  [
    blocker("missing_canary_receipt_digest"; targets; "canary readback receipt digest is missing or not hash-only"; ["priorGateReportHash", "receiptHash", "redactionState"]),
    durable_blocker("durable_identity_evidence_missing"; targets; "promotion target is missing workflow, run, step, checkpoint, replay, rollback, or receipt identity"),
    blocker("operator_acknowledgement_missing"; targets; "operator acknowledgement packet has not been reviewed"; ["operatorScopeHash", "acknowledgementHash", "expiresAtUnixMs"]),
    blocker("release_denial_matrix_missing"; targets; "release denial matrix has not been materialized for the target"; ["denialMatrixHash", "releaseScope", "targetChannel"]),
    blocker("rollback_quarantine_not_armed"; persistent_targets; "rollback and quarantine switches are not armed for persistence promotion"; ["rollbackPlanId", "killSwitchId", "quarantineScope"]),
    blocker("zero_write_or_traffic_receipt_absent"; persistent_targets; "receipt does not prove zero live traffic and zero persisted writes"; ["zeroTrafficProof", "zeroWriteProof", "sideEffectHash"]),
    blocker("canary_scope_exceeds_backend_lane"; targets; "canary evidence is not scoped to the hepta-backend lane"; ["laneId", "agentId", "cargoTargetDirHash"]),
    blocker("external_delivery_policy_missing"; ["external_release_publication_promotion"]; "external delivery policy and readback gate are absent"; ["deliveryPolicyHash", "externalTargetScope", "readbackGate"])
  ] as $promotion_blockers
  | [
    denial("deny_store_persistence_release"; "durable_work_graph_store"; "store persistence cannot release while canary receipt, operator acknowledgement, or rollback evidence is missing"; ["missing_canary_receipt_digest", "durable_identity_evidence_missing", "operator_acknowledgement_missing", "rollback_quarantine_not_armed", "zero_write_or_traffic_receipt_absent"]),
    denial("deny_wal_append_release"; "work_graph_wal"; "WAL append cannot release without hash-only receipt and zero-write proof"; ["missing_canary_receipt_digest", "durable_identity_evidence_missing", "rollback_quarantine_not_armed", "zero_write_or_traffic_receipt_absent"]),
    denial("deny_checkpoint_release"; "work_graph_checkpoint"; "checkpoint persistence cannot release without release denial matrix and quarantine evidence"; ["durable_identity_evidence_missing", "release_denial_matrix_missing", "rollback_quarantine_not_armed", "zero_write_or_traffic_receipt_absent"]),
    denial("deny_readback_receipt_release"; "readback_receipt_store"; "readback receipt persistence cannot release while receipts remain preview-only"; ["missing_canary_receipt_digest", "durable_identity_evidence_missing", "operator_acknowledgement_missing", "release_denial_matrix_missing"]),
    denial("deny_replay_execution_release"; "work_graph_replay_executor"; "replay execution cannot release without lane-bound canary scope and rollback switches"; ["durable_identity_evidence_missing", "canary_scope_exceeds_backend_lane", "rollback_quarantine_not_armed", "zero_write_or_traffic_receipt_absent"]),
    denial("deny_external_publication_release"; "external_delivery"; "external publication cannot release without a separate delivery policy and readback gate"; ["external_delivery_policy_missing", "durable_identity_evidence_missing", "operator_acknowledgement_missing", "release_denial_matrix_missing"])
  ] as $release_denials
  | [
    acknowledgement("operator_promotion_blocker_ack"; "operator"; ["denialReasonIds", "targetIds", "receiptHash", "nextGate"]),
    acknowledgement("auditor_digest_ack"; "auditor"; ["priorGateReportHash", "canaryReceiptHash", "releaseDenialHash", "redactionState"]),
    acknowledgement("release_owner_non_acceptance_ack"; "release_owner"; ["targetChannel", "denialMatrixHash", "nonAcceptanceReason", "expiresAtUnixMs"]),
    acknowledgement("rollback_owner_quarantine_ack"; "rollback_owner"; ["rollbackPlanId", "killSwitchId", "quarantineScope", "recoveryOwnerHash"]),
    acknowledgement("external_delivery_scope_ack"; "delivery_owner"; ["deliveryPolicyHash", "externalTargetScope", "readbackGate", "externalDeliveryDisabled"])
  ] as $operator_acknowledgements
  | [
    quarantine("quarantine_store_persistence_on_missing_receipt"; "missing_canary_receipt_digest"; "graph_state_store"; "kill_work_graph_store_persistence"),
    quarantine("quarantine_wal_append_on_zero_write_failure"; "zero_write_or_traffic_receipt_absent"; "wal_writer"; "kill_work_graph_wal_append"),
    quarantine("quarantine_replay_execution_on_lane_scope_failure"; "canary_scope_exceeds_backend_lane"; "replay_executor"; "kill_work_graph_replay_execution"),
    quarantine("quarantine_release_publication_on_policy_gap"; "external_delivery_policy_missing"; "external_release_pipeline"; "kill_work_graph_external_delivery"),
    quarantine("quarantine_promotion_on_operator_gap"; "operator_acknowledgement_missing"; "promotion_executor"; "kill_work_graph_promotion_executor"),
    quarantine("quarantine_promotion_on_durable_identity_gap"; "durable_identity_evidence_missing"; "promotion_executor"; "kill_work_graph_promotion_identity")
  ] as $rollback_quarantines
  | [
    invariant("promotion_blockers_require_durable_identity_evidence"; "promotion blockers require workflow, run, step, checkpoint, replay, rollback, and receipt evidence"),
    invariant("promotion_blocked_after_canary_until_acknowledged"; "canary readback receipts are evidence only and cannot promote state without operator acknowledgement"),
    invariant("release_denials_are_target_specific"; "each persistence release target has an explicit denial path before any future release"),
    invariant("operator_acknowledgement_is_non_recording"; "operator acknowledgement previews are local and do not write approval receipts"),
    invariant("rollback_quarantine_precedes_promotion_execution"; "promotion execution must be blocked until rollback and quarantine switches are armed"),
    invariant("external_release_has_independent_denial"; "external delivery and publication cannot inherit persistence or scheduler promotion"),
    invariant("persistence_promotion_blocker_preview_has_no_side_effects"; "this gate cannot promote, release, persist receipts, record approvals, quarantine state, or send externally")
  ] as $invariants
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_persistence_promotion_blocker_preview_gate",
      schema_version: "work_graph_persistence_promotion_blocker_preview_v1",
      preview_mode: "read_only_persistence_promotion_blocker_preview_no_promotion",
      promotion_blocker_count: ($promotion_blockers | length),
      release_denial_count: ($release_denials | length),
      operator_acknowledgement_count: ($operator_acknowledgements | length),
      rollback_quarantine_count: ($rollback_quarantines | length),
      invariant_count: ($invariants | length),
      required_prior_gates: prior_gates,
      promotion_blockers: $promotion_blockers,
      release_denials: $release_denials,
      operator_acknowledgements: $operator_acknowledgements,
      rollback_quarantines: $rollback_quarantines,
      durable_identity_evidence: {
        schema_version: $durable_identity_report.schema_version,
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: durable_fields,
        required_for_promotion_target_ids: targets,
        durable_field_count: $durable_identity_report.durable_field_count,
        preview_binding_count: $durable_identity_report.preview_binding_count,
        invariant_count: $durable_identity_report.invariant_count,
        currently_satisfied: false
      },
      invariants: $invariants,
      recommended_next_gate: "hepta_work_graph_persistence_shadow_live_readback_comparison_preview_gate",
      ready_for_shadow_live_readback_comparison_preview: true,
      ready_for_persistence_promotion: false,
      ready_for_release_publication: false,
      ready_for_live_persistence: false,
      source_probes: {
        persistence_promotion_blocker: {
          rust_module_present: $promotion_blocker_rust_module_present,
          report_script_present: $promotion_blocker_report_script_present,
          gate_script_present: $promotion_blocker_gate_script_present
        },
        persistence_canary_readback_receipt: {
          rust_module_present: $canary_receipt_rust_module_present,
          gate_script_present: $canary_receipt_gate_script_present
        },
        durable_identity: {
          rust_module_present: $durable_identity_rust_module_present,
          report_script_present: $durable_identity_report_script_present,
          gate_script_present: $durable_identity_gate_script_present
        }
      },
      side_effects: {
        filesystem_written: false,
        graph_state_persisted: false,
        receipt_persisted: false,
        promotion_performed: false,
        release_published: false,
        operator_acknowledgement_recorded: false,
        rollback_performed: false,
        quarantine_performed: false,
        feature_flag_mutated: false,
        live_traffic_routed: false,
        wal_written: false,
        checkpoint_written: false,
        scheduler_cutover_performed: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
