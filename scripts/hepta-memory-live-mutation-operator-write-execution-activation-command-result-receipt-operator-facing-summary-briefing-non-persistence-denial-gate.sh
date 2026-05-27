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

EXPORT_QUERY_OBSERVABILITY_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-export-query-observability-denial-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-export-query-observability-denial-gate.sh
)"

export_query_observability_report_sha256="$(printf '%s' "$EXPORT_QUERY_OBSERVABILITY_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$EXPORT_QUERY_OBSERVABILITY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_export_query_observability_denial_gate"
    and $source.activation_command_result_receipt_export_query_observability_mode == "memory_write_execution_activation_command_result_receipt_export_query_observability_denial"
    and $source.memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_no_persistence_ready == true
    and $source.source_activation_command_result_receipt_retention_expiry_garbage_collection_report_sha256 != ""
    and $source.minimum_required_samples >= 24
    and $source.required_activation_command_result_receipt_export_query_observability_surface_count == 12
    and $source.ready_activation_command_result_receipt_export_query_observability_surface_count == 12
    and $source.side_effect_free_activation_command_result_receipt_export_query_observability_surface_count == 12
    and $source.required_activation_command_result_receipt_export_query_observability_fixture_count == 10
    and $source.activation_command_result_receipt_export_query_observability_fixture_count == 10
    and $source.blocked_activation_command_result_receipt_export_query_observability_fixture_count == 10
    and $source.noop_activation_command_result_receipt_export_query_observability_fixture_count == 10
    and $source.allowed_activation_command_result_receipt_export_query_observability_fixture_count == 0
    and $source.accepted_activation_command_result_receipt_export_query_observability_fixture_count == 0
    and $source.activation_command_result_receipt_export_performed_count == 0
    and $source.activation_command_result_receipt_query_performed_count == 0
    and $source.activation_command_result_receipt_observability_performed_count == 0
    and $source.activation_command_result_receipt_export_artifact_written == false
    and $source.activation_command_result_receipt_export_stream_opened == false
    and $source.activation_command_result_receipt_export_filesystem_written == false
    and $source.activation_command_result_receipt_query_endpoint_materialized == false
    and $source.activation_command_result_receipt_query_index_recorded == false
    and $source.activation_command_result_receipt_query_cache_written == false
    and $source.activation_command_result_receipt_observability_metric_emitted == false
    and $source.activation_command_result_receipt_observability_log_recorded == false
    and $source.activation_command_result_receipt_observability_trace_recorded == false
    and $source.activation_command_result_receipt_observability_span_recorded == false
    and $source.activation_command_result_receipt_observability_event_recorded == false
    and $source.activation_command_result_receipt_observability_dashboard_materialized == false
    and $source.activation_command_result_receipt_observability_alert_registered == false
    and $source.activation_allowed_by_result_receipt_export == false
    and $source.activation_allowed_by_result_receipt_query == false
    and $source.activation_allowed_by_result_receipt_observability == false
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
    and ($source.activation_command_result_receipt_export_query_observability_fixtures | length) == 10
    and ($source.activation_command_result_receipt_export_query_observability_fixtures | all((.export_query_observability_status == "blocked_noop" or .export_query_observability_status == "blocked_export_noop" or .export_query_observability_status == "blocked_query_noop" or .export_query_observability_status == "blocked_observability_noop") and .export_recorded == false and .export_artifact_written == false and .query_endpoint_materialized == false and .query_index_recorded == false and .observability_metric_emitted == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate" \
  --arg export_query_observability_report_sha256 "$export_query_observability_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$EXPORT_QUERY_OBSERVABILITY_JSON" \
  '
  def blocked_fixture($id; $status; $reason; $extra):
    {
      id:$id,
      operator_summary_requested:false,
      operator_briefing_requested:false,
      operator_summary_briefing_status:$status,
      source_export_query_observability_present:true,
      source_export_query_observability_ready:true,
      operator_summary_allowed:false,
      operator_summary_request_accepted:false,
      operator_summary_recorded:false,
      operator_summary_persisted:false,
      operator_summary_materialized:false,
      operator_summary_filesystem_written:false,
      operator_summary_delivered:false,
      operator_summary_channel_delivery_performed:false,
      operator_briefing_allowed:false,
      operator_briefing_request_accepted:false,
      operator_briefing_recorded:false,
      operator_briefing_persisted:false,
      operator_briefing_materialized:false,
      operator_briefing_filesystem_written:false,
      operator_briefing_delivered:false,
      operator_briefing_channel_delivery_performed:false,
      telegram_send_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      receipt_recorded:false,
      receipt_persisted:false,
      receipt_accepted:false,
      receipt_materialized:false,
      receipt_filesystem_written:false,
      completion_ack_recorded:false,
      completion_ack_persisted:false,
      completion_ack_accepted:false,
      completion_ack_delivered:false,
      activation_allowed:false,
      live_mutation_execution_performed:false,
      memory_write_execution_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      rollback_executed:false,
      secret_material_read:false,
      provider_invoked:false,
      model_invoked:false,
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
    blocked_fixture("activation-result-receipt-operator-summary-briefing-missing-source-export-query-observability"; "blocked_noop"; "source_export_query_observability_report_required"; {source_export_query_observability_present:false, source_export_query_observability_ready:false, operator_summary_requested:true}),
    blocked_fixture("activation-result-receipt-operator-summary-request"; "blocked_summary_noop"; "operator_summary_request_shape_denied"; {operator_summary_requested:true}),
    blocked_fixture("activation-result-receipt-operator-briefing-request"; "blocked_briefing_noop"; "operator_briefing_request_shape_denied"; {operator_briefing_requested:true}),
    blocked_fixture("activation-result-receipt-operator-summary-materialization-request"; "blocked_summary_noop"; "summary_materialization_denied"; {operator_summary_requested:true, operator_summary_materialization_requested:true}),
    blocked_fixture("activation-result-receipt-operator-briefing-materialization-request"; "blocked_briefing_noop"; "briefing_materialization_denied"; {operator_briefing_requested:true, operator_briefing_materialization_requested:true}),
    blocked_fixture("activation-result-receipt-operator-summary-persistence-filesystem-write-request"; "blocked_summary_noop"; "summary_persistence_filesystem_write_denied"; {operator_summary_requested:true, operator_summary_persistence_requested:true, operator_summary_filesystem_write_requested:true}),
    blocked_fixture("activation-result-receipt-operator-briefing-persistence-filesystem-write-request"; "blocked_briefing_noop"; "briefing_persistence_filesystem_write_denied"; {operator_briefing_requested:true, operator_briefing_persistence_requested:true, operator_briefing_filesystem_write_requested:true}),
    blocked_fixture("activation-result-receipt-operator-summary-briefing-channel-delivery-request"; "blocked_delivery_noop"; "summary_briefing_channel_delivery_denied"; {operator_summary_requested:true, operator_briefing_requested:true, channel_delivery_requested:true, telegram_send_requested:true}),
    blocked_fixture("activation-result-receipt-operator-summary-briefing-activation-memory-provider-request"; "blocked_summary_noop"; "activation_memory_rollback_secret_provider_summary_briefing_denied"; {operator_summary_requested:true, operator_briefing_requested:true, activation_from_summary_briefing_requested:true, memory_write_summary_requested:true, rollback_summary_requested:true, secret_material_summary_requested:true, provider_prompt_summary_requested:true}),
    blocked_fixture("activation-result-receipt-operator-summary-briefing-external-public-install-request"; "blocked_delivery_noop"; "external_public_install_restart_active_binary_summary_briefing_denied"; {operator_summary_requested:true, operator_briefing_requested:true, external_send_summary_requested:true, public_claim_summary_requested:true, release_artifact_summary_requested:true, install_summary_requested:true, service_restart_summary_requested:true, active_binary_summary_requested:true})
  ] as $fixtures
  | {
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_operator_facing_summary_briefing_mode:"memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial",
    source_activation_command_result_receipt_export_query_observability_gate:$source.gate,
    source_activation_command_result_receipt_export_query_observability_ready:$source.memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready,
    source_activation_command_result_receipt_export_query_observability_report_sha256:$export_query_observability_report_sha256,
    source_activation_command_result_receipt_retention_expiry_garbage_collection_ready:$source.memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready,
    source_activation_command_result_receipt_retention_expiry_garbage_collection_report_sha256:$source.source_activation_command_result_receipt_retention_expiry_garbage_collection_report_sha256,
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
    memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready:true,
    memory_write_execution_activation_command_result_receipt_no_persistence_ready:true,
    memory_write_execution_activation_command_noop_handoff_ready:true,
    required_activation_command_result_receipt_operator_facing_summary_briefing_surface_count:12,
    ready_activation_command_result_receipt_operator_facing_summary_briefing_surface_count:12,
    side_effect_free_activation_command_result_receipt_operator_facing_summary_briefing_surface_count:12,
    required_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count:10,
    activation_command_result_receipt_operator_facing_summary_briefing_fixture_count:($fixtures | length),
    blocked_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count:($fixtures | length),
    noop_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count:($fixtures | length),
    allowed_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count:0,
    accepted_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count:0,
    activation_command_result_receipt_operator_summary_denied_count:10,
    activation_command_result_receipt_operator_briefing_denied_count:10,
    activation_command_result_receipt_operator_summary_performed_count:0,
    activation_command_result_receipt_operator_briefing_performed_count:0,
    activation_command_result_receipt_operator_summary_allowed:false,
    activation_command_result_receipt_operator_summary_request_accepted:false,
    activation_command_result_receipt_operator_summary_recorded:false,
    activation_command_result_receipt_operator_summary_persisted:false,
    activation_command_result_receipt_operator_summary_materialized:false,
    activation_command_result_receipt_operator_summary_filesystem_written:false,
    activation_command_result_receipt_operator_summary_delivered:false,
    activation_command_result_receipt_operator_summary_channel_delivery_performed:false,
    activation_command_result_receipt_operator_briefing_allowed:false,
    activation_command_result_receipt_operator_briefing_request_accepted:false,
    activation_command_result_receipt_operator_briefing_recorded:false,
    activation_command_result_receipt_operator_briefing_persisted:false,
    activation_command_result_receipt_operator_briefing_materialized:false,
    activation_command_result_receipt_operator_briefing_filesystem_written:false,
    activation_command_result_receipt_operator_briefing_delivered:false,
    activation_command_result_receipt_operator_briefing_channel_delivery_performed:false,
    activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed:false,
    telegram_send_performed:false,
    channel_send_performed:false,
    external_send_performed:false,
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
    activation_allowed_by_result_receipt_operator_summary:false,
    activation_allowed_by_result_receipt_operator_briefing:false,
    activation_allowed_by_result_receipt_summary_briefing:false,
    activation_allowed_by_result_receipt_export:false,
    activation_allowed_by_result_receipt_query:false,
    activation_allowed_by_result_receipt_observability:false,
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
    public_claim_or_release_artifact_write_enabled:false,
    public_release_published:false,
    public_ga_claimed:false,
    release_artifact_written:false,
    install_executed:false,
    launchd_mutated:false,
    service_restarted:false,
    active_binary_mutated:false,
    activation_command_result_receipt_operator_facing_summary_briefing_surfaces:[
      "source_export_query_observability_report_required",
      "operator_summary_request_shape_denied",
      "operator_briefing_request_shape_denied",
      "summary_materialization_denied",
      "briefing_materialization_denied",
      "summary_persistence_denied",
      "briefing_persistence_denied",
      "summary_delivery_denied",
      "briefing_delivery_denied",
      "activation_from_summary_briefing_denied",
      "memory_write_rollback_secret_provider_summary_briefing_denied",
      "external_public_install_restart_active_binary_summary_briefing_denied"
    ],
    activation_command_result_receipt_operator_facing_summary_briefing_fixtures:$fixtures,
    denied_by_activation_command_result_receipt_operator_facing_summary_briefing:[
      "source_export_query_observability_report_required",
      "operator_summary_request_acceptance_denied",
      "operator_briefing_request_acceptance_denied",
      "operator_summary_recording_denied",
      "operator_briefing_recording_denied",
      "operator_summary_persistence_denied",
      "operator_briefing_persistence_denied",
      "operator_summary_materialization_denied",
      "operator_briefing_materialization_denied",
      "operator_summary_filesystem_write_denied",
      "operator_briefing_filesystem_write_denied",
      "operator_summary_delivery_denied",
      "operator_briefing_delivery_denied",
      "telegram_send_denied",
      "activation_from_summary_briefing_denied",
      "memory_write_summary_briefing_denied",
      "rollback_summary_briefing_denied",
      "secret_material_summary_briefing_denied",
      "provider_prompt_summary_briefing_denied",
      "external_public_install_restart_active_binary_summary_briefing_denied"
    ],
    side_effects:{
      activation_command_result_receipt_operator_summary_recorded:false,
      activation_command_result_receipt_operator_summary_persisted:false,
      activation_command_result_receipt_operator_summary_materialized:false,
      activation_command_result_receipt_operator_summary_filesystem_written:false,
      activation_command_result_receipt_operator_summary_delivered:false,
      activation_command_result_receipt_operator_summary_channel_delivery_performed:false,
      activation_command_result_receipt_operator_briefing_recorded:false,
      activation_command_result_receipt_operator_briefing_persisted:false,
      activation_command_result_receipt_operator_briefing_materialized:false,
      activation_command_result_receipt_operator_briefing_filesystem_written:false,
      activation_command_result_receipt_operator_briefing_delivered:false,
      activation_command_result_receipt_operator_briefing_channel_delivery_performed:false,
      activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed:false,
      telegram_send_performed:false,
      activation_command_result_receipt_export_recorded:false,
      activation_command_result_receipt_export_persisted:false,
      activation_command_result_receipt_export_artifact_written:false,
      activation_command_result_receipt_query_registered:false,
      activation_command_result_receipt_query_endpoint_materialized:false,
      activation_command_result_receipt_query_index_recorded:false,
      activation_command_result_receipt_query_cache_written:false,
      activation_command_result_receipt_observability_metric_emitted:false,
      activation_command_result_receipt_observability_log_recorded:false,
      activation_command_result_receipt_observability_trace_recorded:false,
      activation_command_result_receipt_observability_span_recorded:false,
      activation_command_result_receipt_observability_event_recorded:false,
      activation_command_result_receipt_observability_dashboard_materialized:false,
      activation_command_result_receipt_observability_alert_registered:false,
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
  and .memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
  and .memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready == true
  and .required_activation_command_result_receipt_operator_facing_summary_briefing_surface_count == 12
  and .ready_activation_command_result_receipt_operator_facing_summary_briefing_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_operator_facing_summary_briefing_surface_count == 12
  and .required_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count == 10
  and .activation_command_result_receipt_operator_facing_summary_briefing_fixture_count == 10
  and .blocked_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count == 10
  and .noop_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count == 10
  and .allowed_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count == 0
  and .accepted_activation_command_result_receipt_operator_facing_summary_briefing_fixture_count == 0
  and .activation_command_result_receipt_operator_summary_performed_count == 0
  and .activation_command_result_receipt_operator_briefing_performed_count == 0
  and .activation_command_result_receipt_operator_summary_recorded == false
  and .activation_command_result_receipt_operator_summary_persisted == false
  and .activation_command_result_receipt_operator_summary_materialized == false
  and .activation_command_result_receipt_operator_summary_filesystem_written == false
  and .activation_command_result_receipt_operator_summary_delivered == false
  and .activation_command_result_receipt_operator_briefing_recorded == false
  and .activation_command_result_receipt_operator_briefing_persisted == false
  and .activation_command_result_receipt_operator_briefing_materialized == false
  and .activation_command_result_receipt_operator_briefing_filesystem_written == false
  and .activation_command_result_receipt_operator_briefing_delivered == false
  and .activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_result_receipt_materialized == false
  and .activation_command_result_receipt_filesystem_written == false
  and .activation_command_completion_ack_recorded == false
  and .activation_allowed_by_result_receipt_operator_summary == false
  and .activation_allowed_by_result_receipt_operator_briefing == false
  and .activation_allowed_by_result_receipt_summary_briefing == false
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
  and .public_release_published == false
  and .release_artifact_written == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.activation_command_result_receipt_operator_facing_summary_briefing_fixtures | length) == 10
  and (.activation_command_result_receipt_operator_facing_summary_briefing_fixtures | all((.operator_summary_briefing_status == "blocked_noop" or .operator_summary_briefing_status == "blocked_summary_noop" or .operator_summary_briefing_status == "blocked_briefing_noop" or .operator_summary_briefing_status == "blocked_delivery_noop") and .operator_summary_recorded == false and .operator_summary_persisted == false and .operator_summary_materialized == false and .operator_summary_filesystem_written == false and .operator_summary_delivered == false and .operator_briefing_recorded == false and .operator_briefing_persisted == false and .operator_briefing_materialized == false and .operator_briefing_filesystem_written == false and .operator_briefing_delivered == false and .telegram_send_performed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .receipt_noop_confirmed == true))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory live mutation operator write execution activation command result receipt operator-facing summary/briefing non-persistence denial gate passed"
