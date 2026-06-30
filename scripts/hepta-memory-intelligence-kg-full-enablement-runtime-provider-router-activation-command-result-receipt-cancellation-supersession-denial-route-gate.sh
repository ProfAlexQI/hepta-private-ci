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
    echo "missing runtime provider-router activation command result receipt cancellation/supersession route source text: $label" >&2
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
      "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial-gate" \
      scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_gate"
  and .runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_status == "blocked"
  and .source_activation_command_result_receipt_ordering_monotonicity_ready == true
  and .source_activation_command_result_receipt_ordering_monotonicity_status == "blocked"
  and .runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
  and .minimum_required_samples >= 24
  and .cancellation_supersession_surface_count == 14
  and .cancellation_supersession_surface_ready_count == 14
  and .cancellation_supersession_side_effect_free_surface_count == 14
  and .cancellation_supersession_fixture_count == 10
  and .blocked_cancellation_supersession_fixture_count == 10
  and .noop_cancellation_supersession_fixture_count == 10
  and .allowed_cancellation_supersession_fixture_count == 0
  and .accepted_cancellation_supersession_fixture_count == 0
  and .cancellation_fixture_count == 5
  and .supersession_fixture_count == 5
  and .cancellation_denied_count == 5
  and .supersession_denied_count == 5
  and .cancellation_performed_count == 0
  and .supersession_performed_count == 0
  and .replacement_receipt_accepted_count == 0
  and .replacement_receipt_recorded_count == 0
  and .replacement_receipt_persisted_count == 0
  and .tombstone_recorded_count == 0
  and .delete_marker_recorded_count == 0
  and .activation_command_result_receipt_cancellation_allowed == false
  and .activation_command_result_receipt_cancellation_recorded == false
  and .activation_command_result_receipt_cancellation_persisted == false
  and .activation_command_result_receipt_cancellation_request_accepted == false
  and .activation_command_result_receipt_supersession_allowed == false
  and .activation_command_result_receipt_supersession_recorded == false
  and .activation_command_result_receipt_supersession_persisted == false
  and .activation_command_result_receipt_supersession_request_accepted == false
  and .activation_command_result_receipt_replacement_receipt_accepted == false
  and .activation_command_result_receipt_replacement_receipt_recorded == false
  and .activation_command_result_receipt_replacement_receipt_persisted == false
  and .activation_command_result_receipt_replacement_hash_accepted == false
  and .activation_command_result_receipt_tombstone_recorded == false
  and .activation_command_result_receipt_delete_marker_recorded == false
  and .activation_from_cancellation_allowed == false
  and .activation_from_supersession_allowed == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_request_accepted == false
  and .activation_request_recorded == false
  and .activation_request_executed == false
  and .runtime_router_mutated == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .secret_file_read == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .install_executed == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.cancellation_supersession_fixtures | length) == 10
  and (.cancellation_supersession_fixtures | all(
    (.cancellation_supersession_status | startswith("blocked_"))
    and .receipt_noop_confirmed == true
  ))
  and (.denied_by_cancellation_supersession | length) == 27
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial" and .status == "allowed_report_only_next_slice" and .writes_audit_trail == false and .persists_evidence == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SOURCE_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = ${EXPECTED_ROUTE_COUNT};" \
  "native source command count"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT' \
  "runtime provider-router activation command result receipt cancellation/supersession endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial' \
  "runtime provider-router activation command result receipt cancellation/supersession endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial --json' \
  "runtime provider-router activation command result receipt cancellation/supersession source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_report' \
  "runtime provider-router activation command result receipt cancellation/supersession report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_route_enabled": true' \
  "runtime provider-router activation command result receipt cancellation/supersession route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_endpoint_blocks_lifecycle' \
  "runtime provider-router activation command result receipt cancellation/supersession focused test"

TEST_LOG="$(mktemp /tmp/hepta-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_endpoint_blocks_lifecycle \
  -- --nocapture >"$TEST_LOG"

