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

AUDIT_EVIDENCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate.sh
)"

retention_expiry_garbage_collection_fixtures_json="$(
  jq -n '
    def retention_gc_fixture($id; $status; $reason; $extra):
      {
        id:$id,
        retention_requested:true,
        expiry_requested:false,
        garbage_collection_requested:false,
        retention_gc_status:$status,
        source_audit_evidence_present:true,
        source_audit_evidence_ready:true,
        retention_policy_allowed:false,
        retention_policy_recorded:false,
        retention_policy_persisted:false,
        retention_policy_materialized:false,
        retention_index_allowed:false,
        retention_index_recorded:false,
        retention_index_persisted:false,
        expiry_allowed:false,
        expiry_recorded:false,
        expiry_persisted:false,
        expiry_scheduler_registered:false,
        expiry_timer_started:false,
        expiry_materialized:false,
        ttl_update_allowed:false,
        ttl_update_recorded:false,
        ttl_extension_allowed:false,
        ttl_extension_recorded:false,
        garbage_collection_allowed:false,
        garbage_collection_scan_performed:false,
        garbage_collection_candidate_recorded:false,
        garbage_collection_decision_recorded:false,
        garbage_collection_persisted:false,
        delete_allowed:false,
        delete_performed:false,
        delete_marker_recorded:false,
        tombstone_recorded:false,
        sweep_allowed:false,
        sweep_performed:false,
        archive_allowed:false,
        archive_written:false,
        compaction_allowed:false,
        compaction_performed:false,
        compaction_artifact_written:false,
        ledger_retention_recorded:false,
        ledger_retention_persisted:false,
        index_retention_recorded:false,
        index_retention_persisted:false,
        delivery_retention_recorded:false,
        delivery_retention_persisted:false,
        activation_command_result_receipt_audit_trail_recorded:false,
        activation_command_result_receipt_audit_trail_persisted:false,
        activation_command_result_receipt_immutable_evidence_recorded:false,
        activation_command_result_receipt_immutable_evidence_persisted:false,
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
        activation_from_retention_allowed:false,
        activation_from_expiry_allowed:false,
        activation_from_garbage_collection_allowed:false,
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
      retention_gc_fixture("provider-router-activation-command-result-receipt-retention-missing-source-audit-evidence"; "blocked_noop"; "source_audit_trail_immutable_evidence_report_required"; {source_audit_evidence_present:false, source_audit_evidence_ready:false}),
      retention_gc_fixture("provider-router-activation-command-result-receipt-retention-policy-write-request"; "blocked_noop"; "retention_policy_write_request_denied"; {retention_policy_request_shape:"record_blocked_noop_receipt_retention_policy"}),
      retention_gc_fixture("provider-router-activation-command-result-receipt-retention-index-record"; "blocked_noop"; "retention_index_recording_denied"; {retention_index_requested:true}),
      retention_gc_fixture("provider-router-activation-command-result-receipt-expiry-scheduler-timer"; "blocked_expiry_noop"; "expiry_scheduler_timer_denied"; {retention_requested:false, expiry_requested:true, expiry_schedule_requested:true, expiry_timer_requested:true}),
      retention_gc_fixture("provider-router-activation-command-result-receipt-ttl-update-extension"; "blocked_expiry_noop"; "ttl_update_extension_denied"; {retention_requested:false, expiry_requested:true, ttl_update_requested:true, ttl_extension_requested:true}),
      retention_gc_fixture("provider-router-activation-command-result-receipt-garbage-collection-scan"; "blocked_gc_noop"; "garbage_collection_scan_denied"; {retention_requested:false, garbage_collection_requested:true, garbage_collection_scan_requested:true}),
      retention_gc_fixture("provider-router-activation-command-result-receipt-delete-tombstone-sweep"; "blocked_gc_noop"; "delete_tombstone_sweep_denied"; {retention_requested:false, garbage_collection_requested:true, delete_requested:true, tombstone_requested:true, sweep_requested:true}),
      retention_gc_fixture("provider-router-activation-command-result-receipt-archive-compaction"; "blocked_gc_noop"; "archive_compaction_denied"; {retention_requested:false, garbage_collection_requested:true, archive_requested:true, compaction_requested:true}),
      retention_gc_fixture("provider-router-activation-command-result-receipt-activation-memory-kg-provider-retention-gc"; "blocked_gc_noop"; "activation_memory_kg_provider_retention_gc_denied"; {retention_requested:false, expiry_requested:true, garbage_collection_requested:true, activation_from_retention_gc_requested:true, memory_store_gc_evidence_requested:true, live_kg_gc_evidence_requested:true, rollback_gc_evidence_requested:true, secret_material_gc_evidence_requested:true, provider_prompt_gc_evidence_requested:true}),
      retention_gc_fixture("provider-router-activation-command-result-receipt-external-public-install-retention-gc"; "blocked_gc_noop"; "external_public_install_restart_active_binary_retention_gc_denied"; {retention_requested:false, expiry_requested:true, garbage_collection_requested:true, ledger_retention_requested:true, index_retention_requested:true, delivery_retention_requested:true, external_send_gc_evidence_requested:true, public_claim_gc_evidence_requested:true, release_artifact_gc_evidence_requested:true, install_gc_evidence_requested:true, service_restart_gc_evidence_requested:true, active_binary_gc_evidence_requested:true})
    ]
  '
)"

