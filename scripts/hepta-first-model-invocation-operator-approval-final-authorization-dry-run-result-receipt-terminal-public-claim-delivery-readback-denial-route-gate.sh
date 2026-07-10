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
    echo "missing first model invocation final authorization dry-run result receipt terminal public-claim delivery/readback source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ENDPOINT="/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-public-claim-delivery-readback-denial"
SOURCE_COMMAND="/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-public-claim-delivery-readback-denial --json"
FOCUSED_TEST="hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_endpoint_blocks_delivery_readback_receipts_and_authority"
CANARY_MODE="first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_no_delivery_no_readback_no_receipt_no_release_no_channel_no_telegram_no_install"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes terminal public-claim delivery/readback route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_PUBLIC_CLAIM_DELIVERY_READBACK_DENIAL_ENDPOINT' \
  "terminal public-claim delivery/readback endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "$ENDPOINT" \
  "terminal public-claim delivery/readback endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "$SOURCE_COMMAND" \
  "terminal public-claim delivery/readback source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_report' \
  "terminal public-claim delivery/readback report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "$CANARY_MODE" \
  "terminal public-claim delivery/readback execution mode"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "$FOCUSED_TEST" \
  "focused terminal public-claim delivery/readback unit test"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_release_artifact_publication_denial"' \
  "next action remains terminal public-claim delivery/readback release artifact publication denial"

TEST_LOG="$(mktemp /tmp/hepta-first-model-invocation-final-authorization-dry-run-result-receipt-terminal-public-claim-delivery-readback-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  "$FOCUSED_TEST" \
  -- --nocapture >"$TEST_LOG"

