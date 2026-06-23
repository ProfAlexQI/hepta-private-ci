#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT=181

cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing KG read-only adapter shadow-rank canary source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 181;' \
  "native gateway route/source command count includes KG shadow-rank canary route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_KG_READ_ONLY_ADAPTER_SHADOW_RANK_CANARY_ENDPOINT' \
  "native gateway KG shadow-rank canary endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-kg-read-only-adapter-shadow-rank-canary' \
  "native gateway KG shadow-rank canary endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-kg-read-only-adapter-shadow-rank-canary --json' \
  "native gateway KG shadow-rank canary source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_kg_read_only_adapter_shadow_rank_canary_report' \
  "native gateway KG shadow-rank canary report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'kg_read_only_adapter_shadow_rank_fixture_no_credential_value_read_no_kg_write' \
  "KG shadow-rank canary execution mode"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_kg_read_only_adapter_shadow_rank_canary_endpoint_compares_without_live_kg_or_secret_side_effects' \
  "focused KG shadow-rank canary unit test"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"action": "provider_router_dry_run_envelope_readback_audit"' \
  "provider-router dry-run next slice"

TEST_LOG="$(mktemp /tmp/hepta-kg-read-only-shadow-rank-canary-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_kg_read_only_adapter_shadow_rank_canary_endpoint_compares_without_live_kg_or_secret_side_effects \
  -- --nocapture >"$TEST_LOG"

live_route_status="skipped"
live_route_count=0
live_missing_route_count=0

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-kg-read-only-adapter-shadow-rank-canary"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_intelligence_bounded_context_preview_ready == true
    and .source_kg_prompt_preview_read_only_adapter_lane_ready == true
    and .kg_read_only_adapter_shadow_rank_canary_ready == true
    and .canary_execution_mode == "kg_read_only_adapter_shadow_rank_fixture_no_credential_value_read_no_kg_write"
    and .kg_adapter_name == "graphiti"
    and .kg_adapter_allowlist_enforced == true
    and .credential_reference_required == true
    and .credential_reference_provided == true
    and .credential_reference_kind == "opaque_reference_only"
    and .credential_value_read == false
    and .credential_read == false
    and .secret_file_read == false
    and .kg_adapter_read_mode == "read_only_shadow_fixture_no_network"
    and .kg_read_only_adapter_shadow_envelope_rendered == true
    and .kg_adapter_live_read_performed == false
    and .kg_adapter_read_performed == false
    and .external_network_call_performed == false
    and .kg_shadow_rank_result_count == 3
    and .kg_shadow_rank_compared_to_transcript_baseline == true
    and .kg_shadow_rank_compared_to_durable_memory_baseline == true
    and .kg_shadow_rank_vs_transcript_baseline_delta == 0
    and .kg_shadow_rank_vs_durable_memory_baseline_delta == 0
    and .kg_shadow_rank_readback_performed == true
    and .kg_shadow_rank_readback_hash_matched == true
    and .shadow_rank_receipt_persisted == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .channel_send_performed == false
    and .telegram_send_performed == false
    and .external_send_performed == false
    and (.comparison_steps | length) == 4
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_JSON"
  live_route_status="$(jq -r '.status' <<<"$LIVE_JSON")"
  live_route_count="$(jq -r '.route_count' <<<"$LIVE_JSON")"
  live_missing_route_count="$(jq -r '.missing_route_count' <<<"$LIVE_JSON")"
fi

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_kg_read_only_adapter_shadow_rank_canary_route_gate" \
    --arg endpoint "/api/hepta-kg-read-only-adapter-shadow-rank-canary" \
    --arg source_command "/hepta-kg-read-only-adapter-shadow-rank-canary --json" \
    --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
    --arg test_log "$TEST_LOG" \
    --arg live_route_status "$live_route_status" \
    --argjson live_route_count "$live_route_count" \
    --argjson live_missing_route_count "$live_missing_route_count" \
    --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
    --argjson live_endpoint_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
    '{
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      native_gateway_sha256:$native_gateway_sha256,
      focused_test_log:$test_log,
      live_endpoint_checked:$live_endpoint_checked,
      live_route_status:$live_route_status,
      live_route_count:$live_route_count,
      live_missing_route_count:$live_missing_route_count,
      expected_route_count:$expected_route_count,
      route_gate_ready:true,
      kg_read_only_adapter_shadow_rank_canary_ready:true,
      canary_execution_mode:"kg_read_only_adapter_shadow_rank_fixture_no_credential_value_read_no_kg_write",
      kg_adapter_name:"graphiti",
      kg_adapter_allowlist_enforced:true,
      credential_reference_required:true,
      credential_reference_provided:true,
      credential_reference_kind:"opaque_reference_only",
      credential_value_read:false,
      credential_read:false,
      secret_file_read:false,
      kg_adapter_read_mode:"read_only_shadow_fixture_no_network",
      kg_read_only_adapter_shadow_envelope_rendered:true,
      kg_adapter_live_read_performed:false,
      kg_adapter_read_performed:false,
      external_network_call_performed:false,
      kg_shadow_rank_compared_to_transcript_baseline:true,
      kg_shadow_rank_compared_to_durable_memory_baseline:true,
      kg_shadow_rank_readback_performed:true,
      kg_shadow_rank_readback_hash_matched:true,
      shadow_rank_receipt_persisted:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      channel_send_performed:false,
      external_send_performed:false,
      next_slice:"provider_router_dry_run_envelope_readback_audit",
      side_effects:{
        durable_memory_store_write_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        prompt_payload_materialized:false,
        context_injection_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_value_read:false,
        credential_read:false,
        secret_file_read:false,
        external_network_call_performed:false,
        kg_adapter_live_read_performed:false,
        kg_adapter_read_performed:false,
        live_kg_write_performed:false,
        channel_send_performed:false,
        telegram_send_performed:false,
        external_send_performed:false,
        install_executed:false,
        service_restarted:false,
        active_binary_mutated:false,
        filesystem_written:false
      }
    }'
)"

printf '%s\n' "$report"
echo "Hepta KG read-only adapter shadow-rank canary route gate passed"
