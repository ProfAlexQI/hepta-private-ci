#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
EXPECTED_ROUTE_COUNT="$(bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

source scripts/lib/hepta-json-report-capture.sh

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report \
      "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence-gate" \
      scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_gate"
  and .activation_command_result_receipt_no_persistence_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_v1"
  and .runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
  and .runtime_provider_router_activation_command_result_receipt_no_persistence_status == "blocked"
  and .runtime_provider_router_activation_command_noop_handoff_ready == true
  and .runtime_provider_router_activation_command_noop_handoff_status == "blocked"
  and .activation_command_surface_count == 13
  and .activation_command_fixture_count == 10
  and .accepted_activation_command_fixture_count == 0
  and .activation_command_result_receipt_surface_count == 14
  and .activation_command_result_receipt_surface_ready_count == 14
  and .activation_command_result_receipt_side_effect_free_surface_count == 14
  and .activation_command_result_receipt_fixture_count == 10
  and .blocked_activation_command_result_receipt_fixture_count == 10
  and .noop_activation_command_result_receipt_fixture_count == 10
  and .allowed_activation_command_result_receipt_fixture_count == 0
  and .accepted_activation_command_result_receipt_fixture_count == 0
  and .activation_command_result_receipt_denied_count == 10
  and .activation_command_result_receipt_performed_count == 0
  and .activation_command_result_receipt_shape_registered == false
  and .activation_command_result_receipt_schema_accepted == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_result_receipt_materialized == false
  and .activation_command_result_receipt_filesystem_written == false
  and .activation_command_result_receipt_ledger_written == false
  and .activation_command_result_receipt_indexed == false
  and .activation_command_result_receipt_enqueued == false
  and .activation_command_result_receipt_delivered == false
  and .activation_command_result_receipt_exported == false
  and .activation_command_result_receipt_query_registered == false
  and .activation_command_result_receipt_observability_recorded == false
  and .activation_command_completion_ack_recorded == false
  and .activation_command_completion_ack_persisted == false
  and .activation_command_completion_ack_accepted == false
  and .operator_approval_from_receipt_accepted == false
  and .activation_from_receipt_allowed == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_command_handoff_recorded == false
  and .activation_command_handoff_persisted == false
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
  and .credential_read == false
  and .secret_file_read == false
  and .memory_store_write_performed == false
  and .live_kg_write_performed == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.activation_command_result_receipt_surfaces | length) == 14
  and (.activation_command_result_receipt_fixtures | length) == 10
  and (.activation_command_result_receipt_fixtures | all(
    (.activation_command_result_receipt_status | startswith("blocked_"))
    and .activation_command_result_receipt_allowed == false
    and .activation_command_result_receipt_recorded == false
    and .activation_command_result_receipt_persisted == false
    and .activation_command_result_receipt_accepted == false
    and .activation_command_result_receipt_exported == false
    and .activation_command_result_receipt_query_registered == false
    and .activation_command_result_receipt_observability_recorded == false
    and .activation_command_completion_ack_recorded == false
    and .operator_approval_from_receipt_accepted == false
    and .activation_from_receipt_allowed == false
    and .activation_command_enabled == false
    and .activation_command_invoked == false
    and .activation_command_dispatched == false
    and .activation_request_accepted == false
    and .activation_request_executed == false
    and .runtime_router_mutated == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .channel_send_performed == false
    and .service_restart_performed == false
    and .active_binary_mutated == false
    and .receipt_noop_confirmed == true
  ))
  and (.denied_by_activation_command_result_receipt_no_persistence | length) == 35
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial" and .status == "allowed_report_only_next_slice" and .accepts_duplicate_receipt == false and .records_idempotency == false and .persists_replay_state == false and .mutates_runtime == false and .invokes_model == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SOURCE_JSON"

assert_source_contains() {
  local needle="$1"
  local description="$2"
  if ! grep -Fq -- "$needle" codex-rs/hepta-native-gateway/src/native_gateway.rs; then
    echo "native gateway source missing ${description}: ${needle}" >&2
    exit 1
  fi
}

assert_source_contains \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native source command count"
assert_source_contains \
  '/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence' \
  "runtime provider-router activation command result receipt no-persistence endpoint"
