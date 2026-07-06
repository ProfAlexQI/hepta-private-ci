#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-final-index-delivery-receipt-signing-replay-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable terminal public claim delivery receipt artifact signing receipt replay/idempotency attachment report: $SOURCE_REPORT" >&2
  exit 1
}

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_attachment"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_attachment_blocked == true
  and .attachment_blocker_count == 122
  and .signing_receipt_replay_recorded == false
  and .signing_receipt_idempotency_key_recorded == false
  and .signing_receipt_status_upgrade_accepted == false
  and .external_signing_receipt_replay_accepted == false
  and .telegram_signing_receipt_replay_accepted == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_readback",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_attachment_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_attachment_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_attachment_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_attachment_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_attachment_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_readback_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_readback_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_attachment_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_non_persistence_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_non_persistence_final_index_attached,
    readback_mode: "static_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_snapshot_only",
    readback_check_count: 122,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_non_persistence_denial_gate_invoked: false,
    signing_receipt_replay_allowed: false,
    signing_receipt_replay_accepted: false,
    signing_receipt_replay_recorded: false,
    signing_receipt_replay_persisted: false,
    signing_receipt_replay_performed: false,
    signing_receipt_duplicate_accepted: false,
    signing_receipt_duplicate_recorded: false,
    signing_receipt_duplicate_persisted: false,
    signing_receipt_idempotency_key_accepted: false,
    signing_receipt_idempotency_key_recorded: false,
    signing_receipt_idempotency_state_recorded: false,
    signing_receipt_idempotency_state_persisted: false,
    signing_receipt_replay_nonce_accepted: false,
    signing_receipt_replay_nonce_recorded: false,
    signing_receipt_cross_scope_reuse_accepted: false,
    signing_receipt_status_upgrade_accepted: false,
    signing_receipt_ack_replay_accepted: false,
    signing_receipt_ledger_replay_accepted: false,
    signing_receipt_index_replay_accepted: false,
    signing_receipt_query_replay_accepted: false,
    signing_receipt_export_replay_accepted: false,
    signing_receipt_observability_replay_accepted: false,
    signing_receipt_hash_status_rebind_accepted: false,
    artifact_signing_receipt_replay_accepted: false,
    package_signing_receipt_replay_accepted: false,
    signature_manifest_receipt_replay_accepted: false,
    notarization_submission_receipt_replay_accepted: false,
    notarization_ticket_receipt_replay_accepted: false,
    stapling_receipt_replay_accepted: false,
    installer_signing_receipt_replay_accepted: false,
    release_asset_receipt_replay_accepted: false,
    cdn_update_feed_receipt_replay_accepted: false,
    package_registry_receipt_replay_accepted: false,
    external_signing_receipt_replay_accepted: false,
    telegram_signing_receipt_replay_accepted: false,
    operator_approval_from_signing_receipt_replay_derived: false,
    release_publication_authority_from_signing_receipt_replay_derived: false,
    activation_authority_from_signing_receipt_replay_derived: false,
    install_from_signing_receipt_replay_executed: false,
    service_restart_from_signing_receipt_replay_performed: false,
    active_binary_from_signing_receipt_replay_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    external_send_performed: false,
    telegram_send_performed: false,
    terminal_live_gates_invoked: false,
    readback_blocker_count: 122,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_replay_idempotency_final_index_without_receipt_replay",
    side_effect_free: true,
    side_effects: ($source.side_effects + {
      readback_report_written: false,
      signing_receipt_replay_idempotency_readback_recorded: false,
      signing_replay_denial_gate_invoked: false
    })
  }'
