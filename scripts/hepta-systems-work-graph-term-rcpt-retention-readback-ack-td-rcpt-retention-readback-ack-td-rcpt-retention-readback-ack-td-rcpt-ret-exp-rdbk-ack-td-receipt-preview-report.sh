#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

path_exists() { [[ -e "$1" ]]; }
bool_for() {
  if "$@"; then printf 'true\n'; else printf 'false\n'; fi
}

rust_module="codex-rs/hepta-runtime/src/wg_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_td_rcpt_ret_exp_rdbk_ack_td_receipt_preview.rs"
report_script="scripts/hepta-systems-work-graph-term-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-ret-exp-rdbk-ack-td-receipt-preview-report.sh"
gate_script="scripts/hepta-systems-work-graph-term-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-ret-exp-rdbk-ack-td-receipt-preview-gate.sh"
prior_report_script="scripts/hepta-systems-work-graph-term-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-ret-exp-rdbk-ack-td-preview-report.sh"
prior_gate_script="scripts/hepta-systems-work-graph-term-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-retention-readback-ack-td-rcpt-ret-exp-rdbk-ack-td-preview-gate.sh"
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
  def surface_ids: [
    "operator_terminal_decision_receipt_retention_readback_ack_decision_visibility",
    "release_owner_terminal_decision_receipt_retention_readback_ack_decision_visibility",
    "auditor_terminal_decision_receipt_retention_readback_ack_decision_visibility",
    "rollback_owner_terminal_decision_receipt_retention_readback_ack_decision_visibility",
    "runtime_terminal_decision_receipt_retention_readback_ack_summary_visibility",
    "external_delivery_terminal_decision_receipt_retention_readback_ack_echo"
  ];
  def receipt_ids: [
    "operator_terminal_decision_receipt_retention_readback_ack_non_promotion_receipt",
    "release_owner_terminal_decision_receipt_retention_readback_ack_non_promotion_receipt",
    "authority_denial_terminal_decision_receipt_retention_readback_ack_receipt",
    "rollout_denial_terminal_decision_receipt_retention_readback_ack_receipt",
    "release_publication_denial_terminal_decision_receipt_retention_readback_ack_receipt",
    "external_delivery_denial_terminal_decision_receipt_retention_readback_ack_receipt"
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
  def with_durable_fields($fields): durable_fields + $fields;
  def receipt($id; $hash_mode): {
    id: $id,
    source_terminal_decision_surface_ids: surface_ids,
    receipt_hash_mode: $hash_mode,
    required_fields: with_durable_fields([
      "receiptId",
      "sourceTerminalDecisionSurfaceIds",
      "receiptHashMode",
      "receiptHash",
      "redactionHash"
    ]),
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
  def invariant($id; $reason): { id: $id, required: true, reason: $reason };
  def side_effects_false: {
    filesystem_written: false,
    graph_state_persisted: false,
    terminal_decision_recorded: false,
    terminal_decision_receipt_recorded: false,
    terminal_decision_receipt_persisted: false,
    terminal_decision_receipt_retention_state_persisted: false,
    readback_receipt_persisted: false,
    readback_acknowledgement_recorded: false,
    readback_acknowledgement_replay_recorded: false,
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
  };
  [
    receipt("operator_terminal_decision_receipt_retention_readback_ack_non_promotion_receipt"; "hash_only_operator_terminal_decision_receipt_retention_readback_ack_receipt"),
    receipt("release_owner_terminal_decision_receipt_retention_readback_ack_non_promotion_receipt"; "hash_only_release_owner_terminal_decision_receipt_retention_readback_ack_receipt"),
    receipt("authority_denial_terminal_decision_receipt_retention_readback_ack_receipt"; "hash_only_authority_denial_terminal_decision_receipt_retention_readback_ack_receipt"),
    receipt("rollout_denial_terminal_decision_receipt_retention_readback_ack_receipt"; "hash_only_rollout_denial_terminal_decision_receipt_retention_readback_ack_receipt"),
    receipt("release_publication_denial_terminal_decision_receipt_retention_readback_ack_receipt"; "hash_only_release_publication_denial_terminal_decision_receipt_retention_readback_ack_receipt"),
    receipt("external_delivery_denial_terminal_decision_receipt_retention_readback_ack_receipt"; "hash_only_external_delivery_denial_terminal_decision_receipt_retention_readback_ack_receipt")
  ] as $receipts
  | [
    digest_check("terminal_decision_receipt_retention_readback_ack_decision_surface_digest_matches"; with_durable_fields(["terminalDecisionSurfaceId", "terminalDecisionHash", "sourceGateDigest"])),
    digest_check("terminal_decision_receipt_retention_readback_ack_non_promotion_denial_digest_matches"; with_durable_fields(["nonPromotionDenialId", "denialHash", "zeroPromotionHash"])),
    digest_check("terminal_decision_receipt_retention_readback_ack_authority_guard_digest_matches"; with_durable_fields(["authorityGuardId", "authorityGuardHash", "authorityGranted"])),
    digest_check("terminal_decision_receipt_retention_readback_ack_release_delivery_guard_digest_matches"; with_durable_fields(["releaseDeliveryGuardId", "releaseHash", "deliveryHash"])),
    digest_check("terminal_decision_receipt_retention_readback_ack_local_view_digest_matches"; with_durable_fields(["localViewId", "localViewHash", "externalDeliveryEnabled"])),
    digest_check("terminal_decision_receipt_retention_readback_ack_zero_side_effect_digest_matches"; with_durable_fields(["zeroWriteHash", "zeroTrafficHash", "zeroExternalSendHash"]))
  ] as $digest_checks
  | [
    mismatch_denial("durable_identity_evidence_missing"; "terminal decision receipt retention readback acknowledgement terminal decision receipt cannot proceed without durable identity evidence"),
    mismatch_denial("missing_terminal_decision_receipt_retention_readback_ack_surface_cannot_record_receipt"; "missing terminal decision receipt retention readback acknowledgement decision surface cannot record receipt"),
    mismatch_denial("mismatched_terminal_decision_receipt_retention_readback_ack_hash_cannot_accept"; "mismatched terminal decision receipt retention readback acknowledgement decision hash cannot become acceptance"),
    mismatch_denial("stale_term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_replay_digest_cannot_grant_authority"; "stale terminal decision receipt retention readback acknowledgement replay digest cannot grant authority"),
    mismatch_denial("authority_guard_absence_after_terminal_decision_receipt_retention_readback_ack_cannot_start_rollout"; "absent authority guard after terminal decision receipt retention readback acknowledgement cannot start rollout"),
    mismatch_denial("release_delivery_guard_absence_after_terminal_decision_receipt_retention_readback_ack_cannot_publish"; "release and delivery guard absence after terminal decision receipt retention readback acknowledgement cannot publish release state"),
    mismatch_denial("external_delivery_terminal_decision_receipt_retention_readback_ack_receipt_cannot_send"; "external delivery receipt echo cannot send externally"),
    mismatch_denial("terminal_decision_receipt_retention_readback_ack_receipt_is_not_live_completion"; "terminal decision receipt retention readback acknowledgement receipt cannot claim live persistence completion")
  ] as $mismatch_denials
  | [
    receipt_guard("terminal_decision_receipt_retention_readback_ack_receipt_is_hash_only"; with_durable_fields(["receiptId", "receiptHash", "redactionHash"])),
    receipt_guard("terminal_decision_receipt_retention_readback_ack_receipt_is_non_persistent"; with_durable_fields(["receiptPersisted", "receiptStorageScope", "receiptWriteHash"])),
    receipt_guard("terminal_decision_receipt_retention_readback_ack_receipt_is_non_accepting"; with_durable_fields(["acceptanceAllowed", "approvalRecorded", "authorityGranted"])),
    receipt_guard("terminal_decision_receipt_retention_readback_ack_receipt_keeps_release_denied"; with_durable_fields(["releasePublished", "publicClaimRecorded", "artifactPublished"])),
    receipt_guard("terminal_decision_receipt_retention_readback_ack_receipt_keeps_external_delivery_denied"; with_durable_fields(["externalDeliveryEnabled", "destinationPolicy", "externalSendPerformed"]))
  ] as $receipt_guards
  | [
    view("operator_terminal_decision_receipt_retention_readback_ack_non_promotion_receipt_view"; "operator"; with_durable_fields(["receiptId", "receiptHash", "acceptanceAllowed", "nextGate"])),
    view("release_owner_terminal_decision_receipt_retention_readback_ack_non_promotion_receipt_view"; "release_owner"; with_durable_fields(["releasePublished", "publicClaimRecorded", "receiptPersisted", "externalDeliveryDenied"])),
    view("auditor_terminal_decision_receipt_retention_readback_ack_receipt_digest_view"; "auditor"; with_durable_fields(["sourceGateDigest", "receiptHash", "digestCheckId", "mismatchDenialId"])),
    view("runtime_terminal_decision_receipt_retention_readback_ack_receipt_zero_effect_view"; "system"; with_durable_fields(["terminalDecisionReceiptRecorded", "authorityGranted", "trafficRouted", "externalSendPerformed"]))
  ] as $local_views
  | [
    invariant("terminal_receipt_retention_readback_ack_terminal_decision_receipts_require_durable_identity_evidence"; "terminal decision receipt retention readback acknowledgement terminal decision receipts require workflow, run, step, checkpoint, replay, rollback, and receipt evidence"),
    invariant("terminal_decision_receipt_retention_readback_ack_receipts_are_hash_only"; "terminal decision receipt retention readback acknowledgement receipts expose hashes only"),
    invariant("terminal_decision_receipt_retention_readback_ack_receipts_are_not_recorded"; "terminal decision receipt retention readback acknowledgement cannot record receipt state"),
    invariant("terminal_decision_receipt_retention_readback_ack_receipts_are_not_acceptance"; "terminal decision receipt visibility cannot become acceptance or authority"),
    invariant("terminal_decision_receipt_retention_readback_ack_receipts_keep_release_denied"; "release publication, public claim, rollout, and traffic routing remain denied"),
    invariant("terminal_decision_receipt_retention_readback_ack_receipt_views_are_local_only"; "operator, release-owner, auditor, and runtime receipt views cannot be sent externally"),
    invariant("terminal_decision_receipt_retention_readback_ack_receipt_preview_has_no_side_effects"; "this gate cannot persist receipts, record approval, grant authority, publish, or send externally")
  ] as $invariants
  | {
    product: "Hepta",
    runtime: "hepta",
    status: "ready",
    gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_gate",
    schema_version: "work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_v1",
    preview_mode: "read_only_terminal_decision_receipt_retention_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_hash_only_no_recording",
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
    durable_identity_evidence: {
      schema_version: "work_graph_durable_identity_preview_v1",
      required_prior_gate: "hepta_work_graph_durable_identity_preview_gate",
      required_field_ids: durable_fields,
      required_for_receipt_ids: receipt_ids,
      durable_field_count: 7,
      preview_binding_count: 5,
      invariant_count: 7,
      currently_satisfied: false
    },
    invariants: $invariants,
    recommended_next_gate: "hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview_gate",
    ready_for_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_acknowledgement_preview: true,
    ready_for_operator_acceptance: false,
    ready_for_live_persistence: false,
    side_effects: side_effects_false,
    source_probes: {
      term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt: {
        rust_module_present: $rust_module_present,
        report_script_present: $report_script_present,
        gate_script_present: $gate_script_present
      },
      term_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision_receipt_retention_readback_ack_terminal_decision: {
        report_script_present: $prior_report_script_present,
        gate_script_present: $prior_gate_script_present
      },
      durable_identity: {
        report_script_present: $durable_identity_report_script_present,
        gate_script_present: $durable_identity_gate_script_present
      }
    }
  }'
