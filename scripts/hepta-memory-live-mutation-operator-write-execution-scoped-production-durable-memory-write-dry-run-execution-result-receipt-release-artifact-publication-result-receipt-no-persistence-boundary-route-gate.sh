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
    echo "missing scoped production durable Memory write dry-run result receipt release artifact publication result receipt source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ENDPOINT="/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-result-receipt-no-persistence-boundary"
SOURCE_COMMAND="/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-result-receipt-no-persistence-boundary --json"
FOCUSED_TEST="hepta_memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_blocks_receipt_persistence_publication_authority_execution_and_production_side_effects"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes scoped production durable Memory write dry-run result receipt release artifact publication result receipt no-persistence boundary"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_RESULT_RECEIPT_NO_PERSISTENCE_BOUNDARY_ENDPOINT" \
  "scoped production durable Memory write dry-run result receipt release artifact publication result receipt endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$ENDPOINT" \
  "scoped production durable Memory write dry-run result receipt release artifact publication result receipt endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$SOURCE_COMMAND" \
  "scoped production durable Memory write dry-run result receipt release artifact publication result receipt source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_report" \
  "scoped production durable Memory write dry-run result receipt release artifact publication result receipt report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recorded"' \
  "scoped production durable Memory write dry-run result receipt release artifact publication result receipt recorded false field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persisted"' \
  "scoped production durable Memory write dry-run result receipt release artifact publication result receipt persisted false field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_promoted"' \
  "scoped production durable Memory write dry-run result receipt release artifact publication result receipt authority false field"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$FOCUSED_TEST" \
  "focused scoped production durable Memory write dry-run result receipt release artifact publication result receipt no-persistence unit test"

TEST_LOG="$(mktemp /tmp/hepta-scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  "$FOCUSED_TEST" \
  -- --nocapture >"$TEST_LOG"

SCRIPT_GATE_JSON="$(
  capture_json_report \
    "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-result-receipt-no-persistence-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-result-receipt-no-persistence-boundary-gate.sh
)"

