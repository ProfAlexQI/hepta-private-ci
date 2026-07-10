#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT="${HEPTA_EXPECTED_ROUTE_COUNT:-$(bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")}"

cd "$REPO_ROOT"

source scripts/lib/hepta-json-report-capture.sh

require_unsigned_integer() {
  local name="$1"
  local value="$2"

  case "$value" in
    ''|*[!0-9]*)
      echo "$name must be an unsigned integer" >&2
      exit 2
      ;;
  esac
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing runtime provider-router activation command result receipt export/query/observability route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report \
      "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-export-query-observability-denial-gate" \
      scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-export-query-observability-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_gate"
  and .runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_status == "blocked"
  and .source_activation_command_result_receipt_retention_expiry_garbage_collection_ready == true
  and .source_activation_command_result_receipt_retention_expiry_garbage_collection_status == "blocked"
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
  and .activation_allowed_by_result_receipt_export == false
  and .activation_allowed_by_result_receipt_query == false
  and .activation_allowed_by_result_receipt_observability == false
  and .activation_allowed_by_result_receipt == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_request_accepted == false
  and .activation_request_executed == false
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
  and (.export_query_observability_surfaces | length) == 12
  and (.export_query_observability_fixtures | length) == 10
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
' >/dev/null <<<"$SOURCE_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native source command count is derived from the route registry"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT' \
  "runtime provider-router activation command result receipt export/query/observability endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-export-query-observability-denial' \
  "runtime provider-router activation command result receipt export/query/observability endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-export-query-observability-denial --json' \
  "runtime provider-router activation command result receipt export/query/observability source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_report' \
  "runtime provider-router activation command result receipt export/query/observability report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_route_enabled": true' \
  "runtime provider-router activation command result receipt export/query/observability route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_endpoint_blocks_reporting_surfaces' \
  "runtime provider-router activation command result receipt export/query/observability focused test"

TEST_LOG="$(mktemp /tmp/hepta-runtime-provider-router-activation-command-result-receipt-export-query-observability-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_endpoint_blocks_reporting_surfaces \
  -- --nocapture >"$TEST_LOG"

LIVE_JSON='{}'
live_checked=false
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS \
      "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-export-query-observability-denial"
  )"
  jq -e --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" '
    .runtime == "hepta"
    and .status == "ready"
    and .route_count == $expected_route_count
    and .implemented_route_count == $expected_route_count
    and .missing_route_count == 0
    and .native_gateway_source_command_count == $expected_route_count
    and .runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_route_enabled == true
    and .runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready == true
    and .runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_status == "blocked"
    and .source_activation_command_result_receipt_retention_expiry_garbage_collection_ready == true
    and .export_query_observability_surface_count == 12
    and .export_query_observability_fixture_count == 10
    and .blocked_export_query_observability_fixture_count == 10
    and .accepted_export_query_observability_fixture_count == 0
    and .export_performed_count == 0
    and .query_performed_count == 0
    and .observability_performed_count == 0
    and .activation_command_result_receipt_export_recorded == false
    and .activation_command_result_receipt_query_registered == false
    and .activation_command_result_receipt_observability_metric_emitted == false
    and .activation_allowed_by_result_receipt_export == false
    and .activation_allowed_by_result_receipt_query == false
    and .activation_allowed_by_result_receipt_observability == false
    and .activation_allowed_by_result_receipt == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .external_send_performed == false
    and .install_executed == false
    and .service_restart_performed == false
    and .active_binary_mutated == false
    and (.denied_by_export_query_observability | length) == 30
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_JSON"
  live_checked=true
fi

jq -n \
  --arg status "ready" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_route_gate" \
  --arg source_gate "$(jq -r '.gate' <<<"$SOURCE_JSON")" \
  --arg test_log "$TEST_LOG" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson live_checked "$live_checked" \
  --argjson live "$LIVE_JSON" \
  '{
    status:$status,
    gate:$gate,
    source_gate:$source_gate,
    expected_route_count:$expected_route_count,
    focused_test_log:$test_log,
    live_endpoint_checked:$live_checked,
    live_route_count: ($live.route_count // null),
    live_missing_route_count: ($live.missing_route_count // null),
    accepted_export_query_observability_fixture_count: ($live.accepted_export_query_observability_fixture_count // 0),
    export_performed_count: ($live.export_performed_count // 0),
    query_performed_count: ($live.query_performed_count // 0),
    observability_performed_count: ($live.observability_performed_count // 0)
  }'

echo "Hepta memory/intelligence/KG full enablement runtime provider-router activation command result receipt export/query/observability denial route gate passed"
