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

RETENTION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial-gate.sh
)"

source_retention_report_sha256="$(sha256_text "$RETENTION_JSON")"
export_query_observability_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-export-query-observability-denial:$source_retention_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
export_query_observability_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-export-query-observability-denial:no-query:no-export:no-observability:no-readback:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$RETENTION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denied_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_index_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_ledger_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_update_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_extension_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_scheduler_registered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_timer_started_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_candidate_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_decision_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_archive_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_compaction_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_compaction_artifact_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_retention_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_attestation_retention_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_ack_retention_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_release_publication_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_activation_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_install_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_service_restart_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_active_binary_mutated_count"
    ])
    and false_fields($source; [
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed",
      "artifact_download_install_affordance_result_receipt_recorded",
      "artifact_download_install_affordance_result_receipt_persisted",
      "artifact_download_install_affordance_completion_ack_recorded",
      "download_button_rendered",
      "direct_download_url_exposed",
      "package_manager_install_command_rendered",
      "curl_pipe_shell_snippet_rendered",
      "installer_launch_prompt_rendered",
      "auto_update_offer_rendered",
      "external_install_message_sent",
      "telegram_install_message_sent",
      "operator_acceptance_recorded",
      "operator_approval_recorded",
      "release_publication_authority_derived",
      "activation_authority_derived",
      "activation_allowed",
      "activation_performed",
      "memory_store_write_performed",
      "memory_store_mutated",
      "live_kg_write_performed",
      "provider_invoked",
      "model_invoked",
      "credential_read",
      "secret_file_read",
      "install_executed",
      "launchd_mutated",
      "service_restarted",
      "active_binary_mutated",
      "external_send_performed"
    ])
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces | all(
      .artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempted == true
      and .retention_expiry_garbage_collection_noop_confirmed == true
      and false_fields(.; [
        "artifact_download_install_affordance_result_receipt_retention_policy_recorded",
        "artifact_download_install_affordance_result_receipt_ttl_update_recorded",
        "artifact_download_install_affordance_result_receipt_expiry_recorded",
        "artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed",
        "artifact_download_install_affordance_result_receipt_gc_queue_enqueued",
        "artifact_download_install_affordance_result_receipt_delete_marker_recorded",
        "artifact_download_install_affordance_result_receipt_tombstone_recorded",
        "artifact_download_install_affordance_result_receipt_archive_written",
        "artifact_download_install_affordance_result_receipt_compaction_performed",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "release_publication_authority_from_retention_expiry_garbage_collection_derived",
        "activation_authority_from_retention_expiry_garbage_collection_derived",
        "install_from_retention_expiry_garbage_collection_executed",
        "service_restart_from_retention_expiry_garbage_collection_performed",
        "active_binary_from_retention_expiry_garbage_collection_mutated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "external_send_performed"
      ])
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_retention_policy == false
      and .records_expiry == false
      and .performs_garbage_collection == false
      and .exports_receipt == false
      and .registers_query == false
      and .records_observability == false
      and .renders_download_link == false
      and .emits_install_command == false
      and .installs_or_restarts == false
      and .mutates_active_binary == false
      and .mutates_memory_store == false
      and .writes_kg == false
      and .sends_externally == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

export_query_surfaces_json="$(
  jq -n '
    def view_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surface:$id,
        source_retention_expiry_garbage_collection_present:true,
        source_retention_expiry_garbage_collection_ready:true,
        source_retention_expiry_garbage_collection_noop_confirmed:true,
        artifact_download_install_affordance_result_receipt_export_query_observability_attempted:true,
        artifact_download_install_affordance_result_receipt_query_requested:false,
        artifact_download_install_affordance_result_receipt_export_requested:false,
        artifact_download_install_affordance_result_receipt_observability_requested:false,
        artifact_download_install_affordance_result_receipt_query_registered:false,
        artifact_download_install_affordance_result_receipt_query_executed:false,
        artifact_download_install_affordance_result_receipt_query_result_recorded:false,
        artifact_download_install_affordance_result_receipt_query_result_persisted:false,
        artifact_download_install_affordance_result_receipt_search_index_recorded:false,
        artifact_download_install_affordance_result_receipt_search_index_persisted:false,
        artifact_download_install_affordance_result_receipt_export_accepted:false,
        artifact_download_install_affordance_result_receipt_export_snapshot_recorded:false,
        artifact_download_install_affordance_result_receipt_export_snapshot_persisted:false,
        artifact_download_install_affordance_result_receipt_export_file_written:false,
        artifact_download_install_affordance_result_receipt_export_stream_opened:false,
        artifact_download_install_affordance_result_receipt_observability_metric_recorded:false,
        artifact_download_install_affordance_result_receipt_observability_log_recorded:false,
        artifact_download_install_affordance_result_receipt_observability_trace_recorded:false,
        artifact_download_install_affordance_result_receipt_observability_event_recorded:false,
        artifact_download_install_affordance_result_receipt_dashboard_panel_recorded:false,
        artifact_download_install_affordance_result_receipt_alert_registered:false,
        artifact_download_install_affordance_result_receipt_slo_recorded:false,
        artifact_download_install_affordance_result_receipt_operator_summary_recorded:false,
        artifact_download_install_affordance_result_receipt_readback_surface_recorded:false,
        artifact_download_install_affordance_result_receipt_audit_view_recorded:false,
        artifact_download_install_affordance_result_receipt_ledger_observability_recorded:false,
        artifact_download_install_affordance_result_receipt_index_observability_recorded:false,
        artifact_download_install_affordance_result_receipt_delivery_observability_recorded:false,
        artifact_download_install_affordance_result_receipt_retention_policy_recorded:false,
        artifact_download_install_affordance_result_receipt_expiry_recorded:false,
        artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed:false,
        artifact_download_install_affordance_result_receipt_audit_trail_recorded:false,
        artifact_download_install_affordance_result_receipt_immutable_evidence_recorded:false,
        artifact_download_install_affordance_result_receipt_hash_chain_recorded:false,
        artifact_download_install_affordance_result_receipt_completion_ack_recorded:false,
        artifact_download_install_affordance_result_receipt_recorded:false,
        artifact_download_install_affordance_result_receipt_persisted:false,
        artifact_download_install_affordance_result_receipt_accepted:false,
        artifact_download_install_affordance_result_receipt_materialized:false,
        artifact_download_install_affordance_result_receipt_filesystem_written:false,
        artifact_download_install_affordance_result_receipt_ledger_written:false,
        artifact_download_install_affordance_result_receipt_indexed:false,
        artifact_download_install_affordance_result_receipt_enqueued:false,
        artifact_download_install_affordance_result_receipt_delivered:false,
        artifact_download_install_affordance_result_receipt_exported:false,
        artifact_download_install_affordance_result_receipt_query_registered:false,
        artifact_download_install_affordance_result_receipt_observability_recorded:false,
        artifact_download_install_affordance_completion_ack_recorded:false,
        artifact_download_install_affordance_completion_ack_accepted:false,
        download_button_rendered:false,
        direct_download_url_exposed:false,
        package_manager_install_command_rendered:false,
        curl_pipe_shell_snippet_rendered:false,
        installer_launch_prompt_rendered:false,
        auto_update_offer_rendered:false,
        external_install_message_sent:false,
        telegram_install_message_sent:false,
        operator_approval_from_export_query_observability_accepted:false,
        release_publication_authority_from_export_query_observability_derived:false,
        activation_authority_from_export_query_observability_derived:false,
        activation_command_from_export_query_observability_derived:false,
        activation_from_export_query_observability_allowed:false,
        live_execution_from_export_query_observability_allowed:false,
        install_from_export_query_observability_executed:false,
        service_restart_from_export_query_observability_performed:false,
        launchd_from_export_query_observability_mutated:false,
        active_binary_from_export_query_observability_mutated:false,
        activation_activated:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        telegram_send_performed:false,
        channel_send_performed:false,
        external_send_performed:false,
        public_release_claimed:false,
        public_ga_claimed:false,
        release_artifact_written:false,
        public_artifact_written:false,
        export_query_observability_noop_confirmed:true,
        artifact_download_install_affordance_result_receipt_export_query_observability_status:$status,
        reason:$reason
      } + $extra;
    [
      view_surface("source_retention_expiry_garbage_collection_report_required"; "blocked_source_retention_report_required_noop"; "source_retention_expiry_garbage_collection_report_required"; {source_retention_expiry_garbage_collection_report_required:true}),
      view_surface("download_button_result_receipt_query_registration_claim"; "blocked_query_registration_noop"; "download_button_result_receipt_query_registration_claim_denied"; {artifact_download_install_affordance_result_receipt_query_requested:true}),
      view_surface("direct_download_url_result_receipt_query_execution_claim"; "blocked_query_execution_noop"; "direct_download_url_result_receipt_query_execution_claim_denied"; {artifact_download_install_affordance_result_receipt_query_requested:true}),
      view_surface("checksum_prompt_result_receipt_query_result_claim"; "blocked_query_result_noop"; "checksum_prompt_result_receipt_query_result_claim_denied"; {artifact_download_install_affordance_result_receipt_query_requested:true}),
      view_surface("package_manager_install_command_result_receipt_search_index_claim"; "blocked_search_index_noop"; "package_manager_install_command_result_receipt_search_index_claim_denied"; {search_index_requested:true}),
      view_surface("curl_pipe_shell_result_receipt_export_request_claim"; "blocked_export_request_noop"; "curl_pipe_shell_result_receipt_export_request_claim_denied"; {artifact_download_install_affordance_result_receipt_export_requested:true}),
      view_surface("installer_launch_prompt_result_receipt_export_snapshot_claim"; "blocked_export_snapshot_noop"; "installer_launch_prompt_result_receipt_export_snapshot_claim_denied"; {artifact_download_install_affordance_result_receipt_export_requested:true, export_snapshot_requested:true}),
      view_surface("auto_update_offer_result_receipt_export_file_claim"; "blocked_export_file_noop"; "auto_update_offer_result_receipt_export_file_claim_denied"; {artifact_download_install_affordance_result_receipt_export_requested:true, export_file_requested:true}),
      view_surface("release_channel_subscription_result_receipt_export_stream_claim"; "blocked_export_stream_noop"; "release_channel_subscription_result_receipt_export_stream_claim_denied"; {artifact_download_install_affordance_result_receipt_export_requested:true, export_stream_requested:true}),
      view_surface("update_feed_hint_result_receipt_observability_metric_log_claim"; "blocked_metric_log_noop"; "update_feed_hint_result_receipt_observability_metric_log_claim_denied"; {artifact_download_install_affordance_result_receipt_observability_requested:true, metric_observability_requested:true, log_observability_requested:true}),
      view_surface("package_registry_badge_result_receipt_observability_trace_event_claim"; "blocked_trace_event_noop"; "package_registry_badge_result_receipt_observability_trace_event_claim_denied"; {artifact_download_install_affordance_result_receipt_observability_requested:true, trace_observability_requested:true, event_observability_requested:true}),
      view_surface("cdn_mirror_download_result_receipt_dashboard_panel_claim"; "blocked_dashboard_panel_noop"; "cdn_mirror_download_result_receipt_dashboard_panel_claim_denied"; {dashboard_panel_requested:true}),
      view_surface("sbom_provenance_notarization_result_receipt_alert_slo_claim"; "blocked_alert_slo_noop"; "sbom_provenance_notarization_result_receipt_alert_slo_claim_denied"; {alert_slo_requested:true}),
      view_surface("signature_verification_command_result_receipt_operator_summary_readback_claim"; "blocked_operator_summary_readback_noop"; "signature_verification_command_result_receipt_operator_summary_readback_claim_denied"; {operator_summary_readback_requested:true}),
      view_surface("one_click_install_deep_link_result_receipt_audit_view_claim"; "blocked_audit_view_noop"; "one_click_install_deep_link_result_receipt_audit_view_claim_denied"; {audit_view_requested:true}),
      view_surface("external_telegram_install_message_result_receipt_external_observability_claim"; "blocked_external_observability_noop"; "external_telegram_install_message_result_receipt_external_observability_claim_denied"; {external_observability_requested:true, telegram_observability_requested:true}),
      view_surface("release_publication_authority_install_affordance_result_receipt_authority_view_claim"; "blocked_authority_view_noop"; "release_publication_authority_install_affordance_result_receipt_authority_view_claim_denied"; {authority_view_requested:true}),
      view_surface("activation_live_install_restart_active_binary_result_receipt_live_view_claim"; "blocked_live_view_noop"; "activation_live_install_restart_active_binary_result_receipt_live_view_claim_denied"; {live_view_requested:true, install_view_requested:true, service_restart_view_requested:true, active_binary_view_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_gate" \
    --arg source_retention_report_sha256 "$source_retention_report_sha256" \
    --arg export_query_observability_contract_hash_sha256 "$export_query_observability_contract_hash_sha256" \
    --arg export_query_observability_policy_hash_sha256 "$export_query_observability_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$RETENTION_JSON" \
    --argjson surfaces "$export_query_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_v1",
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_mode:"denied_artifact_download_install_result_receipt_cannot_create_export_query_observability_view_or_authority",
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_gate:$source.gate,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_ready,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_report_sha256:$source_retention_report_sha256,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_contract_hash_sha256:$export_query_observability_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_policy_hash_sha256:$export_query_observability_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_ready:true,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempt_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denied_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_release_publication_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_release_publication_authority_derived_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_activation_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_activation_authority_derived_count,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surface_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_attempt_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denied_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surfaces:$surfaces,
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability:[
          "artifact_download_install_affordance_result_receipt_query_registration_denied",
          "artifact_download_install_affordance_result_receipt_query_execution_denied",
          "artifact_download_install_affordance_result_receipt_query_result_recording_denied",
          "artifact_download_install_affordance_result_receipt_query_result_persistence_denied",
          "artifact_download_install_affordance_result_receipt_search_index_recording_denied",
          "artifact_download_install_affordance_result_receipt_search_index_persistence_denied",
          "artifact_download_install_affordance_result_receipt_export_request_denied",
          "artifact_download_install_affordance_result_receipt_export_acceptance_denied",
          "artifact_download_install_affordance_result_receipt_export_snapshot_recording_denied",
          "artifact_download_install_affordance_result_receipt_export_snapshot_persistence_denied",
          "artifact_download_install_affordance_result_receipt_export_file_write_denied",
          "artifact_download_install_affordance_result_receipt_export_stream_open_denied",
          "artifact_download_install_affordance_result_receipt_observability_metric_denied",
          "artifact_download_install_affordance_result_receipt_observability_log_denied",
          "artifact_download_install_affordance_result_receipt_observability_trace_denied",
          "artifact_download_install_affordance_result_receipt_observability_event_denied",
          "artifact_download_install_affordance_result_receipt_dashboard_panel_denied",
          "artifact_download_install_affordance_result_receipt_alert_slo_denied",
          "artifact_download_install_affordance_result_receipt_operator_summary_denied",
          "artifact_download_install_affordance_result_receipt_readback_surface_denied",
          "artifact_download_install_affordance_result_receipt_audit_view_denied",
          "artifact_download_install_affordance_result_receipt_ledger_index_delivery_observability_denied",
          "artifact_download_install_affordance_result_receipt_completion_ack_from_view_denied",
          "artifact_download_install_affordance_result_receipt_acceptance_from_view_denied",
          "artifact_download_install_affordance_release_publication_authority_from_view_denied",
          "artifact_download_install_affordance_activation_authority_from_view_denied",
          "artifact_download_install_affordance_download_install_affordance_from_view_denied",
          "artifact_download_install_affordance_install_restart_active_binary_from_view_denied",
          "artifact_download_install_affordance_memory_provider_secret_external_send_from_view_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate",
            status:"allowed_report_only_next_slice",
            exports_receipt:false,
            registers_query:false,
            records_observability:false,
            records_summary:false,
            records_briefing:false,
            renders_download_link:false,
            emits_install_command:false,
            installs_or_restarts:false,
            mutates_active_binary:false,
            mutates_memory_store:false,
            writes_kg:false,
            sends_externally:false
          }
        ]
      }
      + zero_object([
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_result_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_result_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_search_index_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_search_index_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_snapshot_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_snapshot_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_file_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_stream_opened_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_metric_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_log_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_trace_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_event_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_dashboard_panel_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_alert_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_slo_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_surface_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_view_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_index_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_memory_store_write_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_live_kg_write_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_provider_invoked_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_model_invoked_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_credential_read_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_secret_read_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_external_send_count"
      ])
      + false_object([
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_result_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_search_index_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_snapshot_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_RECEIPT_artifact_download_install_affordance_result_receipt_export_file_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_stream_opened",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_metric_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_log_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_trace_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_event_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_dashboard_panel_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_surface_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_view_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_index_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_chain_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_exported",
        "artifact_download_install_affordance_result_receipt_query_registered",
        "artifact_download_install_affordance_result_receipt_observability_recorded",
        "artifact_download_install_affordance_completion_ack_recorded",
        "download_button_rendered",
        "direct_download_url_exposed",
        "package_manager_install_command_rendered",
        "curl_pipe_shell_snippet_rendered",
        "installer_launch_prompt_rendered",
        "auto_update_offer_rendered",
        "external_install_message_sent",
        "telegram_install_message_sent",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed"
      ])
      + {
        side_effects:false_object([
          "artifact_download_install_affordance_result_receipt_query_registered",
          "artifact_download_install_affordance_result_receipt_query_executed",
          "artifact_download_install_affordance_result_receipt_query_result_recorded",
          "artifact_download_install_affordance_result_receipt_query_result_persisted",
          "artifact_download_install_affordance_result_receipt_search_index_recorded",
          "artifact_download_install_affordance_result_receipt_search_index_persisted",
          "artifact_download_install_affordance_result_receipt_export_accepted",
          "artifact_download_install_affordance_result_receipt_export_snapshot_recorded",
          "artifact_download_install_affordance_result_receipt_export_snapshot_persisted",
          "artifact_download_install_affordance_result_receipt_export_file_written",
          "artifact_download_install_affordance_result_receipt_export_stream_opened",
          "artifact_download_install_affordance_result_receipt_observability_metric_recorded",
          "artifact_download_install_affordance_result_receipt_observability_log_recorded",
          "artifact_download_install_affordance_result_receipt_observability_trace_recorded",
          "artifact_download_install_affordance_result_receipt_observability_event_recorded",
          "artifact_download_install_affordance_result_receipt_dashboard_panel_recorded",
          "artifact_download_install_affordance_result_receipt_alert_registered",
          "artifact_download_install_affordance_result_receipt_slo_recorded",
          "artifact_download_install_affordance_result_receipt_operator_summary_recorded",
          "artifact_download_install_affordance_result_receipt_readback_surface_recorded",
          "artifact_download_install_affordance_result_receipt_audit_view_recorded",
          "artifact_download_install_affordance_result_receipt_ledger_observability_recorded",
          "artifact_download_install_affordance_result_receipt_index_observability_recorded",
          "artifact_download_install_affordance_result_receipt_delivery_observability_recorded",
          "artifact_download_install_affordance_result_receipt_retention_policy_recorded",
          "artifact_download_install_affordance_result_receipt_expiry_recorded",
          "artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed",
          "artifact_download_install_affordance_result_receipt_audit_trail_recorded",
          "artifact_download_install_affordance_result_receipt_immutable_evidence_recorded",
          "artifact_download_install_affordance_result_receipt_hash_chain_recorded",
          "artifact_download_install_affordance_result_receipt_recorded",
          "artifact_download_install_affordance_result_receipt_persisted",
          "artifact_download_install_affordance_result_receipt_exported",
          "artifact_download_install_affordance_result_receipt_query_registered",
          "artifact_download_install_affordance_result_receipt_observability_recorded",
          "artifact_download_install_affordance_completion_ack_recorded",
          "download_button_rendered",
          "direct_download_url_exposed",
          "package_manager_install_command_rendered",
          "curl_pipe_shell_snippet_rendered",
          "installer_launch_prompt_rendered",
          "auto_update_offer_rendered",
          "external_install_message_sent",
          "telegram_install_message_sent",
          "operator_acceptance_recorded",
          "operator_approval_recorded",
          "release_publication_authority_derived",
          "activation_authority_derived",
          "activation_command_derived",
          "activation_allowed",
          "activation_performed",
          "memory_store_write_performed",
          "memory_store_mutated",
          "live_kg_write_performed",
          "provider_invoked",
          "model_invoked",
          "credential_read",
          "secret_file_read",
          "install_executed",
          "launchd_mutated",
          "service_restarted",
          "active_binary_mutated",
          "release_artifact_written",
          "public_artifact_written",
          "public_release_claimed",
          "public_ga_claimed",
          "telegram_send_performed",
          "channel_send_performed",
          "external_send_performed",
          "filesystem_written"
        ])
      }
    '
)"

