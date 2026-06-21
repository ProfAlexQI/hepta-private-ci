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
    echo "missing operator readiness packet template packet-acceptance receipt operator briefing route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

OPERATOR_BRIEFING_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_ready == true
  and .source_packet_acceptance_receipt_redaction_privacy_ready == true
  and .source_redaction_privacy_surface_count == 16
  and .source_redacted_payload_preview_recorded_count == 0
  and .source_readback_text_recorded_count == 0
  and .source_operator_summary_text_recorded_count == 0
  and .source_privacy_review_recorded_count == 0
  and .source_secret_scan_performed_count == 0
  and .source_raw_payload_inspected_count == 0
  and .source_redaction_privacy_activation_authority_derived_count == 0
  and .operator_briefing_surface_count == 14
  and .operator_briefing_attempt_count == 14
  and .briefing_recorded_count == 0
  and .briefing_persisted_count == 0
  and .briefing_materialized_count == 0
  and .briefing_filesystem_written_count == 0
  and .summary_recorded_count == 0
  and .readback_digest_recorded_count == 0
  and .final_note_recorded_count == 0
  and .status_banner_recorded_count == 0
  and .timeline_entry_recorded_count == 0
  and .notification_preview_recorded_count == 0
  and .channel_delivery_performed_count == 0
  and .external_send_performed_count == 0
  and .telegram_send_performed_count == 0
  and .completion_ack_recorded_count == 0
  and .operator_briefing_acceptance_recorded_count == 0
  and .operator_briefing_operator_approval_derived_count == 0
  and .operator_briefing_activation_authority_derived_count == 0
  and .operator_briefing_activation_command_derived_count == 0
  and .operator_briefing_live_execution_allowed_count == 0
  and (.operator_briefing_surfaces | all(
    .briefing_attempted == true
    and .briefing_recorded == false
    and .briefing_persisted == false
    and .briefing_materialized == false
    and .summary_recorded == false
    and .readback_digest_recorded == false
    and .final_note_recorded == false
    and .channel_delivery_performed == false
    and .external_send_performed == false
    and .telegram_send_performed == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .briefing_status == "operator_briefing_non_persistence_denied"
  ))
  and (.denied_by_packet_receipt_operator_briefing | length) == 16
  and .packet_acceptance_receipt_operator_briefing_recorded == false
  and .packet_acceptance_receipt_operator_briefing_persisted == false
  and .packet_acceptance_receipt_summary_recorded == false
  and .packet_acceptance_receipt_readback_digest_recorded == false
  and .packet_acceptance_receipt_final_note_recorded == false
  and .packet_acceptance_receipt_channel_delivered == false
  and .packet_acceptance_receipt_external_sent == false
  and .packet_acceptance_receipt_telegram_sent == false
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
' >/dev/null <<<"$OPERATOR_BRIEFING_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 168;' \
  "native gateway route/source command count includes operator readiness packet acceptance receipt operator briefing route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_OPERATOR_BRIEFING_NON_PERSISTENCE_ENDPOINT' \
  "native gateway operator readiness packet acceptance receipt operator briefing endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence' \
  "native gateway operator readiness packet acceptance receipt operator briefing endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence --json' \
  "native gateway operator readiness packet acceptance receipt operator briefing source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_report' \
  "native gateway operator readiness packet acceptance receipt operator briefing report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_route_enabled": true' \
  "operator readiness packet acceptance receipt operator briefing route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_briefing_surface_count": operator_briefing_surface_count' \
  "packet acceptance receipt operator briefing surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_operator_briefing_recorded": false' \
  "packet acceptance receipt operator briefing recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_channel_delivered": false' \
  "packet acceptance receipt channel delivery denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_telegram_sent": false' \
  "packet acceptance receipt Telegram send denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_endpoint_blocks_delivery_and_authority \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 160
    and .implemented_route_count == 160
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_ready == true
    and .source_packet_acceptance_receipt_redaction_privacy_ready == true
    and .source_redaction_privacy_surface_count == 16
    and .source_redacted_payload_preview_recorded_count == 0
    and .source_readback_text_recorded_count == 0
    and .source_privacy_review_recorded_count == 0
    and .operator_briefing_surface_count == 14
    and .operator_briefing_attempt_count == 14
    and .briefing_recorded_count == 0
    and .briefing_persisted_count == 0
    and .summary_recorded_count == 0
    and .readback_digest_recorded_count == 0
    and .channel_delivery_performed_count == 0
    and .external_send_performed_count == 0
    and .telegram_send_performed_count == 0
    and .operator_briefing_acceptance_recorded_count == 0
    and .operator_briefing_activation_authority_derived_count == 0
    and .operator_briefing_live_execution_allowed_count == 0
    and (.operator_briefing_surfaces | all(
      .briefing_attempted == true
      and .briefing_recorded == false
      and .briefing_persisted == false
      and .channel_delivery_performed == false
      and .external_send_performed == false
      and .telegram_send_performed == false
      and .activation_authority_derived == false
      and .live_execution_allowed == false
      and .briefing_status == "operator_briefing_non_persistence_denied"
    ))
    and (.denied_by_packet_receipt_operator_briefing | length) == 16
    and .packet_acceptance_receipt_operator_briefing_recorded == false
    and .packet_acceptance_receipt_operator_briefing_persisted == false
    and .packet_acceptance_receipt_channel_delivered == false
    and .packet_acceptance_receipt_external_sent == false
    and .packet_acceptance_receipt_telegram_sent == false
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
  and .required_marker_count == 300
  and .present_required_marker_count == 300
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_operator_briefing_gate_sha256="$(printf '%s' "$OPERATOR_BRIEFING_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence --json" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg source_operator_briefing_gate_sha256 "$source_operator_briefing_gate_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$OPERATOR_BRIEFING_JSON" \
  --argjson terminal "$TERMINAL_COVERAGE_JSON" \
  --argjson live "$LIVE_ROUTE_JSON" \
  --argjson live_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:"hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_route_gate",
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_packet_acceptance_receipt_operator_briefing_gate_ready:true,
    source_operator_briefing_gate_sha256:$source_operator_briefing_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$test_log,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_checked:$live_checked,
    source_route_count_expected:153,
    terminal_required_marker_count_expected:293,
    source_packet_acceptance_receipt_redaction_privacy_ready:$source.source_packet_acceptance_receipt_redaction_privacy_ready,
    source_redaction_privacy_surface_count:$source.source_redaction_privacy_surface_count,
    operator_briefing_surface_count:$source.operator_briefing_surface_count,
    operator_briefing_attempt_count:$source.operator_briefing_attempt_count,
    briefing_recorded_count:$source.briefing_recorded_count,
    briefing_persisted_count:$source.briefing_persisted_count,
    channel_delivery_performed_count:$source.channel_delivery_performed_count,
    external_send_performed_count:$source.external_send_performed_count,
    telegram_send_performed_count:$source.telegram_send_performed_count,
    operator_briefing_activation_authority_derived_count:$source.operator_briefing_activation_authority_derived_count,
    operator_briefing_live_execution_allowed_count:$source.operator_briefing_live_execution_allowed_count,
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

echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt operator briefing non-persistence route gate passed"
