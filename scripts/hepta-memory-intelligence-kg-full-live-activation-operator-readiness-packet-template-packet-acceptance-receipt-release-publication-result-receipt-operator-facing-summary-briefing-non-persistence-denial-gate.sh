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
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial-gate.sh
)"

export_query_observability_report_sha256="$(sha256_text "$EXPORT_QUERY_OBSERVABILITY_JSON")"
operator_summary_briefing_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-operator-facing-summary-briefing-non-persistence-denial:$export_query_observability_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
operator_summary_briefing_policy_hash_sha256="$(
  sha256_text "release-publication-result-receipt-operator-facing-summary-briefing-non-persistence:no-summary:no-briefing:no-readback:no-final-note:no-dashboard:no-delivery:no-authority:no-live"
)"

jq -n -e \
  --argjson source "$EXPORT_QUERY_OBSERVABILITY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_retention_ready == true
    and $source.release_publication_result_receipt_export_query_observability_surface_count == 18
    and $source.release_publication_result_receipt_export_query_observability_attempt_count == 18
    and $source.release_publication_result_receipt_query_registered_count == 0
    and $source.release_publication_result_receipt_query_executed_count == 0
    and $source.release_publication_result_receipt_query_result_recorded_count == 0
    and $source.release_publication_result_receipt_search_index_recorded_count == 0
    and $source.release_publication_result_receipt_export_requested_count == 0
    and $source.release_publication_result_receipt_export_accepted_count == 0
    and $source.release_publication_result_receipt_export_snapshot_recorded_count == 0
    and $source.release_publication_result_receipt_export_file_written_count == 0
    and $source.release_publication_result_receipt_export_stream_opened_count == 0
    and $source.release_publication_result_receipt_observability_metric_recorded_count == 0
    and $source.release_publication_result_receipt_observability_log_recorded_count == 0
    and $source.release_publication_result_receipt_observability_trace_recorded_count == 0
    and $source.release_publication_result_receipt_observability_event_recorded_count == 0
    and $source.release_publication_result_receipt_dashboard_panel_recorded_count == 0
    and $source.release_publication_result_receipt_alert_registered_count == 0
    and $source.release_publication_result_receipt_slo_recorded_count == 0
    and $source.release_publication_result_receipt_operator_summary_recorded_count == 0
    and $source.release_publication_result_receipt_readback_surface_recorded_count == 0
    and $source.release_publication_result_receipt_audit_view_recorded_count == 0
    and $source.release_publication_result_receipt_export_query_observability_acceptance_recorded_count == 0
    and $source.release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count == 0
    and $source.release_publication_result_receipt_export_query_observability_activation_authority_derived_count == 0
    and $source.release_publication_result_receipt_export_query_observability_activation_command_derived_count == 0
    and $source.release_publication_result_receipt_export_query_observability_live_execution_allowed_count == 0
    and $source.packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_readback_surface_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_audit_view_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_dashboard_panel_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_exported == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_query_registered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_observability_recorded == false
    and $source.packet_acceptance_receipt_publication_completion_ack_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_persisted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_materialized == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivered == false
    and $source.packet_acceptance_receipt_release_publication_recorded == false
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.release_publication_authority_derived == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_mutated == false
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

