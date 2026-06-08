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

SUMMARY_BRIEFING_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate.sh
)"

summary_briefing_report_sha256="$(sha256_text "$SUMMARY_BRIEFING_JSON")"
final_operator_acknowledgement_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-final-operator-acknowledgement-non-acceptance-denial:$summary_briefing_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
final_operator_acknowledgement_policy_hash_sha256="$(
  sha256_text "release-publication-result-receipt-final-operator-acknowledgement-non-acceptance:no-final-ack:no-received:no-confirmed:no-read:no-delivery:no-authority:no-live"
)"

jq -n -e \
  --argjson source "$SUMMARY_BRIEFING_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_ready == true
    and $source.release_publication_result_receipt_operator_facing_summary_briefing_surface_count == 18
    and $source.release_publication_result_receipt_operator_facing_summary_briefing_attempt_count == 18
    and $source.release_publication_result_receipt_operator_summary_allowed_count == 0
    and $source.release_publication_result_receipt_operator_summary_request_accepted_count == 0
    and $source.release_publication_result_receipt_operator_summary_recorded_count == 0
    and $source.release_publication_result_receipt_operator_summary_persisted_count == 0
    and $source.release_publication_result_receipt_operator_summary_materialized_count == 0
    and $source.release_publication_result_receipt_operator_summary_filesystem_written_count == 0
    and $source.release_publication_result_receipt_operator_summary_delivered_count == 0
    and $source.release_publication_result_receipt_operator_briefing_allowed_count == 0
    and $source.release_publication_result_receipt_operator_briefing_request_accepted_count == 0
    and $source.release_publication_result_receipt_operator_briefing_recorded_count == 0
    and $source.release_publication_result_receipt_operator_briefing_persisted_count == 0
    and $source.release_publication_result_receipt_operator_briefing_materialized_count == 0
    and $source.release_publication_result_receipt_operator_briefing_filesystem_written_count == 0
    and $source.release_publication_result_receipt_operator_briefing_delivered_count == 0
    and $source.release_publication_result_receipt_readback_digest_recorded_count == 0
    and $source.release_publication_result_receipt_final_note_recorded_count == 0
    and $source.release_publication_result_receipt_status_banner_recorded_count == 0
    and $source.release_publication_result_receipt_dashboard_annotation_recorded_count == 0
    and $source.release_publication_result_receipt_notification_preview_recorded_count == 0
    and $source.release_publication_result_receipt_timeline_entry_recorded_count == 0
    and $source.release_publication_result_receipt_audit_narrative_recorded_count == 0
    and $source.release_publication_result_receipt_privacy_review_narrative_recorded_count == 0
    and $source.release_publication_result_receipt_alert_explanation_recorded_count == 0
    and $source.release_publication_result_receipt_slo_report_recorded_count == 0
    and $source.release_publication_result_receipt_operator_summary_briefing_channel_delivery_count == 0
    and $source.release_publication_result_receipt_operator_summary_briefing_external_send_count == 0
    and $source.release_publication_result_receipt_operator_summary_briefing_telegram_send_count == 0
    and $source.release_publication_result_receipt_operator_summary_briefing_completion_ack_recorded_count == 0
    and $source.release_publication_result_receipt_operator_summary_briefing_acceptance_recorded_count == 0
    and $source.release_publication_result_receipt_operator_summary_briefing_operator_approval_derived_count == 0
    and $source.release_publication_result_receipt_operator_summary_briefing_release_publication_authority_derived_count == 0
    and $source.release_publication_result_receipt_operator_summary_briefing_activation_authority_derived_count == 0
    and $source.release_publication_result_receipt_operator_summary_briefing_activation_command_derived_count == 0
    and $source.release_publication_result_receipt_operator_summary_briefing_live_execution_allowed_count == 0
    and $source.packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_readback_digest_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_final_note_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_channel_delivery_performed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_telegram_send_performed == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_external_send_performed == false
    and $source.packet_acceptance_receipt_publication_completion_ack_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_recorded == false
    and $source.packet_acceptance_receipt_release_publication_result_receipt_persisted == false
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

