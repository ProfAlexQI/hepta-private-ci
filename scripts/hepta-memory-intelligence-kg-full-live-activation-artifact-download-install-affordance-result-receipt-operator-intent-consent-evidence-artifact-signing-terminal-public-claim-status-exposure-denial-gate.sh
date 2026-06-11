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

TERMINAL_STATUS_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-status-promotion-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-status-promotion-denial-gate.sh
)"

source_terminal_status_report_sha256="$(sha256_text "$TERMINAL_STATUS_JSON")"
terminal_public_claim_status_exposure_contract_hash_sha256="$(
  sha256_text "hepta-artifact-distribution-signing-notarization-receipt-terminal-public-claim-status-exposure-denial:$source_terminal_status_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
terminal_public_claim_status_exposure_policy_hash_sha256="$(
  sha256_text "artifact-distribution-signing-notarization-receipt-terminal-public-claim-status-exposure:no-public-claim:no-status-exposure:no-release:no-channel:no-telegram:no-install"
)"

jq -n -e \
  --argjson source "$TERMINAL_STATUS_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denial_ready == true
    and $source.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_surface_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_attempt_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denied_count == 18
    and zero_fields($source; [
      "artifact_distribution_signing_notarization_receipt_terminal_decision_recorded_count",
      "artifact_distribution_signing_notarization_receipt_terminal_decision_persisted_count",
      "artifact_distribution_signing_notarization_receipt_terminal_status_recorded_count",
      "artifact_distribution_signing_notarization_receipt_terminal_status_persisted_count",
      "artifact_distribution_signing_notarization_receipt_status_promotion_recorded_count",
      "artifact_distribution_signing_notarization_receipt_channel_decision_delivered_count",
      "artifact_distribution_signing_notarization_receipt_external_decision_sent_count",
      "artifact_distribution_signing_notarization_receipt_telegram_decision_sent_count",
      "operator_approval_from_signing_receipt_terminal_status_derived_count",
      "release_publication_authority_from_signing_receipt_terminal_decision_derived_count",
      "activation_authority_from_signing_receipt_terminal_status_derived_count",
      "install_from_signing_receipt_terminal_status_executed_count",
      "service_restart_from_signing_receipt_terminal_status_performed_count",
      "active_binary_from_signing_receipt_terminal_status_mutated_count",
      "memory_store_write_performed_count",
      "live_kg_write_performed_count",
      "provider_invoked_count",
      "model_invoked_count",
      "credential_read_count",
      "secret_file_read_count",
      "external_send_performed_count"
    ])
    and false_fields($source; [
      "artifact_distribution_signing_notarization_receipt_terminal_decision_recorded",
      "artifact_distribution_signing_notarization_receipt_terminal_status_recorded",
      "artifact_distribution_signing_notarization_receipt_status_promotion_recorded",
      "operator_approval_recorded",
      "release_publication_authority_derived",
      "activation_authority_derived",
      "activation_allowed",
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
    and ($source.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_surfaces | length) == 18
    and ($source.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_surfaces | all(
      .artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_attempted == true
      and .artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_noop_confirmed == true
      and .terminal_decision_allowed == false
      and .terminal_status_allowed == false
      and .status_promotion_allowed == false
      and .terminal_decision_recorded == false
      and .terminal_status_recorded == false
      and .status_promotion_recorded == false
      and .release_publication_authority_from_terminal_decision_derived == false
      and .activation_authority_from_terminal_status_derived == false
      and .install_from_terminal_status_executed == false
      and .service_restart_from_terminal_status_performed == false
      and .active_binary_from_terminal_status_mutated == false
      and .memory_store_write_performed == false
      and .live_kg_write_performed == false
      and .external_send_performed == false
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_terminal_decision == false
      and .records_status_promotion == false
      and .records_public_claim == false
      and .records_status_exposure == false
      and .derives_release_publication_authority == false
      and .derives_activation_authority == false
      and .installs_or_restarts == false
      and .mutates_active_binary == false
      and .mutates_memory_store == false
      and .writes_kg == false
      and .invokes_provider == false
      and .reads_credentials == false
      and .sends_externally == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

terminal_public_claim_status_exposure_surfaces_json="$(
  jq -n '
    def exposure_surface($id; $status; $reason; $extra):
      {
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface:$id,
        source_artifact_distribution_signing_notarization_receipt_terminal_status_ready:true,
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_attempted:true,
        public_claim_requested:false,
        status_exposure_requested:false,
        public_release_claim_requested:false,
        public_status_exposure_requested:false,
        telegram_status_exposure_requested:false,
        release_publication_status_exposure_requested:false,
        install_restart_active_binary_status_exposure_requested:false,
        public_claim_allowed:false,
        status_exposure_allowed:false,
        public_release_claim_allowed:false,
        public_status_exposure_allowed:false,
        public_claim_recorded:false,
        public_claim_persisted:false,
        status_exposure_recorded:false,
        status_exposure_persisted:false,
        channel_status_exposure_delivered:false,
        external_status_exposure_sent:false,
        telegram_status_exposure_sent:false,
        release_artifact_written:false,
        public_artifact_written:false,
        operator_approval_from_public_claim_derived:false,
        release_publication_authority_from_public_claim_derived:false,
        activation_authority_from_status_exposure_derived:false,
        download_link_from_status_exposure_rendered:false,
        install_command_from_status_exposure_emitted:false,
        install_from_status_exposure_executed:false,
        service_restart_from_status_exposure_performed:false,
        active_binary_from_status_exposure_mutated:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        external_send_performed:false,
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_noop_confirmed:true,
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_status:$status,
        reason:$reason
      } + $extra;
    [
      exposure_surface("source_terminal_decision_status_promotion_report_required"; "blocked_source_terminal_status_report_required_noop"; "source_terminal_decision_status_promotion_report_required"; {source_terminal_status_report_required:true}),
      exposure_surface("artifact_signing_receipt_public_claim_attempt"; "blocked_artifact_signing_receipt_public_claim_noop"; "artifact_signing_receipt_public_claim_attempt_denied"; {public_claim_requested:true}),
      exposure_surface("package_signing_receipt_public_status_badge_exposure"; "blocked_package_signing_receipt_public_status_badge_noop"; "package_signing_receipt_public_status_badge_exposure_denied"; {status_exposure_requested:true, public_status_exposure_requested:true}),
      exposure_surface("signature_manifest_public_status_page_exposure"; "blocked_signature_manifest_public_status_page_noop"; "signature_manifest_public_status_page_exposure_denied"; {status_exposure_requested:true, public_status_exposure_requested:true}),
      exposure_surface("notarization_status_public_claim_attempt"; "blocked_notarization_status_public_claim_noop"; "notarization_status_public_claim_attempt_denied"; {public_claim_requested:true}),
      exposure_surface("witness_notary_exported_summary_public_status_exposure"; "blocked_witness_notary_public_status_noop"; "witness_notary_exported_summary_public_status_exposure_denied"; {status_exposure_requested:true, public_status_exposure_requested:true}),
      exposure_surface("tombstone_garbage_collection_final_response_public_claim_attempt"; "blocked_tombstone_gc_final_response_public_claim_noop"; "tombstone_garbage_collection_final_response_public_claim_attempt_denied"; {public_claim_requested:true}),
      exposure_surface("replacement_garbage_collection_completion_public_status_exposure"; "blocked_replacement_gc_completion_public_status_noop"; "replacement_garbage_collection_completion_public_status_exposure_denied"; {status_exposure_requested:true, public_status_exposure_requested:true}),
      exposure_surface("provenance_dashboard_public_status_exposure"; "blocked_provenance_dashboard_public_status_noop"; "provenance_dashboard_public_status_exposure_denied"; {status_exposure_requested:true, public_status_exposure_requested:true}),
      exposure_surface("sbom_audit_public_claim_attempt"; "blocked_sbom_audit_public_claim_noop"; "sbom_audit_public_claim_attempt_denied"; {public_claim_requested:true}),
      exposure_surface("release_asset_public_briefing_exposure"; "blocked_release_asset_public_briefing_noop"; "release_asset_public_briefing_exposure_denied"; {status_exposure_requested:true, public_status_exposure_requested:true}),
      exposure_surface("cdn_dashboard_public_readback_exposure"; "blocked_cdn_dashboard_public_readback_noop"; "cdn_dashboard_public_readback_exposure_denied"; {status_exposure_requested:true, public_status_exposure_requested:true}),
      exposure_surface("package_registry_public_memo_notification_exposure"; "blocked_package_registry_public_notification_noop"; "package_registry_public_memo_notification_exposure_denied"; {status_exposure_requested:true, public_status_exposure_requested:true}),
      exposure_surface("dashboard_hash_public_approval_channel_exposure"; "blocked_dashboard_hash_public_channel_exposure_noop"; "dashboard_hash_public_approval_channel_exposure_denied"; {status_exposure_requested:true, public_status_exposure_requested:true}),
      exposure_surface("external_telegram_public_claim_exposure"; "blocked_external_telegram_public_claim_exposure_noop"; "external_telegram_public_claim_exposure_denied"; {public_claim_requested:true, telegram_status_exposure_requested:true}),
      exposure_surface("release_publication_public_claim_status_exposure"; "blocked_release_publication_public_claim_status_exposure_noop"; "release_publication_public_claim_status_exposure_denied"; {public_claim_requested:true, public_release_claim_requested:true, status_exposure_requested:true, public_status_exposure_requested:true, release_publication_status_exposure_requested:true}),
      exposure_surface("activation_live_install_status_public_exposure"; "blocked_activation_live_install_public_status_exposure_noop"; "activation_live_install_status_public_exposure_denied"; {status_exposure_requested:true, public_status_exposure_requested:true}),
      exposure_surface("install_restart_active_binary_public_status_exposure"; "blocked_install_restart_active_binary_public_status_exposure_noop"; "install_restart_active_binary_public_status_exposure_denied"; {status_exposure_requested:true, public_status_exposure_requested:true, install_restart_active_binary_status_exposure_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denial_gate" \
    --arg source_terminal_status_report_sha256 "$source_terminal_status_report_sha256" \
    --arg terminal_public_claim_status_exposure_contract_hash_sha256 "$terminal_public_claim_status_exposure_contract_hash_sha256" \
    --arg terminal_public_claim_status_exposure_policy_hash_sha256 "$terminal_public_claim_status_exposure_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$TERMINAL_STATUS_JSON" \
    --argjson surfaces "$terminal_public_claim_status_exposure_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_schema_version:"operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denial_v1",
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_mode:"denied_signing_receipt_terminal_status_cannot_create_public_claim_status_exposure_release_channel_telegram_or_live_install",
        source_artifact_distribution_signing_notarization_receipt_terminal_status_gate:$source.gate,
        source_artifact_distribution_signing_notarization_receipt_terminal_status_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denial_ready,
        source_artifact_distribution_signing_notarization_receipt_terminal_status_report_sha256:$source_terminal_status_report_sha256,
        source_artifact_distribution_signing_notarization_receipt_terminal_status_contract_hash_sha256:$source.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_contract_hash_sha256:$terminal_public_claim_status_exposure_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_policy_hash_sha256:$terminal_public_claim_status_exposure_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denial_ready:true,
        source_artifact_distribution_signing_notarization_receipt_terminal_status_surface_count:$source.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_surface_count,
        source_artifact_distribution_signing_notarization_receipt_terminal_status_denied_count:$source.artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denied_count,
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_attempt_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denied_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surfaces:$surfaces,
        denied_by_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure:[
          "source_artifact_distribution_signing_notarization_receipt_terminal_status_report_required",
          "artifact_distribution_signing_notarization_receipt_public_claim_recording_denied",
          "artifact_distribution_signing_notarization_receipt_public_status_exposure_denied",
          "artifact_distribution_signing_notarization_receipt_public_release_claim_denied",
          "artifact_distribution_signing_notarization_receipt_channel_external_telegram_public_status_denied",
          "artifact_distribution_signing_notarization_receipt_release_artifact_write_denied",
          "artifact_distribution_signing_notarization_receipt_public_artifact_write_denied",
          "artifact_distribution_signing_notarization_receipt_operator_approval_from_public_claim_denied",
          "artifact_distribution_signing_notarization_receipt_release_publication_authority_from_public_claim_denied",
          "artifact_distribution_signing_notarization_receipt_activation_authority_from_status_exposure_denied",
          "artifact_distribution_signing_notarization_receipt_download_install_restart_active_binary_from_status_exposure_denied",
          "artifact_distribution_signing_notarization_receipt_memory_provider_secret_external_send_from_public_exposure_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denial_gate",
            status:"allowed_report_only_next_slice",
            records_public_claim:false,
            records_status_exposure:false,
            delivers_channel_status:false,
            sends_telegram:false,
            writes_release_artifact:false,
            writes_public_artifact:false,
            derives_operator_approval:false,
            derives_release_publication_authority:false,
            derives_activation_authority:false,
            renders_download_link:false,
            emits_install_command:false,
            installs_or_restarts:false,
            mutates_active_binary:false,
            mutates_memory_store:false,
            writes_kg:false,
            invokes_provider:false,
            reads_credentials:false,
            sends_externally:false
          }
        ]
      }
      + zero_object([
        "artifact_distribution_signing_notarization_receipt_public_claim_recorded_count",
        "artifact_distribution_signing_notarization_receipt_public_claim_persisted_count",
        "artifact_distribution_signing_notarization_receipt_status_exposure_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_exposure_persisted_count",
        "artifact_distribution_signing_notarization_receipt_channel_status_exposure_delivered_count",
        "artifact_distribution_signing_notarization_receipt_external_status_exposure_sent_count",
        "artifact_distribution_signing_notarization_receipt_telegram_status_exposure_sent_count",
        "release_artifact_written_count",
        "public_artifact_written_count",
        "operator_approval_from_signing_receipt_public_claim_derived_count",
        "release_publication_authority_from_signing_receipt_public_claim_derived_count",
        "activation_authority_from_signing_receipt_status_exposure_derived_count",
        "download_link_from_signing_receipt_status_exposure_rendered_count",
        "install_command_from_signing_receipt_status_exposure_emitted_count",
        "install_from_signing_receipt_status_exposure_executed_count",
        "service_restart_from_signing_receipt_status_exposure_performed_count",
        "active_binary_from_signing_receipt_status_exposure_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "artifact_distribution_signing_notarization_receipt_public_claim_recorded",
        "artifact_distribution_signing_notarization_receipt_status_exposure_recorded",
        "artifact_distribution_signing_notarization_receipt_public_status_exposed",
        "public_release_claimed",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "download_link_rendered",
        "install_command_emitted",
        "activation_allowed",
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
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed"
      ])
      + {
        side_effects:false_object([
          "public_claim_recorded",
          "public_claim_persisted",
          "status_exposure_recorded",
          "status_exposure_persisted",
          "public_status_exposed",
          "channel_status_exposure_delivered",
          "external_status_exposure_sent",
          "telegram_status_exposure_sent",
          "operator_approval_from_public_claim_derived",
          "release_publication_authority_from_public_claim_derived",
          "activation_authority_from_status_exposure_derived",
          "download_link_rendered",
          "install_command_emitted",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denial_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_terminal_status_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_terminal_status_surface_count == 18
  and $report.source_artifact_distribution_signing_notarization_receipt_terminal_status_denied_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_attempt_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denied_count == 18
  and zero_fields($report; [
    "artifact_distribution_signing_notarization_receipt_public_claim_recorded_count",
    "artifact_distribution_signing_notarization_receipt_public_claim_persisted_count",
    "artifact_distribution_signing_notarization_receipt_status_exposure_recorded_count",
    "artifact_distribution_signing_notarization_receipt_status_exposure_persisted_count",
    "artifact_distribution_signing_notarization_receipt_channel_status_exposure_delivered_count",
    "artifact_distribution_signing_notarization_receipt_external_status_exposure_sent_count",
    "artifact_distribution_signing_notarization_receipt_telegram_status_exposure_sent_count",
    "release_artifact_written_count",
    "public_artifact_written_count",
    "operator_approval_from_signing_receipt_public_claim_derived_count",
    "release_publication_authority_from_signing_receipt_public_claim_derived_count",
    "activation_authority_from_signing_receipt_status_exposure_derived_count",
    "download_link_from_signing_receipt_status_exposure_rendered_count",
    "install_command_from_signing_receipt_status_exposure_emitted_count",
    "install_from_signing_receipt_status_exposure_executed_count",
    "service_restart_from_signing_receipt_status_exposure_performed_count",
    "active_binary_from_signing_receipt_status_exposure_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and ($report.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surfaces | length) == 18
  and ($report.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surfaces | all(
    .artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_attempted == true
    and .artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_noop_confirmed == true
    and .public_claim_allowed == false
    and .status_exposure_allowed == false
    and .public_release_claim_allowed == false
    and .public_status_exposure_allowed == false
    and false_fields(.; [
      "public_claim_recorded",
      "public_claim_persisted",
      "status_exposure_recorded",
      "status_exposure_persisted",
      "channel_status_exposure_delivered",
      "external_status_exposure_sent",
      "telegram_status_exposure_sent",
      "release_artifact_written",
      "public_artifact_written",
      "operator_approval_from_public_claim_derived",
      "release_publication_authority_from_public_claim_derived",
      "activation_authority_from_status_exposure_derived",
      "download_link_from_status_exposure_rendered",
      "install_command_from_status_exposure_emitted",
      "install_from_status_exposure_executed",
      "service_restart_from_status_exposure_performed",
      "active_binary_from_status_exposure_mutated",
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
  and ([.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surfaces[] | select(.public_claim_requested == true)] | length) == 6
  and ([.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surfaces[] | select(.status_exposure_requested == true)] | length) == 12
  and ([.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surfaces[] | select(.telegram_status_exposure_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surfaces[] | select(.release_publication_status_exposure_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surfaces[] | select(.install_restart_active_binary_status_exposure_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_public_claim == false
    and .records_status_exposure == false
    and .sends_telegram == false
    and .writes_release_artifact == false
    and .writes_public_artifact == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .invokes_provider == false
    and .reads_credentials == false
    and .sends_externally == false
  ))
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt terminal public claim/status exposure denial gate passed"
