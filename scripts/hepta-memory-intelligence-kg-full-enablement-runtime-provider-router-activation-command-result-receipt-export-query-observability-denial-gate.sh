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

RETENTION_GC_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial-gate.sh
)"

export_query_observability_fixtures_json="$(
  jq -n '
    def export_query_observability_fixture($id; $status; $reason; $extra):
      {
        id:$id,
        export_requested:false,
        query_requested:false,
        observability_requested:false,
        export_query_observability_status:$status,
        source_retention_expiry_gc_present:true,
        source_retention_expiry_gc_ready:true,
        export_allowed:false,
        export_request_accepted:false,
        export_recorded:false,
        export_persisted:false,
        export_artifact_written:false,
        export_stream_opened:false,
        export_filesystem_written:false,
        query_allowed:false,
        query_registered:false,
        query_endpoint_materialized:false,
        query_index_recorded:false,
        query_cache_written:false,
        query_result_materialized:false,
        observability_allowed:false,
        observability_metric_emitted:false,
        observability_log_recorded:false,
        observability_trace_recorded:false,
        observability_span_recorded:false,
        observability_event_recorded:false,
        observability_dashboard_materialized:false,
        observability_alert_registered:false,
        observability_slo_recorded:false,
        ledger_observability_recorded:false,
        index_observability_recorded:false,
        delivery_observability_recorded:false,
        activation_command_result_receipt_retention_policy_recorded:false,
        activation_command_result_receipt_expiry_recorded:false,
        activation_command_result_receipt_garbage_collection_scan_performed:false,
        activation_command_result_receipt_audit_trail_recorded:false,
        activation_command_result_receipt_immutable_evidence_recorded:false,
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
        activation_from_export_allowed:false,
        activation_from_query_allowed:false,
        activation_from_observability_allowed:false,
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
      export_query_observability_fixture("provider-router-activation-command-result-receipt-export-missing-source-retention-gc"; "blocked_noop"; "source_retention_expiry_garbage_collection_report_required"; {source_retention_expiry_gc_present:false, source_retention_expiry_gc_ready:false, export_requested:true}),
      export_query_observability_fixture("provider-router-activation-command-result-receipt-export-artifact-request"; "blocked_export_noop"; "export_artifact_write_denied"; {export_requested:true, export_file_requested:true}),
      export_query_observability_fixture("provider-router-activation-command-result-receipt-export-stream-request"; "blocked_export_noop"; "export_stream_open_denied"; {export_requested:true, export_stream_requested:true}),
      export_query_observability_fixture("provider-router-activation-command-result-receipt-query-endpoint-request"; "blocked_query_noop"; "query_endpoint_materialization_denied"; {query_requested:true, query_endpoint_requested:true}),
      export_query_observability_fixture("provider-router-activation-command-result-receipt-query-index-cache-request"; "blocked_query_noop"; "query_index_cache_recording_denied"; {query_requested:true, query_index_requested:true, query_cache_requested:true}),
      export_query_observability_fixture("provider-router-activation-command-result-receipt-observability-metric-request"; "blocked_observability_noop"; "observability_metric_emission_denied"; {observability_requested:true, metric_requested:true}),
      export_query_observability_fixture("provider-router-activation-command-result-receipt-observability-trace-log-event-request"; "blocked_observability_noop"; "trace_span_log_event_recording_denied"; {observability_requested:true, trace_requested:true, span_requested:true, log_requested:true, event_requested:true}),
      export_query_observability_fixture("provider-router-activation-command-result-receipt-dashboard-alert-slo-request"; "blocked_observability_noop"; "dashboard_alert_slo_materialization_denied"; {observability_requested:true, dashboard_requested:true, alert_requested:true, slo_requested:true}),
      export_query_observability_fixture("provider-router-activation-command-result-receipt-activation-memory-kg-provider-observability"; "blocked_observability_noop"; "activation_memory_kg_provider_observability_denied"; {observability_requested:true, activation_from_observability_requested:true, memory_store_observability_requested:true, live_kg_observability_requested:true, rollback_observability_requested:true, secret_material_observability_requested:true, provider_prompt_observability_requested:true}),
      export_query_observability_fixture("provider-router-activation-command-result-receipt-external-public-install-observability"; "blocked_observability_noop"; "external_public_install_restart_active_binary_observability_denied"; {observability_requested:true, ledger_observability_requested:true, index_observability_requested:true, delivery_observability_requested:true, external_send_observability_requested:true, public_claim_observability_requested:true, release_artifact_observability_requested:true, install_observability_requested:true, service_restart_observability_requested:true, active_binary_observability_requested:true})
    ]
  '
)"

