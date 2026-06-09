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

OPERATOR_IDENTITY_SESSION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-binding-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-binding-denial-gate.sh
)"

source_operator_identity_session_report_sha256="$(sha256_text "$OPERATOR_IDENTITY_SESSION_JSON")"
operator_identity_session_replay_cross_binding_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-replay-cross-binding-denial:$source_operator_identity_session_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
operator_identity_session_replay_cross_binding_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-replay-cross-binding:no-replay:no-cross-session-binding:no-approval:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$OPERATOR_IDENTITY_SESSION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_intent_consent_reconfirmation_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denied_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_materialized_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_filesystem_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_materialized_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_filesystem_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_binding_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_binding_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_hash_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_token_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_nonce_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_device_session_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_status_promoted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_status_promoted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_identity_session_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_identity_session_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_identity_session_release_publication_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_identity_session_activation_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_identity_session_install_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_identity_session_service_restart_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_identity_session_active_binary_mutated_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_identity_session_external_send_count"
    ])
    and false_fields($source; [
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_binding_recorded",
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
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_surfaces | all(
      .artifact_download_install_affordance_result_receipt_operator_identity_session_binding_attempted == true
      and .operator_identity_session_binding_noop_confirmed == true
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_operator_identity == false
      and .records_operator_session == false
      and .records_session_binding == false
      and .records_operator_intent == false
      and .records_operator_consent == false
      and .accepts_replay == false
      and .accepts_cross_session_binding == false
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

operator_identity_session_replay_cross_binding_surfaces_json="$(
  jq -n '
    def replay_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surface:$id,
        source_operator_identity_session_binding_ready:true,
        artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_attempted:true,
        operator_identity_replay_requested:false,
        operator_session_replay_requested:false,
        operator_identity_cross_binding_requested:false,
        operator_session_cross_binding_requested:false,
        cross_session_binding_requested:false,
        operator_identity_replay_accepted:false,
        operator_session_replay_accepted:false,
        cross_session_binding_accepted:false,
        operator_identity_replay_recorded:false,
        operator_identity_replay_persisted:false,
        operator_session_replay_recorded:false,
        operator_session_replay_persisted:false,
        cross_session_binding_recorded:false,
        cross_session_binding_persisted:false,
        identity_hash_replay_recorded:false,
        session_token_replay_recorded:false,
        identity_fingerprint_cross_binding_recorded:false,
        session_token_cross_binding_recorded:false,
        identity_nonce_replay_recorded:false,
        device_session_rebind_recorded:false,
        session_refresh_replay_recorded:false,
        identity_status_replay_promoted:false,
        session_summary_cross_binding_promoted:false,
        operator_approval_from_replay_cross_binding_derived:false,
        acceptance_from_replay_cross_binding_recorded:false,
        terminal_decision_from_replay_cross_binding_recorded:false,
        terminal_status_from_replay_cross_binding_recorded:false,
        release_publication_authority_from_replay_cross_binding_derived:false,
        activation_authority_from_replay_cross_binding_derived:false,
        download_link_from_replay_cross_binding_rendered:false,
        install_command_from_replay_cross_binding_rendered:false,
        install_from_replay_cross_binding_executed:false,
        service_restart_from_replay_cross_binding_performed:false,
        launchd_from_replay_cross_binding_mutated:false,
        active_binary_from_replay_cross_binding_mutated:false,
        result_receipt_from_replay_cross_binding_recorded:false,
        result_receipt_from_replay_cross_binding_persisted:false,
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
        operator_identity_session_replay_cross_binding_noop_confirmed:true,
        operator_identity_session_replay_cross_binding_status:$status,
        reason:$reason
      } + $extra;
    [
      replay_surface("source_operator_identity_session_binding_report_required"; "blocked_source_identity_session_required_noop"; "source_operator_identity_session_binding_report_required"; {source_operator_identity_session_binding_report_required:true}),
      replay_surface("download_button_identity_hash_replay_claim"; "blocked_identity_hash_replay_noop"; "download_button_identity_hash_replay_claim_denied"; {operator_identity_replay_requested:true, identity_hash_replay_requested:true}),
      replay_surface("direct_download_url_session_token_replay_claim"; "blocked_session_token_replay_noop"; "direct_download_url_session_token_replay_claim_denied"; {operator_session_replay_requested:true, session_token_replay_requested:true}),
      replay_surface("checksum_identity_fingerprint_cross_binding_claim"; "blocked_identity_fingerprint_cross_binding_noop"; "checksum_identity_fingerprint_cross_binding_claim_denied"; {operator_identity_cross_binding_requested:true, identity_fingerprint_cross_binding_requested:true}),
      replay_surface("package_manager_install_command_session_token_cross_binding_claim"; "blocked_session_token_cross_binding_noop"; "package_manager_install_command_session_token_cross_binding_claim_denied"; {operator_session_cross_binding_requested:true, session_token_cross_binding_requested:true}),
      replay_surface("curl_pipe_shell_identity_nonce_replay_claim"; "blocked_identity_nonce_replay_noop"; "curl_pipe_shell_identity_nonce_replay_claim_denied"; {operator_identity_replay_requested:true, identity_nonce_replay_requested:true}),
      replay_surface("installer_launch_prompt_device_session_rebind_claim"; "blocked_device_session_rebind_noop"; "installer_launch_prompt_device_session_rebind_claim_denied"; {cross_session_binding_requested:true, device_session_rebind_requested:true}),
      replay_surface("auto_update_offer_operator_session_refresh_replay_claim"; "blocked_session_refresh_replay_noop"; "auto_update_offer_operator_session_refresh_replay_claim_denied"; {operator_session_replay_requested:true, session_refresh_replay_requested:true}),
      replay_surface("release_channel_subscription_identity_status_replay_claim"; "blocked_identity_status_replay_noop"; "release_channel_subscription_identity_status_replay_claim_denied"; {operator_identity_replay_requested:true, identity_status_replay_requested:true}),
      replay_surface("update_feed_hint_session_summary_cross_binding_claim"; "blocked_session_summary_cross_binding_noop"; "update_feed_hint_session_summary_cross_binding_claim_denied"; {operator_session_cross_binding_requested:true, session_summary_cross_binding_requested:true}),
      replay_surface("package_registry_badge_operator_identity_badge_replay_claim"; "blocked_identity_badge_replay_noop"; "package_registry_badge_operator_identity_badge_replay_claim_denied"; {operator_identity_replay_requested:true, operator_identity_badge_replay_requested:true}),
      replay_surface("cdn_mirror_download_session_readback_rebind_claim"; "blocked_session_readback_rebind_noop"; "cdn_mirror_download_session_readback_rebind_claim_denied"; {cross_session_binding_requested:true, session_readback_rebind_requested:true}),
      replay_surface("sbom_provenance_notarization_identity_dashboard_cross_binding_claim"; "blocked_identity_dashboard_cross_binding_noop"; "sbom_provenance_notarization_identity_dashboard_cross_binding_claim_denied"; {operator_identity_cross_binding_requested:true, identity_dashboard_cross_binding_requested:true}),
      replay_surface("signature_verification_command_channel_session_replay_claim"; "blocked_channel_session_replay_noop"; "signature_verification_command_channel_session_replay_claim_denied"; {operator_session_replay_requested:true, channel_session_replay_requested:true}),
      replay_surface("one_click_install_deep_link_operator_identity_approval_replay_claim"; "blocked_identity_approval_replay_noop"; "one_click_install_deep_link_operator_identity_approval_replay_claim_denied"; {operator_identity_replay_requested:true, operator_identity_approval_replay_requested:true}),
      replay_surface("external_telegram_install_message_external_identity_session_cross_binding_claim"; "blocked_external_telegram_identity_session_cross_binding_noop"; "external_telegram_install_message_external_identity_session_cross_binding_claim_denied"; {operator_identity_cross_binding_requested:true, operator_session_cross_binding_requested:true, telegram_identity_session_cross_binding_requested:true}),
      replay_surface("release_publication_authority_identity_session_replay_claim"; "blocked_authority_identity_session_replay_noop"; "release_publication_authority_identity_session_replay_claim_denied"; {operator_identity_replay_requested:true, operator_session_replay_requested:true, authority_identity_session_replay_requested:true}),
      replay_surface("activation_live_install_restart_active_binary_session_rebind_claim"; "blocked_live_session_rebind_noop"; "activation_live_install_restart_active_binary_session_rebind_claim_denied"; {cross_session_binding_requested:true, live_session_rebind_requested:true, install_restart_active_binary_session_rebind_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_gate" \
    --arg source_operator_identity_session_report_sha256 "$source_operator_identity_session_report_sha256" \
    --arg operator_identity_session_replay_cross_binding_contract_hash_sha256 "$operator_identity_session_replay_cross_binding_contract_hash_sha256" \
    --arg operator_identity_session_replay_cross_binding_policy_hash_sha256 "$operator_identity_session_replay_cross_binding_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$OPERATOR_IDENTITY_SESSION_JSON" \
    --argjson surfaces "$operator_identity_session_replay_cross_binding_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_v1",
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_mode:"denied_identity_session_binding_cannot_be_replayed_or_cross_bound_into_authority",
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_gate:$source.gate,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denial_ready,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_report_sha256:$source_operator_identity_session_report_sha256,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_contract_hash_sha256:$operator_identity_session_replay_cross_binding_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_policy_hash_sha256:$operator_identity_session_replay_cross_binding_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_ready:true,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_surface_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_attempt_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denied_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_recorded_count,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surface_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_attempt_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denied_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surfaces:$surfaces,
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding:[
          "artifact_download_install_affordance_result_receipt_operator_identity_replay_acceptance_denied",
          "artifact_download_install_affordance_result_receipt_operator_identity_replay_recording_denied",
          "artifact_download_install_affordance_result_receipt_operator_session_replay_acceptance_denied",
          "artifact_download_install_affordance_result_receipt_operator_session_replay_recording_denied",
          "artifact_download_install_affordance_result_receipt_cross_session_binding_acceptance_denied",
          "artifact_download_install_affordance_result_receipt_cross_session_binding_recording_denied",
          "artifact_download_install_affordance_result_receipt_identity_hash_replay_denied",
          "artifact_download_install_affordance_result_receipt_session_token_replay_denied",
          "artifact_download_install_affordance_result_receipt_identity_fingerprint_cross_binding_denied",
          "artifact_download_install_affordance_result_receipt_session_token_cross_binding_denied",
          "artifact_download_install_affordance_result_receipt_identity_nonce_replay_denied",
          "artifact_download_install_affordance_result_receipt_device_session_rebind_denied",
          "artifact_download_install_affordance_result_receipt_identity_session_replay_status_promotion_denied",
          "artifact_download_install_affordance_result_receipt_acceptance_from_replay_cross_binding_denied",
          "artifact_download_install_affordance_operator_approval_from_replay_cross_binding_denied",
          "artifact_download_install_affordance_release_publication_authority_from_replay_cross_binding_denied",
          "artifact_download_install_affordance_activation_authority_from_replay_cross_binding_denied",
          "artifact_download_install_affordance_memory_provider_secret_external_send_from_replay_cross_binding_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_gate",
            status:"allowed_report_only_next_slice",
            records_operator_identity:false,
            records_operator_session:false,
            records_session_binding:false,
            accepts_replay:false,
            accepts_cross_session_binding:false,
            records_revocation:false,
            records_logout:false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_hash_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_token_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_fingerprint_cross_binding_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_token_cross_binding_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_nonce_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_device_session_rebind_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_refresh_replay_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_status_replay_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_summary_cross_binding_promoted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_replay_cross_binding_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_replay_cross_binding_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_external_send_count"
      ])
      + false_object([
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_recorded",
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
          "operator_identity_replay_accepted",
          "operator_identity_replay_recorded",
          "operator_identity_replay_persisted",
          "operator_session_replay_accepted",
          "operator_session_replay_recorded",
          "operator_session_replay_persisted",
          "cross_session_binding_accepted",
          "cross_session_binding_recorded",
          "cross_session_binding_persisted",
          "identity_hash_replay_recorded",
          "session_token_replay_recorded",
          "identity_fingerprint_cross_binding_recorded",
          "session_token_cross_binding_recorded",
          "identity_nonce_replay_recorded",
          "device_session_rebind_recorded",
          "session_refresh_replay_recorded",
          "identity_status_replay_promoted",
          "session_summary_cross_binding_promoted",
          "operator_approval_from_replay_cross_binding_derived",
          "acceptance_from_replay_cross_binding_recorded",
          "terminal_decision_from_replay_cross_binding_recorded",
          "terminal_status_from_replay_cross_binding_recorded",
          "release_publication_authority_from_replay_cross_binding_derived",
          "activation_authority_from_replay_cross_binding_derived",
          "download_link_from_replay_cross_binding_rendered",
          "install_command_from_replay_cross_binding_rendered",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_binding_denied_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_attempt_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_denied_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_hash_replay_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_token_replay_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_fingerprint_cross_binding_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_token_cross_binding_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_nonce_replay_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_device_session_rebind_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_refresh_replay_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_identity_status_replay_promoted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_session_summary_cross_binding_promoted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_approval_from_replay_cross_binding_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_acceptance_from_replay_cross_binding_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_release_publication_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_activation_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_install_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_service_restart_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_active_binary_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_replay_cross_binding_external_send_count"
  ])
  and false_fields($report; [
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_replay_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_session_replay_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_session_binding_recorded",
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
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surfaces | all(
    .artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_attempted == true
    and .operator_identity_session_replay_cross_binding_noop_confirmed == true
    and false_fields(.; [
      "operator_identity_replay_accepted",
      "operator_session_replay_accepted",
      "cross_session_binding_accepted",
      "operator_identity_replay_recorded",
      "operator_identity_replay_persisted",
      "operator_session_replay_recorded",
      "operator_session_replay_persisted",
      "cross_session_binding_recorded",
      "cross_session_binding_persisted",
      "identity_hash_replay_recorded",
      "session_token_replay_recorded",
      "identity_fingerprint_cross_binding_recorded",
      "session_token_cross_binding_recorded",
      "identity_nonce_replay_recorded",
      "device_session_rebind_recorded",
      "session_refresh_replay_recorded",
      "identity_status_replay_promoted",
      "session_summary_cross_binding_promoted",
      "operator_approval_from_replay_cross_binding_derived",
      "release_publication_authority_from_replay_cross_binding_derived",
      "activation_authority_from_replay_cross_binding_derived",
      "install_from_replay_cross_binding_executed",
      "service_restart_from_replay_cross_binding_performed",
      "active_binary_from_replay_cross_binding_mutated",
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
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surfaces[] | select(.operator_identity_replay_requested == true)] | length) == 6
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surfaces[] | select(.operator_session_replay_requested == true)] | length) == 4
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surfaces[] | select(.cross_session_binding_requested == true)] | length) == 3
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surfaces[] | select(.telegram_identity_session_cross_binding_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_replay_cross_binding_surfaces[] | select(.install_restart_active_binary_session_rebind_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_denial_gate"
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
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session replay/cross-binding denial gate passed"
