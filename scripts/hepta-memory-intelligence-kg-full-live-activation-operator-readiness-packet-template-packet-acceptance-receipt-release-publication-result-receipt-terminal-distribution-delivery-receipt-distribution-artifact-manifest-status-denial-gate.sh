#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

source scripts/lib/hepta-json-report-capture.sh

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_PACKAGE_RELEASE_CHANNEL_STATUS_EXPOSURE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-package-release-channel-status-exposure-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-package-release-channel-status-exposure-denial-gate.sh
)"

delivery_receipt_package_release_channel_status_exposure_report_sha256="$(
  sha256_text "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_PACKAGE_RELEASE_CHANNEL_STATUS_EXPOSURE_JSON"
)"
delivery_receipt_distribution_artifact_manifest_status_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial:$delivery_receipt_package_release_channel_status_exposure_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
delivery_receipt_distribution_artifact_manifest_status_policy_hash_sha256="$(
  sha256_text "release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial:no-artifact-manifest:no-package-manifest:no-checksum-index:no-cdn-update-feed-metadata:no-signing-no-notarization:no-live"
)"

jq -n -e \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_PACKAGE_RELEASE_CHANNEL_STATUS_EXPOSURE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_request_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_persisted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_materialized_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_filesystem_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_delivered_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_index_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_metadata_endpoint_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_mirror_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_release_channel_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_catalog_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_version_manifest_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_manifest_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_manifest_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_download_page_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_release_notes_package_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_announcement_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_release_publication_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_activation_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_live_execution_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_install_executed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_service_restarted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_active_binary_mutated_count == 0
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_accepted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_version_manifest_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_manifest_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_manifest_status_exposed == false
    and $source.release_publication_authority_derived == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_mutated == false
    and $source.live_kg_write_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.credential_read == false
    and $source.secret_file_read == false
    and $source.install_executed == false
    and $source.launchd_mutated == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and $source.public_release_claimed == false
    and $source.public_ga_claimed == false
    and $source.release_artifact_written == false
    and $source.public_artifact_written == false
    and $source.external_send_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

