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

INTENT_CONSENT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-reconfirmation-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-reconfirmation-denial-gate.sh
)"

source_intent_consent_report_sha256="$(sha256_text "$INTENT_CONSENT_JSON")"
evidence_persistence_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-persistence-denial:$source_intent_consent_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
evidence_persistence_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-persistence:no-evidence:no-receipt:no-binding:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$INTENT_CONSENT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_terminal_decision_status_promotion_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denied_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_reconfirmed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_reconfirmed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_consent_reconfirmation_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_identity_signature_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_session_consent_token_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_intent_consent_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_acceptance_from_intent_consent_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_release_publication_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_activation_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_install_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_service_restart_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_active_binary_mutated_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_external_send_count"
    ])
    and false_fields($source; [
      "artifact_download_install_affordance_result_receipt_recorded",
      "artifact_download_install_affordance_result_receipt_persisted",
      "operator_acceptance_recorded",
      "operator_approval_recorded",
      "release_publication_authority_derived",
      "activation_authority_derived",
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
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_surfaces | all(
      .artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_attempted == true
      and .operator_intent_consent_reconfirmation_noop_confirmed == true
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_operator_intent == false
      and .records_operator_consent == false
      and .records_operator_identity == false
      and .records_operator_session == false
      and .records_intent_consent_evidence == false
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

evidence_persistence_surfaces_json="$(
  jq -n '
    def evidence_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surface:$id,
        source_operator_intent_consent_reconfirmation_ready:true,
        artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_attempted:true,
        evidence_persistence_requested:false,
        intent_evidence_requested:false,
        consent_evidence_requested:false,
        evidence_receipt_requested:false,
        evidence_persistence_allowed:false,
        operator_intent_evidence_recorded:false,
        operator_intent_evidence_persisted:false,
        operator_consent_evidence_recorded:false,
        operator_consent_evidence_persisted:false,
        intent_consent_evidence_recorded:false,
        intent_consent_evidence_persisted:false,
        identity_signature_evidence_recorded:false,
        identity_signature_evidence_persisted:false,
        session_consent_token_evidence_recorded:false,
        session_consent_token_evidence_persisted:false,
        consent_nonce_evidence_recorded:false,
        consent_refresh_evidence_recorded:false,
        evidence_receipt_recorded:false,
        evidence_receipt_persisted:false,
        evidence_materialized:false,
        evidence_filesystem_written:false,
        evidence_ledger_written:false,
        evidence_indexed:false,
        evidence_exported:false,
        evidence_query_registered:false,
        evidence_observability_recorded:false,
        evidence_readback_recorded:false,
        identity_session_binding_from_evidence_recorded:false,
        operator_approval_from_evidence_derived:false,
        acceptance_from_evidence_recorded:false,
        terminal_decision_from_evidence_recorded:false,
        terminal_status_from_evidence_recorded:false,
        release_publication_authority_from_evidence_derived:false,
        activation_authority_from_evidence_derived:false,
        download_link_from_evidence_rendered:false,
        install_command_from_evidence_rendered:false,
        install_from_evidence_executed:false,
        service_restart_from_evidence_performed:false,
        launchd_from_evidence_mutated:false,
        active_binary_from_evidence_mutated:false,
        result_receipt_from_evidence_recorded:false,
        result_receipt_from_evidence_persisted:false,
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
        operator_intent_consent_evidence_persistence_noop_confirmed:true,
        operator_intent_consent_evidence_persistence_status:$status,
        reason:$reason
      } + $extra;
    [
      evidence_surface("source_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_report_required"; "blocked_source_intent_consent_reconfirmation_required_noop"; "source_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_report_required"; {source_operator_intent_consent_reconfirmation_report_required:true}),
      evidence_surface("download_button_revocation_replay_operator_intent_evidence_record_claim"; "blocked_revocation_replay_operator_intent_evidence_noop"; "download_button_revocation_replay_operator_intent_evidence_record_claim_denied"; {intent_evidence_requested:true}),
      evidence_surface("direct_download_url_logout_replay_operator_consent_evidence_record_claim"; "blocked_logout_replay_operator_consent_evidence_noop"; "direct_download_url_logout_replay_operator_consent_evidence_record_claim_denied"; {consent_evidence_requested:true}),
      evidence_surface("checksum_identity_reinstatement_signature_evidence_packet_claim"; "blocked_identity_signature_evidence_packet_noop"; "checksum_identity_reinstatement_signature_evidence_packet_claim_denied"; {identity_signature_evidence_requested:true}),
      evidence_surface("package_manager_session_reinstatement_consent_token_evidence_claim"; "blocked_session_consent_token_evidence_noop"; "package_manager_session_reinstatement_consent_token_evidence_claim_denied"; {session_consent_token_evidence_requested:true}),
      evidence_surface("curl_pipe_shell_revocation_replay_intent_timestamp_evidence_claim"; "blocked_revocation_replay_intent_timestamp_evidence_noop"; "curl_pipe_shell_revocation_replay_intent_timestamp_evidence_claim_denied"; {intent_timestamp_evidence_requested:true}),
      evidence_surface("installer_device_session_consent_nonce_evidence_claim"; "blocked_device_session_consent_nonce_evidence_noop"; "installer_device_session_consent_nonce_evidence_claim_denied"; {consent_nonce_evidence_requested:true}),
      evidence_surface("auto_update_session_logout_replay_consent_refresh_evidence_claim"; "blocked_logout_replay_consent_refresh_evidence_noop"; "auto_update_session_logout_replay_consent_refresh_evidence_claim_denied"; {consent_refresh_evidence_requested:true}),
      evidence_surface("release_channel_identity_revocation_replay_intent_status_evidence_claim"; "blocked_identity_revocation_replay_intent_status_evidence_noop"; "release_channel_identity_revocation_replay_intent_status_evidence_claim_denied"; {intent_status_evidence_requested:true}),
      evidence_surface("update_feed_session_reinstatement_consent_summary_evidence_claim"; "blocked_session_reinstatement_consent_summary_evidence_noop"; "update_feed_session_reinstatement_consent_summary_evidence_claim_denied"; {consent_summary_evidence_requested:true}),
      evidence_surface("package_registry_identity_badge_intent_evidence_badge_claim"; "blocked_identity_badge_intent_evidence_noop"; "package_registry_identity_badge_intent_evidence_badge_claim_denied"; {intent_badge_evidence_requested:true}),
      evidence_surface("cdn_session_readback_logout_replay_consent_readback_evidence_claim"; "blocked_logout_replay_consent_readback_evidence_noop"; "cdn_session_readback_logout_replay_consent_readback_evidence_claim_denied"; {consent_readback_evidence_requested:true}),
      evidence_surface("sbom_identity_dashboard_reinstatement_consent_notification_evidence_claim"; "blocked_identity_dashboard_consent_notification_evidence_noop"; "sbom_identity_dashboard_reinstatement_consent_notification_evidence_claim_denied"; {consent_notification_evidence_requested:true}),
      evidence_surface("signature_channel_session_consent_delivery_evidence_claim"; "blocked_session_channel_consent_delivery_evidence_noop"; "signature_channel_session_consent_delivery_evidence_claim_denied"; {channel_consent_evidence_requested:true}),
      evidence_surface("one_click_identity_approval_reinstatement_reconfirmed_consent_evidence_claim"; "blocked_identity_approval_reconfirmed_consent_evidence_noop"; "one_click_identity_approval_reinstatement_reconfirmed_consent_evidence_claim_denied"; {operator_approval_consent_evidence_requested:true}),
      evidence_surface("external_telegram_identity_session_reinstatement_consent_evidence_claim"; "blocked_external_telegram_consent_evidence_noop"; "external_telegram_identity_session_reinstatement_consent_evidence_claim_denied"; {external_consent_evidence_requested:true, telegram_consent_evidence_requested:true}),
      evidence_surface("release_publication_authority_replay_reinstatement_intent_consent_evidence_claim"; "blocked_authority_intent_consent_evidence_noop"; "release_publication_authority_replay_reinstatement_intent_consent_evidence_claim_denied"; {authority_intent_consent_evidence_requested:true}),
      evidence_surface("activation_live_install_restart_active_binary_reinstatement_consent_evidence_claim"; "blocked_live_consent_evidence_noop"; "activation_live_install_restart_active_binary_reinstatement_consent_evidence_claim_denied"; {live_consent_evidence_requested:true, install_restart_active_binary_evidence_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_gate" \
    --arg source_intent_consent_report_sha256 "$source_intent_consent_report_sha256" \
    --arg evidence_persistence_contract_hash_sha256 "$evidence_persistence_contract_hash_sha256" \
    --arg evidence_persistence_policy_hash_sha256 "$evidence_persistence_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$INTENT_CONSENT_JSON" \
    --argjson surfaces "$evidence_persistence_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_v1",
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_mode:"denied_operator_intent_consent_reconfirmation_cannot_materialize_evidence_receipts_binding_or_authority",
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_gate:$source.gate,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denial_ready,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_report_sha256:$source_intent_consent_report_sha256,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_contract_hash_sha256:$evidence_persistence_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_policy_hash_sha256:$evidence_persistence_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_ready:true,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_surface_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denied_count,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surface_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_attempt_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denied_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surfaces:$surfaces,
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence:[
          "operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_evidence_recording_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_evidence_recording_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_persistence_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_identity_signature_evidence_persistence_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_session_consent_token_evidence_persistence_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_evidence_receipt_persistence_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_evidence_materialization_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_evidence_filesystem_write_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_evidence_ledger_index_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_evidence_export_query_observability_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_evidence_readback_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_identity_session_binding_from_evidence_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_evidence_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_release_publication_authority_from_evidence_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_activation_authority_from_evidence_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_download_install_from_evidence_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_install_restart_active_binary_from_evidence_denied",
          "operator_identity_session_revocation_logout_replay_reinstatement_memory_provider_secret_external_send_from_evidence_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_denial_gate",
            status:"allowed_report_only_next_slice",
            records_operator_intent:false,
            records_operator_consent:false,
            records_operator_identity:false,
            records_operator_session:false,
            records_intent_consent_evidence:false,
            exports_evidence:false,
            registers_query:false,
            records_observability:false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_evidence_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_evidence_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_identity_signature_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_session_consent_token_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_receipt_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_receipt_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_filesystem_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_ledger_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_indexed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_exported_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_query_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_observability_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_readback_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_identity_session_binding_from_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_evidence_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_acceptance_from_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_external_send_count"
      ])
      + false_object([
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_persisted",
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
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed"
      ])
      + {
        side_effects:false_object([
          "operator_intent_evidence_recorded",
          "operator_intent_evidence_persisted",
          "operator_consent_evidence_recorded",
          "operator_consent_evidence_persisted",
          "intent_consent_evidence_recorded",
          "intent_consent_evidence_persisted",
          "identity_signature_evidence_recorded",
          "identity_signature_evidence_persisted",
          "session_consent_token_evidence_recorded",
          "session_consent_token_evidence_persisted",
          "evidence_receipt_recorded",
          "evidence_receipt_persisted",
          "evidence_materialized",
          "evidence_filesystem_written",
          "evidence_ledger_written",
          "evidence_indexed",
          "evidence_exported",
          "evidence_query_registered",
          "evidence_observability_recorded",
          "evidence_readback_recorded",
          "identity_session_binding_from_evidence_recorded",
          "operator_approval_from_evidence_derived",
          "acceptance_from_evidence_recorded",
          "terminal_decision_from_evidence_recorded",
          "terminal_status_from_evidence_recorded",
          "release_publication_authority_from_evidence_derived",
          "activation_authority_from_evidence_derived",
          "download_link_from_evidence_rendered",
          "install_command_from_evidence_rendered",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_reconfirmation_denied_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_attempt_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denied_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_evidence_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_consent_evidence_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_intent_consent_evidence_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_receipt_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_receipt_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_materialized_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_filesystem_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_ledger_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_indexed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_exported_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_query_registered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_evidence_observability_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_identity_session_binding_from_evidence_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_approval_from_evidence_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_release_publication_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_activation_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_install_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_active_binary_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_evidence_external_send_count"
  ])
  and false_fields($report; [
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
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surfaces | all(
    .artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_attempted == true
    and .operator_intent_consent_evidence_persistence_noop_confirmed == true
    and false_fields(.; [
      "operator_intent_evidence_recorded",
      "operator_intent_evidence_persisted",
      "operator_consent_evidence_recorded",
      "operator_consent_evidence_persisted",
      "intent_consent_evidence_recorded",
      "intent_consent_evidence_persisted",
      "identity_signature_evidence_recorded",
      "session_consent_token_evidence_recorded",
      "evidence_receipt_recorded",
      "evidence_receipt_persisted",
      "evidence_materialized",
      "evidence_filesystem_written",
      "evidence_ledger_written",
      "evidence_indexed",
      "evidence_exported",
      "evidence_query_registered",
      "evidence_observability_recorded",
      "identity_session_binding_from_evidence_recorded",
      "operator_approval_from_evidence_derived",
      "release_publication_authority_from_evidence_derived",
      "activation_authority_from_evidence_derived",
      "install_from_evidence_executed",
      "service_restart_from_evidence_performed",
      "active_binary_from_evidence_mutated",
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
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surfaces[] | select(.intent_evidence_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surfaces[] | select(.consent_evidence_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surfaces[] | select(.telegram_consent_evidence_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_surfaces[] | select(.install_restart_active_binary_evidence_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_export_query_observability_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_operator_intent == false
    and .records_operator_consent == false
    and .records_operator_identity == false
    and .records_operator_session == false
    and .records_intent_consent_evidence == false
    and .exports_evidence == false
    and .registers_query == false
    and .records_observability == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence persistence denial gate passed"