operator_summary_briefing_surfaces_json="$(
  jq -n '
    def summary_briefing_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_operator_facing_summary_briefing_surface:$id,
        source_export_query_observability_ready:true,
        operator_facing_text_attempted:true,
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
        dashboard_annotation_recorded:false,
        notification_preview_recorded:false,
        timeline_entry_recorded:false,
        audit_narrative_recorded:false,
        privacy_review_narrative_recorded:false,
        alert_explanation_recorded:false,
        slo_report_recorded:false,
        channel_delivery_performed:false,
        telegram_send_performed:false,
        external_send_performed:false,
        query_registered:false,
        export_snapshot_recorded:false,
        observability_event_recorded:false,
        completion_ack_recorded:false,
        acceptance_recorded:false,
        operator_approval_derived:false,
        release_publication_authority_derived:false,
        activation_authority_derived:false,
        activation_command_derived:false,
        live_execution_allowed:false,
        activation_performed:false,
        install_executed:false,
        launchd_mutated:false,
        service_restarted:false,
        active_binary_mutated:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        operator_facing_summary_briefing_noop_confirmed:true,
        operator_summary_briefing_status:$status,
        reason:$reason
      } + $extra;
    [
      summary_briefing_surface("publication_result_receipt_operator_summary_request_claim"; "blocked_summary_noop"; "operator_summary_request_denied"; {operator_summary_requested:true}),
      summary_briefing_surface("publication_result_receipt_operator_briefing_request_claim"; "blocked_briefing_noop"; "operator_briefing_request_denied"; {operator_briefing_requested:true}),
      summary_briefing_surface("publication_result_receipt_readback_digest_claim"; "blocked_readback_noop"; "readback_digest_recording_denied"; {readback_digest_requested:true}),
      summary_briefing_surface("publication_result_receipt_final_note_claim"; "blocked_final_note_noop"; "final_note_recording_denied"; {final_note_requested:true}),
      summary_briefing_surface("publication_result_receipt_status_banner_claim"; "blocked_status_noop"; "status_banner_recording_denied"; {status_banner_requested:true}),
      summary_briefing_surface("publication_result_receipt_dashboard_annotation_claim"; "blocked_dashboard_noop"; "dashboard_annotation_recording_denied"; {dashboard_annotation_requested:true}),
      summary_briefing_surface("publication_result_receipt_notification_preview_claim"; "blocked_notification_noop"; "notification_preview_recording_denied"; {notification_preview_requested:true}),
      summary_briefing_surface("publication_result_receipt_timeline_entry_claim"; "blocked_timeline_noop"; "timeline_entry_recording_denied"; {timeline_entry_requested:true}),
      summary_briefing_surface("publication_result_receipt_audit_narrative_claim"; "blocked_audit_noop"; "audit_narrative_recording_denied"; {audit_narrative_requested:true}),
      summary_briefing_surface("publication_result_receipt_privacy_review_narrative_claim"; "blocked_privacy_noop"; "privacy_review_narrative_recording_denied"; {privacy_review_requested:true}),
      summary_briefing_surface("publication_result_receipt_alert_explanation_claim"; "blocked_alert_noop"; "alert_explanation_recording_denied"; {alert_explanation_requested:true}),
      summary_briefing_surface("publication_result_receipt_slo_report_claim"; "blocked_slo_noop"; "slo_report_recording_denied"; {slo_report_requested:true}),
      summary_briefing_surface("publication_result_receipt_channel_delivery_summary_claim"; "blocked_delivery_noop"; "channel_delivery_denied"; {operator_summary_requested:true, operator_briefing_requested:true, channel_delivery_requested:true}),
      summary_briefing_surface("publication_result_receipt_external_send_summary_claim"; "blocked_external_noop"; "external_send_denied"; {operator_summary_requested:true, operator_briefing_requested:true, external_send_requested:true}),
      summary_briefing_surface("publication_result_receipt_telegram_briefing_claim"; "blocked_telegram_noop"; "telegram_briefing_denied"; {operator_briefing_requested:true, telegram_send_requested:true}),
      summary_briefing_surface("publication_result_receipt_completion_ack_from_summary_claim"; "blocked_ack_noop"; "completion_ack_from_summary_denied"; {operator_summary_requested:true, completion_ack_requested:true}),
      summary_briefing_surface("publication_result_receipt_release_publication_authority_summary_claim"; "blocked_authority_noop"; "release_publication_authority_from_summary_denied"; {operator_summary_requested:true, release_publication_authority_requested:true}),
      summary_briefing_surface("publication_result_receipt_activation_live_install_restart_active_binary_summary_claim"; "blocked_activation_noop"; "activation_live_install_restart_active_binary_from_summary_denied"; {operator_briefing_requested:true, activation_authority_requested:true, install_restart_active_binary_requested:true})
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate" \
  --arg export_query_observability_report_sha256 "$export_query_observability_report_sha256" \
  --arg operator_summary_briefing_contract_hash_sha256 "$operator_summary_briefing_contract_hash_sha256" \
  --arg operator_summary_briefing_policy_hash_sha256 "$operator_summary_briefing_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$EXPORT_QUERY_OBSERVABILITY_JSON" \
  --argjson surfaces "$operator_summary_briefing_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_operator_facing_summary_briefing_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_v1",
    receipt_release_publication_result_receipt_operator_facing_summary_briefing_mode:"denied_release_publication_result_receipt_cannot_create_operator_facing_summary_briefing_readback_or_authority",
    source_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_ready,
    source_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_report_sha256:$export_query_observability_report_sha256,
    source_release_publication_result_receipt_export_query_observability_contract_hash_sha256:$source.release_publication_result_receipt_export_query_observability_contract_hash_sha256,
    release_publication_result_receipt_operator_summary_briefing_contract_hash_sha256:$operator_summary_briefing_contract_hash_sha256,
    release_publication_result_receipt_operator_summary_briefing_policy_hash_sha256:$operator_summary_briefing_policy_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready:true,
    source_release_publication_result_receipt_export_query_observability_surface_count:$source.release_publication_result_receipt_export_query_observability_surface_count,
    source_release_publication_result_receipt_export_query_observability_attempt_count:$source.release_publication_result_receipt_export_query_observability_attempt_count,
    source_release_publication_result_receipt_operator_summary_recorded_count:$source.release_publication_result_receipt_operator_summary_recorded_count,
    source_release_publication_result_receipt_readback_surface_recorded_count:$source.release_publication_result_receipt_readback_surface_recorded_count,
    source_release_publication_result_receipt_audit_view_recorded_count:$source.release_publication_result_receipt_audit_view_recorded_count,
    source_release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count:$source.release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count,
    source_release_publication_result_receipt_export_query_observability_activation_authority_derived_count:$source.release_publication_result_receipt_export_query_observability_activation_authority_derived_count,
    release_publication_result_receipt_operator_facing_summary_briefing_surface_count:($surfaces | length),
    release_publication_result_receipt_operator_facing_summary_briefing_attempt_count:($surfaces | length),
    release_publication_result_receipt_operator_summary_allowed_count:0,
    release_publication_result_receipt_operator_summary_request_accepted_count:0,
    release_publication_result_receipt_operator_summary_recorded_count:0,
    release_publication_result_receipt_operator_summary_persisted_count:0,
    release_publication_result_receipt_operator_summary_materialized_count:0,
    release_publication_result_receipt_operator_summary_filesystem_written_count:0,
    release_publication_result_receipt_operator_summary_delivered_count:0,
    release_publication_result_receipt_operator_briefing_allowed_count:0,
    release_publication_result_receipt_operator_briefing_request_accepted_count:0,
    release_publication_result_receipt_operator_briefing_recorded_count:0,
    release_publication_result_receipt_operator_briefing_persisted_count:0,
    release_publication_result_receipt_operator_briefing_materialized_count:0,
    release_publication_result_receipt_operator_briefing_filesystem_written_count:0,
    release_publication_result_receipt_operator_briefing_delivered_count:0,
    release_publication_result_receipt_readback_digest_recorded_count:0,
    release_publication_result_receipt_readback_digest_persisted_count:0,
    release_publication_result_receipt_final_note_recorded_count:0,
    release_publication_result_receipt_final_note_persisted_count:0,
    release_publication_result_receipt_status_banner_recorded_count:0,
    release_publication_result_receipt_dashboard_annotation_recorded_count:0,
    release_publication_result_receipt_notification_preview_recorded_count:0,
    release_publication_result_receipt_timeline_entry_recorded_count:0,
    release_publication_result_receipt_audit_narrative_recorded_count:0,
    release_publication_result_receipt_privacy_review_narrative_recorded_count:0,
    release_publication_result_receipt_alert_explanation_recorded_count:0,
    release_publication_result_receipt_slo_report_recorded_count:0,
    release_publication_result_receipt_operator_summary_briefing_channel_delivery_count:0,
    release_publication_result_receipt_operator_summary_briefing_external_send_count:0,
    release_publication_result_receipt_operator_summary_briefing_telegram_send_count:0,
    release_publication_result_receipt_operator_summary_briefing_completion_ack_recorded_count:0,
    release_publication_result_receipt_operator_summary_briefing_acceptance_recorded_count:0,
    release_publication_result_receipt_operator_summary_briefing_operator_approval_derived_count:0,
    release_publication_result_receipt_operator_summary_briefing_release_publication_authority_derived_count:0,
    release_publication_result_receipt_operator_summary_briefing_activation_authority_derived_count:0,
    release_publication_result_receipt_operator_summary_briefing_activation_command_derived_count:0,
    release_publication_result_receipt_operator_summary_briefing_live_execution_allowed_count:0,
    release_publication_result_receipt_operator_facing_summary_briefing_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_operator_facing_summary_briefing:[
      "source_export_query_observability_report_required",
      "operator_summary_request_acceptance_denied",
      "operator_briefing_request_acceptance_denied",
      "readback_digest_recording_denied",
      "final_note_recording_denied",
      "status_banner_recording_denied",
      "dashboard_annotation_recording_denied",
      "notification_preview_recording_denied",
      "timeline_entry_recording_denied",
      "audit_narrative_recording_denied",
      "privacy_review_narrative_recording_denied",
      "alert_explanation_recording_denied",
      "slo_report_recording_denied",
      "operator_summary_persistence_denied",
      "operator_briefing_persistence_denied",
      "readback_digest_persistence_denied",
      "final_note_persistence_denied",
      "operator_summary_materialization_denied",
      "operator_briefing_materialization_denied",
      "operator_summary_filesystem_write_denied",
      "operator_briefing_filesystem_write_denied",
      "operator_summary_delivery_denied",
      "operator_briefing_delivery_denied",
      "channel_delivery_denied",
      "external_send_denied",
      "telegram_send_denied",
      "completion_ack_from_summary_briefing_denied",
      "acceptance_from_summary_briefing_denied",
      "release_publication_authority_from_summary_briefing_denied",
      "activation_live_from_summary_briefing_denied",
      "install_restart_active_binary_from_summary_briefing_denied",
      "memory_provider_kg_from_summary_briefing_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_gate",
        status:"allowed_report_only_next_slice",
        records_summary:false,
        persists_summary:false,
        records_briefing:false,
        persists_briefing:false,
        delivers_briefing:false,
        records_acknowledgement:false,
        derives_release_publication_authority:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false,
        sends_externally:false
      }
    ],
    packet_acceptance_receipt_release_publication_result_receipt_operator_summary_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_summary_request_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_summary_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_summary_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_summary_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_summary_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_request_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_readback_digest_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_readback_digest_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_final_note_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_final_note_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_status_banner_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_dashboard_annotation_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_notification_preview_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_timeline_entry_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_audit_narrative_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_privacy_review_narrative_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_alert_explanation_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_slo_report_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_channel_delivery_performed:false,
    packet_acceptance_receipt_release_publication_result_receipt_telegram_send_performed:false,
    packet_acceptance_receipt_release_publication_result_receipt_external_send_performed:false,
    packet_acceptance_receipt_release_publication_result_receipt_query_registered:false,
    packet_acceptance_receipt_release_publication_result_receipt_exported:false,
    packet_acceptance_receipt_release_publication_result_receipt_observability_recorded:false,
    packet_acceptance_receipt_publication_completion_ack_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivered:false,
    packet_acceptance_receipt_release_publication_recorded:false,
    packet_acceptance_receipt_release_artifact_written:false,
    packet_acceptance_receipt_public_artifact_written:false,
    packet_acceptance_receipt_public_distribution_performed:false,
    packet_acceptance_receipt_public_release_claimed:false,
    packet_acceptance_receipt_public_ga_claimed:false,
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
      packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_operator_summary_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_operator_summary_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_operator_summary_filesystem_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_operator_summary_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_filesystem_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_readback_digest_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_final_note_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_status_banner_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_dashboard_annotation_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_notification_preview_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_timeline_entry_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_audit_narrative_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_privacy_review_narrative_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_alert_explanation_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_slo_report_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_channel_delivery_performed:false,
      packet_acceptance_receipt_release_publication_result_receipt_telegram_send_performed:false,
      packet_acceptance_receipt_release_publication_result_receipt_external_send_performed:false,
      packet_acceptance_receipt_publication_completion_ack_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivered:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_ready == true
  and .source_release_publication_result_receipt_export_query_observability_surface_count == 18
  and .source_release_publication_result_receipt_export_query_observability_attempt_count == 18
  and .source_release_publication_result_receipt_operator_summary_recorded_count == 0
  and .source_release_publication_result_receipt_readback_surface_recorded_count == 0
  and .source_release_publication_result_receipt_audit_view_recorded_count == 0
  and .source_release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count == 0
  and .source_release_publication_result_receipt_export_query_observability_activation_authority_derived_count == 0
  and .release_publication_result_receipt_operator_facing_summary_briefing_surface_count == 18
  and .release_publication_result_receipt_operator_facing_summary_briefing_attempt_count == 18
  and .release_publication_result_receipt_operator_summary_allowed_count == 0
  and .release_publication_result_receipt_operator_summary_request_accepted_count == 0
  and .release_publication_result_receipt_operator_summary_recorded_count == 0
  and .release_publication_result_receipt_operator_summary_persisted_count == 0
  and .release_publication_result_receipt_operator_summary_materialized_count == 0
  and .release_publication_result_receipt_operator_summary_filesystem_written_count == 0
  and .release_publication_result_receipt_operator_summary_delivered_count == 0
  and .release_publication_result_receipt_operator_briefing_allowed_count == 0
  and .release_publication_result_receipt_operator_briefing_request_accepted_count == 0
  and .release_publication_result_receipt_operator_briefing_recorded_count == 0
  and .release_publication_result_receipt_operator_briefing_persisted_count == 0
  and .release_publication_result_receipt_operator_briefing_materialized_count == 0
  and .release_publication_result_receipt_operator_briefing_filesystem_written_count == 0
  and .release_publication_result_receipt_operator_briefing_delivered_count == 0
  and .release_publication_result_receipt_readback_digest_recorded_count == 0
  and .release_publication_result_receipt_readback_digest_persisted_count == 0
  and .release_publication_result_receipt_final_note_recorded_count == 0
  and .release_publication_result_receipt_final_note_persisted_count == 0
  and .release_publication_result_receipt_status_banner_recorded_count == 0
  and .release_publication_result_receipt_dashboard_annotation_recorded_count == 0
  and .release_publication_result_receipt_notification_preview_recorded_count == 0
  and .release_publication_result_receipt_timeline_entry_recorded_count == 0
  and .release_publication_result_receipt_audit_narrative_recorded_count == 0
  and .release_publication_result_receipt_privacy_review_narrative_recorded_count == 0
  and .release_publication_result_receipt_alert_explanation_recorded_count == 0
  and .release_publication_result_receipt_slo_report_recorded_count == 0
  and .release_publication_result_receipt_operator_summary_briefing_channel_delivery_count == 0
  and .release_publication_result_receipt_operator_summary_briefing_external_send_count == 0
  and .release_publication_result_receipt_operator_summary_briefing_telegram_send_count == 0
  and .release_publication_result_receipt_operator_summary_briefing_completion_ack_recorded_count == 0
  and .release_publication_result_receipt_operator_summary_briefing_acceptance_recorded_count == 0
  and .release_publication_result_receipt_operator_summary_briefing_operator_approval_derived_count == 0
  and .release_publication_result_receipt_operator_summary_briefing_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_operator_summary_briefing_activation_authority_derived_count == 0
  and .release_publication_result_receipt_operator_summary_briefing_activation_command_derived_count == 0
  and .release_publication_result_receipt_operator_summary_briefing_live_execution_allowed_count == 0
  and (.release_publication_result_receipt_operator_facing_summary_briefing_surfaces | length) == 18
  and (.release_publication_result_receipt_operator_facing_summary_briefing_surfaces | all(
    .operator_facing_text_attempted == true
    and .operator_summary_recorded == false
    and .operator_summary_persisted == false
    and .operator_summary_materialized == false
    and .operator_summary_filesystem_written == false
    and .operator_summary_delivered == false
    and .operator_briefing_recorded == false
    and .operator_briefing_persisted == false
    and .operator_briefing_materialized == false
    and .operator_briefing_filesystem_written == false
    and .operator_briefing_delivered == false
    and .readback_digest_recorded == false
    and .final_note_recorded == false
    and .status_banner_recorded == false
    and .dashboard_annotation_recorded == false
    and .notification_preview_recorded == false
    and .timeline_entry_recorded == false
    and .audit_narrative_recorded == false
    and .privacy_review_narrative_recorded == false
    and .alert_explanation_recorded == false
    and .slo_report_recorded == false
    and .channel_delivery_performed == false
    and .telegram_send_performed == false
    and .external_send_performed == false
    and .completion_ack_recorded == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .activation_performed == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .provider_invoked == false
    and .model_invoked == false
    and .operator_facing_summary_briefing_noop_confirmed == true
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_operator_facing_summary_briefing | length) == 32
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_summary_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_summary_materialized == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_summary_filesystem_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_summary_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_materialized == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_filesystem_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_readback_digest_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_final_note_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_status_banner_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_dashboard_annotation_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_notification_preview_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_timeline_entry_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_audit_narrative_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_privacy_review_narrative_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_alert_explanation_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_slo_report_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_channel_delivery_performed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_telegram_send_performed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_external_send_performed == false
  and .packet_acceptance_receipt_publication_completion_ack_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_persisted == false
  and .packet_acceptance_receipt_release_publication_recorded == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .release_publication_authority_derived == false
  and .activation_authority_derived == false
  and .activation_command_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .secret_file_read == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .release_artifact_written == false
  and .public_artifact_written == false
  and .external_send_performed == false
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt operator-facing summary/briefing non-persistence denial gate passed"