LIVE_JSON='{}'
live_checked=false
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS \
      "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial"
  )"
  jq -e --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" '
    .runtime == "hepta"
    and .status == "ready"
    and .route_count == $expected_route_count
    and .implemented_route_count == $expected_route_count
    and .missing_route_count == 0
    and .native_gateway_source_command_count == $expected_route_count
    and .runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_route_enabled == true
    and .runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready == true
    and .runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_status == "blocked"
    and .source_activation_command_result_receipt_ordering_monotonicity_ready == true
    and .cancellation_supersession_fixture_count == 10
    and .blocked_cancellation_supersession_fixture_count == 10
    and .accepted_cancellation_supersession_fixture_count == 0
    and .cancellation_performed_count == 0
    and .supersession_performed_count == 0
    and .replacement_receipt_recorded_count == 0
    and .replacement_receipt_persisted_count == 0
    and .tombstone_recorded_count == 0
    and .delete_marker_recorded_count == 0
    and .activation_command_result_receipt_cancellation_allowed == false
    and .activation_command_result_receipt_supersession_allowed == false
    and .activation_command_result_receipt_replacement_receipt_accepted == false
    and .activation_from_cancellation_allowed == false
    and .activation_from_supersession_allowed == false
    and .activation_command_enabled == false
    and .activation_command_invoked == false
    and .activation_command_dispatched == false
    and .activation_request_executed == false
    and .runtime_router_mutated == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .service_restart_performed == false
    and .active_binary_mutated == false
    and (.denied_by_cancellation_supersession | length) == 27
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_JSON"
  live_checked=true
fi

native_gateway_sha256="$(shasum -a 256 "$NATIVE_GATEWAY_SOURCE" | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "ready" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial --json" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg focused_test_log "$TEST_LOG" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson source "$SOURCE_JSON" \
  --argjson live "$LIVE_JSON" \
  --argjson live_checked "$live_checked" \
  '{
    product:$product,
    runtime:$runtime,
    status:$status,
    base_url:$base_url,
    gate:$gate,
    endpoint:$endpoint,
    source_command:$source_command,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$focused_test_log,
    live_endpoint_checked:$live_checked,
    live_route_status:(if $live_checked then $live.status else "skipped" end),
    live_route_count:(if $live_checked then $live.route_count else 0 end),
    live_missing_route_count:(if $live_checked then $live.missing_route_count else 0 end),
    expected_route_count:$expected_route_count,
    source_activation_command_result_receipt_cancellation_supersession_denial_gate:$source.gate,
    source_activation_command_result_receipt_cancellation_supersession_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_ready,
    source_activation_command_result_receipt_cancellation_supersession_denial_status:$source.runtime_provider_router_activation_command_result_receipt_cancellation_supersession_denial_status,
    source_ordering_monotonicity_ready:$source.source_activation_command_result_receipt_ordering_monotonicity_ready,
    route_gate_ready:true,
    cancellation_supersession_fixture_count:$source.cancellation_supersession_fixture_count,
    blocked_cancellation_supersession_fixture_count:$source.blocked_cancellation_supersession_fixture_count,
    accepted_cancellation_supersession_fixture_count:$source.accepted_cancellation_supersession_fixture_count,
    cancellation_performed_count:$source.cancellation_performed_count,
    supersession_performed_count:$source.supersession_performed_count,
    replacement_receipt_recorded_count:$source.replacement_receipt_recorded_count,
    replacement_receipt_persisted_count:$source.replacement_receipt_persisted_count,
    denied_by_cancellation_supersession_count:($source.denied_by_cancellation_supersession | length),
    next_slice:"runtime_provider_router_activation_command_result_receipt_audit_trail_immutable_evidence_denial",
    side_effects:{
      activation_command_result_receipt_cancellation_recorded:false,
      activation_command_result_receipt_supersession_recorded:false,
      activation_command_result_receipt_replacement_receipt_recorded:false,
      activation_command_result_receipt_tombstone_recorded:false,
      activation_command_result_receipt_delete_marker_recorded:false,
      activation_from_cancellation_allowed:false,
      activation_from_supersession_allowed:false,
      activation_command_enabled:false,
      activation_command_invoked:false,
      activation_command_dispatched:false,
      activation_request_executed:false,
      runtime_router_mutated:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      secret_file_read:false,
      memory_store_write_performed:false,
      live_kg_write_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      install_executed:false,
      service_restarted:false,
      active_binary_mutated:false
    }
  }'

echo "Hepta runtime provider-router activation command result receipt cancellation/supersession denial route gate passed"
