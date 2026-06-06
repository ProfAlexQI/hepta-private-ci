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

EXPORT_QUERY_OBSERVABILITY_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial-gate.sh
)"

operator_summary_briefing_fixtures_json="$(
  jq -n '
    def operator_summary_briefing_fixture($id; $status; $reason; $extra):
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
        activation_command_result_receipt_export_recorded:false,
        activation_command_result_receipt_export_persisted:false,
        activation_command_result_receipt_export_artifact_written:false,
        activation_command_result_receipt_export_stream_opened:false,
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
        public_release_claimed:false,
        public_ga_claimed:false,
        release_artifact_written:false,
        install_executed:false,
        launchd_mutated:false,
        service_restart_performed:false,
        active_binary_mutated:false,
        summary_briefing_noop_confirmed:true,
        reason:$reason
      } + $extra;
    [
      operator_summary_briefing_fixture("operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-summary-missing-source-export-query-observability"; "blocked_noop"; "source_export_query_observability_report_required"; {source_export_query_observability_present:false, source_export_query_observability_ready:false, operator_summary_requested:true}),
      operator_summary_briefing_fixture("operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-summary-request"; "blocked_summary_noop"; "operator_summary_request_shape_denied"; {operator_summary_requested:true}),
      operator_summary_briefing_fixture("operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-briefing-request"; "blocked_briefing_noop"; "operator_briefing_request_shape_denied"; {operator_briefing_requested:true}),
      operator_summary_briefing_fixture("operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-summary-materialization-request"; "blocked_summary_noop"; "operator_summary_materialization_denied"; {operator_summary_requested:true, operator_summary_materialization_requested:true}),
      operator_summary_briefing_fixture("operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-briefing-materialization-request"; "blocked_briefing_noop"; "operator_briefing_materialization_denied"; {operator_briefing_requested:true, operator_briefing_materialization_requested:true}),
      operator_summary_briefing_fixture("operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-summary-persistence-filesystem-request"; "blocked_summary_noop"; "operator_summary_persistence_filesystem_write_denied"; {operator_summary_requested:true, operator_summary_persistence_requested:true, operator_summary_filesystem_write_requested:true}),
      operator_summary_briefing_fixture("operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-briefing-persistence-filesystem-request"; "blocked_briefing_noop"; "operator_briefing_persistence_filesystem_write_denied"; {operator_briefing_requested:true, operator_briefing_persistence_requested:true, operator_briefing_filesystem_write_requested:true}),
      operator_summary_briefing_fixture("operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-summary-briefing-channel-delivery-request"; "blocked_delivery_noop"; "operator_summary_briefing_channel_delivery_denied"; {operator_summary_requested:true, operator_briefing_requested:true, channel_delivery_requested:true, telegram_send_requested:true}),
      operator_summary_briefing_fixture("operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-summary-briefing-activation-memory-kg-provider"; "blocked_summary_noop"; "activation_memory_kg_rollback_secret_provider_summary_briefing_denied"; {operator_summary_requested:true, operator_briefing_requested:true, activation_from_summary_briefing_requested:true, memory_store_summary_requested:true, live_kg_summary_requested:true, rollback_summary_requested:true, secret_material_summary_requested:true, provider_prompt_summary_requested:true}),
      operator_summary_briefing_fixture("operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-summary-briefing-external-public-install"; "blocked_delivery_noop"; "external_public_install_restart_active_binary_summary_briefing_denied"; {operator_summary_requested:true, operator_briefing_requested:true, external_send_summary_requested:true, public_claim_summary_requested:true, release_artifact_summary_requested:true, install_summary_requested:true, service_restart_summary_requested:true, active_binary_summary_requested:true})
    ]
  '
)"

