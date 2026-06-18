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
    echo "missing operator readiness packet template packet-acceptance receipt final acknowledgement route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

FINAL_ACKNOWLEDGEMENT_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_ready == true
  and .source_packet_acceptance_receipt_operator_briefing_ready == true
  and .source_operator_briefing_surface_count == 14
  and .source_briefing_recorded_count == 0
  and .source_briefing_persisted_count == 0
  and .source_briefing_materialized_count == 0
  and .source_summary_recorded_count == 0
  and .source_readback_digest_recorded_count == 0
  and .source_final_note_recorded_count == 0
  and .source_channel_delivery_performed_count == 0
  and .source_external_send_performed_count == 0
  and .source_telegram_send_performed_count == 0
  and .source_completion_ack_recorded_count == 0
  and .source_operator_briefing_activation_authority_derived_count == 0
  and .final_acknowledgement_surface_count == 14
  and .final_acknowledgement_attempt_count == 14
  and .final_acknowledgement_accepted_count == 0
  and .final_acknowledgement_recorded_count == 0
  and .final_acknowledgement_persisted_count == 0
  and .final_acknowledgement_materialized_count == 0
  and .final_acknowledgement_delivered_count == 0
  and .operator_received_recorded_count == 0
  and .operator_confirmed_recorded_count == 0
  and .operator_read_recorded_count == 0
  and .completion_ack_recorded_count == 0
  and .status_ack_recorded_count == 0
  and .briefing_ack_recorded_count == 0
  and .readback_ack_recorded_count == 0
  and .channel_ack_delivered_count == 0
  and .external_ack_sent_count == 0
  and .final_acknowledgement_acceptance_recorded_count == 0
  and .final_acknowledgement_operator_approval_derived_count == 0
  and .final_acknowledgement_activation_authority_derived_count == 0
  and .final_acknowledgement_activation_command_derived_count == 0
  and .final_acknowledgement_live_execution_allowed_count == 0
  and (.final_acknowledgement_surfaces | all(
    .final_acknowledgement_attempted == true
    and .final_acknowledgement_accepted == false
    and .final_acknowledgement_recorded == false
    and .final_acknowledgement_persisted == false
    and .final_acknowledgement_materialized == false
    and .final_acknowledgement_delivered == false
    and .operator_received_recorded == false
    and .operator_confirmed_recorded == false
    and .operator_read_recorded == false
    and .completion_ack_recorded == false
    and .status_ack_recorded == false
    and .briefing_ack_recorded == false
    and .readback_ack_recorded == false
    and .channel_ack_delivered == false
    and .external_ack_sent == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .final_acknowledgement_status == "final_acknowledgement_non_acceptance_denied"
  ))
  and (.denied_by_packet_receipt_final_acknowledgement | length) == 17
  and .packet_acceptance_receipt_final_acknowledgement_accepted == false
  and .packet_acceptance_receipt_final_acknowledgement_recorded == false
  and .packet_acceptance_receipt_final_acknowledgement_persisted == false
  and .packet_acceptance_receipt_final_acknowledgement_materialized == false
  and .packet_acceptance_receipt_final_acknowledgement_delivered == false
  and .packet_acceptance_receipt_operator_received_recorded == false
  and .packet_acceptance_receipt_operator_confirmed_recorded == false
  and .packet_acceptance_receipt_operator_read_recorded == false
  and .packet_acceptance_receipt_completion_ack_recorded == false
  and .packet_acceptance_receipt_status_ack_recorded == false
  and .packet_acceptance_receipt_briefing_ack_recorded == false
  and .packet_acceptance_receipt_readback_ack_recorded == false
  and .packet_acceptance_receipt_channel_ack_delivered == false
  and .packet_acceptance_receipt_external_ack_sent == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .activation_authority_derived == false
  and .activation_command_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .hepta_intelligence_context_attached == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .external_kg_adapter_read_performed == false
  and .network_call_performed == false
  and .external_db_write_performed == false
  and .live_kg_write_performed == false
  and .credential_read == false
  and .secret_file_read == false
  and .install_executed == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .external_send_performed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$FINAL_ACKNOWLEDGEMENT_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 143;' \
  "native gateway route/source command count includes packet acceptance receipt final acknowledgement route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_FINAL_ACKNOWLEDGEMENT_NON_ACCEPTANCE_ENDPOINT' \
  "native gateway final acknowledgement endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance' \
  "native gateway final acknowledgement endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance --json' \
  "native gateway final acknowledgement source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_report' \
  "native gateway final acknowledgement report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_route_enabled": true' \
  "packet acceptance receipt final acknowledgement route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"final_acknowledgement_surface_count": final_acknowledgement_surface_count' \
  "packet acceptance receipt final acknowledgement surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_final_acknowledgement_recorded": false' \
  "packet acceptance receipt final acknowledgement recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_channel_ack_delivered": false' \
  "packet acceptance receipt channel acknowledgement delivery denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_external_ack_sent": false' \
  "packet acceptance receipt external acknowledgement send denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_endpoint_blocks_acceptance_and_authority \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 143
    and .implemented_route_count == 143
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_ready == true
    and .source_packet_acceptance_receipt_operator_briefing_ready == true
    and .source_operator_briefing_surface_count == 14
    and .source_briefing_recorded_count == 0
    and .source_briefing_persisted_count == 0
    and .source_completion_ack_recorded_count == 0
    and .source_operator_briefing_activation_authority_derived_count == 0
    and .final_acknowledgement_surface_count == 14
    and .final_acknowledgement_attempt_count == 14
    and .final_acknowledgement_recorded_count == 0
    and .final_acknowledgement_persisted_count == 0
    and .operator_received_recorded_count == 0
    and .operator_confirmed_recorded_count == 0
    and .operator_read_recorded_count == 0
    and .operator_seen_recorded_count == 0
    and .completion_ack_recorded_count == 0
    and .status_ack_recorded_count == 0
    and .briefing_ack_recorded_count == 0
    and .readback_ack_recorded_count == 0
    and .channel_ack_delivered_count == 0
    and .external_ack_sent_count == 0
    and .final_acknowledgement_activation_authority_derived_count == 0
    and .final_acknowledgement_live_execution_allowed_count == 0
    and (.final_acknowledgement_surfaces | all(
      .final_acknowledgement_attempted == true
      and .final_acknowledgement_recorded == false
      and .final_acknowledgement_persisted == false
      and .operator_received_recorded == false
      and .operator_seen_recorded == false
      and .channel_ack_delivered == false
      and .external_ack_sent == false
      and .activation_authority_derived == false
      and .live_execution_allowed == false
      and .final_acknowledgement_status == "final_acknowledgement_non_acceptance_denied"
    ))
    and (.denied_by_packet_receipt_final_acknowledgement | length) == 17
    and .packet_acceptance_receipt_final_acknowledgement_recorded == false
    and .packet_acceptance_receipt_final_acknowledgement_persisted == false
    and .packet_acceptance_receipt_operator_received_recorded == false
    and .packet_acceptance_receipt_operator_seen_recorded == false
    and .packet_acceptance_receipt_channel_ack_delivered == false
    and .packet_acceptance_receipt_external_ack_sent == false
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
  and .required_marker_count == 283
  and .present_required_marker_count == 283
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_final_acknowledgement_gate_sha256="$(printf '%s' "$FINAL_ACKNOWLEDGEMENT_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance --json" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg source_final_acknowledgement_gate_sha256 "$source_final_acknowledgement_gate_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$FINAL_ACKNOWLEDGEMENT_JSON" \
  --argjson terminal "$TERMINAL_COVERAGE_JSON" \
  --argjson live "$LIVE_ROUTE_JSON" \
  --argjson live_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:"hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_route_gate",
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_packet_acceptance_receipt_final_acknowledgement_gate_ready:true,
    source_final_acknowledgement_gate_sha256:$source_final_acknowledgement_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$test_log,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_checked:$live_checked,
    source_route_count_expected:143,
    terminal_required_marker_count_expected:283,
    source_packet_acceptance_receipt_operator_briefing_ready:$source.source_packet_acceptance_receipt_operator_briefing_ready,
    source_operator_briefing_surface_count:$source.source_operator_briefing_surface_count,
    final_acknowledgement_surface_count:$source.final_acknowledgement_surface_count,
    final_acknowledgement_attempt_count:$source.final_acknowledgement_attempt_count,
    final_acknowledgement_recorded_count:$source.final_acknowledgement_recorded_count,
    final_acknowledgement_persisted_count:$source.final_acknowledgement_persisted_count,
    operator_received_recorded_count:$source.operator_received_recorded_count,
    operator_confirmed_recorded_count:$source.operator_confirmed_recorded_count,
    operator_read_recorded_count:$source.operator_read_recorded_count,
    completion_ack_recorded_count:$source.completion_ack_recorded_count,
    status_ack_recorded_count:$source.status_ack_recorded_count,
    channel_ack_delivered_count:$source.channel_ack_delivered_count,
    external_ack_sent_count:$source.external_ack_sent_count,
    final_acknowledgement_activation_authority_derived_count:$source.final_acknowledgement_activation_authority_derived_count,
    final_acknowledgement_live_execution_allowed_count:$source.final_acknowledgement_live_execution_allowed_count,
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

echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt final acknowledgement non-acceptance route gate passed"
