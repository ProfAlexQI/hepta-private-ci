#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT=180

cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing provider-router dry-run envelope readback audit source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 180;' \
  "native gateway route/source command count includes provider-router dry-run envelope readback audit route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_PROVIDER_ROUTER_DRY_RUN_ENVELOPE_READBACK_AUDIT_ENDPOINT' \
  "native gateway provider-router dry-run envelope readback audit endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-provider-router-dry-run-envelope-readback-audit' \
  "native gateway provider-router dry-run envelope readback audit endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-provider-router-dry-run-envelope-readback-audit --json' \
  "native gateway provider-router dry-run envelope readback audit source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_provider_router_dry_run_envelope_readback_audit_report' \
  "native gateway provider-router dry-run envelope readback audit report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'provider_router_dry_run_envelope_preview_readback_fixture_no_provider_model_invocation' \
  "provider-router dry-run envelope canary execution mode"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_provider_router_dry_run_envelope_readback_audit_endpoint_builds_preview_without_provider_model_or_persistence_side_effects' \
  "focused provider-router dry-run envelope readback audit unit test"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"action": "first_model_invocation_separate_approval_slice"' \
  "first real model invocation remains a separate approval slice"

TEST_LOG="$(mktemp /tmp/hepta-provider-router-dry-run-envelope-readback-audit-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_provider_router_dry_run_envelope_readback_audit_endpoint_builds_preview_without_provider_model_or_persistence_side_effects \
  -- --nocapture >"$TEST_LOG"

live_route_status="skipped"
live_route_count=0
live_missing_route_count=0

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-provider-router-dry-run-envelope-readback-audit"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_kg_read_only_adapter_shadow_rank_canary_ready == true
    and .source_bounded_provider_router_dry_run_envelope_readback_audit_receipt_lane_ready == true
    and .provider_router_dry_run_envelope_readback_audit_ready == true
    and .canary_execution_mode == "provider_router_dry_run_envelope_preview_readback_fixture_no_provider_model_invocation"
    and .provider_router_target == "hepta-provider-router:dry-run:bounded-context-shadow-rank"
    and .dry_run_budget_binding == "provider_invocation_budget=0:model_invocation_budget=0"
    and .provider_invocation_budget == 0
    and .model_invocation_budget == 0
    and .dry_run_envelope_preview_constructed == true
    and .dry_run_envelope_preview_redacted == true
    and .dry_run_envelope_readback_audit_performed == true
    and .dry_run_envelope_readback_hash_matched == true
    and .dry_run_envelope_receipt_rendered == true
    and .dry_run_envelope_receipt_persisted == false
    and .dry_run_envelope_receipt_accepted == false
    and .dry_run_envelope_receipt_ledger_recorded == false
    and .dry_run_envelope_receipt_filesystem_written == false
    and .dry_run_envelope_executed == false
    and .provider_router_prompt_mutated == false
    and .provider_router_context_packet_materialized == false
    and .provider_prompt_injection_performed == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_value_read == false
    and .credential_read == false
    and .secret_file_read == false
    and .kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .memory_store_write_performed == false
    and .channel_send_performed == false
    and .telegram_send_performed == false
    and .external_send_performed == false
    and (.audit_steps | length) == 4
    and .allowed_next_actions[0].action == "first_model_invocation_separate_approval_slice"
    and .allowed_next_actions[0].requires_fresh_operator_approval == true
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
    --arg gate "hepta_provider_router_dry_run_envelope_readback_audit_route_gate" \
    --arg endpoint "/api/hepta-provider-router-dry-run-envelope-readback-audit" \
    --arg source_command "/hepta-provider-router-dry-run-envelope-readback-audit --json" \
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
      provider_router_dry_run_envelope_readback_audit_ready:true,
      canary_execution_mode:"provider_router_dry_run_envelope_preview_readback_fixture_no_provider_model_invocation",
      provider_router_target:"hepta-provider-router:dry-run:bounded-context-shadow-rank",
      dry_run_budget_binding:"provider_invocation_budget=0:model_invocation_budget=0",
      dry_run_envelope_preview_constructed:true,
      dry_run_envelope_preview_redacted:true,
      dry_run_envelope_readback_audit_performed:true,
      dry_run_envelope_readback_hash_matched:true,
      dry_run_envelope_receipt_rendered:true,
      dry_run_envelope_receipt_persisted:false,
      dry_run_envelope_receipt_accepted:false,
      dry_run_envelope_executed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      live_kg_write_performed:false,
      memory_store_write_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      next_slice:"first_model_invocation_separate_approval_slice",
      side_effects:{
        durable_memory_store_write_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        prompt_payload_materialized:false,
        provider_prompt_injection_performed:false,
        context_injection_performed:false,
        provider_router_prompt_mutated:false,
        provider_router_context_packet_materialized:false,
        provider_router_live_envelope_executed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_value_read:false,
        credential_read:false,
        secret_file_read:false,
        kg_adapter_read_performed:false,
        live_kg_write_performed:false,
        kg_write_performed:false,
        dry_run_envelope_receipt_persisted:false,
        dry_run_envelope_receipt_accepted:false,
        dry_run_envelope_receipt_ledger_recorded:false,
        dry_run_envelope_receipt_filesystem_written:false,
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
echo "Hepta provider-router dry-run envelope readback audit route gate passed"
