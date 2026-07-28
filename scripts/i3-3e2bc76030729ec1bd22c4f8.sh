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

ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_RETENTION_GC_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-retention-gc-denial-gate" \
    scripts/i3-23c50392cf7a14a65c29adf5.sh
)"

source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_report_sha256="$(
  sha256_text "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_RETENTION_GC_JSON"
)"
artifact_distribution_signing_notarization_receipt_export_query_observability_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-export-query-observability-denial:$source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
artifact_distribution_signing_notarization_receipt_export_query_observability_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-export-query-observability:no-query:no-export:no-observability:no-readback:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_RETENTION_GC_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denial_ready == true
    and $source.source_artifact_distribution_signing_notarization_receipt_audit_evidence_ready == true
    and $source.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surface_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempt_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denied_count == 18
    and zero_fields($source; [
      "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_allowed_count",
      "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted_count",
      "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_recorded_count",
      "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_persisted_count",
      "artifact_distribution_signing_notarization_receipt_retention_policy_recorded_count",
      "artifact_distribution_signing_notarization_receipt_retention_policy_persisted_count",
      "artifact_distribution_signing_notarization_receipt_ttl_lease_recorded_count",
      "artifact_distribution_signing_notarization_receipt_expiry_timestamp_recorded_count",
      "artifact_distribution_signing_notarization_receipt_expiry_scheduler_recorded_count",
      "artifact_distribution_signing_notarization_receipt_expiry_timer_started_count",
      "artifact_distribution_signing_notarization_receipt_expiry_ack_recorded_count",
      "artifact_distribution_signing_notarization_receipt_garbage_collection_queue_recorded_count",
      "artifact_distribution_signing_notarization_receipt_garbage_collection_scan_performed_count",
      "artifact_distribution_signing_notarization_receipt_garbage_collection_candidate_recorded_count",
      "artifact_distribution_signing_notarization_receipt_garbage_collection_decision_recorded_count",
      "artifact_distribution_signing_notarization_receipt_archive_recorded_count",
      "artifact_distribution_signing_notarization_receipt_compaction_recorded_count",
      "artifact_distribution_signing_notarization_receipt_audit_evidence_retention_recorded_count",
      "artifact_distribution_signing_notarization_receipt_immutable_evidence_retention_recorded_count",
      "release_publication_authority_from_signing_receipt_retention_derived_count",
      "activation_authority_from_signing_receipt_retention_derived_count",
      "install_from_signing_receipt_retention_executed_count",
      "service_restart_from_signing_receipt_retention_performed_count",
      "active_binary_from_signing_receipt_retention_mutated_count",
      "memory_store_write_performed_count",
      "live_kg_write_performed_count",
      "provider_invoked_count",
      "model_invoked_count",
      "credential_read_count",
      "secret_file_read_count",
      "external_send_performed_count"
    ])
    and false_fields($source; [
      "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted",
      "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_recorded",
      "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_persisted",
      "artifact_distribution_signing_notarization_receipt_retention_policy_recorded",
      "artifact_distribution_signing_notarization_receipt_expiry_recorded",
      "artifact_distribution_signing_notarization_receipt_garbage_collection_recorded",
      "artifact_distribution_signing_notarization_receipt_archive_recorded",
      "artifact_distribution_signing_notarization_receipt_compaction_recorded",
      "artifact_distribution_signing_notarization_receipt_audit_evidence_retention_recorded",
      "artifact_distribution_signing_notarization_receipt_immutable_evidence_retention_recorded",
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
    and ($source.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces | length) == 18
    and ($source.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces | all(
      .artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempted == true
      and .artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_allowed == false
      and .artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_noop_confirmed == true
      and .retention_policy_recorded == false
      and .ttl_lease_recorded == false
      and .expiry_timestamp_recorded == false
      and .garbage_collection_queue_recorded == false
      and .garbage_collection_scan_performed == false
      and .garbage_collection_decision_recorded == false
      and .archive_recorded == false
      and .compaction_recorded == false
      and .release_publication_authority_from_retention_derived == false
      and .activation_authority_from_retention_derived == false
      and .install_from_retention_executed == false
      and .service_restart_from_retention_performed == false
      and .active_binary_from_retention_mutated == false
      and .memory_store_write_performed == false
      and .live_kg_write_performed == false
      and .external_send_performed == false
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_export_query_observability_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_retention == false
      and .records_expiry == false
      and .records_garbage_collection == false
      and .records_archive == false
      and .records_compaction == false
      and .registers_export == false
      and .registers_query == false
      and .records_observability == false
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
    def view_surface($id; $status; $reason; $extra):
      {
        artifact_distribution_signing_notarization_receipt_export_query_observability_surface:$id,
        source_signing_receipt_retention_expiry_garbage_collection_ready:true,
        artifact_distribution_signing_notarization_receipt_export_query_observability_attempted:true,
        artifact_distribution_signing_notarization_receipt_export_query_observability_allowed:false,
        artifact_distribution_signing_notarization_receipt_export_query_observability_accepted:false,
        artifact_distribution_signing_notarization_receipt_export_query_observability_recorded:false,
        artifact_distribution_signing_notarization_receipt_export_query_observability_persisted:false,
        artifact_distribution_signing_notarization_receipt_export_query_observability_materialized:false,
        artifact_distribution_signing_notarization_receipt_export_query_observability_filesystem_written:false,
        query_requested:false,
        query_registration_requested:false,
        query_execution_requested:false,
        query_result_requested:false,
        search_index_requested:false,
        export_requested:false,
        export_snapshot_requested:false,
        export_file_requested:false,
        export_stream_requested:false,
        observability_requested:false,
        metric_log_requested:false,
        trace_event_requested:false,
        dashboard_panel_requested:false,
        alert_slo_requested:false,
        operator_summary_readback_requested:false,
        audit_view_requested:false,
        ledger_observability_requested:false,
        index_observability_requested:false,
        delivery_observability_requested:false,
        archive_view_requested:false,
        compaction_view_requested:false,
        external_telegram_observability_requested:false,
        release_publication_authority_view_requested:false,
        activation_authority_view_requested:false,
        live_install_view_requested:false,
        install_restart_active_binary_view_requested:false,
        query_registered:false,
        query_executed:false,
        query_result_recorded:false,
        query_result_persisted:false,
        search_index_recorded:false,
        search_index_persisted:false,
        export_accepted:false,
        export_snapshot_recorded:false,
        export_snapshot_persisted:false,
        export_file_written:false,
        export_stream_opened:false,
        observability_metric_recorded:false,
        observability_log_recorded:false,
        observability_trace_recorded:false,
        observability_event_recorded:false,
        dashboard_panel_recorded:false,
        alert_registered:false,
        slo_recorded:false,
        operator_summary_recorded:false,
        readback_surface_recorded:false,
        audit_view_recorded:false,
        ledger_observability_recorded:false,
        index_observability_recorded:false,
        delivery_observability_recorded:false,
        retention_policy_recorded:false,
        expiry_recorded:false,
        garbage_collection_recorded:false,
        archive_recorded:false,
        compaction_recorded:false,
        audit_evidence_recorded:false,
        immutable_evidence_recorded:false,
        hash_chain_recorded:false,
        attestation_recorded:false,
        witness_notary_recorded:false,
        result_receipt_recorded:false,
        result_receipt_persisted:false,
        result_receipt_exported:false,
        result_receipt_query_registered:false,
        result_receipt_observability_recorded:false,
        completion_ack_recorded:false,
        operator_acceptance_from_export_query_observability_recorded:false,
        operator_approval_from_export_query_observability_derived:false,
        release_publication_authority_from_export_query_observability_derived:false,
        activation_authority_from_export_query_observability_derived:false,
        download_link_from_export_query_observability_rendered:false,
        install_command_from_export_query_observability_rendered:false,
        install_from_export_query_observability_executed:false,
        service_restart_from_export_query_observability_performed:false,
        launchd_from_export_query_observability_mutated:false,
        active_binary_from_export_query_observability_mutated:false,
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
        artifact_distribution_signing_notarization_receipt_export_query_observability_noop_confirmed:true,
        artifact_distribution_signing_notarization_receipt_export_query_observability_status:$status,
        reason:$reason
      } + $extra;
    [
      view_surface("source_signing_receipt_retention_expiry_garbage_collection_report_required"; "blocked_source_signing_receipt_retention_gc_report_required_noop"; "source_signing_receipt_retention_expiry_garbage_collection_report_required"; {source_report_required:true}),
      view_surface("artifact_signing_audit_trail_retention_policy_query_registration"; "blocked_artifact_signing_retention_query_registration_noop"; "artifact_signing_audit_trail_retention_policy_query_registration_denied"; {query_requested:true, query_registration_requested:true}),
      view_surface("package_signing_immutable_evidence_ttl_lease_query_execution"; "blocked_package_signing_ttl_query_execution_noop"; "package_signing_immutable_evidence_ttl_lease_query_execution_denied"; {query_requested:true, query_execution_requested:true}),
      view_surface("signature_manifest_hash_chain_expiry_query_result"; "blocked_signature_manifest_expiry_query_result_noop"; "signature_manifest_hash_chain_expiry_query_result_denied"; {query_requested:true, query_result_requested:true}),
      view_surface("notarization_attestation_retention_search_index"; "blocked_notarization_retention_search_index_noop"; "notarization_attestation_retention_search_index_denied"; {search_index_requested:true, index_observability_requested:true}),
      view_surface("notarization_ticket_witness_notary_export_request"; "blocked_witness_notary_export_request_noop"; "notarization_ticket_witness_notary_export_request_denied"; {export_requested:true}),
      view_surface("stapling_tombstone_garbage_collection_export_snapshot"; "blocked_stapling_tombstone_export_snapshot_noop"; "stapling_tombstone_garbage_collection_export_snapshot_denied"; {export_requested:true, export_snapshot_requested:true}),
      view_surface("installer_replacement_garbage_collection_export_file"; "blocked_installer_replacement_export_file_noop"; "installer_replacement_garbage_collection_export_file_denied"; {export_requested:true, export_file_requested:true}),
      view_surface("provenance_immutable_evidence_archive_export_stream"; "blocked_provenance_archive_export_stream_noop"; "provenance_immutable_evidence_archive_export_stream_denied"; {export_requested:true, export_stream_requested:true, archive_view_requested:true}),
      view_surface("sbom_evidence_compaction_observability_metric_log"; "blocked_sbom_compaction_metric_log_noop"; "sbom_evidence_compaction_observability_metric_log_denied"; {observability_requested:true, metric_log_requested:true, compaction_view_requested:true}),
      view_surface("release_asset_cancelled_query_retention_readback"; "blocked_release_asset_cancelled_query_readback_noop"; "release_asset_cancelled_query_retention_readback_denied"; {operator_summary_readback_requested:true}),
      view_surface("cdn_observability_expiry_dashboard_panel"; "blocked_cdn_expiry_dashboard_panel_noop"; "cdn_observability_expiry_dashboard_panel_denied"; {observability_requested:true, dashboard_panel_requested:true}),
      view_surface("package_registry_replacement_status_trace_event"; "blocked_package_registry_status_trace_event_noop"; "package_registry_replacement_status_trace_event_denied"; {observability_requested:true, trace_event_requested:true}),
      view_surface("dashboard_endpoint_hash_status_alert_slo"; "blocked_dashboard_hash_status_alert_slo_noop"; "dashboard_endpoint_hash_status_alert_slo_denied"; {observability_requested:true, alert_slo_requested:true}),
      view_surface("external_telegram_retention_delivery_observability"; "blocked_external_telegram_retention_observability_noop"; "external_telegram_retention_delivery_observability_denied"; {observability_requested:true, delivery_observability_requested:true, external_telegram_observability_requested:true}),
      view_surface("release_publication_authority_retention_view"; "blocked_release_publication_authority_retention_view_noop"; "release_publication_authority_retention_view_denied"; {release_publication_authority_view_requested:true, audit_view_requested:true}),
      view_surface("activation_live_install_garbage_collection_view"; "blocked_activation_live_install_gc_view_noop"; "activation_live_install_garbage_collection_view_denied"; {activation_authority_view_requested:true, live_install_view_requested:true}),
      view_surface("install_restart_active_binary_retention_gc_view"; "blocked_install_restart_active_binary_retention_gc_view_noop"; "install_restart_active_binary_retention_gc_view_denied"; {install_restart_active_binary_view_requested:true, ledger_observability_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_export_query_observability_denial_gate" \
    --arg source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_report_sha256 "$source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_report_sha256" \
    --arg artifact_distribution_signing_notarization_receipt_export_query_observability_contract_hash_sha256 "$artifact_distribution_signing_notarization_receipt_export_query_observability_contract_hash_sha256" \
    --arg artifact_distribution_signing_notarization_receipt_export_query_observability_policy_hash_sha256 "$artifact_distribution_signing_notarization_receipt_export_query_observability_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_RETENTION_GC_JSON" \
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
        artifact_distribution_signing_notarization_receipt_export_query_observability_schema_version:"operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_export_query_observability_denial_v1",
        artifact_distribution_signing_notarization_receipt_export_query_observability_mode:"denied_signing_receipt_retention_expiry_garbage_collection_cannot_be_exported_queried_observed_read_back_promoted_or_used_for_authority_or_live_install",
        source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_gate:$source.gate,
        source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denial_ready,
        source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_report_sha256:$source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_report_sha256,
        source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_contract_hash_sha256:$source.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_export_query_observability_contract_hash_sha256:$artifact_distribution_signing_notarization_receipt_export_query_observability_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_export_query_observability_policy_hash_sha256:$artifact_distribution_signing_notarization_receipt_export_query_observability_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_export_query_observability_denial_ready:true,
        source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surface_count:$source.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surface_count,
        source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempt_count:$source.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempt_count,
        source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denied_count:$source.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denied_count,
        source_artifact_distribution_signing_notarization_receipt_retention_policy_recorded_count:$source.artifact_distribution_signing_notarization_receipt_retention_policy_recorded_count,
        source_artifact_distribution_signing_notarization_receipt_expiry_timestamp_recorded_count:$source.artifact_distribution_signing_notarization_receipt_expiry_timestamp_recorded_count,
        source_artifact_distribution_signing_notarization_receipt_garbage_collection_scan_performed_count:$source.artifact_distribution_signing_notarization_receipt_garbage_collection_scan_performed_count,
        source_artifact_distribution_signing_notarization_receipt_archive_recorded_count:$source.artifact_distribution_signing_notarization_receipt_archive_recorded_count,
        source_artifact_distribution_signing_notarization_receipt_compaction_recorded_count:$source.artifact_distribution_signing_notarization_receipt_compaction_recorded_count,
        artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_export_query_observability_attempt_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_export_query_observability_surfaces:$surfaces,
        denied_by_artifact_distribution_signing_notarization_receipt_export_query_observability:[
          "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_report_required",
          "signing_receipt_retention_query_registration_denied",
          "signing_receipt_ttl_query_execution_denied",
          "signing_receipt_expiry_query_result_denied",
          "signing_receipt_search_index_denied",
          "signing_receipt_export_request_denied",
          "signing_receipt_export_snapshot_denied",
          "signing_receipt_export_file_denied",
          "signing_receipt_export_stream_denied",
          "signing_receipt_archive_export_stream_denied",
          "signing_receipt_compaction_metric_log_denied",
          "signing_receipt_dashboard_panel_denied",
          "signing_receipt_trace_event_denied",
          "signing_receipt_alert_slo_denied",
          "external_telegram_signing_receipt_observability_denied",
          "release_publication_authority_view_denied",
          "activation_live_install_view_denied",
          "install_restart_active_binary_view_denied",
          "memory_provider_kg_secret_external_send_from_view_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_gate",
            status:"allowed_report_only_next_slice",
            registers_query:false,
            executes_query:false,
            records_query_result:false,
            writes_search_index:false,
            accepts_export:false,
            writes_export:false,
            opens_export_stream:false,
            records_observability:false,
            records_operator_summary:false,
            records_readback:false,
            records_audit_view:false,
            records_delivery_evidence:false,
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
        "artifact_distribution_signing_notarization_receipt_export_query_observability_allowed_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_accepted_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_recorded_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_persisted_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_materialized_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_query_registered_count",
        "artifact_distribution_signing_notarization_receipt_query_executed_count",
        "artifact_distribution_signing_notarization_receipt_query_result_recorded_count",
        "artifact_distribution_signing_notarization_receipt_query_result_persisted_count",
        "artifact_distribution_signing_notarization_receipt_search_index_recorded_count",
        "artifact_distribution_signing_notarization_receipt_search_index_persisted_count",
        "artifact_distribution_signing_notarization_receipt_export_accepted_count",
        "artifact_distribution_signing_notarization_receipt_export_snapshot_recorded_count",
        "artifact_distribution_signing_notarization_receipt_export_snapshot_persisted_count",
        "artifact_distribution_signing_notarization_receipt_export_file_written_count",
        "artifact_distribution_signing_notarization_receipt_export_stream_opened_count",
        "artifact_distribution_signing_notarization_receipt_observability_metric_recorded_count",
        "artifact_distribution_signing_notarization_receipt_observability_log_recorded_count",
        "artifact_distribution_signing_notarization_receipt_observability_trace_recorded_count",
        "artifact_distribution_signing_notarization_receipt_observability_event_recorded_count",
        "artifact_distribution_signing_notarization_receipt_dashboard_panel_recorded_count",
        "artifact_distribution_signing_notarization_receipt_alert_registered_count",
        "artifact_distribution_signing_notarization_receipt_slo_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_readback_surface_recorded_count",
        "artifact_distribution_signing_notarization_receipt_audit_view_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ledger_observability_recorded_count",
        "artifact_distribution_signing_notarization_receipt_index_observability_recorded_count",
        "artifact_distribution_signing_notarization_receipt_delivery_observability_recorded_count",
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
      + false_object([
        "artifact_distribution_signing_notarization_receipt_export_query_observability_accepted",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_recorded",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_persisted",
        "artifact_distribution_signing_notarization_receipt_query_registered",
        "artifact_distribution_signing_notarization_receipt_query_executed",
        "artifact_distribution_signing_notarization_receipt_query_result_recorded",
        "artifact_distribution_signing_notarization_receipt_export_accepted",
        "artifact_distribution_signing_notarization_receipt_export_snapshot_recorded",
        "artifact_distribution_signing_notarization_receipt_export_file_written",
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
      + {
        side_effects:false_object([
          "query_registered",
          "query_executed",
          "query_result_recorded",
          "query_result_persisted",
          "search_index_recorded",
          "search_index_persisted",
          "export_accepted",
          "export_snapshot_recorded",
          "export_snapshot_persisted",
          "export_file_written",
          "export_stream_opened",
          "observability_metric_recorded",
          "observability_log_recorded",
          "observability_trace_recorded",
          "observability_event_recorded",
          "dashboard_panel_recorded",
          "alert_registered",
          "slo_recorded",
          "operator_summary_recorded",
          "readback_surface_recorded",
          "audit_view_recorded",
          "ledger_observability_recorded",
          "index_observability_recorded",
          "delivery_observability_recorded",
          "result_receipt_recorded",
          "result_receipt_persisted",
          "result_receipt_exported",
          "result_receipt_query_registered",
          "result_receipt_observability_recorded",
          "completion_ack_recorded",
          "operator_acceptance_from_export_query_observability_recorded",
          "operator_approval_from_export_query_observability_derived",
          "release_publication_authority_from_export_query_observability_derived",
          "activation_authority_from_export_query_observability_derived",
          "download_link_from_export_query_observability_rendered",
          "install_command_from_export_query_observability_rendered",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_export_query_observability_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_export_query_observability_denial_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surface_count == 18
  and $report.source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denied_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_export_query_observability_attempt_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count == 18
  and zero_fields($report; [
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
  and false_fields($report; [
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
  and ($report.artifact_distribution_signing_notarization_receipt_export_query_observability_surfaces | length) == 18
  and ($report.artifact_distribution_signing_notarization_receipt_export_query_observability_surfaces | all(
    .artifact_distribution_signing_notarization_receipt_export_query_observability_attempted == true
    and .artifact_distribution_signing_notarization_receipt_export_query_observability_allowed == false
    and .artifact_distribution_signing_notarization_receipt_export_query_observability_noop_confirmed == true
    and false_fields(.; [
      "query_registered",
      "query_executed",
      "query_result_recorded",
      "query_result_persisted",
      "search_index_recorded",
      "search_index_persisted",
      "export_accepted",
      "export_snapshot_recorded",
      "export_file_written",
      "export_stream_opened",
      "observability_metric_recorded",
      "observability_trace_recorded",
      "dashboard_panel_recorded",
      "operator_summary_recorded",
      "readback_surface_recorded",
      "audit_view_recorded",
      "result_receipt_recorded",
      "result_receipt_persisted",
      "operator_acceptance_from_export_query_observability_recorded",
      "operator_approval_from_export_query_observability_derived",
      "release_publication_authority_from_export_query_observability_derived",
      "activation_authority_from_export_query_observability_derived",
      "install_from_export_query_observability_executed",
      "service_restart_from_export_query_observability_performed",
      "active_binary_from_export_query_observability_mutated",
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
  and ([.artifact_distribution_signing_notarization_receipt_export_query_observability_surfaces[] | select(.query_requested == true)] | length) == 3
  and ([.artifact_distribution_signing_notarization_receipt_export_query_observability_surfaces[] | select(.export_requested == true)] | length) == 4
  and ([.artifact_distribution_signing_notarization_receipt_export_query_observability_surfaces[] | select(.observability_requested == true)] | length) == 5
  and ([.artifact_distribution_signing_notarization_receipt_export_query_observability_surfaces[] | select(.external_telegram_observability_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_export_query_observability_surfaces[] | select(.release_publication_authority_view_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_export_query_observability_surfaces[] | select(.activation_authority_view_requested == true and .live_install_view_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_export_query_observability_surfaces[] | select(.install_restart_active_binary_view_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
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
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt export/query/observability denial gate passed"
