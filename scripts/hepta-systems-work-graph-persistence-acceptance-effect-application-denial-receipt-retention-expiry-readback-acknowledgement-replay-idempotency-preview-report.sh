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

replay_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview.rs
)"
replay_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-replay-idempotency-preview-report.sh
)"
replay_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-replay-idempotency-preview-gate.sh
)"
ack_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview.rs
)"
ack_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-preview-gate.sh
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
  --argjson replay_rust_module_present "$replay_rust_module_present" \
  --argjson replay_report_script_present "$replay_report_script_present" \
  --argjson replay_gate_script_present "$replay_gate_script_present" \
  --argjson ack_rust_module_present "$ack_rust_module_present" \
  --argjson ack_gate_script_present "$ack_gate_script_present" \
  --argjson durable_identity_rust_module_present "$durable_identity_rust_module_present" \
  --argjson durable_identity_report_script_present "$durable_identity_report_script_present" \
  --argjson durable_identity_gate_script_present "$durable_identity_gate_script_present" \
  --argjson durable_identity_report "$durable_identity_report" \
  '
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
    "hepta_work_graph_persistence_promotion_blocker_preview_gate",
    "hepta_work_graph_persistence_shadow_live_readback_comparison_preview_gate",
    "hepta_work_graph_persistence_enforcement_rollout_blocker_preview_gate",
    "hepta_work_graph_persistence_operator_readiness_packet_preview_gate",
    "hepta_work_graph_persistence_operator_readiness_receipt_preview_gate",
    "hepta_work_graph_persistence_operator_readiness_receipt_acknowledgement_preview_gate",
    "hepta_work_graph_persistence_acceptance_authority_blocker_preview_gate",
    "hepta_work_graph_persistence_acceptance_record_intake_preview_gate",
    "hepta_work_graph_persistence_acceptance_record_receipt_preview_gate",
    "hepta_work_graph_persistence_acceptance_record_receipt_acknowledgement_preview_gate",
    "hepta_work_graph_persistence_acceptance_effect_application_blocker_preview_gate",
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_preview_gate",
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_acknowledgement_preview_gate",
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_replay_idempotency_preview_gate",
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_preview_gate",
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_receipt_preview_gate",
    "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_preview_gate",
    "hepta_work_graph_durable_identity_preview_gate"
  ];
  def durable_fields: [
    "workflow_id",
    "run_id",
    "step_id",
    "checkpoint",
    "replay_key",
    "rollback_anchor",
    "receipt_hash"
  ];
  def ack_ids: [
    "retention_policy_readback_receipt_acknowledgement",
    "expiry_guard_readback_receipt_acknowledgement",
    "supersession_guard_readback_receipt_acknowledgement",
    "garbage_collection_denial_readback_receipt_acknowledgement",
    "zero_effect_digest_readback_receipt_acknowledgement",
    "release_external_denial_readback_receipt_acknowledgement"
  ];
  def scenario_ids: [
    "duplicate_retention_readback_receipt_replay",
    "duplicate_retention_readback_acknowledgement_replay",
    "stale_retention_readback_digest_replay",
    "superseded_retention_scope_acknowledgement_replay",
    "cross_scope_retention_readback_acknowledgement_replay",
    "out_of_order_retention_readback_acknowledgement_replay"
  ];
  def scenario($id; $mode): {
    id: $id,
    source_acknowledgement_ids: ack_ids,
    replay_mode: $mode,
    required_fields: (durable_fields + ["replayScenarioId", "sourceAcknowledgementIds", "replayMode", "zeroMutationProofHash"]),
    acknowledgement_recording_allowed: false,
    mutation_allowed: false
  };
  def guard($id; $fields): {
    id: $id,
    required_fields: $fields,
    blocks_replay_mutation: true
  };
  def denial($id; $reason): {
    id: $id,
    applies_to_replay_scenario_ids: scenario_ids,
    reason: $reason,
    blocks_acknowledgement_recording: true,
    blocks_acceptance: true,
    blocks_mutation: true
  };
  def check($id; $fields): {
    id: $id,
    compared_fields: $fields,
    blocks_out_of_order_replay: true
  };
  def view($id; $audience; $fields): {
    id: $id,
    audience: $audience,
    required_fields: $fields,
    external_delivery_enabled: false
  };
  def invariant($id; $reason): {
    id: $id,
    required: true,
    reason: $reason
  };
  [
    scenario("duplicate_retention_readback_receipt_replay"; "duplicate_readback_receipt"),
    scenario("duplicate_retention_readback_acknowledgement_replay"; "duplicate_acknowledgement"),
    scenario("stale_retention_readback_digest_replay"; "stale_readback_digest"),
    scenario("superseded_retention_scope_acknowledgement_replay"; "superseded_retention_scope"),
    scenario("cross_scope_retention_readback_acknowledgement_replay"; "cross_scope_acknowledgement"),
    scenario("out_of_order_retention_readback_acknowledgement_replay"; "out_of_order_acknowledgement")
  ] as $replay_scenarios
  | [
    guard("retention_readback_receipt_idempotency_key_required"; durable_fields + ["readbackReceiptId", "retentionScope", "readbackReceiptHash"]),
    guard("retention_readback_acknowledgement_idempotency_key_required"; durable_fields + ["acknowledgementId", "acknowledgementHash", "localViewHash"]),
    guard("retention_readback_prior_gate_digest_binding_required"; durable_fields + ["priorGateId", "priorGateDigest", "sourceReportHash"]),
    guard("retention_scope_epoch_binding_required"; durable_fields + ["retentionScopeId", "scopeEpoch", "supersessionHash"]),
    guard("retention_zero_side_effect_digest_binding_required"; durable_fields + ["zeroWriteHash", "zeroTrafficHash", "zeroExternalSendHash"]),
    guard("retention_readback_acknowledgement_sequence_required"; durable_fields + ["readbackSequence", "acknowledgementSequence", "sequenceHash"]),
    guard("replay_does_not_unlock_retention_or_acceptance"; durable_fields + ["retentionStatePersisted", "acceptanceAllowed", "authorityGranted"])
  ] as $idempotency_guards
  | [
    denial("durable_identity_evidence_missing"; "retention readback acknowledgement replay cannot proceed without durable identity evidence"),
    denial("duplicate_readback_cannot_record_acknowledgement"; "duplicate readback receipt replay cannot record acknowledgement"),
    denial("duplicate_acknowledgement_cannot_record_acceptance"; "duplicate acknowledgement replay cannot record acceptance"),
    denial("stale_readback_digest_cannot_grant_authority"; "stale readback digest replay cannot grant authority"),
    denial("cross_scope_replay_cannot_enable_live_persistence"; "cross-scope retention replay cannot enable live persistence, WAL, or checkpoints"),
    denial("out_of_order_replay_cannot_start_rollout"; "out-of-order retention replay cannot start rollout or route traffic"),
    denial("superseded_replay_cannot_publish_release"; "superseded retention replay cannot publish release status"),
    denial("replayed_retention_acknowledgement_cannot_send_external_delivery"; "replayed retention acknowledgement cannot send external delivery")
  ] as $replay_denials
  | [
    check("retention_readback_receipt_sequence_check"; durable_fields + ["readbackReceiptId", "readbackSequence", "readbackReceiptHash"]),
    check("retention_readback_acknowledgement_sequence_check"; durable_fields + ["acknowledgementId", "acknowledgementSequence", "acknowledgementHash"]),
    check("retention_prior_gate_digest_monotonicity_check"; durable_fields + ["priorGateId", "priorGateDigest", "observedAt"]),
    check("retention_scope_epoch_monotonicity_check"; durable_fields + ["retentionScopeId", "scopeEpoch", "supersessionHash"]),
    check("retention_zero_effect_digest_stability_check"; durable_fields + ["zeroWriteHash", "zeroTrafficHash", "zeroExternalSendHash"])
  ] as $monotonicity_checks
  | [
    view("operator_retention_readback_ack_replay_idempotency_view"; "operator"; durable_fields + ["replayScenarioId", "idempotencyKey", "acknowledgementRecordingAllowed", "nextGate"]),
    view("auditor_retention_readback_ack_replay_digest_view"; "auditor"; durable_fields + ["readbackReceiptHash", "acknowledgementHash", "priorGateDigest", "monotonicityCheckId"]),
    view("release_owner_retention_readback_ack_replay_denial_view"; "release_owner"; durable_fields + ["releaseDenied", "publicationDenied", "externalDeliveryDenied", "replayDenialId"]),
    view("runtime_retention_readback_ack_replay_zero_effect_view"; "system"; durable_fields + ["replayRecorded", "acknowledgementRecorded", "retentionStatePersisted", "authorityGranted", "trafficRouted", "externalSendPerformed"])
  ] as $local_views
  | [
    invariant("retention_readback_ack_replay_requires_durable_identity_evidence"; "retention readback acknowledgement replay requires workflow, run, step, checkpoint, replay, rollback, and receipt evidence"),
    invariant("retention_readback_ack_replay_is_idempotent"; "duplicate readback, duplicate acknowledgement, and stale digest replay cannot change state"),
    invariant("retention_readback_ack_replay_keeps_zero_side_effects"; "replay must preserve zero writes, zero traffic, zero release, and zero external sends"),
    invariant("retention_readback_ack_replay_requires_acknowledgement_gate"; "replay idempotency requires the retention readback acknowledgement gate"),
    invariant("retention_readback_ack_replay_is_scope_bound"; "cross-scope and superseded acknowledgement replay cannot unlock effect application"),
    invariant("retention_readback_ack_replay_views_are_local_only"; "operator, auditor, release-owner, and runtime views cannot be sent externally"),
    invariant("retention_readback_ack_replay_preview_has_no_side_effects"; "this gate cannot persist replay records, record acknowledgement, grant authority, publish, or send externally")
  ] as $invariants
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate",
      schema_version: "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_v1",
      preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_no_replay_write",
      replay_scenario_count: ($replay_scenarios | length),
      idempotency_guard_count: ($idempotency_guards | length),
      replay_denial_count: ($replay_denials | length),
      monotonicity_check_count: ($monotonicity_checks | length),
      local_view_count: ($local_views | length),
      invariant_count: ($invariants | length),
      required_prior_gates: prior_gates,
      replay_scenarios: $replay_scenarios,
      idempotency_guards: $idempotency_guards,
      replay_denials: $replay_denials,
      monotonicity_checks: $monotonicity_checks,
      local_views: $local_views,
      durable_identity_evidence: {
        schema_version: $durable_identity_report.schema_version,
        required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
        required_field_ids: durable_fields,
        required_for_replay_scenario_ids: scenario_ids,
        durable_field_count: (durable_fields | length),
        preview_binding_count: 5,
        invariant_count: ($invariants | length),
        currently_satisfied: false
      },
      invariants: $invariants,
      recommended_next_gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate",
      ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview: true,
      ready_for_operator_acceptance: false,
      ready_for_live_persistence: false,
      source_probes: {
        persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_replay_idempotency: {
          rust_module_present: $replay_rust_module_present,
          report_script_present: $replay_report_script_present,
          gate_script_present: $replay_gate_script_present
        },
        persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement: {
          rust_module_present: $ack_rust_module_present,
          gate_script_present: $ack_gate_script_present
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
        retention_state_persisted: false,
        readback_receipt_persisted: false,
        readback_acknowledgement_recorded: false,
        replay_recorded: false,
        operator_acceptance_recorded: false,
        approval_recorded: false,
        authority_granted: false,
        live_persistence_enabled: false,
        wal_written: false,
        checkpoint_written: false,
        enforcement_enabled: false,
        rollout_started: false,
        traffic_routed: false,
        release_published: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
