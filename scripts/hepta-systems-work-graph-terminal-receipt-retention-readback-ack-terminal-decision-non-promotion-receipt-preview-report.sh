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

rust_module="codex-rs/hepta-runtime/src/work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview.rs"
report_script="scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-non-promotion-receipt-preview-report.sh"
gate_script="scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-non-promotion-receipt-preview-gate.sh"
prior_report_script="scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-non-promotion-preview-report.sh"
prior_gate_script="scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-non-promotion-preview-gate.sh"

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
  def surface_ids: [
    "operator_terminal_retention_readback_ack_decision_visibility",
    "release_owner_terminal_retention_readback_ack_decision_visibility",
    "auditor_terminal_retention_readback_ack_decision_visibility",
    "rollback_owner_terminal_retention_readback_ack_decision_visibility",
    "runtime_terminal_retention_readback_ack_state_summary_visibility",
    "external_delivery_terminal_retention_readback_ack_decision_echo"
  ];
  def receipt_ids: [
    "operator_terminal_retention_readback_ack_non_promotion_decision_receipt",
    "release_owner_terminal_retention_readback_ack_non_promotion_decision_receipt",
    "authority_denial_terminal_retention_readback_ack_non_promotion_receipt",
    "rollout_denial_terminal_retention_readback_ack_non_promotion_receipt",
    "release_publication_denial_terminal_retention_readback_ack_non_promotion_receipt",
    "external_delivery_denial_terminal_retention_readback_ack_non_promotion_receipt"
  ];
  def receipt($id; $hash_mode): {
    id: $id,
    source_terminal_decision_surface_ids: surface_ids,
    receipt_hash_mode: $hash_mode,
    persisted: false,
    receipt_recording_allowed: false,
    acceptance_allowed: false,
    external_delivery_enabled: false
  };
  def digest_check($id; $fields): {
    id: $id,
    compared_fields: $fields,
    blocks_receipt_recording: true
  };
  def mismatch_denial($id; $reason): {
    id: $id,
    applies_to_receipt_ids: receipt_ids,
    reason: $reason,
    blocks_receipt_recording: true,
    blocks_acceptance: true,
    blocks_authority: true,
    blocks_rollout: true,
    blocks_release_publication: true,
    blocks_public_claim: true,
    blocks_external_delivery: true
  };
  def receipt_guard($id; $fields): {
    id: $id,
    required_fields: $fields,
    receipt_recording_allowed: false,
    promotion_allowed: false,
    public_claim_allowed: false
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
    receipt("operator_terminal_retention_readback_ack_non_promotion_decision_receipt"; "hash_only_operator_terminal_retention_readback_ack_decision_receipt"),
    receipt("release_owner_terminal_retention_readback_ack_non_promotion_decision_receipt"; "hash_only_release_owner_terminal_retention_readback_ack_decision_receipt"),
    receipt("authority_denial_terminal_retention_readback_ack_non_promotion_receipt"; "hash_only_authority_denial_terminal_retention_readback_ack_receipt"),
    receipt("rollout_denial_terminal_retention_readback_ack_non_promotion_receipt"; "hash_only_rollout_denial_terminal_retention_readback_ack_receipt"),
    receipt("release_publication_denial_terminal_retention_readback_ack_non_promotion_receipt"; "hash_only_release_publication_denial_terminal_retention_readback_ack_receipt"),
    receipt("external_delivery_denial_terminal_retention_readback_ack_non_promotion_receipt"; "hash_only_external_delivery_denial_terminal_retention_readback_ack_receipt")
  ] as $receipts
  | [
    digest_check("terminal_retention_readback_ack_decision_surface_digest_matches"; ["terminalDecisionSurfaceId", "terminalDecisionHash", "sourceGateDigest"]),
    digest_check("terminal_retention_readback_ack_non_promotion_denial_digest_matches"; ["nonPromotionDenialId", "denialHash", "zeroPromotionHash"]),
    digest_check("terminal_retention_readback_ack_authority_guard_digest_matches"; ["authorityGuardId", "authorityGuardHash", "authorityGranted"]),
    digest_check("terminal_retention_readback_ack_release_delivery_guard_digest_matches"; ["releaseDeliveryGuardId", "releaseHash", "deliveryHash"]),
    digest_check("terminal_retention_readback_ack_local_view_digest_matches"; ["localViewId", "localViewHash", "externalDeliveryEnabled"]),
    digest_check("terminal_retention_readback_ack_zero_side_effect_digest_matches"; ["zeroWriteHash", "zeroTrafficHash", "zeroExternalSendHash"])
  ] as $digest_checks
  | [
    mismatch_denial("missing_terminal_retention_readback_ack_decision_surface_cannot_record_receipt"; "missing terminal retention readback acknowledgement decision surface cannot record receipt"),
    mismatch_denial("mismatched_terminal_retention_readback_ack_decision_hash_cannot_accept"; "mismatched terminal retention readback acknowledgement decision hash cannot become acceptance"),
    mismatch_denial("stale_terminal_retention_readback_ack_replay_digest_cannot_grant_authority"; "stale terminal retention readback acknowledgement replay digest cannot grant authority"),
    mismatch_denial("authority_guard_absence_after_terminal_retention_readback_ack_cannot_start_rollout"; "absent authority guard after terminal retention readback acknowledgement cannot start rollout or route traffic"),
    mismatch_denial("release_delivery_guard_absence_after_terminal_retention_readback_ack_cannot_publish"; "release and delivery guard absence after terminal retention readback acknowledgement cannot publish release state"),
    mismatch_denial("external_delivery_terminal_retention_readback_ack_receipt_echo_cannot_send"; "external delivery receipt echo cannot send externally"),
    mismatch_denial("terminal_retention_readback_ack_receipt_readback_is_not_live_completion"; "terminal retention readback acknowledgement receipt readback cannot claim live persistence completion")
  ] as $mismatch_denials
  | [
    receipt_guard("terminal_retention_readback_ack_decision_receipt_is_hash_only"; ["receiptId", "receiptHash", "redactionHash"]),
    receipt_guard("terminal_retention_readback_ack_decision_receipt_is_non_persistent"; ["receiptPersisted", "receiptStorageScope", "receiptWriteHash"]),
    receipt_guard("terminal_retention_readback_ack_decision_receipt_is_non_accepting"; ["acceptanceAllowed", "approvalRecorded", "authorityGranted"]),
    receipt_guard("terminal_retention_readback_ack_decision_receipt_keeps_release_denied"; ["releasePublished", "publicClaimRecorded", "artifactPublished"]),
    receipt_guard("terminal_retention_readback_ack_decision_receipt_keeps_external_delivery_denied"; ["externalDeliveryEnabled", "destinationPolicy", "externalSendPerformed"])
  ] as $receipt_guards
  | [
    view("operator_terminal_retention_readback_ack_non_promotion_receipt_view"; "operator"; ["receiptId", "receiptHash", "acceptanceAllowed", "nextGate"]),
    view("release_owner_terminal_retention_readback_ack_non_promotion_receipt_view"; "release_owner"; ["releasePublished", "publicClaimRecorded", "receiptPersisted", "externalDeliveryDenied"]),
    view("auditor_terminal_retention_readback_ack_non_promotion_receipt_digest_view"; "auditor"; ["sourceGateDigest", "receiptHash", "digestCheckId", "mismatchDenialId"]),
    view("runtime_terminal_retention_readback_ack_non_promotion_receipt_zero_effect_view"; "system"; ["terminalDecisionReceiptRecorded", "authorityGranted", "trafficRouted", "externalSendPerformed"])
  ] as $local_views
  | [
    invariant("terminal_retention_readback_ack_non_promotion_receipts_are_hash_only"; "terminal retention readback acknowledgement decision non-promotion receipts expose hashes only"),
    invariant("terminal_retention_readback_ack_non_promotion_receipts_are_not_recorded"; "terminal decision receipt readback cannot record receipt state"),
    invariant("terminal_retention_readback_ack_non_promotion_receipts_are_not_acceptance"; "terminal decision receipt visibility cannot become acceptance or authority"),
    invariant("terminal_retention_readback_ack_non_promotion_receipts_keep_release_denied"; "release publication, public claim, rollout, and traffic routing remain denied"),
    invariant("terminal_retention_readback_ack_non_promotion_receipt_views_are_local_only"; "operator, release-owner, auditor, and runtime receipt views cannot be sent externally"),
    invariant("terminal_retention_readback_ack_non_promotion_receipt_preview_has_no_side_effects"; "this gate cannot persist receipts, record approval, grant authority, publish, or send externally")
  ] as $invariants
  | {
    product: "Hepta",
    runtime: "hepta",
    status: "ready",
    gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate",
    schema_version: "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_v1",
    preview_mode: "read_only_terminal_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_hash_only_no_recording",
    receipt_count: ($receipts | length),
    digest_check_count: ($digest_checks | length),
    mismatch_denial_count: ($mismatch_denials | length),
    receipt_guard_count: ($receipt_guards | length),
    local_view_count: ($local_views | length),
    invariant_count: ($invariants | length),
    required_prior_gates: $required_prior_gates,
    receipts: $receipts,
    digest_checks: $digest_checks,
    mismatch_denials: $mismatch_denials,
    receipt_guards: $receipt_guards,
    local_views: $local_views,
    invariants: $invariants,
    recommended_next_gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate",
    ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview: true,
    ready_for_operator_acceptance: false,
    ready_for_live_persistence: false,
    side_effects: {
      filesystem_written: false,
      graph_state_persisted: false,
      terminal_decision_recorded: false,
      terminal_decision_receipt_recorded: false,
      terminal_receipt_retention_state_persisted: false,
      readback_receipt_persisted: false,
      readback_acknowledgement_recorded: false,
      terminal_decision_receipt_persisted: false,
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
      terminal_receipt_retention_readback_ack_terminal_decision_non_promotion_receipt: {
        rust_module_present: $rust_module_present,
        report_script_present: $report_script_present,
        gate_script_present: $gate_script_present
      },
      terminal_receipt_retention_readback_ack_terminal_decision_non_promotion: {
        report_script_present: $prior_report_script_present,
        gate_script_present: $prior_gate_script_present
      }
    }
  }
'
