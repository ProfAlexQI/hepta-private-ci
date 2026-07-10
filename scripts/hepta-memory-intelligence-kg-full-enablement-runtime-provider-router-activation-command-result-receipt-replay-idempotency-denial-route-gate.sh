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
    echo "missing runtime provider-router activation command result receipt replay/idempotency route source text: $label" >&2
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
      "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial-gate" \
      scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_gate"
  and .runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
  and .runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status == "blocked"
  and .source_activation_command_result_receipt_no_persistence_ready == true
  and .source_activation_command_result_receipt_no_persistence_status == "blocked"
  and .activation_command_result_receipt_surface_count == 14
  and .activation_command_result_receipt_fixture_count == 10
  and .replay_idempotency_surface_count == 14
  and .replay_idempotency_fixture_count == 10
  and .blocked_replay_idempotency_fixture_count == 10
  and .noop_replay_idempotency_fixture_count == 10
  and .allowed_replay_idempotency_fixture_count == 0
  and .accepted_replay_idempotency_fixture_count == 0
  and .replay_idempotency_performed_count == 0
  and .duplicate_result_receipt_accepted_count == 0
  and .idempotency_state_recorded_count == 0
  and .activation_command_result_receipt_replay_allowed == false
  and .activation_command_result_receipt_replay_recorded == false
  and .activation_command_result_receipt_replay_persisted == false
  and .activation_command_result_receipt_replay_performed == false
  and .activation_command_result_receipt_duplicate_accepted == false
  and .activation_command_result_receipt_idempotency_key_recorded == false
  and .activation_command_result_receipt_idempotency_state_recorded == false
  and .activation_command_result_receipt_idempotency_state_persisted == false
  and .activation_command_result_receipt_replay_nonce_accepted == false
  and .activation_command_result_receipt_cross_scope_reuse_accepted == false
  and .activation_command_result_receipt_status_upgrade_accepted == false
  and .activation_command_result_receipt_completed_status_accepted == false
  and .activation_command_result_receipt_ack_replay_accepted == false
  and .activation_command_result_receipt_ledger_replay_accepted == false
  and .activation_command_result_receipt_index_replay_accepted == false
  and .activation_command_result_receipt_delivery_replay_accepted == false
  and .activation_command_result_receipt_query_replay_accepted == false
  and .activation_command_result_receipt_observability_replay_accepted == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_completion_ack_recorded == false
  and .operator_approval_from_replay_accepted == false
  and .activation_from_replay_allowed == false
  and .activation_from_receipt_allowed == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_request_accepted == false
  and .activation_request_recorded == false
  and .activation_request_executed == false
  and .activation_activated == false
  and .runtime_router_mutated == false
  and .runtime_attachment_performed == false
  and .live_context_attached == false
  and .context_injection_performed == false
  and .adapter_invoked == false
  and .provider_invoked == false
  and .model_invoked == false
  and .auth_secret_read == false
  and .credential_read == false
  and .secret_file_read == false
  and .usage_recorded == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .receipt_recorded == false
  and .receipt_persisted == false
  and .receipt_accepted == false
  and .rollback_executed == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.replay_idempotency_fixtures | length) == 10
  and (.replay_idempotency_fixtures | all(
    (.replay_status | startswith("blocked_"))
    and .activation_command_result_receipt_replay_allowed == false
    and .activation_command_result_receipt_duplicate_accepted == false
    and .activation_command_result_receipt_idempotency_state_recorded == false
    and .activation_command_result_receipt_idempotency_state_persisted == false
    and .activation_command_result_receipt_cross_scope_reuse_accepted == false
    and .activation_from_replay_allowed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .receipt_noop_confirmed == true
  ))
  and (.denied_by_replay_idempotency | length) == 26
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial" and .status == "allowed_report_only_next_slice" and .persists_ordering_state == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SOURCE_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native source command count"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT' \
  "runtime provider-router activation command result receipt replay/idempotency endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial' \
  "runtime provider-router activation command result receipt replay/idempotency endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial --json' \
  "runtime provider-router activation command result receipt replay/idempotency source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_report' \
  "runtime provider-router activation command result receipt replay/idempotency report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_route_enabled": true' \
  "runtime provider-router activation command result receipt replay/idempotency route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_endpoint_blocks_replay' \
  "runtime provider-router activation command result receipt replay/idempotency focused test"

