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

RETENTION_GC_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-retention-expiry-garbage-collection-denial-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-retention-expiry-garbage-collection-denial-gate.sh
)"

retention_gc_report_sha256="$(printf '%s' "$RETENTION_GC_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$RETENTION_GC_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_gate"
    and $source.activation_command_result_receipt_retention_expiry_garbage_collection_mode == "memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial"
    and $source.memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_no_persistence_ready == true
    and $source.source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256 != ""
    and $source.source_activation_command_result_receipt_cancellation_supersession_report_sha256 != ""
    and $source.source_activation_command_result_receipt_ordering_monotonicity_report_sha256 != ""
    and $source.source_activation_command_result_receipt_replay_idempotency_report_sha256 != ""
    and $source.source_activation_command_result_receipt_no_persistence_report_sha256 != ""
    and $source.minimum_required_samples >= 24
    and $source.required_activation_command_result_receipt_retention_expiry_garbage_collection_surface_count == 12
    and $source.ready_activation_command_result_receipt_retention_expiry_garbage_collection_surface_count == 12
    and $source.side_effect_free_activation_command_result_receipt_retention_expiry_garbage_collection_surface_count == 12
    and $source.required_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count == 10
    and $source.activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count == 10
    and $source.blocked_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count == 10
    and $source.noop_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count == 10
    and $source.allowed_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count == 0
    and $source.accepted_activation_command_result_receipt_retention_expiry_garbage_collection_fixture_count == 0
    and $source.activation_command_result_receipt_retention_performed_count == 0
    and $source.activation_command_result_receipt_expiry_performed_count == 0
    and $source.activation_command_result_receipt_garbage_collection_performed_count == 0
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
    and ($source.activation_command_result_receipt_retention_expiry_garbage_collection_fixtures | length) == 10
    and ($source.activation_command_result_receipt_retention_expiry_garbage_collection_fixtures | all((.retention_gc_status == "blocked_noop" or .retention_gc_status == "blocked_expiry_noop" or .retention_gc_status == "blocked_gc_noop") and .retention_policy_recorded == false and .expiry_recorded == false and .garbage_collection_scan_performed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_export_query_observability_denial_gate" \
  --arg retention_gc_report_sha256 "$retention_gc_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$RETENTION_GC_JSON" \
  '
  def blocked_fixture($id; $status; $reason; $extra):
    {
      id:$id,
      export_requested:false,
      query_requested:false,
      observability_requested:false,
      export_query_observability_status:$status,
      source_retention_expiry_gc_present:true,
      source_retention_expiry_gc_ready:true,
      export_allowed:false,
      export_requested_accepted:false,
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
      receipt_recorded:false,
      receipt_persisted:false,
      receipt_accepted:false,
      receipt_materialized:false,
      receipt_filesystem_written:false,
      completion_ack_recorded:false,
      completion_ack_persisted:false,
      completion_ack_accepted:false,
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
  [
    blocked_fixture("activation-result-receipt-export-missing-source-retention-gc"; "blocked_noop"; "source_retention_expiry_gc_report_required"; {source_retention_expiry_gc_present:false, source_retention_expiry_gc_ready:false, export_requested:true}),
    blocked_fixture("activation-result-receipt-export-artifact-request"; "blocked_export_noop"; "export_artifact_write_denied"; {export_requested:true, export_file_requested:true}),
    blocked_fixture("activation-result-receipt-export-stream-request"; "blocked_export_noop"; "export_stream_open_denied"; {export_requested:true, export_stream_requested:true}),
    blocked_fixture("activation-result-receipt-query-endpoint-request"; "blocked_query_noop"; "query_endpoint_materialization_denied"; {query_requested:true, query_endpoint_requested:true}),
    blocked_fixture("activation-result-receipt-query-index-cache-request"; "blocked_query_noop"; "query_index_cache_recording_denied"; {query_requested:true, query_index_requested:true, query_cache_requested:true}),
    blocked_fixture("activation-result-receipt-observability-metric-request"; "blocked_observability_noop"; "observability_metric_emission_denied"; {observability_requested:true, metric_requested:true}),
    blocked_fixture("activation-result-receipt-observability-trace-log-request"; "blocked_observability_noop"; "trace_span_log_recording_denied"; {observability_requested:true, trace_requested:true, span_requested:true, log_requested:true}),
    blocked_fixture("activation-result-receipt-dashboard-alert-slo-request"; "blocked_observability_noop"; "dashboard_alert_slo_materialization_denied"; {observability_requested:true, dashboard_requested:true, alert_requested:true, slo_requested:true}),
    blocked_fixture("activation-result-receipt-activation-memory-provider-observability"; "blocked_observability_noop"; "activation_memory_provider_observability_denied"; {observability_requested:true, activation_from_observability_requested:true, memory_write_observability_requested:true, rollback_observability_requested:true, secret_material_observability_requested:true, provider_prompt_observability_requested:true}),
    blocked_fixture("activation-result-receipt-external-public-install-observability"; "blocked_observability_noop"; "external_public_install_restart_active_binary_observability_denied"; {observability_requested:true, ledger_observability_requested:true, index_observability_requested:true, delivery_observability_requested:true, external_send_observability_requested:true, public_claim_observability_requested:true, release_artifact_observability_requested:true, install_observability_requested:true, service_restart_observability_requested:true, active_binary_observability_requested:true})
  ] as $fixtures
  | {
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_export_query_observability_mode:"memory_write_execution_activation_command_result_receipt_export_query_observability_denial",
    source_activation_command_result_receipt_retention_expiry_garbage_collection_gate:$source.gate,
    source_activation_command_result_receipt_retention_expiry_garbage_collection_ready:$source.memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready,
    source_activation_command_result_receipt_retention_expiry_garbage_collection_report_sha256:$retention_gc_report_sha256,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_ready:$source.memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256:$source.source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256,
    source_activation_command_result_receipt_cancellation_supersession_ready:$source.memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready,
    source_activation_command_result_receipt_cancellation_supersession_report_sha256:$source.source_activation_command_result_receipt_cancellation_supersession_report_sha256,
    source_activation_command_result_receipt_ordering_monotonicity_ready:$source.memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready,
    source_activation_command_result_receipt_ordering_monotonicity_report_sha256:$source.source_activation_command_result_receipt_ordering_monotonicity_report_sha256,
    source_activation_command_result_receipt_replay_idempotency_ready:$source.memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready,
    source_activation_command_result_receipt_replay_idempotency_report_sha256:$source.source_activation_command_result_receipt_replay_idempotency_report_sha256,
    source_activation_command_result_receipt_no_persistence_ready:$source.memory_write_execution_activation_command_result_receipt_no_persistence_ready,
    source_activation_command_result_receipt_no_persistence_report_sha256:$source.source_activation_command_result_receipt_no_persistence_report_sha256,
    source_activation_command_noop_handoff_ready:$source.memory_write_execution_activation_command_noop_handoff_ready,
    source_activation_command_noop_handoff_report_sha256:$source.source_activation_command_noop_handoff_report_sha256,
    source_memory_write_execution_preflight_report_sha256:$source.source_memory_write_execution_preflight_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_no_persistence_ready:true,
    memory_write_execution_activation_command_noop_handoff_ready:true,
    required_activation_command_result_receipt_export_query_observability_surface_count:12,
    ready_activation_command_result_receipt_export_query_observability_surface_count:12,
    side_effect_free_activation_command_result_receipt_export_query_observability_surface_count:12,
    required_activation_command_result_receipt_export_query_observability_fixture_count:10,
    activation_command_result_receipt_export_query_observability_fixture_count:($fixtures | length),
    blocked_activation_command_result_receipt_export_query_observability_fixture_count:($fixtures | length),
    noop_activation_command_result_receipt_export_query_observability_fixture_count:($fixtures | length),
    allowed_activation_command_result_receipt_export_query_observability_fixture_count:0,
    accepted_activation_command_result_receipt_export_query_observability_fixture_count:0,
    activation_command_result_receipt_export_denied_count:10,
    activation_command_result_receipt_query_denied_count:10,
    activation_command_result_receipt_observability_denied_count:10,
    activation_command_result_receipt_export_performed_count:0,
    activation_command_result_receipt_query_performed_count:0,
    activation_command_result_receipt_observability_performed_count:0,
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
    activation_command_result_receipt_export_query_observability_surfaces:[
      "source_retention_expiry_garbage_collection_report_required",
      "export_request_shape_denied",
      "export_artifact_write_denied",
      "query_endpoint_materialization_denied",
      "query_index_cache_recording_denied",
      "observability_metric_emission_denied",
      "trace_span_log_event_recording_denied",
      "dashboard_alert_slo_materialization_denied",
      "ledger_index_delivery_observability_evidence_denied",
      "activation_from_export_query_observability_denied",
      "memory_write_rollback_secret_provider_observability_denied",
      "external_public_install_restart_active_binary_observability_denied"
    ],
    activation_command_result_receipt_export_query_observability_fixtures:$fixtures,
    denied_by_activation_command_result_receipt_export_query_observability:[
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
      "memory_write_observability_denied",
      "rollback_observability_denied",
      "secret_material_observability_denied",
      "provider_prompt_observability_denied",
      "external_public_install_restart_active_binary_observability_denied"
    ],
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
  }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready == true
  and .memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
  and .required_activation_command_result_receipt_export_query_observability_surface_count == 12
  and .ready_activation_command_result_receipt_export_query_observability_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_export_query_observability_surface_count == 12
  and .required_activation_command_result_receipt_export_query_observability_fixture_count == 10
  and .activation_command_result_receipt_export_query_observability_fixture_count == 10
  and .blocked_activation_command_result_receipt_export_query_observability_fixture_count == 10
  and .noop_activation_command_result_receipt_export_query_observability_fixture_count == 10
  and .allowed_activation_command_result_receipt_export_query_observability_fixture_count == 0
  and .accepted_activation_command_result_receipt_export_query_observability_fixture_count == 0
  and .activation_command_result_receipt_export_performed_count == 0
  and .activation_command_result_receipt_query_performed_count == 0
  and .activation_command_result_receipt_observability_performed_count == 0
  and .activation_command_result_receipt_export_artifact_written == false
  and .activation_command_result_receipt_export_stream_opened == false
  and .activation_command_result_receipt_query_endpoint_materialized == false
  and .activation_command_result_receipt_query_index_recorded == false
  and .activation_command_result_receipt_query_cache_written == false
  and .activation_command_result_receipt_observability_metric_emitted == false
  and .activation_command_result_receipt_observability_log_recorded == false
  and .activation_command_result_receipt_observability_trace_recorded == false
  and .activation_command_result_receipt_observability_span_recorded == false
  and .activation_command_result_receipt_observability_event_recorded == false
  and .activation_command_result_receipt_observability_dashboard_materialized == false
  and .activation_command_result_receipt_observability_alert_registered == false
  and .activation_allowed_by_result_receipt_export == false
  and .activation_allowed_by_result_receipt_query == false
  and .activation_allowed_by_result_receipt_observability == false
  and .activation_allowed_by_result_receipt == false
  and .activation_allowed == false
  and .activation_performed == false
  and .live_mutation_execution_performed == false
  and .memory_write_execution_performed == false
  and .memory_store_write_performed == false
  and .memory_store_write_performed_count == 0
  and .memory_store_mutated == false
  and .rollback_executed == false
  and .secret_material_read == false
  and .provider_invoked == false
  and .model_invoked == false
  and .external_send_performed == false
  and .public_release_published == false
  and .release_artifact_written == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.activation_command_result_receipt_export_query_observability_fixtures | length) == 10
  and (.activation_command_result_receipt_export_query_observability_fixtures | all((.export_query_observability_status == "blocked_noop" or .export_query_observability_status == "blocked_export_noop" or .export_query_observability_status == "blocked_query_noop" or .export_query_observability_status == "blocked_observability_noop") and .export_recorded == false and .export_artifact_written == false and .query_endpoint_materialized == false and .query_index_recorded == false and .observability_metric_emitted == false and .observability_trace_recorded == false and .observability_span_recorded == false and .observability_log_recorded == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory live mutation operator write execution activation command result receipt export/query/observability denial gate passed"
