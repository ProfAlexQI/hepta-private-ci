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

REPLAY_IDEMPOTENCY_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-replay-idempotency-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-replay-idempotency-denial-gate.sh
)"

source_replay_idempotency_report_sha256="$(sha256_text "$REPLAY_IDEMPOTENCY_JSON")"
ordering_monotonicity_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial:$source_replay_idempotency_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
ordering_monotonicity_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial:no-sequence-cursor:no-monotonicity-state:no-out-of-order:no-gap-fill:no-latest-wins:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$REPLAY_IDEMPOTENCY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_ready == true
    and $source.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_no_persistence_ready == true
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface_count == 18
    and $source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_attempt_count == 18
    and zero_fields($source; [
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
    ])
    and false_fields($source; [
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
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surfaces | length) == 18
    and ($source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surfaces | all(
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
        "artifact_download_install_affordance_result_receipt_cross_scope_reuse_accepted",
        "artifact_download_install_affordance_result_receipt_status_upgrade_accepted",
        "artifact_download_install_affordance_result_receipt_completed_status_accepted",
        "artifact_download_install_affordance_result_receipt_ack_replay_accepted",
        "artifact_download_install_affordance_result_receipt_ledger_replay_accepted",
        "artifact_download_install_affordance_result_receipt_index_replay_accepted",
        "artifact_download_install_affordance_result_receipt_delivery_replay_accepted",
        "artifact_download_install_affordance_result_receipt_query_replay_accepted",
        "artifact_download_install_affordance_result_receipt_observability_replay_accepted",
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
      ])
    ))
    and ($source.allowed_next_actions | any(
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
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

ordering_monotonicity_surfaces_json="$(
  jq -n '
    def ordering_surface($id; $status; $reason; $extra):
      {
        release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface:$id,
        source_replay_idempotency_present:true,
        source_replay_idempotency_ready:true,
        source_replay_idempotency_noop_confirmed:true,
        canonical_noop_result_receipt_order_identity_required:true,
        artifact_download_install_affordance_result_receipt_ordering_requested:true,
        artifact_download_install_affordance_result_receipt_ordering_status:$status,
        artifact_download_install_affordance_result_receipt_ordering_allowed:false,
        artifact_download_install_affordance_result_receipt_ordering_recorded:false,
        artifact_download_install_affordance_result_receipt_ordering_persisted:false,
        artifact_download_install_affordance_result_receipt_ordering_performed:false,
        artifact_download_install_affordance_result_receipt_sequence_cursor_accepted:false,
        artifact_download_install_affordance_result_receipt_sequence_cursor_recorded:false,
        artifact_download_install_affordance_result_receipt_sequence_cursor_persisted:false,
        artifact_download_install_affordance_result_receipt_monotonicity_state_recorded:false,
        artifact_download_install_affordance_result_receipt_monotonicity_state_persisted:false,
        artifact_download_install_affordance_result_receipt_monotonicity_state_materialized:false,
        artifact_download_install_affordance_result_receipt_monotonicity_filesystem_written:false,
        artifact_download_install_affordance_result_receipt_timestamp_ordering_accepted:false,
        artifact_download_install_affordance_result_receipt_epoch_ordering_accepted:false,
        artifact_download_install_affordance_result_receipt_stage_ordering_accepted:false,
        artifact_download_install_affordance_result_receipt_same_sequence_hash_override_accepted:false,
        artifact_download_install_affordance_result_receipt_latest_wins_overwrite_accepted:false,
        artifact_download_install_affordance_result_receipt_gap_fill_accepted:false,
        artifact_download_install_affordance_result_receipt_ack_before_noop_accepted:false,
        artifact_download_install_affordance_result_receipt_ledger_ordering_bypass_accepted:false,
        artifact_download_install_affordance_result_receipt_index_ordering_bypass_accepted:false,
        artifact_download_install_affordance_result_receipt_delivery_ordering_bypass_accepted:false,
        artifact_download_install_affordance_result_receipt_export_ordering_bypass_accepted:false,
        artifact_download_install_affordance_result_receipt_query_ordering_bypass_accepted:false,
        artifact_download_install_affordance_result_receipt_observability_ordering_bypass_accepted:false,
        artifact_download_install_affordance_result_receipt_runtime_ordering_bypass_accepted:false,
        artifact_download_install_affordance_result_receipt_provider_ordering_bypass_accepted:false,
        artifact_download_install_affordance_result_receipt_memory_kg_ordering_bypass_accepted:false,
        artifact_download_install_affordance_result_receipt_external_public_install_ordering_bypass_accepted:false,
        artifact_download_install_affordance_result_receipt_replay_allowed:false,
        artifact_download_install_affordance_result_receipt_duplicate_accepted:false,
        artifact_download_install_affordance_result_receipt_idempotency_key_accepted:false,
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
        operator_approval_from_ordering_accepted:false,
        release_publication_authority_from_ordering_derived:false,
        activation_authority_from_ordering_derived:false,
        activation_command_from_ordering_derived:false,
        activation_from_ordering_allowed:false,
        live_execution_from_ordering_allowed:false,
        install_from_ordering_executed:false,
        service_restart_from_ordering_performed:false,
        launchd_from_ordering_mutated:false,
        active_binary_from_ordering_mutated:false,
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
      ordering_surface("source_replay_idempotency_report_required"; "blocked_source_report_required_noop"; "source_replay_idempotency_report_required"; {source_replay_idempotency_report_required:true}),
      ordering_surface("download_button_result_receipt_sequence_cursor_recording"; "blocked_sequence_cursor_recording_noop"; "download_button_result_receipt_sequence_cursor_recording_denied"; {sequence_cursor_recording_requested:true}),
      ordering_surface("direct_download_url_result_receipt_out_of_order_sequence"; "blocked_out_of_order_sequence_noop"; "direct_download_url_result_receipt_out_of_order_sequence_denied"; {out_of_order_sequence_requested:true, requested_sequence:2, observed_previous_sequence:3}),
      ordering_surface("checksum_prompt_result_receipt_sequence_gap_skip"; "blocked_sequence_gap_noop"; "checksum_prompt_result_receipt_sequence_gap_skip_denied"; {sequence_gap_requested:true, requested_sequence:5, expected_next_sequence:1}),
      ordering_surface("package_manager_install_command_result_receipt_timestamp_rollback"; "blocked_timestamp_rollback_noop"; "package_manager_install_command_result_receipt_timestamp_rollback_denied"; {timestamp_rollback_requested:true}),
      ordering_surface("curl_pipe_shell_result_receipt_epoch_rollback"; "blocked_epoch_rollback_noop"; "curl_pipe_shell_result_receipt_epoch_rollback_denied"; {epoch_rollback_requested:true}),
      ordering_surface("installer_launch_prompt_result_receipt_same_sequence_different_hash"; "blocked_same_sequence_hash_noop"; "installer_launch_prompt_result_receipt_same_sequence_different_hash_denied"; {same_sequence_different_hash_requested:true}),
      ordering_surface("auto_update_offer_result_receipt_latest_wins_overwrite"; "blocked_latest_wins_overwrite_noop"; "auto_update_offer_result_receipt_latest_wins_overwrite_denied"; {latest_wins_overwrite_requested:true}),
      ordering_surface("release_channel_subscription_result_receipt_ack_before_noop"; "blocked_ack_before_noop_noop"; "release_channel_subscription_result_receipt_ack_before_noop_denied"; {completion_ack_before_noop_requested:true}),
      ordering_surface("update_feed_hint_result_receipt_stage_transition_bypass"; "blocked_stage_transition_ordering_noop"; "update_feed_hint_result_receipt_stage_transition_bypass_denied"; {stage_transition_ordering_bypass_requested:true}),
      ordering_surface("package_registry_badge_result_receipt_ledger_index_delivery_bypass"; "blocked_ledger_index_delivery_ordering_noop"; "package_registry_badge_result_receipt_ledger_index_delivery_bypass_denied"; {ledger_ordering_bypass_requested:true, index_ordering_bypass_requested:true, delivery_ordering_bypass_requested:true}),
      ordering_surface("cdn_mirror_download_result_receipt_export_query_observability_bypass"; "blocked_export_query_observability_ordering_noop"; "cdn_mirror_download_result_receipt_export_query_observability_bypass_denied"; {export_ordering_bypass_requested:true, query_ordering_bypass_requested:true, observability_ordering_bypass_requested:true}),
      ordering_surface("sbom_provenance_notarization_result_receipt_hash_status_ordering_rebind"; "blocked_hash_status_ordering_rebind_noop"; "sbom_provenance_notarization_result_receipt_hash_status_ordering_rebind_denied"; {hash_ordering_rebind_requested:true, status_ordering_rebind_requested:true}),
      ordering_surface("signature_verification_command_result_receipt_signature_timestamp_ordering"; "blocked_signature_timestamp_ordering_noop"; "signature_verification_command_result_receipt_signature_timestamp_ordering_denied"; {signature_ordering_requested:true, timestamp_ordering_requested:true}),
      ordering_surface("one_click_install_deep_link_result_receipt_activation_authority_ordering_bypass"; "blocked_activation_authority_ordering_noop"; "one_click_install_deep_link_result_receipt_activation_authority_ordering_bypass_denied"; {activation_authority_ordering_bypass_requested:true}),
      ordering_surface("external_telegram_install_message_result_receipt_external_ordering_bypass"; "blocked_external_delivery_ordering_noop"; "external_telegram_install_message_result_receipt_external_ordering_bypass_denied"; {external_delivery_ordering_bypass_requested:true, telegram_delivery_ordering_bypass_requested:true}),
      ordering_surface("release_publication_authority_install_affordance_result_receipt_ordering_bypass"; "blocked_release_publication_authority_ordering_noop"; "release_publication_authority_install_affordance_result_receipt_ordering_bypass_denied"; {release_publication_authority_ordering_bypass_requested:true}),
      ordering_surface("activation_live_install_restart_active_binary_result_receipt_ordering_bypass"; "blocked_activation_live_install_restart_active_binary_ordering_noop"; "activation_live_install_restart_active_binary_result_receipt_ordering_bypass_denied"; {activation_ordering_bypass_requested:true, install_ordering_bypass_requested:true, service_restart_ordering_bypass_requested:true, active_binary_ordering_bypass_requested:true})
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_gate" \
  --arg source_replay_idempotency_report_sha256 "$source_replay_idempotency_report_sha256" \
  --arg ordering_monotonicity_contract_hash_sha256 "$ordering_monotonicity_contract_hash_sha256" \
  --arg ordering_monotonicity_policy_hash_sha256 "$ordering_monotonicity_policy_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$REPLAY_IDEMPOTENCY_JSON" \
  --argjson surfaces "$ordering_monotonicity_surfaces_json" \
  '
    def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
    def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);

    {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_v1",
      receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_mode:"denied_ordering_cursor_monotonicity_or_latest_wins_attempt_cannot_create_result_receipt_or_install_activation_authority",
      source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_gate:$source.gate,
      source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_denial_ready,
      source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_report_sha256:$source_replay_idempotency_report_sha256,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_contract_hash_sha256:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_contract_hash_sha256,
      release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_contract_hash_sha256:$ordering_monotonicity_contract_hash_sha256,
      release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_policy_hash_sha256:$ordering_monotonicity_policy_hash_sha256,
      minimum_required_samples:$min_long_soak_samples,
      memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_ready:true,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface_count,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_attempt_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_attempt_count,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_allowed_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_allowed_count,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_accepted_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_accepted_count,
      source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_recorded_count:$source.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_recorded_count,
      release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface_count:($surfaces | length),
      release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_attempt_count:($surfaces | length),
      release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denied_count:($surfaces | length),
      release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surfaces:$surfaces,
      denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity:[
        "source_result_receipt_replay_idempotency_report_required",
        "canonical_noop_result_receipt_order_identity_required",
        "sequence_cursor_acceptance_denied",
        "sequence_cursor_recording_denied",
        "sequence_cursor_persistence_denied",
        "monotonicity_state_recording_denied",
        "monotonicity_state_persistence_denied",
        "monotonicity_state_materialization_denied",
        "out_of_order_sequence_denied",
        "sequence_gap_or_skip_denied",
        "timestamp_rollback_denied",
        "epoch_rollback_denied",
        "same_sequence_different_hash_denied",
        "latest_wins_overwrite_denied",
        "completion_ack_before_noop_denied",
        "stage_transition_ordering_denied",
        "ledger_index_delivery_ordering_bypass_denied",
        "export_query_observability_ordering_bypass_denied",
        "hash_status_ordering_rebind_denied",
        "signature_timestamp_ordering_denied",
        "operator_identity_reuse_ordering_denied",
        "release_publication_authority_ordering_denied",
        "activation_authority_ordering_denied",
        "runtime_provider_memory_kg_ordering_bypass_denied",
        "external_public_release_ordering_bypass_denied",
        "install_restart_active_binary_ordering_bypass_denied"
      ],
      allowed_next_actions:[
        {
          action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_gate",
          status:"allowed_report_only_next_slice",
          accepts_cancellation:false,
          accepts_supersession:false,
          accepts_out_of_order_receipt:false,
          records_result_receipt:false,
          persists_replacement_receipt:false,
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
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_allowed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_materialized_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_filesystem_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_timestamp_ordering_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_epoch_ordering_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_stage_ordering_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_same_sequence_hash_override_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_latest_wins_overwrite_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_gap_fill_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ack_before_noop_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_ordering_bypass_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_index_ordering_bypass_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivery_ordering_bypass_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_export_ordering_bypass_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_ordering_bypass_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_ordering_bypass_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_runtime_ordering_bypass_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_provider_ordering_bypass_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_memory_kg_ordering_bypass_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_external_public_install_ordering_bypass_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_materialized_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_filesystem_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ledger_written_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_indexed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_delivered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_exported_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_query_registered_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_observability_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_recorded_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_ordering_accepted_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_ordering_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_ordering_derived_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_ordering_executed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restart_from_ordering_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_ordering_mutated_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_memory_store_ordering_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_kg_ordering_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_provider_ordering_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_model_ordering_performed_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_credential_read_from_ordering_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_secret_read_from_ordering_count",
      "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_external_send_from_ordering_count"
    ])
    + false_object([
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_allowed",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded",
      "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_persisted",
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
        "artifact_download_install_affordance_result_receipt_ordering_allowed",
        "artifact_download_install_affordance_result_receipt_ordering_recorded",
        "artifact_download_install_affordance_result_receipt_ordering_persisted",
        "artifact_download_install_affordance_result_receipt_ordering_performed",
        "artifact_download_install_affordance_result_receipt_sequence_cursor_accepted",
        "artifact_download_install_affordance_result_receipt_sequence_cursor_recorded",
        "artifact_download_install_affordance_result_receipt_sequence_cursor_persisted",
        "artifact_download_install_affordance_result_receipt_monotonicity_state_recorded",
        "artifact_download_install_affordance_result_receipt_monotonicity_state_persisted",
        "artifact_download_install_affordance_result_receipt_monotonicity_state_materialized",
        "artifact_download_install_affordance_result_receipt_monotonicity_filesystem_written",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_ready == true
  and $report.source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_ready == true
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_surface_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_attempt_count == 18
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_allowed_count == 0
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_duplicate_accepted_count == 0
  and $report.source_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_idempotency_state_recorded_count == 0
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_attempt_count == 18
  and $report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denied_count == 18
  and zero_fields($report; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_allowed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_materialized_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_filesystem_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_timestamp_ordering_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_epoch_ordering_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_latest_wins_overwrite_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_gap_fill_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ack_before_noop_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_completion_ack_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_operator_approval_from_ordering_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_from_ordering_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_from_ordering_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_from_ordering_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restart_from_ordering_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_from_ordering_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_memory_store_ordering_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_kg_ordering_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_provider_ordering_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_model_ordering_performed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_credential_read_from_ordering_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_secret_read_from_ordering_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_external_send_from_ordering_count"
  ])
  and false_fields($report; [
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_allowed",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded",
    "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_persisted",
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
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surfaces | length) == 18
  and ($report.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surfaces | all(
    .artifact_download_install_affordance_result_receipt_ordering_requested == true
    and .receipt_noop_confirmed == true
    and false_fields(.; [
      "artifact_download_install_affordance_result_receipt_ordering_allowed",
      "artifact_download_install_affordance_result_receipt_ordering_recorded",
      "artifact_download_install_affordance_result_receipt_ordering_persisted",
      "artifact_download_install_affordance_result_receipt_ordering_performed",
      "artifact_download_install_affordance_result_receipt_sequence_cursor_accepted",
      "artifact_download_install_affordance_result_receipt_sequence_cursor_recorded",
      "artifact_download_install_affordance_result_receipt_sequence_cursor_persisted",
      "artifact_download_install_affordance_result_receipt_monotonicity_state_recorded",
      "artifact_download_install_affordance_result_receipt_monotonicity_state_persisted",
      "artifact_download_install_affordance_result_receipt_monotonicity_state_materialized",
      "artifact_download_install_affordance_result_receipt_monotonicity_filesystem_written",
      "artifact_download_install_affordance_result_receipt_timestamp_ordering_accepted",
      "artifact_download_install_affordance_result_receipt_epoch_ordering_accepted",
      "artifact_download_install_affordance_result_receipt_stage_ordering_accepted",
      "artifact_download_install_affordance_result_receipt_same_sequence_hash_override_accepted",
      "artifact_download_install_affordance_result_receipt_latest_wins_overwrite_accepted",
      "artifact_download_install_affordance_result_receipt_gap_fill_accepted",
      "artifact_download_install_affordance_result_receipt_ack_before_noop_accepted",
      "artifact_download_install_affordance_result_receipt_ledger_ordering_bypass_accepted",
      "artifact_download_install_affordance_result_receipt_index_ordering_bypass_accepted",
      "artifact_download_install_affordance_result_receipt_delivery_ordering_bypass_accepted",
      "artifact_download_install_affordance_result_receipt_external_public_install_ordering_bypass_accepted",
      "artifact_download_install_affordance_result_receipt_recorded",
      "artifact_download_install_affordance_result_receipt_persisted",
      "artifact_download_install_affordance_completion_ack_recorded",
      "operator_approval_from_ordering_accepted",
      "release_publication_authority_from_ordering_derived",
      "activation_authority_from_ordering_derived",
      "install_from_ordering_executed",
      "service_restart_from_ordering_performed",
      "active_binary_from_ordering_mutated",
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
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surfaces[] | select(.sequence_cursor_recording_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surfaces[] | select(.out_of_order_sequence_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surfaces[] | select(.sequence_gap_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surfaces[] | select(.latest_wins_overwrite_requested == true)] | length) == 1
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity | length) == 26
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_cancellation_supersession_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .accepts_cancellation == false
    and .accepts_supersession == false
    and .accepts_out_of_order_receipt == false
    and .persists_replacement_receipt == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .sends_externally == false
  ))
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt ordering/monotonicity denial gate passed"
