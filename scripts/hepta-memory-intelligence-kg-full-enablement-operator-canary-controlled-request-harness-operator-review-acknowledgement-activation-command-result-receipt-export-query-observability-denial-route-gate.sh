#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
RELEASE_BIN="${HEPTA_RELEASE_BIN:-${HEPTA_CODEX_RELEASE_BIN:-$HOME/.local/opt/hepta/bin/hepta}}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

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
    echo "missing operator canary activation command result receipt export/query/observability route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

EXPORT_QUERY_OBSERVABILITY_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_status == "blocked"
  and .source_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial_gate"
  and .source_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_status == "blocked"
  and .source_retention_expiry_garbage_collection_fixture_count == 10
  and .source_blocked_retention_expiry_garbage_collection_fixture_count == 10
  and .source_accepted_retention_expiry_garbage_collection_fixture_count == 0
  and .source_retention_performed_count == 0
  and .source_expiry_performed_count == 0
  and .source_garbage_collection_performed_count == 0
  and .export_query_observability_surface_count == 12
  and .export_query_observability_surface_ready_count == 12
  and .export_query_observability_side_effect_free_surface_count == 12
  and .export_query_observability_fixture_count == 10
  and .blocked_export_query_observability_fixture_count == 10
  and .noop_export_query_observability_fixture_count == 10
  and .allowed_export_query_observability_fixture_count == 0
  and .accepted_export_query_observability_fixture_count == 0
  and .export_performed_count == 0
  and .query_performed_count == 0
  and .observability_performed_count == 0
  and .activation_command_result_receipt_export_allowed == false
  and .activation_command_result_receipt_export_request_accepted == false
  and .activation_command_result_receipt_export_recorded == false
  and .activation_command_result_receipt_export_persisted == false
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
  and .operator_approval_from_export_accepted == false
  and .operator_approval_from_query_accepted == false
  and .operator_approval_from_observability_accepted == false
  and .activation_allowed_by_result_receipt_export == false
  and .activation_allowed_by_result_receipt_query == false
  and .activation_allowed_by_result_receipt_observability == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_request_accepted == false
  and .activation_request_executed == false
  and .dispatch_performed_count == 0
  and .execution_performed_count == 0
  and .runtime_router_mutated_count == 0
  and .context_injection_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .memory_store_write_performed_count == 0
  and .external_kg_adapter_read_performed_count == 0
  and .live_kg_write_performed_count == 0
  and .credential_read_count == 0
  and .secret_file_read_count == 0
  and .channel_send_performed_count == 0
  and .install_performed_count == 0
  and .service_restarted_count == 0
  and .active_binary_mutated_count == 0
  and .upstream_fetch_performed_count == 0
  and .upstream_merge_performed_count == 0
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and (.export_query_observability_fixtures | length) == 10
  and (.export_query_observability_fixtures | all(
    (.export_query_observability_status | startswith("blocked"))
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
    and .activation_command_result_receipt_accepted == false
    and .operator_approval_from_export_accepted == false
    and .activation_from_export_allowed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .receipt_noop_confirmed == true
  ))
  and .denied_by_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_count >= 240
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$EXPORT_QUERY_OBSERVABILITY_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 142;' \
  "native gateway route/source command count includes activation command result receipt export/query/observability denial route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT' \
  "native gateway activation command result receipt export/query/observability endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial' \
  "native gateway activation command result receipt export/query/observability endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial --json' \
  "native gateway activation command result receipt export/query/observability source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_report' \
  "native gateway activation command result receipt export/query/observability report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_route_enabled": true' \
  "activation command result receipt export/query/observability route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_export_artifact_written"' \
  "activation command result receipt export artifact denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_query_endpoint_materialized"' \
  "activation command result receipt query endpoint denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_observability_metric_emitted"' \
  "activation command result receipt observability metric denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_observability_dashboard_materialized"' \
  "activation command result receipt observability dashboard denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-canary-activation-command-result-receipt-export-query-observability-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_endpoint_blocks_reporting_surfaces \
  -- --nocapture >"$TEST_LOG"

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 142
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_route_ready == true
    and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_route_enabled == true
    and .export_query_observability_fixture_count == 10
    and .blocked_export_query_observability_fixture_count == 10
    and .allowed_export_query_observability_fixture_count == 0
    and .export_performed_count == 0
    and .query_performed_count == 0
    and .observability_performed_count == 0
    and .activation_command_result_receipt_export_artifact_written == false
    and .activation_command_result_receipt_export_stream_opened == false
    and .activation_command_result_receipt_query_registered == false
    and .activation_command_result_receipt_query_endpoint_materialized == false
    and .activation_command_result_receipt_observability_metric_emitted == false
    and .activation_command_result_receipt_observability_log_recorded == false
    and .activation_command_result_receipt_observability_trace_recorded == false
    and .activation_command_result_receipt_observability_dashboard_materialized == false
    and .activation_command_result_receipt_observability_alert_registered == false
    and .operator_approval_from_export_accepted == false
    and .activation_allowed_by_result_receipt_export == false
    and .activation_allowed_by_result_receipt_query == false
    and .activation_allowed_by_result_receipt_observability == false
    and .provider_invoked_count == 0
    and .model_invoked_count == 0
    and .memory_store_write_performed_count == 0
    and .external_kg_adapter_read_performed_count == 0
    and .live_kg_write_performed_count == 0
    and .credential_read_count == 0
    and .secret_file_read_count == 0
    and .channel_send_performed_count == 0
    and .install_performed_count == 0
    and .service_restarted_count == 0
    and .active_binary_mutated_count == 0
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_ROUTE_JSON"
else
  LIVE_ROUTE_JSON='null'
