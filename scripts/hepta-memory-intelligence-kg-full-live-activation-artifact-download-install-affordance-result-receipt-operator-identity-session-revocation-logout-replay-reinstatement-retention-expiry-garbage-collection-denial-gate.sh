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

OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_AUDIT_EVIDENCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-audit-evidence-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-audit-evidence-denial-gate.sh
)"

source_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_report_sha256="$(
  sha256_text "$OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_AUDIT_EVIDENCE_JSON"
)"
operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-retention-expiry-garbage-collection-denial:$source_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-retention-expiry-garbage-collection:no-retention:no-expiry:no-gc:no-archive:no-compaction:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_AUDIT_EVIDENCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denied_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_immutable_evidence_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_immutable_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_immutable_evidence_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_hash_chain_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_merkle_root_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_attestation_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_witness_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_notary_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ledger_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_index_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_delivery_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_query_evidence_registered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_observability_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_readback_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_acceptance_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_release_publication_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_activation_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_install_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_service_restart_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_active_binary_mutated_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_external_send_count"
    ])
    and false_fields($source; [
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_immutable_evidence_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_hash_chain_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ledger_evidence_recorded",
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
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_surfaces | all(
      .artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_attempted == true
      and .operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_noop_confirmed == true
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_audit_trail == false
      and .persists_immutable_evidence == false
      and .records_retention == false
      and .records_expiry == false
      and .records_garbage_collection == false
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

operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces_json="$(
  jq -n '
    def retention_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surface:$id,
        source_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_ready:true,
        artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_attempted:true,
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
        completion_ack_retention_requested:false,
        cancellation_supersession_retention_requested:false,
        ordering_monotonicity_retention_requested:false,
        external_telegram_retention_requested:false,
        release_publication_retention_authority_requested:false,
        activation_retention_authority_requested:false,
        live_install_gc_evidence_requested:false,
        retention_policy_accepted:false,
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
        completion_ack_retention_recorded:false,
        cancellation_supersession_retention_recorded:false,
        ordering_monotonicity_retention_recorded:false,
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
        operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_noop_confirmed:true,
        operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_status:$status,
        reason:$reason
      } + $extra;
    [
      retention_surface("source_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_report_required"; "blocked_source_audit_evidence_required_noop"; "source_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_report_required"; {source_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_report_required:true}),
      retention_surface("download_button_revocation_replay_audit_retention_policy_claim"; "blocked_revocation_replay_audit_retention_noop"; "download_button_revocation_replay_audit_retention_policy_claim_denied"; {retention_policy_requested:true, audit_evidence_retention_requested:true}),
      retention_surface("direct_download_url_logout_replay_immutable_evidence_ttl_lease_claim"; "blocked_logout_replay_immutable_evidence_ttl_noop"; "direct_download_url_logout_replay_immutable_evidence_ttl_lease_claim_denied"; {ttl_lease_requested:true, immutable_evidence_retention_requested:true}),
      retention_surface("checksum_identity_reinstatement_hash_chain_expiry_timestamp_claim"; "blocked_identity_reinstatement_hash_expiry_noop"; "checksum_identity_reinstatement_hash_chain_expiry_timestamp_claim_denied"; {expiry_timestamp_requested:true, hash_attestation_retention_requested:true}),
      retention_surface("package_manager_session_reinstatement_attestation_retention_ledger_claim"; "blocked_session_reinstatement_attestation_retention_noop"; "package_manager_session_reinstatement_attestation_retention_ledger_claim_denied"; {retention_policy_requested:true, hash_attestation_retention_requested:true}),
      retention_surface("curl_pipe_shell_revocation_replay_audit_expiry_scheduler_claim"; "blocked_revocation_replay_expiry_scheduler_noop"; "curl_pipe_shell_revocation_replay_audit_expiry_scheduler_claim_denied"; {expiry_scheduler_requested:true, expiry_timer_requested:true, expiry_ack_requested:true}),
      retention_surface("installer_device_session_reinstatement_ledger_garbage_collection_queue_claim"; "blocked_device_session_reinstatement_gc_queue_noop"; "installer_device_session_reinstatement_ledger_garbage_collection_queue_claim_denied"; {garbage_collection_queue_requested:true, audit_evidence_retention_requested:true}),
      retention_surface("auto_update_session_logout_replay_index_garbage_collection_scan_claim"; "blocked_session_logout_replay_gc_scan_noop"; "auto_update_session_logout_replay_index_garbage_collection_scan_claim_denied"; {garbage_collection_scan_requested:true, garbage_collection_candidate_requested:true}),
      retention_surface("release_channel_identity_revocation_replay_evidence_gc_candidate_claim"; "blocked_identity_revocation_replay_gc_candidate_noop"; "release_channel_identity_revocation_replay_evidence_gc_candidate_claim_denied"; {garbage_collection_candidate_requested:true, garbage_collection_decision_requested:true}),
      retention_surface("update_feed_session_reinstatement_completion_ack_retention_claim"; "blocked_session_reinstatement_completion_retention_noop"; "update_feed_session_reinstatement_completion_ack_retention_claim_denied"; {retention_policy_requested:true, completion_ack_retention_requested:true}),
      retention_surface("package_registry_identity_badge_reinstatement_audit_evidence_archive_claim"; "blocked_identity_reinstatement_archive_noop"; "package_registry_identity_badge_reinstatement_audit_evidence_archive_claim_denied"; {archive_requested:true, audit_evidence_retention_requested:true, cancellation_supersession_retention_requested:true}),
      retention_surface("cdn_session_readback_logout_replay_evidence_compaction_claim"; "blocked_logout_replay_compaction_noop"; "cdn_session_readback_logout_replay_evidence_compaction_claim_denied"; {compaction_requested:true, ordering_monotonicity_retention_requested:true}),
      retention_surface("sbom_identity_dashboard_reinstatement_hash_attestation_retention_claim"; "blocked_identity_reinstatement_hash_attestation_retention_noop"; "sbom_identity_dashboard_reinstatement_hash_attestation_retention_claim_denied"; {retention_policy_requested:true, hash_attestation_retention_requested:true}),
      retention_surface("signature_channel_session_reinstatement_witness_notary_expiry_claim"; "blocked_session_reinstatement_witness_notary_expiry_noop"; "signature_channel_session_reinstatement_witness_notary_expiry_claim_denied"; {expiry_timestamp_requested:true, witness_notary_expiry_requested:true}),
      retention_surface("one_click_identity_approval_reinstatement_activation_evidence_retention_claim"; "blocked_identity_approval_reinstatement_activation_retention_noop"; "one_click_identity_approval_reinstatement_activation_evidence_retention_claim_denied"; {activation_retention_authority_requested:true, audit_evidence_retention_requested:true}),
      retention_surface("external_telegram_identity_session_reinstatement_retention_delivery_claim"; "blocked_external_telegram_retention_delivery_noop"; "external_telegram_identity_session_reinstatement_retention_delivery_claim_denied"; {external_telegram_retention_requested:true, audit_evidence_retention_requested:true}),
      retention_surface("release_publication_authority_replay_reinstatement_retention_authority_claim"; "blocked_release_publication_retention_authority_noop"; "release_publication_authority_replay_reinstatement_retention_authority_claim_denied"; {release_publication_retention_authority_requested:true, retention_policy_requested:true}),
      retention_surface("activation_live_install_restart_active_binary_garbage_collection_evidence_claim"; "blocked_live_install_restart_active_binary_gc_noop"; "activation_live_install_restart_active_binary_garbage_collection_evidence_claim_denied"; {live_install_gc_evidence_requested:true, garbage_collection_queue_requested:true, garbage_collection_decision_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_gate" \
    --arg source_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_report_sha256 "$source_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_report_sha256" \
    --arg operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_contract_hash_sha256 "$operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_contract_hash_sha256" \
    --arg operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_policy_hash_sha256 "$operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_AUDIT_EVIDENCE_JSON" \
    --argjson surfaces "$operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_v1",
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_mode:"denied_audit_evidence_cannot_create_retention_expiry_gc_lifecycle_or_authority",
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_gate:$source.gate,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denial_ready,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_report_sha256:$source_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_report_sha256,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_contract_hash_sha256:$operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_policy_hash_sha256:$operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_ready:true,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_surface_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_attempt_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denied_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_trail_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_immutable_evidence_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_immutable_evidence_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_hash_chain_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_hash_chain_recorded_count,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surface_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_attempt_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denied_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces:$surfaces,
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection:[
          "source_audit_evidence_report_required",
          "retention_policy_denied",
          "ttl_lease_denied",
          "expiry_timestamp_denied",
          "expiry_scheduler_timer_ack_denied",
          "garbage_collection_queue_denied",
          "garbage_collection_scan_denied",
          "garbage_collection_candidate_decision_denied",
          "tombstone_delete_marker_gc_denied",
          "archive_compaction_denied",
          "audit_evidence_retention_denied",
          "immutable_evidence_retention_denied",
          "hash_attestation_retention_denied",
          "witness_notary_expiry_denied",
          "completion_ack_retention_denied",
          "cancellation_supersession_retention_denied",
          "ordering_monotonicity_retention_denied",
          "external_telegram_retention_delivery_denied",
          "release_publication_retention_authority_denied",
          "activation_retention_authority_denied",
          "live_install_restart_active_binary_gc_denied",
          "memory_kg_provider_secret_external_send_from_retention_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_gate",
            status:"allowed_report_only_next_slice",
            records_operator_identity:false,
            records_operator_session:false,
            accepts_replay:false,
            records_reinstatement:false,
            records_audit_trail:false,
            persists_immutable_evidence:false,
            records_retention:false,
            records_expiry:false,
            records_garbage_collection:false,
            records_archive:false,
            records_compaction:false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ttl_lease_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ttl_lease_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_timestamp_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_scheduler_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_timer_started_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_ack_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_state_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_queue_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_scan_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_candidate_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_decision_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_state_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_tombstone_gc_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_delete_marker_gc_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_archive_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_compaction_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_immutable_evidence_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_hash_attestation_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_witness_notary_expiry_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_completion_ack_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_cancellation_supersession_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ordering_monotonicity_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_external_send_count"
      ])
      + false_object([
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_completion_ack_recorded",
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
          "completion_ack_retention_recorded",
          "cancellation_supersession_retention_recorded",
          "ordering_monotonicity_retention_recorded",
          "retention_expiry_garbage_collection_acceptance_recorded",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_audit_evidence_denied_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_attempt_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_denied_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_policy_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ttl_lease_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_ttl_lease_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_timestamp_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_scheduler_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_timer_started_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_ack_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_state_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_queue_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_scan_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_candidate_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_decision_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_state_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_tombstone_gc_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_delete_marker_gc_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_archive_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_compaction_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_acceptance_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_release_publication_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_activation_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_install_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_service_restart_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_active_binary_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_external_send_count"
  ])
  and false_fields($report; [
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_expiry_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_garbage_collection_recorded",
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
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces | all(
    .artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_attempted == true
    and .operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_noop_confirmed == true
    and false_fields(.; [
      "retention_policy_accepted",
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
      "completion_ack_retention_recorded",
      "cancellation_supersession_retention_recorded",
      "ordering_monotonicity_retention_recorded",
      "result_receipt_from_retention_recorded",
      "result_receipt_from_retention_persisted",
      "operator_approval_from_retention_derived",
      "release_publication_authority_from_retention_derived",
      "activation_authority_from_retention_derived",
      "install_from_retention_executed",
      "service_restart_from_retention_performed",
      "active_binary_from_retention_mutated",
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
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces[] | select(.retention_policy_requested == true)] | length) == 5
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces[] | select(.ttl_lease_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces[] | select(.expiry_scheduler_requested == true and .expiry_timer_requested == true and .expiry_ack_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces[] | select(.garbage_collection_queue_requested == true)] | length) == 2
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces[] | select(.garbage_collection_scan_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces[] | select(.archive_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces[] | select(.compaction_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces[] | select(.external_telegram_retention_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_retention_expiry_garbage_collection_surfaces[] | select(.live_install_gc_evidence_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_export_query_observability_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_retention == false
    and .records_expiry == false
    and .records_garbage_collection == false
    and .records_archive == false
    and .records_compaction == false
    and .registers_query == false
    and .records_observability == false
    and .derives_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .sends_externally == false
  ))
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
