#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-non-persistence-final-index-artifact-signing-terminal-public-claim-delivery-receipt-replay-idempotency-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable artifact signing terminal public claim delivery receipt replay/idempotency attachment report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the artifact signing terminal public claim delivery receipt replay/idempotency static readback report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_attachment"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_attachment_blocked == true
  and .terminal_public_claim_delivery_receipt_replay_recorded == false
  and .terminal_public_claim_delivery_receipt_idempotency_key_recorded == false
  and .terminal_public_claim_delivery_receipt_status_upgrade_accepted == false
  and .operator_approval_from_delivery_receipt_replay_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_readback",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_attachment_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_attachment_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_attachment_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_attachment_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_attachment_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_readback_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_readback_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_attachment_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_final_index_attached,
    readback_mode: "static_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_snapshot_only",
    readback_check_count: 94,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_denial_gate_present: $source.operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_denial_gate_present,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_denial_doc_present: $source.operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_denial_doc_present,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_denial_gate_invoked: false,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_denial_gate_invoked: false,
    terminal_public_claim_delivery_receipt_replay_allowed: false,
    terminal_public_claim_delivery_receipt_replay_accepted: false,
    terminal_public_claim_delivery_receipt_replay_recorded: false,
    terminal_public_claim_delivery_receipt_replay_persisted: false,
    terminal_public_claim_delivery_receipt_replay_performed: false,
    terminal_public_claim_delivery_receipt_duplicate_accepted: false,
    terminal_public_claim_delivery_receipt_duplicate_recorded: false,
    terminal_public_claim_delivery_receipt_duplicate_persisted: false,
    terminal_public_claim_delivery_receipt_idempotency_key_accepted: false,
    terminal_public_claim_delivery_receipt_idempotency_key_recorded: false,
    terminal_public_claim_delivery_receipt_idempotency_state_recorded: false,
    terminal_public_claim_delivery_receipt_idempotency_state_persisted: false,
    terminal_public_claim_delivery_receipt_idempotency_state_materialized: false,
    terminal_public_claim_delivery_receipt_replay_nonce_accepted: false,
    terminal_public_claim_delivery_receipt_replay_nonce_recorded: false,
    terminal_public_claim_delivery_receipt_cross_scope_reuse_accepted: false,
    terminal_public_claim_delivery_receipt_status_upgrade_accepted: false,
    terminal_public_claim_delivery_receipt_completed_status_accepted: false,
    terminal_public_claim_delivery_receipt_ack_replay_accepted: false,
    terminal_public_claim_delivery_receipt_ledger_replay_accepted: false,
    terminal_public_claim_delivery_receipt_index_replay_accepted: false,
    terminal_public_claim_delivery_receipt_delivery_replay_accepted: false,
    terminal_public_claim_delivery_receipt_query_replay_accepted: false,
    terminal_public_claim_delivery_receipt_export_replay_accepted: false,
    terminal_public_claim_delivery_receipt_observability_replay_accepted: false,
    terminal_public_claim_delivery_receipt_hash_status_rebind_accepted: false,
    external_delivery_receipt_replay_accepted: false,
    telegram_delivery_receipt_replay_accepted: false,
    readback_receipt_backfill_replay_accepted: false,
    operator_approval_from_delivery_receipt_replay_derived: false,
    release_publication_authority_from_delivery_receipt_replay_derived: false,
    activation_authority_from_delivery_receipt_replay_derived: false,
    download_link_from_delivery_receipt_replay_rendered: false,
    install_command_from_delivery_receipt_replay_emitted: false,
    install_from_delivery_receipt_replay_executed: false,
    service_restart_from_delivery_receipt_replay_performed: false,
    active_binary_from_delivery_receipt_replay_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    terminal_live_gates_invoked: false,
    readback_blocker_count: 94,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_final_index_without_receipt_replay",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-replay-idempotency-readback-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_REPLAY_IDEMPOTENCY_READBACK_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_attachment_report: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-non-persistence-final-index-artifact-signing-terminal-public-claim-delivery-receipt-replay-idempotency-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
