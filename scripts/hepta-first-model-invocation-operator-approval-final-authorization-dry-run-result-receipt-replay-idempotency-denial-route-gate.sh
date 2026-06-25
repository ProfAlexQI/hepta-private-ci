#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT=188

cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing first model invocation final authorization dry-run result receipt replay/idempotency source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 188;' \
  "native gateway route/source command count includes first model invocation final authorization dry-run result receipt replay/idempotency route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT' \
  "native gateway first model invocation final authorization dry-run result receipt replay/idempotency endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-replay-idempotency-denial' \
  "native gateway first model invocation final authorization dry-run result receipt replay/idempotency endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-replay-idempotency-denial --json' \
  "native gateway first model invocation final authorization dry-run result receipt replay/idempotency source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_report' \
  "native gateway first model invocation final authorization dry-run result receipt replay/idempotency report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_no_provider_model_invocation' \
  "first model invocation final authorization dry-run result receipt replay/idempotency execution mode"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_endpoint_blocks_replay_and_invocation_side_effects' \
  "focused first model invocation final authorization dry-run result receipt replay/idempotency unit test"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial"' \
  "next action remains ordering/monotonicity denial before invocation"

TEST_LOG="$(mktemp /tmp/hepta-first-model-invocation-final-authorization-dry-run-result-receipt-replay-idempotency-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_endpoint_blocks_replay_and_invocation_side_effects \
  -- --nocapture >"$TEST_LOG"

