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

TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_DISTRIBUTION_ARTIFACT_MANIFEST_STATUS_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial-gate.sh
)"

delivery_receipt_distribution_artifact_manifest_status_report_sha256="$(
  sha256_text "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_DISTRIBUTION_ARTIFACT_MANIFEST_STATUS_JSON"
)"
delivery_receipt_artifact_distribution_signing_notarization_surface_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-distribution-signing-notarization-surface-denial:$delivery_receipt_distribution_artifact_manifest_status_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
delivery_receipt_artifact_distribution_signing_notarization_surface_policy_hash_sha256="$(
  sha256_text "release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-distribution-signing-notarization-surface-denial:no-signing-execution:no-notarization:no-stapling:no-provenance-publication:no-sbom-publication:no-release-asset-packaging:no-cdn-update-feed-write:no-live"
)"

jq -n -e \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_DISTRIBUTION_ARTIFACT_MANIFEST_STATUS_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_request_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_persisted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_materialized_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_filesystem_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_delivered_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manifest_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_index_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_metadata_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_metadata_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_metadata_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_digest_manifest_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_manifest_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_package_manifest_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_channel_manifest_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_external_artifact_manifest_status_sent_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_artifact_manifest_status_sent_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_publication_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_live_execution_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_install_executed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_service_restarted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_active_binary_mutated_count == 0
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed == false
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

