#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
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
    echo "missing operator readiness packet template packet-acceptance receipt export/query/observability route source text: $label" >&2
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
    "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_ready == true
  and .source_packet_acceptance_receipt_retention_expiry_gc_ready == true
  and .source_retention_expiry_gc_surface_count == 17
  and .source_retention_expiry_gc_attempt_count == 17
  and .source_retention_policy_recorded_count == 0
  and .source_expiry_recorded_count == 0
  and .source_garbage_collection_scan_performed_count == 0
  and .source_archive_written_count == 0
  and .source_compaction_performed_count == 0
  and .source_retention_gc_activation_authority_derived_count == 0
  and .export_query_observability_surface_count == 16
  and .export_query_observability_attempt_count == 16
  and .query_registered_count == 0
  and .query_executed_count == 0
  and .query_result_recorded_count == 0
  and .query_result_persisted_count == 0
  and .search_index_recorded_count == 0
  and .search_index_persisted_count == 0
  and .export_requested_count == 0
  and .export_snapshot_recorded_count == 0
  and .export_snapshot_persisted_count == 0
  and .export_file_written_count == 0
  and .observability_metric_recorded_count == 0
  and .observability_event_recorded_count == 0
  and .dashboard_panel_recorded_count == 0
  and .operator_summary_recorded_count == 0
  and .readback_surface_recorded_count == 0
  and .audit_view_recorded_count == 0
  and .external_delivery_performed_count == 0
  and .completion_ack_recorded_count == 0
  and .export_query_observability_acceptance_recorded_count == 0
  and .export_query_observability_operator_approval_derived_count == 0
  and .export_query_observability_activation_authority_derived_count == 0
  and .export_query_observability_activation_command_derived_count == 0
  and .export_query_observability_live_execution_allowed_count == 0
  and (.export_query_observability_surfaces | all(
    .export_query_or_observability_attempted == true
    and .query_registered == false
    and .query_executed == false
    and .query_result_recorded == false
    and .search_index_recorded == false
    and .export_requested == false
    and .export_snapshot_recorded == false
    and .export_file_written == false
    and .observability_metric_recorded == false
    and .observability_event_recorded == false
    and .dashboard_panel_recorded == false
    and .operator_summary_recorded == false
    and .readback_surface_recorded == false
    and .audit_view_recorded == false
    and .external_delivery_performed == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .export_query_observability_status == "export_query_observability_denied"
  ))
  and (.denied_by_packet_receipt_export_query_observability | length) == 18
  and .packet_acceptance_receipt_query_registered == false
  and .packet_acceptance_receipt_query_executed == false
  and .packet_acceptance_receipt_query_result_recorded == false
  and .packet_acceptance_receipt_export_snapshot_recorded == false
  and .packet_acceptance_receipt_export_file_written == false
  and .packet_acceptance_receipt_observability_metric_recorded == false
  and .packet_acceptance_receipt_observability_event_recorded == false
  and .packet_acceptance_receipt_dashboard_panel_recorded == false
  and .packet_acceptance_receipt_operator_summary_recorded == false
  and .packet_acceptance_receipt_readback_surface_recorded == false
  and .packet_acceptance_receipt_external_delivery_performed == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .activation_authority_derived == false
  and .activation_command_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$EXPORT_QUERY_OBSERVABILITY_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 144;' \
  "native gateway route/source command count includes operator readiness packet acceptance receipt export/query/observability route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT' \
  "native gateway operator readiness packet acceptance receipt export/query/observability endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-denial' \
  "native gateway operator readiness packet acceptance receipt export/query/observability endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-denial --json' \
  "native gateway operator readiness packet acceptance receipt export/query/observability source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_report' \
  "native gateway operator readiness packet acceptance receipt export/query/observability report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_route_enabled": true' \
  "operator readiness packet acceptance receipt export/query/observability route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"export_query_observability_surface_count": export_query_observability_surface_count' \
  "packet acceptance receipt export/query/observability surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_query_registered": false' \
  "packet acceptance receipt query registration denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_export_snapshot_recorded": false' \
  "packet acceptance receipt export snapshot denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_observability_metric_recorded": false' \
  "packet acceptance receipt observability metric denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_endpoint_blocks_query_export_observability \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-denial"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 144
    and .implemented_route_count == 144
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_ready == true
    and .source_packet_acceptance_receipt_retention_expiry_gc_ready == true
    and .source_retention_expiry_gc_surface_count == 17
    and .source_retention_expiry_gc_attempt_count == 17
    and .source_retention_policy_recorded_count == 0
    and .source_expiry_recorded_count == 0
    and .source_garbage_collection_scan_performed_count == 0
    and .source_archive_written_count == 0
    and .source_compaction_performed_count == 0
    and .source_retention_gc_activation_authority_derived_count == 0
    and .export_query_observability_surface_count == 16
    and .export_query_observability_attempt_count == 16
    and .query_registered_count == 0
    and .query_executed_count == 0
    and .query_result_recorded_count == 0
    and .search_index_recorded_count == 0
    and .export_requested_count == 0
    and .export_snapshot_recorded_count == 0
    and .export_file_written_count == 0
    and .observability_metric_recorded_count == 0
    and .observability_event_recorded_count == 0
    and .dashboard_panel_recorded_count == 0
    and .operator_summary_recorded_count == 0
    and .readback_surface_recorded_count == 0
    and .audit_view_recorded_count == 0
    and .external_delivery_performed_count == 0
    and .completion_ack_recorded_count == 0
    and .export_query_observability_activation_authority_derived_count == 0
    and .export_query_observability_activation_command_derived_count == 0
    and .export_query_observability_live_execution_allowed_count == 0
    and (.export_query_observability_surfaces | all(
      .export_query_or_observability_attempted == true
      and .query_registered == false
      and .query_executed == false
      and .export_snapshot_recorded == false
      and .observability_metric_recorded == false
      and .external_delivery_performed == false
      and .activation_authority_derived == false
      and .live_execution_allowed == false
      and .export_query_observability_status == "export_query_observability_denied"
    ))
    and (.denied_by_packet_receipt_export_query_observability | length) == 18
    and .packet_acceptance_receipt_query_registered == false
    and .packet_acceptance_receipt_export_snapshot_recorded == false
    and .packet_acceptance_receipt_observability_metric_recorded == false
    and .packet_acceptance_receipt_external_delivery_performed == false
    and .operator_acceptance_recorded == false
    and .operator_approval_recorded == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .activation_allowed == false
    and .activation_performed == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .install_executed == false
    and .service_restarted == false
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
  and .required_marker_count == 284
  and .present_required_marker_count == 284
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
  --arg base_url "$BASE_URL" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-denial --json" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg source_export_query_observability_gate_sha256 "$source_export_query_observability_gate_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$EXPORT_QUERY_OBSERVABILITY_JSON" \
  --argjson terminal "$TERMINAL_COVERAGE_JSON" \
  --argjson live "$LIVE_ROUTE_JSON" \
  --argjson live_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:"hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_route_gate",
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_packet_acceptance_receipt_export_query_observability_gate_ready:true,
    source_export_query_observability_gate_sha256:$source_export_query_observability_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$test_log,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_checked:$live_checked,
    source_route_count_expected:144,
    terminal_required_marker_count_expected:284,
    source_packet_acceptance_receipt_retention_expiry_gc_ready:$source.source_packet_acceptance_receipt_retention_expiry_gc_ready,
    source_retention_expiry_gc_surface_count:$source.source_retention_expiry_gc_surface_count,
    export_query_observability_surface_count:$source.export_query_observability_surface_count,
    export_query_observability_attempt_count:$source.export_query_observability_attempt_count,
    query_registered_count:$source.query_registered_count,
    query_executed_count:$source.query_executed_count,
    export_requested_count:$source.export_requested_count,
    export_snapshot_recorded_count:$source.export_snapshot_recorded_count,
    observability_metric_recorded_count:$source.observability_metric_recorded_count,
    observability_event_recorded_count:$source.observability_event_recorded_count,
    external_delivery_performed_count:$source.external_delivery_performed_count,
    export_query_observability_activation_authority_derived_count:$source.export_query_observability_activation_authority_derived_count,
    export_query_observability_live_execution_allowed_count:$source.export_query_observability_live_execution_allowed_count,
    route_source_texts_ready:true,
    terminal_coverage_ready:true,
    terminal_required_marker_count:$terminal.required_marker_count,
    terminal_present_required_marker_count:$terminal.present_required_marker_count,
    terminal_missing_required_marker_count:$terminal.missing_required_marker_count,
    live_route_status:($live.status // null),
    live_route_count:($live.route_count // null),
    live_missing_route_count:($live.missing_route_count // null),
    side_effects:{
      route_gate_filesystem_written:false,
      route_gate_runtime_mutated:false,
      route_gate_service_restarted:false,
      route_gate_external_send_performed:false,
      source_gate_side_effects:$source.side_effects,
      live_route_side_effects:($live.side_effects // {})
    }
  }'

echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt export/query/observability denial route gate passed"
