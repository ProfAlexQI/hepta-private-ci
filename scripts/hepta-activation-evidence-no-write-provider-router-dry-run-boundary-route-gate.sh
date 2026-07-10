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
    echo "missing activation evidence no-write provider-router boundary source text: $label" >&2
    exit 1
  fi
}

SOURCE_PROVIDER_GATE="scripts/hepta-provider-router-dry-run-envelope-readback-audit-route-gate.sh"
SOURCE_EVIDENCE_GATE="scripts/hepta-core-activation-evidence-receipt-materialization-dry-run-gate.sh"
NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"
ENDPOINT="/api/hepta-activation-evidence-no-write-provider-router-dry-run-boundary"
SOURCE_COMMAND="/hepta-activation-evidence-no-write-provider-router-dry-run-boundary --json"

PROVIDER_SOURCE_JSON="$(
  capture_json_report \
    "hepta-provider-router-dry-run-envelope-readback-audit-route-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_EXPECTED_ROUTE_COUNT="$EXPECTED_ROUTE_COUNT" \
      "$SOURCE_PROVIDER_GATE"
)"

jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
  .runtime == "hepta"
  and .status == "ready"
  and .expected_route_count == $expected
  and .provider_router_dry_run_envelope_readback_audit_ready == true
  and .dry_run_envelope_preview_constructed == true
  and .dry_run_envelope_readback_audit_performed == true
  and .dry_run_envelope_readback_hash_matched == true
  and .dry_run_envelope_receipt_rendered == true
  and .dry_run_envelope_receipt_persisted == false
  and .dry_run_envelope_receipt_accepted == false
  and .dry_run_envelope_executed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .live_kg_write_performed == false
  and .memory_store_write_performed == false
  and .external_send_performed == false
' >/dev/null <<<"$PROVIDER_SOURCE_JSON"

EVIDENCE_SOURCE_JSON="$(
  capture_json_report \
    "hepta-core-activation-evidence-receipt-materialization-dry-run-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      "$SOURCE_EVIDENCE_GATE"
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_evidence_receipt_materialization_dry_run_gate"
  and .receipt_materialization_dry_run_ready == true
  and .required_source_count == 5
  and .ready_source_count == 5
  and .activation_blocking_source_count == 5
  and .minimum_required_long_soak_samples >= 24
  and .required_no_write_sink_surface_count == 6
  and .ready_no_write_sink_surface_count == 6
  and .required_materialization_fixture_count == 3
  and .blocked_materialization_fixture_count == 3
  and .allowed_materialization_fixture_count == 0
  and .required_output_path_allowlist_entry_count == 6
  and .required_output_path_binding_count == 8
  and .recorded_output_path_binding_count == 0
  and .long_soak_executed_by_this_gate == false
  and .long_soak_evidence_recorded == false
  and .operator_approval_recorded == false
  and .activation_request_recorded == false
  and .ledger_record_recorded == false
  and .receipt_materialization_plan_recorded == false
  and .receipt_materialized == false
  and .receipt_persisted == false
  and .filesystem_persistence_allowed == false
  and .filesystem_persistence_execution_performed == false
  and .workspace_write_performed == false
  and .activation_allowed == false
  and .active_wiring_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$EVIDENCE_SOURCE_JSON"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes activation evidence no-write boundary"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "HEPTA_ACTIVATION_EVIDENCE_NO_WRITE_PROVIDER_ROUTER_DRY_RUN_BOUNDARY_ENDPOINT" \
  "activation evidence no-write endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$ENDPOINT" \
  "activation evidence no-write endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$SOURCE_COMMAND" \
  "activation evidence no-write source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_activation_evidence_no_write_provider_router_dry_run_boundary_report" \
  "activation evidence no-write report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"activation_evidence_no_write_provider_router_dry_run_boundary_route_enabled\"" \
  "activation evidence no-write route enabled field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"denied_by_activation_evidence_no_write_provider_router_dry_run_boundary_count\"" \
  "activation evidence no-write denial count field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "insert_report_json!" \
  "activation evidence no-write segmented report assembly"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_activation_evidence_no_write_provider_router_dry_run_boundary_endpoint_blocks_evidence_persistence_and_invocation_side_effects" \
  "focused activation evidence no-write unit test"

