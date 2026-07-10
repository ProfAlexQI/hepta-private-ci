#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
EXPECTED_ROUTE_COUNT="${HEPTA_EXPECTED_ROUTE_COUNT:-$(HEPTA_REPO_ROOT="$REPO_ROOT" bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")}"
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
    echo "missing operator readiness packet template packet-acceptance receipt release publication result receipt cancellation/supersession route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

CANCELLATION_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="${HEPTA_RELEASE_BIN:-}" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_ready == true
  and .source_packet_acceptance_receipt_release_publication_result_receipt_ordering_ready == true
  and .source_release_publication_result_receipt_ordering_surface_count == 14
  and .source_release_publication_result_receipt_ordering_attempt_count == 14
  and .source_release_publication_result_receipt_ordering_recorded_count == 0
  and .source_release_publication_result_receipt_ordering_persisted_count == 0
  and .source_release_publication_result_receipt_sequence_cursor_recorded_count == 0
  and .source_release_publication_result_receipt_monotonicity_state_recorded_count == 0
  and .source_release_publication_result_receipt_ordering_release_publication_authority_derived_count == 0
  and .source_release_publication_result_receipt_ordering_activation_authority_derived_count == 0
  and .release_publication_result_receipt_cancellation_supersession_surface_count == 14
  and .release_publication_result_receipt_cancellation_supersession_attempt_count == 14
  and .release_publication_result_receipt_cancellation_accepted_count == 0
  and .release_publication_result_receipt_cancellation_recorded_count == 0
  and .release_publication_result_receipt_cancellation_persisted_count == 0
  and .release_publication_result_receipt_revocation_accepted_count == 0
  and .release_publication_result_receipt_withdrawal_accepted_count == 0
  and .release_publication_result_receipt_supersession_accepted_count == 0
  and .release_publication_result_receipt_supersession_recorded_count == 0
  and .release_publication_result_receipt_supersession_persisted_count == 0
  and .release_publication_result_receipt_replacement_receipt_accepted_count == 0
  and .release_publication_result_receipt_replacement_receipt_recorded_count == 0
  and .release_publication_result_receipt_replacement_receipt_persisted_count == 0
  and .release_publication_result_receipt_tombstone_recorded_count == 0
  and .release_publication_result_receipt_tombstone_persisted_count == 0
  and .release_publication_result_receipt_delete_marker_recorded_count == 0
  and .release_publication_result_receipt_latest_replacement_accepted_count == 0
  and .release_publication_result_receipt_ack_replacement_accepted_count == 0
  and .release_publication_result_receipt_query_replacement_registered_count == 0
  and .release_publication_result_receipt_export_replacement_recorded_count == 0
  and .release_publication_result_receipt_observability_replacement_recorded_count == 0
  and .release_publication_result_receipt_cancellation_supersession_acceptance_recorded_count == 0
  and .release_publication_result_receipt_cancellation_supersession_release_publication_authority_derived_count == 0
  and .release_publication_result_receipt_cancellation_supersession_activation_authority_derived_count == 0
  and .release_publication_result_receipt_cancellation_supersession_live_execution_allowed_count == 0
  and (.release_publication_result_receipt_cancellation_supersession_surfaces | all(
    .cancellation_supersession_attempted == true
    and .cancellation_accepted == false
    and .cancellation_recorded == false
    and .revocation_accepted == false
    and .withdrawal_accepted == false
    and .supersession_accepted == false
    and .supersession_recorded == false
    and .replacement_receipt_accepted == false
    and .replacement_receipt_recorded == false
    and .tombstone_recorded == false
    and .delete_marker_recorded == false
    and .latest_replacement_accepted == false
    and .ack_replacement_accepted == false
    and .query_replacement_registered == false
    and .export_replacement_recorded == false
    and .observability_replacement_recorded == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .live_execution_allowed == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .cancellation_supersession_noop_confirmed == true
    and .release_publication_result_receipt_cancellation_supersession_status == "release_publication_result_receipt_cancellation_supersession_denied"
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession | length) == 24
  and .packet_acceptance_receipt_release_publication_result_receipt_cancellation_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_supersession_accepted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_replacement_receipt_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_tombstone_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_latest_replacement_accepted == false
  and .operator_approval_recorded == false
  and .activation_authority_derived == false
  and .activation_performed == false
  and .memory_store_write_performed == false
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .credential_read == false
  and .install_executed == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .external_send_performed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$CANCELLATION_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes packet acceptance receipt release publication result receipt cancellation/supersession route"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT' \
  "native gateway release publication result receipt cancellation/supersession endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial' \
  "native gateway release publication result receipt cancellation/supersession endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial --json' \
  "native gateway release publication result receipt cancellation/supersession source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_report' \
  "native gateway release publication result receipt cancellation/supersession report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_route_enabled": true' \
  "packet acceptance receipt release publication result receipt cancellation/supersession route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"release_publication_result_receipt_cancellation_supersession_surface_count": release_publication_result_receipt_cancellation_supersession_surface_count' \
  "packet acceptance receipt release publication result receipt cancellation/supersession surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded"' \
  "packet acceptance receipt release publication result receipt cancellation recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_release_publication_result_receipt_latest_replacement_accepted"' \
  "packet acceptance receipt release publication result receipt latest replacement denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_endpoint_blocks_cancellation_and_authority \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial"
  )"
  jq -e '
    .status == "ready"
    and .route_count == .implemented_route_count
    and .implemented_route_count == .route_count
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_ready == true
    and .source_packet_acceptance_receipt_release_publication_result_receipt_ordering_ready == true
    and .release_publication_result_receipt_cancellation_supersession_surface_count == 14
    and .release_publication_result_receipt_cancellation_supersession_attempt_count == 14
    and .release_publication_result_receipt_cancellation_recorded_count == 0
    and .release_publication_result_receipt_revocation_accepted_count == 0
    and .release_publication_result_receipt_withdrawal_accepted_count == 0
    and .release_publication_result_receipt_supersession_recorded_count == 0
    and .release_publication_result_receipt_replacement_receipt_recorded_count == 0
    and .release_publication_result_receipt_tombstone_recorded_count == 0
    and .release_publication_result_receipt_delete_marker_recorded_count == 0
    and .release_publication_result_receipt_latest_replacement_accepted_count == 0
    and .release_publication_result_receipt_cancellation_supersession_activation_authority_derived_count == 0
    and .release_publication_result_receipt_cancellation_supersession_live_execution_allowed_count == 0
    and (.release_publication_result_receipt_cancellation_supersession_surfaces | all(
      .cancellation_supersession_attempted == true
      and .cancellation_recorded == false
      and .revocation_accepted == false
      and .withdrawal_accepted == false
      and .supersession_recorded == false
      and .replacement_receipt_recorded == false
      and .tombstone_recorded == false
      and .delete_marker_recorded == false
      and .latest_replacement_accepted == false
      and .release_publication_authority_derived == false
      and .activation_authority_derived == false
      and .live_execution_allowed == false
      and .install_executed == false
      and .service_restarted == false
      and .active_binary_mutated == false
    ))
    and (.denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession | length) == 24
    and .operator_approval_recorded == false
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
source_cancellation_gate_sha256="$(printf '%s' "$CANCELLATION_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial --json" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg source_cancellation_gate_sha256 "$source_cancellation_gate_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$CANCELLATION_JSON" \
  --argjson terminal "$TERMINAL_COVERAGE_JSON" \
  --argjson live "$LIVE_ROUTE_JSON" \
  --argjson live_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:"hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_route_gate",
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_cancellation_supersession_gate_ready:true,
    source_cancellation_gate_sha256:$source_cancellation_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$test_log,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_checked:$live_checked,
    source_route_count_expected:153,
    terminal_required_marker_count_expected:293,
    source_packet_acceptance_receipt_release_publication_result_receipt_ordering_ready:$source.source_packet_acceptance_receipt_release_publication_result_receipt_ordering_ready,
    release_publication_result_receipt_cancellation_supersession_surface_count:$source.release_publication_result_receipt_cancellation_supersession_surface_count,
    release_publication_result_receipt_cancellation_recorded_count:$source.release_publication_result_receipt_cancellation_recorded_count,
    release_publication_result_receipt_supersession_recorded_count:$source.release_publication_result_receipt_supersession_recorded_count,
    release_publication_result_receipt_replacement_receipt_recorded_count:$source.release_publication_result_receipt_replacement_receipt_recorded_count,
    release_publication_result_receipt_latest_replacement_accepted_count:$source.release_publication_result_receipt_latest_replacement_accepted_count,
    release_publication_result_receipt_cancellation_supersession_activation_authority_derived_count:$source.release_publication_result_receipt_cancellation_supersession_activation_authority_derived_count,
    release_publication_result_receipt_cancellation_supersession_live_execution_allowed_count:$source.release_publication_result_receipt_cancellation_supersession_live_execution_allowed_count,
    route_source_texts_ready:true,
    terminal_coverage_ready:true,
    terminal_required_marker_count:$terminal.required_marker_count,
    terminal_present_required_marker_count:$terminal.present_required_marker_count,
    terminal_missing_required_marker_count:$terminal.missing_required_marker_count,
    live_route_status:($live.status // null),
    live_route_count:($live.route_count // null),
    live_missing_route_count:($live.missing_route_count // null),
    side_effects:{
      route_gate_filesystem_written:false,
      route_gate_runtime_mutated:false,
      route_gate_service_restarted:false,
      route_gate_external_send_performed:false,
      source_gate_side_effects:$source.side_effects,
      live_route_side_effects:($live.side_effects // {})
    }
  }'

echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt cancellation/supersession denial route gate passed"
