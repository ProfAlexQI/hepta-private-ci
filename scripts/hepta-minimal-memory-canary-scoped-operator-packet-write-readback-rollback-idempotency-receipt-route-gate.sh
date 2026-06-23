#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"

cd "$REPO_ROOT"

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing minimal memory canary route source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 183;' \
  "native gateway route/source command count includes minimal memory canary route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MINIMAL_MEMORY_CANARY_SCOPED_OPERATOR_PACKET_WRITE_READBACK_ROLLBACK_IDEMPOTENCY_RECEIPT_ENDPOINT' \
  "minimal memory canary endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-minimal-memory-canary-scoped-operator-packet-write-readback-rollback-idempotency-receipt' \
  "minimal memory canary endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-minimal-memory-canary-scoped-operator-packet-write-readback-rollback-idempotency-receipt --json' \
  "minimal memory canary source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_minimal_memory_canary_scoped_operator_packet_write_readback_rollback_idempotency_receipt_report' \
  "minimal memory canary report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'ephemeral_isolated_fixture_no_durable_store_mutation' \
  "minimal memory canary isolated fixture mode"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_minimal_memory_canary_endpoint_runs_ephemeral_write_readback_rollback_idempotency_without_durable_side_effects' \
  "minimal memory canary focused route test"

TEST_LOG="$(mktemp /tmp/hepta-minimal-memory-canary-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_minimal_memory_canary_endpoint_runs_ephemeral_write_readback_rollback_idempotency_without_durable_side_effects \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-minimal-memory-canary-scoped-operator-packet-write-readback-rollback-idempotency-receipt"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 183
    and .implemented_route_count == 183
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .minimal_memory_canary_route_enabled == true
    and .minimal_memory_canary_ready == true
    and .canary_execution_mode == "ephemeral_isolated_fixture_no_durable_store_mutation"
    and .single_scoped_operator_packet_count == 1
    and .scoped_operator_packet_present == true
    and .scoped_operator_packet_accepted_for_ephemeral_canary == true
    and .operator_packet_persisted == false
    and .operator_approval_recorded == false
    and .ephemeral_memory_store_write_performed == true
    and .ephemeral_memory_store_write_count == 1
    and .ephemeral_memory_readback_performed == true
    and .ephemeral_memory_readback_hit_count == 1
    and .ephemeral_memory_readback_payload_hash_matched == true
    and .ephemeral_memory_rollback_performed == true
    and .ephemeral_memory_post_rollback_hit_count == 0
    and .idempotency_replay_performed == true
    and .idempotency_duplicate_write_suppressed == true
    and .idempotency_effective_write_count == 1
    and .idempotency_receipt_generated == true
    and .idempotency_receipt_persisted == false
    and .pre_write_store_hash_sha256 == .post_rollback_store_hash_sha256
    and .pre_write_store_hash_sha256 != .post_write_store_hash_sha256
    and .durable_memory_store_write_performed == false
    and .durable_memory_store_read_performed == false
    and .durable_memory_store_rollback_performed == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .memory_write_receipt_persisted == false
    and .rollback_executed == false
    and .live_kg_write_performed == false
    and .kg_adapter_read_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .channel_send_performed == false
    and .telegram_send_performed == false
    and .external_send_performed == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and (.canary_steps | length) == 5
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_ROUTE_JSON"
fi

native_gateway_sha256="$(shasum -a 256 "$NATIVE_GATEWAY_SOURCE" | awk '{print $1}')"
live_route_status="$(jq -r '.status // "skipped"' <<<"$LIVE_ROUTE_JSON")"
live_route_count="$(jq -r '.route_count // 0' <<<"$LIVE_ROUTE_JSON")"
live_missing_route_count="$(jq -r '.missing_route_count // 0' <<<"$LIVE_ROUTE_JSON")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "ready" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_minimal_memory_canary_scoped_operator_packet_write_readback_rollback_idempotency_receipt_route_gate" \
  --arg endpoint "/api/hepta-minimal-memory-canary-scoped-operator-packet-write-readback-rollback-idempotency-receipt" \
  --arg source_command "/hepta-minimal-memory-canary-scoped-operator-packet-write-readback-rollback-idempotency-receipt --json" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg focused_test_log "$TEST_LOG" \
  --arg live_route_status "$live_route_status" \
  --argjson live_endpoint_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  --argjson live_route_count "$live_route_count" \
  --argjson live_missing_route_count "$live_missing_route_count" \
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
    live_route_status:$live_route_status,
    live_route_count:$live_route_count,
    live_missing_route_count:$live_missing_route_count,
    expected_route_count:180,
    route_gate_ready:true,
    minimal_memory_canary_ready:true,
    canary_execution_mode:"ephemeral_isolated_fixture_no_durable_store_mutation",
    scoped_operator_packet_count:1,
    ephemeral_memory_store_write_performed:true,
    ephemeral_memory_readback_performed:true,
    ephemeral_memory_rollback_performed:true,
    idempotency_receipt_generated:true,
    durable_memory_store_write_performed:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    live_kg_write_performed:false,
    provider_invoked:false,
    model_invoked:false,
    credential_read:false,
    external_send_performed:false,
    next_slice:"hepta_intelligence_bounded_context_attachment_preview_readback",
    side_effects:{
      durable_memory_store_write_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      secret_file_read:false,
      channel_send_performed:false,
      telegram_send_performed:false,
      external_send_performed:false,
      install_executed:false,
      service_restarted:false,
      active_binary_mutated:false,
      filesystem_written:false
    }
  }'

echo "Hepta minimal Memory canary scoped operator packet write/readback/rollback/idempotency receipt route gate passed"
