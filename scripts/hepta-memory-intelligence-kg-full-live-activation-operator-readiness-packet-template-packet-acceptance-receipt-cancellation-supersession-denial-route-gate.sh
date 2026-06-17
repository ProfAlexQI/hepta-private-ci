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
    echo "missing operator readiness packet template packet-acceptance receipt cancellation/supersession route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

SOURCE_CANCELLATION_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_ready == true
  and .source_packet_acceptance_receipt_ordering_monotonicity_ready == true
  and .source_ordering_surface_count == 14
  and .source_ordering_attempt_count == 14
  and .source_ordering_recorded_count == 0
  and .source_ordering_persisted_count == 0
  and .source_sequence_cursor_recorded_count == 0
  and .source_monotonicity_state_recorded_count == 0
  and .source_ordering_acceptance_recorded_count == 0
  and .source_ordering_activation_authority_derived_count == 0
  and .cancellation_supersession_surface_count == 14
  and .cancellation_supersession_attempt_count == 14
  and .cancellation_accepted_count == 0
  and .cancellation_recorded_count == 0
  and .cancellation_persisted_count == 0
  and .supersession_accepted_count == 0
  and .supersession_recorded_count == 0
  and .supersession_persisted_count == 0
  and .replacement_receipt_accepted_count == 0
  and .replacement_receipt_recorded_count == 0
  and .replacement_receipt_persisted_count == 0
  and .tombstone_recorded_count == 0
  and .tombstone_persisted_count == 0
  and .delete_marker_recorded_count == 0
  and .latest_replacement_accepted_count == 0
  and .cancellation_supersession_acceptance_recorded_count == 0
  and .cancellation_supersession_operator_approval_derived_count == 0
  and .cancellation_supersession_activation_authority_derived_count == 0
  and .cancellation_supersession_activation_command_derived_count == 0
  and .cancellation_supersession_live_execution_allowed_count == 0
  and (.cancellation_surfaces | all(
    .cancellation_or_supersession_attempted == true
    and .cancellation_accepted == false
    and .cancellation_recorded == false
    and .cancellation_persisted == false
    and .supersession_accepted == false
    and .supersession_recorded == false
    and .supersession_persisted == false
    and .replacement_receipt_accepted == false
    and .replacement_receipt_recorded == false
    and .replacement_receipt_persisted == false
    and .tombstone_recorded == false
    and .tombstone_persisted == false
    and .delete_marker_recorded == false
    and .latest_replacement_accepted == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .cancellation_supersession_status == "cancellation_supersession_denied"
  ))
  and (.denied_by_packet_receipt_cancellation_supersession | length) == 20
  and .packet_acceptance_receipt_cancellation_accepted == false
  and .packet_acceptance_receipt_cancellation_recorded == false
  and .packet_acceptance_receipt_cancellation_persisted == false
  and .packet_acceptance_receipt_supersession_accepted == false
  and .packet_acceptance_receipt_supersession_recorded == false
  and .packet_acceptance_receipt_supersession_persisted == false
  and .packet_acceptance_receipt_replacement_accepted == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .activation_authority_derived == false
  and .activation_command_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SOURCE_CANCELLATION_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 138;' \
  "native gateway route/source command count includes operator readiness packet acceptance receipt cancellation/supersession route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT' \
  "native gateway operator readiness packet acceptance receipt cancellation/supersession endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial' \
  "native gateway operator readiness packet acceptance receipt cancellation/supersession endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial --json' \
  "native gateway operator readiness packet acceptance receipt cancellation/supersession source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_report' \
  "native gateway operator readiness packet acceptance receipt cancellation/supersession report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_route_enabled": true' \
  "operator readiness packet acceptance receipt cancellation/supersession route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"cancellation_supersession_surface_count": cancellation_supersession_surface_count' \
  "packet acceptance receipt cancellation/supersession surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_cancellation_accepted": false' \
  "packet acceptance receipt cancellation acceptance denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_supersession_accepted": false' \
  "packet acceptance receipt supersession acceptance denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_endpoint_blocks_cancellation \
  -- --nocapture >"$TEST_LOG"

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 138
    and .implemented_route_count == 138
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_ready == true
    and .source_packet_acceptance_receipt_ordering_monotonicity_ready == true
    and .source_ordering_surface_count == 14
    and .source_ordering_attempt_count == 14
    and .source_ordering_recorded_count == 0
    and .source_ordering_persisted_count == 0
    and .source_sequence_cursor_recorded_count == 0
    and .source_monotonicity_state_recorded_count == 0
    and .source_ordering_acceptance_recorded_count == 0
    and .source_ordering_activation_authority_derived_count == 0
    and .cancellation_supersession_surface_count == 14
    and .cancellation_supersession_attempt_count == 14
    and .cancellation_accepted_count == 0
    and .cancellation_recorded_count == 0
    and .cancellation_persisted_count == 0
    and .supersession_accepted_count == 0
    and .supersession_recorded_count == 0
    and .supersession_persisted_count == 0
    and .replacement_receipt_accepted_count == 0
    and .replacement_receipt_recorded_count == 0
    and .replacement_receipt_persisted_count == 0
    and .tombstone_recorded_count == 0
    and .tombstone_persisted_count == 0
    and .delete_marker_recorded_count == 0
    and .latest_replacement_accepted_count == 0
    and .cancellation_supersession_acceptance_recorded_count == 0
    and .cancellation_supersession_operator_approval_derived_count == 0
    and .cancellation_supersession_activation_authority_derived_count == 0
    and .cancellation_supersession_activation_command_derived_count == 0
    and .cancellation_supersession_live_execution_allowed_count == 0
    and (.cancellation_surfaces | all(
      .cancellation_or_supersession_attempted == true
      and .cancellation_accepted == false
      and .cancellation_recorded == false
      and .cancellation_persisted == false
      and .supersession_accepted == false
      and .supersession_recorded == false
      and .supersession_persisted == false
      and .replacement_receipt_accepted == false
      and .replacement_receipt_recorded == false
      and .replacement_receipt_persisted == false
      and .tombstone_recorded == false
      and .tombstone_persisted == false
      and .delete_marker_recorded == false
      and .latest_replacement_accepted == false
      and .acceptance_recorded == false
      and .operator_approval_derived == false
      and .activation_authority_derived == false
      and .activation_command_derived == false
      and .live_execution_allowed == false
      and .cancellation_supersession_status == "cancellation_supersession_denied"
    ))
    and (.denied_by_packet_receipt_cancellation_supersession | length) == 20
    and .packet_acceptance_receipt_cancellation_accepted == false
    and .packet_acceptance_receipt_cancellation_recorded == false
    and .packet_acceptance_receipt_cancellation_persisted == false
    and .packet_acceptance_receipt_supersession_accepted == false
    and .packet_acceptance_receipt_supersession_recorded == false
    and .packet_acceptance_receipt_supersession_persisted == false
    and .packet_acceptance_receipt_replacement_accepted == false
    and .operator_acceptance_recorded == false
    and .operator_approval_recorded == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .activation_allowed == false
    and .activation_performed == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .public_release_claimed == false
    and .release_artifact_written == false
    and .external_send_performed == false
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
  and .required_marker_count == 278
  and .present_required_marker_count == 278
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_cancellation_gate_sha256="$(printf '%s' "$SOURCE_CANCELLATION_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "ready" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial --json" \
  --arg source_cancellation_gate_sha256 "$source_cancellation_gate_sha256" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$SOURCE_CANCELLATION_JSON" \
  --argjson terminal "$TERMINAL_COVERAGE_JSON" \
  --argjson live "$LIVE_ROUTE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:$status,
    base_url:$base_url,
    gate:$gate,
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_packet_acceptance_receipt_cancellation_supersession_gate_ready: $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_ready,
    source_cancellation_gate_sha256:$source_cancellation_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$test_log,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_checked: ($live != null),
    source_route_count_expected:138,
    terminal_required_marker_count_expected:278,
    source_packet_acceptance_receipt_ordering_monotonicity_ready: $source.source_packet_acceptance_receipt_ordering_monotonicity_ready,
    cancellation_supersession_surface_count: $source.cancellation_supersession_surface_count,
    cancellation_supersession_attempt_count: $source.cancellation_supersession_attempt_count,
    cancellation_accepted_count: $source.cancellation_accepted_count,
    cancellation_recorded_count: $source.cancellation_recorded_count,
    cancellation_persisted_count: $source.cancellation_persisted_count,
    supersession_accepted_count: $source.supersession_accepted_count,
    supersession_recorded_count: $source.supersession_recorded_count,
    supersession_persisted_count: $source.supersession_persisted_count,
    replacement_receipt_recorded_count: $source.replacement_receipt_recorded_count,
    tombstone_recorded_count: $source.tombstone_recorded_count,
    delete_marker_recorded_count: $source.delete_marker_recorded_count,
    cancellation_supersession_activation_authority_derived_count: $source.cancellation_supersession_activation_authority_derived_count,
    cancellation_supersession_live_execution_allowed_count: $source.cancellation_supersession_live_execution_allowed_count,
    route_source_texts_ready:true,
    terminal_coverage_ready: $terminal.preflight_terminal_coverage_inventory_ready,
    terminal_required_marker_count: $terminal.required_marker_count,
    terminal_present_required_marker_count: $terminal.present_required_marker_count,
    terminal_missing_required_marker_count: $terminal.missing_required_marker_count,
    side_effects:{
      route_gate_filesystem_written:false,
      route_gate_runtime_mutated:false,
      route_gate_service_restarted:false,
      route_gate_external_send_performed:false,
      source_gate_side_effects: $source.side_effects,
      live_route_side_effects: (if $live == null then null else $live.side_effects end)
    }
  }'

echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt cancellation/supersession denial route gate passed"
