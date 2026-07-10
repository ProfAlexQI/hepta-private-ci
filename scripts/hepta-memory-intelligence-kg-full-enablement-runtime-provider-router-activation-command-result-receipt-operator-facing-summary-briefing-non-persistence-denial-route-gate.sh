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
    echo "missing runtime provider-router activation command result receipt operator summary/briefing route source text: $label" >&2
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
      "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate" \
      scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate"
  and .runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status == "blocked"
  and .source_activation_command_result_receipt_export_query_observability_ready == true
  and .source_activation_command_result_receipt_export_query_observability_status == "blocked"
  and .runtime_provider_router_activation_command_result_receipt_export_query_observability_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
  and .minimum_required_samples >= 24
  and .export_query_observability_surface_count == 12
  and .export_query_observability_fixture_count == 10
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
  and .activation_command_result_receipt_operator_summary_recorded == false
  and .activation_command_result_receipt_operator_summary_persisted == false
  and .activation_command_result_receipt_operator_summary_delivered == false
  and .activation_command_result_receipt_operator_briefing_recorded == false
  and .activation_command_result_receipt_operator_briefing_persisted == false
  and .activation_command_result_receipt_operator_briefing_delivered == false
  and .activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .activation_allowed_by_result_receipt_operator_summary == false
  and .activation_allowed_by_result_receipt_operator_briefing == false
  and .activation_allowed_by_result_receipt_summary_briefing == false
  and .activation_allowed_by_result_receipt == false
  and .activation_activated == false
  and .runtime_router_mutated == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .secret_file_read == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .install_executed == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.operator_facing_summary_briefing_surfaces | length) == 12
  and (.operator_facing_summary_briefing_fixtures | length) == 10
  and ([.operator_facing_summary_briefing_fixtures[] | select(.source_export_query_observability_present == false)] | length) == 1
  and ([.operator_facing_summary_briefing_fixtures[] | select(.operator_summary_requested == true)] | length) == 7
  and ([.operator_facing_summary_briefing_fixtures[] | select(.operator_briefing_requested == true)] | length) == 6
  and ([.operator_facing_summary_briefing_fixtures[] | select(.channel_delivery_requested == true and .telegram_send_requested == true)] | length) == 1
  and ([.operator_facing_summary_briefing_fixtures[] | select(.activation_from_summary_briefing_requested == true and .memory_store_summary_requested == true and .live_kg_summary_requested == true and .provider_prompt_summary_requested == true)] | length) == 1
  and ([.operator_facing_summary_briefing_fixtures[] | select(.external_send_summary_requested == true and .install_summary_requested == true and .active_binary_summary_requested == true)] | length) == 1
  and (.denied_by_operator_facing_summary_briefing | length) == 21
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial" and .status == "allowed_report_only_next_slice" and .accepts_operator_acknowledgement == false and .persists_acknowledgement == false and .activates_runtime == false and .invokes_model == false and .writes_kg == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SOURCE_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native source command count"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT' \
  "runtime provider-router activation command result receipt operator summary/briefing endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial' \
  "runtime provider-router activation command result receipt operator summary/briefing endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial --json' \
  "runtime provider-router activation command result receipt operator summary/briefing source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report' \
  "runtime provider-router activation command result receipt operator summary/briefing report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled": true' \
  "runtime provider-router activation command result receipt operator summary/briefing route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_summary_briefing_endpoint_blocks_delivery_and_authority' \
  "runtime provider-router activation command result receipt operator summary/briefing focused test"

TEST_LOG="$(mktemp /tmp/hepta-runtime-provider-router-activation-command-result-receipt-operator-summary-briefing-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_summary_briefing_endpoint_blocks_delivery_and_authority \
  -- --nocapture >"$TEST_LOG"

LIVE_JSON='{}'
live_checked=false
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS \
      "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial"
  )"
  jq -e --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" '
    .runtime == "hepta"
    and .status == "ready"
    and .route_count == $expected_route_count
    and .implemented_route_count == $expected_route_count
    and .missing_route_count == 0
    and .native_gateway_source_command_count == $expected_route_count
    and .runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled == true
    and .runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
    and .runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_status == "blocked"
    and .source_activation_command_result_receipt_export_query_observability_ready == true
    and .operator_facing_summary_briefing_surface_count == 12
    and .operator_facing_summary_briefing_fixture_count == 10
    and .accepted_operator_facing_summary_briefing_fixture_count == 0
    and .operator_summary_performed_count == 0
    and .operator_briefing_performed_count == 0
    and .operator_summary_briefing_delivery_performed_count == 0
    and .activation_command_result_receipt_operator_summary_recorded == false
    and .activation_command_result_receipt_operator_summary_persisted == false
    and .activation_command_result_receipt_operator_briefing_recorded == false
    and .activation_command_result_receipt_operator_briefing_persisted == false
    and .activation_command_result_receipt_operator_summary_briefing_channel_delivery_performed == false
    and .activation_allowed_by_result_receipt_summary_briefing == false
    and .activation_allowed_by_result_receipt == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .telegram_send_performed == false
    and .external_send_performed == false
    and .install_executed == false
    and .service_restart_performed == false
    and .active_binary_mutated == false
    and (.denied_by_operator_facing_summary_briefing | length) == 21
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_JSON"
  live_checked=true
fi

jq -n \
  --arg status "ready" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route_gate" \
  --arg source_gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate" \
  --arg focused_test_log "$TEST_LOG" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson live_checked "$live_checked" \
  --argjson live_route_count "$(jq -r '.route_count // 0' <<<"$LIVE_JSON")" \
  --argjson live_missing_route_count "$(jq -r '.missing_route_count // 0' <<<"$LIVE_JSON")" \
  --argjson accepted_operator_summary_briefing_fixture_count "$(jq -r '.accepted_operator_facing_summary_briefing_fixture_count // 0' <<<"$SOURCE_JSON")" \
  --argjson operator_summary_performed_count "$(jq -r '.operator_summary_performed_count // 0' <<<"$SOURCE_JSON")" \
  --argjson operator_briefing_performed_count "$(jq -r '.operator_briefing_performed_count // 0' <<<"$SOURCE_JSON")" \
  --argjson operator_summary_briefing_delivery_performed_count "$(jq -r '.operator_summary_briefing_delivery_performed_count // 0' <<<"$SOURCE_JSON")" \
  '{
    status:$status,
    gate:$gate,
    source_gate:$source_gate,
    expected_route_count:$expected_route_count,
    focused_test_log:$focused_test_log,
    live_endpoint_checked:$live_checked,
    live_route_count:$live_route_count,
    live_missing_route_count:$live_missing_route_count,
    accepted_operator_summary_briefing_fixture_count:$accepted_operator_summary_briefing_fixture_count,
    operator_summary_performed_count:$operator_summary_performed_count,
    operator_briefing_performed_count:$operator_briefing_performed_count,
    operator_summary_briefing_delivery_performed_count:$operator_summary_briefing_delivery_performed_count
  }'

echo "Hepta memory/intelligence/KG full enablement runtime provider-router activation command result receipt operator-facing summary/briefing non-persistence denial route gate passed"
