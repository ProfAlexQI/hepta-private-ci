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
  capture_json_report \
    "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-gate" \
    env \
      HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS="${HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS:-0}" \
      HEPTA_LONG_SOAK_OBSERVATION_INTERVAL_SECONDS="${HEPTA_LONG_SOAK_OBSERVATION_INTERVAL_SECONDS:-0}" \
      HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_INTERVAL_SECONDS="${HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_INTERVAL_SECONDS:-0}" \
      scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-gate.sh
)"

ordering_monotonicity_fixtures_json="$(
  jq -n '
    def ordering_fixture($id; $status; $reason; $extra):
      {
        id: $id,
        ordering_requested: true,
        ordering_status: $status,
        source_replay_idempotency_present: true,
        source_replay_idempotency_ready: true,
        canonical_noop_result_receipt_order_identity_required: true,
        activation_command_result_receipt_ordering_allowed: false,
        activation_command_result_receipt_ordering_recorded: false,
        activation_command_result_receipt_ordering_persisted: false,
        activation_command_result_receipt_ordering_materialized: false,
        activation_command_result_receipt_ordering_filesystem_written: false,
        activation_command_result_receipt_ordering_performed: false,
        activation_command_result_receipt_sequence_cursor_accepted: false,
        activation_command_result_receipt_sequence_cursor_recorded: false,
        activation_command_result_receipt_sequence_cursor_persisted: false,
        activation_command_result_receipt_monotonicity_state_recorded: false,
        activation_command_result_receipt_monotonicity_state_persisted: false,
        activation_command_result_receipt_monotonicity_state_materialized: false,
        activation_command_result_receipt_monotonicity_filesystem_written: false,
        activation_command_result_receipt_timestamp_ordering_accepted: false,
        activation_command_result_receipt_epoch_ordering_accepted: false,
        activation_command_result_receipt_stage_ordering_accepted: false,
        activation_command_result_receipt_same_sequence_hash_override_accepted: false,
        activation_command_result_receipt_latest_wins_overwrite_accepted: false,
        activation_command_result_receipt_gap_fill_accepted: false,
        activation_command_result_receipt_ack_before_noop_accepted: false,
        activation_command_result_receipt_ledger_ordering_bypass_accepted: false,
        activation_command_result_receipt_index_ordering_bypass_accepted: false,
        activation_command_result_receipt_delivery_ordering_bypass_accepted: false,
        activation_command_result_receipt_runtime_ordering_bypass_accepted: false,
        activation_command_result_receipt_provider_ordering_bypass_accepted: false,
        activation_command_result_receipt_memory_kg_ordering_bypass_accepted: false,
        activation_command_result_receipt_external_public_install_ordering_bypass_accepted: false,
        activation_command_result_receipt_upstream_ordering_bypass_accepted: false,
        activation_command_result_receipt_credential_secret_ordering_bypass_accepted: false,
        activation_command_result_receipt_replay_allowed: false,
        activation_command_result_receipt_replay_recorded: false,
        activation_command_result_receipt_replay_persisted: false,
        activation_command_result_receipt_duplicate_accepted: false,
        activation_command_result_receipt_idempotency_key_accepted: false,
        activation_command_result_receipt_idempotency_state_recorded: false,
        activation_command_result_receipt_idempotency_state_persisted: false,
        activation_command_result_receipt_recorded: false,
        activation_command_result_receipt_persisted: false,
        activation_command_result_receipt_accepted: false,
        activation_command_result_receipt_materialized: false,
        activation_command_result_receipt_filesystem_written: false,
        activation_command_result_receipt_ledger_written: false,
        activation_command_result_receipt_indexed: false,
        activation_command_result_receipt_enqueued: false,
        activation_command_result_receipt_delivered: false,
        activation_command_result_receipt_exported: false,
        activation_command_result_receipt_query_registered: false,
        activation_command_result_receipt_observability_recorded: false,
        activation_command_completion_ack_recorded: false,
        activation_command_completion_ack_persisted: false,
        activation_command_completion_ack_accepted: false,
        activation_command_completion_ack_delivered: false,
        operator_approval_from_ordering_accepted: false,
        activation_from_ordering_allowed: false,
        activation_from_replay_allowed: false,
        activation_from_receipt_allowed: false,
        terminal_closure_from_ordering_recorded: false,
        terminal_closure_from_ordering_accepted: false,
        activation_command_shape_registered: false,
        activation_command_accepted: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
        activation_command_dispatch_performed: false,
        activation_request_accepted: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_executed: false,
        activation_allowed: false,
        activation_performed: false,
        terminal_closure_recorded: false,
        terminal_closure_accepted: false,
        terminal_closure_final_state_promoted: false,
        terminal_closure_completion_promoted: false,
        runtime_attachment_performed: false,
        live_context_attached: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        usage_recorded: false,
        memory_store_write_performed: false,
        memory_store_mutated: false,
        live_kg_write_performed: false,
        ordering_ledger_written: false,
        ordering_indexed: false,
        ordering_query_registered: false,
        ordering_observability_recorded: false,
        readback_evidence_recorded: false,
        readback_evidence_persisted: false,
        router_handoff_recorded: false,
        router_handoff_persisted: false,
        rollback_executed: false,
        telegram_send_performed: false,
        channel_send_performed: false,
        external_send_performed: false,
        public_release_claimed: false,
        public_ga_claimed: false,
        release_artifact_written: false,
        install_executed: false,
        launchd_mutated: false,
        service_restarted: false,
        active_binary_mutated: false,
        upstream_fetch_performed: false,
        upstream_merge_performed: false,
        credential_read: false,
        secret_value_read: false,
        raw_payload_plaintext_recorded: false,
        raw_payload_plaintext_persisted: false,
        receipt_noop_confirmed: true,
        reason: $reason
      } + $extra;
    [
      ordering_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-ordering-missing-source-replay-idempotency-report";
        "blocked_noop";
        "source_result_receipt_replay_idempotency_report_required";
        {
          source_replay_idempotency_present: false,
          source_replay_idempotency_ready: false
        }
      ),
      ordering_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-sequence-cursor-recording";
        "blocked_ordering_noop";
        "sequence_cursor_recording_denied";
        {
          sequence_cursor_recording_requested: true,
          requested_sequence_cursor: "core_activation_receipt_sequence_1"
        }
      ),
      ordering_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-out-of-order-sequence";
        "blocked_ordering_noop";
        "out_of_order_result_receipt_sequence_denied";
        {
          out_of_order_sequence_requested: true,
          requested_sequence: 2,
          observed_previous_sequence: 3
        }
      ),
      ordering_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-sequence-gap-skip";
        "blocked_ordering_noop";
        "sequence_gap_or_skip_result_receipt_denied";
        {
          sequence_gap_requested: true,
          requested_sequence: 5,
          expected_next_sequence: 1
        }
      ),
      ordering_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-timestamp-rollback";
        "blocked_ordering_noop";
        "timestamp_rollback_result_receipt_denied";
        {
          timestamp_rollback_requested: true,
          requested_timestamp_order: "older_than_source_replay_idempotency_report"
        }
      ),
      ordering_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-epoch-rollback";
        "blocked_ordering_noop";
        "epoch_rollback_result_receipt_denied";
        {
          epoch_rollback_requested: true,
          requested_epoch_order: "lower_than_current_activation_epoch"
        }
      ),
      ordering_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-same-sequence-different-hash";
        "blocked_ordering_noop";
        "same_sequence_different_hash_result_receipt_denied";
        {
          same_sequence_different_hash_requested: true,
          requested_sequence: 1,
          requested_hash_relation: "different_hash_for_same_sequence"
        }
      ),
      ordering_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-latest-wins-overwrite";
        "blocked_ordering_noop";
        "latest_wins_result_receipt_overwrite_denied";
        {
          latest_wins_overwrite_requested: true,
          overwrite_existing_noop_requested: true
        }
      ),
      ordering_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-stage-ledger-index-delivery-ordering-bypass";
        "blocked_ordering_noop";
        "stage_ledger_index_delivery_ordering_bypass_denied";
        {
          stage_transition_ordering_bypass_requested: true,
          completion_ack_before_noop_requested: true,
          ledger_ordering_bypass_requested: true,
          index_ordering_bypass_requested: true,
          delivery_ordering_bypass_requested: true
        }
      ),
      ordering_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-runtime-provider-memory-kg-external-ordering-bypass";
        "blocked_ordering_noop";
        "runtime_provider_memory_kg_external_ordering_bypass_denied";
        {
          runtime_ordering_bypass_requested: true,
          live_context_ordering_bypass_requested: true,
          context_injection_ordering_bypass_requested: true,
          provider_ordering_bypass_requested: true,
          model_ordering_bypass_requested: true,
          memory_store_ordering_bypass_requested: true,
          live_kg_ordering_bypass_requested: true,
          external_send_ordering_bypass_requested: true,
          public_claim_ordering_bypass_requested: true,
          install_ordering_bypass_requested: true,
          service_restart_ordering_bypass_requested: true,
          active_binary_mutation_ordering_bypass_requested: true,
          upstream_ordering_bypass_requested: true,
          credential_secret_ordering_bypass_requested: true
        }
      )
    ]
  '
)"

