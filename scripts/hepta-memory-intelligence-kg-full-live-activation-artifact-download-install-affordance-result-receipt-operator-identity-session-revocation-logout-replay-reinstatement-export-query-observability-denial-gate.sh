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

RETENTION_EXPIRY_GARBAGE_COLLECTION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-retention-expiry-garbage-collection-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-retention-expiry-garbage-collection-denial-gate.sh
)"

source_retention_expiry_garbage_collection_report_sha256="$(
  sha256_text "$RETENTION_EXPIRY_GARBAGE_COLLECTION_JSON"
)"
operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-export-query-observability-denial:$source_retention_expiry_garbage_collection_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-export-query-observability:no-query:no-export:no-observability:no-readback:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$RETENTION_EXPIRY_GARBAGE_COLLECTION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denied_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_timestamp_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_scheduler_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_state_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_queue_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_scan_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_candidate_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_decision_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_archive_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_compaction_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_acceptance_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_release_publication_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_activation_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_install_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_service_restart_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_active_binary_mutated_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_external_send_count"
    ])
    and false_fields($source; [
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_recorded",
      "artifact_download_install_affordance_result_receipt_recorded",
      "artifact_download_install_affordance_result_receipt_persisted",
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
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces | all(
      .artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_attempted == true
      and .operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_noop_confirmed == true
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_retention == false
      and .records_expiry == false
      and .records_garbage_collection == false
      and .registers_query == false
      and .records_observability == false
      and .derives_authority == false
      and .installs_or_restarts == false
      and .mutates_active_binary == false
      and .mutates_memory_store == false
      and .writes_kg == false
      and .sends_externally == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surfaces_json="$(
  jq -n '
    def view_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surface:$id,
        source_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_ready:true,
        artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_attempted:true,
        operator_identity_session_revocation_logout_replay_reinstatement_query_requested:false,
        operator_identity_session_revocation_logout_replay_reinstatement_export_requested:false,
        operator_identity_session_revocation_logout_replay_reinstatement_observability_requested:false,
        search_index_requested:false,
        export_snapshot_requested:false,
        export_file_requested:false,
        export_stream_requested:false,
        metric_observability_requested:false,
        log_observability_requested:false,
        trace_observability_requested:false,
        event_observability_requested:false,
        dashboard_panel_requested:false,
        alert_slo_requested:false,
        operator_summary_readback_requested:false,
        audit_view_requested:false,
        external_observability_requested:false,
        telegram_observability_requested:false,
        authority_view_requested:false,
        live_view_requested:false,
        install_view_requested:false,
        service_restart_view_requested:false,
        active_binary_view_requested:false,
        query_registered:false,
        query_executed:false,
        query_result_recorded:false,
        query_result_persisted:false,
        search_index_recorded:false,
        search_index_persisted:false,
        export_accepted:false,
        export_snapshot_recorded:false,
        export_snapshot_persisted:false,
        export_file_written:false,
        export_stream_opened:false,
        observability_metric_recorded:false,
        observability_log_recorded:false,
        observability_trace_recorded:false,
        observability_event_recorded:false,
        dashboard_panel_recorded:false,
        alert_registered:false,
        slo_recorded:false,
        operator_summary_recorded:false,
        readback_surface_recorded:false,
        audit_view_recorded:false,
        ledger_observability_recorded:false,
        index_observability_recorded:false,
        delivery_observability_recorded:false,
        retention_policy_recorded:false,
        expiry_recorded:false,
        garbage_collection_scan_performed:false,
        audit_trail_recorded:false,
        immutable_evidence_recorded:false,
        hash_chain_recorded:false,
        result_receipt_recorded:false,
        result_receipt_persisted:false,
        result_receipt_exported:false,
        result_receipt_query_registered:false,
        result_receipt_observability_recorded:false,
        completion_ack_recorded:false,
        operator_approval_from_export_query_observability_accepted:false,
        release_publication_authority_from_export_query_observability_derived:false,
        activation_authority_from_export_query_observability_derived:false,
        activation_command_from_export_query_observability_derived:false,
        install_from_export_query_observability_executed:false,
        service_restart_from_export_query_observability_performed:false,
        launchd_from_export_query_observability_mutated:false,
        active_binary_from_export_query_observability_mutated:false,
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
        operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_noop_confirmed:true,
        operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_status:$status,
        reason:$reason
      } + $extra;
    [
      view_surface("source_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_report_required"; "blocked_source_retention_report_required_noop"; "source_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_report_required"; {source_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_report_required:true}),
      view_surface("download_button_revocation_replay_query_registration_claim"; "blocked_revocation_replay_query_registration_noop"; "download_button_revocation_replay_query_registration_claim_denied"; {operator_identity_session_revocation_logout_replay_reinstatement_query_requested:true}),
      view_surface("direct_download_url_logout_replay_query_execution_claim"; "blocked_logout_replay_query_execution_noop"; "direct_download_url_logout_replay_query_execution_claim_denied"; {operator_identity_session_revocation_logout_replay_reinstatement_query_requested:true}),
      view_surface("checksum_identity_reinstatement_query_result_claim"; "blocked_identity_reinstatement_query_result_noop"; "checksum_identity_reinstatement_query_result_claim_denied"; {operator_identity_session_revocation_logout_replay_reinstatement_query_requested:true}),
      view_surface("package_manager_session_reinstatement_search_index_claim"; "blocked_session_reinstatement_search_index_noop"; "package_manager_session_reinstatement_search_index_claim_denied"; {search_index_requested:true}),
      view_surface("curl_pipe_shell_revocation_replay_export_request_claim"; "blocked_revocation_replay_export_request_noop"; "curl_pipe_shell_revocation_replay_export_request_claim_denied"; {operator_identity_session_revocation_logout_replay_reinstatement_export_requested:true}),
      view_surface("installer_device_session_reinstatement_export_snapshot_claim"; "blocked_device_session_reinstatement_export_snapshot_noop"; "installer_device_session_reinstatement_export_snapshot_claim_denied"; {operator_identity_session_revocation_logout_replay_reinstatement_export_requested:true, export_snapshot_requested:true}),
      view_surface("auto_update_session_logout_replay_export_file_claim"; "blocked_session_logout_replay_export_file_noop"; "auto_update_session_logout_replay_export_file_claim_denied"; {operator_identity_session_revocation_logout_replay_reinstatement_export_requested:true, export_file_requested:true}),
      view_surface("release_channel_identity_revocation_replay_export_stream_claim"; "blocked_identity_revocation_replay_export_stream_noop"; "release_channel_identity_revocation_replay_export_stream_claim_denied"; {operator_identity_session_revocation_logout_replay_reinstatement_export_requested:true, export_stream_requested:true}),
      view_surface("update_feed_session_reinstatement_observability_metric_log_claim"; "blocked_session_reinstatement_metric_log_noop"; "update_feed_session_reinstatement_observability_metric_log_claim_denied"; {operator_identity_session_revocation_logout_replay_reinstatement_observability_requested:true, metric_observability_requested:true, log_observability_requested:true}),
      view_surface("package_registry_identity_badge_observability_trace_event_claim"; "blocked_identity_badge_trace_event_noop"; "package_registry_identity_badge_observability_trace_event_claim_denied"; {operator_identity_session_revocation_logout_replay_reinstatement_observability_requested:true, trace_observability_requested:true, event_observability_requested:true}),
      view_surface("cdn_session_readback_logout_replay_dashboard_panel_claim"; "blocked_logout_replay_dashboard_panel_noop"; "cdn_session_readback_logout_replay_dashboard_panel_claim_denied"; {dashboard_panel_requested:true}),
      view_surface("sbom_identity_dashboard_reinstatement_alert_slo_claim"; "blocked_identity_reinstatement_alert_slo_noop"; "sbom_identity_dashboard_reinstatement_alert_slo_claim_denied"; {alert_slo_requested:true}),
      view_surface("signature_channel_session_operator_summary_readback_claim"; "blocked_session_operator_summary_readback_noop"; "signature_channel_session_operator_summary_readback_claim_denied"; {operator_summary_readback_requested:true}),
      view_surface("one_click_identity_approval_reinstatement_audit_view_claim"; "blocked_identity_approval_reinstatement_audit_view_noop"; "one_click_identity_approval_reinstatement_audit_view_claim_denied"; {audit_view_requested:true}),
      view_surface("external_telegram_identity_session_reinstatement_external_observability_claim"; "blocked_external_telegram_observability_noop"; "external_telegram_identity_session_reinstatement_external_observability_claim_denied"; {operator_identity_session_revocation_logout_replay_reinstatement_observability_requested:true, external_observability_requested:true, telegram_observability_requested:true}),
      view_surface("release_publication_authority_replay_reinstatement_authority_view_claim"; "blocked_release_publication_authority_view_noop"; "release_publication_authority_replay_reinstatement_authority_view_claim_denied"; {authority_view_requested:true}),
      view_surface("activation_live_install_restart_active_binary_result_receipt_live_view_claim"; "blocked_live_install_restart_active_binary_view_noop"; "activation_live_install_restart_active_binary_result_receipt_live_view_claim_denied"; {live_view_requested:true, install_view_requested:true, service_restart_view_requested:true, active_binary_view_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_gate" \
    --arg source_retention_expiry_garbage_collection_report_sha256 "$source_retention_expiry_garbage_collection_report_sha256" \
    --arg operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_contract_hash_sha256 "$operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_contract_hash_sha256" \
    --arg operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_policy_hash_sha256 "$operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$RETENTION_EXPIRY_GARBAGE_COLLECTION_JSON" \
    --argjson surfaces "$operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_v1",
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_mode:"denied_retention_expiry_garbage_collection_cannot_create_export_query_observability_view_or_authority",
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_gate:$source.gate,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_ready,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_report_sha256:$source_retention_expiry_garbage_collection_report_sha256,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_contract_hash_sha256:$operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_policy_hash_sha256:$operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_ready:true,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surface_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denied_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_timestamp_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_timestamp_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_scan_performed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_scan_performed_count,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surface_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_attempt_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denied_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surfaces:$surfaces,
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability:[
          "source_retention_expiry_garbage_collection_report_required",
          "operator_identity_session_query_registration_denied",
          "operator_identity_session_query_execution_denied",
          "operator_identity_session_query_result_denied",
          "operator_identity_session_search_index_denied",
          "operator_identity_session_export_request_denied",
          "operator_identity_session_export_snapshot_denied",
          "operator_identity_session_export_file_denied",
          "operator_identity_session_export_stream_denied",
          "operator_identity_session_observability_metric_log_denied",
          "operator_identity_session_observability_trace_event_denied",
          "operator_identity_session_dashboard_panel_denied",
          "operator_identity_session_alert_slo_denied",
          "operator_identity_session_operator_summary_readback_denied",
          "operator_identity_session_audit_view_denied",
          "operator_identity_session_external_telegram_observability_denied",
          "operator_identity_session_release_publication_authority_view_denied",
          "operator_identity_session_live_install_restart_active_binary_view_denied",
          "operator_identity_session_acceptance_or_approval_from_view_denied",
          "operator_identity_session_release_activation_authority_from_view_denied",
          "operator_identity_session_download_install_from_view_denied",
          "operator_identity_session_memory_kg_provider_secret_external_send_from_view_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_non_persistence_denial_gate",
            status:"allowed_report_only_next_slice",
            exports_receipt:false,
            registers_query:false,
            records_observability:false,
            records_summary:false,
            records_briefing:false,
            derives_authority:false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_result_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_result_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_search_index_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_search_index_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_snapshot_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_snapshot_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_file_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_stream_opened_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_metric_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_log_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_trace_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_event_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_dashboard_panel_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_alert_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_slo_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_summary_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_readback_surface_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_view_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ledger_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_index_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_delivery_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_external_send_count"
      ])
      + false_object([
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_result_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_snapshot_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_file_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_metric_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_trace_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_exported",
        "artifact_download_install_affordance_result_receipt_query_registered",
        "artifact_download_install_affordance_result_receipt_observability_recorded",
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
          "query_registered",
          "query_executed",
          "query_result_recorded",
          "query_result_persisted",
          "search_index_recorded",
          "search_index_persisted",
          "export_accepted",
          "export_snapshot_recorded",
          "export_snapshot_persisted",
          "export_file_written",
          "export_stream_opened",
          "observability_metric_recorded",
          "observability_log_recorded",
          "observability_trace_recorded",
          "observability_event_recorded",
          "dashboard_panel_recorded",
          "alert_registered",
          "slo_recorded",
          "operator_summary_recorded",
          "readback_surface_recorded",
          "audit_view_recorded",
          "ledger_observability_recorded",
          "index_observability_recorded",
          "delivery_observability_recorded",
          "result_receipt_recorded",
          "result_receipt_persisted",
          "result_receipt_exported",
          "result_receipt_query_registered",
          "result_receipt_observability_recorded",
          "completion_ack_recorded",
          "operator_approval_from_export_query_observability_accepted",
          "release_publication_authority_from_export_query_observability_derived",
          "activation_authority_from_export_query_observability_derived",
          "download_link_from_export_query_observability_rendered",
          "install_command_from_export_query_observability_rendered",
          "install_executed",
          "launchd_mutated",
          "service_restarted",
          "active_binary_mutated",
          "memory_store_write_performed",
          "memory_store_mutated",
          "live_kg_write_performed",
          "provider_invoked",
          "model_invoked",
          "credential_read",
          "secret_file_read",
          "telegram_send_performed",
          "channel_send_performed",
          "external_send_performed",
          "release_artifact_written",
          "public_artifact_written",
          "public_release_claimed",
          "public_ga_claimed",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denied_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_attempt_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denied_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_registered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_result_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_result_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_search_index_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_search_index_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_snapshot_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_file_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_metric_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_trace_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_dashboard_panel_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_summary_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_acceptance_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_release_publication_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_activation_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_install_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_service_restart_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_active_binary_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_external_send_count"
  ])
  and false_fields($report; [
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_registered",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_executed",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_accepted",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_snapshot_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_metric_recorded",
    "artifact_download_install_affordance_result_receipt_recorded",
    "artifact_download_install_affordance_result_receipt_persisted",
    "artifact_download_install_affordance_result_receipt_exported",
    "artifact_download_install_affordance_result_receipt_query_registered",
    "artifact_download_install_affordance_result_receipt_observability_recorded",
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
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surfaces | all(
    .artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_attempted == true
    and .operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_noop_confirmed == true
    and false_fields(.; [
      "query_registered",
      "query_executed",
      "query_result_recorded",
      "query_result_persisted",
      "search_index_recorded",
      "search_index_persisted",
      "export_accepted",
      "export_snapshot_recorded",
      "export_file_written",
      "export_stream_opened",
      "observability_metric_recorded",
      "observability_trace_recorded",
      "dashboard_panel_recorded",
      "operator_summary_recorded",
      "result_receipt_recorded",
      "result_receipt_persisted",
      "operator_approval_from_export_query_observability_accepted",
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
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surfaces[] | select(.operator_identity_session_revocation_logout_replay_reinstatement_query_requested == true)] | length) == 3
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surfaces[] | select(.operator_identity_session_revocation_logout_replay_reinstatement_export_requested == true)] | length) == 4
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surfaces[] | select(.operator_identity_session_revocation_logout_replay_reinstatement_observability_requested == true)] | length) == 3
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surfaces[] | select(.external_observability_requested == true and .telegram_observability_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_surfaces[] | select(.live_view_requested == true and .install_view_requested == true and .active_binary_view_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_facing_summary_briefing_non_persistence_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .exports_receipt == false
    and .registers_query == false
    and .records_observability == false
    and .records_summary == false
    and .records_briefing == false
    and .derives_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .sends_externally == false
  ))
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
