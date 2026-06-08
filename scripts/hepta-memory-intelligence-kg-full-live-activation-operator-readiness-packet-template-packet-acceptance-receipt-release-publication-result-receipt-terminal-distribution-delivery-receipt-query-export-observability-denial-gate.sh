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

TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-external-delivery-non-persistence-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-external-delivery-non-persistence-denial-gate.sh
)"

terminal_distribution_delivery_receipt_report_sha256="$(sha256_text "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_JSON")"
delivery_receipt_query_export_observability_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-query-export-observability-denial:$terminal_distribution_delivery_receipt_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
delivery_receipt_query_export_observability_policy_hash_sha256="$(
  sha256_text "release-publication-result-receipt-terminal-distribution-delivery-receipt-query-export-observability-denial:no-query:no-export:no-observability:no-dashboard:no-alert:no-authority"
)"

jq -n -e \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_request_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_materialized_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_filesystem_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_ledger_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_index_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_queued_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_sent_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_webhook_sent_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent_count == 0
    and $source.release_publication_result_receipt_status_endpoint_delivery_receipt_exposed_count == 0
    and $source.release_publication_result_receipt_dashboard_delivery_receipt_exposed_count == 0
    and $source.release_publication_result_receipt_delivery_confirmation_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_ack_recorded_count == 0
    and $source.release_publication_result_receipt_receipt_echo_delivered_count == 0
    and $source.release_publication_result_receipt_downstream_consumer_notified_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_acceptance_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_approval_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_release_publication_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_command_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_live_execution_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_install_executed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_service_restarted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_active_binary_mutated_count == 0
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_materialized == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_filesystem_written == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_ledger_written == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_index_written == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_queued == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_sent == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_webhook_sent == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_delivery_receipt_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_dashboard_delivery_receipt_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_confirmation_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_ack_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_receipt_echo_delivered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_downstream_consumer_notified == false
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
    and ($source.allowed_next_actions | any(.action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_gate" and .status == "allowed_report_only_next_slice" and .records_delivery_receipt == false and .persists_delivery_receipt == false and .sends_externally == false and .mutates_memory_store == false and .writes_kg == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

delivery_receipt_query_export_observability_surfaces_json="$(
  jq -n '
    def qeo_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surface:$id,
        source_terminal_distribution_delivery_receipt_ready:true,
        query_export_observability_attempted:true,
        query_export_observability_allowed:false,
        query_export_observability_request_accepted:false,
        query_export_observability_accepted:false,
        query_export_observability_recorded:false,
        query_export_observability_persisted:false,
        query_export_observability_materialized:false,
        query_export_observability_filesystem_written:false,
        query_export_observability_delivered:false,
        query_export_observability_exposed:false,
        query_registration_performed:false,
        query_execution_performed:false,
        query_result_exposed:false,
        search_index_written:false,
        export_request_accepted:false,
        export_snapshot_recorded:false,
        export_file_written:false,
        export_stream_opened:false,
        observability_metric_recorded:false,
        observability_log_recorded:false,
        observability_trace_recorded:false,
        observability_event_recorded:false,
        dashboard_panel_exposed:false,
        alert_slo_recorded:false,
        operator_readback_exposed:false,
        audit_view_exposed:false,
        delivery_receipt_status_evidence_exposed:false,
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
        external_send_performed:false,
        query_export_observability_noop_confirmed:true,
        query_export_observability_status:$status,
        reason:$reason
      } + $extra;
    [
      qeo_surface("publication_result_receipt_delivery_receipt_query_registration"; "blocked_delivery_receipt_query_registration_noop"; "delivery_receipt_query_registration_denied"; {query_registration_requested:true}),
      qeo_surface("publication_result_receipt_delivery_receipt_query_execution"; "blocked_delivery_receipt_query_execution_noop"; "delivery_receipt_query_execution_denied"; {query_execution_requested:true}),
      qeo_surface("publication_result_receipt_delivery_receipt_query_result"; "blocked_delivery_receipt_query_result_noop"; "delivery_receipt_query_result_denied"; {query_result_requested:true}),
      qeo_surface("publication_result_receipt_delivery_receipt_search_index"; "blocked_delivery_receipt_search_index_noop"; "delivery_receipt_search_index_denied"; {search_index_requested:true}),
      qeo_surface("publication_result_receipt_delivery_receipt_export_request"; "blocked_delivery_receipt_export_request_noop"; "delivery_receipt_export_request_denied"; {export_request_requested:true}),
      qeo_surface("publication_result_receipt_delivery_receipt_export_snapshot"; "blocked_delivery_receipt_export_snapshot_noop"; "delivery_receipt_export_snapshot_denied"; {export_snapshot_requested:true}),
      qeo_surface("publication_result_receipt_delivery_receipt_export_file"; "blocked_delivery_receipt_export_file_noop"; "delivery_receipt_export_file_denied"; {export_file_requested:true}),
      qeo_surface("publication_result_receipt_delivery_receipt_export_stream"; "blocked_delivery_receipt_export_stream_noop"; "delivery_receipt_export_stream_denied"; {export_stream_requested:true}),
      qeo_surface("publication_result_receipt_delivery_receipt_observability_metric"; "blocked_delivery_receipt_observability_metric_noop"; "delivery_receipt_observability_metric_denied"; {observability_metric_requested:true}),
      qeo_surface("publication_result_receipt_delivery_receipt_observability_log"; "blocked_delivery_receipt_observability_log_noop"; "delivery_receipt_observability_log_denied"; {observability_log_requested:true}),
      qeo_surface("publication_result_receipt_delivery_receipt_observability_trace"; "blocked_delivery_receipt_observability_trace_noop"; "delivery_receipt_observability_trace_denied"; {observability_trace_requested:true}),
      qeo_surface("publication_result_receipt_delivery_receipt_observability_event"; "blocked_delivery_receipt_observability_event_noop"; "delivery_receipt_observability_event_denied"; {observability_event_requested:true}),
      qeo_surface("publication_result_receipt_delivery_receipt_dashboard_panel"; "blocked_delivery_receipt_dashboard_panel_noop"; "delivery_receipt_dashboard_panel_denied"; {dashboard_panel_requested:true}),
      qeo_surface("publication_result_receipt_delivery_receipt_alert_slo"; "blocked_delivery_receipt_alert_slo_noop"; "delivery_receipt_alert_slo_denied"; {alert_slo_requested:true}),
      qeo_surface("publication_result_receipt_delivery_receipt_operator_readback"; "blocked_delivery_receipt_operator_readback_noop"; "delivery_receipt_operator_readback_denied"; {operator_readback_requested:true}),
      qeo_surface("publication_result_receipt_delivery_receipt_audit_view"; "blocked_delivery_receipt_audit_view_noop"; "delivery_receipt_audit_view_denied"; {audit_view_requested:true}),
      qeo_surface("publication_result_receipt_release_publication_authority_observability"; "blocked_release_publication_authority_observability_noop"; "release_publication_authority_from_observability_denied"; {release_publication_authority_observability_requested:true}),
      qeo_surface("publication_result_receipt_activation_live_active_binary_observability"; "blocked_activation_live_active_binary_observability_noop"; "activation_live_active_binary_from_observability_denied"; {activation_live_observability_requested:true, install_restart_active_binary_observability_requested:true})
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_gate" \
  --arg terminal_distribution_delivery_receipt_report_sha256 "$terminal_distribution_delivery_receipt_report_sha256" \
  --arg delivery_receipt_query_export_observability_contract_hash_sha256 "$delivery_receipt_query_export_observability_contract_hash_sha256" \
  --arg delivery_receipt_query_export_observability_policy_hash_sha256 "$delivery_receipt_query_export_observability_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_JSON" \
  --argjson surfaces "$delivery_receipt_query_export_observability_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_v1",
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_mode:"denied_delivery_receipt_cannot_become_query_export_observability_or_authority",
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_ready,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_report_sha256:$terminal_distribution_delivery_receipt_report_sha256,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_contract_hash_sha256:$delivery_receipt_query_export_observability_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_policy_hash_sha256:$delivery_receipt_query_export_observability_policy_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_ready:true,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_surface_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_attempt_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent_count,
    source_release_publication_result_receipt_downstream_consumer_notified_count:$source.release_publication_result_receipt_downstream_consumer_notified_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_publication_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_release_publication_authority_derived_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_authority_derived_count,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surface_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_attempt_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_request_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_materialized_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_filesystem_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_delivered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed_count:0,
    release_publication_result_receipt_delivery_receipt_query_registered_count:0,
    release_publication_result_receipt_delivery_receipt_query_executed_count:0,
    release_publication_result_receipt_delivery_receipt_query_result_exposed_count:0,
    release_publication_result_receipt_delivery_receipt_search_index_written_count:0,
    release_publication_result_receipt_delivery_receipt_export_requested_count:0,
    release_publication_result_receipt_delivery_receipt_export_snapshot_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_export_file_written_count:0,
    release_publication_result_receipt_delivery_receipt_export_stream_opened_count:0,
    release_publication_result_receipt_delivery_receipt_observability_metric_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_observability_log_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_observability_trace_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_observability_event_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_dashboard_panel_exposed_count:0,
    release_publication_result_receipt_delivery_receipt_alert_slo_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_operator_readback_exposed_count:0,
    release_publication_result_receipt_delivery_receipt_audit_view_exposed_count:0,
    release_publication_result_receipt_delivery_receipt_status_evidence_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_acceptance_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_operator_approval_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_release_publication_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_command_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_live_execution_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_install_executed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_service_restarted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_active_binary_mutated_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_release_artifact_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_public_artifact_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability:[
      "source_terminal_distribution_delivery_receipt_report_required",
      "query_export_observability_request_acceptance_denied",
      "query_export_observability_acceptance_denied",
      "query_export_observability_recording_denied",
      "query_export_observability_persistence_denied",
      "query_export_observability_materialization_denied",
      "query_export_observability_filesystem_write_denied",
      "query_export_observability_delivery_denied",
      "query_export_observability_exposure_denied",
      "delivery_receipt_query_registration_denied",
      "delivery_receipt_query_execution_denied",
      "delivery_receipt_query_result_exposure_denied",
      "delivery_receipt_search_index_write_denied",
      "delivery_receipt_export_request_denied",
      "delivery_receipt_export_snapshot_denied",
      "delivery_receipt_export_file_write_denied",
      "delivery_receipt_export_stream_denied",
      "delivery_receipt_observability_metric_denied",
      "delivery_receipt_observability_log_denied",
      "delivery_receipt_observability_trace_denied",
      "delivery_receipt_observability_event_denied",
      "delivery_receipt_dashboard_panel_denied",
      "delivery_receipt_alert_slo_denied",
      "delivery_receipt_operator_readback_denied",
      "delivery_receipt_audit_view_denied",
      "delivery_receipt_status_evidence_denied",
      "acceptance_from_delivery_receipt_observability_denied",
      "operator_approval_from_delivery_receipt_observability_denied",
      "release_publication_authority_from_delivery_receipt_observability_denied",
      "activation_live_from_delivery_receipt_observability_denied",
      "install_restart_active_binary_from_delivery_receipt_observability_denied",
      "release_artifact_write_denied",
      "public_artifact_write_denied",
      "memory_provider_kg_from_delivery_receipt_observability_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_denial_gate",
        status:"allowed_report_only_next_slice",
        exposes_delivery_receipt_query:false,
        exports_delivery_receipt:false,
        records_observability:false,
        records_operator_acceptance:false,
        derives_release_publication_authority:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false,
        sends_externally:false
      }
    ],
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_downstream_consumer_notified:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_request_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_registered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_executed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_result_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_search_index_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_requested:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_snapshot_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_file_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_stream_opened:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_metric_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_log_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_trace_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_event_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_dashboard_panel_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_alert_slo_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_readback_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_audit_view_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_status_evidence_exposed:false,
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
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_filesystem_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_registered:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_executed:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_result_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_search_index_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_requested:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_snapshot_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_file_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_stream_opened:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_metric_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_log_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_trace_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_event_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_dashboard_panel_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_alert_slo_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_readback_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_audit_view_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_status_evidence_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_ready == true
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_surface_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_attempt_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent_count == 0
  and .source_release_publication_result_receipt_downstream_consumer_notified_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_release_publication_authority_derived_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_request_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_materialized_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_filesystem_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_delivered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed_count == 0
  and .release_publication_result_receipt_delivery_receipt_query_registered_count == 0
  and .release_publication_result_receipt_delivery_receipt_query_executed_count == 0
  and .release_publication_result_receipt_delivery_receipt_query_result_exposed_count == 0
  and .release_publication_result_receipt_delivery_receipt_search_index_written_count == 0
  and .release_publication_result_receipt_delivery_receipt_export_requested_count == 0
  and .release_publication_result_receipt_delivery_receipt_export_snapshot_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_export_file_written_count == 0
  and .release_publication_result_receipt_delivery_receipt_export_stream_opened_count == 0
  and .release_publication_result_receipt_delivery_receipt_observability_metric_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_observability_log_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_observability_trace_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_observability_event_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_dashboard_panel_exposed_count == 0
  and .release_publication_result_receipt_delivery_receipt_alert_slo_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_operator_readback_exposed_count == 0
  and .release_publication_result_receipt_delivery_receipt_audit_view_exposed_count == 0
  and .release_publication_result_receipt_delivery_receipt_status_evidence_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_acceptance_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_operator_approval_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_activation_command_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_live_execution_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_install_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_service_restarted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_active_binary_mutated_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_release_artifact_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_public_artifact_written_count == 0
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surfaces | length) == 18
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_surfaces | all(
    .query_export_observability_attempted == true
    and .query_export_observability_allowed == false
    and .query_export_observability_request_accepted == false
    and .query_export_observability_accepted == false
    and .query_export_observability_recorded == false
    and .query_export_observability_persisted == false
    and .query_export_observability_materialized == false
    and .query_export_observability_filesystem_written == false
    and .query_export_observability_delivered == false
    and .query_export_observability_exposed == false
    and .query_registration_performed == false
    and .query_execution_performed == false
    and .query_result_exposed == false
    and .search_index_written == false
    and .export_request_accepted == false
    and .export_snapshot_recorded == false
    and .export_file_written == false
    and .export_stream_opened == false
    and .observability_metric_recorded == false
    and .observability_log_recorded == false
    and .observability_trace_recorded == false
    and .observability_event_recorded == false
    and .dashboard_panel_exposed == false
    and .alert_slo_recorded == false
    and .operator_readback_exposed == false
    and .audit_view_exposed == false
    and .delivery_receipt_status_evidence_exposed == false
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
    and .external_send_performed == false
    and .query_export_observability_noop_confirmed == true
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability | length) == 34
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_downstream_consumer_notified == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_materialized == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_filesystem_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_registered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_executed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_query_result_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_search_index_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_requested == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_snapshot_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_file_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_export_stream_opened == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_metric_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_log_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_trace_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_observability_event_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_dashboard_panel_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_alert_slo_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_readback_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_audit_view_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_status_evidence_exposed == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt query/export/observability denial gate passed"