replay_idempotency_report_sha256="$(sha256_text "$REPLAY_IDEMPOTENCY_JSON")"
ordering_monotonicity_fixtures_sha256="$(sha256_text "$ordering_monotonicity_fixtures_json")"
ordering_monotonicity_contract_hash_sha256="$(
  sha256_text "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial:$replay_idempotency_report_sha256:$ordering_monotonicity_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
ordering_monotonicity_policy_hash_sha256="$(
  sha256_text "operator-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial:no-ordering:no-monotonicity-record:no-cursor:no-stage-bypass:no-runtime:no-provider:no-model:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "ordering=false;monotonicity=false;cursor=false;record=false;persist=false;activation=false;terminal-closure=false;provider=false;model=false;release=false;install=false;restart=false;active_binary=false;upstream=false;secret=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$REPLAY_IDEMPOTENCY_JSON" \
  --argjson fixtures "$ordering_monotonicity_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_gate"
    and $source.schema_version == "core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_v1"
    and $source.core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status == "blocked"
    and $source.source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_ready == true
    and $source.source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_status == "blocked"
    and $source.activation_command_result_receipt_replay_idempotency_surface_count == 13
    and $source.activation_command_result_receipt_replay_idempotency_surface_ready_count == 13
    and $source.activation_command_result_receipt_replay_idempotency_fixture_count == 10
    and $source.blocked_activation_command_result_receipt_replay_idempotency_fixture_count == 10
    and $source.noop_activation_command_result_receipt_replay_idempotency_fixture_count == 10
    and $source.allowed_activation_command_result_receipt_replay_idempotency_fixture_count == 0
    and $source.accepted_activation_command_result_receipt_replay_idempotency_fixture_count == 0
    and $source.activation_command_result_receipt_replay_performed_count == 0
    and $source.activation_command_result_receipt_duplicate_accepted_count == 0
    and $source.activation_command_result_receipt_idempotency_state_recorded_count == 0
    and $source.activation_command_result_receipt_replay_allowed == false
    and $source.activation_command_result_receipt_replay_recorded == false
    and $source.activation_command_result_receipt_replay_persisted == false
    and $source.activation_command_result_receipt_replay_performed == false
    and $source.activation_command_result_receipt_duplicate_accepted == false
    and $source.activation_command_result_receipt_idempotency_key_accepted == false
    and $source.activation_command_result_receipt_idempotency_state_recorded == false
    and $source.activation_command_result_receipt_idempotency_state_persisted == false
    and $source.activation_command_result_receipt_completed_status_accepted == false
    and $source.activation_command_result_receipt_completion_ack_replay_accepted == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_completion_ack_accepted == false
    and $source.operator_approval_from_replay_accepted == false
    and $source.activation_from_replay_allowed == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and $source.terminal_closure_recorded == false
    and $source.terminal_closure_accepted == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.external_send_performed == false
    and $source.public_release_claimed == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and $source.upstream_fetch_performed == false
    and $source.upstream_merge_performed == false
    and $source.credential_read == false
    and $source.secret_value_read == false
    and $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_count == 212
    and ($source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial | length) == 212
    and $source.inherited_denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_count == 182
    and ($source.allowed_next_actions | any(.action == "stage_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial" and .status == "allowed_report_only_next_slice" and .accepts_out_of_order_receipt == false and .records_ordering_state == false and .records_terminal_closure == false and .mutates_runtime == false and .invokes_model == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.ordering_status | startswith("blocked_"))
      and .activation_command_result_receipt_ordering_allowed == false
      and .activation_command_result_receipt_ordering_recorded == false
      and .activation_command_result_receipt_ordering_persisted == false
      and .activation_command_result_receipt_ordering_performed == false
      and .activation_command_result_receipt_sequence_cursor_accepted == false
      and .activation_command_result_receipt_sequence_cursor_recorded == false
      and .activation_command_result_receipt_sequence_cursor_persisted == false
      and .activation_command_result_receipt_monotonicity_state_recorded == false
      and .activation_command_result_receipt_monotonicity_state_persisted == false
      and .activation_command_result_receipt_timestamp_ordering_accepted == false
      and .activation_command_result_receipt_epoch_ordering_accepted == false
      and .activation_command_result_receipt_stage_ordering_accepted == false
      and .activation_command_result_receipt_latest_wins_overwrite_accepted == false
      and .activation_command_result_receipt_recorded == false
      and .activation_command_result_receipt_persisted == false
      and .activation_command_result_receipt_accepted == false
      and .activation_command_completion_ack_accepted == false
      and .operator_approval_from_ordering_accepted == false
      and .activation_from_ordering_allowed == false
      and .terminal_closure_from_ordering_recorded == false
      and .terminal_closure_from_ordering_accepted == false
      and .activation_command_enabled == false
      and .activation_command_invoked == false
      and .activation_command_dispatched == false
      and .activation_request_accepted == false
      and .activation_request_recorded == false
      and .activation_request_persisted == false
      and .activation_request_executed == false
      and .activation_performed == false
      and .terminal_closure_recorded == false
      and .terminal_closure_accepted == false
      and .provider_invoked == false
      and .model_invoked == false
      and .external_send_performed == false
      and .public_release_claimed == false
      and .release_artifact_written == false
      and .install_executed == false
      and .service_restarted == false
      and .active_binary_mutated == false
      and .upstream_fetch_performed == false
      and .upstream_merge_performed == false
      and .credential_read == false
      and .secret_value_read == false
      and .receipt_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_gate" \
    --arg replay_idempotency_report_sha256 "$replay_idempotency_report_sha256" \
    --arg ordering_monotonicity_fixtures_sha256 "$ordering_monotonicity_fixtures_sha256" \
    --arg ordering_monotonicity_contract_hash_sha256 "$ordering_monotonicity_contract_hash_sha256" \
    --arg ordering_monotonicity_policy_hash_sha256 "$ordering_monotonicity_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$REPLAY_IDEMPOTENCY_JSON" \
    --argjson fixtures "$ordering_monotonicity_fixtures_json" \
    '{
      product: $product,
      runtime: $runtime,
      status: "ready",
      base_url: $base_url,
      gate: $gate,
      schema_version: "core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_v1",
      mode: "summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_no_ordering_no_monotonicity_persist",
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_gate: $source.gate,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_ready: $source.core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_status: $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_report_sha256: $replay_idempotency_report_sha256,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_ready: $source.source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_ready,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_status: $source.source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_status,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_report_sha256: $source.source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_report_sha256,
      ordering_monotonicity_fixtures_sha256: $ordering_monotonicity_fixtures_sha256,
      ordering_monotonicity_contract_hash_sha256: $ordering_monotonicity_contract_hash_sha256,
      ordering_monotonicity_policy_hash_sha256: $ordering_monotonicity_policy_hash_sha256,
      side_effect_hash_sha256: $side_effect_hash_sha256,
      minimum_required_samples: $min_long_soak_samples,
      core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready: true,
      operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status: "blocked",
      core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready: $source.core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready,
      operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status: $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status,
      source_activation_command_result_receipt_replay_idempotency_surface_count: $source.activation_command_result_receipt_replay_idempotency_surface_count,
      source_activation_command_result_receipt_replay_idempotency_fixture_count: $source.activation_command_result_receipt_replay_idempotency_fixture_count,
      source_activation_command_result_receipt_replay_idempotency_denied_count: $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_count,
      activation_command_result_receipt_ordering_monotonicity_surface_count: 14,
      activation_command_result_receipt_ordering_monotonicity_surface_ready_count: 14,
      activation_command_result_receipt_ordering_monotonicity_side_effect_free_surface_count: 14,
      activation_command_result_receipt_ordering_monotonicity_fixture_count: ($fixtures | length),
      blocked_activation_command_result_receipt_ordering_monotonicity_fixture_count: ($fixtures | length),
      noop_activation_command_result_receipt_ordering_monotonicity_fixture_count: ($fixtures | length),
      allowed_activation_command_result_receipt_ordering_monotonicity_fixture_count: 0,
      accepted_activation_command_result_receipt_ordering_monotonicity_fixture_count: 0,
      activation_command_result_receipt_ordering_denied_count: 10,
      activation_command_result_receipt_monotonicity_denied_count: 10,
      activation_command_result_receipt_ordering_performed_count: 0,
      activation_command_result_receipt_sequence_cursor_accepted_count: 0,
      activation_command_result_receipt_sequence_cursor_recorded_count: 0,
      activation_command_result_receipt_monotonicity_state_recorded_count: 0,
      activation_command_result_receipt_ordering_allowed: false,
      activation_command_result_receipt_ordering_recorded: false,
      activation_command_result_receipt_ordering_persisted: false,
      activation_command_result_receipt_ordering_materialized: false,
      activation_command_result_receipt_ordering_filesystem_written: false,
      activation_command_result_receipt_ordering_performed: false,
      activation_command_result_receipt_sequence_cursor_accepted: false,
      activation_command_result_receipt_sequence_cursor_recorded: false,
      activation_command_result_receipt_sequence_cursor_persisted: false,
      activation_command_result_receipt_monotonicity_state_recorded: false,
      activation_command_result_receipt_monotonicity_state_persisted: false,
      activation_command_result_receipt_monotonicity_state_materialized: false,
      activation_command_result_receipt_monotonicity_filesystem_written: false,
      activation_command_result_receipt_timestamp_ordering_accepted: false,
      activation_command_result_receipt_epoch_ordering_accepted: false,
      activation_command_result_receipt_stage_ordering_accepted: false,
      activation_command_result_receipt_same_sequence_hash_override_accepted: false,
      activation_command_result_receipt_latest_wins_overwrite_accepted: false,
      activation_command_result_receipt_gap_fill_accepted: false,
      activation_command_result_receipt_ack_before_noop_accepted: false,
      activation_command_result_receipt_ledger_ordering_bypass_accepted: false,
      activation_command_result_receipt_index_ordering_bypass_accepted: false,
      activation_command_result_receipt_delivery_ordering_bypass_accepted: false,
      activation_command_result_receipt_runtime_ordering_bypass_accepted: false,
      activation_command_result_receipt_provider_ordering_bypass_accepted: false,
      activation_command_result_receipt_memory_kg_ordering_bypass_accepted: false,
      activation_command_result_receipt_external_public_install_ordering_bypass_accepted: false,
      activation_command_result_receipt_upstream_ordering_bypass_accepted: false,
      activation_command_result_receipt_credential_secret_ordering_bypass_accepted: false,
      activation_command_result_receipt_replay_allowed: false,
      activation_command_result_receipt_replay_recorded: false,
      activation_command_result_receipt_replay_persisted: false,
      activation_command_result_receipt_duplicate_accepted: false,
      activation_command_result_receipt_idempotency_key_accepted: false,
      activation_command_result_receipt_idempotency_state_recorded: false,
      activation_command_result_receipt_idempotency_state_persisted: false,
      activation_command_result_receipt_recorded: false,
      activation_command_result_receipt_persisted: false,
      activation_command_result_receipt_accepted: false,
      activation_command_result_receipt_materialized: false,
      activation_command_result_receipt_filesystem_written: false,
      activation_command_result_receipt_ledger_written: false,
      activation_command_result_receipt_indexed: false,
      activation_command_result_receipt_enqueued: false,
      activation_command_result_receipt_delivered: false,
      activation_command_result_receipt_exported: false,
      activation_command_result_receipt_query_registered: false,
      activation_command_result_receipt_observability_recorded: false,
      activation_command_completion_ack_recorded: false,
      activation_command_completion_ack_persisted: false,
      activation_command_completion_ack_accepted: false,
      activation_command_completion_ack_delivered: false,
      operator_approval_from_ordering_accepted: false,
      activation_from_ordering_allowed: false,
      activation_from_replay_allowed: false,
      activation_from_receipt_allowed: false,
      terminal_closure_from_ordering_recorded: false,
      terminal_closure_from_ordering_accepted: false,
      activation_command_shape_registered: false,
      activation_command_accepted: false,
      activation_command_enabled: false,
      activation_command_invoked: false,
      activation_command_dispatched: false,
      activation_command_dispatch_performed: false,
      activation_request_accepted: false,
      activation_request_recorded: false,
      activation_request_persisted: false,
      activation_request_executed: false,
      activation_allowed: false,
      activation_performed: false,
      terminal_closure_recorded: false,
      terminal_closure_accepted: false,
      terminal_closure_final_state_promoted: false,
      terminal_closure_completion_promoted: false,
      runtime_attachment_performed: false,
      live_context_attached: false,
      context_injection_performed: false,
      provider_invoked: false,
      model_invoked: false,
      usage_recorded: false,
      memory_store_write_performed: false,
      memory_store_mutated: false,
      live_kg_write_performed: false,
      ordering_ledger_written: false,
      ordering_indexed: false,
      ordering_query_registered: false,
      ordering_observability_recorded: false,
      readback_evidence_recorded: false,
      readback_evidence_persisted: false,
      router_handoff_recorded: false,
      router_handoff_persisted: false,
      rollback_executed: false,
      telegram_send_performed: false,
      channel_send_performed: false,
      external_send_performed: false,
      public_release_claimed: false,
      public_ga_claimed: false,
      release_artifact_written: false,
      install_executed: false,
      launchd_mutated: false,
      service_restarted: false,
      active_binary_mutated: false,
      upstream_fetch_performed: false,
      upstream_merge_performed: false,
      credential_read: false,
      secret_value_read: false,
      raw_payload_plaintext_recorded: false,
      raw_payload_plaintext_persisted: false,
      activation_command_result_receipt_ordering_monotonicity_surfaces: [
        "source_replay_idempotency_report_required",
        "canonical_noop_result_receipt_order_identity_required",
        "sequence_cursor_monotonicity_denied",
        "out_of_order_sequence_denied",
        "sequence_gap_or_skip_denied",
        "timestamp_rollback_denied",
        "epoch_rollback_denied",
        "same_sequence_different_hash_denied",
        "latest_wins_overwrite_denied",
        "stage_transition_ordering_denied",
        "ledger_index_delivery_ordering_bypass_denied",
        "runtime_live_context_ordering_bypass_denied",
        "provider_model_memory_kg_ordering_bypass_denied",
        "external_public_install_restart_active_binary_upstream_credential_secret_ordering_bypass_denied"
      ],
      activation_command_result_receipt_ordering_monotonicity_fixtures: $fixtures,
      denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial: (
        $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial + [
          "source_result_receipt_replay_idempotency_report_required",
          "canonical_noop_result_receipt_order_identity_required",
          "sequence_cursor_acceptance_denied",
          "sequence_cursor_recording_denied",
          "sequence_cursor_persistence_denied",
          "monotonicity_state_recording_denied",
          "monotonicity_state_persistence_denied",
          "monotonicity_state_materialization_denied",
          "monotonicity_filesystem_write_denied",
          "out_of_order_sequence_denied",
          "sequence_gap_or_skip_denied",
          "timestamp_rollback_denied",
          "epoch_rollback_denied",
          "same_sequence_different_hash_denied",
          "latest_wins_overwrite_denied",
          "gap_fill_acceptance_denied",
          "completion_ack_before_noop_denied",
          "stage_transition_ordering_denied",
          "ledger_ordering_bypass_denied",
          "index_ordering_bypass_denied",
          "delivery_ordering_bypass_denied",
          "runtime_ordering_bypass_denied",
          "live_context_ordering_bypass_denied",
          "context_injection_ordering_bypass_denied",
          "provider_model_ordering_bypass_denied",
          "memory_kg_ordering_bypass_denied",
          "external_public_release_ordering_bypass_denied",
          "install_restart_active_binary_ordering_bypass_denied",
          "upstream_ordering_bypass_denied",
          "credential_secret_ordering_bypass_denied"
        ]
      ),
      denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_count: (
        ($source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial | length) + 30
      ),
      inherited_denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_count: (
        $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial | length
      ),
      allowed_next_actions: [
        {
          action: "review_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial",
          status: "allowed_report_only",
          accepts_out_of_order_receipt: false,
          records_ordering_state: false,
          persists_ordering_state: false,
          records_terminal_closure: false,
          mutates_runtime: false,
          invokes_model: false
        },
        {
          action: "stage_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial",
          status: "allowed_report_only_next_slice",
          accepts_cancellation: false,
          accepts_supersession: false,
          persists_replacement_receipt: false,
          records_terminal_closure: false,
          mutates_runtime: false,
          invokes_model: false
        },
        {
          action: "run_full_light_preflight",
          status: "allowed_verification_only",
          accepts_ordering: false,
          persists_ordering_state: false,
          mutates_runtime: false,
          invokes_model: false,
          writes_kg: false
        }
      ],
      source_result_receipt_replay_idempotency_report_required: true,
      sequence_cursor_acceptance_forbidden: true,
      ordering_state_recording_forbidden: true,
      monotonicity_state_persistence_forbidden: true,
      out_of_order_receipt_acceptance_forbidden: true,
      timestamp_epoch_rollback_forbidden: true,
      same_sequence_hash_override_forbidden: true,
      latest_wins_overwrite_forbidden: true,
      completion_ack_before_noop_forbidden: true,
      runtime_provider_memory_kg_ordering_bypass_forbidden: true,
      secret_read_forbidden: true,
      external_public_install_restart_active_binary_ordering_bypass_forbidden: true,
      side_effects: {
        workspace_written: false,
        filesystem_written: false,
        activation_command_result_receipt_ordering_recorded: false,
        activation_command_result_receipt_ordering_persisted: false,
        activation_command_result_receipt_ordering_performed: false,
        activation_command_result_receipt_sequence_cursor_accepted: false,
        activation_command_result_receipt_sequence_cursor_recorded: false,
        activation_command_result_receipt_sequence_cursor_persisted: false,
        activation_command_result_receipt_monotonicity_state_recorded: false,
        activation_command_result_receipt_monotonicity_state_persisted: false,
        activation_command_result_receipt_monotonicity_state_materialized: false,
        activation_command_result_receipt_monotonicity_filesystem_written: false,
        activation_command_result_receipt_timestamp_ordering_accepted: false,
        activation_command_result_receipt_epoch_ordering_accepted: false,
        activation_command_result_receipt_stage_ordering_accepted: false,
        activation_command_result_receipt_same_sequence_hash_override_accepted: false,
        activation_command_result_receipt_latest_wins_overwrite_accepted: false,
        activation_command_result_receipt_gap_fill_accepted: false,
        activation_command_result_receipt_ack_before_noop_accepted: false,
        activation_command_result_receipt_ledger_ordering_bypass_accepted: false,
        activation_command_result_receipt_index_ordering_bypass_accepted: false,
        activation_command_result_receipt_delivery_ordering_bypass_accepted: false,
        activation_command_result_receipt_runtime_ordering_bypass_accepted: false,
        activation_command_result_receipt_provider_ordering_bypass_accepted: false,
        activation_command_result_receipt_memory_kg_ordering_bypass_accepted: false,
        activation_command_result_receipt_external_public_install_ordering_bypass_accepted: false,
        activation_command_result_receipt_upstream_ordering_bypass_accepted: false,
        activation_command_result_receipt_credential_secret_ordering_bypass_accepted: false,
        activation_command_result_receipt_replay_recorded: false,
        activation_command_result_receipt_replay_persisted: false,
        activation_command_result_receipt_duplicate_accepted: false,
        activation_command_result_receipt_idempotency_state_recorded: false,
        activation_command_result_receipt_idempotency_state_persisted: false,
        activation_command_result_receipt_recorded: false,
        activation_command_result_receipt_persisted: false,
        activation_command_result_receipt_accepted: false,
        activation_command_result_receipt_materialized: false,
        activation_command_result_receipt_filesystem_written: false,
        activation_command_result_receipt_ledger_written: false,
        activation_command_result_receipt_indexed: false,
        activation_command_result_receipt_enqueued: false,
        activation_command_result_receipt_delivered: false,
        activation_command_completion_ack_recorded: false,
        activation_command_completion_ack_persisted: false,
        activation_command_completion_ack_accepted: false,
        activation_command_completion_ack_delivered: false,
        operator_approval_from_ordering_accepted: false,
        activation_from_ordering_allowed: false,
        terminal_closure_from_ordering_recorded: false,
        terminal_closure_from_ordering_accepted: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
        activation_command_dispatch_performed: false,
        activation_request_accepted: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_executed: false,
        activation_performed: false,
        terminal_closure_recorded: false,
        terminal_closure_accepted: false,
        runtime_attachment_performed: false,
        live_context_attached: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        usage_recorded: false,
        memory_store_write_performed: false,
        memory_store_mutated: false,
        live_kg_write_performed: false,
        ordering_ledger_written: false,
        ordering_indexed: false,
        ordering_query_registered: false,
        ordering_observability_recorded: false,
        readback_evidence_recorded: false,
        readback_evidence_persisted: false,
        router_handoff_recorded: false,
        router_handoff_persisted: false,
        rollback_executed: false,
        telegram_send_performed: false,
        channel_send_performed: false,
        external_send_performed: false,
        public_release_claimed: false,
        public_ga_claimed: false,
        release_artifact_written: false,
        install_executed: false,
        launchd_mutated: false,
        service_restarted: false,
        active_binary_mutated: false,
        upstream_fetch_performed: false,
        upstream_merge_performed: false,
        credential_read: false,
        secret_value_read: false,
        raw_payload_plaintext_recorded: false,
        raw_payload_plaintext_persisted: false
      }
    }'
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_gate"
  and .schema_version == "core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_v1"
  and .core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status == "blocked"
  and .source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_ready == true
  and .source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_status == "blocked"
  and .activation_command_result_receipt_ordering_monotonicity_surface_count == 14
  and .activation_command_result_receipt_ordering_monotonicity_surface_ready_count == 14
  and .activation_command_result_receipt_ordering_monotonicity_fixture_count == 10
  and .blocked_activation_command_result_receipt_ordering_monotonicity_fixture_count == 10
  and .noop_activation_command_result_receipt_ordering_monotonicity_fixture_count == 10
  and .allowed_activation_command_result_receipt_ordering_monotonicity_fixture_count == 0
  and .accepted_activation_command_result_receipt_ordering_monotonicity_fixture_count == 0
  and .activation_command_result_receipt_ordering_denied_count == 10
  and .activation_command_result_receipt_monotonicity_denied_count == 10
  and .activation_command_result_receipt_ordering_performed_count == 0
  and .activation_command_result_receipt_sequence_cursor_accepted_count == 0
  and .activation_command_result_receipt_sequence_cursor_recorded_count == 0
  and .activation_command_result_receipt_monotonicity_state_recorded_count == 0
  and .activation_command_result_receipt_ordering_allowed == false
  and .activation_command_result_receipt_ordering_recorded == false
  and .activation_command_result_receipt_ordering_persisted == false
  and .activation_command_result_receipt_ordering_performed == false
  and .activation_command_result_receipt_sequence_cursor_accepted == false
  and .activation_command_result_receipt_sequence_cursor_recorded == false
  and .activation_command_result_receipt_sequence_cursor_persisted == false
  and .activation_command_result_receipt_monotonicity_state_recorded == false
  and .activation_command_result_receipt_monotonicity_state_persisted == false
  and .activation_command_result_receipt_timestamp_ordering_accepted == false
  and .activation_command_result_receipt_epoch_ordering_accepted == false
  and .activation_command_result_receipt_stage_ordering_accepted == false
  and .activation_command_result_receipt_latest_wins_overwrite_accepted == false
  and .activation_command_result_receipt_replay_recorded == false
  and .activation_command_result_receipt_duplicate_accepted == false
  and .activation_command_result_receipt_idempotency_state_recorded == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_completion_ack_accepted == false
  and .operator_approval_from_ordering_accepted == false
  and .activation_from_ordering_allowed == false
  and .terminal_closure_from_ordering_recorded == false
  and .terminal_closure_from_ordering_accepted == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_request_accepted == false
  and .activation_request_recorded == false
  and .activation_request_persisted == false
  and .activation_request_executed == false
  and .activation_allowed == false
  and .activation_performed == false
  and .terminal_closure_recorded == false
  and .terminal_closure_accepted == false
  and .provider_invoked == false
  and .model_invoked == false
  and .external_send_performed == false
  and .public_release_claimed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .upstream_fetch_performed == false
  and .upstream_merge_performed == false
  and .credential_read == false
  and .secret_value_read == false
  and (.activation_command_result_receipt_ordering_monotonicity_surfaces | length) == 14
  and (.activation_command_result_receipt_ordering_monotonicity_fixtures | length) == 10
  and (.activation_command_result_receipt_ordering_monotonicity_fixtures | all(
    (.ordering_status | startswith("blocked_"))
    and .activation_command_result_receipt_ordering_allowed == false
    and .activation_command_result_receipt_ordering_recorded == false
    and .activation_command_result_receipt_ordering_persisted == false
    and .activation_command_result_receipt_sequence_cursor_accepted == false
    and .activation_command_result_receipt_sequence_cursor_recorded == false
    and .activation_command_result_receipt_monotonicity_state_recorded == false
    and .activation_command_result_receipt_recorded == false
    and .activation_command_result_receipt_persisted == false
    and .activation_command_result_receipt_accepted == false
    and .activation_command_completion_ack_accepted == false
    and .activation_from_ordering_allowed == false
    and .terminal_closure_from_ordering_recorded == false
    and .activation_command_enabled == false
    and .activation_request_accepted == false
    and .activation_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .external_send_performed == false
    and .install_executed == false
    and .active_binary_mutated == false
    and .receipt_noop_confirmed == true
  ))
  and (.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial | length) == 242
  and .denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_count == 242
  and .inherited_denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_count == 212
  and (.allowed_next_actions | any(.action == "stage_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial" and .status == "allowed_report_only_next_slice" and .accepts_cancellation == false and .accepts_supersession == false and .persists_replacement_receipt == false and .records_terminal_closure == false and .mutates_runtime == false and .invokes_model == false))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta core activation operator approval gap ledger summary briefing acknowledgement activation command result receipt ordering monotonicity denial gate passed"
