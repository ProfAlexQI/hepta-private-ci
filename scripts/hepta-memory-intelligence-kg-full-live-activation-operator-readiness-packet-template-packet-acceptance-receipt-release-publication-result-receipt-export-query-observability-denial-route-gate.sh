#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
RELEASE_BIN="${HEPTA_RELEASE_BIN:-}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

if [[ -z "$RELEASE_BIN" && -x "$HOME/.local/opt/hepta/bin/hepta" ]]; then
  RELEASE_BIN="$HOME/.local/opt/hepta/bin/hepta"
fi

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
    echo "missing operator readiness packet template packet-acceptance receipt release publication result receipt export/query/observability route source text: $label" >&2
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
    "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_retention_ready == true
  and .source_release_publication_result_receipt_retention_surface_count == 18
  and .source_release_publication_result_receipt_retention_attempt_count == 18
  and .source_release_publication_result_receipt_retention_policy_recorded_count == 0
  and .source_release_publication_result_receipt_expiry_recorded_count == 0
  and .source_release_publication_result_receipt_garbage_collection_scan_performed_count == 0
  and .release_publication_result_receipt_export_query_observability_surface_count == 18
  and .release_publication_result_receipt_export_query_observability_attempt_count == 18
  and .release_publication_result_receipt_query_registered_count == 0
  and .release_publication_result_receipt_query_executed_count == 0
  and .release_publication_result_receipt_query_result_recorded_count == 0
  and .release_publication_result_receipt_search_index_recorded_count == 0
  and .release_publication_result_receipt_export_requested_count == 0
  and .release_publication_result_receipt_export_accepted_count == 0
  and .release_publication_result_receipt_export_snapshot_recorded_count == 0
  and .release_publication_result_receipt_export_file_written_count == 0
  and .release_publication_result_receipt_export_stream_opened_count == 0
  and .release_publication_result_receipt_observability_metric_recorded_count == 0
  and .release_publication_result_receipt_observability_log_recorded_count == 0
  and .release_publication_result_receipt_observability_trace_recorded_count == 0
  and .release_publication_result_receipt_observability_event_recorded_count == 0
  and .release_publication_result_receipt_dashboard_panel_recorded_count == 0
  and .release_publication_result_receipt_alert_registered_count == 0
  and .release_publication_result_receipt_slo_recorded_count == 0
  and .release_publication_result_receipt_operator_summary_recorded_count == 0
  and .release_publication_result_receipt_readback_surface_recorded_count == 0
  and .release_publication_result_receipt_audit_view_recorded_count == 0
  and .release_publication_result_receipt_export_query_observability_acceptance_recorded_count == 0
  and .release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_export_query_observability_activation_authority_derived_count == 0
  and .release_publication_result_receipt_export_query_observability_live_execution_allowed_count == 0
  and (.release_publication_result_receipt_export_query_observability_surfaces | length) == 18
  and (.release_publication_result_receipt_export_query_observability_surfaces | all(
    .export_query_or_observability_attempted == true
    and .query_registered == false
    and .query_executed == false
    and .query_result_recorded == false
    and .search_index_recorded == false
    and .export_requested == false
    and .export_snapshot_recorded == false
    and .export_file_written == false
    and .export_stream_opened == false
    and .observability_metric_recorded == false
    and .observability_event_recorded == false
    and .dashboard_panel_recorded == false
    and .operator_summary_recorded == false
    and .readback_surface_recorded == false
    and .audit_view_recorded == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .live_execution_allowed == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .export_query_observability_noop_confirmed == true
    and .release_publication_result_receipt_export_query_observability_status == "release_publication_result_receipt_export_query_observability_denied"
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_export_query_observability | length) == 29
  and .packet_acceptance_receipt_release_publication_result_receipt_query_registered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_query_executed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_export_requested == false
  and .packet_acceptance_receipt_release_publication_result_receipt_export_file_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_observability_metric_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_dashboard_panel_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_audit_view_recorded == false
  and .operator_approval_recorded == false
  and .release_publication_authority_derived == false
  and .activation_authority_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and .memory_store_write_performed == false
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .credential_read == false
  and .install_executed == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .external_send_performed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$EXPORT_QUERY_OBSERVABILITY_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 145;' \
  "native gateway route/source command count includes packet acceptance receipt release publication result receipt export/query/observability route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT' \
  "native gateway release publication result receipt export/query/observability endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial' \
  "native gateway release publication result receipt export/query/observability endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial --json' \
  "native gateway release publication result receipt export/query/observability source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_report' \
  "native gateway release publication result receipt export/query/observability report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_route_enabled": true' \
  "packet acceptance receipt release publication result receipt export/query/observability route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"release_publication_result_receipt_export_query_observability_surface_count": export_query_observability_surface_count' \
  "packet acceptance receipt release publication result receipt export/query/observability surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"release_publication_result_receipt_query_registered_count": 0' \
  "packet acceptance receipt release publication result receipt query registration denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_endpoint_blocks_view_materialization \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 145
    and .implemented_route_count == 145
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_ready == true
    and .release_publication_result_receipt_export_query_observability_surface_count == 18
    and .release_publication_result_receipt_query_registered_count == 0
    and .release_publication_result_receipt_export_file_written_count == 0
    and .release_publication_result_receipt_observability_metric_recorded_count == 0
    and .release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count == 0
    and .release_publication_result_receipt_export_query_observability_activation_authority_derived_count == 0
    and .release_publication_result_receipt_export_query_observability_live_execution_allowed_count == 0
    and (.release_publication_result_receipt_export_query_observability_surfaces | length) == 18
    and (.denied_by_packet_receipt_release_publication_result_receipt_export_query_observability | length) == 29
    and .operator_approval_recorded == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .activation_performed == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .credential_read == false
    and .install_executed == false
    and .external_send_performed == false
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_ROUTE_JSON"
fi

TERMINAL_COVERAGE_JSON="$(
  capture_json_report \
    "hepta-preflight-terminal-coverage-inventory-gate" \
    scripts/hepta-preflight-terminal-coverage-inventory-gate.sh
)"

