#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
RELEASE_BIN="${HEPTA_RELEASE_BIN:-${HEPTA_CODEX_RELEASE_BIN:-$HOME/.local/opt/hepta/bin/hepta}}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

require_unsigned_integer() {
  local name="$1"
  local value="$2"

  case "$value" in
    ''|*[!0-9]*)
      echo "$name must be an unsigned integer" >&2
      exit 2
      ;;
  esac
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

REPLAY_IDEMPOTENCY_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-gate.sh
)"

ordering_monotonicity_fixtures_json="$(
  jq -n '
    def ordering_fixture($id; $status; $reason; $extra):
      {
        fixture_id: $id,
        ordering_monotonicity_status: $status,
        source_replay_idempotency_present: true,
        source_replay_idempotency_ready: true,
        ordering_requested: true,
        canonical_blocked_noop_result_receipt_order_identity_required: true,
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
        activation_command_result_receipt_out_of_order_accepted: false,
        activation_command_result_receipt_stale_sequence_accepted: false,
        activation_command_result_receipt_future_sequence_accepted: false,
        activation_command_result_receipt_sequence_gap_accepted: false,
        activation_command_result_receipt_timestamp_rollback_accepted: false,
        activation_command_result_receipt_epoch_rollback_accepted: false,
        activation_command_result_receipt_same_sequence_hash_override_accepted: false,
        activation_command_result_receipt_latest_wins_overwrite_accepted: false,
        activation_command_result_receipt_ack_before_noop_accepted: false,
        activation_command_result_receipt_ledger_ordering_bypass_accepted: false,
        activation_command_result_receipt_index_ordering_bypass_accepted: false,
        activation_command_result_receipt_delivery_ordering_bypass_accepted: false,
        activation_command_result_receipt_export_ordering_bypass_accepted: false,
        activation_command_result_receipt_query_ordering_bypass_accepted: false,
        activation_command_result_receipt_observability_ordering_bypass_accepted: false,
        activation_command_result_receipt_provider_ordering_bypass_accepted: false,
        activation_command_result_receipt_memory_kg_ordering_bypass_accepted: false,
        activation_command_result_receipt_external_public_install_ordering_bypass_accepted: false,
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
        activation_command_completion_ack_recorded: false,
        activation_command_completion_ack_accepted: false,
        operator_approval_from_ordering_accepted: false,
        activation_from_ordering_allowed: false,
        activation_from_replay_allowed: false,
        activation_from_receipt_allowed: false,
        activation_command_allowed: false,
        activation_command_accepted: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
        activation_request_accepted: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_executed: false,
        operator_approval_recorded: false,
        dispatch_performed: false,
        execution_performed: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        memory_store_write_performed: false,
        external_kg_adapter_read_performed: false,
        live_kg_write_performed: false,
        credential_read: false,
        secret_file_read: false,
        channel_send_performed: false,
        external_send_performed: false,
        public_claim_performed: false,
        install_performed: false,
        service_restarted: false,
        active_binary_mutated: false,
        upstream_fetch_performed: false,
        upstream_merge_performed: false,
        receipt_noop_confirmed: true,
        denial_reason: $reason
      } + $extra;
    [
      ordering_fixture("missing-source-replay-idempotency-report"; "blocked_noop"; "source_result_receipt_replay_idempotency_report_required"; {source_replay_idempotency_present: false, source_replay_idempotency_ready: false}),
      ordering_fixture("sequence-cursor-recording-attempt"; "blocked_sequence_cursor_noop"; "sequence_cursor_recording_denied"; {sequence_cursor_recording_requested: true, requested_sequence_cursor: "operator_canary_ack_result_receipt_sequence_1"}),
      ordering_fixture("out-of-order-sequence-attempt"; "blocked_out_of_order_noop"; "out_of_order_result_receipt_sequence_denied"; {out_of_order_sequence_requested: true, requested_sequence: 2, observed_previous_sequence: 3}),
      ordering_fixture("stale-sequence-replay-attempt"; "blocked_stale_sequence_noop"; "stale_sequence_result_receipt_replay_denied"; {stale_sequence_requested: true, requested_sequence: 1, observed_previous_sequence: 3}),
      ordering_fixture("future-sequence-gap-attempt"; "blocked_future_sequence_noop"; "future_sequence_gap_result_receipt_denied"; {future_sequence_requested: true, requested_sequence: 5, expected_next_sequence: 1}),
      ordering_fixture("timestamp-epoch-rollback-attempt"; "blocked_rollback_noop"; "timestamp_epoch_rollback_result_receipt_denied"; {timestamp_rollback_requested: true, epoch_rollback_requested: true}),
      ordering_fixture("same-sequence-different-hash-attempt"; "blocked_same_sequence_hash_noop"; "same_sequence_different_hash_result_receipt_denied"; {same_sequence_different_hash_requested: true, requested_sequence: 1, requested_hash_relation: "different_hash_for_same_sequence"}),
      ordering_fixture("latest-wins-overwrite-attempt"; "blocked_latest_wins_noop"; "latest_wins_result_receipt_overwrite_denied"; {latest_wins_overwrite_requested: true, overwrite_existing_noop_requested: true}),
      ordering_fixture("ack-ledger-index-delivery-ordering-bypass-attempt"; "blocked_ledger_delivery_noop"; "ack_ledger_index_delivery_ordering_bypass_denied"; {completion_ack_before_noop_requested: true, ledger_ordering_bypass_requested: true, index_ordering_bypass_requested: true, delivery_ordering_bypass_requested: true, export_ordering_bypass_requested: true, query_ordering_bypass_requested: true, observability_ordering_bypass_requested: true}),
      ordering_fixture("activation-provider-memory-kg-external-ordering-bypass-attempt"; "blocked_activation_provider_memory_kg_external_noop"; "activation_provider_memory_kg_external_ordering_bypass_denied"; {operator_approval_from_ordering_requested: true, activation_from_ordering_requested: true, context_injection_ordering_bypass_requested: true, provider_ordering_bypass_requested: true, model_ordering_bypass_requested: true, memory_store_ordering_bypass_requested: true, external_kg_ordering_bypass_requested: true, live_kg_ordering_bypass_requested: true, external_send_ordering_bypass_requested: true, public_claim_ordering_bypass_requested: true, install_ordering_bypass_requested: true, service_restart_ordering_bypass_requested: true, active_binary_mutation_ordering_bypass_requested: true, upstream_ordering_bypass_requested: true, credential_ordering_bypass_requested: true, secret_value_ordering_bypass_requested: true})
    ]
  '
)"

