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
    echo "missing first model invocation final authorization dry-run result receipt terminal public-claim/status exposure source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"
ENDPOINT="/api/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-public-claim-status-exposure-denial"
SOURCE_COMMAND="/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-public-claim-status-exposure-denial --json"
FOCUSED_TEST="hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_endpoint_blocks_public_status_exposure_and_authority"
CANARY_MODE="first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_no_status_exposure_no_public_claim_no_release_no_artifact_no_provider_model_invocation"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes terminal public-claim/status exposure route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT' \
  "terminal public-claim/status exposure endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "$ENDPOINT" \
  "terminal public-claim/status exposure endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "$SOURCE_COMMAND" \
  "terminal public-claim/status exposure source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_report' \
  "terminal public-claim/status exposure report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "$CANARY_MODE" \
  "terminal public-claim/status exposure execution mode"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "$FOCUSED_TEST" \
  "focused terminal public-claim/status exposure unit test"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial"' \
  "next action remains terminal public-claim delivery readback denial"

TEST_LOG="$(mktemp /tmp/hepta-first-model-invocation-final-authorization-dry-run-result-receipt-terminal-public-claim-status-exposure-route-tests.XXXXXX)"
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
    and .source_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready == true
    and .first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_ready == true
    and .canary_execution_mode == $canary
    and .result_receipt_terminal_public_claim_status_exposure_state == "final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denied"
    and .terminal_public_claim_status_exposure_surface_count == 18
    and .terminal_public_claim_status_exposure_attempt_count == 18
    and .terminal_public_claim_status_exposure_allowed_count == 0
    and .terminal_public_claim_status_exposure_request_accepted_count == 0
    and .terminal_public_claim_status_exposure_accepted_count == 0
    and .terminal_public_claim_status_exposure_recorded_count == 0
    and .terminal_public_claim_status_exposure_persisted_count == 0
    and .terminal_public_claim_status_exposure_materialized_count == 0
    and .terminal_public_claim_status_exposure_filesystem_written_count == 0
    and .terminal_public_claim_status_exposure_delivered_count == 0
    and .terminal_public_claim_status_exposed_count == 0
    and .public_status_claimed_count == 0
    and .public_release_claimed_count == 0
    and .public_ga_claimed_count == 0
    and .dashboard_status_exposed_count == 0
    and .public_badge_exposed_count == 0
    and .status_endpoint_exposed_count == 0
    and .query_status_exposed_count == 0
    and .export_status_exposed_count == 0
    and .observability_status_exposed_count == 0
    and .release_notes_status_exposed_count == 0
    and .changelog_status_exposed_count == 0
    and .version_tag_status_exposed_count == 0
    and .artifact_availability_status_exposed_count == 0
    and .distribution_queue_status_exposed_count == 0
    and .channel_status_delivered_count == 0
    and .external_status_sent_count == 0
    and .telegram_status_sent_count == 0
    and .release_publication_authority_derived_count == 0
    and .activation_authority_derived_count == 0
    and .live_execution_allowed_count == 0
    and (.terminal_public_claim_status_exposure_surfaces | length) == 18
    and (.terminal_public_claim_status_exposure_surfaces | all(
      .public_claim_status_exposure_attempted == true
      and .public_claim_status_exposure_allowed == false
      and .public_claim_status_exposure_request_accepted == false
      and .public_claim_status_exposure_recorded == false
      and .public_claim_status_exposure_persisted == false
      and .public_claim_status_exposure_materialized == false
      and .public_claim_status_exposure_filesystem_written == false
      and .public_claim_status_exposure_delivered == false
      and .public_claim_status_exposed == false
      and .public_status_claimed == false
      and .public_release_claimed == false
      and .public_ga_claimed == false
      and .status_endpoint_exposed == false
      and .query_status_exposed == false
      and .export_status_exposed == false
      and .observability_status_exposed == false
      and .channel_status_delivered == false
      and .external_status_sent == false
      and .telegram_status_sent == false
      and .release_publication_authority_derived == false
      and .activation_authority_derived == false
      and .live_execution_allowed == false
      and .public_claim_status_exposure_noop_confirmed == true
    ))
    and .terminal_public_claim_status_exposure_readback_hash_matched == true
    and .denied_by_first_model_invocation_terminal_public_claim_status_exposure_count == 34
    and .terminal_public_claim_status_exposure_accepted == false
    and .terminal_public_claim_status_exposed == false
    and .status_endpoint_exposed == false
    and .query_status_exposed == false
    and .export_status_exposed == false
    and .observability_status_exposed == false
    and .operator_approval_recorded == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .activation_performed == false
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
    and .allowed_next_actions[0].action == "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial"
    and .allowed_next_actions[0].accepts_public_status == false
    and .allowed_next_actions[0].claims_public_release == false
    and .allowed_next_actions[0].delivers_channel == false
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
    --arg gate "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_route_gate" \
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
      first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_ready:true,
      canary_execution_mode:"first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_no_status_exposure_no_public_claim_no_release_no_artifact_no_provider_model_invocation",
      result_receipt_terminal_public_claim_status_exposure_state:"final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denied",
      terminal_public_claim_status_exposure_surface_count:18,
      terminal_public_claim_status_exposure_attempt_count:18,
      terminal_public_claim_status_exposure_allowed_count:0,
      terminal_public_claim_status_exposure_accepted_count:0,
      terminal_public_claim_status_exposure_recorded_count:0,
      terminal_public_claim_status_exposure_persisted_count:0,
      terminal_public_claim_status_exposure_materialized_count:0,
      terminal_public_claim_status_exposure_delivered_count:0,
      terminal_public_claim_status_exposed_count:0,
      public_status_claimed_count:0,
      public_release_claimed_count:0,
      public_ga_claimed_count:0,
      status_endpoint_exposed_count:0,
      query_status_exposed_count:0,
      export_status_exposed_count:0,
      observability_status_exposed_count:0,
      channel_status_delivered_count:0,
      external_status_sent_count:0,
      telegram_status_sent_count:0,
      release_publication_authority_derived_count:0,
      activation_authority_derived_count:0,
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
      next_slice:"first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial"
    }'
)"

printf '%s\n' "$report"
echo "Hepta first model invocation operator approval final authorization dry-run result receipt terminal public-claim/status exposure denial route gate passed"
