#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-codex-rs/Cargo.toml}"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
REQUIRE_LIVE_ENDPOINT="${HEPTA_ROUTE_GATE_REQUIRE_LIVE_ENDPOINT:-0}"
EXPECTED_ROUTE_COUNT="${HEPTA_EXPECTED_ROUTE_COUNT:-$(bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")}"
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
    echo "missing operator readiness packet template packet-acceptance receipt release publication result receipt terminal distribution delivery receipt distribution artifact/manifest status route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

DISTRIBUTION_ARTIFACT_MANIFEST_STATUS_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_package_release_channel_status_exposure_ready == true
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_attempt_count == 18
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_recorded_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_persisted_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_filesystem_written_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manifest_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_index_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_metadata_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_external_artifact_manifest_status_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_telegram_artifact_manifest_status_sent_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_authority_derived_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_live_execution_allowed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_install_executed_count == 0
  and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_active_binary_mutated_count == 0
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surfaces | length) == 18
  and (.release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surfaces | all(
    .distribution_artifact_manifest_status_attempted == true
    and .distribution_artifact_manifest_status_allowed == false
    and .distribution_artifact_manifest_status_accepted == false
    and .distribution_artifact_manifest_status_recorded == false
    and .distribution_artifact_manifest_status_persisted == false
    and .distribution_artifact_manifest_status_filesystem_written == false
    and .distribution_artifact_manifest_status_exposed == false
    and .distribution_artifact_manifest_exposed == false
    and .package_manifest_status_exposed == false
    and .checksum_index_status_exposed == false
    and .artifact_metadata_status_exposed == false
    and .package_signing_status_exposed == false
    and .notarization_status_exposed == false
    and .external_artifact_manifest_status_sent == false
    and .telegram_artifact_manifest_status_sent == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .live_execution_allowed == false
    and .install_executed == false
    and .active_binary_mutated == false
    and .distribution_artifact_manifest_status_noop_confirmed == true
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status | length) >= 30
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed == false
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
' >/dev/null <<<"$DISTRIBUTION_ARTIFACT_MANIFEST_STATUS_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes terminal distribution delivery receipt distribution artifact/manifest status route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_DELIVERY_RECEIPT_DISTRIBUTION_ARTIFACT_MANIFEST_STATUS_DENIAL_ENDPOINT' \
  "native gateway terminal distribution delivery receipt distribution artifact/manifest status endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial' \
  "native gateway terminal distribution delivery receipt distribution artifact/manifest status endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial --json' \
  "native gateway terminal distribution delivery receipt distribution artifact/manifest status source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_report' \
  "native gateway terminal distribution delivery receipt distribution artifact/manifest status report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_route_enabled": true' \
  "terminal distribution delivery receipt distribution artifact/manifest status route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface_count": distribution_artifact_manifest_status_surface_count' \
  "terminal distribution delivery receipt distribution artifact/manifest status surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_endpoint_blocks_manifest_status_surfaces' \
  "terminal distribution delivery receipt distribution artifact/manifest status focused endpoint test"

TEST_LOG="$(mktemp /tmp/hepta-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_endpoint_blocks_manifest_status_surfaces \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_ready == true
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface_count == 18
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_accepted_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manifest_status_exposed_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_index_status_exposed_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_publication_authority_derived_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_authority_derived_count == 0
    and .release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_live_execution_allowed_count == 0
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

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_distribution_artifact_manifest_status_gate_sha256="$(printf '%s' "$DISTRIBUTION_ARTIFACT_MANIFEST_STATUS_JSON" | shasum -a 256 | awk '{print $1}')"
live_route_status="$(jq -r '.status // "skipped"' <<<"$LIVE_ROUTE_JSON")"
live_route_count="$(jq -r '.route_count // 0' <<<"$LIVE_ROUTE_JSON")"
live_missing_route_count="$(jq -r '.missing_route_count // 0' <<<"$LIVE_ROUTE_JSON")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_denial_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial --json" \
  --arg source_distribution_artifact_manifest_status_gate_sha256 "$source_distribution_artifact_manifest_status_gate_sha256" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg focused_test_log "$TEST_LOG" \
  --arg live_route_status "$live_route_status" \
  --argjson live_endpoint_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  --argjson live_route_count "$live_route_count" \
  --argjson live_missing_route_count "$live_missing_route_count" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_distribution_artifact_manifest_status_gate_ready:true,
    source_distribution_artifact_manifest_status_gate_sha256:$source_distribution_artifact_manifest_status_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$focused_test_log,
    live_endpoint_checked:$live_endpoint_checked,
    expected_route_count:$expected_route_count,
    route_source_texts_ready:true,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_surface_count:18,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_manifest_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_checksum_index_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_package_signing_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_notarization_status_exposed_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_release_publication_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_activation_authority_derived_count:0,
    release_publication_result_receipt_terminal_distribution_delivery_receipt_distribution_artifact_manifest_status_live_execution_allowed_count:0,
    live_route_status:$live_route_status,
    live_route_count:$live_route_count,
    live_missing_route_count:$live_missing_route_count,
    side_effects:{
      route_gate_filesystem_written:false,
      route_gate_runtime_mutated:false,
      route_gate_service_restarted:false,
      route_gate_external_send_performed:false,
      source_gate_side_effects:{
        distribution_artifact_manifest_status_exposed:false,
        package_manifest_status_exposed:false,
        checksum_index_status_exposed:false,
        package_signing_status_exposed:false,
        notarization_status_exposed:false,
        release_publication_authority_derived:false,
        activation_authority_derived:false,
        live_execution_allowed:false
      }
    }
  }'

echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt distribution artifact/manifest status denial route gate passed"
