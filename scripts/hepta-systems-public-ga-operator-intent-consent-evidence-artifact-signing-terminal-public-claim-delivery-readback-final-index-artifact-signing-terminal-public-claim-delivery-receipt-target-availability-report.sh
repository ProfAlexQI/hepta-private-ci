#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-readback-final-index-report.sh"
TARGET_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-non-persistence-denial-gate.sh"
TARGET_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_NON_PERSISTENCE_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable artifact signing terminal public claim delivery/readback final index report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the artifact signing terminal public claim delivery receipt target availability report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"
target_gate_present=false
target_doc_present=false
[[ -x "$TARGET_GATE" ]] && target_gate_present=true
[[ -f "$TARGET_DOC" ]] && target_doc_present=true

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index_blocked == true
  and .public_claim_delivery_recorded == false
  and .status_readback_recorded == false
  and .delivery_receipt_recorded == false
  and .operator_approval_from_delivery_readback_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  --argjson target_gate_present "$target_gate_present" \
  --argjson target_doc_present "$target_doc_present" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_target_availability",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_target_availability_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_target_availability_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index_attached: true,
    target_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_denial_gate_present: $target_gate_present,
    target_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_denial_doc_present: $target_doc_present,
    target_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_denial_gate_invoked: false,
    target_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_creation_required: (($target_gate_present and $target_doc_present) | not),
    artifact_signing_terminal_public_claim_delivery_receipt_attachment_allowed: ($target_gate_present and $target_doc_present),
    artifact_signing_terminal_public_claim_delivery_receipt_target_created_after_preflight: ($target_gate_present and $target_doc_present),
    artifact_signing_terminal_public_claim_delivery_receipt_attachment_performed: false,
    terminal_public_claim_delivery_readback_final_index_consumed: true,
    terminal_public_claim_delivery_readback_gate_invoked: false,
    public_claim_delivery_recorded: false,
    public_claim_delivery_persisted: false,
    status_readback_recorded: false,
    status_readback_persisted: false,
    channel_delivery_recorded: false,
    channel_delivery_persisted: false,
    external_delivery_readback_sent: false,
    telegram_delivery_readback_sent: false,
    delivery_receipt_recorded: false,
    delivery_receipt_persisted: false,
    readback_receipt_recorded: false,
    readback_receipt_persisted: false,
    delivery_receipt_materialized: false,
    delivery_receipt_filesystem_written: false,
    delivery_receipt_ledger_written: false,
    delivery_receipt_index_written: false,
    delivery_receipt_query_registered: false,
    delivery_receipt_exported: false,
    delivery_receipt_observability_recorded: false,
    delivery_receipt_status_exposed: false,
    delivery_receipt_acknowledgement_accepted: false,
    operator_approval_from_delivery_receipt_derived: false,
    release_publication_authority_from_delivery_receipt_derived: false,
    activation_authority_from_delivery_receipt_derived: false,
    download_link_from_delivery_receipt_rendered: false,
    install_command_from_delivery_receipt_emitted: false,
    install_from_delivery_receipt_executed: false,
    service_restart_from_delivery_receipt_performed: false,
    active_binary_from_delivery_receipt_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    terminal_live_gates_invoked: false,
    final_blocker_count: 91,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_without_delivery",
    local_gate: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-readback-final-index-artifact-signing-terminal-public-claim-delivery-receipt-target-availability-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_READBACK_FINAL_INDEX_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_TARGET_AVAILABILITY_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-readback-final-index-report.sh",
      target_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_denial_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-non-persistence-denial-gate.sh",
      target_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_NON_PERSISTENCE_DENIAL_GATE.md"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects + {
      target_delivery_receipt_denial_gate_invocation: false,
      delivery_receipt_write: false,
      delivery_receipt_persistence: false,
      delivery_receipt_materialization: false,
      delivery_receipt_ledger_write: false,
      delivery_receipt_index_write: false,
      delivery_receipt_query_registration: false,
      delivery_receipt_export: false,
      delivery_receipt_observability: false
    })
  }'
