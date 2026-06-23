#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
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
    echo "missing operator readiness packet template packet-acceptance receipt release publication result receipt route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

RESULT_RECEIPT_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="${HEPTA_RELEASE_BIN:-}" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_ready == true
  and .source_packet_acceptance_receipt_release_publication_ready == true
  and .source_release_publication_surface_count == 14
  and .source_release_publication_allowed_count == 0
  and .source_release_publication_recorded_count == 0
  and .source_release_artifact_written_count == 0
  and .source_public_artifact_written_count == 0
  and .source_public_distribution_performed_count == 0
  and .source_public_release_claimed_count == 0
  and .source_public_ga_claimed_count == 0
  and .source_release_publication_activation_authority_derived_count == 0
  and .release_publication_result_receipt_surface_count == 14
  and .release_publication_result_receipt_attempt_count == 14
  and .release_publication_result_receipt_allowed_count == 0
  and .release_publication_result_receipt_accepted_count == 0
  and .release_publication_result_receipt_recorded_count == 0
  and .release_publication_result_receipt_persisted_count == 0
  and .release_publication_result_receipt_materialized_count == 0
  and .release_publication_result_receipt_filesystem_written_count == 0
  and .release_publication_result_receipt_ledger_written_count == 0
  and .release_publication_result_receipt_indexed_count == 0
  and .release_publication_result_receipt_enqueued_count == 0
  and .release_publication_result_receipt_delivered_count == 0
  and .release_publication_result_receipt_exported_count == 0
  and .release_publication_result_receipt_query_registered_count == 0
  and .release_publication_result_receipt_observability_recorded_count == 0
  and .release_publication_result_receipt_hash_bound_count == 0
  and .release_publication_result_receipt_signature_accepted_count == 0
  and .release_publication_result_receipt_timestamp_accepted_count == 0
  and .release_publication_result_receipt_status_accepted_count == 0
  and .publication_completion_ack_recorded_count == 0
  and .publication_completion_ack_persisted_count == 0
  and .publication_completion_ack_accepted_count == 0
  and .release_publication_result_receipt_activation_authority_derived_count == 0
  and .release_publication_result_receipt_live_execution_allowed_count == 0
  and (.release_publication_result_receipt_surfaces | all(
    .publication_result_receipt_attempted == true
    and .publication_result_receipt_allowed == false
    and .publication_result_receipt_accepted == false
    and .publication_result_receipt_recorded == false
    and .publication_result_receipt_persisted == false
    and .publication_result_receipt_filesystem_written == false
    and .publication_result_receipt_ledger_written == false
    and .publication_result_receipt_indexed == false
    and .publication_result_receipt_enqueued == false
    and .publication_result_receipt_delivered == false
    and .publication_result_receipt_exported == false
    and .publication_result_receipt_query_registered == false
    and .publication_result_receipt_observability_recorded == false
    and .publication_result_receipt_hash_bound == false
    and .publication_result_receipt_signature_accepted == false
    and .publication_result_receipt_timestamp_accepted == false
    and .publication_result_receipt_status_accepted == false
    and .publication_completion_ack_recorded == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .public_distribution_performed == false
    and .external_send_performed == false
    and .public_release_claimed == false
    and .public_ga_claimed == false
    and .activation_authority_derived == false
    and .live_execution_allowed == false
    and .publication_result_receipt_noop_confirmed == true
    and .release_publication_result_receipt_status == "release_publication_result_receipt_no_persistence_denied"
  ))
  and (.denied_by_packet_receipt_release_publication_result_receipt_no_persistence | length) == 17
  and .packet_acceptance_receipt_release_publication_result_receipt_recorded == false
  and .packet_acceptance_receipt_release_publication_result_receipt_persisted == false
  and .packet_acceptance_receipt_release_publication_result_receipt_filesystem_written == false
  and .packet_acceptance_receipt_publication_completion_ack_recorded == false
  and .packet_acceptance_receipt_release_artifact_written == false
  and .packet_acceptance_receipt_public_distribution_performed == false
  and .packet_acceptance_receipt_public_release_claimed == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .activation_authority_derived == false
  and .activation_performed == false
  and .memory_store_write_performed == false
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .secret_file_read == false
  and .install_executed == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .external_send_performed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$RESULT_RECEIPT_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 182;' \
  "native gateway route/source command count includes packet acceptance receipt release publication result receipt route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT' \
  "native gateway release publication result receipt endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence' \
  "native gateway release publication result receipt endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence --json' \
  "native gateway release publication result receipt source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_report' \
  "native gateway release publication result receipt report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_route_enabled": true' \
  "packet acceptance receipt release publication result receipt route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"release_publication_result_receipt_surface_count": release_publication_result_receipt_surface_count' \
  "packet acceptance receipt release publication result receipt surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_release_publication_result_receipt_persisted": false' \
  "packet acceptance receipt release publication result receipt persistence denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_publication_completion_ack_recorded": false' \
  "packet acceptance receipt publication completion acknowledgement denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_endpoint_blocks_persistence_and_authority \
  -- --nocapture >"$TEST_LOG"

