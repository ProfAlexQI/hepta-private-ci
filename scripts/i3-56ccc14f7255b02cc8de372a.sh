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
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-summary-briefing-non-persistence-denial-gate" \
    scripts/i3-473ca3ed3d5ff6d0741bec38.sh
)"

source_summary_briefing_report_sha256="$(sha256_text "$SUMMARY_BRIEFING_JSON")"
final_acknowledgement_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-final-operator-acknowledgement-non-acceptance-denial:$source_summary_briefing_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
final_acknowledgement_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-final-operator-acknowledgement-non-acceptance:no-ack:no-received:no-confirmed:no-read:no-seen:no-final-response:no-status:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$SUMMARY_BRIEFING_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_non_persistence_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_non_persistence_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_denied_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_briefing_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_briefing_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_readback_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_status_banner_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_exported_summary_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_briefing_card_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_notification_timeline_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_summary_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_dashboard_narrative_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_audit_narrative_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_briefing_delivery_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_operator_memo_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_approval_summary_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_external_telegram_briefing_delivered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_summary_briefing_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_summary_release_publication_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_summary_activation_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_summary_install_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_summary_install_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_summary_active_binary_mutated_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_summary_external_send_count"
    ])
    and false_fields($source; [
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_summary_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_briefing_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_briefing_persisted",
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
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_surfaces | all(
      .artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_attempted == true
      and .operator_intent_consent_evidence_summary_briefing_noop_confirmed == true
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_non_acceptance_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_operator_intent == false
      and .records_operator_consent == false
      and .records_operator_identity == false
      and .records_operator_session == false
      and .records_intent_consent_evidence == false
      and .exports_evidence == false
      and .registers_query == false
      and .records_observability == false
      and .records_summary == false
      and .records_briefing == false
      and .records_acknowledgement == false
      and .derives_authority == false
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

final_acknowledgement_surfaces_json="$(
  jq -n '
    def ack_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_surface:$id,
        source_operator_intent_consent_evidence_summary_briefing_ready:true,
        artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_attempted:true,
        final_operator_acknowledgement_requested:false,
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
        completion_acknowledgement_recorded:false,
        status_acknowledgement_recorded:false,
        summary_acknowledgement_recorded:false,
        briefing_acknowledgement_recorded:false,
        readback_digest_acknowledgement_recorded:false,
        dashboard_acknowledgement_recorded:false,
        notification_acknowledgement_recorded:false,
        channel_acknowledgement_delivered:false,
        external_acknowledgement_sent:false,
        telegram_acknowledgement_sent:false,
        acknowledgement_acceptance_recorded:false,
        operator_approval_from_acknowledgement_derived:false,
        release_publication_authority_from_acknowledgement_derived:false,
        activation_authority_from_acknowledgement_derived:false,
        activation_command_from_acknowledgement_derived:false,
        activation_from_acknowledgement_allowed:false,
        live_execution_from_acknowledgement_allowed:false,
        download_link_from_acknowledgement_rendered:false,
        install_command_from_acknowledgement_rendered:false,
        install_from_acknowledgement_executed:false,
        service_restart_from_acknowledgement_performed:false,
        launchd_from_acknowledgement_mutated:false,
        active_binary_from_acknowledgement_mutated:false,
        result_receipt_from_acknowledgement_recorded:false,
        result_receipt_from_acknowledgement_persisted:false,
        operator_summary_from_acknowledgement_recorded:false,
        operator_briefing_from_acknowledgement_recorded:false,
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
        final_operator_acknowledgement_noop_confirmed:true,
        final_operator_acknowledgement_status:$status,
        reason:$reason
      } + $extra;
    [
      ack_surface("source_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_report_required"; "blocked_source_summary_briefing_required_noop"; "source_operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_report_required"; {source_operator_intent_consent_evidence_summary_briefing_report_required:true}),
      ack_surface("download_button_revocation_replay_final_operator_acknowledgement_claim"; "blocked_revocation_replay_final_ack_noop"; "download_button_revocation_replay_final_operator_acknowledgement_claim_denied"; {final_operator_acknowledgement_requested:true}),
      ack_surface("direct_download_url_logout_replay_operator_received_claim"; "blocked_logout_replay_operator_received_noop"; "direct_download_url_logout_replay_operator_received_claim_denied"; {operator_received_requested:true}),
      ack_surface("checksum_identity_reinstatement_operator_confirmed_claim"; "blocked_identity_reinstatement_operator_confirmed_noop"; "checksum_identity_reinstatement_operator_confirmed_claim_denied"; {operator_confirmed_requested:true}),
      ack_surface("package_manager_session_reinstatement_operator_read_claim"; "blocked_session_reinstatement_operator_read_noop"; "package_manager_session_reinstatement_operator_read_claim_denied"; {operator_read_requested:true}),
      ack_surface("curl_pipe_shell_revocation_replay_operator_seen_claim"; "blocked_revocation_replay_operator_seen_noop"; "curl_pipe_shell_revocation_replay_operator_seen_claim_denied"; {operator_seen_requested:true}),
      ack_surface("installer_device_session_reinstatement_final_response_claim"; "blocked_device_session_reinstatement_final_response_noop"; "installer_device_session_reinstatement_final_response_claim_denied"; {final_response_requested:true}),
      ack_surface("auto_update_session_logout_replay_completion_acknowledgement_claim"; "blocked_session_logout_replay_completion_ack_noop"; "auto_update_session_logout_replay_completion_acknowledgement_claim_denied"; {completion_acknowledgement_requested:true}),
      ack_surface("release_channel_identity_revocation_replay_status_acknowledgement_claim"; "blocked_identity_revocation_replay_status_ack_noop"; "release_channel_identity_revocation_replay_status_acknowledgement_claim_denied"; {status_acknowledgement_requested:true}),
      ack_surface("update_feed_session_reinstatement_summary_acknowledgement_claim"; "blocked_session_reinstatement_summary_ack_noop"; "update_feed_session_reinstatement_summary_acknowledgement_claim_denied"; {summary_acknowledgement_requested:true}),
      ack_surface("package_registry_identity_badge_briefing_acknowledgement_claim"; "blocked_identity_badge_briefing_ack_noop"; "package_registry_identity_badge_briefing_acknowledgement_claim_denied"; {briefing_acknowledgement_requested:true}),
      ack_surface("cdn_session_readback_logout_replay_digest_acknowledgement_claim"; "blocked_logout_replay_readback_ack_noop"; "cdn_session_readback_logout_replay_digest_acknowledgement_claim_denied"; {readback_digest_acknowledgement_requested:true}),
      ack_surface("sbom_identity_dashboard_reinstatement_notification_acknowledgement_claim"; "blocked_identity_reinstatement_dashboard_notification_ack_noop"; "sbom_identity_dashboard_reinstatement_notification_acknowledgement_claim_denied"; {dashboard_acknowledgement_requested:true, notification_acknowledgement_requested:true}),
      ack_surface("signature_channel_session_operator_memo_acknowledgement_claim"; "blocked_session_channel_ack_noop"; "signature_channel_session_operator_memo_acknowledgement_claim_denied"; {channel_acknowledgement_requested:true}),
      ack_surface("one_click_identity_approval_reinstatement_operator_approval_acknowledgement_claim"; "blocked_identity_approval_ack_noop"; "one_click_identity_approval_reinstatement_operator_approval_acknowledgement_claim_denied"; {operator_approval_acknowledgement_requested:true}),
      ack_surface("external_telegram_identity_session_reinstatement_external_telegram_acknowledgement_claim"; "blocked_external_telegram_ack_noop"; "external_telegram_identity_session_reinstatement_external_telegram_acknowledgement_claim_denied"; {external_acknowledgement_requested:true, telegram_acknowledgement_requested:true}),
      ack_surface("release_publication_authority_replay_reinstatement_acknowledgement_claim"; "blocked_authority_ack_noop"; "release_publication_authority_replay_reinstatement_acknowledgement_claim_denied"; {authority_acknowledgement_requested:true}),
      ack_surface("activation_live_install_restart_active_binary_reinstatement_acknowledgement_claim"; "blocked_live_ack_noop"; "activation_live_install_restart_active_binary_reinstatement_acknowledgement_claim_denied"; {live_acknowledgement_requested:true, install_restart_active_binary_acknowledgement_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_non_acceptance_denial_gate" \
    --arg source_summary_briefing_report_sha256 "$source_summary_briefing_report_sha256" \
    --arg final_acknowledgement_contract_hash_sha256 "$final_acknowledgement_contract_hash_sha256" \
    --arg final_acknowledgement_policy_hash_sha256 "$final_acknowledgement_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$SUMMARY_BRIEFING_JSON" \
    --argjson surfaces "$final_acknowledgement_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_non_acceptance_denial_v1",
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_mode:"denied_operator_intent_consent_evidence_summary_briefing_cannot_create_final_operator_acknowledgement_acceptance_or_authority",
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_gate:$source.gate,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_non_persistence_denial_ready,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_report_sha256:$source_summary_briefing_report_sha256,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_contract_hash_sha256:$final_acknowledgement_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_policy_hash_sha256:$final_acknowledgement_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_non_acceptance_denial_ready:true,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_surface_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_attempt_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_denied_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_briefing_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_briefing_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_readback_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_readback_recorded_count,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_surface_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_attempt_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_denied_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_surfaces:$surfaces,
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement:[
          "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_acceptance_denied",
          "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_recording_denied",
          "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_persistence_denied",
          "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_materialization_denied",
          "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_filesystem_write_denied",
          "artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_delivery_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_operator_received_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_operator_confirmed_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_operator_read_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_operator_seen_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_final_response_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_completion_acknowledgement_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_status_acknowledgement_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_summary_briefing_acknowledgement_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_readback_dashboard_notification_acknowledgement_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_channel_external_telegram_acknowledgement_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_acceptance_from_acknowledgement_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_acknowledgement_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_release_publication_authority_from_acknowledgement_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_activation_authority_from_acknowledgement_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_download_install_affordance_from_acknowledgement_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_install_restart_active_binary_from_acknowledgement_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_memory_provider_secret_external_send_from_acknowledgement_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_decision_status_promotion_denial_gate",
            status:"allowed_report_only_next_slice",
            records_terminal_decision:false,
            records_status_promotion:false,
            accepts_operator_acknowledgement:false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_received_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_confirmed_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_read_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_seen_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_response_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_summary_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_briefing_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_digest_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_dashboard_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_notification_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_channel_acknowledgement_delivered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_external_acknowledgement_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_telegram_acknowledgement_sent_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_acknowledgement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_external_send_count"
      ])
      + false_object([
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_received_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_confirmed_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_read_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_seen_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_response_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_acknowledgement_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_acknowledgement_recorded",
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
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed"
      ])
      + {
        side_effects:false_object([
          "final_operator_acknowledgement_accepted",
          "final_operator_acknowledgement_recorded",
          "final_operator_acknowledgement_persisted",
          "final_operator_acknowledgement_materialized",
          "final_operator_acknowledgement_filesystem_written",
          "final_operator_acknowledgement_delivered",
          "operator_received_recorded",
          "operator_confirmed_recorded",
          "operator_read_recorded",
          "operator_seen_recorded",
          "final_response_recorded",
          "completion_acknowledgement_recorded",
          "status_acknowledgement_recorded",
          "summary_acknowledgement_recorded",
          "briefing_acknowledgement_recorded",
          "readback_digest_acknowledgement_recorded",
          "dashboard_acknowledgement_recorded",
          "notification_acknowledgement_recorded",
          "channel_acknowledgement_delivered",
          "external_acknowledgement_sent",
          "telegram_acknowledgement_sent",
          "acceptance_from_acknowledgement_recorded",
          "operator_approval_from_acknowledgement_derived",
          "release_publication_authority_from_acknowledgement_derived",
          "activation_authority_from_acknowledgement_derived",
          "download_link_from_acknowledgement_rendered",
          "install_command_from_acknowledgement_rendered",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_non_acceptance_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_non_acceptance_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_summary_briefing_denied_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_attempt_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_denied_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_materialized_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_filesystem_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_delivered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_received_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_confirmed_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_read_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_seen_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_response_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_acknowledgement_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_acknowledgement_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_summary_acknowledgement_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_briefing_acknowledgement_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_digest_acknowledgement_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_dashboard_acknowledgement_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_notification_acknowledgement_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_channel_acknowledgement_delivered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_external_acknowledgement_sent_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_telegram_acknowledgement_sent_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_acknowledgement_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_release_publication_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_activation_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_install_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_service_restart_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_active_binary_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_final_acknowledgement_external_send_count"
  ])
  and false_fields($report; [
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
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_surfaces | all(
    .artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_attempted == true
    and .final_operator_acknowledgement_noop_confirmed == true
    and false_fields(.; [
      "final_operator_acknowledgement_accepted",
      "final_operator_acknowledgement_recorded",
      "final_operator_acknowledgement_persisted",
      "final_operator_acknowledgement_materialized",
      "final_operator_acknowledgement_filesystem_written",
      "final_operator_acknowledgement_delivered",
      "operator_received_recorded",
      "operator_confirmed_recorded",
      "operator_read_recorded",
      "operator_seen_recorded",
      "final_response_recorded",
      "completion_acknowledgement_recorded",
      "status_acknowledgement_recorded",
      "summary_acknowledgement_recorded",
      "briefing_acknowledgement_recorded",
      "readback_digest_acknowledgement_recorded",
      "dashboard_acknowledgement_recorded",
      "notification_acknowledgement_recorded",
      "channel_acknowledgement_delivered",
      "external_acknowledgement_sent",
      "telegram_acknowledgement_sent",
      "operator_approval_from_acknowledgement_derived",
      "release_publication_authority_from_acknowledgement_derived",
      "activation_authority_from_acknowledgement_derived",
      "install_from_acknowledgement_executed",
      "service_restart_from_acknowledgement_performed",
      "active_binary_from_acknowledgement_mutated",
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
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_surfaces[] | select(.final_operator_acknowledgement_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_surfaces[] | select(.telegram_acknowledgement_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_final_operator_acknowledgement_surfaces[] | select(.install_restart_active_binary_acknowledgement_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_terminal_decision_status_promotion_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_terminal_decision == false
    and .records_status_promotion == false
    and .accepts_operator_acknowledgement == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence final operator acknowledgement non-acceptance denial gate passed"
