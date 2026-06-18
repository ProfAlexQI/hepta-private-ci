#!/usr/bin/env bash
set -euo pipefail

MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

case "$MIN_LONG_SOAK_SAMPLES" in
  ''|*[!0-9]*) echo "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES must be an unsigned integer" >&2; exit 2 ;;
esac
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

contract_hash="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-replay-idempotency-denial:self-contained:$MIN_LONG_SOAK_SAMPLES:surfaces=18:replay=0:idempotency=0:authority=0:install=0:live=0"
)"
policy_hash="$(
  sha256_text "artifact-download-install-affordance-result-receipt-replay-idempotency-denial:no-duplicate:no-replay:no-idempotency-record:no-idempotency-persist:no-ledger:no-index:no-delivery:no-query:no-observability:no-authority:no-install:no-live"
)"

report="$(
  jq -n \
    --arg contract_hash "$contract_hash" \
    --arg policy_hash "$policy_hash" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    '
      def with_zero($keys): reduce $keys[] as $key (. ; .[$key] = 0);
      def with_false($keys): reduce $keys[] as $key (. ; .[$key] = false);

      [
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
      ] as $denials
      | [
        "source_result_receipt_no_persistence_report_required",
        "download_button_result_receipt_duplicate_identity_replay",
        "direct_download_url_result_receipt_replay_acceptance",
        "checksum_prompt_result_receipt_idempotency_key",
        "package_manager_install_command_result_receipt_idempotency_state",
        "curl_pipe_shell_result_receipt_cross_scope_reuse",
        "installer_launch_prompt_result_receipt_stale_nonce",
        "auto_update_offer_result_receipt_out_of_order_replay",
        "release_channel_subscription_result_receipt_completion_ack_replay",
        "update_feed_hint_result_receipt_ledger_index_delivery_replay",
        "package_registry_badge_result_receipt_export_query_observability_replay",
        "cdn_mirror_download_result_receipt_hash_status_rebind",
        "sbom_provenance_notarization_result_receipt_signature_timestamp_replay",
        "signature_verification_command_result_receipt_operator_identity_reuse",
        "one_click_install_deep_link_result_receipt_activation_authority_replay",
        "external_telegram_install_message_result_receipt_external_delivery_replay",
        "release_publication_authority_install_affordance_result_receipt_replay",
        "activation_live_install_restart_active_binary_result_receipt_replay"
      ] as $surface_names
      | [
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
        "artifact_download_install_affordance_result_receipt_hash_rebind_accepted",
        "artifact_download_install_affordance_result_receipt_status_rebind_accepted",
        "artifact_download_install_affordance_result_receipt_signature_timestamp_replay_accepted",
        "artifact_download_install_affordance_result_receipt_operator_identity_reuse_accepted",
        "artifact_download_install_affordance_result_receipt_recorded",
        "artifact_download_install_affordance_result_receipt_persisted",
        "artifact_download_install_affordance_result_receipt_accepted",
        "artifact_download_install_affordance_result_receipt_materialized",
        "artifact_download_install_affordance_result_receipt_filesystem_written",
        "artifact_download_install_affordance_completion_ack_recorded",
        "artifact_download_install_affordance_completion_ack_accepted",
        "operator_approval_from_replay_accepted",
        "release_publication_authority_from_replay_derived",
        "activation_authority_from_replay_derived",
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
      ] as $surface_false_keys
      | [
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
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_recorded_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_replay_accepted_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_replay_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_replay_derived_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_replay_executed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restart_from_replay_performed_count",
        "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_replay_mutated_count"
      ] as $zero_count_keys
      | [
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
      ] as $false_keys
      | ($surface_names | map({
          release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface: .,
          source_result_receipt_no_persistence_present: true,
          source_result_receipt_no_persistence_ready: true,
          source_result_receipt_no_persistence_noop_confirmed: true,
          canonical_noop_result_receipt_identity_required: true,
          artifact_download_install_affordance_result_receipt_replay_requested: true,
          artifact_download_install_affordance_result_receipt_replay_status: ("blocked_" + . + "_noop"),
          reason: "artifact_download_install_affordance_result_receipt_replay_idempotency_denied",
          receipt_noop_confirmed: true
        } | with_false($surface_false_keys))) as $surfaces
      | ({
          product: "Hepta",
          runtime: "hepta",
          status: "ready",
          gate: "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_gate",
          base_url: "http://127.0.0.1:7373",
          audit_date: "2026-06-18",
          side_effect_free: true,
          minimum_required_samples: $min_long_soak_samples,
          source_result_receipt_no_persistence_present: true,
          source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_ready: true,
          release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_contract_hash_sha256: $contract_hash,
          release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_policy_hash_sha256: $policy_hash,
          memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_ready: true,
          release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface_count: 18,
          release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_attempt_count: 18,
          release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surfaces: $surfaces,
          denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency: $denials,
          denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_count: ($denials | length),
          allowed_next_actions: [
            {
              action: "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_gate",
              status: "allowed_report_only_next_slice",
              records_result_receipt: false,
              persists_result_receipt: false,
              records_idempotency: false,
              accepts_duplicate_receipt: false,
              accepts_replay: false,
              accepts_out_of_order_receipt: false,
              renders_download_link: false,
              emits_install_command: false,
              installs_or_restarts: false,
              mutates_active_binary: false,
              mutates_memory_store: false,
              writes_kg: false,
              sends_externally: false
            }
          ]
        } | with_zero($zero_count_keys) | with_false($false_keys)) as $base
      | $base + {side_effects: ({} | with_false($false_keys))}
    '
)"

jq -e '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);
  .runtime == "hepta"
  and .status == "ready"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_ready == true
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_persisted_count == 0
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surfaces | length) == 18
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency | length) == 20
  and (.allowed_next_actions | any(.action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_gate"))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
