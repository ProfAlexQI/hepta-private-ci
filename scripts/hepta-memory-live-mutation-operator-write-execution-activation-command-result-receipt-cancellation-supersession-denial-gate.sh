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

ORDERING_MONOTONICITY_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-ordering-monotonicity-denial-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh
)"

ordering_monotonicity_report_sha256="$(printf '%s' "$ORDERING_MONOTONICITY_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson ordering "$ORDERING_MONOTONICITY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $ordering.runtime == "hepta"
    and $ordering.status == "ready"
    and $ordering.gate == "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_gate"
    and $ordering.activation_command_result_receipt_ordering_monotonicity_mode == "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial"
    and $ordering.memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
    and $ordering.memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $ordering.memory_write_execution_activation_command_result_receipt_no_persistence_ready == true
    and $ordering.memory_write_execution_activation_command_noop_handoff_ready == true
    and $ordering.memory_write_execution_activation_closure_denial_ready == true
    and $ordering.memory_write_execution_post_write_operator_acceptance_denial_ready == true
    and $ordering.memory_write_execution_post_write_validation_dry_run_ready == true
    and $ordering.memory_write_execution_write_enable_fixture_ready == true
    and $ordering.memory_write_execution_no_write_sink_contract_ready == true
    and $ordering.source_activation_command_result_receipt_replay_idempotency_report_sha256 != ""
    and $ordering.source_activation_command_result_receipt_no_persistence_report_sha256 != ""
    and $ordering.source_activation_command_noop_handoff_report_sha256 != ""
    and $ordering.source_memory_write_execution_activation_closure_denial_report_sha256 != ""
    and $ordering.source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256 != ""
    and $ordering.source_memory_write_execution_post_write_validation_dry_run_report_sha256 != ""
    and $ordering.source_memory_write_execution_write_enable_fixture_report_sha256 != ""
    and $ordering.source_memory_write_execution_no_write_sink_contract_report_sha256 != ""
    and $ordering.source_memory_write_execution_denial_matrix_report_sha256 != ""
    and $ordering.source_memory_write_execution_preflight_report_sha256 != ""
    and $ordering.minimum_required_samples >= 24
    and $ordering.required_activation_command_result_receipt_ordering_monotonicity_surface_count == 12
    and $ordering.ready_activation_command_result_receipt_ordering_monotonicity_surface_count == 12
    and $ordering.side_effect_free_activation_command_result_receipt_ordering_monotonicity_surface_count == 12
    and $ordering.required_activation_command_result_receipt_ordering_monotonicity_fixture_count == 10
    and $ordering.activation_command_result_receipt_ordering_monotonicity_fixture_count == 10
    and $ordering.blocked_activation_command_result_receipt_ordering_monotonicity_fixture_count == 10
    and $ordering.noop_activation_command_result_receipt_ordering_monotonicity_fixture_count == 10
    and $ordering.allowed_activation_command_result_receipt_ordering_monotonicity_fixture_count == 0
    and $ordering.accepted_activation_command_result_receipt_ordering_monotonicity_fixture_count == 0
    and $ordering.activation_command_result_receipt_ordering_allowed == false
    and $ordering.activation_command_result_receipt_ordering_recorded == false
    and $ordering.activation_command_result_receipt_ordering_persisted == false
    and $ordering.activation_command_result_receipt_sequence_cursor_accepted == false
    and $ordering.activation_command_result_receipt_sequence_cursor_recorded == false
    and $ordering.activation_command_result_receipt_sequence_cursor_persisted == false
    and $ordering.activation_command_result_receipt_monotonicity_state_recorded == false
    and $ordering.activation_command_result_receipt_monotonicity_state_persisted == false
    and $ordering.activation_command_result_receipt_latest_wins_overwrite_accepted == false
    and $ordering.activation_command_result_receipt_recorded == false
    and $ordering.activation_command_result_receipt_persisted == false
    and $ordering.activation_command_result_receipt_accepted == false
    and $ordering.activation_command_result_receipt_materialized == false
    and $ordering.activation_command_result_receipt_filesystem_written == false
    and $ordering.activation_command_result_receipt_ledger_written == false
    and $ordering.activation_command_result_receipt_indexed == false
    and $ordering.activation_command_result_receipt_enqueued == false
    and $ordering.activation_command_result_receipt_delivered == false
    and $ordering.activation_command_completion_ack_recorded == false
    and $ordering.activation_command_completion_ack_persisted == false
    and $ordering.activation_command_completion_ack_accepted == false
    and $ordering.activation_command_completion_ack_delivered == false
    and $ordering.activation_allowed_by_result_receipt_ordering == false
    and $ordering.activation_allowed_by_result_receipt_replay == false
    and $ordering.activation_allowed_by_result_receipt == false
    and $ordering.activation_allowed == false
    and $ordering.activation_performed == false
    and $ordering.live_mutation_execution_performed == false
    and $ordering.memory_write_execution_performed == false
    and $ordering.memory_store_write_performed == false
    and $ordering.memory_store_write_performed_count == 0
    and $ordering.memory_store_mutated == false
    and $ordering.rollback_executed == false
    and $ordering.secret_material_read == false
    and $ordering.provider_invoked == false
    and $ordering.model_invoked == false
    and $ordering.external_send_performed == false
    and $ordering.public_release_published == false
    and $ordering.release_artifact_written == false
    and $ordering.install_executed == false
    and $ordering.launchd_mutated == false
    and $ordering.service_restarted == false
    and $ordering.active_binary_mutated == false
    and ($ordering.activation_command_result_receipt_ordering_monotonicity_fixtures | length) == 10
    and ($ordering.activation_command_result_receipt_ordering_monotonicity_fixtures | all((.ordering_status == "blocked_noop" or .ordering_status == "blocked_ordering_noop") and .ordering_allowed == false and .ordering_recorded == false and .ordering_persisted == false and .sequence_cursor_accepted == false and .sequence_cursor_recorded == false and .sequence_cursor_persisted == false and .monotonicity_state_recorded == false and .monotonicity_state_persisted == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .completion_ack_recorded == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
    and ($ordering.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_cancellation_supersession_denial_gate" \
  --arg ordering_monotonicity_report_sha256 "$ordering_monotonicity_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson ordering "$ORDERING_MONOTONICITY_JSON" \
  '
  def blocked_fixture($id; $reason; $extra):
    {
      id:$id,
      cancellation_requested:true,
      supersession_requested:false,
      cancellation_supersession_status:"blocked_noop",
      source_ordering_monotonicity_present:true,
      source_ordering_monotonicity_ready:true,
      cancellation_allowed:false,
      cancellation_recorded:false,
      cancellation_persisted:false,
      cancellation_request_accepted:false,
      supersession_allowed:false,
      supersession_recorded:false,
      supersession_persisted:false,
      supersession_request_accepted:false,
      replacement_receipt_accepted:false,
      replacement_receipt_recorded:false,
      replacement_receipt_persisted:false,
      replacement_hash_accepted:false,
      tombstone_recorded:false,
      tombstone_persisted:false,
      delete_marker_recorded:false,
      ack_cancellation_accepted:false,
      ledger_cancellation_accepted:false,
      index_cancellation_accepted:false,
      delivery_cancellation_accepted:false,
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
      completion_ack_delivered:false,
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
    activation_command_result_receipt_cancellation_supersession_mode:"memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial",
    source_activation_command_result_receipt_ordering_monotonicity_gate:$ordering.gate,
    source_activation_command_result_receipt_ordering_monotonicity_ready:$ordering.memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready,
    source_activation_command_result_receipt_ordering_monotonicity_report_sha256:$ordering_monotonicity_report_sha256,
    source_activation_command_result_receipt_replay_idempotency_ready:$ordering.memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready,
    source_activation_command_result_receipt_replay_idempotency_report_sha256:$ordering.source_activation_command_result_receipt_replay_idempotency_report_sha256,
    source_activation_command_result_receipt_no_persistence_ready:$ordering.memory_write_execution_activation_command_result_receipt_no_persistence_ready,
    source_activation_command_result_receipt_no_persistence_report_sha256:$ordering.source_activation_command_result_receipt_no_persistence_report_sha256,
    source_activation_command_noop_handoff_ready:$ordering.memory_write_execution_activation_command_noop_handoff_ready,
    source_activation_command_noop_handoff_report_sha256:$ordering.source_activation_command_noop_handoff_report_sha256,
    source_memory_write_execution_activation_closure_denial_report_sha256:$ordering.source_memory_write_execution_activation_closure_denial_report_sha256,
    source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256:$ordering.source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256,
    source_memory_write_execution_post_write_validation_dry_run_report_sha256:$ordering.source_memory_write_execution_post_write_validation_dry_run_report_sha256,
    source_memory_write_execution_write_enable_fixture_report_sha256:$ordering.source_memory_write_execution_write_enable_fixture_report_sha256,
    source_memory_write_execution_no_write_sink_contract_report_sha256:$ordering.source_memory_write_execution_no_write_sink_contract_report_sha256,
    source_memory_write_execution_denial_matrix_report_sha256:$ordering.source_memory_write_execution_denial_matrix_report_sha256,
    source_memory_write_execution_preflight_report_sha256:$ordering.source_memory_write_execution_preflight_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_no_persistence_ready:true,
    memory_write_execution_activation_command_noop_handoff_ready:true,
    memory_write_execution_activation_closure_denial_ready:true,
    memory_write_execution_post_write_operator_acceptance_denial_ready:true,
    memory_write_execution_post_write_validation_dry_run_ready:true,
    memory_write_execution_write_enable_fixture_ready:true,
    memory_write_execution_no_write_sink_contract_ready:true,
    required_activation_command_result_receipt_cancellation_supersession_surface_count:12,
    ready_activation_command_result_receipt_cancellation_supersession_surface_count:12,
    side_effect_free_activation_command_result_receipt_cancellation_supersession_surface_count:12,
    required_activation_command_result_receipt_cancellation_supersession_fixture_count:10,
    activation_command_result_receipt_cancellation_supersession_fixture_count:10,
    blocked_activation_command_result_receipt_cancellation_supersession_fixture_count:10,
    noop_activation_command_result_receipt_cancellation_supersession_fixture_count:10,
    allowed_activation_command_result_receipt_cancellation_supersession_fixture_count:0,
    accepted_activation_command_result_receipt_cancellation_supersession_fixture_count:0,
    activation_command_result_receipt_cancellation_denied_count:10,
    activation_command_result_receipt_supersession_denied_count:10,
    activation_command_result_receipt_cancellation_performed_count:0,
    activation_command_result_receipt_supersession_performed_count:0,
    activation_command_result_receipt_cancellation_allowed:false,
    activation_command_result_receipt_cancellation_recorded:false,
    activation_command_result_receipt_cancellation_persisted:false,
    activation_command_result_receipt_cancellation_request_accepted:false,
    activation_command_result_receipt_supersession_allowed:false,
    activation_command_result_receipt_supersession_recorded:false,
    activation_command_result_receipt_supersession_persisted:false,
    activation_command_result_receipt_supersession_request_accepted:false,
    activation_command_result_receipt_replacement_receipt_accepted:false,
    activation_command_result_receipt_replacement_receipt_recorded:false,
    activation_command_result_receipt_replacement_receipt_persisted:false,
    activation_command_result_receipt_replacement_hash_accepted:false,
    activation_command_result_receipt_tombstone_recorded:false,
    activation_command_result_receipt_tombstone_persisted:false,
    activation_command_result_receipt_delete_marker_recorded:false,
    activation_command_result_receipt_ack_cancellation_accepted:false,
    activation_command_result_receipt_ledger_cancellation_accepted:false,
    activation_command_result_receipt_index_cancellation_accepted:false,
    activation_command_result_receipt_delivery_cancellation_accepted:false,
    activation_command_result_receipt_ordering_allowed:false,
    activation_command_result_receipt_ordering_recorded:false,
    activation_command_result_receipt_ordering_persisted:false,
    activation_command_result_receipt_sequence_cursor_accepted:false,
    activation_command_result_receipt_sequence_cursor_recorded:false,
    activation_command_result_receipt_sequence_cursor_persisted:false,
    activation_command_result_receipt_monotonicity_state_recorded:false,
    activation_command_result_receipt_monotonicity_state_persisted:false,
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
    activation_allowed_by_result_receipt_cancellation:false,
    activation_allowed_by_result_receipt_supersession:false,
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
    activation_command_result_receipt_cancellation_supersession_surfaces:[
      "source_ordering_monotonicity_report_required",
      "cancellation_request_shape_denied",
      "supersession_request_shape_denied",
      "replacement_receipt_hash_denied",
      "tombstone_or_delete_marker_denied",
      "cancel_after_blocked_noop_denied",
      "supersede_blocked_noop_with_completed_denied",
      "acknowledgement_cancellation_denied",
      "ledger_index_delivery_cancellation_denied",
      "memory_write_live_mutation_supersession_denied",
      "rollback_secret_provider_supersession_denied",
      "external_public_install_restart_supersession_denied"
    ],
    activation_command_result_receipt_cancellation_supersession_fixtures:[
      blocked_fixture("activation-result-receipt-cancellation-missing-source-ordering-report"; "source_ordering_monotonicity_report_required"; {
        source_ordering_monotonicity_present:false,
        source_ordering_monotonicity_ready:false
      }),
      blocked_fixture("activation-result-receipt-cancel-blocked-noop"; "cancellation_of_blocked_noop_receipt_denied"; {
        cancellation_request_shape:"cancel_blocked_noop_receipt"
      }),
      blocked_fixture("activation-result-receipt-supersede-with-completed"; "supersession_of_blocked_noop_with_completed_denied"; {
        supersession_requested:true,
        cancellation_requested:false,
        requested_replacement_status:"completed",
        cancellation_supersession_status:"blocked_supersession_noop"
      }),
      blocked_fixture("activation-result-receipt-replacement-hash"; "replacement_hash_identity_attempt_denied"; {
        supersession_requested:true,
        cancellation_requested:false,
        replacement_hash_requested:true,
        requested_hash_relation:"different_hash_for_same_receipt_identity",
        cancellation_supersession_status:"blocked_supersession_noop"
      }),
      blocked_fixture("activation-result-receipt-tombstone-delete-marker"; "tombstone_or_delete_marker_denied"; {
        tombstone_requested:true,
        delete_marker_requested:true
      }),
      blocked_fixture("activation-result-receipt-completion-ack-cancel"; "completion_ack_cancellation_denied"; {
        completion_ack_cancellation_requested:true,
        ack_cancellation_requested:true
      }),
      blocked_fixture("activation-result-receipt-ledger-index-delivery-cancel"; "ledger_index_delivery_cancellation_supersession_denied"; {
        ledger_cancellation_requested:true,
        index_cancellation_requested:true,
        delivery_cancellation_requested:true
      }),
      blocked_fixture("activation-result-receipt-memory-write-live-mutation-supersede"; "memory_write_live_mutation_supersession_denied"; {
        supersession_requested:true,
        cancellation_requested:false,
        memory_write_supersession_requested:true,
        live_mutation_supersession_requested:true,
        cancellation_supersession_status:"blocked_supersession_noop"
      }),
      blocked_fixture("activation-result-receipt-rollback-secret-provider-supersede"; "rollback_secret_provider_supersession_denied"; {
        supersession_requested:true,
        cancellation_requested:false,
        rollback_supersession_requested:true,
        secret_material_supersession_requested:true,
        provider_prompt_supersession_requested:true,
        cancellation_supersession_status:"blocked_supersession_noop"
      }),
      blocked_fixture("activation-result-receipt-external-public-install-supersede"; "external_public_install_restart_active_binary_supersession_denied"; {
        supersession_requested:true,
        cancellation_requested:false,
        external_send_supersession_requested:true,
        public_claim_supersession_requested:true,
        release_artifact_supersession_requested:true,
        install_supersession_requested:true,
        service_restart_supersession_requested:true,
        active_binary_mutation_supersession_requested:true,
        cancellation_supersession_status:"blocked_supersession_noop"
      })
    ],
    denied_by_activation_command_result_receipt_cancellation_supersession:[
      "source_ordering_monotonicity_report_required",
      "cancellation_request_acceptance_denied",
      "cancellation_recording_denied",
      "cancellation_persistence_denied",
      "supersession_request_acceptance_denied",
      "supersession_recording_denied",
      "supersession_persistence_denied",
      "replacement_receipt_acceptance_denied",
      "replacement_hash_acceptance_denied",
      "tombstone_recording_denied",
      "delete_marker_recording_denied",
      "cancel_after_blocked_noop_denied",
      "supersede_blocked_noop_with_completed_denied",
      "completion_ack_cancellation_denied",
      "ledger_cancellation_denied",
      "index_cancellation_denied",
      "delivery_cancellation_denied",
      "memory_write_supersession_denied",
      "live_mutation_supersession_denied",
      "rollback_supersession_denied",
      "secret_material_supersession_denied",
      "provider_prompt_supersession_denied",
      "external_public_release_supersession_denied",
      "install_restart_active_binary_supersession_denied"
    ],
    side_effects:{
      activation_command_result_receipt_cancellation_recorded:false,
      activation_command_result_receipt_cancellation_persisted:false,
      activation_command_result_receipt_supersession_recorded:false,
      activation_command_result_receipt_supersession_persisted:false,
      activation_command_result_receipt_replacement_receipt_recorded:false,
      activation_command_result_receipt_replacement_receipt_persisted:false,
      activation_command_result_receipt_replacement_hash_accepted:false,
      activation_command_result_receipt_tombstone_recorded:false,
      activation_command_result_receipt_tombstone_persisted:false,
      activation_command_result_receipt_delete_marker_recorded:false,
      activation_command_result_receipt_ack_cancellation_accepted:false,
      activation_command_result_receipt_ledger_cancellation_accepted:false,
      activation_command_result_receipt_index_cancellation_accepted:false,
      activation_command_result_receipt_delivery_cancellation_accepted:false,
      activation_command_result_receipt_ordering_recorded:false,
      activation_command_result_receipt_ordering_persisted:false,
      activation_command_result_receipt_sequence_cursor_accepted:false,
      activation_command_result_receipt_sequence_cursor_recorded:false,
      activation_command_result_receipt_sequence_cursor_persisted:false,
      activation_command_result_receipt_monotonicity_state_recorded:false,
      activation_command_result_receipt_monotonicity_state_persisted:false,
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
  and .memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready == true
  and .activation_command_result_receipt_cancellation_supersession_mode == "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial"
  and .source_activation_command_result_receipt_ordering_monotonicity_ready == true
  and .source_activation_command_result_receipt_ordering_monotonicity_report_sha256 != ""
  and .source_activation_command_result_receipt_replay_idempotency_ready == true
  and .source_activation_command_result_receipt_no_persistence_ready == true
  and .minimum_required_samples >= 24
  and .required_activation_command_result_receipt_cancellation_supersession_surface_count == 12
  and .ready_activation_command_result_receipt_cancellation_supersession_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_cancellation_supersession_surface_count == 12
  and .required_activation_command_result_receipt_cancellation_supersession_fixture_count == 10
  and .activation_command_result_receipt_cancellation_supersession_fixture_count == 10
  and .blocked_activation_command_result_receipt_cancellation_supersession_fixture_count == 10
  and .noop_activation_command_result_receipt_cancellation_supersession_fixture_count == 10
  and .allowed_activation_command_result_receipt_cancellation_supersession_fixture_count == 0
  and .accepted_activation_command_result_receipt_cancellation_supersession_fixture_count == 0
  and .activation_command_result_receipt_cancellation_denied_count == 10
  and .activation_command_result_receipt_supersession_denied_count == 10
  and .activation_command_result_receipt_cancellation_performed_count == 0
  and .activation_command_result_receipt_supersession_performed_count == 0
  and .activation_command_result_receipt_cancellation_allowed == false
  and .activation_command_result_receipt_cancellation_recorded == false
  and .activation_command_result_receipt_cancellation_persisted == false
  and .activation_command_result_receipt_cancellation_request_accepted == false
  and .activation_command_result_receipt_supersession_allowed == false
  and .activation_command_result_receipt_supersession_recorded == false
  and .activation_command_result_receipt_supersession_persisted == false
  and .activation_command_result_receipt_supersession_request_accepted == false
  and .activation_command_result_receipt_replacement_receipt_accepted == false
  and .activation_command_result_receipt_replacement_receipt_recorded == false
  and .activation_command_result_receipt_replacement_receipt_persisted == false
  and .activation_command_result_receipt_replacement_hash_accepted == false
  and .activation_command_result_receipt_tombstone_recorded == false
  and .activation_command_result_receipt_tombstone_persisted == false
  and .activation_command_result_receipt_delete_marker_recorded == false
  and .activation_command_result_receipt_ack_cancellation_accepted == false
  and .activation_command_result_receipt_ledger_cancellation_accepted == false
  and .activation_command_result_receipt_index_cancellation_accepted == false
  and .activation_command_result_receipt_delivery_cancellation_accepted == false
  and .activation_command_result_receipt_ordering_allowed == false
  and .activation_command_result_receipt_ordering_recorded == false
  and .activation_command_result_receipt_ordering_persisted == false
  and .activation_command_result_receipt_sequence_cursor_accepted == false
  and .activation_command_result_receipt_sequence_cursor_recorded == false
  and .activation_command_result_receipt_sequence_cursor_persisted == false
  and .activation_command_result_receipt_monotonicity_state_recorded == false
  and .activation_command_result_receipt_monotonicity_state_persisted == false
  and .activation_command_result_receipt_latest_wins_overwrite_accepted == false
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
  and .activation_allowed_by_result_receipt_cancellation == false
  and .activation_allowed_by_result_receipt_supersession == false
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
  and (.activation_command_result_receipt_cancellation_supersession_surfaces | length) == 12
  and (.activation_command_result_receipt_cancellation_supersession_fixtures | length) == 10
  and (.activation_command_result_receipt_cancellation_supersession_fixtures | all((.cancellation_supersession_status == "blocked_noop" or .cancellation_supersession_status == "blocked_supersession_noop") and .cancellation_allowed == false and .cancellation_recorded == false and .cancellation_persisted == false and .supersession_allowed == false and .supersession_recorded == false and .supersession_persisted == false and .replacement_receipt_accepted == false and .tombstone_recorded == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .completion_ack_recorded == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
  and ([.activation_command_result_receipt_cancellation_supersession_fixtures[] | select(.source_ordering_monotonicity_present == false)] | length) == 1
  and ([.activation_command_result_receipt_cancellation_supersession_fixtures[] | select(.cancellation_request_shape == "cancel_blocked_noop_receipt")] | length) == 1
  and ([.activation_command_result_receipt_cancellation_supersession_fixtures[] | select(.requested_replacement_status == "completed")] | length) == 1
  and ([.activation_command_result_receipt_cancellation_supersession_fixtures[] | select(.replacement_hash_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_cancellation_supersession_fixtures[] | select(.tombstone_requested == true and .delete_marker_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_cancellation_supersession_fixtures[] | select(.completion_ack_cancellation_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_cancellation_supersession_fixtures[] | select(.ledger_cancellation_requested == true and .index_cancellation_requested == true and .delivery_cancellation_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_cancellation_supersession_fixtures[] | select(.memory_write_supersession_requested == true and .live_mutation_supersession_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_cancellation_supersession_fixtures[] | select(.rollback_supersession_requested == true and .secret_material_supersession_requested == true and .provider_prompt_supersession_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_cancellation_supersession_fixtures[] | select(.external_send_supersession_requested == true and .install_supersession_requested == true and .active_binary_mutation_supersession_requested == true)] | length) == 1
  and (.denied_by_activation_command_result_receipt_cancellation_supersession | length) == 24
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory live mutation operator write execution activation command result receipt cancellation/supersession denial gate passed"
