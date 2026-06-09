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

TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_SURFACE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-distribution-signing-notarization-surface-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-distribution-signing-notarization-surface-denial-gate.sh
)"

delivery_receipt_artifact_distribution_signing_notarization_surface_report_sha256="$(
  sha256_text "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_SURFACE_JSON"
)"
delivery_receipt_artifact_download_install_affordance_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial:$delivery_receipt_artifact_distribution_signing_notarization_surface_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
delivery_receipt_artifact_download_install_affordance_policy_hash_sha256="$(
  sha256_text "release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial:no-download-link:no-install-command:no-installer-prompt:no-update-offer:no-external-install-message:no-live"
)"

jq -n -e \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_SURFACE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_request_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_persisted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_materialized_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_filesystem_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_delivered_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_executed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_manifest_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_ticket_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_stapling_executed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_signing_executed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_provenance_attestation_published_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_manifest_published_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_packaged_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_bundle_packaged_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_artifact_published_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_external_package_channel_published_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_package_channel_published_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_release_publication_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_activation_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_live_execution_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_install_executed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_service_restarted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_active_binary_mutated_count == 0
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_executed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted == false
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

delivery_receipt_artifact_download_install_affordance_surfaces_json="$(
  jq -n '
    def affordance_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance:$id,
        source_artifact_distribution_signing_notarization_surface_ready:true,
        artifact_download_install_affordance_attempted:true,
        artifact_download_install_affordance_allowed:false,
        artifact_download_install_affordance_request_accepted:false,
        artifact_download_install_affordance_accepted:false,
        artifact_download_install_affordance_recorded:false,
        artifact_download_install_affordance_persisted:false,
        artifact_download_install_affordance_materialized:false,
        artifact_download_install_affordance_filesystem_written:false,
        artifact_download_install_affordance_delivered:false,
        artifact_download_install_affordance_exposed:false,
        artifact_download_install_affordance_published:false,
        artifact_download_install_affordance_executed:false,
        download_button_rendered:false,
        direct_download_url_exposed:false,
        checksum_prompt_rendered:false,
        package_manager_install_command_rendered:false,
        curl_pipe_shell_snippet_rendered:false,
        installer_launch_prompt_rendered:false,
        auto_update_offer_rendered:false,
        release_channel_subscribe_prompt_rendered:false,
        update_feed_consumer_hint_rendered:false,
        package_registry_install_badge_rendered:false,
        cdn_mirror_download_link_rendered:false,
        sbom_provenance_download_link_rendered:false,
        notarization_ticket_download_link_rendered:false,
        signature_verification_command_rendered:false,
        one_click_install_deep_link_rendered:false,
        external_install_message_sent:false,
        telegram_install_message_sent:false,
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
        artifact_download_install_affordance_noop_confirmed:true,
        artifact_download_install_affordance_status:$status,
        reason:$reason
      } + $extra;
    [
      affordance_surface("delivery_receipt_artifact_download_button"; "blocked_artifact_download_button_noop"; "artifact_download_button_denied"; {artifact_download_button_requested:true}),
      affordance_surface("delivery_receipt_direct_download_url"; "blocked_direct_download_url_noop"; "direct_download_url_denied"; {direct_download_url_requested:true}),
      affordance_surface("delivery_receipt_checksum_copy_prompt"; "blocked_checksum_copy_prompt_noop"; "checksum_copy_prompt_denied"; {checksum_copy_prompt_requested:true}),
      affordance_surface("delivery_receipt_package_manager_install_command"; "blocked_package_manager_install_command_noop"; "package_manager_install_command_denied"; {package_manager_install_command_requested:true}),
      affordance_surface("delivery_receipt_curl_pipe_shell_snippet"; "blocked_curl_pipe_shell_snippet_noop"; "curl_pipe_shell_snippet_denied"; {curl_pipe_shell_snippet_requested:true}),
      affordance_surface("delivery_receipt_installer_launch_prompt"; "blocked_installer_launch_prompt_noop"; "installer_launch_prompt_denied"; {installer_launch_prompt_requested:true}),
      affordance_surface("delivery_receipt_auto_update_offer"; "blocked_auto_update_offer_noop"; "auto_update_offer_denied"; {auto_update_offer_requested:true}),
      affordance_surface("delivery_receipt_release_channel_subscribe_prompt"; "blocked_release_channel_subscribe_prompt_noop"; "release_channel_subscribe_prompt_denied"; {release_channel_subscribe_prompt_requested:true}),
      affordance_surface("delivery_receipt_update_feed_consumer_hint"; "blocked_update_feed_consumer_hint_noop"; "update_feed_consumer_hint_denied"; {update_feed_consumer_hint_requested:true}),
      affordance_surface("delivery_receipt_package_registry_install_badge"; "blocked_package_registry_install_badge_noop"; "package_registry_install_badge_denied"; {package_registry_install_badge_requested:true}),
      affordance_surface("delivery_receipt_cdn_mirror_download_link"; "blocked_cdn_mirror_download_link_noop"; "cdn_mirror_download_link_denied"; {cdn_mirror_download_link_requested:true}),
      affordance_surface("delivery_receipt_sbom_provenance_download_link"; "blocked_sbom_provenance_download_link_noop"; "sbom_provenance_download_link_denied"; {sbom_provenance_download_link_requested:true}),
      affordance_surface("delivery_receipt_notarization_ticket_download_link"; "blocked_notarization_ticket_download_link_noop"; "notarization_ticket_download_link_denied"; {notarization_ticket_download_link_requested:true}),
      affordance_surface("delivery_receipt_signature_verification_command"; "blocked_signature_verification_command_noop"; "signature_verification_command_denied"; {signature_verification_command_requested:true}),
      affordance_surface("delivery_receipt_one_click_install_deep_link"; "blocked_one_click_install_deep_link_noop"; "one_click_install_deep_link_denied"; {one_click_install_deep_link_requested:true}),
      affordance_surface("delivery_receipt_external_telegram_install_message"; "blocked_external_telegram_install_message_noop"; "external_telegram_install_message_denied"; {external_install_message_requested:true, telegram_install_message_requested:true}),
      affordance_surface("delivery_receipt_release_publication_authority_install_affordance"; "blocked_release_publication_authority_install_affordance_noop"; "release_publication_authority_from_install_affordance_denied"; {release_publication_authority_install_affordance_requested:true}),
      affordance_surface("delivery_receipt_activation_live_install_restart_active_binary_affordance"; "blocked_activation_live_install_restart_active_binary_affordance_noop"; "activation_live_install_restart_active_binary_from_install_affordance_denied"; {activation_live_install_affordance_requested:true, install_restart_active_binary_affordance_requested:true})
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_gate" \
  --arg delivery_receipt_artifact_distribution_signing_notarization_surface_report_sha256 "$delivery_receipt_artifact_distribution_signing_notarization_surface_report_sha256" \
  --arg delivery_receipt_artifact_download_install_affordance_contract_hash_sha256 "$delivery_receipt_artifact_download_install_affordance_contract_hash_sha256" \
  --arg delivery_receipt_artifact_download_install_affordance_policy_hash_sha256 "$delivery_receipt_artifact_download_install_affordance_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_SURFACE_JSON" \
  --argjson surfaces "$delivery_receipt_artifact_download_install_affordance_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_v1",
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_mode:"denied_signing_notarization_surface_cannot_be_reframed_as_download_link_install_command_installer_prompt_update_offer_external_install_message_or_live_authority",
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_denial_ready,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_report_sha256:$delivery_receipt_artifact_distribution_signing_notarization_surface_report_sha256,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_contract_hash_sha256:$delivery_receipt_artifact_download_install_affordance_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_policy_hash_sha256:$delivery_receipt_artifact_download_install_affordance_policy_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_ready:true,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_attempt_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_packaged_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_packaged_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_written_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_written_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_written_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_written_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_release_publication_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_release_publication_authority_derived_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_activation_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_activation_authority_derived_count,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_attempt_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_request_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_materialized_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_filesystem_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_delivered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_published_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_prompt_rendered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_curl_pipe_shell_snippet_rendered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_launch_prompt_rendered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_auto_update_offer_rendered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_release_channel_subscribe_prompt_rendered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_consumer_hint_rendered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_registry_install_badge_rendered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_mirror_download_link_rendered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_sbom_provenance_download_link_rendered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_ticket_download_link_rendered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_signature_verification_command_rendered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_one_click_install_deep_link_rendered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_install_message_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_acceptance_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_command_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_executed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restarted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_mutated_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance:[
      "source_artifact_distribution_signing_notarization_surface_report_required",
      "artifact_download_install_affordance_request_acceptance_denied",
      "artifact_download_install_affordance_acceptance_denied",
      "artifact_download_install_affordance_recording_denied",
      "artifact_download_install_affordance_persistence_denied",
      "artifact_download_install_affordance_materialization_denied",
      "artifact_download_install_affordance_filesystem_write_denied",
      "artifact_download_install_affordance_delivery_denied",
      "artifact_download_install_affordance_exposure_denied",
      "artifact_download_install_affordance_publication_denied",
      "artifact_download_install_affordance_execution_denied",
      "download_button_rendering_denied",
      "direct_download_url_exposure_denied",
      "package_manager_install_command_denied",
      "curl_pipe_shell_snippet_denied",
      "installer_launch_prompt_denied",
      "auto_update_offer_denied",
      "release_channel_subscribe_prompt_denied",
      "update_feed_consumer_hint_denied",
      "package_registry_install_badge_denied",
      "cdn_mirror_download_link_denied",
      "sbom_provenance_download_link_denied",
      "notarization_ticket_download_link_denied",
      "signature_verification_command_denied",
      "one_click_install_deep_link_denied",
      "external_install_message_denied",
      "telegram_install_message_denied",
      "operator_approval_from_install_affordance_denied",
      "release_publication_authority_from_install_affordance_denied",
      "activation_live_from_install_affordance_denied",
      "install_restart_active_binary_from_install_affordance_denied",
      "memory_provider_kg_from_install_affordance_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_gate",
        status:"allowed_report_only_next_slice",
        renders_download_link:false,
        emits_install_command:false,
        prompts_installer:false,
        publishes_update_offer:false,
        sends_external_install_message:false,
        records_operator_acceptance:false,
        derives_release_publication_authority:false,
        derives_activation_authority:false,
        activates_live:false,
        installs_or_restarts:false,
        mutates_active_binary:false,
        mutates_memory_store:false,
        writes_kg:false,
        sends_externally:false
      }
    ],
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_request_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_published:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_curl_pipe_shell_snippet_rendered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_launch_prompt_rendered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_auto_update_offer_rendered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_install_message_sent:false,
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
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_filesystem_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_published:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_curl_pipe_shell_snippet_rendered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_launch_prompt_rendered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_auto_update_offer_rendered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_install_message_sent:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_ready == true
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_attempt_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_signing_executed_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_submitted_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_asset_packaged_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_cdn_artifact_written_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_update_feed_artifact_written_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_release_publication_authority_derived_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_request_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_materialized_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_filesystem_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_delivered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_published_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_curl_pipe_shell_snippet_rendered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_launch_prompt_rendered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_auto_update_offer_rendered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_install_message_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_acceptance_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_command_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restarted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_mutated_count == 0
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surfaces | length) == 18
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surfaces | all(
    .artifact_download_install_affordance_attempted == true
    and .artifact_download_install_affordance_allowed == false
    and .artifact_download_install_affordance_request_accepted == false
    and .artifact_download_install_affordance_accepted == false
    and .artifact_download_install_affordance_recorded == false
    and .artifact_download_install_affordance_persisted == false
    and .artifact_download_install_affordance_materialized == false
    and .artifact_download_install_affordance_filesystem_written == false
    and .artifact_download_install_affordance_delivered == false
    and .artifact_download_install_affordance_exposed == false
    and .artifact_download_install_affordance_published == false
    and .artifact_download_install_affordance_executed == false
    and .download_button_rendered == false
    and .direct_download_url_exposed == false
    and .package_manager_install_command_rendered == false
    and .curl_pipe_shell_snippet_rendered == false
    and .installer_launch_prompt_rendered == false
    and .auto_update_offer_rendered == false
    and .external_install_message_sent == false
    and .telegram_install_message_sent == false
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
    and .artifact_download_install_affordance_noop_confirmed == true
  ))
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_executed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_published == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_curl_pipe_shell_snippet_rendered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_launch_prompt_rendered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_auto_update_offer_rendered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_install_message_sent == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance denial gate passed"
