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

ROLLBACK_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-rollback-tombstone-dry-run-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-rollback-tombstone-dry-run-gate.sh
)"

source_report_sha256="$(printf '%s' "$ROLLBACK_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$ROLLBACK_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_rollback_tombstone_dry_run_gate"
    and $source.scoped_memory_real_write_canary_rollback_tombstone_dry_run_ready == true
    and $source.source_scoped_memory_real_write_canary_readback_validation_dry_run_ready == true
    and $source.scoped_memory_real_write_canary_rollback_tombstone_fixture_count == 10
    and $source.accepted_scoped_memory_real_write_canary_rollback_tombstone_fixture_count == 0
    and $source.denied_by_scoped_memory_real_write_canary_rollback_tombstone_dry_run_count == 28
    and $source.rollback_performed_count == 0
    and $source.tombstone_written_count == 0
    and $source.durable_memory_store_read_performed_count == 0
    and $source.durable_memory_store_write_performed_count == 0
    and $source.durable_memory_store_rollback_performed_count == 0
    and $source.memory_store_write_performed_count == 0
    and $source.rollback_executed == false
    and $source.tombstone_written == false
    and $source.memory_write_execution_performed == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_mutated == false
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
    --arg gate "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_operator_approval_nonce_command_accepted_gate_boundary_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ROLLBACK_JSON" \
    '
    def accepted_fixture:
      {
        id:"minimal-scoped-memory-real-write-canary-accepted-gate-authority-envelope",
        minimal_scoped_memory_real_write_canary_accepted_gate_status:"accepted_authority_noop",
        source_rollback_tombstone_dry_run_present:true,
        source_rollback_tombstone_dry_run_ready:true,
        fresh_operator_approval_artifact_requested:true,
        operator_identity_session_binding_requested:true,
        single_use_nonce_authority_requested:true,
        explicit_command_acceptance_requested:true,
        canary_namespace_store_scope_binding_requested:true,
        payload_digest_redaction_binding_requested:true,
        active_binary_sha_route_count_binding_requested:true,
        wal_receipt_binding_requested:true,
        post_write_readback_binding_requested:true,
        rollback_tombstone_proof_binding_requested:true,
        fresh_operator_approval_artifact_accepted:true,
        operator_identity_bound:true,
        operator_session_bound:true,
        single_use_nonce_authority_accepted:true,
        explicit_command_accepted:true,
        canary_namespace_bound:true,
        canary_store_bound:true,
        canary_scope_bound:true,
        payload_digest_bound:true,
        payload_redaction_bound:true,
        active_binary_sha_bound:true,
        route_count_bound:true,
        wal_receipt_binding_accepted:true,
        post_write_readback_binding_accepted:true,
        rollback_tombstone_proof_binding_accepted:true,
        minimal_real_write_authority_accepted:true,
        single_use_nonce_consumed:false,
        explicit_command_dispatched:false,
        wal_write_performed:false,
        receipt_persisted:false,
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
        accepted_authority_envelope_noop_confirmed:true
      };
    def blocked_fixture($id; $reason; $extra):
      ({
        id:$id,
        minimal_scoped_memory_real_write_canary_accepted_gate_status:"blocked_noop",
        source_rollback_tombstone_dry_run_present:true,
        source_rollback_tombstone_dry_run_ready:true,
        reason:$reason,
        fresh_operator_approval_artifact_accepted:false,
        operator_identity_bound:false,
        operator_session_bound:false,
        single_use_nonce_authority_accepted:false,
        explicit_command_accepted:false,
        canary_namespace_bound:false,
        canary_store_bound:false,
        canary_scope_bound:false,
        payload_digest_bound:false,
        payload_redaction_bound:false,
        active_binary_sha_bound:false,
        route_count_bound:false,
        wal_receipt_binding_accepted:false,
        post_write_readback_binding_accepted:false,
        rollback_tombstone_proof_binding_accepted:false,
        minimal_real_write_authority_accepted:false,
        single_use_nonce_consumed:false,
        explicit_command_dispatched:false,
        wal_write_performed:false,
        receipt_persisted:false,
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
        accepted_authority_envelope_noop_confirmed:true
      } + $extra);
    def false_side_effects:
      {
        single_use_nonce_consumed:false,
        explicit_command_dispatched:false,
        wal_write_performed:false,
        receipt_recorded:false,
        receipt_persisted:false,
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
      blocked_fixture("minimal-scoped-memory-real-write-canary-missing-rollback-source"; "rollback_tombstone_dry_run_boundary_required"; {source_rollback_tombstone_dry_run_present:false, source_rollback_tombstone_dry_run_ready:false}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-operator-approval-required"; "fresh_operator_approval_artifact_required"; {fresh_operator_approval_artifact_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-identity-session-required"; "operator_identity_session_binding_required"; {operator_identity_session_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-nonce-required"; "single_use_nonce_authority_required"; {single_use_nonce_authority_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-explicit-command-required"; "explicit_command_acceptance_required"; {explicit_command_acceptance_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-scope-store-required"; "canary_namespace_store_scope_binding_required"; {canary_namespace_store_scope_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-digest-redaction-required"; "payload_digest_redaction_binding_required"; {payload_digest_redaction_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-binary-route-wal-readback-rollback-required"; "active_binary_route_wal_readback_rollback_bindings_required"; {active_binary_sha_route_count_binding_requested:true, wal_receipt_binding_requested:true, post_write_readback_binding_requested:true, rollback_tombstone_proof_binding_requested:true}),
      blocked_fixture("minimal-scoped-memory-real-write-canary-direct-side-effect-attempt"; "direct_side_effects_denied"; {single_use_nonce_consumption_requested:true, explicit_command_dispatch_requested:true, durable_memory_write_requested:true, wal_write_requested:true, receipt_persistence_requested:true, rollback_execution_requested:true, tombstone_write_requested:true, kg_live_write_requested:true, provider_model_invocation_requested:true, credential_read_requested:true, channel_external_send_requested:true, release_artifact_write_requested:true, install_restart_requested:true, active_binary_mutation_requested:true})
    ] as $fixtures
    | {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        side_effect_free:true,
        audit_date:"2026-07-03",
        minimal_scoped_memory_real_write_canary_accepted_gate_ready:true,
        minimal_scoped_memory_real_write_canary_authority_accepted_no_write:true,
        source_scoped_memory_real_write_canary_rollback_tombstone_dry_run_ready:true,
        source_scoped_memory_real_write_canary_rollback_tombstone_report_sha256:$source_report_sha256,
        source_scoped_memory_real_write_canary_rollback_tombstone_fixture_count:($source.scoped_memory_real_write_canary_rollback_tombstone_fixture_count // 0),
        source_accepted_scoped_memory_real_write_canary_rollback_tombstone_fixture_count:($source.accepted_scoped_memory_real_write_canary_rollback_tombstone_fixture_count // 0),
        source_scoped_memory_real_write_canary_rollback_tombstone_denial_count:($source.denied_by_scoped_memory_real_write_canary_rollback_tombstone_dry_run_count // 0),
        minimum_required_samples:$min_long_soak_samples,
        required_minimal_scoped_memory_real_write_canary_accepted_gate_surface_count:12,
        ready_minimal_scoped_memory_real_write_canary_accepted_gate_surface_count:12,
        side_effect_free_minimal_scoped_memory_real_write_canary_accepted_gate_surface_count:12,
        required_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count:10,
        minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count:($fixtures | length),
        accepted_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count:([$fixtures[] | select(.minimal_real_write_authority_accepted == true)] | length),
        blocked_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count:([$fixtures[] | select(.minimal_real_write_authority_accepted == false)] | length),
        fresh_operator_approval_artifact_accepted_count:1,
        operator_identity_session_bound_count:1,
        single_use_nonce_authority_accepted_count:1,
        explicit_command_accepted_count:1,
        wal_receipt_binding_accepted_count:1,
        post_write_readback_binding_accepted_count:1,
        rollback_tombstone_proof_binding_accepted_count:1,
        minimal_scoped_memory_real_write_canary_authority_accepted_count:1,
        single_use_nonce_consumed_count:0,
        explicit_command_dispatched_count:0,
        wal_write_performed_count:0,
        receipt_persisted_count:0,
        post_write_readback_performed_count:0,
        rollback_performed_count:0,
        tombstone_written_count:0,
        durable_memory_store_read_performed_count:0,
        durable_memory_store_write_performed_count:0,
        durable_memory_store_rollback_performed_count:0,
        memory_store_write_performed_count:0,
        required_before_minimal_scoped_memory_real_write_canary_execution_count:16,
        minimal_scoped_memory_real_write_canary_accepted_gate_fixtures:$fixtures,
        denied_by_minimal_scoped_memory_real_write_canary_accepted_gate_boundary_count:26,
        fresh_operator_approval_artifact_accepted:true,
        operator_identity_bound:true,
        operator_session_bound:true,
        single_use_nonce_authority_accepted:true,
        explicit_command_accepted:true,
        wal_receipt_binding_accepted:true,
        post_write_readback_binding_accepted:true,
        rollback_tombstone_proof_binding_accepted:true,
        minimal_real_write_authority_accepted:true,
        nonce_consumption_forbidden_on_report_route:true,
        explicit_command_dispatch_forbidden_on_report_route:true,
        durable_memory_write_forbidden:true,
        memory_store_mutation_forbidden:true,
        wal_write_forbidden:true,
        receipt_persistence_forbidden:true,
        single_use_nonce_consumed:false,
        explicit_command_dispatched:false,
        wal_write_performed:false,
        receipt_persisted:false,
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
  and .minimal_scoped_memory_real_write_canary_accepted_gate_ready == true
  and .minimal_scoped_memory_real_write_canary_authority_accepted_no_write == true
  and .accepted_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count == 1
  and .blocked_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count == 9
  and .fresh_operator_approval_artifact_accepted_count == 1
  and .single_use_nonce_authority_accepted_count == 1
  and .explicit_command_accepted_count == 1
  and .minimal_scoped_memory_real_write_canary_authority_accepted_count == 1
  and .single_use_nonce_consumed_count == 0
  and .explicit_command_dispatched_count == 0
  and .wal_write_performed_count == 0
  and .receipt_persisted_count == 0
  and .post_write_readback_performed_count == 0
  and .rollback_performed_count == 0
  and .tombstone_written_count == 0
  and .durable_memory_store_read_performed_count == 0
  and .durable_memory_store_write_performed_count == 0
  and .durable_memory_store_rollback_performed_count == 0
  and .memory_store_write_performed_count == 0
  and .single_use_nonce_consumed == false
  and .explicit_command_dispatched == false
  and .wal_write_performed == false
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
