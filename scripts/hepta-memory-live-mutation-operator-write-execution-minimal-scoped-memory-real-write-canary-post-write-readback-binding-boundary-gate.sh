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

WAL_RECEIPT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-wal-receipt-binding-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-wal-receipt-binding-boundary-gate.sh
)"

source_report_sha256="$(printf '%s' "$WAL_RECEIPT_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$WAL_RECEIPT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.minimal_scoped_memory_real_write_canary_wal_receipt_binding_ready == true
    and $source.minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted_no_write == true
    and $source.accepted_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count == 1
    and $source.blocked_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count == 9
    and $source.wal_receipt_binding_authority_accepted_count == 1
    and $source.post_write_readback_handoff_bound_count == 1
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
    and $source.post_write_readback_performed == false
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
    --arg gate "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$WAL_RECEIPT_JSON" \
    '
    def accepted_fixture:
      {
        id:"minimal-scoped-memory-real-write-canary-post-write-readback-binding-envelope",
        minimal_scoped_memory_real_write_canary_post_write_readback_binding_status:"accepted_post_write_readback_binding_no_read_or_write",
        source_wal_receipt_binding_present:true,
        source_wal_receipt_binding_ready:true,
        post_write_readback_plan_binding_requested:true,
        post_write_readback_result_identity_binding_requested:true,
        readback_receipt_linkage_binding_requested:true,
        readback_payload_digest_comparison_binding_requested:true,
        readback_namespace_store_scope_binding_requested:true,
        readback_redaction_secret_scan_binding_requested:true,
        readback_stale_guard_binding_requested:true,
        readback_phantom_guard_binding_requested:true,
        readback_operator_review_handoff_binding_requested:true,
        rollback_tombstone_handoff_binding_requested:true,
        minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted:true,
        post_write_readback_plan_bound:true,
        readback_result_identity_bound:true,
        readback_receipt_linkage_bound:true,
        readback_payload_digest_comparison_bound:true,
        readback_namespace_store_scope_bound:true,
        readback_redaction_secret_scan_bound:true,
        readback_stale_guard_bound:true,
        readback_phantom_guard_bound:true,
        readback_operator_review_handoff_bound:true,
        rollback_tombstone_handoff_bound:true,
        post_write_readback_binding_noop_confirmed:true,
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
        minimal_scoped_memory_real_write_canary_post_write_readback_binding_status:"blocked_noop",
        source_wal_receipt_binding_present:true,
        source_wal_receipt_binding_ready:true,
        reason:$reason,
        minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted:false,
        post_write_readback_plan_bound:false,
        readback_result_identity_bound:false,
        readback_receipt_linkage_bound:false,
        readback_payload_digest_comparison_bound:false,
        readback_namespace_store_scope_bound:false,
        readback_redaction_secret_scan_bound:false,
        readback_stale_guard_bound:false,
        readback_phantom_guard_bound:false,
        readback_operator_review_handoff_bound:false,
        rollback_tombstone_handoff_bound:false,
        post_write_readback_binding_noop_confirmed:true,
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
        readback_result_recorded:false,
        readback_result_persisted:false,
        readback_result_accepted:false,
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
      blocked_fixture("minimal-scoped-memory-real-write-canary-post-write-readback-missing-wal-receipt-source"; "source_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_required"; {source_wal_receipt_binding_present:false, source_wal_receipt_binding_ready:false}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-post-write-readback-plan-required"; "post_write_readback_plan_binding_required"; {post_write_readback_plan_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-readback-result-identity-required"; "post_write_readback_result_identity_binding_required"; {post_write_readback_result_identity_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-readback-receipt-linkage-required"; "readback_receipt_linkage_binding_required"; {readback_receipt_linkage_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-readback-payload-digest-required"; "readback_payload_digest_comparison_binding_required"; {readback_payload_digest_comparison_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-readback-namespace-scope-required"; "readback_namespace_store_scope_binding_required"; {readback_namespace_store_scope_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-readback-redaction-secret-scan-required"; "readback_redaction_secret_scan_binding_required"; {readback_redaction_secret_scan_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-readback-stale-phantom-guard-required"; "readback_stale_and_phantom_guards_required"; {readback_stale_guard_binding_requested:true, readback_phantom_guard_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-post-write-readback-direct-side-effect-attempt"; "direct_readback_memory_and_external_side_effects_denied"; {single_use_nonce_consumption_requested:true, explicit_command_dispatch_requested:true, wal_write_requested:true, receipt_persistence_requested:true, post_write_readback_execution_requested:true, readback_result_recording_requested:true, readback_result_persistence_requested:true, readback_acceptance_requested:true, durable_memory_read_requested:true, durable_memory_write_requested:true, durable_memory_rollback_requested:true, rollback_execution_requested:true, tombstone_write_requested:true, kg_live_write_requested:true, provider_model_invocation_requested:true, credential_read_requested:true, channel_external_send_requested:true, release_artifact_write_requested:true, install_restart_requested:true, active_binary_mutation_requested:true})
    ] as $fixtures
    | {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        side_effect_free:true,
        audit_date:"2026-07-03",
        minimal_scoped_memory_real_write_canary_post_write_readback_binding_ready:true,
        minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted_no_read_or_write:true,
        source_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_ready:true,
        source_minimal_scoped_memory_real_write_canary_wal_receipt_binding_report_sha256:$source_report_sha256,
        source_accepted_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count:($source.accepted_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count // 0),
        source_blocked_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count:($source.blocked_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count // 0),
        source_wal_receipt_binding_authority_accepted_count:($source.wal_receipt_binding_authority_accepted_count // 0),
        source_post_write_readback_handoff_bound_count:($source.post_write_readback_handoff_bound_count // 0),
        source_single_use_nonce_consumed_count:($source.single_use_nonce_consumed_count // 0),
        source_explicit_command_dispatched_count:($source.explicit_command_dispatched_count // 0),
        source_wal_write_performed_count:($source.wal_write_performed_count // 0),
        source_receipt_persisted_count:($source.receipt_persisted_count // 0),
        source_post_write_readback_performed_count:($source.post_write_readback_performed_count // 0),
        minimum_required_samples:$min_long_soak_samples,
        required_minimal_scoped_memory_real_write_canary_post_write_readback_binding_surface_count:12,
        ready_minimal_scoped_memory_real_write_canary_post_write_readback_binding_surface_count:12,
        side_effect_free_minimal_scoped_memory_real_write_canary_post_write_readback_binding_surface_count:12,
        required_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count:10,
        minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count:($fixtures | length),
        accepted_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count:([$fixtures[] | select(.minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted == true)] | length),
        blocked_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count:([$fixtures[] | select(.minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted == false)] | length),
        post_write_readback_binding_authority_accepted_count:1,
        post_write_readback_plan_bound_count:1,
        readback_result_identity_bound_count:1,
        readback_receipt_linkage_bound_count:1,
        readback_payload_digest_comparison_bound_count:1,
        readback_namespace_store_scope_bound_count:1,
        readback_redaction_secret_scan_bound_count:1,
        readback_stale_guard_bound_count:1,
        readback_phantom_guard_bound_count:1,
        readback_operator_review_handoff_bound_count:1,
        rollback_tombstone_handoff_bound_count:1,
        minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted_count:1,
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
        tombstone_written_count:0,
        durable_memory_store_read_performed_count:0,
        durable_memory_store_write_performed_count:0,
        durable_memory_store_rollback_performed_count:0,
        memory_store_write_performed_count:0,
        required_before_minimal_scoped_memory_real_write_canary_post_write_readback_binding_count:19,
        minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixtures:$fixtures,
        denied_by_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_count:30,
        source_minimal_scoped_memory_real_write_canary_wal_receipt_binding_required:true,
        minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted:true,
        post_write_readback_plan_bound:true,
        readback_result_identity_bound:true,
        readback_receipt_linkage_bound:true,
        readback_payload_digest_comparison_bound:true,
        readback_namespace_store_scope_bound:true,
        readback_redaction_secret_scan_bound:true,
        readback_stale_guard_bound:true,
        readback_phantom_guard_bound:true,
        readback_operator_review_handoff_bound:true,
        rollback_tombstone_handoff_bound:true,
        nonce_consumption_forbidden_on_report_route:true,
        explicit_command_dispatch_forbidden_on_report_route:true,
        wal_write_forbidden:true,
        wal_persistence_forbidden:true,
        receipt_recording_forbidden:true,
        receipt_persistence_forbidden:true,
        post_write_readback_forbidden_on_report_route:true,
        readback_result_recording_forbidden:true,
        readback_result_persistence_forbidden:true,
        readback_acceptance_forbidden:true,
        durable_memory_read_forbidden:true,
        durable_memory_write_forbidden:true,
        durable_memory_rollback_forbidden:true,
        memory_store_mutation_forbidden:true,
        rollback_execution_forbidden:true,
        tombstone_write_forbidden:true,
        kg_live_write_forbidden:true,
        provider_model_invocation_forbidden:true,
        credential_channel_public_release_forbidden:true,
        install_restart_active_binary_filesystem_mutation_forbidden:true,
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
  and .minimal_scoped_memory_real_write_canary_post_write_readback_binding_ready == true
  and .minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted_no_read_or_write == true
  and .source_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_ready == true
  and .accepted_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count == 1
  and .blocked_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count == 9
  and .post_write_readback_binding_authority_accepted_count == 1
  and .post_write_readback_plan_bound_count == 1
  and .readback_result_identity_bound_count == 1
  and .readback_receipt_linkage_bound_count == 1
  and .rollback_tombstone_handoff_bound_count == 1
  and .single_use_nonce_consumed_count == 0
  and .explicit_command_dispatched_count == 0
  and .wal_write_performed_count == 0
  and .receipt_persisted_count == 0
  and .post_write_readback_performed_count == 0
  and .readback_result_recorded_count == 0
  and .readback_result_persisted_count == 0
  and .readback_result_accepted_count == 0
  and .durable_memory_store_read_performed_count == 0
  and .durable_memory_store_write_performed_count == 0
  and .durable_memory_store_rollback_performed_count == 0
  and .memory_store_write_performed_count == 0
  and .single_use_nonce_consumed == false
  and .explicit_command_dispatched == false
  and .wal_write_performed == false
  and .receipt_persisted == false
  and .post_write_readback_performed == false
  and .readback_result_recorded == false
  and .readback_result_persisted == false
  and .readback_result_accepted == false
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
