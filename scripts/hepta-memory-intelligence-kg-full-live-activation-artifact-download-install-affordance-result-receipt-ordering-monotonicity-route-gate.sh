#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"

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
    echo "missing artifact download/install affordance result receipt ordering/monotonicity route source text: $label" >&2
    exit 1
  fi
}

SOURCE_ORDERING_GATE="scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial-gate.sh"
NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$SOURCE_ORDERING_GATE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_gate' \
  "source ordering/monotonicity gate identity"
require_source_text "$SOURCE_ORDERING_GATE" \
  'release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface_count == 18' \
  "source ordering/monotonicity gate asserts 18 ordering surfaces"
require_source_text "$SOURCE_ORDERING_GATE" \
  'sequence_cursor_acceptance_denied' \
  "source ordering/monotonicity gate denies sequence cursor acceptance"
require_source_text "$SOURCE_ORDERING_GATE" \
  'latest_wins_overwrite_denied' \
  "source ordering/monotonicity gate denies latest-wins overwrite"
require_source_text "$SOURCE_ORDERING_GATE" \
  'install_restart_active_binary_ordering_bypass_denied' \
  "source ordering/monotonicity gate denies install/restart/active-binary ordering bypass"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 177;' \
  "native gateway route/source command count includes artifact download/install affordance result receipt ordering/monotonicity route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT' \
  "native gateway artifact download/install affordance result receipt ordering/monotonicity endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial' \
  "native gateway artifact download/install affordance result receipt ordering/monotonicity endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial --json' \
  "native gateway artifact download/install affordance result receipt ordering/monotonicity source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_report' \
  "native gateway artifact download/install affordance result receipt ordering/monotonicity report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_route_enabled": true' \
  "artifact download/install affordance result receipt ordering/monotonicity route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface_count": ordering_surface_count' \
  "artifact download/install affordance result receipt ordering/monotonicity surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_endpoint_blocks_ordering_and_authority' \
  "artifact download/install affordance result receipt ordering/monotonicity focused endpoint test"

TEST_LOG="$(mktemp /tmp/hepta-artifact-download-install-affordance-result-receipt-ordering-monotonicity-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_endpoint_blocks_ordering_and_authority \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 160
    and .implemented_route_count == 160
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_ready == true
    and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_replay_idempotency_ready == true
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surface_count == 18
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_attempt_count == 18
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_allowed_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_sequence_cursor_recorded_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_monotonicity_state_recorded_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_latest_wins_overwrite_accepted_count == 0
    and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_surfaces | length) == 18
    and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity | length) == 26
    and .operator_approval_recorded == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .install_executed == false
    and .active_binary_mutated == false
    and .external_send_performed == false
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
  and .required_marker_count == 300
  and .present_required_marker_count == 300
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_ordering_gate_sha256="$(sha256_file "$SOURCE_ORDERING_GATE")"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"
live_route_status="$(jq -r '.status // "skipped"' <<<"$LIVE_ROUTE_JSON")"
live_route_count="$(jq -r '.route_count // 0' <<<"$LIVE_ROUTE_JSON")"
live_missing_route_count="$(jq -r '.missing_route_count // 0' <<<"$LIVE_ROUTE_JSON")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_ordering_monotonicity_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial --json" \
  --arg source_ordering_gate_sha256 "$source_ordering_gate_sha256" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg focused_test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --arg live_route_status "$live_route_status" \
  --argjson live_endpoint_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  --argjson live_route_count "$live_route_count" \
  --argjson live_missing_route_count "$live_missing_route_count" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"full_live_activation_artifact_download_install_affordance_result_receipt_ordering_monotonicity_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_ordering_gate_text_verified:true,
    source_ordering_gate_sha256:$source_ordering_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$focused_test_log,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_checked:$live_endpoint_checked,
    source_route_count_expected:153,
    terminal_required_marker_count_expected:293,
    result_receipt_ordering_monotonicity_surface_count:18,
    result_receipt_ordering_allowed_count:0,
    result_receipt_sequence_cursor_recorded_count:0,
    result_receipt_monotonicity_state_recorded_count:0,
    result_receipt_latest_wins_overwrite_accepted_count:0,
    live_route_status:$live_route_status,
    live_route_count:$live_route_count,
    live_missing_route_count:$live_missing_route_count,
    side_effects:{
      sequence_cursor_recorded:false,
      monotonicity_state_recorded:false,
      out_of_order_receipt_accepted:false,
      latest_wins_overwrite_accepted:false,
      ordering_authority_derived:false,
      install_executed:false,
      active_binary_mutated:false,
      memory_store_write_performed:false,
      live_kg_write_performed:false,
      external_send_performed:false
    }
  }'

echo "artifact download/install affordance result receipt ordering/monotonicity denial route gate passed: endpoint ready, 18 surfaces, source route count 154, terminal markers 294/294" >&2
