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
  echo "jq is required to build the artifact signing receipt signing receipt signing receipt package/release/channel status denial report" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_GATE="scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-public-status-denial-gate.sh"
[[ -x "$SOURCE_GATE" ]] || {
  echo "missing executable artifact signing receipt signing receipt signing receipt terminal public claim/status exposure denial gate: $SOURCE_GATE" >&2
  exit 1
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    "$SOURCE_GATE"
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
signing_receipt_package_channel_contract_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-artifact-signing-receipt-signing-receipt-signing-receipt-package-release-channel-status-denial:$source_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
signing_receipt_package_channel_policy_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-signing-receipt-package-channel:no-package-status:no-release-channel:no-registry:no-feed:no-telegram:no-authority:no-install"
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denial_ready == true
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_surface_count == 18
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denied_count == 18
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_public_claim_recorded_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_public_status_exposed_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_public_ga_status_exposed_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_public_release_status_exposed_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_channel_status_delivered_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_external_status_sent_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_telegram_status_sent_count == 0
    and $source.release_publication_authority_from_public_status_derived_count == 0
    and $source.activation_authority_from_public_status_derived_count == 0
    and $source.install_from_public_status_executed_count == 0
    and $source.active_binary_from_public_status_mutated_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_recorded == false
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_public_claim_recorded == false
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_public_status_exposed == false
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_public_ga_status_exposed == false
    and $source.public_status_claimed == false
    and $source.public_ga_claimed == false
    and $source.public_release_claimed == false
    and $source.operator_approval_recorded == false
    and $source.release_publication_authority_from_public_status_derived == false
    and $source.activation_authority_from_public_status_derived == false
    and $source.provider_invoked == false
    and $source.credential_read == false
    and $source.install_executed == false
    and $source.active_binary_mutated == false
    and $source.external_send_performed == false
    and ($source.allowed_next_actions | any(
      .action == "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_denial_gate"
      and .records_package_release_channel_status == false
      and .derives_release_publication_authority == false
      and .derives_activation_authority == false
      and .installs_or_restarts == false
      and .mutates_active_binary == false
      and .reads_credentials == false
      and .sends_externally == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

surfaces_json="$(
  jq -n '[
    "source_signing_receipt_public_status_report_required",
    "signing_receipt_package_release_channel_status_claim",
    "signing_receipt_package_registry_status",
    "signing_receipt_release_channel_status",
    "signing_receipt_artifact_channel_status",
    "signing_receipt_install_channel_status",
    "signing_receipt_update_feed_status",
    "signing_receipt_version_channel_status",
    "signing_receipt_distribution_channel_status",
    "signing_receipt_dashboard_channel_status",
    "signing_receipt_status_endpoint_package_channel",
    "signing_receipt_query_package_channel_status",
    "signing_receipt_export_package_channel_status",
    "signing_receipt_observability_package_channel_status",
    "signing_receipt_external_channel_status",
    "signing_receipt_telegram_channel_status",
    "signing_receipt_authority_channel_status",
    "signing_receipt_activation_install_channel_status"
  ] | map({
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_surface: .,
    source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_ready: true,
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
    package_release_channel_status_status: "artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_denied"
  })'
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_denial_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --arg signing_receipt_package_channel_contract_hash_sha256 "$signing_receipt_package_channel_contract_hash_sha256" \
    --arg signing_receipt_package_channel_policy_hash_sha256 "$signing_receipt_package_channel_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$SOURCE_JSON" \
    --argjson surfaces "$surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_schema_version: "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_denial_v1",
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_mode: "denied_signing_receipt_public_status_cannot_create_package_release_channel_status_or_delivery",
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_gate: $source.gate,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_ready: $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denial_ready,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_report_sha256: $source_report_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_contract_hash_sha256: $signing_receipt_package_channel_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_policy_hash_sha256: $signing_receipt_package_channel_policy_hash_sha256,
        minimum_required_samples: $min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_denial_ready: true,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_surface_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_surface_count,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denied_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denied_count,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_surface_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_attempt_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_denied_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_surfaces: $surfaces,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_accepted_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_recorded_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_persisted_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_materialized_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_filesystem_written_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_delivered_count: 0,
        allowed_next_actions: [
          {
            action: "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_distribution_artifact_manifest_status_denial_gate",
            status: "allowed_report_only_next_slice",
            records_package_release_channel_status: false,
            records_distribution_manifest_status: false,
            derives_operator_approval: false,
            derives_release_publication_authority: false,
            derives_activation_authority: false,
            installs_or_restarts: false,
            mutates_active_binary: false,
            invokes_provider: false,
            reads_credentials: false,
            sends_externally: false
          }
        ],
        side_effect_free: true
      }
      + zero_object([
        "package_channel_status_exposed_count",
        "release_channel_status_exposed_count",
        "registry_channel_status_exposed_count",
        "artifact_channel_status_exposed_count",
        "install_channel_status_exposed_count",
        "update_feed_status_exposed_count",
        "version_channel_status_exposed_count",
        "distribution_channel_status_exposed_count",
        "dashboard_channel_status_exposed_count",
        "status_endpoint_channel_status_exposed_count",
        "query_status_exposed_count",
        "export_status_exposed_count",
        "observability_status_exposed_count",
        "external_status_sent_count",
        "telegram_status_sent_count",
        "release_publication_authority_from_package_channel_derived_count",
        "activation_authority_from_package_channel_derived_count",
        "install_from_package_channel_executed_count",
        "active_binary_from_package_channel_mutated_count",
        "provider_invoked_count",
        "credential_read_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_recorded",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_persisted",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_materialized",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_filesystem_written",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_delivered",
        "package_channel_status_exposed",
        "release_channel_status_exposed",
        "registry_channel_status_exposed",
        "artifact_channel_status_exposed",
        "install_channel_status_exposed",
        "update_feed_status_exposed",
        "version_channel_status_exposed",
        "distribution_channel_status_exposed",
        "dashboard_channel_status_exposed",
        "status_endpoint_channel_status_exposed",
        "query_status_exposed",
        "export_status_exposed",
        "observability_status_exposed",
        "external_status_sent",
        "telegram_status_sent",
        "public_status_claimed",
        "public_release_claimed",
        "public_ga_claimed",
        "operator_approval_from_package_channel_derived",
        "release_publication_authority_from_package_channel_derived",
        "activation_authority_from_package_channel_derived",
        "install_from_package_channel_executed",
        "service_restart_from_package_channel_performed",
        "active_binary_from_package_channel_mutated",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read"
      ])
      + {
        side_effects: false_object([
          "package_release_channel_status_recorded",
          "package_release_channel_status_persisted",
          "package_release_channel_status_materialized",
          "package_release_channel_status_filesystem_written",
          "package_release_channel_status_delivered",
          "package_channel_status_exposed",
          "release_channel_status_exposed",
          "registry_channel_status_exposed",
          "update_feed_status_exposed",
          "dashboard_channel_status_exposed",
          "status_endpoint_channel_status_exposed",
          "query_status_exposed",
          "export_status_exposed",
          "observability_status_exposed",
          "external_status_sent",
          "telegram_status_sent",
          "public_status_claimed",
          "public_release_claimed",
          "public_ga_claimed",
          "operator_approval_from_package_channel_derived",
          "release_publication_authority_from_package_channel_derived",
          "activation_authority_from_package_channel_derived",
          "install_from_package_channel_executed",
          "service_restart_from_package_channel_performed",
          "active_binary_from_package_channel_mutated",
          "memory_store_write_performed",
          "live_kg_write_performed",
          "provider_invoked",
          "model_invoked",
          "credential_read",
          "secret_file_read",
          "external_send_performed",
          "telegram_send_performed",
          "terminal_live_url_contacted",
          "long_soak_started",
          "public_release_published",
          "public_ga_promoted",
          "release_deployed"
        ])
      }
    '
)"

