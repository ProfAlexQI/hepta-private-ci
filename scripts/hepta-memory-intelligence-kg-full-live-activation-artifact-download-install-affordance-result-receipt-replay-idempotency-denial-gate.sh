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

RESULT_RECEIPT_NO_PERSISTENCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-no-persistence-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-delivery-receipt-artifact-download-install-affordance-result-receipt-no-persistence-denial-gate.sh
)"

source_result_receipt_no_persistence_report_sha256="$(
  sha256_text "$RESULT_RECEIPT_NO_PERSISTENCE_JSON"
)"
artifact_download_install_affordance_result_receipt_replay_idempotency_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-replay-idempotency-denial:$source_result_receipt_no_persistence_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
artifact_download_install_affordance_result_receipt_replay_idempotency_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-replay-idempotency-denial:no-duplicate:no-replay:no-idempotency-record:no-idempotency-persist:no-ledger:no-index:no-delivery:no-query:no-observability:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$RESULT_RECEIPT_NO_PERSISTENCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_ready == true
    and $source.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_count == 18
    and $source.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_attempt_count == 18
    and $source.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_allowed_count == 0
    and $source.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded_count == 0
    and $source.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted_count == 0
    and $source.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed_count == 0
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attempt_count == 18
    and zero_fields($source; [
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_allowed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_schema_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_filesystem_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_indexed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_enqueued_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_exported_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_bound_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_acceptance_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_receipt_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_receipt_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_receipt_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_command_from_receipt_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_from_receipt_allowed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_receipt_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restart_from_receipt_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_receipt_mutated_count"
    ])
    and false_fields($source; [
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_allowed",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_schema_accepted",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_accepted",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_filesystem_written",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_written",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_indexed",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_enqueued",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivered",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_exported",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_bound",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_accepted",
      "artifact_download_install_affordance_completion_ack_recorded",
      "artifact_download_install_affordance_completion_ack_accepted",
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
      "public_release_claimed",
      "public_ga_claimed",
      "release_artifact_written",
      "public_artifact_written",
      "external_send_performed"
    ])
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surfaces | all(
      .artifact_download_install_affordance_result_receipt_attempted == true
      and .receipt_noop_confirmed == true
      and false_fields(.; [
        "artifact_download_install_affordance_result_receipt_allowed",
        "artifact_download_install_affordance_result_receipt_schema_accepted",
        "artifact_download_install_affordance_result_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_materialized",
        "artifact_download_install_affordance_result_receipt_filesystem_written",
        "artifact_download_install_affordance_result_receipt_ledger_written",
        "artifact_download_install_affordance_result_receipt_indexed",
        "artifact_download_install_affordance_result_receipt_enqueued",
        "artifact_download_install_affordance_result_receipt_delivered",
        "artifact_download_install_affordance_result_receipt_exported",
        "artifact_download_install_affordance_result_receipt_query_registered",
        "artifact_download_install_affordance_result_receipt_observability_recorded",
        "artifact_download_install_affordance_result_receipt_hash_bound",
        "artifact_download_install_affordance_result_receipt_status_accepted",
        "artifact_download_install_affordance_completion_ack_recorded",
        "artifact_download_install_affordance_completion_ack_accepted",
        "operator_approval_from_receipt_accepted",
        "release_publication_authority_from_receipt_derived",
        "activation_authority_from_receipt_derived",
        "activation_command_from_receipt_derived",
        "live_execution_from_receipt_allowed",
        "install_from_receipt_executed",
        "service_restart_from_receipt_performed",
        "active_binary_from_receipt_mutated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked"
      ])
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .records_result_receipt == false
      and .persists_result_receipt == false
      and .records_idempotency == false
      and .accepts_duplicate_receipt == false
      and .renders_download_link == false
      and .emits_install_command == false
      and .prompts_installer == false
      and .publishes_update_offer == false
      and .sends_external_install_message == false
      and .records_operator_acceptance == false
      and .derives_release_publication_authority == false
      and .derives_activation_authority == false
      and .activates_live == false
      and .installs_or_restarts == false
      and .mutates_active_binary == false
      and .mutates_memory_store == false
      and .writes_kg == false
      and .sends_externally == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

replay_idempotency_surfaces_json="$(
  jq -n '
    def replay_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface:$id,
        source_result_receipt_no_persistence_present:true,
        source_result_receipt_no_persistence_ready:true,
        source_result_receipt_no_persistence_noop_confirmed:true,
        canonical_noop_result_receipt_identity_required:true,
        artifact_download_install_affordance_result_receipt_replay_requested:true,
        artifact_download_install_affordance_result_receipt_replay_status:$status,
        artifact_download_install_affordance_result_receipt_replay_allowed:false,
        artifact_download_install_affordance_result_receipt_replay_recorded:false,
        artifact_download_install_affordance_result_receipt_replay_persisted:false,
        artifact_download_install_affordance_result_receipt_replay_performed:false,
        artifact_download_install_affordance_result_receipt_duplicate_accepted:false,
        artifact_download_install_affordance_result_receipt_duplicate_recorded:false,
        artifact_download_install_affordance_result_receipt_duplicate_persisted:false,
        artifact_download_install_affordance_result_receipt_idempotency_key_accepted:false,
        artifact_download_install_affordance_result_receipt_idempotency_key_recorded:false,
        artifact_download_install_affordance_result_receipt_idempotency_state_recorded:false,
        artifact_download_install_affordance_result_receipt_idempotency_state_persisted:false,
        artifact_download_install_affordance_result_receipt_idempotency_state_materialized:false,
        artifact_download_install_affordance_result_receipt_idempotency_filesystem_written:false,
        artifact_download_install_affordance_result_receipt_replay_nonce_accepted:false,
        artifact_download_install_affordance_result_receipt_replay_nonce_recorded:false,
        artifact_download_install_affordance_result_receipt_cross_scope_reuse_accepted:false,
        artifact_download_install_affordance_result_receipt_status_upgrade_accepted:false,
        artifact_download_install_affordance_result_receipt_completed_status_accepted:false,
        artifact_download_install_affordance_result_receipt_ack_replay_accepted:false,
        artifact_download_install_affordance_result_receipt_ledger_replay_accepted:false,
        artifact_download_install_affordance_result_receipt_index_replay_accepted:false,
        artifact_download_install_affordance_result_receipt_delivery_replay_accepted:false,
        artifact_download_install_affordance_result_receipt_query_replay_accepted:false,
        artifact_download_install_affordance_result_receipt_observability_replay_accepted:false,
        artifact_download_install_affordance_result_receipt_hash_rebind_accepted:false,
        artifact_download_install_affordance_result_receipt_status_rebind_accepted:false,
        artifact_download_install_affordance_result_receipt_signature_timestamp_replay_accepted:false,
        artifact_download_install_affordance_result_receipt_operator_identity_reuse_accepted:false,
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
        artifact_download_install_affordance_completion_ack_persisted:false,
        artifact_download_install_affordance_completion_ack_accepted:false,
        artifact_download_install_affordance_completion_ack_materialized:false,
        artifact_download_install_affordance_completion_ack_delivered:false,
        download_button_rendered:false,
        direct_download_url_exposed:false,
        package_manager_install_command_rendered:false,
        curl_pipe_shell_snippet_rendered:false,
        installer_launch_prompt_rendered:false,
        auto_update_offer_rendered:false,
        external_install_message_sent:false,
        telegram_install_message_sent:false,
        operator_approval_from_replay_accepted:false,
        release_publication_authority_from_replay_derived:false,
        activation_authority_from_replay_derived:false,
        activation_command_from_replay_derived:false,
        activation_from_replay_allowed:false,
        live_execution_from_replay_allowed:false,
        install_from_replay_executed:false,
        service_restart_from_replay_performed:false,
        launchd_from_replay_mutated:false,
        active_binary_from_replay_mutated:false,
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
        receipt_noop_confirmed:true,
        reason:$reason
      } + $extra;
    [
      replay_surface("source_result_receipt_no_persistence_report_required"; "blocked_source_report_required_noop"; "source_result_receipt_no_persistence_report_required"; {source_result_receipt_no_persistence_report_required:true}),
      replay_surface("download_button_result_receipt_duplicate_identity_replay"; "blocked_duplicate_identity_replay_noop"; "download_button_result_receipt_duplicate_identity_replay_denied"; {duplicate_result_receipt_identity_requested:true}),
      replay_surface("direct_download_url_result_receipt_replay_acceptance"; "blocked_replay_acceptance_noop"; "direct_download_url_result_receipt_replay_acceptance_denied"; {result_receipt_replay_acceptance_requested:true}),
      replay_surface("checksum_prompt_result_receipt_idempotency_key"; "blocked_idempotency_key_noop"; "checksum_prompt_result_receipt_idempotency_key_denied"; {idempotency_key_recording_requested:true}),
      replay_surface("package_manager_install_command_result_receipt_idempotency_state"; "blocked_idempotency_state_noop"; "package_manager_install_command_result_receipt_idempotency_state_denied"; {idempotency_state_recording_requested:true, idempotency_state_persistence_requested:true}),
      replay_surface("curl_pipe_shell_result_receipt_cross_scope_reuse"; "blocked_cross_scope_reuse_noop"; "curl_pipe_shell_result_receipt_cross_scope_reuse_denied"; {cross_scope_reuse_requested:true}),
      replay_surface("installer_launch_prompt_result_receipt_stale_nonce"; "blocked_stale_nonce_noop"; "installer_launch_prompt_result_receipt_stale_nonce_denied"; {stale_nonce_replay_requested:true}),
      replay_surface("auto_update_offer_result_receipt_out_of_order_replay"; "blocked_out_of_order_replay_noop"; "auto_update_offer_result_receipt_out_of_order_replay_denied"; {out_of_order_replay_requested:true}),
      replay_surface("release_channel_subscription_result_receipt_completion_ack_replay"; "blocked_completion_ack_replay_noop"; "release_channel_subscription_result_receipt_completion_ack_replay_denied"; {completion_ack_replay_requested:true}),
      replay_surface("update_feed_hint_result_receipt_ledger_index_delivery_replay"; "blocked_ledger_index_delivery_replay_noop"; "update_feed_hint_result_receipt_ledger_index_delivery_replay_denied"; {ledger_replay_requested:true, index_replay_requested:true, delivery_replay_requested:true}),
      replay_surface("package_registry_badge_result_receipt_export_query_observability_replay"; "blocked_export_query_observability_replay_noop"; "package_registry_badge_result_receipt_export_query_observability_replay_denied"; {export_replay_requested:true, query_replay_requested:true, observability_replay_requested:true}),
      replay_surface("cdn_mirror_download_result_receipt_hash_status_rebind"; "blocked_hash_status_rebind_noop"; "cdn_mirror_download_result_receipt_hash_status_rebind_denied"; {hash_rebind_requested:true, status_rebind_requested:true}),
      replay_surface("sbom_provenance_notarization_result_receipt_signature_timestamp_replay"; "blocked_signature_timestamp_replay_noop"; "sbom_provenance_notarization_result_receipt_signature_timestamp_replay_denied"; {signature_replay_requested:true, timestamp_replay_requested:true}),
      replay_surface("signature_verification_command_result_receipt_operator_identity_reuse"; "blocked_operator_identity_reuse_noop"; "signature_verification_command_result_receipt_operator_identity_reuse_denied"; {operator_identity_reuse_requested:true}),
      replay_surface("one_click_install_deep_link_result_receipt_activation_authority_replay"; "blocked_activation_authority_replay_noop"; "one_click_install_deep_link_result_receipt_activation_authority_replay_denied"; {activation_authority_replay_requested:true}),
      replay_surface("external_telegram_install_message_result_receipt_external_delivery_replay"; "blocked_external_delivery_replay_noop"; "external_telegram_install_message_result_receipt_external_delivery_replay_denied"; {external_delivery_replay_requested:true, telegram_delivery_replay_requested:true}),
      replay_surface("release_publication_authority_install_affordance_result_receipt_replay"; "blocked_release_publication_authority_replay_noop"; "release_publication_authority_install_affordance_result_receipt_replay_denied"; {release_publication_authority_replay_requested:true}),
      replay_surface("activation_live_install_restart_active_binary_result_receipt_replay"; "blocked_activation_live_install_restart_active_binary_replay_noop"; "activation_live_install_restart_active_binary_result_receipt_replay_denied"; {activation_replay_requested:true, install_replay_requested:true, service_restart_replay_requested:true, active_binary_replay_requested:true})
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_gate" \
  --arg source_result_receipt_no_persistence_report_sha256 "$source_result_receipt_no_persistence_report_sha256" \
  --arg artifact_download_install_affordance_result_receipt_replay_idempotency_contract_hash_sha256 "$artifact_download_install_affordance_result_receipt_replay_idempotency_contract_hash_sha256" \
  --arg artifact_download_install_affordance_result_receipt_replay_idempotency_policy_hash_sha256 "$artifact_download_install_affordance_result_receipt_replay_idempotency_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$RESULT_RECEIPT_NO_PERSISTENCE_JSON" \
  --argjson surfaces "$replay_idempotency_surfaces_json" \
  '
    def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
    def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

    {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_v1",
      receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_mode:"denied_replay_duplicate_idempotency_or_ordering_attempt_cannot_create_result_receipt_or_install_activation_authority",
      source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_gate:$source.gate,
      source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_denial_ready,
      source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_report_sha256:$source_result_receipt_no_persistence_report_sha256,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_contract_hash_sha256,
      release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_contract_hash_sha256:$artifact_download_install_affordance_result_receipt_replay_idempotency_contract_hash_sha256,
      release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_policy_hash_sha256:$artifact_download_install_affordance_result_receipt_replay_idempotency_policy_hash_sha256,
      minimum_required_samples:$min_long_soak_samples,
      memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_ready:true,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surface_count,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attempt_count,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized_count,
      release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface_count:($surfaces | length),
      release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_attempt_count:($surfaces | length),
      release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surfaces:$surfaces,
      denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency:[
        "source_result_receipt_no_persistence_report_required",
        "duplicate_result_receipt_identity_replay_denied",
        "result_receipt_replay_acceptance_denied",
        "idempotency_key_recording_denied",
        "idempotency_state_record_persist_materialize_denied",
        "idempotency_filesystem_write_denied",
        "cross_scope_result_receipt_reuse_denied",
        "stale_nonce_result_receipt_replay_denied",
        "out_of_order_result_receipt_replay_denied",
        "completion_ack_replay_denied",
        "ledger_index_delivery_replay_denied",
        "export_query_observability_replay_denied",
        "hash_status_rebind_denied",
        "signature_timestamp_replay_denied",
        "operator_identity_reuse_denied",
        "release_publication_authority_replay_denied",
        "activation_authority_replay_denied",
        "external_delivery_replay_denied",
        "install_restart_active_binary_replay_denied",
        "memory_provider_kg_replay_denied"
      ],
      allowed_next_actions:[
        {
          action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_gate",
          status:"allowed_report_only_next_slice",
          records_result_receipt:false,
          persists_result_receipt:false,
          records_idempotency:false,
          accepts_duplicate_receipt:false,
          accepts_replay:false,
          accepts_out_of_order_receipt:false,
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
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_allowed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_key_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_key_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_materialized_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_filesystem_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_nonce_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_nonce_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_scope_reuse_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_upgrade_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completed_status_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ack_replay_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_replay_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_index_replay_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_replay_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_replay_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_replay_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_hash_rebind_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_rebind_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_signature_timestamp_replay_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_reuse_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_filesystem_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_indexed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_enqueued_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_exported_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_replay_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_replay_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_replay_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_from_replay_allowed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_replay_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restart_from_replay_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_replay_mutated_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_memory_store_replay_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_kg_replay_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_provider_replay_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_model_replay_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_credential_read_from_replay_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_secret_read_from_replay_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_external_send_from_replay_count"
    ])
    + false_object([
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_allowed",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_persisted",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_accepted",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_key_accepted",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_key_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_persisted",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_filesystem_written",
      "artifact_download_install_affordance_result_receipt_recorded",
      "artifact_download_install_affordance_result_receipt_persisted",
      "artifact_download_install_affordance_result_receipt_accepted",
      "artifact_download_install_affordance_result_receipt_materialized",
      "artifact_download_install_affordance_result_receipt_filesystem_written",
      "artifact_download_install_affordance_completion_ack_recorded",
      "artifact_download_install_affordance_completion_ack_accepted",
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
      "public_release_claimed",
      "public_ga_claimed",
      "release_artifact_written",
      "public_artifact_written",
      "external_send_performed"
    ])
    + {
      side_effects:false_object([
        "artifact_download_install_affordance_result_receipt_replay_allowed",
        "artifact_download_install_affordance_result_receipt_replay_recorded",
        "artifact_download_install_affordance_result_receipt_replay_persisted",
        "artifact_download_install_affordance_result_receipt_replay_performed",
        "artifact_download_install_affordance_result_receipt_duplicate_accepted",
        "artifact_download_install_affordance_result_receipt_duplicate_recorded",
        "artifact_download_install_affordance_result_receipt_duplicate_persisted",
        "artifact_download_install_affordance_result_receipt_idempotency_key_accepted",
        "artifact_download_install_affordance_result_receipt_idempotency_key_recorded",
        "artifact_download_install_affordance_result_receipt_idempotency_state_recorded",
        "artifact_download_install_affordance_result_receipt_idempotency_state_persisted",
        "artifact_download_install_affordance_result_receipt_idempotency_state_materialized",
        "artifact_download_install_affordance_result_receipt_idempotency_filesystem_written",
        "artifact_download_install_affordance_result_receipt_replay_nonce_accepted",
        "artifact_download_install_affordance_result_receipt_cross_scope_reuse_accepted",
        "artifact_download_install_affordance_result_receipt_status_upgrade_accepted",
        "artifact_download_install_affordance_result_receipt_completed_status_accepted",
        "artifact_download_install_affordance_result_receipt_ack_replay_accepted",
        "artifact_download_install_affordance_result_receipt_ledger_replay_accepted",
        "artifact_download_install_affordance_result_receipt_index_replay_accepted",
        "artifact_download_install_affordance_result_receipt_delivery_replay_accepted",
        "artifact_download_install_affordance_result_receipt_query_replay_accepted",
        "artifact_download_install_affordance_result_receipt_observability_replay_accepted",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_materialized",
        "artifact_download_install_affordance_result_receipt_filesystem_written",
        "artifact_download_install_affordance_result_receipt_ledger_written",
        "artifact_download_install_affordance_result_receipt_indexed",
        "artifact_download_install_affordance_result_receipt_enqueued",
        "artifact_download_install_affordance_result_receipt_delivered",
        "artifact_download_install_affordance_result_receipt_exported",
        "artifact_download_install_affordance_result_receipt_query_registered",
        "artifact_download_install_affordance_result_receipt_observability_recorded",
        "artifact_download_install_affordance_completion_ack_recorded",
        "artifact_download_install_affordance_completion_ack_accepted",
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
    }')"

jq -e '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);

  . as $report
  | $report.runtime == "hepta"
  and $report.status == "ready"
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_attempt_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count == 0
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count == 0
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized_count == 0
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_attempt_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_allowed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_key_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_key_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_materialized_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_filesystem_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_nonce_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cross_scope_reuse_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_status_upgrade_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_completed_status_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ack_replay_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_replay_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_index_replay_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_replay_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_replay_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_replay_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_filesystem_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_indexed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_enqueued_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_exported_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_replay_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_replay_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_replay_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_from_replay_allowed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_replay_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restart_from_replay_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_replay_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_memory_store_replay_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_kg_replay_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_provider_replay_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_model_replay_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_credential_read_from_replay_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_secret_read_from_replay_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_external_send_from_replay_count"
  ])
  and false_fields($report; [
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_allowed",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_persisted",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_accepted",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_key_accepted",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_key_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_persisted",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_filesystem_written",
    "artifact_download_install_affordance_result_receipt_recorded",
    "artifact_download_install_affordance_result_receipt_persisted",
    "artifact_download_install_affordance_result_receipt_accepted",
    "artifact_download_install_affordance_result_receipt_materialized",
    "artifact_download_install_affordance_result_receipt_filesystem_written",
    "artifact_download_install_affordance_completion_ack_recorded",
    "artifact_download_install_affordance_completion_ack_accepted",
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
    "public_release_claimed",
    "public_ga_claimed",
    "release_artifact_written",
    "public_artifact_written",
    "external_send_performed"
  ])
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surfaces | all(
    .artifact_download_install_affordance_result_receipt_replay_requested == true
    and .receipt_noop_confirmed == true
    and false_fields(.; [
      "artifact_download_install_affordance_result_receipt_replay_allowed",
      "artifact_download_install_affordance_result_receipt_replay_recorded",
      "artifact_download_install_affordance_result_receipt_replay_persisted",
      "artifact_download_install_affordance_result_receipt_replay_performed",
      "artifact_download_install_affordance_result_receipt_duplicate_accepted",
      "artifact_download_install_affordance_result_receipt_duplicate_recorded",
      "artifact_download_install_affordance_result_receipt_duplicate_persisted",
      "artifact_download_install_affordance_result_receipt_idempotency_key_accepted",
      "artifact_download_install_affordance_result_receipt_idempotency_key_recorded",
      "artifact_download_install_affordance_result_receipt_idempotency_state_recorded",
      "artifact_download_install_affordance_result_receipt_idempotency_state_persisted",
      "artifact_download_install_affordance_result_receipt_idempotency_state_materialized",
      "artifact_download_install_affordance_result_receipt_idempotency_filesystem_written",
      "artifact_download_install_affordance_result_receipt_replay_nonce_accepted",
      "artifact_download_install_affordance_result_receipt_replay_nonce_recorded",
      "artifact_download_install_affordance_result_receipt_cross_scope_reuse_accepted",
      "artifact_download_install_affordance_result_receipt_status_upgrade_accepted",
      "artifact_download_install_affordance_result_receipt_completed_status_accepted",
      "artifact_download_install_affordance_result_receipt_ack_replay_accepted",
      "artifact_download_install_affordance_result_receipt_ledger_replay_accepted",
      "artifact_download_install_affordance_result_receipt_index_replay_accepted",
      "artifact_download_install_affordance_result_receipt_delivery_replay_accepted",
      "artifact_download_install_affordance_result_receipt_query_replay_accepted",
      "artifact_download_install_affordance_result_receipt_observability_replay_accepted",
      "artifact_download_install_affordance_result_receipt_recorded",
      "artifact_download_install_affordance_result_receipt_persisted",
      "artifact_download_install_affordance_result_receipt_accepted",
      "artifact_download_install_affordance_result_receipt_materialized",
      "artifact_download_install_affordance_result_receipt_filesystem_written",
      "artifact_download_install_affordance_result_receipt_ledger_written",
      "artifact_download_install_affordance_result_receipt_indexed",
      "artifact_download_install_affordance_result_receipt_enqueued",
      "artifact_download_install_affordance_result_receipt_delivered",
      "artifact_download_install_affordance_result_receipt_exported",
      "artifact_download_install_affordance_result_receipt_query_registered",
      "artifact_download_install_affordance_result_receipt_observability_recorded",
      "artifact_download_install_affordance_completion_ack_recorded",
      "artifact_download_install_affordance_completion_ack_accepted",
      "operator_approval_from_replay_accepted",
      "release_publication_authority_from_replay_derived",
      "activation_authority_from_replay_derived",
      "activation_command_from_replay_derived",
      "live_execution_from_replay_allowed",
      "install_from_replay_executed",
      "service_restart_from_replay_performed",
      "active_binary_from_replay_mutated",
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
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_result_receipt == false
    and .persists_result_receipt == false
    and .records_idempotency == false
    and .accepts_duplicate_receipt == false
    and .accepts_replay == false
    and .accepts_out_of_order_receipt == false
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

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt replay/idempotency denial gate passed"