audit_evidence_report_sha256="$(sha256_text "$AUDIT_EVIDENCE_JSON")"
retention_expiry_garbage_collection_fixtures_sha256="$(sha256_text "$retention_expiry_garbage_collection_fixtures_json")"
retention_expiry_garbage_collection_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial:$audit_evidence_report_sha256:$retention_expiry_garbage_collection_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
retention_expiry_garbage_collection_policy_hash_sha256="$(
  sha256_text "runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial:no-retention:no-expiry:no-gc:no-delete:no-archive:no-runtime:no-provider:no-model:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "retention=false;expiry=false;gc=false;delete=false;archive=false;record=false;persist=false;activation=false;runtime=false;provider=false;model=false;memory=false;kg=false;secret=false;external=false;install=false;restart=false;active_binary=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$AUDIT_EVIDENCE_JSON" \
  --argjson fixtures "$retention_expiry_garbage_collection_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate"
    and $source.activation_command_result_receipt_audit_trail_immutable_evidence_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_v1"
    and $source.runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status == "blocked"
    and $source.runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
    and $source.minimum_required_samples >= 24
    and $source.audit_trail_immutable_evidence_surface_count == 12
    and $source.audit_trail_immutable_evidence_surface_ready_count == 12
    and $source.audit_trail_immutable_evidence_fixture_count == 10
    and $source.blocked_audit_trail_immutable_evidence_fixture_count == 10
    and $source.allowed_audit_trail_immutable_evidence_fixture_count == 0
    and $source.accepted_audit_trail_immutable_evidence_fixture_count == 0
    and $source.activation_command_result_receipt_audit_trail_allowed == false
    and $source.activation_command_result_receipt_audit_trail_recorded == false
    and $source.activation_command_result_receipt_audit_trail_persisted == false
    and $source.activation_command_result_receipt_immutable_evidence_allowed == false
    and $source.activation_command_result_receipt_immutable_evidence_recorded == false
    and $source.activation_command_result_receipt_immutable_evidence_persisted == false
    and $source.activation_command_result_receipt_hash_chain_recorded == false
    and $source.activation_command_result_receipt_merkle_root_recorded == false
    and $source.activation_command_result_receipt_attestation_recorded == false
    and $source.activation_command_result_receipt_witness_recorded == false
    and $source.activation_command_result_receipt_notary_recorded == false
    and $source.activation_command_result_receipt_ledger_evidence_recorded == false
    and $source.activation_command_result_receipt_index_evidence_recorded == false
    and $source.activation_command_result_receipt_delivery_evidence_recorded == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_completion_ack_recorded == false
    and $source.activation_command_completion_ack_accepted == false
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
    and ($source.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial" and .status == "allowed_report_only_next_slice" and .performs_retention == false and .performs_gc == false and .mutates_runtime == false and .invokes_model == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.retention_gc_status == "blocked_noop" or .retention_gc_status == "blocked_expiry_noop" or .retention_gc_status == "blocked_gc_noop")
      and .retention_policy_allowed == false
      and .retention_policy_recorded == false
      and .retention_policy_persisted == false
      and .expiry_allowed == false
      and .expiry_recorded == false
      and .expiry_scheduler_registered == false
      and .garbage_collection_allowed == false
      and .garbage_collection_scan_performed == false
      and .garbage_collection_decision_recorded == false
      and .delete_performed == false
      and .tombstone_recorded == false
      and .sweep_performed == false
      and .archive_written == false
      and .compaction_performed == false
      and .activation_command_result_receipt_recorded == false
      and .activation_command_result_receipt_persisted == false
      and .activation_command_result_receipt_accepted == false
      and .activation_command_completion_ack_recorded == false
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
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_gate" \
  --arg audit_evidence_report_sha256 "$audit_evidence_report_sha256" \
  --arg retention_expiry_garbage_collection_fixtures_sha256 "$retention_expiry_garbage_collection_fixtures_sha256" \
  --arg retention_expiry_garbage_collection_contract_hash_sha256 "$retention_expiry_garbage_collection_contract_hash_sha256" \
  --arg retention_expiry_garbage_collection_policy_hash_sha256 "$retention_expiry_garbage_collection_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$AUDIT_EVIDENCE_JSON" \
  --argjson fixtures "$retention_expiry_garbage_collection_fixtures_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_retention_expiry_garbage_collection_schema_version:"memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_v1",
    activation_command_result_receipt_retention_expiry_garbage_collection_mode:"runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_no_retention_no_expiry_no_gc",
    source_activation_command_result_receipt_audit_trail_immutable_evidence_gate:$source.gate,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_ready:$source.runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_status:$source.runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256:$audit_evidence_report_sha256,
    source_activation_command_result_receipt_cancellation_supersession_ready:$source.runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready,
    source_activation_command_result_receipt_cancellation_supersession_report_sha256:$source.source_activation_command_result_receipt_cancellation_supersession_report_sha256,
    source_activation_command_result_receipt_ordering_monotonicity_ready:$source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready,
    source_activation_command_result_receipt_ordering_monotonicity_report_sha256:$source.source_activation_command_result_receipt_ordering_monotonicity_report_sha256,
    source_activation_command_result_receipt_replay_idempotency_ready:$source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready,
    source_activation_command_result_receipt_replay_idempotency_report_sha256:$source.source_activation_command_result_receipt_replay_idempotency_report_sha256,
    source_activation_command_result_receipt_no_persistence_ready:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready,
    source_activation_command_result_receipt_no_persistence_report_sha256:$source.source_activation_command_result_receipt_no_persistence_report_sha256,
    retention_expiry_garbage_collection_fixtures_sha256:$retention_expiry_garbage_collection_fixtures_sha256,
    retention_expiry_garbage_collection_contract_hash_sha256:$retention_expiry_garbage_collection_contract_hash_sha256,
    retention_expiry_garbage_collection_policy_hash_sha256:$retention_expiry_garbage_collection_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready:true,
    runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status:"blocked",
    runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready,
    runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready,
    runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready,
    runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready,
    runtime_provider_router_activation_command_result_receipt_no_persistence_ready:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready,
    audit_trail_immutable_evidence_surface_count:$source.audit_trail_immutable_evidence_surface_count,
    audit_trail_immutable_evidence_fixture_count:$source.audit_trail_immutable_evidence_fixture_count,
    retention_expiry_garbage_collection_surface_count:12,
    retention_expiry_garbage_collection_surface_ready_count:12,
    retention_expiry_garbage_collection_side_effect_free_surface_count:12,
    retention_expiry_garbage_collection_fixture_count:($fixtures | length),
    blocked_retention_expiry_garbage_collection_fixture_count:($fixtures | length),
    noop_retention_expiry_garbage_collection_fixture_count:($fixtures | length),
    allowed_retention_expiry_garbage_collection_fixture_count:0,
    accepted_retention_expiry_garbage_collection_fixture_count:0,
    retention_denied_count:($fixtures | length),
    expiry_denied_count:($fixtures | length),
    garbage_collection_denied_count:($fixtures | length),
    retention_performed_count:0,
    expiry_performed_count:0,
    garbage_collection_performed_count:0,
    activation_command_result_receipt_retention_policy_allowed:false,
    activation_command_result_receipt_retention_policy_recorded:false,
    activation_command_result_receipt_retention_policy_persisted:false,
    activation_command_result_receipt_retention_policy_materialized:false,
    activation_command_result_receipt_retention_index_allowed:false,
    activation_command_result_receipt_retention_index_recorded:false,
    activation_command_result_receipt_retention_index_persisted:false,
    activation_command_result_receipt_expiry_allowed:false,
    activation_command_result_receipt_expiry_recorded:false,
    activation_command_result_receipt_expiry_persisted:false,
    activation_command_result_receipt_expiry_scheduler_registered:false,
    activation_command_result_receipt_expiry_timer_started:false,
    activation_command_result_receipt_expiry_materialized:false,
    activation_command_result_receipt_ttl_update_allowed:false,
    activation_command_result_receipt_ttl_update_recorded:false,
    activation_command_result_receipt_ttl_extension_allowed:false,
    activation_command_result_receipt_ttl_extension_recorded:false,
    activation_command_result_receipt_garbage_collection_allowed:false,
    activation_command_result_receipt_garbage_collection_scan_performed:false,
    activation_command_result_receipt_garbage_collection_candidate_recorded:false,
    activation_command_result_receipt_garbage_collection_decision_recorded:false,
    activation_command_result_receipt_garbage_collection_persisted:false,
    activation_command_result_receipt_delete_allowed:false,
    activation_command_result_receipt_delete_performed:false,
    activation_command_result_receipt_delete_marker_recorded:false,
    activation_command_result_receipt_tombstone_recorded:false,
    activation_command_result_receipt_sweep_allowed:false,
    activation_command_result_receipt_sweep_performed:false,
    activation_command_result_receipt_archive_allowed:false,
    activation_command_result_receipt_archive_written:false,
    activation_command_result_receipt_compaction_allowed:false,
    activation_command_result_receipt_compaction_performed:false,
    activation_command_result_receipt_compaction_artifact_written:false,
    activation_command_result_receipt_ledger_retention_recorded:false,
    activation_command_result_receipt_ledger_retention_persisted:false,
    activation_command_result_receipt_index_retention_recorded:false,
    activation_command_result_receipt_index_retention_persisted:false,
    activation_command_result_receipt_delivery_retention_recorded:false,
    activation_command_result_receipt_delivery_retention_persisted:false,
    activation_command_result_receipt_audit_trail_recorded:false,
    activation_command_result_receipt_audit_trail_persisted:false,
    activation_command_result_receipt_immutable_evidence_recorded:false,
    activation_command_result_receipt_immutable_evidence_persisted:false,
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
    activation_allowed_by_result_receipt_retention:false,
    activation_allowed_by_result_receipt_expiry:false,
    activation_allowed_by_result_receipt_garbage_collection:false,
    activation_allowed_by_result_receipt_audit_trail:false,
    activation_allowed_by_result_receipt_immutable_evidence:false,
    activation_allowed_by_result_receipt:false,
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
    retention_expiry_garbage_collection_surfaces:[
      "source_audit_trail_immutable_evidence_report_required",
      "retention_policy_request_shape_denied",
      "retention_index_recording_denied",
      "expiry_scheduler_registration_denied",
      "ttl_update_extension_denied",
      "garbage_collection_scan_denied",
      "delete_tombstone_sweep_denied",
      "archive_compaction_denied",
      "ledger_index_delivery_retention_evidence_denied",
      "activation_from_retention_expiry_gc_denied",
      "memory_kg_rollback_secret_provider_gc_denied",
      "external_public_install_restart_active_binary_gc_denied"
    ],
    retention_expiry_garbage_collection_fixtures:$fixtures,
    denied_by_retention_expiry_garbage_collection:[
      "source_audit_trail_immutable_evidence_report_required",
      "retention_policy_request_acceptance_denied",
      "retention_policy_recording_denied",
      "retention_policy_persistence_denied",
      "retention_index_recording_denied",
      "expiry_request_acceptance_denied",
      "expiry_recording_denied",
      "expiry_scheduler_registration_denied",
      "expiry_timer_start_denied",
      "ttl_update_denied",
      "ttl_extension_denied",
      "garbage_collection_request_acceptance_denied",
      "garbage_collection_scan_denied",
      "garbage_collection_candidate_recording_denied",
      "garbage_collection_decision_recording_denied",
      "delete_marker_recording_denied",
      "tombstone_recording_denied",
      "sweep_execution_denied",
      "archive_write_denied",
      "compaction_execution_denied",
      "ledger_retention_recording_denied",
      "index_retention_recording_denied",
      "delivery_retention_recording_denied",
      "activation_from_retention_expiry_gc_denied",
      "memory_kg_gc_denied",
      "rollback_gc_denied",
      "secret_material_gc_denied",
      "provider_prompt_gc_denied",
      "external_public_install_restart_active_binary_gc_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial",
        status:"allowed_report_only",
        performs_retention:false,
        performs_expiry:false,
        performs_gc:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"stage_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial",
        status:"allowed_report_only_next_slice",
        exports_receipt:false,
        registers_query:false,
        records_observability:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"run_full_light_preflight",
        status:"allowed_verification_only",
        performs_retention:false,
        performs_gc:false,
        mutates_runtime:false,
        invokes_model:false,
        writes_kg:false
      }
    ],
    source_audit_trail_immutable_evidence_report_required:true,
    retention_acceptance_forbidden:true,
    retention_recording_forbidden:true,
    retention_persistence_forbidden:true,
    expiry_acceptance_forbidden:true,
    expiry_scheduler_registration_forbidden:true,
    ttl_update_forbidden:true,
    garbage_collection_forbidden:true,
    delete_tombstone_sweep_forbidden:true,
    archive_compaction_forbidden:true,
    runtime_provider_memory_kg_gc_evidence_forbidden:true,
    secret_read_forbidden:true,
    external_public_install_restart_active_binary_gc_forbidden:true,
    side_effects:{
      activation_command_result_receipt_retention_policy_recorded:false,
      activation_command_result_receipt_retention_policy_persisted:false,
      activation_command_result_receipt_retention_policy_materialized:false,
      activation_command_result_receipt_retention_index_recorded:false,
      activation_command_result_receipt_retention_index_persisted:false,
      activation_command_result_receipt_expiry_recorded:false,
      activation_command_result_receipt_expiry_persisted:false,
      activation_command_result_receipt_expiry_scheduler_registered:false,
      activation_command_result_receipt_expiry_timer_started:false,
      activation_command_result_receipt_expiry_materialized:false,
      activation_command_result_receipt_ttl_update_recorded:false,
      activation_command_result_receipt_ttl_extension_recorded:false,
      activation_command_result_receipt_garbage_collection_scan_performed:false,
      activation_command_result_receipt_garbage_collection_candidate_recorded:false,
      activation_command_result_receipt_garbage_collection_decision_recorded:false,
      activation_command_result_receipt_garbage_collection_persisted:false,
      activation_command_result_receipt_delete_performed:false,
      activation_command_result_receipt_delete_marker_recorded:false,
      activation_command_result_receipt_tombstone_recorded:false,
      activation_command_result_receipt_sweep_performed:false,
      activation_command_result_receipt_archive_written:false,
      activation_command_result_receipt_compaction_performed:false,
      activation_command_result_receipt_compaction_artifact_written:false,
      activation_command_result_receipt_ledger_retention_recorded:false,
      activation_command_result_receipt_ledger_retention_persisted:false,
      activation_command_result_receipt_index_retention_recorded:false,
      activation_command_result_receipt_index_retention_persisted:false,
      activation_command_result_receipt_delivery_retention_recorded:false,
      activation_command_result_receipt_delivery_retention_persisted:false,
      activation_command_result_receipt_audit_trail_recorded:false,
      activation_command_result_receipt_audit_trail_persisted:false,
      activation_command_result_receipt_immutable_evidence_recorded:false,
      activation_command_result_receipt_immutable_evidence_persisted:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_gate"
  and .activation_command_result_receipt_retention_expiry_garbage_collection_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_v1"
  and .runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status == "blocked"
  and .source_activation_command_result_receipt_audit_trail_immutable_evidence_ready == true
  and .source_activation_command_result_receipt_audit_trail_immutable_evidence_status == "blocked"
  and .source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256 != ""
  and .runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
  and .minimum_required_samples >= 24
  and .retention_expiry_garbage_collection_surface_count == 12
  and .retention_expiry_garbage_collection_surface_ready_count == 12
  and .retention_expiry_garbage_collection_side_effect_free_surface_count == 12
  and .retention_expiry_garbage_collection_fixture_count == 10
  and .blocked_retention_expiry_garbage_collection_fixture_count == 10
  and .noop_retention_expiry_garbage_collection_fixture_count == 10
  and .allowed_retention_expiry_garbage_collection_fixture_count == 0
  and .accepted_retention_expiry_garbage_collection_fixture_count == 0
  and .retention_denied_count == 10
  and .expiry_denied_count == 10
  and .garbage_collection_denied_count == 10
  and .retention_performed_count == 0
  and .expiry_performed_count == 0
  and .garbage_collection_performed_count == 0
  and .activation_command_result_receipt_retention_policy_allowed == false
  and .activation_command_result_receipt_retention_policy_recorded == false
  and .activation_command_result_receipt_retention_policy_persisted == false
  and .activation_command_result_receipt_retention_policy_materialized == false
  and .activation_command_result_receipt_retention_index_allowed == false
  and .activation_command_result_receipt_retention_index_recorded == false
  and .activation_command_result_receipt_expiry_allowed == false
  and .activation_command_result_receipt_expiry_recorded == false
  and .activation_command_result_receipt_expiry_scheduler_registered == false
  and .activation_command_result_receipt_expiry_timer_started == false
  and .activation_command_result_receipt_ttl_update_allowed == false
  and .activation_command_result_receipt_ttl_extension_allowed == false
  and .activation_command_result_receipt_garbage_collection_allowed == false
  and .activation_command_result_receipt_garbage_collection_scan_performed == false
  and .activation_command_result_receipt_garbage_collection_decision_recorded == false
  and .activation_command_result_receipt_delete_allowed == false
  and .activation_command_result_receipt_delete_performed == false
  and .activation_command_result_receipt_delete_marker_recorded == false
  and .activation_command_result_receipt_tombstone_recorded == false
  and .activation_command_result_receipt_sweep_allowed == false
  and .activation_command_result_receipt_sweep_performed == false
  and .activation_command_result_receipt_archive_allowed == false
  and .activation_command_result_receipt_archive_written == false
  and .activation_command_result_receipt_compaction_allowed == false
  and .activation_command_result_receipt_compaction_performed == false
  and .activation_command_result_receipt_ledger_retention_recorded == false
  and .activation_command_result_receipt_index_retention_recorded == false
  and .activation_command_result_receipt_delivery_retention_recorded == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_completion_ack_recorded == false
  and .activation_command_completion_ack_accepted == false
  and .activation_allowed_by_result_receipt_retention == false
  and .activation_allowed_by_result_receipt_expiry == false
  and .activation_allowed_by_result_receipt_garbage_collection == false
  and .activation_allowed_by_result_receipt_audit_trail == false
  and .activation_allowed_by_result_receipt_immutable_evidence == false
  and .activation_allowed_by_result_receipt == false
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
  and (.retention_expiry_garbage_collection_surfaces | length) == 12
  and (.retention_expiry_garbage_collection_fixtures | length) == 10
  and (.retention_expiry_garbage_collection_fixtures | all(
    (.retention_gc_status == "blocked_noop" or .retention_gc_status == "blocked_expiry_noop" or .retention_gc_status == "blocked_gc_noop")
    and .retention_policy_allowed == false
    and .retention_policy_recorded == false
    and .retention_policy_persisted == false
    and .expiry_allowed == false
    and .expiry_recorded == false
    and .expiry_scheduler_registered == false
    and .garbage_collection_allowed == false
    and .garbage_collection_scan_performed == false
    and .garbage_collection_decision_recorded == false
    and .delete_performed == false
    and .tombstone_recorded == false
    and .sweep_performed == false
    and .archive_written == false
    and .compaction_performed == false
    and .activation_command_result_receipt_recorded == false
    and .activation_command_result_receipt_persisted == false
    and .activation_command_result_receipt_accepted == false
    and .activation_command_completion_ack_recorded == false
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
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.source_audit_evidence_present == false)] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.retention_policy_request_shape == "record_blocked_noop_receipt_retention_policy")] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.retention_index_requested == true)] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.expiry_schedule_requested == true and .expiry_timer_requested == true)] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.ttl_update_requested == true and .ttl_extension_requested == true)] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.garbage_collection_scan_requested == true)] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.delete_requested == true and .tombstone_requested == true and .sweep_requested == true)] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.archive_requested == true and .compaction_requested == true)] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.activation_from_retention_gc_requested == true and .memory_store_gc_evidence_requested == true and .live_kg_gc_evidence_requested == true and .provider_prompt_gc_evidence_requested == true)] | length) == 1
  and ([.retention_expiry_garbage_collection_fixtures[] | select(.external_send_gc_evidence_requested == true and .install_gc_evidence_requested == true and .active_binary_gc_evidence_requested == true)] | length) == 1
  and (.denied_by_retention_expiry_garbage_collection | length) == 29
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial" and .status == "allowed_report_only_next_slice" and .exports_receipt == false and .registers_query == false and .records_observability == false and .mutates_runtime == false and .invokes_model == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement runtime provider-router activation command result receipt retention/expiry/garbage-collection denial gate passed"
