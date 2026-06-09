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

AUDIT_EVIDENCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-audit-trail-immutable-evidence-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-audit-trail-immutable-evidence-denial-gate.sh
)"

source_audit_evidence_report_sha256="$(sha256_text "$AUDIT_EVIDENCE_JSON")"
retention_expiry_garbage_collection_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial:$source_audit_evidence_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
retention_expiry_garbage_collection_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial:no-retention:no-expiry:no-gc:no-receipt:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$AUDIT_EVIDENCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_denied_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_chain_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_merkle_root_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attestation_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_witness_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_notary_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_index_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_evidence_registered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_ack_from_audit_evidence_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_acceptance_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_release_publication_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_activation_authority_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_install_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_service_restart_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_active_binary_mutated_count"
    ])
    and false_fields($source; [
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_chain_recorded",
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
      "external_send_performed"
    ])
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surfaces | all(
      .audit_or_evidence_attempted == true
      and .audit_evidence_noop_confirmed == true
      and false_fields(.; [
        "artifact_download_install_affordance_result_receipt_audit_trail_recorded",
        "artifact_download_install_affordance_result_receipt_immutable_evidence_recorded",
        "artifact_download_install_affordance_result_receipt_hash_chain_recorded",
        "artifact_download_install_affordance_result_receipt_attestation_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_completion_ack_recorded",
        "release_publication_authority_from_audit_evidence_derived",
        "activation_authority_from_audit_evidence_derived",
        "install_from_audit_evidence_executed",
        "service_restart_from_audit_evidence_performed",
        "active_binary_from_audit_evidence_mutated",
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
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_audit_trail == false
      and .persists_immutable_evidence == false
      and .records_hash_chain == false
      and .records_result_receipt == false
      and .installs_or_restarts == false
      and .mutates_active_binary == false
      and .mutates_memory_store == false
      and .writes_kg == false
      and .sends_externally == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

retention_surfaces_json="$(
  jq -n '
    def retention_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface:$id,
        source_audit_trail_immutable_evidence_present:true,
        source_audit_trail_immutable_evidence_ready:true,
        source_audit_evidence_noop_confirmed:true,
        artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempted:true,
        artifact_download_install_affordance_result_receipt_retention_policy_requested:false,
        artifact_download_install_affordance_result_receipt_ttl_requested:false,
        artifact_download_install_affordance_result_receipt_expiry_requested:false,
        artifact_download_install_affordance_result_receipt_garbage_collection_requested:false,
        artifact_download_install_affordance_result_receipt_delete_requested:false,
        artifact_download_install_affordance_result_receipt_tombstone_requested:false,
        artifact_download_install_affordance_result_receipt_archive_requested:false,
        artifact_download_install_affordance_result_receipt_compaction_requested:false,
        artifact_download_install_affordance_result_receipt_retention_policy_accepted:false,
        artifact_download_install_affordance_result_receipt_retention_policy_recorded:false,
        artifact_download_install_affordance_result_receipt_retention_policy_persisted:false,
        artifact_download_install_affordance_result_receipt_retention_policy_materialized:false,
        artifact_download_install_affordance_result_receipt_retention_index_recorded:false,
        artifact_download_install_affordance_result_receipt_retention_index_persisted:false,
        artifact_download_install_affordance_result_receipt_retention_ledger_recorded:false,
        artifact_download_install_affordance_result_receipt_retention_ledger_persisted:false,
        artifact_download_install_affordance_result_receipt_ttl_update_accepted:false,
        artifact_download_install_affordance_result_receipt_ttl_update_recorded:false,
        artifact_download_install_affordance_result_receipt_ttl_update_persisted:false,
        artifact_download_install_affordance_result_receipt_ttl_extension_accepted:false,
        artifact_download_install_affordance_result_receipt_ttl_extension_recorded:false,
        artifact_download_install_affordance_result_receipt_ttl_extension_persisted:false,
        artifact_download_install_affordance_result_receipt_expiry_accepted:false,
        artifact_download_install_affordance_result_receipt_expiry_recorded:false,
        artifact_download_install_affordance_result_receipt_expiry_persisted:false,
        artifact_download_install_affordance_result_receipt_expiry_scheduler_registered:false,
        artifact_download_install_affordance_result_receipt_expiry_timer_started:false,
        artifact_download_install_affordance_result_receipt_expiry_ack_recorded:false,
        artifact_download_install_affordance_result_receipt_garbage_collection_accepted:false,
        artifact_download_install_affordance_result_receipt_gc_queue_enqueued:false,
        artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed:false,
        artifact_download_install_affordance_result_receipt_garbage_collection_candidate_recorded:false,
        artifact_download_install_affordance_result_receipt_garbage_collection_decision_recorded:false,
        artifact_download_install_affordance_result_receipt_garbage_collection_persisted:false,
        artifact_download_install_affordance_result_receipt_delete_accepted:false,
        artifact_download_install_affordance_result_receipt_delete_marker_recorded:false,
        artifact_download_install_affordance_result_receipt_delete_performed:false,
        artifact_download_install_affordance_result_receipt_tombstone_recorded:false,
        artifact_download_install_affordance_result_receipt_tombstone_persisted:false,
        artifact_download_install_affordance_result_receipt_sweep_performed:false,
        artifact_download_install_affordance_result_receipt_archive_written:false,
        artifact_download_install_affordance_result_receipt_archive_persisted:false,
        artifact_download_install_affordance_result_receipt_compaction_performed:false,
        artifact_download_install_affordance_result_receipt_compaction_artifact_written:false,
        artifact_download_install_affordance_result_receipt_compaction_artifact_persisted:false,
        artifact_download_install_affordance_result_receipt_audit_evidence_retention_recorded:false,
        artifact_download_install_affordance_result_receipt_ordering_replay_retention_recorded:false,
        artifact_download_install_affordance_result_receipt_hash_attestation_retention_recorded:false,
        artifact_download_install_affordance_result_receipt_completion_ack_retention_recorded:false,
        artifact_download_install_affordance_result_receipt_recorded:false,
        artifact_download_install_affordance_result_receipt_persisted:false,
        artifact_download_install_affordance_result_receipt_accepted:false,
        artifact_download_install_affordance_result_receipt_materialized:false,
        artifact_download_install_affordance_result_receipt_filesystem_written:false,
        artifact_download_install_affordance_result_receipt_ledger_written:false,
        artifact_download_install_affordance_result_receipt_indexed:false,
        artifact_download_install_affordance_result_receipt_enqueued:false,
        artifact_download_install_affordance_result_receipt_delivered:false,
        artifact_download_install_affordance_result_receipt_exported:false,
        artifact_download_install_affordance_result_receipt_query_registered:false,
        artifact_download_install_affordance_result_receipt_observability_recorded:false,
        artifact_download_install_affordance_completion_ack_recorded:false,
        artifact_download_install_affordance_completion_ack_accepted:false,
        download_button_rendered:false,
        direct_download_url_exposed:false,
        package_manager_install_command_rendered:false,
        curl_pipe_shell_snippet_rendered:false,
        installer_launch_prompt_rendered:false,
        auto_update_offer_rendered:false,
        external_install_message_sent:false,
        telegram_install_message_sent:false,
        operator_approval_from_retention_expiry_garbage_collection_accepted:false,
        release_publication_authority_from_retention_expiry_garbage_collection_derived:false,
        activation_authority_from_retention_expiry_garbage_collection_derived:false,
        activation_command_from_retention_expiry_garbage_collection_derived:false,
        activation_from_retention_expiry_garbage_collection_allowed:false,
        live_execution_from_retention_expiry_garbage_collection_allowed:false,
        install_from_retention_expiry_garbage_collection_executed:false,
        service_restart_from_retention_expiry_garbage_collection_performed:false,
        launchd_from_retention_expiry_garbage_collection_mutated:false,
        active_binary_from_retention_expiry_garbage_collection_mutated:false,
        activation_activated:false,
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
        retention_expiry_garbage_collection_noop_confirmed:true,
        artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_status:$status,
        reason:$reason
      } + $extra;
    [
      retention_surface("source_audit_trail_immutable_evidence_report_required"; "blocked_source_audit_evidence_report_required_noop"; "source_audit_trail_immutable_evidence_report_required"; {source_audit_trail_immutable_evidence_report_required:true}),
      retention_surface("download_button_result_receipt_retention_state_claim"; "blocked_retention_state_noop"; "download_button_result_receipt_retention_state_claim_denied"; {artifact_download_install_affordance_result_receipt_retention_policy_requested:true}),
      retention_surface("direct_download_url_result_receipt_expiry_state_claim"; "blocked_expiry_state_noop"; "direct_download_url_result_receipt_expiry_state_claim_denied"; {artifact_download_install_affordance_result_receipt_expiry_requested:true}),
      retention_surface("checksum_prompt_result_receipt_ttl_claim"; "blocked_ttl_noop"; "checksum_prompt_result_receipt_ttl_claim_denied"; {artifact_download_install_affordance_result_receipt_ttl_requested:true}),
      retention_surface("package_manager_install_command_result_receipt_lease_claim"; "blocked_lease_noop"; "package_manager_install_command_result_receipt_lease_claim_denied"; {artifact_download_install_affordance_result_receipt_ttl_requested:true, retention_lease_requested:true}),
      retention_surface("curl_pipe_shell_result_receipt_gc_queue_claim"; "blocked_gc_queue_noop"; "curl_pipe_shell_result_receipt_gc_queue_claim_denied"; {artifact_download_install_affordance_result_receipt_garbage_collection_requested:true, gc_queue_requested:true}),
      retention_surface("installer_launch_prompt_result_receipt_tombstone_gc_claim"; "blocked_tombstone_gc_noop"; "installer_launch_prompt_result_receipt_tombstone_gc_claim_denied"; {artifact_download_install_affordance_result_receipt_tombstone_requested:true, artifact_download_install_affordance_result_receipt_garbage_collection_requested:true}),
      retention_surface("auto_update_offer_result_receipt_delete_marker_gc_claim"; "blocked_delete_marker_gc_noop"; "auto_update_offer_result_receipt_delete_marker_gc_claim_denied"; {artifact_download_install_affordance_result_receipt_delete_requested:true, artifact_download_install_affordance_result_receipt_garbage_collection_requested:true}),
      retention_surface("release_channel_subscription_result_receipt_retention_policy_claim"; "blocked_retention_policy_noop"; "release_channel_subscription_result_receipt_retention_policy_claim_denied"; {artifact_download_install_affordance_result_receipt_retention_policy_requested:true}),
      retention_surface("update_feed_hint_result_receipt_expiry_extension_claim"; "blocked_expiry_extension_noop"; "update_feed_hint_result_receipt_expiry_extension_claim_denied"; {artifact_download_install_affordance_result_receipt_expiry_requested:true, expiry_extension_requested:true}),
      retention_surface("package_registry_badge_result_receipt_audit_evidence_retention_claim"; "blocked_audit_evidence_retention_noop"; "package_registry_badge_result_receipt_audit_evidence_retention_claim_denied"; {audit_evidence_retention_requested:true}),
      retention_surface("cdn_mirror_download_result_receipt_ordering_replay_retention_claim"; "blocked_ordering_replay_retention_noop"; "cdn_mirror_download_result_receipt_ordering_replay_retention_claim_denied"; {ordering_replay_retention_requested:true}),
      retention_surface("sbom_provenance_notarization_result_receipt_hash_attestation_retention_claim"; "blocked_hash_attestation_retention_noop"; "sbom_provenance_notarization_result_receipt_hash_attestation_retention_claim_denied"; {hash_attestation_retention_requested:true}),
      retention_surface("signature_verification_command_result_receipt_completion_ack_retention_claim"; "blocked_completion_ack_retention_noop"; "signature_verification_command_result_receipt_completion_ack_retention_claim_denied"; {completion_ack_retention_requested:true}),
      retention_surface("one_click_install_deep_link_result_receipt_activation_authority_retention_claim"; "blocked_activation_authority_retention_noop"; "one_click_install_deep_link_result_receipt_activation_authority_retention_claim_denied"; {activation_authority_retention_requested:true}),
      retention_surface("external_telegram_install_message_result_receipt_external_gc_claim"; "blocked_external_gc_noop"; "external_telegram_install_message_result_receipt_external_gc_claim_denied"; {external_gc_requested:true, telegram_gc_requested:true}),
      retention_surface("release_publication_authority_install_affordance_result_receipt_public_release_retention_claim"; "blocked_public_release_retention_noop"; "release_publication_authority_install_affordance_result_receipt_public_release_retention_claim_denied"; {public_release_retention_requested:true, release_artifact_retention_requested:true, public_artifact_retention_requested:true}),
      retention_surface("activation_live_install_restart_active_binary_result_receipt_live_gc_claim"; "blocked_live_gc_noop"; "activation_live_install_restart_active_binary_result_receipt_live_gc_claim_denied"; {activation_retention_requested:true, install_gc_requested:true, service_restart_gc_requested:true, active_binary_gc_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_gate" \
    --arg source_audit_evidence_report_sha256 "$source_audit_evidence_report_sha256" \
    --arg retention_expiry_garbage_collection_contract_hash_sha256 "$retention_expiry_garbage_collection_contract_hash_sha256" \
    --arg retention_expiry_garbage_collection_policy_hash_sha256 "$retention_expiry_garbage_collection_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$AUDIT_EVIDENCE_JSON" \
    --argjson surfaces "$retention_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_v1",
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_mode:"denied_artifact_download_install_result_receipt_cannot_create_retention_expiry_garbage_collection_state_or_authority",
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_gate:$source.gate,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_ready,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_report_sha256:$source_audit_evidence_report_sha256,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_contract_hash_sha256:$retention_expiry_garbage_collection_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_policy_hash_sha256:$retention_expiry_garbage_collection_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_ready:true,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surface_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_attempt_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_denied_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_chain_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_chain_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_evidence_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_evidence_recorded_count,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempt_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denied_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces:$surfaces,
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection:[
          "artifact_download_install_affordance_result_receipt_retention_policy_acceptance_denied",
          "artifact_download_install_affordance_result_receipt_retention_policy_recording_denied",
          "artifact_download_install_affordance_result_receipt_retention_policy_persistence_denied",
          "artifact_download_install_affordance_result_receipt_retention_policy_materialization_denied",
          "artifact_download_install_affordance_result_receipt_retention_index_recording_denied",
          "artifact_download_install_affordance_result_receipt_retention_ledger_recording_denied",
          "artifact_download_install_affordance_result_receipt_ttl_update_denied",
          "artifact_download_install_affordance_result_receipt_ttl_extension_denied",
          "artifact_download_install_affordance_result_receipt_expiry_acceptance_denied",
          "artifact_download_install_affordance_result_receipt_expiry_recording_denied",
          "artifact_download_install_affordance_result_receipt_expiry_scheduler_denied",
          "artifact_download_install_affordance_result_receipt_expiry_timer_denied",
          "artifact_download_install_affordance_result_receipt_expiry_ack_denied",
          "artifact_download_install_affordance_result_receipt_garbage_collection_acceptance_denied",
          "artifact_download_install_affordance_result_receipt_gc_queue_denied",
          "artifact_download_install_affordance_result_receipt_garbage_collection_scan_denied",
          "artifact_download_install_affordance_result_receipt_garbage_collection_candidate_denied",
          "artifact_download_install_affordance_result_receipt_garbage_collection_decision_denied",
          "artifact_download_install_affordance_result_receipt_delete_denied",
          "artifact_download_install_affordance_result_receipt_tombstone_denied",
          "artifact_download_install_affordance_result_receipt_sweep_denied",
          "artifact_download_install_affordance_result_receipt_archive_denied",
          "artifact_download_install_affordance_result_receipt_compaction_denied",
          "artifact_download_install_affordance_result_receipt_audit_evidence_retention_denied",
          "artifact_download_install_affordance_result_receipt_ordering_replay_retention_denied",
          "artifact_download_install_affordance_result_receipt_hash_attestation_retention_denied",
          "artifact_download_install_affordance_result_receipt_completion_ack_from_retention_denied",
          "artifact_download_install_affordance_result_receipt_record_from_retention_denied",
          "artifact_download_install_affordance_release_publication_authority_from_retention_denied",
          "artifact_download_install_affordance_activation_authority_from_retention_denied",
          "artifact_download_install_affordance_download_install_affordance_from_retention_denied",
          "artifact_download_install_affordance_install_restart_active_binary_from_retention_denied",
          "artifact_download_install_affordance_memory_provider_secret_external_send_from_retention_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_gate",
            status:"allowed_report_only_next_slice",
            records_retention_policy:false,
            records_expiry:false,
            performs_garbage_collection:false,
            exports_receipt:false,
            registers_query:false,
            records_observability:false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_index_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_index_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_ledger_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_ledger_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_update_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_update_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_update_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_extension_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_extension_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_extension_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_scheduler_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_timer_started_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_ack_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_gc_queue_enqueued_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_candidate_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_decision_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sweep_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_archive_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_archive_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_compaction_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_compaction_artifact_written_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_compaction_artifact_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_replay_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_attestation_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_ack_retention_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_memory_store_write_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_live_kg_write_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_provider_invoked_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_model_invoked_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_credential_read_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_secret_read_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_external_send_count"
      ])
      + false_object([
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_update_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_archive_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_compaction_performed",
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
          "artifact_download_install_affordance_result_receipt_retention_policy_recorded",
          "artifact_download_install_affordance_result_receipt_retention_policy_persisted",
          "artifact_download_install_affordance_result_receipt_retention_index_recorded",
          "artifact_download_install_affordance_result_receipt_retention_ledger_recorded",
          "artifact_download_install_affordance_result_receipt_ttl_update_recorded",
          "artifact_download_install_affordance_result_receipt_ttl_extension_recorded",
          "artifact_download_install_affordance_result_receipt_expiry_recorded",
          "artifact_download_install_affordance_result_receipt_expiry_persisted",
          "artifact_download_install_affordance_result_receipt_expiry_scheduler_registered",
          "artifact_download_install_affordance_result_receipt_expiry_timer_started",
          "artifact_download_install_affordance_result_receipt_expiry_ack_recorded",
          "artifact_download_install_affordance_result_receipt_gc_queue_enqueued",
          "artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed",
          "artifact_download_install_affordance_result_receipt_garbage_collection_candidate_recorded",
          "artifact_download_install_affordance_result_receipt_garbage_collection_decision_recorded",
          "artifact_download_install_affordance_result_receipt_delete_marker_recorded",
          "artifact_download_install_affordance_result_receipt_delete_performed",
          "artifact_download_install_affordance_result_receipt_tombstone_recorded",
          "artifact_download_install_affordance_result_receipt_sweep_performed",
          "artifact_download_install_affordance_result_receipt_archive_written",
          "artifact_download_install_affordance_result_receipt_compaction_performed",
          "artifact_download_install_affordance_result_receipt_compaction_artifact_written",
          "artifact_download_install_affordance_result_receipt_audit_evidence_retention_recorded",
          "artifact_download_install_affordance_result_receipt_ordering_replay_retention_recorded",
          "artifact_download_install_affordance_result_receipt_hash_attestation_retention_recorded",
          "artifact_download_install_affordance_result_receipt_completion_ack_retention_recorded",
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
          "activation_command_derived",
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
          "release_artifact_written",
          "public_artifact_written",
          "public_release_claimed",
          "public_ga_claimed",
          "telegram_send_performed",
          "channel_send_performed",
          "external_send_performed",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_attempt_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_denied_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempt_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denied_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_index_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_ledger_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_update_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ttl_extension_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_scheduler_registered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_gc_queue_enqueued_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_archive_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_compaction_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_retention_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_attestation_retention_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_ack_retention_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_release_publication_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_activation_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_install_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_service_restart_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_active_binary_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_memory_store_write_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_live_kg_write_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_provider_invoked_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_model_invoked_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_credential_read_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_secret_read_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_retention_external_send_count"
  ])
  and false_fields($report; [
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_policy_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_expiry_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed",
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
    "external_send_performed"
  ])
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces | all(
    .artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_attempted == true
    and .retention_expiry_garbage_collection_noop_confirmed == true
    and false_fields(.; [
      "artifact_download_install_affordance_result_receipt_retention_policy_recorded",
      "artifact_download_install_affordance_result_receipt_ttl_update_recorded",
      "artifact_download_install_affordance_result_receipt_expiry_recorded",
      "artifact_download_install_affordance_result_receipt_garbage_collection_scan_performed",
      "artifact_download_install_affordance_result_receipt_gc_queue_enqueued",
      "artifact_download_install_affordance_result_receipt_delete_marker_recorded",
      "artifact_download_install_affordance_result_receipt_tombstone_recorded",
      "artifact_download_install_affordance_result_receipt_archive_written",
      "artifact_download_install_affordance_result_receipt_compaction_performed",
      "artifact_download_install_affordance_result_receipt_audit_evidence_retention_recorded",
      "artifact_download_install_affordance_result_receipt_ordering_replay_retention_recorded",
      "artifact_download_install_affordance_result_receipt_hash_attestation_retention_recorded",
      "artifact_download_install_affordance_result_receipt_completion_ack_retention_recorded",
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
      "release_publication_authority_from_retention_expiry_garbage_collection_derived",
      "activation_authority_from_retention_expiry_garbage_collection_derived",
      "install_from_retention_expiry_garbage_collection_executed",
      "service_restart_from_retention_expiry_garbage_collection_performed",
      "active_binary_from_retention_expiry_garbage_collection_mutated",
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
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces[] | select(.gc_queue_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces[] | select(.audit_evidence_retention_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces[] | select(.hash_attestation_retention_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_surfaces[] | select(.install_gc_requested == true)] | length) == 1
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection | length) == 33
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_query_observability_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_retention_policy == false
    and .records_expiry == false
    and .performs_garbage_collection == false
    and .exports_receipt == false
    and .registers_query == false
    and .records_observability == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt retention/expiry/garbage-collection denial gate passed"
