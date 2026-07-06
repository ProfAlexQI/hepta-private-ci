#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-replay-final-index-delivery-receipt-signing-ordering-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable terminal public claim delivery receipt artifact signing receipt ordering/monotonicity attachment report: $SOURCE_REPORT" >&2
  exit 1
}

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_attachment"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_attachment_blocked == true
  and .attachment_blocker_count == 124
  and .signing_receipt_ordering_recorded == false
  and .signing_receipt_sequence_cursor_recorded == false
  and .signing_receipt_ordered_status_accepted == false
  and .external_signing_receipt_ordering_accepted == false
  and .telegram_signing_receipt_ordering_accepted == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_readback",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_attachment_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_attachment_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_attachment_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_attachment_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_attachment_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_readback_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_readback_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_attachment_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_final_index_attached,
    readback_mode: "static_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_snapshot_only",
    readback_check_count: 124,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_denial_gate_invoked: false,
    signing_receipt_ordering_recorded: false,
    signing_receipt_ordering_persisted: false,
    signing_receipt_sequence_cursor_recorded: false,
    signing_receipt_sequence_cursor_persisted: false,
    signing_receipt_monotonicity_state_recorded: false,
    signing_receipt_monotonicity_state_persisted: false,
    signing_receipt_latest_wins_state_recorded: false,
    signing_receipt_latest_wins_overwrite_accepted: false,
    signing_receipt_ordered_status_accepted: false,
    signing_receipt_ordered_ack_accepted: false,
    signing_receipt_duplicate_order_accepted: false,
    signing_receipt_out_of_order_replay_accepted: false,
    signing_receipt_stale_sequence_accepted: false,
    signing_receipt_cross_scope_order_accepted: false,
    signing_receipt_hash_sequence_rebind_accepted: false,
    artifact_signing_receipt_ordering_accepted: false,
    package_signing_receipt_ordering_accepted: false,
    signature_manifest_receipt_ordering_accepted: false,
    notarization_submission_receipt_ordering_accepted: false,
    notarization_ticket_receipt_ordering_accepted: false,
    stapling_receipt_ordering_accepted: false,
    installer_signing_receipt_ordering_accepted: false,
    release_asset_receipt_ordering_accepted: false,
    cdn_update_feed_receipt_ordering_accepted: false,
    package_registry_receipt_ordering_accepted: false,
    external_signing_receipt_ordering_accepted: false,
    telegram_signing_receipt_ordering_accepted: false,
    operator_approval_from_signing_receipt_ordering_derived: false,
    release_publication_authority_from_signing_receipt_ordering_derived: false,
    activation_authority_from_signing_receipt_ordering_derived: false,
    install_from_signing_receipt_ordering_executed: false,
    service_restart_from_signing_receipt_ordering_performed: false,
    active_binary_from_signing_receipt_ordering_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    external_send_performed: false,
    telegram_send_performed: false,
    terminal_live_gates_invoked: false,
    readback_blocker_count: 124,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_ordering_monotonicity_final_index_without_receipt_ordering",
    side_effect_free: true,
    side_effects: ($source.side_effects + {
      readback_report_written: false,
      signing_receipt_ordering_monotonicity_readback_recorded: false,
      signing_ordering_denial_gate_invoked: false
    })
  }'
