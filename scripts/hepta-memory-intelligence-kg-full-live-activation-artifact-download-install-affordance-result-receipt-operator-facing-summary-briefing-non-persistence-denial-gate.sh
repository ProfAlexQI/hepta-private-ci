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

EXPORT_QUERY_OBSERVABILITY_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-export-query-observability-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-export-query-observability-denial-gate.sh
)"

source_export_query_observability_report_sha256="$(sha256_text "$EXPORT_QUERY_OBSERVABILITY_JSON")"
operator_summary_briefing_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-facing-summary-briefing-non-persistence-denial:$source_export_query_observability_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
operator_summary_briefing_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-facing-summary-briefing-non-persistence:no-summary:no-briefing:no-readback:no-status:no-delivery:no-approval:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$EXPORT_QUERY_OBSERVABILITY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denied_count == 18
    and zero_fields($source; [
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
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_export_query_observability_active_binary_mutated_count"
    ])
    and false_fields($source; [
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
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surfaces | all(
      .artifact_download_install_affordance_result_receipt_export_query_observability_attempted == true
      and .export_query_observability_noop_confirmed == true
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate"
      and .status == "allowed_report_only_next_slice"
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
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

operator_summary_briefing_surfaces_json="$(
  jq -n '
    def summary_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surface:$id,
        source_export_query_observability_ready:true,
        artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_attempted:true,
        operator_summary_requested:false,
        operator_briefing_requested:false,
        operator_readback_requested:false,
        status_promotion_requested:false,
        operator_summary_allowed:false,
        operator_summary_request_accepted:false,
        operator_summary_recorded:false,
        operator_summary_persisted:false,
        operator_summary_materialized:false,
        operator_summary_filesystem_written:false,
        operator_summary_delivered:false,
        operator_briefing_allowed:false,
        operator_briefing_request_accepted:false,
        operator_briefing_recorded:false,
        operator_briefing_persisted:false,
        operator_briefing_materialized:false,
        operator_briefing_filesystem_written:false,
        operator_briefing_delivered:false,
        readback_digest_recorded:false,
        readback_digest_persisted:false,
        final_note_recorded:false,
        final_note_persisted:false,
        status_banner_recorded:false,
        status_banner_persisted:false,
        dashboard_annotation_recorded:false,
        notification_preview_recorded:false,
        timeline_entry_recorded:false,
        audit_narrative_recorded:false,
        privacy_review_narrative_recorded:false,
        alert_explanation_recorded:false,
        slo_report_recorded:false,
        query_registration_from_summary_recorded:false,
        export_snapshot_from_summary_recorded:false,
        observability_event_from_summary_recorded:false,
        completion_ack_from_summary_recorded:false,
        result_receipt_from_summary_recorded:false,
        result_receipt_from_summary_persisted:false,
        operator_acceptance_from_summary_recorded:false,
        operator_approval_from_summary_derived:false,
        release_publication_authority_from_summary_derived:false,
        activation_authority_from_summary_derived:false,
        activation_command_from_summary_derived:false,
        activation_from_summary_allowed:false,
        live_execution_from_summary_allowed:false,
        download_link_from_summary_rendered:false,
        install_command_from_summary_rendered:false,
        install_from_summary_executed:false,
        service_restart_from_summary_performed:false,
        launchd_from_summary_mutated:false,
        active_binary_from_summary_mutated:false,
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
        operator_summary_briefing_noop_confirmed:true,
        operator_summary_briefing_status:$status,
        reason:$reason
      } + $extra;
    [
      summary_surface("source_export_query_observability_report_required"; "blocked_source_export_query_observability_required_noop"; "source_export_query_observability_report_required"; {source_export_query_observability_report_required:true}),
      summary_surface("download_button_operator_summary_request_claim"; "blocked_summary_request_noop"; "download_button_operator_summary_request_claim_denied"; {operator_summary_requested:true}),
      summary_surface("direct_download_url_operator_briefing_request_claim"; "blocked_briefing_request_noop"; "direct_download_url_operator_briefing_request_claim_denied"; {operator_briefing_requested:true}),
      summary_surface("checksum_prompt_summary_readback_digest_claim"; "blocked_readback_digest_noop"; "checksum_prompt_summary_readback_digest_claim_denied"; {operator_summary_requested:true, operator_readback_requested:true}),
      summary_surface("package_manager_install_command_briefing_status_banner_claim"; "blocked_status_banner_noop"; "package_manager_install_command_briefing_status_banner_claim_denied"; {operator_briefing_requested:true, status_promotion_requested:true}),
      summary_surface("curl_pipe_shell_summary_exported_text_claim"; "blocked_summary_export_text_noop"; "curl_pipe_shell_summary_exported_text_claim_denied"; {operator_summary_requested:true, summary_export_text_requested:true}),
      summary_surface("installer_launch_prompt_briefing_materialization_claim"; "blocked_briefing_materialization_noop"; "installer_launch_prompt_briefing_materialization_claim_denied"; {operator_briefing_requested:true, briefing_materialization_requested:true}),
      summary_surface("auto_update_offer_summary_persistence_claim"; "blocked_summary_persistence_noop"; "auto_update_offer_summary_persistence_claim_denied"; {operator_summary_requested:true, summary_persistence_requested:true}),
      summary_surface("release_channel_subscription_briefing_persistence_claim"; "blocked_briefing_persistence_noop"; "release_channel_subscription_briefing_persistence_claim_denied"; {operator_briefing_requested:true, briefing_persistence_requested:true}),
      summary_surface("update_feed_hint_notification_preview_claim"; "blocked_notification_preview_noop"; "update_feed_hint_notification_preview_claim_denied"; {operator_summary_requested:true, notification_preview_requested:true}),
      summary_surface("package_registry_badge_timeline_entry_claim"; "blocked_timeline_entry_noop"; "package_registry_badge_timeline_entry_claim_denied"; {operator_briefing_requested:true, timeline_entry_requested:true}),
      summary_surface("cdn_mirror_download_dashboard_annotation_claim"; "blocked_dashboard_annotation_noop"; "cdn_mirror_download_dashboard_annotation_claim_denied"; {operator_summary_requested:true, dashboard_annotation_requested:true}),
      summary_surface("sbom_provenance_notarization_audit_narrative_claim"; "blocked_audit_narrative_noop"; "sbom_provenance_notarization_audit_narrative_claim_denied"; {operator_briefing_requested:true, audit_narrative_requested:true}),
      summary_surface("signature_verification_command_operator_summary_readback_claim"; "blocked_operator_summary_readback_noop"; "signature_verification_command_operator_summary_readback_claim_denied"; {operator_summary_requested:true, operator_readback_requested:true}),
      summary_surface("one_click_install_deep_link_operator_approval_summary_claim"; "blocked_operator_approval_summary_noop"; "one_click_install_deep_link_operator_approval_summary_claim_denied"; {operator_summary_requested:true, operator_approval_summary_requested:true}),
      summary_surface("external_telegram_install_message_operator_briefing_delivery_claim"; "blocked_briefing_delivery_noop"; "external_telegram_install_message_operator_briefing_delivery_claim_denied"; {operator_briefing_requested:true, channel_delivery_requested:true, telegram_delivery_requested:true}),
      summary_surface("release_publication_authority_install_affordance_operator_authority_briefing_claim"; "blocked_authority_briefing_noop"; "release_publication_authority_install_affordance_operator_authority_briefing_claim_denied"; {operator_briefing_requested:true, authority_briefing_requested:true}),
      summary_surface("activation_live_install_restart_active_binary_operator_briefing_claim"; "blocked_live_briefing_noop"; "activation_live_install_restart_active_binary_operator_briefing_claim_denied"; {operator_briefing_requested:true, live_install_restart_active_binary_briefing_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate" \
    --arg source_export_query_observability_report_sha256 "$source_export_query_observability_report_sha256" \
    --arg operator_summary_briefing_contract_hash_sha256 "$operator_summary_briefing_contract_hash_sha256" \
    --arg operator_summary_briefing_policy_hash_sha256 "$operator_summary_briefing_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$EXPORT_QUERY_OBSERVABILITY_JSON" \
    --argjson surfaces "$operator_summary_briefing_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_v1",
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_mode:"denied_artifact_download_install_result_receipt_cannot_create_operator_summary_briefing_status_or_authority",
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_gate:$source.gate,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_ready,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_report_sha256:$source_export_query_observability_report_sha256,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_contract_hash_sha256:$operator_summary_briefing_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_policy_hash_sha256:$operator_summary_briefing_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready:true,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surface_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_attempt_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denied_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_snapshot_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_snapshot_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_metric_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_metric_recorded_count,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surface_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_attempt_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_denied_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surfaces:$surfaces,
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing:[
          "artifact_download_install_affordance_result_receipt_operator_summary_acceptance_denied",
          "artifact_download_install_affordance_result_receipt_operator_summary_recording_denied",
          "artifact_download_install_affordance_result_receipt_operator_summary_persistence_denied",
          "artifact_download_install_affordance_result_receipt_operator_summary_materialization_denied",
          "artifact_download_install_affordance_result_receipt_operator_summary_filesystem_write_denied",
          "artifact_download_install_affordance_result_receipt_operator_summary_delivery_denied",
          "artifact_download_install_affordance_result_receipt_operator_briefing_acceptance_denied",
          "artifact_download_install_affordance_result_receipt_operator_briefing_recording_denied",
          "artifact_download_install_affordance_result_receipt_operator_briefing_persistence_denied",
          "artifact_download_install_affordance_result_receipt_operator_briefing_materialization_denied",
          "artifact_download_install_affordance_result_receipt_operator_briefing_filesystem_write_denied",
          "artifact_download_install_affordance_result_receipt_operator_briefing_delivery_denied",
          "artifact_download_install_affordance_result_receipt_readback_digest_denied",
          "artifact_download_install_affordance_result_receipt_final_note_denied",
          "artifact_download_install_affordance_result_receipt_status_banner_denied",
          "artifact_download_install_affordance_result_receipt_dashboard_annotation_denied",
          "artifact_download_install_affordance_result_receipt_notification_timeline_denied",
          "artifact_download_install_affordance_result_receipt_audit_privacy_narrative_denied",
          "artifact_download_install_affordance_result_receipt_alert_slo_explanation_denied",
          "artifact_download_install_affordance_result_receipt_query_export_observability_from_summary_denied",
          "artifact_download_install_affordance_result_receipt_completion_ack_from_summary_denied",
          "artifact_download_install_affordance_result_receipt_acceptance_from_summary_denied",
          "artifact_download_install_affordance_operator_approval_from_summary_denied",
          "artifact_download_install_affordance_release_publication_authority_from_summary_denied",
          "artifact_download_install_affordance_activation_authority_from_summary_denied",
          "artifact_download_install_affordance_download_install_affordance_from_summary_denied",
          "artifact_download_install_affordance_install_restart_active_binary_from_summary_denied",
          "artifact_download_install_affordance_memory_provider_secret_external_send_from_summary_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_gate",
            status:"allowed_report_only_next_slice",
            accepts_operator_acknowledgement:false,
            persists_acknowledgement:false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_digest_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_note_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_banner_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_dashboard_annotation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_notification_preview_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_timeline_entry_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_narrative_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_privacy_review_narrative_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_alert_explanation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_slo_report_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_ack_from_summary_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_summary_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_external_send_count"
      ])
      + false_object([
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_digest_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_note_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_banner_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_dashboard_annotation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_notification_preview_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_timeline_entry_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_narrative_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_alert_explanation_recorded",
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
          "operator_summary_recorded",
          "operator_summary_persisted",
          "operator_summary_materialized",
          "operator_summary_filesystem_written",
          "operator_summary_delivered",
          "operator_briefing_recorded",
          "operator_briefing_persisted",
          "operator_briefing_materialized",
          "operator_briefing_filesystem_written",
          "operator_briefing_delivered",
          "readback_digest_recorded",
          "final_note_recorded",
          "status_banner_recorded",
          "dashboard_annotation_recorded",
          "notification_preview_recorded",
          "timeline_entry_recorded",
          "audit_narrative_recorded",
          "privacy_review_narrative_recorded",
          "alert_explanation_recorded",
          "slo_report_recorded",
          "query_registration_from_summary_recorded",
          "export_snapshot_from_summary_recorded",
          "observability_event_from_summary_recorded",
          "completion_ack_from_summary_recorded",
          "operator_approval_from_summary_derived",
          "release_publication_authority_from_summary_derived",
          "activation_authority_from_summary_derived",
          "download_link_from_summary_rendered",
          "install_command_from_summary_rendered",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_attempt_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denied_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_attempt_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_denied_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_materialized_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_filesystem_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_summary_delivered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_materialized_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_filesystem_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_briefing_delivered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_digest_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_note_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_banner_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_dashboard_annotation_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_notification_preview_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_timeline_entry_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_narrative_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_privacy_review_narrative_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_alert_explanation_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_slo_report_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_ack_from_summary_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_summary_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_release_publication_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_activation_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_install_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_service_restart_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_active_binary_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_summary_briefing_external_send_count"
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
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surfaces | all(
    .artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_attempted == true
    and .operator_summary_briefing_noop_confirmed == true
    and false_fields(.; [
      "operator_summary_recorded",
      "operator_summary_persisted",
      "operator_summary_materialized",
      "operator_summary_filesystem_written",
      "operator_summary_delivered",
      "operator_briefing_recorded",
      "operator_briefing_persisted",
      "operator_briefing_materialized",
      "operator_briefing_filesystem_written",
      "operator_briefing_delivered",
      "readback_digest_recorded",
      "final_note_recorded",
      "status_banner_recorded",
      "dashboard_annotation_recorded",
      "notification_preview_recorded",
      "timeline_entry_recorded",
      "audit_narrative_recorded",
      "alert_explanation_recorded",
      "slo_report_recorded",
      "query_registration_from_summary_recorded",
      "export_snapshot_from_summary_recorded",
      "observability_event_from_summary_recorded",
      "completion_ack_from_summary_recorded",
      "operator_approval_from_summary_derived",
      "release_publication_authority_from_summary_derived",
      "activation_authority_from_summary_derived",
      "install_from_summary_executed",
      "service_restart_from_summary_performed",
      "active_binary_from_summary_mutated",
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
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surfaces[] | select(.operator_summary_requested == true)] | length) == 8
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surfaces[] | select(.operator_briefing_requested == true)] | length) == 9
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surfaces[] | select(.telegram_delivery_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_surfaces[] | select(.live_install_restart_active_binary_briefing_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .accepts_operator_acknowledgement == false
    and .persists_acknowledgement == false
    and .records_summary == false
    and .records_briefing == false
    and .derives_authority == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator-facing summary/briefing non-persistence denial gate passed"
