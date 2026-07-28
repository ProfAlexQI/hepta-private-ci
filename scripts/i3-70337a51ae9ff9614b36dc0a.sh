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

ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_SUMMARY_BRIEFING_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-summary-briefing-denial-gate" \
    scripts/i3-04b4ddd17a52efa504c34208.sh
)"

source_artifact_distribution_signing_notarization_receipt_summary_briefing_report_sha256="$(
  sha256_text "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_SUMMARY_BRIEFING_JSON"
)"
artifact_distribution_signing_notarization_receipt_final_acknowledgement_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-final-operator-acknowledgement-non-acceptance-denial:$source_artifact_distribution_signing_notarization_receipt_summary_briefing_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
artifact_distribution_signing_notarization_receipt_final_acknowledgement_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-final-operator-acknowledgement:no-ack:no-received:no-confirmed:no-read:no-seen:no-final-response:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_SUMMARY_BRIEFING_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
    and $source.source_artifact_distribution_signing_notarization_receipt_export_query_observability_ready == true
    and $source.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempt_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count == 18
    and zero_fields($source; [
      "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_allowed_count",
      "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted_count",
      "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded_count",
      "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted_count",
      "artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count",
      "artifact_distribution_signing_notarization_receipt_operator_briefing_recorded_count",
      "artifact_distribution_signing_notarization_receipt_readback_recorded_count",
      "artifact_distribution_signing_notarization_receipt_status_banner_recorded_count",
      "artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded_count",
      "artifact_distribution_signing_notarization_receipt_summary_briefing_acceptance_recorded_count",
      "operator_approval_from_signing_receipt_summary_briefing_derived_count",
      "release_publication_authority_from_signing_receipt_summary_briefing_derived_count",
      "activation_authority_from_signing_receipt_summary_briefing_derived_count",
      "install_from_signing_receipt_summary_briefing_executed_count",
      "service_restart_from_signing_receipt_summary_briefing_performed_count",
      "active_binary_from_signing_receipt_summary_briefing_mutated_count",
      "memory_store_write_performed_count",
      "live_kg_write_performed_count",
      "provider_invoked_count",
      "model_invoked_count",
      "credential_read_count",
      "secret_file_read_count",
      "external_send_performed_count"
    ])
    and false_fields($source; [
      "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted",
      "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded",
      "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted",
      "artifact_distribution_signing_notarization_receipt_operator_summary_recorded",
      "artifact_distribution_signing_notarization_receipt_operator_briefing_recorded",
      "artifact_distribution_signing_notarization_receipt_readback_recorded",
      "artifact_distribution_signing_notarization_receipt_status_banner_recorded",
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
    and ($source.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces | length) == 18
    and ($source.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces | all(
      .artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempted == true
      and .artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_allowed == false
      and .artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_noop_confirmed == true
      and .operator_summary_recorded == false
      and .operator_briefing_recorded == false
      and .signing_receipt_readback_recorded == false
      and .status_banner_recorded == false
      and .briefing_delivery_recorded == false
      and .operator_acceptance_from_summary_recorded == false
      and .operator_acceptance_from_briefing_recorded == false
      and .release_publication_authority_from_summary_briefing_derived == false
      and .activation_authority_from_summary_briefing_derived == false
      and .install_from_summary_briefing_executed == false
      and .service_restart_from_summary_briefing_performed == false
      and .active_binary_from_summary_briefing_mutated == false
      and .memory_store_write_performed == false
      and .live_kg_write_performed == false
      and .external_send_performed == false
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_summary == false
      and .records_briefing == false
      and .records_readback == false
      and .records_delivery == false
      and .records_acknowledgement == false
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

final_acknowledgement_surfaces_json="$(
  jq -n '
    def ack_surface($id; $status; $reason; $extra):
      {
        artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface:$id,
        source_signing_receipt_summary_briefing_ready:true,
        artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempted:true,
        artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_allowed:false,
        final_operator_acknowledgement_requested:false,
        final_operator_acknowledgement_request_accepted:false,
        final_operator_acknowledgement_accepted:false,
        final_operator_acknowledgement_recorded:false,
        final_operator_acknowledgement_persisted:false,
        final_operator_acknowledgement_materialized:false,
        final_operator_acknowledgement_filesystem_written:false,
        final_operator_acknowledgement_delivered:false,
        operator_received_requested:false,
        operator_received_recorded:false,
        operator_confirmed_requested:false,
        operator_confirmed_recorded:false,
        operator_read_requested:false,
        operator_read_recorded:false,
        operator_seen_requested:false,
        operator_seen_recorded:false,
        final_response_requested:false,
        final_response_recorded:false,
        completion_acknowledgement_requested:false,
        completion_acknowledgement_recorded:false,
        status_acknowledgement_requested:false,
        status_acknowledgement_recorded:false,
        summary_acknowledgement_requested:false,
        summary_acknowledgement_recorded:false,
        briefing_acknowledgement_requested:false,
        briefing_acknowledgement_recorded:false,
        readback_digest_acknowledgement_requested:false,
        readback_digest_acknowledgement_recorded:false,
        dashboard_acknowledgement_requested:false,
        dashboard_acknowledgement_recorded:false,
        notification_acknowledgement_requested:false,
        notification_acknowledgement_recorded:false,
        channel_acknowledgement_requested:false,
        channel_acknowledgement_delivered:false,
        external_acknowledgement_requested:false,
        external_acknowledgement_sent:false,
        telegram_acknowledgement_requested:false,
        telegram_acknowledgement_sent:false,
        operator_approval_acknowledgement_requested:false,
        authority_acknowledgement_requested:false,
        live_acknowledgement_requested:false,
        install_restart_active_binary_acknowledgement_requested:false,
        acknowledgement_acceptance_recorded:false,
        operator_acceptance_from_acknowledgement_recorded:false,
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
        signing_receipt_summary_from_acknowledgement_recorded:false,
        signing_receipt_briefing_from_acknowledgement_recorded:false,
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
        artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_noop_confirmed:true,
        artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_status:$status,
        reason:$reason
      } + $extra;
    [
      ack_surface("source_signing_receipt_summary_briefing_report_required"; "blocked_source_signing_receipt_summary_briefing_required_noop"; "source_signing_receipt_summary_briefing_report_required"; {source_report_required:true}),
      ack_surface("artifact_signing_summary_final_operator_acknowledgement_claim"; "blocked_artifact_signing_summary_final_ack_noop"; "artifact_signing_summary_final_operator_acknowledgement_claim_denied"; {final_operator_acknowledgement_requested:true}),
      ack_surface("package_signing_briefing_operator_received_claim"; "blocked_package_signing_briefing_operator_received_noop"; "package_signing_briefing_operator_received_claim_denied"; {operator_received_requested:true}),
      ack_surface("signature_manifest_readback_operator_confirmed_claim"; "blocked_signature_manifest_readback_operator_confirmed_noop"; "signature_manifest_readback_operator_confirmed_claim_denied"; {operator_confirmed_requested:true}),
      ack_surface("notarization_status_banner_operator_read_claim"; "blocked_notarization_status_banner_operator_read_noop"; "notarization_status_banner_operator_read_claim_denied"; {operator_read_requested:true}),
      ack_surface("witness_notary_exported_summary_operator_seen_claim"; "blocked_witness_notary_summary_operator_seen_noop"; "witness_notary_exported_summary_operator_seen_claim_denied"; {operator_seen_requested:true}),
      ack_surface("tombstone_garbage_collection_briefing_card_final_response_claim"; "blocked_tombstone_gc_briefing_final_response_noop"; "tombstone_garbage_collection_briefing_card_final_response_claim_denied"; {final_response_requested:true}),
      ack_surface("replacement_garbage_collection_notification_completion_acknowledgement_claim"; "blocked_replacement_gc_completion_ack_noop"; "replacement_garbage_collection_notification_completion_acknowledgement_claim_denied"; {completion_acknowledgement_requested:true}),
      ack_surface("provenance_dashboard_narrative_status_acknowledgement_claim"; "blocked_provenance_dashboard_status_ack_noop"; "provenance_dashboard_narrative_status_acknowledgement_claim_denied"; {status_acknowledgement_requested:true}),
      ack_surface("sbom_audit_narrative_summary_acknowledgement_claim"; "blocked_sbom_audit_summary_ack_noop"; "sbom_audit_narrative_summary_acknowledgement_claim_denied"; {summary_acknowledgement_requested:true}),
      ack_surface("release_asset_final_summary_briefing_acknowledgement_claim"; "blocked_release_asset_briefing_ack_noop"; "release_asset_final_summary_briefing_acknowledgement_claim_denied"; {briefing_acknowledgement_requested:true}),
      ack_surface("cdn_dashboard_briefing_readback_digest_acknowledgement_claim"; "blocked_cdn_dashboard_readback_ack_noop"; "cdn_dashboard_briefing_readback_digest_acknowledgement_claim_denied"; {readback_digest_acknowledgement_requested:true}),
      ack_surface("package_registry_operator_memo_dashboard_notification_acknowledgement_claim"; "blocked_package_registry_dashboard_notification_ack_noop"; "package_registry_operator_memo_dashboard_notification_acknowledgement_claim_denied"; {dashboard_acknowledgement_requested:true, notification_acknowledgement_requested:true}),
      ack_surface("dashboard_hash_approval_summary_channel_acknowledgement_claim"; "blocked_dashboard_hash_channel_ack_noop"; "dashboard_hash_approval_summary_channel_acknowledgement_claim_denied"; {channel_acknowledgement_requested:true, operator_approval_acknowledgement_requested:true}),
      ack_surface("external_telegram_observability_briefing_acknowledgement_claim"; "blocked_external_telegram_briefing_ack_noop"; "external_telegram_observability_briefing_acknowledgement_claim_denied"; {external_acknowledgement_requested:true, telegram_acknowledgement_requested:true}),
      ack_surface("release_publication_authority_view_acknowledgement_claim"; "blocked_release_publication_authority_ack_noop"; "release_publication_authority_view_acknowledgement_claim_denied"; {authority_acknowledgement_requested:true}),
      ack_surface("activation_live_install_view_acknowledgement_claim"; "blocked_activation_live_install_ack_noop"; "activation_live_install_view_acknowledgement_claim_denied"; {live_acknowledgement_requested:true}),
      ack_surface("install_restart_active_binary_status_acknowledgement_claim"; "blocked_install_restart_active_binary_ack_noop"; "install_restart_active_binary_status_acknowledgement_claim_denied"; {install_restart_active_binary_acknowledgement_requested:true, live_acknowledgement_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_gate" \
    --arg source_artifact_distribution_signing_notarization_receipt_summary_briefing_report_sha256 "$source_artifact_distribution_signing_notarization_receipt_summary_briefing_report_sha256" \
    --arg artifact_distribution_signing_notarization_receipt_final_acknowledgement_contract_hash_sha256 "$artifact_distribution_signing_notarization_receipt_final_acknowledgement_contract_hash_sha256" \
    --arg artifact_distribution_signing_notarization_receipt_final_acknowledgement_policy_hash_sha256 "$artifact_distribution_signing_notarization_receipt_final_acknowledgement_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_SUMMARY_BRIEFING_JSON" \
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
        artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_schema_version:"operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_v1",
        artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_mode:"denied_signing_receipt_summary_briefing_cannot_be_acknowledged_accepted_promoted_or_used_for_authority_or_live_install",
        source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_gate:$source.gate,
        source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_ready,
        source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_report_sha256:$source_artifact_distribution_signing_notarization_receipt_summary_briefing_report_sha256,
        source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_contract_hash_sha256:$source.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_contract_hash_sha256:$artifact_distribution_signing_notarization_receipt_final_acknowledgement_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_policy_hash_sha256:$artifact_distribution_signing_notarization_receipt_final_acknowledgement_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_ready:true,
        source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count:$source.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count,
        source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempt_count:$source.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempt_count,
        source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count:$source.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count,
        source_artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count:$source.artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count,
        source_artifact_distribution_signing_notarization_receipt_operator_briefing_recorded_count:$source.artifact_distribution_signing_notarization_receipt_operator_briefing_recorded_count,
        source_artifact_distribution_signing_notarization_receipt_readback_recorded_count:$source.artifact_distribution_signing_notarization_receipt_readback_recorded_count,
        source_artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded_count:$source.artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded_count,
        artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempt_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces:$surfaces,
        denied_by_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement:[
          "source_artifact_distribution_signing_notarization_receipt_summary_briefing_report_required",
          "artifact_signing_summary_final_operator_acknowledgement_denied",
          "package_signing_briefing_operator_received_denied",
          "signature_manifest_readback_operator_confirmed_denied",
          "notarization_status_banner_operator_read_denied",
          "witness_notary_exported_summary_operator_seen_denied",
          "tombstone_garbage_collection_briefing_card_final_response_denied",
          "replacement_garbage_collection_notification_completion_acknowledgement_denied",
          "provenance_dashboard_narrative_status_acknowledgement_denied",
          "sbom_audit_narrative_summary_acknowledgement_denied",
          "release_asset_final_summary_briefing_acknowledgement_denied",
          "cdn_dashboard_briefing_readback_digest_acknowledgement_denied",
          "package_registry_operator_memo_dashboard_notification_acknowledgement_denied",
          "dashboard_hash_approval_summary_channel_acknowledgement_denied",
          "external_telegram_observability_briefing_acknowledgement_denied",
          "release_publication_authority_view_acknowledgement_denied",
          "activation_live_install_view_acknowledgement_denied",
          "install_restart_active_binary_status_acknowledgement_denied",
          "memory_provider_kg_secret_external_send_from_acknowledgement_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denial_gate",
            status:"allowed_report_only_next_slice",
            records_final_acknowledgement:false,
            records_received_confirmed_read_seen:false,
            records_terminal_decision:false,
            records_status_promotion:false,
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
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_allowed_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_persisted_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_materialized_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_delivered_count",
        "artifact_distribution_signing_notarization_receipt_operator_received_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_confirmed_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_read_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_seen_recorded_count",
        "artifact_distribution_signing_notarization_receipt_final_response_recorded_count",
        "artifact_distribution_signing_notarization_receipt_completion_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_summary_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_briefing_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_readback_digest_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_dashboard_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_notification_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_channel_acknowledgement_delivered_count",
        "artifact_distribution_signing_notarization_receipt_external_acknowledgement_sent_count",
        "artifact_distribution_signing_notarization_receipt_telegram_acknowledgement_sent_count",
        "artifact_distribution_signing_notarization_receipt_acceptance_from_acknowledgement_recorded_count",
        "operator_approval_from_signing_receipt_acknowledgement_derived_count",
        "release_publication_authority_from_signing_receipt_acknowledgement_derived_count",
        "activation_authority_from_signing_receipt_acknowledgement_derived_count",
        "download_link_from_signing_receipt_acknowledgement_rendered_count",
        "install_command_from_signing_receipt_acknowledgement_rendered_count",
        "install_from_signing_receipt_acknowledgement_executed_count",
        "service_restart_from_signing_receipt_acknowledgement_performed_count",
        "active_binary_from_signing_receipt_acknowledgement_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_persisted",
        "artifact_distribution_signing_notarization_receipt_operator_received_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_confirmed_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_read_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_seen_recorded",
        "artifact_distribution_signing_notarization_receipt_final_response_recorded",
        "artifact_distribution_signing_notarization_receipt_completion_acknowledgement_recorded",
        "artifact_distribution_signing_notarization_receipt_status_acknowledgement_recorded",
        "artifact_distribution_signing_notarization_receipt_acknowledgement_acceptance_recorded",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count == 18
  and $report.source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempt_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count == 18
  and zero_fields($report; [
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_allowed_count",
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted_count",
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded_count",
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_persisted_count",
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_materialized_count",
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_filesystem_written_count",
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_delivered_count",
    "artifact_distribution_signing_notarization_receipt_operator_received_recorded_count",
    "artifact_distribution_signing_notarization_receipt_operator_confirmed_recorded_count",
    "artifact_distribution_signing_notarization_receipt_operator_read_recorded_count",
    "artifact_distribution_signing_notarization_receipt_operator_seen_recorded_count",
    "artifact_distribution_signing_notarization_receipt_final_response_recorded_count",
    "artifact_distribution_signing_notarization_receipt_completion_acknowledgement_recorded_count",
    "artifact_distribution_signing_notarization_receipt_status_acknowledgement_recorded_count",
    "artifact_distribution_signing_notarization_receipt_summary_acknowledgement_recorded_count",
    "artifact_distribution_signing_notarization_receipt_briefing_acknowledgement_recorded_count",
    "artifact_distribution_signing_notarization_receipt_readback_digest_acknowledgement_recorded_count",
    "artifact_distribution_signing_notarization_receipt_dashboard_acknowledgement_recorded_count",
    "artifact_distribution_signing_notarization_receipt_notification_acknowledgement_recorded_count",
    "artifact_distribution_signing_notarization_receipt_channel_acknowledgement_delivered_count",
    "artifact_distribution_signing_notarization_receipt_external_acknowledgement_sent_count",
    "artifact_distribution_signing_notarization_receipt_telegram_acknowledgement_sent_count",
    "artifact_distribution_signing_notarization_receipt_acceptance_from_acknowledgement_recorded_count",
    "operator_approval_from_signing_receipt_acknowledgement_derived_count",
    "release_publication_authority_from_signing_receipt_acknowledgement_derived_count",
    "activation_authority_from_signing_receipt_acknowledgement_derived_count",
    "install_from_signing_receipt_acknowledgement_executed_count",
    "service_restart_from_signing_receipt_acknowledgement_performed_count",
    "active_binary_from_signing_receipt_acknowledgement_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and false_fields($report; [
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted",
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded",
    "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_persisted",
    "artifact_distribution_signing_notarization_receipt_operator_received_recorded",
    "artifact_distribution_signing_notarization_receipt_operator_confirmed_recorded",
    "artifact_distribution_signing_notarization_receipt_operator_read_recorded",
    "artifact_distribution_signing_notarization_receipt_operator_seen_recorded",
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
  and ($report.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces | length) == 18
  and ($report.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces | all(
    .artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempted == true
    and .artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_allowed == false
    and .artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_noop_confirmed == true
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
  and ([$report.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces[] | select(.final_operator_acknowledgement_requested == true)] | length) == 1
  and ([$report.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces[] | select(.telegram_acknowledgement_requested == true)] | length) == 1
  and ([$report.artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces[] | select(.install_restart_active_binary_acknowledgement_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_final_acknowledgement == false
    and .records_terminal_decision == false
    and .records_status_promotion == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt final operator acknowledgement non-acceptance denial gate passed"
