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

TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_PRIVACY_REDACTION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-privacy-redaction-exposure-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-privacy-redaction-exposure-denial-gate.sh
)"

delivery_receipt_privacy_redaction_report_sha256="$(
  sha256_text "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_PRIVACY_REDACTION_JSON"
)"
delivery_receipt_operator_briefing_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-operator-briefing-non-persistence-denial:$delivery_receipt_privacy_redaction_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
delivery_receipt_operator_briefing_policy_hash_sha256="$(
  sha256_text "release-publication-result-receipt-terminal-distribution-delivery-receipt-operator-briefing-non-persistence:no-summary:no-briefing:no-readback:no-final-note:no-notification:no-channel:no-authority:no-live"
)"

jq -n -e \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_PRIVACY_REDACTION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_request_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_accepted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_persisted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_materialized_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_filesystem_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_delivered_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_exposed_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_redacted_payload_preview_rendered_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_payload_hash_preview_rendered_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_payload_diff_rendered_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_payload_summary_rendered_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_operator_readback_text_rendered_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_privacy_review_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_secret_scan_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_pii_scan_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_raw_payload_inspected_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_plaintext_materialized_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_redaction_bypass_performed_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_hash_to_payload_linked_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_export_redacted_payload_written_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_observability_redacted_payload_recorded_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_dashboard_redaction_badge_exposed_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_audit_redaction_view_exposed_count == 0
    and $source.release_publication_result_receipt_delivery_receipt_payload_exposure_evidence_exposed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_acceptance_recorded_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_operator_approval_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_release_publication_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_activation_authority_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_activation_command_derived_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_live_execution_allowed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_install_executed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_service_restarted_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_active_binary_mutated_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_release_artifact_written_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_public_artifact_written_count == 0
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_allowed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_request_accepted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_accepted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_persisted == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_materialized == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_filesystem_written == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_delivered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_exposed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_redacted_payload_preview_rendered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_hash_preview_rendered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_diff_rendered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_summary_rendered == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_privacy_review_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_secret_scan_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_pii_scan_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_raw_payload_inspected == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_plaintext_materialized == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_redaction_bypass_performed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_hash_to_payload_linked == false
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
    and ($source.allowed_next_actions | any(.action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_non_persistence_denial_gate" and .status == "allowed_report_only_next_slice" and .renders_payload == false and .exposes_redacted_payload == false and .records_privacy_review == false and .records_operator_acceptance == false and .derives_release_publication_authority == false and .derives_activation_authority == false and .activates_live == false and .mutates_memory_store == false and .writes_kg == false and .sends_externally == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

delivery_receipt_operator_briefing_surfaces_json="$(
  jq -n '
    def briefing_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_surface:$id,
        source_terminal_distribution_delivery_receipt_privacy_redaction_ready:true,
        operator_briefing_attempted:true,
        operator_briefing_allowed:false,
        operator_briefing_request_accepted:false,
        operator_briefing_accepted:false,
        operator_briefing_recorded:false,
        operator_briefing_persisted:false,
        operator_briefing_materialized:false,
        operator_briefing_filesystem_written:false,
        operator_briefing_delivered:false,
        operator_summary_recorded:false,
        operator_summary_persisted:false,
        operator_summary_materialized:false,
        operator_summary_delivered:false,
        readback_digest_recorded:false,
        readback_digest_persisted:false,
        final_note_recorded:false,
        final_note_persisted:false,
        status_banner_recorded:false,
        notification_preview_recorded:false,
        timeline_entry_recorded:false,
        dashboard_annotation_recorded:false,
        audit_narrative_recorded:false,
        privacy_review_narrative_recorded:false,
        payload_safe_summary_recorded:false,
        redaction_summary_recorded:false,
        alert_explanation_recorded:false,
        slo_report_recorded:false,
        channel_message_delivered:false,
        external_message_sent:false,
        telegram_message_sent:false,
        completion_ack_recorded:false,
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
        operator_briefing_noop_confirmed:true,
        operator_briefing_status:$status,
        reason:$reason
      } + $extra;
    [
      briefing_surface("publication_result_receipt_delivery_receipt_operator_summary"; "blocked_operator_summary_noop"; "operator_summary_recording_denied"; {operator_summary_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_operator_briefing"; "blocked_operator_briefing_noop"; "operator_briefing_recording_denied"; {operator_briefing_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_readback_digest"; "blocked_readback_digest_noop"; "readback_digest_recording_denied"; {readback_digest_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_final_note"; "blocked_final_note_noop"; "final_note_recording_denied"; {final_note_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_status_banner"; "blocked_status_banner_noop"; "status_banner_recording_denied"; {status_banner_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_notification_preview"; "blocked_notification_preview_noop"; "notification_preview_recording_denied"; {notification_preview_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_timeline_entry"; "blocked_timeline_entry_noop"; "timeline_entry_recording_denied"; {timeline_entry_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_dashboard_annotation"; "blocked_dashboard_annotation_noop"; "dashboard_annotation_recording_denied"; {dashboard_annotation_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_audit_narrative"; "blocked_audit_narrative_noop"; "audit_narrative_recording_denied"; {audit_narrative_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_privacy_review_narrative"; "blocked_privacy_review_narrative_noop"; "privacy_review_narrative_recording_denied"; {privacy_review_narrative_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_payload_safe_summary"; "blocked_payload_safe_summary_noop"; "payload_safe_summary_recording_denied"; {payload_safe_summary_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_redaction_summary"; "blocked_redaction_summary_noop"; "redaction_summary_recording_denied"; {redaction_summary_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_alert_explanation"; "blocked_alert_explanation_noop"; "alert_explanation_recording_denied"; {alert_explanation_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_slo_report"; "blocked_slo_report_noop"; "slo_report_recording_denied"; {slo_report_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_channel_message"; "blocked_channel_message_noop"; "channel_message_delivery_denied"; {channel_message_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_external_message"; "blocked_external_message_noop"; "external_message_send_denied"; {external_message_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_telegram_message"; "blocked_telegram_message_noop"; "telegram_message_send_denied"; {telegram_message_requested:true}),
      briefing_surface("publication_result_receipt_delivery_receipt_authority_live_active_binary_briefing"; "blocked_authority_live_active_binary_briefing_noop"; "authority_live_active_binary_from_briefing_denied"; {release_publication_authority_briefing_requested:true, activation_live_briefing_requested:true, install_restart_active_binary_briefing_requested:true})
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_non_persistence_denial_gate" \
  --arg delivery_receipt_privacy_redaction_report_sha256 "$delivery_receipt_privacy_redaction_report_sha256" \
  --arg delivery_receipt_operator_briefing_contract_hash_sha256 "$delivery_receipt_operator_briefing_contract_hash_sha256" \
  --arg delivery_receipt_operator_briefing_policy_hash_sha256 "$delivery_receipt_operator_briefing_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_PRIVACY_REDACTION_JSON" \
  --argjson surfaces "$delivery_receipt_operator_briefing_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_non_persistence_denial_v1",
    receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_mode:"denied_delivery_receipt_privacy_redaction_views_cannot_become_operator_briefing_or_authority",
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_denial_ready,
    source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_report_sha256:$delivery_receipt_privacy_redaction_report_sha256,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_contract_hash_sha256:$delivery_receipt_operator_briefing_contract_hash_sha256,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_policy_hash_sha256:$delivery_receipt_operator_briefing_policy_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_non_persistence_denial_ready:true,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_surface_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_attempt_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_recorded_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_persisted_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_persisted_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_exposed_count,
    source_release_publication_result_receipt_delivery_receipt_redacted_payload_preview_rendered_count:$source.release_publication_result_receipt_delivery_receipt_redacted_payload_preview_rendered_count,
    source_release_publication_result_receipt_delivery_receipt_payload_hash_preview_rendered_count:$source.release_publication_result_receipt_delivery_receipt_payload_hash_preview_rendered_count,
    source_release_publication_result_receipt_delivery_receipt_payload_summary_rendered_count:$source.release_publication_result_receipt_delivery_receipt_payload_summary_rendered_count,
    source_release_publication_result_receipt_delivery_receipt_privacy_review_recorded_count:$source.release_publication_result_receipt_delivery_receipt_privacy_review_recorded_count,
    source_release_publication_result_receipt_delivery_receipt_secret_scan_recorded_count:$source.release_publication_result_receipt_delivery_receipt_secret_scan_recorded_count,
    source_release_publication_result_receipt_delivery_receipt_pii_scan_recorded_count:$source.release_publication_result_receipt_delivery_receipt_pii_scan_recorded_count,
    source_release_publication_result_receipt_delivery_receipt_raw_payload_inspected_count:$source.release_publication_result_receipt_delivery_receipt_raw_payload_inspected_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_release_publication_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_release_publication_authority_derived_count,
    source_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_activation_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_activation_authority_derived_count,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_surface_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_attempt_count:($surfaces | length),
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_request_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_accepted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_persisted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_materialized_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_filesystem_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_delivered_count:0,
    release_publication_result_receipt_delivery_receipt_operator_summary_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_operator_summary_persisted_count:0,
    release_publication_result_receipt_delivery_receipt_operator_summary_materialized_count:0,
    release_publication_result_receipt_delivery_receipt_operator_summary_delivered_count:0,
    release_publication_result_receipt_delivery_receipt_operator_briefing_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_operator_briefing_persisted_count:0,
    release_publication_result_receipt_delivery_receipt_operator_briefing_materialized_count:0,
    release_publication_result_receipt_delivery_receipt_operator_briefing_delivered_count:0,
    release_publication_result_receipt_delivery_receipt_readback_digest_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_readback_digest_persisted_count:0,
    release_publication_result_receipt_delivery_receipt_final_note_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_final_note_persisted_count:0,
    release_publication_result_receipt_delivery_receipt_status_banner_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_notification_preview_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_timeline_entry_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_dashboard_annotation_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_audit_narrative_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_privacy_review_narrative_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_payload_safe_summary_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_redaction_summary_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_alert_explanation_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_slo_report_recorded_count:0,
    release_publication_result_receipt_delivery_receipt_channel_message_delivered_count:0,
    release_publication_result_receipt_delivery_receipt_external_message_sent_count:0,
    release_publication_result_receipt_delivery_receipt_telegram_message_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_completion_ack_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_acceptance_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_operator_approval_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_release_publication_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_activation_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_activation_command_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_live_execution_allowed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_install_executed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_service_restarted_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_active_binary_mutated_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_release_artifact_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_public_artifact_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing:[
      "source_terminal_distribution_delivery_receipt_privacy_redaction_report_required",
      "operator_briefing_request_acceptance_denied",
      "operator_briefing_acceptance_denied",
      "operator_briefing_surface_recording_denied",
      "operator_briefing_surface_persistence_denied",
      "operator_briefing_surface_materialization_denied",
      "operator_briefing_surface_filesystem_write_denied",
      "operator_briefing_surface_delivery_denied",
      "operator_summary_recording_denied",
      "operator_summary_persistence_denied",
      "operator_summary_materialization_denied",
      "operator_summary_delivery_denied",
      "operator_briefing_recording_denied",
      "operator_briefing_persistence_denied",
      "operator_briefing_materialization_denied",
      "operator_briefing_delivery_denied",
      "readback_digest_recording_denied",
      "final_note_recording_denied",
      "status_banner_recording_denied",
      "notification_preview_recording_denied",
      "timeline_entry_recording_denied",
      "dashboard_annotation_recording_denied",
      "audit_narrative_recording_denied",
      "privacy_review_narrative_recording_denied",
      "payload_safe_summary_recording_denied",
      "redaction_summary_recording_denied",
      "alert_explanation_recording_denied",
      "slo_report_recording_denied",
      "channel_message_delivery_denied",
      "external_message_send_denied",
      "telegram_message_send_denied",
      "completion_ack_from_briefing_denied",
      "acceptance_from_briefing_denied",
      "operator_approval_from_briefing_denied",
      "release_publication_authority_from_briefing_denied",
      "activation_live_from_briefing_denied",
      "install_restart_active_binary_from_briefing_denied",
      "release_artifact_write_denied",
      "public_artifact_write_denied",
      "memory_provider_kg_from_briefing_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_final_operator_acknowledgement_non_acceptance_denial_gate",
        status:"allowed_report_only_next_slice",
        records_summary:false,
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
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_request_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_exposed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_redacted_payload_preview_rendered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_hash_preview_rendered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_diff_rendered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_summary_rendered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_readback_text_rendered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_privacy_review_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_secret_scan_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_pii_scan_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_raw_payload_inspected:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_plaintext_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_redaction_bypass_performed:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_hash_to_payload_linked:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_request_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_summary_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_summary_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_summary_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_briefing_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_briefing_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_briefing_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_readback_digest_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_final_note_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_status_banner_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_notification_preview_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_timeline_entry_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_dashboard_annotation_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_audit_narrative_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_privacy_review_narrative_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_safe_summary_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_redaction_summary_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_alert_explanation_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_slo_report_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_channel_message_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_external_message_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_telegram_message_sent:false,
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
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_accepted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_filesystem_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_summary_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_summary_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_summary_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_briefing_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_briefing_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_briefing_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_readback_digest_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_final_note_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_status_banner_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_notification_preview_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_timeline_entry_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_dashboard_annotation_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_audit_narrative_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_privacy_review_narrative_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_safe_summary_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_redaction_summary_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_alert_explanation_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_slo_report_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_channel_message_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_external_message_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_telegram_message_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_exposed:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_summary_rendered:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_privacy_review_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_secret_scan_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_pii_scan_recorded:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_non_persistence_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_non_persistence_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_ready == true
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_surface_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_attempt_count == 18
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_recorded_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_persisted_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposed_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_redacted_payload_preview_rendered_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_payload_hash_preview_rendered_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_payload_summary_rendered_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_privacy_review_recorded_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_secret_scan_recorded_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_pii_scan_recorded_count == 0
  and .source_release_publication_result_receipt_delivery_receipt_raw_payload_inspected_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_release_publication_authority_derived_count == 0
  and .source_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_request_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_persisted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_materialized_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_filesystem_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_delivered_count == 0
  and .release_publication_result_receipt_delivery_receipt_operator_summary_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_operator_summary_persisted_count == 0
  and .release_publication_result_receipt_delivery_receipt_operator_summary_materialized_count == 0
  and .release_publication_result_receipt_delivery_receipt_operator_summary_delivered_count == 0
  and .release_publication_result_receipt_delivery_receipt_operator_briefing_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_operator_briefing_persisted_count == 0
  and .release_publication_result_receipt_delivery_receipt_operator_briefing_materialized_count == 0
  and .release_publication_result_receipt_delivery_receipt_operator_briefing_delivered_count == 0
  and .release_publication_result_receipt_delivery_receipt_readback_digest_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_final_note_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_status_banner_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_notification_preview_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_timeline_entry_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_dashboard_annotation_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_audit_narrative_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_privacy_review_narrative_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_payload_safe_summary_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_redaction_summary_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_alert_explanation_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_slo_report_recorded_count == 0
  and .release_publication_result_receipt_delivery_receipt_channel_message_delivered_count == 0
  and .release_publication_result_receipt_delivery_receipt_external_message_sent_count == 0
  and .release_publication_result_receipt_delivery_receipt_telegram_message_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_completion_ack_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_acceptance_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_operator_approval_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_activation_command_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_live_execution_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_install_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_service_restarted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_active_binary_mutated_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_release_artifact_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_public_artifact_written_count == 0
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_surfaces | length) == 18
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_surfaces | all(
    .operator_briefing_attempted == true
    and .operator_briefing_allowed == false
    and .operator_briefing_request_accepted == false
    and .operator_briefing_accepted == false
    and .operator_briefing_recorded == false
    and .operator_briefing_persisted == false
    and .operator_briefing_materialized == false
    and .operator_briefing_filesystem_written == false
    and .operator_briefing_delivered == false
    and .operator_summary_recorded == false
    and .operator_summary_persisted == false
    and .operator_summary_materialized == false
    and .operator_summary_delivered == false
    and .readback_digest_recorded == false
    and .readback_digest_persisted == false
    and .final_note_recorded == false
    and .final_note_persisted == false
    and .status_banner_recorded == false
    and .notification_preview_recorded == false
    and .timeline_entry_recorded == false
    and .dashboard_annotation_recorded == false
    and .audit_narrative_recorded == false
    and .privacy_review_narrative_recorded == false
    and .payload_safe_summary_recorded == false
    and .redaction_summary_recorded == false
    and .alert_explanation_recorded == false
    and .slo_report_recorded == false
    and .channel_message_delivered == false
    and .external_message_sent == false
    and .telegram_message_sent == false
    and .completion_ack_recorded == false
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
    and .operator_briefing_noop_confirmed == true
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing | length) == 40
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_materialized == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_filesystem_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_privacy_redaction_exposure_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_summary_rendered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_privacy_review_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_secret_scan_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_pii_scan_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_materialized == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_filesystem_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_operator_briefing_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_summary_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_summary_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_summary_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_briefing_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_briefing_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_operator_briefing_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_readback_digest_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_final_note_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_status_banner_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_notification_preview_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_timeline_entry_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_dashboard_annotation_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_audit_narrative_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_privacy_review_narrative_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_payload_safe_summary_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_redaction_summary_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_alert_explanation_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_slo_report_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_channel_message_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_external_message_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_delivery_receipt_telegram_message_sent == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt operator briefing non-persistence denial gate passed"
