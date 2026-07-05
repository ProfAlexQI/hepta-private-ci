#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

path_exists() { [[ -e "$1" ]]; }
bool_for() {
  if "$@"; then printf 'true\n'; else printf 'false\n'; fi
}

rust_module="codex-rs/hepta-runtime/src/wg_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_preview.rs"
report_script="scripts/hepta-systems-work-graph-term-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-expiry-readback-ack-replay-idempotency-preview-report.sh"
gate_script="scripts/hepta-systems-work-graph-term-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-expiry-readback-ack-replay-idempotency-preview-gate.sh"
prior_report_script="scripts/hepta-systems-work-graph-term-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-expiry-readback-ack-preview-report.sh"
prior_gate_script="scripts/hepta-systems-work-graph-term-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-expiry-readback-ack-preview-gate.sh"

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
    "operator_terminal_decision_receipt_retention_readback_acknowledgement",
    "auditor_terminal_decision_receipt_retention_readback_acknowledgement",
    "release_owner_terminal_decision_receipt_retention_readback_acknowledgement",
    "authority_denial_terminal_decision_receipt_retention_readback_acknowledgement",
    "public_claim_denial_terminal_decision_receipt_retention_readback_acknowledgement",
    "external_delivery_denial_terminal_decision_receipt_retention_readback_acknowledgement"
  ];
  def scenario_ids: [
    "duplicate_terminal_decision_receipt_retention_readback_receipt_replay",
    "duplicate_terminal_decision_receipt_retention_readback_acknowledgement_replay",
    "stale_terminal_decision_receipt_retention_readback_digest_replay",
    "superseded_terminal_decision_receipt_retention_readback_scope_replay",
    "cross_scope_terminal_decision_receipt_retention_readback_acknowledgement_replay",
    "out_of_order_terminal_decision_receipt_retention_readback_acknowledgement_replay"
  ];
  def scenario($id; $mode): {
    id: $id,
    source_acknowledgement_ids: ack_ids,
    replay_mode: $mode,
    acknowledgement_recording_allowed: false,
    mutation_allowed: false
  };
  def idempotency_guard($id; $fields): {
    id: $id,
    required_fields: $fields,
    blocks_replay_mutation: true
  };
  def replay_denial($id; $reason): {
    id: $id,
    applies_to_replay_scenario_ids: scenario_ids,
    reason: $reason,
    blocks_acknowledgement_recording: true,
    blocks_acceptance: true,
    blocks_approval: true,
    blocks_authority: true,
    blocks_rollout: true,
    blocks_release_publication: true,
    blocks_public_claim: true,
    blocks_external_delivery: true
  };
  def monotonicity_check($id; $fields): {
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
  def invariant($id; $text): {
    id: $id,
    required: true,
    reason: $text
  };
  [
    scenario("duplicate_terminal_decision_receipt_retention_readback_receipt_replay"; "duplicate_readback_receipt"),
    scenario("duplicate_terminal_decision_receipt_retention_readback_acknowledgement_replay"; "duplicate_acknowledgement"),
    scenario("stale_terminal_decision_receipt_retention_readback_digest_replay"; "stale_digest"),
    scenario("superseded_terminal_decision_receipt_retention_readback_scope_replay"; "superseded_scope"),
    scenario("cross_scope_terminal_decision_receipt_retention_readback_acknowledgement_replay"; "cross_scope_acknowledgement"),
    scenario("out_of_order_terminal_decision_receipt_retention_readback_acknowledgement_replay"; "out_of_order_acknowledgement")
  ] as $replay_scenarios
  | [
    idempotency_guard("terminal_decision_receipt_retention_readback_receipt_idempotency_key_required"; ["readbackReceiptId", "readbackReceiptHash", "retentionPolicyHash"]),
    idempotency_guard("terminal_decision_receipt_retention_readback_acknowledgement_idempotency_key_required"; ["acknowledgementId", "acknowledgementHash", "localViewHash"]),
    idempotency_guard("terminal_decision_receipt_retention_readback_prior_gate_digest_binding_required"; ["priorGateId", "priorGateDigest", "sourceReportHash"]),
    idempotency_guard("terminal_decision_receipt_retention_readback_scope_epoch_binding_required"; ["receiptScope", "acknowledgementScope", "scopeEpoch"]),
    idempotency_guard("terminal_decision_receipt_retention_readback_supersession_guard_required"; ["supersessionHash", "retentionWindow", "expiryState"]),
    idempotency_guard("terminal_decision_receipt_retention_readback_zero_effect_digest_required"; ["zeroWriteHash", "zeroTrafficHash", "zeroExternalSendHash"]),
    idempotency_guard("terminal_decision_receipt_retention_readback_release_public_claim_denial_binding_required"; ["releaseDenied", "publicClaimDenied", "externalDeliveryDenied"])
  ] as $idempotency_guards
  | [
    replay_denial("terminal_decision_receipt_retention_readback_duplicate_receipt_denied"; "duplicate readback receipt cannot mutate or record"),
    replay_denial("terminal_decision_receipt_retention_readback_duplicate_ack_denied"; "duplicate acknowledgement cannot record acknowledgement"),
    replay_denial("terminal_decision_receipt_retention_readback_stale_digest_denied"; "stale digest cannot accept or grant authority"),
    replay_denial("terminal_decision_receipt_retention_readback_superseded_scope_denied"; "superseded scope cannot update retention state"),
    replay_denial("terminal_decision_receipt_retention_readback_cross_scope_denied"; "cross-scope acknowledgement cannot bind authority"),
    replay_denial("terminal_decision_receipt_retention_readback_out_of_order_denied"; "out-of-order replay cannot advance monotonicity"),
    replay_denial("terminal_decision_receipt_retention_readback_external_delivery_replay_denied"; "replay cannot send external delivery or record public claims")
  ] as $replay_denials
  | [
    monotonicity_check("check_terminal_decision_receipt_retention_readback_receipt_sequence"; ["readbackReceiptSequence", "priorReadbackReceiptSequence", "scopeEpoch"]),
    monotonicity_check("check_terminal_decision_receipt_retention_readback_acknowledgement_sequence"; ["acknowledgementSequence", "priorAcknowledgementSequence", "scopeEpoch"]),
    monotonicity_check("check_terminal_decision_receipt_retention_readback_digest_epoch"; ["digestEpoch", "priorDigestEpoch", "supersessionHash"]),
    monotonicity_check("check_terminal_decision_receipt_retention_readback_release_public_claim_epoch"; ["releaseDenialEpoch", "publicClaimDenialEpoch", "externalDeliveryDenialEpoch"]),
    monotonicity_check("check_terminal_decision_receipt_retention_readback_next_gate_order"; ["currentGate", "nextGate", "priorGate"])
  ] as $monotonicity_checks
  | [
    view("operator_terminal_decision_receipt_retention_readback_ack_replay_view"; "operator"; ["replayScenarioId", "acknowledgementId", "recordingDenied", "nextGate"]),
    view("auditor_terminal_decision_receipt_retention_readback_ack_replay_digest_view"; "auditor"; ["idempotencyKeyHash", "priorGateDigest", "monotonicityCheckId", "zeroEffectHash"]),
    view("release_owner_terminal_decision_receipt_retention_readback_ack_replay_denial_view"; "release_owner"; ["releaseDenied", "publicationDenied", "publicClaimDenied", "externalDeliveryDenied"]),
    view("runtime_terminal_decision_receipt_retention_readback_ack_replay_zero_effect_view"; "system"; ["replayRecorded", "authorityGranted", "trafficRouted", "externalSendPerformed"])
  ] as $local_views
  | [
    invariant("terminal_decision_receipt_retention_readback_ack_replay_is_idempotent"; "duplicate readback receipt and acknowledgement replays are idempotent no-ops"),
    invariant("terminal_decision_receipt_retention_readback_ack_replay_blocks_recording"; "replay cannot record acknowledgement, receipt, acceptance, or approval"),
    invariant("terminal_decision_receipt_retention_readback_ack_replay_blocks_authority"; "replay cannot grant authority or enable live persistence"),
    invariant("terminal_decision_receipt_retention_readback_ack_replay_blocks_rollout"; "replay cannot start rollout, traffic, release publication, or public claims"),
    invariant("terminal_decision_receipt_retention_readback_ack_replay_views_are_local_only"; "replay views remain local and hash-only"),
    invariant("terminal_decision_receipt_retention_readback_ack_replay_preview_has_no_side_effects"; "this gate cannot persist, write WAL/checkpoints, publish, record public claims, or send externally")
  ] as $invariants
  | {
    product: "Hepta",
    runtime: "hepta",
    status: "ready",
    gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_gate",
    schema_version: "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_v1",
    preview_mode: "read_only_terminal_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_replay_idempotency_preview_no_replay_write",
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
    recommended_next_gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview_gate",
    ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_preview: true,
    ready_for_operator_acceptance: false,
    ready_for_live_persistence: false,
    side_effects: {
      filesystem_written: false,
      graph_state_persisted: false,
      terminal_decision_recorded: false,
      terminal_decision_receipt_recorded: false,
      terminal_decision_receipt_persisted: false,
      terminal_decision_receipt_acknowledgement_recorded: false,
      retention_state_persisted: false,
      readback_receipt_persisted: false,
      receipt_acknowledgement_recorded: false,
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
      term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay: {
        rust_module_present: $rust_module_present,
        report_script_present: $report_script_present,
        gate_script_present: $gate_script_present
      },
      term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_acknowledgement: {
        report_script_present: $prior_report_script_present,
        gate_script_present: $prior_gate_script_present
      }
    }
  }
'