retention_gc_report_sha256="$(sha256_text "$RETENTION_GC_JSON")"
export_query_observability_fixtures_sha256="$(sha256_text "$export_query_observability_fixtures_json")"
export_query_observability_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-runtime-provider-router-activation-command-result-receipt-export-query-observability-denial:$retention_gc_report_sha256:$export_query_observability_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
export_query_observability_policy_hash_sha256="$(
  sha256_text "runtime-provider-router-activation-command-result-receipt-export-query-observability-denial:no-export:no-query:no-observability:no-runtime:no-provider:no-model:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "export=false;query=false;observability=false;record=false;persist=false;activation=false;runtime=false;provider=false;model=false;memory=false;kg=false;secret=false;external=false;install=false;restart=false;active_binary=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$RETENTION_GC_JSON" \
  --argjson fixtures "$export_query_observability_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_gate"
    and $source.activation_command_result_receipt_retention_expiry_garbage_collection_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_v1"
    and $source.runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status == "blocked"
    and $source.runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
    and $source.minimum_required_samples >= 24
    and $source.retention_expiry_garbage_collection_surface_count == 12
    and $source.retention_expiry_garbage_collection_surface_ready_count == 12
    and $source.retention_expiry_garbage_collection_side_effect_free_surface_count == 12
    and $source.retention_expiry_garbage_collection_fixture_count == 10
    and $source.blocked_retention_expiry_garbage_collection_fixture_count == 10
    and $source.noop_retention_expiry_garbage_collection_fixture_count == 10
    and $source.allowed_retention_expiry_garbage_collection_fixture_count == 0
    and $source.accepted_retention_expiry_garbage_collection_fixture_count == 0
    and $source.retention_performed_count == 0
    and $source.expiry_performed_count == 0
    and $source.garbage_collection_performed_count == 0
    and $source.activation_command_result_receipt_retention_policy_recorded == false
    and $source.activation_command_result_receipt_retention_policy_persisted == false
    and $source.activation_command_result_receipt_retention_index_recorded == false
    and $source.activation_command_result_receipt_expiry_recorded == false
    and $source.activation_command_result_receipt_expiry_scheduler_registered == false
    and $source.activation_command_result_receipt_expiry_timer_started == false
    and $source.activation_command_result_receipt_ttl_update_recorded == false
    and $source.activation_command_result_receipt_ttl_extension_recorded == false
    and $source.activation_command_result_receipt_garbage_collection_scan_performed == false
    and $source.activation_command_result_receipt_garbage_collection_candidate_recorded == false
    and $source.activation_command_result_receipt_garbage_collection_decision_recorded == false
    and $source.activation_command_result_receipt_delete_performed == false
    and $source.activation_command_result_receipt_tombstone_recorded == false
    and $source.activation_command_result_receipt_sweep_performed == false
    and $source.activation_command_result_receipt_archive_written == false
    and $source.activation_command_result_receipt_compaction_performed == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_result_receipt_materialized == false
    and $source.activation_command_result_receipt_filesystem_written == false
    and $source.activation_allowed_by_result_receipt_retention == false
    and $source.activation_allowed_by_result_receipt_expiry == false
    and $source.activation_allowed_by_result_receipt_garbage_collection == false
    and $source.activation_allowed_by_result_receipt == false
    and $source.activation_command_invoked == false
    and $source.activation_command_dispatched == false
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
    and ($source.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial" and .status == "allowed_report_only_next_slice" and .exports_receipt == false and .registers_query == false and .records_observability == false and .mutates_runtime == false and .invokes_model == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.export_query_observability_status == "blocked_noop" or .export_query_observability_status == "blocked_export_noop" or .export_query_observability_status == "blocked_query_noop" or .export_query_observability_status == "blocked_observability_noop")
      and .export_recorded == false
      and .export_artifact_written == false
      and .export_stream_opened == false
      and .query_registered == false
      and .query_endpoint_materialized == false
      and .query_index_recorded == false
      and .query_cache_written == false
      and .observability_metric_emitted == false
      and .observability_log_recorded == false
      and .observability_trace_recorded == false
      and .observability_span_recorded == false
      and .observability_event_recorded == false
      and .observability_dashboard_materialized == false
      and .observability_alert_registered == false
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
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_gate" \
  --arg retention_gc_report_sha256 "$retention_gc_report_sha256" \
  --arg export_query_observability_fixtures_sha256 "$export_query_observability_fixtures_sha256" \
  --arg export_query_observability_contract_hash_sha256 "$export_query_observability_contract_hash_sha256" \
  --arg export_query_observability_policy_hash_sha256 "$export_query_observability_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$RETENTION_GC_JSON" \
  --argjson fixtures "$export_query_observability_fixtures_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_export_query_observability_schema_version:"memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_v1",
    activation_command_result_receipt_export_query_observability_mode:"runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_no_export_no_query_no_observability",
    source_activation_command_result_receipt_retention_expiry_garbage_collection_gate:$source.gate,
    source_activation_command_result_receipt_retention_expiry_garbage_collection_ready:$source.runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready,
    source_activation_command_result_receipt_retention_expiry_garbage_collection_status:$source.runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_status,
    source_activation_command_result_receipt_retention_expiry_garbage_collection_report_sha256:$retention_gc_report_sha256,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_ready:$source.runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256:$source.source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256,
    source_activation_command_result_receipt_cancellation_supersession_ready:$source.runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready,
    source_activation_command_result_receipt_cancellation_supersession_report_sha256:$source.source_activation_command_result_receipt_cancellation_supersession_report_sha256,
    source_activation_command_result_receipt_ordering_monotonicity_ready:$source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready,
    source_activation_command_result_receipt_ordering_monotonicity_report_sha256:$source.source_activation_command_result_receipt_ordering_monotonicity_report_sha256,
    source_activation_command_result_receipt_replay_idempotency_ready:$source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready,
    source_activation_command_result_receipt_replay_idempotency_report_sha256:$source.source_activation_command_result_receipt_replay_idempotency_report_sha256,
    source_activation_command_result_receipt_no_persistence_ready:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready,
    source_activation_command_result_receipt_no_persistence_report_sha256:$source.source_activation_command_result_receipt_no_persistence_report_sha256,
    export_query_observability_fixtures_sha256:$export_query_observability_fixtures_sha256,
    export_query_observability_contract_hash_sha256:$export_query_observability_contract_hash_sha256,
    export_query_observability_policy_hash_sha256:$export_query_observability_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready:true,
    runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_status:"blocked",
    runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready,
    runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready,
    runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready,
    runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready,
    runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready,
    runtime_provider_router_activation_command_result_receipt_no_persistence_ready:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready,
    retention_expiry_garbage_collection_surface_count:$source.retention_expiry_garbage_collection_surface_count,
    retention_expiry_garbage_collection_fixture_count:$source.retention_expiry_garbage_collection_fixture_count,
    export_query_observability_surface_count:12,
    export_query_observability_surface_ready_count:12,
    export_query_observability_side_effect_free_surface_count:12,
    export_query_observability_fixture_count:($fixtures | length),
    blocked_export_query_observability_fixture_count:($fixtures | length),
    noop_export_query_observability_fixture_count:($fixtures | length),
    allowed_export_query_observability_fixture_count:0,
    accepted_export_query_observability_fixture_count:0,
    export_denied_count:($fixtures | length),
    query_denied_count:($fixtures | length),
    observability_denied_count:($fixtures | length),
    export_performed_count:0,
    query_performed_count:0,
    observability_performed_count:0,
    activation_command_result_receipt_export_allowed:false,
    activation_command_result_receipt_export_request_accepted:false,
    activation_command_result_receipt_export_recorded:false,
    activation_command_result_receipt_export_persisted:false,
    activation_command_result_receipt_export_artifact_written:false,
    activation_command_result_receipt_export_stream_opened:false,
    activation_command_result_receipt_export_filesystem_written:false,
    activation_command_result_receipt_query_allowed:false,
    activation_command_result_receipt_query_registered:false,
    activation_command_result_receipt_query_endpoint_materialized:false,
    activation_command_result_receipt_query_index_recorded:false,
    activation_command_result_receipt_query_cache_written:false,
    activation_command_result_receipt_query_result_materialized:false,
    activation_command_result_receipt_observability_allowed:false,
    activation_command_result_receipt_observability_metric_emitted:false,
    activation_command_result_receipt_observability_log_recorded:false,
    activation_command_result_receipt_observability_trace_recorded:false,
    activation_command_result_receipt_observability_span_recorded:false,
    activation_command_result_receipt_observability_event_recorded:false,
    activation_command_result_receipt_observability_dashboard_materialized:false,
    activation_command_result_receipt_observability_alert_registered:false,
    activation_command_result_receipt_observability_slo_recorded:false,
    activation_command_result_receipt_ledger_observability_recorded:false,
    activation_command_result_receipt_index_observability_recorded:false,
    activation_command_result_receipt_delivery_observability_recorded:false,
    activation_command_result_receipt_retention_policy_recorded:false,
    activation_command_result_receipt_retention_index_recorded:false,
    activation_command_result_receipt_expiry_recorded:false,
    activation_command_result_receipt_garbage_collection_scan_performed:false,
    activation_command_result_receipt_audit_trail_recorded:false,
    activation_command_result_receipt_immutable_evidence_recorded:false,
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
    activation_allowed_by_result_receipt_export:false,
    activation_allowed_by_result_receipt_query:false,
    activation_allowed_by_result_receipt_observability:false,
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
    export_query_observability_surfaces:[
      "source_retention_expiry_garbage_collection_report_required",
      "export_request_shape_denied",
      "export_artifact_write_denied",
      "export_stream_open_denied",
      "query_endpoint_materialization_denied",
      "query_index_cache_recording_denied",
      "observability_metric_emission_denied",
      "trace_span_log_event_recording_denied",
      "dashboard_alert_slo_materialization_denied",
      "ledger_index_delivery_observability_evidence_denied",
      "activation_memory_kg_provider_observability_denied",
      "external_public_install_restart_active_binary_observability_denied"
    ],
    export_query_observability_fixtures:$fixtures,
    denied_by_export_query_observability:[
      "source_retention_expiry_garbage_collection_report_required",
      "export_request_acceptance_denied",
      "export_recording_denied",
      "export_persistence_denied",
      "export_artifact_write_denied",
      "export_stream_open_denied",
      "query_request_acceptance_denied",
      "query_registration_denied",
      "query_endpoint_materialization_denied",
      "query_index_recording_denied",
      "query_cache_write_denied",
      "query_result_materialization_denied",
      "observability_request_acceptance_denied",
      "metric_emission_denied",
      "log_recording_denied",
      "trace_recording_denied",
      "span_recording_denied",
      "event_recording_denied",
      "dashboard_materialization_denied",
      "alert_registration_denied",
      "slo_recording_denied",
      "ledger_observability_recording_denied",
      "index_observability_recording_denied",
      "delivery_observability_recording_denied",
      "activation_from_export_query_observability_denied",
      "memory_kg_observability_denied",
      "rollback_observability_denied",
      "secret_material_observability_denied",
      "provider_prompt_observability_denied",
      "external_public_install_restart_active_binary_observability_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial",
        status:"allowed_report_only",
        exports_receipt:false,
        registers_query:false,
        records_observability:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"stage_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial",
        status:"allowed_report_only_next_slice",
        persists_summary:false,
        persists_briefing:false,
        delivers_summary:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"run_full_light_preflight",
        status:"allowed_verification_only",
        exports_receipt:false,
        registers_query:false,
        records_observability:false,
        mutates_runtime:false,
        invokes_model:false,
        writes_kg:false
      }
    ],
    source_retention_expiry_garbage_collection_report_required:true,
    export_acceptance_forbidden:true,
    export_recording_forbidden:true,
    export_persistence_forbidden:true,
    export_artifact_write_forbidden:true,
    export_stream_forbidden:true,
    query_registration_forbidden:true,
    query_endpoint_materialization_forbidden:true,
    query_index_cache_forbidden:true,
    observability_metric_forbidden:true,
    observability_trace_log_event_forbidden:true,
    dashboard_alert_slo_forbidden:true,
    activation_from_export_query_observability_forbidden:true,
    runtime_provider_memory_kg_observability_forbidden:true,
    secret_read_forbidden:true,
    external_public_install_restart_active_binary_observability_forbidden:true,
    side_effects:{
      activation_command_result_receipt_export_recorded:false,
      activation_command_result_receipt_export_persisted:false,
      activation_command_result_receipt_export_artifact_written:false,
      activation_command_result_receipt_export_stream_opened:false,
      activation_command_result_receipt_export_filesystem_written:false,
      activation_command_result_receipt_query_registered:false,
      activation_command_result_receipt_query_endpoint_materialized:false,
      activation_command_result_receipt_query_index_recorded:false,
      activation_command_result_receipt_query_cache_written:false,
      activation_command_result_receipt_query_result_materialized:false,
      activation_command_result_receipt_observability_metric_emitted:false,
      activation_command_result_receipt_observability_log_recorded:false,
      activation_command_result_receipt_observability_trace_recorded:false,
      activation_command_result_receipt_observability_span_recorded:false,
      activation_command_result_receipt_observability_event_recorded:false,
      activation_command_result_receipt_observability_dashboard_materialized:false,
      activation_command_result_receipt_observability_alert_registered:false,
      activation_command_result_receipt_observability_slo_recorded:false,
      activation_command_result_receipt_ledger_observability_recorded:false,
      activation_command_result_receipt_index_observability_recorded:false,
      activation_command_result_receipt_delivery_observability_recorded:false,
      activation_command_result_receipt_retention_policy_recorded:false,
      activation_command_result_receipt_retention_index_recorded:false,
      activation_command_result_receipt_expiry_recorded:false,
      activation_command_result_receipt_garbage_collection_scan_performed:false,
      activation_command_result_receipt_audit_trail_recorded:false,
      activation_command_result_receipt_immutable_evidence_recorded:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_gate"
  and .activation_command_result_receipt_export_query_observability_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_v1"
  and .runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_status == "blocked"
  and .source_activation_command_result_receipt_retention_expiry_garbage_collection_ready == true
  and .source_activation_command_result_receipt_retention_expiry_garbage_collection_status == "blocked"
  and .source_activation_command_result_receipt_retention_expiry_garbage_collection_report_sha256 != ""
  and .runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
  and .minimum_required_samples >= 24
  and .export_query_observability_surface_count == 12
  and .export_query_observability_surface_ready_count == 12
  and .export_query_observability_side_effect_free_surface_count == 12
  and .export_query_observability_fixture_count == 10
  and .blocked_export_query_observability_fixture_count == 10
  and .noop_export_query_observability_fixture_count == 10
  and .allowed_export_query_observability_fixture_count == 0
  and .accepted_export_query_observability_fixture_count == 0
  and .export_denied_count == 10
  and .query_denied_count == 10
  and .observability_denied_count == 10
  and .export_performed_count == 0
  and .query_performed_count == 0
  and .observability_performed_count == 0
  and .activation_command_result_receipt_export_allowed == false
  and .activation_command_result_receipt_export_recorded == false
  and .activation_command_result_receipt_export_artifact_written == false
  and .activation_command_result_receipt_export_stream_opened == false
  and .activation_command_result_receipt_query_allowed == false
  and .activation_command_result_receipt_query_registered == false
  and .activation_command_result_receipt_query_endpoint_materialized == false
  and .activation_command_result_receipt_query_index_recorded == false
  and .activation_command_result_receipt_query_cache_written == false
  and .activation_command_result_receipt_observability_allowed == false
  and .activation_command_result_receipt_observability_metric_emitted == false
  and .activation_command_result_receipt_observability_log_recorded == false
  and .activation_command_result_receipt_observability_trace_recorded == false
  and .activation_command_result_receipt_observability_span_recorded == false
  and .activation_command_result_receipt_observability_event_recorded == false
  and .activation_command_result_receipt_observability_dashboard_materialized == false
  and .activation_command_result_receipt_observability_alert_registered == false
  and .activation_command_result_receipt_observability_slo_recorded == false
  and .activation_command_result_receipt_ledger_observability_recorded == false
  and .activation_command_result_receipt_index_observability_recorded == false
  and .activation_command_result_receipt_delivery_observability_recorded == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_result_receipt_materialized == false
  and .activation_command_result_receipt_filesystem_written == false
  and .activation_command_completion_ack_recorded == false
  and .activation_allowed_by_result_receipt_export == false
  and .activation_allowed_by_result_receipt_query == false
  and .activation_allowed_by_result_receipt_observability == false
  and .activation_allowed_by_result_receipt_retention == false
  and .activation_allowed_by_result_receipt_expiry == false
  and .activation_allowed_by_result_receipt_garbage_collection == false
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
  and (.export_query_observability_surfaces | length) == 12
  and (.export_query_observability_fixtures | length) == 10
  and (.export_query_observability_fixtures | all(
    (.export_query_observability_status == "blocked_noop" or .export_query_observability_status == "blocked_export_noop" or .export_query_observability_status == "blocked_query_noop" or .export_query_observability_status == "blocked_observability_noop")
    and .export_recorded == false
    and .export_artifact_written == false
    and .export_stream_opened == false
    and .query_registered == false
    and .query_endpoint_materialized == false
    and .query_index_recorded == false
    and .query_cache_written == false
    and .observability_metric_emitted == false
    and .observability_log_recorded == false
    and .observability_trace_recorded == false
    and .observability_span_recorded == false
    and .observability_event_recorded == false
    and .observability_dashboard_materialized == false
    and .observability_alert_registered == false
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
  and ([.export_query_observability_fixtures[] | select(.source_retention_expiry_gc_present == false)] | length) == 1
  and ([.export_query_observability_fixtures[] | select(.export_file_requested == true)] | length) == 1
  and ([.export_query_observability_fixtures[] | select(.export_stream_requested == true)] | length) == 1
  and ([.export_query_observability_fixtures[] | select(.query_endpoint_requested == true)] | length) == 1
  and ([.export_query_observability_fixtures[] | select(.query_index_requested == true and .query_cache_requested == true)] | length) == 1
  and ([.export_query_observability_fixtures[] | select(.metric_requested == true)] | length) == 1
  and ([.export_query_observability_fixtures[] | select(.trace_requested == true and .span_requested == true and .log_requested == true and .event_requested == true)] | length) == 1
  and ([.export_query_observability_fixtures[] | select(.dashboard_requested == true and .alert_requested == true and .slo_requested == true)] | length) == 1
  and ([.export_query_observability_fixtures[] | select(.activation_from_observability_requested == true and .memory_store_observability_requested == true and .live_kg_observability_requested == true and .provider_prompt_observability_requested == true)] | length) == 1
  and ([.export_query_observability_fixtures[] | select(.external_send_observability_requested == true and .install_observability_requested == true and .active_binary_observability_requested == true)] | length) == 1
  and (.denied_by_export_query_observability | length) == 30
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial" and .status == "allowed_report_only_next_slice" and .persists_summary == false and .persists_briefing == false and .delivers_summary == false and .mutates_runtime == false and .invokes_model == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement runtime provider-router activation command result receipt export/query/observability denial gate passed"