live_route_status="skipped"
live_route_count=0
live_missing_route_count=0

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(curl -fsS "$BASE_URL$ENDPOINT")"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" --arg canary "$CANARY_MODE" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_ready == true
    and .first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_ready == true
    and .canary_execution_mode == $canary
    and .result_receipt_terminal_public_claim_delivery_readback_state == "final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denied"
    and .source_terminal_public_claim_status_exposure_surface_count == 18
    and .source_terminal_public_claim_status_exposed_count == 0
    and .source_public_status_claimed_count == 0
    and .source_channel_status_delivered_count == 0
    and .source_external_status_sent_count == 0
    and .source_telegram_status_sent_count == 0
    and .source_release_publication_authority_derived_count == 0
    and .source_activation_authority_derived_count == 0
    and .terminal_public_claim_delivery_readback_surface_count == 18
    and .terminal_public_claim_delivery_readback_attempt_count == 18
    and .terminal_public_claim_delivery_readback_denied_count == 18
    and .terminal_public_claim_delivery_readback_allowed_count == 0
    and .terminal_public_claim_delivery_readback_accepted_count == 0
    and .terminal_public_claim_delivery_readback_recorded_count == 0
    and .terminal_public_claim_delivery_readback_persisted_count == 0
    and .terminal_public_claim_delivery_readback_delivered_count == 0
    and .terminal_public_claim_delivery_readback_status_read_count == 0
    and .public_claim_delivery_recorded_count == 0
    and .public_claim_delivery_persisted_count == 0
    and .status_readback_recorded_count == 0
    and .status_readback_persisted_count == 0
    and .channel_delivery_recorded_count == 0
    and .channel_delivery_persisted_count == 0
    and .channel_status_readback_delivered_count == 0
    and .external_delivery_readback_sent_count == 0
    and .telegram_delivery_readback_sent_count == 0
    and .delivery_receipt_recorded_count == 0
    and .delivery_receipt_persisted_count == 0
    and .readback_receipt_recorded_count == 0
    and .readback_receipt_persisted_count == 0
    and .release_artifact_written_count == 0
    and .public_artifact_written_count == 0
    and .operator_approval_from_delivery_readback_derived_count == 0
    and .release_publication_authority_from_delivery_readback_derived_count == 0
    and .activation_authority_from_delivery_readback_derived_count == 0
    and .download_link_from_delivery_readback_rendered_count == 0
    and .install_command_from_delivery_readback_emitted_count == 0
    and .install_from_delivery_readback_executed_count == 0
    and .service_restart_from_delivery_readback_performed_count == 0
    and .active_binary_from_delivery_readback_mutated_count == 0
    and .memory_store_write_performed_count == 0
    and .live_kg_write_performed_count == 0
    and .provider_invoked_count == 0
    and .model_invoked_count == 0
    and .credential_read_count == 0
    and .secret_file_read_count == 0
    and .external_send_performed_count == 0
    and (.terminal_public_claim_delivery_readback_surfaces | length) == 18
    and (.terminal_public_claim_delivery_readback_surfaces | all(
      .terminal_public_claim_delivery_readback_attempted == true
      and .terminal_public_claim_delivery_readback_noop_confirmed == true
      and .public_claim_delivery_allowed == false
      and .status_readback_allowed == false
      and .channel_delivery_allowed == false
      and .telegram_delivery_allowed == false
      and .external_delivery_allowed == false
      and .delivery_receipt_allowed == false
      and .readback_receipt_allowed == false
      and .release_artifact_write_allowed == false
      and .public_artifact_write_allowed == false
      and .activation_authority_derivation_allowed == false
      and .install_restart_active_binary_mutation_allowed == false
      and .provider_invocation_allowed == false
      and .model_invocation_allowed == false
      and .credential_read_allowed == false
    ))
    and ([.terminal_public_claim_delivery_readback_surfaces[] | select(.telegram_delivery_requested == true)] | length) == 1
    and ([.terminal_public_claim_delivery_readback_surfaces[] | select(.public_claim_delivery_requested == true)] | length) == 4
    and .terminal_public_claim_delivery_readback_readback_hash_matched == true
    and .denied_by_first_model_invocation_terminal_public_claim_delivery_readback_count == 26
    and .terminal_public_claim_delivery_readback_accepted == false
    and .terminal_public_claim_delivery_readback_recorded == false
    and .terminal_public_claim_delivery_readback_persisted == false
    and .terminal_public_claim_delivery_readback_delivered == false
    and .delivery_receipt_recorded == false
    and .delivery_receipt_persisted == false
    and .readback_receipt_recorded == false
    and .readback_receipt_persisted == false
    and .operator_approval_recorded == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .external_send_performed == false
    and (.audit_steps | length) == 6
    and .allowed_next_actions[0].action == "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_release_artifact_publication_denial"
    and .allowed_next_actions[0].records_public_claim_delivery == false
    and .allowed_next_actions[0].records_status_readback == false
    and .allowed_next_actions[0].sends_telegram == false
    and .allowed_next_actions[0].writes_release_artifact == false
    and .allowed_next_actions[0].installs_or_restarts == false
    and .allowed_next_actions[0].invokes_provider == false
    and .allowed_next_actions[0].reads_credentials == false
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
    --arg gate "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_route_gate" \
    --arg endpoint "$ENDPOINT" \
    --arg source_command "$SOURCE_COMMAND" \
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
      first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_ready:true,
      canary_execution_mode:"first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_no_delivery_no_readback_no_receipt_no_release_no_channel_no_telegram_no_install",
      result_receipt_terminal_public_claim_delivery_readback_state:"final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denied",
      terminal_public_claim_delivery_readback_surface_count:18,
      terminal_public_claim_delivery_readback_attempt_count:18,
      terminal_public_claim_delivery_readback_denied_count:18,
      public_claim_delivery_recorded_count:0,
      status_readback_recorded_count:0,
      channel_status_readback_delivered_count:0,
      external_delivery_readback_sent_count:0,
      telegram_delivery_readback_sent_count:0,
      delivery_receipt_recorded_count:0,
      readback_receipt_recorded_count:0,
      release_artifact_written_count:0,
      public_artifact_written_count:0,
      release_publication_authority_from_delivery_readback_derived_count:0,
      activation_authority_from_delivery_readback_derived_count:0,
      install_from_delivery_readback_executed_count:0,
      active_binary_from_delivery_readback_mutated_count:0,
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
      next_slice:"first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_release_artifact_publication_denial"
    }'
)"

printf '%s\n' "$report"
echo "Hepta first model invocation operator approval final authorization dry-run result receipt terminal public-claim delivery/readback denial route gate passed"