TEST_LOG="$(mktemp /tmp/hepta-activation-evidence-no-write-provider-router-boundary-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_activation_evidence_no_write_provider_router_dry_run_boundary_endpoint_blocks_evidence_persistence_and_invocation_side_effects \
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
    and .activation_evidence_no_write_provider_router_dry_run_boundary_route_enabled == true
    and .activation_evidence_no_write_provider_router_dry_run_boundary_ready == true
    and .activation_evidence_no_write_provider_router_dry_run_boundary_status == "blocked_report_only"
    and .source_provider_router_dry_run_envelope_readback_audit_ready == true
    and .activation_evidence_candidate_count == 8
    and .accepted_activation_evidence_candidate_count == 0
    and (.activation_evidence_candidates | length) == 8
    and (.activation_evidence_candidates | all(.accepted == false))
    and .required_materialization_field_count == 20
    and .recorded_materialization_field_count == 0
    and .required_no_write_sink_surface_count == 6
    and .ready_no_write_sink_surface_count == 6
    and .materialization_fixture_count == 3
    and .blocked_materialization_fixture_count == 3
    and .allowed_materialization_fixture_count == 0
    and .output_path_allowlist_entry_count == 6
    and .output_path_binding_count == 8
    and .recorded_output_path_binding_count == 0
    and .redacted_or_hashed_output_path_binding_count == 8
    and .boundary_readback_performed == true
    and .boundary_readback_hash_matched == true
    and .long_soak_executed_by_this_route == false
    and .long_soak_evidence_recorded == false
    and .activation_evidence_recorded == false
    and .activation_evidence_persisted == false
    and .activation_evidence_materialized == false
    and .activation_evidence_filesystem_written == false
    and .receipt_materialization_plan_recorded == false
    and .receipt_materialized == false
    and .receipt_persisted == false
    and .receipt_ledger_recorded == false
    and .output_path_selected == false
    and .fresh_long_soak_evidence_accepted == false
    and .operator_approval_recorded == false
    and .filesystem_persistence_approval_recorded == false
    and .filesystem_persistence_allowed == false
    and .filesystem_persistence_execution_performed == false
    and .activation_allowed == false
    and .active_wiring_allowed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .live_kg_write_performed == false
    and .memory_store_write_performed == false
    and .channel_send_performed == false
    and .external_send_performed == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .denied_by_activation_evidence_no_write_provider_router_dry_run_boundary_count == 20
    and (.denied_by_activation_evidence_no_write_provider_router_dry_run_boundary | length) == 20
    and (.boundary_steps | length) == 4
    and .allowed_next_actions[0].action == "first_model_invocation_separate_approval_slice"
    and .allowed_next_actions[0].uses_activation_evidence_no_write_provider_router_dry_run_boundary == true
    and .allowed_next_actions[0].requires_fresh_operator_approval == true
    and .allowed_next_actions[0].requires_fresh_long_soak_evidence == true
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
provider_source_gate_sha256="$(sha256_file "$SOURCE_PROVIDER_GATE")"
evidence_source_gate_sha256="$(sha256_file "$SOURCE_EVIDENCE_GATE")"
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
  --arg provider_source_gate_sha256 "$provider_source_gate_sha256" \
  --arg evidence_source_gate_sha256 "$evidence_source_gate_sha256" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg live_route_status "$live_route_status" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson live_endpoint_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  --argjson live_route_count "$live_route_count" \
  --argjson live_missing_route_count "$live_missing_route_count" \
  --argjson terminal_required_marker_count "$terminal_required_marker_count" \
  --argjson terminal_present_required_marker_count "$terminal_present_required_marker_count" \
  --argjson terminal_missing_required_marker_count "$terminal_missing_required_marker_count" \
  '{
    product: $product,
    runtime: $runtime,
    status: "ready",
    base_url: $base_url,
    gate: "hepta_activation_evidence_no_write_provider_router_dry_run_boundary_route_gate",
    endpoint: $endpoint,
    source_command: $source_command,
    native_route: true,
    side_effect_free: true,
    expected_route_count: $expected_route_count,
    route_source_text_verified: true,
    focused_endpoint_test_passed: true,
    test_log: $test_log,
    native_gateway_sha256: $native_gateway_sha256,
    provider_source_gate_sha256: $provider_source_gate_sha256,
    evidence_source_gate_sha256: $evidence_source_gate_sha256,
    source_provider_router_dry_run_envelope_readback_audit_ready: true,
    source_core_activation_evidence_receipt_materialization_dry_run_ready: true,
    activation_evidence_candidate_count: 8,
    accepted_activation_evidence_candidate_count: 0,
    required_materialization_field_count: 20,
    recorded_materialization_field_count: 0,
    required_no_write_sink_surface_count: 6,
    ready_no_write_sink_surface_count: 6,
    materialization_fixture_count: 3,
    blocked_materialization_fixture_count: 3,
    output_path_allowlist_entry_count: 6,
    output_path_binding_count: 8,
    recorded_output_path_binding_count: 0,
    boundary_readback_performed: true,
    boundary_readback_hash_matched: true,
    denied_by_activation_evidence_no_write_provider_router_dry_run_boundary_count: 20,
    terminal_required_marker_count: $terminal_required_marker_count,
    terminal_present_required_marker_count: $terminal_present_required_marker_count,
    terminal_missing_required_marker_count: $terminal_missing_required_marker_count,
    terminal_coverage_sha256: $terminal_coverage_sha256,
    live_endpoint_checked: $live_endpoint_checked,
    live_route_status: $live_route_status,
    live_route_count: $live_route_count,
    live_missing_route_count: $live_missing_route_count,
    side_effects: {
      activation_evidence_recorded: false,
      activation_evidence_persisted: false,
      activation_evidence_materialized: false,
      activation_evidence_filesystem_written: false,
      receipt_materialized: false,
      receipt_persisted: false,
      receipt_ledger_recorded: false,
      output_path_selected: false,
      fresh_long_soak_evidence_accepted: false,
      operator_approval_recorded: false,
      filesystem_persistence_approval_recorded: false,
      filesystem_persistence_execution_performed: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      live_kg_write_performed: false,
      memory_store_write_performed: false,
      channel_send_performed: false,
      external_send_performed: false,
      install_executed: false,
      service_restarted: false,
      active_binary_mutated: false,
      release_artifact_written: false,
      public_artifact_written: false,
      filesystem_written: false
    },
    next_slice: "first_model_invocation_separate_approval_slice"
  }'

echo "Hepta activation evidence no-write provider-router dry-run boundary route gate passed"