fi

TERMINAL_COVERAGE_JSON="$(
  capture_json_report \
    "hepta-preflight-terminal-coverage-inventory-gate" \
    scripts/hepta-preflight-terminal-coverage-inventory-gate.sh
)"
jq -e '
  .status == "ready"
  and .preflight_terminal_coverage_inventory_ready == true
  and .required_marker_count == 282
  and .present_required_marker_count == 282
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_export_query_observability_gate_sha256="$(printf '%s' "$EXPORT_QUERY_OBSERVABILITY_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "ready" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial --json" \
  --arg source_export_query_observability_gate_sha256 "$source_export_query_observability_gate_sha256" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$EXPORT_QUERY_OBSERVABILITY_JSON" \
  --argjson terminal "$TERMINAL_COVERAGE_JSON" \
  --argjson live "$LIVE_ROUTE_JSON" \
  --arg require_live "$REQUIRE_LIVE_ENDPOINT" \
  '{
    product: $product,
    runtime: $runtime,
    status: $status,
    base_url: $base_url,
    gate: $gate,
    endpoint: $endpoint,
    source_command: $source_command,
    activation_mode: "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_native_route_status",
    source_activation_command_result_receipt_export_query_observability_denial_gate: $source.gate,
    source_activation_command_result_receipt_export_query_observability_denial_gate_ready: $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_ready,
    source_activation_command_result_receipt_export_query_observability_denial_gate_status: $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_export_query_observability_denial_status,
    source_export_query_observability_gate_sha256: $source_export_query_observability_gate_sha256,
    source_route_wired: true,
    source_route_count_expected:142,
    source_route_tested_by_native_gateway_unit_test: true,
    native_gateway_source: "codex-rs/cli/src/native_gateway.rs",
    native_gateway_sha256: $native_gateway_sha256,
    native_gateway_unit_test_log: $test_log,
    live_endpoint_required: ($require_live == "1"),
    live_endpoint_ready: (if $require_live == "1" then ($live.status == "ready") else null end),
    export_query_observability_fixture_count: $source.export_query_observability_fixture_count,
    blocked_export_query_observability_fixture_count: $source.blocked_export_query_observability_fixture_count,
    noop_export_query_observability_fixture_count: $source.noop_export_query_observability_fixture_count,
    accepted_export_query_observability_fixture_count: $source.accepted_export_query_observability_fixture_count,
    export_performed_count: $source.export_performed_count,
    query_performed_count: $source.query_performed_count,
    observability_performed_count: $source.observability_performed_count,
    activation_command_result_receipt_export_artifact_written: $source.activation_command_result_receipt_export_artifact_written,
    activation_command_result_receipt_export_stream_opened: $source.activation_command_result_receipt_export_stream_opened,
    activation_command_result_receipt_query_registered: $source.activation_command_result_receipt_query_registered,
    activation_command_result_receipt_query_endpoint_materialized: $source.activation_command_result_receipt_query_endpoint_materialized,
    activation_command_result_receipt_observability_metric_emitted: $source.activation_command_result_receipt_observability_metric_emitted,
    activation_command_result_receipt_observability_log_recorded: $source.activation_command_result_receipt_observability_log_recorded,
    activation_command_result_receipt_observability_trace_recorded: $source.activation_command_result_receipt_observability_trace_recorded,
    activation_command_result_receipt_observability_dashboard_materialized: $source.activation_command_result_receipt_observability_dashboard_materialized,
    activation_command_result_receipt_observability_alert_registered: $source.activation_command_result_receipt_observability_alert_registered,
    activation_command_enabled: $source.activation_command_enabled,
    activation_command_invoked: $source.activation_command_invoked,
    activation_command_dispatched: $source.activation_command_dispatched,
    dispatch_performed_count: $source.dispatch_performed_count,
    execution_performed_count: $source.execution_performed_count,
    provider_invoked_count: $source.provider_invoked_count,
    model_invoked_count: $source.model_invoked_count,
    memory_store_write_performed_count: $source.memory_store_write_performed_count,
    external_kg_adapter_read_performed_count: $source.external_kg_adapter_read_performed_count,
    live_kg_write_performed_count: $source.live_kg_write_performed_count,
    credential_read_count: $source.credential_read_count,
    secret_file_read_count: $source.secret_file_read_count,
    channel_send_performed_count: $source.channel_send_performed_count,
    install_performed_count: $source.install_performed_count,
    service_restarted_count: $source.service_restarted_count,
    active_binary_mutated_count: $source.active_binary_mutated_count,
    terminal_coverage_sha256: $terminal_coverage_sha256,
    terminal_required_marker_count: $terminal.required_marker_count,
    terminal_present_required_marker_count: $terminal.present_required_marker_count,
    terminal_missing_required_marker_count: $terminal.missing_required_marker_count,
    terminal_duplicate_required_marker_count: $terminal.duplicate_required_marker_count,
    terminal_out_of_order_required_marker_count: $terminal.out_of_order_required_marker_count,
    side_effects: $source.side_effects
  }'

echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation command result receipt export/query/observability denial route gate passed"
