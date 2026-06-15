#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
RELEASE_BIN="${HEPTA_RELEASE_BIN:-${HEPTA_CODEX_RELEASE_BIN:-$HOME/.local/opt/hepta/bin/hepta}}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_unsigned_integer() {
  local name="$1"
  local value="$2"

  case "$value" in
    ''|*[!0-9]*)
      echo "$name must be an unsigned integer" >&2
      exit 2
      ;;
  esac
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing operator canary activation command result receipt final operator acknowledgement route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

FINAL_ACK_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status == "blocked"
  and .source_activation_command_result_receipt_operator_facing_summary_briefing_ready == true
  and .source_activation_command_result_receipt_operator_facing_summary_briefing_status == "blocked"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
  and .required_activation_command_result_receipt_final_operator_acknowledgement_surface_count == 12
  and .ready_activation_command_result_receipt_final_operator_acknowledgement_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_final_operator_acknowledgement_surface_count == 12
  and .required_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
  and .activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
  and .blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
  and .noop_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
  and .allowed_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 0
  and .accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 0
  and .activation_command_result_receipt_final_operator_acknowledgement_performed_count == 0
  and .activation_command_result_receipt_final_operator_acknowledgement_allowed == false
  and .activation_command_result_receipt_final_operator_acknowledgement_request_accepted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_accepted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_recorded == false
  and .activation_command_result_receipt_final_operator_acknowledgement_persisted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_materialized == false
  and .activation_command_result_receipt_final_operator_acknowledgement_filesystem_written == false
  and .activation_command_result_receipt_final_operator_acknowledgement_delivered == false
  and .activation_command_result_receipt_final_operator_acknowledgement_channel_delivery_performed == false
  and .activation_command_result_receipt_final_operator_acknowledgement_identity_accepted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_signature_accepted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_timestamp_accepted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted == false
  and .activation_command_result_receipt_final_operator_acknowledgement_completion_promoted == false
  and .activation_command_result_receipt_operator_final_acceptance_recorded == false
  and .activation_command_result_receipt_operator_final_acceptance_persisted == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_allowed_by_result_receipt_final_operator_acknowledgement == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_allowed == false
  and .activation_performed == false
  and .live_mutation_execution_performed == false
  and .runtime_router_mutated == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .auth_secret_read == false
  and .credential_read == false
  and .secret_file_read == false
  and .memory_store_write_performed == false
  and .memory_store_write_performed_count == 0
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .rollback_executed == false
  and .public_release_claimed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.activation_command_result_receipt_final_operator_acknowledgement_fixtures | length) == 10
  and (.activation_command_result_receipt_final_operator_acknowledgement_fixtures | all(
    (.final_operator_acknowledgement_status | startswith("blocked"))
    and .acknowledgement_accepted == false
    and .acknowledgement_recorded == false
    and .acknowledgement_persisted == false
    and .acknowledgement_materialized == false
    and .acknowledgement_filesystem_written == false
    and .acknowledgement_delivered == false
    and .acknowledgement_identity_accepted == false
    and .acknowledgement_signature_accepted == false
    and .acknowledgement_final_state_promoted == false
    and .operator_final_acceptance_recorded == false
    and .operator_final_acceptance_persisted == false
    and .telegram_send_performed == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .activation_command_result_receipt_accepted == false
    and .activation_allowed == false
    and .activation_performed == false
    and .live_mutation_execution_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .final_acknowledgement_noop_confirmed == true
  ))
  and ([.activation_command_result_receipt_final_operator_acknowledgement_fixtures[] | select(.source_summary_briefing_present == false)] | length) == 1
  and ([.activation_command_result_receipt_final_operator_acknowledgement_fixtures[] | select(.telegram_send_requested == true and .channel_delivery_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_final_operator_acknowledgement_fixtures[] | select(.activation_from_acknowledgement_requested == true and .memory_store_acknowledgement_requested == true and .live_kg_acknowledgement_requested == true and .provider_prompt_acknowledgement_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_final_operator_acknowledgement_fixtures[] | select(.external_send_acknowledgement_requested == true and .install_acknowledgement_requested == true and .active_binary_acknowledgement_requested == true)] | length) == 1
  and (.denied_by_activation_command_result_receipt_final_operator_acknowledgement | length) == 17
  and (.allowed_next_actions | any(.action == "stage_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial" and .status == "allowed_report_only_next_slice" and .claims_public_release == false and .writes_release_artifact == false and .activates_runtime == false and .invokes_model == false and .writes_memory_or_kg == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$FINAL_ACK_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 125;' \
  "native gateway route/source command count includes activation command result receipt final operator acknowledgement route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT' \
  "native gateway activation command result receipt final operator acknowledgement endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial' \
  "native gateway activation command result receipt final operator acknowledgement endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json' \
  "native gateway activation command result receipt final operator acknowledgement source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report' \
  "native gateway activation command result receipt final operator acknowledgement report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled": true' \
  "activation command result receipt final operator acknowledgement route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_final_operator_acknowledgement_accepted"' \
  "activation command result receipt final operator acknowledgement acceptance denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_final_operator_acknowledgement_recorded"' \
  "activation command result receipt final operator acknowledgement recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted"' \
  "activation command result receipt final operator acknowledgement final-state promotion denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-canary-activation-command-result-receipt-final-operator-acknowledgement-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_endpoint_blocks_acceptance \
  -- --nocapture >"$TEST_LOG"

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 125
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_route_ready == true
    and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled == true
    and .activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
    and .blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
    and .allowed_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 0
    and .activation_command_result_receipt_final_operator_acknowledgement_performed_count == 0
    and .activation_command_result_receipt_final_operator_acknowledgement_accepted == false
    and .activation_command_result_receipt_final_operator_acknowledgement_recorded == false
    and .activation_command_result_receipt_final_operator_acknowledgement_persisted == false
    and .activation_command_result_receipt_final_operator_acknowledgement_materialized == false
    and .activation_command_result_receipt_final_operator_acknowledgement_filesystem_written == false
    and .activation_command_result_receipt_final_operator_acknowledgement_delivered == false
    and .activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted == false
    and .activation_command_result_receipt_operator_final_acceptance_recorded == false
    and .telegram_send_performed == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .activation_command_invoked == false
    and .activation_command_dispatched == false
    and .activation_performed == false
    and .live_mutation_execution_performed == false
    and .runtime_router_mutated == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .install_executed == false
    and .service_restart_performed == false
    and .active_binary_mutated == false
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_ROUTE_JSON"
else
  LIVE_ROUTE_JSON='null'
fi

TERMINAL_COVERAGE_JSON="$(
  capture_json_report \
    "hepta-preflight-terminal-coverage-inventory-gate" \
    scripts/hepta-preflight-terminal-coverage-inventory-gate.sh
)"
jq -e '
  .status == "ready"
  and .preflight_terminal_coverage_inventory_ready == true
  and .required_marker_count == 265
  and .present_required_marker_count == 265
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_final_ack_gate_sha256="$(printf '%s' "$FINAL_ACK_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "ready" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json" \
  --arg source_final_ack_gate_sha256 "$source_final_ack_gate_sha256" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$FINAL_ACK_JSON" \
  --argjson terminal "$TERMINAL_COVERAGE_JSON" \
  --argjson live "$LIVE_ROUTE_JSON" \
  --arg require_live "$REQUIRE_LIVE_ENDPOINT" \
  '{
    product: $product,
    runtime: $runtime,
    status: $status,
    base_url: $base_url,
    gate: $gate,
    endpoint: $endpoint,
    source_command: $source_command,
    activation_mode: "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_native_route_status",
    source_activation_command_result_receipt_final_operator_acknowledgement_gate: $source.gate,
    source_activation_command_result_receipt_final_operator_acknowledgement_gate_ready: $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready,
    source_activation_command_result_receipt_final_operator_acknowledgement_gate_status: $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status,
    source_final_ack_gate_sha256: $source_final_ack_gate_sha256,
    source_route_wired: true,
    source_route_count_expected:125,
    source_route_tested_by_native_gateway_unit_test: true,
    native_gateway_source: "codex-rs/cli/src/native_gateway.rs",
    native_gateway_sha256: $native_gateway_sha256,
    native_gateway_unit_test_log: $test_log,
    live_endpoint_required: ($require_live == "1"),
    live_endpoint_ready: (if $require_live == "1" then ($live.status == "ready") else null end),
    final_operator_acknowledgement_fixture_count: $source.activation_command_result_receipt_final_operator_acknowledgement_fixture_count,
    blocked_final_operator_acknowledgement_fixture_count: $source.blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count,
    noop_final_operator_acknowledgement_fixture_count: $source.noop_activation_command_result_receipt_final_operator_acknowledgement_fixture_count,
    accepted_final_operator_acknowledgement_fixture_count: $source.accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count,
    final_operator_acknowledgement_performed_count: $source.activation_command_result_receipt_final_operator_acknowledgement_performed_count,
    activation_command_result_receipt_final_operator_acknowledgement_accepted: $source.activation_command_result_receipt_final_operator_acknowledgement_accepted,
    activation_command_result_receipt_final_operator_acknowledgement_recorded: $source.activation_command_result_receipt_final_operator_acknowledgement_recorded,
    activation_command_result_receipt_final_operator_acknowledgement_persisted: $source.activation_command_result_receipt_final_operator_acknowledgement_persisted,
    activation_command_result_receipt_final_operator_acknowledgement_materialized: $source.activation_command_result_receipt_final_operator_acknowledgement_materialized,
    activation_command_result_receipt_final_operator_acknowledgement_filesystem_written: $source.activation_command_result_receipt_final_operator_acknowledgement_filesystem_written,
    activation_command_result_receipt_final_operator_acknowledgement_delivered: $source.activation_command_result_receipt_final_operator_acknowledgement_delivered,
    activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted: $source.activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted,
    activation_command_result_receipt_operator_final_acceptance_recorded: $source.activation_command_result_receipt_operator_final_acceptance_recorded,
    telegram_send_performed: $source.telegram_send_performed,
    channel_send_performed: $source.channel_send_performed,
    external_send_performed: $source.external_send_performed,
    activation_command_invoked: $source.activation_command_invoked,
    activation_command_dispatched: $source.activation_command_dispatched,
    activation_performed: $source.activation_performed,
    live_mutation_execution_performed: $source.live_mutation_execution_performed,
    provider_invoked: $source.provider_invoked,
    model_invoked: $source.model_invoked,
    memory_store_write_performed: $source.memory_store_write_performed,
    memory_store_mutated: $source.memory_store_mutated,
    live_kg_write_performed: $source.live_kg_write_performed,
    credential_read: $source.credential_read,
    secret_file_read: $source.secret_file_read,
    install_executed: $source.install_executed,
    service_restart_performed: $source.service_restart_performed,
    active_binary_mutated: $source.active_binary_mutated,
    terminal_coverage_sha256: $terminal_coverage_sha256,
    terminal_required_marker_count: $terminal.required_marker_count,
    terminal_present_required_marker_count: $terminal.present_required_marker_count,
    terminal_missing_required_marker_count: $terminal.missing_required_marker_count,
    terminal_duplicate_required_marker_count: $terminal.duplicate_required_marker_count,
    terminal_out_of_order_required_marker_count: $terminal.out_of_order_required_marker_count,
    side_effects: $source.side_effects
  }'

echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation command result receipt final operator acknowledgement non-acceptance denial route gate passed"
