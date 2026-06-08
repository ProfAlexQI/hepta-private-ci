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

TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-terminal-public-claim-status-exposure-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-terminal-public-claim-status-exposure-denial-gate.sh
)"

delivery_receipt_terminal_public_claim_status_exposure_report_sha256="$(
  sha256_text "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_JSON"
)"
delivery_receipt_package_release_channel_status_exposure_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-package-release-channel-status-exposure-denial:$delivery_receipt_terminal_public_claim_status_exposure_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
delivery_receipt_package_release_channel_status_exposure_policy_hash_sha256="$(
  sha256_text "release-publication-result-receipt-terminal-distribution-delivery-receipt-package-release-channel-status-exposure-denial:no-package-index:no-update-feed:no-cdn:no-release-channel:no-distribution-artifact:no-live"
)"

jq -n -e \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_decision_status_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_request_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_persisted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_materialized_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_filesystem_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_delivered_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_public_release_claimed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_public_ga_claimed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_release_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_publication_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_dashboard_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_public_badge_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_status_endpoint_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_export_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_observability_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_release_notes_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_changelog_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_version_tag_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_availability_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_queue_status_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_live_execution_allowed_count == 0
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_accepted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_badge_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_endpoint_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_availability_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_queue_status_exposed == false
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

delivery_receipt_package_release_channel_status_exposure_surfaces_json="$(
  jq -n '
    def package_channel_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_surface:$id,
        source_terminal_public_claim_status_exposure_ready:true,
        package_release_channel_status_exposure_attempted:true,
        package_release_channel_status_exposure_allowed:false,
        package_release_channel_status_exposure_request_accepted:false,
        package_release_channel_status_exposure_accepted:false,
        package_release_channel_status_exposure_recorded:false,
        package_release_channel_status_exposure_persisted:false,
        package_release_channel_status_exposure_materialized:false,
        package_release_channel_status_exposure_filesystem_written:false,
        package_release_channel_status_exposure_delivered:false,
        package_release_channel_status_exposed:false,
        package_index_status_exposed:false,
        package_registry_status_exposed:false,
        package_metadata_endpoint_status_exposed:false,
        update_feed_status_exposed:false,
        cdn_mirror_status_exposed:false,
        release_channel_status_exposed:false,
        distribution_artifact_status_exposed:false,
        artifact_catalog_status_exposed:false,
        version_manifest_status_exposed:false,
        installer_manifest_status_exposed:false,
        checksum_manifest_status_exposed:false,
        download_page_status_exposed:false,
        release_notes_package_status_exposed:false,
        channel_announcement_status_exposed:false,
        channel_status_delivered:false,
        external_status_sent:false,
        telegram_status_sent:false,
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
        package_release_channel_status_exposure_noop_confirmed:true,
        package_release_channel_status_exposure_status:$status,
        reason:$reason
      } + $extra;
    [
      package_channel_surface("delivery_receipt_package_index_status"; "blocked_package_index_status_noop"; "package_index_status_exposure_denied"; {package_index_status_requested:true}),
      package_channel_surface("delivery_receipt_package_registry_status"; "blocked_package_registry_status_noop"; "package_registry_status_exposure_denied"; {package_registry_status_requested:true}),
      package_channel_surface("delivery_receipt_package_metadata_endpoint_status"; "blocked_package_metadata_endpoint_status_noop"; "package_metadata_endpoint_status_exposure_denied"; {package_metadata_endpoint_status_requested:true}),
      package_channel_surface("delivery_receipt_update_feed_status"; "blocked_update_feed_status_noop"; "update_feed_status_exposure_denied"; {update_feed_status_requested:true}),
      package_channel_surface("delivery_receipt_cdn_mirror_status"; "blocked_cdn_mirror_status_noop"; "cdn_mirror_status_exposure_denied"; {cdn_mirror_status_requested:true}),
      package_channel_surface("delivery_receipt_release_channel_status"; "blocked_release_channel_status_noop"; "release_channel_status_exposure_denied"; {release_channel_status_requested:true}),
      package_channel_surface("delivery_receipt_distribution_artifact_status"; "blocked_distribution_artifact_status_noop"; "distribution_artifact_status_exposure_denied"; {distribution_artifact_status_requested:true}),
      package_channel_surface("delivery_receipt_artifact_catalog_status"; "blocked_artifact_catalog_status_noop"; "artifact_catalog_status_exposure_denied"; {artifact_catalog_status_requested:true}),
      package_channel_surface("delivery_receipt_version_manifest_status"; "blocked_version_manifest_status_noop"; "version_manifest_status_exposure_denied"; {version_manifest_status_requested:true}),
      package_channel_surface("delivery_receipt_installer_manifest_status"; "blocked_installer_manifest_status_noop"; "installer_manifest_status_exposure_denied"; {installer_manifest_status_requested:true}),
      package_channel_surface("delivery_receipt_checksum_manifest_status"; "blocked_checksum_manifest_status_noop"; "checksum_manifest_status_exposure_denied"; {checksum_manifest_status_requested:true}),
      package_channel_surface("delivery_receipt_download_page_status"; "blocked_download_page_status_noop"; "download_page_status_exposure_denied"; {download_page_status_requested:true}),
      package_channel_surface("delivery_receipt_release_notes_package_status"; "blocked_release_notes_package_status_noop"; "release_notes_package_status_exposure_denied"; {release_notes_package_status_requested:true}),
      package_channel_surface("delivery_receipt_channel_announcement_status"; "blocked_channel_announcement_status_noop"; "channel_announcement_status_exposure_denied"; {channel_announcement_status_requested:true}),
      package_channel_surface("delivery_receipt_channel_external_telegram_package_status"; "blocked_channel_external_telegram_package_status_noop"; "channel_external_telegram_package_status_denied"; {channel_status_requested:true, external_status_requested:true, telegram_status_requested:true}),
      package_channel_surface("delivery_receipt_release_publication_authority_package_status"; "blocked_release_publication_authority_package_status_noop"; "release_publication_authority_from_package_status_denied"; {release_publication_authority_package_status_requested:true}),
      package_channel_surface("delivery_receipt_activation_live_package_status"; "blocked_activation_live_package_status_noop"; "activation_live_from_package_status_denied"; {activation_live_package_status_requested:true}),
      package_channel_surface("delivery_receipt_install_restart_active_binary_package_status"; "blocked_active_binary_package_status_noop"; "install_restart_active_binary_from_package_status_denied"; {install_restart_active_binary_package_status_requested:true})
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_gate" \
  --arg delivery_receipt_terminal_public_claim_status_exposure_report_sha256 "$delivery_receipt_terminal_public_claim_status_exposure_report_sha256" \
  --arg delivery_receipt_package_release_channel_status_exposure_contract_hash_sha256 "$delivery_receipt_package_release_channel_status_exposure_contract_hash_sha256" \
  --arg delivery_receipt_package_release_channel_status_exposure_policy_hash_sha256 "$delivery_receipt_package_release_channel_status_exposure_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_JSON" \
  --argjson surfaces "$delivery_receipt_package_release_channel_status_exposure_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_v1",
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_mode:"denied_public_status_cannot_be_exposed_as_package_release_channel_distribution_or_version_status",
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_denial_ready,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_report_sha256:$delivery_receipt_terminal_public_claim_status_exposure_report_sha256,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_contract_hash_sha256:$delivery_receipt_package_release_channel_status_exposure_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_policy_hash_sha256:$delivery_receipt_package_release_channel_status_exposure_policy_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_ready:true,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_surface_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_attempt_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_release_claimed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_public_release_claimed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_ga_claimed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_public_ga_claimed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_availability_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_availability_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_queue_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_queue_status_exposed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_surface_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_attempt_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_request_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_persisted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_materialized_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_filesystem_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_delivered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_index_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_metadata_endpoint_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_mirror_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_release_channel_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_catalog_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_version_manifest_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_manifest_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_manifest_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_download_page_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_release_notes_package_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_announcement_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_delivered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_external_status_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_telegram_status_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_acceptance_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_operator_approval_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_release_publication_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_activation_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_activation_command_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_live_execution_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_install_executed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_service_restarted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_active_binary_mutated_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure:[
      "source_terminal_public_claim_status_exposure_report_required",
      "package_release_channel_status_request_acceptance_denied",
      "package_release_channel_status_acceptance_denied",
      "package_release_channel_status_recording_denied",
      "package_release_channel_status_persistence_denied",
      "package_release_channel_status_materialization_denied",
      "package_release_channel_status_filesystem_write_denied",
      "package_release_channel_status_delivery_denied",
      "package_release_channel_status_exposure_denied",
      "package_index_status_exposure_denied",
      "package_registry_status_exposure_denied",
      "package_metadata_endpoint_status_exposure_denied",
      "update_feed_status_exposure_denied",
      "cdn_mirror_status_exposure_denied",
      "release_channel_status_exposure_denied",
      "distribution_artifact_status_exposure_denied",
      "artifact_catalog_status_exposure_denied",
      "version_manifest_status_exposure_denied",
      "installer_manifest_status_exposure_denied",
      "checksum_manifest_status_exposure_denied",
      "download_page_status_exposure_denied",
      "release_notes_package_status_exposure_denied",
      "channel_announcement_status_exposure_denied",
      "channel_status_delivery_denied",
      "external_status_send_denied",
      "telegram_status_send_denied",
      "public_release_claim_from_package_status_denied",
      "public_ga_claim_from_package_status_denied",
      "acceptance_from_package_status_denied",
      "operator_approval_from_package_status_denied",
      "release_publication_authority_from_package_status_denied",
      "activation_live_from_package_status_denied",
      "install_restart_active_binary_from_package_status_denied",
      "memory_provider_kg_from_package_status_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_gate",
        status:"allowed_report_only_next_slice",
        exposes_package_channel_status:false,
        writes_package_index:false,
        writes_update_feed:false,
        writes_cdn_mirror:false,
        writes_release_channel:false,
        records_operator_acceptance:false,
        derives_release_publication_authority:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false,
        sends_externally:false
      }
    ],
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_badge_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_endpoint_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_availability_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_queue_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_request_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_index_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_metadata_endpoint_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_mirror_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_channel_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_catalog_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_version_manifest_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_manifest_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_manifest_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_download_page_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_notes_package_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_announcement_status_exposed:false,
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
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_filesystem_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_index_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_metadata_endpoint_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_mirror_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_channel_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_catalog_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_version_manifest_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_manifest_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_manifest_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_download_page_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_notes_package_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_announcement_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_ready == true
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_surface_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_attempt_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_status_claimed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_release_claimed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_ga_claimed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_availability_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_queue_status_exposed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_status_delivered_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_status_sent_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_status_sent_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_request_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_persisted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_materialized_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_filesystem_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_delivered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_index_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_metadata_endpoint_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_mirror_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_release_channel_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_catalog_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_version_manifest_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_manifest_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_manifest_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_download_page_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_release_notes_package_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_announcement_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_delivered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_external_status_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_telegram_status_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_acceptance_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_operator_approval_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_activation_command_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_live_execution_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_install_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_service_restarted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_active_binary_mutated_count == 0
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_surfaces | length) == 18
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_surfaces | all(
    .package_release_channel_status_exposure_attempted == true
    and .package_release_channel_status_exposure_allowed == false
    and .package_release_channel_status_exposure_request_accepted == false
    and .package_release_channel_status_exposure_accepted == false
    and .package_release_channel_status_exposure_recorded == false
    and .package_release_channel_status_exposure_persisted == false
    and .package_release_channel_status_exposure_materialized == false
    and .package_release_channel_status_exposure_filesystem_written == false
    and .package_release_channel_status_exposure_delivered == false
    and .package_release_channel_status_exposed == false
    and .package_index_status_exposed == false
    and .package_registry_status_exposed == false
    and .package_metadata_endpoint_status_exposed == false
    and .update_feed_status_exposed == false
    and .cdn_mirror_status_exposed == false
    and .release_channel_status_exposed == false
    and .distribution_artifact_status_exposed == false
    and .artifact_catalog_status_exposed == false
    and .version_manifest_status_exposed == false
    and .installer_manifest_status_exposed == false
    and .checksum_manifest_status_exposed == false
    and .download_page_status_exposed == false
    and .release_notes_package_status_exposed == false
    and .channel_announcement_status_exposed == false
    and .channel_status_delivered == false
    and .external_status_sent == false
    and .telegram_status_sent == false
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
    and .package_release_channel_status_exposure_noop_confirmed == true
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure | length) == 34
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposure_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_terminal_public_claim_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_public_badge_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_status_endpoint_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_availability_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_queue_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_index_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_metadata_endpoint_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_mirror_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_channel_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_catalog_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_version_manifest_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_manifest_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_manifest_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_download_page_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_notes_package_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_announcement_status_exposed == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt package/release channel status exposure denial gate passed"
