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

CANCELLATION_SUPERSESSION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial-gate.sh
)"

audit_trail_immutable_evidence_fixtures_json="$(
  jq -n '
    def audit_fixture($id; $status; $reason; $extra):
      {
        id:$id,
        audit_trail_requested:true,
        immutable_evidence_requested:false,
        audit_evidence_status:$status,
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
        index_evidence_persisted:false,
        delivery_evidence_recorded:false,
        delivery_evidence_persisted:false,
        audit_trail_exported:false,
        immutable_evidence_exported:false,
        audit_evidence_query_registered:false,
        audit_evidence_observability_recorded:false,
        activation_command_result_receipt_cancellation_allowed:false,
        activation_command_result_receipt_cancellation_recorded:false,
        activation_command_result_receipt_cancellation_persisted:false,
        activation_command_result_receipt_supersession_allowed:false,
        activation_command_result_receipt_supersession_recorded:false,
        activation_command_result_receipt_supersession_persisted:false,
        replacement_receipt_accepted:false,
        replacement_receipt_recorded:false,
        replacement_receipt_persisted:false,
        tombstone_recorded:false,
        delete_marker_recorded:false,
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
        activation_command_result_receipt_exported:false,
        activation_command_result_receipt_query_registered:false,
        activation_command_result_receipt_observability_recorded:false,
        activation_command_completion_ack_recorded:false,
        activation_command_completion_ack_persisted:false,
        activation_command_completion_ack_accepted:false,
        activation_command_completion_ack_delivered:false,
        activation_from_audit_trail_allowed:false,
        activation_from_immutable_evidence_allowed:false,
        activation_from_cancellation_allowed:false,
        activation_from_supersession_allowed:false,
        activation_from_ordering_allowed:false,
        activation_from_replay_allowed:false,
        activation_from_receipt_allowed:false,
        activation_command_enabled:false,
        activation_command_invoked:false,
        activation_command_dispatched:false,
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
        provider_prompt_replayed:false,
        auth_secret_read:false,
        credential_read:false,
        secret_file_read:false,
        usage_recorded:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
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
        receipt_noop_confirmed:true,
        reason:$reason
      } + $extra;
    [
      audit_fixture("provider-router-activation-command-result-receipt-audit-missing-source-cancellation-supersession-report"; "blocked_noop"; "source_cancellation_supersession_report_required"; {source_cancellation_supersession_present:false, source_cancellation_supersession_ready:false}),
      audit_fixture("provider-router-activation-command-result-receipt-audit-trail-append-request"; "blocked_noop"; "audit_trail_append_request_denied"; {audit_trail_request_shape:"append_blocked_noop_result_receipt_audit_trail"}),
      audit_fixture("provider-router-activation-command-result-receipt-immutable-evidence-packet"; "blocked_evidence_noop"; "immutable_evidence_packet_request_denied"; {immutable_evidence_requested:true, audit_trail_requested:false, immutable_evidence_request_shape:"seal_blocked_noop_result_receipt_as_immutable_evidence"}),
      audit_fixture("provider-router-activation-command-result-receipt-hash-chain-merkle-root"; "blocked_evidence_noop"; "hash_chain_merkle_root_evidence_denied"; {immutable_evidence_requested:true, audit_trail_requested:false, hash_chain_requested:true, merkle_root_requested:true}),
      audit_fixture("provider-router-activation-command-result-receipt-attestation-witness-notary"; "blocked_evidence_noop"; "attestation_witness_notary_evidence_denied"; {immutable_evidence_requested:true, audit_trail_requested:false, attestation_requested:true, witness_requested:true, notary_requested:true}),
      audit_fixture("provider-router-activation-command-result-receipt-audit-trail-materialization"; "blocked_noop"; "audit_trail_materialization_filesystem_denied"; {audit_trail_materialization_requested:true, audit_trail_filesystem_write_requested:true}),
      audit_fixture("provider-router-activation-command-result-receipt-ledger-index-delivery-evidence"; "blocked_noop"; "ledger_index_delivery_evidence_denied"; {ledger_evidence_requested:true, index_evidence_requested:true, delivery_evidence_requested:true}),
      audit_fixture("provider-router-activation-command-result-receipt-activation-from-audit-evidence"; "blocked_evidence_noop"; "activation_from_audit_evidence_denied"; {immutable_evidence_requested:true, audit_trail_requested:false, activation_from_audit_evidence_requested:true}),
      audit_fixture("provider-router-activation-command-result-receipt-memory-kg-rollback-secret-provider-evidence"; "blocked_evidence_noop"; "memory_kg_rollback_secret_provider_evidence_denied"; {immutable_evidence_requested:true, audit_trail_requested:false, memory_store_evidence_requested:true, live_kg_evidence_requested:true, rollback_evidence_requested:true, secret_material_evidence_requested:true, provider_prompt_evidence_requested:true}),
      audit_fixture("provider-router-activation-command-result-receipt-external-public-install-evidence"; "blocked_evidence_noop"; "external_public_install_restart_active_binary_evidence_denied"; {immutable_evidence_requested:true, audit_trail_requested:false, external_send_evidence_requested:true, public_claim_evidence_requested:true, release_artifact_evidence_requested:true, install_evidence_requested:true, service_restart_evidence_requested:true, active_binary_mutation_evidence_requested:true})
    ]
  '
)"

