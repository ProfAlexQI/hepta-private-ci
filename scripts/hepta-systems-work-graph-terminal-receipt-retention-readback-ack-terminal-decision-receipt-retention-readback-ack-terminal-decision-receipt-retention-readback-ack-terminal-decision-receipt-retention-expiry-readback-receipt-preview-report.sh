#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

path_exists() { [[ -e "$1" ]]; }
bool_for() {
  if "$@"; then printf 'true\n'; else printf 'false\n'; fi
}

rust_module="codex-rs/hepta-runtime/src/wg_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt_preview.rs"
report_script="scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-expiry-readback-receipt-preview-report.sh"
gate_script="scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-expiry-readback-receipt-preview-gate.sh"
prior_report_script="scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-expiry-preview-report.sh"
prior_gate_script="scripts/hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-retention-expiry-preview-gate.sh"
durable_identity_report_script="scripts/hepta-systems-work-graph-durable-identity-preview-report.sh"
durable_identity_gate_script="scripts/hepta-systems-work-graph-durable-identity-preview-gate.sh"

required_prior_gates="$("$ROOT/$prior_report_script" | jq -c '(.required_prior_gates | map(select(. != "hepta_work_graph_durable_identity_preview_gate"))) + [.gate, "hepta_work_graph_durable_identity_preview_gate"]')"
rust_module_present="$(bool_for path_exists "$rust_module")"
report_script_present="$(bool_for path_exists "$report_script")"
gate_script_present="$(bool_for path_exists "$gate_script")"
prior_report_script_present="$(bool_for path_exists "$prior_report_script")"
prior_gate_script_present="$(bool_for path_exists "$prior_gate_script")"
durable_identity_report_script_present="$(bool_for path_exists "$durable_identity_report_script")"
durable_identity_gate_script_present="$(bool_for path_exists "$durable_identity_gate_script")"

