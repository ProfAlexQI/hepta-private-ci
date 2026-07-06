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

rust_module="codex-rs/hepta-runtime/src/wg_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_td_rcpt_ret_exp_rdbk_ack_td_receipt_ack_replay_preview.rs"
report_script="scripts/hepta-systems-work-graph-term-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-ret-exp-rdbk-ack-td-receipt-ack-replay-idempotency-preview-report.sh"
gate_script="scripts/hepta-systems-work-graph-term-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-ret-exp-rdbk-ack-td-receipt-ack-replay-idempotency-preview-gate.sh"
prior_report_script="scripts/hepta-systems-work-graph-term-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-ret-exp-rdbk-ack-td-receipt-ack-preview-report.sh"
prior_gate_script="scripts/hepta-systems-work-graph-term-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-ret-exp-rdbk-ack-td-receipt-ack-preview-gate.sh"

required_prior_gates="$("$ROOT/$prior_report_script" | jq -c '.required_prior_gates + [.gate]')"
rust_module_present="$(bool_for path_exists "$rust_module")"
report_script_present="$(bool_for path_exists "$report_script")"
gate_script_present="$(bool_for path_exists "$gate_script")"
prior_report_script_present="$(bool_for path_exists "$prior_report_script")"
prior_gate_script_present="$(bool_for path_exists "$prior_gate_script")"