final_acknowledgement_surfaces_json="$(
  jq -n '
    def final_ack_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_final_operator_acknowledgement_surface:$id,
        source_operator_summary_briefing_ready:true,
        final_operator_acknowledgement_attempted:true,
        final_operator_acknowledgement_allowed:false,
        final_operator_acknowledgement_request_accepted:false,
        final_operator_acknowledgement_accepted:false,
        final_operator_acknowledgement_recorded:false,
        final_operator_acknowledgement_persisted:false,
        final_operator_acknowledgement_materialized:false,
        final_operator_acknowledgement_filesystem_written:false,
        final_operator_acknowledgement_delivered:false,
        operator_received_recorded:false,
        operator_confirmed_recorded:false,
        operator_read_recorded:false,
        operator_seen_recorded:false,
        final_response_recorded:false,
        completion_ack_recorded:false,
        status_ack_recorded:false,
        summary_ack_recorded:false,
        briefing_ack_recorded:false,
        readback_digest_ack_recorded:false,
        dashboard_ack_recorded:false,
        notification_ack_recorded:false,
        channel_ack_delivered:false,
        external_ack_sent:false,
        telegram_ack_sent:false,
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
        final_operator_acknowledgement_noop_confirmed:true,
        final_operator_acknowledgement_status:$status,
        reason:$reason
      } + $extra;
    [
      final_ack_surface("publication_result_receipt_final_operator_acknowledgement_claim"; "blocked_final_ack_noop"; "final_operator_acknowledgement_acceptance_denied"; {final_operator_acknowledgement_requested:true}),
      final_ack_surface("publication_result_receipt_operator_received_claim"; "blocked_received_noop"; "operator_received_recording_denied"; {operator_received_requested:true}),
      final_ack_surface("publication_result_receipt_operator_confirmed_claim"; "blocked_confirmed_noop"; "operator_confirmed_recording_denied"; {operator_confirmed_requested:true}),
      final_ack_surface("publication_result_receipt_operator_read_claim"; "blocked_read_noop"; "operator_read_recording_denied"; {operator_read_requested:true}),
      final_ack_surface("publication_result_receipt_operator_seen_claim"; "blocked_seen_noop"; "operator_seen_recording_denied"; {operator_seen_requested:true}),
      final_ack_surface("publication_result_receipt_final_response_claim"; "blocked_final_response_noop"; "final_response_recording_denied"; {final_response_requested:true}),
      final_ack_surface("publication_result_receipt_completion_acknowledgement_claim"; "blocked_completion_noop"; "completion_acknowledgement_recording_denied"; {completion_ack_requested:true}),
      final_ack_surface("publication_result_receipt_status_acknowledgement_claim"; "blocked_status_noop"; "status_acknowledgement_recording_denied"; {status_ack_requested:true}),
      final_ack_surface("publication_result_receipt_summary_acknowledgement_claim"; "blocked_summary_ack_noop"; "summary_acknowledgement_recording_denied"; {summary_ack_requested:true}),
      final_ack_surface("publication_result_receipt_briefing_acknowledgement_claim"; "blocked_briefing_ack_noop"; "briefing_acknowledgement_recording_denied"; {briefing_ack_requested:true}),
      final_ack_surface("publication_result_receipt_readback_digest_acknowledgement_claim"; "blocked_readback_ack_noop"; "readback_digest_acknowledgement_recording_denied"; {readback_digest_ack_requested:true}),
      final_ack_surface("publication_result_receipt_dashboard_notification_acknowledgement_claim"; "blocked_dashboard_notification_ack_noop"; "dashboard_notification_acknowledgement_recording_denied"; {dashboard_ack_requested:true, notification_ack_requested:true}),
      final_ack_surface("publication_result_receipt_channel_acknowledgement_claim"; "blocked_channel_ack_noop"; "channel_acknowledgement_delivery_denied"; {channel_ack_requested:true}),
      final_ack_surface("publication_result_receipt_external_acknowledgement_claim"; "blocked_external_ack_noop"; "external_acknowledgement_send_denied"; {external_ack_requested:true}),
      final_ack_surface("publication_result_receipt_telegram_acknowledgement_claim"; "blocked_telegram_ack_noop"; "telegram_acknowledgement_send_denied"; {telegram_ack_requested:true}),
      final_ack_surface("publication_result_receipt_release_publication_authority_acknowledgement_claim"; "blocked_authority_ack_noop"; "release_publication_authority_from_acknowledgement_denied"; {release_publication_authority_ack_requested:true}),
      final_ack_surface("publication_result_receipt_activation_live_acknowledgement_claim"; "blocked_activation_ack_noop"; "activation_live_from_acknowledgement_denied"; {activation_authority_ack_requested:true, live_execution_ack_requested:true}),
      final_ack_surface("publication_result_receipt_install_restart_active_binary_acknowledgement_claim"; "blocked_active_binary_ack_noop"; "install_restart_active_binary_from_acknowledgement_denied"; {install_restart_active_binary_ack_requested:true})
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_gate" \
  --arg summary_briefing_report_sha256 "$summary_briefing_report_sha256" \
  --arg final_operator_acknowledgement_contract_hash_sha256 "$final_operator_acknowledgement_contract_hash_sha256" \
  --arg final_operator_acknowledgement_policy_hash_sha256 "$final_operator_acknowledgement_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$SUMMARY_BRIEFING_JSON" \
  --argjson surfaces "$final_acknowledgement_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_result_receipt_final_operator_acknowledgement_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_v1",
    receipt_release_publication_result_receipt_final_operator_acknowledgement_mode:"denied_release_publication_result_receipt_cannot_become_final_operator_acknowledgement_acceptance_or_authority",
    source_packet_acceptance_receipt_release_publication_result_receipt_operator_summary_briefing_gate:$source.gate,
    source_packet_acceptance_receipt_release_publication_result_receipt_operator_summary_briefing_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready,
    source_packet_acceptance_receipt_release_publication_result_receipt_operator_summary_briefing_report_sha256:$summary_briefing_report_sha256,
    source_release_publication_result_receipt_operator_summary_briefing_contract_hash_sha256:$source.release_publication_result_receipt_operator_summary_briefing_contract_hash_sha256,
    release_publication_result_receipt_final_operator_acknowledgement_contract_hash_sha256:$final_operator_acknowledgement_contract_hash_sha256,
    release_publication_result_receipt_final_operator_acknowledgement_policy_hash_sha256:$final_operator_acknowledgement_policy_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready:true,
    source_release_publication_result_receipt_operator_summary_briefing_surface_count:$source.release_publication_result_receipt_operator_facing_summary_briefing_surface_count,
    source_release_publication_result_receipt_operator_summary_briefing_attempt_count:$source.release_publication_result_receipt_operator_facing_summary_briefing_attempt_count,
    source_release_publication_result_receipt_operator_summary_recorded_count:$source.release_publication_result_receipt_operator_summary_recorded_count,
    source_release_publication_result_receipt_operator_briefing_recorded_count:$source.release_publication_result_receipt_operator_briefing_recorded_count,
    source_release_publication_result_receipt_readback_digest_recorded_count:$source.release_publication_result_receipt_readback_digest_recorded_count,
    source_release_publication_result_receipt_final_note_recorded_count:$source.release_publication_result_receipt_final_note_recorded_count,
    source_release_publication_result_receipt_operator_summary_briefing_completion_ack_recorded_count:$source.release_publication_result_receipt_operator_summary_briefing_completion_ack_recorded_count,
    source_release_publication_result_receipt_operator_summary_briefing_release_publication_authority_derived_count:$source.release_publication_result_receipt_operator_summary_briefing_release_publication_authority_derived_count,
    source_release_publication_result_receipt_operator_summary_briefing_activation_authority_derived_count:$source.release_publication_result_receipt_operator_summary_briefing_activation_authority_derived_count,
    release_publication_result_receipt_final_operator_acknowledgement_surface_count:($surfaces | length),
    release_publication_result_receipt_final_operator_acknowledgement_attempt_count:($surfaces | length),
    release_publication_result_receipt_final_operator_acknowledgement_allowed_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_request_accepted_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_accepted_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_recorded_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_persisted_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_materialized_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_filesystem_written_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_delivered_count:0,
    release_publication_result_receipt_operator_received_recorded_count:0,
    release_publication_result_receipt_operator_confirmed_recorded_count:0,
    release_publication_result_receipt_operator_read_recorded_count:0,
    release_publication_result_receipt_operator_seen_recorded_count:0,
    release_publication_result_receipt_final_response_recorded_count:0,
    release_publication_result_receipt_completion_ack_recorded_count:0,
    release_publication_result_receipt_status_ack_recorded_count:0,
    release_publication_result_receipt_summary_ack_recorded_count:0,
    release_publication_result_receipt_briefing_ack_recorded_count:0,
    release_publication_result_receipt_readback_digest_ack_recorded_count:0,
    release_publication_result_receipt_dashboard_ack_recorded_count:0,
    release_publication_result_receipt_notification_ack_recorded_count:0,
    release_publication_result_receipt_channel_ack_delivered_count:0,
    release_publication_result_receipt_external_ack_sent_count:0,
    release_publication_result_receipt_telegram_ack_sent_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_acceptance_recorded_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_operator_approval_derived_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_release_publication_authority_derived_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_activation_authority_derived_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_activation_command_derived_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_live_execution_allowed_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_install_executed_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_service_restarted_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_active_binary_mutated_count:0,
    release_publication_result_receipt_final_operator_acknowledgement_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication_result_receipt_final_operator_acknowledgement:[
      "source_operator_summary_briefing_report_required",
      "final_operator_acknowledgement_request_acceptance_denied",
      "final_operator_acknowledgement_acceptance_denied",
      "final_operator_acknowledgement_recording_denied",
      "final_operator_acknowledgement_persistence_denied",
      "final_operator_acknowledgement_materialization_denied",
      "final_operator_acknowledgement_filesystem_write_denied",
      "final_operator_acknowledgement_delivery_denied",
      "operator_received_recording_denied",
      "operator_confirmed_recording_denied",
      "operator_read_recording_denied",
      "operator_seen_recording_denied",
      "final_response_recording_denied",
      "completion_acknowledgement_recording_denied",
      "status_acknowledgement_recording_denied",
      "summary_acknowledgement_recording_denied",
      "briefing_acknowledgement_recording_denied",
      "readback_digest_acknowledgement_recording_denied",
      "dashboard_notification_acknowledgement_recording_denied",
      "channel_acknowledgement_delivery_denied",
      "external_acknowledgement_send_denied",
      "telegram_acknowledgement_send_denied",
      "acceptance_from_final_operator_acknowledgement_denied",
      "operator_approval_from_final_operator_acknowledgement_denied",
      "release_publication_authority_from_final_operator_acknowledgement_denied",
      "activation_live_from_final_operator_acknowledgement_denied",
      "install_restart_active_binary_from_final_operator_acknowledgement_denied",
      "memory_provider_kg_from_final_operator_acknowledgement_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_gate",
        status:"allowed_report_only_next_slice",
        records_acknowledgement:false,
        persists_acknowledgement:false,
        records_operator_acceptance:false,
        derives_release_publication_authority:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false,
        sends_externally:false
      }
    ],
    packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_allowed:false,
    packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_request_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_accepted:false,
    packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_persisted:false,
    packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_materialized:false,
    packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_filesystem_written:false,
    packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_received_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_confirmed_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_read_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_seen_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_final_response_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_completion_ack_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_status_ack_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_summary_ack_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_briefing_ack_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_readback_digest_ack_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_dashboard_ack_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_notification_ack_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_channel_ack_delivered:false,
    packet_acceptance_receipt_release_publication_result_receipt_external_ack_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_telegram_ack_sent:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_readback_digest_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_final_note_recorded:false,
    packet_acceptance_receipt_release_publication_result_receipt_channel_delivery_performed:false,
    packet_acceptance_receipt_release_publication_result_receipt_telegram_send_performed:false,
    packet_acceptance_receipt_release_publication_result_receipt_external_send_performed:false,
    packet_acceptance_receipt_publication_completion_ack_recorded:false,
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
      packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_persisted:false,
      packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_materialized:false,
      packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_filesystem_written:false,
      packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_operator_received_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_operator_confirmed_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_operator_read_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_operator_seen_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_final_response_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_completion_ack_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_status_ack_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_summary_ack_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_briefing_ack_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_readback_digest_ack_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_channel_ack_delivered:false,
      packet_acceptance_receipt_release_publication_result_receipt_external_ack_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_telegram_ack_sent:false,
      packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_readback_digest_recorded:false,
      packet_acceptance_receipt_release_publication_result_receipt_final_note_recorded:false,
      packet_acceptance_receipt_publication_completion_ack_recorded:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_operator_summary_briefing_ready == true
  and .source_release_publication_result_receipt_operator_summary_briefing_surface_count == 18
  and .source_release_publication_result_receipt_operator_summary_briefing_attempt_count == 18
  and .source_release_publication_result_receipt_operator_summary_recorded_count == 0
  and .source_release_publication_result_receipt_operator_briefing_recorded_count == 0
  and .source_release_publication_result_receipt_readback_digest_recorded_count == 0
  and .source_release_publication_result_receipt_final_note_recorded_count == 0
  and .source_release_publication_result_receipt_operator_summary_briefing_completion_ack_recorded_count == 0
  and .source_release_publication_result_receipt_operator_summary_briefing_release_publication_authority_derived_count == 0
  and .source_release_publication_result_receipt_operator_summary_briefing_activation_authority_derived_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_surface_count == 18
  and .release_publication_result_receipt_final_operator_acknowledgement_attempt_count == 18
  and .release_publication_result_receipt_final_operator_acknowledgement_allowed_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_request_accepted_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_accepted_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_recorded_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_persisted_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_materialized_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_filesystem_written_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_delivered_count == 0
  and .release_publication_result_receipt_operator_received_recorded_count == 0
  and .release_publication_result_receipt_operator_confirmed_recorded_count == 0
  and .release_publication_result_receipt_operator_read_recorded_count == 0
  and .release_publication_result_receipt_operator_seen_recorded_count == 0
  and .release_publication_result_receipt_final_response_recorded_count == 0
  and .release_publication_result_receipt_completion_ack_recorded_count == 0
  and .release_publication_result_receipt_status_ack_recorded_count == 0
  and .release_publication_result_receipt_summary_ack_recorded_count == 0
  and .release_publication_result_receipt_briefing_ack_recorded_count == 0
  and .release_publication_result_receipt_readback_digest_ack_recorded_count == 0
  and .release_publication_result_receipt_dashboard_ack_recorded_count == 0
  and .release_publication_result_receipt_notification_ack_recorded_count == 0
  and .release_publication_result_receipt_channel_ack_delivered_count == 0
  and .release_publication_result_receipt_external_ack_sent_count == 0
  and .release_publication_result_receipt_telegram_ack_sent_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_acceptance_recorded_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_operator_approval_derived_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_activation_authority_derived_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_activation_command_derived_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_live_execution_allowed_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_install_executed_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_service_restarted_count == 0
  and .release_publication_result_receipt_final_operator_acknowledgement_active_binary_mutated_count == 0
  and (.release_publication_result_receipt_final_operator_acknowledgement_surfaces | length) == 18
  and (.release_publication_result_receipt_final_operator_acknowledgement_surfaces | all(
    .final_operator_acknowledgement_attempted == true
    and .final_operator_acknowledgement_accepted == false
    and .final_operator_acknowledgement_recorded == false
    and .final_operator_acknowledgement_persisted == false
    and .final_operator_acknowledgement_materialized == false
    and .final_operator_acknowledgement_filesystem_written == false
    and .final_operator_acknowledgement_delivered == false
    and .operator_received_recorded == false
    and .operator_confirmed_recorded == false
    and .operator_read_recorded == false
    and .operator_seen_recorded == false
    and .final_response_recorded == false
    and .completion_ack_recorded == false
    and .status_ack_recorded == false
    and .summary_ack_recorded == false
    and .briefing_ack_recorded == false
    and .readback_digest_ack_recorded == false
    and .dashboard_ack_recorded == false
    and .notification_ack_recorded == false
    and .channel_ack_delivered == false
    and .external_ack_sent == false
    and .telegram_ack_sent == false
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
    and .provider_invoked == false
    and .model_invoked == false
    and .final_operator_acknowledgement_noop_confirmed == true
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_final_operator_acknowledgement | length) == 28
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_materialized == false
  and .packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_filesystem_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_received_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_confirmed_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_read_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_seen_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_final_response_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_completion_ack_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_status_ack_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_summary_ack_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_briefing_ack_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_readback_digest_ack_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_channel_ack_delivered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_external_ack_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_telegram_ack_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_recorded == false
  and .packet_acceptance_receipt_publication_completion_ack_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_persisted == false
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
  and .release_artifact_written == false
  and .public_artifact_written == false
  and .external_send_performed == false
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt final operator acknowledgement non-acceptance denial gate passed"
