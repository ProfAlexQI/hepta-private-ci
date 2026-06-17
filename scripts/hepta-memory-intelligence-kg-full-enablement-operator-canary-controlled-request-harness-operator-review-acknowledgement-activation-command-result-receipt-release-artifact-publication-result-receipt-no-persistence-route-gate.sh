#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
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
    echo "missing operator canary release artifact publication result receipt route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

PUBLICATION_RESULT_RECEIPT_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_ready == true
  and .required_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count == 12
  and .ready_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count == 12
  and .activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count == 10
  and .blocked_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count == 10
  and .allowed_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count == 0
  and .accepted_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count == 0
  and .publication_result_receipt_recorded == false
  and .publication_result_receipt_persisted == false
  and .publication_result_receipt_materialized == false
  and .publication_result_receipt_filesystem_written == false
  and .publication_result_receipt_ledger_written == false
  and .publication_result_receipt_indexed == false
  and .publication_result_receipt_enqueued == false
  and .publication_result_receipt_delivered == false
  and .publication_result_receipt_exported == false
  and .publication_result_receipt_query_registered == false
  and .publication_result_receipt_observability_recorded == false
  and .publication_result_receipt_hash_bound == false
  and .publication_result_receipt_signature_accepted == false
  and .publication_result_receipt_timestamp_accepted == false
  and .publication_result_receipt_status_accepted == false
  and .publication_completion_ack_recorded == false
  and .publication_completion_ack_persisted == false
  and .publication_completion_ack_accepted == false
  and .release_artifact_written == false
  and .public_artifact_written == false
  and .public_distribution_performed == false
  and .public_release_published == false
  and .public_ga_claimed == false
  and .terminal_operator_decision_promoted_to_release_approval == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .activation_allowed_by_publication_result_receipt == false
  and .activation_allowed == false
  and .activation_performed == false
  and .live_mutation_execution_performed == false
  and .memory_write_execution_performed == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .rollback_executed == false
  and .secret_material_read == false
  and .provider_invoked == false
  and .model_invoked == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.activation_command_result_receipt_release_artifact_publication_result_receipt_fixtures | length) == 10
  and (.activation_command_result_receipt_release_artifact_publication_result_receipt_fixtures | all(
    (.publication_result_receipt_status | startswith("blocked"))
    and .publication_result_receipt_accepted == false
    and .publication_result_receipt_recorded == false
    and .publication_result_receipt_persisted == false
    and .publication_result_receipt_filesystem_written == false
    and .publication_result_receipt_delivered == false
    and .publication_result_receipt_exported == false
    and .publication_result_receipt_query_registered == false
    and .publication_completion_ack_recorded == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .public_distribution_performed == false
    and .public_release_published == false
    and .external_send_performed == false
    and .activation_allowed == false
    and .live_mutation_execution_performed == false
    and .memory_store_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .install_executed == false
    and .active_binary_mutated == false
    and .publication_result_receipt_noop_confirmed == true
  ))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$PUBLICATION_RESULT_RECEIPT_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 141;' \
  "native gateway route/source command count includes release artifact publication result receipt route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT' \
  "native gateway release artifact publication result receipt endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence' \
  "native gateway release artifact publication result receipt endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence --json' \
  "native gateway release artifact publication result receipt source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_report' \
  "native gateway release artifact publication result receipt report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_route_enabled": true' \
  "release artifact publication result receipt route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"publication_result_receipt_recorded"' \
  "publication result receipt recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"publication_result_receipt_persisted"' \
  "publication result receipt persistence denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"publication_completion_ack_recorded"' \
  "publication completion acknowledgement denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-canary-release-publication-result-receipt-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_endpoint_blocks_persistence \
  -- --nocapture >"$TEST_LOG"

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 141
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_route_ready == true
    and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_route_enabled == true
    and .activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count == 10
    and .blocked_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count == 10
    and .allowed_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count == 0
    and .publication_result_receipt_accepted == false
    and .publication_result_receipt_recorded == false
    and .publication_result_receipt_persisted == false
    and .publication_result_receipt_materialized == false
    and .publication_result_receipt_filesystem_written == false
    and .publication_result_receipt_ledger_written == false
    and .publication_result_receipt_indexed == false
    and .publication_result_receipt_enqueued == false
    and .publication_result_receipt_delivered == false
    and .publication_result_receipt_exported == false
    and .publication_result_receipt_query_registered == false
    and .publication_result_receipt_observability_recorded == false
    and .publication_result_receipt_hash_bound == false
    and .publication_result_receipt_signature_accepted == false
    and .publication_completion_ack_recorded == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .public_distribution_performed == false
    and .public_release_published == false
    and .public_ga_claimed == false
    and .telegram_send_performed == false
    and .channel_send_performed == false
    and .external_send_performed == false
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
  and .required_marker_count == 281
  and .present_required_marker_count == 281
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_publication_result_receipt_gate_sha256="$(printf '%s' "$PUBLICATION_RESULT_RECEIPT_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "ready" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence" \
  --arg source_command "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence --json" \
  --arg source_publication_result_receipt_gate_sha256 "$source_publication_result_receipt_gate_sha256" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$PUBLICATION_RESULT_RECEIPT_JSON" \
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
    activation_mode: "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_native_route_status",
    source_activation_command_result_receipt_release_artifact_publication_result_receipt_gate: $source.gate,
    source_activation_command_result_receipt_release_artifact_publication_result_receipt_gate_ready: $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_ready,
    source_publication_result_receipt_gate_sha256: $source_publication_result_receipt_gate_sha256,
    source_route_wired: true,
    source_route_count_expected:141,
    source_route_tested_by_native_gateway_unit_test: true,
    native_gateway_source: "codex-rs/cli/src/native_gateway.rs",
    native_gateway_sha256: $native_gateway_sha256,
    native_gateway_unit_test_log: $test_log,
    live_endpoint_required: ($require_live == "1"),
    live_endpoint_ready: (if $require_live == "1" then ($live.status == "ready") else null end),
    publication_result_receipt_fixture_count: $source.activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count,
    blocked_publication_result_receipt_fixture_count: $source.blocked_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count,
    accepted_publication_result_receipt_fixture_count: $source.accepted_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count,
    publication_result_receipt_recorded: $source.publication_result_receipt_recorded,
    publication_result_receipt_persisted: $source.publication_result_receipt_persisted,
    publication_result_receipt_materialized: $source.publication_result_receipt_materialized,
    publication_result_receipt_filesystem_written: $source.publication_result_receipt_filesystem_written,
    publication_result_receipt_delivered: $source.publication_result_receipt_delivered,
    publication_result_receipt_exported: $source.publication_result_receipt_exported,
    publication_result_receipt_query_registered: $source.publication_result_receipt_query_registered,
    publication_result_receipt_observability_recorded: $source.publication_result_receipt_observability_recorded,
    publication_completion_ack_recorded: $source.publication_completion_ack_recorded,
    release_artifact_written: $source.release_artifact_written,
    public_artifact_written: $source.public_artifact_written,
    public_distribution_performed: $source.public_distribution_performed,
    public_release_published: $source.public_release_published,
    public_ga_claimed: $source.public_ga_claimed,
    telegram_send_performed: $source.telegram_send_performed,
    channel_send_performed: $source.channel_send_performed,
    external_send_performed: $source.external_send_performed,
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

echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation command result receipt release artifact publication result receipt no-persistence route gate passed"
