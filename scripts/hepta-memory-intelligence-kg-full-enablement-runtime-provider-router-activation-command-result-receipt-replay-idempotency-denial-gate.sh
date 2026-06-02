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

NO_PERSISTENCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence-gate.sh
)"

replay_idempotency_fixtures_json="$(
  jq -n '
    def replay_fixture($id; $status; $reason; $extra):
      {
        id:$id,
        replay_requested:true,
        replay_status:$status,
        source_no_persistence_present:true,
        source_no_persistence_ready:true,
        canonical_noop_result_receipt_identity_required:true,
        activation_command_result_receipt_replay_allowed:false,
        activation_command_result_receipt_replay_recorded:false,
        activation_command_result_receipt_replay_persisted:false,
        activation_command_result_receipt_replay_performed:false,
        activation_command_result_receipt_duplicate_accepted:false,
        activation_command_result_receipt_duplicate_recorded:false,
        activation_command_result_receipt_duplicate_persisted:false,
        activation_command_result_receipt_idempotency_key_accepted:false,
        activation_command_result_receipt_idempotency_key_recorded:false,
        activation_command_result_receipt_idempotency_state_recorded:false,
        activation_command_result_receipt_idempotency_state_persisted:false,
        activation_command_result_receipt_idempotency_state_materialized:false,
        activation_command_result_receipt_idempotency_filesystem_written:false,
        activation_command_result_receipt_replay_nonce_accepted:false,
        activation_command_result_receipt_replay_nonce_recorded:false,
        activation_command_result_receipt_cross_scope_reuse_accepted:false,
        activation_command_result_receipt_status_upgrade_accepted:false,
        activation_command_result_receipt_completed_status_accepted:false,
        activation_command_result_receipt_ack_replay_accepted:false,
        activation_command_result_receipt_ledger_replay_accepted:false,
        activation_command_result_receipt_index_replay_accepted:false,
        activation_command_result_receipt_delivery_replay_accepted:false,
        activation_command_result_receipt_query_replay_accepted:false,
        activation_command_result_receipt_observability_replay_accepted:false,
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
        activation_command_completion_ack_materialized:false,
        activation_command_completion_ack_delivered:false,
        operator_approval_from_replay_accepted:false,
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
        replay_ledger_written:false,
        replay_indexed:false,
        replay_query_registered:false,
        replay_observability_recorded:false,
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
      replay_fixture("provider-router-activation-command-result-receipt-replay-missing-source-no-persistence-report"; "blocked_noop"; "source_result_receipt_no_persistence_report_required"; {source_no_persistence_present:false, source_no_persistence_ready:false}),
      replay_fixture("provider-router-activation-command-result-receipt-duplicate-identity-replay-attempt"; "blocked_duplicate_noop"; "duplicate_result_receipt_identity_replay_denied"; {duplicate_result_receipt_identity_requested:true}),
      replay_fixture("provider-router-activation-command-result-receipt-replay-acceptance-attempt"; "blocked_replay_noop"; "result_receipt_replay_acceptance_denied"; {result_receipt_replay_acceptance_requested:true}),
      replay_fixture("provider-router-activation-command-result-receipt-idempotency-key-recording-attempt"; "blocked_idempotency_noop"; "idempotency_key_recording_denied"; {idempotency_key_recording_requested:true}),
      replay_fixture("provider-router-activation-command-result-receipt-idempotency-state-persistence-attempt"; "blocked_idempotency_persist_noop"; "idempotency_state_persistence_materialization_denied"; {idempotency_state_persistence_requested:true, idempotency_state_materialization_requested:true, idempotency_filesystem_write_requested:true}),
      replay_fixture("provider-router-activation-command-result-receipt-cross-scope-reuse-attempt"; "blocked_cross_scope_noop"; "cross_scope_result_receipt_reuse_denied"; {cross_scope_reuse_requested:true}),
      replay_fixture("provider-router-activation-command-result-receipt-stale-nonce-order-replay-attempt"; "blocked_nonce_order_noop"; "stale_nonce_out_of_order_receipt_replay_denied"; {stale_nonce_replay_requested:true, out_of_order_replay_requested:true}),
      replay_fixture("provider-router-activation-command-result-receipt-completion-ack-replay-attempt"; "blocked_ack_replay_noop"; "completion_ack_replay_denied"; {completion_ack_replay_requested:true}),
      replay_fixture("provider-router-activation-command-result-receipt-runtime-provider-memory-kg-replay-attempt"; "blocked_runtime_provider_memory_kg_noop"; "runtime_provider_memory_kg_replay_denied"; {runtime_replay_requested:true, provider_replay_requested:true, model_replay_requested:true, usage_replay_requested:true, memory_store_replay_requested:true, live_kg_replay_requested:true}),
      replay_fixture("provider-router-activation-command-result-receipt-external-public-install-restart-active-binary-replay-attempt"; "blocked_external_noop"; "external_public_install_restart_active_binary_replay_denied"; {external_send_replay_requested:true, public_claim_replay_requested:true, public_ga_replay_requested:true, release_artifact_replay_requested:true, install_replay_requested:true, launchd_restart_replay_requested:true, service_restart_replay_requested:true, active_binary_mutation_replay_requested:true})
    ]
  '
)"

