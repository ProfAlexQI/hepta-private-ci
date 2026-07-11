#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT="${HEPTA_EXPECTED_ROUTE_COUNT:-$(bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")}"

cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing first model invocation final authorization dry-run result receipt final operator acknowledgement source text: $label" >&2
    exit 1
  fi
}

source "$REPO_ROOT/scripts/lib/hepta-source-set.sh"
NATIVE_GATEWAY_SOURCE="hepta-native-gateway-source-set-v1"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes final operator acknowledgement route"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  'HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT' \
  "final operator acknowledgement endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-denial' \
  "final operator acknowledgement endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json' \
  "final operator acknowledgement source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report' \
  "final operator acknowledgement report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_no_ack_no_accept_no_delivery_no_provider_model_invocation' \
  "final operator acknowledgement execution mode"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_endpoint_blocks_acknowledgement_and_authority' \
  "focused final operator acknowledgement unit test"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial"' \
  "next action remains terminal operator decision/public-claim non-promotion denial"

TEST_LOG="$(mktemp /tmp/hepta-first-model-invocation-final-authorization-dry-run-result-receipt-final-operator-acknowledgement-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_endpoint_blocks_acknowledgement_and_authority \
  -- --nocapture >"$TEST_LOG"

live_route_status="skipped"
live_route_count=0
live_missing_route_count=0

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-denial"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
    and .first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
    and .canary_execution_mode == "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_no_ack_no_accept_no_delivery_no_provider_model_invocation"
    and .result_receipt_final_operator_acknowledgement_state == "final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denied"
    and .final_operator_acknowledgement_fixture_count == 8
    and .blocked_final_operator_acknowledgement_fixture_count == 8
    and .noop_final_operator_acknowledgement_fixture_count == 8
    and .allowed_final_operator_acknowledgement_fixture_count == 0
    and .accepted_final_operator_acknowledgement_fixture_count == 0
    and .final_operator_acknowledgement_performed_count == 0
    and .final_operator_acknowledgement_accepted_count == 0
    and .final_operator_acknowledgement_recorded_count == 0
    and .final_operator_acknowledgement_persisted_count == 0
    and .final_operator_acknowledgement_delivered_count == 0
    and .final_operator_acknowledgement_final_state_promoted_count == 0
    and .final_operator_acknowledgement_completion_promoted_count == 0
    and (.final_operator_acknowledgement_fixtures | length) == 8
    and (.final_operator_acknowledgement_fixtures | all(
      (.final_operator_acknowledgement_status | startswith("blocked_"))
      and .final_acknowledgement_noop_confirmed == true
    ))
    and ([.final_operator_acknowledgement_fixtures[] | select(.source_operator_facing_summary_briefing_present == false)] | length) == 1
    and ([.final_operator_acknowledgement_fixtures[] | select(.telegram_acknowledgement_requested == true)] | length) == 1
    and ([.final_operator_acknowledgement_fixtures[] | select(.activation_from_acknowledgement_requested == true)] | length) == 1
    and .final_authorization_dry_run_result_receipt_final_operator_acknowledgement_readback_hash_matched == true
    and .final_operator_acknowledgement_allowed == false
    and .final_operator_acknowledgement_request_accepted == false
    and .final_operator_acknowledgement_accepted == false
    and .final_operator_acknowledgement_recorded == false
    and .final_operator_acknowledgement_persisted == false
    and .final_operator_acknowledgement_materialized == false
    and .final_operator_acknowledgement_filesystem_written == false
    and .final_operator_acknowledgement_delivered == false
    and .final_operator_acknowledgement_channel_delivery_performed == false
    and .final_operator_acknowledgement_identity_accepted == false
    and .final_operator_acknowledgement_signature_accepted == false
    and .final_operator_acknowledgement_timestamp_accepted == false
    and .final_operator_acknowledgement_final_state_promoted == false
    and .final_operator_acknowledgement_completion_promoted == false
    and .final_operator_acceptance_recorded == false
    and .final_operator_acceptance_persisted == false
    and .completion_acknowledgement_recorded == false
    and .status_acknowledgement_recorded == false
    and .summary_acknowledgement_recorded == false
    and .briefing_acknowledgement_recorded == false
    and .readback_digest_acknowledgement_recorded == false
    and .dashboard_acknowledgement_recorded == false
    and .notification_acknowledgement_recorded == false
    and .channel_acknowledgement_delivered == false
    and .external_acknowledgement_sent == false
    and .telegram_acknowledgement_sent == false
    and .operator_approval_from_acknowledgement_derived == false
    and .activation_authority_from_acknowledgement_derived == false
    and .provider_invocation_authorized == false
    and .model_invocation_authorized == false
    and .provider_invocation_authorized_from_acknowledgement == false
    and .model_invocation_authorized_from_acknowledgement == false
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
    and .release_artifact_written == false
    and .public_claim_recorded == false
    and .public_release_claimed == false
    and .install_executed == false
    and .service_restart_performed == false
    and .active_binary_mutated == false
    and (.audit_steps | length) == 6
    and .allowed_next_actions[0].action == "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial"
    and .allowed_next_actions[0].accepts_acknowledgement == false
    and .allowed_next_actions[0].claims_public_release == false
    and .allowed_next_actions[0].writes_release_artifact == false
    and .allowed_next_actions[0].activates_runtime == false
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
    --arg gate "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_gate" \
    --arg endpoint "/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-denial" \
    --arg source_command "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json" \
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
      first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready:true,
      canary_execution_mode:"first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_no_ack_no_accept_no_delivery_no_provider_model_invocation",
      result_receipt_final_operator_acknowledgement_state:"final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denied",
      final_operator_acknowledgement_fixture_count:8,
      blocked_final_operator_acknowledgement_fixture_count:8,
      allowed_final_operator_acknowledgement_fixture_count:0,
      accepted_final_operator_acknowledgement_fixture_count:0,
      final_operator_acknowledgement_performed_count:0,
      final_operator_acknowledgement_recorded_count:0,
      final_operator_acknowledgement_persisted_count:0,
      final_operator_acknowledgement_delivered_count:0,
      final_operator_acknowledgement_final_state_promoted_count:0,
      provider_invocation_authorized:false,
      model_invocation_authorized:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      live_kg_write_performed:false,
      memory_store_write_performed:false,
      channel_send_performed:false,
      telegram_send_performed:false,
      external_send_performed:false,
      next_slice:"first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial"
    }'
)"

printf '%s\n' "$report"
echo "Hepta first model invocation operator approval final authorization dry-run result receipt final operator acknowledgement non-acceptance denial route gate passed"