printf '%s\n' "$report"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_denial_ready == true
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_ready == true
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_surface_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_denied_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_recorded_count == 0
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_persisted_count == 0
  and .package_channel_status_exposed_count == 0
  and .release_channel_status_exposed_count == 0
  and .registry_channel_status_exposed_count == 0
  and .update_feed_status_exposed_count == 0
  and .external_status_sent_count == 0
  and .telegram_status_sent_count == 0
  and .release_publication_authority_from_package_channel_derived_count == 0
  and .activation_authority_from_package_channel_derived_count == 0
  and .install_from_package_channel_executed_count == 0
  and .active_binary_from_package_channel_mutated_count == 0
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_recorded == false
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_persisted == false
  and .package_channel_status_exposed == false
  and .release_channel_status_exposed == false
  and .registry_channel_status_exposed == false
  and .external_status_sent == false
  and .telegram_status_sent == false
  and .public_status_claimed == false
  and .public_ga_claimed == false
  and .public_release_claimed == false
  and .release_publication_authority_from_package_channel_derived == false
  and .activation_authority_from_package_channel_derived == false
  and .provider_invoked == false
  and .credential_read == false
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_surfaces | length) == 18
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_package_release_channel_status_surfaces | all(
    .package_release_channel_status_attempted == true
    and .package_release_channel_status_noop_confirmed == true
    and .package_release_channel_status_allowed == false
    and .package_release_channel_status_recorded == false
    and .package_channel_status_exposed == false
    and .release_channel_status_exposed == false
    and .external_status_sent == false
    and .telegram_status_sent == false
  ))
  and (.allowed_next_actions | any(
    .action == "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_distribution_artifact_manifest_status_denial_gate"
    and .records_package_release_channel_status == false
    and .records_distribution_manifest_status == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .reads_credentials == false
    and .sends_externally == false
  ))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

echo "Hepta memory/intelligence/KG artifact signing terminal public claim delivery receipt artifact signing receipt signing receipt signing receipt package/release/channel status denial gate passed" >&2
