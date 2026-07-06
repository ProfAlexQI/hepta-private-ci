#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-retention-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable terminal public claim delivery receipt artifact signing receipt retention/expiry/GC readback report: $SOURCE_REPORT" >&2
  exit 1
}

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_readback"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_readback_blocked == true
  and .readback_blocker_count == 130
  and .signing_receipt_retention_policy_recorded == false
  and .signing_receipt_garbage_collection_executed == false
  and .operator_approval_from_signing_receipt_retention_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_readback_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_readback_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_readback_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_readback_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_final_index_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_final_index_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_readback_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_audit_evidence_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_audit_evidence_final_index_attached,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_audit_evidence_denial_gate_invoked: false,
    signing_receipt_retention_expiry_gc_recorded: false,
    signing_receipt_retention_policy_recorded: false,
    signing_receipt_ttl_lease_recorded: false,
    signing_receipt_expiry_timestamp_recorded: false,
    signing_receipt_expiry_scheduler_recorded: false,
    signing_receipt_expiry_timer_started: false,
    signing_receipt_expiry_ack_recorded: false,
    signing_receipt_garbage_collection_queue_recorded: false,
    signing_receipt_garbage_collection_scan_performed: false,
    signing_receipt_garbage_collection_decision_recorded: false,
    signing_receipt_garbage_collection_executed: false,
    signing_receipt_tombstone_gc_recorded: false,
    signing_receipt_delete_marker_gc_recorded: false,
    signing_receipt_archive_recorded: false,
    signing_receipt_compaction_recorded: false,
    signing_receipt_compaction_performed: false,
    external_signing_receipt_retention_recorded: false,
    telegram_signing_receipt_retention_recorded: false,
    operator_approval_from_signing_receipt_retention_derived: false,
    release_publication_authority_from_signing_receipt_retention_derived: false,
    activation_authority_from_signing_receipt_retention_derived: false,
    install_from_signing_receipt_retention_executed: false,
    service_restart_from_signing_receipt_retention_performed: false,
    active_binary_from_signing_receipt_retention_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    final_blocker_count: 130,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_retention_expiry_gc_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_export_query_observability_without_retention",
    side_effect_free: true,
    side_effects: ($source.side_effects + {
      final_index_report_written: false,
      signing_receipt_retention_expiry_gc_final_index_recorded: false,
      signing_retention_denial_gate_invoked: false
    })
  }'