jq -n \
  --argjson required_prior_gates "$required_prior_gates" \
  --argjson rust_module_present "$rust_module_present" \
  --argjson report_script_present "$report_script_present" \
  --argjson gate_script_present "$gate_script_present" \
  --argjson prior_report_script_present "$prior_report_script_present" \
  --argjson prior_gate_script_present "$prior_gate_script_present" \
  '
  def ack_ids: [
    "operator_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement",
    "release_owner_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement",
    "authority_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
    "rollout_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
    "release_publication_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement",
    "external_delivery_denial_terminal_decision_receipt_retention_readback_ack_receipt_acknowledgement"
  ];
  def scenario_ids: [
    "duplicate_terminal_decision_receipt_retention_readback_ack_decision_receipt_replay",
    "duplicate_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement_replay",
    "stale_terminal_decision_receipt_retention_readback_ack_decision_receipt_digest_replay",
    "superseded_terminal_decision_receipt_retention_readback_ack_decision_receipt_scope_replay",
    "cross_scope_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement_replay",
    "out_of_order_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement_replay"
  ];
  def scenario($id; $mode): {
    id: $id,
    source_acknowledgement_ids: ack_ids,
    replay_mode: $mode,
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
    scenario("duplicate_terminal_decision_receipt_retention_readback_ack_decision_receipt_replay"; "duplicate_receipt"),
    scenario("duplicate_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement_replay"; "duplicate_acknowledgement"),
    scenario("stale_terminal_decision_receipt_retention_readback_ack_decision_receipt_digest_replay"; "stale_receipt_digest"),
    scenario("superseded_terminal_decision_receipt_retention_readback_ack_decision_receipt_scope_replay"; "superseded_receipt_scope"),
    scenario("cross_scope_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement_replay"; "cross_scope_acknowledgement"),
    scenario("out_of_order_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement_replay"; "out_of_order_acknowledgement")
  ] as $replay_scenarios
  | [
    guard("terminal_decision_receipt_retention_readback_ack_decision_receipt_idempotency_key_required"; ["receiptId", "receiptHash", "terminalDecisionHash"]),
    guard("terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement_idempotency_key_required"; ["acknowledgementId", "acknowledgementHash", "localViewHash"]),
    guard("terminal_decision_receipt_retention_readback_ack_decision_receipt_prior_gate_digest_binding_required"; ["priorGateId", "priorGateDigest", "sourceReportHash"]),
    guard("terminal_decision_receipt_retention_readback_ack_decision_receipt_scope_epoch_binding_required"; ["receiptScope", "scopeEpoch", "supersessionHash"]),
    guard("terminal_decision_receipt_retention_readback_ack_decision_receipt_zero_side_effect_digest_binding_required"; ["zeroWriteHash", "zeroTrafficHash", "zeroExternalSendHash"]),
    guard("terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement_sequence_required"; ["receiptSequence", "acknowledgementSequence", "sequenceHash"]),
    guard("terminal_decision_receipt_retention_readback_ack_decision_receipt_replay_keeps_non_promotion_denied"; ["acknowledgementRecorded", "acceptanceAllowed", "authorityGranted"])
  ] as $idempotency_guards
  | [
    denial("duplicate_terminal_decision_receipt_retention_readback_ack_decision_receipt_cannot_record_acknowledgement"; "duplicate terminal retention readback acknowledgement decision receipt replay cannot record acknowledgement"),
    denial("duplicate_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement_cannot_record_acceptance"; "duplicate terminal retention readback acknowledgement decision receipt acknowledgement replay cannot record acceptance"),
    denial("stale_terminal_decision_receipt_retention_readback_ack_decision_receipt_digest_cannot_grant_authority"; "stale terminal retention readback acknowledgement decision receipt digest cannot grant authority"),
    denial("cross_scope_terminal_decision_receipt_retention_readback_ack_decision_receipt_replay_cannot_enable_live_persistence"; "cross-scope terminal retention readback acknowledgement decision receipt replay cannot enable live persistence, WAL, or checkpoints"),
    denial("out_of_order_terminal_decision_receipt_retention_readback_ack_decision_receipt_replay_cannot_start_rollout"; "out-of-order terminal retention readback acknowledgement decision receipt replay cannot start rollout or route traffic"),
    denial("superseded_terminal_decision_receipt_retention_readback_ack_decision_receipt_replay_cannot_publish_or_claim"; "superseded terminal retention readback acknowledgement decision receipt replay cannot publish release status or record public claims"),
    denial("replayed_terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement_cannot_send_external_delivery"; "replayed terminal retention readback acknowledgement decision receipt acknowledgement cannot send external delivery")
  ] as $replay_denials
  | [
    check("terminal_decision_receipt_retention_readback_ack_decision_receipt_sequence_check"; ["receiptId", "receiptSequence", "receiptHash"]),
    check("terminal_decision_receipt_retention_readback_ack_decision_receipt_acknowledgement_sequence_check"; ["acknowledgementId", "acknowledgementSequence", "acknowledgementHash"]),
    check("terminal_decision_receipt_retention_readback_ack_decision_receipt_prior_gate_digest_monotonicity_check"; ["priorGateId", "priorGateDigest", "observedAt"]),
    check("terminal_decision_receipt_retention_readback_ack_decision_receipt_scope_epoch_monotonicity_check"; ["receiptScope", "scopeEpoch", "supersessionHash"]),
    check("terminal_decision_receipt_retention_readback_ack_decision_receipt_zero_effect_digest_stability_check"; ["zeroWriteHash", "zeroTrafficHash", "zeroExternalSendHash"])
  ] as $monotonicity_checks
  | [
    view("operator_terminal_decision_receipt_retention_readback_ack_decision_receipt_ack_replay_idempotency_view"; "operator"; ["replayScenarioId", "idempotencyKey", "acknowledgementRecordingAllowed", "nextGate"]),
    view("auditor_terminal_decision_receipt_retention_readback_ack_decision_receipt_ack_replay_digest_view"; "auditor"; ["receiptHash", "acknowledgementHash", "priorGateDigest", "monotonicityCheckId"]),
    view("release_owner_terminal_decision_receipt_retention_readback_ack_decision_receipt_ack_replay_denial_view"; "release_owner"; ["releaseDenied", "publicationDenied", "publicClaimDenied", "externalDeliveryDenied"]),
    view("runtime_terminal_decision_receipt_retention_readback_ack_decision_receipt_ack_replay_zero_effect_view"; "system"; ["replayRecorded", "acknowledgementRecorded", "authorityGranted", "trafficRouted", "externalSendPerformed"])
  ] as $local_views
  | [
    invariant("terminal_decision_receipt_retention_readback_ack_decision_receipt_ack_replay_is_idempotent"; "duplicate receipt, duplicate acknowledgement, and stale digest replay cannot change state"),
    invariant("terminal_decision_receipt_retention_readback_ack_decision_receipt_ack_replay_keeps_zero_side_effects"; "replay must preserve zero writes, zero traffic, zero release, zero public claims, and zero external sends"),
    invariant("terminal_decision_receipt_retention_readback_ack_decision_receipt_ack_replay_requires_acknowledgement_gate"; "replay idempotency requires the terminal decision receipt acknowledgement gate"),
    invariant("terminal_decision_receipt_retention_readback_ack_decision_receipt_ack_replay_is_scope_bound"; "cross-scope and superseded acknowledgement replay cannot unlock receipt recording"),
    invariant("terminal_decision_receipt_retention_readback_ack_decision_receipt_ack_replay_views_are_local_only"; "operator, auditor, release-owner, and runtime views cannot be sent externally"),
    invariant("terminal_decision_receipt_retention_readback_ack_decision_receipt_ack_replay_preview_has_no_side_effects"; "this gate cannot persist replay records, record acknowledgement, grant authority, publish, record public claims, or send externally")
  ] as $invariants
  | {
    product: "Hepta",
    runtime: "hepta",
    status: "ready",
    gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_gate",
    schema_version: "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_v1",
    preview_mode: "read_only_terminal_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_replay_idempotency_preview_no_replay_write",
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
    invariants: $invariants,
    recommended_next_gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview_gate",
    ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_preview: true,
    ready_for_operator_acceptance: false,
    ready_for_live_persistence: false,
    side_effects: {
      filesystem_written: false,
      graph_state_persisted: false,
      terminal_decision_recorded: false,
      terminal_decision_persisted: false,
      terminal_decision_receipt_recorded: false,
      terminal_decision_receipt_persisted: false,
      terminal_decision_receipt_acknowledgement_recorded: false,
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
    },
    source_probes: {
      term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_td_rcpt_ret_exp_rdbk_ack_td_receipt_ack_replay_idempotency: {
        rust_module_present: $rust_module_present,
        report_script_present: $report_script_present,
        gate_script_present: $gate_script_present
      },
      term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_td_rcpt_ret_exp_rdbk_ack_td_receipt_acknowledgement: {
        report_script_present: $prior_report_script_present,
        gate_script_present: $prior_gate_script_present
      }
    }
  }
'
