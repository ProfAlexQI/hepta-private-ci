#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT=191

cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing first model invocation final authorization dry-run result receipt terminal operator decision/public-claim source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 191;' \
  "native gateway route/source command count includes terminal operator decision public-claim route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT' \
  "terminal operator decision public-claim endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial' \
  "terminal operator decision public-claim endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial --json' \
  "terminal operator decision public-claim source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report' \
  "terminal operator decision public-claim report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_no_decision_no_public_claim_no_release_no_artifact_no_provider_model_invocation' \
  "terminal operator decision public-claim execution mode"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_endpoint_blocks_terminal_decision_public_claim_and_authority' \
  "focused terminal operator decision public-claim unit test"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial"' \
  "next action remains terminal public claim/status exposure denial"

TEST_LOG="$(mktemp /tmp/hepta-first-model-invocation-final-authorization-dry-run-result-receipt-terminal-operator-decision-public-claim-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_endpoint_blocks_terminal_decision_public_claim_and_authority \
  -- --nocapture >"$TEST_LOG"

live_route_status="skipped"
live_route_count=0
live_missing_route_count=0

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
    and .first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready == true
    and .canary_execution_mode == "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_no_decision_no_public_claim_no_release_no_artifact_no_provider_model_invocation"
    and .result_receipt_terminal_operator_decision_public_claim_state == "final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denied"
    and .terminal_operator_decision_public_claim_fixture_count == 10
    and .blocked_terminal_operator_decision_public_claim_fixture_count == 10
    and .noop_terminal_operator_decision_public_claim_fixture_count == 10
    and .allowed_terminal_operator_decision_public_claim_fixture_count == 0
    and .accepted_terminal_operator_decision_public_claim_fixture_count == 0
    and .terminal_operator_decision_performed_count == 0
    and .public_claim_promotion_performed_count == 0
    and .terminal_operator_decision_accepted_count == 0
    and .terminal_operator_decision_recorded_count == 0
    and .terminal_operator_decision_persisted_count == 0
    and .terminal_operator_decision_delivered_count == 0
    and .public_claim_recorded_count == 0
    and .public_claim_promoted_count == 0
    and .public_release_published_count == 0
    and .release_artifact_written_count == 0
    and (.terminal_operator_decision_public_claim_fixtures | length) == 10
    and (.terminal_operator_decision_public_claim_fixtures | all(
      (.terminal_operator_decision_public_claim_status | startswith("blocked_"))
      and .terminal_operator_decision_public_claim_noop_confirmed == true
    ))
    and ([.terminal_operator_decision_public_claim_fixtures[] | select(.source_final_operator_acknowledgement_present == false)] | length) == 1
    and ([.terminal_operator_decision_public_claim_fixtures[] | select(.public_claim_promotion_requested == true)] | length) == 1
    and ([.terminal_operator_decision_public_claim_fixtures[] | select(.service_restart_decision_requested == true)] | length) == 1
    and .final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_readback_hash_matched == true
    and .terminal_operator_decision_allowed == false
    and .terminal_operator_decision_request_accepted == false
    and .terminal_operator_decision_accepted == false
    and .terminal_operator_decision_recorded == false
    and .terminal_operator_decision_persisted == false
    and .terminal_operator_decision_materialized == false
    and .terminal_operator_decision_filesystem_written == false
    and .terminal_operator_decision_delivered == false
    and .terminal_operator_decision_channel_delivery_performed == false
    and .terminal_operator_decision_identity_accepted == false
    and .terminal_operator_decision_signature_accepted == false
    and .terminal_operator_decision_timestamp_accepted == false
    and .terminal_operator_decision_final_state_promoted == false
    and .terminal_operator_decision_completion_promoted == false
    and .public_claim_requested == false
    and .public_claim_accepted == false
    and .public_claim_recorded == false
    and .public_claim_persisted == false
    and .public_claim_materialized == false
    and .public_claim_promoted == false
    and .public_ga_claimed == false
    and .public_release_claimed == false
    and .public_release_published == false
    and .public_distribution_performed == false
    and .public_artifact_written == false
    and .release_artifact_written == false
    and .activation_allowed_by_terminal_operator_decision == false
    and .activation_allowed_by_result_receipt == false
    and .activation_allowed == false
    and .activation_performed == false
    and .provider_invocation_authorized == false
    and .model_invocation_authorized == false
    and .provider_invocation_authorized_from_terminal_decision == false
    and .model_invocation_authorized_from_terminal_decision == false
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
    and .install_executed == false
    and .launchd_mutated == false
    and .service_restart_performed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and (.audit_steps | length) == 6
    and .allowed_next_actions[0].action == "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial"
    and .allowed_next_actions[0].accepts_terminal_decision == false
    and .allowed_next_actions[0].claims_public_release == false
    and .allowed_next_actions[0].exposes_public_status == false
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
    --arg gate "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_route_gate" \
    --arg endpoint "/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial" \
    --arg source_command "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial --json" \
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
      first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready:true,
      canary_execution_mode:"first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_no_decision_no_public_claim_no_release_no_artifact_no_provider_model_invocation",
      result_receipt_terminal_operator_decision_public_claim_state:"final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denied",
      terminal_operator_decision_public_claim_fixture_count:10,
      blocked_terminal_operator_decision_public_claim_fixture_count:10,
      allowed_terminal_operator_decision_public_claim_fixture_count:0,
      accepted_terminal_operator_decision_public_claim_fixture_count:0,
      terminal_operator_decision_performed_count:0,
      public_claim_promotion_performed_count:0,
      terminal_operator_decision_recorded_count:0,
      terminal_operator_decision_persisted_count:0,
      public_claim_recorded_count:0,
      public_claim_promoted_count:0,
      public_release_published_count:0,
      release_artifact_written_count:0,
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
      install_executed:false,
      service_restarted:false,
      active_binary_mutated:false,
      next_slice:"first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial"
    }'
)"

printf '%s\n' "$report"
echo "Hepta first model invocation operator approval final authorization dry-run result receipt terminal operator decision public-claim non-promotion denial route gate passed"
