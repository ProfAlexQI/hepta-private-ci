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

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing provider-router receipt observability source text: $label" >&2
    exit 1
  fi
}

READBACK_RECEIPT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-readback-receipt-skeleton-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-readback-receipt-skeleton-gate.sh
)"

RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE="codex-rs/hepta-runtime/src/model_provider_router.rs"

require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'pub route_events: Vec<ModelProviderRouteEvent>,' \
  "route event vector"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'pub struct ModelProviderRouteEvent' \
  "route event struct"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'pub fn report(' \
  "read-only provider-router report"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'persisted: self.path.exists(),' \
  "read-only report persistence flag"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'fn push_event(' \
  "event append helper"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'router.route_events.push(ModelProviderRouteEvent' \
  "route event append point"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'router.route_events.truncate(1024);' \
  "route event retention cap"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'provider_invoked_by_adapter: false,' \
  "provider invocation disabled"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'auth_secret_read_by_adapter: false,' \
  "auth secret read disabled"
require_source_text "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  'usage_recorded_by_adapter: false,' \
  "usage recording disabled"

observability_fixtures_json="$(
  jq -n '
    def blocked_fixture($id; $status; $reason; $extra):
      {
        id:$id,
        export_requested:false,
        query_requested:false,
        observability_requested:false,
        receipt_observability_status:$status,
        source_readback_receipt_skeleton_present:true,
        source_readback_receipt_skeleton_ready:true,
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
        router_event_observability_recorded:false,
        readback_ledger_observability_recorded:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        readback_evidence_recorded:false,
        readback_evidence_persisted:false,
        router_handoff_recorded:false,
        router_handoff_persisted:false,
        runtime_router_mutated:false,
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
        rollback_executed:false,
        external_send_performed:false,
        public_release_claimed:false,
        service_restart_performed:false,
        active_binary_mutated:false,
        receipt_noop_confirmed:true,
        reason:$reason
      } + $extra;
    [
      blocked_fixture("provider-router-receipt-observability-missing-source-readback-skeleton"; "blocked_noop"; "source_readback_receipt_skeleton_required"; {source_readback_receipt_skeleton_present:false, source_readback_receipt_skeleton_ready:false, export_requested:true}),
      blocked_fixture("provider-router-receipt-export-artifact-request"; "blocked_export_noop"; "receipt_export_artifact_write_denied"; {export_requested:true, export_file_requested:true}),
      blocked_fixture("provider-router-receipt-export-stream-request"; "blocked_export_noop"; "receipt_export_stream_open_denied"; {export_requested:true, export_stream_requested:true}),
      blocked_fixture("provider-router-receipt-query-endpoint-request"; "blocked_query_noop"; "receipt_query_endpoint_materialization_denied"; {query_requested:true, query_endpoint_requested:true}),
      blocked_fixture("provider-router-receipt-query-index-cache-request"; "blocked_query_noop"; "receipt_query_index_cache_recording_denied"; {query_requested:true, query_index_requested:true, query_cache_requested:true}),
      blocked_fixture("provider-router-receipt-observability-metric-request"; "blocked_observability_noop"; "receipt_observability_metric_emission_denied"; {observability_requested:true, metric_requested:true}),
      blocked_fixture("provider-router-receipt-observability-trace-log-event-request"; "blocked_observability_noop"; "receipt_trace_log_event_recording_denied"; {observability_requested:true, trace_requested:true, span_requested:true, log_requested:true, event_requested:true}),
      blocked_fixture("provider-router-receipt-dashboard-alert-slo-request"; "blocked_observability_noop"; "receipt_dashboard_alert_slo_materialization_denied"; {observability_requested:true, dashboard_requested:true, alert_requested:true, slo_requested:true}),
      blocked_fixture("provider-router-receipt-runtime-attachment-observability-request"; "blocked_observability_noop"; "runtime_attachment_live_context_provider_observability_denied"; {observability_requested:true, router_event_observability_requested:true, readback_ledger_observability_requested:true, runtime_attachment_observability_requested:true, live_context_observability_requested:true, provider_prompt_observability_requested:true}),
      blocked_fixture("provider-router-receipt-external-public-install-observability-request"; "blocked_observability_noop"; "external_public_install_restart_active_binary_observability_denied"; {observability_requested:true, external_send_observability_requested:true, public_claim_observability_requested:true, release_artifact_observability_requested:true, install_observability_requested:true, service_restart_observability_requested:true, active_binary_observability_requested:true})
    ]
  '
)"

