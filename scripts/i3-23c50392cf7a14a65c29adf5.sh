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

ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_AUDIT_EVIDENCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-audit-evidence-denial-gate" \
    scripts/i3-b0f3c9318ab22c55487cb1e2.sh
)"

source_artifact_distribution_signing_notarization_receipt_audit_evidence_report_sha256="$(
  sha256_text "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_AUDIT_EVIDENCE_JSON"
)"
artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-retention-expiry-garbage-collection-denial:$source_artifact_distribution_signing_notarization_receipt_audit_evidence_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-retention-expiry-garbage-collection:no-retention:no-expiry:no-gc:no-archive:no-compaction:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_AUDIT_EVIDENCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_audit_evidence_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_audit_evidence_denial_ready == true
    and $source.source_artifact_distribution_signing_notarization_receipt_cancellation_supersession_ready == true
    and $source.artifact_distribution_signing_notarization_receipt_audit_evidence_surface_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_audit_evidence_attempt_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_audit_evidence_denied_count == 18
    and zero_fields($source; [
      "artifact_distribution_signing_notarization_receipt_audit_evidence_allowed_count",
      "artifact_distribution_signing_notarization_receipt_audit_evidence_accepted_count",
      "artifact_distribution_signing_notarization_receipt_audit_evidence_recorded_count",
      "artifact_distribution_signing_notarization_receipt_audit_evidence_persisted_count",
      "artifact_distribution_signing_notarization_receipt_audit_evidence_materialized_count",
      "artifact_distribution_signing_notarization_receipt_audit_evidence_filesystem_written_count",
      "artifact_distribution_signing_notarization_receipt_audit_trail_recorded_count",
      "artifact_distribution_signing_notarization_receipt_audit_trail_persisted_count",
      "artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded_count",
      "artifact_distribution_signing_notarization_receipt_immutable_evidence_persisted_count",
      "artifact_distribution_signing_notarization_receipt_hash_chain_recorded_count",
      "artifact_distribution_signing_notarization_receipt_merkle_root_recorded_count",
      "artifact_distribution_signing_notarization_receipt_attestation_recorded_count",
      "artifact_distribution_signing_notarization_receipt_witness_recorded_count",
      "artifact_distribution_signing_notarization_receipt_notary_recorded_count",
      "artifact_distribution_signing_notarization_receipt_ledger_recorded_count",
      "artifact_distribution_signing_notarization_receipt_ledger_persisted_count",
      "artifact_distribution_signing_notarization_receipt_index_recorded_count",
      "artifact_distribution_signing_notarization_receipt_index_persisted_count",
      "artifact_distribution_signing_notarization_receipt_delivery_evidence_recorded_count",
      "artifact_distribution_signing_notarization_receipt_delivery_evidence_delivered_count",
      "artifact_distribution_signing_notarization_receipt_query_export_evidence_recorded_count",
      "artifact_distribution_signing_notarization_receipt_observability_evidence_recorded_count",
      "artifact_distribution_signing_notarization_receipt_readback_evidence_recorded_count",
      "artifact_distribution_signing_notarization_receipt_status_evidence_recorded_count",
      "artifact_distribution_signing_notarization_receipt_hash_status_evidence_recorded_count",
      "release_publication_authority_from_signing_receipt_audit_evidence_derived_count",
      "activation_authority_from_signing_receipt_audit_evidence_derived_count",
      "install_from_signing_receipt_audit_evidence_executed_count",
      "service_restart_from_signing_receipt_audit_evidence_performed_count",
      "active_binary_from_signing_receipt_audit_evidence_mutated_count",
      "memory_store_write_performed_count",
      "live_kg_write_performed_count",
      "provider_invoked_count",
      "model_invoked_count",
      "credential_read_count",
      "secret_file_read_count",
      "external_send_performed_count"
    ])
    and false_fields($source; [
      "artifact_distribution_signing_notarization_receipt_audit_evidence_accepted",
      "artifact_distribution_signing_notarization_receipt_audit_evidence_recorded",
      "artifact_distribution_signing_notarization_receipt_audit_evidence_persisted",
      "artifact_distribution_signing_notarization_receipt_audit_evidence_materialized",
      "artifact_distribution_signing_notarization_receipt_audit_trail_recorded",
      "artifact_distribution_signing_notarization_receipt_audit_trail_persisted",
      "artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded",
      "artifact_distribution_signing_notarization_receipt_immutable_evidence_persisted",
      "artifact_distribution_signing_notarization_receipt_hash_chain_recorded",
      "artifact_distribution_signing_notarization_receipt_attestation_recorded",
      "artifact_distribution_signing_notarization_receipt_ledger_recorded",
      "public_release_claimed",
      "public_ga_claimed",
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
    and ($source.artifact_distribution_signing_notarization_receipt_audit_evidence_surfaces | length) == 18
    and ($source.artifact_distribution_signing_notarization_receipt_audit_evidence_surfaces | all(
      .artifact_distribution_signing_notarization_receipt_audit_evidence_attempted == true
      and .artifact_distribution_signing_notarization_receipt_audit_evidence_allowed == false
      and .artifact_distribution_signing_notarization_receipt_audit_evidence_noop_confirmed == true
      and .artifact_distribution_signing_notarization_receipt_audit_trail_recorded == false
      and .artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded == false
      and .artifact_distribution_signing_notarization_receipt_ledger_recorded == false
      and .release_publication_authority_derived == false
      and .activation_authority_derived == false
      and .install_executed == false
      and .service_restarted == false
      and .active_binary_mutated == false
      and .memory_store_write_performed == false
      and .live_kg_write_performed == false
      and .external_send_performed == false
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denial_gate"
      and .records_audit_evidence == false
      and .records_immutable_evidence == false
      and .records_hash_chain == false
      and .records_attestation == false
      and .records_witness == false
      and .records_notary == false
      and .records_ledger == false
      and .persists_audit_evidence == false
      and .accepts_retention == false
      and .accepts_expiry == false
      and .performs_garbage_collection == false
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
    def retention_surface($id; $status; $reason; $extra):
      {
        surface:$id,
        source_signing_receipt_audit_evidence_denial_ready:true,
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempted:true,
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_allowed:false,
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted:false,
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_recorded:false,
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_persisted:false,
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_materialized:false,
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_filesystem_written:false,
        retention_policy_requested:false,
        ttl_lease_requested:false,
        expiry_timestamp_requested:false,
        expiry_scheduler_requested:false,
        expiry_timer_requested:false,
        expiry_ack_requested:false,
        garbage_collection_queue_requested:false,
        garbage_collection_scan_requested:false,
        garbage_collection_candidate_requested:false,
        garbage_collection_decision_requested:false,
        tombstone_gc_requested:false,
        delete_marker_gc_requested:false,
        archive_requested:false,
        compaction_requested:false,
        audit_evidence_retention_requested:false,
        immutable_evidence_retention_requested:false,
        hash_attestation_retention_requested:false,
        witness_notary_expiry_requested:false,
        ledger_index_retention_requested:false,
        delivery_evidence_retention_requested:false,
        status_evidence_expiry_requested:false,
        external_telegram_retention_requested:false,
        release_publication_retention_authority_requested:false,
        activation_retention_authority_requested:false,
        live_install_gc_evidence_requested:false,
        retention_policy_recorded:false,
        retention_policy_persisted:false,
        ttl_lease_recorded:false,
        ttl_lease_persisted:false,
        expiry_timestamp_recorded:false,
        expiry_scheduler_recorded:false,
        expiry_timer_started:false,
        expiry_ack_recorded:false,
        expiry_state_persisted:false,
        garbage_collection_queue_recorded:false,
        garbage_collection_scan_performed:false,
        garbage_collection_candidate_recorded:false,
        garbage_collection_decision_recorded:false,
        garbage_collection_state_persisted:false,
        tombstone_gc_recorded:false,
        delete_marker_gc_recorded:false,
        archive_recorded:false,
        compaction_recorded:false,
        audit_evidence_retention_recorded:false,
        immutable_evidence_retention_recorded:false,
        hash_attestation_retention_recorded:false,
        witness_notary_expiry_recorded:false,
        ledger_index_retention_recorded:false,
        delivery_evidence_retention_recorded:false,
        status_evidence_expiry_recorded:false,
        result_receipt_from_retention_recorded:false,
        result_receipt_from_retention_persisted:false,
        operator_approval_from_retention_derived:false,
        release_publication_authority_from_retention_derived:false,
        activation_authority_from_retention_derived:false,
        download_link_from_retention_rendered:false,
        install_command_from_retention_rendered:false,
        install_from_retention_executed:false,
        service_restart_from_retention_performed:false,
        launchd_from_retention_mutated:false,
        active_binary_from_retention_mutated:false,
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
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_noop_confirmed:true,
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_status:$status,
        reason:$reason
      } + $extra;
    [
      retention_surface("source_signing_receipt_audit_evidence_report_required"; "blocked_source_signing_receipt_audit_evidence_required_noop"; "source_signing_receipt_audit_evidence_report_required"; {source_report_required:true}),
      retention_surface("artifact_signing_audit_trail_retention_policy"; "blocked_artifact_signing_audit_trail_retention_noop"; "artifact_signing_audit_trail_retention_policy_denied"; {retention_policy_requested:true, audit_evidence_retention_requested:true}),
      retention_surface("package_signing_immutable_evidence_ttl_lease"; "blocked_package_signing_immutable_evidence_ttl_noop"; "package_signing_immutable_evidence_ttl_lease_denied"; {ttl_lease_requested:true, immutable_evidence_retention_requested:true}),
      retention_surface("signature_manifest_hash_chain_expiry_timestamp"; "blocked_signature_manifest_hash_expiry_noop"; "signature_manifest_hash_chain_expiry_timestamp_denied"; {expiry_timestamp_requested:true, hash_attestation_retention_requested:true}),
      retention_surface("notarization_submission_attestation_retention_ledger"; "blocked_notarization_attestation_retention_ledger_noop"; "notarization_submission_attestation_retention_ledger_denied"; {retention_policy_requested:true, ledger_index_retention_requested:true}),
      retention_surface("notarization_ticket_witness_notary_expiry_scheduler"; "blocked_witness_notary_expiry_scheduler_noop"; "notarization_ticket_witness_notary_expiry_scheduler_denied"; {expiry_scheduler_requested:true, expiry_timer_requested:true, expiry_ack_requested:true, witness_notary_expiry_requested:true}),
      retention_surface("stapling_tombstone_garbage_collection_queue"; "blocked_stapling_tombstone_gc_queue_noop"; "stapling_tombstone_garbage_collection_queue_denied"; {garbage_collection_queue_requested:true, tombstone_gc_requested:true}),
      retention_surface("installer_replacement_evidence_garbage_collection_scan"; "blocked_installer_replacement_gc_scan_noop"; "installer_replacement_evidence_garbage_collection_scan_denied"; {garbage_collection_scan_requested:true, garbage_collection_candidate_requested:true}),
      retention_surface("provenance_immutable_evidence_archive"; "blocked_provenance_immutable_evidence_archive_noop"; "provenance_immutable_evidence_archive_denied"; {archive_requested:true, immutable_evidence_retention_requested:true}),
      retention_surface("sbom_evidence_compaction"; "blocked_sbom_evidence_compaction_noop"; "sbom_evidence_compaction_denied"; {compaction_requested:true, audit_evidence_retention_requested:true}),
      retention_surface("release_asset_cancelled_query_retention"; "blocked_release_asset_query_retention_noop"; "release_asset_cancelled_query_retention_denied"; {retention_policy_requested:true, audit_evidence_retention_requested:true}),
      retention_surface("cdn_observability_expiry_ack"; "blocked_cdn_observability_expiry_ack_noop"; "cdn_observability_expiry_ack_denied"; {expiry_ack_requested:true, status_evidence_expiry_requested:true}),
      retention_surface("package_registry_replacement_status_gc_decision"; "blocked_package_registry_status_gc_decision_noop"; "package_registry_replacement_status_gc_decision_denied"; {garbage_collection_decision_requested:true, garbage_collection_candidate_requested:true}),
      retention_surface("dashboard_endpoint_hash_status_retention"; "blocked_dashboard_hash_status_retention_noop"; "dashboard_endpoint_hash_status_retention_denied"; {retention_policy_requested:true, delivery_evidence_retention_requested:true}),
      retention_surface("external_telegram_retention_delivery"; "blocked_external_telegram_retention_delivery_noop"; "external_telegram_retention_delivery_denied"; {external_telegram_retention_requested:true, delivery_evidence_retention_requested:true}),
      retention_surface("release_publication_authority_retention"; "blocked_release_publication_retention_authority_noop"; "release_publication_authority_retention_denied"; {release_publication_retention_authority_requested:true, retention_policy_requested:true}),
      retention_surface("activation_live_install_garbage_collection_evidence"; "blocked_activation_live_install_gc_noop"; "activation_live_install_garbage_collection_evidence_denied"; {activation_retention_authority_requested:true, live_install_gc_evidence_requested:true}),
      retention_surface("install_restart_active_binary_retention_gc_path"; "blocked_install_restart_active_binary_retention_gc_noop"; "install_restart_active_binary_retention_gc_path_denied"; {live_install_gc_evidence_requested:true, garbage_collection_queue_requested:true, garbage_collection_decision_requested:true, delete_marker_gc_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denial_gate" \
    --arg source_artifact_distribution_signing_notarization_receipt_audit_evidence_report_sha256 "$source_artifact_distribution_signing_notarization_receipt_audit_evidence_report_sha256" \
    --arg artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_contract_hash_sha256 "$artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_contract_hash_sha256" \
    --arg artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_policy_hash_sha256 "$artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_AUDIT_EVIDENCE_JSON" \
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
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_schema_version:"operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denial_v1",
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_mode:"denied_signing_receipt_audit_evidence_cannot_be_retained_expired_garbage_collected_archived_compacted_promoted_or_used_for_authority_or_live_install",
        source_artifact_distribution_signing_notarization_receipt_audit_evidence_gate:$source.gate,
        source_artifact_distribution_signing_notarization_receipt_audit_evidence_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_audit_evidence_denial_ready,
        source_artifact_distribution_signing_notarization_receipt_audit_evidence_report_sha256:$source_artifact_distribution_signing_notarization_receipt_audit_evidence_report_sha256,
        source_artifact_distribution_signing_notarization_receipt_audit_evidence_contract_hash_sha256:$source.artifact_distribution_signing_notarization_receipt_audit_evidence_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_contract_hash_sha256:$artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_policy_hash_sha256:$artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denial_ready:true,
        source_artifact_distribution_signing_notarization_receipt_audit_evidence_surface_count:$source.artifact_distribution_signing_notarization_receipt_audit_evidence_surface_count,
        source_artifact_distribution_signing_notarization_receipt_audit_evidence_attempt_count:$source.artifact_distribution_signing_notarization_receipt_audit_evidence_attempt_count,
        source_artifact_distribution_signing_notarization_receipt_audit_evidence_denied_count:$source.artifact_distribution_signing_notarization_receipt_audit_evidence_denied_count,
        source_artifact_distribution_signing_notarization_receipt_audit_evidence_recorded_count:$source.artifact_distribution_signing_notarization_receipt_audit_evidence_recorded_count,
        source_artifact_distribution_signing_notarization_receipt_audit_trail_recorded_count:$source.artifact_distribution_signing_notarization_receipt_audit_trail_recorded_count,
        source_artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded_count:$source.artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded_count,
        source_artifact_distribution_signing_notarization_receipt_ledger_recorded_count:$source.artifact_distribution_signing_notarization_receipt_ledger_recorded_count,
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surface_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempt_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denied_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces:$surfaces,
        denied_by_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection:[
          "source_artifact_distribution_signing_notarization_receipt_audit_evidence_report_required",
          "signing_receipt_retention_policy_denied",
          "signing_receipt_ttl_lease_denied",
          "signing_receipt_expiry_timestamp_denied",
          "signing_receipt_expiry_scheduler_timer_ack_denied",
          "signing_receipt_garbage_collection_queue_denied",
          "signing_receipt_garbage_collection_scan_denied",
          "signing_receipt_garbage_collection_candidate_decision_denied",
          "signing_receipt_tombstone_delete_marker_gc_denied",
          "signing_receipt_archive_denied",
          "signing_receipt_compaction_denied",
          "signing_receipt_audit_evidence_retention_denied",
          "signing_receipt_immutable_evidence_retention_denied",
          "signing_receipt_hash_attestation_retention_denied",
          "signing_receipt_witness_notary_expiry_denied",
          "external_telegram_signing_receipt_retention_delivery_denied",
          "release_publication_retention_authority_denied",
          "activation_live_install_gc_evidence_denied",
          "install_restart_active_binary_retention_gc_denied",
          "memory_provider_kg_secret_external_send_from_retention_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_export_query_observability_denial_gate",
            status:"allowed_report_only_next_slice",
            records_retention:false,
            records_expiry:false,
            records_garbage_collection:false,
            records_archive:false,
            records_compaction:false,
            registers_export:false,
            registers_query:false,
            records_observability:false,
            records_operator_acceptance:false,
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
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_allowed_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_recorded_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_persisted_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_materialized_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_retention_policy_recorded_count",
        "artifact_distribution_signing_notarization_receipt_retention_policy_persisted_count",
        "artifact_distribution_signing_notarization_receipt_ttl_lease_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ttl_lease_persisted_count",
        "artifact_distribution_signing_notarization_receipt_expiry_timestamp_recorded_count",
        "artifact_distribution_signing_notarization_receipt_expiry_scheduler_recorded_count",
        "artifact_distribution_signing_notarization_receipt_expiry_timer_started_count",
        "artifact_distribution_signing_notarization_receipt_expiry_ack_recorded_count",
        "artifact_distribution_signing_notarization_receipt_expiry_state_persisted_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_queue_recorded_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_scan_performed_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_candidate_recorded_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_decision_recorded_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_state_persisted_count",
        "artifact_distribution_signing_notarization_receipt_tombstone_gc_recorded_count",
        "artifact_distribution_signing_notarization_receipt_delete_marker_gc_recorded_count",
        "artifact_distribution_signing_notarization_receipt_archive_recorded_count",
        "artifact_distribution_signing_notarization_receipt_compaction_recorded_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_retention_recorded_count",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_retention_recorded_count",
        "artifact_distribution_signing_notarization_receipt_hash_attestation_retention_recorded_count",
        "artifact_distribution_signing_notarization_receipt_witness_notary_expiry_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ledger_index_retention_recorded_count",
        "artifact_distribution_signing_notarization_receipt_delivery_evidence_retention_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_evidence_expiry_recorded_count",
        "release_publication_authority_from_signing_receipt_retention_derived_count",
        "activation_authority_from_signing_receipt_retention_derived_count",
        "download_link_from_signing_receipt_retention_rendered_count",
        "install_command_from_signing_receipt_retention_rendered_count",
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
      + false_object([
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_recorded",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_persisted",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_materialized",
        "artifact_distribution_signing_notarization_receipt_retention_policy_recorded",
        "artifact_distribution_signing_notarization_receipt_expiry_recorded",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_recorded",
        "artifact_distribution_signing_notarization_receipt_archive_recorded",
        "artifact_distribution_signing_notarization_receipt_compaction_recorded",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_retention_recorded",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_retention_recorded",
        "public_release_claimed",
        "public_ga_claimed",
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
          "retention_policy_recorded",
          "retention_policy_persisted",
          "ttl_lease_recorded",
          "ttl_lease_persisted",
          "expiry_timestamp_recorded",
          "expiry_scheduler_recorded",
          "expiry_timer_started",
          "expiry_ack_recorded",
          "expiry_state_persisted",
          "garbage_collection_queue_recorded",
          "garbage_collection_scan_performed",
          "garbage_collection_candidate_recorded",
          "garbage_collection_decision_recorded",
          "garbage_collection_state_persisted",
          "tombstone_gc_recorded",
          "delete_marker_gc_recorded",
          "archive_recorded",
          "compaction_recorded",
          "audit_evidence_retention_recorded",
          "immutable_evidence_retention_recorded",
          "hash_attestation_retention_recorded",
          "witness_notary_expiry_recorded",
          "ledger_index_retention_recorded",
          "delivery_evidence_retention_recorded",
          "status_evidence_expiry_recorded",
          "result_receipt_from_retention_recorded",
          "result_receipt_from_retention_persisted",
          "operator_approval_from_retention_derived",
          "release_publication_authority_from_retention_derived",
          "activation_authority_from_retention_derived",
          "download_link_from_retention_rendered",
          "install_command_from_retention_rendered",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denial_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_audit_evidence_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_audit_evidence_surface_count == 18
  and $report.source_artifact_distribution_signing_notarization_receipt_audit_evidence_denied_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surface_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempt_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denied_count == 18
  and zero_fields($report; [
    "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_allowed_count",
    "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted_count",
    "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_recorded_count",
    "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_persisted_count",
    "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_materialized_count",
    "artifact_distribution_signing_notarization_receipt_retention_policy_recorded_count",
    "artifact_distribution_signing_notarization_receipt_ttl_lease_recorded_count",
    "artifact_distribution_signing_notarization_receipt_expiry_timestamp_recorded_count",
    "artifact_distribution_signing_notarization_receipt_expiry_scheduler_recorded_count",
    "artifact_distribution_signing_notarization_receipt_expiry_timer_started_count",
    "artifact_distribution_signing_notarization_receipt_expiry_ack_recorded_count",
    "artifact_distribution_signing_notarization_receipt_garbage_collection_queue_recorded_count",
    "artifact_distribution_signing_notarization_receipt_garbage_collection_scan_performed_count",
    "artifact_distribution_signing_notarization_receipt_garbage_collection_decision_recorded_count",
    "artifact_distribution_signing_notarization_receipt_archive_recorded_count",
    "artifact_distribution_signing_notarization_receipt_compaction_recorded_count",
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
  and false_fields($report; [
    "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted",
    "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_recorded",
    "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_persisted",
    "artifact_distribution_signing_notarization_receipt_retention_policy_recorded",
    "artifact_distribution_signing_notarization_receipt_expiry_recorded",
    "artifact_distribution_signing_notarization_receipt_garbage_collection_recorded",
    "artifact_distribution_signing_notarization_receipt_archive_recorded",
    "artifact_distribution_signing_notarization_receipt_compaction_recorded",
    "public_release_claimed",
    "public_ga_claimed",
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
  and ($report.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces | length) == 18
  and ($report.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces | all(
    .artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempted == true
    and .artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_allowed == false
    and .artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_noop_confirmed == true
    and .retention_policy_recorded == false
    and .ttl_lease_recorded == false
    and .expiry_timestamp_recorded == false
    and .expiry_scheduler_recorded == false
    and .expiry_timer_started == false
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
  and ([.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces[] | select(.retention_policy_requested == true)] | length) == 5
  and ([.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces[] | select(.ttl_lease_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces[] | select(.expiry_scheduler_requested == true and .expiry_timer_requested == true and .expiry_ack_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces[] | select(.garbage_collection_queue_requested == true)] | length) == 2
  and ([.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces[] | select(.garbage_collection_scan_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces[] | select(.archive_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces[] | select(.compaction_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces[] | select(.external_telegram_retention_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces[] | select(.live_install_gc_evidence_requested == true)] | length) == 2
  and ($report.allowed_next_actions | any(
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
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt retention/expiry/garbage-collection denial gate passed"