jq -e '
  .status == "ready"
  and .memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_ready == true
  and .scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_accepted == true
  and .source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_ready == true
  and .source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count == 1
  and .source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count == 9
  and .source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_count >= 100
  and (.dry_run_execution_result_receipt_release_artifact_publication_result_receipt_hash_sha256 | type == "string" and length > 0)
  and (.dry_run_execution_result_receipt_release_artifact_publication_result_receipt_result_hash_sha256 | type == "string" and length > 0)
  and .required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_surface_count == 14
  and .ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_surface_count == 14
  and .scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_fixture_count == 10
  and .accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_fixture_count == 1
  and .blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_fixture_count == 9
  and .denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_count >= 115
  and .scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_result_accepted_count == 1
  and .dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recorded_count == 0
  and .dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persisted_count == 0
  and .dry_run_execution_result_receipt_release_artifact_publication_result_receipt_ledger_written_count == 0
  and .dry_run_execution_result_receipt_release_artifact_publication_completion_ack_recorded_count == 0
  and .dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_promoted_count == 0
  and .dry_run_execution_result_receipt_release_artifact_publication_recorded_count == 0
  and .dry_run_execution_result_receipt_release_artifact_written_count == 0
  and .dry_run_execution_result_receipt_public_artifact_written_count == 0
  and .activation_performed_count == 0
  and .dry_run_execution_executed_count == 0
  and .production_durable_memory_store_write_performed_count == 0
  and .memory_store_write_performed_count == 0
  and .wal_write_performed_count == 0
  and .receipt_persisted_count == 0
  and .live_kg_write_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .credential_read_count == 0
  and .channel_send_performed_count == 0
  and .external_send_performed_count == 0
  and .release_artifact_written_count == 0
  and .public_artifact_written_count == 0
  and .install_executed_count == 0
  and .service_restarted_count == 0
  and .active_binary_mutated_count == 0
  and .dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recorded == false
  and .dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persisted == false
  and .dry_run_execution_result_receipt_release_artifact_publication_result_receipt_ledger_written == false
  and .dry_run_execution_result_receipt_release_artifact_publication_result_receipt_indexed == false
  and .dry_run_execution_result_receipt_release_artifact_publication_result_receipt_delivered == false
  and .dry_run_execution_result_receipt_release_artifact_publication_completion_ack_recorded == false
  and .dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_promoted == false
  and .dry_run_execution_result_receipt_release_artifact_written == false
  and .dry_run_execution_result_receipt_public_artifact_written == false
  and .activation_performed == false
  and .dry_run_execution_executed == false
  and .production_durable_memory_store_write_performed == false
  and .memory_store_write_performed == false
  and .wal_write_performed == false
  and .receipt_persisted == false
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .release_artifact_written == false
  and .public_artifact_written == false
  and .install_executed == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .side_effects.scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_performed == true
  and .side_effects.scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_result_accepted == true
  and .side_effects.dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recorded == false
  and .side_effects.dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persisted == false
  and .side_effects.dry_run_execution_result_receipt_release_artifact_publication_result_receipt_ledger_written == false
  and .side_effects.dry_run_execution_result_receipt_release_artifact_publication_completion_ack_recorded == false
  and .side_effects.dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_promoted == false
  and .side_effects.activation_performed == false
  and .side_effects.dry_run_execution_executed == false
  and .side_effects.production_durable_memory_store_write_performed == false
  and .side_effects.memory_store_write_performed == false
  and .side_effects.wal_write_performed == false
  and .side_effects.receipt_persisted == false
  and .side_effects.external_send_performed == false
  and .side_effects.release_artifact_written == false
  and .side_effects.public_artifact_written == false
  and .side_effects.active_binary_mutated == false
  and .allowed_next_actions[0].action == "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_require_live_gate"
  and .allowed_next_actions[0].persists_publication_result_receipt == false
  and .allowed_next_actions[0].publishes_release_artifact == false
  and .allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_replay_idempotency_denial_boundary"
  and .allowed_next_actions[1].accepts_replay == false
' >/dev/null <<<"$SCRIPT_GATE_JSON"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(curl -fsS "$BASE_URL$ENDPOINT")"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_ready == true
    and .scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_accepted == true
    and .accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_fixture_count == 1
    and .blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_fixture_count == 9
    and .denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_count >= 115
    and .dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recorded == false
    and .dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persisted == false
    and .dry_run_execution_result_receipt_release_artifact_publication_result_receipt_ledger_written == false
    and .dry_run_execution_result_receipt_release_artifact_publication_completion_ack_recorded == false
    and .dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_promoted == false
    and .dry_run_execution_result_receipt_release_artifact_written == false
    and .dry_run_execution_result_receipt_public_artifact_written == false
    and .activation_performed == false
    and .dry_run_execution_executed == false
    and .production_durable_memory_store_write_performed == false
    and .memory_store_write_performed == false
    and .wal_write_performed == false
    and .receipt_persisted == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_replay_idempotency_denial_boundary"
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
    gate: "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_route_gate",
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
    records_publication_result_receipt: false,
    persists_publication_result_receipt: false,
    writes_receipt_ledger: false,
    indexes_receipt: false,
    queues_or_delivers_receipt: false,
    exports_or_queries_receipt: false,
    records_observability: false,
    records_completion_ack: false,
    publishes_release_artifact: false,
    claims_public_release: false,
    writes_release_artifact: false,
    writes_public_artifact: false,
    promotes_activation_authority: false,
    executes_dry_run: false,
    writes_production_durable_memory: false,
    writes_memory_store: false,
    writes_wal: false,
    persists_receipt: false,
    writes_kg: false,
    invokes_provider: false,
    reads_credentials: false,
    sends_externally: false,
    installs_or_restarts: false,
    mutates_active_binary: false
  }'
