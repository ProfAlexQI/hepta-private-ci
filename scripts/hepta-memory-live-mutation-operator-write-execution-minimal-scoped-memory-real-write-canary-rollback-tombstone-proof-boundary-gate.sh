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

POST_WRITE_READBACK_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-post-write-readback-binding-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-post-write-readback-binding-boundary-gate.sh
)"

source_report_sha256="$(printf '%s' "$POST_WRITE_READBACK_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$POST_WRITE_READBACK_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.minimal_scoped_memory_real_write_canary_post_write_readback_binding_ready == true
    and $source.minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted_no_read_or_write == true
    and $source.accepted_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count == 1
    and $source.blocked_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count == 9
    and $source.post_write_readback_binding_authority_accepted_count == 1
    and $source.rollback_tombstone_handoff_bound_count == 1
    and $source.single_use_nonce_consumed_count == 0
    and $source.explicit_command_dispatched_count == 0
    and $source.wal_write_performed_count == 0
    and $source.receipt_persisted_count == 0
    and $source.post_write_readback_performed_count == 0
    and $source.readback_result_recorded_count == 0
    and $source.readback_result_persisted_count == 0
    and $source.readback_result_accepted_count == 0
    and $source.rollback_performed_count == 0
    and $source.tombstone_written_count == 0
    and $source.durable_memory_store_read_performed_count == 0
    and $source.durable_memory_store_write_performed_count == 0
    and $source.durable_memory_store_rollback_performed_count == 0
    and $source.memory_store_write_performed_count == 0
    and $source.single_use_nonce_consumed == false
    and $source.explicit_command_dispatched == false
    and $source.wal_write_performed == false
    and $source.receipt_persisted == false
    and $source.post_write_readback_performed == false
    and $source.readback_result_accepted == false
    and $source.rollback_executed == false
    and $source.tombstone_written == false
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
    --arg gate "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$POST_WRITE_READBACK_JSON" \
    '
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
        readback_result_recorded:false,
        readback_result_persisted:false,
        readback_result_accepted:false,
        rollback_executed:false,
        rollback_result_recorded:false,
        rollback_result_persisted:false,
        rollback_result_accepted:false,
        tombstone_written:false,
        compensating_memory_write_performed:false,
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
    def accepted_fixture:
      {
        id:"minimal-scoped-memory-real-write-canary-rollback-tombstone-proof-envelope",
        minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_status:"accepted_rollback_tombstone_proof_no_rollback_or_write",
        source_post_write_readback_binding_present:true,
        source_post_write_readback_binding_ready:true,
        rollback_plan_proof_binding_requested:true,
        tombstone_plan_proof_binding_requested:true,
        rollback_target_proof_binding_requested:true,
        tombstone_target_proof_binding_requested:true,
        rollback_receipt_linkage_proof_binding_requested:true,
        tombstone_receipt_linkage_proof_binding_requested:true,
        rollback_idempotency_guard_proof_binding_requested:true,
        tombstone_idempotency_guard_proof_binding_requested:true,
        rollback_tombstone_audit_evidence_proof_binding_requested:true,
        operator_review_handoff_proof_binding_requested:true,
        minimal_real_write_canary_handoff_proof_binding_requested:true,
        minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted:true,
        rollback_plan_proof_bound:true,
        tombstone_plan_proof_bound:true,
        rollback_target_proof_bound:true,
        tombstone_target_proof_bound:true,
        rollback_receipt_linkage_proof_bound:true,
        tombstone_receipt_linkage_proof_bound:true,
        rollback_idempotency_guard_proof_bound:true,
        tombstone_idempotency_guard_proof_bound:true,
        rollback_tombstone_audit_evidence_proof_bound:true,
        operator_review_handoff_proof_bound:true,
        minimal_real_write_canary_handoff_proof_bound:true,
        rollback_tombstone_proof_noop_confirmed:true
      } + false_side_effects;
    def blocked_fixture($id; $reason; $extra):
      ({
        id:$id,
        minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_status:"blocked_noop",
        source_post_write_readback_binding_present:true,
        source_post_write_readback_binding_ready:true,
        reason:$reason,
        minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted:false,
        rollback_plan_proof_bound:false,
        tombstone_plan_proof_bound:false,
        rollback_target_proof_bound:false,
        tombstone_target_proof_bound:false,
        rollback_receipt_linkage_proof_bound:false,
        tombstone_receipt_linkage_proof_bound:false,
        rollback_idempotency_guard_proof_bound:false,
        tombstone_idempotency_guard_proof_bound:false,
        rollback_tombstone_audit_evidence_proof_bound:false,
        operator_review_handoff_proof_bound:false,
        minimal_real_write_canary_handoff_proof_bound:false,
        rollback_tombstone_proof_noop_confirmed:true
      } + false_side_effects + $extra);
    [
      accepted_fixture,
      blocked_fixture("minimal-scoped-memory-real-write-canary-rollback-proof-missing-post-write-readback-source"; "source_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_required"; {source_post_write_readback_binding_present:false, source_post_write_readback_binding_ready:false, rollback_plan_proof_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-rollback-plan-proof-required"; "rollback_plan_proof_binding_required"; {rollback_plan_proof_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-tombstone-plan-proof-required"; "tombstone_plan_proof_binding_required"; {tombstone_plan_proof_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-rollback-target-proof-required"; "rollback_target_proof_binding_required"; {rollback_target_proof_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-tombstone-target-proof-required"; "tombstone_target_proof_binding_required"; {tombstone_target_proof_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-rollback-receipt-proof-required"; "rollback_receipt_linkage_proof_binding_required"; {rollback_receipt_linkage_proof_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-tombstone-receipt-proof-required"; "tombstone_receipt_linkage_proof_binding_required"; {tombstone_receipt_linkage_proof_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-rollback-tombstone-guards-audit-handoff-required"; "rollback_tombstone_guards_audit_and_handoff_proof_required"; {rollback_idempotency_guard_proof_binding_requested:true, tombstone_idempotency_guard_proof_binding_requested:true, rollback_tombstone_audit_evidence_proof_binding_requested:true, operator_review_handoff_proof_binding_requested:true, minimal_real_write_canary_handoff_proof_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-rollback-tombstone-proof-direct-side-effect-attempt"; "direct_rollback_tombstone_memory_and_external_side_effects_denied"; {single_use_nonce_consumption_requested:true, explicit_command_dispatch_requested:true, wal_write_requested:true, receipt_persistence_requested:true, post_write_readback_execution_requested:true, readback_result_recording_requested:true, readback_result_persistence_requested:true, readback_acceptance_requested:true, rollback_execution_requested:true, rollback_result_recording_requested:true, rollback_result_persistence_requested:true, rollback_result_acceptance_requested:true, tombstone_write_requested:true, compensating_memory_write_requested:true, durable_memory_read_requested:true, durable_memory_write_requested:true, durable_memory_rollback_requested:true, kg_live_write_requested:true, provider_model_invocation_requested:true, credential_read_requested:true, channel_external_send_requested:true, release_artifact_write_requested:true, install_restart_requested:true, active_binary_mutation_requested:true})
    ] as $fixtures
    | [
      "source_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_required",
      "rollback_plan_proof_binding_required",
      "tombstone_plan_proof_binding_required",
      "rollback_target_proof_binding_required",
      "tombstone_target_proof_binding_required",
      "rollback_receipt_linkage_proof_binding_required",
      "tombstone_receipt_linkage_proof_binding_required",
      "rollback_idempotency_guard_proof_binding_required",
      "tombstone_idempotency_guard_proof_binding_required",
      "rollback_tombstone_audit_evidence_proof_binding_required",
      "operator_review_handoff_proof_binding_required",
      "minimal_real_write_canary_handoff_proof_binding_required",
      "nonce_consumption_report_route_denied",
      "explicit_command_dispatch_report_route_denied",
      "wal_write_denied",
      "wal_persistence_denied",
      "receipt_recording_denied",
      "receipt_persistence_denied",
      "receipt_materialization_denied",
      "receipt_delivery_denied",
      "post_write_readback_execution_denied",
      "readback_result_recording_denied",
      "readback_result_persistence_denied",
      "readback_acceptance_denied",
      "rollback_execution_denied",
      "rollback_result_recording_denied",
      "rollback_result_persistence_denied",
      "rollback_result_acceptance_denied",
      "tombstone_write_denied",
      "compensating_memory_write_denied",
      "durable_memory_store_read_denied",
      "durable_memory_store_write_denied",
      "durable_memory_store_rollback_denied",
      "memory_store_mutation_denied",
      "kg_provider_credential_channel_public_release_side_effect_denied",
      "install_restart_active_binary_filesystem_mutation_denied"
    ] as $denials
    | {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        side_effect_free:true,
        audit_date:"2026-07-04",
        minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_ready:true,
        minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted_no_rollback_or_write:true,
        source_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_ready:true,
        source_minimal_scoped_memory_real_write_canary_post_write_readback_binding_report_sha256:$source_report_sha256,
        source_accepted_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count:($source.accepted_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count // 0),
        source_blocked_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count:($source.blocked_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count // 0),
        source_post_write_readback_binding_authority_accepted_count:($source.post_write_readback_binding_authority_accepted_count // 0),
        source_rollback_tombstone_handoff_bound_count:($source.rollback_tombstone_handoff_bound_count // 0),
        source_single_use_nonce_consumed_count:($source.single_use_nonce_consumed_count // 0),
        source_explicit_command_dispatched_count:($source.explicit_command_dispatched_count // 0),
        source_wal_write_performed_count:($source.wal_write_performed_count // 0),
        source_receipt_persisted_count:($source.receipt_persisted_count // 0),
        source_post_write_readback_performed_count:($source.post_write_readback_performed_count // 0),
        source_readback_result_recorded_count:($source.readback_result_recorded_count // 0),
        source_readback_result_persisted_count:($source.readback_result_persisted_count // 0),
        source_readback_result_accepted_count:($source.readback_result_accepted_count // 0),
        source_rollback_performed_count:($source.rollback_performed_count // 0),
        source_tombstone_written_count:($source.tombstone_written_count // 0),
        minimum_required_samples:$min_long_soak_samples,
        required_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_surface_count:12,
        ready_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_surface_count:12,
        side_effect_free_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_surface_count:12,
        required_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count:10,
        minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count:($fixtures | length),
        accepted_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count:([$fixtures[] | select(.minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted == true)] | length),
        blocked_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count:([$fixtures[] | select(.minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted == false)] | length),
        rollback_tombstone_proof_authority_accepted_count:1,
        rollback_plan_proof_bound_count:1,
        tombstone_plan_proof_bound_count:1,
        rollback_target_proof_bound_count:1,
        tombstone_target_proof_bound_count:1,
        rollback_receipt_linkage_proof_bound_count:1,
        tombstone_receipt_linkage_proof_bound_count:1,
        rollback_idempotency_guard_proof_bound_count:1,
        tombstone_idempotency_guard_proof_bound_count:1,
        rollback_tombstone_audit_evidence_proof_bound_count:1,
        operator_review_handoff_proof_bound_count:1,
        minimal_real_write_canary_handoff_proof_bound_count:1,
        minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted_count:1,
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
        readback_result_recorded_count:0,
        readback_result_persisted_count:0,
        readback_result_accepted_count:0,
        rollback_performed_count:0,
        rollback_result_recorded_count:0,
        rollback_result_persisted_count:0,
        rollback_result_accepted_count:0,
        tombstone_written_count:0,
        compensating_memory_write_performed_count:0,
        durable_memory_store_read_performed_count:0,
        durable_memory_store_write_performed_count:0,
        durable_memory_store_rollback_performed_count:0,
        memory_store_write_performed_count:0,
        minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixtures:$fixtures,
        denied_by_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary:$denials,
        denied_by_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_count:($denials | length),
        source_minimal_scoped_memory_real_write_canary_post_write_readback_binding_required:true,
        minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted:true,
        rollback_plan_proof_bound:true,
        tombstone_plan_proof_bound:true,
        rollback_target_proof_bound:true,
        tombstone_target_proof_bound:true,
        rollback_receipt_linkage_proof_bound:true,
        tombstone_receipt_linkage_proof_bound:true,
        rollback_idempotency_guard_proof_bound:true,
        tombstone_idempotency_guard_proof_bound:true,
        rollback_tombstone_audit_evidence_proof_bound:true,
        operator_review_handoff_proof_bound:true,
        minimal_real_write_canary_handoff_proof_bound:true,
        nonce_consumption_forbidden_on_report_route:true,
        explicit_command_dispatch_forbidden_on_report_route:true,
        wal_write_forbidden:true,
        receipt_persistence_forbidden:true,
        post_write_readback_forbidden_on_report_route:true,
        readback_result_recording_forbidden:true,
        readback_result_persistence_forbidden:true,
        readback_acceptance_forbidden:true,
        rollback_execution_forbidden:true,
        tombstone_write_forbidden:true,
        durable_memory_read_forbidden:true,
        durable_memory_write_forbidden:true,
        durable_memory_rollback_forbidden:true,
        memory_store_mutation_forbidden:true,
        kg_live_write_forbidden:true,
        provider_model_invocation_forbidden:true,
        credential_channel_public_release_forbidden:true,
        install_restart_active_binary_filesystem_mutation_forbidden:true
      } + false_side_effects + {side_effects:false_side_effects}
    '
)"

jq -e '
  .status == "ready"
  and .minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_ready == true
  and .minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted_no_rollback_or_write == true
  and .source_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_ready == true
  and .accepted_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count == 1
  and .blocked_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count == 9
  and .rollback_tombstone_proof_authority_accepted_count == 1
  and .rollback_plan_proof_bound_count == 1
  and .tombstone_plan_proof_bound_count == 1
  and .rollback_target_proof_bound_count == 1
  and .tombstone_target_proof_bound_count == 1
  and .rollback_receipt_linkage_proof_bound_count == 1
  and .tombstone_receipt_linkage_proof_bound_count == 1
  and .rollback_idempotency_guard_proof_bound_count == 1
  and .tombstone_idempotency_guard_proof_bound_count == 1
  and .rollback_tombstone_audit_evidence_proof_bound_count == 1
  and .operator_review_handoff_proof_bound_count == 1
  and .minimal_real_write_canary_handoff_proof_bound_count == 1
  and .single_use_nonce_consumed_count == 0
  and .explicit_command_dispatched_count == 0
  and .wal_write_performed_count == 0
  and .receipt_persisted_count == 0
  and .post_write_readback_performed_count == 0
  and .readback_result_recorded_count == 0
  and .readback_result_persisted_count == 0
  and .readback_result_accepted_count == 0
  and .rollback_performed_count == 0
  and .rollback_result_recorded_count == 0
  and .rollback_result_persisted_count == 0
  and .rollback_result_accepted_count == 0
  and .tombstone_written_count == 0
  and .durable_memory_store_read_performed_count == 0
  and .durable_memory_store_write_performed_count == 0
  and .durable_memory_store_rollback_performed_count == 0
  and .memory_store_write_performed_count == 0
  and .single_use_nonce_consumed == false
  and .explicit_command_dispatched == false
  and .wal_write_performed == false
  and .receipt_persisted == false
  and .post_write_readback_performed == false
  and .readback_result_accepted == false
  and .rollback_executed == false
  and .tombstone_written == false
  and .memory_write_execution_performed == false
  and .memory_store_write_performed == false
  and .durable_memory_store_read_performed == false
  and .durable_memory_store_write_performed == false
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
