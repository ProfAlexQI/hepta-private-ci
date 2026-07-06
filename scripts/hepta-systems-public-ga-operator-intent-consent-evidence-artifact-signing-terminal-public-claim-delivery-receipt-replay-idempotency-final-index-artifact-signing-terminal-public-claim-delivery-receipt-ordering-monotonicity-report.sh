#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-replay-idempotency-final-index-report.sh"
ORDERING_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity-denial-gate.sh"
ORDERING_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_ORDERING_MONOTONICITY_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable artifact signing terminal public claim delivery receipt replay/idempotency final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$ORDERING_GATE" ]] || {
  echo "missing artifact signing terminal public claim delivery receipt ordering/monotonicity denial gate: $ORDERING_GATE" >&2
  exit 1
}
[[ -f "$ORDERING_DOC" ]] || {
  echo "missing artifact signing terminal public claim delivery receipt ordering/monotonicity denial doc: $ORDERING_DOC" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the artifact signing terminal public claim delivery receipt ordering/monotonicity attachment report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_final_index"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_final_index_blocked == true
  and .terminal_public_claim_delivery_receipt_replay_recorded == false
  and .terminal_public_claim_delivery_receipt_idempotency_key_recorded == false
  and .terminal_public_claim_delivery_receipt_status_upgrade_accepted == false
  and .terminal_public_claim_delivery_receipt_ack_replay_accepted == false
  and .terminal_public_claim_delivery_receipt_hash_status_rebind_accepted == false
  and .release_publication_authority_from_delivery_receipt_replay_derived == false
  and .activation_authority_from_delivery_receipt_replay_derived == false
  and .public_ga_claimed == false
' <<<"$source_json" >/dev/null

ordering_static_mention_count="$(
  grep -Eci 'ordering|monotonic|sequence|cursor|latest|wins|duplicate|stale|late|arrival|future|gap|timestamp|epoch|rollback|same.sequence|different.hash|query|export|observability|delivery|status|hash|readback|telegram|external|authority|download|install|restart|active-binary|live' "$ORDERING_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson ordering_static_mention_count "$ordering_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_final_index_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_final_index_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_final_index_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_final_index_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_final_index_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_attachment_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_attachment_blocked: true,
    artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_gate_present: true,
    artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_doc_present: true,
    artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_static_mention_count: $ordering_static_mention_count,
    artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_denial_gate_invoked: false,
    long_soak_required_by_source_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_gate: true,
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
    attachment_blocker_count: 96,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_readback_without_receipt_ordering",
    local_gate: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-replay-idempotency-final-index-artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_REPLAY_IDEMPOTENCY_FINAL_INDEX_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_ORDERING_MONOTONICITY_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_final_index_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-replay-idempotency-final-index-report.sh",
      artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity-denial-gate.sh",
      artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_ORDERING_MONOTONICITY_DENIAL_GATE.md"
    },
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_gate_invoked: false,
      artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_denial_gate_invoked: false,
      terminal_public_claim_delivery_receipt_ordering_monotonicity_recorded: false,
      terminal_public_claim_delivery_receipt_ordering_monotonicity_persisted: false,
      terminal_public_claim_delivery_receipt_ordering_monotonicity_materialized: false,
      terminal_public_claim_delivery_receipt_sequence_cursor_recorded: false,
      terminal_public_claim_delivery_receipt_sequence_cursor_persisted: false,
      terminal_public_claim_delivery_receipt_monotonicity_state_recorded: false,
      terminal_public_claim_delivery_receipt_monotonicity_state_persisted: false,
      terminal_public_claim_delivery_receipt_monotonicity_state_materialized: false,
      terminal_public_claim_delivery_receipt_latest_wins_overwrite_accepted: false,
      terminal_public_claim_delivery_receipt_ordered_status_accepted: false,
      terminal_public_claim_delivery_receipt_ordered_acknowledgement_accepted: false,
      terminal_public_claim_delivery_receipt_ordered_ledger_index_accepted: false,
      terminal_public_claim_delivery_receipt_ordered_query_export_accepted: false,
      terminal_public_claim_delivery_receipt_ordered_observability_accepted: false,
      terminal_public_claim_delivery_receipt_ordered_hash_status_accepted: false,
      operator_approval_from_delivery_receipt_ordering_derived: false,
      release_publication_authority_from_delivery_receipt_ordering_derived: false,
      activation_authority_from_delivery_receipt_ordering_derived: false,
      download_link_from_delivery_receipt_ordering_rendered: false,
      install_command_from_delivery_receipt_ordering_emitted: false,
      install_from_delivery_receipt_ordering_executed: false,
      service_restart_from_delivery_receipt_ordering_performed: false,
      active_binary_from_delivery_receipt_ordering_mutated: false,
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
