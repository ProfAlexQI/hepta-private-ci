#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable artifact signing terminal public claim delivery receipt cancellation/supersession readback report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the artifact signing terminal public claim delivery receipt cancellation/supersession final index report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_readback"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_readback_blocked == true
  and .terminal_public_claim_delivery_receipt_cancellation_supersession_recorded == false
  and .terminal_public_claim_delivery_receipt_replacement_receipt_recorded == false
  and .terminal_public_claim_delivery_receipt_tombstone_recorded == false
  and .operator_approval_from_delivery_receipt_cancellation_supersession_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_readback_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_readback_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_readback_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_readback_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_final_index_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_final_index_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_readback_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_final_index_attached,
    artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_gate_invoked: false,
    long_soak_started: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    terminal_public_claim_delivery_receipt_cancellation_supersession_allowed: false,
    terminal_public_claim_delivery_receipt_cancellation_supersession_accepted: false,
    terminal_public_claim_delivery_receipt_cancellation_supersession_recorded: false,
    terminal_public_claim_delivery_receipt_cancellation_supersession_persisted: false,
    terminal_public_claim_delivery_receipt_cancellation_supersession_materialized: false,
    terminal_public_claim_delivery_receipt_cancellation_supersession_filesystem_written: false,
    terminal_public_claim_delivery_receipt_cancellation_accepted: false,
    terminal_public_claim_delivery_receipt_cancellation_recorded: false,
    terminal_public_claim_delivery_receipt_withdrawal_accepted: false,
    terminal_public_claim_delivery_receipt_supersession_accepted: false,
    terminal_public_claim_delivery_receipt_supersession_recorded: false,
    terminal_public_claim_delivery_receipt_replacement_receipt_recorded: false,
    terminal_public_claim_delivery_receipt_tombstone_recorded: false,
    terminal_public_claim_delivery_receipt_delete_marker_recorded: false,
    terminal_public_claim_delivery_receipt_lifecycle_cancellation_supersession_recorded: false,
    terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_recorded: false,
    external_delivery_receipt_supersession_accepted: false,
    telegram_delivery_receipt_supersession_accepted: false,
    readback_receipt_backfill_cancellation_supersession_accepted: false,
    operator_approval_from_delivery_receipt_cancellation_supersession_derived: false,
    release_publication_authority_from_delivery_receipt_cancellation_supersession_derived: false,
    activation_authority_from_delivery_receipt_cancellation_supersession_derived: false,
    download_link_from_delivery_receipt_cancellation_supersession_rendered: false,
    install_command_from_delivery_receipt_cancellation_supersession_emitted: false,
    install_from_delivery_receipt_cancellation_supersession_executed: false,
    service_restart_from_delivery_receipt_cancellation_supersession_performed: false,
    active_binary_from_delivery_receipt_cancellation_supersession_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    final_blocker_count: 98,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_without_cancellation",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-final-index-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_CANCELLATION_SUPERSESSION_FINAL_INDEX_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_readback_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-readback-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