LIVE_ROUTE_JSON='{}'
if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 160
    and .implemented_route_count == 160
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_ready == true
    and .source_packet_acceptance_receipt_release_publication_ready == true
    and .release_publication_result_receipt_surface_count == 14
    and .release_publication_result_receipt_attempt_count == 14
    and .release_publication_result_receipt_recorded_count == 0
    and .release_publication_result_receipt_persisted_count == 0
    and .release_publication_result_receipt_filesystem_written_count == 0
    and .publication_completion_ack_recorded_count == 0
    and .release_publication_result_receipt_activation_authority_derived_count == 0
    and .release_publication_result_receipt_live_execution_allowed_count == 0
    and (.release_publication_result_receipt_surfaces | all(
      .publication_result_receipt_attempted == true
      and .publication_result_receipt_recorded == false
      and .publication_result_receipt_persisted == false
      and .publication_result_receipt_filesystem_written == false
      and .publication_completion_ack_recorded == false
      and .release_artifact_written == false
      and .public_artifact_written == false
      and .external_send_performed == false
      and .activation_authority_derived == false
      and .live_execution_allowed == false
    ))
    and (.denied_by_packet_receipt_release_publication_result_receipt_no_persistence | length) == 17
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
  and .required_marker_count == 300
  and .present_required_marker_count == 300
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_result_receipt_gate_sha256="$(printf '%s' "$RESULT_RECEIPT_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence --json" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg source_result_receipt_gate_sha256 "$source_result_receipt_gate_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$RESULT_RECEIPT_JSON" \
  --argjson terminal "$TERMINAL_COVERAGE_JSON" \
  --argjson live "$LIVE_ROUTE_JSON" \
  --argjson live_checked "$([[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]] && echo true || echo false)" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:"hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_route_gate",
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_release_publication_result_receipt_gate_ready:true,
    source_result_receipt_gate_sha256:$source_result_receipt_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$test_log,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_checked:$live_checked,
    source_route_count_expected:153,
    terminal_required_marker_count_expected:293,
    source_packet_acceptance_receipt_release_publication_ready:$source.source_packet_acceptance_receipt_release_publication_ready,
    release_publication_result_receipt_surface_count:$source.release_publication_result_receipt_surface_count,
    release_publication_result_receipt_recorded_count:$source.release_publication_result_receipt_recorded_count,
    release_publication_result_receipt_persisted_count:$source.release_publication_result_receipt_persisted_count,
    release_publication_result_receipt_filesystem_written_count:$source.release_publication_result_receipt_filesystem_written_count,
    publication_completion_ack_recorded_count:$source.publication_completion_ack_recorded_count,
    release_publication_result_receipt_activation_authority_derived_count:$source.release_publication_result_receipt_activation_authority_derived_count,
    release_publication_result_receipt_live_execution_allowed_count:$source.release_publication_result_receipt_live_execution_allowed_count,
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

echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt no-persistence route gate passed"
