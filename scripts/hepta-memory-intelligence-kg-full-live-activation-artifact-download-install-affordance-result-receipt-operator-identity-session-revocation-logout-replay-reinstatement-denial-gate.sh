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

OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-denial-gate.sh
)"

source_operator_identity_session_revocation_logout_report_sha256="$(sha256_text "$OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_JSON")"
operator_identity_session_revocation_logout_replay_reinstatement_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-denial:$source_operator_identity_session_revocation_logout_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
operator_identity_session_revocation_logout_replay_reinstatement_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement:no-replay:no-reinstatement:no-lifecycle:no-approval:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denied_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_revocation_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_revocation_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_logout_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_logout_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_invalidation_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_token_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_nonce_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_device_session_logout_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_revocation_refresh_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_revocation_status_promoted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_logout_summary_promoted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_revocation_logout_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_revocation_logout_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_release_publication_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_activation_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_install_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_service_restart_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_active_binary_mutated_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_external_send_count"
    ])
    and false_fields($source; [
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_lifecycle_recorded",
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
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_surfaces | all(
      .artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_attempted == true
      and .operator_identity_session_revocation_logout_noop_confirmed == true
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_operator_identity == false
      and .records_operator_session == false
      and .records_session_binding == false
      and .accepts_replay == false
      and .accepts_cross_session_binding == false
      and .records_revocation == false
      and .records_logout == false
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

operator_identity_session_revocation_logout_replay_reinstatement_surfaces_json="$(
  jq -n '
    def reinstatement_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surface:$id,
        source_operator_identity_session_revocation_logout_ready:true,
        artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_attempted:true,
        revocation_logout_replay_requested:false,
        identity_revocation_replay_requested:false,
        logout_replay_requested:false,
        session_logout_replay_requested:false,
        identity_reinstatement_requested:false,
        session_reinstatement_requested:false,
        session_lifecycle_mutation_requested:false,
        revocation_logout_replay_accepted:false,
        revocation_logout_replay_recorded:false,
        revocation_logout_replay_persisted:false,
        logout_replay_accepted:false,
        logout_replay_recorded:false,
        logout_replay_persisted:false,
        identity_reinstatement_recorded:false,
        identity_reinstatement_persisted:false,
        session_reinstatement_recorded:false,
        session_reinstatement_persisted:false,
        reinstatement_token_recorded:false,
        reinstatement_nonce_recorded:false,
        device_session_reinstatement_recorded:false,
        session_logout_replay_refresh_recorded:false,
        identity_revocation_replay_status_promoted:false,
        session_reinstatement_summary_promoted:false,
        operator_approval_from_revocation_logout_replay_reinstatement_derived:false,
        acceptance_from_revocation_logout_replay_reinstatement_recorded:false,
        terminal_decision_from_revocation_logout_replay_reinstatement_recorded:false,
        terminal_status_from_revocation_logout_replay_reinstatement_recorded:false,
        release_publication_authority_from_revocation_logout_replay_reinstatement_derived:false,
        activation_authority_from_revocation_logout_replay_reinstatement_derived:false,
        download_link_from_revocation_logout_replay_reinstatement_rendered:false,
        install_command_from_revocation_logout_replay_reinstatement_rendered:false,
        install_from_revocation_logout_replay_reinstatement_executed:false,
        service_restart_from_revocation_logout_replay_reinstatement_performed:false,
        launchd_from_revocation_logout_replay_reinstatement_mutated:false,
        active_binary_from_revocation_logout_replay_reinstatement_mutated:false,
        result_receipt_from_revocation_logout_replay_reinstatement_recorded:false,
        result_receipt_from_revocation_logout_replay_reinstatement_persisted:false,
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
        operator_identity_session_revocation_logout_replay_reinstatement_noop_confirmed:true,
        operator_identity_session_revocation_logout_replay_reinstatement_status:$status,
        reason:$reason
      } + $extra;
    [
      reinstatement_surface("source_operator_identity_session_revocation_logout_report_required"; "blocked_source_revocation_logout_required_noop"; "source_operator_identity_session_revocation_logout_report_required"; {source_operator_identity_session_revocation_logout_report_required:true}),
      reinstatement_surface("download_button_identity_revocation_replay_claim"; "blocked_identity_revocation_replay_noop"; "download_button_identity_revocation_replay_claim_denied"; {revocation_logout_replay_requested:true, identity_revocation_replay_requested:true, session_lifecycle_mutation_requested:true}),
      reinstatement_surface("direct_download_url_session_logout_replay_claim"; "blocked_session_logout_replay_noop"; "direct_download_url_session_logout_replay_claim_denied"; {logout_replay_requested:true, session_logout_replay_requested:true, session_lifecycle_mutation_requested:true}),
      reinstatement_surface("checksum_prompt_identity_reinstatement_claim"; "blocked_identity_reinstatement_noop"; "checksum_prompt_identity_reinstatement_claim_denied"; {identity_reinstatement_requested:true, session_lifecycle_mutation_requested:true}),
      reinstatement_surface("package_manager_install_command_session_reinstatement_token_claim"; "blocked_session_reinstatement_token_noop"; "package_manager_install_command_session_reinstatement_token_claim_denied"; {session_reinstatement_requested:true, reinstatement_token_requested:true}),
      reinstatement_surface("curl_pipe_shell_revocation_replay_nonce_claim"; "blocked_revocation_replay_nonce_noop"; "curl_pipe_shell_revocation_replay_nonce_claim_denied"; {revocation_logout_replay_requested:true, revocation_replay_nonce_requested:true}),
      reinstatement_surface("installer_launch_prompt_device_session_reinstatement_claim"; "blocked_device_session_reinstatement_noop"; "installer_launch_prompt_device_session_reinstatement_claim_denied"; {session_reinstatement_requested:true, device_session_reinstatement_requested:true}),
      reinstatement_surface("auto_update_offer_session_logout_replay_refresh_claim"; "blocked_session_logout_replay_refresh_noop"; "auto_update_offer_session_logout_replay_refresh_claim_denied"; {logout_replay_requested:true, session_logout_replay_refresh_requested:true}),
      reinstatement_surface("release_channel_subscription_identity_revocation_replay_status_claim"; "blocked_identity_revocation_replay_status_noop"; "release_channel_subscription_identity_revocation_replay_status_claim_denied"; {revocation_logout_replay_requested:true, identity_revocation_replay_status_requested:true}),
      reinstatement_surface("update_feed_hint_session_reinstatement_summary_claim"; "blocked_session_reinstatement_summary_noop"; "update_feed_hint_session_reinstatement_summary_claim_denied"; {session_reinstatement_requested:true, reinstatement_summary_requested:true}),
      reinstatement_surface("package_registry_badge_operator_identity_badge_reinstatement_claim"; "blocked_identity_badge_reinstatement_noop"; "package_registry_badge_operator_identity_badge_reinstatement_claim_denied"; {identity_reinstatement_requested:true, identity_badge_reinstatement_requested:true}),
      reinstatement_surface("cdn_mirror_download_session_readback_logout_replay_claim"; "blocked_session_readback_logout_replay_noop"; "cdn_mirror_download_session_readback_logout_replay_claim_denied"; {logout_replay_requested:true, session_readback_logout_replay_requested:true}),
      reinstatement_surface("sbom_provenance_notarization_identity_dashboard_reinstatement_claim"; "blocked_identity_dashboard_reinstatement_noop"; "sbom_provenance_notarization_identity_dashboard_reinstatement_claim_denied"; {identity_reinstatement_requested:true, identity_dashboard_reinstatement_requested:true}),
      reinstatement_surface("signature_verification_command_channel_session_reinstatement_claim"; "blocked_channel_session_reinstatement_noop"; "signature_verification_command_channel_session_reinstatement_claim_denied"; {session_reinstatement_requested:true, channel_session_reinstatement_requested:true}),
      reinstatement_surface("one_click_install_deep_link_operator_identity_approval_reinstatement_claim"; "blocked_identity_approval_reinstatement_noop"; "one_click_install_deep_link_operator_identity_approval_reinstatement_claim_denied"; {identity_reinstatement_requested:true, operator_identity_approval_reinstatement_requested:true}),
      reinstatement_surface("external_telegram_install_message_identity_session_reinstatement_claim"; "blocked_external_telegram_reinstatement_noop"; "external_telegram_install_message_identity_session_reinstatement_claim_denied"; {identity_reinstatement_requested:true, session_reinstatement_requested:true, telegram_identity_session_reinstatement_requested:true}),
      reinstatement_surface("release_publication_authority_revocation_logout_replay_reinstatement_claim"; "blocked_authority_revocation_logout_replay_reinstatement_noop"; "release_publication_authority_revocation_logout_replay_reinstatement_claim_denied"; {revocation_logout_replay_requested:true, session_reinstatement_requested:true, authority_revocation_logout_replay_reinstatement_requested:true}),
      reinstatement_surface("activation_live_install_restart_active_binary_session_reinstatement_claim"; "blocked_live_session_reinstatement_noop"; "activation_live_install_restart_active_binary_session_reinstatement_claim_denied"; {session_reinstatement_requested:true, live_session_reinstatement_requested:true, install_restart_active_binary_session_reinstatement_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_gate" \
    --arg source_operator_identity_session_revocation_logout_report_sha256 "$source_operator_identity_session_revocation_logout_report_sha256" \
    --arg operator_identity_session_revocation_logout_replay_reinstatement_contract_hash_sha256 "$operator_identity_session_revocation_logout_replay_reinstatement_contract_hash_sha256" \
    --arg operator_identity_session_revocation_logout_replay_reinstatement_policy_hash_sha256 "$operator_identity_session_revocation_logout_replay_reinstatement_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_JSON" \
    --argjson surfaces "$operator_identity_session_revocation_logout_replay_reinstatement_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_v1",
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_mode:"denied_revocation_logout_cannot_create_replay_reinstatement_or_session_lifecycle_authority",
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_gate:$source.gate,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_ready,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_report_sha256:$source_operator_identity_session_revocation_logout_report_sha256,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_contract_hash_sha256:$operator_identity_session_revocation_logout_replay_reinstatement_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_policy_hash_sha256:$operator_identity_session_revocation_logout_replay_reinstatement_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_ready:true,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_surface_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_attempt_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denied_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_revocation_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_revocation_recorded_count,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surface_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_attempt_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denied_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surfaces:$surfaces,
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement:[
          "artifact_download_install_affordance_result_receipt_revocation_replay_acceptance_denied",
          "artifact_download_install_affordance_result_receipt_revocation_replay_recording_denied",
          "artifact_download_install_affordance_result_receipt_logout_replay_acceptance_denied",
          "artifact_download_install_affordance_result_receipt_logout_replay_recording_denied",
          "artifact_download_install_affordance_result_receipt_identity_reinstatement_recording_denied",
          "artifact_download_install_affordance_result_receipt_session_reinstatement_recording_denied",
          "artifact_download_install_affordance_result_receipt_reinstatement_token_recording_denied",
          "artifact_download_install_affordance_result_receipt_reinstatement_nonce_recording_denied",
          "artifact_download_install_affordance_result_receipt_device_session_reinstatement_denied",
          "artifact_download_install_affordance_result_receipt_reinstatement_status_promotion_denied",
          "artifact_download_install_affordance_result_receipt_acceptance_from_revocation_logout_replay_reinstatement_denied",
          "artifact_download_install_affordance_operator_approval_from_revocation_logout_replay_reinstatement_denied",
          "artifact_download_install_affordance_release_publication_authority_from_revocation_logout_replay_reinstatement_denied",
          "artifact_download_install_affordance_activation_authority_from_revocation_logout_replay_reinstatement_denied",
          "artifact_download_install_affordance_download_install_from_revocation_logout_replay_reinstatement_denied",
          "artifact_download_install_affordance_memory_provider_secret_external_send_from_revocation_logout_replay_reinstatement_denied",
          "artifact_download_install_affordance_session_lifecycle_mutation_from_denied_receipt_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_gate",
            status:"allowed_report_only_next_slice",
            records_operator_identity:false,
            records_operator_session:false,
            records_session_binding:false,
            accepts_replay:false,
            accepts_cross_session_binding:false,
            records_revocation:false,
            records_logout:false,
            accepts_revocation_replay:false,
            records_reinstatement:false,
            accepts_reinstatement_replay:false,
            records_ordering:false,
            records_monotonicity:false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_replay_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_reinstatement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_reinstatement_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_reinstatement_token_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_reinstatement_nonce_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_device_session_reinstatement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_logout_replay_refresh_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_revocation_replay_status_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_summary_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_revocation_logout_replay_reinstatement_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_revocation_logout_replay_reinstatement_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_external_send_count"
      ])
      + false_object([
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_lifecycle_recorded",
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
          "revocation_logout_replay_accepted",
          "revocation_logout_replay_recorded",
          "revocation_logout_replay_persisted",
          "logout_replay_accepted",
          "logout_replay_recorded",
          "logout_replay_persisted",
          "identity_reinstatement_recorded",
          "identity_reinstatement_persisted",
          "session_reinstatement_recorded",
          "session_reinstatement_persisted",
          "reinstatement_token_recorded",
          "reinstatement_nonce_recorded",
          "device_session_reinstatement_recorded",
          "session_logout_replay_refresh_recorded",
          "identity_revocation_replay_status_promoted",
          "session_reinstatement_summary_promoted",
          "operator_approval_from_revocation_logout_replay_reinstatement_derived",
          "acceptance_from_revocation_logout_replay_reinstatement_recorded",
          "terminal_decision_from_revocation_logout_replay_reinstatement_recorded",
          "terminal_status_from_revocation_logout_replay_reinstatement_recorded",
          "release_publication_authority_from_revocation_logout_replay_reinstatement_derived",
          "activation_authority_from_revocation_logout_replay_reinstatement_derived",
          "download_link_from_revocation_logout_replay_reinstatement_rendered",
          "install_command_from_revocation_logout_replay_reinstatement_rendered",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denied_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_attempt_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_denied_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_revocation_logout_replay_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_replay_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_replay_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_logout_replay_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_reinstatement_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_reinstatement_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_reinstatement_token_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_reinstatement_nonce_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_device_session_reinstatement_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_logout_replay_refresh_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_revocation_replay_status_promoted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_reinstatement_summary_promoted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_revocation_logout_replay_reinstatement_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_revocation_logout_replay_reinstatement_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_release_publication_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_activation_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_install_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_service_restart_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_active_binary_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_revocation_logout_replay_reinstatement_external_send_count"
  ])
  and false_fields($report; [
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_revocation_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_logout_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_lifecycle_recorded",
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
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surfaces | all(
    .artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_attempted == true
    and .operator_identity_session_revocation_logout_replay_reinstatement_noop_confirmed == true
    and false_fields(.; [
      "revocation_logout_replay_accepted",
      "revocation_logout_replay_recorded",
      "revocation_logout_replay_persisted",
      "logout_replay_accepted",
      "logout_replay_recorded",
      "logout_replay_persisted",
      "identity_reinstatement_recorded",
      "identity_reinstatement_persisted",
      "session_reinstatement_recorded",
      "session_reinstatement_persisted",
      "reinstatement_token_recorded",
      "reinstatement_nonce_recorded",
      "device_session_reinstatement_recorded",
      "session_logout_replay_refresh_recorded",
      "identity_revocation_replay_status_promoted",
      "session_reinstatement_summary_promoted",
      "operator_approval_from_revocation_logout_replay_reinstatement_derived",
      "release_publication_authority_from_revocation_logout_replay_reinstatement_derived",
      "activation_authority_from_revocation_logout_replay_reinstatement_derived",
      "install_from_revocation_logout_replay_reinstatement_executed",
      "service_restart_from_revocation_logout_replay_reinstatement_performed",
      "active_binary_from_revocation_logout_replay_reinstatement_mutated",
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
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surfaces[] | select(.revocation_logout_replay_requested == true)] | length) == 4
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surfaces[] | select(.logout_replay_requested == true)] | length) == 3
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surfaces[] | select(.identity_reinstatement_requested == true)] | length) == 5
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surfaces[] | select(.session_reinstatement_requested == true)] | length) == 7
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surfaces[] | select(.telegram_identity_session_reinstatement_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_surfaces[] | select(.install_restart_active_binary_session_reinstatement_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_operator_identity == false
    and .records_operator_session == false
    and .records_session_binding == false
    and .accepts_replay == false
    and .accepts_cross_session_binding == false
    and .records_revocation == false
    and .records_logout == false
    and .accepts_revocation_replay == false
    and .records_reinstatement == false
    and .accepts_reinstatement_replay == false
    and .records_ordering == false
    and .records_monotonicity == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement denial gate passed"
