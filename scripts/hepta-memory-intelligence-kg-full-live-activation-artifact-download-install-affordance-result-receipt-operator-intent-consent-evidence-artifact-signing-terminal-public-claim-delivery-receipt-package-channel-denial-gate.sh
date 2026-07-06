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
  echo "jq is required to build the terminal public claim delivery receipt package/release/channel status denial report" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_GATE="scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-public-status-denial-gate.sh"
[[ -x "$SOURCE_GATE" ]] || {
  echo "missing executable terminal public claim delivery receipt public claim/status exposure denial gate: $SOURCE_GATE" >&2
  exit 1
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    "$SOURCE_GATE"
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
package_channel_contract_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-package-release-channel-status-denial:$source_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
package_channel_policy_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-package-release-channel:no-package-status:no-release-channel:no-update-feed:no-registry:no-telegram:no-authority:no-install"
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_denial_ready == true
    and $source.terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_surface_count == 18
    and $source.terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_denied_count == 18
    and $source.terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_recorded_count == 0
    and $source.terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_persisted_count == 0
    and $source.terminal_public_claim_delivery_receipt_public_status_exposed_count == 0
    and $source.terminal_public_claim_delivery_receipt_public_ga_status_exposed_count == 0
    and $source.terminal_public_claim_delivery_receipt_public_release_status_exposed_count == 0
    and $source.terminal_public_claim_delivery_receipt_channel_status_delivered_count == 0
    and $source.terminal_public_claim_delivery_receipt_export_status_exposed_count == 0
    and $source.terminal_public_claim_delivery_receipt_observability_status_exposed_count == 0
    and $source.terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_recorded == false
    and $source.terminal_public_claim_delivery_receipt_public_status_exposed == false
    and $source.public_release_claimed == false
    and $source.public_ga_claimed == false
    and $source.release_publication_authority_derived == false
    and $source.activation_authority_derived == false
    and $source.provider_invoked == false
    and $source.credential_read == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

surfaces_json="$(
  jq -n '[
    "source_terminal_public_claim_status_exposure_report_required",
    "delivery_receipt_package_release_channel_status_claim",
    "delivery_receipt_package_registry_channel_status",
    "delivery_receipt_release_channel_status",
    "delivery_receipt_artifact_channel_status",
    "delivery_receipt_install_channel_status",
    "delivery_receipt_update_feed_status",
    "delivery_receipt_version_channel_status",
    "delivery_receipt_distribution_channel_status",
    "delivery_receipt_dashboard_channel_status",
    "delivery_receipt_status_endpoint_package_channel",
    "delivery_receipt_query_package_channel_status",
    "delivery_receipt_export_package_channel_status",
    "delivery_receipt_observability_package_channel_status",
    "delivery_receipt_external_channel_status",
    "delivery_receipt_telegram_channel_status",
    "delivery_receipt_authority_channel_status",
    "delivery_receipt_activation_install_channel_status"
  ] | map({
    terminal_public_claim_delivery_receipt_package_release_channel_status_surface: .,
    source_terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_ready: true,
    package_release_channel_status_attempted: true,
    package_release_channel_status_allowed: false,
    package_release_channel_status_accepted: false,
    package_release_channel_status_recorded: false,
    package_release_channel_status_persisted: false,
    package_release_channel_status_materialized: false,
    package_release_channel_status_filesystem_written: false,
    package_release_channel_status_delivered: false,
    package_channel_status_exposed: false,
    release_channel_status_exposed: false,
    registry_channel_status_exposed: false,
    artifact_channel_status_exposed: false,
    install_channel_status_exposed: false,
    update_feed_status_exposed: false,
    version_channel_status_exposed: false,
    distribution_channel_status_exposed: false,
    dashboard_channel_status_exposed: false,
    status_endpoint_channel_status_exposed: false,
    query_status_exposed: false,
    export_status_exposed: false,
    observability_status_exposed: false,
    external_status_sent: false,
    telegram_status_sent: false,
    package_release_channel_status_noop_confirmed: true,
    package_release_channel_status_status: "package_release_channel_status_denied"
  })'
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_package_release_channel_status_denial_gate" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg package_channel_contract_hash_sha256 "$package_channel_contract_hash_sha256" \
  --arg package_channel_policy_hash_sha256 "$package_channel_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$SOURCE_JSON" \
  --argjson surfaces "$surfaces_json" \
  '{
    product: $product,
    runtime: $runtime,
    status: "ready",
    base_url: $base_url,
    gate: $gate,
    terminal_public_claim_delivery_receipt_package_release_channel_status_schema_version: "terminal_public_claim_delivery_receipt_package_release_channel_status_denial_v1",
    terminal_public_claim_delivery_receipt_package_release_channel_status_mode: "denied_public_status_cannot_create_package_release_channel_status_or_delivery",
    source_terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_gate: $source.gate,
    source_terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_ready: $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_denial_ready,
    source_terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_report_sha256: $source_report_sha256,
    terminal_public_claim_delivery_receipt_package_release_channel_status_contract_hash_sha256: $package_channel_contract_hash_sha256,
    terminal_public_claim_delivery_receipt_package_release_channel_status_policy_hash_sha256: $package_channel_policy_hash_sha256,
    minimum_required_samples: $min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_package_release_channel_status_denial_ready: true,
    source_terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_surface_count: $source.terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_surface_count,
    source_terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_denied_count: $source.terminal_public_claim_delivery_receipt_terminal_public_claim_status_exposure_denied_count,
    terminal_public_claim_delivery_receipt_package_release_channel_status_surface_count: ($surfaces | length),
    terminal_public_claim_delivery_receipt_package_release_channel_status_attempt_count: ($surfaces | length),
    terminal_public_claim_delivery_receipt_package_release_channel_status_denied_count: ($surfaces | length),
    terminal_public_claim_delivery_receipt_package_release_channel_status_surfaces: $surfaces,
    terminal_public_claim_delivery_receipt_package_release_channel_status_accepted_count: 0,
    terminal_public_claim_delivery_receipt_package_release_channel_status_recorded_count: 0,
    terminal_public_claim_delivery_receipt_package_release_channel_status_persisted_count: 0,
    terminal_public_claim_delivery_receipt_package_release_channel_status_materialized_count: 0,
    terminal_public_claim_delivery_receipt_package_release_channel_status_filesystem_written_count: 0,
    terminal_public_claim_delivery_receipt_package_release_channel_status_delivered_count: 0,
    package_channel_status_exposed_count: 0,
    release_channel_status_exposed_count: 0,
    registry_channel_status_exposed_count: 0,
    artifact_channel_status_exposed_count: 0,
    install_channel_status_exposed_count: 0,
    update_feed_status_exposed_count: 0,
    version_channel_status_exposed_count: 0,
    distribution_channel_status_exposed_count: 0,
    dashboard_channel_status_exposed_count: 0,
    status_endpoint_channel_status_exposed_count: 0,
    query_status_exposed_count: 0,
    export_status_exposed_count: 0,
    observability_status_exposed_count: 0,
    external_status_sent_count: 0,
    telegram_status_sent_count: 0,
    release_publication_authority_from_package_channel_derived_count: 0,
    activation_authority_from_package_channel_derived_count: 0,
    install_from_package_channel_executed_count: 0,
    active_binary_from_package_channel_mutated_count: 0,
    provider_invoked_count: 0,
    credential_read_count: 0,
    terminal_public_claim_delivery_receipt_package_release_channel_status_recorded: false,
    terminal_public_claim_delivery_receipt_package_release_channel_status_persisted: false,
    package_channel_status_exposed: false,
    release_channel_status_exposed: false,
    registry_channel_status_exposed: false,
    artifact_channel_status_exposed: false,
    install_channel_status_exposed: false,
    update_feed_status_exposed: false,
    version_channel_status_exposed: false,
    distribution_channel_status_exposed: false,
    dashboard_channel_status_exposed: false,
    status_endpoint_channel_status_exposed: false,
    query_status_exposed: false,
    export_status_exposed: false,
    observability_status_exposed: false,
    external_status_sent: false,
    telegram_status_sent: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    operator_approval_from_package_channel_derived: false,
    release_publication_authority_from_package_channel_derived: false,
    activation_authority_from_package_channel_derived: false,
    install_from_package_channel_executed: false,
    service_restart_from_package_channel_performed: false,
    active_binary_from_package_channel_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    allowed_next_actions: [
      {
        action: "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_package_release_channel_status_readback_without_public_status",
        status: "allowed_report_only_next_slice",
        records_package_release_channel_status: false,
        exposes_package_status: false,
        exposes_release_channel_status: false,
        sends_externally: false,
        sends_telegram: false,
        derives_release_publication_authority: false,
        derives_activation_authority: false,
        installs_or_restarts: false,
        mutates_active_binary: false,
        reads_credentials: false
      }
    ],
    side_effect_free: true,
    side_effects: {
      package_release_channel_status_recorded: false,
      package_release_channel_status_persisted: false,
      package_release_channel_status_materialized: false,
      package_release_channel_status_filesystem_written: false,
      package_release_channel_status_delivered: false,
      package_channel_status_exposed: false,
      release_channel_status_exposed: false,
      registry_channel_status_exposed: false,
      update_feed_status_exposed: false,
      dashboard_channel_status_exposed: false,
      status_endpoint_channel_status_exposed: false,
      query_status_exposed: false,
      export_status_exposed: false,
      observability_status_exposed: false,
      external_status_sent: false,
      telegram_status_sent: false,
      public_status_claimed: false,
      public_release_claimed: false,
      public_ga_claimed: false,
      operator_approval_from_package_channel_derived: false,
      release_publication_authority_from_package_channel_derived: false,
      activation_authority_from_package_channel_derived: false,
      install_from_package_channel_executed: false,
      service_restart_from_package_channel_performed: false,
      active_binary_from_package_channel_mutated: false,
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