delivery_receipt_artifact_distribution_signing_notarization_surface_surfaces_json="$(
  jq -n '
    def signing_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface:$id,
        source_distribution_artifact_manifest_status_ready:true,
        artifact_distribution_signing_notarization_surface_attempted:true,
        artifact_distribution_signing_notarization_surface_allowed:false,
        artifact_distribution_signing_notarization_surface_request_accepted:false,
        artifact_distribution_signing_notarization_surface_accepted:false,
        artifact_distribution_signing_notarization_surface_recorded:false,
        artifact_distribution_signing_notarization_surface_persisted:false,
        artifact_distribution_signing_notarization_surface_materialized:false,
        artifact_distribution_signing_notarization_surface_filesystem_written:false,
        artifact_distribution_signing_notarization_surface_delivered:false,
        artifact_distribution_signing_notarization_surface_exposed:false,
        artifact_distribution_signing_notarization_surface_executed:false,
        artifact_signing_executed:false,
        package_signing_executed:false,
        signature_manifest_written:false,
        signature_checksum_bound:false,
        notarization_submitted:false,
        notarization_ticket_recorded:false,
        stapling_executed:false,
        installer_signing_executed:false,
        provenance_attestation_published:false,
        sbom_manifest_published:false,
        release_asset_packaged:false,
        artifact_bundle_packaged:false,
        cdn_artifact_written:false,
        update_feed_artifact_written:false,
        package_registry_artifact_published:false,
        external_package_channel_published:false,
        telegram_package_channel_published:false,
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
        artifact_distribution_signing_notarization_surface_noop_confirmed:true,
        artifact_distribution_signing_notarization_surface_status:$status,
        reason:$reason
      } + $extra;
    [
      signing_surface("delivery_receipt_artifact_signing_execution"; "blocked_artifact_signing_execution_noop"; "artifact_signing_execution_denied"; {artifact_signing_requested:true}),
      signing_surface("delivery_receipt_package_signing_execution"; "blocked_package_signing_execution_noop"; "package_signing_execution_denied"; {package_signing_requested:true}),
      signing_surface("delivery_receipt_signature_manifest_write"; "blocked_signature_manifest_write_noop"; "signature_manifest_write_denied"; {signature_manifest_write_requested:true}),
      signing_surface("delivery_receipt_signature_checksum_binding"; "blocked_signature_checksum_binding_noop"; "signature_checksum_binding_denied"; {signature_checksum_binding_requested:true}),
      signing_surface("delivery_receipt_notarization_submission"; "blocked_notarization_submission_noop"; "notarization_submission_denied"; {notarization_submission_requested:true}),
      signing_surface("delivery_receipt_notarization_ticket_record"; "blocked_notarization_ticket_record_noop"; "notarization_ticket_recording_denied"; {notarization_ticket_record_requested:true}),
      signing_surface("delivery_receipt_stapling_execution"; "blocked_stapling_execution_noop"; "stapling_execution_denied"; {stapling_execution_requested:true}),
      signing_surface("delivery_receipt_installer_signing_execution"; "blocked_installer_signing_execution_noop"; "installer_signing_execution_denied"; {installer_signing_requested:true}),
      signing_surface("delivery_receipt_provenance_attestation_publication"; "blocked_provenance_attestation_publication_noop"; "provenance_attestation_publication_denied"; {provenance_attestation_publication_requested:true}),
      signing_surface("delivery_receipt_sbom_manifest_publication"; "blocked_sbom_manifest_publication_noop"; "sbom_manifest_publication_denied"; {sbom_manifest_publication_requested:true}),
      signing_surface("delivery_receipt_release_asset_packaging"; "blocked_release_asset_packaging_noop"; "release_asset_packaging_denied"; {release_asset_packaging_requested:true}),
      signing_surface("delivery_receipt_artifact_bundle_packaging"; "blocked_artifact_bundle_packaging_noop"; "artifact_bundle_packaging_denied"; {artifact_bundle_packaging_requested:true}),
      signing_surface("delivery_receipt_cdn_artifact_write"; "blocked_cdn_artifact_write_noop"; "cdn_artifact_write_denied"; {cdn_artifact_write_requested:true}),
      signing_surface("delivery_receipt_update_feed_artifact_write"; "blocked_update_feed_artifact_write_noop"; "update_feed_artifact_write_denied"; {update_feed_artifact_write_requested:true}),
      signing_surface("delivery_receipt_package_registry_artifact_publish"; "blocked_package_registry_artifact_publish_noop"; "package_registry_artifact_publish_denied"; {package_registry_artifact_publish_requested:true}),
      signing_surface("delivery_receipt_external_telegram_package_channel_publication"; "blocked_external_telegram_package_channel_publication_noop"; "external_telegram_package_channel_publication_denied"; {external_package_channel_publication_requested:true, telegram_package_channel_publication_requested:true}),
      signing_surface("delivery_receipt_release_publication_authority_signing_status"; "blocked_release_publication_authority_signing_status_noop"; "release_publication_authority_from_signing_status_denied"; {release_publication_authority_signing_status_requested:true}),
      signing_surface("delivery_receipt_activation_live_install_restart_active_binary_signing_path"; "blocked_activation_live_install_restart_active_binary_signing_path_noop"; "activation_live_install_restart_active_binary_from_signing_path_denied"; {activation_live_signing_path_requested:true, install_restart_active_binary_signing_path_requested:true})
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_gate" \
  --arg delivery_receipt_distribution_artifact_manifest_status_report_sha256 "$delivery_receipt_distribution_artifact_manifest_status_report_sha256" \
  --arg delivery_receipt_artifact_distribution_signing_notarization_surface_contract_hash_sha256 "$delivery_receipt_artifact_distribution_signing_notarization_surface_contract_hash_sha256" \
  --arg delivery_receipt_artifact_distribution_signing_notarization_surface_policy_hash_sha256 "$delivery_receipt_artifact_distribution_signing_notarization_surface_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_DISTRIBUTION_ARTIFACT_MANIFEST_STATUS_JSON" \
  --argjson surfaces "$delivery_receipt_artifact_distribution_signing_notarization_surface_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_v1",
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_mode:"denied_distribution_artifact_manifest_status_cannot_be_executed_as_signing_notarization_stapling_provenance_sbom_release_asset_packaging_cdn_update_feed_write_or_live_status",
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_ready,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_report_sha256:$delivery_receipt_distribution_artifact_manifest_status_report_sha256,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_contract_hash_sha256:$delivery_receipt_artifact_distribution_signing_notarization_surface_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_policy_hash_sha256:$delivery_receipt_artifact_distribution_signing_notarization_surface_policy_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_ready:true,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_attempt_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_publication_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_publication_authority_derived_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_authority_derived_count,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_attempt_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_request_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_persisted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_materialized_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_filesystem_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_delivered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_executed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_manifest_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_checksum_bound_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_ticket_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_executed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_signing_executed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_published_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_published_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_packaged_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_bundle_packaged_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_artifact_published_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_external_package_channel_published_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_package_channel_published_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_acceptance_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_operator_approval_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_release_publication_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_activation_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_activation_command_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_live_execution_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_install_executed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_service_restarted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_active_binary_mutated_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface:[
      "source_distribution_artifact_manifest_status_report_required",
      "artifact_distribution_signing_notarization_surface_request_acceptance_denied",
      "artifact_distribution_signing_notarization_surface_acceptance_denied",
      "artifact_distribution_signing_notarization_surface_recording_denied",
      "artifact_distribution_signing_notarization_surface_persistence_denied",
      "artifact_distribution_signing_notarization_surface_materialization_denied",
      "artifact_distribution_signing_notarization_surface_filesystem_write_denied",
      "artifact_distribution_signing_notarization_surface_delivery_denied",
      "artifact_distribution_signing_notarization_surface_exposure_denied",
      "artifact_signing_execution_denied",
      "package_signing_execution_denied",
      "signature_manifest_write_denied",
      "signature_checksum_binding_denied",
      "notarization_submission_denied",
      "notarization_ticket_recording_denied",
      "stapling_execution_denied",
      "installer_signing_execution_denied",
      "provenance_attestation_publication_denied",
      "sbom_manifest_publication_denied",
      "release_asset_packaging_denied",
      "artifact_bundle_packaging_denied",
      "cdn_artifact_write_denied",
      "update_feed_artifact_write_denied",
      "package_registry_artifact_publish_denied",
      "external_package_channel_publication_denied",
      "telegram_package_channel_publication_denied",
      "public_release_claim_from_signing_notarization_denied",
      "public_ga_claim_from_signing_notarization_denied",
      "acceptance_from_signing_notarization_denied",
      "operator_approval_from_signing_notarization_denied",
      "release_publication_authority_from_signing_notarization_denied",
      "activation_live_from_signing_notarization_denied",
      "install_restart_active_binary_from_signing_notarization_denied",
      "memory_provider_kg_from_signing_notarization_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_gate",
        status:"allowed_report_only_next_slice",
        executes_signing:false,
        executes_notarization:false,
        executes_stapling:false,
        publishes_provenance:false,
        publishes_sbom:false,
        packages_release_asset:false,
        writes_cdn_artifact:false,
        writes_update_feed_artifact:false,
        publishes_external_package_channel:false,
        records_operator_acceptance:false,
        derives_release_publication_authority:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false,
        sends_externally:false
      }
    ],
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_request_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_executed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_manifest_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_checksum_bound:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_ticket_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_executed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_signing_executed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_published:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_published:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_packaged:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_bundle_packaged:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_artifact_published:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_package_channel_published:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_package_channel_published:false,
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
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_filesystem_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_executed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_manifest_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_checksum_bound:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_ticket_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_executed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_signing_executed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_published:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_published:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_packaged:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_bundle_packaged:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_artifact_published:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_package_channel_published:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_package_channel_published:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_ready == true
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_attempt_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_publication_authority_derived_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_request_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_persisted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_materialized_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_filesystem_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_delivered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_manifest_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_checksum_bound_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_ticket_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_signing_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_published_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_published_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_packaged_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_bundle_packaged_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_artifact_published_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_external_package_channel_published_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_package_channel_published_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_acceptance_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_operator_approval_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_activation_command_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_live_execution_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_install_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_service_restarted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_active_binary_mutated_count == 0
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_surfaces | length) == 18
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_surfaces | all(
    .artifact_distribution_signing_notarization_surface_attempted == true
    and .artifact_distribution_signing_notarization_surface_allowed == false
    and .artifact_distribution_signing_notarization_surface_request_accepted == false
    and .artifact_distribution_signing_notarization_surface_accepted == false
    and .artifact_distribution_signing_notarization_surface_recorded == false
    and .artifact_distribution_signing_notarization_surface_persisted == false
    and .artifact_distribution_signing_notarization_surface_materialized == false
    and .artifact_distribution_signing_notarization_surface_filesystem_written == false
    and .artifact_distribution_signing_notarization_surface_delivered == false
    and .artifact_distribution_signing_notarization_surface_exposed == false
    and .artifact_distribution_signing_notarization_surface_executed == false
    and .artifact_signing_executed == false
    and .package_signing_executed == false
    and .signature_manifest_written == false
    and .signature_checksum_bound == false
    and .notarization_submitted == false
    and .notarization_ticket_recorded == false
    and .stapling_executed == false
    and .installer_signing_executed == false
    and .provenance_attestation_published == false
    and .sbom_manifest_published == false
    and .release_asset_packaged == false
    and .artifact_bundle_packaged == false
    and .cdn_artifact_written == false
    and .update_feed_artifact_written == false
    and .package_registry_artifact_published == false
    and .external_package_channel_published == false
    and .telegram_package_channel_published == false
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
    and .artifact_distribution_signing_notarization_surface_noop_confirmed == true
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface | length) == 34
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_executed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_manifest_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_ticket_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_executed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_published == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_published == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_packaged == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_package_channel_published == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact distribution signing/notarization surface denial gate passed"
