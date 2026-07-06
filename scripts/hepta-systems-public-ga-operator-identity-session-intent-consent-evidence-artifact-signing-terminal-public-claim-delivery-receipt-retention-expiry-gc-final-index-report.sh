#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-retention-expiry-gc-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable terminal public claim delivery receipt retention/expiry/GC readback report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the terminal public claim delivery receipt retention/expiry/GC final index report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_readback"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_readback_blocked == true
  and .retention_policy_recorded == false
  and .expiry_timer_started == false
  and .garbage_collection_executed == false
  and .delivery_receipt_retention_authority_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_readback_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_readback_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_readback_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_readback_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_final_index_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_final_index_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_readback_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_final_index_attached,
    artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denial_gate_invoked: false,
    long_soak_started: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_accepted: false,
    terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_recorded: false,
    terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_persisted: false,
    terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_materialized: false,
    terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_filesystem_written: false,
    retention_policy_recorded: false,
    retention_policy_persisted: false,
    ttl_lease_recorded: false,
    ttl_lease_persisted: false,
    expiry_timestamp_recorded: false,
    expiry_scheduler_recorded: false,
    expiry_timer_started: false,
    expiry_ack_recorded: false,
    expiry_state_persisted: false,
    garbage_collection_queue_recorded: false,
    garbage_collection_scan_performed: false,
    garbage_collection_candidate_recorded: false,
    garbage_collection_decision_recorded: false,
    garbage_collection_executed: false,
    tombstone_gc_recorded: false,
    delete_marker_gc_recorded: false,
    archive_recorded: false,
    archive_persisted: false,
    compaction_recorded: false,
    compaction_performed: false,
    audit_evidence_retention_recorded: false,
    immutable_evidence_retention_recorded: false,
    hash_attestation_retention_recorded: false,
    witness_notary_expiry_recorded: false,
    ledger_index_retention_recorded: false,
    delivery_evidence_retention_recorded: false,
    status_evidence_expiry_recorded: false,
    external_telegram_retention_recorded: false,
    operator_approval_from_delivery_receipt_retention_derived: false,
    release_publication_authority_from_delivery_receipt_retention_derived: false,
    activation_authority_from_delivery_receipt_retention_derived: false,
    delivery_receipt_retention_authority_derived: false,
    download_link_from_delivery_receipt_retention_rendered: false,
    install_command_from_delivery_receipt_retention_rendered: false,
    install_from_delivery_receipt_retention_executed: false,
    service_restart_from_delivery_receipt_retention_performed: false,
    active_binary_from_delivery_receipt_retention_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    final_blocker_count: 102,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_without_retention",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-retention-expiry-gc-final-index-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_RETENTION_EXPIRY_GC_FINAL_INDEX_2026-06-21.md",
    source_files: {
      terminal_public_claim_delivery_receipt_retention_expiry_gc_readback_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-retention-expiry-gc-readback-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
