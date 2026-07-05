#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-cancel-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable terminal public claim delivery receipt artifact signing receipt cancellation/supersession readback report: $SOURCE_REPORT" >&2
  exit 1
}

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_readback"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_readback_blocked == true
  and .readback_blocker_count == 154
  and .signing_receipt_cancellation_recorded == false
  and .signing_receipt_supersession_recorded == false
  and .operator_approval_from_signing_receipt_cancellation_supersession_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_readback_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_readback_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_readback_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_readback_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_final_index_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_final_index_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_readback_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_ordering_monotonicity_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_ordering_monotonicity_final_index_attached,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_ordering_monotonicity_denial_gate_invoked: false,
    signing_receipt_cancellation_supersession_recorded: false,
    signing_receipt_cancellation_recorded: false,
    signing_receipt_withdrawal_recorded: false,
    signing_receipt_supersession_recorded: false,
    signing_receipt_replacement_receipt_recorded: false,
    signing_receipt_tombstone_recorded: false,
    signing_receipt_delete_marker_recorded: false,
    signing_receipt_latest_replacement_accepted: false,
    signing_receipt_ack_replacement_accepted: false,
    signing_receipt_cancelled_query_registered: false,
    signing_receipt_superseded_export_recorded: false,
    signing_receipt_replacement_observability_recorded: false,
    signing_receipt_lifecycle_cancellation_supersession_recorded: false,
    signing_receipt_result_from_cancellation_supersession_recorded: false,
    artifact_signing_receipt_cancellation_accepted: false,
    package_signing_receipt_cancellation_accepted: false,
    signature_manifest_receipt_cancellation_accepted: false,
    notarization_ticket_receipt_supersession_accepted: false,
    release_asset_receipt_supersession_accepted: false,
    cdn_update_feed_receipt_supersession_accepted: false,
    package_registry_receipt_supersession_accepted: false,
    external_signing_receipt_supersession_accepted: false,
    telegram_signing_receipt_supersession_accepted: false,
    operator_approval_from_signing_receipt_cancellation_supersession_derived: false,
    release_publication_authority_from_signing_receipt_cancellation_supersession_derived: false,
    activation_authority_from_signing_receipt_cancellation_supersession_derived: false,
    install_from_signing_receipt_cancellation_supersession_executed: false,
    service_restart_from_signing_receipt_cancellation_supersession_performed: false,
    active_binary_from_signing_receipt_cancellation_supersession_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    external_send_performed: false,
    telegram_send_performed: false,
    terminal_live_gates_invoked: false,
    final_blocker_count: 154,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_cancellation_supersession_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_audit_evidence_without_cancellation",
    side_effect_free: true,
    side_effects: ($source.side_effects + {
      final_index_report_written: false,
      signing_receipt_cancellation_supersession_final_index_recorded: false,
      signing_receipt_cancel_denial_gate_invoked: false
    })
  }'
