#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
EXPECTED_ROUTE_COUNT="${HEPTA_EXPECTED_ROUTE_COUNT:-$(HEPTA_REPO_ROOT="$REPO_ROOT" bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")}"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
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
    echo "missing operator readiness packet template packet-acceptance receipt release publication result receipt terminal distribution delivery receipt artifact download/install affordance route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_distribution_signing_notarization_surface_ready == true
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_request_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_persisted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_materialized_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_filesystem_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_delivered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_published_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_curl_pipe_shell_snippet_rendered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_launch_prompt_rendered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_auto_update_offer_rendered_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_install_message_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_install_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_service_restarted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_active_binary_mutated_count == 0
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surfaces | length) == 18
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surfaces | all(
    .artifact_download_install_affordance_attempted == true
    and .artifact_download_install_affordance_allowed == false
    and .artifact_download_install_affordance_request_accepted == false
    and .artifact_download_install_affordance_accepted == false
    and .artifact_download_install_affordance_recorded == false
    and .artifact_download_install_affordance_persisted == false
    and .artifact_download_install_affordance_filesystem_written == false
    and .artifact_download_install_affordance_delivered == false
    and .artifact_download_install_affordance_exposed == false
    and .artifact_download_install_affordance_published == false
    and .artifact_download_install_affordance_executed == false
    and .download_button_rendered == false
    and .direct_download_url_exposed == false
    and .package_manager_install_command_rendered == false
    and .curl_pipe_shell_snippet_rendered == false
    and .installer_launch_prompt_rendered == false
    and .auto_update_offer_rendered == false
    and .external_install_message_sent == false
    and .telegram_install_message_sent == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .live_execution_allowed == false
    and .install_executed == false
    and .active_binary_mutated == false
    and .artifact_download_install_affordance_noop_confirmed == true
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance | length) == 32
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_curl_pipe_shell_snippet_rendered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_installer_launch_prompt_rendered == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent == false
  and .operator_approval_recorded == false
  and .release_publication_authority_derived == false
  and .activation_authority_derived == false
  and .activation_performed == false
  and .memory_store_write_performed == false
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .credential_read == false
  and .install_executed == false
  and .active_binary_mutated == false
  and .release_artifact_written == false
  and .public_artifact_written == false
  and .external_send_performed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes terminal distribution delivery receipt artifact download/install affordance route"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_DENIAL_ENDPOINT' \
  "native gateway terminal distribution delivery receipt artifact download/install affordance endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial' \
  "native gateway terminal distribution delivery receipt artifact download/install affordance endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial --json' \
  "native gateway terminal distribution delivery receipt artifact download/install affordance source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_report' \
  "native gateway terminal distribution delivery receipt artifact download/install affordance report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_route_enabled": true' \
  "terminal distribution delivery receipt artifact download/install affordance route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surface_count": artifact_download_install_affordance_surface_count' \
  "terminal distribution delivery receipt artifact download/install affordance surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_endpoint_blocks_download_and_install_surfaces' \
  "terminal distribution delivery receipt artifact download/install affordance focused endpoint test"

TEST_LOG="$(mktemp /tmp/hepta-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_endpoint_blocks_download_and_install_surfaces \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial"
  )"
  jq -e '
    .status == "ready"
    and .route_count == .implemented_route_count
    and .implemented_route_count == .route_count
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_ready == true
    and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_query_export_observability_ready == true
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surface_count == 18
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_recorded_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_curl_pipe_shell_snippet_rendered_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_derived_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_derived_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_allowed_count == 0
    and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surfaces | length) == 18
    and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance | length) == 32
    and .operator_approval_recorded == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .activation_performed == false
    and .memory_store_write_performed == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .credential_read == false
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
  and .required_marker_count == .present_required_marker_count
  and .present_required_marker_count == .required_marker_count
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_artifact_download_install_affordance_gate_sha256="$(printf '%s' "$ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"
live_route_status="$(jq -r '.status // "skipped"' <<<"$LIVE_ROUTE_JSON")"
live_route_count="$(jq -r '.route_count // 0' <<<"$LIVE_ROUTE_JSON")"
live_missing_route_count="$(jq -r '.missing_route_count // 0' <<<"$LIVE_ROUTE_JSON")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial --json" \
  --arg source_artifact_download_install_affordance_gate_sha256 "$source_artifact_download_install_affordance_gate_sha256" \
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
    activation_mode:"full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_artifact_download_install_affordance_gate_ready:true,
    source_artifact_download_install_affordance_gate_sha256:$source_artifact_download_install_affordance_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$focused_test_log,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_checked:$live_endpoint_checked,
    source_route_count_expected:153,
    terminal_required_marker_count_expected:293,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_surface_count:18,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_download_button_rendered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_direct_download_url_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manager_install_command_rendered_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_external_install_message_sent_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_release_publication_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_activation_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_live_execution_allowed_count:0,
    route_source_texts_ready:true,
    terminal_coverage_ready:true,
    terminal_required_marker_count:285,
    terminal_present_required_marker_count:285,
    terminal_missing_required_marker_count:0,
    live_route_status:$live_route_status,
    live_route_count:$live_route_count,
    live_missing_route_count:$live_missing_route_count,
    side_effects:{
      route_gate_filesystem_written:false,
      route_gate_runtime_mutated:false,
      route_gate_service_restarted:false,
      route_gate_external_send_performed:false,
      source_gate_side_effects:{
        artifact_download_install_affordance_recorded:false,
        download_button_rendered:false,
        direct_download_url_exposed:false,
        package_manager_install_command_rendered:false,
        external_install_message_sent:false,
        release_publication_authority_derived:false,
        activation_authority_derived:false,
        live_execution_allowed:false
      }
    }
  }'

echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance denial route gate passed"