TEST_LOG="$(mktemp /tmp/hepta-runtime-provider-router-activation-command-result-receipt-replay-idempotency-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_endpoint_blocks_replay \
  -- --nocapture >"$TEST_LOG"

LIVE_JSON='{}'
live_checked=false
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS \
      "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial"
  )"
  jq -e --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" '
    .runtime == "hepta"
    and .status == "ready"
    and .route_count == $expected_route_count
    and .implemented_route_count == $expected_route_count
    and .missing_route_count == 0
    and .native_gateway_source_command_count == $expected_route_count
    and .runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_route_enabled == true
    and .runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and .runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status == "blocked"
    and .source_activation_command_result_receipt_no_persistence_ready == true
    and .replay_idempotency_fixture_count == 10
    and .accepted_replay_idempotency_fixture_count == 0
  and .replay_idempotency_performed_count == 0
  and .duplicate_result_receipt_accepted_count == 0
  and .idempotency_state_recorded_count == 0
    and .activation_command_result_receipt_replay_recorded == false
    and .activation_command_result_receipt_duplicate_accepted == false
    and .activation_command_result_receipt_idempotency_state_recorded == false
    and .activation_from_replay_allowed == false
    and .activation_command_enabled == false
    and .activation_command_invoked == false
    and .activation_command_dispatched == false
    and .activation_request_executed == false
    and .activation_activated == false
    and .runtime_router_mutated == false
    and .context_injection_performed == false
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
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial --json" \
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
    source_activation_command_result_receipt_replay_idempotency_denial_gate:$source.gate,
    source_activation_command_result_receipt_replay_idempotency_denial_ready:$source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_ready,
    source_activation_command_result_receipt_replay_idempotency_denial_status:$source.runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial_status,
    source_activation_command_result_receipt_no_persistence_ready:$source.source_activation_command_result_receipt_no_persistence_ready,
    route_gate_ready:true,
    replay_idempotency_fixture_count:$source.replay_idempotency_fixture_count,
    blocked_replay_idempotency_fixture_count:$source.blocked_replay_idempotency_fixture_count,
    accepted_replay_idempotency_fixture_count:$source.accepted_replay_idempotency_fixture_count,
    replay_idempotency_performed_count:$source.replay_idempotency_performed_count,
    denied_by_replay_idempotency_count:($source.denied_by_replay_idempotency | length),
    next_slice:"runtime_provider_router_activation_command_result_receipt_ordering_monotonicity_denial",
    side_effects:{
      activation_command_result_receipt_replay_recorded:$source.activation_command_result_receipt_replay_recorded,
      activation_command_result_receipt_duplicate_accepted:$source.activation_command_result_receipt_duplicate_accepted,
      activation_command_result_receipt_idempotency_state_recorded:$source.activation_command_result_receipt_idempotency_state_recorded,
      activation_command_result_receipt_idempotency_state_persisted:$source.activation_command_result_receipt_idempotency_state_persisted,
      activation_from_replay_allowed:$source.activation_from_replay_allowed,
      activation_command_enabled:$source.activation_command_enabled,
      activation_command_invoked:$source.activation_command_invoked,
      activation_command_dispatched:$source.activation_command_dispatched,
      activation_request_executed:$source.activation_request_executed,
      activation_activated:$source.activation_activated,
      runtime_router_mutated:$source.runtime_router_mutated,
      live_context_attached:$source.live_context_attached,
      context_injection_performed:$source.context_injection_performed,
      adapter_invoked:$source.adapter_invoked,
      provider_invoked:$source.provider_invoked,
      model_invoked:$source.model_invoked,
      credential_read:$source.credential_read,
      secret_file_read:$source.secret_file_read,
      memory_store_write_performed:$source.memory_store_write_performed,
      live_kg_write_performed:$source.live_kg_write_performed,
      channel_send_performed:$source.channel_send_performed,
      external_send_performed:$source.external_send_performed,
      install_executed:$source.install_executed,
      service_restarted:$source.service_restart_performed,
      active_binary_mutated:$source.active_binary_mutated
    }
  }'

echo "Hepta runtime provider-router activation command result receipt replay/idempotency denial route gate passed"
