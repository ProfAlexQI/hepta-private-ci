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
    echo "missing operator canary activation command result receipt release artifact publication route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

RELEASE_ARTIFACT_PUBLICATION_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready == true
  and .required_activation_command_result_receipt_release_artifact_publication_surface_count == 12
  and .ready_activation_command_result_receipt_release_artifact_publication_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_release_artifact_publication_surface_count == 12
  and .required_activation_command_result_receipt_release_artifact_publication_fixture_count == 10
  and .activation_command_result_receipt_release_artifact_publication_fixture_count == 10
  and .blocked_activation_command_result_receipt_release_artifact_publication_fixture_count == 10
  and .noop_activation_command_result_receipt_release_artifact_publication_fixture_count == 10
  and .allowed_activation_command_result_receipt_release_artifact_publication_fixture_count == 0
  and .accepted_activation_command_result_receipt_release_artifact_publication_fixture_count == 0
  and .activation_command_result_receipt_release_artifact_publication_performed_count == 0
  and .release_artifact_publication_allowed == false
  and .release_artifact_publication_accepted == false
  and .release_artifact_publication_recorded == false
  and .release_artifact_publication_persisted == false
  and .release_artifact_publication_materialized == false
  and .release_artifact_filesystem_written == false
  and .release_artifact_written == false
  and .public_artifact_written == false
  and .artifact_signature_accepted == false
  and .artifact_notarization_accepted == false
  and .publication_queue_enqueued == false
  and .publication_manifest_written == false
  and .public_distribution_performed == false
  and .public_release_published == false
  and .public_ga_claimed == false
  and .public_claim_promoted == false
  and .public_version_tag_created == false
  and .release_notes_materialized == false
  and .changelog_materialized == false
  and .terminal_operator_decision_promoted_to_release_approval == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_allowed_by_release_artifact_publication == false
  and .activation_allowed_by_terminal_operator_decision == false
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
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.activation_command_result_receipt_release_artifact_publication_fixtures | length) == 10
  and (.activation_command_result_receipt_release_artifact_publication_fixtures | all(
    (.release_artifact_publication_status | startswith("blocked"))
    and .release_artifact_publication_accepted == false
    and .release_artifact_publication_recorded == false
    and .release_artifact_publication_persisted == false
    and .release_artifact_publication_materialized == false
    and .release_artifact_filesystem_written == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .artifact_signature_accepted == false
    and .artifact_notarization_accepted == false
    and .publication_queue_enqueued == false
    and .publication_manifest_written == false
    and .public_distribution_performed == false
    and .public_release_published == false
    and .public_ga_claimed == false
    and .public_claim_promoted == false
    and .terminal_operator_decision_promoted_to_release_approval == false
    and .telegram_send_performed == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .activation_allowed == false
    and .live_mutation_execution_performed == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .provider_invoked == false
    and .model_invoked == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .release_artifact_publication_noop_confirmed == true
  ))
  and ([.activation_command_result_receipt_release_artifact_publication_fixtures[] | select(.source_terminal_operator_decision_present == false)] | length) == 1
  and ([.activation_command_result_receipt_release_artifact_publication_fixtures[] | select(.public_release_publish_requested == true and .public_ga_claim_requested == true)] | length) == 1
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$RELEASE_ARTIFACT_PUBLICATION_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 118;' \
  "native gateway route/source command count includes activation command result receipt release artifact publication route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT' \
  "native gateway activation command result receipt release artifact publication endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial' \
  "native gateway activation command result receipt release artifact publication endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial --json' \
  "native gateway activation command result receipt release artifact publication source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_report' \
  "native gateway activation command result receipt release artifact publication report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_route_enabled": true' \
  "activation command result receipt release artifact publication route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"release_artifact_publication_accepted"' \
  "release artifact publication acceptance denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"release_artifact_written"' \
  "release artifact write denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"public_release_published"' \
  "public release publication denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-canary-activation-command-result-receipt-release-artifact-publication-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_endpoint_blocks_release \
  -- --nocapture >"$TEST_LOG"

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 118
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .source_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_ready == true
    and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_route_enabled == true
    and .activation_command_result_receipt_release_artifact_publication_fixture_count == 10
    and .blocked_activation_command_result_receipt_release_artifact_publication_fixture_count == 10
    and .allowed_activation_command_result_receipt_release_artifact_publication_fixture_count == 0
    and .activation_command_result_receipt_release_artifact_publication_performed_count == 0
    and .release_artifact_publication_accepted == false
    and .release_artifact_publication_recorded == false
    and .release_artifact_publication_persisted == false
    and .release_artifact_publication_materialized == false
    and .release_artifact_filesystem_written == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .artifact_signature_accepted == false
    and .artifact_notarization_accepted == false
    and .publication_queue_enqueued == false
    and .publication_manifest_written == false
    and .public_distribution_performed == false
    and .public_release_published == false
    and .public_ga_claimed == false
    and .public_claim_promoted == false
    and .terminal_operator_decision_promoted_to_release_approval == false
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
  and .required_marker_count == 258
  and .present_required_marker_count == 258
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_release_artifact_publication_gate_sha256="$(printf '%s' "$RELEASE_ARTIFACT_PUBLICATION_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "ready" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial --json" \
  --arg source_release_artifact_publication_gate_sha256 "$source_release_artifact_publication_gate_sha256" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$RELEASE_ARTIFACT_PUBLICATION_JSON" \
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
    activation_mode: "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_native_route_status",
    source_activation_command_result_receipt_release_artifact_publication_gate: $source.gate,
    source_activation_command_result_receipt_release_artifact_publication_gate_ready: $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_ready,
    source_release_artifact_publication_gate_sha256: $source_release_artifact_publication_gate_sha256,
    source_route_wired: true,
    source_route_count_expected:118,
    source_route_tested_by_native_gateway_unit_test: true,
    native_gateway_source: "codex-rs/cli/src/native_gateway.rs",
    native_gateway_sha256: $native_gateway_sha256,
    native_gateway_unit_test_log: $test_log,
    live_endpoint_required: ($require_live == "1"),
    live_endpoint_ready: (if $require_live == "1" then ($live.status == "ready") else null end),
    release_artifact_publication_fixture_count: $source.activation_command_result_receipt_release_artifact_publication_fixture_count,
    blocked_release_artifact_publication_fixture_count: $source.blocked_activation_command_result_receipt_release_artifact_publication_fixture_count,
    noop_release_artifact_publication_fixture_count: $source.noop_activation_command_result_receipt_release_artifact_publication_fixture_count,
    accepted_release_artifact_publication_fixture_count: $source.accepted_activation_command_result_receipt_release_artifact_publication_fixture_count,
    release_artifact_publication_performed_count: $source.activation_command_result_receipt_release_artifact_publication_performed_count,
    release_artifact_publication_recorded: $source.release_artifact_publication_recorded,
    release_artifact_publication_persisted: $source.release_artifact_publication_persisted,
    release_artifact_publication_materialized: $source.release_artifact_publication_materialized,
    release_artifact_filesystem_written: $source.release_artifact_filesystem_written,
    release_artifact_written: $source.release_artifact_written,
    public_artifact_written: $source.public_artifact_written,
    artifact_signature_accepted: $source.artifact_signature_accepted,
    artifact_notarization_accepted: $source.artifact_notarization_accepted,
    publication_queue_enqueued: $source.publication_queue_enqueued,
    publication_manifest_written: $source.publication_manifest_written,
    public_distribution_performed: $source.public_distribution_performed,
    public_release_published: $source.public_release_published,
    public_ga_claimed: $source.public_ga_claimed,
    public_claim_promoted: $source.public_claim_promoted,
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

echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation command result receipt release artifact publication denial route gate passed"