delivery_receipt_distribution_artifact_manifest_status_surfaces_json="$(
  jq -n '
    def artifact_manifest_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface:$id,
        source_package_release_channel_status_exposure_ready:true,
        distribution_artifact_manifest_status_attempted:true,
        distribution_artifact_manifest_status_allowed:false,
        distribution_artifact_manifest_status_request_accepted:false,
        distribution_artifact_manifest_status_accepted:false,
        distribution_artifact_manifest_status_recorded:false,
        distribution_artifact_manifest_status_persisted:false,
        distribution_artifact_manifest_status_materialized:false,
        distribution_artifact_manifest_status_filesystem_written:false,
        distribution_artifact_manifest_status_delivered:false,
        distribution_artifact_manifest_status_exposed:false,
        distribution_artifact_manifest_exposed:false,
        package_manifest_status_exposed:false,
        checksum_index_status_exposed:false,
        artifact_metadata_status_exposed:false,
        cdn_artifact_metadata_status_exposed:false,
        update_feed_artifact_metadata_status_exposed:false,
        package_signing_status_exposed:false,
        notarization_status_exposed:false,
        stapling_status_exposed:false,
        provenance_attestation_status_exposed:false,
        sbom_manifest_status_exposed:false,
        artifact_digest_manifest_status_exposed:false,
        release_asset_manifest_status_exposed:false,
        installer_package_manifest_status_exposed:false,
        package_channel_manifest_status_exposed:false,
        external_artifact_manifest_status_sent:false,
        telegram_artifact_manifest_status_sent:false,
        public_release_claimed:false,
        public_ga_claimed:false,
        acceptance_recorded:false,
        operator_approval_derived:false,
        release_publication_authority_derived:false,
        activation_authority_derived:false,
        activation_command_derived:false,
        live_execution_allowed:false,
        activation_performed:false,
        install_executed:false,
        service_restarted:false,
        launchd_mutated:false,
        active_binary_mutated:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        distribution_artifact_manifest_status_noop_confirmed:true,
        distribution_artifact_manifest_status_status:$status,
        reason:$reason
      } + $extra;
    [
      artifact_manifest_surface("delivery_receipt_distribution_artifact_manifest_status"; "blocked_distribution_artifact_manifest_status_noop"; "distribution_artifact_manifest_status_denied"; {distribution_artifact_manifest_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_package_manifest_status"; "blocked_package_manifest_status_noop"; "package_manifest_status_denied"; {package_manifest_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_checksum_index_status"; "blocked_checksum_index_status_noop"; "checksum_index_status_denied"; {checksum_index_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_artifact_metadata_status"; "blocked_artifact_metadata_status_noop"; "artifact_metadata_status_denied"; {artifact_metadata_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_cdn_artifact_metadata_status"; "blocked_cdn_artifact_metadata_status_noop"; "cdn_artifact_metadata_status_denied"; {cdn_artifact_metadata_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_update_feed_artifact_metadata_status"; "blocked_update_feed_artifact_metadata_status_noop"; "update_feed_artifact_metadata_status_denied"; {update_feed_artifact_metadata_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_package_signing_status"; "blocked_package_signing_status_noop"; "package_signing_status_denied"; {package_signing_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_notarization_status"; "blocked_notarization_status_noop"; "notarization_status_denied"; {notarization_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_stapling_status"; "blocked_stapling_status_noop"; "stapling_status_denied"; {stapling_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_provenance_attestation_status"; "blocked_provenance_attestation_status_noop"; "provenance_attestation_status_denied"; {provenance_attestation_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_sbom_manifest_status"; "blocked_sbom_manifest_status_noop"; "sbom_manifest_status_denied"; {sbom_manifest_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_artifact_digest_manifest_status"; "blocked_artifact_digest_manifest_status_noop"; "artifact_digest_manifest_status_denied"; {artifact_digest_manifest_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_release_asset_manifest_status"; "blocked_release_asset_manifest_status_noop"; "release_asset_manifest_status_denied"; {release_asset_manifest_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_installer_package_manifest_status"; "blocked_installer_package_manifest_status_noop"; "installer_package_manifest_status_denied"; {installer_package_manifest_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_package_channel_manifest_status"; "blocked_package_channel_manifest_status_noop"; "package_channel_manifest_status_denied"; {package_channel_manifest_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_external_telegram_artifact_manifest_status"; "blocked_external_telegram_artifact_manifest_status_noop"; "external_telegram_artifact_manifest_status_denied"; {external_artifact_manifest_status_requested:true, telegram_artifact_manifest_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_release_publication_authority_artifact_manifest_status"; "blocked_release_publication_authority_artifact_manifest_status_noop"; "release_publication_authority_from_artifact_manifest_status_denied"; {release_publication_authority_artifact_manifest_status_requested:true}),
      artifact_manifest_surface("delivery_receipt_activation_live_install_restart_active_binary_artifact_manifest_status"; "blocked_activation_live_install_restart_active_binary_artifact_manifest_status_noop"; "activation_live_install_restart_active_binary_from_artifact_manifest_status_denied"; {activation_live_artifact_manifest_status_requested:true, install_restart_active_binary_artifact_manifest_status_requested:true})
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_gate" \
  --arg delivery_receipt_package_release_channel_status_exposure_report_sha256 "$delivery_receipt_package_release_channel_status_exposure_report_sha256" \
  --arg delivery_receipt_distribution_artifact_manifest_status_contract_hash_sha256 "$delivery_receipt_distribution_artifact_manifest_status_contract_hash_sha256" \
  --arg delivery_receipt_distribution_artifact_manifest_status_policy_hash_sha256 "$delivery_receipt_distribution_artifact_manifest_status_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_PACKAGE_RELEASE_CHANNEL_STATUS_EXPOSURE_JSON" \
  --argjson surfaces "$delivery_receipt_distribution_artifact_manifest_status_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_v1",
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_mode:"denied_package_release_channel_status_cannot_be_exposed_as_distribution_artifact_manifest_package_manifest_checksum_index_metadata_signing_notarization_or_live_status",
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_ready,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_report_sha256:$delivery_receipt_package_release_channel_status_exposure_report_sha256,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_contract_hash_sha256:$delivery_receipt_distribution_artifact_manifest_status_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_policy_hash_sha256:$delivery_receipt_distribution_artifact_manifest_status_policy_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_ready:true,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_surface_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_attempt_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_index_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_index_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_mirror_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_mirror_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_version_manifest_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_version_manifest_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_manifest_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_manifest_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_manifest_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_manifest_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_release_publication_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_release_publication_authority_derived_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_activation_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_activation_authority_derived_count,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_attempt_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_request_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_persisted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_materialized_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_filesystem_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_delivered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manifest_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_index_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_metadata_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_metadata_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_metadata_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_digest_manifest_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_manifest_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_package_manifest_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_channel_manifest_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_external_artifact_manifest_status_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_artifact_manifest_status_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_acceptance_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_operator_approval_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_publication_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_command_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_live_execution_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_install_executed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_service_restarted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_active_binary_mutated_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status:[
      "source_package_release_channel_status_exposure_report_required",
      "distribution_artifact_manifest_status_request_acceptance_denied",
      "distribution_artifact_manifest_status_acceptance_denied",
      "distribution_artifact_manifest_status_recording_denied",
      "distribution_artifact_manifest_status_persistence_denied",
      "distribution_artifact_manifest_status_materialization_denied",
      "distribution_artifact_manifest_status_filesystem_write_denied",
      "distribution_artifact_manifest_status_delivery_denied",
      "distribution_artifact_manifest_status_exposure_denied",
      "distribution_artifact_manifest_exposure_denied",
      "package_manifest_status_exposure_denied",
      "checksum_index_status_exposure_denied",
      "artifact_metadata_status_exposure_denied",
      "cdn_artifact_metadata_status_exposure_denied",
      "update_feed_artifact_metadata_status_exposure_denied",
      "package_signing_status_denied",
      "notarization_status_denied",
      "stapling_status_denied",
      "provenance_attestation_status_denied",
      "sbom_manifest_status_denied",
      "artifact_digest_manifest_status_denied",
      "release_asset_manifest_status_denied",
      "installer_package_manifest_status_denied",
      "package_channel_manifest_status_denied",
      "external_artifact_manifest_status_send_denied",
      "telegram_artifact_manifest_status_send_denied",
      "public_release_claim_from_artifact_manifest_denied",
      "public_ga_claim_from_artifact_manifest_denied",
      "acceptance_from_artifact_manifest_denied",
      "operator_approval_from_artifact_manifest_denied",
      "release_publication_authority_from_artifact_manifest_denied",
      "activation_live_from_artifact_manifest_denied",
      "install_restart_active_binary_from_artifact_manifest_denied",
      "memory_provider_kg_from_artifact_manifest_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_gate",
        status:"allowed_report_only_next_slice",
        exposes_distribution_artifact_manifest_status:false,
        writes_package_manifest:false,
        writes_checksum_index:false,
        writes_cdn_update_feed_metadata:false,
        records_package_signing:false,
        records_notarization:false,
        records_operator_acceptance:false,
        derives_release_publication_authority:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false,
        sends_externally:false
      }
    ],
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_request_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manifest_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_index_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_metadata_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_metadata_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_metadata_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_digest_manifest_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_manifest_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_package_manifest_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_channel_manifest_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted:false,
    packet_acceptance_receipt_release_publication_recorded:false,
    operator_acceptance_recorded:false,
    operator_approval_recorded:false,
    release_publication_authority_derived:false,
    activation_authority_derived:false,
    activation_command_derived:false,
    activation_allowed:false,
    activation_performed:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    live_kg_write_performed:false,
    provider_invoked:false,
    model_invoked:false,
    credential_read:false,
    secret_file_read:false,
    install_executed:false,
    launchd_mutated:false,
    service_restarted:false,
    active_binary_mutated:false,
    public_release_claimed:false,
    public_ga_claimed:false,
    release_artifact_written:false,
    public_artifact_written:false,
    external_send_performed:false,
    side_effects:{
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_filesystem_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manifest_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_index_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_metadata_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_metadata_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_metadata_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_digest_manifest_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_manifest_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_package_manifest_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_channel_manifest_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_artifact_manifest_status_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_artifact_manifest_status_sent:false,
      operator_acceptance_recorded:false,
      operator_approval_recorded:false,
      release_publication_authority_derived:false,
      activation_authority_derived:false,
      activation_command_derived:false,
      activation_allowed:false,
      activation_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      secret_file_read:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      release_artifact_written:false,
      public_artifact_written:false,
      public_release_claimed:false,
      public_ga_claimed:false,
      external_send_performed:false,
      filesystem_written:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_ready == true
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_surface_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_attempt_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_index_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_mirror_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_version_manifest_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_manifest_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_manifest_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_release_publication_authority_derived_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_request_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_persisted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_materialized_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_filesystem_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_delivered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manifest_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_index_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_metadata_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_metadata_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_metadata_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_digest_manifest_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_manifest_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_package_manifest_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_channel_manifest_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_external_artifact_manifest_status_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_artifact_manifest_status_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_acceptance_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_operator_approval_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_command_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_live_execution_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_install_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_service_restarted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_active_binary_mutated_count == 0
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surfaces | length) == 18
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surfaces | all(
    .distribution_artifact_manifest_status_attempted == true
    and .distribution_artifact_manifest_status_allowed == false
    and .distribution_artifact_manifest_status_request_accepted == false
    and .distribution_artifact_manifest_status_accepted == false
    and .distribution_artifact_manifest_status_recorded == false
    and .distribution_artifact_manifest_status_persisted == false
    and .distribution_artifact_manifest_status_materialized == false
    and .distribution_artifact_manifest_status_filesystem_written == false
    and .distribution_artifact_manifest_status_delivered == false
    and .distribution_artifact_manifest_status_exposed == false
    and .distribution_artifact_manifest_exposed == false
    and .package_manifest_status_exposed == false
    and .checksum_index_status_exposed == false
    and .artifact_metadata_status_exposed == false
    and .cdn_artifact_metadata_status_exposed == false
    and .update_feed_artifact_metadata_status_exposed == false
    and .package_signing_status_exposed == false
    and .notarization_status_exposed == false
    and .stapling_status_exposed == false
    and .provenance_attestation_status_exposed == false
    and .sbom_manifest_status_exposed == false
    and .artifact_digest_manifest_status_exposed == false
    and .release_asset_manifest_status_exposed == false
    and .installer_package_manifest_status_exposed == false
    and .package_channel_manifest_status_exposed == false
    and .external_artifact_manifest_status_sent == false
    and .telegram_artifact_manifest_status_sent == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .distribution_artifact_manifest_status_noop_confirmed == true
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status | length) == 34
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manifest_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_index_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_metadata_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_metadata_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_metadata_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_digest_manifest_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_manifest_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_package_manifest_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_channel_manifest_status_exposed == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .release_publication_authority_derived == false
  and .activation_authority_derived == false
  and .activation_command_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .secret_file_read == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and .public_artifact_written == false
  and .external_send_performed == false
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt distribution artifact/manifest status denial gate passed"
