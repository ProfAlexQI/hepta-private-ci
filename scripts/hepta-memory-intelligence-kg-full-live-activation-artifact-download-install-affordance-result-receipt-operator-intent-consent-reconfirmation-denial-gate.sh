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

TERMINAL_DECISION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-terminal-decision-status-promotion-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-terminal-decision-status-promotion-denial-gate.sh
)"

source_terminal_decision_report_sha256="$(sha256_text "$TERMINAL_DECISION_JSON")"
operator_intent_consent_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-reconfirmation-denial:$source_terminal_decision_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
operator_intent_consent_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-intent-consent-reconfirmation:no-intent:no-consent:no-reconfirmation:no-approval:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$TERMINAL_DECISION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denied_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_materialized_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_filesystem_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_delivered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_status_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_status_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_status_materialized_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_status_filesystem_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_promotion_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_status_promoted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_acknowledgement_status_promoted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_status_promoted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_summary_status_promoted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_briefing_status_promoted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_channel_decision_delivered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_external_decision_sent_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_telegram_decision_sent_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_terminal_decision_release_publication_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_terminal_status_activation_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_terminal_status_install_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_terminal_status_service_restart_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_terminal_status_active_binary_mutated_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_terminal_status_external_send_count"
    ])
    and false_fields($source; [
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
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_surfaces | all(
      .artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_attempted == true
      and .terminal_decision_status_promotion_noop_confirmed == true
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_operator_intent == false
      and .records_operator_consent == false
      and .records_terminal_decision == false
      and .records_status_promotion == false
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

operator_intent_consent_surfaces_json="$(
  jq -n '
    def consent_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surface:$id,
        source_terminal_decision_status_promotion_ready:true,
        artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_attempted:true,
        operator_intent_reconfirmation_requested:false,
        operator_consent_reconfirmation_requested:false,
        operator_intent_reconfirmation_allowed:false,
        operator_consent_reconfirmation_allowed:false,
        operator_intent_reconfirmed:false,
        operator_consent_reconfirmed:false,
        operator_intent_recorded:false,
        operator_intent_persisted:false,
        operator_intent_materialized:false,
        operator_intent_filesystem_written:false,
        operator_consent_recorded:false,
        operator_consent_persisted:false,
        operator_consent_materialized:false,
        operator_consent_filesystem_written:false,
        consent_reconfirmation_recorded:false,
        consent_reconfirmation_persisted:false,
        intent_hash_recorded:false,
        consent_token_recorded:false,
        intent_nonce_recorded:false,
        double_confirm_recorded:false,
        explicit_intent_status_promoted:false,
        explicit_consent_status_promoted:false,
        operator_approval_from_intent_consent_derived:false,
        acceptance_from_intent_consent_recorded:false,
        terminal_decision_from_intent_consent_recorded:false,
        terminal_status_from_intent_consent_recorded:false,
        release_publication_authority_from_intent_consent_derived:false,
        activation_authority_from_intent_consent_derived:false,
        download_link_from_intent_consent_rendered:false,
        install_command_from_intent_consent_rendered:false,
        install_from_intent_consent_executed:false,
        service_restart_from_intent_consent_performed:false,
        launchd_from_intent_consent_mutated:false,
        active_binary_from_intent_consent_mutated:false,
        result_receipt_from_intent_consent_recorded:false,
        result_receipt_from_intent_consent_persisted:false,
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
        operator_intent_consent_reconfirmation_noop_confirmed:true,
        operator_intent_consent_reconfirmation_status:$status,
        reason:$reason
      } + $extra;
    [
      consent_surface("source_terminal_decision_status_promotion_report_required"; "blocked_source_terminal_status_required_noop"; "source_terminal_decision_status_promotion_report_required"; {source_terminal_decision_status_promotion_report_required:true}),
      consent_surface("download_button_operator_intent_reconfirmation_claim"; "blocked_operator_intent_noop"; "download_button_operator_intent_reconfirmation_claim_denied"; {operator_intent_reconfirmation_requested:true}),
      consent_surface("direct_download_url_operator_consent_reconfirmation_claim"; "blocked_operator_consent_noop"; "direct_download_url_operator_consent_reconfirmation_claim_denied"; {operator_consent_reconfirmation_requested:true}),
      consent_surface("checksum_prompt_explicit_intent_hash_claim"; "blocked_intent_hash_noop"; "checksum_prompt_explicit_intent_hash_claim_denied"; {intent_hash_requested:true}),
      consent_surface("package_manager_install_command_consent_token_claim"; "blocked_consent_token_noop"; "package_manager_install_command_consent_token_claim_denied"; {consent_token_requested:true}),
      consent_surface("curl_pipe_shell_intent_nonce_claim"; "blocked_intent_nonce_noop"; "curl_pipe_shell_intent_nonce_claim_denied"; {intent_nonce_requested:true}),
      consent_surface("installer_launch_prompt_double_confirm_claim"; "blocked_double_confirm_noop"; "installer_launch_prompt_double_confirm_claim_denied"; {double_confirm_requested:true}),
      consent_surface("auto_update_offer_consent_refresh_claim"; "blocked_consent_refresh_noop"; "auto_update_offer_consent_refresh_claim_denied"; {consent_refresh_requested:true}),
      consent_surface("release_channel_subscription_intent_status_claim"; "blocked_intent_status_noop"; "release_channel_subscription_intent_status_claim_denied"; {intent_status_requested:true}),
      consent_surface("update_feed_hint_consent_summary_claim"; "blocked_consent_summary_noop"; "update_feed_hint_consent_summary_claim_denied"; {consent_summary_requested:true}),
      consent_surface("package_registry_badge_operator_intent_badge_claim"; "blocked_intent_badge_noop"; "package_registry_badge_operator_intent_badge_claim_denied"; {operator_intent_badge_requested:true}),
      consent_surface("cdn_mirror_download_consent_readback_claim"; "blocked_consent_readback_noop"; "cdn_mirror_download_consent_readback_claim_denied"; {consent_readback_requested:true}),
      consent_surface("sbom_provenance_notarization_consent_dashboard_claim"; "blocked_consent_dashboard_noop"; "sbom_provenance_notarization_consent_dashboard_claim_denied"; {consent_dashboard_requested:true}),
      consent_surface("signature_verification_command_channel_consent_claim"; "blocked_channel_consent_noop"; "signature_verification_command_channel_consent_claim_denied"; {channel_consent_requested:true}),
      consent_surface("one_click_install_deep_link_operator_intent_approval_claim"; "blocked_intent_approval_noop"; "one_click_install_deep_link_operator_intent_approval_claim_denied"; {operator_intent_approval_requested:true}),
      consent_surface("external_telegram_install_message_external_consent_claim"; "blocked_external_telegram_consent_noop"; "external_telegram_install_message_external_consent_claim_denied"; {external_consent_requested:true, telegram_consent_requested:true}),
      consent_surface("release_publication_authority_intent_consent_claim"; "blocked_authority_intent_consent_noop"; "release_publication_authority_intent_consent_claim_denied"; {authority_intent_consent_requested:true}),
      consent_surface("activation_live_install_restart_active_binary_consent_claim"; "blocked_live_consent_noop"; "activation_live_install_restart_active_binary_consent_claim_denied"; {live_consent_requested:true, install_restart_active_binary_consent_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_gate" \
    --arg source_terminal_decision_report_sha256 "$source_terminal_decision_report_sha256" \
    --arg operator_intent_consent_contract_hash_sha256 "$operator_intent_consent_contract_hash_sha256" \
    --arg operator_intent_consent_policy_hash_sha256 "$operator_intent_consent_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$TERMINAL_DECISION_JSON" \
    --argjson surfaces "$operator_intent_consent_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_v1",
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_mode:"denied_terminal_status_cannot_create_operator_intent_consent_reconfirmation_or_authority",
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_gate:$source.gate,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_ready,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_report_sha256:$source_terminal_decision_report_sha256,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_contract_hash_sha256:$operator_intent_consent_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_policy_hash_sha256:$operator_intent_consent_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_ready:true,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_surface_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_attempt_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denied_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_promotion_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_promotion_recorded_count,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surface_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_attempt_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denied_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surfaces:$surfaces,
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation:[
          "artifact_download_install_affordance_result_receipt_operator_intent_reconfirmation_denied",
          "artifact_download_install_affordance_result_receipt_operator_consent_reconfirmation_denied",
          "artifact_download_install_affordance_result_receipt_operator_intent_recording_denied",
          "artifact_download_install_affordance_result_receipt_operator_intent_persistence_denied",
          "artifact_download_install_affordance_result_receipt_operator_consent_recording_denied",
          "artifact_download_install_affordance_result_receipt_operator_consent_persistence_denied",
          "artifact_download_install_affordance_result_receipt_consent_reconfirmation_recording_denied",
          "artifact_download_install_affordance_result_receipt_intent_hash_recording_denied",
          "artifact_download_install_affordance_result_receipt_consent_token_recording_denied",
          "artifact_download_install_affordance_result_receipt_intent_nonce_recording_denied",
          "artifact_download_install_affordance_result_receipt_double_confirm_recording_denied",
          "artifact_download_install_affordance_result_receipt_explicit_intent_consent_status_promotion_denied",
          "artifact_download_install_affordance_result_receipt_acceptance_from_intent_consent_denied",
          "artifact_download_install_affordance_operator_approval_from_intent_consent_denied",
          "artifact_download_install_affordance_release_publication_authority_from_intent_consent_denied",
          "artifact_download_install_affordance_activation_authority_from_intent_consent_denied",
          "artifact_download_install_affordance_download_install_from_intent_consent_denied",
          "artifact_download_install_affordance_install_restart_active_binary_from_intent_consent_denied",
          "artifact_download_install_affordance_memory_provider_secret_external_send_from_intent_consent_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_gate",
            status:"allowed_report_only_next_slice",
            records_operator_identity:false,
            records_operator_session:false,
            records_operator_intent:false,
            records_operator_consent:false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_reconfirmed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_reconfirmed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_consent_reconfirmation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_consent_reconfirmation_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_intent_hash_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_consent_token_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_intent_nonce_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_double_confirm_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_explicit_intent_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_explicit_consent_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_intent_consent_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_intent_consent_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_external_send_count"
      ])
      + false_object([
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_consent_reconfirmation_recorded",
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
          "operator_intent_reconfirmed",
          "operator_consent_reconfirmed",
          "operator_intent_recorded",
          "operator_intent_persisted",
          "operator_intent_materialized",
          "operator_intent_filesystem_written",
          "operator_consent_recorded",
          "operator_consent_persisted",
          "operator_consent_materialized",
          "operator_consent_filesystem_written",
          "consent_reconfirmation_recorded",
          "consent_reconfirmation_persisted",
          "intent_hash_recorded",
          "consent_token_recorded",
          "intent_nonce_recorded",
          "double_confirm_recorded",
          "explicit_intent_status_promoted",
          "explicit_consent_status_promoted",
          "operator_approval_from_intent_consent_derived",
          "acceptance_from_intent_consent_recorded",
          "terminal_decision_from_intent_consent_recorded",
          "terminal_status_from_intent_consent_recorded",
          "release_publication_authority_from_intent_consent_derived",
          "activation_authority_from_intent_consent_derived",
          "download_link_from_intent_consent_rendered",
          "install_command_from_intent_consent_rendered",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denied_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_attempt_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_denied_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_reconfirmed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_reconfirmed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_consent_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_consent_reconfirmation_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_consent_reconfirmation_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_intent_hash_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_consent_token_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_intent_nonce_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_double_confirm_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_explicit_intent_status_promoted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_explicit_consent_status_promoted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_intent_consent_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_intent_consent_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_release_publication_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_activation_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_install_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_service_restart_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_active_binary_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_intent_consent_external_send_count"
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
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surfaces | all(
    .artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_attempted == true
    and .operator_intent_consent_reconfirmation_noop_confirmed == true
    and false_fields(.; [
      "operator_intent_reconfirmed",
      "operator_consent_reconfirmed",
      "operator_intent_recorded",
      "operator_intent_persisted",
      "operator_consent_recorded",
      "operator_consent_persisted",
      "consent_reconfirmation_recorded",
      "consent_reconfirmation_persisted",
      "intent_hash_recorded",
      "consent_token_recorded",
      "intent_nonce_recorded",
      "double_confirm_recorded",
      "explicit_intent_status_promoted",
      "explicit_consent_status_promoted",
      "operator_approval_from_intent_consent_derived",
      "release_publication_authority_from_intent_consent_derived",
      "activation_authority_from_intent_consent_derived",
      "install_from_intent_consent_executed",
      "service_restart_from_intent_consent_performed",
      "active_binary_from_intent_consent_mutated",
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
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surfaces[] | select(.operator_intent_reconfirmation_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surfaces[] | select(.operator_consent_reconfirmation_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surfaces[] | select(.telegram_consent_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_surfaces[] | select(.install_restart_active_binary_consent_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_operator_identity == false
    and .records_operator_session == false
    and .records_operator_intent == false
    and .records_operator_consent == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator intent/consent reconfirmation denial gate passed"
