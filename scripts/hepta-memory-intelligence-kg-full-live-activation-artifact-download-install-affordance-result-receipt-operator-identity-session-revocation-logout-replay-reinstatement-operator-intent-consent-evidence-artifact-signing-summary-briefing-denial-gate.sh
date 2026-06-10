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

ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_OBSERVABILITY_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-observability-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-observability-denial-gate.sh
)"

source_artifact_distribution_signing_notarization_receipt_export_query_observability_report_sha256="$(
  sha256_text "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_OBSERVABILITY_JSON"
)"
artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-operator-facing-summary-briefing-non-persistence-denial:$source_artifact_distribution_signing_notarization_receipt_export_query_observability_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-operator-facing-summary-briefing:no-summary:no-briefing:no-readback:no-delivery:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_OBSERVABILITY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_export_query_observability_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_export_query_observability_denial_ready == true
    and $source.source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_ready == true
    and $source.artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_export_query_observability_attempt_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count == 18
    and zero_fields($source; [
      "artifact_distribution_signing_notarization_receipt_export_query_observability_allowed_count",
      "artifact_distribution_signing_notarization_receipt_export_query_observability_accepted_count",
      "artifact_distribution_signing_notarization_receipt_export_query_observability_recorded_count",
      "artifact_distribution_signing_notarization_receipt_export_query_observability_persisted_count",
      "artifact_distribution_signing_notarization_receipt_query_registered_count",
      "artifact_distribution_signing_notarization_receipt_query_executed_count",
      "artifact_distribution_signing_notarization_receipt_query_result_recorded_count",
      "artifact_distribution_signing_notarization_receipt_query_result_persisted_count",
      "artifact_distribution_signing_notarization_receipt_search_index_recorded_count",
      "artifact_distribution_signing_notarization_receipt_export_accepted_count",
      "artifact_distribution_signing_notarization_receipt_export_snapshot_recorded_count",
      "artifact_distribution_signing_notarization_receipt_export_file_written_count",
      "artifact_distribution_signing_notarization_receipt_export_stream_opened_count",
      "artifact_distribution_signing_notarization_receipt_observability_metric_recorded_count",
      "artifact_distribution_signing_notarization_receipt_observability_trace_recorded_count",
      "artifact_distribution_signing_notarization_receipt_dashboard_panel_recorded_count",
      "artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count",
      "artifact_distribution_signing_notarization_receipt_audit_view_recorded_count",
      "artifact_distribution_signing_notarization_receipt_export_query_observability_acceptance_recorded_count",
      "release_publication_authority_from_signing_receipt_export_query_observability_derived_count",
      "activation_authority_from_signing_receipt_export_query_observability_derived_count",
      "download_link_from_signing_receipt_export_query_observability_rendered_count",
      "install_command_from_signing_receipt_export_query_observability_rendered_count",
      "install_from_signing_receipt_export_query_observability_executed_count",
      "service_restart_from_signing_receipt_export_query_observability_performed_count",
      "active_binary_from_signing_receipt_export_query_observability_mutated_count",
      "memory_store_write_performed_count",
      "live_kg_write_performed_count",
      "provider_invoked_count",
      "model_invoked_count",
      "credential_read_count",
      "secret_file_read_count",
      "external_send_performed_count"
    ])
    and false_fields($source; [
      "artifact_distribution_signing_notarization_receipt_export_query_observability_accepted",
      "artifact_distribution_signing_notarization_receipt_export_query_observability_recorded",
      "artifact_distribution_signing_notarization_receipt_export_query_observability_persisted",
      "artifact_distribution_signing_notarization_receipt_query_registered",
      "artifact_distribution_signing_notarization_receipt_query_executed",
      "artifact_distribution_signing_notarization_receipt_export_accepted",
      "artifact_distribution_signing_notarization_receipt_export_snapshot_recorded",
      "artifact_distribution_signing_notarization_receipt_observability_metric_recorded",
      "artifact_distribution_signing_notarization_receipt_observability_trace_recorded",
      "artifact_distribution_signing_notarization_receipt_dashboard_panel_recorded",
      "artifact_distribution_signing_notarization_receipt_operator_summary_recorded",
      "artifact_distribution_signing_notarization_receipt_audit_view_recorded",
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
    and ($source.artifact_distribution_signing_notarization_receipt_export_query_observability_surfaces | length) == 18
    and ($source.artifact_distribution_signing_notarization_receipt_export_query_observability_surfaces | all(
      .artifact_distribution_signing_notarization_receipt_export_query_observability_attempted == true
      and .artifact_distribution_signing_notarization_receipt_export_query_observability_allowed == false
      and .artifact_distribution_signing_notarization_receipt_export_query_observability_noop_confirmed == true
      and .query_registered == false
      and .query_executed == false
      and .export_accepted == false
      and .export_file_written == false
      and .observability_metric_recorded == false
      and .observability_trace_recorded == false
      and .dashboard_panel_recorded == false
      and .operator_summary_recorded == false
      and .readback_surface_recorded == false
      and .audit_view_recorded == false
      and .release_publication_authority_from_export_query_observability_derived == false
      and .activation_authority_from_export_query_observability_derived == false
      and .install_from_export_query_observability_executed == false
      and .service_restart_from_export_query_observability_performed == false
      and .active_binary_from_export_query_observability_mutated == false
      and .memory_store_write_performed == false
      and .live_kg_write_performed == false
      and .external_send_performed == false
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .registers_query == false
      and .executes_query == false
      and .records_query_result == false
      and .writes_search_index == false
      and .accepts_export == false
      and .writes_export == false
      and .records_observability == false
      and .records_operator_summary == false
      and .records_readback == false
      and .records_audit_view == false
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

surfaces_json="$(
  jq -n '
    def briefing_surface($id; $status; $reason; $extra):
      {
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface:$id,
        source_signing_receipt_export_query_observability_ready:true,
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempted:true,
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_allowed:false,
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted:false,
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded:false,
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted:false,
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_materialized:false,
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_filesystem_written:false,
        summary_requested:false,
        briefing_requested:false,
        readback_requested:false,
        status_banner_requested:false,
        exported_summary_requested:false,
        briefing_card_requested:false,
        notification_timeline_requested:false,
        dashboard_narrative_requested:false,
        audit_narrative_requested:false,
        delivery_requested:false,
        final_summary_requested:false,
        operator_memo_requested:false,
        approval_summary_requested:false,
        external_briefing_requested:false,
        telegram_briefing_requested:false,
        authority_briefing_requested:false,
        live_status_briefing_requested:false,
        install_restart_active_binary_status_requested:false,
        operator_summary_recorded:false,
        operator_summary_persisted:false,
        operator_briefing_recorded:false,
        operator_briefing_persisted:false,
        signing_receipt_readback_recorded:false,
        signing_receipt_readback_persisted:false,
        status_banner_recorded:false,
        exported_summary_recorded:false,
        briefing_card_recorded:false,
        notification_timeline_recorded:false,
        dashboard_narrative_recorded:false,
        audit_narrative_recorded:false,
        briefing_delivery_recorded:false,
        final_summary_recorded:false,
        operator_memo_recorded:false,
        approval_summary_recorded:false,
        external_briefing_delivered:false,
        telegram_briefing_delivered:false,
        authority_briefing_recorded:false,
        live_status_briefing_recorded:false,
        signing_receipt_query_registered:false,
        signing_receipt_query_executed:false,
        signing_receipt_query_result_recorded:false,
        signing_receipt_export_accepted:false,
        signing_receipt_export_file_written:false,
        signing_receipt_export_stream_opened:false,
        signing_receipt_observability_recorded:false,
        signing_receipt_dashboard_recorded:false,
        signing_receipt_alert_recorded:false,
        signing_receipt_result_receipt_recorded:false,
        signing_receipt_result_receipt_persisted:false,
        signing_receipt_completion_ack_recorded:false,
        operator_acceptance_from_summary_recorded:false,
        operator_acceptance_from_briefing_recorded:false,
        operator_approval_from_summary_derived:false,
        operator_approval_from_briefing_derived:false,
        release_publication_authority_from_summary_briefing_derived:false,
        activation_authority_from_summary_briefing_derived:false,
        download_link_from_summary_briefing_rendered:false,
        install_command_from_summary_briefing_rendered:false,
        install_from_summary_briefing_executed:false,
        service_restart_from_summary_briefing_performed:false,
        launchd_from_summary_briefing_mutated:false,
        active_binary_from_summary_briefing_mutated:false,
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
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_noop_confirmed:true,
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_status:$status,
        reason:$reason
      } + $extra;
    [
      briefing_surface("source_signing_receipt_export_query_observability_report_required"; "blocked_source_signing_receipt_observability_report_required_noop"; "source_signing_receipt_export_query_observability_report_required"; {source_report_required:true}),
      briefing_surface("artifact_signing_retention_query_operator_summary"; "blocked_artifact_signing_query_summary_noop"; "artifact_signing_retention_query_operator_summary_denied"; {summary_requested:true}),
      briefing_surface("package_signing_ttl_query_operator_briefing"; "blocked_package_signing_query_briefing_noop"; "package_signing_ttl_query_operator_briefing_denied"; {briefing_requested:true}),
      briefing_surface("signature_manifest_expiry_query_readback_digest"; "blocked_signature_manifest_query_readback_noop"; "signature_manifest_expiry_query_readback_digest_denied"; {readback_requested:true}),
      briefing_surface("notarization_search_index_status_banner"; "blocked_notarization_search_index_status_banner_noop"; "notarization_search_index_status_banner_denied"; {status_banner_requested:true}),
      briefing_surface("witness_notary_exported_summary_text"; "blocked_witness_notary_exported_summary_noop"; "witness_notary_exported_summary_text_denied"; {exported_summary_requested:true, summary_requested:true}),
      briefing_surface("tombstone_garbage_collection_export_briefing_card"; "blocked_tombstone_gc_briefing_card_noop"; "tombstone_garbage_collection_export_briefing_card_denied"; {briefing_card_requested:true, briefing_requested:true}),
      briefing_surface("replacement_garbage_collection_notification_timeline"; "blocked_replacement_gc_notification_timeline_noop"; "replacement_garbage_collection_notification_timeline_denied"; {notification_timeline_requested:true}),
      briefing_surface("provenance_archive_dashboard_narrative"; "blocked_provenance_archive_dashboard_narrative_noop"; "provenance_archive_dashboard_narrative_denied"; {dashboard_narrative_requested:true}),
      briefing_surface("sbom_compaction_audit_narrative"; "blocked_sbom_compaction_audit_narrative_noop"; "sbom_compaction_audit_narrative_denied"; {audit_narrative_requested:true}),
      briefing_surface("release_asset_retention_readback_final_summary"; "blocked_release_asset_final_summary_noop"; "release_asset_retention_readback_final_summary_denied"; {final_summary_requested:true, summary_requested:true, readback_requested:true}),
      briefing_surface("cdn_expiry_dashboard_briefing_panel"; "blocked_cdn_expiry_dashboard_briefing_noop"; "cdn_expiry_dashboard_briefing_panel_denied"; {briefing_requested:true, dashboard_narrative_requested:true}),
      briefing_surface("package_registry_trace_event_operator_memo"; "blocked_package_registry_trace_operator_memo_noop"; "package_registry_trace_event_operator_memo_denied"; {operator_memo_requested:true}),
      briefing_surface("dashboard_hash_alert_slo_approval_summary"; "blocked_dashboard_hash_alert_approval_summary_noop"; "dashboard_hash_alert_slo_approval_summary_denied"; {approval_summary_requested:true, summary_requested:true}),
      briefing_surface("external_telegram_observability_briefing_delivery"; "blocked_external_telegram_observability_briefing_noop"; "external_telegram_observability_briefing_delivery_denied"; {external_briefing_requested:true, telegram_briefing_requested:true, delivery_requested:true, briefing_requested:true}),
      briefing_surface("release_publication_authority_view_briefing"; "blocked_release_publication_authority_view_briefing_noop"; "release_publication_authority_view_briefing_denied"; {authority_briefing_requested:true, briefing_requested:true}),
      briefing_surface("activation_live_install_view_status_briefing"; "blocked_activation_live_install_status_briefing_noop"; "activation_live_install_view_status_briefing_denied"; {live_status_briefing_requested:true, briefing_requested:true}),
      briefing_surface("install_restart_active_binary_view_status_briefing"; "blocked_install_restart_active_binary_status_briefing_noop"; "install_restart_active_binary_view_status_briefing_denied"; {install_restart_active_binary_status_requested:true, live_status_briefing_requested:true, briefing_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_gate" \
    --arg source_artifact_distribution_signing_notarization_receipt_export_query_observability_report_sha256 "$source_artifact_distribution_signing_notarization_receipt_export_query_observability_report_sha256" \
    --arg artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_contract_hash_sha256 "$artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_contract_hash_sha256" \
    --arg artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_policy_hash_sha256 "$artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_OBSERVABILITY_JSON" \
    --argjson surfaces "$surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_schema_version:"operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_v1",
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_mode:"denied_signing_receipt_export_query_observability_cannot_be_summarized_briefed_read_back_delivered_promoted_or_used_for_authority_or_live_install",
        source_artifact_distribution_signing_notarization_receipt_export_query_observability_gate:$source.gate,
        source_artifact_distribution_signing_notarization_receipt_export_query_observability_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_export_query_observability_denial_ready,
        source_artifact_distribution_signing_notarization_receipt_export_query_observability_report_sha256:$source_artifact_distribution_signing_notarization_receipt_export_query_observability_report_sha256,
        source_artifact_distribution_signing_notarization_receipt_export_query_observability_contract_hash_sha256:$source.artifact_distribution_signing_notarization_receipt_export_query_observability_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_contract_hash_sha256:$artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_policy_hash_sha256:$artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_ready:true,
        source_artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count:$source.artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count,
        source_artifact_distribution_signing_notarization_receipt_export_query_observability_attempt_count:$source.artifact_distribution_signing_notarization_receipt_export_query_observability_attempt_count,
        source_artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count:$source.artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count,
        source_artifact_distribution_signing_notarization_receipt_query_registered_count:$source.artifact_distribution_signing_notarization_receipt_query_registered_count,
        source_artifact_distribution_signing_notarization_receipt_export_accepted_count:$source.artifact_distribution_signing_notarization_receipt_export_accepted_count,
        source_artifact_distribution_signing_notarization_receipt_observability_metric_recorded_count:$source.artifact_distribution_signing_notarization_receipt_observability_metric_recorded_count,
        source_artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count:$source.artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count,
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempt_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces:$surfaces,
        denied_by_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing:[
          "source_artifact_distribution_signing_notarization_receipt_export_query_observability_report_required",
          "artifact_signing_query_operator_summary_denied",
          "package_signing_query_operator_briefing_denied",
          "signature_manifest_query_readback_denied",
          "notarization_search_index_status_banner_denied",
          "witness_notary_exported_summary_denied",
          "tombstone_garbage_collection_briefing_card_denied",
          "replacement_garbage_collection_notification_timeline_denied",
          "provenance_archive_dashboard_narrative_denied",
          "sbom_compaction_audit_narrative_denied",
          "release_asset_retention_final_summary_denied",
          "cdn_expiry_dashboard_briefing_denied",
          "package_registry_trace_operator_memo_denied",
          "dashboard_hash_alert_approval_summary_denied",
          "external_telegram_observability_briefing_denied",
          "release_publication_authority_view_briefing_denied",
          "activation_live_install_status_briefing_denied",
          "install_restart_active_binary_status_briefing_denied",
          "memory_provider_kg_secret_external_send_from_summary_briefing_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_gate",
            status:"allowed_report_only_next_slice",
            records_summary:false,
            records_briefing:false,
            records_readback:false,
            records_status_banner:false,
            records_delivery:false,
            records_acknowledgement:false,
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
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_allowed_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_materialized_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_summary_persisted_count",
        "artifact_distribution_signing_notarization_receipt_operator_briefing_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_briefing_persisted_count",
        "artifact_distribution_signing_notarization_receipt_readback_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_banner_recorded_count",
        "artifact_distribution_signing_notarization_receipt_exported_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_briefing_card_recorded_count",
        "artifact_distribution_signing_notarization_receipt_notification_timeline_recorded_count",
        "artifact_distribution_signing_notarization_receipt_dashboard_narrative_recorded_count",
        "artifact_distribution_signing_notarization_receipt_audit_narrative_recorded_count",
        "artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded_count",
        "artifact_distribution_signing_notarization_receipt_final_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_memo_recorded_count",
        "artifact_distribution_signing_notarization_receipt_approval_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_external_briefing_delivered_count",
        "artifact_distribution_signing_notarization_receipt_telegram_briefing_delivered_count",
        "artifact_distribution_signing_notarization_receipt_authority_briefing_recorded_count",
        "artifact_distribution_signing_notarization_receipt_live_status_briefing_recorded_count",
        "artifact_distribution_signing_notarization_receipt_summary_briefing_acceptance_recorded_count",
        "operator_approval_from_signing_receipt_summary_briefing_derived_count",
        "release_publication_authority_from_signing_receipt_summary_briefing_derived_count",
        "activation_authority_from_signing_receipt_summary_briefing_derived_count",
        "download_link_from_signing_receipt_summary_briefing_rendered_count",
        "install_command_from_signing_receipt_summary_briefing_rendered_count",
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
      + false_object([
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted",
        "artifact_distribution_signing_notarization_receipt_operator_summary_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_summary_persisted",
        "artifact_distribution_signing_notarization_receipt_operator_briefing_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_briefing_persisted",
        "artifact_distribution_signing_notarization_receipt_readback_recorded",
        "artifact_distribution_signing_notarization_receipt_status_banner_recorded",
        "artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded",
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
          "operator_summary_recorded",
          "operator_summary_persisted",
          "operator_briefing_recorded",
          "operator_briefing_persisted",
          "readback_recorded",
          "status_banner_recorded",
          "exported_summary_recorded",
          "briefing_card_recorded",
          "notification_timeline_recorded",
          "dashboard_narrative_recorded",
          "audit_narrative_recorded",
          "briefing_delivery_recorded",
          "final_summary_recorded",
          "operator_memo_recorded",
          "approval_summary_recorded",
          "external_briefing_delivered",
          "telegram_briefing_delivered",
          "authority_briefing_recorded",
          "live_status_briefing_recorded",
          "summary_briefing_acceptance_recorded",
          "operator_approval_from_summary_briefing_derived",
          "release_publication_authority_from_summary_briefing_derived",
          "activation_authority_from_summary_briefing_derived",
          "download_link_from_summary_briefing_rendered",
          "install_command_from_summary_briefing_rendered",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_export_query_observability_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count == 18
  and $report.source_artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempt_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count == 18
  and zero_fields($report; [
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
  and false_fields($report; [
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
  and ($report.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces | length) == 18
  and ($report.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces | all(
    .artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempted == true
    and .artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_allowed == false
    and .artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_noop_confirmed == true
    and false_fields(.; [
      "operator_summary_recorded",
      "operator_summary_persisted",
      "operator_briefing_recorded",
      "operator_briefing_persisted",
      "signing_receipt_readback_recorded",
      "status_banner_recorded",
      "exported_summary_recorded",
      "briefing_card_recorded",
      "notification_timeline_recorded",
      "dashboard_narrative_recorded",
      "audit_narrative_recorded",
      "briefing_delivery_recorded",
      "final_summary_recorded",
      "operator_memo_recorded",
      "approval_summary_recorded",
      "external_briefing_delivered",
      "telegram_briefing_delivered",
      "authority_briefing_recorded",
      "live_status_briefing_recorded",
      "operator_acceptance_from_summary_recorded",
      "operator_acceptance_from_briefing_recorded",
      "operator_approval_from_summary_derived",
      "operator_approval_from_briefing_derived",
      "release_publication_authority_from_summary_briefing_derived",
      "activation_authority_from_summary_briefing_derived",
      "install_from_summary_briefing_executed",
      "service_restart_from_summary_briefing_performed",
      "active_binary_from_summary_briefing_mutated",
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
  and ([.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces[] | select(.summary_requested == true)] | length) == 4
  and ([.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces[] | select(.briefing_requested == true)] | length) == 7
  and ([.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces[] | select(.external_briefing_requested == true and .telegram_briefing_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces[] | select(.authority_briefing_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces[] | select(.live_status_briefing_requested == true)] | length) == 2
  and ([.artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces[] | select(.install_restart_active_binary_status_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
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
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt operator-facing summary/briefing non-persistence denial gate passed"
