#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT=189

cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing first model invocation final authorization dry-run result receipt source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 189;' \
  "native gateway route/source command count includes first model invocation final authorization dry-run result receipt route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT' \
  "native gateway first model invocation final authorization dry-run result receipt endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-no-persistence' \
  "native gateway first model invocation final authorization dry-run result receipt endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-no-persistence --json' \
  "native gateway first model invocation final authorization dry-run result receipt source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_report' \
  "native gateway first model invocation final authorization dry-run result receipt report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_no_provider_model_invocation' \
  "first model invocation final authorization dry-run result receipt execution mode"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_endpoint_blocks_receipt_persistence_and_invocation_side_effects' \
  "focused first model invocation final authorization dry-run result receipt unit test"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial"' \
  "next action remains replay/idempotency denial before invocation"

TEST_LOG="$(mktemp /tmp/hepta-first-model-invocation-final-authorization-dry-run-result-receipt-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_endpoint_blocks_receipt_persistence_and_invocation_side_effects \
  -- --nocapture >"$TEST_LOG"

live_route_status="skipped"
live_route_count=0
live_missing_route_count=0

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-no-persistence"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_first_model_invocation_approval_final_authorization_dry_run_envelope_ready == true
    and .first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_ready == true
    and .canary_execution_mode == "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_no_provider_model_invocation"
    and .result_receipt_state == "final_authorization_dry_run_result_receipt_rendered_but_not_persisted_or_accepted"
    and .final_authorization_dry_run_result_receipt_rendered == true
    and .final_authorization_dry_run_result_receipt_hash_matched == true
    and .final_authorization_dry_run_result_receipt_readback_performed == true
    and .final_authorization_dry_run_result_receipt_readback_hash_matched == true
    and .final_authorization_dry_run_result_receipt_recorded == false
    and .final_authorization_dry_run_result_receipt_persisted == false
    and .final_authorization_dry_run_result_receipt_accepted == false
    and .final_authorization_dry_run_result_receipt_materialized == false
    and .final_authorization_dry_run_result_receipt_filesystem_written == false
    and .final_authorization_dry_run_result_receipt_ledger_recorded == false
    and .final_authorization_dry_run_result_receipt_indexed == false
    and .final_authorization_dry_run_result_receipt_enqueued == false
    and .final_authorization_dry_run_result_receipt_delivered == false
    and .final_authorization_dry_run_result_receipt_exported == false
    and .final_authorization_dry_run_result_receipt_query_registered == false
    and .final_authorization_dry_run_result_receipt_observability_recorded == false
    and .completion_ack_recorded == false
    and .completion_ack_persisted == false
    and .completion_ack_accepted == false
    and .fresh_live_accepted_operator_approval_artifact_required == true
    and .fresh_live_accepted_operator_approval_artifact_present == false
    and .fresh_live_accepted_operator_approval_artifact_verified == false
    and .single_use_approval_nonce_required == true
    and .single_use_approval_nonce_verified == false
    and .single_use_approval_nonce_consumed == false
    and .operator_identity_session_binding_required == true
    and .operator_identity_session_binding_verified == false
    and .operator_identity_session_bound == false
    and .explicit_invocation_command_required == true
    and .explicit_invocation_command_accepted == false
    and .final_authorization_candidate_present == true
    and .final_authorization_preconditions_satisfied == false
    and .final_authorization_denied == true
    and .final_authorization_accepted == false
    and .final_authorization_from_result_receipt_allowed == false
    and .operator_approval_from_result_receipt_accepted == false
    and .activation_from_result_receipt_allowed == false
    and .approval_packet_accepted == false
    and .operator_approval_recorded == false
    and .operator_consent_recorded == false
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
    and (.audit_steps | length) == 6
    and .allowed_next_actions[0].action == "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial"
    and .allowed_next_actions[0].records_result_receipt == false
    and .allowed_next_actions[0].persists_result_receipt == false
    and .allowed_next_actions[0].accepts_result_receipt == false
    and .allowed_next_actions[0].requires_fresh_accepted_operator_approval_artifact == true
    and .allowed_next_actions[0].requires_single_use_approval_nonce == true
    and .allowed_next_actions[0].requires_operator_identity_session_binding == true
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
    --arg gate "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_route_gate" \
    --arg endpoint "/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-no-persistence" \
    --arg source_command "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-no-persistence --json" \
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
      first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_ready:true,
      canary_execution_mode:"first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_no_provider_model_invocation",
      result_receipt_state:"final_authorization_dry_run_result_receipt_rendered_but_not_persisted_or_accepted",
      final_authorization_dry_run_result_receipt_rendered:true,
      final_authorization_dry_run_result_receipt_recorded:false,
      final_authorization_dry_run_result_receipt_persisted:false,
      final_authorization_dry_run_result_receipt_accepted:false,
      final_authorization_from_result_receipt_allowed:false,
      operator_approval_from_result_receipt_accepted:false,
      activation_from_result_receipt_allowed:false,
      provider_invocation_authorized:false,
      model_invocation_authorized:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      live_kg_write_performed:false,
      memory_store_write_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      next_slice:"first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial"
    }'
)"

printf '%s\n' "$report"
echo "Hepta first model invocation operator approval final authorization dry-run result receipt no-persistence route gate passed"
