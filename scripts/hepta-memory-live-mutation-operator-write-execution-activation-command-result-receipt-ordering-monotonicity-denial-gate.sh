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

REPLAY_IDEMPOTENCY_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-replay-idempotency-denial-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-replay-idempotency-denial-gate.sh
)"

replay_idempotency_report_sha256="$(printf '%s' "$REPLAY_IDEMPOTENCY_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson replay_idempotency "$REPLAY_IDEMPOTENCY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $replay_idempotency.runtime == "hepta"
    and $replay_idempotency.status == "ready"
    and $replay_idempotency.gate == "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_replay_idempotency_denial_gate"
    and $replay_idempotency.activation_command_result_receipt_replay_idempotency_mode == "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial"
    and $replay_idempotency.memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $replay_idempotency.memory_write_execution_activation_command_result_receipt_no_persistence_ready == true
    and $replay_idempotency.memory_write_execution_activation_command_noop_handoff_ready == true
    and $replay_idempotency.memory_write_execution_activation_closure_denial_ready == true
    and $replay_idempotency.memory_write_execution_post_write_operator_acceptance_denial_ready == true
    and $replay_idempotency.memory_write_execution_post_write_validation_dry_run_ready == true
    and $replay_idempotency.memory_write_execution_write_enable_fixture_ready == true
    and $replay_idempotency.memory_write_execution_no_write_sink_contract_ready == true
    and $replay_idempotency.source_activation_command_result_receipt_no_persistence_report_sha256 != ""
    and $replay_idempotency.source_activation_command_noop_handoff_report_sha256 != ""
    and $replay_idempotency.source_memory_write_execution_activation_closure_denial_report_sha256 != ""
    and $replay_idempotency.source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256 != ""
    and $replay_idempotency.source_memory_write_execution_post_write_validation_dry_run_report_sha256 != ""
    and $replay_idempotency.source_memory_write_execution_write_enable_fixture_report_sha256 != ""
    and $replay_idempotency.source_memory_write_execution_no_write_sink_contract_report_sha256 != ""
    and $replay_idempotency.source_memory_write_execution_denial_matrix_report_sha256 != ""
    and $replay_idempotency.source_memory_write_execution_preflight_report_sha256 != ""
    and $replay_idempotency.minimum_required_samples >= 24
    and $replay_idempotency.required_activation_command_result_receipt_replay_idempotency_surface_count == 12
    and $replay_idempotency.ready_activation_command_result_receipt_replay_idempotency_surface_count == 12
    and $replay_idempotency.side_effect_free_activation_command_result_receipt_replay_idempotency_surface_count == 12
    and $replay_idempotency.required_activation_command_result_receipt_replay_idempotency_fixture_count == 10
    and $replay_idempotency.activation_command_result_receipt_replay_idempotency_fixture_count == 10
    and $replay_idempotency.blocked_activation_command_result_receipt_replay_idempotency_fixture_count == 10
    and $replay_idempotency.noop_activation_command_result_receipt_replay_idempotency_fixture_count == 10
    and $replay_idempotency.allowed_activation_command_result_receipt_replay_idempotency_fixture_count == 0
    and $replay_idempotency.accepted_activation_command_result_receipt_replay_idempotency_fixture_count == 0
    and $replay_idempotency.activation_command_result_receipt_replay_performed_count == 0
    and $replay_idempotency.activation_command_result_receipt_duplicate_accepted_count == 0
    and $replay_idempotency.activation_command_result_receipt_idempotency_state_recorded_count == 0
    and $replay_idempotency.activation_command_result_receipt_replay_allowed == false
    and $replay_idempotency.activation_command_result_receipt_replay_recorded == false
    and $replay_idempotency.activation_command_result_receipt_replay_persisted == false
    and $replay_idempotency.activation_command_result_receipt_duplicate_accepted == false
    and $replay_idempotency.activation_command_result_receipt_idempotency_key_accepted == false
    and $replay_idempotency.activation_command_result_receipt_idempotency_state_recorded == false
    and $replay_idempotency.activation_command_result_receipt_idempotency_state_persisted == false
    and $replay_idempotency.activation_command_result_receipt_status_upgrade_accepted == false
    and $replay_idempotency.activation_command_result_receipt_completed_status_accepted == false
    and $replay_idempotency.activation_command_result_receipt_recorded == false
    and $replay_idempotency.activation_command_result_receipt_persisted == false
    and $replay_idempotency.activation_command_result_receipt_accepted == false
    and $replay_idempotency.activation_command_completion_ack_recorded == false
    and $replay_idempotency.activation_allowed_by_result_receipt_replay == false
    and $replay_idempotency.activation_allowed == false
    and $replay_idempotency.activation_performed == false
    and $replay_idempotency.live_mutation_execution_performed == false
    and $replay_idempotency.memory_store_write_performed == false
    and $replay_idempotency.memory_store_write_performed_count == 0
    and $replay_idempotency.memory_store_mutated == false
    and $replay_idempotency.rollback_executed == false
    and $replay_idempotency.secret_material_read == false
    and $replay_idempotency.provider_invoked == false
    and $replay_idempotency.model_invoked == false
    and $replay_idempotency.external_send_performed == false
    and $replay_idempotency.public_release_published == false
    and $replay_idempotency.release_artifact_written == false
    and $replay_idempotency.install_executed == false
    and $replay_idempotency.launchd_mutated == false
    and $replay_idempotency.service_restarted == false
    and $replay_idempotency.active_binary_mutated == false
    and ($replay_idempotency.activation_command_result_receipt_replay_idempotency_fixtures | length) == 10
    and ($replay_idempotency.activation_command_result_receipt_replay_idempotency_fixtures | all((.replay_status == "blocked_noop" or .replay_status == "blocked_duplicate_noop") and .replay_allowed == false and .replay_recorded == false and .replay_persisted == false and .duplicate_accepted == false and .idempotency_key_accepted == false and .idempotency_state_recorded == false and .idempotency_state_persisted == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .completion_ack_recorded == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
    and ($replay_idempotency.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_gate" \
  --arg replay_idempotency_report_sha256 "$replay_idempotency_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson replay_idempotency "$REPLAY_IDEMPOTENCY_JSON" \
  '
  def blocked_fixture($id; $reason; $extra):
    {
      id:$id,
      ordering_requested:true,
      ordering_status:"blocked_noop",
      source_replay_idempotency_present:true,
      source_replay_idempotency_ready:true,
      ordering_allowed:false,
      ordering_recorded:false,
      ordering_persisted:false,
      sequence_cursor_accepted:false,
      sequence_cursor_recorded:false,
      sequence_cursor_persisted:false,
      monotonicity_state_recorded:false,
      monotonicity_state_persisted:false,
      timestamp_ordering_accepted:false,
      epoch_ordering_accepted:false,
      stage_ordering_accepted:false,
      latest_wins_overwrite_accepted:false,
      receipt_recorded:false,
      receipt_persisted:false,
      receipt_accepted:false,
      receipt_materialized:false,
      receipt_filesystem_written:false,
      receipt_ledger_written:false,
      receipt_indexed:false,
      receipt_delivered:false,
      completion_ack_recorded:false,
      completion_ack_persisted:false,
      completion_ack_accepted:false,
      activation_allowed:false,
      live_mutation_execution_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      rollback_executed:false,
      secret_material_read:false,
      provider_invoked:false,
      model_invoked:false,
      external_send_performed:false,
      public_release_published:false,
      release_artifact_written:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      receipt_noop_confirmed:true,
      reason:$reason
    } + $extra;
  {
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_ordering_monotonicity_mode:"memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial",
    source_activation_command_result_receipt_replay_idempotency_gate:$replay_idempotency.gate,
    source_activation_command_result_receipt_replay_idempotency_ready:$replay_idempotency.memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready,
    source_activation_command_result_receipt_replay_idempotency_report_sha256:$replay_idempotency_report_sha256,
    source_activation_command_result_receipt_no_persistence_ready:$replay_idempotency.memory_write_execution_activation_command_result_receipt_no_persistence_ready,
    source_activation_command_result_receipt_no_persistence_report_sha256:$replay_idempotency.source_activation_command_result_receipt_no_persistence_report_sha256,
    source_activation_command_noop_handoff_ready:$replay_idempotency.memory_write_execution_activation_command_noop_handoff_ready,
    source_activation_command_noop_handoff_report_sha256:$replay_idempotency.source_activation_command_noop_handoff_report_sha256,
    source_memory_write_execution_activation_closure_denial_report_sha256:$replay_idempotency.source_memory_write_execution_activation_closure_denial_report_sha256,
    source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256:$replay_idempotency.source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256,
    source_memory_write_execution_post_write_validation_dry_run_report_sha256:$replay_idempotency.source_memory_write_execution_post_write_validation_dry_run_report_sha256,
    source_memory_write_execution_write_enable_fixture_report_sha256:$replay_idempotency.source_memory_write_execution_write_enable_fixture_report_sha256,
    source_memory_write_execution_no_write_sink_contract_report_sha256:$replay_idempotency.source_memory_write_execution_no_write_sink_contract_report_sha256,
    source_memory_write_execution_denial_matrix_report_sha256:$replay_idempotency.source_memory_write_execution_denial_matrix_report_sha256,
    source_memory_write_execution_preflight_report_sha256:$replay_idempotency.source_memory_write_execution_preflight_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_no_persistence_ready:true,
    memory_write_execution_activation_command_noop_handoff_ready:true,
    memory_write_execution_activation_closure_denial_ready:true,
    memory_write_execution_post_write_operator_acceptance_denial_ready:true,
    memory_write_execution_post_write_validation_dry_run_ready:true,
    memory_write_execution_write_enable_fixture_ready:true,
    memory_write_execution_no_write_sink_contract_ready:true,
    required_activation_command_result_receipt_ordering_monotonicity_surface_count:12,
    ready_activation_command_result_receipt_ordering_monotonicity_surface_count:12,
    side_effect_free_activation_command_result_receipt_ordering_monotonicity_surface_count:12,
    required_activation_command_result_receipt_ordering_monotonicity_fixture_count:10,
    activation_command_result_receipt_ordering_monotonicity_fixture_count:10,
    blocked_activation_command_result_receipt_ordering_monotonicity_fixture_count:10,
    noop_activation_command_result_receipt_ordering_monotonicity_fixture_count:10,
    allowed_activation_command_result_receipt_ordering_monotonicity_fixture_count:0,
    accepted_activation_command_result_receipt_ordering_monotonicity_fixture_count:0,
    activation_command_result_receipt_ordering_violation_denied_count:10,
    activation_command_result_receipt_monotonicity_violation_denied_count:10,
    activation_command_result_receipt_ordering_performed_count:0,
    activation_command_result_receipt_sequence_cursor_accepted_count:0,
    activation_command_result_receipt_sequence_cursor_recorded_count:0,
    activation_command_result_receipt_monotonicity_state_recorded_count:0,
    activation_command_result_receipt_ordering_allowed:false,
    activation_command_result_receipt_ordering_recorded:false,
    activation_command_result_receipt_ordering_persisted:false,
    activation_command_result_receipt_sequence_cursor_accepted:false,
    activation_command_result_receipt_sequence_cursor_recorded:false,
    activation_command_result_receipt_sequence_cursor_persisted:false,
    activation_command_result_receipt_monotonicity_state_recorded:false,
    activation_command_result_receipt_monotonicity_state_persisted:false,
    activation_command_result_receipt_timestamp_ordering_accepted:false,
    activation_command_result_receipt_epoch_ordering_accepted:false,
    activation_command_result_receipt_stage_ordering_accepted:false,
    activation_command_result_receipt_same_sequence_hash_override_accepted:false,
    activation_command_result_receipt_latest_wins_overwrite_accepted:false,
    activation_command_result_receipt_gap_fill_accepted:false,
    activation_command_result_receipt_ack_before_noop_accepted:false,
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
    activation_command_enabled:false,
    activation_command_invoked:false,
    activation_command_dispatched:false,
    activation_allowed_by_result_receipt_ordering:false,
    activation_allowed_by_result_receipt_replay:false,
    activation_allowed_by_result_receipt:false,
    activation_allowed:false,
    activation_performed:false,
    live_mutation_execution_ready:false,
    live_mutation_execution_allowed:false,
    live_mutation_execution_performed:false,
    memory_write_execution_allowed:false,
    memory_write_execution_ready:false,
    memory_write_execution_performed:false,
    memory_store_write_path_enabled:false,
    memory_store_write_allowed:false,
    memory_store_write_performed:false,
    memory_store_write_performed_count:0,
    memory_store_mutation_allowed:false,
    memory_store_mutated:false,
    rollback_execution_allowed:false,
    rollback_executed:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    secret_material_read:false,
    provider_prompt_replay_enabled:false,
    provider_invoked:false,
    model_invoked:false,
    external_send_enabled:false,
    external_send_performed:false,
    public_claim_or_release_artifact_write_enabled:false,
    public_release_published:false,
    public_ga_claimed:false,
    release_artifact_written:false,
    install_executed:false,
    launchd_mutated:false,
    service_restarted:false,
    active_binary_mutated:false,
    activation_command_result_receipt_ordering_monotonicity_surfaces:[
      "source_replay_idempotency_report_required",
      "canonical_noop_receipt_order_identity_required",
      "sequence_cursor_monotonicity_denied",
      "out_of_order_sequence_denied",
      "sequence_gap_or_skip_denied",
      "timestamp_rollback_denied",
      "epoch_rollback_denied",
      "same_sequence_different_hash_denied",
      "latest_wins_overwrite_denied",
      "stage_transition_ordering_denied",
      "ledger_index_delivery_ordering_bypass_denied",
      "external_public_install_ordering_bypass_denied"
    ],
    activation_command_result_receipt_ordering_monotonicity_fixtures:[
      blocked_fixture("activation-result-receipt-ordering-missing-source-replay-idempotency-report"; "source_result_receipt_replay_idempotency_report_required"; {
        source_replay_idempotency_present:false,
        source_replay_idempotency_ready:false
      }),
      blocked_fixture("activation-result-receipt-out-of-order-sequence"; "out_of_order_result_receipt_sequence_denied"; {
        out_of_order_sequence_requested:true,
        requested_sequence:2,
        observed_previous_sequence:3,
        ordering_status:"blocked_ordering_noop"
      }),
      blocked_fixture("activation-result-receipt-sequence-gap-skip"; "sequence_gap_or_skip_result_receipt_denied"; {
        sequence_gap_requested:true,
        requested_sequence:5,
        expected_next_sequence:1,
        ordering_status:"blocked_ordering_noop"
      }),
      blocked_fixture("activation-result-receipt-timestamp-rollback"; "timestamp_rollback_result_receipt_denied"; {
        timestamp_rollback_requested:true,
        requested_timestamp_order:"older_than_source_noop_handoff",
        ordering_status:"blocked_ordering_noop"
      }),
      blocked_fixture("activation-result-receipt-epoch-rollback"; "epoch_rollback_result_receipt_denied"; {
        epoch_rollback_requested:true,
        requested_epoch_order:"lower_than_current_activation_epoch",
        ordering_status:"blocked_ordering_noop"
      }),
      blocked_fixture("activation-result-receipt-same-sequence-different-hash"; "same_sequence_different_hash_result_receipt_denied"; {
        same_sequence_different_hash_requested:true,
        requested_sequence:1,
        requested_hash_relation:"different_hash_for_same_sequence",
        ordering_status:"blocked_ordering_noop"
      }),
      blocked_fixture("activation-result-receipt-latest-wins-overwrite"; "latest_wins_result_receipt_overwrite_denied"; {
        latest_wins_overwrite_requested:true,
        overwrite_existing_noop_requested:true,
        ordering_status:"blocked_ordering_noop"
      }),
      blocked_fixture("activation-result-receipt-stage-transition-before-noop"; "stage_transition_ordering_bypass_denied"; {
        stage_transition_ordering_bypass_requested:true,
        completion_ack_before_noop_requested:true,
        requested_stage:"completed_before_blocked_noop",
        ordering_status:"blocked_ordering_noop"
      }),
      blocked_fixture("activation-result-receipt-ledger-index-delivery-ordering-bypass"; "ledger_index_delivery_ordering_bypass_denied"; {
        ledger_ordering_bypass_requested:true,
        index_ordering_bypass_requested:true,
        delivery_ordering_bypass_requested:true,
        ordering_status:"blocked_ordering_noop"
      }),
      blocked_fixture("activation-result-receipt-external-public-install-ordering-bypass"; "external_public_install_restart_ordering_bypass_denied"; {
        external_send_ordering_bypass_requested:true,
        public_claim_ordering_bypass_requested:true,
        release_artifact_ordering_bypass_requested:true,
        install_ordering_bypass_requested:true,
        service_restart_ordering_bypass_requested:true,
        active_binary_mutation_ordering_bypass_requested:true,
        ordering_status:"blocked_ordering_noop"
      })
    ],
    denied_by_activation_command_result_receipt_ordering_monotonicity:[
      "source_result_receipt_replay_idempotency_report_required",
      "canonical_noop_receipt_order_identity_required",
      "sequence_cursor_acceptance_denied",
      "sequence_cursor_recording_denied",
      "sequence_cursor_persistence_denied",
      "monotonicity_state_recording_denied",
      "monotonicity_state_persistence_denied",
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
      "memory_write_ordering_bypass_denied",
      "live_mutation_ordering_bypass_denied",
      "rollback_ordering_bypass_denied",
      "secret_provider_ordering_bypass_denied",
      "external_public_release_ordering_bypass_denied",
      "install_restart_active_binary_ordering_bypass_denied"
    ],
    side_effects:{
      activation_command_result_receipt_ordering_recorded:false,
      activation_command_result_receipt_ordering_persisted:false,
      activation_command_result_receipt_sequence_cursor_accepted:false,
      activation_command_result_receipt_sequence_cursor_recorded:false,
      activation_command_result_receipt_sequence_cursor_persisted:false,
      activation_command_result_receipt_monotonicity_state_recorded:false,
      activation_command_result_receipt_monotonicity_state_persisted:false,
      activation_command_result_receipt_timestamp_ordering_accepted:false,
      activation_command_result_receipt_stage_ordering_accepted:false,
      activation_command_result_receipt_latest_wins_overwrite_accepted:false,
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
      activation_command_enabled:false,
      activation_command_invoked:false,
      activation_command_dispatched:false,
      activation_performed:false,
      live_mutation_execution_performed:false,
      memory_write_execution_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      rollback_executed:false,
      raw_payload_inspected:false,
      payload_plaintext_persisted:false,
      secret_file_read:false,
      credential_read:false,
      provider_invoked:false,
      model_invoked:false,
      provider_prompt_replayed:false,
      channel_send_performed:false,
      external_send_performed:false,
      runtime_store_mutated:false,
      gateway_event_enqueued:false,
      capability_registry_mutated:false,
      plugin_registry_mutated:false,
      skill_workshop_written:false,
      filesystem_written:false,
      release_artifact_written:false,
      public_artifact_written:false,
      public_release_published:false,
      public_ga_claimed:false,
      install_executed:false,
      active_binary_mutated:false,
      launchd_mutated:false,
      service_restarted:false
    }
  }')"

jq -e '
  .status == "ready"
  and .memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
  and .activation_command_result_receipt_ordering_monotonicity_mode == "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial"
  and .source_activation_command_result_receipt_replay_idempotency_ready == true
  and .source_activation_command_result_receipt_replay_idempotency_report_sha256 != ""
  and .source_activation_command_result_receipt_no_persistence_ready == true
  and .minimum_required_samples >= 24
  and .required_activation_command_result_receipt_ordering_monotonicity_surface_count == 12
  and .ready_activation_command_result_receipt_ordering_monotonicity_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_ordering_monotonicity_surface_count == 12
  and .required_activation_command_result_receipt_ordering_monotonicity_fixture_count == 10
  and .activation_command_result_receipt_ordering_monotonicity_fixture_count == 10
  and .blocked_activation_command_result_receipt_ordering_monotonicity_fixture_count == 10
  and .noop_activation_command_result_receipt_ordering_monotonicity_fixture_count == 10
  and .allowed_activation_command_result_receipt_ordering_monotonicity_fixture_count == 0
  and .accepted_activation_command_result_receipt_ordering_monotonicity_fixture_count == 0
  and .activation_command_result_receipt_ordering_violation_denied_count == 10
  and .activation_command_result_receipt_monotonicity_violation_denied_count == 10
  and .activation_command_result_receipt_ordering_performed_count == 0
  and .activation_command_result_receipt_sequence_cursor_accepted_count == 0
  and .activation_command_result_receipt_sequence_cursor_recorded_count == 0
  and .activation_command_result_receipt_monotonicity_state_recorded_count == 0
  and .activation_command_result_receipt_ordering_allowed == false
  and .activation_command_result_receipt_ordering_recorded == false
  and .activation_command_result_receipt_ordering_persisted == false
  and .activation_command_result_receipt_sequence_cursor_accepted == false
  and .activation_command_result_receipt_sequence_cursor_recorded == false
  and .activation_command_result_receipt_sequence_cursor_persisted == false
  and .activation_command_result_receipt_monotonicity_state_recorded == false
  and .activation_command_result_receipt_monotonicity_state_persisted == false
  and .activation_command_result_receipt_timestamp_ordering_accepted == false
  and .activation_command_result_receipt_epoch_ordering_accepted == false
  and .activation_command_result_receipt_stage_ordering_accepted == false
  and .activation_command_result_receipt_same_sequence_hash_override_accepted == false
  and .activation_command_result_receipt_latest_wins_overwrite_accepted == false
  and .activation_command_result_receipt_gap_fill_accepted == false
  and .activation_command_result_receipt_ack_before_noop_accepted == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_result_receipt_materialized == false
  and .activation_command_result_receipt_filesystem_written == false
  and .activation_command_result_receipt_ledger_written == false
  and .activation_command_result_receipt_indexed == false
  and .activation_command_result_receipt_enqueued == false
  and .activation_command_result_receipt_delivered == false
  and .activation_command_completion_ack_recorded == false
  and .activation_command_completion_ack_persisted == false
  and .activation_command_completion_ack_accepted == false
  and .activation_command_completion_ack_delivered == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_allowed_by_result_receipt_ordering == false
  and .activation_allowed_by_result_receipt_replay == false
  and .activation_allowed_by_result_receipt == false
  and .activation_allowed == false
  and .activation_performed == false
  and .live_mutation_execution_ready == false
  and .live_mutation_execution_allowed == false
  and .live_mutation_execution_performed == false
  and .memory_write_execution_allowed == false
  and .memory_write_execution_ready == false
  and .memory_write_execution_performed == false
  and .memory_store_write_path_enabled == false
  and .memory_store_write_allowed == false
  and .memory_store_write_performed == false
  and .memory_store_write_performed_count == 0
  and .memory_store_mutation_allowed == false
  and .memory_store_mutated == false
  and .rollback_execution_allowed == false
  and .rollback_executed == false
  and .secret_material_read == false
  and .provider_prompt_replay_enabled == false
  and .provider_invoked == false
  and .model_invoked == false
  and .external_send_enabled == false
  and .external_send_performed == false
  and .public_claim_or_release_artifact_write_enabled == false
  and .public_release_published == false
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.activation_command_result_receipt_ordering_monotonicity_surfaces | length) == 12
  and (.activation_command_result_receipt_ordering_monotonicity_fixtures | length) == 10
  and (.activation_command_result_receipt_ordering_monotonicity_fixtures | all((.ordering_status == "blocked_noop" or .ordering_status == "blocked_ordering_noop") and .ordering_allowed == false and .ordering_recorded == false and .ordering_persisted == false and .sequence_cursor_accepted == false and .sequence_cursor_recorded == false and .sequence_cursor_persisted == false and .monotonicity_state_recorded == false and .monotonicity_state_persisted == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .completion_ack_recorded == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
  and ([.activation_command_result_receipt_ordering_monotonicity_fixtures[] | select(.out_of_order_sequence_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_ordering_monotonicity_fixtures[] | select(.sequence_gap_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_ordering_monotonicity_fixtures[] | select(.timestamp_rollback_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_ordering_monotonicity_fixtures[] | select(.epoch_rollback_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_ordering_monotonicity_fixtures[] | select(.same_sequence_different_hash_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_ordering_monotonicity_fixtures[] | select(.latest_wins_overwrite_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_ordering_monotonicity_fixtures[] | select(.completion_ack_before_noop_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_ordering_monotonicity_fixtures[] | select(.ledger_ordering_bypass_requested == true and .index_ordering_bypass_requested == true and .delivery_ordering_bypass_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_ordering_monotonicity_fixtures[] | select(.external_send_ordering_bypass_requested == true and .install_ordering_bypass_requested == true and .active_binary_mutation_ordering_bypass_requested == true)] | length) == 1
  and (.denied_by_activation_command_result_receipt_ordering_monotonicity | length) == 24
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory live mutation operator write execution activation command result receipt ordering/monotonicity denial gate passed"
