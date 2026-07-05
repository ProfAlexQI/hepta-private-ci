#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-replay-idempotency-final-index-artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable artifact signing terminal public claim delivery receipt ordering/monotonicity attachment report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the artifact signing terminal public claim delivery receipt ordering/monotonicity readback report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_attachment"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_attachment_blocked == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_gate_invoked == false
  and .terminal_public_claim_delivery_receipt_ordering_monotonicity_recorded == false
  and .terminal_public_claim_delivery_receipt_sequence_cursor_recorded == false
  and .terminal_public_claim_delivery_receipt_monotonicity_state_recorded == false
  and .operator_approval_from_delivery_receipt_ordering_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_readback",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_attachment_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_attachment_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_attachment_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_attachment_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_attachment_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_readback_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_readback_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_attachment_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_final_index_attached,
    artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_denial_gate_invoked: false,
    readback_check_count: 96,
    long_soak_started: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    terminal_public_claim_delivery_receipt_ordering_monotonicity_allowed: false,
    terminal_public_claim_delivery_receipt_ordering_monotonicity_accepted: false,
    terminal_public_claim_delivery_receipt_ordering_monotonicity_recorded: false,
    terminal_public_claim_delivery_receipt_ordering_monotonicity_persisted: false,
    terminal_public_claim_delivery_receipt_ordering_monotonicity_materialized: false,
    terminal_public_claim_delivery_receipt_ordering_monotonicity_filesystem_written: false,
    terminal_public_claim_delivery_receipt_sequence_cursor_recorded: false,
    terminal_public_claim_delivery_receipt_sequence_cursor_persisted: false,
    terminal_public_claim_delivery_receipt_monotonicity_state_recorded: false,
    terminal_public_claim_delivery_receipt_monotonicity_state_persisted: false,
    terminal_public_claim_delivery_receipt_monotonicity_state_materialized: false,
    terminal_public_claim_delivery_receipt_duplicate_sequence_accepted: false,
    terminal_public_claim_delivery_receipt_stale_sequence_accepted: false,
    terminal_public_claim_delivery_receipt_late_arrival_accepted: false,
    terminal_public_claim_delivery_receipt_future_gap_accepted: false,
    terminal_public_claim_delivery_receipt_timestamp_rollback_accepted: false,
    terminal_public_claim_delivery_receipt_epoch_rollback_accepted: false,
    terminal_public_claim_delivery_receipt_same_sequence_different_hash_accepted: false,
    terminal_public_claim_delivery_receipt_latest_wins_overwrite_accepted: false,
    terminal_public_claim_delivery_receipt_ordered_status_accepted: false,
    terminal_public_claim_delivery_receipt_ordered_acknowledgement_accepted: false,
    terminal_public_claim_delivery_receipt_ordered_ledger_index_accepted: false,
    terminal_public_claim_delivery_receipt_ordered_query_export_accepted: false,
    terminal_public_claim_delivery_receipt_ordered_observability_accepted: false,
    terminal_public_claim_delivery_receipt_ordered_hash_status_accepted: false,
    public_claim_delivery_receipt_ordering_accepted: false,
    status_readback_delivery_receipt_ordering_accepted: false,
    channel_delivery_receipt_ordering_accepted: false,
    telegram_delivery_receipt_ordering_accepted: false,
    external_delivery_receipt_ordering_accepted: false,
    readback_receipt_backfill_ordering_accepted: false,
    operator_approval_from_delivery_receipt_ordering_derived: false,
    release_publication_authority_from_delivery_receipt_ordering_derived: false,
    activation_authority_from_delivery_receipt_ordering_derived: false,
    download_link_from_delivery_receipt_ordering_rendered: false,
    install_command_from_delivery_receipt_ordering_emitted: false,
    install_from_delivery_receipt_ordering_executed: false,
    service_restart_from_delivery_receipt_ordering_performed: false,
    active_binary_from_delivery_receipt_ordering_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_final_index_without_receipt_ordering",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity-readback-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_ORDERING_MONOTONICITY_READBACK_2026-06-21.md",
    source_files: {
      artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_attachment_report: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-replay-idempotency-final-index-artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
