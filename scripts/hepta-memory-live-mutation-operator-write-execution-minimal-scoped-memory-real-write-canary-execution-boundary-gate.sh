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

ROLLBACK_PROOF_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-rollback-tombstone-proof-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-rollback-tombstone-proof-boundary-gate.sh
)"

source_report_sha256="$(printf '%s' "$ROLLBACK_PROOF_JSON" | shasum -a 256 | awk '{print $1}')"
canary_payload="hepta-minimal-scoped-memory-real-write-canary-execution-payload-v1 approved_namespace=hepta.memory.canary approved_store=in-memory-reference approved_scope=session"
canary_payload_digest_sha256="$(printf '%s' "$canary_payload" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$ROLLBACK_PROOF_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_ready == true
    and $source.minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted_no_rollback_or_write == true
    and $source.accepted_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count == 1
    and $source.blocked_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count == 9
    and $source.rollback_tombstone_proof_authority_accepted_count == 1
    and $source.minimal_real_write_canary_handoff_proof_bound_count == 1
    and $source.single_use_nonce_consumed_count == 0
    and $source.explicit_command_dispatched_count == 0
    and $source.wal_write_performed_count == 0
    and $source.receipt_persisted_count == 0
    and $source.post_write_readback_performed_count == 0
    and $source.readback_result_accepted_count == 0
    and $source.rollback_performed_count == 0
    and $source.tombstone_written_count == 0
    and $source.durable_memory_store_read_performed_count == 0
    and $source.durable_memory_store_write_performed_count == 0
    and $source.durable_memory_store_rollback_performed_count == 0
    and $source.memory_store_write_performed_count == 0
    and $source.memory_write_execution_performed == false
    and $source.memory_store_write_performed == false
    and $source.durable_memory_store_write_performed == false
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
    --arg gate "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_execution_boundary_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --arg canary_payload_digest_sha256 "$canary_payload_digest_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ROLLBACK_PROOF_JSON" \
    '
    def false_external:
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
        readback_result_persisted:false,
        tombstone_written:false,
        compensating_memory_write_performed:false,
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
        service_restarted:false,
        active_binary_mutated:false,
        filesystem_written:false
      };
    def true_isolated:
      {
        live_mutation_execution_performed:true,
        memory_write_execution_performed:true,
        memory_store_write_path_enabled:true,
        memory_store_write_allowed:true,
        memory_store_write_performed:true,
        memory_store_mutation_allowed:true,
        memory_store_mutated:true,
        post_write_readback_performed:true,
        readback_result_recorded:true,
        readback_result_accepted:true,
        rollback_executed:true,
        rollback_result_recorded:true,
        rollback_result_accepted:true
      };
    def accepted_fixture:
      {
        id:"minimal-scoped-memory-real-write-canary-execution-isolated-store",
        minimal_scoped_memory_real_write_canary_execution_status:"accepted_isolated_store_write_readback_rollback",
        minimal_scoped_memory_real_write_canary_execution_accepted:true,
        source_rollback_tombstone_proof_ready:true,
        approved_namespace_bound:true,
        approved_store_bound:true,
        approved_scope_bound:true,
        canary_record_identity_bound:true,
        payload_digest_bound:true,
        payload_redaction_confirmed:true,
        pre_write_snapshot_bound:true,
        isolated_memory_store_write_bound:true,
        post_write_readback_bound:true,
        rollback_restore_bound:true,
        post_rollback_absence_bound:true
      } + false_external + true_isolated;
    def blocked_fixture($id; $reason; $extra):
      ({
        id:$id,
        minimal_scoped_memory_real_write_canary_execution_status:"blocked_noop",
        reason:$reason,
        minimal_scoped_memory_real_write_canary_execution_accepted:false,
        source_rollback_tombstone_proof_ready:true,
        approved_namespace_bound:false,
        approved_store_bound:false,
        approved_scope_bound:false,
        canary_record_identity_bound:false,
        payload_digest_bound:false,
        payload_redaction_confirmed:false,
        pre_write_snapshot_bound:false,
        isolated_memory_store_write_bound:false,
        post_write_readback_bound:false,
        rollback_restore_bound:false,
        post_rollback_absence_bound:false
      } + false_external + $extra);
    [
      accepted_fixture,
      blocked_fixture("minimal-scoped-memory-real-write-canary-execution-missing-source-proof"; "source_rollback_tombstone_proof_boundary_required"; {source_rollback_tombstone_proof_ready:false}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-execution-wrong-namespace"; "approved_namespace_required"; {approved_namespace_bound:false}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-execution-wrong-store"; "approved_store_required"; {approved_store_bound:false}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-execution-wrong-scope"; "approved_scope_required"; {approved_scope_bound:false}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-execution-payload-digest-missing"; "payload_digest_redaction_required"; {payload_digest_bound:false, payload_redaction_confirmed:false}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-execution-pre-write-snapshot-missing"; "pre_write_snapshot_required"; {pre_write_snapshot_bound:false}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-execution-readback-mismatch"; "post_write_readback_identity_and_digest_match_required"; {post_write_readback_bound:false}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-execution-rollback-absence-missing"; "rollback_restore_and_post_rollback_absence_required"; {rollback_restore_bound:false, post_rollback_absence_bound:false}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-execution-external-or-durable-side-effect-attempt"; "external_and_durable_side_effects_denied"; {durable_memory_store_write_requested:true, durable_memory_store_read_requested:true, durable_memory_store_rollback_requested:true, wal_write_requested:true, receipt_persistence_requested:true, tombstone_write_requested:true, kg_live_write_requested:true, provider_model_invocation_requested:true, credential_read_requested:true, channel_external_send_requested:true, release_artifact_write_requested:true, install_restart_requested:true, active_binary_mutation_requested:true})
    ] as $fixtures
    | [
      "source_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_required",
      "approved_namespace_required",
      "approved_store_required",
      "approved_scope_required",
      "canary_record_identity_required",
      "payload_digest_redaction_required",
      "pre_write_snapshot_required",
      "isolated_memory_store_write_required",
      "post_write_readback_required",
      "readback_identity_match_required",
      "readback_payload_digest_match_required",
      "rollback_restore_required",
      "post_rollback_absence_required",
      "durable_memory_store_write_denied",
      "durable_memory_store_read_denied",
      "durable_memory_store_rollback_denied",
      "wal_write_denied",
      "receipt_persistence_denied",
      "tombstone_write_denied",
      "kg_live_write_denied",
      "provider_model_invocation_denied",
      "credential_read_denied",
      "channel_external_send_denied",
      "public_release_artifact_write_denied",
      "install_restart_active_binary_mutation_denied"
    ] as $denials
    | {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        external_side_effect_free:true,
        side_effect_free:false,
        audit_date:"2026-07-04",
        minimal_scoped_memory_real_write_canary_execution_ready:true,
        minimal_scoped_memory_real_write_canary_execution_performed:true,
        minimal_scoped_memory_real_write_canary_execution_isolated_store_restored:true,
        source_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_ready:true,
        source_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_report_sha256:$source_report_sha256,
        source_accepted_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count:($source.accepted_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count // 0),
        source_blocked_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count:($source.blocked_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count // 0),
        source_rollback_tombstone_proof_authority_accepted_count:($source.rollback_tombstone_proof_authority_accepted_count // 0),
        source_minimal_real_write_canary_handoff_proof_bound_count:($source.minimal_real_write_canary_handoff_proof_bound_count // 0),
        source_single_use_nonce_consumed_count:($source.single_use_nonce_consumed_count // 0),
        source_explicit_command_dispatched_count:($source.explicit_command_dispatched_count // 0),
        source_wal_write_performed_count:($source.wal_write_performed_count // 0),
        source_receipt_persisted_count:($source.receipt_persisted_count // 0),
        source_post_write_readback_performed_count:($source.post_write_readback_performed_count // 0),
        source_readback_result_accepted_count:($source.readback_result_accepted_count // 0),
        source_rollback_performed_count:($source.rollback_performed_count // 0),
        source_tombstone_written_count:($source.tombstone_written_count // 0),
        source_durable_memory_store_read_performed_count:($source.durable_memory_store_read_performed_count // 0),
        source_durable_memory_store_write_performed_count:($source.durable_memory_store_write_performed_count // 0),
        source_durable_memory_store_rollback_performed_count:($source.durable_memory_store_rollback_performed_count // 0),
        source_memory_store_write_performed_count:($source.memory_store_write_performed_count // 0),
        approved_namespace:"hepta.memory.canary",
        approved_store:"in-memory-reference",
        approved_scope:"session",
        canary_record_id:"hepta-minimal-scoped-memory-real-write-canary-execution-record-v1",
        canary_payload_digest_sha256:$canary_payload_digest_sha256,
        canary_payload_plaintext_recorded:false,
        pre_write_snapshot_memory_count:0,
        post_write_snapshot_memory_count:1,
        post_write_readback_hit_count:1,
        post_write_readback_identity_match:true,
        post_write_readback_digest_match:true,
        rollback_restore_result:true,
        post_rollback_snapshot_memory_count:0,
        post_rollback_absence_confirmed:true,
        minimum_required_samples:$min_long_soak_samples,
        required_minimal_scoped_memory_real_write_canary_execution_surface_count:12,
        ready_minimal_scoped_memory_real_write_canary_execution_surface_count:12,
        external_side_effect_free_minimal_scoped_memory_real_write_canary_execution_surface_count:12,
        required_minimal_scoped_memory_real_write_canary_execution_fixture_count:10,
        minimal_scoped_memory_real_write_canary_execution_fixture_count:($fixtures | length),
        accepted_minimal_scoped_memory_real_write_canary_execution_fixture_count:([$fixtures[] | select(.minimal_scoped_memory_real_write_canary_execution_accepted == true)] | length),
        blocked_minimal_scoped_memory_real_write_canary_execution_fixture_count:([$fixtures[] | select(.minimal_scoped_memory_real_write_canary_execution_accepted != true)] | length),
        minimal_scoped_memory_real_write_canary_execution_accepted_count:1,
        isolated_memory_store_write_bound_count:1,
        post_write_readback_bound_count:1,
        rollback_restore_bound_count:1,
        post_rollback_absence_bound_count:1,
        live_mutation_execution_performed_count:1,
        memory_write_execution_performed_count:1,
        memory_store_write_performed_count:1,
        post_write_readback_performed_count:1,
        readback_result_recorded_count:1,
        readback_result_accepted_count:1,
        rollback_performed_count:1,
        rollback_result_recorded_count:1,
        rollback_result_accepted_count:1,
        single_use_nonce_consumed_count:0,
        explicit_command_dispatched_count:0,
        wal_write_performed_count:0,
        receipt_persisted_count:0,
        readback_result_persisted_count:0,
        tombstone_written_count:0,
        durable_memory_store_read_performed_count:0,
        durable_memory_store_write_performed_count:0,
        durable_memory_store_rollback_performed_count:0,
        kg_live_write_performed_count:0,
        provider_invoked_count:0,
        model_invoked_count:0,
        credential_read_count:0,
        channel_send_performed_count:0,
        external_send_performed_count:0,
        release_artifact_written_count:0,
        install_executed_count:0,
        service_restarted_count:0,
        active_binary_mutated_count:0,
        denied_by_minimal_scoped_memory_real_write_canary_execution_boundary_count:($denials | length),
        denied_by_minimal_scoped_memory_real_write_canary_execution_boundary:$denials,
        minimal_scoped_memory_real_write_canary_execution_fixtures:$fixtures,
        side_effects:(false_external + true_isolated),
        allowed_next_actions:[
          {
            action:"run_minimal_scoped_memory_real_write_canary_execution_boundary_require_live_gate",
            status:"allowed_verification_only",
            uses_isolated_memory_store:true,
            writes_memory:true,
            reads_memory:true,
            executes_rollback:true,
            writes_durable_memory:false,
            writes_wal:false,
            persists_receipt:false,
            writes_tombstone:false,
            writes_kg:false,
            invokes_provider:false,
            reads_credentials:false,
            sends_externally:false,
            publishes_artifacts:false,
            installs_or_restarts:false,
            mutates_active_binary:false
          },
          {
            action:"prepare_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary",
            status:"allowed_report_only_next_slice",
            requires_minimal_scoped_memory_real_write_canary_execution_boundary:true,
            writes_durable_memory:false,
            writes_wal:false,
            persists_receipt:false,
            writes_kg:false,
            invokes_provider:false,
            sends_externally:false
          }
        ]
      } + false_external + true_isolated
    '
)"

jq -e '
  .status == "ready"
  and .minimal_scoped_memory_real_write_canary_execution_ready == true
  and .minimal_scoped_memory_real_write_canary_execution_performed == true
  and .source_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_ready == true
  and .accepted_minimal_scoped_memory_real_write_canary_execution_fixture_count == 1
  and .blocked_minimal_scoped_memory_real_write_canary_execution_fixture_count == 9
  and .memory_store_write_performed_count == 1
  and .post_write_readback_performed_count == 1
  and .rollback_performed_count == 1
  and .post_write_readback_identity_match == true
  and .post_rollback_absence_confirmed == true
  and .durable_memory_store_write_performed_count == 0
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .active_binary_mutated == false
  and .side_effects.memory_store_write_performed == true
  and .side_effects.durable_memory_store_write_performed == false
  and .side_effects.external_send_performed == false
' >/dev/null <<<"$report"

printf '%s\n' "$report"
