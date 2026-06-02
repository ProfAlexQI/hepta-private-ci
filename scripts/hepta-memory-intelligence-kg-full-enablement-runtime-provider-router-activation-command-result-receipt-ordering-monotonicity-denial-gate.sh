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
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial-gate.sh
)"

ordering_monotonicity_fixtures_json="$(
  jq -n '
    def ordering_fixture($id; $status; $reason; $extra):
      {
        id:$id,
        ordering_requested:true,
        ordering_status:$status,
        source_replay_idempotency_present:true,
        source_replay_idempotency_ready:true,
        canonical_noop_result_receipt_order_identity_required:true,
        activation_command_result_receipt_ordering_allowed:false,
        activation_command_result_receipt_ordering_recorded:false,
        activation_command_result_receipt_ordering_persisted:false,
        activation_command_result_receipt_ordering_performed:false,
        activation_command_result_receipt_sequence_cursor_accepted:false,
        activation_command_result_receipt_sequence_cursor_recorded:false,
        activation_command_result_receipt_sequence_cursor_persisted:false,
        activation_command_result_receipt_monotonicity_state_recorded:false,
        activation_command_result_receipt_monotonicity_state_persisted:false,
        activation_command_result_receipt_monotonicity_state_materialized:false,
        activation_command_result_receipt_monotonicity_filesystem_written:false,
        activation_command_result_receipt_timestamp_ordering_accepted:false,
        activation_command_result_receipt_epoch_ordering_accepted:false,
        activation_command_result_receipt_stage_ordering_accepted:false,
        activation_command_result_receipt_same_sequence_hash_override_accepted:false,
        activation_command_result_receipt_latest_wins_overwrite_accepted:false,
        activation_command_result_receipt_gap_fill_accepted:false,
        activation_command_result_receipt_ack_before_noop_accepted:false,
        activation_command_result_receipt_ledger_ordering_bypass_accepted:false,
        activation_command_result_receipt_index_ordering_bypass_accepted:false,
        activation_command_result_receipt_delivery_ordering_bypass_accepted:false,
        activation_command_result_receipt_runtime_ordering_bypass_accepted:false,
        activation_command_result_receipt_provider_ordering_bypass_accepted:false,
        activation_command_result_receipt_memory_kg_ordering_bypass_accepted:false,
        activation_command_result_receipt_external_public_install_ordering_bypass_accepted:false,
        activation_command_result_receipt_replay_allowed:false,
        activation_command_result_receipt_replay_recorded:false,
        activation_command_result_receipt_replay_persisted:false,
        activation_command_result_receipt_duplicate_accepted:false,
        activation_command_result_receipt_idempotency_key_accepted:false,
        activation_command_result_receipt_idempotency_state_recorded:false,
        activation_command_result_receipt_idempotency_state_persisted:false,
        activation_command_result_receipt_recorded:false,
        activation_command_result_receipt_persisted:false,
        activation_command_result_receipt_accepted:false,
        activation_command_result_receipt_materialized:false,
        activation_command_result_receipt_filesystem_written:false,
        activation_command_result_receipt_ledger_written:false,
        activation_command_result_receipt_indexed:false,
        activation_command_result_receipt_enqueued:false,
        activation_command_result_receipt_delivered:false,
        activation_command_result_receipt_exported:false,
        activation_command_result_receipt_query_registered:false,
        activation_command_result_receipt_observability_recorded:false,
        activation_command_completion_ack_recorded:false,
        activation_command_completion_ack_persisted:false,
        activation_command_completion_ack_accepted:false,
        activation_command_completion_ack_delivered:false,
        operator_approval_from_ordering_accepted:false,
        activation_from_ordering_allowed:false,
        activation_from_replay_allowed:false,
        activation_from_receipt_allowed:false,
        activation_command_enabled:false,
        activation_command_invoked:false,
        activation_command_dispatched:false,
        activation_command_dispatch_performed:false,
        activation_request_accepted:false,
        activation_request_recorded:false,
        activation_request_persisted:false,
        activation_request_executed:false,
        activation_activated:false,
        runtime_router_mutated:false,
        runtime_attachment_performed:false,
        live_context_attached:false,
        context_injection_performed:false,
        adapter_invoked:false,
        provider_invoked:false,
        model_invoked:false,
        auth_secret_read:false,
        credential_read:false,
        secret_file_read:false,
        usage_recorded:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        ordering_ledger_written:false,
        ordering_indexed:false,
        ordering_query_registered:false,
        ordering_observability_recorded:false,
        receipt_noop_confirmed:true,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        readback_evidence_recorded:false,
        readback_evidence_persisted:false,
        router_handoff_recorded:false,
        router_handoff_persisted:false,
        rollback_executed:false,
        telegram_send_performed:false,
        channel_send_performed:false,
        external_send_performed:false,
        public_release_claimed:false,
        public_ga_claimed:false,
        release_artifact_written:false,
        install_executed:false,
        launchd_mutated:false,
        service_restart_performed:false,
        active_binary_mutated:false,
        reason:$reason
      } + $extra;
    [
      ordering_fixture("provider-router-activation-command-result-receipt-ordering-missing-source-replay-idempotency-report"; "blocked_noop"; "source_result_receipt_replay_idempotency_report_required"; {source_replay_idempotency_present:false, source_replay_idempotency_ready:false}),
      ordering_fixture("provider-router-activation-command-result-receipt-sequence-cursor-recording-attempt"; "blocked_ordering_noop"; "sequence_cursor_recording_denied"; {sequence_cursor_recording_requested:true, requested_sequence_cursor:"provider_router_activation_receipt_sequence_1"}),
      ordering_fixture("provider-router-activation-command-result-receipt-out-of-order-sequence-attempt"; "blocked_ordering_noop"; "out_of_order_result_receipt_sequence_denied"; {out_of_order_sequence_requested:true, requested_sequence:2, observed_previous_sequence:3}),
      ordering_fixture("provider-router-activation-command-result-receipt-sequence-gap-skip-attempt"; "blocked_ordering_noop"; "sequence_gap_or_skip_result_receipt_denied"; {sequence_gap_requested:true, requested_sequence:5, expected_next_sequence:1}),
      ordering_fixture("provider-router-activation-command-result-receipt-timestamp-rollback-attempt"; "blocked_ordering_noop"; "timestamp_rollback_result_receipt_denied"; {timestamp_rollback_requested:true, requested_timestamp_order:"older_than_source_replay_idempotency_report"}),
      ordering_fixture("provider-router-activation-command-result-receipt-epoch-rollback-attempt"; "blocked_ordering_noop"; "epoch_rollback_result_receipt_denied"; {epoch_rollback_requested:true, requested_epoch_order:"lower_than_current_activation_epoch"}),
      ordering_fixture("provider-router-activation-command-result-receipt-same-sequence-different-hash-attempt"; "blocked_ordering_noop"; "same_sequence_different_hash_result_receipt_denied"; {same_sequence_different_hash_requested:true, requested_sequence:1, requested_hash_relation:"different_hash_for_same_sequence"}),
      ordering_fixture("provider-router-activation-command-result-receipt-latest-wins-overwrite-attempt"; "blocked_ordering_noop"; "latest_wins_result_receipt_overwrite_denied"; {latest_wins_overwrite_requested:true, overwrite_existing_noop_requested:true}),
      ordering_fixture("provider-router-activation-command-result-receipt-stage-ledger-index-delivery-ordering-bypass-attempt"; "blocked_ordering_noop"; "stage_ledger_index_delivery_ordering_bypass_denied"; {stage_transition_ordering_bypass_requested:true, completion_ack_before_noop_requested:true, ledger_ordering_bypass_requested:true, index_ordering_bypass_requested:true, delivery_ordering_bypass_requested:true}),
      ordering_fixture("provider-router-activation-command-result-receipt-runtime-provider-memory-kg-external-ordering-bypass-attempt"; "blocked_ordering_noop"; "runtime_provider_memory_kg_external_ordering_bypass_denied"; {runtime_ordering_bypass_requested:true, provider_ordering_bypass_requested:true, model_ordering_bypass_requested:true, memory_store_ordering_bypass_requested:true, live_kg_ordering_bypass_requested:true, external_send_ordering_bypass_requested:true, public_claim_ordering_bypass_requested:true, install_ordering_bypass_requested:true, service_restart_ordering_bypass_requested:true, active_binary_mutation_ordering_bypass_requested:true})
    ]
  '
)"