live_route_status="skipped"
live_route_count=0
live_missing_route_count=0

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-replay-idempotency-denial"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_no_persistence_ready == true
    and .first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_ready == true
    and .canary_execution_mode == "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_no_provider_model_invocation"
    and .result_receipt_replay_idempotency_state == "final_authorization_dry_run_result_receipt_replay_duplicate_retry_idempotency_denied"
    and .final_authorization_dry_run_result_receipt_replay_idempotency_readback_hash_matched == true
    and .replay_idempotency_fixture_count == 8
    and .blocked_replay_idempotency_fixture_count == 8
    and .noop_replay_idempotency_fixture_count == 8
    and .allowed_replay_idempotency_fixture_count == 0
    and .accepted_replay_idempotency_fixture_count == 0
    and .replay_idempotency_performed_count == 0
    and .duplicate_result_receipt_accepted_count == 0
    and .retry_result_receipt_accepted_count == 0
    and .idempotency_state_recorded_count == 0
    and .idempotency_state_persisted_count == 0
    and (.replay_idempotency_fixtures | length) == 8
    and (.replay_idempotency_fixtures | all(
      (.replay_idempotency_status | startswith("blocked_"))
      and .activation_from_replay_allowed == false
      and .receipt_noop_confirmed == true
    ))
    and .final_authorization_dry_run_result_receipt_replay_allowed == false
    and .final_authorization_dry_run_result_receipt_replayed == false
    and .final_authorization_dry_run_result_receipt_replay_recorded == false
    and .final_authorization_dry_run_result_receipt_replay_persisted == false
    and .final_authorization_dry_run_result_receipt_replay_performed == false
    and .final_authorization_dry_run_result_receipt_duplicate_accepted == false
    and .final_authorization_dry_run_result_receipt_retry_accepted == false
    and .final_authorization_dry_run_result_receipt_idempotency_key_accepted == false
    and .final_authorization_dry_run_result_receipt_idempotency_key_registered == false
    and .final_authorization_dry_run_result_receipt_idempotency_key_recorded == false
    and .final_authorization_dry_run_result_receipt_idempotency_key_persisted == false
    and .final_authorization_dry_run_result_receipt_idempotency_state_recorded == false
    and .final_authorization_dry_run_result_receipt_idempotency_state_persisted == false
    and .final_authorization_dry_run_result_receipt_idempotency_cache_written == false
    and .final_authorization_dry_run_result_receipt_idempotency_cache_hit_promoted == false
    and .final_authorization_dry_run_result_receipt_replay_nonce_accepted == false
    and .final_authorization_dry_run_result_receipt_cross_scope_reuse_accepted == false
    and .final_authorization_dry_run_result_receipt_status_upgrade_accepted == false
    and .final_authorization_dry_run_result_receipt_completed_status_accepted == false
    and .completion_ack_replay_accepted == false
    and .final_authorization_dry_run_result_receipt_ledger_replay_accepted == false
    and .final_authorization_dry_run_result_receipt_index_replay_accepted == false
    and .final_authorization_dry_run_result_receipt_delivery_replay_accepted == false
    and .final_authorization_dry_run_result_receipt_export_replay_accepted == false
    and .final_authorization_dry_run_result_receipt_query_replay_accepted == false
    and .final_authorization_dry_run_result_receipt_observability_replay_accepted == false
    and .final_authorization_from_replay_allowed == false
    and .operator_approval_from_replay_accepted == false
    and .activation_from_replay_allowed == false
    and .final_authorization_from_result_receipt_allowed == false
    and .operator_approval_from_result_receipt_accepted == false
    and .activation_from_result_receipt_allowed == false
    and .final_authorization_dry_run_result_receipt_recorded == false
    and .final_authorization_dry_run_result_receipt_persisted == false
    and .final_authorization_dry_run_result_receipt_accepted == false
    and .completion_ack_recorded == false
    and .completion_ack_persisted == false
    and .completion_ack_accepted == false
    and .fresh_live_accepted_operator_approval_artifact_present == false
    and .single_use_approval_nonce_verified == false
    and .single_use_approval_nonce_consumed == false
    and .operator_identity_session_binding_verified == false
    and .operator_identity_session_bound == false
    and .explicit_invocation_command_accepted == false
    and .final_authorization_preconditions_satisfied == false
    and .final_authorization_denied == true
    and .final_authorization_accepted == false
    and .approval_packet_accepted == false
    and .operator_approval_recorded == false
    and .operator_consent_recorded == false
    and .provider_invocation_authorized == false
    and .model_invocation_authorized == false
    and .provider_invocation_authorized_from_replay == false
    and .model_invocation_authorized_from_replay == false
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
    and (.audit_steps | length) == 6
    and .allowed_next_actions[0].action == "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial"
    and .allowed_next_actions[0].registers_idempotency_key == false
    and .allowed_next_actions[0].writes_idempotency_cache == false
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
    --arg gate "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_route_gate" \
    --arg endpoint "/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-replay-idempotency-denial" \
    --arg source_command "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-replay-idempotency-denial --json" \
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
      first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_ready:true,
      canary_execution_mode:"first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_no_provider_model_invocation",
      result_receipt_replay_idempotency_state:"final_authorization_dry_run_result_receipt_replay_duplicate_retry_idempotency_denied",
      replay_idempotency_fixture_count:8,
      blocked_replay_idempotency_fixture_count:8,
      allowed_replay_idempotency_fixture_count:0,
      accepted_replay_idempotency_fixture_count:0,
      replay_idempotency_performed_count:0,
      duplicate_result_receipt_accepted_count:0,
      retry_result_receipt_accepted_count:0,
      idempotency_state_recorded_count:0,
      idempotency_state_persisted_count:0,
      final_authorization_dry_run_result_receipt_replay_allowed:false,
      final_authorization_dry_run_result_receipt_duplicate_accepted:false,
      final_authorization_dry_run_result_receipt_retry_accepted:false,
      final_authorization_dry_run_result_receipt_idempotency_key_registered:false,
      final_authorization_dry_run_result_receipt_idempotency_cache_written:false,
      final_authorization_from_replay_allowed:false,
      operator_approval_from_replay_accepted:false,
      activation_from_replay_allowed:false,
      provider_invocation_authorized:false,
      model_invocation_authorized:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      live_kg_write_performed:false,
      memory_store_write_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      next_slice:"first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial"
    }'
)"

printf '%s\n' "$report"
echo "Hepta first model invocation operator approval final authorization dry-run result receipt replay/idempotency denial route gate passed"
