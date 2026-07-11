#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT="${HEPTA_EXPECTED_ROUTE_COUNT:-$(bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_source_text() {
  local source_file="$1"
  local source_text="$2"
  local label="$3"

  if ! rg -Fq "$source_text" "$source_file"; then
    echo "missing memory write execution activation command result receipt release artifact publication boundary source text: $label" >&2
    exit 1
  fi
}

source "$REPO_ROOT/scripts/lib/hepta-source-set.sh"
NATIVE_GATEWAY_SOURCE="hepta-native-gateway-source-set-v1"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"
ENDPOINT="/api/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-release-artifact-publication-denial-boundary"
SOURCE_COMMAND="/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-release-artifact-publication-denial-boundary --json"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes memory release artifact publication boundary"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_BOUNDARY_ENDPOINT" \
  "memory release artifact publication boundary endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$ENDPOINT" \
  "memory release artifact publication boundary endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$SOURCE_COMMAND" \
  "memory release artifact publication boundary source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_report" \
  "memory release artifact publication boundary report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_ready\"" \
  "memory release artifact publication boundary ready field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"denied_by_activation_command_result_receipt_release_artifact_publication_count\"" \
  "memory release artifact publication denied count"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_endpoint_blocks_publication_and_authority" \
  "focused memory release artifact publication boundary unit test"

TEST_LOG="$(mktemp /tmp/hepta-memory-write-execution-activation-command-result-receipt-release-artifact-publication-boundary-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_endpoint_blocks_publication_and_authority \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(curl -fsS "$BASE_URL$ENDPOINT")"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_ready == true
    and .activation_command_result_receipt_release_artifact_publication_mode == "memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial"
    and .source_activation_command_result_receipt_terminal_operator_decision_public_claim_boundary_ready == true
    and .source_activation_command_result_receipt_terminal_operator_decision_public_claim_ready == true
    and .source_activation_command_result_receipt_terminal_operator_decision_public_claim_report_sha256 != ""
    and .source_terminal_operator_decision_public_claim_fixture_count == 10
    and .source_accepted_terminal_operator_decision_public_claim_fixture_count == 0
    and .source_terminal_operator_decision_performed_count == 0
    and .source_public_claim_promotion_performed_count == 0
    and .source_terminal_operator_decision_public_claim_denial_count == 55
    and .memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_ready == true
    and .memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready == true
    and .memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
    and .memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
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
    and .release_artifact_written_count == 0
    and .public_artifact_written_count == 0
    and .public_distribution_performed_count == 0
    and .publication_manifest_written_count == 0
    and .publication_queue_enqueued_count == 0
    and .memory_store_write_performed_count == 0
    and .activation_command_result_receipt_release_artifact_publication_allowed == false
    and .activation_command_result_receipt_release_artifact_publication_requested == false
    and .activation_command_result_receipt_release_artifact_publication_accepted == false
    and .activation_command_result_receipt_release_artifact_publication_recorded == false
    and .activation_command_result_receipt_release_artifact_publication_persisted == false
    and .activation_command_result_receipt_release_artifact_publication_materialized == false
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
    and .activation_allowed_by_release_artifact_publication == false
    and .activation_allowed_by_terminal_operator_decision == false
    and .activation_allowed_by_result_receipt == false
    and .activation_allowed == false
    and .activation_performed == false
    and .live_mutation_execution_performed == false
    and .memory_write_execution_performed == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .rollback_executed == false
    and .secret_material_read == false
    and .credential_read == false
    and .secret_file_read == false
    and .provider_invoked == false
    and .model_invoked == false
    and .install_executed == false
    and .launchd_mutated == false
    and .service_restart_performed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and (.activation_command_result_receipt_release_artifact_publication_surfaces | length) == 12
    and (.activation_command_result_receipt_release_artifact_publication_fixtures | length) == 10
    and (.activation_command_result_receipt_release_artifact_publication_fixtures | all((.release_artifact_publication_status | startswith("blocked")) and .release_artifact_publication_allowed == false and .release_artifact_publication_accepted == false and .release_artifact_publication_recorded == false and .release_artifact_publication_persisted == false and .release_artifact_filesystem_written == false and .release_artifact_written == false and .public_artifact_written == false and .publication_queue_enqueued == false and .publication_manifest_written == false and .public_distribution_performed == false and .public_release_published == false and .public_ga_claimed == false and .public_claim_promoted == false and .telegram_send_performed == false and .channel_send_performed == false and .external_send_performed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .release_artifact_publication_noop_confirmed == true))
    and ([.activation_command_result_receipt_release_artifact_publication_fixtures[] | select(.source_terminal_operator_decision_present == false)] | length) == 1
    and ([.activation_command_result_receipt_release_artifact_publication_fixtures[] | select(.release_artifact_publication_requested == true)] | length) == 5
    and ([.activation_command_result_receipt_release_artifact_publication_fixtures[] | select(.release_artifact_write_requested == true)] | length) == 1
    and ([.activation_command_result_receipt_release_artifact_publication_fixtures[] | select(.public_artifact_write_requested == true)] | length) == 1
    and ([.activation_command_result_receipt_release_artifact_publication_fixtures[] | select(.artifact_signature_requested == true and .artifact_notarization_requested == true)] | length) == 1
    and ([.activation_command_result_receipt_release_artifact_publication_fixtures[] | select(.service_restart_publication_requested == true and .active_binary_publication_requested == true)] | length) == 1
    and .denied_by_activation_command_result_receipt_release_artifact_publication_count == 69
    and (.denied_by_activation_command_result_receipt_release_artifact_publication | length) == 69
    and .release_artifact_publication_forbidden == true
    and .release_artifact_write_forbidden == true
    and .public_artifact_write_forbidden == true
    and .artifact_signature_notarization_forbidden == true
    and .publication_queue_forbidden == true
    and .publication_manifest_forbidden == true
    and .public_distribution_forbidden == true
    and .public_release_publication_forbidden == true
    and .public_ga_claim_forbidden == true
    and .terminal_operator_decision_release_approval_forbidden == true
    and .activation_from_release_artifact_publication_forbidden == true
    and .allowed_next_actions[0].action == "run_memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_require_live_gate"
    and .allowed_next_actions[0].publishes_release_artifact == false
    and .allowed_next_actions[0].writes_release_artifact == false
    and .allowed_next_actions[1].action == "prepare_scoped_memory_real_write_canary_operator_approval_packet"
    and .allowed_next_actions[1].requires_explicit_operator_acceptance == true
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_ROUTE_JSON"
fi

