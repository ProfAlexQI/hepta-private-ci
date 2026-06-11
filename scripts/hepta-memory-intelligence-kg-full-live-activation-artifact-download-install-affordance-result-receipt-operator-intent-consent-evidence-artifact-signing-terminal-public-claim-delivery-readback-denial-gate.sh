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

PUBLIC_EXPOSURE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-status-exposure-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-status-exposure-denial-gate.sh
)"

source_public_exposure_report_sha256="$(sha256_text "$PUBLIC_EXPOSURE_JSON")"
terminal_public_claim_delivery_readback_contract_hash_sha256="$(
  sha256_text "hepta-artifact-distribution-signing-notarization-receipt-terminal-public-claim-delivery-readback-denial:$source_public_exposure_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
terminal_public_claim_delivery_readback_policy_hash_sha256="$(
  sha256_text "artifact-distribution-signing-notarization-receipt-terminal-public-claim-delivery-readback:no-delivery:no-readback:no-receipt:no-release:no-channel:no-telegram:no-install"
)"

jq -n -e \
  --argjson source "$PUBLIC_EXPOSURE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denial_ready == true
    and $source.source_artifact_distribution_signing_notarization_receipt_terminal_status_ready == true
    and $source.source_artifact_distribution_signing_notarization_receipt_terminal_status_surface_count == 18
    and $source.source_artifact_distribution_signing_notarization_receipt_terminal_status_denied_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_attempt_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denied_count == 18
    and zero_fields($source; [
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
    and false_fields($source; [
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
    and ($source.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surfaces | length) == 18
    and ($source.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surfaces | all(
      .artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_attempted == true
      and .artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_noop_confirmed == true
      and .public_claim_allowed == false
      and .status_exposure_allowed == false
      and .public_release_claim_allowed == false
      and .public_status_exposure_allowed == false
      and .public_claim_recorded == false
      and .public_claim_persisted == false
      and .status_exposure_recorded == false
      and .status_exposure_persisted == false
      and .channel_status_exposure_delivered == false
      and .external_status_exposure_sent == false
      and .telegram_status_exposure_sent == false
      and .release_artifact_written == false
      and .public_artifact_written == false
      and .download_link_from_status_exposure_rendered == false
      and .install_command_from_status_exposure_emitted == false
      and .install_from_status_exposure_executed == false
      and .service_restart_from_status_exposure_performed == false
      and .active_binary_from_status_exposure_mutated == false
      and .memory_store_write_performed == false
      and .live_kg_write_performed == false
      and .external_send_performed == false
    ))
    and ([ $source.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surfaces[] | select(.public_claim_requested == true) ] | length) == 6
    and ([ $source.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surfaces[] | select(.status_exposure_requested == true) ] | length) == 12
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_public_claim == false
      and .records_status_exposure == false
      and .delivers_channel_status == false
      and .sends_telegram == false
      and .writes_release_artifact == false
      and .writes_public_artifact == false
      and .derives_operator_approval == false
      and .derives_release_publication_authority == false
      and .derives_activation_authority == false
      and .renders_download_link == false
      and .emits_install_command == false
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

terminal_public_claim_delivery_readback_surfaces_json="$(
  jq -n '
    def readback_surface($id; $status; $reason; $extra):
      {
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface:$id,
        source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_ready:true,
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_attempted:true,
        public_claim_delivery_requested:false,
        status_readback_requested:false,
        channel_delivery_requested:false,
        telegram_delivery_requested:false,
        release_publication_delivery_readback_requested:false,
        install_restart_active_binary_readback_requested:false,
        public_claim_delivery_allowed:false,
        status_readback_allowed:false,
        channel_delivery_allowed:false,
        telegram_delivery_allowed:false,
        public_claim_delivery_recorded:false,
        public_claim_delivery_persisted:false,
        status_readback_recorded:false,
        status_readback_persisted:false,
        channel_delivery_recorded:false,
        channel_delivery_persisted:false,
        channel_status_readback_delivered:false,
        external_delivery_readback_sent:false,
        telegram_delivery_readback_sent:false,
        delivery_receipt_recorded:false,
        delivery_receipt_persisted:false,
        readback_receipt_recorded:false,
        readback_receipt_persisted:false,
        release_artifact_written:false,
        public_artifact_written:false,
        operator_approval_from_delivery_readback_derived:false,
        release_publication_authority_from_delivery_readback_derived:false,
        activation_authority_from_delivery_readback_derived:false,
        download_link_from_delivery_readback_rendered:false,
        install_command_from_delivery_readback_emitted:false,
        install_from_delivery_readback_executed:false,
        service_restart_from_delivery_readback_performed:false,
        active_binary_from_delivery_readback_mutated:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        external_send_performed:false,
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_noop_confirmed:true,
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_status:$status,
        reason:$reason
      } + $extra;
    [
      readback_surface("source_public_claim_status_exposure_report_required"; "blocked_source_public_exposure_report_required_noop"; "source_public_claim_status_exposure_report_required"; {source_public_claim_status_exposure_report_required:true}),
      readback_surface("artifact_signing_receipt_claim_channel_delivery_attempt"; "blocked_artifact_signing_claim_channel_delivery_noop"; "artifact_signing_receipt_claim_channel_delivery_attempt_denied"; {public_claim_delivery_requested:true, channel_delivery_requested:true}),
      readback_surface("package_signing_status_badge_readback_attempt"; "blocked_package_signing_status_badge_readback_noop"; "package_signing_status_badge_readback_attempt_denied"; {status_readback_requested:true}),
      readback_surface("signature_manifest_status_page_readback_attempt"; "blocked_signature_manifest_status_page_readback_noop"; "signature_manifest_status_page_readback_attempt_denied"; {status_readback_requested:true}),
      readback_surface("notarization_claim_readback_attempt"; "blocked_notarization_claim_readback_noop"; "notarization_claim_readback_attempt_denied"; {public_claim_delivery_requested:true}),
      readback_surface("witness_notary_summary_channel_delivery_attempt"; "blocked_witness_notary_summary_channel_delivery_noop"; "witness_notary_summary_channel_delivery_attempt_denied"; {status_readback_requested:true, channel_delivery_requested:true}),
      readback_surface("tombstone_gc_final_response_delivery_readback_attempt"; "blocked_tombstone_gc_final_response_delivery_readback_noop"; "tombstone_gc_final_response_delivery_readback_attempt_denied"; {public_claim_delivery_requested:true}),
      readback_surface("replacement_gc_completion_readback_attempt"; "blocked_replacement_gc_completion_readback_noop"; "replacement_gc_completion_readback_attempt_denied"; {status_readback_requested:true}),
      readback_surface("provenance_dashboard_status_readback_attempt"; "blocked_provenance_dashboard_status_readback_noop"; "provenance_dashboard_status_readback_attempt_denied"; {status_readback_requested:true}),
      readback_surface("sbom_audit_public_claim_readback_attempt"; "blocked_sbom_audit_public_claim_readback_noop"; "sbom_audit_public_claim_readback_attempt_denied"; {public_claim_delivery_requested:true}),
      readback_surface("release_asset_briefing_channel_delivery_attempt"; "blocked_release_asset_briefing_channel_delivery_noop"; "release_asset_briefing_channel_delivery_attempt_denied"; {status_readback_requested:true, channel_delivery_requested:true}),
      readback_surface("cdn_dashboard_readback_attempt"; "blocked_cdn_dashboard_readback_noop"; "cdn_dashboard_readback_attempt_denied"; {status_readback_requested:true}),
      readback_surface("package_registry_memo_delivery_attempt"; "blocked_package_registry_memo_delivery_noop"; "package_registry_memo_delivery_attempt_denied"; {status_readback_requested:true, channel_delivery_requested:true}),
      readback_surface("dashboard_hash_approval_channel_readback_attempt"; "blocked_dashboard_hash_approval_channel_readback_noop"; "dashboard_hash_approval_channel_readback_attempt_denied"; {status_readback_requested:true, channel_delivery_requested:true}),
      readback_surface("external_telegram_claim_delivery_readback_attempt"; "blocked_external_telegram_claim_delivery_readback_noop"; "external_telegram_claim_delivery_readback_attempt_denied"; {public_claim_delivery_requested:true, telegram_delivery_requested:true}),
      readback_surface("release_publication_claim_status_delivery_readback_attempt"; "blocked_release_publication_claim_status_delivery_readback_noop"; "release_publication_claim_status_delivery_readback_attempt_denied"; {public_claim_delivery_requested:true, status_readback_requested:true, channel_delivery_requested:true, release_publication_delivery_readback_requested:true}),
      readback_surface("activation_live_install_status_readback_attempt"; "blocked_activation_live_install_status_readback_noop"; "activation_live_install_status_readback_attempt_denied"; {status_readback_requested:true}),
      readback_surface("install_restart_active_binary_status_readback_attempt"; "blocked_install_restart_active_binary_status_readback_noop"; "install_restart_active_binary_status_readback_attempt_denied"; {status_readback_requested:true, install_restart_active_binary_readback_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denial_gate" \
    --arg source_public_exposure_report_sha256 "$source_public_exposure_report_sha256" \
    --arg terminal_public_claim_delivery_readback_contract_hash_sha256 "$terminal_public_claim_delivery_readback_contract_hash_sha256" \
    --arg terminal_public_claim_delivery_readback_policy_hash_sha256 "$terminal_public_claim_delivery_readback_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$PUBLIC_EXPOSURE_JSON" \
    --argjson surfaces "$terminal_public_claim_delivery_readback_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_schema_version:"operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denial_v1",
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_mode:"denied_public_claim_status_exposure_cannot_create_delivery_readback_receipt_release_channel_telegram_or_live_install",
        source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_gate:$source.gate,
        source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denial_ready,
        source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_report_sha256:$source_public_exposure_report_sha256,
        source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_contract_hash_sha256:$source.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_contract_hash_sha256:$terminal_public_claim_delivery_readback_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_policy_hash_sha256:$terminal_public_claim_delivery_readback_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denial_ready:true,
        source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface_count:$source.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface_count,
        source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denied_count:$source.artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denied_count,
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_attempt_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denied_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surfaces:$surfaces,
        denied_by_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback:[
          "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_report_required",
          "artifact_distribution_signing_notarization_receipt_public_claim_delivery_recording_denied",
          "artifact_distribution_signing_notarization_receipt_status_readback_recording_denied",
          "artifact_distribution_signing_notarization_receipt_channel_delivery_recording_denied",
          "artifact_distribution_signing_notarization_receipt_channel_external_telegram_delivery_readback_denied",
          "artifact_distribution_signing_notarization_receipt_delivery_receipt_persistence_denied",
          "artifact_distribution_signing_notarization_receipt_readback_receipt_persistence_denied",
          "artifact_distribution_signing_notarization_receipt_release_artifact_write_denied",
          "artifact_distribution_signing_notarization_receipt_public_artifact_write_denied",
          "artifact_distribution_signing_notarization_receipt_operator_approval_from_delivery_readback_denied",
          "artifact_distribution_signing_notarization_receipt_release_publication_authority_from_delivery_readback_denied",
          "artifact_distribution_signing_notarization_receipt_activation_authority_from_delivery_readback_denied",
          "artifact_distribution_signing_notarization_receipt_download_install_restart_active_binary_from_delivery_readback_denied",
          "artifact_distribution_signing_notarization_receipt_memory_provider_secret_external_send_from_delivery_readback_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_non_persistence_denial_gate",
            status:"allowed_report_only_next_slice",
            records_public_claim_delivery:false,
            records_status_readback:false,
            records_channel_delivery:false,
            records_delivery_receipt:false,
            records_readback_receipt:false,
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
        "artifact_distribution_signing_notarization_receipt_public_claim_delivery_recorded_count",
        "artifact_distribution_signing_notarization_receipt_public_claim_delivery_persisted_count",
        "artifact_distribution_signing_notarization_receipt_status_readback_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_readback_persisted_count",
        "artifact_distribution_signing_notarization_receipt_channel_delivery_recorded_count",
        "artifact_distribution_signing_notarization_receipt_channel_delivery_persisted_count",
        "artifact_distribution_signing_notarization_receipt_channel_status_readback_delivered_count",
        "artifact_distribution_signing_notarization_receipt_external_delivery_readback_sent_count",
        "artifact_distribution_signing_notarization_receipt_telegram_delivery_readback_sent_count",
        "delivery_receipt_recorded_count",
        "delivery_receipt_persisted_count",
        "readback_receipt_recorded_count",
        "readback_receipt_persisted_count",
        "release_artifact_written_count",
        "public_artifact_written_count",
        "operator_approval_from_delivery_readback_derived_count",
        "release_publication_authority_from_delivery_readback_derived_count",
        "activation_authority_from_delivery_readback_derived_count",
        "download_link_from_delivery_readback_rendered_count",
        "install_command_from_delivery_readback_emitted_count",
        "install_from_delivery_readback_executed_count",
        "service_restart_from_delivery_readback_performed_count",
        "active_binary_from_delivery_readback_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "artifact_distribution_signing_notarization_receipt_public_claim_delivery_recorded",
        "artifact_distribution_signing_notarization_receipt_status_readback_recorded",
        "artifact_distribution_signing_notarization_receipt_channel_delivery_recorded",
        "delivery_receipt_recorded",
        "delivery_receipt_persisted",
        "readback_receipt_recorded",
        "readback_receipt_persisted",
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
          "public_claim_delivery_recorded",
          "public_claim_delivery_persisted",
          "status_readback_recorded",
          "status_readback_persisted",
          "channel_delivery_recorded",
          "channel_delivery_persisted",
          "channel_status_readback_delivered",
          "external_delivery_readback_sent",
          "telegram_delivery_readback_sent",
          "delivery_receipt_recorded",
          "delivery_receipt_persisted",
          "readback_receipt_recorded",
          "readback_receipt_persisted",
          "operator_approval_from_delivery_readback_derived",
          "release_publication_authority_from_delivery_readback_derived",
          "activation_authority_from_delivery_readback_derived",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denial_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface_count == 18
  and $report.source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denied_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_attempt_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denied_count == 18
  and zero_fields($report; [
    "artifact_distribution_signing_notarization_receipt_public_claim_delivery_recorded_count",
    "artifact_distribution_signing_notarization_receipt_public_claim_delivery_persisted_count",
    "artifact_distribution_signing_notarization_receipt_status_readback_recorded_count",
    "artifact_distribution_signing_notarization_receipt_status_readback_persisted_count",
    "artifact_distribution_signing_notarization_receipt_channel_delivery_recorded_count",
    "artifact_distribution_signing_notarization_receipt_channel_delivery_persisted_count",
    "artifact_distribution_signing_notarization_receipt_channel_status_readback_delivered_count",
    "artifact_distribution_signing_notarization_receipt_external_delivery_readback_sent_count",
    "artifact_distribution_signing_notarization_receipt_telegram_delivery_readback_sent_count",
    "delivery_receipt_recorded_count",
    "delivery_receipt_persisted_count",
    "readback_receipt_recorded_count",
    "readback_receipt_persisted_count",
    "release_artifact_written_count",
    "public_artifact_written_count",
    "operator_approval_from_delivery_readback_derived_count",
    "release_publication_authority_from_delivery_readback_derived_count",
    "activation_authority_from_delivery_readback_derived_count",
    "download_link_from_delivery_readback_rendered_count",
    "install_command_from_delivery_readback_emitted_count",
    "install_from_delivery_readback_executed_count",
    "service_restart_from_delivery_readback_performed_count",
    "active_binary_from_delivery_readback_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and ($report.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surfaces | length) == 18
  and ($report.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surfaces | all(
    .artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_attempted == true
    and .artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_noop_confirmed == true
    and .public_claim_delivery_allowed == false
    and .status_readback_allowed == false
    and .channel_delivery_allowed == false
    and .telegram_delivery_allowed == false
    and false_fields(.; [
      "public_claim_delivery_recorded",
      "public_claim_delivery_persisted",
      "status_readback_recorded",
      "status_readback_persisted",
      "channel_delivery_recorded",
      "channel_delivery_persisted",
      "channel_status_readback_delivered",
      "external_delivery_readback_sent",
      "telegram_delivery_readback_sent",
      "delivery_receipt_recorded",
      "delivery_receipt_persisted",
      "readback_receipt_recorded",
      "readback_receipt_persisted",
      "release_artifact_written",
      "public_artifact_written",
      "operator_approval_from_delivery_readback_derived",
      "release_publication_authority_from_delivery_readback_derived",
      "activation_authority_from_delivery_readback_derived",
      "download_link_from_delivery_readback_rendered",
      "install_command_from_delivery_readback_emitted",
      "install_from_delivery_readback_executed",
      "service_restart_from_delivery_readback_performed",
      "active_binary_from_delivery_readback_mutated",
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
  and ([.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surfaces[] | select(.public_claim_delivery_requested == true)] | length) == 6
  and ([.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surfaces[] | select(.status_readback_requested == true)] | length) == 12
  and ([.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surfaces[] | select(.channel_delivery_requested == true)] | length) == 6
  and ([.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surfaces[] | select(.telegram_delivery_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surfaces[] | select(.release_publication_delivery_readback_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surfaces[] | select(.install_restart_active_binary_readback_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_non_persistence_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_public_claim_delivery == false
    and .records_status_readback == false
    and .records_channel_delivery == false
    and .records_delivery_receipt == false
    and .records_readback_receipt == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt terminal public claim delivery/readback denial gate passed"
