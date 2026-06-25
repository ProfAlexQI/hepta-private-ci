#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT=189
RELEASE_BIN="${HEPTA_RELEASE_BIN:-}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

if [[ -z "$RELEASE_BIN" && -x "$HOME/.local/opt/hepta/bin/hepta" ]]; then
  RELEASE_BIN="$HOME/.local/opt/hepta/bin/hepta"
fi

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
    echo "missing artifact download/install affordance result receipt final operator acknowledgement route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

FINAL_ACKNOWLEDGEMENT_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh
)"

jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);

  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_facing_summary_briefing_ready == true
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_denied_count == 18
  and zero_fields(.; [
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_accepted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_persisted_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_filesystem_written_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_delivered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_received_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_confirmed_recorded_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_channel_acknowledgement_delivered_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_external_acknowledgement_sent_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_release_publication_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_activation_authority_derived_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_install_executed_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_active_binary_mutated_count",
    "release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_external_send_count"
  ])
  and false_fields(.; [
    "artifact_download_install_affordance_result_receipt_recorded",
    "artifact_download_install_affordance_result_receipt_persisted",
    "operator_approval_recorded",
    "release_publication_authority_derived",
    "activation_authority_derived",
    "install_executed",
    "service_restarted",
    "active_binary_mutated",
    "external_send_performed"
  ])
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surfaces | length) == 18
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surfaces[] | select(.final_operator_acknowledgement_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surfaces[] | select(.telegram_acknowledgement_requested == true)] | length) == 1
  and ([.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surfaces[] | select(.install_restart_active_binary_acknowledgement_requested == true)] | length) == 1
  and (.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_terminal_decision_status_promotion_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .records_terminal_decision == false
    and .records_status_promotion == false
    and .accepts_operator_acknowledgement == false
    and .derives_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .sends_externally == false
  ))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$FINAL_ACKNOWLEDGEMENT_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 189;' \
  "native gateway route/source command count includes artifact download/install affordance result receipt final operator acknowledgement route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT' \
  "native gateway artifact download/install affordance result receipt final operator acknowledgement endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-final-operator-acknowledgement-non-acceptance-denial' \
  "native gateway artifact download/install affordance result receipt final operator acknowledgement endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json' \
  "native gateway artifact download/install affordance result receipt final operator acknowledgement source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report' \
  "native gateway artifact download/install affordance result receipt final operator acknowledgement report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled": true' \
  "artifact download/install affordance result receipt final operator acknowledgement route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surface_count": final_acknowledgement_surface_count' \
  "artifact download/install affordance result receipt final operator acknowledgement surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_endpoint_blocks_acknowledgement_and_authority' \
  "artifact download/install affordance result receipt final operator acknowledgement focused endpoint test"

TEST_LOG="$(mktemp /tmp/hepta-artifact-download-install-affordance-result-receipt-final-operator-acknowledgement-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_endpoint_blocks_acknowledgement_and_authority \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-final-operator-acknowledgement-non-acceptance-denial"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_surface_count == 18
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_recorded_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_received_recorded_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_confirmed_recorded_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_release_publication_authority_derived_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_activation_authority_derived_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_final_acknowledgement_install_executed_count == 0
    and .operator_approval_recorded == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .activation_performed == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .credential_read == false
    and .install_executed == false
    and .external_send_performed == false
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_ROUTE_JSON"
fi

TERMINAL_COVERAGE_JSON="$(
  capture_json_report \
    "hepta-preflight-terminal-coverage-inventory-gate" \
    scripts/hepta-preflight-terminal-coverage-inventory-gate.sh
)"

jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
  .status == "ready"
  and .preflight_terminal_coverage_inventory_ready == true
  and .required_marker_count == 300
  and .present_required_marker_count == 300
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_final_acknowledgement_gate_sha256="$(printf '%s' "$FINAL_ACKNOWLEDGEMENT_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"
live_route_status="$(jq -r '.status // "skipped"' <<<"$LIVE_ROUTE_JSON")"
live_route_count="$(jq -r '.route_count // 0' <<<"$LIVE_ROUTE_JSON")"
live_missing_route_count="$(jq -r '.missing_route_count // 0' <<<"$LIVE_ROUTE_JSON")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-final-operator-acknowledgement-non-acceptance-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg source_final_acknowledgement_gate_sha256 "$source_final_acknowledgement_gate_sha256" \
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
    activation_mode:"full_live_activation_artifact_download_install_affordance_result_receipt_final_operator_acknowledgement_native_route_status",
    side_effect_free:true,
    native_gateway_sha256:$native_gateway_sha256,
    source_final_acknowledgement_gate_sha256:$source_final_acknowledgement_gate_sha256,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    focused_test_log:$focused_test_log,
    live_endpoint_checked:$live_endpoint_checked,
    live_route_status:$live_route_status,
    live_route_count:$live_route_count,
    live_missing_route_count:$live_missing_route_count,
    expected_route_count:189,
    expected_terminal_required_marker_count:300,
    route_gate_ready:true,
    source_gate_ready:true,
    operator_acknowledgement_accepted:false,
    operator_acknowledgement_recorded:false,
    operator_approval_recorded:false,
    release_publication_authority_derived:false,
    activation_authority_derived:false,
    install_executed:false,
    service_restarted:false,
    active_binary_mutated:false,
    memory_store_write_performed:false,
    live_kg_write_performed:false,
    provider_invoked:false,
    model_invoked:false,
    credential_read:false,
    external_send_performed:false,
    next_slice:"terminal_decision_status_promotion_denial",
    side_effects:{
      filesystem_written:false,
      operator_acknowledgement_accepted:false,
      operator_acknowledgement_recorded:false,
      operator_approval_recorded:false,
      release_publication_authority_derived:false,
      activation_authority_derived:false,
      install_executed:false,
      service_restarted:false,
      active_binary_mutated:false,
      memory_store_write_performed:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      external_send_performed:false
    }
  }'

echo "Hepta memory/intelligence/KG full live activation artifact download/install affordance result receipt final operator acknowledgement route gate passed"
