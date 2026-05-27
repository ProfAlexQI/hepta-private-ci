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

CANCELLATION_SUPERSESSION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-cancellation-supersession-denial-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-cancellation-supersession-denial-gate.sh
)"

cancellation_supersession_report_sha256="$(printf '%s' "$CANCELLATION_SUPERSESSION_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$CANCELLATION_SUPERSESSION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_cancellation_supersession_denial_gate"
    and $source.activation_command_result_receipt_cancellation_supersession_mode == "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial"
    and $source.memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_no_persistence_ready == true
    and $source.memory_write_execution_activation_command_noop_handoff_ready == true
    and $source.memory_write_execution_activation_closure_denial_ready == true
    and $source.memory_write_execution_post_write_operator_acceptance_denial_ready == true
    and $source.memory_write_execution_post_write_validation_dry_run_ready == true
    and $source.memory_write_execution_write_enable_fixture_ready == true
    and $source.memory_write_execution_no_write_sink_contract_ready == true
    and $source.source_activation_command_result_receipt_ordering_monotonicity_report_sha256 != ""
    and $source.source_activation_command_result_receipt_replay_idempotency_report_sha256 != ""
    and $source.source_activation_command_result_receipt_no_persistence_report_sha256 != ""
    and $source.source_activation_command_noop_handoff_report_sha256 != ""
    and $source.source_memory_write_execution_activation_closure_denial_report_sha256 != ""
    and $source.source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256 != ""
    and $source.source_memory_write_execution_post_write_validation_dry_run_report_sha256 != ""
    and $source.source_memory_write_execution_write_enable_fixture_report_sha256 != ""
    and $source.source_memory_write_execution_no_write_sink_contract_report_sha256 != ""
    and $source.source_memory_write_execution_denial_matrix_report_sha256 != ""
    and $source.source_memory_write_execution_preflight_report_sha256 != ""
    and $source.minimum_required_samples >= 24
    and $source.required_activation_command_result_receipt_cancellation_supersession_surface_count == 12
    and $source.ready_activation_command_result_receipt_cancellation_supersession_surface_count == 12
    and $source.side_effect_free_activation_command_result_receipt_cancellation_supersession_surface_count == 12
    and $source.required_activation_command_result_receipt_cancellation_supersession_fixture_count == 10
    and $source.activation_command_result_receipt_cancellation_supersession_fixture_count == 10
    and $source.blocked_activation_command_result_receipt_cancellation_supersession_fixture_count == 10
    and $source.noop_activation_command_result_receipt_cancellation_supersession_fixture_count == 10
    and $source.allowed_activation_command_result_receipt_cancellation_supersession_fixture_count == 0
    and $source.accepted_activation_command_result_receipt_cancellation_supersession_fixture_count == 0
    and $source.activation_command_result_receipt_cancellation_allowed == false
    and $source.activation_command_result_receipt_cancellation_recorded == false
    and $source.activation_command_result_receipt_cancellation_persisted == false
    and $source.activation_command_result_receipt_cancellation_request_accepted == false
    and $source.activation_command_result_receipt_supersession_allowed == false
    and $source.activation_command_result_receipt_supersession_recorded == false
    and $source.activation_command_result_receipt_supersession_persisted == false
    and $source.activation_command_result_receipt_supersession_request_accepted == false
    and $source.activation_command_result_receipt_replacement_receipt_accepted == false
    and $source.activation_command_result_receipt_replacement_hash_accepted == false
    and $source.activation_command_result_receipt_tombstone_recorded == false
    and $source.activation_command_result_receipt_delete_marker_recorded == false
    and $source.activation_command_result_receipt_ack_cancellation_accepted == false
    and $source.activation_command_result_receipt_ledger_cancellation_accepted == false
    and $source.activation_command_result_receipt_index_cancellation_accepted == false
    and $source.activation_command_result_receipt_delivery_cancellation_accepted == false
    and $source.activation_command_result_receipt_ordering_allowed == false
    and $source.activation_command_result_receipt_ordering_recorded == false
    and $source.activation_command_result_receipt_ordering_persisted == false
    and $source.activation_command_result_receipt_sequence_cursor_accepted == false
    and $source.activation_command_result_receipt_monotonicity_state_recorded == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_result_receipt_materialized == false
    and $source.activation_command_result_receipt_filesystem_written == false
    and $source.activation_command_result_receipt_ledger_written == false
    and $source.activation_command_result_receipt_indexed == false
    and $source.activation_command_result_receipt_enqueued == false
    and $source.activation_command_result_receipt_delivered == false
    and $source.activation_command_completion_ack_recorded == false
    and $source.activation_command_completion_ack_accepted == false
    and $source.activation_allowed_by_result_receipt_cancellation == false
    and $source.activation_allowed_by_result_receipt_supersession == false
    and $source.activation_allowed_by_result_receipt_ordering == false
    and $source.activation_allowed_by_result_receipt_replay == false
    and $source.activation_allowed_by_result_receipt == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and $source.live_mutation_execution_performed == false
    and $source.memory_write_execution_performed == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_write_performed_count == 0
    and $source.memory_store_mutated == false
    and $source.rollback_executed == false
    and $source.secret_material_read == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.external_send_performed == false
    and $source.public_release_published == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.launchd_mutated == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and ($source.activation_command_result_receipt_cancellation_supersession_fixtures | length) == 10
    and ($source.activation_command_result_receipt_cancellation_supersession_fixtures | all((.cancellation_supersession_status == "blocked_noop" or .cancellation_supersession_status == "blocked_supersession_noop") and .cancellation_allowed == false and .cancellation_recorded == false and .cancellation_persisted == false and .supersession_allowed == false and .supersession_recorded == false and .supersession_persisted == false and .replacement_receipt_accepted == false and .tombstone_recorded == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .completion_ack_recorded == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate" \
  --arg cancellation_supersession_report_sha256 "$cancellation_supersession_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$CANCELLATION_SUPERSESSION_JSON" \
  '
  def blocked_fixture($id; $reason; $extra):
    {
      id:$id,
      audit_trail_requested:true,
      immutable_evidence_requested:false,
      audit_evidence_status:"blocked_noop",
      source_cancellation_supersession_present:true,
      source_cancellation_supersession_ready:true,
      audit_trail_allowed:false,
      audit_trail_recorded:false,
      audit_trail_persisted:false,
      audit_trail_materialized:false,
      audit_trail_filesystem_written:false,
      immutable_evidence_allowed:false,
      immutable_evidence_recorded:false,
      immutable_evidence_persisted:false,
      immutable_evidence_materialized:false,
      immutable_evidence_filesystem_written:false,
      hash_chain_recorded:false,
      hash_chain_persisted:false,
      merkle_root_recorded:false,
      merkle_root_persisted:false,
      attestation_recorded:false,
      attestation_persisted:false,
      witness_recorded:false,
      witness_persisted:false,
      notary_recorded:false,
      notary_persisted:false,
      ledger_evidence_recorded:false,
      ledger_evidence_persisted:false,
      index_evidence_recorded:false,
      delivery_evidence_recorded:false,
      receipt_recorded:false,
      receipt_persisted:false,
      receipt_accepted:false,
      receipt_materialized:false,
      receipt_filesystem_written:false,
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
    activation_command_result_receipt_audit_trail_immutable_evidence_mode:"memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial",
    source_activation_command_result_receipt_cancellation_supersession_gate:$source.gate,
    source_activation_command_result_receipt_cancellation_supersession_ready:$source.memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready,
    source_activation_command_result_receipt_cancellation_supersession_report_sha256:$cancellation_supersession_report_sha256,
    source_activation_command_result_receipt_ordering_monotonicity_ready:$source.memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready,
    source_activation_command_result_receipt_ordering_monotonicity_report_sha256:$source.source_activation_command_result_receipt_ordering_monotonicity_report_sha256,
    source_activation_command_result_receipt_replay_idempotency_ready:$source.memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready,
    source_activation_command_result_receipt_replay_idempotency_report_sha256:$source.source_activation_command_result_receipt_replay_idempotency_report_sha256,
    source_activation_command_result_receipt_no_persistence_ready:$source.memory_write_execution_activation_command_result_receipt_no_persistence_ready,
    source_activation_command_result_receipt_no_persistence_report_sha256:$source.source_activation_command_result_receipt_no_persistence_report_sha256,
    source_activation_command_noop_handoff_ready:$source.memory_write_execution_activation_command_noop_handoff_ready,
    source_activation_command_noop_handoff_report_sha256:$source.source_activation_command_noop_handoff_report_sha256,
    source_memory_write_execution_activation_closure_denial_report_sha256:$source.source_memory_write_execution_activation_closure_denial_report_sha256,
    source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256:$source.source_memory_write_execution_post_write_operator_acceptance_denial_report_sha256,
    source_memory_write_execution_post_write_validation_dry_run_report_sha256:$source.source_memory_write_execution_post_write_validation_dry_run_report_sha256,
    source_memory_write_execution_write_enable_fixture_report_sha256:$source.source_memory_write_execution_write_enable_fixture_report_sha256,
    source_memory_write_execution_no_write_sink_contract_report_sha256:$source.source_memory_write_execution_no_write_sink_contract_report_sha256,
    source_memory_write_execution_denial_matrix_report_sha256:$source.source_memory_write_execution_denial_matrix_report_sha256,
    source_memory_write_execution_preflight_report_sha256:$source.source_memory_write_execution_preflight_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready:true,
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
    required_activation_command_result_receipt_audit_trail_immutable_evidence_surface_count:12,
    ready_activation_command_result_receipt_audit_trail_immutable_evidence_surface_count:12,
    side_effect_free_activation_command_result_receipt_audit_trail_immutable_evidence_surface_count:12,
    required_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count:10,
    activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count:10,
    blocked_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count:10,
    noop_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count:10,
    allowed_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count:0,
    accepted_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count:0,
    activation_command_result_receipt_audit_trail_denied_count:10,
    activation_command_result_receipt_immutable_evidence_denied_count:10,
    activation_command_result_receipt_audit_trail_performed_count:0,
    activation_command_result_receipt_immutable_evidence_performed_count:0,
    activation_command_result_receipt_audit_trail_allowed:false,
    activation_command_result_receipt_audit_trail_recorded:false,
    activation_command_result_receipt_audit_trail_persisted:false,
    activation_command_result_receipt_audit_trail_materialized:false,
    activation_command_result_receipt_audit_trail_filesystem_written:false,
    activation_command_result_receipt_immutable_evidence_allowed:false,
    activation_command_result_receipt_immutable_evidence_recorded:false,
    activation_command_result_receipt_immutable_evidence_persisted:false,
    activation_command_result_receipt_immutable_evidence_materialized:false,
    activation_command_result_receipt_immutable_evidence_filesystem_written:false,
    activation_command_result_receipt_hash_chain_recorded:false,
    activation_command_result_receipt_hash_chain_persisted:false,
    activation_command_result_receipt_merkle_root_recorded:false,
    activation_command_result_receipt_merkle_root_persisted:false,
    activation_command_result_receipt_attestation_recorded:false,
    activation_command_result_receipt_attestation_persisted:false,
    activation_command_result_receipt_witness_recorded:false,
    activation_command_result_receipt_witness_persisted:false,
    activation_command_result_receipt_notary_recorded:false,
    activation_command_result_receipt_notary_persisted:false,
    activation_command_result_receipt_ledger_evidence_recorded:false,
    activation_command_result_receipt_ledger_evidence_persisted:false,
    activation_command_result_receipt_index_evidence_recorded:false,
    activation_command_result_receipt_delivery_evidence_recorded:false,
    activation_command_result_receipt_cancellation_allowed:false,
    activation_command_result_receipt_cancellation_recorded:false,
    activation_command_result_receipt_cancellation_persisted:false,
    activation_command_result_receipt_supersession_allowed:false,
    activation_command_result_receipt_supersession_recorded:false,
    activation_command_result_receipt_supersession_persisted:false,
    activation_command_result_receipt_replacement_receipt_accepted:false,
    activation_command_result_receipt_replacement_hash_accepted:false,
    activation_command_result_receipt_tombstone_recorded:false,
    activation_command_result_receipt_delete_marker_recorded:false,
    activation_command_result_receipt_ordering_allowed:false,
    activation_command_result_receipt_ordering_recorded:false,
    activation_command_result_receipt_ordering_persisted:false,
    activation_command_result_receipt_sequence_cursor_accepted:false,
    activation_command_result_receipt_monotonicity_state_recorded:false,
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
    activation_allowed_by_result_receipt_audit_trail:false,
    activation_allowed_by_result_receipt_immutable_evidence:false,
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
    activation_command_result_receipt_audit_trail_immutable_evidence_surfaces:[
      "source_cancellation_supersession_report_required",
      "audit_trail_request_shape_denied",
      "immutable_evidence_request_shape_denied",
      "append_only_audit_log_recording_denied",
      "evidence_hash_chain_recording_denied",
      "attestation_witness_recording_denied",
      "audit_trail_materialization_denied",
      "immutable_evidence_persistence_denied",
      "ledger_index_delivery_evidence_denied",
      "activation_from_audit_evidence_denied",
      "memory_write_rollback_secret_provider_evidence_denied",
      "external_public_install_restart_evidence_denied"
    ],
    activation_command_result_receipt_audit_trail_immutable_evidence_fixtures:[
      blocked_fixture("activation-result-receipt-audit-missing-source-cancellation-report"; "source_cancellation_supersession_report_required"; {
        source_cancellation_supersession_present:false,
        source_cancellation_supersession_ready:false
      }),
      blocked_fixture("activation-result-receipt-audit-trail-append-request"; "audit_trail_append_request_denied"; {
        audit_trail_request_shape:"append_blocked_noop_receipt_audit_trail"
      }),
      blocked_fixture("activation-result-receipt-immutable-evidence-packet"; "immutable_evidence_packet_request_denied"; {
        immutable_evidence_requested:true,
        audit_trail_requested:false,
        immutable_evidence_request_shape:"seal_blocked_noop_receipt_as_immutable_evidence",
        audit_evidence_status:"blocked_evidence_noop"
      }),
      blocked_fixture("activation-result-receipt-hash-chain-merkle-root"; "hash_chain_merkle_root_evidence_denied"; {
        immutable_evidence_requested:true,
        audit_trail_requested:false,
        hash_chain_requested:true,
        merkle_root_requested:true,
        audit_evidence_status:"blocked_evidence_noop"
      }),
      blocked_fixture("activation-result-receipt-attestation-witness-notary"; "attestation_witness_notary_evidence_denied"; {
        immutable_evidence_requested:true,
        audit_trail_requested:false,
        attestation_requested:true,
        witness_requested:true,
        notary_requested:true,
        audit_evidence_status:"blocked_evidence_noop"
      }),
      blocked_fixture("activation-result-receipt-audit-trail-materialization"; "audit_trail_materialization_filesystem_denied"; {
        audit_trail_materialization_requested:true,
        audit_trail_filesystem_write_requested:true
      }),
      blocked_fixture("activation-result-receipt-ledger-index-delivery-evidence"; "ledger_index_delivery_evidence_denied"; {
        ledger_evidence_requested:true,
        index_evidence_requested:true,
        delivery_evidence_requested:true
      }),
      blocked_fixture("activation-result-receipt-activation-from-audit-evidence"; "activation_from_audit_evidence_denied"; {
        immutable_evidence_requested:true,
        audit_trail_requested:false,
        activation_from_audit_evidence_requested:true,
        audit_evidence_status:"blocked_evidence_noop"
      }),
      blocked_fixture("activation-result-receipt-memory-rollback-secret-provider-evidence"; "memory_write_rollback_secret_provider_evidence_denied"; {
        immutable_evidence_requested:true,
        audit_trail_requested:false,
        memory_write_evidence_requested:true,
        rollback_evidence_requested:true,
        secret_material_evidence_requested:true,
        provider_prompt_evidence_requested:true,
        audit_evidence_status:"blocked_evidence_noop"
      }),
      blocked_fixture("activation-result-receipt-external-public-install-evidence"; "external_public_install_restart_active_binary_evidence_denied"; {
        immutable_evidence_requested:true,
        audit_trail_requested:false,
        external_send_evidence_requested:true,
        public_claim_evidence_requested:true,
        release_artifact_evidence_requested:true,
        install_evidence_requested:true,
        service_restart_evidence_requested:true,
        active_binary_mutation_evidence_requested:true,
        audit_evidence_status:"blocked_evidence_noop"
      })
    ],
    denied_by_activation_command_result_receipt_audit_trail_immutable_evidence:[
      "source_cancellation_supersession_report_required",
      "audit_trail_request_acceptance_denied",
      "audit_trail_recording_denied",
      "audit_trail_persistence_denied",
      "audit_trail_materialization_denied",
      "immutable_evidence_request_acceptance_denied",
      "immutable_evidence_recording_denied",
      "immutable_evidence_persistence_denied",
      "immutable_evidence_materialization_denied",
      "hash_chain_recording_denied",
      "merkle_root_recording_denied",
      "attestation_recording_denied",
      "witness_recording_denied",
      "notary_recording_denied",
      "ledger_evidence_recording_denied",
      "index_evidence_recording_denied",
      "delivery_evidence_recording_denied",
      "activation_from_audit_evidence_denied",
      "memory_write_evidence_denied",
      "live_mutation_evidence_denied",
      "rollback_evidence_denied",
      "secret_material_evidence_denied",
      "provider_prompt_evidence_denied",
      "external_public_install_restart_active_binary_evidence_denied"
    ],
    side_effects:{
      activation_command_result_receipt_audit_trail_recorded:false,
      activation_command_result_receipt_audit_trail_persisted:false,
      activation_command_result_receipt_audit_trail_materialized:false,
      activation_command_result_receipt_audit_trail_filesystem_written:false,
      activation_command_result_receipt_immutable_evidence_recorded:false,
      activation_command_result_receipt_immutable_evidence_persisted:false,
      activation_command_result_receipt_immutable_evidence_materialized:false,
      activation_command_result_receipt_immutable_evidence_filesystem_written:false,
      activation_command_result_receipt_hash_chain_recorded:false,
      activation_command_result_receipt_hash_chain_persisted:false,
      activation_command_result_receipt_merkle_root_recorded:false,
      activation_command_result_receipt_merkle_root_persisted:false,
      activation_command_result_receipt_attestation_recorded:false,
      activation_command_result_receipt_attestation_persisted:false,
      activation_command_result_receipt_witness_recorded:false,
      activation_command_result_receipt_witness_persisted:false,
      activation_command_result_receipt_notary_recorded:false,
      activation_command_result_receipt_notary_persisted:false,
      activation_command_result_receipt_ledger_evidence_recorded:false,
      activation_command_result_receipt_ledger_evidence_persisted:false,
      activation_command_result_receipt_index_evidence_recorded:false,
      activation_command_result_receipt_delivery_evidence_recorded:false,
      activation_command_result_receipt_cancellation_recorded:false,
      activation_command_result_receipt_cancellation_persisted:false,
      activation_command_result_receipt_supersession_recorded:false,
      activation_command_result_receipt_supersession_persisted:false,
      activation_command_result_receipt_replacement_receipt_recorded:false,
      activation_command_result_receipt_replacement_receipt_persisted:false,
      activation_command_result_receipt_replacement_hash_accepted:false,
      activation_command_result_receipt_tombstone_recorded:false,
      activation_command_result_receipt_delete_marker_recorded:false,
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
  and .memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
  and .activation_command_result_receipt_audit_trail_immutable_evidence_mode == "memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial"
  and .source_activation_command_result_receipt_cancellation_supersession_ready == true
  and .source_activation_command_result_receipt_cancellation_supersession_report_sha256 != ""
  and .source_activation_command_result_receipt_ordering_monotonicity_ready == true
  and .source_activation_command_result_receipt_replay_idempotency_ready == true
  and .source_activation_command_result_receipt_no_persistence_ready == true
  and .minimum_required_samples >= 24
  and .required_activation_command_result_receipt_audit_trail_immutable_evidence_surface_count == 12
  and .ready_activation_command_result_receipt_audit_trail_immutable_evidence_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_audit_trail_immutable_evidence_surface_count == 12
  and .required_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count == 10
  and .activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count == 10
  and .blocked_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count == 10
  and .noop_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count == 10
  and .allowed_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count == 0
  and .accepted_activation_command_result_receipt_audit_trail_immutable_evidence_fixture_count == 0
  and .activation_command_result_receipt_audit_trail_denied_count == 10
  and .activation_command_result_receipt_immutable_evidence_denied_count == 10
  and .activation_command_result_receipt_audit_trail_performed_count == 0
  and .activation_command_result_receipt_immutable_evidence_performed_count == 0
  and .activation_command_result_receipt_audit_trail_allowed == false
  and .activation_command_result_receipt_audit_trail_recorded == false
  and .activation_command_result_receipt_audit_trail_persisted == false
  and .activation_command_result_receipt_audit_trail_materialized == false
  and .activation_command_result_receipt_audit_trail_filesystem_written == false
  and .activation_command_result_receipt_immutable_evidence_allowed == false
  and .activation_command_result_receipt_immutable_evidence_recorded == false
  and .activation_command_result_receipt_immutable_evidence_persisted == false
  and .activation_command_result_receipt_immutable_evidence_materialized == false
  and .activation_command_result_receipt_immutable_evidence_filesystem_written == false
  and .activation_command_result_receipt_hash_chain_recorded == false
  and .activation_command_result_receipt_hash_chain_persisted == false
  and .activation_command_result_receipt_merkle_root_recorded == false
  and .activation_command_result_receipt_attestation_recorded == false
  and .activation_command_result_receipt_witness_recorded == false
  and .activation_command_result_receipt_notary_recorded == false
  and .activation_command_result_receipt_ledger_evidence_recorded == false
  and .activation_command_result_receipt_index_evidence_recorded == false
  and .activation_command_result_receipt_delivery_evidence_recorded == false
  and .activation_command_result_receipt_cancellation_allowed == false
  and .activation_command_result_receipt_supersession_allowed == false
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
  and .activation_command_completion_ack_accepted == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_allowed_by_result_receipt_audit_trail == false
  and .activation_allowed_by_result_receipt_immutable_evidence == false
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
  and (.activation_command_result_receipt_audit_trail_immutable_evidence_surfaces | length) == 12
  and (.activation_command_result_receipt_audit_trail_immutable_evidence_fixtures | length) == 10
  and (.activation_command_result_receipt_audit_trail_immutable_evidence_fixtures | all((.audit_evidence_status == "blocked_noop" or .audit_evidence_status == "blocked_evidence_noop") and .audit_trail_allowed == false and .audit_trail_recorded == false and .audit_trail_persisted == false and .immutable_evidence_allowed == false and .immutable_evidence_recorded == false and .immutable_evidence_persisted == false and .hash_chain_recorded == false and .attestation_recorded == false and .witness_recorded == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .completion_ack_recorded == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
  and ([.activation_command_result_receipt_audit_trail_immutable_evidence_fixtures[] | select(.source_cancellation_supersession_present == false)] | length) == 1
  and ([.activation_command_result_receipt_audit_trail_immutable_evidence_fixtures[] | select(.audit_trail_request_shape == "append_blocked_noop_receipt_audit_trail")] | length) == 1
  and ([.activation_command_result_receipt_audit_trail_immutable_evidence_fixtures[] | select(.immutable_evidence_request_shape == "seal_blocked_noop_receipt_as_immutable_evidence")] | length) == 1
  and ([.activation_command_result_receipt_audit_trail_immutable_evidence_fixtures[] | select(.hash_chain_requested == true and .merkle_root_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_audit_trail_immutable_evidence_fixtures[] | select(.attestation_requested == true and .witness_requested == true and .notary_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_audit_trail_immutable_evidence_fixtures[] | select(.audit_trail_materialization_requested == true and .audit_trail_filesystem_write_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_audit_trail_immutable_evidence_fixtures[] | select(.ledger_evidence_requested == true and .index_evidence_requested == true and .delivery_evidence_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_audit_trail_immutable_evidence_fixtures[] | select(.activation_from_audit_evidence_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_audit_trail_immutable_evidence_fixtures[] | select(.memory_write_evidence_requested == true and .rollback_evidence_requested == true and .provider_prompt_evidence_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_audit_trail_immutable_evidence_fixtures[] | select(.external_send_evidence_requested == true and .install_evidence_requested == true and .active_binary_mutation_evidence_requested == true)] | length) == 1
  and (.denied_by_activation_command_result_receipt_audit_trail_immutable_evidence | length) == 24
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory live mutation operator write execution activation command result receipt audit-trail/immutable-evidence denial gate passed"
