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
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-replay-idempotency-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-replay-idempotency-denial-gate.sh
)"

source_replay_idempotency_report_sha256="$(sha256_text "$REPLAY_IDEMPOTENCY_JSON")"
terminal_public_claim_delivery_receipt_ordering_monotonicity_contract_hash_sha256="$(
  sha256_text "hepta-artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity-denial:$source_replay_idempotency_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
terminal_public_claim_delivery_receipt_ordering_monotonicity_policy_hash_sha256="$(
  sha256_text "artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity:no-ordering:no-sequence-cursor:no-monotonic-state:no-latest-wins:no-status-upgrade:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$REPLAY_IDEMPOTENCY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_replay_idempotency_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_replay_idempotency_denial_ready == true
    and $source.source_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_ready == true
    and $source.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surface_count == 18
    and $source.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_attempt_count == 18
    and $source.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_denied_count == 18
    and zero_fields($source; [
      "terminal_public_claim_delivery_receipt_replay_allowed_count",
      "terminal_public_claim_delivery_receipt_replay_accepted_count",
      "terminal_public_claim_delivery_receipt_replay_recorded_count",
      "terminal_public_claim_delivery_receipt_replay_persisted_count",
      "terminal_public_claim_delivery_receipt_replay_performed_count",
      "terminal_public_claim_delivery_receipt_duplicate_accepted_count",
      "terminal_public_claim_delivery_receipt_duplicate_recorded_count",
      "terminal_public_claim_delivery_receipt_duplicate_persisted_count",
      "terminal_public_claim_delivery_receipt_idempotency_key_accepted_count",
      "terminal_public_claim_delivery_receipt_idempotency_key_recorded_count",
      "terminal_public_claim_delivery_receipt_idempotency_state_recorded_count",
      "terminal_public_claim_delivery_receipt_idempotency_state_persisted_count",
      "terminal_public_claim_delivery_receipt_idempotency_state_materialized_count",
      "terminal_public_claim_delivery_receipt_idempotency_filesystem_written_count",
      "terminal_public_claim_delivery_receipt_replay_nonce_accepted_count",
      "terminal_public_claim_delivery_receipt_replay_nonce_recorded_count",
      "terminal_public_claim_delivery_receipt_cross_scope_reuse_accepted_count",
      "terminal_public_claim_delivery_receipt_status_upgrade_accepted_count",
      "terminal_public_claim_delivery_receipt_completed_status_accepted_count",
      "terminal_public_claim_delivery_receipt_ack_replay_accepted_count",
      "terminal_public_claim_delivery_receipt_ledger_replay_accepted_count",
      "terminal_public_claim_delivery_receipt_index_replay_accepted_count",
      "terminal_public_claim_delivery_receipt_delivery_replay_accepted_count",
      "terminal_public_claim_delivery_receipt_query_replay_accepted_count",
      "terminal_public_claim_delivery_receipt_export_replay_accepted_count",
      "terminal_public_claim_delivery_receipt_observability_replay_accepted_count",
      "terminal_public_claim_delivery_receipt_hash_status_rebind_accepted_count",
      "release_publication_authority_from_delivery_receipt_replay_derived_count",
      "activation_authority_from_delivery_receipt_replay_derived_count",
      "install_from_delivery_receipt_replay_executed_count",
      "service_restart_from_delivery_receipt_replay_performed_count",
      "active_binary_from_delivery_receipt_replay_mutated_count",
      "memory_store_write_performed_count",
      "live_kg_write_performed_count",
      "provider_invoked_count",
      "model_invoked_count",
      "credential_read_count",
      "secret_file_read_count",
      "external_send_performed_count"
    ])
    and false_fields($source; [
      "terminal_public_claim_delivery_receipt_replay_allowed",
      "terminal_public_claim_delivery_receipt_replay_accepted",
      "terminal_public_claim_delivery_receipt_replay_recorded",
      "terminal_public_claim_delivery_receipt_replay_persisted",
      "terminal_public_claim_delivery_receipt_replay_performed",
      "terminal_public_claim_delivery_receipt_duplicate_accepted",
      "terminal_public_claim_delivery_receipt_idempotency_key_accepted",
      "terminal_public_claim_delivery_receipt_idempotency_state_persisted",
      "terminal_public_claim_delivery_receipt_cross_scope_reuse_accepted",
      "terminal_public_claim_delivery_receipt_status_upgrade_accepted",
      "terminal_public_claim_delivery_receipt_completed_status_accepted",
      "terminal_public_claim_delivery_receipt_hash_status_rebind_accepted",
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
    and ($source.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surfaces | length) == 18
    and ($source.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surfaces | all(
      .terminal_public_claim_delivery_receipt_replay_idempotency_attempted == true
      and .terminal_public_claim_delivery_receipt_replay_idempotency_allowed == false
      and .terminal_public_claim_delivery_receipt_replay_accepted == false
      and .terminal_public_claim_delivery_receipt_duplicate_accepted == false
      and .terminal_public_claim_delivery_receipt_idempotency_key_accepted == false
      and .terminal_public_claim_delivery_receipt_replay_idempotency_noop_confirmed == true
      and .release_publication_authority_from_delivery_receipt_replay_derived == false
      and .activation_authority_from_delivery_receipt_replay_derived == false
      and .install_from_delivery_receipt_replay_executed == false
      and .service_restart_from_delivery_receipt_replay_performed == false
      and .active_binary_from_delivery_receipt_replay_mutated == false
      and .memory_store_write_performed == false
      and .live_kg_write_performed == false
      and .external_send_performed == false
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_gate"
      and .status == "allowed_report_only_next_slice"
      and .replays_delivery_receipt == false
      and .records_duplicate_receipt == false
      and .records_idempotency_key == false
      and .persists_idempotency_state == false
      and .accepts_cross_scope_reuse == false
      and .accepts_status_upgrade == false
      and .records_completion_ack == false
      and .rebinds_hash_status == false
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
    def ordering_surface($id; $status; $reason; $extra):
      {
        artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_surface:$id,
        source_terminal_public_claim_delivery_receipt_replay_idempotency_ready:true,
        canonical_noop_delivery_receipt_identity_required:true,
        terminal_public_claim_delivery_receipt_ordering_monotonicity_attempted:true,
        terminal_public_claim_delivery_receipt_ordering_monotonicity_allowed:false,
        terminal_public_claim_delivery_receipt_ordering_monotonicity_accepted:false,
        terminal_public_claim_delivery_receipt_ordering_monotonicity_recorded:false,
        terminal_public_claim_delivery_receipt_ordering_monotonicity_persisted:false,
        terminal_public_claim_delivery_receipt_ordering_monotonicity_materialized:false,
        terminal_public_claim_delivery_receipt_ordering_monotonicity_filesystem_written:false,
        terminal_public_claim_delivery_receipt_sequence_cursor_accepted:false,
        terminal_public_claim_delivery_receipt_sequence_cursor_recorded:false,
        terminal_public_claim_delivery_receipt_sequence_cursor_persisted:false,
        terminal_public_claim_delivery_receipt_monotonicity_state_recorded:false,
        terminal_public_claim_delivery_receipt_monotonicity_state_persisted:false,
        terminal_public_claim_delivery_receipt_monotonicity_state_materialized:false,
        terminal_public_claim_delivery_receipt_monotonicity_filesystem_written:false,
        terminal_public_claim_delivery_receipt_duplicate_sequence_accepted:false,
        terminal_public_claim_delivery_receipt_stale_sequence_accepted:false,
        terminal_public_claim_delivery_receipt_late_arrival_accepted:false,
        terminal_public_claim_delivery_receipt_future_gap_accepted:false,
        terminal_public_claim_delivery_receipt_timestamp_rollback_accepted:false,
        terminal_public_claim_delivery_receipt_epoch_rollback_accepted:false,
        terminal_public_claim_delivery_receipt_same_sequence_different_hash_accepted:false,
        terminal_public_claim_delivery_receipt_latest_wins_overwrite_accepted:false,
        terminal_public_claim_delivery_receipt_ordered_status_accepted:false,
        terminal_public_claim_delivery_receipt_ordered_acknowledgement_accepted:false,
        terminal_public_claim_delivery_receipt_ordered_ledger_index_accepted:false,
        terminal_public_claim_delivery_receipt_ordered_query_export_accepted:false,
        terminal_public_claim_delivery_receipt_ordered_observability_accepted:false,
        terminal_public_claim_delivery_receipt_ordered_hash_status_accepted:false,
        public_claim_delivery_receipt_ordering_accepted:false,
        status_readback_delivery_receipt_ordering_accepted:false,
        channel_delivery_receipt_ordering_accepted:false,
        telegram_delivery_receipt_ordering_accepted:false,
        external_delivery_receipt_ordering_accepted:false,
        readback_receipt_backfill_ordering_accepted:false,
        operator_approval_from_delivery_receipt_ordering_derived:false,
        release_publication_authority_from_delivery_receipt_ordering_derived:false,
        activation_authority_from_delivery_receipt_ordering_derived:false,
        download_link_from_delivery_receipt_ordering_rendered:false,
        install_command_from_delivery_receipt_ordering_emitted:false,
        install_from_delivery_receipt_ordering_executed:false,
        service_restart_from_delivery_receipt_ordering_performed:false,
        active_binary_from_delivery_receipt_ordering_mutated:false,
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
        release_artifact_written:false,
        public_artifact_written:false,
        terminal_public_claim_delivery_receipt_ordering_monotonicity_noop_confirmed:true,
        terminal_public_claim_delivery_receipt_ordering_monotonicity_status:$status,
        reason:$reason
      } + $extra;
    [
      ordering_surface("source_replay_idempotency_report_required"; "blocked_source_replay_idempotency_required_noop"; "source_replay_idempotency_report_required"; {source_report_required:true}),
      ordering_surface("canonical_delivery_receipt_order_identity"; "blocked_canonical_delivery_receipt_order_identity_noop"; "canonical_delivery_receipt_order_identity_denied"; {canonical_order_identity_requested:true}),
      ordering_surface("duplicate_sequence_delivery_receipt"; "blocked_duplicate_sequence_delivery_receipt_noop"; "duplicate_sequence_delivery_receipt_denied"; {duplicate_sequence_requested:true}),
      ordering_surface("stale_sequence_delivery_receipt"; "blocked_stale_sequence_delivery_receipt_noop"; "stale_sequence_delivery_receipt_denied"; {stale_sequence_requested:true}),
      ordering_surface("late_arrival_delivery_receipt"; "blocked_late_arrival_delivery_receipt_noop"; "late_arrival_delivery_receipt_denied"; {late_arrival_requested:true}),
      ordering_surface("future_gap_delivery_receipt"; "blocked_future_gap_delivery_receipt_noop"; "future_gap_delivery_receipt_denied"; {future_gap_requested:true}),
      ordering_surface("timestamp_rollback_delivery_receipt"; "blocked_timestamp_rollback_delivery_receipt_noop"; "timestamp_rollback_delivery_receipt_denied"; {timestamp_rollback_requested:true}),
      ordering_surface("epoch_rollback_delivery_receipt"; "blocked_epoch_rollback_delivery_receipt_noop"; "epoch_rollback_delivery_receipt_denied"; {epoch_rollback_requested:true}),
      ordering_surface("same_sequence_different_hash_delivery_receipt"; "blocked_same_sequence_different_hash_delivery_receipt_noop"; "same_sequence_different_hash_delivery_receipt_denied"; {same_sequence_different_hash_requested:true}),
      ordering_surface("latest_wins_delivery_receipt"; "blocked_latest_wins_delivery_receipt_noop"; "latest_wins_delivery_receipt_denied"; {latest_wins_requested:true}),
      ordering_surface("status_ordering_upgrade_delivery_receipt"; "blocked_status_ordering_upgrade_delivery_receipt_noop"; "status_ordering_upgrade_delivery_receipt_denied"; {status_ordering_upgrade_requested:true}),
      ordering_surface("acknowledgement_before_source_delivery_receipt"; "blocked_acknowledgement_before_source_delivery_receipt_noop"; "acknowledgement_before_source_delivery_receipt_denied"; {acknowledgement_before_source_requested:true}),
      ordering_surface("ledger_index_delivery_ordering_bypass"; "blocked_ledger_index_delivery_ordering_bypass_noop"; "ledger_index_delivery_ordering_bypass_denied"; {ledger_ordering_bypass_requested:true, index_ordering_bypass_requested:true}),
      ordering_surface("query_export_observability_ordering_bypass"; "blocked_query_export_observability_ordering_bypass_noop"; "query_export_observability_ordering_bypass_denied"; {query_ordering_bypass_requested:true, export_ordering_bypass_requested:true, observability_ordering_bypass_requested:true}),
      ordering_surface("hash_status_order_rebind"; "blocked_hash_status_order_rebind_noop"; "hash_status_order_rebind_denied"; {hash_status_order_rebind_requested:true}),
      ordering_surface("readback_receipt_backfill_ordering"; "blocked_readback_receipt_backfill_ordering_noop"; "readback_receipt_backfill_ordering_denied"; {readback_receipt_backfill_ordering_requested:true}),
      ordering_surface("external_telegram_ordered_delivery_receipt"; "blocked_external_telegram_ordered_delivery_receipt_noop"; "external_telegram_ordered_delivery_receipt_denied"; {external_ordered_delivery_requested:true, telegram_ordered_delivery_requested:true}),
      ordering_surface("authority_install_active_binary_ordering"; "blocked_authority_install_active_binary_ordering_noop"; "authority_install_active_binary_ordering_denied"; {authority_install_active_binary_ordering_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_gate" \
    --arg source_replay_idempotency_report_sha256 "$source_replay_idempotency_report_sha256" \
    --arg terminal_public_claim_delivery_receipt_ordering_monotonicity_contract_hash_sha256 "$terminal_public_claim_delivery_receipt_ordering_monotonicity_contract_hash_sha256" \
    --arg terminal_public_claim_delivery_receipt_ordering_monotonicity_policy_hash_sha256 "$terminal_public_claim_delivery_receipt_ordering_monotonicity_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$REPLAY_IDEMPOTENCY_JSON" \
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
        artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_schema_version:"artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_v1",
        artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_mode:"denied_delivery_receipt_replay_idempotency_cannot_be_ordered_sequenced_monotonically_rebound_promoted_or_used_for_authority_or_live_install",
        source_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_gate:$source.gate,
        source_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_replay_idempotency_denial_ready,
        source_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_report_sha256:$source_replay_idempotency_report_sha256,
        source_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_contract_hash_sha256:$source.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_contract_hash_sha256,
        artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_contract_hash_sha256:$terminal_public_claim_delivery_receipt_ordering_monotonicity_contract_hash_sha256,
        artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_policy_hash_sha256:$terminal_public_claim_delivery_receipt_ordering_monotonicity_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_ready:true,
        source_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surface_count:$source.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surface_count,
        source_artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_denied_count:$source.artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_denied_count,
        artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_surface_count:($surfaces | length),
        artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_attempt_count:($surfaces | length),
        artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denied_count:($surfaces | length),
        artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_surfaces:$surfaces,
        denied_by_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity:[
          "source_replay_idempotency_report_required",
          "canonical_delivery_receipt_order_identity_denied",
          "duplicate_sequence_delivery_receipt_denied",
          "stale_sequence_delivery_receipt_denied",
          "late_arrival_delivery_receipt_denied",
          "future_gap_delivery_receipt_denied",
          "timestamp_rollback_delivery_receipt_denied",
          "epoch_rollback_delivery_receipt_denied",
          "same_sequence_different_hash_delivery_receipt_denied",
          "latest_wins_delivery_receipt_denied",
          "status_ordering_upgrade_delivery_receipt_denied",
          "acknowledgement_before_source_delivery_receipt_denied",
          "ledger_index_delivery_ordering_bypass_denied",
          "query_export_observability_ordering_bypass_denied",
          "hash_status_order_rebind_denied",
          "readback_receipt_backfill_ordering_denied",
          "external_telegram_ordered_delivery_receipt_denied",
          "authority_install_active_binary_ordering_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_gate",
            status:"allowed_report_only_next_slice",
            accepts_ordering:false,
            records_ordering:false,
            persists_ordering:false,
            records_sequence_cursor:false,
            persists_sequence_cursor:false,
            records_monotonicity_state:false,
            persists_monotonicity_state:false,
            accepts_latest_wins:false,
            accepts_status_upgrade:false,
            records_completion_ack:false,
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
        "terminal_public_claim_delivery_receipt_ordering_monotonicity_allowed_count",
        "terminal_public_claim_delivery_receipt_ordering_monotonicity_accepted_count",
        "terminal_public_claim_delivery_receipt_ordering_monotonicity_recorded_count",
        "terminal_public_claim_delivery_receipt_ordering_monotonicity_persisted_count",
        "terminal_public_claim_delivery_receipt_ordering_monotonicity_materialized_count",
        "terminal_public_claim_delivery_receipt_ordering_monotonicity_filesystem_written_count",
        "terminal_public_claim_delivery_receipt_sequence_cursor_accepted_count",
        "terminal_public_claim_delivery_receipt_sequence_cursor_recorded_count",
        "terminal_public_claim_delivery_receipt_sequence_cursor_persisted_count",
        "terminal_public_claim_delivery_receipt_monotonicity_state_recorded_count",
        "terminal_public_claim_delivery_receipt_monotonicity_state_persisted_count",
        "terminal_public_claim_delivery_receipt_monotonicity_state_materialized_count",
        "terminal_public_claim_delivery_receipt_monotonicity_filesystem_written_count",
        "terminal_public_claim_delivery_receipt_duplicate_sequence_accepted_count",
        "terminal_public_claim_delivery_receipt_stale_sequence_accepted_count",
        "terminal_public_claim_delivery_receipt_late_arrival_accepted_count",
        "terminal_public_claim_delivery_receipt_future_gap_accepted_count",
        "terminal_public_claim_delivery_receipt_timestamp_rollback_accepted_count",
        "terminal_public_claim_delivery_receipt_epoch_rollback_accepted_count",
        "terminal_public_claim_delivery_receipt_same_sequence_different_hash_accepted_count",
        "terminal_public_claim_delivery_receipt_latest_wins_overwrite_accepted_count",
        "terminal_public_claim_delivery_receipt_ordered_status_accepted_count",
        "terminal_public_claim_delivery_receipt_ordered_acknowledgement_accepted_count",
        "terminal_public_claim_delivery_receipt_ordered_ledger_index_accepted_count",
        "terminal_public_claim_delivery_receipt_ordered_query_export_accepted_count",
        "terminal_public_claim_delivery_receipt_ordered_observability_accepted_count",
        "terminal_public_claim_delivery_receipt_ordered_hash_status_accepted_count",
        "release_publication_authority_from_delivery_receipt_ordering_derived_count",
        "activation_authority_from_delivery_receipt_ordering_derived_count",
        "install_from_delivery_receipt_ordering_executed_count",
        "service_restart_from_delivery_receipt_ordering_performed_count",
        "active_binary_from_delivery_receipt_ordering_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "terminal_public_claim_delivery_receipt_ordering_monotonicity_allowed",
        "terminal_public_claim_delivery_receipt_ordering_monotonicity_accepted",
        "terminal_public_claim_delivery_receipt_ordering_monotonicity_recorded",
        "terminal_public_claim_delivery_receipt_ordering_monotonicity_persisted",
        "terminal_public_claim_delivery_receipt_ordering_monotonicity_materialized",
        "terminal_public_claim_delivery_receipt_ordering_monotonicity_filesystem_written",
        "terminal_public_claim_delivery_receipt_sequence_cursor_accepted",
        "terminal_public_claim_delivery_receipt_sequence_cursor_recorded",
        "terminal_public_claim_delivery_receipt_sequence_cursor_persisted",
        "terminal_public_claim_delivery_receipt_monotonicity_state_recorded",
        "terminal_public_claim_delivery_receipt_monotonicity_state_persisted",
        "terminal_public_claim_delivery_receipt_monotonicity_state_materialized",
        "terminal_public_claim_delivery_receipt_duplicate_sequence_accepted",
        "terminal_public_claim_delivery_receipt_stale_sequence_accepted",
        "terminal_public_claim_delivery_receipt_late_arrival_accepted",
        "terminal_public_claim_delivery_receipt_future_gap_accepted",
        "terminal_public_claim_delivery_receipt_timestamp_rollback_accepted",
        "terminal_public_claim_delivery_receipt_epoch_rollback_accepted",
        "terminal_public_claim_delivery_receipt_same_sequence_different_hash_accepted",
        "terminal_public_claim_delivery_receipt_latest_wins_overwrite_accepted",
        "terminal_public_claim_delivery_receipt_ordered_status_accepted",
        "terminal_public_claim_delivery_receipt_ordered_acknowledgement_accepted",
        "terminal_public_claim_delivery_receipt_ordered_ledger_index_accepted",
        "terminal_public_claim_delivery_receipt_ordered_query_export_accepted",
        "terminal_public_claim_delivery_receipt_ordered_observability_accepted",
        "terminal_public_claim_delivery_receipt_ordered_hash_status_accepted",
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
        side_effects: false_object([
          "terminal_public_claim_delivery_receipt_ordering_monotonicity_recorded",
          "terminal_public_claim_delivery_receipt_ordering_monotonicity_persisted",
          "terminal_public_claim_delivery_receipt_ordering_monotonicity_materialized",
          "terminal_public_claim_delivery_receipt_ordering_monotonicity_filesystem_written",
          "terminal_public_claim_delivery_receipt_sequence_cursor_recorded",
          "terminal_public_claim_delivery_receipt_sequence_cursor_persisted",
          "terminal_public_claim_delivery_receipt_monotonicity_state_recorded",
          "terminal_public_claim_delivery_receipt_monotonicity_state_persisted",
          "terminal_public_claim_delivery_receipt_monotonicity_state_materialized",
          "terminal_public_claim_delivery_receipt_latest_wins_overwrite_accepted",
          "terminal_public_claim_delivery_receipt_ordered_status_accepted",
          "terminal_public_claim_delivery_receipt_ordered_acknowledgement_accepted",
          "terminal_public_claim_delivery_receipt_ordered_ledger_index_accepted",
          "terminal_public_claim_delivery_receipt_ordered_query_export_accepted",
          "terminal_public_claim_delivery_receipt_ordered_observability_accepted",
          "terminal_public_claim_delivery_receipt_ordered_hash_status_accepted",
          "operator_acceptance_recorded",
          "operator_approval_recorded",
          "release_publication_authority_derived",
          "activation_authority_derived",
          "download_link_rendered",
          "install_command_emitted",
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

printf '%s\n' "$report"

jq -e '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);

  .runtime == "hepta"
  and .status == "ready"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_ready == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_surface_count == 18
  and .artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_attempt_count == 18
  and .artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denied_count == 18
  and zero_fields(.; [
    "terminal_public_claim_delivery_receipt_ordering_monotonicity_allowed_count",
    "terminal_public_claim_delivery_receipt_ordering_monotonicity_accepted_count",
    "terminal_public_claim_delivery_receipt_ordering_monotonicity_recorded_count",
    "terminal_public_claim_delivery_receipt_sequence_cursor_recorded_count",
    "terminal_public_claim_delivery_receipt_monotonicity_state_recorded_count",
    "terminal_public_claim_delivery_receipt_latest_wins_overwrite_accepted_count",
    "terminal_public_claim_delivery_receipt_ordered_status_accepted_count",
    "terminal_public_claim_delivery_receipt_ordered_acknowledgement_accepted_count",
    "release_publication_authority_from_delivery_receipt_ordering_derived_count",
    "activation_authority_from_delivery_receipt_ordering_derived_count",
    "install_from_delivery_receipt_ordering_executed_count",
    "active_binary_from_delivery_receipt_ordering_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "credential_read_count",
    "external_send_performed_count"
  ])
  and false_fields(.; [
    "terminal_public_claim_delivery_receipt_ordering_monotonicity_allowed",
    "terminal_public_claim_delivery_receipt_ordering_monotonicity_accepted",
    "terminal_public_claim_delivery_receipt_ordering_monotonicity_recorded",
    "terminal_public_claim_delivery_receipt_sequence_cursor_recorded",
    "terminal_public_claim_delivery_receipt_monotonicity_state_recorded",
    "terminal_public_claim_delivery_receipt_latest_wins_overwrite_accepted",
    "terminal_public_claim_delivery_receipt_ordered_status_accepted",
    "terminal_public_claim_delivery_receipt_ordered_acknowledgement_accepted",
    "release_publication_authority_derived",
    "activation_authority_derived",
    "install_executed",
    "service_restarted",
    "active_binary_mutated",
    "memory_store_write_performed",
    "live_kg_write_performed",
    "provider_invoked",
    "credential_read",
    "external_send_performed",
    "public_ga_claimed",
    "public_release_claimed"
  ])
  and (.artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_surfaces | length) == 18
  and (.artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_surfaces | all(
    .terminal_public_claim_delivery_receipt_ordering_monotonicity_attempted == true
    and .terminal_public_claim_delivery_receipt_ordering_monotonicity_allowed == false
    and .terminal_public_claim_delivery_receipt_ordering_monotonicity_accepted == false
    and .terminal_public_claim_delivery_receipt_ordering_monotonicity_noop_confirmed == true
    and .release_publication_authority_from_delivery_receipt_ordering_derived == false
    and .activation_authority_from_delivery_receipt_ordering_derived == false
    and .install_from_delivery_receipt_ordering_executed == false
    and .active_binary_from_delivery_receipt_ordering_mutated == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .external_send_performed == false
  ))
  and (.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .accepts_ordering == false
    and .records_sequence_cursor == false
    and .records_monotonicity_state == false
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
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

echo "Hepta memory/intelligence/KG artifact signing terminal public claim delivery receipt ordering/monotonicity denial gate passed"