no_persistence_report_sha256="$(sha256_text "$NO_PERSISTENCE_JSON")"
replay_idempotency_fixtures_sha256="$(sha256_text "$replay_idempotency_fixtures_json")"
replay_idempotency_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial:$no_persistence_report_sha256:$replay_idempotency_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
replay_idempotency_policy_hash_sha256="$(
  sha256_text "runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial:no-duplicate:no-replay:no-idempotency-record:no-persist:no-runtime:no-provider:no-model:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "replay=false;duplicate=false;idempotency=false;record=false;persist=false;activation=false;runtime=false;provider=false;model=false;memory=false;kg=false;secret=false;external=false;install=false;restart=false;active_binary=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$NO_PERSISTENCE_JSON" \
  --argjson fixtures "$replay_idempotency_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_gate"
    and $source.activation_command_result_receipt_no_persistence_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_v1"
    and $source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_no_persistence_status == "blocked"
    and $source.runtime_provider_router_activation_command_noop_handoff_ready == true
    and $source.runtime_provider_router_activation_command_noop_handoff_status == "blocked"
    and $source.activation_command_surface_count == 13
    and $source.activation_command_surface_ready_count == 13
    and $source.activation_command_fixture_count == 10
    and $source.blocked_activation_command_fixture_count == 10
    and $source.noop_activation_command_fixture_count == 10
    and $source.allowed_activation_command_fixture_count == 0
    and $source.accepted_activation_command_fixture_count == 0
    and $source.activation_command_result_receipt_surface_count == 14
    and $source.activation_command_result_receipt_surface_ready_count == 14
    and $source.activation_command_result_receipt_fixture_count == 10
    and $source.blocked_activation_command_result_receipt_fixture_count == 10
    and $source.noop_activation_command_result_receipt_fixture_count == 10
    and $source.allowed_activation_command_result_receipt_fixture_count == 0
    and $source.accepted_activation_command_result_receipt_fixture_count == 0
    and $source.activation_command_result_receipt_denied_count == 10
    and $source.activation_command_result_receipt_performed_count == 0
    and $source.activation_command_result_receipt_shape_registered == false
    and $source.activation_command_result_receipt_allowed == false
    and $source.activation_command_result_receipt_schema_accepted == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_result_receipt_materialized == false
    and $source.activation_command_result_receipt_filesystem_written == false
    and $source.activation_command_result_receipt_ledger_written == false
    and $source.activation_command_result_receipt_indexed == false
    and $source.activation_command_result_receipt_enqueued == false
    and $source.activation_command_result_receipt_delivered == false
    and $source.activation_command_result_receipt_exported == false
    and $source.activation_command_result_receipt_query_registered == false
    and $source.activation_command_result_receipt_observability_recorded == false
    and $source.activation_command_completion_ack_recorded == false
    and $source.activation_command_completion_ack_persisted == false
    and $source.activation_command_completion_ack_accepted == false
    and $source.activation_command_completion_ack_delivered == false
    and $source.operator_approval_from_receipt_accepted == false
    and $source.activation_from_receipt_allowed == false
    and $source.activation_command_enabled == false
    and $source.activation_command_invoked == false
    and $source.activation_command_dispatched == false
    and $source.activation_command_handoff_recorded == false
    and $source.activation_command_handoff_persisted == false
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
    and $source.readback_evidence_recorded == false
    and $source.readback_evidence_persisted == false
    and $source.router_handoff_recorded == false
    and $source.router_handoff_persisted == false
    and $source.telegram_send_performed == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.rollback_executed == false
    and $source.service_restart_performed == false
    and $source.active_binary_mutated == false
    and ($source.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial" and .status == "allowed_report_only_next_slice" and .accepts_duplicate_receipt == false and .records_idempotency == false and .persists_replay_state == false and .mutates_runtime == false and .invokes_model == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.replay_status | startswith("blocked_"))
      and .activation_command_result_receipt_replay_allowed == false
      and .activation_command_result_receipt_replay_recorded == false
      and .activation_command_result_receipt_replay_persisted == false
      and .activation_command_result_receipt_replay_performed == false
      and .activation_command_result_receipt_duplicate_accepted == false
      and .activation_command_result_receipt_idempotency_key_accepted == false
      and .activation_command_result_receipt_idempotency_state_recorded == false
      and .activation_command_result_receipt_idempotency_state_persisted == false
      and .activation_command_result_receipt_replay_nonce_accepted == false
      and .activation_command_result_receipt_cross_scope_reuse_accepted == false
      and .activation_command_result_receipt_completed_status_accepted == false
      and .activation_command_result_receipt_ack_replay_accepted == false
      and .activation_command_result_receipt_recorded == false
      and .activation_command_result_receipt_persisted == false
      and .activation_command_result_receipt_accepted == false
      and .activation_command_result_receipt_materialized == false
      and .activation_command_result_receipt_filesystem_written == false
      and .activation_command_completion_ack_recorded == false
      and .operator_approval_from_replay_accepted == false
      and .activation_from_replay_allowed == false
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
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_gate" \
  --arg no_persistence_report_sha256 "$no_persistence_report_sha256" \
  --arg replay_idempotency_fixtures_sha256 "$replay_idempotency_fixtures_sha256" \
  --arg replay_idempotency_contract_hash_sha256 "$replay_idempotency_contract_hash_sha256" \
  --arg replay_idempotency_policy_hash_sha256 "$replay_idempotency_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$NO_PERSISTENCE_JSON" \
  --argjson fixtures "$replay_idempotency_fixtures_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_replay_idempotency_schema_version:"memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_v1",
    activation_command_result_receipt_replay_idempotency_mode:"runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_no_duplicate_no_replay_no_idempotency_persist",
    source_activation_command_result_receipt_no_persistence_gate:$source.gate,
    source_activation_command_result_receipt_no_persistence_ready:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready,
    source_activation_command_result_receipt_no_persistence_status:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_status,
    source_activation_command_result_receipt_no_persistence_report_sha256:$no_persistence_report_sha256,
    source_activation_command_noop_handoff_gate:$source.source_activation_command_noop_handoff_gate,
    source_activation_command_noop_handoff_ready:$source.runtime_provider_router_activation_command_noop_handoff_ready,
    source_activation_command_noop_handoff_status:$source.runtime_provider_router_activation_command_noop_handoff_status,
    source_activation_command_noop_handoff_report_sha256:$source.source_activation_command_noop_handoff_report_sha256,
    replay_idempotency_fixtures_sha256:$replay_idempotency_fixtures_sha256,
    replay_idempotency_contract_hash_sha256:$replay_idempotency_contract_hash_sha256,
    replay_idempotency_policy_hash_sha256:$replay_idempotency_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready:true,
    runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status:"blocked",
    runtime_provider_router_activation_command_result_receipt_no_persistence_ready:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready,
    runtime_provider_router_activation_command_result_receipt_no_persistence_status:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_status,
    activation_command_result_receipt_surface_count:$source.activation_command_result_receipt_surface_count,
    activation_command_result_receipt_surface_ready_count:$source.activation_command_result_receipt_surface_ready_count,
    activation_command_result_receipt_fixture_count:$source.activation_command_result_receipt_fixture_count,
    replay_idempotency_surface_count:14,
    replay_idempotency_surface_ready_count:14,
    replay_idempotency_side_effect_free_surface_count:14,
    replay_idempotency_fixture_count:($fixtures | length),
    blocked_replay_idempotency_fixture_count:($fixtures | length),
    noop_replay_idempotency_fixture_count:($fixtures | length),
    allowed_replay_idempotency_fixture_count:0,
    accepted_replay_idempotency_fixture_count:0,
    duplicate_result_receipt_replay_fixture_count:1,
    receipt_replay_acceptance_fixture_count:1,
    idempotency_key_recording_fixture_count:1,
    idempotency_state_persistence_fixture_count:1,
    cross_scope_result_receipt_reuse_fixture_count:1,
    nonce_order_replay_fixture_count:1,
    completion_ack_replay_fixture_count:1,
    runtime_provider_memory_kg_replay_fixture_count:1,
    external_public_install_replay_fixture_count:1,
    replay_idempotency_denied_count:10,
    duplicate_result_receipt_denied_count:10,
    idempotency_state_denied_count:10,
    replay_idempotency_performed_count:0,
    duplicate_result_receipt_accepted_count:0,
    idempotency_state_recorded_count:0,
    activation_command_result_receipt_replay_allowed:false,
    activation_command_result_receipt_replay_recorded:false,
    activation_command_result_receipt_replay_persisted:false,
    activation_command_result_receipt_replay_performed:false,
    activation_command_result_receipt_duplicate_accepted:false,
    activation_command_result_receipt_duplicate_recorded:false,
    activation_command_result_receipt_duplicate_persisted:false,
    activation_command_result_receipt_idempotency_key_accepted:false,
    activation_command_result_receipt_idempotency_key_recorded:false,
    activation_command_result_receipt_idempotency_state_recorded:false,
    activation_command_result_receipt_idempotency_state_persisted:false,
    activation_command_result_receipt_idempotency_state_materialized:false,
    activation_command_result_receipt_idempotency_filesystem_written:false,
    activation_command_result_receipt_replay_nonce_accepted:false,
    activation_command_result_receipt_replay_nonce_recorded:false,
    activation_command_result_receipt_cross_scope_reuse_accepted:false,
    activation_command_result_receipt_status_upgrade_accepted:false,
    activation_command_result_receipt_completed_status_accepted:false,
    activation_command_result_receipt_ack_replay_accepted:false,
    activation_command_result_receipt_ledger_replay_accepted:false,
    activation_command_result_receipt_index_replay_accepted:false,
    activation_command_result_receipt_delivery_replay_accepted:false,
    activation_command_result_receipt_query_replay_accepted:false,
    activation_command_result_receipt_observability_replay_accepted:false,
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
    activation_command_completion_ack_materialized:false,
    activation_command_completion_ack_delivered:false,
    operator_approval_from_replay_accepted:false,
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
    replay_ledger_written:false,
    replay_indexed:false,
    replay_query_registered:false,
    replay_observability_recorded:false,
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
    replay_idempotency_surfaces:[
      "source_result_receipt_no_persistence_report_required",
      "canonical_noop_result_receipt_identity_required",
      "duplicate_receipt_rejection_required",
      "replay_request_rejection_required",
      "idempotency_key_state_recording_denied",
      "idempotency_persistence_materialization_denied",
      "cross_scope_receipt_reuse_denied",
      "nonce_order_freshness_replay_denied",
      "completion_ack_replay_denied",
      "activation_from_replay_denied",
      "runtime_router_live_context_replay_denied",
      "adapter_provider_model_replay_denied",
      "usage_memory_kg_replay_denied",
      "external_public_install_restart_active_binary_replay_denied"
    ],
    replay_idempotency_fixtures:$fixtures,
    denied_by_replay_idempotency:[
      "source_result_receipt_no_persistence_report_required",
      "canonical_noop_result_receipt_identity_required",
      "duplicate_result_receipt_identity_replay_denied",
      "result_receipt_replay_acceptance_denied",
      "idempotency_key_recording_denied",
      "idempotency_state_recording_denied",
      "idempotency_state_persistence_denied",
      "idempotency_state_materialization_denied",
      "idempotency_filesystem_write_denied",
      "cross_scope_result_receipt_reuse_denied",
      "stale_nonce_replay_denied",
      "out_of_order_receipt_replay_denied",
      "completion_ack_replay_denied",
      "activation_from_replay_denied",
      "runtime_router_replay_denied",
      "live_context_replay_denied",
      "context_injection_replay_denied",
      "adapter_invocation_replay_denied",
      "provider_model_replay_denied",
      "usage_record_replay_denied",
      "memory_store_replay_denied",
      "live_kg_replay_denied",
      "secret_material_replay_denied",
      "external_send_replay_denied",
      "public_claim_replay_denied",
      "install_restart_active_binary_replay_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial",
        status:"allowed_report_only",
        accepts_duplicate_receipt:false,
        records_idempotency:false,
        persists_replay_state:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"stage_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial",
        status:"allowed_report_only_next_slice",
        accepts_out_of_order_receipt:false,
        records_monotonic_clock:false,
        persists_ordering_state:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"run_full_light_preflight",
        status:"allowed_verification_only",
        accepts_duplicate_receipt:false,
        persists_replay_state:false,
        mutates_runtime:false,
        invokes_model:false,
        writes_kg:false
      }
    ],
    source_result_receipt_no_persistence_report_required:true,
    duplicate_result_receipt_acceptance_forbidden:true,
    result_receipt_replay_acceptance_forbidden:true,
    idempotency_key_recording_forbidden:true,
    idempotency_state_persistence_forbidden:true,
    cross_scope_receipt_reuse_forbidden:true,
    completion_ack_replay_forbidden:true,
    activation_from_replay_forbidden:true,
    runtime_provider_memory_kg_replay_forbidden:true,
    secret_read_forbidden:true,
    external_public_install_restart_active_binary_replay_forbidden:true,
    side_effects:{
      activation_command_result_receipt_replay_recorded:false,
      activation_command_result_receipt_replay_persisted:false,
      activation_command_result_receipt_replay_performed:false,
      activation_command_result_receipt_duplicate_accepted:false,
      activation_command_result_receipt_duplicate_recorded:false,
      activation_command_result_receipt_duplicate_persisted:false,
      activation_command_result_receipt_idempotency_key_recorded:false,
      activation_command_result_receipt_idempotency_state_recorded:false,
      activation_command_result_receipt_idempotency_state_persisted:false,
      activation_command_result_receipt_idempotency_state_materialized:false,
      activation_command_result_receipt_idempotency_filesystem_written:false,
      activation_command_result_receipt_replay_nonce_recorded:false,
      activation_command_result_receipt_cross_scope_reuse_accepted:false,
      activation_command_result_receipt_status_upgrade_accepted:false,
      activation_command_result_receipt_completed_status_accepted:false,
      activation_command_result_receipt_ack_replay_accepted:false,
      activation_command_result_receipt_ledger_replay_accepted:false,
      activation_command_result_receipt_index_replay_accepted:false,
      activation_command_result_receipt_delivery_replay_accepted:false,
      activation_command_result_receipt_query_replay_accepted:false,
      activation_command_result_receipt_observability_replay_accepted:false,
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
      operator_approval_from_replay_accepted:false,
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
      replay_ledger_written:false,
      replay_indexed:false,
      replay_query_registered:false,
      replay_observability_recorded:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_gate"
  and .activation_command_result_receipt_replay_idempotency_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_v1"
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
  and .replay_idempotency_side_effect_free_surface_count == 14
  and .replay_idempotency_fixture_count == 10
  and .blocked_replay_idempotency_fixture_count == 10
  and .noop_replay_idempotency_fixture_count == 10
  and .allowed_replay_idempotency_fixture_count == 0
  and .accepted_replay_idempotency_fixture_count == 0
  and .duplicate_result_receipt_replay_fixture_count == 1
  and .receipt_replay_acceptance_fixture_count == 1
  and .idempotency_key_recording_fixture_count == 1
  and .idempotency_state_persistence_fixture_count == 1
  and .cross_scope_result_receipt_reuse_fixture_count == 1
  and .nonce_order_replay_fixture_count == 1
  and .completion_ack_replay_fixture_count == 1
  and .runtime_provider_memory_kg_replay_fixture_count == 1
  and .external_public_install_replay_fixture_count == 1
  and .replay_idempotency_denied_count == 10
  and .duplicate_result_receipt_denied_count == 10
  and .idempotency_state_denied_count == 10
  and .replay_idempotency_performed_count == 0
  and .duplicate_result_receipt_accepted_count == 0
  and .idempotency_state_recorded_count == 0
  and .activation_command_result_receipt_replay_allowed == false
  and .activation_command_result_receipt_replay_recorded == false
  and .activation_command_result_receipt_replay_persisted == false
  and .activation_command_result_receipt_replay_performed == false
  and .activation_command_result_receipt_duplicate_accepted == false
  and .activation_command_result_receipt_duplicate_recorded == false
  and .activation_command_result_receipt_duplicate_persisted == false
  and .activation_command_result_receipt_idempotency_key_accepted == false
  and .activation_command_result_receipt_idempotency_key_recorded == false
  and .activation_command_result_receipt_idempotency_state_recorded == false
  and .activation_command_result_receipt_idempotency_state_persisted == false
  and .activation_command_result_receipt_idempotency_state_materialized == false
  and .activation_command_result_receipt_idempotency_filesystem_written == false
  and .activation_command_result_receipt_replay_nonce_accepted == false
  and .activation_command_result_receipt_replay_nonce_recorded == false
  and .activation_command_result_receipt_cross_scope_reuse_accepted == false
  and .activation_command_result_receipt_status_upgrade_accepted == false
  and .activation_command_result_receipt_completed_status_accepted == false
  and .activation_command_result_receipt_ack_replay_accepted == false
  and .activation_command_result_receipt_ledger_replay_accepted == false
  and .activation_command_result_receipt_index_replay_accepted == false
  and .activation_command_result_receipt_delivery_replay_accepted == false
  and .activation_command_result_receipt_query_replay_accepted == false
  and .activation_command_result_receipt_observability_replay_accepted == false
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
  and .operator_approval_from_replay_accepted == false
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
  and .replay_ledger_written == false
  and .replay_indexed == false
  and .replay_query_registered == false
  and .replay_observability_recorded == false
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
  and (.replay_idempotency_surfaces | length) == 14
  and (.replay_idempotency_fixtures | length) == 10
  and (.replay_idempotency_fixtures | all(
    (.replay_status | startswith("blocked_"))
    and .activation_command_result_receipt_replay_allowed == false
    and .activation_command_result_receipt_replay_recorded == false
    and .activation_command_result_receipt_replay_persisted == false
    and .activation_command_result_receipt_replay_performed == false
    and .activation_command_result_receipt_duplicate_accepted == false
    and .activation_command_result_receipt_idempotency_key_accepted == false
    and .activation_command_result_receipt_idempotency_state_recorded == false
    and .activation_command_result_receipt_idempotency_state_persisted == false
    and .activation_command_result_receipt_replay_nonce_accepted == false
    and .activation_command_result_receipt_cross_scope_reuse_accepted == false
    and .activation_command_result_receipt_completed_status_accepted == false
    and .activation_command_result_receipt_ack_replay_accepted == false
    and .activation_command_result_receipt_recorded == false
    and .activation_command_result_receipt_persisted == false
    and .activation_command_result_receipt_accepted == false
    and .activation_command_result_receipt_materialized == false
    and .activation_command_result_receipt_filesystem_written == false
    and .activation_command_completion_ack_recorded == false
    and .operator_approval_from_replay_accepted == false
    and .activation_from_replay_allowed == false
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
  and ([.replay_idempotency_fixtures[] | select(.duplicate_result_receipt_identity_requested == true)] | length) == 1
  and ([.replay_idempotency_fixtures[] | select(.result_receipt_replay_acceptance_requested == true)] | length) == 1
  and ([.replay_idempotency_fixtures[] | select(.idempotency_key_recording_requested == true)] | length) == 1
  and ([.replay_idempotency_fixtures[] | select(.idempotency_state_persistence_requested == true and .idempotency_state_materialization_requested == true)] | length) == 1
  and ([.replay_idempotency_fixtures[] | select(.cross_scope_reuse_requested == true)] | length) == 1
  and ([.replay_idempotency_fixtures[] | select(.stale_nonce_replay_requested == true and .out_of_order_replay_requested == true)] | length) == 1
  and ([.replay_idempotency_fixtures[] | select(.completion_ack_replay_requested == true)] | length) == 1
  and ([.replay_idempotency_fixtures[] | select(.runtime_replay_requested == true and .provider_replay_requested == true and .memory_store_replay_requested == true and .live_kg_replay_requested == true)] | length) == 1
  and ([.replay_idempotency_fixtures[] | select(.external_send_replay_requested == true and .install_replay_requested == true and .active_binary_mutation_replay_requested == true)] | length) == 1
  and (.denied_by_replay_idempotency | length) == 26
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial" and .status == "allowed_report_only_next_slice" and .accepts_out_of_order_receipt == false and .records_monotonic_clock == false and .persists_ordering_state == false and .mutates_runtime == false and .invokes_model == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement runtime provider-router activation command result receipt replay/idempotency denial gate passed"