replay_idempotency_report_sha256="$(sha256_text "$REPLAY_IDEMPOTENCY_JSON")"
ordering_monotonicity_fixtures_sha256="$(sha256_text "$ordering_monotonicity_fixtures_json")"
ordering_monotonicity_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial:$replay_idempotency_report_sha256:$ordering_monotonicity_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
ordering_monotonicity_policy_hash_sha256="$(
  sha256_text "runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial:no-ordering:no-monotonicity-record:no-cursor:no-stage-bypass:no-runtime:no-provider:no-model:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "ordering=false;monotonicity=false;cursor=false;record=false;persist=false;activation=false;runtime=false;provider=false;model=false;memory=false;kg=false;secret=false;external=false;install=false;restart=false;active_binary=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$REPLAY_IDEMPOTENCY_JSON" \
  --argjson fixtures "$ordering_monotonicity_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_gate"
    and $source.activation_command_result_receipt_replay_idempotency_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_v1"
    and $source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status == "blocked"
    and $source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_no_persistence_status == "blocked"
    and $source.activation_command_result_receipt_surface_count == 14
    and $source.activation_command_result_receipt_surface_ready_count == 14
    and $source.activation_command_result_receipt_fixture_count == 10
    and $source.replay_idempotency_surface_count == 14
    and $source.replay_idempotency_surface_ready_count == 14
    and $source.replay_idempotency_fixture_count == 10
    and $source.blocked_replay_idempotency_fixture_count == 10
    and $source.noop_replay_idempotency_fixture_count == 10
    and $source.allowed_replay_idempotency_fixture_count == 0
    and $source.accepted_replay_idempotency_fixture_count == 0
    and $source.replay_idempotency_denied_count == 10
    and $source.replay_idempotency_performed_count == 0
    and $source.duplicate_result_receipt_accepted_count == 0
    and $source.idempotency_state_recorded_count == 0
    and $source.activation_command_result_receipt_replay_allowed == false
    and $source.activation_command_result_receipt_replay_recorded == false
    and $source.activation_command_result_receipt_replay_persisted == false
    and $source.activation_command_result_receipt_replay_performed == false
    and $source.activation_command_result_receipt_duplicate_accepted == false
    and $source.activation_command_result_receipt_idempotency_key_accepted == false
    and $source.activation_command_result_receipt_idempotency_state_recorded == false
    and $source.activation_command_result_receipt_idempotency_state_persisted == false
    and $source.activation_command_result_receipt_status_upgrade_accepted == false
    and $source.activation_command_result_receipt_completed_status_accepted == false
    and $source.activation_command_result_receipt_ack_replay_accepted == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_result_receipt_materialized == false
    and $source.activation_command_result_receipt_filesystem_written == false
    and $source.activation_command_completion_ack_recorded == false
    and $source.operator_approval_from_replay_accepted == false
    and $source.activation_from_replay_allowed == false
    and $source.activation_from_receipt_allowed == false
    and $source.activation_command_enabled == false
    and $source.activation_command_invoked == false
    and $source.activation_command_dispatched == false
    and $source.activation_request_accepted == false
    and $source.activation_request_recorded == false
    and $source.activation_request_persisted == false
    and $source.activation_request_executed == false
    and $source.activation_activated == false
    and $source.runtime_router_mutated == false
    and $source.runtime_attachment_performed == false
    and $source.live_context_attached == false
    and $source.context_injection_performed == false
    and $source.adapter_invoked == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.auth_secret_read == false
    and $source.credential_read == false
    and $source.secret_file_read == false
    and $source.usage_recorded == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_mutated == false
    and $source.live_kg_write_performed == false
    and $source.receipt_recorded == false
    and $source.receipt_persisted == false
    and $source.receipt_accepted == false
    and $source.rollback_executed == false
    and $source.telegram_send_performed == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.public_release_claimed == false
    and $source.public_ga_claimed == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.launchd_mutated == false
    and $source.service_restart_performed == false
    and $source.active_binary_mutated == false
    and ($source.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial" and .status == "allowed_report_only_next_slice" and .accepts_out_of_order_receipt == false and .records_monotonic_clock == false and .persists_ordering_state == false and .mutates_runtime == false and .invokes_model == false))
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
      and .activation_command_completion_ack_recorded == false
      and .operator_approval_from_ordering_accepted == false
      and .activation_from_ordering_allowed == false
      and .activation_command_enabled == false
      and .activation_command_invoked == false
      and .activation_command_dispatched == false
      and .activation_request_accepted == false
      and .activation_request_recorded == false
      and .activation_request_persisted == false
      and .activation_request_executed == false
      and .activation_activated == false
      and .runtime_router_mutated == false
      and .runtime_attachment_performed == false
      and .live_context_attached == false
      and .context_injection_performed == false
      and .adapter_invoked == false
      and .provider_invoked == false
      and .model_invoked == false
      and .auth_secret_read == false
      and .credential_read == false
      and .secret_file_read == false
      and .usage_recorded == false
      and .memory_store_write_performed == false
      and .memory_store_mutated == false
      and .live_kg_write_performed == false
      and .receipt_recorded == false
      and .receipt_persisted == false
      and .receipt_accepted == false
      and .rollback_executed == false
      and .telegram_send_performed == false
      and .channel_send_performed == false
      and .external_send_performed == false
      and .public_release_claimed == false
      and .public_ga_claimed == false
      and .release_artifact_written == false
      and .install_executed == false
      and .launchd_mutated == false
      and .service_restart_performed == false
      and .active_binary_mutated == false
      and .receipt_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_gate" \
  --arg replay_idempotency_report_sha256 "$replay_idempotency_report_sha256" \
  --arg ordering_monotonicity_fixtures_sha256 "$ordering_monotonicity_fixtures_sha256" \
  --arg ordering_monotonicity_contract_hash_sha256 "$ordering_monotonicity_contract_hash_sha256" \
  --arg ordering_monotonicity_policy_hash_sha256 "$ordering_monotonicity_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$REPLAY_IDEMPOTENCY_JSON" \
  --argjson fixtures "$ordering_monotonicity_fixtures_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_ordering_monotonicity_schema_version:"memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_v1",
    activation_command_result_receipt_ordering_monotonicity_mode:"runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_no_ordering_no_monotonicity_persist",
    source_activation_command_result_receipt_replay_idempotency_gate:$source.gate,
    source_activation_command_result_receipt_replay_idempotency_ready:$source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready,
    source_activation_command_result_receipt_replay_idempotency_status:$source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status,
    source_activation_command_result_receipt_replay_idempotency_report_sha256:$replay_idempotency_report_sha256,
    source_activation_command_result_receipt_no_persistence_gate:$source.source_activation_command_result_receipt_no_persistence_gate,
    source_activation_command_result_receipt_no_persistence_ready:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready,
    source_activation_command_result_receipt_no_persistence_status:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_status,
    source_activation_command_result_receipt_no_persistence_report_sha256:$source.source_activation_command_result_receipt_no_persistence_report_sha256,
    source_activation_command_noop_handoff_gate:$source.source_activation_command_noop_handoff_gate,
    source_activation_command_noop_handoff_ready:$source.source_activation_command_noop_handoff_ready,
    source_activation_command_noop_handoff_status:$source.source_activation_command_noop_handoff_status,
    source_activation_command_noop_handoff_report_sha256:$source.source_activation_command_noop_handoff_report_sha256,
    ordering_monotonicity_fixtures_sha256:$ordering_monotonicity_fixtures_sha256,
    ordering_monotonicity_contract_hash_sha256:$ordering_monotonicity_contract_hash_sha256,
    ordering_monotonicity_policy_hash_sha256:$ordering_monotonicity_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready:true,
    runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_status:"blocked",
    runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready,
    runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status:$source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status,
    runtime_provider_router_activation_command_result_receipt_no_persistence_ready:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready,
    runtime_provider_router_activation_command_result_receipt_no_persistence_status:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_status,
    activation_command_result_receipt_surface_count:$source.activation_command_result_receipt_surface_count,
    activation_command_result_receipt_surface_ready_count:$source.activation_command_result_receipt_surface_ready_count,
    activation_command_result_receipt_fixture_count:$source.activation_command_result_receipt_fixture_count,
    replay_idempotency_surface_count:$source.replay_idempotency_surface_count,
    replay_idempotency_surface_ready_count:$source.replay_idempotency_surface_ready_count,
    replay_idempotency_fixture_count:$source.replay_idempotency_fixture_count,
    ordering_monotonicity_surface_count:14,
    ordering_monotonicity_surface_ready_count:14,
    ordering_monotonicity_side_effect_free_surface_count:14,
    ordering_monotonicity_fixture_count:($fixtures | length),
    blocked_ordering_monotonicity_fixture_count:($fixtures | length),
    noop_ordering_monotonicity_fixture_count:($fixtures | length),
    allowed_ordering_monotonicity_fixture_count:0,
    accepted_ordering_monotonicity_fixture_count:0,
    sequence_cursor_recording_fixture_count:1,
    out_of_order_sequence_fixture_count:1,
    sequence_gap_fixture_count:1,
    timestamp_rollback_fixture_count:1,
    epoch_rollback_fixture_count:1,
    same_sequence_hash_fixture_count:1,
    latest_wins_overwrite_fixture_count:1,
    stage_ledger_delivery_bypass_fixture_count:1,
    runtime_provider_memory_kg_external_bypass_fixture_count:1,
    ordering_monotonicity_denied_count:10,
    sequence_cursor_denied_count:10,
    monotonicity_state_denied_count:10,
    ordering_monotonicity_performed_count:0,
    sequence_cursor_accepted_count:0,
    sequence_cursor_recorded_count:0,
    monotonicity_state_recorded_count:0,
    activation_command_result_receipt_ordering_allowed:false,
    activation_command_result_receipt_ordering_recorded:false,
    activation_command_result_receipt_ordering_persisted:false,
    activation_command_result_receipt_ordering_performed:false,
    activation_command_result_receipt_sequence_cursor_accepted:false,
    activation_command_result_receipt_sequence_cursor_recorded:false,
    activation_command_result_receipt_sequence_cursor_persisted:false,
    activation_command_result_receipt_monotonicity_state_recorded:false,
    activation_command_result_receipt_monotonicity_state_persisted:false,
    activation_command_result_receipt_monotonicity_state_materialized:false,
    activation_command_result_receipt_monotonicity_filesystem_written:false,
    activation_command_result_receipt_timestamp_ordering_accepted:false,
    activation_command_result_receipt_epoch_ordering_accepted:false,
    activation_command_result_receipt_stage_ordering_accepted:false,
    activation_command_result_receipt_same_sequence_hash_override_accepted:false,
    activation_command_result_receipt_latest_wins_overwrite_accepted:false,
    activation_command_result_receipt_gap_fill_accepted:false,
    activation_command_result_receipt_ack_before_noop_accepted:false,
    activation_command_result_receipt_ledger_ordering_bypass_accepted:false,
    activation_command_result_receipt_index_ordering_bypass_accepted:false,
    activation_command_result_receipt_delivery_ordering_bypass_accepted:false,
    activation_command_result_receipt_runtime_ordering_bypass_accepted:false,
    activation_command_result_receipt_provider_ordering_bypass_accepted:false,
    activation_command_result_receipt_memory_kg_ordering_bypass_accepted:false,
    activation_command_result_receipt_external_public_install_ordering_bypass_accepted:false,
    activation_command_result_receipt_replay_allowed:false,
    activation_command_result_receipt_replay_recorded:false,
    activation_command_result_receipt_replay_persisted:false,
    activation_command_result_receipt_duplicate_accepted:false,
    activation_command_result_receipt_idempotency_key_accepted:false,
    activation_command_result_receipt_idempotency_state_recorded:false,
    activation_command_result_receipt_idempotency_state_persisted:false,
    activation_command_result_receipt_recorded:false,
    activation_command_result_receipt_persisted:false,
    activation_command_result_receipt_accepted:false,
    activation_command_result_receipt_materialized:false,
    activation_command_result_receipt_filesystem_written:false,
    activation_command_result_receipt_ledger_written:false,
    activation_command_result_receipt_indexed:false,
    activation_command_result_receipt_enqueued:false,
    activation_command_result_receipt_delivered:false,
    activation_command_result_receipt_exported:false,
    activation_command_result_receipt_query_registered:false,
    activation_command_result_receipt_observability_recorded:false,
    activation_command_completion_ack_recorded:false,
    activation_command_completion_ack_persisted:false,
    activation_command_completion_ack_accepted:false,
    activation_command_completion_ack_delivered:false,
    operator_approval_from_ordering_accepted:false,
    activation_from_ordering_allowed:false,
    activation_from_replay_allowed:false,
    activation_from_receipt_allowed:false,
    activation_command_shape_registered:false,
    activation_command_allowed:false,
    activation_command_accepted:false,
    activation_command_enabled:false,
    activation_command_invoked:false,
    activation_command_dispatched:false,
    activation_command_dispatch_performed:false,
    activation_command_noop_decision_recorded:false,
    activation_command_noop_decision_persisted:false,
    activation_command_handoff_recorded:false,
    activation_command_handoff_persisted:false,
    activation_request_allowed:false,
    activation_request_accepted:false,
    activation_request_recorded:false,
    activation_request_persisted:false,
    activation_request_executed:false,
    activation_activated:false,
    runtime_router_mutated:false,
    runtime_attachment_performed:false,
    live_context_attached:false,
    context_injection_performed:false,
    adapter_invoked:false,
    provider_invoked:false,
    model_invoked:false,
    auth_secret_read:false,
    credential_read:false,
    secret_file_read:false,
    usage_recorded:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    live_kg_write_performed:false,
    ordering_ledger_written:false,
    ordering_indexed:false,
    ordering_query_registered:false,
    ordering_observability_recorded:false,
    receipt_recorded:false,
    receipt_persisted:false,
    receipt_accepted:false,
    readback_evidence_recorded:false,
    readback_evidence_persisted:false,
    router_handoff_recorded:false,
    router_handoff_persisted:false,
    rollback_executed:false,
    telegram_send_performed:false,
    channel_send_performed:false,
    external_send_performed:false,
    public_release_claimed:false,
    public_ga_claimed:false,
    release_artifact_written:false,
    install_executed:false,
    launchd_mutated:false,
    service_restart_performed:false,
    active_binary_mutated:false,
    ordering_monotonicity_surfaces:[
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
      "runtime_router_live_context_ordering_bypass_denied",
      "adapter_provider_model_memory_kg_ordering_bypass_denied",
      "external_public_install_restart_active_binary_ordering_bypass_denied"
    ],
    ordering_monotonicity_fixtures:$fixtures,
    denied_by_ordering_monotonicity:[
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
      "ledger_ordering_bypass_denied",
      "index_ordering_bypass_denied",
      "delivery_ordering_bypass_denied",
      "runtime_router_ordering_bypass_denied",
      "live_context_ordering_bypass_denied",
      "adapter_provider_model_ordering_bypass_denied",
      "usage_memory_kg_ordering_bypass_denied",
      "secret_material_ordering_bypass_denied",
      "external_public_release_ordering_bypass_denied",
      "install_restart_active_binary_ordering_bypass_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial",
        status:"allowed_report_only",
        accepts_out_of_order_receipt:false,
        records_monotonic_clock:false,
        persists_ordering_state:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"stage_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial",
        status:"allowed_report_only_next_slice",
        accepts_cancellation:false,
        accepts_supersession:false,
        persists_replacement_receipt:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"run_full_light_preflight",
        status:"allowed_verification_only",
        accepts_ordering:false,
        persists_ordering_state:false,
        mutates_runtime:false,
        invokes_model:false,
        writes_kg:false
      }
    ],
    source_replay_idempotency_report_required:true,
    sequence_cursor_acceptance_forbidden:true,
    ordering_state_recording_forbidden:true,
    monotonicity_state_persistence_forbidden:true,
    out_of_order_receipt_acceptance_forbidden:true,
    timestamp_epoch_rollback_forbidden:true,
    same_sequence_hash_override_forbidden:true,
    latest_wins_overwrite_forbidden:true,
    completion_ack_before_noop_forbidden:true,
    runtime_provider_memory_kg_ordering_bypass_forbidden:true,
    secret_read_forbidden:true,
    external_public_install_restart_active_binary_ordering_bypass_forbidden:true,
    side_effects:{
      activation_command_result_receipt_ordering_recorded:false,
      activation_command_result_receipt_ordering_persisted:false,
      activation_command_result_receipt_ordering_performed:false,
      activation_command_result_receipt_sequence_cursor_accepted:false,
      activation_command_result_receipt_sequence_cursor_recorded:false,
      activation_command_result_receipt_sequence_cursor_persisted:false,
      activation_command_result_receipt_monotonicity_state_recorded:false,
      activation_command_result_receipt_monotonicity_state_persisted:false,
      activation_command_result_receipt_monotonicity_state_materialized:false,
      activation_command_result_receipt_monotonicity_filesystem_written:false,
      activation_command_result_receipt_timestamp_ordering_accepted:false,
      activation_command_result_receipt_epoch_ordering_accepted:false,
      activation_command_result_receipt_stage_ordering_accepted:false,
      activation_command_result_receipt_same_sequence_hash_override_accepted:false,
      activation_command_result_receipt_latest_wins_overwrite_accepted:false,
      activation_command_result_receipt_gap_fill_accepted:false,
      activation_command_result_receipt_ack_before_noop_accepted:false,
      activation_command_result_receipt_ledger_ordering_bypass_accepted:false,
      activation_command_result_receipt_index_ordering_bypass_accepted:false,
      activation_command_result_receipt_delivery_ordering_bypass_accepted:false,
      activation_command_result_receipt_runtime_ordering_bypass_accepted:false,
      activation_command_result_receipt_provider_ordering_bypass_accepted:false,
      activation_command_result_receipt_memory_kg_ordering_bypass_accepted:false,
      activation_command_result_receipt_external_public_install_ordering_bypass_accepted:false,
      activation_command_result_receipt_replay_recorded:false,
      activation_command_result_receipt_replay_persisted:false,
      activation_command_result_receipt_duplicate_accepted:false,
      activation_command_result_receipt_idempotency_state_recorded:false,
      activation_command_result_receipt_idempotency_state_persisted:false,
      activation_command_result_receipt_recorded:false,
      activation_command_result_receipt_persisted:false,
      activation_command_result_receipt_accepted:false,
      activation_command_result_receipt_materialized:false,
      activation_command_result_receipt_filesystem_written:false,
      activation_command_result_receipt_ledger_written:false,
      activation_command_result_receipt_indexed:false,
      activation_command_result_receipt_enqueued:false,
      activation_command_result_receipt_delivered:false,
      activation_command_completion_ack_recorded:false,
      activation_command_completion_ack_persisted:false,
      activation_command_completion_ack_accepted:false,
      activation_command_completion_ack_delivered:false,
      operator_approval_from_ordering_accepted:false,
      activation_from_ordering_allowed:false,
      activation_from_replay_allowed:false,
      activation_from_receipt_allowed:false,
      activation_command_enabled:false,
      activation_command_invoked:false,
      activation_command_dispatched:false,
      activation_command_dispatch_performed:false,
      activation_command_noop_decision_recorded:false,
      activation_command_noop_decision_persisted:false,
      activation_command_handoff_recorded:false,
      activation_command_handoff_persisted:false,
      activation_request_recorded:false,
      activation_request_persisted:false,
      activation_request_executed:false,
      activation_activated:false,
      runtime_router_mutated:false,
      runtime_attachment_performed:false,
      live_context_attached:false,
      context_injection_performed:false,
      adapter_invoked:false,
      provider_invoked:false,
      model_invoked:false,
      auth_secret_read:false,
      credential_read:false,
      secret_file_read:false,
      usage_recorded:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      live_kg_write_performed:false,
      ordering_ledger_written:false,
      ordering_indexed:false,
      ordering_query_registered:false,
      ordering_observability_recorded:false,
      receipt_recorded:false,
      receipt_persisted:false,
      receipt_accepted:false,
      readback_evidence_recorded:false,
      readback_evidence_persisted:false,
      router_handoff_recorded:false,
      router_handoff_persisted:false,
      rollback_executed:false,
      telegram_send_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      filesystem_written:false,
      public_release_claimed:false,
      public_ga_claimed:false,
      release_artifact_written:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_gate"
  and .activation_command_result_receipt_ordering_monotonicity_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_v1"
  and .runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_status == "blocked"
  and .runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status == "blocked"
  and .runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
  and .runtime_provider_router_activation_command_result_receipt_no_persistence_status == "blocked"
  and .minimum_required_samples >= 24
  and .activation_command_result_receipt_surface_count == 14
  and .activation_command_result_receipt_surface_ready_count == 14
  and .activation_command_result_receipt_fixture_count == 10
  and .replay_idempotency_surface_count == 14
  and .replay_idempotency_surface_ready_count == 14
  and .replay_idempotency_fixture_count == 10
  and .ordering_monotonicity_surface_count == 14
  and .ordering_monotonicity_surface_ready_count == 14
  and .ordering_monotonicity_side_effect_free_surface_count == 14
  and .ordering_monotonicity_fixture_count == 10
  and .blocked_ordering_monotonicity_fixture_count == 10
  and .noop_ordering_monotonicity_fixture_count == 10
  and .allowed_ordering_monotonicity_fixture_count == 0
  and .accepted_ordering_monotonicity_fixture_count == 0
  and .sequence_cursor_recording_fixture_count == 1
  and .out_of_order_sequence_fixture_count == 1
  and .sequence_gap_fixture_count == 1
  and .timestamp_rollback_fixture_count == 1
  and .epoch_rollback_fixture_count == 1
  and .same_sequence_hash_fixture_count == 1
  and .latest_wins_overwrite_fixture_count == 1
  and .stage_ledger_delivery_bypass_fixture_count == 1
  and .runtime_provider_memory_kg_external_bypass_fixture_count == 1
  and .ordering_monotonicity_denied_count == 10
  and .sequence_cursor_denied_count == 10
  and .monotonicity_state_denied_count == 10
  and .ordering_monotonicity_performed_count == 0
  and .sequence_cursor_accepted_count == 0
  and .sequence_cursor_recorded_count == 0
  and .monotonicity_state_recorded_count == 0
  and .activation_command_result_receipt_ordering_allowed == false
  and .activation_command_result_receipt_ordering_recorded == false
  and .activation_command_result_receipt_ordering_persisted == false
  and .activation_command_result_receipt_ordering_performed == false
  and .activation_command_result_receipt_sequence_cursor_accepted == false
  and .activation_command_result_receipt_sequence_cursor_recorded == false
  and .activation_command_result_receipt_sequence_cursor_persisted == false
  and .activation_command_result_receipt_monotonicity_state_recorded == false
  and .activation_command_result_receipt_monotonicity_state_persisted == false
  and .activation_command_result_receipt_monotonicity_state_materialized == false
  and .activation_command_result_receipt_monotonicity_filesystem_written == false
  and .activation_command_result_receipt_timestamp_ordering_accepted == false
  and .activation_command_result_receipt_epoch_ordering_accepted == false
  and .activation_command_result_receipt_stage_ordering_accepted == false
  and .activation_command_result_receipt_same_sequence_hash_override_accepted == false
  and .activation_command_result_receipt_latest_wins_overwrite_accepted == false
  and .activation_command_result_receipt_gap_fill_accepted == false
  and .activation_command_result_receipt_ack_before_noop_accepted == false
  and .activation_command_result_receipt_ledger_ordering_bypass_accepted == false
  and .activation_command_result_receipt_index_ordering_bypass_accepted == false
  and .activation_command_result_receipt_delivery_ordering_bypass_accepted == false
  and .activation_command_result_receipt_runtime_ordering_bypass_accepted == false
  and .activation_command_result_receipt_provider_ordering_bypass_accepted == false
  and .activation_command_result_receipt_memory_kg_ordering_bypass_accepted == false
  and .activation_command_result_receipt_external_public_install_ordering_bypass_accepted == false
  and .activation_command_result_receipt_replay_allowed == false
  and .activation_command_result_receipt_replay_recorded == false
  and .activation_command_result_receipt_replay_persisted == false
  and .activation_command_result_receipt_duplicate_accepted == false
  and .activation_command_result_receipt_idempotency_state_recorded == false
  and .activation_command_result_receipt_idempotency_state_persisted == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_result_receipt_materialized == false
  and .activation_command_result_receipt_filesystem_written == false
  and .activation_command_result_receipt_ledger_written == false
  and .activation_command_result_receipt_indexed == false
  and .activation_command_result_receipt_enqueued == false
  and .activation_command_result_receipt_delivered == false
  and .activation_command_result_receipt_exported == false
  and .activation_command_result_receipt_query_registered == false
  and .activation_command_result_receipt_observability_recorded == false
  and .activation_command_completion_ack_recorded == false
  and .activation_command_completion_ack_persisted == false
  and .activation_command_completion_ack_accepted == false
  and .activation_command_completion_ack_delivered == false
  and .operator_approval_from_ordering_accepted == false
  and .activation_from_ordering_allowed == false
  and .activation_from_replay_allowed == false
  and .activation_from_receipt_allowed == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_command_dispatch_performed == false
  and .activation_request_accepted == false
  and .activation_request_recorded == false
  and .activation_request_persisted == false
  and .activation_request_executed == false
  and .activation_activated == false
  and .runtime_router_mutated == false
  and .runtime_attachment_performed == false
  and .live_context_attached == false
  and .context_injection_performed == false
  and .adapter_invoked == false
  and .provider_invoked == false
  and .model_invoked == false
  and .auth_secret_read == false
  and .credential_read == false
  and .secret_file_read == false
  and .usage_recorded == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .ordering_ledger_written == false
  and .ordering_indexed == false
  and .ordering_query_registered == false
  and .ordering_observability_recorded == false
  and .receipt_recorded == false
  and .receipt_persisted == false
  and .receipt_accepted == false
  and .readback_evidence_recorded == false
  and .readback_evidence_persisted == false
  and .router_handoff_recorded == false
  and .router_handoff_persisted == false
  and .rollback_executed == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.ordering_monotonicity_surfaces | length) == 14
  and (.ordering_monotonicity_fixtures | length) == 10
  and (.ordering_monotonicity_fixtures | all(
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
    and .activation_command_completion_ack_recorded == false
    and .operator_approval_from_ordering_accepted == false
    and .activation_from_ordering_allowed == false
    and .activation_command_enabled == false
    and .activation_command_invoked == false
    and .activation_command_dispatched == false
    and .activation_request_accepted == false
    and .activation_request_recorded == false
    and .activation_request_persisted == false
    and .activation_request_executed == false
    and .activation_activated == false
    and .runtime_router_mutated == false
    and .runtime_attachment_performed == false
    and .live_context_attached == false
    and .context_injection_performed == false
    and .adapter_invoked == false
    and .provider_invoked == false
    and .model_invoked == false
    and .auth_secret_read == false
    and .credential_read == false
    and .secret_file_read == false
    and .usage_recorded == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .receipt_recorded == false
    and .receipt_persisted == false
    and .receipt_accepted == false
    and .rollback_executed == false
    and .telegram_send_performed == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .public_release_claimed == false
    and .public_ga_claimed == false
    and .release_artifact_written == false
    and .install_executed == false
    and .launchd_mutated == false
    and .service_restart_performed == false
    and .active_binary_mutated == false
    and .receipt_noop_confirmed == true
  ))
  and ([.ordering_monotonicity_fixtures[] | select(.sequence_cursor_recording_requested == true)] | length) == 1
  and ([.ordering_monotonicity_fixtures[] | select(.out_of_order_sequence_requested == true)] | length) == 1
  and ([.ordering_monotonicity_fixtures[] | select(.sequence_gap_requested == true)] | length) == 1
  and ([.ordering_monotonicity_fixtures[] | select(.timestamp_rollback_requested == true)] | length) == 1
  and ([.ordering_monotonicity_fixtures[] | select(.epoch_rollback_requested == true)] | length) == 1
  and ([.ordering_monotonicity_fixtures[] | select(.same_sequence_different_hash_requested == true)] | length) == 1
  and ([.ordering_monotonicity_fixtures[] | select(.latest_wins_overwrite_requested == true)] | length) == 1
  and ([.ordering_monotonicity_fixtures[] | select(.stage_transition_ordering_bypass_requested == true and .ledger_ordering_bypass_requested == true and .delivery_ordering_bypass_requested == true)] | length) == 1
  and ([.ordering_monotonicity_fixtures[] | select(.runtime_ordering_bypass_requested == true and .provider_ordering_bypass_requested == true and .memory_store_ordering_bypass_requested == true and .live_kg_ordering_bypass_requested == true)] | length) == 1
  and (.denied_by_ordering_monotonicity | length) == 26
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial" and .status == "allowed_report_only_next_slice" and .accepts_cancellation == false and .accepts_supersession == false and .persists_replacement_receipt == false and .mutates_runtime == false and .invokes_model == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement runtime provider-router activation command result receipt ordering/monotonicity denial gate passed"