TERMINAL_COVERAGE_JSON="$(
  capture_json_report \
    "hepta-preflight-terminal-coverage-inventory-gate" \
    scripts/hepta-preflight-terminal-coverage-inventory-gate.sh
)"

jq -e '
  .status == "ready"
  and .preflight_terminal_coverage_inventory_ready == true
  and .present_required_marker_count == .required_marker_count
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_required_marker_count="$(jq -r '.required_marker_count // 0' <<<"$TERMINAL_COVERAGE_JSON")"
terminal_present_required_marker_count="$(jq -r '.present_required_marker_count // 0' <<<"$TERMINAL_COVERAGE_JSON")"
terminal_missing_required_marker_count="$(jq -r '.missing_required_marker_count // 0' <<<"$TERMINAL_COVERAGE_JSON")"
live_route_status="$(jq -r '.status // "skipped"' <<<"$LIVE_ROUTE_JSON")"
live_route_count="$(jq -r '.route_count // 0' <<<"$LIVE_ROUTE_JSON")"
live_missing_route_count="$(jq -r '.missing_route_count // 0' <<<"$LIVE_ROUTE_JSON")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg endpoint "$ENDPOINT" \
  --arg source_command "$SOURCE_COMMAND" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg live_route_status "$live_route_status" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson live_route_count "$live_route_count" \
  --argjson live_missing_route_count "$live_missing_route_count" \
  --argjson terminal_required_marker_count "$terminal_required_marker_count" \
  --argjson terminal_present_required_marker_count "$terminal_present_required_marker_count" \
  --argjson terminal_missing_required_marker_count "$terminal_missing_required_marker_count" \
  '{
    product: $product,
    runtime: $runtime,
    status: "ready",
    gate: "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_route_gate",
    endpoint: $endpoint,
    source_command: $source_command,
    base_url: $base_url,
    native_gateway_sha256: $native_gateway_sha256,
    expected_route_count: $expected_route_count,
    focused_test_passed: true,
    focused_test_log: $test_log,
    require_live_endpoint: (env.HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT // "0"),
    live_route_status: $live_route_status,
    live_route_count: $live_route_count,
    live_missing_route_count: $live_missing_route_count,
    terminal_coverage_sha256: $terminal_coverage_sha256,
    terminal_required_marker_count: $terminal_required_marker_count,
    terminal_present_required_marker_count: $terminal_present_required_marker_count,
    terminal_missing_required_marker_count: $terminal_missing_required_marker_count,
    route_gate_ready: true,
    publishes_release_artifact: false,
    claims_public_release: false,
    writes_release_artifact: false,
    writes_public_artifact: false,
    signs_or_notarizes_artifact: false,
    enqueues_publication: false,
    performs_public_distribution: false,
    writes_memory: false,
    executes_rollback: false,
    writes_kg: false,
    invokes_provider: false,
    reads_credentials: false,
    sends_externally: false,
    installs_or_restarts: false,
    mutates_active_binary: false
  }'

echo "Hepta memory live mutation operator write execution activation command result receipt release artifact publication denial boundary route gate passed"