jq -e '
  .status == "ready"
  and .preflight_terminal_coverage_inventory_ready == true
  and .required_marker_count == 285
  and .present_required_marker_count == 285
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_export_query_observability_gate_sha256="$(printf '%s' "$EXPORT_QUERY_OBSERVABILITY_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"
live_route_status="$(jq -r '.status // "skipped"' <<<"$LIVE_ROUTE_JSON")"
live_route_count="$(jq -r '.route_count // 0' <<<"$LIVE_ROUTE_JSON")"
live_missing_route_count="$(jq -r '.missing_route_count // 0' <<<"$LIVE_ROUTE_JSON")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial --json" \
  --arg source_export_query_observability_gate_sha256 "$source_export_query_observability_gate_sha256" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg focused_test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --arg live_route_status "$live_route_status" \
  --argjson live_endpoint_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  --argjson live_route_count "$live_route_count" \
  --argjson live_missing_route_count "$live_missing_route_count" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_export_query_observability_gate_ready:true,
    source_export_query_observability_gate_sha256:$source_export_query_observability_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$focused_test_log,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_checked:$live_endpoint_checked,
    source_route_count_expected:145,
    terminal_required_marker_count_expected:285,
    source_packet_acceptance_receipt_release_publication_result_receipt_retention_ready:true,
    release_publication_result_receipt_export_query_observability_surface_count:18,
    release_publication_result_receipt_query_registered_count:0,
    release_publication_result_receipt_query_executed_count:0,
    release_publication_result_receipt_export_file_written_count:0,
    release_publication_result_receipt_export_stream_opened_count:0,
    release_publication_result_receipt_observability_metric_recorded_count:0,
    release_publication_result_receipt_dashboard_panel_recorded_count:0,
    release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count:0,
    release_publication_result_receipt_export_query_observability_activation_authority_derived_count:0,
    release_publication_result_receipt_export_query_observability_live_execution_allowed_count:0,
    route_source_texts_ready:true,
    terminal_coverage_ready:true,
    terminal_required_marker_count:275,
    terminal_present_required_marker_count:275,
    terminal_missing_required_marker_count:0,
    live_route_status:$live_route_status,
    live_route_count:$live_route_count,
    live_missing_route_count:$live_missing_route_count,
    side_effects:{
      route_gate_filesystem_written:false,
      route_gate_runtime_mutated:false,
      route_gate_service_restarted:false,
      route_gate_external_send_performed:false,
      source_gate_side_effects:{
        query_registered:false,
        query_executed:false,
        export_file_written:false,
        export_stream_opened:false,
        observability_recorded:false,
        release_publication_authority_derived:false,
        activation_authority_derived:false,
        live_execution_allowed:false,
        memory_store_write_performed:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        credential_read:false,
        install_executed:false,
        service_restarted:false,
        active_binary_mutated:false,
        external_send_performed:false,
        filesystem_written:false
      },
      live_route_side_effects:(if $live_endpoint_checked then {} else {} end)
    }
  }'

echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt export/query/observability denial route gate passed"
