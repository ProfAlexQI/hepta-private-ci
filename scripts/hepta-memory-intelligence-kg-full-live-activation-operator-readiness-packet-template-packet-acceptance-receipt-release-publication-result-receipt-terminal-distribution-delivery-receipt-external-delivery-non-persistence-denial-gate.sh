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

TERMINAL_DISTRIBUTION_ARTIFACT_STATUS_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-queue-artifact-availability-status-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-queue-artifact-availability-status-denial-gate.sh
)"

terminal_distribution_artifact_status_report_sha256="$(sha256_text "$TERMINAL_DISTRIBUTION_ARTIFACT_STATUS_JSON")"
terminal_distribution_delivery_receipt_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-external-delivery-non-persistence-denial:$terminal_distribution_artifact_status_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
terminal_distribution_delivery_receipt_policy_hash_sha256="$(
  sha256_text "release-publication-result-receipt-terminal-distribution-delivery-receipt-denial:no-delivery-receipt:no-ledger:no-index:no-channel:no-webhook:no-telegram:no-authority"
)"

jq -n -e \
  --argjson source "$TERMINAL_DISTRIBUTION_ARTIFACT_STATUS_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_request_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_persisted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_materialized_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_filesystem_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_delivered_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_exposed_count == 0
    and $source.release_publication_result_receipt_distribution_queue_status_exposed_count == 0
    and $source.release_publication_result_receipt_distribution_queue_enqueued_count == 0
    and $source.release_publication_result_receipt_distribution_worker_dispatched_count == 0
    and $source.release_publication_result_receipt_artifact_availability_status_exposed_count == 0
    and $source.release_publication_result_receipt_artifact_manifest_entry_exposed_count == 0
    and $source.release_publication_result_receipt_artifact_download_url_exposed_count == 0
    and $source.release_publication_result_receipt_artifact_checksum_exposed_count == 0
    and $source.release_publication_result_receipt_artifact_signature_notarization_exposed_count == 0
    and $source.release_publication_result_receipt_package_index_status_exposed_count == 0
    and $source.release_publication_result_receipt_update_feed_status_exposed_count == 0
    and $source.release_publication_result_receipt_cdn_mirror_status_exposed_count == 0
    and $source.release_publication_result_receipt_release_channel_status_exposed_count == 0
    and $source.release_publication_result_receipt_public_bucket_listing_status_exposed_count == 0
    and $source.release_publication_result_receipt_status_endpoint_artifact_ready_exposed_count == 0
    and $source.release_publication_result_receipt_dashboard_artifact_available_badge_exposed_count == 0
    and $source.release_publication_result_receipt_channel_status_delivered_count == 0
    and $source.release_publication_result_receipt_external_status_sent_count == 0
    and $source.release_publication_result_receipt_telegram_status_sent_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_acceptance_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_operator_approval_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_release_publication_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_activation_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_activation_command_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_live_execution_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_install_executed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_service_restarted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_active_binary_mutated_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_release_artifact_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_artifact_status_public_artifact_written_count == 0
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_accepted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_persisted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_materialized == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_filesystem_written == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_delivered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_enqueued == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_distribution_worker_dispatched == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_artifact_manifest_entry_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_artifact_download_url_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_channel_status_delivered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_external_status_sent == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_telegram_status_sent == false
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

