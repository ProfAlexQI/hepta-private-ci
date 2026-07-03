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

ACCEPTED_GATE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-operator-approval-nonce-command-accepted-gate-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-operator-approval-nonce-command-accepted-gate-boundary-gate.sh
)"

source_report_sha256="$(printf '%s' "$ACCEPTED_GATE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$ACCEPTED_GATE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.minimal_scoped_memory_real_write_canary_accepted_gate_ready == true
    and $source.minimal_scoped_memory_real_write_canary_authority_accepted_no_write == true
    and $source.accepted_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count == 1
    and $source.blocked_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count == 9
    and $source.minimal_scoped_memory_real_write_canary_authority_accepted_count == 1
    and $source.single_use_nonce_consumed_count == 0
    and $source.explicit_command_dispatched_count == 0
    and $source.wal_write_performed_count == 0
    and $source.receipt_persisted_count == 0
    and $source.post_write_readback_performed_count == 0
    and $source.durable_memory_store_read_performed_count == 0
    and $source.durable_memory_store_write_performed_count == 0
    and $source.durable_memory_store_rollback_performed_count == 0
    and $source.memory_store_write_performed_count == 0
    and $source.single_use_nonce_consumed == false
    and $source.explicit_command_dispatched == false
    and $source.wal_write_performed == false
    and $source.receipt_persisted == false
    and $source.memory_write_execution_performed == false
    and $source.memory_store_write_performed == false
    and $source.durable_memory_store_write_performed == false
    and $source.durable_memory_store_read_performed == false
    and $source.durable_memory_store_rollback_performed == false
    and $source.live_kg_write_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.credential_read == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.active_binary_mutated == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ACCEPTED_GATE_JSON" \
    '
    def accepted_fixture:
      {
        id:"minimal-scoped-memory-real-write-canary-wal-receipt-binding-envelope",
        minimal_scoped_memory_real_write_canary_wal_receipt_binding_status:"accepted_wal_receipt_binding_noop",
        source_accepted_gate_present:true,
        source_accepted_gate_ready:true,
        wal_namespace_store_scope_binding_requested:true,
        wal_record_identity_binding_requested:true,
        wal_sequence_monotonicity_guard_requested:true,
        wal_idempotency_key_requested:true,
        payload_digest_redaction_binding_requested:true,
        receipt_identity_binding_requested:true,
        receipt_hash_chain_binding_requested:true,
        receipt_replay_guard_requested:true,
        receipt_audit_evidence_binding_requested:true,
        post_write_readback_handoff_binding_requested:true,
        minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted:true,
        wal_namespace_bound:true,
        wal_store_bound:true,
        wal_scope_bound:true,
        wal_record_id_bound:true,
        wal_sequence_guard_bound:true,
        wal_idempotency_key_bound:true,
        wal_payload_digest_bound:true,
        wal_payload_redaction_bound:true,
        receipt_id_bound:true,
        receipt_hash_chain_bound:true,
        receipt_replay_guard_bound:true,
        receipt_audit_evidence_bound:true,
        post_write_readback_handoff_bound:true,
        wal_receipt_binding_noop_confirmed:true,
        single_use_nonce_consumed:false,
        explicit_command_dispatched:false,
        wal_write_performed:false,
        wal_recorded:false,
        wal_persisted:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_materialized:false,
        receipt_delivered:false,
        post_write_readback_performed:false,
        rollback_executed:false,
        tombstone_written:false,
        memory_write_execution_performed:false,
        memory_store_write_performed:false,
        durable_memory_store_write_performed:false,
        durable_memory_store_read_performed:false,
        durable_memory_store_rollback_performed:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        channel_send_performed:false,
        external_send_performed:false,
        release_artifact_written:false,
        install_executed:false,
        active_binary_mutated:false
      };
    def blocked_fixture($id; $reason; $extra):
      ({
        id:$id,
        minimal_scoped_memory_real_write_canary_wal_receipt_binding_status:"blocked_noop",
        source_accepted_gate_present:true,
        source_accepted_gate_ready:true,
        reason:$reason,
        minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted:false,
        wal_namespace_bound:false,
        wal_store_bound:false,
        wal_scope_bound:false,
        wal_record_id_bound:false,
        wal_sequence_guard_bound:false,
        wal_idempotency_key_bound:false,
        wal_payload_digest_bound:false,
        wal_payload_redaction_bound:false,
        receipt_id_bound:false,
        receipt_hash_chain_bound:false,
        receipt_replay_guard_bound:false,
        receipt_audit_evidence_bound:false,
        post_write_readback_handoff_bound:false,
        wal_receipt_binding_noop_confirmed:true,
        single_use_nonce_consumed:false,
        explicit_command_dispatched:false,
        wal_write_performed:false,
        wal_recorded:false,
        wal_persisted:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_materialized:false,
        receipt_delivered:false,
        post_write_readback_performed:false,
        rollback_executed:false,
        tombstone_written:false,
        memory_write_execution_performed:false,
        memory_store_write_performed:false,
        durable_memory_store_write_performed:false,
        durable_memory_store_read_performed:false,
        durable_memory_store_rollback_performed:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        channel_send_performed:false,
        external_send_performed:false,
        release_artifact_written:false,
        install_executed:false,
        active_binary_mutated:false
      } + $extra);
    def false_side_effects:
      {
        single_use_nonce_consumed:false,
        explicit_command_dispatched:false,
        wal_write_performed:false,
        wal_recorded:false,
        wal_persisted:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_materialized:false,
        receipt_delivered:false,
        post_write_readback_performed:false,
        rollback_executed:false,
        tombstone_written:false,
        activation_performed:false,
        live_mutation_execution_performed:false,
        memory_write_execution_performed:false,
        memory_store_write_performed:false,
        durable_memory_store_write_performed:false,
        durable_memory_store_read_performed:false,
        durable_memory_store_rollback_performed:false,
        credential_read:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        channel_send_performed:false,
        external_send_performed:false,
        release_artifact_written:false,
        install_executed:false,
        active_binary_mutated:false
      };
    [
      accepted_fixture,
      blocked_fixture("minimal-scoped-memory-real-write-canary-wal-receipt-missing-accepted-gate-source"; "source_minimal_scoped_memory_real_write_canary_accepted_gate_required"; {source_accepted_gate_present:false, source_accepted_gate_ready:false}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-wal-scope-required"; "wal_namespace_store_scope_binding_required"; {wal_namespace_store_scope_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-wal-record-required"; "wal_record_identity_binding_required"; {wal_record_identity_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-wal-sequence-required"; "wal_sequence_monotonicity_guard_required"; {wal_sequence_monotonicity_guard_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-wal-idempotency-required"; "wal_idempotency_key_required"; {wal_idempotency_key_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-receipt-identity-required"; "receipt_identity_binding_required"; {receipt_identity_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-receipt-hash-replay-required"; "receipt_hash_chain_and_replay_guard_required"; {receipt_hash_chain_binding_requested:true, receipt_replay_guard_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-receipt-audit-readback-required"; "receipt_audit_evidence_and_readback_handoff_required"; {receipt_audit_evidence_binding_requested:true, post_write_readback_handoff_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-wal-receipt-direct-side-effect-attempt"; "direct_wal_receipt_memory_and_external_side_effects_denied"; {single_use_nonce_consumption_requested:true, explicit_command_dispatch_requested:true, wal_write_requested:true, wal_persistence_requested:true, receipt_recording_requested:true, receipt_persistence_requested:true, durable_memory_write_requested:true, kg_live_write_requested:true, provider_model_invocation_requested:true, credential_read_requested:true, channel_external_send_requested:true, release_artifact_write_requested:true, install_restart_requested:true, active_binary_mutation_requested:true})
    ] as $fixtures
    | {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        side_effect_free:true,
        audit_date:"2026-07-03",
        minimal_scoped_memory_real_write_canary_wal_receipt_binding_ready:true,
        minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted_no_write:true,
        source_minimal_scoped_memory_real_write_canary_accepted_gate_boundary_ready:true,
        source_minimal_scoped_memory_real_write_canary_accepted_gate_report_sha256:$source_report_sha256,
        source_accepted_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count:($source.accepted_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count // 0),
        source_blocked_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count:($source.blocked_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count // 0),
        source_minimal_scoped_memory_real_write_canary_authority_accepted_count:($source.minimal_scoped_memory_real_write_canary_authority_accepted_count // 0),
        source_single_use_nonce_consumed_count:($source.single_use_nonce_consumed_count // 0),
        source_explicit_command_dispatched_count:($source.explicit_command_dispatched_count // 0),
        source_wal_write_performed_count:($source.wal_write_performed_count // 0),
        source_receipt_persisted_count:($source.receipt_persisted_count // 0),
        minimum_required_samples:$min_long_soak_samples,
        required_minimal_scoped_memory_real_write_canary_wal_receipt_binding_surface_count:12,
        ready_minimal_scoped_memory_real_write_canary_wal_receipt_binding_surface_count:12,
        side_effect_free_minimal_scoped_memory_real_write_canary_wal_receipt_binding_surface_count:12,
        required_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count:10,
        minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count:($fixtures | length),
        accepted_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count:([$fixtures[] | select(.minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted == true)] | length),
        blocked_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count:([$fixtures[] | select(.minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted == false)] | length),
        wal_receipt_binding_authority_accepted_count:1,
        wal_namespace_store_scope_bound_count:1,
        wal_record_id_bound_count:1,
        wal_sequence_guard_bound_count:1,
        wal_idempotency_key_bound_count:1,
        wal_payload_digest_redaction_bound_count:1,
        receipt_id_bound_count:1,
        receipt_hash_chain_bound_count:1,
        receipt_replay_guard_bound_count:1,
        receipt_audit_evidence_bound_count:1,
        post_write_readback_handoff_bound_count:1,
        minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted_count:1,
        single_use_nonce_consumed_count:0,
        explicit_command_dispatched_count:0,
        wal_write_performed_count:0,
        wal_recorded_count:0,
        wal_persisted_count:0,
        receipt_recorded_count:0,
        receipt_persisted_count:0,
        receipt_materialized_count:0,
        receipt_delivered_count:0,
        post_write_readback_performed_count:0,
        rollback_performed_count:0,
        tombstone_written_count:0,
        durable_memory_store_read_performed_count:0,
        durable_memory_store_write_performed_count:0,
        durable_memory_store_rollback_performed_count:0,
        memory_store_write_performed_count:0,
        required_before_minimal_scoped_memory_real_write_canary_wal_receipt_binding_count:17,
        minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixtures:$fixtures,
        denied_by_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_count:28,
        source_minimal_scoped_memory_real_write_canary_accepted_gate_required:true,
        minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted:true,
        wal_namespace_bound:true,
        wal_store_bound:true,
        wal_scope_bound:true,
        wal_record_id_bound:true,
        wal_sequence_guard_bound:true,
        wal_idempotency_key_bound:true,
        wal_payload_digest_bound:true,
        wal_payload_redaction_bound:true,
        receipt_id_bound:true,
        receipt_hash_chain_bound:true,
        receipt_replay_guard_bound:true,
        receipt_audit_evidence_bound:true,
        post_write_readback_handoff_bound:true,
        nonce_consumption_forbidden_on_report_route:true,
        explicit_command_dispatch_forbidden_on_report_route:true,
        wal_write_forbidden:true,
        wal_persistence_forbidden:true,
        receipt_recording_forbidden:true,
        receipt_persistence_forbidden:true,
        single_use_nonce_consumed:false,
        explicit_command_dispatched:false,
        wal_write_performed:false,
        wal_recorded:false,
        wal_persisted:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_materialized:false,
        receipt_delivered:false,
        post_write_readback_performed:false,
        rollback_executed:false,
        tombstone_written:false,
        memory_write_execution_performed:false,
        memory_store_write_performed:false,
        durable_memory_store_write_performed:false,
        durable_memory_store_read_performed:false,
        durable_memory_store_rollback_performed:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        channel_send_performed:false,
        external_send_performed:false,
        release_artifact_written:false,
        install_executed:false,
        active_binary_mutated:false,
        side_effects:false_side_effects
      }
    '
)"

jq -e '
  .status == "ready"
  and .minimal_scoped_memory_real_write_canary_wal_receipt_binding_ready == true
  and .minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted_no_write == true
  and .source_minimal_scoped_memory_real_write_canary_accepted_gate_boundary_ready == true
  and .accepted_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count == 1
  and .blocked_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count == 9
  and .wal_receipt_binding_authority_accepted_count == 1
  and .wal_record_id_bound_count == 1
  and .receipt_id_bound_count == 1
  and .post_write_readback_handoff_bound_count == 1
  and .single_use_nonce_consumed_count == 0
  and .explicit_command_dispatched_count == 0
  and .wal_write_performed_count == 0
  and .wal_recorded_count == 0
  and .wal_persisted_count == 0
  and .receipt_recorded_count == 0
  and .receipt_persisted_count == 0
  and .receipt_materialized_count == 0
  and .receipt_delivered_count == 0
  and .durable_memory_store_read_performed_count == 0
  and .durable_memory_store_write_performed_count == 0
  and .durable_memory_store_rollback_performed_count == 0
  and .memory_store_write_performed_count == 0
  and .single_use_nonce_consumed == false
  and .explicit_command_dispatched == false
  and .wal_write_performed == false
  and .receipt_recorded == false
  and .receipt_persisted == false
  and .memory_write_execution_performed == false
  and .memory_store_write_performed == false
  and .durable_memory_store_write_performed == false
  and .durable_memory_store_read_performed == false
  and .durable_memory_store_rollback_performed == false
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .active_binary_mutated == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
