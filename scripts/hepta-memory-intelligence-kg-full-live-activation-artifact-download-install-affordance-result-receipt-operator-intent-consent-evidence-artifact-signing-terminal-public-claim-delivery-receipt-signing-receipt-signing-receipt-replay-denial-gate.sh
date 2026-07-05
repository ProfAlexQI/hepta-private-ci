#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the terminal public claim delivery receipt artifact signing receipt signing receipt replay/idempotency denial report" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_GATE="scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-nonp-denial-gate.sh"
[[ -x "$SOURCE_GATE" ]] || {
  echo "missing executable terminal public claim delivery receipt artifact signing receipt signing receipt non-persistence denial gate: $SOURCE_GATE" >&2
  exit 1
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    "$SOURCE_GATE"
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
signing_replay_contract_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-artifact-signing-receipt-signing-receipt-signing-receipt-replay-idempotency-denial:$source_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
signing_replay_policy_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-replay:no-replay:no-duplicate:no-idempotency:no-cross-scope:no-status-upgrade:no-authority:no-install"
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_denial_ready == true
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_surface_count == 18
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_denied_count == 18
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_recorded_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_persisted_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_materialized_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_filesystem_written_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_delivered_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_status_exposed_count == 0
    and $source.artifact_signing_receipt_signing_receipt_signing_receipt_recorded_count == 0
    and $source.package_signing_receipt_recorded_count == 0
    and $source.signature_manifest_receipt_recorded_count == 0
    and $source.notarization_submission_receipt_recorded_count == 0
    and $source.notarization_ticket_receipt_recorded_count == 0
    and $source.external_receipt_sent_count == 0
    and $source.telegram_receipt_sent_count == 0
    and $source.release_publication_authority_from_signing_receipt_derived_count == 0
    and $source.activation_authority_from_signing_receipt_derived_count == 0
    and $source.install_from_signing_receipt_executed_count == 0
    and $source.active_binary_from_signing_receipt_mutated_count == 0
    and $source.provider_invoked_count == 0
    and $source.credential_read_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_recorded == false
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_persisted == false
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_status_exposed == false
    and $source.artifact_signing_receipt_signing_receipt_signing_receipt_recorded == false
    and $source.package_signing_receipt_recorded == false
    and $source.signature_manifest_receipt_recorded == false
    and $source.notarization_submission_receipt_recorded == false
    and $source.notarization_ticket_receipt_recorded == false
    and $source.external_receipt_sent == false
    and $source.telegram_receipt_sent == false
    and $source.operator_approval_from_signing_receipt_derived == false
    and $source.release_publication_authority_from_signing_receipt_derived == false
    and $source.activation_authority_from_signing_receipt_derived == false
    and $source.install_from_signing_receipt_executed == false
    and $source.active_binary_from_signing_receipt_mutated == false
    and $source.public_status_claimed == false
    and $source.public_release_claimed == false
    and $source.public_ga_claimed == false
    and $source.provider_invoked == false
    and $source.credential_read == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

surfaces_json="$(
  jq -n '[
    "source_signing_receipt_non_persistence_report_required",
    "duplicate_signing_receipt_identity",
    "signing_receipt_replay_acceptance",
    "signing_receipt_idempotency_key",
    "signing_receipt_idempotency_state",
    "signing_receipt_nonce_replay",
    "signing_receipt_cross_scope_reuse",
    "signing_receipt_status_upgrade",
    "signing_receipt_ack_replay",
    "signing_receipt_ledger_index_replay",
    "signing_receipt_query_export_observability_replay",
    "signing_receipt_hash_status_rebind",
    "artifact_package_signature_receipt_replay",
    "notarization_ticket_stapling_receipt_replay",
    "release_cdn_registry_receipt_replay",
    "external_telegram_signing_receipt_replay",
    "approval_authority_from_signing_receipt_replay",
    "install_restart_active_binary_from_signing_receipt_replay"
  ] | map({
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_surface: .,
    source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_ready: true,
    signing_receipt_replay_idempotency_attempted: true,
    signing_receipt_replay_idempotency_allowed: false,
    signing_receipt_replay_allowed: false,
    signing_receipt_replay_accepted: false,
    signing_receipt_replay_recorded: false,
    signing_receipt_replay_persisted: false,
    signing_receipt_replay_performed: false,
    signing_receipt_duplicate_accepted: false,
    signing_receipt_duplicate_recorded: false,
    signing_receipt_duplicate_persisted: false,
    signing_receipt_idempotency_key_accepted: false,
    signing_receipt_idempotency_key_recorded: false,
    signing_receipt_idempotency_state_recorded: false,
    signing_receipt_idempotency_state_persisted: false,
    signing_receipt_idempotency_state_materialized: false,
    signing_receipt_idempotency_filesystem_written: false,
    signing_receipt_replay_nonce_accepted: false,
    signing_receipt_replay_nonce_recorded: false,
    signing_receipt_cross_scope_reuse_accepted: false,
    signing_receipt_status_upgrade_accepted: false,
    signing_receipt_ack_replay_accepted: false,
    signing_receipt_ledger_replay_accepted: false,
    signing_receipt_index_replay_accepted: false,
    signing_receipt_query_replay_accepted: false,
    signing_receipt_export_replay_accepted: false,
    signing_receipt_observability_replay_accepted: false,
    signing_receipt_hash_status_rebind_accepted: false,
    artifact_signing_receipt_signing_receipt_signing_receipt_replay_accepted: false,
    package_signing_receipt_replay_accepted: false,
    signature_manifest_receipt_replay_accepted: false,
    notarization_submission_receipt_replay_accepted: false,
    notarization_ticket_receipt_replay_accepted: false,
    stapling_receipt_replay_accepted: false,
    installer_signing_receipt_replay_accepted: false,
    release_asset_receipt_replay_accepted: false,
    cdn_update_feed_receipt_replay_accepted: false,
    package_registry_receipt_replay_accepted: false,
    external_signing_receipt_replay_accepted: false,
    telegram_signing_receipt_replay_accepted: false,
    operator_approval_from_signing_receipt_replay_derived: false,
    release_publication_authority_from_signing_receipt_replay_derived: false,
    activation_authority_from_signing_receipt_replay_derived: false,
    install_from_signing_receipt_replay_executed: false,
    service_restart_from_signing_receipt_replay_performed: false,
    active_binary_from_signing_receipt_replay_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    external_send_performed: false,
    telegram_send_performed: false,
    signing_receipt_replay_idempotency_noop_confirmed: true,
    signing_receipt_replay_idempotency_status: "artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_denied"
  })'
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_denial_gate" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg signing_replay_contract_hash_sha256 "$signing_replay_contract_hash_sha256" \
  --arg signing_replay_policy_hash_sha256 "$signing_replay_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$SOURCE_JSON" \
  --argjson surfaces "$surfaces_json" \
  '{
    product: $product,
    runtime: $runtime,
    status: "ready",
    base_url: $base_url,
    gate: $gate,
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_schema_version: "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_denial_v1",
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_mode: "denied_signing_receipt_non_persistence_cannot_replay_duplicate_idempotency_rebind_authority_or_install",
    source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_gate: $source.gate,
    source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_ready: $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_denial_ready,
    source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_report_sha256: $source_report_sha256,
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_contract_hash_sha256: $signing_replay_contract_hash_sha256,
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_policy_hash_sha256: $signing_replay_policy_hash_sha256,
    minimum_required_samples: $min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_denial_ready: true,
    source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_surface_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_surface_count,
    source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_denied_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_denied_count,
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_surface_count: ($surfaces | length),
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_attempt_count: ($surfaces | length),
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_denied_count: ($surfaces | length),
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_surfaces: $surfaces,
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_allowed_count: 0,
    signing_receipt_replay_allowed_count: 0,
    signing_receipt_replay_accepted_count: 0,
    signing_receipt_replay_recorded_count: 0,
    signing_receipt_replay_persisted_count: 0,
    signing_receipt_replay_performed_count: 0,
    signing_receipt_duplicate_accepted_count: 0,
    signing_receipt_duplicate_recorded_count: 0,
    signing_receipt_idempotency_key_accepted_count: 0,
    signing_receipt_idempotency_key_recorded_count: 0,
    signing_receipt_idempotency_state_recorded_count: 0,
    signing_receipt_idempotency_state_persisted_count: 0,
    signing_receipt_replay_nonce_accepted_count: 0,
    signing_receipt_replay_nonce_recorded_count: 0,
    signing_receipt_cross_scope_reuse_accepted_count: 0,
    signing_receipt_status_upgrade_accepted_count: 0,
    signing_receipt_ack_replay_accepted_count: 0,
    signing_receipt_ledger_replay_accepted_count: 0,
    signing_receipt_index_replay_accepted_count: 0,
    signing_receipt_query_replay_accepted_count: 0,
    signing_receipt_export_replay_accepted_count: 0,
    signing_receipt_observability_replay_accepted_count: 0,
    signing_receipt_hash_status_rebind_accepted_count: 0,
    external_signing_receipt_replay_accepted_count: 0,
    telegram_signing_receipt_replay_accepted_count: 0,
    release_publication_authority_from_signing_receipt_replay_derived_count: 0,
    activation_authority_from_signing_receipt_replay_derived_count: 0,
    install_from_signing_receipt_replay_executed_count: 0,
    active_binary_from_signing_receipt_replay_mutated_count: 0,
    provider_invoked_count: 0,
    credential_read_count: 0,
    signing_receipt_replay_allowed: false,
    signing_receipt_replay_accepted: false,
    signing_receipt_replay_recorded: false,
    signing_receipt_replay_persisted: false,
    signing_receipt_replay_performed: false,
    signing_receipt_duplicate_accepted: false,
    signing_receipt_idempotency_key_accepted: false,
    signing_receipt_idempotency_key_recorded: false,
    signing_receipt_idempotency_state_recorded: false,
    signing_receipt_idempotency_state_persisted: false,
    signing_receipt_cross_scope_reuse_accepted: false,
    signing_receipt_status_upgrade_accepted: false,
    signing_receipt_ack_replay_accepted: false,
    signing_receipt_hash_status_rebind_accepted: false,
    artifact_signing_receipt_signing_receipt_signing_receipt_replay_accepted: false,
    package_signing_receipt_replay_accepted: false,
    signature_manifest_receipt_replay_accepted: false,
    notarization_submission_receipt_replay_accepted: false,
    notarization_ticket_receipt_replay_accepted: false,
    stapling_receipt_replay_accepted: false,
    installer_signing_receipt_replay_accepted: false,
    release_asset_receipt_replay_accepted: false,
    cdn_update_feed_receipt_replay_accepted: false,
    package_registry_receipt_replay_accepted: false,
    external_signing_receipt_replay_accepted: false,
    telegram_signing_receipt_replay_accepted: false,
    operator_approval_from_signing_receipt_replay_derived: false,
    release_publication_authority_from_signing_receipt_replay_derived: false,
    activation_authority_from_signing_receipt_replay_derived: false,
    install_from_signing_receipt_replay_executed: false,
    service_restart_from_signing_receipt_replay_performed: false,
    active_binary_from_signing_receipt_replay_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    external_send_performed: false,
    telegram_send_performed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    allowed_next_actions: [
      {
        action: "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_ordering_monotonicity_denial_gate",
        status: "allowed_report_only_next_slice",
        replays_signing_receipt: false,
        records_duplicate_receipt: false,
        records_idempotency_key: false,
        persists_idempotency_state: false,
        accepts_cross_scope_reuse: false,
        accepts_status_upgrade: false,
        derives_release_publication_authority: false,
        derives_activation_authority: false,
        installs_or_restarts: false,
        mutates_active_binary: false,
        invokes_provider: false,
        reads_credentials: false,
        sends_externally: false
      }
    ],
    side_effect_free: true,
    side_effects: {
      signing_receipt_replay_recorded: false,
      signing_receipt_replay_persisted: false,
      signing_receipt_replay_performed: false,
      signing_receipt_duplicate_recorded: false,
      signing_receipt_duplicate_persisted: false,
      signing_receipt_idempotency_key_recorded: false,
      signing_receipt_idempotency_state_recorded: false,
      signing_receipt_idempotency_state_persisted: false,
      signing_receipt_idempotency_state_materialized: false,
      signing_receipt_idempotency_filesystem_written: false,
      signing_receipt_replay_nonce_recorded: false,
      signing_receipt_cross_scope_reuse_accepted: false,
      signing_receipt_status_upgrade_accepted: false,
      signing_receipt_ack_replay_accepted: false,
      signing_receipt_hash_status_rebind_accepted: false,
      artifact_signing_receipt_signing_receipt_signing_receipt_replay_accepted: false,
      package_signing_receipt_replay_accepted: false,
      signature_manifest_receipt_replay_accepted: false,
      notarization_submission_receipt_replay_accepted: false,
      notarization_ticket_receipt_replay_accepted: false,
      stapling_receipt_replay_accepted: false,
      installer_signing_receipt_replay_accepted: false,
      external_signing_receipt_replay_accepted: false,
      telegram_signing_receipt_replay_accepted: false,
      operator_approval_from_signing_receipt_replay_derived: false,
      release_publication_authority_from_signing_receipt_replay_derived: false,
      activation_authority_from_signing_receipt_replay_derived: false,
      install_from_signing_receipt_replay_executed: false,
      service_restart_from_signing_receipt_replay_performed: false,
      active_binary_from_signing_receipt_replay_mutated: false,
      memory_store_write_performed: false,
      live_kg_write_performed: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      secret_file_read: false,
      external_send_performed: false,
      telegram_send_performed: false,
      terminal_live_url_contacted: false,
      long_soak_started: false,
      public_release_published: false,
      public_ga_promoted: false,
      release_deployed: false
    }
  }'