export_query_observability_report_sha256="$(sha256_text "$EXPORT_QUERY_OBSERVABILITY_JSON")"
operator_summary_briefing_fixtures_sha256="$(sha256_text "$operator_summary_briefing_fixtures_json")"
operator_summary_briefing_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial:$export_query_observability_report_sha256:$operator_summary_briefing_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
operator_summary_briefing_policy_hash_sha256="$(
  sha256_text "operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence:no-summary:no-briefing:no-delivery:no-receipt:no-runtime:no-provider:no-model:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "summary=false;briefing=false;delivery=false;receipt=false;activation=false;runtime=false;provider=false;model=false;memory=false;kg=false;secret=false;external=false;install=false;restart=false;active_binary=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$EXPORT_QUERY_OBSERVABILITY_JSON" \
  --argjson fixtures "$operator_summary_briefing_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_gate"
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_schema_version == "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_v1"
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_status == "blocked"
    and $source.source_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_status == "blocked"
    and $source.source_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_report_sha256 != ""
    and $source.minimum_required_samples >= 24
    and $source.export_query_observability_surface_count == 12
    and $source.export_query_observability_surface_ready_count == 12
    and $source.export_query_observability_side_effect_free_surface_count == 12
    and $source.export_query_observability_fixture_count == 10
    and $source.blocked_export_query_observability_fixture_count == 10
    and $source.noop_export_query_observability_fixture_count == 10
    and $source.allowed_export_query_observability_fixture_count == 0
    and $source.accepted_export_query_observability_fixture_count == 0
    and $source.export_performed_count == 0
    and $source.query_performed_count == 0
    and $source.observability_performed_count == 0
    and $source.activation_command_result_receipt_export_recorded == false
    and $source.activation_command_result_receipt_export_persisted == false
    and $source.activation_command_result_receipt_export_artifact_written == false
    and $source.activation_command_result_receipt_export_stream_opened == false
    and $source.activation_command_result_receipt_query_registered == false
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
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_result_receipt_materialized == false
    and $source.activation_command_result_receipt_filesystem_written == false
    and $source.activation_command_completion_ack_recorded == false
    and $source.activation_allowed_by_result_receipt_export == false
    and $source.activation_allowed_by_result_receipt_query == false
    and $source.activation_allowed_by_result_receipt_observability == false
    and $source.activation_allowed_by_result_receipt == false
    and $source.activation_command_invoked == false
    and $source.activation_command_dispatched == false
    and ($source.activation_activated // false) == false
    and ($source.runtime_router_mutated // false) == false
    and ($source.runtime_attachment_performed // false) == false
    and ($source.live_context_attached // false) == false
    and ($source.context_injection_performed // false) == false
    and ($source.adapter_invoked // false) == false
    and ($source.provider_invoked // false) == false
    and ($source.model_invoked // false) == false
    and ($source.auth_secret_read // false) == false
    and ($source.credential_read // false) == false
    and ($source.secret_file_read // false) == false
    and ($source.usage_recorded // false) == false
    and ($source.memory_store_write_performed // false) == false
    and ($source.memory_store_mutated // false) == false
    and ($source.live_kg_write_performed // false) == false
    and ($source.rollback_executed // false) == false
    and ($source.telegram_send_performed // false) == false
    and ($source.channel_send_performed // false) == false
    and ($source.external_send_performed // false) == false
    and ($source.install_executed // false) == false
    and ($source.service_restart_performed // false) == false
    and ($source.active_binary_mutated // false) == false
    and ($source.allowed_next_actions | any(.action == "stage_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial" and .status == "allowed_report_only_next_slice" and .persists_summary == false and .persists_briefing == false and .delivers_summary == false and .mutates_runtime == false and .invokes_model == false and .writes_memory_or_kg == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.operator_summary_briefing_status == "blocked_noop" or .operator_summary_briefing_status == "blocked_summary_noop" or .operator_summary_briefing_status == "blocked_briefing_noop" or .operator_summary_briefing_status == "blocked_delivery_noop")
      and .operator_summary_recorded == false
      and .operator_summary_persisted == false
      and .operator_summary_materialized == false
      and .operator_summary_filesystem_written == false
      and .operator_summary_delivered == false
      and .operator_briefing_recorded == false
      and .operator_briefing_persisted == false
      and .operator_briefing_materialized == false
      and .operator_briefing_filesystem_written == false
      and .operator_briefing_delivered == false
      and .telegram_send_performed == false
      and .channel_send_performed == false
      and .external_send_performed == false
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
      and .summary_briefing_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate" \
  --arg export_query_observability_report_sha256 "$export_query_observability_report_sha256" \
  --arg operator_summary_briefing_fixtures_sha256 "$operator_summary_briefing_fixtures_sha256" \
  --arg operator_summary_briefing_contract_hash_sha256 "$operator_summary_briefing_contract_hash_sha256" \
  --arg operator_summary_briefing_policy_hash_sha256 "$operator_summary_briefing_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$EXPORT_QUERY_OBSERVABILITY_JSON" \
  --argjson fixtures "$operator_summary_briefing_fixtures_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_operator_facing_summary_briefing_schema_version:"memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_v1",
    activation_command_result_receipt_operator_facing_summary_briefing_mode:"operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_no_summary_no_briefing_no_delivery",
    source_activation_command_result_receipt_export_query_observability_gate:$source.gate,
    source_activation_command_result_receipt_export_query_observability_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready,
    source_activation_command_result_receipt_export_query_observability_status:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_status,
    source_activation_command_result_receipt_export_query_observability_report_sha256:$export_query_observability_report_sha256,
    source_activation_command_result_receipt_retention_expiry_garbage_collection_ready:($source.source_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_status == "blocked"),
    source_activation_command_result_receipt_retention_expiry_garbage_collection_report_sha256:$source.source_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_report_sha256,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_ready:true,
    source_activation_command_result_receipt_audit_trail_immutable_evidence_report_sha256:null,
    source_activation_command_result_receipt_cancellation_supersession_ready:true,
    source_activation_command_result_receipt_cancellation_supersession_report_sha256:null,
    source_activation_command_result_receipt_ordering_monotonicity_ready:true,
    source_activation_command_result_receipt_ordering_monotonicity_report_sha256:null,
    source_activation_command_result_receipt_replay_idempotency_ready:true,
    source_activation_command_result_receipt_replay_idempotency_report_sha256:null,
    source_activation_command_result_receipt_no_persistence_ready:true,
    source_activation_command_result_receipt_no_persistence_report_sha256:null,
    operator_summary_briefing_fixtures_sha256:$operator_summary_briefing_fixtures_sha256,
    operator_summary_briefing_contract_hash_sha256:$operator_summary_briefing_contract_hash_sha256,
    operator_summary_briefing_policy_hash_sha256:$operator_summary_briefing_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready:true,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status:"blocked",
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready:($source.source_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_status == "blocked"),
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready:true,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready:true,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready:true,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready:true,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_ready:true,
    export_query_observability_surface_count:$source.export_query_observability_surface_count,
    export_query_observability_fixture_count:$source.export_query_observability_fixture_count,
    operator_facing_summary_briefing_surface_count:12,
    operator_facing_summary_briefing_surface_ready_count:12,
    operator_facing_summary_briefing_side_effect_free_surface_count:12,
    operator_facing_summary_briefing_fixture_count:($fixtures | length),
    blocked_operator_facing_summary_briefing_fixture_count:($fixtures | length),
    noop_operator_facing_summary_briefing_fixture_count:($fixtures | length),
    allowed_operator_facing_summary_briefing_fixture_count:0,
    accepted_operator_facing_summary_briefing_fixture_count:0,
    operator_summary_denied_count:($fixtures | length),
    operator_briefing_denied_count:($fixtures | length),
    operator_summary_performed_count:0,
    operator_briefing_performed_count:0,
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
    activation_command_result_receipt_export_recorded:false,
    activation_command_result_receipt_export_persisted:false,
    activation_command_result_receipt_export_artifact_written:false,
    activation_command_result_receipt_export_stream_opened:false,
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
    activation_allowed_by_result_receipt_operator_summary:false,
    activation_allowed_by_result_receipt_operator_briefing:false,
    activation_allowed_by_result_receipt_summary_briefing:false,
    activation_allowed_by_result_receipt_export:false,
    activation_allowed_by_result_receipt_query:false,
    activation_allowed_by_result_receipt_observability:false,
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
    public_release_claimed:false,
    public_ga_claimed:false,
    release_artifact_written:false,
    install_executed:false,
    launchd_mutated:false,
    service_restart_performed:false,
    active_binary_mutated:false,
    operator_facing_summary_briefing_surfaces:[
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
      "memory_kg_rollback_secret_provider_summary_briefing_denied",
      "external_public_install_restart_active_binary_summary_briefing_denied"
    ],
    operator_facing_summary_briefing_fixtures:$fixtures,
    denied_by_operator_facing_summary_briefing:[
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
      "channel_delivery_denied",
      "activation_from_summary_briefing_denied",
      "memory_kg_summary_briefing_denied",
      "rollback_summary_briefing_denied",
      "secret_material_summary_briefing_denied",
      "provider_prompt_summary_briefing_denied",
      "external_public_install_restart_active_binary_summary_briefing_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial",
        status:"allowed_report_only",
        persists_summary:false,
        persists_briefing:false,
        delivers_summary:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"stage_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial",
        status:"allowed_report_only_next_slice",
        accepts_operator_acknowledgement:false,
        persists_acknowledgement:false,
        activates_runtime:false,
        invokes_model:false,
        writes_kg:false
      },
      {
        action:"run_full_light_preflight",
        status:"allowed_verification_only",
        persists_summary:false,
        persists_briefing:false,
        delivers_summary:false,
        mutates_runtime:false,
        invokes_model:false,
        writes_kg:false
      }
    ],
    source_export_query_observability_report_required:true,
    operator_summary_acceptance_forbidden:true,
    operator_briefing_acceptance_forbidden:true,
    operator_summary_recording_forbidden:true,
    operator_briefing_recording_forbidden:true,
    operator_summary_persistence_forbidden:true,
    operator_briefing_persistence_forbidden:true,
    operator_summary_materialization_forbidden:true,
    operator_briefing_materialization_forbidden:true,
    operator_summary_filesystem_write_forbidden:true,
    operator_briefing_filesystem_write_forbidden:true,
    operator_summary_delivery_forbidden:true,
    operator_briefing_delivery_forbidden:true,
    telegram_send_forbidden:true,
    channel_delivery_forbidden:true,
    activation_from_summary_briefing_forbidden:true,
    runtime_provider_memory_kg_summary_briefing_forbidden:true,
    secret_read_forbidden:true,
    external_public_install_restart_active_binary_summary_briefing_forbidden:true,
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
      channel_send_performed:false,
      external_send_performed:false,
      activation_command_result_receipt_export_recorded:false,
      activation_command_result_receipt_export_persisted:false,
      activation_command_result_receipt_export_artifact_written:false,
      activation_command_result_receipt_export_stream_opened:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate"
  and .activation_command_result_receipt_operator_facing_summary_briefing_schema_version == "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_v1"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status == "blocked"
  and .source_activation_command_result_receipt_export_query_observability_ready == true
  and .source_activation_command_result_receipt_export_query_observability_status == "blocked"
  and .source_activation_command_result_receipt_export_query_observability_report_sha256 != ""
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_ready == true
  and .minimum_required_samples >= 24
  and .operator_facing_summary_briefing_surface_count == 12
  and .operator_facing_summary_briefing_surface_ready_count == 12
  and .operator_facing_summary_briefing_side_effect_free_surface_count == 12
  and .operator_facing_summary_briefing_fixture_count == 10
  and .blocked_operator_facing_summary_briefing_fixture_count == 10
  and .noop_operator_facing_summary_briefing_fixture_count == 10
  and .allowed_operator_facing_summary_briefing_fixture_count == 0
  and .accepted_operator_facing_summary_briefing_fixture_count == 0
  and .operator_summary_denied_count == 10
  and .operator_briefing_denied_count == 10
  and .operator_summary_performed_count == 0
  and .operator_briefing_performed_count == 0
  and .activation_command_result_receipt_operator_summary_allowed == false
  and .activation_command_result_receipt_operator_summary_request_accepted == false
  and .activation_command_result_receipt_operator_summary_recorded == false
  and .activation_command_result_receipt_operator_summary_persisted == false
  and .activation_command_result_receipt_operator_summary_materialized == false
  and .activation_command_result_receipt_operator_summary_filesystem_written == false
  and .activation_command_result_receipt_operator_summary_delivered == false
  and .activation_command_result_receipt_operator_briefing_allowed == false
  and .activation_command_result_receipt_operator_briefing_request_accepted == false
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
  and .activation_allowed_by_result_receipt_operator_summary == false
  and .activation_allowed_by_result_receipt_operator_briefing == false
  and .activation_allowed_by_result_receipt_summary_briefing == false
  and .activation_allowed_by_result_receipt == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
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
  and .public_release_claimed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.operator_facing_summary_briefing_surfaces | length) == 12
  and (.operator_facing_summary_briefing_fixtures | length) == 10
  and (.operator_facing_summary_briefing_fixtures | all(
    (.operator_summary_briefing_status == "blocked_noop" or .operator_summary_briefing_status == "blocked_summary_noop" or .operator_summary_briefing_status == "blocked_briefing_noop" or .operator_summary_briefing_status == "blocked_delivery_noop")
    and .operator_summary_recorded == false
    and .operator_summary_persisted == false
    and .operator_summary_materialized == false
    and .operator_summary_filesystem_written == false
    and .operator_summary_delivered == false
    and .operator_briefing_recorded == false
    and .operator_briefing_persisted == false
    and .operator_briefing_materialized == false
    and .operator_briefing_filesystem_written == false
    and .operator_briefing_delivered == false
    and .telegram_send_performed == false
    and .channel_send_performed == false
    and .external_send_performed == false
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
    and .summary_briefing_noop_confirmed == true
  ))
  and ([.operator_facing_summary_briefing_fixtures[] | select(.source_export_query_observability_present == false)] | length) == 1
  and ([.operator_facing_summary_briefing_fixtures[] | select(.operator_summary_requested == true)] | length) >= 7
  and ([.operator_facing_summary_briefing_fixtures[] | select(.operator_briefing_requested == true)] | length) >= 6
  and ([.operator_facing_summary_briefing_fixtures[] | select(.channel_delivery_requested == true and .telegram_send_requested == true)] | length) == 1
  and ([.operator_facing_summary_briefing_fixtures[] | select(.activation_from_summary_briefing_requested == true and .memory_store_summary_requested == true and .live_kg_summary_requested == true and .provider_prompt_summary_requested == true)] | length) == 1
  and ([.operator_facing_summary_briefing_fixtures[] | select(.external_send_summary_requested == true and .install_summary_requested == true and .active_binary_summary_requested == true)] | length) == 1
  and (.denied_by_operator_facing_summary_briefing | length) == 21
  and (.allowed_next_actions | any(.action == "stage_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial" and .status == "allowed_report_only_next_slice" and .accepts_operator_acknowledgement == false and .persists_acknowledgement == false and .activates_runtime == false and .invokes_model == false and .writes_kg == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt operator-facing summary/briefing non-persistence denial gate passed"
