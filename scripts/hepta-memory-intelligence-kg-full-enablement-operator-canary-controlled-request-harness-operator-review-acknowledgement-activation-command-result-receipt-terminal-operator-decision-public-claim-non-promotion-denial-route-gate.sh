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
    echo "missing operator canary activation command result receipt terminal decision public-claim route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

TERMINAL_DECISION_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
  and .required_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count == 12
  and .ready_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count == 12
  and .required_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 10
  and .activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 10
  and .blocked_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 10
  and .noop_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 10
  and .allowed_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 0
  and .accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 0
  and .activation_command_result_receipt_terminal_operator_decision_performed_count == 0
  and .activation_command_result_receipt_public_claim_promotion_performed_count == 0
  and .activation_command_result_receipt_terminal_operator_decision_recorded == false
  and .activation_command_result_receipt_terminal_operator_decision_persisted == false
  and .activation_command_result_receipt_terminal_operator_decision_materialized == false
  and .activation_command_result_receipt_terminal_operator_decision_filesystem_written == false
  and .activation_command_result_receipt_terminal_operator_decision_delivered == false
  and .activation_command_result_receipt_terminal_operator_decision_identity_accepted == false
  and .activation_command_result_receipt_terminal_operator_decision_signature_accepted == false
  and .activation_command_result_receipt_terminal_operator_decision_final_state_promoted == false
  and .activation_command_result_receipt_terminal_operator_decision_completion_promoted == false
  and .activation_command_result_receipt_public_claim_recorded == false
  and .activation_command_result_receipt_public_claim_persisted == false
  and .activation_command_result_receipt_public_claim_materialized == false
  and .activation_command_result_receipt_public_claim_promoted == false
  and .activation_command_result_receipt_public_ga_claimed == false
  and .activation_command_result_receipt_public_release_published == false
  and .activation_command_result_receipt_public_distribution_performed == false
  and .activation_command_result_receipt_public_artifact_written == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_completion_ack_recorded == false
  and .activation_allowed_by_result_receipt_terminal_operator_decision == false
  and .activation_allowed_by_result_receipt == false
  and .activation_allowed == false
  and .activation_performed == false
  and .live_mutation_execution_performed == false
  and .memory_write_execution_performed == false
  and .memory_store_write_performed == false
  and .memory_store_write_performed_count == 0
  and .memory_store_mutated == false
  and .rollback_executed == false
  and .secret_material_read == false
  and .provider_invoked == false
  and .model_invoked == false
  and .public_release_published == false
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.activation_command_result_receipt_terminal_operator_decision_public_claim_fixtures | length) == 10
  and (.activation_command_result_receipt_terminal_operator_decision_public_claim_fixtures | all(
    (.terminal_operator_decision_status | startswith("blocked"))
    and .terminal_decision_recorded == false
    and .terminal_decision_persisted == false
    and .terminal_decision_materialized == false
    and .terminal_decision_filesystem_written == false
    and .terminal_decision_delivered == false
    and .terminal_decision_accepted == false
    and .terminal_decision_identity_accepted == false
    and .terminal_decision_signature_accepted == false
    and .terminal_decision_final_state_promoted == false
    and .public_claim_promoted == false
    and .public_release_published == false
    and .public_ga_claimed == false
    and .telegram_send_performed == false
    and .receipt_recorded == false
    and .receipt_persisted == false
    and .receipt_accepted == false
    and .activation_allowed == false
    and .live_mutation_execution_performed == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .rollback_executed == false
    and .terminal_operator_decision_noop_confirmed == true
  ))
  and ([.activation_command_result_receipt_terminal_operator_decision_public_claim_fixtures[] | select(.source_final_acknowledgement_present == false)] | length) == 1
  and ([.activation_command_result_receipt_terminal_operator_decision_public_claim_fixtures[] | select(.public_ga_claim_requested == true and .public_release_publish_requested == true)] | length) == 1
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$TERMINAL_DECISION_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 176;' \
  "native gateway route/source command count includes activation command result receipt terminal decision public-claim route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT' \
  "native gateway activation command result receipt terminal decision public-claim endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial' \
  "native gateway activation command result receipt terminal decision public-claim endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial --json' \
  "native gateway activation command result receipt terminal decision public-claim source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report' \
  "native gateway activation command result receipt terminal decision public-claim report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_route_enabled": true' \
  "activation command result receipt terminal decision public-claim route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_terminal_operator_decision_accepted"' \
  "activation command result receipt terminal decision acceptance denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_public_claim_promoted"' \
  "activation command result receipt public-claim promotion denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_command_result_receipt_public_release_published"' \
  "activation command result receipt public release publication denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-canary-activation-command-result-receipt-terminal-decision-public-claim-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_endpoint_blocks_promotion \
  -- --nocapture >"$TEST_LOG"

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 160
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_ready == true
    and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_route_enabled == true
    and .activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 10
    and .blocked_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 10
    and .allowed_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 0
    and .activation_command_result_receipt_terminal_operator_decision_performed_count == 0
    and .activation_command_result_receipt_public_claim_promotion_performed_count == 0
    and .activation_command_result_receipt_terminal_operator_decision_accepted == false
    and .activation_command_result_receipt_terminal_operator_decision_recorded == false
    and .activation_command_result_receipt_terminal_operator_decision_persisted == false
    and .activation_command_result_receipt_terminal_operator_decision_materialized == false
    and .activation_command_result_receipt_terminal_operator_decision_filesystem_written == false
    and .activation_command_result_receipt_terminal_operator_decision_delivered == false
    and .activation_command_result_receipt_public_claim_promoted == false
    and .activation_command_result_receipt_public_ga_claimed == false
    and .activation_command_result_receipt_public_release_published == false
    and .activation_command_result_receipt_public_artifact_written == false
    and .telegram_send_performed == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .activation_command_invoked == false
    and .activation_command_dispatched == false
    and .activation_performed == false
    and .live_mutation_execution_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .install_executed == false
    and .service_restarted == false
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
  and .required_marker_count == 300
  and .present_required_marker_count == 300
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_terminal_decision_gate_sha256="$(printf '%s' "$TERMINAL_DECISION_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "ready" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial --json" \
  --arg source_terminal_decision_gate_sha256 "$source_terminal_decision_gate_sha256" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$TERMINAL_DECISION_JSON" \
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
    activation_mode: "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_native_route_status",
    source_activation_command_result_receipt_terminal_operator_decision_public_claim_gate: $source.gate,
    source_activation_command_result_receipt_terminal_operator_decision_public_claim_gate_ready: $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready,
    source_terminal_decision_gate_sha256: $source_terminal_decision_gate_sha256,
    source_route_wired: true,
    source_route_count_expected:153,
    source_route_tested_by_native_gateway_unit_test: true,
    native_gateway_source: "codex-rs/cli/src/native_gateway.rs",
    native_gateway_sha256: $native_gateway_sha256,
    native_gateway_unit_test_log: $test_log,
    live_endpoint_required: ($require_live == "1"),
    live_endpoint_ready: (if $require_live == "1" then ($live.status == "ready") else null end),
    terminal_operator_decision_public_claim_fixture_count: $source.activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count,
    blocked_terminal_operator_decision_public_claim_fixture_count: $source.blocked_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count,
    noop_terminal_operator_decision_public_claim_fixture_count: $source.noop_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count,
    accepted_terminal_operator_decision_public_claim_fixture_count: $source.accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count,
    terminal_operator_decision_performed_count: $source.activation_command_result_receipt_terminal_operator_decision_performed_count,
    public_claim_promotion_performed_count: $source.activation_command_result_receipt_public_claim_promotion_performed_count,
    activation_command_result_receipt_terminal_operator_decision_recorded: $source.activation_command_result_receipt_terminal_operator_decision_recorded,
    activation_command_result_receipt_terminal_operator_decision_persisted: $source.activation_command_result_receipt_terminal_operator_decision_persisted,
    activation_command_result_receipt_terminal_operator_decision_delivered: $source.activation_command_result_receipt_terminal_operator_decision_delivered,
    activation_command_result_receipt_public_claim_promoted: $source.activation_command_result_receipt_public_claim_promoted,
    activation_command_result_receipt_public_release_published: $source.activation_command_result_receipt_public_release_published,
    activation_command_result_receipt_public_artifact_written: $source.activation_command_result_receipt_public_artifact_written,
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
    service_restarted: $source.service_restarted,
    active_binary_mutated: $source.active_binary_mutated,
    terminal_coverage_sha256: $terminal_coverage_sha256,
    terminal_required_marker_count: $terminal.required_marker_count,
    terminal_present_required_marker_count: $terminal.present_required_marker_count,
    terminal_missing_required_marker_count: $terminal.missing_required_marker_count,
    terminal_duplicate_required_marker_count: $terminal.duplicate_required_marker_count,
    terminal_out_of_order_required_marker_count: $terminal.out_of_order_required_marker_count,
    side_effects: $source.side_effects
  }'

echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation command result receipt terminal operator decision public-claim non-promotion denial route gate passed"