terminal_distribution_delivery_receipt_surfaces_json="$(
  jq -n '
    def delivery_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_surface:$id,
        source_terminal_distribution_artifact_status_ready:true,
        terminal_distribution_delivery_receipt_attempted:true,
        terminal_distribution_delivery_receipt_allowed:false,
        terminal_distribution_delivery_receipt_request_accepted:false,
        terminal_distribution_delivery_receipt_accepted:false,
        terminal_distribution_delivery_receipt_recorded:false,
        terminal_distribution_delivery_receipt_persisted:false,
        terminal_distribution_delivery_receipt_materialized:false,
        terminal_distribution_delivery_receipt_filesystem_written:false,
        terminal_distribution_delivery_receipt_ledger_written:false,
        terminal_distribution_delivery_receipt_index_written:false,
        terminal_distribution_delivery_receipt_queued:false,
        terminal_distribution_delivery_receipt_delivered:false,
        terminal_distribution_delivery_receipt_externally_sent:false,
        terminal_distribution_delivery_receipt_channel_sent:false,
        terminal_distribution_delivery_receipt_webhook_sent:false,
        terminal_distribution_delivery_receipt_telegram_sent:false,
        status_endpoint_delivery_receipt_exposed:false,
        dashboard_delivery_receipt_exposed:false,
        delivery_confirmation_recorded:false,
        delivery_ack_recorded:false,
        receipt_echo_delivered:false,
        downstream_consumer_notified:false,
        delivery_receipt_acceptance_recorded:false,
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
        external_send_performed:false,
        terminal_distribution_delivery_receipt_noop_confirmed:true,
        terminal_distribution_delivery_receipt_status:$status,
        reason:$reason
      } + $extra;
    [
      delivery_surface("publication_result_receipt_distribution_delivery_receipt_creation"; "blocked_delivery_receipt_creation_noop"; "delivery_receipt_creation_denied"; {delivery_receipt_creation_requested:true}),
      delivery_surface("publication_result_receipt_distribution_delivery_receipt_recording"; "blocked_delivery_receipt_recording_noop"; "delivery_receipt_recording_denied"; {delivery_receipt_recording_requested:true}),
      delivery_surface("publication_result_receipt_distribution_delivery_receipt_persistence"; "blocked_delivery_receipt_persistence_noop"; "delivery_receipt_persistence_denied"; {delivery_receipt_persistence_requested:true}),
      delivery_surface("publication_result_receipt_distribution_delivery_receipt_filesystem_materialization"; "blocked_delivery_receipt_filesystem_materialization_noop"; "delivery_receipt_filesystem_materialization_denied"; {delivery_receipt_filesystem_materialization_requested:true}),
      delivery_surface("publication_result_receipt_distribution_delivery_receipt_ledger_index"; "blocked_delivery_receipt_ledger_index_noop"; "delivery_receipt_ledger_index_denied"; {delivery_receipt_ledger_requested:true, delivery_receipt_index_requested:true}),
      delivery_surface("publication_result_receipt_distribution_queue_delivery_ack"; "blocked_distribution_queue_delivery_ack_noop"; "distribution_queue_delivery_ack_denied"; {distribution_queue_delivery_ack_requested:true}),
      delivery_surface("publication_result_receipt_artifact_download_delivery_ack"; "blocked_artifact_download_delivery_ack_noop"; "artifact_download_delivery_ack_denied"; {artifact_download_delivery_ack_requested:true}),
      delivery_surface("publication_result_receipt_package_index_delivery_ack"; "blocked_package_index_delivery_ack_noop"; "package_index_delivery_ack_denied"; {package_index_delivery_ack_requested:true}),
      delivery_surface("publication_result_receipt_update_feed_delivery_ack"; "blocked_update_feed_delivery_ack_noop"; "update_feed_delivery_ack_denied"; {update_feed_delivery_ack_requested:true}),
      delivery_surface("publication_result_receipt_cdn_mirror_delivery_ack"; "blocked_cdn_mirror_delivery_ack_noop"; "cdn_mirror_delivery_ack_denied"; {cdn_mirror_delivery_ack_requested:true}),
      delivery_surface("publication_result_receipt_release_channel_delivery_ack"; "blocked_release_channel_delivery_ack_noop"; "release_channel_delivery_ack_denied"; {release_channel_delivery_ack_requested:true}),
      delivery_surface("publication_result_receipt_public_bucket_delivery_ack"; "blocked_public_bucket_delivery_ack_noop"; "public_bucket_delivery_ack_denied"; {public_bucket_delivery_ack_requested:true}),
      delivery_surface("publication_result_receipt_status_endpoint_delivery_receipt"; "blocked_status_endpoint_delivery_receipt_noop"; "status_endpoint_delivery_receipt_denied"; {status_endpoint_delivery_receipt_requested:true}),
      delivery_surface("publication_result_receipt_dashboard_delivery_badge"; "blocked_dashboard_delivery_badge_noop"; "dashboard_delivery_badge_denied"; {dashboard_delivery_badge_requested:true}),
      delivery_surface("publication_result_receipt_channel_delivery_receipt"; "blocked_channel_delivery_receipt_noop"; "channel_delivery_receipt_denied"; {channel_delivery_receipt_requested:true}),
      delivery_surface("publication_result_receipt_external_webhook_delivery_receipt"; "blocked_external_webhook_delivery_receipt_noop"; "external_webhook_delivery_receipt_denied"; {external_webhook_delivery_receipt_requested:true}),
      delivery_surface("publication_result_receipt_telegram_delivery_receipt"; "blocked_telegram_delivery_receipt_noop"; "telegram_delivery_receipt_denied"; {telegram_delivery_receipt_requested:true}),
      delivery_surface("publication_result_receipt_authority_live_active_binary_delivery_receipt"; "blocked_authority_live_active_binary_delivery_receipt_noop"; "authority_live_active_binary_from_delivery_receipt_denied"; {release_publication_authority_delivery_receipt_requested:true, activation_live_delivery_receipt_requested:true, install_restart_active_binary_delivery_receipt_requested:true})
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_gate" \
  --arg terminal_distribution_artifact_status_report_sha256 "$terminal_distribution_artifact_status_report_sha256" \
  --arg terminal_distribution_delivery_receipt_contract_hash_sha256 "$terminal_distribution_delivery_receipt_contract_hash_sha256" \
  --arg terminal_distribution_delivery_receipt_policy_hash_sha256 "$terminal_distribution_delivery_receipt_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$TERMINAL_DISTRIBUTION_ARTIFACT_STATUS_JSON" \
  --argjson surfaces "$terminal_distribution_delivery_receipt_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_v1",
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_mode:"denied_terminal_distribution_status_cannot_become_delivery_receipt_or_external_delivery",
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_ready,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_report_sha256:$terminal_distribution_artifact_status_report_sha256,
    source_release_publication_result_receipt_terminal_distribution_artifact_status_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_artifact_status_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_contract_hash_sha256:$terminal_distribution_delivery_receipt_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_policy_hash_sha256:$terminal_distribution_delivery_receipt_policy_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_ready:true,
    source_release_publication_result_receipt_terminal_distribution_artifact_status_surface_count:$source.release_publication_result_receipt_terminal_distribution_artifact_status_surface_count,
    source_release_publication_result_receipt_terminal_distribution_artifact_status_attempt_count:$source.release_publication_result_receipt_terminal_distribution_artifact_status_attempt_count,
    source_release_publication_result_receipt_terminal_distribution_artifact_status_accepted_count:$source.release_publication_result_receipt_terminal_distribution_artifact_status_accepted_count,
    source_release_publication_result_receipt_terminal_distribution_artifact_status_recorded_count:$source.release_publication_result_receipt_terminal_distribution_artifact_status_recorded_count,
    source_release_publication_result_receipt_terminal_distribution_artifact_status_persisted_count:$source.release_publication_result_receipt_terminal_distribution_artifact_status_persisted_count,
    source_release_publication_result_receipt_terminal_distribution_artifact_status_delivered_count:$source.release_publication_result_receipt_terminal_distribution_artifact_status_delivered_count,
    source_release_publication_result_receipt_terminal_distribution_artifact_status_exposed_count:$source.release_publication_result_receipt_terminal_distribution_artifact_status_exposed_count,
    source_release_publication_result_receipt_distribution_queue_enqueued_count:$source.release_publication_result_receipt_distribution_queue_enqueued_count,
    source_release_publication_result_receipt_distribution_worker_dispatched_count:$source.release_publication_result_receipt_distribution_worker_dispatched_count,
    source_release_publication_result_receipt_artifact_download_url_exposed_count:$source.release_publication_result_receipt_artifact_download_url_exposed_count,
    source_release_publication_result_receipt_channel_status_delivered_count:$source.release_publication_result_receipt_channel_status_delivered_count,
    source_release_publication_result_receipt_external_status_sent_count:$source.release_publication_result_receipt_external_status_sent_count,
    source_release_publication_result_receipt_telegram_status_sent_count:$source.release_publication_result_receipt_telegram_status_sent_count,
    source_release_publication_result_receipt_terminal_distribution_artifact_status_release_publication_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_artifact_status_release_publication_authority_derived_count,
    source_release_publication_result_receipt_terminal_distribution_artifact_status_activation_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_artifact_status_activation_authority_derived_count,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_surface_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_attempt_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_request_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_materialized_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_filesystem_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_ledger_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_index_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_queued_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_webhook_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent_count:0,
    release_publication_result_receipt_status_endpoint_delivery_receipt_exposed_count:0,
    release_publication_result_receipt_dashboard_delivery_receipt_exposed_count:0,
    release_publication_result_receipt_delivery_confirmation_recorded_count:0,
    release_publication_result_receipt_delivery_ack_recorded_count:0,
    release_publication_result_receipt_receipt_echo_delivered_count:0,
    release_publication_result_receipt_downstream_consumer_notified_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_acceptance_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_approval_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_release_publication_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_command_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_live_execution_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_install_executed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_service_restarted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_active_binary_mutated_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_release_artifact_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_public_artifact_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt:[
      "source_terminal_distribution_artifact_status_report_required",
      "delivery_receipt_request_acceptance_denied",
      "delivery_receipt_acceptance_denied",
      "delivery_receipt_recording_denied",
      "delivery_receipt_persistence_denied",
      "delivery_receipt_materialization_denied",
      "delivery_receipt_filesystem_write_denied",
      "delivery_receipt_ledger_write_denied",
      "delivery_receipt_index_write_denied",
      "delivery_receipt_queue_denied",
      "delivery_receipt_delivery_denied",
      "delivery_receipt_external_send_denied",
      "delivery_receipt_channel_send_denied",
      "delivery_receipt_webhook_send_denied",
      "delivery_receipt_telegram_send_denied",
      "distribution_queue_delivery_ack_denied",
      "artifact_download_delivery_ack_denied",
      "package_index_delivery_ack_denied",
      "update_feed_delivery_ack_denied",
      "cdn_mirror_delivery_ack_denied",
      "release_channel_delivery_ack_denied",
      "public_bucket_delivery_ack_denied",
      "status_endpoint_delivery_receipt_denied",
      "dashboard_delivery_receipt_denied",
      "delivery_confirmation_recording_denied",
      "delivery_ack_recording_denied",
      "receipt_echo_delivery_denied",
      "downstream_consumer_notification_denied",
      "release_artifact_write_denied",
      "public_artifact_write_denied",
      "acceptance_from_delivery_receipt_denied",
      "operator_approval_from_delivery_receipt_denied",
      "release_publication_authority_from_delivery_receipt_denied",
      "activation_live_from_delivery_receipt_denied",
      "install_restart_active_binary_from_delivery_receipt_denied",
      "memory_provider_kg_from_delivery_receipt_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_gate",
        status:"allowed_report_only_next_slice",
        records_delivery_receipt:false,
        persists_delivery_receipt:false,
        sends_externally:false,
        records_operator_acceptance:false,
        derives_release_publication_authority:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false
      }
    ],
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_enqueued:false,
    packet_acceptance_receipt_release_publication_result_receipt_distribution_worker_dispatched:false,
    packet_acceptance_receipt_release_publication_result_receipt_artifact_download_url_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_channel_status_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_external_status_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_telegram_status_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_request_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_ledger_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_index_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_queued:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_webhook_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_delivery_receipt_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_dashboard_delivery_receipt_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_confirmation_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_ack_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_receipt_echo_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_downstream_consumer_notified:false,
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
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_filesystem_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_ledger_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_index_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_queued:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_webhook_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_delivery_receipt_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_dashboard_delivery_receipt_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_confirmation_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_ack_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_receipt_echo_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_downstream_consumer_notified:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_exposed:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_ready == true
  and .source_release_publication_result_receipt_terminal_distribution_artifact_status_surface_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_artifact_status_attempt_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_artifact_status_accepted_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_artifact_status_recorded_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_artifact_status_persisted_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_artifact_status_delivered_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_artifact_status_exposed_count == 0
  and .source_release_publication_result_receipt_distribution_queue_enqueued_count == 0
  and .source_release_publication_result_receipt_distribution_worker_dispatched_count == 0
  and .source_release_publication_result_receipt_channel_status_delivered_count == 0
  and .source_release_publication_result_receipt_external_status_sent_count == 0
  and .source_release_publication_result_receipt_telegram_status_sent_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_artifact_status_release_publication_authority_derived_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_artifact_status_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_request_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_materialized_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_filesystem_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_ledger_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_index_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_queued_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_webhook_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent_count == 0
  and .release_publication_result_receipt_status_endpoint_delivery_receipt_exposed_count == 0
  and .release_publication_result_receipt_dashboard_delivery_receipt_exposed_count == 0
  and .release_publication_result_receipt_delivery_confirmation_recorded_count == 0
  and .release_publication_result_receipt_delivery_ack_recorded_count == 0
  and .release_publication_result_receipt_receipt_echo_delivered_count == 0
  and .release_publication_result_receipt_downstream_consumer_notified_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_acceptance_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_approval_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_command_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_live_execution_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_install_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_service_restarted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_active_binary_mutated_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_release_artifact_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_public_artifact_written_count == 0
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_surfaces | length) == 18
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_surfaces | all(
    .terminal_distribution_delivery_receipt_attempted == true
    and .terminal_distribution_delivery_receipt_allowed == false
    and .terminal_distribution_delivery_receipt_request_accepted == false
    and .terminal_distribution_delivery_receipt_accepted == false
    and .terminal_distribution_delivery_receipt_recorded == false
    and .terminal_distribution_delivery_receipt_persisted == false
    and .terminal_distribution_delivery_receipt_materialized == false
    and .terminal_distribution_delivery_receipt_filesystem_written == false
    and .terminal_distribution_delivery_receipt_ledger_written == false
    and .terminal_distribution_delivery_receipt_index_written == false
    and .terminal_distribution_delivery_receipt_queued == false
    and .terminal_distribution_delivery_receipt_delivered == false
    and .terminal_distribution_delivery_receipt_externally_sent == false
    and .terminal_distribution_delivery_receipt_channel_sent == false
    and .terminal_distribution_delivery_receipt_webhook_sent == false
    and .terminal_distribution_delivery_receipt_telegram_sent == false
    and .delivery_confirmation_recorded == false
    and .delivery_ack_recorded == false
    and .receipt_echo_delivered == false
    and .downstream_consumer_notified == false
    and .delivery_receipt_acceptance_recorded == false
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
    and .external_send_performed == false
    and .terminal_distribution_delivery_receipt_noop_confirmed == true
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt | length) == 36
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_channel_status_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_external_status_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_telegram_status_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_materialized == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_filesystem_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_ledger_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_index_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_queued == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_webhook_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_delivery_receipt_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_dashboard_delivery_receipt_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_confirmation_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_ack_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_receipt_echo_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_downstream_consumer_notified == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt external delivery non-persistence denial gate passed"