cancellation_supersession_report_sha256="$(sha256_text "$CANCELLATION_SUPERSESSION_JSON")"
audit_trail_immutable_evidence_fixtures_sha256="$(sha256_text "$audit_trail_immutable_evidence_fixtures_json")"
audit_trail_immutable_evidence_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial:$cancellation_supersession_report_sha256:$audit_trail_immutable_evidence_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
audit_trail_immutable_evidence_policy_hash_sha256="$(
  sha256_text "runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial:no-audit-write:no-evidence-persist:no-hash-chain:no-attestation:no-runtime:no-provider:no-model:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "audit=false;evidence=false;hash_chain=false;attestation=false;record=false;persist=false;activation=false;runtime=false;provider=false;model=false;memory=false;kg=false;secret=false;external=false;install=false;restart=false;active_binary=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$CANCELLATION_SUPERSESSION_JSON" \
  --argjson fixtures "$audit_trail_immutable_evidence_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_gate"
    and $source.activation_command_result_receipt_cancellation_supersession_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_v1"
    and $source.runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_status == "blocked"
    and $source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
    and $source.cancellation_supersession_surface_count == 14
    and $source.cancellation_supersession_surface_ready_count == 14
    and $source.cancellation_supersession_fixture_count == 10
    and $source.blocked_cancellation_supersession_fixture_count == 10
    and $source.allowed_cancellation_supersession_fixture_count == 0
    and $source.accepted_cancellation_supersession_fixture_count == 0
    and $source.activation_command_result_receipt_cancellation_allowed == false
    and $source.activation_command_result_receipt_cancellation_recorded == false
    and $source.activation_command_result_receipt_cancellation_persisted == false
    and $source.activation_command_result_receipt_supersession_allowed == false
    and $source.activation_command_result_receipt_supersession_recorded == false
    and $source.activation_command_result_receipt_supersession_persisted == false
    and $source.activation_command_result_receipt_replacement_receipt_accepted == false
    and $source.activation_command_result_receipt_replacement_hash_accepted == false
    and $source.activation_command_result_receipt_tombstone_recorded == false
    and $source.activation_command_result_receipt_delete_marker_recorded == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_completion_ack_recorded == false
    and $source.activation_command_completion_ack_accepted == false
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
    and $source.rollback_executed == false
    and $source.external_send_performed == false
    and $source.install_executed == false
    and $source.service_restart_performed == false
    and $source.active_binary_mutated == false
    and ($source.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial" and .status == "allowed_report_only_next_slice" and .writes_audit_trail == false and .persists_evidence == false and .mutates_runtime == false and .invokes_model == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.audit_evidence_status | startswith("blocked"))
      and .audit_trail_allowed == false
      and .audit_trail_recorded == false
      and .audit_trail_persisted == false
      and .immutable_evidence_allowed == false
      and .immutable_evidence_recorded == false
      and .immutable_evidence_persisted == false
      and .hash_chain_recorded == false
      and .merkle_root_recorded == false
      and .attestation_recorded == false
      and .witness_recorded == false
      and .notary_recorded == false
      and .activation_command_result_receipt_recorded == false
      and .activation_command_result_receipt_persisted == false
      and .activation_command_result_receipt_accepted == false
      and .activation_activated == false
      and .runtime_router_mutated == false
      and .provider_invoked == false
      and .model_invoked == false
      and .auth_secret_read == false
      and .credential_read == false
      and .secret_file_read == false
      and .memory_store_write_performed == false
      and .memory_store_mutated == false
      and .live_kg_write_performed == false
      and .rollback_executed == false
      and .external_send_performed == false
      and .install_executed == false
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
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate" \
  --arg cancellation_supersession_report_sha256 "$cancellation_supersession_report_sha256" \
  --arg audit_trail_immutable_evidence_fixtures_sha256 "$audit_trail_immutable_evidence_fixtures_sha256" \
  --arg audit_trail_immutable_evidence_contract_hash_sha256 "$audit_trail_immutable_evidence_contract_hash_sha256" \
  --arg audit_trail_immutable_evidence_policy_hash_sha256 "$audit_trail_immutable_evidence_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$CANCELLATION_SUPERSESSION_JSON" \
  --argjson fixtures "$audit_trail_immutable_evidence_fixtures_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_audit_trail_immutable_evidence_schema_version:"memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_v1",
    activation_command_result_receipt_audit_trail_immutable_evidence_mode:"runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_no_audit_write_no_evidence_persist",
    source_activation_command_result_receipt_cancellation_supersession_gate:$source.gate,
    source_activation_command_result_receipt_cancellation_supersession_ready:$source.runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready,
    source_activation_command_result_receipt_cancellation_supersession_status:$source.runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_status,
    source_activation_command_result_receipt_cancellation_supersession_report_sha256:$cancellation_supersession_report_sha256,
    source_activation_command_result_receipt_ordering_monotonicity_ready:$source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready,
    source_activation_command_result_receipt_ordering_monotonicity_report_sha256:$source.source_activation_command_result_receipt_ordering_monotonicity_report_sha256,
    source_activation_command_result_receipt_replay_idempotency_ready:$source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready,
    source_activation_command_result_receipt_replay_idempotency_report_sha256:$source.source_activation_command_result_receipt_replay_idempotency_report_sha256,
    source_activation_command_result_receipt_no_persistence_ready:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready,
    source_activation_command_result_receipt_no_persistence_report_sha256:$source.source_activation_command_result_receipt_no_persistence_report_sha256,
    audit_trail_immutable_evidence_fixtures_sha256:$audit_trail_immutable_evidence_fixtures_sha256,
    audit_trail_immutable_evidence_contract_hash_sha256:$audit_trail_immutable_evidence_contract_hash_sha256,
    audit_trail_immutable_evidence_policy_hash_sha256:$audit_trail_immutable_evidence_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready:true,
    runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status:"blocked",
    runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready,
    runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready,
    runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready,
    runtime_provider_router_activation_command_result_receipt_no_persistence_ready:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready,
    activation_command_result_receipt_surface_count:$source.activation_command_result_receipt_surface_count,
    activation_command_result_receipt_surface_ready_count:$source.activation_command_result_receipt_surface_ready_count,
    cancellation_supersession_surface_count:$source.cancellation_supersession_surface_count,
    cancellation_supersession_surface_ready_count:$source.cancellation_supersession_surface_ready_count,
    audit_trail_immutable_evidence_surface_count:12,
    audit_trail_immutable_evidence_surface_ready_count:12,
    audit_trail_immutable_evidence_side_effect_free_surface_count:12,
    audit_trail_immutable_evidence_fixture_count:($fixtures | length),
    blocked_audit_trail_immutable_evidence_fixture_count:($fixtures | length),
    noop_audit_trail_immutable_evidence_fixture_count:($fixtures | length),
    allowed_audit_trail_immutable_evidence_fixture_count:0,
    accepted_audit_trail_immutable_evidence_fixture_count:0,
    audit_trail_denied_count:($fixtures | length),
    immutable_evidence_denied_count:($fixtures | map(select(.immutable_evidence_requested == true)) | length),
    audit_trail_performed_count:0,
    immutable_evidence_performed_count:0,
    hash_chain_recorded_count:0,
    merkle_root_recorded_count:0,
    attestation_recorded_count:0,
    witness_recorded_count:0,
    notary_recorded_count:0,
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
    activation_command_result_receipt_index_evidence_persisted:false,
    activation_command_result_receipt_delivery_evidence_recorded:false,
    activation_command_result_receipt_delivery_evidence_persisted:false,
    activation_command_result_receipt_cancellation_allowed:false,
    activation_command_result_receipt_cancellation_recorded:false,
    activation_command_result_receipt_cancellation_persisted:false,
    activation_command_result_receipt_supersession_allowed:false,
    activation_command_result_receipt_supersession_recorded:false,
    activation_command_result_receipt_supersession_persisted:false,
    activation_command_result_receipt_replacement_receipt_accepted:false,
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
    activation_command_result_receipt_exported:false,
    activation_command_result_receipt_query_registered:false,
    activation_command_result_receipt_observability_recorded:false,
    activation_command_completion_ack_recorded:false,
    activation_command_completion_ack_persisted:false,
    activation_command_completion_ack_accepted:false,
    activation_command_completion_ack_delivered:false,
    activation_from_audit_trail_allowed:false,
    activation_from_immutable_evidence_allowed:false,
    activation_from_cancellation_allowed:false,
    activation_from_supersession_allowed:false,
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
    provider_prompt_replayed:false,
    auth_secret_read:false,
    credential_read:false,
    secret_file_read:false,
    usage_recorded:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    live_kg_write_performed:false,
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
    audit_trail_immutable_evidence_surfaces:[
      "source_cancellation_supersession_report_required",
      "audit_trail_request_shape_denied",
      "immutable_evidence_request_shape_denied",
      "append_only_audit_log_recording_denied",
      "evidence_hash_chain_recording_denied",
      "attestation_witness_notary_recording_denied",
      "audit_trail_materialization_denied",
      "immutable_evidence_persistence_denied",
      "ledger_index_delivery_evidence_denied",
      "activation_from_audit_evidence_denied",
      "memory_kg_rollback_secret_provider_evidence_denied",
      "external_public_install_restart_active_binary_evidence_denied"
    ],
    audit_trail_immutable_evidence_fixtures:$fixtures,
    denied_by_audit_trail_immutable_evidence:[
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
      "memory_store_evidence_denied",
      "live_kg_evidence_denied",
      "rollback_evidence_denied",
      "secret_material_evidence_denied",
      "provider_prompt_evidence_denied",
      "external_public_install_restart_active_binary_evidence_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial",
        status:"allowed_report_only",
        writes_audit_trail:false,
        persists_evidence:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"stage_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial",
        status:"allowed_report_only_next_slice",
        writes_audit_trail:false,
        persists_evidence:false,
        performs_retention:false,
        performs_gc:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"run_full_light_preflight",
        status:"allowed_verification_only",
        writes_audit_trail:false,
        persists_evidence:false,
        mutates_runtime:false,
        invokes_model:false,
        writes_kg:false
      }
    ],
    source_cancellation_supersession_report_required:true,
    audit_trail_acceptance_forbidden:true,
    audit_trail_recording_forbidden:true,
    audit_trail_persistence_forbidden:true,
    immutable_evidence_acceptance_forbidden:true,
    immutable_evidence_recording_forbidden:true,
    immutable_evidence_persistence_forbidden:true,
    hash_chain_or_merkle_root_recording_forbidden:true,
    attestation_witness_notary_recording_forbidden:true,
    runtime_provider_memory_kg_evidence_forbidden:true,
    secret_read_forbidden:true,
    external_public_install_restart_active_binary_evidence_forbidden:true,
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
      activation_command_result_receipt_index_evidence_persisted:false,
      activation_command_result_receipt_delivery_evidence_recorded:false,
      activation_command_result_receipt_delivery_evidence_persisted:false,
      activation_command_result_receipt_cancellation_recorded:false,
      activation_command_result_receipt_cancellation_persisted:false,
      activation_command_result_receipt_supersession_recorded:false,
      activation_command_result_receipt_supersession_persisted:false,
      activation_command_result_receipt_replacement_receipt_recorded:false,
      activation_command_result_receipt_replacement_receipt_persisted:false,
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
      activation_activated:false,
      runtime_router_mutated:false,
      runtime_attachment_performed:false,
      live_context_attached:false,
      context_injection_performed:false,
      adapter_invoked:false,
      provider_invoked:false,
      model_invoked:false,
      provider_prompt_replayed:false,
      auth_secret_read:false,
      credential_read:false,
      secret_file_read:false,
      usage_recorded:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      live_kg_write_performed:false,
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
      public_artifact_written:false,
      install_executed:false,
      launchd_mutated:false,
      service_restart_performed:false,
      active_binary_mutated:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate"
  and .activation_command_result_receipt_audit_trail_immutable_evidence_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_v1"
  and .runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status == "blocked"
  and .source_activation_command_result_receipt_cancellation_supersession_ready == true
  and .source_activation_command_result_receipt_cancellation_supersession_status == "blocked"
  and .source_activation_command_result_receipt_cancellation_supersession_report_sha256 != ""
  and .runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
  and .minimum_required_samples >= 24
  and .audit_trail_immutable_evidence_surface_count == 12
  and .audit_trail_immutable_evidence_surface_ready_count == 12
  and .audit_trail_immutable_evidence_side_effect_free_surface_count == 12
  and .audit_trail_immutable_evidence_fixture_count == 10
  and .blocked_audit_trail_immutable_evidence_fixture_count == 10
  and .noop_audit_trail_immutable_evidence_fixture_count == 10
  and .allowed_audit_trail_immutable_evidence_fixture_count == 0
  and .accepted_audit_trail_immutable_evidence_fixture_count == 0
  and .audit_trail_denied_count == 10
  and .immutable_evidence_denied_count == 6
  and .audit_trail_performed_count == 0
  and .immutable_evidence_performed_count == 0
  and .hash_chain_recorded_count == 0
  and .merkle_root_recorded_count == 0
  and .attestation_recorded_count == 0
  and .witness_recorded_count == 0
  and .notary_recorded_count == 0
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
  and .activation_command_completion_ack_recorded == false
  and .activation_command_completion_ack_accepted == false
  and .activation_from_audit_trail_allowed == false
  and .activation_from_immutable_evidence_allowed == false
  and .activation_from_cancellation_allowed == false
  and .activation_from_supersession_allowed == false
  and .activation_from_ordering_allowed == false
  and .activation_from_replay_allowed == false
  and .activation_from_receipt_allowed == false
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
  and .provider_prompt_replayed == false
  and .auth_secret_read == false
  and .credential_read == false
  and .secret_file_read == false
  and .usage_recorded == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
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
  and (.audit_trail_immutable_evidence_surfaces | length) == 12
  and (.audit_trail_immutable_evidence_fixtures | length) == 10
  and (.audit_trail_immutable_evidence_fixtures | all(
    (.audit_evidence_status == "blocked_noop" or .audit_evidence_status == "blocked_evidence_noop")
    and .audit_trail_allowed == false
    and .audit_trail_recorded == false
    and .audit_trail_persisted == false
    and .immutable_evidence_allowed == false
    and .immutable_evidence_recorded == false
    and .immutable_evidence_persisted == false
    and .hash_chain_recorded == false
    and .attestation_recorded == false
    and .witness_recorded == false
    and .activation_command_result_receipt_recorded == false
    and .activation_command_result_receipt_persisted == false
    and .activation_command_result_receipt_accepted == false
    and .activation_activated == false
    and .runtime_router_mutated == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .rollback_executed == false
    and .external_send_performed == false
    and .install_executed == false
    and .service_restart_performed == false
    and .active_binary_mutated == false
    and .receipt_noop_confirmed == true
  ))
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.source_cancellation_supersession_present == false)] | length) == 1
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.audit_trail_request_shape == "append_blocked_noop_result_receipt_audit_trail")] | length) == 1
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.immutable_evidence_request_shape == "seal_blocked_noop_result_receipt_as_immutable_evidence")] | length) == 1
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.hash_chain_requested == true and .merkle_root_requested == true)] | length) == 1
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.attestation_requested == true and .witness_requested == true and .notary_requested == true)] | length) == 1
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.audit_trail_materialization_requested == true and .audit_trail_filesystem_write_requested == true)] | length) == 1
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.ledger_evidence_requested == true and .index_evidence_requested == true and .delivery_evidence_requested == true)] | length) == 1
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.activation_from_audit_evidence_requested == true)] | length) == 1
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.memory_store_evidence_requested == true and .live_kg_evidence_requested == true and .provider_prompt_evidence_requested == true)] | length) == 1
  and ([.audit_trail_immutable_evidence_fixtures[] | select(.external_send_evidence_requested == true and .install_evidence_requested == true and .active_binary_mutation_evidence_requested == true)] | length) == 1
  and (.denied_by_audit_trail_immutable_evidence | length) == 24
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial" and .status == "allowed_report_only_next_slice" and .performs_retention == false and .performs_gc == false and .mutates_runtime == false and .invokes_model == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement runtime provider-router activation command result receipt audit-trail/immutable-evidence denial gate passed"
