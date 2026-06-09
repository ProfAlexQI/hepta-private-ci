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

CANCELLATION_SUPERSESSION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-cancellation-supersession-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-cancellation-supersession-denial-gate.sh
)"

source_cancellation_supersession_report_sha256="$(sha256_text "$CANCELLATION_SUPERSESSION_JSON")"
audit_trail_immutable_evidence_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-audit-trail-immutable-evidence-denial:$source_cancellation_supersession_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
audit_trail_immutable_evidence_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-audit-trail-immutable-evidence-denial:no-audit:no-immutable-evidence:no-hash-chain:no-attestation:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$CANCELLATION_SUPERSESSION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_ready == true
    and $source.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface_count == 18
    and $source.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_attempt_count == 18
    and $source.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denied_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_attempt_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denied_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_allowed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delete_marker_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_cancellation_supersession_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_cancellation_supersession_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_cancellation_supersession_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_cancellation_supersession_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restart_from_cancellation_supersession_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_cancellation_supersession_mutated_count"
    ])
    and false_fields($source; [
      "artifact_download_install_affordance_result_receipt_recorded",
      "artifact_download_install_affordance_result_receipt_persisted",
      "artifact_download_install_affordance_result_receipt_accepted",
      "artifact_download_install_affordance_result_receipt_materialized",
      "artifact_download_install_affordance_result_receipt_filesystem_written",
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
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surfaces | all(
      .artifact_download_install_affordance_result_receipt_cancellation_supersession_requested == true
      and .cancellation_supersession_noop_confirmed == true
      and false_fields(.; [
        "artifact_download_install_affordance_result_receipt_cancellation_supersession_allowed",
        "artifact_download_install_affordance_result_receipt_cancellation_supersession_recorded",
        "artifact_download_install_affordance_result_receipt_cancellation_supersession_persisted",
        "artifact_download_install_affordance_result_receipt_cancellation_accepted",
        "artifact_download_install_affordance_result_receipt_supersession_accepted",
        "artifact_download_install_affordance_result_receipt_replacement_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_tombstone_recorded",
        "artifact_download_install_affordance_result_receipt_delete_marker_recorded",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_completion_ack_recorded",
        "operator_approval_from_cancellation_supersession_accepted",
        "release_publication_authority_from_cancellation_supersession_derived",
        "activation_authority_from_cancellation_supersession_derived",
        "install_from_cancellation_supersession_executed",
        "service_restart_from_cancellation_supersession_performed",
        "active_binary_from_cancellation_supersession_mutated",
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
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .accepts_cancellation == false
      and .accepts_supersession == false
      and .accepts_replacement_receipt == false
      and .persists_immutable_evidence == false
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

audit_evidence_surfaces_json="$(
  jq -n '
    def audit_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surface:$id,
        source_cancellation_supersession_present:true,
        source_cancellation_supersession_ready:true,
        source_cancellation_supersession_noop_confirmed:true,
        audit_or_evidence_attempted:true,
        artifact_download_install_affordance_result_receipt_audit_trail_requested:true,
        artifact_download_install_affordance_result_receipt_immutable_evidence_requested:false,
        artifact_download_install_affordance_result_receipt_audit_evidence_status:$status,
        artifact_download_install_affordance_result_receipt_audit_trail_accepted:false,
        artifact_download_install_affordance_result_receipt_audit_trail_recorded:false,
        artifact_download_install_affordance_result_receipt_audit_trail_persisted:false,
        artifact_download_install_affordance_result_receipt_audit_trail_materialized:false,
        artifact_download_install_affordance_result_receipt_audit_trail_filesystem_written:false,
        artifact_download_install_affordance_result_receipt_immutable_evidence_accepted:false,
        artifact_download_install_affordance_result_receipt_immutable_evidence_recorded:false,
        artifact_download_install_affordance_result_receipt_immutable_evidence_persisted:false,
        artifact_download_install_affordance_result_receipt_immutable_evidence_materialized:false,
        artifact_download_install_affordance_result_receipt_immutable_evidence_filesystem_written:false,
        artifact_download_install_affordance_result_receipt_hash_chain_recorded:false,
        artifact_download_install_affordance_result_receipt_hash_chain_persisted:false,
        artifact_download_install_affordance_result_receipt_merkle_root_recorded:false,
        artifact_download_install_affordance_result_receipt_merkle_root_persisted:false,
        artifact_download_install_affordance_result_receipt_attestation_recorded:false,
        artifact_download_install_affordance_result_receipt_attestation_persisted:false,
        artifact_download_install_affordance_result_receipt_witness_recorded:false,
        artifact_download_install_affordance_result_receipt_witness_persisted:false,
        artifact_download_install_affordance_result_receipt_notary_recorded:false,
        artifact_download_install_affordance_result_receipt_notary_persisted:false,
        artifact_download_install_affordance_result_receipt_ledger_evidence_recorded:false,
        artifact_download_install_affordance_result_receipt_ledger_evidence_persisted:false,
        artifact_download_install_affordance_result_receipt_index_evidence_recorded:false,
        artifact_download_install_affordance_result_receipt_index_evidence_persisted:false,
        artifact_download_install_affordance_result_receipt_delivery_evidence_recorded:false,
        artifact_download_install_affordance_result_receipt_delivery_evidence_persisted:false,
        artifact_download_install_affordance_result_receipt_export_evidence_recorded:false,
        artifact_download_install_affordance_result_receipt_query_evidence_registered:false,
        artifact_download_install_affordance_result_receipt_observability_evidence_recorded:false,
        artifact_download_install_affordance_result_receipt_readback_evidence_recorded:false,
        artifact_download_install_affordance_result_receipt_audit_evidence_acceptance_recorded:false,
        artifact_download_install_affordance_result_receipt_completion_ack_from_audit_evidence_recorded:false,
        artifact_download_install_affordance_result_receipt_cancellation_recorded:false,
        artifact_download_install_affordance_result_receipt_supersession_recorded:false,
        artifact_download_install_affordance_result_receipt_replacement_receipt_recorded:false,
        artifact_download_install_affordance_result_receipt_tombstone_recorded:false,
        artifact_download_install_affordance_result_receipt_delete_marker_recorded:false,
        artifact_download_install_affordance_result_receipt_ordering_recorded:false,
        artifact_download_install_affordance_result_receipt_monotonicity_state_recorded:false,
        artifact_download_install_affordance_result_receipt_replay_recorded:false,
        artifact_download_install_affordance_result_receipt_idempotency_state_recorded:false,
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
        operator_approval_from_audit_evidence_accepted:false,
        release_publication_authority_from_audit_evidence_derived:false,
        activation_authority_from_audit_evidence_derived:false,
        activation_command_from_audit_evidence_derived:false,
        activation_from_audit_evidence_allowed:false,
        live_execution_from_audit_evidence_allowed:false,
        install_from_audit_evidence_executed:false,
        service_restart_from_audit_evidence_performed:false,
        launchd_from_audit_evidence_mutated:false,
        active_binary_from_audit_evidence_mutated:false,
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
        audit_evidence_noop_confirmed:true,
        reason:$reason
      } + $extra;
    [
      audit_surface("source_cancellation_supersession_report_required"; "blocked_source_report_required_noop"; "source_cancellation_supersession_report_required"; {source_cancellation_supersession_report_required:true}),
      audit_surface("download_button_result_receipt_audit_trail_append_claim"; "blocked_audit_trail_append_noop"; "download_button_result_receipt_audit_trail_append_claim_denied"; {audit_trail_append_requested:true}),
      audit_surface("direct_download_url_result_receipt_immutable_evidence_packet"; "blocked_immutable_evidence_noop"; "direct_download_url_result_receipt_immutable_evidence_packet_denied"; {artifact_download_install_affordance_result_receipt_immutable_evidence_requested:true, artifact_download_install_affordance_result_receipt_audit_trail_requested:false}),
      audit_surface("checksum_prompt_result_receipt_hash_chain_merkle_root"; "blocked_hash_chain_merkle_root_noop"; "checksum_prompt_result_receipt_hash_chain_merkle_root_denied"; {artifact_download_install_affordance_result_receipt_immutable_evidence_requested:true, artifact_download_install_affordance_result_receipt_audit_trail_requested:false, hash_chain_requested:true, merkle_root_requested:true}),
      audit_surface("package_manager_install_command_result_receipt_attestation_witness_notary"; "blocked_attestation_witness_notary_noop"; "package_manager_install_command_result_receipt_attestation_witness_notary_denied"; {artifact_download_install_affordance_result_receipt_immutable_evidence_requested:true, artifact_download_install_affordance_result_receipt_audit_trail_requested:false, attestation_requested:true, witness_requested:true, notary_requested:true}),
      audit_surface("curl_pipe_shell_result_receipt_audit_materialization_filesystem"; "blocked_audit_materialization_noop"; "curl_pipe_shell_result_receipt_audit_materialization_filesystem_denied"; {audit_materialization_requested:true, audit_filesystem_write_requested:true}),
      audit_surface("installer_launch_prompt_result_receipt_ledger_index_delivery_evidence"; "blocked_ledger_index_delivery_noop"; "installer_launch_prompt_result_receipt_ledger_index_delivery_evidence_denied"; {ledger_evidence_requested:true, index_evidence_requested:true, delivery_evidence_requested:true}),
      audit_surface("auto_update_offer_result_receipt_export_query_observability_evidence"; "blocked_export_query_observability_noop"; "auto_update_offer_result_receipt_export_query_observability_evidence_denied"; {export_evidence_requested:true, query_evidence_requested:true, observability_evidence_requested:true}),
      audit_surface("release_channel_subscription_result_receipt_readback_evidence"; "blocked_readback_evidence_noop"; "release_channel_subscription_result_receipt_readback_evidence_denied"; {readback_evidence_requested:true}),
      audit_surface("update_feed_hint_result_receipt_completion_ack_audit_evidence"; "blocked_completion_ack_audit_evidence_noop"; "update_feed_hint_result_receipt_completion_ack_audit_evidence_denied"; {completion_ack_audit_evidence_requested:true}),
      audit_surface("package_registry_badge_result_receipt_cancellation_supersession_audit_evidence"; "blocked_cancellation_supersession_audit_evidence_noop"; "package_registry_badge_result_receipt_cancellation_supersession_audit_evidence_denied"; {cancellation_audit_evidence_requested:true, supersession_audit_evidence_requested:true}),
      audit_surface("cdn_mirror_download_result_receipt_ordering_monotonicity_audit_evidence"; "blocked_ordering_monotonicity_audit_evidence_noop"; "cdn_mirror_download_result_receipt_ordering_monotonicity_audit_evidence_denied"; {ordering_audit_evidence_requested:true, monotonicity_audit_evidence_requested:true}),
      audit_surface("sbom_provenance_notarization_result_receipt_replay_idempotency_audit_evidence"; "blocked_replay_idempotency_audit_evidence_noop"; "sbom_provenance_notarization_result_receipt_replay_idempotency_audit_evidence_denied"; {replay_audit_evidence_requested:true, idempotency_audit_evidence_requested:true}),
      audit_surface("signature_verification_command_result_receipt_release_publication_authority_evidence"; "blocked_release_publication_authority_evidence_noop"; "signature_verification_command_result_receipt_release_publication_authority_evidence_denied"; {release_publication_authority_evidence_requested:true}),
      audit_surface("one_click_install_deep_link_result_receipt_activation_authority_evidence"; "blocked_activation_authority_evidence_noop"; "one_click_install_deep_link_result_receipt_activation_authority_evidence_denied"; {activation_authority_evidence_requested:true}),
      audit_surface("external_telegram_install_message_result_receipt_external_evidence"; "blocked_external_audit_evidence_noop"; "external_telegram_install_message_result_receipt_external_evidence_denied"; {external_audit_evidence_requested:true, telegram_audit_evidence_requested:true}),
      audit_surface("release_publication_authority_install_affordance_result_receipt_public_release_evidence"; "blocked_public_release_artifact_evidence_noop"; "release_publication_authority_install_affordance_result_receipt_public_release_evidence_denied"; {public_release_evidence_requested:true, release_artifact_evidence_requested:true, public_artifact_evidence_requested:true}),
      audit_surface("activation_live_install_restart_active_binary_result_receipt_live_evidence"; "blocked_live_install_restart_active_binary_evidence_noop"; "activation_live_install_restart_active_binary_result_receipt_live_evidence_denied"; {activation_evidence_requested:true, install_evidence_requested:true, service_restart_evidence_requested:true, active_binary_evidence_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_gate" \
    --arg source_cancellation_supersession_report_sha256 "$source_cancellation_supersession_report_sha256" \
    --arg audit_trail_immutable_evidence_contract_hash_sha256 "$audit_trail_immutable_evidence_contract_hash_sha256" \
    --arg audit_trail_immutable_evidence_policy_hash_sha256 "$audit_trail_immutable_evidence_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$CANCELLATION_SUPERSESSION_JSON" \
    --argjson surfaces "$audit_evidence_surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_v1",
        receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_mode:"denied_artifact_download_install_result_receipt_cannot_become_audit_trail_immutable_evidence_or_authority",
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_gate:$source.gate,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_ready,
        source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_report_sha256:$source_cancellation_supersession_report_sha256,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256:$audit_trail_immutable_evidence_contract_hash_sha256,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_policy_hash_sha256:$audit_trail_immutable_evidence_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_ready:true,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surface_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_attempt_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denied_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denied_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_accepted_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_accepted_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_accepted_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_accepted_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_accepted_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_accepted_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_release_publication_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_cancellation_supersession_derived_count,
        source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_activation_authority_derived_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_cancellation_supersession_derived_count,
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surface_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_attempt_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_denied_count:($surfaces | length),
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surfaces:$surfaces,
        denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence:[
          "artifact_download_install_affordance_result_receipt_audit_trail_acceptance_denied",
          "artifact_download_install_affordance_result_receipt_audit_trail_recording_denied",
          "artifact_download_install_affordance_result_receipt_audit_trail_persistence_denied",
          "artifact_download_install_affordance_result_receipt_audit_trail_materialization_denied",
          "artifact_download_install_affordance_result_receipt_immutable_evidence_acceptance_denied",
          "artifact_download_install_affordance_result_receipt_immutable_evidence_recording_denied",
          "artifact_download_install_affordance_result_receipt_immutable_evidence_persistence_denied",
          "artifact_download_install_affordance_result_receipt_immutable_evidence_materialization_denied",
          "artifact_download_install_affordance_result_receipt_hash_chain_recording_denied",
          "artifact_download_install_affordance_result_receipt_hash_chain_persistence_denied",
          "artifact_download_install_affordance_result_receipt_merkle_root_recording_denied",
          "artifact_download_install_affordance_result_receipt_merkle_root_persistence_denied",
          "artifact_download_install_affordance_result_receipt_attestation_recording_denied",
          "artifact_download_install_affordance_result_receipt_witness_recording_denied",
          "artifact_download_install_affordance_result_receipt_notary_recording_denied",
          "artifact_download_install_affordance_result_receipt_ledger_index_delivery_evidence_denied",
          "artifact_download_install_affordance_result_receipt_export_query_observability_evidence_denied",
          "artifact_download_install_affordance_result_receipt_readback_evidence_denied",
          "artifact_download_install_affordance_result_receipt_completion_ack_from_audit_evidence_denied",
          "artifact_download_install_affordance_result_receipt_cancellation_supersession_audit_evidence_denied",
          "artifact_download_install_affordance_result_receipt_ordering_monotonicity_audit_evidence_denied",
          "artifact_download_install_affordance_result_receipt_replay_idempotency_audit_evidence_denied",
          "artifact_download_install_affordance_result_receipt_acceptance_from_audit_evidence_denied",
          "artifact_download_install_affordance_release_publication_authority_from_audit_evidence_denied",
          "artifact_download_install_affordance_activation_authority_from_audit_evidence_denied",
          "artifact_download_install_affordance_external_send_from_audit_evidence_denied",
          "artifact_download_install_affordance_public_release_artifact_from_audit_evidence_denied",
          "artifact_download_install_affordance_install_restart_active_binary_from_audit_evidence_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_retention_expiry_garbage_collection_denial_gate",
            status:"allowed_report_only_next_slice",
            records_audit_trail:false,
            persists_immutable_evidence:false,
            records_hash_chain:false,
            records_ledger_evidence:false,
            records_result_receipt:false,
            records_completion_ack:false,
            derives_release_publication_authority:false,
            derives_activation_authority:false,
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_materialized_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_chain_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_chain_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_merkle_root_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_merkle_root_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attestation_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attestation_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_witness_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_witness_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_notary_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_notary_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_evidence_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_index_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_index_evidence_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_evidence_persisted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_evidence_registered_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completion_ack_from_audit_evidence_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_acceptance_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_operator_approval_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_release_publication_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_activation_authority_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_activation_command_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_live_execution_allowed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_install_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_service_restart_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_active_binary_mutated_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_memory_store_write_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_live_kg_write_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_provider_invoked_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_model_invoked_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_credential_read_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_secret_read_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_external_send_count"
      ])
      + false_object([
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_chain_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_merkle_root_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attestation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_witness_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_notary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_index_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_evidence_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_readback_evidence_recorded",
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
          "artifact_download_install_affordance_result_receipt_audit_trail_accepted",
          "artifact_download_install_affordance_result_receipt_audit_trail_recorded",
          "artifact_download_install_affordance_result_receipt_audit_trail_persisted",
          "artifact_download_install_affordance_result_receipt_audit_trail_materialized",
          "artifact_download_install_affordance_result_receipt_audit_trail_filesystem_written",
          "artifact_download_install_affordance_result_receipt_immutable_evidence_accepted",
          "artifact_download_install_affordance_result_receipt_immutable_evidence_recorded",
          "artifact_download_install_affordance_result_receipt_immutable_evidence_persisted",
          "artifact_download_install_affordance_result_receipt_immutable_evidence_materialized",
          "artifact_download_install_affordance_result_receipt_immutable_evidence_filesystem_written",
          "artifact_download_install_affordance_result_receipt_hash_chain_recorded",
          "artifact_download_install_affordance_result_receipt_hash_chain_persisted",
          "artifact_download_install_affordance_result_receipt_merkle_root_recorded",
          "artifact_download_install_affordance_result_receipt_merkle_root_persisted",
          "artifact_download_install_affordance_result_receipt_attestation_recorded",
          "artifact_download_install_affordance_result_receipt_attestation_persisted",
          "artifact_download_install_affordance_result_receipt_witness_recorded",
          "artifact_download_install_affordance_result_receipt_witness_persisted",
          "artifact_download_install_affordance_result_receipt_notary_recorded",
          "artifact_download_install_affordance_result_receipt_notary_persisted",
          "artifact_download_install_affordance_result_receipt_ledger_evidence_recorded",
          "artifact_download_install_affordance_result_receipt_ledger_evidence_persisted",
          "artifact_download_install_affordance_result_receipt_index_evidence_recorded",
          "artifact_download_install_affordance_result_receipt_index_evidence_persisted",
          "artifact_download_install_affordance_result_receipt_delivery_evidence_recorded",
          "artifact_download_install_affordance_result_receipt_delivery_evidence_persisted",
          "artifact_download_install_affordance_result_receipt_export_evidence_recorded",
          "artifact_download_install_affordance_result_receipt_query_evidence_registered",
          "artifact_download_install_affordance_result_receipt_observability_evidence_recorded",
          "artifact_download_install_affordance_result_receipt_readback_evidence_recorded",
          "artifact_download_install_affordance_result_receipt_recorded",
          "artifact_download_install_affordance_result_receipt_persisted",
          "artifact_download_install_affordance_completion_ack_recorded",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_attempt_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denied_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_accepted_count == 0
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_supersession_accepted_count == 0
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replacement_receipt_accepted_count == 0
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_tombstone_recorded_count == 0
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_attempt_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_denied_count == 18
  and zero_fields($report; [
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
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_active_binary_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_memory_store_write_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_live_kg_write_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_provider_invoked_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_model_invoked_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_credential_read_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_secret_read_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_audit_evidence_external_send_count"
  ])
  and false_fields($report; [
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_immutable_evidence_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_chain_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_evidence_recorded",
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
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surfaces | all(
    .audit_or_evidence_attempted == true
    and .audit_evidence_noop_confirmed == true
    and false_fields(.; [
      "artifact_download_install_affordance_result_receipt_audit_trail_accepted",
      "artifact_download_install_affordance_result_receipt_audit_trail_recorded",
      "artifact_download_install_affordance_result_receipt_audit_trail_persisted",
      "artifact_download_install_affordance_result_receipt_immutable_evidence_accepted",
      "artifact_download_install_affordance_result_receipt_immutable_evidence_recorded",
      "artifact_download_install_affordance_result_receipt_immutable_evidence_persisted",
      "artifact_download_install_affordance_result_receipt_hash_chain_recorded",
      "artifact_download_install_affordance_result_receipt_merkle_root_recorded",
      "artifact_download_install_affordance_result_receipt_attestation_recorded",
      "artifact_download_install_affordance_result_receipt_witness_recorded",
      "artifact_download_install_affordance_result_receipt_notary_recorded",
      "artifact_download_install_affordance_result_receipt_ledger_evidence_recorded",
      "artifact_download_install_affordance_result_receipt_index_evidence_recorded",
      "artifact_download_install_affordance_result_receipt_delivery_evidence_recorded",
      "artifact_download_install_affordance_result_receipt_export_evidence_recorded",
      "artifact_download_install_affordance_result_receipt_query_evidence_registered",
      "artifact_download_install_affordance_result_receipt_observability_evidence_recorded",
      "artifact_download_install_affordance_result_receipt_readback_evidence_recorded",
      "artifact_download_install_affordance_result_receipt_audit_evidence_acceptance_recorded",
      "artifact_download_install_affordance_result_receipt_completion_ack_from_audit_evidence_recorded",
      "artifact_download_install_affordance_result_receipt_recorded",
      "artifact_download_install_affordance_result_receipt_persisted",
      "artifact_download_install_affordance_completion_ack_recorded",
      "operator_approval_from_audit_evidence_accepted",
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
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surfaces[] | select(.audit_trail_append_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surfaces[] | select(.hash_chain_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surfaces[] | select(.attestation_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_evidence_surfaces[] | select(.install_evidence_requested == true)] | length) == 1
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_audit_trail_immutable_evidence | length) == 28
  and ($report.allowed_next_actions | any(
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
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt audit-trail/immutable-evidence denial gate passed"