assert_source_contains \
  '/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence --json' \
  "runtime provider-router activation command result receipt no-persistence source command"
assert_source_contains \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_report' \
  "runtime provider-router activation command result receipt no-persistence report function"
assert_source_contains \
  '"runtime_provider_router_activation_command_result_receipt_no_persistence_route_enabled": true' \
  "runtime provider-router activation command result receipt no-persistence route enabled field"
assert_source_contains \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_endpoint_blocks_receipts' \
  "runtime provider-router activation command result receipt no-persistence focused test"

TEST_LOG="$(mktemp /tmp/hepta-runtime-provider-router-activation-command-result-receipt-no-persistence-route-tests.XXXXXX)"
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_endpoint_blocks_receipts \
  -- --nocapture >"$TEST_LOG"

LIVE_JSON='{}'
live_checked=false
if [[ "${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS \
      "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence"
  )"
  jq -e --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" '
    .runtime == "hepta"
    and .status == "ready"
    and .route_count == $expected_route_count
    and .implemented_route_count == $expected_route_count
    and .missing_route_count == 0
    and .native_gateway_source_command_count == $expected_route_count
    and .runtime_provider_router_activation_command_result_receipt_no_persistence_route_enabled == true
    and .runtime_provider_router_activation_command_result_receipt_no_persistence_ready == true
    and .runtime_provider_router_activation_command_result_receipt_no_persistence_status == "blocked"
    and .activation_command_result_receipt_fixture_count == 10
    and .accepted_activation_command_result_receipt_fixture_count == 0
    and .denied_by_activation_command_result_receipt_no_persistence_count == 35
    and .activation_command_result_receipt_recorded == false
    and .activation_command_result_receipt_persisted == false
    and .activation_command_result_receipt_accepted == false
    and .activation_command_result_receipt_exported == false
    and .activation_command_result_receipt_query_registered == false
    and .activation_command_result_receipt_observability_recorded == false
    and .activation_command_completion_ack_recorded == false
    and .operator_approval_from_receipt_accepted == false
    and .activation_from_receipt_allowed == false
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

native_gateway_sha256="$(shasum -a 256 codex-rs/hepta-native-gateway/src/native_gateway.rs | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "ready" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_command_result_receipt_no_persistence_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence" \
  --arg source_command "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence --json" \
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
    source_activation_command_result_receipt_no_persistence_gate:$source.gate,
    source_activation_command_result_receipt_no_persistence_ready:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_ready,
    source_activation_command_result_receipt_no_persistence_status:$source.runtime_provider_router_activation_command_result_receipt_no_persistence_status,
    route_gate_ready:true,
    activation_command_result_receipt_surface_count:$source.activation_command_result_receipt_surface_count,
    activation_command_result_receipt_fixture_count:$source.activation_command_result_receipt_fixture_count,
    accepted_activation_command_result_receipt_fixture_count:$source.accepted_activation_command_result_receipt_fixture_count,
    denied_by_activation_command_result_receipt_no_persistence_count:($source.denied_by_activation_command_result_receipt_no_persistence | length),
    next_slice:"runtime_provider_router_activation_command_result_receipt_replay_idempotency_denial",
    side_effects:{
      activation_command_result_receipt_recorded:$source.activation_command_result_receipt_recorded,
      activation_command_result_receipt_persisted:$source.activation_command_result_receipt_persisted,
      activation_command_result_receipt_accepted:$source.activation_command_result_receipt_accepted,
      activation_command_result_receipt_exported:$source.activation_command_result_receipt_exported,
      activation_command_result_receipt_query_registered:$source.activation_command_result_receipt_query_registered,
      activation_command_result_receipt_observability_recorded:$source.activation_command_result_receipt_observability_recorded,
      activation_command_completion_ack_recorded:$source.activation_command_completion_ack_recorded,
      operator_approval_from_receipt_accepted:$source.operator_approval_from_receipt_accepted,
      activation_from_receipt_allowed:$source.activation_from_receipt_allowed,
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
      active_binary_mutated:$source.active_binary_mutated,
      filesystem_written:$source.side_effects.filesystem_written
    }
  }'

echo "Hepta runtime provider-router activation command result receipt no-persistence route gate passed"
