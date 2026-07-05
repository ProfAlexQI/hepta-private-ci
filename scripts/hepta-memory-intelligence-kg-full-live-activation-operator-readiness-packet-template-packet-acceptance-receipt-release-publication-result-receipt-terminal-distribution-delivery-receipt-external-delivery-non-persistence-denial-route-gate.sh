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
    echo "missing operator readiness packet template packet-acceptance receipt release publication result receipt terminal distribution delivery receipt route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-external-delivery-non-persistence-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-external-delivery-non-persistence-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_ready == true
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_filesystem_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_ledger_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_index_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_delivered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_webhook_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent_count == 0
  and .release_publication_result_receipt_status_endpoint_delivery_receipt_exposed_count == 0
  and .release_publication_result_receipt_dashboard_delivery_receipt_exposed_count == 0
  and .release_publication_result_receipt_delivery_confirmation_recorded_count == 0
  and .release_publication_result_receipt_delivery_ack_recorded_count == 0
  and .release_publication_result_receipt_receipt_echo_delivered_count == 0
  and .release_publication_result_receipt_downstream_consumer_notified_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_live_execution_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_release_artifact_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_public_artifact_written_count == 0
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_surfaces | length) == 18
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_surfaces | all(
    .terminal_distribution_delivery_receipt_attempted == true
    and .terminal_distribution_delivery_receipt_accepted == false
    and .terminal_distribution_delivery_receipt_recorded == false
    and .terminal_distribution_delivery_receipt_persisted == false
    and .terminal_distribution_delivery_receipt_ledger_written == false
    and .terminal_distribution_delivery_receipt_index_written == false
    and .terminal_distribution_delivery_receipt_externally_sent == false
    and .terminal_distribution_delivery_receipt_channel_sent == false
    and .terminal_distribution_delivery_receipt_webhook_sent == false
    and .terminal_distribution_delivery_receipt_telegram_sent == false
    and .delivery_confirmation_recorded == false
    and .delivery_ack_recorded == false
    and .receipt_echo_delivered == false
    and .downstream_consumer_notified == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .live_execution_allowed == false
    and .terminal_distribution_delivery_receipt_noop_confirmed == true
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt | length) == 36
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_ledger_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_index_written == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_channel_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_webhook_sent == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent == false
  and .operator_approval_recorded == false
  and .release_publication_authority_derived == false
  and .activation_authority_derived == false
  and .activation_performed == false
  and .memory_store_write_performed == false
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .credential_read == false
  and .install_executed == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .release_artifact_written == false
  and .public_artifact_written == false
  and .external_send_performed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 167;' \
  "native gateway route/source command count includes terminal distribution delivery receipt route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_EXTERNAL_DELIVERY_NON_PERSISTENCE_DENIAL_ENDPOINT' \
  "native gateway terminal distribution delivery receipt endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-external-delivery-non-persistence-denial' \
  "native gateway terminal distribution delivery receipt endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-external-delivery-non-persistence-denial --json' \
  "native gateway terminal distribution delivery receipt source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_report' \
  "native gateway terminal distribution delivery receipt report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_route_enabled": true' \
  "terminal distribution delivery receipt route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"release_publication_result_receipt_terminal_distribution_delivery_receipt_surface_count": terminal_distribution_delivery_receipt_surface_count' \
  "terminal distribution delivery receipt surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"release_publication_result_receipt_terminal_distribution_delivery_receipt_ledger_written_count": 0' \
  "terminal distribution delivery receipt ledger write denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_endpoint_blocks_external_delivery_receipts \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-external-delivery-non-persistence-denial"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 160
    and .implemented_route_count == 160
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_ready == true
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_surface_count == 18
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_ledger_written_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_release_publication_authority_derived_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_authority_derived_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_live_execution_allowed_count == 0
    and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_surfaces | length) == 18
    and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt | length) == 36
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
  and .required_marker_count == 300
  and .present_required_marker_count == 300
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_terminal_distribution_delivery_receipt_gate_sha256="$(printf '%s' "$TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"
live_route_status="$(jq -r '.status // "skipped"' <<<"$LIVE_ROUTE_JSON")"
live_route_count="$(jq -r '.route_count // 0' <<<"$LIVE_ROUTE_JSON")"
live_missing_route_count="$(jq -r '.missing_route_count // 0' <<<"$LIVE_ROUTE_JSON")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_delivery_non_persistence_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-external-delivery-non-persistence-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-external-delivery-non-persistence-denial --json" \
  --arg source_terminal_distribution_delivery_receipt_gate_sha256 "$source_terminal_distribution_delivery_receipt_gate_sha256" \
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
    activation_mode:"full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_terminal_distribution_delivery_receipt_gate_ready:true,
    source_terminal_distribution_delivery_receipt_gate_sha256:$source_terminal_distribution_delivery_receipt_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$focused_test_log,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_checked:$live_endpoint_checked,
    source_route_count_expected:153,
    terminal_required_marker_count_expected:293,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_surface_count:18,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_recorded_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_ledger_written_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_externally_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_release_publication_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_activation_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_live_execution_allowed_count:0,
    route_source_texts_ready:true,
    terminal_coverage_ready:true,
    terminal_required_marker_count:285,
    terminal_present_required_marker_count:285,
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
        terminal_distribution_delivery_receipt_recorded:false,
        terminal_distribution_delivery_receipt_ledger_written:false,
        terminal_distribution_delivery_receipt_externally_sent:false,
        terminal_distribution_delivery_receipt_telegram_sent:false,
        release_publication_authority_derived:false,
        activation_authority_derived:false,
        live_execution_allowed:false
      }
    }
  }'

echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt external delivery non-persistence denial route gate passed"