jq -e '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);

  . as $report
  | $report.runtime == "hepta"
  and $report.status == "ready"
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempt_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denied_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_attempt_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denied_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_result_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_result_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_search_index_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_snapshot_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_file_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_stream_opened_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_metric_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_log_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_trace_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_event_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_dashboard_panel_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_alert_registered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_slo_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_surface_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_view_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_observability_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_index_observability_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_observability_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_acceptance_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_release_publication_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_activation_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_install_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_service_restart_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_active_binary_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_memory_store_write_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_live_kg_write_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_provider_invoked_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_model_invoked_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_credential_read_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_secret_read_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_external_send_count"
  ])
  and false_fields($report; [
    "artifact_download_install_affordance_result_receipt_recorded",
    "artifact_download_install_affordance_result_receipt_persisted",
    "artifact_download_install_affordance_result_receipt_exported",
    "artifact_download_install_affordance_result_receipt_query_registered",
    "artifact_download_install_affordance_result_receipt_observability_recorded",
    "artifact_download_install_affordance_completion_ack_recorded",
    "download_button_rendered",
    "direct_download_url_exposed",
    "package_manager_install_command_rendered",
    "curl_pipe_shell_snippet_rendered",
    "installer_launch_prompt_rendered",
    "auto_update_offer_rendered",
    "external_install_message_sent",
    "telegram_install_message_sent",
    "operator_acceptance_recorded",
    "operator_approval_recorded",
    "release_publication_authority_derived",
    "activation_authority_derived",
    "activation_allowed",
    "activation_performed",
    "memory_store_write_performed",
    "memory_store_mutated",
    "live_kg_write_performed",
    "provider_invoked",
    "model_invoked",
    "credential_read",
    "secret_file_read",
    "install_executed",
    "launchd_mutated",
    "service_restarted",
    "active_binary_mutated",
    "external_send_performed"
  ])
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surfaces | all(
    .artifact_download_install_affordance_result_receipt_export_query_observability_attempted == true
    and .export_query_observability_noop_confirmed == true
    and false_fields(.; [
      "artifact_download_install_affordance_result_receipt_query_registered",
      "artifact_download_install_affordance_result_receipt_query_executed",
      "artifact_download_install_affordance_result_receipt_query_result_recorded",
      "artifact_download_install_affordance_result_receipt_search_index_recorded",
      "artifact_download_install_affordance_result_receipt_export_accepted",
      "artifact_download_install_affordance_result_receipt_export_snapshot_recorded",
      "artifact_download_install_affordance_result_receipt_export_file_written",
      "artifact_download_install_affordance_result_receipt_export_stream_opened",
      "artifact_download_install_affordance_result_receipt_observability_metric_recorded",
      "artifact_download_install_affordance_result_receipt_observability_log_recorded",
      "artifact_download_install_affordance_result_receipt_observability_trace_recorded",
      "artifact_download_install_affordance_result_receipt_observability_event_recorded",
      "artifact_download_install_affordance_result_receipt_dashboard_panel_recorded",
      "artifact_download_install_affordance_result_receipt_alert_registered",
      "artifact_download_install_affordance_result_receipt_slo_recorded",
      "artifact_download_install_affordance_result_receipt_operator_summary_recorded",
      "artifact_download_install_affordance_result_receipt_readback_surface_recorded",
      "artifact_download_install_affordance_result_receipt_audit_view_recorded",
      "artifact_download_install_affordance_result_receipt_ledger_observability_recorded",
      "artifact_download_install_affordance_result_receipt_index_observability_recorded",
      "artifact_download_install_affordance_result_receipt_delivery_observability_recorded",
      "artifact_download_install_affordance_result_receipt_recorded",
      "artifact_download_install_affordance_result_receipt_persisted",
      "artifact_download_install_affordance_completion_ack_recorded",
      "download_button_rendered",
      "direct_download_url_exposed",
      "package_manager_install_command_rendered",
      "curl_pipe_shell_snippet_rendered",
      "installer_launch_prompt_rendered",
      "auto_update_offer_rendered",
      "external_install_message_sent",
      "telegram_install_message_sent",
      "release_publication_authority_from_export_query_observability_derived",
      "activation_authority_from_export_query_observability_derived",
      "install_from_export_query_observability_executed",
      "service_restart_from_export_query_observability_performed",
      "active_binary_from_export_query_observability_mutated",
      "memory_store_write_performed",
      "memory_store_mutated",
      "live_kg_write_performed",
      "provider_invoked",
      "model_invoked",
      "credential_read",
      "secret_file_read",
      "external_send_performed"
    ])
  ))
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surfaces[] | select(.export_file_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surfaces[] | select(.trace_observability_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surfaces[] | select(.audit_view_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surfaces[] | select(.install_view_requested == true)] | length) == 1
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability | length) == 29
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .exports_receipt == false
    and .registers_query == false
    and .records_observability == false
    and .records_summary == false
    and .records_briefing == false
    and .renders_download_link == false
    and .emits_install_command == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .sends_externally == false
  ))
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt export/query/observability denial gate passed"