jq -n \
  --argjson required_prior_gates "$required_prior_gates" \
  --argjson rust_module_present "$rust_module_present" \
  --argjson report_script_present "$report_script_present" \
  --argjson gate_script_present "$gate_script_present" \
  --argjson prior_report_script_present "$prior_report_script_present" \
  --argjson prior_gate_script_present "$prior_gate_script_present" \
  --argjson durable_identity_report_script_present "$durable_identity_report_script_present" \
  --argjson durable_identity_gate_script_present "$durable_identity_gate_script_present" \
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
  def receipt_ids: [
    "terminal_decision_receipt_retention_policy_readback_receipt",
    "terminal_decision_receipt_expiry_guard_readback_receipt",
    "terminal_decision_receipt_supersession_guard_readback_receipt",
    "terminal_decision_receipt_gc_denial_readback_receipt",
    "terminal_decision_receipt_zero_effect_digest_readback_receipt",
    "terminal_decision_receipt_release_public_claim_denial_readback_receipt"
  ];
  def receipt($id; $surface): {
    id: $id,
    source_retention_surface: $surface,
    required_fields: (durable_fields + ["readbackReceiptId", "sourceRetentionSurface", "retentionPolicyHash", "expiryGuardHash", "supersessionHash", "garbageCollectionDenialHash", "zeroEffectHash", "nextGate"]),
    redaction_state: "hash_only_redacted",
    persistence_enabled: false,
    external_delivery_enabled: false
  };
  def digest_check($id; $fields): {
    id: $id,
    compared_fields: (durable_fields + $fields),
    blocks_receipt_acceptance: true
  };
  def mismatch_denial($id; $reason): {
    id: $id,
    applies_to_receipt_ids: receipt_ids,
    reason: $reason,
    blocks_receipt_recording: true,
    blocks_acknowledgement_recording: true,
    blocks_acceptance: true,
    blocks_persistence: true,
    blocks_authority: true,
    blocks_rollout: true,
    blocks_release_publication: true,
    blocks_public_claim: true,
    blocks_external_delivery: true
  };
  def receipt_guard($id; $fields): {
    id: $id,
    required_fields: (durable_fields + $fields),
    blocks_recording: true
  };
  def view($id; $audience; $fields): {
    id: $id,
    audience: $audience,
    required_fields: (durable_fields + $fields),
    external_delivery_enabled: false
  };
  def invariant($id; $reason): {
    id: $id,
    required: true,
    reason: $reason
  };
  [
    receipt("terminal_decision_receipt_retention_policy_readback_receipt"; "terminal_decision_receipt_retention_policies"),
    receipt("terminal_decision_receipt_expiry_guard_readback_receipt"; "terminal_decision_receipt_retention_expiry_guards"),
    receipt("terminal_decision_receipt_supersession_guard_readback_receipt"; "terminal_decision_receipt_retention_supersession_guards"),
    receipt("terminal_decision_receipt_gc_denial_readback_receipt"; "terminal_decision_receipt_garbage_collection_denials"),
    receipt("terminal_decision_receipt_zero_effect_digest_readback_receipt"; "terminal_decision_receipt_retention_zero_effect_digests"),
    receipt("terminal_decision_receipt_release_public_claim_denial_readback_receipt"; "terminal_decision_receipt_release_public_claim_external_delivery_denials")
  ] as $readback_receipts
  | [
    digest_check("check_terminal_decision_receipt_retention_policy_digest"; ["retentionPolicyIds", "retentionWindowHash", "hashOnly"]),
    digest_check("check_terminal_decision_receipt_expiry_guard_digest"; ["expiryGuardIds", "expired", "blocksPersistence"]),
    digest_check("check_terminal_decision_receipt_supersession_digest"; ["supersessionGuardIds", "scopeEpochHash", "blocksMutation"]),
    digest_check("check_terminal_decision_receipt_gc_denial_digest"; ["garbageCollectionDenialIds", "gcAllowed", "tombstonePersisted"]),
    digest_check("check_terminal_decision_receipt_zero_effect_digest"; ["zeroWriteHash", "zeroTrafficHash", "zeroExternalSendHash"]),
    digest_check("check_terminal_decision_receipt_prior_gate_digest"; ["priorGateId", "priorGateDigest", "readbackReceiptHash"]),
    digest_check("check_terminal_decision_receipt_retention_durable_identity_digest"; ["durableIdentityEvidenceHash", "durableFieldCount", "receiptHash"])
  ] as $digest_checks
  | [
    mismatch_denial("durable_identity_evidence_missing"; "terminal decision receipt retention readback receipt preview cannot advance without durable identity evidence"),
    mismatch_denial("missing_terminal_decision_receipt_retention_policy_digest"; "terminal decision receipt retention readback is missing policy digest"),
    mismatch_denial("expired_terminal_decision_receipt_replayed"; "expired terminal decision receipt acknowledgement evidence was replayed"),
    mismatch_denial("superseded_terminal_decision_receipt_scope_replayed"; "superseded terminal decision receipt scope was replayed"),
    mismatch_denial("terminal_decision_receipt_gc_tombstone_persistence_attempted"; "terminal decision receipt retention readback attempted to persist a GC tombstone"),
    mismatch_denial("terminal_decision_receipt_zero_effect_digest_nonzero"; "terminal decision receipt retention readback does not prove zero side effects"),
    mismatch_denial("terminal_decision_receipt_public_claim_attempted"; "terminal decision receipt retention readback cannot record public claims"),
    mismatch_denial("terminal_decision_receipt_external_delivery_attempted"; "terminal decision receipt retention readback cannot send external delivery")
  ] as $mismatch_denials
  | [
    receipt_guard("hash_only_terminal_decision_receipt_retention_receipt_required"; ["retentionPolicyHash", "expiryGuardHash", "supersessionHash"]),
    receipt_guard("non_persistent_terminal_decision_receipt_readback_required"; ["persistenceEnabled", "receiptPersisted", "tombstonePersisted"]),
    receipt_guard("terminal_decision_receipt_local_view_only_required"; ["operatorViewHash", "auditorViewHash", "releaseOwnerViewHash"]),
    receipt_guard("terminal_decision_receipt_bounded_retention_window_required"; ["retentionWindow", "expiryState", "scopeEpoch"]),
    receipt_guard("terminal_decision_receipt_next_gate_acknowledgement_required"; ["recommendedNextGate", "acknowledgementAllowed", "acceptanceAllowed"])
  ] as $receipt_guards
  | [
    view("operator_terminal_decision_receipt_retention_readback_receipt_view"; "operator"; ["readbackReceiptId", "retentionPolicyId", "expired", "nextGate"]),
    view("auditor_terminal_decision_receipt_retention_readback_digest_view"; "auditor"; ["readbackReceiptHash", "retentionPolicyHash", "gcDenialHash", "zeroEffectHash"]),
    view("release_owner_terminal_decision_receipt_retention_readback_denial_view"; "release_owner"; ["releaseDenied", "publicationDenied", "publicClaimDenied", "externalDeliveryDenied"]),
    view("runtime_terminal_decision_receipt_retention_readback_zero_effect_view"; "system"; ["retentionStatePersisted", "readbackReceiptPersisted", "authorityGranted", "publicClaimRecorded", "externalSendPerformed"])
  ] as $local_views
  | [
    invariant("terminal_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipts_require_durable_identity_evidence"; "terminal decision receipt retention readback receipts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence"),
    invariant("terminal_decision_receipt_retention_readback_receipts_are_hash_only"; "terminal decision receipt retention readback receipts contain hash-only redacted evidence"),
    invariant("terminal_decision_receipt_retention_readback_receipts_are_non_persistent"; "terminal decision receipt retention readback cannot write receipt, retention, expiry, or tombstone state"),
    invariant("terminal_decision_receipt_retention_readback_receipts_block_acceptance"; "terminal decision receipt retention readback cannot become acceptance or approval recording"),
    invariant("terminal_decision_receipt_retention_readback_receipts_block_gc_mutation"; "terminal decision receipt retention readback cannot perform garbage collection or persist tombstones"),
    invariant("terminal_decision_receipt_retention_readback_views_are_local_only"; "operator, auditor, release-owner, and runtime terminal decision receipt readback views cannot be sent externally"),
    invariant("terminal_decision_receipt_retention_readback_preview_has_no_side_effects"; "this gate cannot persist, grant authority, enable live execution, publish, record public claims, or send externally")
  ] as $invariants
  | {
    product: "Hepta",
    runtime: "hepta",
    status: "ready",
    gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_gate",
    schema_version: "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_v1",
    preview_mode: "read_only_terminal_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_receipt_preview_no_receipt_write",
    readback_receipt_count: ($readback_receipts | length),
    digest_check_count: ($digest_checks | length),
    mismatch_denial_count: ($mismatch_denials | length),
    receipt_guard_count: ($receipt_guards | length),
    local_view_count: ($local_views | length),
    invariant_count: ($invariants | length),
    required_prior_gates: $required_prior_gates,
    readback_receipts: $readback_receipts,
    digest_checks: $digest_checks,
    mismatch_denials: $mismatch_denials,
    receipt_guards: $receipt_guards,
    local_views: $local_views,
    durable_identity_evidence: {
      schema_version: "work_graph_durable_identity_preview_v1",
      required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
      required_field_ids: durable_fields,
      required_for_readback_receipt_ids: receipt_ids,
      durable_field_count: (durable_fields | length),
      preview_binding_count: 5,
      invariant_count: 7,
      currently_satisfied: false
    },
    invariants: $invariants,
    recommended_next_gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview_gate",
    ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_preview: true,
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
      retention_state_persisted: false,
      readback_receipt_persisted: false,
      receipt_acknowledgement_recorded: false,
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
      terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_receipt: {
        rust_module_present: $rust_module_present,
        report_script_present: $report_script_present,
        gate_script_present: $gate_script_present
      },
      terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_expiry: {
        report_script_present: $prior_report_script_present,
        gate_script_present: $prior_gate_script_present
      },
      durable_identity: {
        report_script_present: $durable_identity_report_script_present,
        gate_script_present: $durable_identity_gate_script_present
      }
    }
  }
'