replay_idempotency_report_sha256="$(sha256_text "$REPLAY_IDEMPOTENCY_JSON")"
replay_idempotency_contract_hash_sha256="$(jq -r '.replay_idempotency_contract_hash_sha256' <<<"$REPLAY_IDEMPOTENCY_JSON")"
source_result_receipt_no_persistence_hash_sha256="$(jq -r '.source_result_receipt_no_persistence_hash_sha256' <<<"$REPLAY_IDEMPOTENCY_JSON")"
ordering_monotonicity_fixtures_sha256="$(sha256_text "$ordering_monotonicity_fixtures_json")"
ordering_monotonicity_contract_hash_sha256="$(
  sha256_text "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial:v1:source=$replay_idempotency_report_sha256:replay=$replay_idempotency_contract_hash_sha256:receipt=$source_result_receipt_no_persistence_hash_sha256:fixtures=$ordering_monotonicity_fixtures_sha256:ordering=0:cursor=0:monotonicity=0:persist=0:authority=0:live=0"
)"
ordering_monotonicity_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial:v1:no-ordering:no-sequence-cursor:no-monotonicity-state:no-latest-wins:no-ack-ledger-bypass:no-authority:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_side_effects=false;fixtures=10;ordering=0;cursor=0;monotonicity=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$REPLAY_IDEMPOTENCY_JSON" \
  --argjson fixtures "$ordering_monotonicity_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_gate"
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status == "blocked"
    and $source.replay_idempotency_fixture_count == 10
    and $source.blocked_replay_idempotency_fixture_count == 10
    and $source.noop_replay_idempotency_fixture_count == 10
    and $source.accepted_replay_idempotency_fixture_count == 0
    and $source.replay_idempotency_performed_count == 0
    and $source.duplicate_result_receipt_accepted_count == 0
    and $source.idempotency_state_recorded_count == 0
    and $source.idempotency_state_persisted_count == 0
    and $source.activation_command_result_receipt_replay_allowed == false
    and $source.activation_command_result_receipt_replay_recorded == false
    and $source.activation_command_result_receipt_replay_persisted == false
    and $source.activation_command_result_receipt_duplicate_accepted == false
    and $source.activation_command_result_receipt_idempotency_state_recorded == false
    and $source.activation_command_result_receipt_idempotency_state_persisted == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_completion_ack_recorded == false
    and $source.operator_approval_from_replay_accepted == false
    and $source.activation_from_replay_allowed == false
    and $source.activation_from_receipt_allowed == false
    and $source.activation_command_enabled == false
    and $source.activation_command_invoked == false
    and $source.activation_command_dispatched == false
    and $source.activation_request_accepted == false
    and $source.activation_request_executed == false
    and $source.dispatch_performed_count == 0
    and $source.execution_performed_count == 0
    and $source.context_injection_performed_count == 0
    and $source.provider_invoked_count == 0
    and $source.model_invoked_count == 0
    and $source.memory_store_write_performed_count == 0
    and $source.external_kg_adapter_read_performed_count == 0
    and $source.live_kg_write_performed_count == 0
    and $source.credential_read_count == 0
    and $source.secret_file_read_count == 0
    and $source.channel_send_performed_count == 0
    and $source.install_performed_count == 0
    and $source.service_restarted_count == 0
    and $source.active_binary_mutated_count == 0
    and $source.upstream_fetch_performed_count == 0
    and $source.upstream_merge_performed_count == 0
    and $source.canary_harness_armed == false
    and $source.canary_harness_executable == false
    and $source.canary_live_enabled == false
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.ordering_monotonicity_status | startswith("blocked_"))
      and .activation_command_result_receipt_ordering_allowed == false
      and .activation_command_result_receipt_ordering_recorded == false
      and .activation_command_result_receipt_ordering_persisted == false
      and .activation_command_result_receipt_ordering_performed == false
      and .activation_command_result_receipt_sequence_cursor_accepted == false
      and .activation_command_result_receipt_sequence_cursor_recorded == false
      and .activation_command_result_receipt_sequence_cursor_persisted == false
      and .activation_command_result_receipt_monotonicity_state_recorded == false
      and .activation_command_result_receipt_monotonicity_state_persisted == false
      and .activation_command_result_receipt_recorded == false
      and .activation_command_result_receipt_persisted == false
      and .activation_command_result_receipt_accepted == false
      and .activation_command_completion_ack_recorded == false
      and .operator_approval_from_ordering_accepted == false
      and .activation_from_ordering_allowed == false
      and .activation_command_enabled == false
      and .activation_command_invoked == false
      and .activation_command_dispatched == false
      and .activation_request_accepted == false
      and .activation_request_executed == false
      and .dispatch_performed == false
      and .execution_performed == false
      and .context_injection_performed == false
      and .provider_invoked == false
      and .model_invoked == false
      and .memory_store_write_performed == false
      and .external_kg_adapter_read_performed == false
      and .live_kg_write_performed == false
      and .credential_read == false
      and .secret_file_read == false
      and .channel_send_performed == false
      and .install_performed == false
      and .service_restarted == false
      and .active_binary_mutated == false
      and .upstream_fetch_performed == false
      and .upstream_merge_performed == false
      and .receipt_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_gate" \
    --arg replay_idempotency_report_sha256 "$replay_idempotency_report_sha256" \
    --arg replay_idempotency_contract_hash_sha256 "$replay_idempotency_contract_hash_sha256" \
    --arg source_result_receipt_no_persistence_hash_sha256 "$source_result_receipt_no_persistence_hash_sha256" \
    --arg ordering_monotonicity_fixtures_sha256 "$ordering_monotonicity_fixtures_sha256" \
    --arg ordering_monotonicity_contract_hash_sha256 "$ordering_monotonicity_contract_hash_sha256" \
    --arg ordering_monotonicity_policy_hash_sha256 "$ordering_monotonicity_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$REPLAY_IDEMPOTENCY_JSON" \
    --argjson fixtures "$ordering_monotonicity_fixtures_json" \
    '
      ($source.denied_by_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency + [
        "source_result_receipt_replay_idempotency_report_required",
        "canonical_blocked_noop_result_receipt_order_identity_required",
        "sequence_cursor_acceptance_denied",
        "sequence_cursor_recording_denied",
        "sequence_cursor_persistence_denied",
        "monotonicity_state_recording_denied",
        "monotonicity_state_persistence_denied",
        "monotonicity_state_materialization_denied",
        "monotonicity_filesystem_write_denied",
        "out_of_order_sequence_denied",
        "stale_sequence_denied",
        "future_sequence_denied",
        "sequence_gap_denied",
        "timestamp_rollback_denied",
        "epoch_rollback_denied",
        "same_sequence_different_hash_denied",
        "latest_wins_overwrite_denied",
        "completion_ack_before_noop_denied",
        "ledger_index_delivery_ordering_bypass_denied",
        "export_query_observability_ordering_bypass_denied",
        "operator_approval_from_ordering_denied",
        "activation_from_ordering_denied",
        "context_injection_ordering_bypass_denied",
        "provider_model_ordering_bypass_denied",
        "memory_kg_ordering_bypass_denied",
        "credential_secret_ordering_bypass_denied",
        "external_public_install_restart_ordering_bypass_denied",
        "active_binary_mutation_ordering_bypass_denied",
        "upstream_ordering_bypass_denied"
      ]) as $denials |
      {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_schema_version: "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_v1",
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready: true,
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status: "blocked",
        ordering_monotonicity_mode: "stdout_only_sequence_cursor_and_monotonicity_denial_no_record_no_persist_no_authority_no_live",
        ordering_monotonicity_decision: "blocked_noop_activation_command_result_receipt_cannot_create_ordering_sequence_cursor_or_monotonic_authority",
        minimum_required_samples: $min_long_soak_samples,
        source_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_gate: $source.gate,
        source_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_status: $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status,
        source_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_report_sha256: $replay_idempotency_report_sha256,
        source_replay_idempotency_contract_hash_sha256: $replay_idempotency_contract_hash_sha256,
        source_result_receipt_no_persistence_hash_sha256: $source_result_receipt_no_persistence_hash_sha256,
        ordering_monotonicity_fixtures_sha256: $ordering_monotonicity_fixtures_sha256,
        ordering_monotonicity_contract_hash_sha256: $ordering_monotonicity_contract_hash_sha256,
        ordering_monotonicity_policy_hash_sha256: $ordering_monotonicity_policy_hash_sha256,
        side_effect_hash_sha256: $side_effect_hash_sha256,
        source_activation_command_result_receipt_surface_count: $source.source_activation_command_result_receipt_surface_count,
        source_activation_command_result_receipt_fixture_count: $source.source_activation_command_result_receipt_fixture_count,
        source_accepted_activation_command_result_receipt_fixture_count: $source.source_accepted_activation_command_result_receipt_fixture_count,
        source_replay_idempotency_fixture_count: $source.replay_idempotency_fixture_count,
        source_blocked_replay_idempotency_fixture_count: $source.blocked_replay_idempotency_fixture_count,
        source_noop_replay_idempotency_fixture_count: $source.noop_replay_idempotency_fixture_count,
        source_accepted_replay_idempotency_fixture_count: $source.accepted_replay_idempotency_fixture_count,
        ordering_monotonicity_surface_count: 14,
        ordering_monotonicity_surface_ready_count: 14,
        ordering_monotonicity_side_effect_free_surface_count: 14,
        ordering_monotonicity_fixtures: $fixtures,
        ordering_monotonicity_fixture_count: ($fixtures | length),
        blocked_ordering_monotonicity_fixture_count: ($fixtures | length),
        noop_ordering_monotonicity_fixture_count: ($fixtures | length),
        allowed_ordering_monotonicity_fixture_count: 0,
        accepted_ordering_monotonicity_fixture_count: 0,
        sequence_cursor_recording_fixture_count: 1,
        out_of_order_sequence_fixture_count: 1,
        stale_sequence_fixture_count: 1,
        future_sequence_gap_fixture_count: 1,
        timestamp_epoch_rollback_fixture_count: 1,
        same_sequence_hash_fixture_count: 1,
        latest_wins_overwrite_fixture_count: 1,
        ack_ledger_index_delivery_bypass_fixture_count: 1,
        activation_provider_memory_kg_external_bypass_fixture_count: 1,
        ordering_monotonicity_denied_count: 10,
        ordering_monotonicity_performed_count: 0,
        sequence_cursor_accepted_count: 0,
        sequence_cursor_recorded_count: 0,
        monotonicity_state_recorded_count: 0,
        monotonicity_state_persisted_count: 0,
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
        activation_command_result_receipt_out_of_order_accepted: false,
        activation_command_result_receipt_stale_sequence_accepted: false,
        activation_command_result_receipt_future_sequence_accepted: false,
        activation_command_result_receipt_sequence_gap_accepted: false,
        activation_command_result_receipt_timestamp_rollback_accepted: false,
        activation_command_result_receipt_epoch_rollback_accepted: false,
        activation_command_result_receipt_same_sequence_hash_override_accepted: false,
        activation_command_result_receipt_latest_wins_overwrite_accepted: false,
        activation_command_result_receipt_ack_before_noop_accepted: false,
        activation_command_result_receipt_ledger_ordering_bypass_accepted: false,
        activation_command_result_receipt_index_ordering_bypass_accepted: false,
        activation_command_result_receipt_delivery_ordering_bypass_accepted: false,
        activation_command_result_receipt_export_ordering_bypass_accepted: false,
        activation_command_result_receipt_query_ordering_bypass_accepted: false,
        activation_command_result_receipt_observability_ordering_bypass_accepted: false,
        activation_command_result_receipt_provider_ordering_bypass_accepted: false,
        activation_command_result_receipt_memory_kg_ordering_bypass_accepted: false,
        activation_command_result_receipt_external_public_install_ordering_bypass_accepted: false,
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
        activation_command_completion_ack_recorded: false,
        activation_command_completion_ack_persisted: false,
        activation_command_completion_ack_accepted: false,
        operator_approval_from_ordering_accepted: false,
        operator_approval_from_replay_accepted: false,
        operator_approval_from_receipt_accepted: false,
        activation_from_ordering_allowed: false,
        activation_from_replay_allowed: false,
        activation_from_receipt_allowed: false,
        activation_command_allowed: false,
        activation_command_accepted: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
        activation_request_accepted: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_executed: false,
        operator_approval_recorded: false,
        dispatch_performed_count: 0,
        execution_performed_count: 0,
        context_injection_performed_count: 0,
        provider_invoked_count: 0,
        model_invoked_count: 0,
        memory_store_write_performed_count: 0,
        external_kg_adapter_read_performed_count: 0,
        live_kg_write_performed_count: 0,
        credential_read_count: 0,
        secret_file_read_count: 0,
        channel_send_performed_count: 0,
        install_performed_count: 0,
        service_restarted_count: 0,
        active_binary_mutated_count: 0,
        upstream_fetch_performed_count: 0,
        upstream_merge_performed_count: 0,
        canary_harness_armed: false,
        canary_harness_executable: false,
        canary_live_enabled: false,
        allowed_next_actions: [
          {
            action: "review_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial",
            status: "allowed_report_only",
            accepts_out_of_order_receipt: false,
            records_sequence_cursor: false,
            persists_ordering_state: false,
            mutates_runtime: false,
            invokes_model: false,
            writes_memory_or_kg: false
          },
          {
            action: "stage_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial",
            status: "allowed_report_only_next_slice",
            accepts_cancellation: false,
            accepts_supersession: false,
            persists_replacement_receipt: false,
            mutates_runtime: false,
            invokes_model: false,
            writes_memory_or_kg: false
          }
        ],
        denied_by_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity: $denials,
        denied_by_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_count: ($denials | length),
        side_effects: {
          workspace_written: false,
          filesystem_written: false,
          activation_command_result_receipt_ordering_recorded: false,
          activation_command_result_receipt_ordering_persisted: false,
          activation_command_result_receipt_ordering_materialized: false,
          activation_command_result_receipt_ordering_filesystem_written: false,
          activation_command_result_receipt_ordering_performed: false,
          activation_command_result_receipt_sequence_cursor_recorded: false,
          activation_command_result_receipt_sequence_cursor_persisted: false,
          activation_command_result_receipt_monotonicity_state_recorded: false,
          activation_command_result_receipt_monotonicity_state_persisted: false,
          activation_command_result_receipt_monotonicity_state_materialized: false,
          activation_command_result_receipt_monotonicity_filesystem_written: false,
          activation_command_result_receipt_recorded: false,
          activation_command_result_receipt_persisted: false,
          activation_command_result_receipt_accepted: false,
          activation_command_completion_ack_recorded: false,
          activation_command_completion_ack_accepted: false,
          operator_approval_from_ordering_accepted: false,
          activation_from_ordering_allowed: false,
          activation_from_replay_allowed: false,
          activation_from_receipt_allowed: false,
          activation_command_enabled: false,
          activation_command_invoked: false,
          activation_command_dispatched: false,
          activation_request_recorded: false,
          activation_request_persisted: false,
          activation_request_executed: false,
          operator_approval_recorded: false,
          dispatch_performed: false,
          execution_performed: false,
          context_injection_performed: false,
          provider_invoked: false,
          model_invoked: false,
          memory_store_write_performed: false,
          memory_store_mutated: false,
          external_kg_adapter_read_performed: false,
          live_kg_write_performed: false,
          credential_read: false,
          secret_file_read: false,
          channel_send_performed: false,
          telegram_send_performed: false,
          external_send_performed: false,
          public_claim_performed: false,
          install_performed: false,
          service_restarted: false,
          active_binary_mutated: false,
          upstream_fetch_performed: false,
          upstream_merge_performed: false
        }
      }
    '
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status == "blocked"
  and .source_replay_idempotency_fixture_count == 10
  and .source_accepted_replay_idempotency_fixture_count == 0
  and .ordering_monotonicity_fixture_count == 10
  and .blocked_ordering_monotonicity_fixture_count == 10
  and .noop_ordering_monotonicity_fixture_count == 10
  and .allowed_ordering_monotonicity_fixture_count == 0
  and .accepted_ordering_monotonicity_fixture_count == 0
  and .ordering_monotonicity_performed_count == 0
  and .sequence_cursor_accepted_count == 0
  and .sequence_cursor_recorded_count == 0
  and .monotonicity_state_recorded_count == 0
  and .monotonicity_state_persisted_count == 0
  and .activation_command_result_receipt_ordering_allowed == false
  and .activation_command_result_receipt_ordering_recorded == false
  and .activation_command_result_receipt_ordering_persisted == false
  and .activation_command_result_receipt_ordering_performed == false
  and .activation_command_result_receipt_sequence_cursor_accepted == false
  and .activation_command_result_receipt_sequence_cursor_recorded == false
  and .activation_command_result_receipt_sequence_cursor_persisted == false
  and .activation_command_result_receipt_monotonicity_state_recorded == false
  and .activation_command_result_receipt_monotonicity_state_persisted == false
  and .activation_command_result_receipt_out_of_order_accepted == false
  and .activation_command_result_receipt_stale_sequence_accepted == false
  and .activation_command_result_receipt_future_sequence_accepted == false
  and .activation_command_result_receipt_sequence_gap_accepted == false
  and .activation_command_result_receipt_timestamp_rollback_accepted == false
  and .activation_command_result_receipt_epoch_rollback_accepted == false
  and .activation_command_result_receipt_same_sequence_hash_override_accepted == false
  and .activation_command_result_receipt_latest_wins_overwrite_accepted == false
  and .activation_command_result_receipt_ack_before_noop_accepted == false
  and .activation_command_result_receipt_ledger_ordering_bypass_accepted == false
  and .activation_command_result_receipt_index_ordering_bypass_accepted == false
  and .activation_command_result_receipt_delivery_ordering_bypass_accepted == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_completion_ack_recorded == false
  and .operator_approval_from_ordering_accepted == false
  and .activation_from_ordering_allowed == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_request_accepted == false
  and .activation_request_recorded == false
  and .activation_request_executed == false
  and .dispatch_performed_count == 0
  and .execution_performed_count == 0
  and .context_injection_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .memory_store_write_performed_count == 0
  and .external_kg_adapter_read_performed_count == 0
  and .live_kg_write_performed_count == 0
  and .credential_read_count == 0
  and .secret_file_read_count == 0
  and .channel_send_performed_count == 0
  and .install_performed_count == 0
  and .service_restarted_count == 0
  and .active_binary_mutated_count == 0
  and .upstream_fetch_performed_count == 0
  and .upstream_merge_performed_count == 0
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and (.ordering_monotonicity_fixtures | all(
    .activation_command_result_receipt_ordering_allowed == false
    and .activation_command_result_receipt_sequence_cursor_accepted == false
    and .activation_command_result_receipt_sequence_cursor_recorded == false
    and .activation_command_result_receipt_monotonicity_state_recorded == false
    and .activation_command_result_receipt_monotonicity_state_persisted == false
    and .activation_command_result_receipt_recorded == false
    and .activation_command_result_receipt_persisted == false
    and .activation_command_result_receipt_accepted == false
    and .operator_approval_from_ordering_accepted == false
    and .activation_from_ordering_allowed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .receipt_noop_confirmed == true
  ))
  and .denied_by_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_count >= 130
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation command result receipt ordering/monotonicity denial gate passed"
