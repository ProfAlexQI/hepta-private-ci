#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT=176

cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing first model invocation approval preflight source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 176;' \
  "native gateway route/source command count includes first model invocation approval preflight route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_FIRST_MODEL_INVOCATION_SEPARATE_APPROVAL_SLICE_PREFLIGHT_ENDPOINT' \
  "native gateway first model invocation approval preflight endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-first-model-invocation-separate-approval-slice-preflight' \
  "native gateway first model invocation approval preflight endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-first-model-invocation-separate-approval-slice-preflight --json' \
  "native gateway first model invocation approval preflight source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_first_model_invocation_separate_approval_slice_preflight_report' \
  "native gateway first model invocation approval preflight report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'first_model_invocation_separate_approval_preflight_no_provider_model_invocation' \
  "first model invocation approval preflight execution mode"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_first_model_invocation_separate_approval_slice_preflight_endpoint_requires_approval_without_invocation_side_effects' \
  "focused first model invocation approval preflight unit test"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"action": "first_model_invocation_operator_approval_packet_review"' \
  "next action remains approval packet review before invocation"

TEST_LOG="$(mktemp /tmp/hepta-first-model-invocation-separate-approval-slice-preflight-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_first_model_invocation_separate_approval_slice_preflight_endpoint_requires_approval_without_invocation_side_effects \
  -- --nocapture >"$TEST_LOG"

live_route_status="skipped"
live_route_count=0
live_missing_route_count=0

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-first-model-invocation-separate-approval-slice-preflight"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_provider_router_dry_run_envelope_readback_audit_ready == true
    and .first_model_invocation_separate_approval_slice_preflight_ready == true
    and .canary_execution_mode == "first_model_invocation_separate_approval_preflight_no_provider_model_invocation"
    and .approval_state == "requires_fresh_operator_approval_and_explicit_command"
    and .fresh_operator_approval_required == true
    and .explicit_command_required == true
    and .single_use_approval_nonce_required == true
    and .operator_identity_session_binding_required == true
    and .approval_packet_preview_constructed == true
    and .approval_packet_preview_redacted == true
    and .approval_packet_readback_audit_performed == true
    and .approval_packet_readback_hash_matched == true
    and .approval_packet_receipt_rendered == true
    and .approval_packet_accepted == false
    and .approval_packet_persisted == false
    and .approval_packet_ledger_recorded == false
    and .approval_packet_filesystem_written == false
    and .candidate_provider_invocation_requested == true
    and .candidate_model_invocation_requested == true
    and .provider_invocation_authorized == false
    and .model_invocation_authorized == false
    and .provider_invocation_budget == 0
    and .model_invocation_budget == 0
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_value_read == false
    and .credential_read == false
    and .secret_file_read == false
    and .provider_router_live_envelope_executed == false
    and .provider_prompt_injection_performed == false
    and .context_injection_performed == false
    and .kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .memory_store_write_performed == false
    and .channel_send_performed == false
    and .telegram_send_performed == false
    and .external_send_performed == false
    and (.audit_steps | length) == 4
    and .allowed_next_actions[0].action == "first_model_invocation_operator_approval_packet_review"
    and .allowed_next_actions[0].requires_fresh_operator_approval == true
    and .allowed_next_actions[0].requires_explicit_command == true
    and .allowed_next_actions[0].invokes_provider == false
    and .allowed_next_actions[0].invokes_model == false
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
    --arg gate "hepta_first_model_invocation_separate_approval_slice_preflight_route_gate" \
    --arg endpoint "/api/hepta-first-model-invocation-separate-approval-slice-preflight" \
    --arg source_command "/hepta-first-model-invocation-separate-approval-slice-preflight --json" \
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
      first_model_invocation_separate_approval_slice_preflight_ready:true,
      canary_execution_mode:"first_model_invocation_separate_approval_preflight_no_provider_model_invocation",
      approval_state:"requires_fresh_operator_approval_and_explicit_command",
      fresh_operator_approval_required:true,
      explicit_command_required:true,
      single_use_approval_nonce_required:true,
      approval_packet_preview_constructed:true,
      approval_packet_preview_redacted:true,
      approval_packet_readback_audit_performed:true,
      approval_packet_readback_hash_matched:true,
      approval_packet_receipt_rendered:true,
      approval_packet_accepted:false,
      approval_packet_persisted:false,
      candidate_provider_invocation_requested:true,
      candidate_model_invocation_requested:true,
      provider_invocation_authorized:false,
      model_invocation_authorized:false,
      provider_invocation_budget:0,
      model_invocation_budget:0,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      live_kg_write_performed:false,
      memory_store_write_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      next_slice:"first_model_invocation_operator_approval_packet_review",
      side_effects:{
        approval_packet_accepted:false,
        approval_packet_persisted:false,
        approval_packet_ledger_recorded:false,
        approval_packet_filesystem_written:false,
        operator_approval_recorded:false,
        operator_consent_recorded:false,
        provider_invocation_authorized:false,
        model_invocation_authorized:false,
        provider_router_live_envelope_executed:false,
        provider_prompt_injection_performed:false,
        context_injection_performed:false,
        provider_invoked:false,
        model_invoked:false,
        usage_record_persisted:false,
        credential_value_read:false,
        credential_read:false,
        secret_file_read:false,
        kg_adapter_read_performed:false,
        live_kg_write_performed:false,
        kg_write_performed:false,
        durable_memory_store_write_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
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
echo "Hepta first model invocation separate approval slice preflight route gate passed"
