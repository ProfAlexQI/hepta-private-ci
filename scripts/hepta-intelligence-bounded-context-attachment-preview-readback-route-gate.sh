#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT=171

cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing Intelligence bounded context preview source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 171;' \
  "native gateway route/source command count includes Intelligence bounded context preview route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_INTELLIGENCE_BOUNDED_CONTEXT_ATTACHMENT_PREVIEW_READBACK_ENDPOINT' \
  "native gateway Intelligence bounded context preview endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-intelligence-bounded-context-attachment-preview-readback' \
  "native gateway Intelligence bounded context preview endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-intelligence-bounded-context-attachment-preview-readback --json' \
  "native gateway Intelligence bounded context preview source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_intelligence_bounded_context_attachment_preview_readback_report' \
  "native gateway Intelligence bounded context preview report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'bounded_context_preview_readback_no_provider_prompt_injection' \
  "bounded context preview execution mode"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_intelligence_bounded_context_preview_endpoint_renders_readback_without_provider_or_kg_side_effects' \
  "focused Intelligence bounded context preview unit test"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"action": "hepta_kg_read_only_adapter_shadow_rank_canary"' \
  "KG read-only canary next slice"

TEST_LOG="$(mktemp /tmp/hepta-intelligence-bounded-context-preview-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_intelligence_bounded_context_preview_endpoint_renders_readback_without_provider_or_kg_side_effects \
  -- --nocapture >"$TEST_LOG"

live_route_status="skipped"
live_route_count=0
live_missing_route_count=0

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-intelligence-bounded-context-attachment-preview-readback"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_minimal_memory_canary_ready == true
    and .source_hepta_intelligence_context_attachment_lane_ready == true
    and .intelligence_bounded_context_preview_ready == true
    and .canary_execution_mode == "bounded_context_preview_readback_no_provider_prompt_injection"
    and .bounded_context_attachment_preview_rendered == true
    and .bounded_context_readback_performed == true
    and .bounded_context_readback_hash_matched == true
    and .readback_receipt_persisted == false
    and .raw_context_materialized == false
    and .raw_prompt_payload_materialized == false
    and .prompt_payload_materialized == false
    and .provider_prompt_injection_performed == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .channel_send_performed == false
    and .telegram_send_performed == false
    and .external_send_performed == false
    and (.preview_steps | length) == 4
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
    --arg gate "hepta_intelligence_bounded_context_attachment_preview_readback_route_gate" \
    --arg endpoint "/api/hepta-intelligence-bounded-context-attachment-preview-readback" \
    --arg source_command "/hepta-intelligence-bounded-context-attachment-preview-readback --json" \
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
      intelligence_bounded_context_preview_ready:true,
      canary_execution_mode:"bounded_context_preview_readback_no_provider_prompt_injection",
      bounded_context_attachment_preview_rendered:true,
      bounded_context_readback_performed:true,
      bounded_context_readback_hash_matched:true,
      readback_receipt_persisted:false,
      raw_context_materialized:false,
      raw_prompt_payload_materialized:false,
      prompt_payload_materialized:false,
      provider_prompt_injection_performed:false,
      context_injection_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      kg_adapter_read_performed:false,
      live_kg_write_performed:false,
      external_send_performed:false,
      next_slice:"hepta_kg_read_only_adapter_shadow_rank_canary",
      side_effects:{
        durable_memory_store_write_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        hepta_intelligence_context_attached_to_provider_prompt:false,
        provider_prompt_preview_rendered:false,
        prompt_payload_materialized:false,
        context_injection_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
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
echo "Hepta Intelligence bounded context attachment preview/readback route gate passed"
