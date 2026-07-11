#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
EXPECTED_ROUTE_COUNT="$(bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
NATIVE_GATEWAY_SOURCE="hepta-native-gateway-source-set-v1"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

cd "$REPO_ROOT"

source scripts/lib/hepta-json-report-capture.sh
source scripts/lib/hepta-source-set.sh

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report \
      "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix-gate" \
      scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_gate"
  and .activation_request_denial_matrix_schema_version == "memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_v1"
  and .runtime_provider_router_activation_request_denial_matrix_ready == true
  and .runtime_provider_router_activation_request_denial_matrix_status == "blocked"
  and .operator_acknowledgement_non_acceptance_ready == true
  and .operator_acknowledgement_non_acceptance_status == "blocked"
  and .operator_acknowledgement_fixture_count == 10
  and .accepted_operator_acknowledgement_fixture_count == 0
  and .operator_acknowledgement_performed_count == 0
  and .activation_request_surface_count == 12
  and .activation_request_surface_ready_count == 12
  and .activation_request_side_effect_free_surface_count == 12
  and .activation_request_fixture_count == 10
  and .blocked_activation_request_fixture_count == 10
  and .noop_activation_request_fixture_count == 10
  and .allowed_activation_request_fixture_count == 0
  and .accepted_activation_request_fixture_count == 0
  and .activation_request_denied_count == 10
  and .activation_request_performed_count == 0
  and .activation_execution_performed_count == 0
  and .activation_request_accepted == false
  and .activation_request_recorded == false
  and .activation_request_persisted == false
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
  and .readback_evidence_recorded == false
  and .readback_evidence_persisted == false
  and .router_handoff_recorded == false
  and .router_handoff_persisted == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.activation_request_surfaces | length) == 12
  and (.activation_request_fixtures | length) == 10
  and (.activation_request_fixtures | all(
    (.activation_request_status | startswith("blocked_"))
    and .activation_request_noop_confirmed == true
    and .activation_request_recorded == false
    and .activation_request_persisted == false
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
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .receipt_recorded == false
    and .receipt_persisted == false
    and .receipt_accepted == false
    and .readback_evidence_recorded == false
    and .readback_evidence_persisted == false
    and .router_handoff_recorded == false
    and .router_handoff_persisted == false
    and .active_binary_mutated == false
  ))
  and (.denied_by_activation_request_denial_matrix | length) == 27
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_activation_command_noop_handoff" and .status == "allowed_report_only_next_slice" and .accepts_activation_request == false and .executes_activation == false and .invokes_model == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SOURCE_JSON"

assert_source_contains() {
  local source_file="$1"
  local needle="$2"
  local description="$3"
  if ! hepta_source_path_contains "$source_file" "$needle"; then
    echo "$source_file missing ${description}: ${needle}" >&2
    exit 1
  fi
}

assert_source_contains \
  "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native source command count"
assert_source_contains \
  "$ROUTE_REGISTRY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix' \
  "runtime provider-router activation request denial matrix endpoint"
assert_source_contains \
  "$ROUTE_REGISTRY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix --json' \
  "runtime provider-router activation request denial matrix source command"
assert_source_contains \
  "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_report' \
  "runtime provider-router activation request denial matrix report function"
assert_source_contains \
  "$NATIVE_GATEWAY_SOURCE" \
  '"runtime_provider_router_activation_request_denial_matrix_route_enabled": true' \
  "runtime provider-router activation request denial matrix route enabled field"
assert_source_contains \
  "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_endpoint_blocks_activation_requests' \
  "runtime provider-router activation request denial matrix focused test"

TEST_LOG="$(mktemp /tmp/hepta-runtime-provider-router-activation-request-denial-matrix-route-tests.XXXXXX)"
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_endpoint_blocks_activation_requests \
  -- --nocapture >"$TEST_LOG"

LIVE_JSON='{}'
live_checked=false
if [[ "${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS \
      "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix"
  )"
  jq -e --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" '
    .runtime == "hepta"
    and .status == "ready"
    and .route_count == $expected_route_count
    and .implemented_route_count == $expected_route_count
    and .missing_route_count == 0
    and .native_gateway_source_command_count == $expected_route_count
    and .runtime_provider_router_activation_request_denial_matrix_route_enabled == true
    and .runtime_provider_router_activation_request_denial_matrix_ready == true
    and .runtime_provider_router_activation_request_denial_matrix_status == "blocked"
    and .activation_request_fixture_count == 10
    and .accepted_activation_request_fixture_count == 0
    and .denied_by_activation_request_denial_matrix_count == 27
    and .activation_request_accepted == false
    and .activation_request_recorded == false
    and .activation_request_persisted == false
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
  --arg gate "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_activation_request_denial_matrix_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix" \
  --arg source_command "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix --json" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg focused_test_log "$TEST_LOG" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson source "$SOURCE_JSON" \
  --argjson live "$LIVE_JSON" \
  --argjson live_endpoint_checked "$live_checked" \
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
    live_endpoint_checked:$live_endpoint_checked,
    live_route_status:(if $live_endpoint_checked then $live.status else "skipped" end),
    live_route_count:(if $live_endpoint_checked then $live.route_count else 0 end),
    live_missing_route_count:(if $live_endpoint_checked then $live.missing_route_count else 0 end),
    expected_route_count:$expected_route_count,
    source_activation_request_denial_matrix_gate:$source.gate,
    source_activation_request_denial_matrix_ready:$source.runtime_provider_router_activation_request_denial_matrix_ready,
    source_activation_request_denial_matrix_status:$source.runtime_provider_router_activation_request_denial_matrix_status,
    route_gate_ready:true,
    runtime_provider_router_activation_request_denial_matrix_ready:true,
    activation_request_surface_count:12,
    activation_request_fixture_count:10,
    accepted_activation_request_fixture_count:0,
    denied_by_activation_request_denial_matrix_count:27,
    next_slice:"runtime_provider_router_activation_command_noop_handoff",
    side_effects:{
      activation_request_recorded:false,
      activation_request_persisted:false,
      activation_request_accepted:false,
      activation_request_executed:false,
      activation_activated:false,
      runtime_router_mutated:false,
      runtime_attachment_performed:false,
      live_context_attached:false,
      context_injection_performed:false,
      adapter_invoked:false,
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
      active_binary_mutated:false,
      filesystem_written:false
    }
  }'

echo "Hepta runtime provider-router activation request denial matrix route gate passed"
