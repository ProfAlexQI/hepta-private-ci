#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-final-index-report.sh"
AUDIT_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-audit-evidence-denial-gate.sh"
AUDIT_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_AUDIT_EVIDENCE_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable artifact signing terminal public claim delivery receipt cancellation/supersession final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$AUDIT_GATE" ]] || {
  echo "missing artifact signing terminal public claim delivery receipt audit evidence denial gate: $AUDIT_GATE" >&2
  exit 1
}
[[ -f "$AUDIT_DOC" ]] || {
  echo "missing artifact signing terminal public claim delivery receipt audit evidence denial doc: $AUDIT_DOC" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the artifact signing terminal public claim delivery receipt audit evidence attachment report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_final_index"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_final_index_blocked == true
  and .terminal_public_claim_delivery_receipt_cancellation_supersession_recorded == false
  and .terminal_public_claim_delivery_receipt_replacement_receipt_recorded == false
  and .terminal_public_claim_delivery_receipt_tombstone_recorded == false
  and .terminal_public_claim_delivery_receipt_result_from_cancellation_supersession_recorded == false
  and .operator_approval_from_delivery_receipt_cancellation_supersession_derived == false
  and .public_ga_claimed == false
' <<<"$source_json" >/dev/null

audit_static_mention_count="$(
  grep -Eci 'audit|evidence|immutable|hash|merkle|attestation|witness|notary|ledger|index|delivery|query|export|observability|readback|status|authority|install|restart|active-binary|telegram|external|credential|secret|provider|model|live' "$AUDIT_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson audit_static_mention_count "$audit_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_final_index_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_final_index_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_final_index_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_final_index_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_final_index_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_attachment_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_attachment_blocked: true,
    artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denial_gate_present: true,
    artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denial_doc_present: true,
    artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_static_mention_count: $audit_static_mention_count,
    artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_gate_invoked: false,
    long_soak_required_by_source_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_gate: true,
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
    public_claim_delivery_receipt_audit_evidence_recorded: false,
    status_readback_delivery_receipt_audit_evidence_recorded: false,
    channel_delivery_receipt_audit_evidence_delivered: false,
    telegram_delivery_receipt_audit_evidence_delivered: false,
    external_delivery_receipt_audit_evidence_delivered: false,
    readback_receipt_backfill_audit_evidence_recorded: false,
    operator_approval_from_delivery_receipt_audit_evidence_derived: false,
    release_publication_authority_from_delivery_receipt_audit_evidence_derived: false,
    activation_authority_from_delivery_receipt_audit_evidence_derived: false,
    download_link_from_delivery_receipt_audit_evidence_rendered: false,
    install_command_from_delivery_receipt_audit_evidence_emitted: false,
    install_from_delivery_receipt_audit_evidence_executed: false,
    service_restart_from_delivery_receipt_audit_evidence_performed: false,
    active_binary_from_delivery_receipt_audit_evidence_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    attachment_blocker_count: 100,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_readback_without_audit_evidence",
    local_gate: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-final-index-artifact-signing-terminal-public-claim-delivery-receipt-audit-evidence-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_CANCELLATION_SUPERSESSION_FINAL_INDEX_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_AUDIT_EVIDENCE_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_final_index_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-final-index-report.sh",
      artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denial_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-audit-evidence-denial-gate.sh",
      artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_AUDIT_EVIDENCE_DENIAL_GATE.md"
    },
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denial_gate_invoked: false,
      artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_gate_invoked: false,
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
      operator_approval_from_delivery_receipt_audit_evidence_derived: false,
      release_publication_authority_from_delivery_receipt_audit_evidence_derived: false,
      activation_authority_from_delivery_receipt_audit_evidence_derived: false,
      download_link_from_delivery_receipt_audit_evidence_rendered: false,
      install_command_from_delivery_receipt_audit_evidence_emitted: false,
      install_from_delivery_receipt_audit_evidence_executed: false,
      service_restart_from_delivery_receipt_audit_evidence_performed: false,
      active_binary_from_delivery_receipt_audit_evidence_mutated: false,
      memory_store_write_performed: false,
      live_kg_write_performed: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      secret_file_read: false,
      telegram_send_performed: false,
      external_send_performed: false,
      long_soak_started: false,
      terminal_live_gate_invoked: false,
      terminal_live_url_contacted: false,
      public_ga_readiness_script_invoked: false,
      public_claim_non_promotion_denial_gate_invoked: false,
      public_ga_claim_recorded: false,
      public_ga_promoted: false,
      public_release_published: false,
      rollback_executed: false,
      external_network_read: false,
      release_artifact_written: false,
      public_artifact_written: false,
      filesystem_written: false
    }
  }'
