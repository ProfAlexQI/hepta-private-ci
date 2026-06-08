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

TERMINAL_PUBLIC_STATUS_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-public-claim-status-exposure-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-public-claim-status-exposure-denial-gate.sh
)"

terminal_public_status_report_sha256="$(sha256_text "$TERMINAL_PUBLIC_STATUS_JSON")"
terminal_distribution_artifact_status_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-queue-artifact-availability-status-denial:$terminal_public_status_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
terminal_distribution_artifact_status_policy_hash_sha256="$(
  sha256_text "release-publication-result-receipt-terminal-distribution-artifact-status-denial:no-distribution-queue:no-artifact-availability:no-download-url:no-feed:no-channel:no-active-binary"
)"

jq -n -e \
  --argjson source "$TERMINAL_PUBLIC_STATUS_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_ready == true
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count == 18
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_request_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_persisted_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_materialized_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_filesystem_written_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_delivered_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposed_count == 0
    and $source.release_publication_result_receipt_public_status_claimed_count == 0
    and $source.release_publication_result_receipt_public_release_claimed_count == 0
    and $source.release_publication_result_receipt_public_ga_claimed_count == 0
    and $source.release_publication_result_receipt_release_status_exposed_count == 0
    and $source.release_publication_result_receipt_publication_status_exposed_count == 0
    and $source.release_publication_result_receipt_dashboard_status_exposed_count == 0
    and $source.release_publication_result_receipt_public_badge_exposed_count == 0
    and $source.release_publication_result_receipt_status_endpoint_exposed_count == 0
    and $source.release_publication_result_receipt_query_status_exposed_count == 0
    and $source.release_publication_result_receipt_export_status_exposed_count == 0
    and $source.release_publication_result_receipt_observability_status_exposed_count == 0
    and $source.release_publication_result_receipt_release_notes_status_exposed_count == 0
    and $source.release_publication_result_receipt_changelog_status_exposed_count == 0
    and $source.release_publication_result_receipt_version_tag_status_exposed_count == 0
    and $source.release_publication_result_receipt_artifact_availability_status_exposed_count == 0
    and $source.release_publication_result_receipt_distribution_queue_status_exposed_count == 0
    and $source.release_publication_result_receipt_channel_status_delivered_count == 0
    and $source.release_publication_result_receipt_external_status_sent_count == 0
    and $source.release_publication_result_receipt_telegram_status_sent_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_acceptance_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_operator_approval_derived_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_activation_command_derived_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_live_execution_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_install_executed_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_service_restarted_count == 0
    and $source.release_publication_result_receipt_terminal_public_claim_status_exposure_active_binary_mutated_count == 0
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_accepted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_public_badge_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_query_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_export_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_observability_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_release_notes_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_changelog_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_version_tag_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_artifact_availability_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_status_exposed == false
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
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