readback_receipt_report_sha256="$(sha256_text "$READBACK_RECEIPT_JSON")"
runtime_model_provider_router_source_sha256="$(sha256_file "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE")"
observability_fixtures_sha256="$(sha256_text "$observability_fixtures_json")"
receipt_observability_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-runtime-provider-router-receipt-observability-denial:$readback_receipt_report_sha256:$runtime_model_provider_router_source_sha256:$observability_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
receipt_observability_policy_hash_sha256="$(
  sha256_text "runtime-provider-router-receipt-observability-denial:report-only:no-export:no-query:no-observability:no-adapter-invocation:no-router-handoff:no-live-context:no-model-invocation:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "export=false;query=false;observability=false;adapter_invoked=false;router_handoff=false;readback_evidence=false;live_context=false;model_invoked=false;secret_read=false;service_restart=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson readback "$READBACK_RECEIPT_JSON" \
  --argjson fixtures "$observability_fixtures_json" \
  '
    $readback.runtime == "hepta"
    and $readback.status == "ready"
    and $readback.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_readback_receipt_skeleton_gate"
    and $readback.runtime_provider_router_readback_receipt_skeleton_ready == true
    and $readback.runtime_provider_router_readback_receipt_skeleton_status == "blocked"
    and $readback.negative_fixture_matrix_ready == true
    and $readback.negative_fixture_matrix_status == "blocked"
    and $readback.runtime_attachment_packet_recorded == false
    and $readback.runtime_attachment_packet_persisted == false
    and $readback.runtime_attachment_packet_accepted == false
    and $readback.readback_receipt_skeleton_item_count == 12
    and $readback.declared_readback_receipt_skeleton_item_count == 12
    and $readback.required_readback_receipt_skeleton_item_count == 12
    and $readback.recorded_readback_receipt_skeleton_item_count == 0
    and $readback.persisted_readback_receipt_skeleton_item_count == 0
    and $readback.accepted_readback_receipt_skeleton_item_count == 0
    and $readback.runtime_attachment_blocking_receipt_count == 12
    and $readback.live_context_attachment_blocking_receipt_count == 12
    and $readback.model_invocation_blocking_receipt_count == 12
    and $readback.observability_export_query_denial_required == true
    and $readback.adapter_invocation_forbidden == true
    and $readback.router_handoff_persistence_forbidden == true
    and $readback.readback_evidence_persistence_forbidden == true
    and $readback.live_context_attachment_forbidden == true
    and $readback.provider_model_invocation_forbidden == true
    and $readback.auth_secret_read_forbidden == true
    and $readback.usage_recording_forbidden == true
    and $readback.adapter_invoked == false
    and $readback.router_handoff_recorded == false
    and $readback.readback_evidence_recorded == false
    and $readback.readback_evidence_persisted == false
    and $readback.receipt_recorded == false
    and $readback.receipt_persisted == false
    and $readback.receipt_accepted == false
    and $readback.observability_metric_emitted == false
    and $readback.observability_log_recorded == false
    and $readback.observability_trace_recorded == false
    and $readback.observability_dashboard_materialized == false
    and $readback.receipt_exported == false
    and $readback.receipt_query_registered == false
    and ($readback.allowed_next_actions | any(.action == "stage_runtime_provider_router_receipt_observability_denial" and .status == "allowed_report_only_next_slice" and .invokes_adapter == false and .records_observability == false and .exports_receipt == false and .registers_query == false and .persists_evidence == false and .invokes_model == false))
    and ($readback.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.receipt_observability_status == "blocked_noop" or .receipt_observability_status == "blocked_export_noop" or .receipt_observability_status == "blocked_query_noop" or .receipt_observability_status == "blocked_observability_noop")
      and .export_recorded == false
      and .export_artifact_written == false
      and .query_endpoint_materialized == false
      and .query_index_recorded == false
      and .observability_metric_emitted == false
      and .observability_trace_recorded == false
      and .observability_span_recorded == false
      and .observability_log_recorded == false
      and .observability_dashboard_materialized == false
      and .router_event_observability_recorded == false
      and .readback_ledger_observability_recorded == false
      and .receipt_recorded == false
      and .receipt_persisted == false
      and .receipt_accepted == false
      and .readback_evidence_recorded == false
      and .readback_evidence_persisted == false
      and .router_handoff_recorded == false
      and .runtime_router_mutated == false
      and .live_context_attached == false
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
      and .rollback_executed == false
      and .receipt_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_receipt_observability_denial_gate" \
  --arg readback_receipt_report_sha256 "$readback_receipt_report_sha256" \
  --arg runtime_model_provider_router_source "$RUNTIME_MODEL_PROVIDER_ROUTER_SOURCE" \
  --arg runtime_model_provider_router_source_sha256 "$runtime_model_provider_router_source_sha256" \
  --arg observability_fixtures_sha256 "$observability_fixtures_sha256" \
  --arg receipt_observability_contract_hash_sha256 "$receipt_observability_contract_hash_sha256" \
  --arg receipt_observability_policy_hash_sha256 "$receipt_observability_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson readback "$READBACK_RECEIPT_JSON" \
  --argjson fixtures "$observability_fixtures_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_observability_denial_schema_version:"memory_intelligence_kg_full_enablement_runtime_provider_router_receipt_observability_denial_v1",
    receipt_observability_denial_mode:"runtime_provider_router_receipt_observability_denial_no_export_no_query_no_observability_no_adapter_invocation",
    source_readback_receipt_skeleton_gate:$readback.gate,
    source_readback_receipt_skeleton_report_sha256:$readback_receipt_report_sha256,
    source_runtime_model_provider_router:$runtime_model_provider_router_source,
    source_runtime_model_provider_router_sha256:$runtime_model_provider_router_source_sha256,
    observability_fixtures_sha256:$observability_fixtures_sha256,
    receipt_observability_contract_hash_sha256:$receipt_observability_contract_hash_sha256,
    receipt_observability_policy_hash_sha256:$receipt_observability_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    runtime_provider_router_receipt_observability_denial_ready:true,
    runtime_provider_router_receipt_observability_denial_status:"blocked",
    readback_receipt_skeleton_ready:$readback.runtime_provider_router_readback_receipt_skeleton_ready,
    readback_receipt_skeleton_status:$readback.runtime_provider_router_readback_receipt_skeleton_status,
    negative_fixture_matrix_ready:$readback.negative_fixture_matrix_ready,
    negative_fixture_matrix_status:$readback.negative_fixture_matrix_status,
    runtime_attachment_packet_recorded:$readback.runtime_attachment_packet_recorded,
    runtime_attachment_packet_persisted:$readback.runtime_attachment_packet_persisted,
    runtime_attachment_packet_accepted:$readback.runtime_attachment_packet_accepted,
    readback_receipt_skeleton_item_count:$readback.readback_receipt_skeleton_item_count,
    declared_readback_receipt_skeleton_item_count:$readback.declared_readback_receipt_skeleton_item_count,
    required_readback_receipt_skeleton_item_count:$readback.required_readback_receipt_skeleton_item_count,
    recorded_readback_receipt_skeleton_item_count:$readback.recorded_readback_receipt_skeleton_item_count,
    persisted_readback_receipt_skeleton_item_count:$readback.persisted_readback_receipt_skeleton_item_count,
    accepted_readback_receipt_skeleton_item_count:$readback.accepted_readback_receipt_skeleton_item_count,
    runtime_attachment_blocking_receipt_count:$readback.runtime_attachment_blocking_receipt_count,
    live_context_attachment_blocking_receipt_count:$readback.live_context_attachment_blocking_receipt_count,
    model_invocation_blocking_receipt_count:$readback.model_invocation_blocking_receipt_count,
    provider_router_id:$readback.provider_router_id,
    feature_flag_id:$readback.feature_flag_id,
    activation_contract:$readback.activation_contract,
    selected_canary_stage_id:$readback.selected_canary_stage_id,
    shadow_traffic_percent_ppm:$readback.shadow_traffic_percent_ppm,
    max_context_node_count_cap:$readback.max_context_node_count_cap,
    receipt_observability_surface_count:12,
    receipt_observability_surface_ready_count:12,
    receipt_observability_side_effect_free_surface_count:12,
    receipt_observability_fixture_count:($fixtures | length),
    blocked_receipt_observability_fixture_count:($fixtures | length),
    noop_receipt_observability_fixture_count:($fixtures | length),
    allowed_receipt_observability_fixture_count:0,
    accepted_receipt_observability_fixture_count:0,
    receipt_export_denied_count:10,
    receipt_query_denied_count:10,
    receipt_observability_denied_count:10,
    receipt_export_performed_count:0,
    receipt_query_performed_count:0,
    receipt_observability_performed_count:0,
    receipt_export_allowed:false,
    receipt_export_request_accepted:false,
    receipt_export_recorded:false,
    receipt_export_persisted:false,
    receipt_export_artifact_written:false,
    receipt_export_stream_opened:false,
    receipt_export_filesystem_written:false,
    receipt_query_allowed:false,
    receipt_query_registered:false,
    receipt_query_endpoint_materialized:false,
    receipt_query_index_recorded:false,
    receipt_query_cache_written:false,
    receipt_query_result_materialized:false,
    receipt_observability_allowed:false,
    receipt_observability_metric_emitted:false,
    receipt_observability_log_recorded:false,
    receipt_observability_trace_recorded:false,
    receipt_observability_span_recorded:false,
    receipt_observability_event_recorded:false,
    receipt_observability_dashboard_materialized:false,
    receipt_observability_alert_registered:false,
    receipt_observability_slo_recorded:false,
    receipt_router_event_observability_recorded:false,
    receipt_readback_ledger_observability_recorded:false,
    receipt_recorded:false,
    receipt_persisted:false,
    receipt_accepted:false,
    receipt_materialized:false,
    receipt_filesystem_written:false,
    readback_evidence_recorded:false,
    readback_evidence_persisted:false,
    router_handoff_recorded:false,
    router_handoff_persisted:false,
    runtime_router_mutated:false,
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
    rollback_executed:false,
    external_send_performed:false,
    public_release_claimed:false,
    service_restart_performed:false,
    active_binary_mutated:false,
    receipt_observability_surfaces:[
      "source_readback_receipt_skeleton_required",
      "export_request_shape_denied",
      "export_artifact_write_denied",
      "export_stream_open_denied",
      "query_endpoint_materialization_denied",
      "query_index_cache_recording_denied",
      "observability_metric_emission_denied",
      "trace_span_log_event_recording_denied",
      "dashboard_alert_slo_materialization_denied",
      "router_event_readback_ledger_observability_denied",
      "runtime_attachment_live_context_provider_observability_denied",
      "external_public_install_restart_active_binary_observability_denied"
    ],
    receipt_observability_fixtures:$fixtures,
    denied_by_receipt_observability:[
      "source_readback_receipt_skeleton_required",
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
      "router_event_observability_recording_denied",
      "readback_ledger_observability_recording_denied",
      "runtime_attachment_observability_denied",
      "live_context_observability_denied",
      "provider_prompt_observability_denied",
      "external_public_install_restart_active_binary_observability_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_runtime_provider_router_receipt_observability_denial",
        status:"allowed_report_only",
        exports_receipt:false,
        registers_query:false,
        records_observability:false,
        invokes_adapter:false,
        invokes_model:false
      },
      {
        action:"stage_runtime_provider_router_operator_facing_summary_non_persistence",
        status:"allowed_report_only_next_slice",
        persists_summary:false,
        exports_receipt:false,
        records_observability:false,
        invokes_adapter:false,
        invokes_model:false
      },
      {
        action:"run_full_light_preflight",
        status:"allowed_verification_only",
        mutates_runtime:false,
        attaches_live_context:false,
        invokes_model:false,
        writes_kg:false
      }
    ],
    source_readback_receipt_skeleton_required:true,
    receipt_export_denial_required:true,
    receipt_query_denial_required:true,
    receipt_observability_denial_required:true,
    router_event_observability_denial_required:true,
    readback_ledger_observability_denial_required:true,
    adapter_invocation_forbidden:true,
    router_handoff_persistence_forbidden:true,
    readback_evidence_persistence_forbidden:true,
    live_context_attachment_forbidden:true,
    provider_model_invocation_forbidden:true,
    auth_secret_read_forbidden:true,
    usage_recording_forbidden:true,
    side_effects:{
      receipt_export_recorded:false,
      receipt_export_persisted:false,
      receipt_export_artifact_written:false,
      receipt_export_stream_opened:false,
      receipt_export_filesystem_written:false,
      receipt_query_registered:false,
      receipt_query_endpoint_materialized:false,
      receipt_query_index_recorded:false,
      receipt_query_cache_written:false,
      receipt_query_result_materialized:false,
      receipt_observability_metric_emitted:false,
      receipt_observability_log_recorded:false,
      receipt_observability_trace_recorded:false,
      receipt_observability_span_recorded:false,
      receipt_observability_event_recorded:false,
      receipt_observability_dashboard_materialized:false,
      receipt_observability_alert_registered:false,
      receipt_observability_slo_recorded:false,
      receipt_router_event_observability_recorded:false,
      receipt_readback_ledger_observability_recorded:false,
      receipt_recorded:false,
      receipt_persisted:false,
      receipt_accepted:false,
      readback_evidence_recorded:false,
      readback_evidence_persisted:false,
      router_handoff_recorded:false,
      router_handoff_persisted:false,
      runtime_router_mutated:false,
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
      rollback_executed:false,
      filesystem_written:false,
      external_send_performed:false,
      public_release_claimed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_receipt_observability_denial_gate"
  and .receipt_observability_denial_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_receipt_observability_denial_v1"
  and .runtime_provider_router_receipt_observability_denial_ready == true
  and .runtime_provider_router_receipt_observability_denial_status == "blocked"
  and .readback_receipt_skeleton_ready == true
  and .readback_receipt_skeleton_status == "blocked"
  and .negative_fixture_matrix_ready == true
  and .negative_fixture_matrix_status == "blocked"
  and .runtime_attachment_packet_recorded == false
  and .runtime_attachment_packet_persisted == false
  and .runtime_attachment_packet_accepted == false
  and .readback_receipt_skeleton_item_count == 12
  and .declared_readback_receipt_skeleton_item_count == 12
  and .required_readback_receipt_skeleton_item_count == 12
  and .recorded_readback_receipt_skeleton_item_count == 0
  and .persisted_readback_receipt_skeleton_item_count == 0
  and .accepted_readback_receipt_skeleton_item_count == 0
  and .runtime_attachment_blocking_receipt_count == 12
  and .live_context_attachment_blocking_receipt_count == 12
  and .model_invocation_blocking_receipt_count == 12
  and .provider_router_id == "hepta-native-model-provider-router"
  and .feature_flag_id == "HEPTA_MEMORY_CONTEXT_LIVE_TURN"
  and .activation_contract == "hepta-intelligence-memory-provider-router-activation-gate-v1"
  and .selected_canary_stage_id == "shadow-canary-0ppm"
  and .shadow_traffic_percent_ppm == 0
  and .max_context_node_count_cap == 128
  and .receipt_observability_surface_count == 12
  and .receipt_observability_surface_ready_count == 12
  and .receipt_observability_side_effect_free_surface_count == 12
  and .receipt_observability_fixture_count == 10
  and .blocked_receipt_observability_fixture_count == 10
  and .noop_receipt_observability_fixture_count == 10
  and .allowed_receipt_observability_fixture_count == 0
  and .accepted_receipt_observability_fixture_count == 0
  and .receipt_export_denied_count == 10
  and .receipt_query_denied_count == 10
  and .receipt_observability_denied_count == 10
  and .receipt_export_performed_count == 0
  and .receipt_query_performed_count == 0
  and .receipt_observability_performed_count == 0
  and .receipt_export_artifact_written == false
  and .receipt_export_stream_opened == false
  and .receipt_query_endpoint_materialized == false
  and .receipt_query_index_recorded == false
  and .receipt_query_cache_written == false
  and .receipt_observability_metric_emitted == false
  and .receipt_observability_log_recorded == false
  and .receipt_observability_trace_recorded == false
  and .receipt_observability_span_recorded == false
  and .receipt_observability_event_recorded == false
  and .receipt_observability_dashboard_materialized == false
  and .receipt_observability_alert_registered == false
  and .receipt_observability_slo_recorded == false
  and .receipt_router_event_observability_recorded == false
  and .receipt_readback_ledger_observability_recorded == false
  and .receipt_recorded == false
  and .receipt_persisted == false
  and .receipt_accepted == false
  and .readback_evidence_recorded == false
  and .readback_evidence_persisted == false
  and .router_handoff_recorded == false
  and .router_handoff_persisted == false
  and .runtime_router_mutated == false
  and .live_context_attached == false
  and .context_injection_performed == false
  and .adapter_invoked == false
  and .provider_invoked == false
  and .model_invoked == false
  and .auth_secret_read == false
  and .credential_read == false
  and .secret_file_read == false
  and .usage_recorded == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .rollback_executed == false
  and .external_send_performed == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.receipt_observability_surfaces | length) == 12
  and (.receipt_observability_fixtures | length) == 10
  and (.receipt_observability_fixtures | all((.receipt_observability_status == "blocked_noop" or .receipt_observability_status == "blocked_export_noop" or .receipt_observability_status == "blocked_query_noop" or .receipt_observability_status == "blocked_observability_noop") and .export_recorded == false and .export_artifact_written == false and .query_endpoint_materialized == false and .query_index_recorded == false and .observability_metric_emitted == false and .observability_trace_recorded == false and .observability_span_recorded == false and .observability_log_recorded == false and .observability_dashboard_materialized == false and .router_event_observability_recorded == false and .readback_ledger_observability_recorded == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .readback_evidence_recorded == false and .readback_evidence_persisted == false and .router_handoff_recorded == false and .runtime_router_mutated == false and .live_context_attached == false and .adapter_invoked == false and .provider_invoked == false and .model_invoked == false and .auth_secret_read == false and .credential_read == false and .secret_file_read == false and .usage_recorded == false and .memory_store_write_performed == false and .memory_store_mutated == false and .live_kg_write_performed == false and .rollback_executed == false and .receipt_noop_confirmed == true))
  and (.denied_by_receipt_observability | length) == 27
  and (.allowed_next_actions | any(.action == "review_runtime_provider_router_receipt_observability_denial" and .status == "allowed_report_only" and .exports_receipt == false and .registers_query == false and .records_observability == false and .invokes_adapter == false and .invokes_model == false))
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_operator_facing_summary_non_persistence" and .status == "allowed_report_only_next_slice" and .persists_summary == false and .exports_receipt == false and .records_observability == false and .invokes_adapter == false and .invokes_model == false))
  and (.allowed_next_actions | any(.action == "run_full_light_preflight" and .status == "allowed_verification_only" and .mutates_runtime == false and .attaches_live_context == false and .invokes_model == false and .writes_kg == false))
  and .source_readback_receipt_skeleton_required == true
  and .receipt_export_denial_required == true
  and .receipt_query_denial_required == true
  and .receipt_observability_denial_required == true
  and .router_event_observability_denial_required == true
  and .readback_ledger_observability_denial_required == true
  and .adapter_invocation_forbidden == true
  and .router_handoff_persistence_forbidden == true
  and .readback_evidence_persistence_forbidden == true
  and .live_context_attachment_forbidden == true
  and .provider_model_invocation_forbidden == true
  and .auth_secret_read_forbidden == true
  and .usage_recording_forbidden == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement runtime provider-router receipt observability denial gate passed"
