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
    echo "missing memory write execution activation command result receipt final operator acknowledgement boundary source text: $label" >&2
    exit 1
  fi
}

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"
ENDPOINT="/api/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary"
SOURCE_COMMAND="/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary --json"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = ${EXPECTED_ROUTE_COUNT};" \
  "native gateway route/source command count includes memory write execution activation command result receipt final operator acknowledgement denial boundary"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_BOUNDARY_ENDPOINT" \
  "memory write execution activation command result receipt final operator acknowledgement denial boundary endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$ENDPOINT" \
  "memory write execution activation command result receipt final operator acknowledgement denial boundary endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" "$SOURCE_COMMAND" \
  "memory write execution activation command result receipt final operator acknowledgement denial boundary source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_report" \
  "memory write execution activation command result receipt final operator acknowledgement denial boundary report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_ready\"" \
  "memory write execution activation command result receipt final operator acknowledgement denial boundary ready field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"denied_by_activation_command_result_receipt_final_operator_acknowledgement_count\"" \
  "memory write execution activation command result receipt final operator acknowledgement denied count"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_endpoint_blocks_acceptance_and_authority" \
  "focused memory write execution activation command result receipt final operator acknowledgement boundary unit test"

TEST_LOG="$(mktemp /tmp/hepta-memory-write-execution-activation-command-result-receipt-final-operator-acknowledgement-boundary-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_endpoint_blocks_acceptance_and_authority \
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
    and .memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_ready == true
    and .activation_command_result_receipt_final_operator_acknowledgement_mode == "memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial"
    and .source_activation_command_result_receipt_operator_facing_summary_briefing_boundary_ready == true
    and .source_activation_command_result_receipt_operator_facing_summary_briefing_ready == true
    and .source_activation_command_result_receipt_operator_facing_summary_briefing_boundary_report_sha256 != ""
    and .minimum_required_samples >= 24
    and .memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
    and .memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
    and .memory_write_execution_activation_command_result_receipt_export_query_observability_denial_ready == true
    and .memory_write_execution_activation_command_result_receipt_retention_expiry_garbage_collection_denial_ready == true
    and .memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
    and .memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready == true
    and .memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
    and .memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and .memory_write_execution_activation_command_result_receipt_no_persistence_ready == true
    and .memory_write_execution_activation_command_noop_handoff_ready == true
    and .required_activation_command_result_receipt_final_operator_acknowledgement_surface_count == 12
    and .ready_activation_command_result_receipt_final_operator_acknowledgement_surface_count == 12
    and .side_effect_free_activation_command_result_receipt_final_operator_acknowledgement_surface_count == 12
    and .required_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
    and .activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
    and .blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
    and .noop_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 10
    and .allowed_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 0
    and .accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count == 0
    and .activation_command_result_receipt_final_operator_acknowledgement_denied_count == 10
    and .activation_command_result_receipt_final_operator_acknowledgement_performed_count == 0
    and .activation_command_result_receipt_final_operator_acknowledgement_recorded == false
    and .activation_command_result_receipt_final_operator_acknowledgement_persisted == false
    and .activation_command_result_receipt_final_operator_acknowledgement_materialized == false
    and .activation_command_result_receipt_final_operator_acknowledgement_filesystem_written == false
    and .activation_command_result_receipt_final_operator_acknowledgement_delivered == false
    and .activation_command_result_receipt_final_operator_acknowledgement_identity_accepted == false
    and .activation_command_result_receipt_final_operator_acknowledgement_signature_accepted == false
    and .activation_command_result_receipt_final_operator_acknowledgement_final_state_promoted == false
    and .activation_command_result_receipt_operator_final_acceptance_recorded == false
    and .activation_command_result_receipt_operator_final_acceptance_persisted == false
    and .telegram_send_performed == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .activation_command_result_receipt_recorded == false
    and .activation_command_result_receipt_persisted == false
    and .activation_command_result_receipt_accepted == false
    and .activation_command_completion_ack_recorded == false
    and .activation_allowed_by_result_receipt_final_operator_acknowledgement == false
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
    and .credential_read == false
    and .secret_file_read == false
    and .kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .public_release_published == false
    and .public_ga_claimed == false
    and .release_artifact_written == false
    and .install_executed == false
    and .launchd_mutated == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and (.activation_command_result_receipt_final_operator_acknowledgement_surfaces | length) == 12
    and (.activation_command_result_receipt_final_operator_acknowledgement_fixtures | length) == 10
    and (.activation_command_result_receipt_final_operator_acknowledgement_fixtures | all((.final_operator_acknowledgement_status | startswith("blocked")) and .acknowledgement_recorded == false and .acknowledgement_persisted == false and .acknowledgement_materialized == false and .acknowledgement_filesystem_written == false and .acknowledgement_delivered == false and .acknowledgement_accepted == false and .acknowledgement_final_state_promoted == false and .operator_final_acceptance_recorded == false and .operator_final_acceptance_persisted == false and .telegram_send_performed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .provider_invoked == false and .model_invoked == false and .final_acknowledgement_noop_confirmed == true))
    and ([.activation_command_result_receipt_final_operator_acknowledgement_fixtures[] | select(.source_summary_briefing_present == false)] | length) == 1
    and ([.activation_command_result_receipt_final_operator_acknowledgement_fixtures[] | select(.final_operator_acknowledgement_requested == true)] | length) == 10
    and ([.activation_command_result_receipt_final_operator_acknowledgement_fixtures[] | select(.acknowledgement_delivery_requested == true and .telegram_send_requested == true and .channel_delivery_requested == true)] | length) == 1
    and ([.activation_command_result_receipt_final_operator_acknowledgement_fixtures[] | select(.final_state_promotion_requested == true and .completion_promotion_requested == true)] | length) == 1
    and ([.activation_command_result_receipt_final_operator_acknowledgement_fixtures[] | select(.activation_from_acknowledgement_requested == true and .memory_write_acknowledgement_requested == true and .provider_prompt_acknowledgement_requested == true)] | length) == 1
    and ([.activation_command_result_receipt_final_operator_acknowledgement_fixtures[] | select(.external_send_acknowledgement_requested == true and .release_artifact_acknowledgement_requested == true and .install_acknowledgement_requested == true and .active_binary_acknowledgement_requested == true)] | length) == 1
    and .source_operator_facing_summary_briefing_denial_count == 20
    and .denied_by_activation_command_result_receipt_final_operator_acknowledgement_count == 37
    and (.denied_by_activation_command_result_receipt_final_operator_acknowledgement | length) == 37
    and .allowed_next_actions[0].action == "run_memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_require_live_gate"
    and .allowed_next_actions[1].action == "prepare_memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary"
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
    gate: "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_route_gate",
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
    accepts_operator_acknowledgement: false,
    records_acknowledgement: false,
    persists_acknowledgement: false,
    delivers_acknowledgement: false,
    promotes_final_state: false,
    writes_memory: false,
    executes_rollback: false,
    writes_kg: false,
    invokes_provider: false,
    reads_credentials: false,
    sends_externally: false,
    publishes_artifacts: false,
    installs_or_restarts: false,
    mutates_active_binary: false
  }'
