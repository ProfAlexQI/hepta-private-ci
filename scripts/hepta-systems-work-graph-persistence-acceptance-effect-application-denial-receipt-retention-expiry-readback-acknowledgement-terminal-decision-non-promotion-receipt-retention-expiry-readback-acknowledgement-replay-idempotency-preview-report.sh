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
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview.rs
)"
replay_report_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-acknowledgement-replay-idempotency-preview-report.sh
)"
replay_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-acknowledgement-replay-idempotency-preview-gate.sh
)"
ack_rust_module_present="$(
  bool_for path_exists codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview.rs
)"
ack_gate_script_present="$(
  bool_for path_exists scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-acknowledgement-preview-gate.sh
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

prior_report_script="$ROOT/scripts/hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-retention-expiry-readback-acknowledgement-preview-report.sh"
required_prior_gates="$(
  "$prior_report_script" |
    jq -c '.required_prior_gates
      | map(select(. != "hepta_work_graph_durable_identity_preview_gate"))
      + [
          "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate",
          "hepta_work_graph_durable_identity_preview_gate"
        ]'
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
  --argjson required_prior_gates "$required_prior_gates" \
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
  def ack_ids: [
    "terminal_receipt_retention_policy_readback_acknowledgement",
    "terminal_receipt_expiry_guard_readback_acknowledgement",
    "terminal_receipt_supersession_guard_readback_acknowledgement",
    "terminal_receipt_gc_denial_readback_acknowledgement",
    "terminal_receipt_zero_effect_digest_readback_acknowledgement",
    "terminal_receipt_release_public_claim_denial_readback_acknowledgement"
  ];
  def scenario_ids: [
    "duplicate_terminal_receipt_retention_readback_receipt_replay",
    "duplicate_terminal_receipt_retention_readback_acknowledgement_replay",
    "stale_terminal_receipt_retention_readback_digest_replay",
    "superseded_terminal_receipt_retention_scope_acknowledgement_replay",
    "cross_scope_terminal_receipt_retention_acknowledgement_replay",
    "out_of_order_terminal_receipt_retention_acknowledgement_replay"
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
    blocks_authority: true,
    blocks_rollout: true,
    blocks_release_publication: true,
    blocks_public_claim: true,
    blocks_external_delivery: true
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
    scenario("duplicate_terminal_receipt_retention_readback_receipt_replay"; "duplicate_readback_receipt"),
    scenario("duplicate_terminal_receipt_retention_readback_acknowledgement_replay"; "duplicate_acknowledgement"),
    scenario("stale_terminal_receipt_retention_readback_digest_replay"; "stale_readback_digest"),
    scenario("superseded_terminal_receipt_retention_scope_acknowledgement_replay"; "superseded_retention_scope"),
    scenario("cross_scope_terminal_receipt_retention_acknowledgement_replay"; "cross_scope_acknowledgement"),
    scenario("out_of_order_terminal_receipt_retention_acknowledgement_replay"; "out_of_order_acknowledgement")
  ] as $replay_scenarios
  | [
    guard("terminal_retention_readback_receipt_idempotency_key_required"; durable_fields + ["readbackReceiptId", "retentionScope", "readbackReceiptHash"]),
    guard("terminal_retention_readback_acknowledgement_idempotency_key_required"; durable_fields + ["acknowledgementId", "acknowledgementHash", "readbackReceiptHash"]),
    guard("terminal_retention_prior_gate_digest_binding_required"; durable_fields + ["priorGateId", "priorGateDigest", "sourceReportHash"]),
    guard("terminal_retention_scope_epoch_binding_required"; durable_fields + ["retentionScope", "scopeEpoch", "supersessionHash"]),
    guard("terminal_retention_zero_side_effect_digest_binding_required"; durable_fields + ["zeroWriteHash", "zeroTrafficHash", "zeroExternalSendHash"]),
    guard("terminal_retention_acknowledgement_sequence_required"; durable_fields + ["readbackSequence", "acknowledgementSequence", "sequenceHash"]),
    guard("terminal_retention_replay_keeps_non_promotion_denied"; durable_fields + ["acknowledgementRecorded", "acceptanceAllowed", "authorityGranted"])
  ] as $idempotency_guards
  | [
    denial("durable_identity_evidence_missing"; "terminal receipt retention readback acknowledgement replay cannot proceed without durable identity evidence"),
    denial("duplicate_terminal_retention_readback_receipt_cannot_record_acknowledgement"; "duplicate terminal retention readback receipt replay cannot record acknowledgement"),
    denial("duplicate_terminal_retention_acknowledgement_cannot_record_acceptance"; "duplicate terminal retention acknowledgement replay cannot record acceptance"),
    denial("stale_terminal_retention_digest_cannot_grant_authority"; "stale terminal retention digest replay cannot grant authority"),
    denial("cross_scope_terminal_retention_replay_cannot_enable_live_persistence"; "cross-scope terminal retention replay cannot enable live persistence, WAL, or checkpoints"),
    denial("out_of_order_terminal_retention_replay_cannot_start_rollout"; "out-of-order terminal retention replay cannot start rollout or route traffic"),
    denial("superseded_terminal_retention_replay_cannot_publish_or_claim"; "superseded terminal retention replay cannot publish release status or record public claims"),
    denial("replayed_terminal_retention_acknowledgement_cannot_send_external_delivery"; "replayed terminal retention acknowledgement cannot send external delivery")
  ] as $replay_denials
  | [
    check("terminal_retention_readback_receipt_sequence_check"; durable_fields + ["readbackReceiptId", "readbackSequence", "readbackReceiptHash"]),
    check("terminal_retention_acknowledgement_sequence_check"; durable_fields + ["acknowledgementId", "acknowledgementSequence", "acknowledgementHash"]),
    check("terminal_retention_prior_gate_digest_monotonicity_check"; durable_fields + ["priorGateId", "priorGateDigest", "observedAt"]),
    check("terminal_retention_scope_epoch_monotonicity_check"; durable_fields + ["retentionScope", "scopeEpoch", "supersessionHash"]),
    check("terminal_retention_zero_effect_digest_stability_check"; durable_fields + ["zeroWriteHash", "zeroTrafficHash", "zeroExternalSendHash"])
  ] as $monotonicity_checks
  | [
    view("operator_terminal_retention_readback_ack_replay_idempotency_view"; "operator"; durable_fields + ["replayScenarioId", "idempotencyKey", "acknowledgementRecordingAllowed", "nextGate"]),
    view("auditor_terminal_retention_readback_ack_replay_digest_view"; "auditor"; durable_fields + ["readbackReceiptHash", "acknowledgementHash", "priorGateDigest", "monotonicityCheckId"]),
    view("release_owner_terminal_retention_readback_ack_replay_denial_view"; "release_owner"; durable_fields + ["releaseDenied", "publicationDenied", "publicClaimDenied", "externalDeliveryDenied"]),
    view("runtime_terminal_retention_readback_ack_replay_zero_effect_view"; "system"; durable_fields + ["replayRecorded", "acknowledgementRecorded", "authorityGranted", "publicClaimRecorded", "externalSendPerformed"])
  ] as $local_views
  | [
    invariant("terminal_receipt_retention_readback_ack_replay_requires_durable_identity_evidence"; "terminal receipt retention readback acknowledgement replay requires workflow, run, step, checkpoint, replay, rollback, and receipt evidence"),
    invariant("terminal_retention_readback_ack_replay_is_idempotent"; "duplicate receipt, duplicate acknowledgement, and stale digest replay cannot change state"),
    invariant("terminal_retention_readback_ack_replay_keeps_zero_side_effects"; "replay must preserve zero writes, zero traffic, zero release, zero public claims, and zero external sends"),
    invariant("terminal_retention_readback_ack_replay_requires_acknowledgement_gate"; "replay idempotency requires the terminal receipt retention readback acknowledgement gate"),
    invariant("terminal_retention_readback_ack_replay_is_scope_bound"; "cross-scope and superseded acknowledgement replay cannot unlock receipt recording"),
    invariant("terminal_retention_readback_ack_replay_views_are_local_only"; "operator, auditor, release-owner, and runtime views cannot be sent externally"),
    invariant("terminal_retention_readback_ack_replay_preview_has_no_side_effects"; "this gate cannot persist replay records, record acknowledgement, grant authority, publish, record public claims, or send externally")
  ] as $invariants
  | {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate",
      schema_version: "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_v1",
      preview_mode: "read_only_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_no_replay_write",
      replay_scenario_count: ($replay_scenarios | length),
      idempotency_guard_count: ($idempotency_guards | length),
      replay_denial_count: ($replay_denials | length),
      monotonicity_check_count: ($monotonicity_checks | length),
      local_view_count: ($local_views | length),
      invariant_count: ($invariants | length),
      required_prior_gates: $required_prior_gates,
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
      recommended_next_gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate",
      ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview: true,
      ready_for_operator_acceptance: false,
      ready_for_live_persistence: false,
      source_probes: {
        persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency: {
          rust_module_present: $replay_rust_module_present,
          report_script_present: $replay_report_script_present,
          gate_script_present: $replay_gate_script_present
        },
        persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement: {
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
        terminal_decision_recorded: false,
        terminal_decision_receipt_recorded: false,
        terminal_receipt_retention_state_persisted: false,
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
        public_claim_recorded: false,
        external_send_performed: false,
        model_invoked: false
      }
    }'
