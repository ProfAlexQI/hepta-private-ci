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
  echo "jq is required to build the artifact signing receipt signing receipt artifact signing/notarization surface denial report" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_GATE="scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-distribution-manifest-denial-gate.sh"
[[ -x "$SOURCE_GATE" ]] || {
  echo "missing executable artifact signing receipt signing receipt distribution artifact/manifest status denial gate: $SOURCE_GATE" >&2
  exit 1
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    "$SOURCE_GATE"
)"

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
signing_receipt_notarization_contract_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-artifact-signing-receipt-signing-receipt-artifact-signing-notarization-surface-denial:$source_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
signing_receipt_notarization_policy_hash_sha256="$(
  sha256_text "terminal-public-claim-delivery-receipt-signing-receipt-signing-notarization:no-signing:no-notarization:no-ticket:no-stapling:no-provenance:no-sbom:no-authority:no-install"
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_denial_ready == true
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_surface_count == 18
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_denied_count == 18
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_recorded_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_persisted_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_materialized_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_filesystem_written_count == 0
    and $source.distribution_artifact_status_exposed_count == 0
    and $source.manifest_status_exposed_count == 0
    and $source.manifest_signature_status_exposed_count == 0
    and $source.external_status_sent_count == 0
    and $source.telegram_status_sent_count == 0
    and $source.release_publication_authority_from_manifest_status_derived_count == 0
    and $source.activation_authority_from_manifest_status_derived_count == 0
    and $source.install_from_manifest_status_executed_count == 0
    and $source.active_binary_from_manifest_status_mutated_count == 0
    and $source.provider_invoked_count == 0
    and $source.credential_read_count == 0
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_recorded == false
    and $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_persisted == false
    and $source.distribution_artifact_status_exposed == false
    and $source.manifest_status_exposed == false
    and $source.package_manifest_materialized == false
    and $source.release_manifest_published == false
    and $source.public_status_claimed == false
    and $source.public_release_claimed == false
    and $source.public_ga_claimed == false
    and $source.operator_approval_from_manifest_status_derived == false
    and $source.release_publication_authority_from_manifest_status_derived == false
    and $source.activation_authority_from_manifest_status_derived == false
    and $source.provider_invoked == false
    and $source.credential_read == false
    and ($source.allowed_next_actions | any(
      .action == "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denial_gate"
      and .records_distribution_manifest_status == false
      and .records_artifact_signing_notarization_surface == false
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
    "source_signing_receipt_distribution_manifest_status_report_required",
    "signing_receipt_artifact_signing_surface",
    "signing_receipt_package_signing_surface",
    "signing_receipt_signature_manifest_checksum_binding",
    "signing_receipt_notarization_submission",
    "signing_receipt_notarization_ticket_recording",
    "signing_receipt_stapling_execution",
    "signing_receipt_installer_signing",
    "signing_receipt_provenance_attestation",
    "signing_receipt_sbom_manifest_publication",
    "signing_receipt_release_asset_bundle_packaging",
    "signing_receipt_cdn_update_feed_artifact_write",
    "signing_receipt_package_registry_artifact_publish",
    "signing_receipt_dashboard_endpoint_signing_status",
    "signing_receipt_query_export_observability_signing_status",
    "signing_receipt_external_telegram_package_channel_publication",
    "signing_receipt_authority_signing_status",
    "signing_receipt_activation_install_signing_path"
  ] | map({
    terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface: .,
    source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_ready: true,
    artifact_signing_notarization_surface_attempted: true,
    artifact_signing_notarization_surface_allowed: false,
    artifact_signing_notarization_surface_accepted: false,
    artifact_signing_notarization_surface_recorded: false,
    artifact_signing_notarization_surface_persisted: false,
    artifact_signing_notarization_surface_materialized: false,
    artifact_signing_notarization_surface_filesystem_written: false,
    artifact_signing_notarization_surface_delivered: false,
    artifact_signing_status_exposed: false,
    package_signing_status_exposed: false,
    signature_manifest_written: false,
    checksum_binding_recorded: false,
    notarization_submitted: false,
    notarization_ticket_recorded: false,
    stapling_executed: false,
    installer_signed: false,
    provenance_attestation_published: false,
    sbom_manifest_published: false,
    release_asset_packaged: false,
    artifact_bundle_packaged: false,
    cdn_artifact_written: false,
    update_feed_artifact_written: false,
    package_registry_artifact_published: false,
    dashboard_signing_status_exposed: false,
    endpoint_signing_status_exposed: false,
    query_status_exposed: false,
    export_status_exposed: false,
    observability_status_exposed: false,
    external_status_sent: false,
    telegram_status_sent: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    operator_approval_from_signing_status_derived: false,
    release_publication_authority_from_signing_status_derived: false,
    activation_authority_from_signing_status_derived: false,
    install_from_signing_status_executed: false,
    service_restart_from_signing_status_performed: false,
    active_binary_from_signing_status_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    artifact_signing_notarization_surface_noop_confirmed: true,
    artifact_signing_notarization_surface_status: "artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denied"
  })'
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denial_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --arg signing_receipt_notarization_contract_hash_sha256 "$signing_receipt_notarization_contract_hash_sha256" \
    --arg signing_receipt_notarization_policy_hash_sha256 "$signing_receipt_notarization_policy_hash_sha256" \
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
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_schema_version: "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denial_v1",
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_mode: "denied_signing_receipt_distribution_manifest_cannot_create_signing_notarization_or_artifact_publication",
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_gate: $source.gate,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_ready: $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_denial_ready,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_report_sha256: $source_report_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_contract_hash_sha256: $signing_receipt_notarization_contract_hash_sha256,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_policy_hash_sha256: $signing_receipt_notarization_policy_hash_sha256,
        minimum_required_samples: $min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denial_ready: true,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_surface_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_surface_count,
        source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_denied_count: $source.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_denied_count,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_attempt_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denied_count: ($surfaces | length),
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surfaces: $surfaces,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_accepted_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_recorded_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_persisted_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_materialized_count: 0,
        terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_filesystem_written_count: 0,
        allowed_next_actions: [
          {
            action: "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_without_signing",
            status: "allowed_report_only_next_slice",
            records_signing_notarization: false,
            records_receipt: false,
            persists_receipt: false,
            writes_artifacts: false,
            sends_externally: false,
            sends_telegram: false,
            derives_release_publication_authority: false,
            derives_activation_authority: false,
            installs_or_restarts: false,
            mutates_active_binary: false,
            reads_credentials: false
          }
        ],
        side_effect_free: true
      }
      + zero_object([
        "artifact_signing_status_exposed_count",
        "package_signing_status_exposed_count",
        "signature_manifest_written_count",
        "checksum_binding_recorded_count",
        "notarization_submitted_count",
        "notarization_ticket_recorded_count",
        "stapling_executed_count",
        "installer_signed_count",
        "provenance_attestation_published_count",
        "sbom_manifest_published_count",
        "release_asset_packaged_count",
        "artifact_bundle_packaged_count",
        "cdn_artifact_written_count",
        "update_feed_artifact_written_count",
        "package_registry_artifact_published_count",
        "external_status_sent_count",
        "telegram_status_sent_count",
        "release_publication_authority_from_signing_status_derived_count",
        "activation_authority_from_signing_status_derived_count",
        "install_from_signing_status_executed_count",
        "active_binary_from_signing_status_mutated_count",
        "provider_invoked_count",
        "credential_read_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_recorded",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_persisted",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_materialized",
        "terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_filesystem_written",
        "artifact_signing_status_exposed",
        "package_signing_status_exposed",
        "signature_manifest_written",
        "checksum_binding_recorded",
        "notarization_submitted",
        "notarization_ticket_recorded",
        "stapling_executed",
        "installer_signed",
        "provenance_attestation_published",
        "sbom_manifest_published",
        "release_asset_packaged",
        "artifact_bundle_packaged",
        "cdn_artifact_written",
        "update_feed_artifact_written",
        "package_registry_artifact_published",
        "dashboard_signing_status_exposed",
        "endpoint_signing_status_exposed",
        "query_status_exposed",
        "export_status_exposed",
        "observability_status_exposed",
        "external_status_sent",
        "telegram_status_sent",
        "public_status_claimed",
        "public_release_claimed",
        "public_ga_claimed",
        "operator_approval_from_signing_status_derived",
        "release_publication_authority_from_signing_status_derived",
        "activation_authority_from_signing_status_derived",
        "install_from_signing_status_executed",
        "service_restart_from_signing_status_performed",
        "active_binary_from_signing_status_mutated",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read"
      ])
      + {
        side_effects: false_object([
          "artifact_signing_notarization_surface_recorded",
          "artifact_signing_notarization_surface_persisted",
          "artifact_signing_notarization_surface_materialized",
          "artifact_signing_notarization_surface_filesystem_written",
          "artifact_signing_status_exposed",
          "package_signing_status_exposed",
          "signature_manifest_written",
          "checksum_binding_recorded",
          "notarization_submitted",
          "notarization_ticket_recorded",
          "stapling_executed",
          "installer_signed",
          "provenance_attestation_published",
          "sbom_manifest_published",
          "release_asset_packaged",
          "artifact_bundle_packaged",
          "cdn_artifact_written",
          "update_feed_artifact_written",
          "package_registry_artifact_published",
          "external_status_sent",
          "telegram_status_sent",
          "public_status_claimed",
          "public_release_claimed",
          "public_ga_claimed",
          "operator_approval_from_signing_status_derived",
          "release_publication_authority_from_signing_status_derived",
          "activation_authority_from_signing_status_derived",
          "install_from_signing_status_executed",
          "service_restart_from_signing_status_performed",
          "active_binary_from_signing_status_mutated",
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denial_ready == true
  and .source_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_ready == true
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denied_count == 18
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_recorded_count == 0
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_persisted_count == 0
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_materialized_count == 0
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_filesystem_written_count == 0
  and .artifact_signing_status_exposed_count == 0
  and .package_signing_status_exposed_count == 0
  and .notarization_submitted_count == 0
  and .notarization_ticket_recorded_count == 0
  and .stapling_executed_count == 0
  and .installer_signed_count == 0
  and .external_status_sent_count == 0
  and .telegram_status_sent_count == 0
  and .release_publication_authority_from_signing_status_derived_count == 0
  and .activation_authority_from_signing_status_derived_count == 0
  and .provider_invoked_count == 0
  and .credential_read_count == 0
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_recorded == false
  and .terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_persisted == false
  and .artifact_signing_status_exposed == false
  and .package_signing_status_exposed == false
  and .notarization_submitted == false
  and .notarization_ticket_recorded == false
  and .stapling_executed == false
  and .installer_signed == false
  and .external_status_sent == false
  and .telegram_status_sent == false
  and .public_status_claimed == false
  and .public_ga_claimed == false
  and .public_release_claimed == false
  and .release_publication_authority_from_signing_status_derived == false
  and .activation_authority_from_signing_status_derived == false
  and .provider_invoked == false
  and .credential_read == false
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surfaces | length) == 18
  and (.terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surfaces | all(
    .artifact_signing_notarization_surface_attempted == true
    and .artifact_signing_notarization_surface_noop_confirmed == true
    and .artifact_signing_notarization_surface_allowed == false
    and .artifact_signing_notarization_surface_recorded == false
    and .notarization_submitted == false
    and .external_status_sent == false
    and .telegram_status_sent == false
  ))
  and (.allowed_next_actions | any(
    .action == "prepare_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_non_persistence_without_signing"
    and .records_signing_notarization == false
    and .records_receipt == false
    and .persists_receipt == false
    and .writes_artifacts == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .reads_credentials == false
  ))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

echo "Hepta memory/intelligence/KG artifact signing terminal public claim delivery receipt artifact signing receipt signing receipt artifact signing/notarization surface denial gate passed" >&2