terminal_distribution_artifact_status_surfaces_json="$(
  jq -n '
    def distribution_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_artifact_status_surface:$id,
        source_terminal_public_claim_status_exposure_ready:true,
        terminal_distribution_artifact_status_attempted:true,
        terminal_distribution_artifact_status_allowed:false,
        terminal_distribution_artifact_status_request_accepted:false,
        terminal_distribution_artifact_status_accepted:false,
        terminal_distribution_artifact_status_recorded:false,
        terminal_distribution_artifact_status_persisted:false,
        terminal_distribution_artifact_status_materialized:false,
        terminal_distribution_artifact_status_filesystem_written:false,
        terminal_distribution_artifact_status_delivered:false,
        terminal_distribution_artifact_status_exposed:false,
        distribution_queue_status_exposed:false,
        distribution_queue_enqueued:false,
        distribution_worker_dispatched:false,
        artifact_availability_status_exposed:false,
        artifact_manifest_entry_exposed:false,
        artifact_download_url_exposed:false,
        artifact_checksum_exposed:false,
        artifact_signature_notarization_exposed:false,
        package_index_status_exposed:false,
        update_feed_status_exposed:false,
        cdn_mirror_status_exposed:false,
        release_channel_status_exposed:false,
        public_bucket_listing_status_exposed:false,
        status_endpoint_artifact_ready_exposed:false,
        dashboard_artifact_available_badge_exposed:false,
        channel_status_delivered:false,
        external_status_sent:false,
        telegram_status_sent:false,
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
        release_artifact_written:false,
        public_artifact_written:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        terminal_distribution_artifact_status_noop_confirmed:true,
        terminal_distribution_artifact_status:$status,
        reason:$reason
      } + $extra;
    [
      distribution_surface("publication_result_receipt_distribution_queue_ready_status"; "blocked_distribution_queue_ready_noop"; "distribution_queue_ready_status_denied"; {distribution_queue_ready_requested:true}),
      distribution_surface("publication_result_receipt_distribution_queue_enqueued_status"; "blocked_distribution_queue_enqueued_noop"; "distribution_queue_enqueue_denied"; {distribution_queue_enqueue_requested:true}),
      distribution_surface("publication_result_receipt_distribution_worker_dispatch_status"; "blocked_distribution_worker_dispatch_noop"; "distribution_worker_dispatch_denied"; {distribution_worker_dispatch_requested:true}),
      distribution_surface("publication_result_receipt_artifact_availability_ready_status"; "blocked_artifact_availability_ready_noop"; "artifact_availability_ready_status_denied"; {artifact_availability_ready_requested:true}),
      distribution_surface("publication_result_receipt_artifact_manifest_entry_status"; "blocked_artifact_manifest_entry_noop"; "artifact_manifest_entry_denied"; {artifact_manifest_entry_requested:true}),
      distribution_surface("publication_result_receipt_artifact_download_url_status"; "blocked_artifact_download_url_noop"; "artifact_download_url_denied"; {artifact_download_url_requested:true}),
      distribution_surface("publication_result_receipt_artifact_checksum_status"; "blocked_artifact_checksum_noop"; "artifact_checksum_exposure_denied"; {artifact_checksum_requested:true}),
      distribution_surface("publication_result_receipt_artifact_signature_notarization_status"; "blocked_artifact_signature_notarization_noop"; "artifact_signature_notarization_status_denied"; {artifact_signature_notarization_requested:true}),
      distribution_surface("publication_result_receipt_package_index_status"; "blocked_package_index_status_noop"; "package_index_status_denied"; {package_index_status_requested:true}),
      distribution_surface("publication_result_receipt_update_feed_status"; "blocked_update_feed_status_noop"; "update_feed_status_denied"; {update_feed_status_requested:true}),
      distribution_surface("publication_result_receipt_cdn_mirror_status"; "blocked_cdn_mirror_status_noop"; "cdn_mirror_status_denied"; {cdn_mirror_status_requested:true}),
      distribution_surface("publication_result_receipt_release_channel_status"; "blocked_release_channel_status_noop"; "release_channel_status_denied"; {release_channel_status_requested:true}),
      distribution_surface("publication_result_receipt_public_bucket_listing_status"; "blocked_public_bucket_listing_noop"; "public_bucket_listing_status_denied"; {public_bucket_listing_status_requested:true}),
      distribution_surface("publication_result_receipt_status_endpoint_artifact_ready_status"; "blocked_status_endpoint_artifact_ready_noop"; "status_endpoint_artifact_ready_denied"; {status_endpoint_artifact_ready_requested:true}),
      distribution_surface("publication_result_receipt_dashboard_artifact_available_badge_status"; "blocked_dashboard_artifact_available_badge_noop"; "dashboard_artifact_available_badge_denied"; {dashboard_artifact_available_badge_requested:true}),
      distribution_surface("publication_result_receipt_channel_external_telegram_distribution_status"; "blocked_channel_external_telegram_distribution_status_noop"; "channel_external_telegram_distribution_status_denied"; {channel_status_requested:true, external_status_requested:true, telegram_status_requested:true}),
      distribution_surface("publication_result_receipt_release_publication_authority_distribution_status"; "blocked_release_publication_authority_distribution_status_noop"; "release_publication_authority_from_distribution_status_denied"; {release_publication_authority_distribution_status_requested:true}),
      distribution_surface("publication_result_receipt_activation_live_install_restart_active_binary_distribution_status"; "blocked_activation_live_active_binary_distribution_status_noop"; "activation_live_active_binary_from_distribution_status_denied"; {activation_live_distribution_status_requested:true, install_restart_active_binary_distribution_status_requested:true})
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_gate" \
  --arg terminal_public_status_report_sha256 "$terminal_public_status_report_sha256" \
  --arg terminal_distribution_artifact_status_contract_hash_sha256 "$terminal_distribution_artifact_status_contract_hash_sha256" \
  --arg terminal_distribution_artifact_status_policy_hash_sha256 "$terminal_distribution_artifact_status_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$TERMINAL_PUBLIC_STATUS_JSON" \
  --argjson surfaces "$terminal_distribution_artifact_status_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_terminal_distribution_artifact_status_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_denial_v1",
    receipt_release_publication_result_receipt_terminal_distribution_artifact_status_mode:"denied_terminal_public_status_cannot_become_distribution_queue_or_artifact_availability_status",
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_ready,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_report_sha256:$terminal_public_status_report_sha256,
    source_release_publication_result_receipt_terminal_public_claim_status_exposure_contract_hash_sha256:$source.release_publication_result_receipt_terminal_public_claim_status_exposure_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_artifact_status_contract_hash_sha256:$terminal_distribution_artifact_status_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_artifact_status_policy_hash_sha256:$terminal_distribution_artifact_status_policy_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_ready:true,
    source_release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count:$source.release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count,
    source_release_publication_result_receipt_terminal_public_claim_status_exposure_attempt_count:$source.release_publication_result_receipt_terminal_public_claim_status_exposure_attempt_count,
    source_release_publication_result_receipt_terminal_public_claim_status_exposure_accepted_count:$source.release_publication_result_receipt_terminal_public_claim_status_exposure_accepted_count,
    source_release_publication_result_receipt_terminal_public_claim_status_exposure_recorded_count:$source.release_publication_result_receipt_terminal_public_claim_status_exposure_recorded_count,
    source_release_publication_result_receipt_terminal_public_claim_status_exposed_count:$source.release_publication_result_receipt_terminal_public_claim_status_exposed_count,
    source_release_publication_result_receipt_artifact_availability_status_exposed_count:$source.release_publication_result_receipt_artifact_availability_status_exposed_count,
    source_release_publication_result_receipt_distribution_queue_status_exposed_count:$source.release_publication_result_receipt_distribution_queue_status_exposed_count,
    source_release_publication_result_receipt_channel_status_delivered_count:$source.release_publication_result_receipt_channel_status_delivered_count,
    source_release_publication_result_receipt_external_status_sent_count:$source.release_publication_result_receipt_external_status_sent_count,
    source_release_publication_result_receipt_telegram_status_sent_count:$source.release_publication_result_receipt_telegram_status_sent_count,
    source_release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count:$source.release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count,
    source_release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count:$source.release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count,
    release_publication_result_receipt_terminal_distribution_artifact_status_surface_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_artifact_status_attempt_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_artifact_status_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_request_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_persisted_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_materialized_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_filesystem_written_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_delivered_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_exposed_count:0,
    release_publication_result_receipt_distribution_queue_status_exposed_count:0,
    release_publication_result_receipt_distribution_queue_enqueued_count:0,
    release_publication_result_receipt_distribution_worker_dispatched_count:0,
    release_publication_result_receipt_artifact_availability_status_exposed_count:0,
    release_publication_result_receipt_artifact_manifest_entry_exposed_count:0,
    release_publication_result_receipt_artifact_download_url_exposed_count:0,
    release_publication_result_receipt_artifact_checksum_exposed_count:0,
    release_publication_result_receipt_artifact_signature_notarization_exposed_count:0,
    release_publication_result_receipt_package_index_status_exposed_count:0,
    release_publication_result_receipt_update_feed_status_exposed_count:0,
    release_publication_result_receipt_cdn_mirror_status_exposed_count:0,
    release_publication_result_receipt_release_channel_status_exposed_count:0,
    release_publication_result_receipt_public_bucket_listing_status_exposed_count:0,
    release_publication_result_receipt_status_endpoint_artifact_ready_exposed_count:0,
    release_publication_result_receipt_dashboard_artifact_available_badge_exposed_count:0,
    release_publication_result_receipt_channel_status_delivered_count:0,
    release_publication_result_receipt_external_status_sent_count:0,
    release_publication_result_receipt_telegram_status_sent_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_acceptance_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_operator_approval_derived_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_release_publication_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_activation_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_activation_command_derived_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_live_execution_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_install_executed_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_service_restarted_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_active_binary_mutated_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_release_artifact_written_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_public_artifact_written_count:0,
    release_publication_result_receipt_terminal_distribution_artifact_status_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_artifact_status:[
      "source_terminal_public_claim_status_exposure_report_required",
      "distribution_artifact_status_request_acceptance_denied",
      "distribution_artifact_status_acceptance_denied",
      "distribution_artifact_status_recording_denied",
      "distribution_artifact_status_persistence_denied",
      "distribution_artifact_status_materialization_denied",
      "distribution_artifact_status_filesystem_write_denied",
      "distribution_artifact_status_delivery_denied",
      "distribution_artifact_status_exposure_denied",
      "distribution_queue_status_exposure_denied",
      "distribution_queue_enqueue_denied",
      "distribution_worker_dispatch_denied",
      "artifact_availability_status_exposure_denied",
      "artifact_manifest_entry_exposure_denied",
      "artifact_download_url_exposure_denied",
      "artifact_checksum_exposure_denied",
      "artifact_signature_notarization_status_denied",
      "package_index_status_exposure_denied",
      "update_feed_status_exposure_denied",
      "cdn_mirror_status_exposure_denied",
      "release_channel_status_exposure_denied",
      "public_bucket_listing_status_exposure_denied",
      "status_endpoint_artifact_ready_exposure_denied",
      "dashboard_artifact_available_badge_exposure_denied",
      "channel_status_delivery_denied",
      "external_status_send_denied",
      "telegram_status_send_denied",
      "release_artifact_write_denied",
      "public_artifact_write_denied",
      "acceptance_from_distribution_status_denied",
      "operator_approval_from_distribution_status_denied",
      "release_publication_authority_from_distribution_status_denied",
      "activation_live_from_distribution_status_denied",
      "install_restart_active_binary_from_distribution_status_denied",
      "memory_provider_kg_from_distribution_status_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_denial_gate",
        status:"allowed_report_only_next_slice",
        exposes_distribution_queue:false,
        exposes_artifact_availability:false,
        writes_release_artifact:false,
        writes_public_artifact:false,
        records_operator_acceptance:false,
        derives_release_publication_authority:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false,
        sends_externally:false
      }
    ],
    packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_artifact_availability_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_request_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_enqueued:false,
    packet_acceptance_receipt_release_publication_result_receipt_distribution_worker_dispatched:false,
    packet_acceptance_receipt_release_publication_result_receipt_artifact_manifest_entry_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_artifact_download_url_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_artifact_checksum_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_artifact_signature_notarization_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_package_index_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_update_feed_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_cdn_mirror_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_release_channel_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_public_bucket_listing_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_artifact_ready_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_dashboard_artifact_available_badge_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_channel_status_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_external_status_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_telegram_status_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_persisted:false,
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
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_filesystem_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_enqueued:false,
      packet_acceptance_receipt_release_publication_result_receipt_distribution_worker_dispatched:false,
      packet_acceptance_receipt_release_publication_result_receipt_artifact_availability_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_artifact_manifest_entry_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_artifact_download_url_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_artifact_checksum_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_artifact_signature_notarization_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_package_index_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_update_feed_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_cdn_mirror_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_release_channel_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_public_bucket_listing_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_artifact_ready_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_dashboard_artifact_available_badge_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_channel_status_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_external_status_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_telegram_status_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_persisted:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_ready == true
  and .source_release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count == 18
  and .source_release_publication_result_receipt_terminal_public_claim_status_exposure_attempt_count == 18
  and .source_release_publication_result_receipt_terminal_public_claim_status_exposure_accepted_count == 0
  and .source_release_publication_result_receipt_terminal_public_claim_status_exposure_recorded_count == 0
  and .source_release_publication_result_receipt_terminal_public_claim_status_exposed_count == 0
  and .source_release_publication_result_receipt_artifact_availability_status_exposed_count == 0
  and .source_release_publication_result_receipt_distribution_queue_status_exposed_count == 0
  and .source_release_publication_result_receipt_channel_status_delivered_count == 0
  and .source_release_publication_result_receipt_external_status_sent_count == 0
  and .source_release_publication_result_receipt_telegram_status_sent_count == 0
  and .source_release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count == 0
  and .source_release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_artifact_status_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_artifact_status_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_request_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_persisted_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_materialized_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_filesystem_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_delivered_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_exposed_count == 0
  and .release_publication_result_receipt_distribution_queue_status_exposed_count == 0
  and .release_publication_result_receipt_distribution_queue_enqueued_count == 0
  and .release_publication_result_receipt_distribution_worker_dispatched_count == 0
  and .release_publication_result_receipt_artifact_availability_status_exposed_count == 0
  and .release_publication_result_receipt_artifact_manifest_entry_exposed_count == 0
  and .release_publication_result_receipt_artifact_download_url_exposed_count == 0
  and .release_publication_result_receipt_artifact_checksum_exposed_count == 0
  and .release_publication_result_receipt_artifact_signature_notarization_exposed_count == 0
  and .release_publication_result_receipt_package_index_status_exposed_count == 0
  and .release_publication_result_receipt_update_feed_status_exposed_count == 0
  and .release_publication_result_receipt_cdn_mirror_status_exposed_count == 0
  and .release_publication_result_receipt_release_channel_status_exposed_count == 0
  and .release_publication_result_receipt_public_bucket_listing_status_exposed_count == 0
  and .release_publication_result_receipt_status_endpoint_artifact_ready_exposed_count == 0
  and .release_publication_result_receipt_dashboard_artifact_available_badge_exposed_count == 0
  and .release_publication_result_receipt_channel_status_delivered_count == 0
  and .release_publication_result_receipt_external_status_sent_count == 0
  and .release_publication_result_receipt_telegram_status_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_acceptance_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_operator_approval_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_activation_command_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_live_execution_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_install_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_service_restarted_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_active_binary_mutated_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_release_artifact_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_artifact_status_public_artifact_written_count == 0
  and (.release_publication_result_receipt_terminal_distribution_artifact_status_surfaces | length) == 18
  and (.release_publication_result_receipt_terminal_distribution_artifact_status_surfaces | all(
    .terminal_distribution_artifact_status_attempted == true
    and .terminal_distribution_artifact_status_allowed == false
    and .terminal_distribution_artifact_status_request_accepted == false
    and .terminal_distribution_artifact_status_accepted == false
    and .terminal_distribution_artifact_status_recorded == false
    and .terminal_distribution_artifact_status_persisted == false
    and .terminal_distribution_artifact_status_materialized == false
    and .terminal_distribution_artifact_status_filesystem_written == false
    and .terminal_distribution_artifact_status_delivered == false
    and .terminal_distribution_artifact_status_exposed == false
    and .distribution_queue_status_exposed == false
    and .distribution_queue_enqueued == false
    and .distribution_worker_dispatched == false
    and .artifact_availability_status_exposed == false
    and .artifact_manifest_entry_exposed == false
    and .artifact_download_url_exposed == false
    and .package_index_status_exposed == false
    and .update_feed_status_exposed == false
    and .cdn_mirror_status_exposed == false
    and .release_channel_status_exposed == false
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
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .terminal_distribution_artifact_status_noop_confirmed == true
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_artifact_status | length) == 35
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_artifact_availability_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_materialized == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_filesystem_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_enqueued == false
  and .packet_acceptance_receipt_release_publication_result_receipt_distribution_worker_dispatched == false
  and .packet_acceptance_receipt_release_publication_result_receipt_artifact_manifest_entry_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_artifact_download_url_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_package_index_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_update_feed_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_cdn_mirror_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_release_channel_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_artifact_ready_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_dashboard_artifact_available_badge_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_channel_status_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_external_status_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_telegram_status_sent == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution queue artifact availability status denial gate passed"
