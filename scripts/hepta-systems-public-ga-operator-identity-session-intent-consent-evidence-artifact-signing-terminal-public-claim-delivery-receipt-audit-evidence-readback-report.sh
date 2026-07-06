#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-final-index-artifact-signing-terminal-public-claim-delivery-receipt-audit-evidence-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable artifact signing terminal public claim delivery receipt audit evidence attachment report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the artifact signing terminal public claim delivery receipt audit evidence readback report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_attachment"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_attachment_blocked == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denial_gate_invoked == false
  and .terminal_public_claim_delivery_receipt_audit_evidence_recorded == false
  and .terminal_public_claim_delivery_receipt_hash_chain_recorded == false
  and .terminal_public_claim_delivery_receipt_attestation_recorded == false
  and .operator_approval_from_delivery_receipt_audit_evidence_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_readback",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_attachment_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_attachment_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_attachment_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_attachment_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_attachment_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_readback_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_readback_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_attachment_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_final_index_attached,
    artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_gate_invoked: false,
    readback_check_count: 100,
    long_soak_started: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    terminal_public_claim_delivery_receipt_audit_evidence_allowed: false,
    terminal_public_claim_delivery_receipt_audit_evidence_accepted: false,
    terminal_public_claim_delivery_receipt_audit_evidence_recorded: false,
    terminal_public_claim_delivery_receipt_audit_evidence_persisted: false,
    terminal_public_claim_delivery_receipt_audit_evidence_materialized: false,
    terminal_public_claim_delivery_receipt_audit_evidence_filesystem_written: false,
    terminal_public_claim_delivery_receipt_audit_trail_recorded: false,
    terminal_public_claim_delivery_receipt_immutable_evidence_recorded: false,
    terminal_public_claim_delivery_receipt_hash_chain_recorded: false,
    terminal_public_claim_delivery_receipt_merkle_root_recorded: false,
    terminal_public_claim_delivery_receipt_attestation_recorded: false,
    terminal_public_claim_delivery_receipt_witness_recorded: false,
    terminal_public_claim_delivery_receipt_notary_recorded: false,
    terminal_public_claim_delivery_receipt_ledger_recorded: false,
    terminal_public_claim_delivery_receipt_index_recorded: false,
    terminal_public_claim_delivery_receipt_delivery_evidence_recorded: false,
    terminal_public_claim_delivery_receipt_query_export_evidence_recorded: false,
    terminal_public_claim_delivery_receipt_observability_evidence_recorded: false,
    terminal_public_claim_delivery_receipt_readback_evidence_recorded: false,
    terminal_public_claim_delivery_receipt_status_evidence_recorded: false,
    terminal_public_claim_delivery_receipt_hash_status_evidence_recorded: false,
    external_delivery_receipt_audit_evidence_delivered: false,
    telegram_delivery_receipt_audit_evidence_delivered: false,
    readback_receipt_backfill_audit_evidence_recorded: false,
    operator_approval_from_delivery_receipt_audit_evidence_derived: false,
    release_publication_authority_from_delivery_receipt_audit_evidence_derived: false,
    activation_authority_from_delivery_receipt_audit_evidence_derived: false,
    download_link_from_delivery_receipt_audit_evidence_rendered: false,
    install_command_from_delivery_receipt_audit_evidence_emitted: false,
    install_from_delivery_receipt_audit_evidence_executed: false,
    service_restart_from_delivery_receipt_audit_evidence_performed: false,
    active_binary_from_delivery_receipt_audit_evidence_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_final_index_without_audit_evidence",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-audit-evidence-readback-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_AUDIT_EVIDENCE_READBACK_2026-06-21.md",
    source_files: {
      artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_attachment_report: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-final-index-artifact-signing-terminal-public-claim-delivery-receipt-audit-evidence-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
