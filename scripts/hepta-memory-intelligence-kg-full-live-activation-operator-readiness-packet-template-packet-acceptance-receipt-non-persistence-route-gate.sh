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
    echo "missing operator readiness packet template packet-acceptance receipt route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

SOURCE_RECEIPT_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-non-persistence-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-non-persistence-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_ready == true
  and .source_packet_assembly_ready == true
  and .source_packet_assembly_attempt_count == 4
  and .source_packet_assembled_count == 0
  and .source_packet_accepted_count == 0
  and .source_packet_activation_authority_derived_count == 0
  and .receipt_surface_count == 8
  and .receipt_generated_count == 8
  and .receipt_recorded_count == 0
  and .receipt_persisted_count == 0
  and .receipt_materialized_count == 0
  and .receipt_indexed_count == 0
  and .receipt_queryable_count == 0
  and .receipt_exportable_count == 0
  and .receipt_observable_count == 0
  and .receipt_delivered_count == 0
  and .receipt_acceptance_recorded_count == 0
  and .receipt_operator_approval_derived_count == 0
  and .receipt_activation_authority_derived_count == 0
  and .receipt_activation_command_derived_count == 0
  and .receipt_live_execution_allowed_count == 0
  and (.receipt_surfaces | all(
    .receipt_generated == true
    and .receipt_recorded == false
    and .receipt_persisted == false
    and .receipt_materialized == false
    and .receipt_indexed == false
    and .receipt_queryable == false
    and .receipt_exportable == false
    and .receipt_observable == false
    and .receipt_delivered == false
    and .receipt_acceptance_recorded == false
    and .receipt_operator_approval_derived == false
    and .receipt_activation_authority_derived == false
    and .receipt_activation_command_derived == false
    and .receipt_live_execution_allowed == false
    and .receipt_status == "non_persistent_report_only"
  ))
  and (.denied_by_packet_acceptance_receipt | length) == 10
  and .packet_acceptance_receipt_recorded == false
  and .packet_acceptance_receipt_persisted == false
  and .packet_acceptance_receipt_materialized == false
  and .packet_acceptance_receipt_indexed == false
  and .packet_acceptance_receipt_delivered == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .activation_authority_derived == false
  and .activation_command_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SOURCE_RECEIPT_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 157;' \
  "native gateway route/source command count includes operator readiness packet acceptance receipt route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_NON_PERSISTENCE_ENDPOINT' \
  "native gateway operator readiness packet acceptance receipt endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-non-persistence' \
  "native gateway operator readiness packet acceptance receipt endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-non-persistence --json' \
  "native gateway operator readiness packet acceptance receipt source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_report' \
  "native gateway operator readiness packet acceptance receipt report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_route_enabled": true' \
  "operator readiness packet acceptance receipt route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"receipt_surface_count": receipt_surface_count' \
  "packet acceptance receipt surface count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_persisted": false' \
  "packet acceptance receipt persistence denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_acceptance_receipt_authority_derived": false' \
  "packet acceptance receipt authority derivation denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-readiness-packet-template-packet-acceptance-receipt-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_endpoint_blocks_persistence \
  -- --nocapture >"$TEST_LOG"

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-non-persistence"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 157
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_ready == true
    and .source_packet_assembly_ready == true
    and .source_packet_assembly_attempt_count == 4
    and .source_packet_assembled_count == 0
    and .source_packet_accepted_count == 0
    and .source_packet_activation_authority_derived_count == 0
    and .receipt_surface_count == 8
    and .receipt_generated_count == 8
    and .receipt_recorded_count == 0
    and .receipt_persisted_count == 0
    and .receipt_materialized_count == 0
    and .receipt_indexed_count == 0
    and .receipt_queryable_count == 0
    and .receipt_exportable_count == 0
    and .receipt_observable_count == 0
    and .receipt_delivered_count == 0
    and .receipt_acceptance_recorded_count == 0
    and .receipt_operator_approval_derived_count == 0
    and .receipt_activation_authority_derived_count == 0
    and .receipt_activation_command_derived_count == 0
    and .receipt_live_execution_allowed_count == 0
    and (.receipt_surfaces | all(
      .receipt_generated == true
      and .receipt_recorded == false
      and .receipt_persisted == false
      and .receipt_materialized == false
      and .receipt_indexed == false
      and .receipt_queryable == false
      and .receipt_exportable == false
      and .receipt_observable == false
      and .receipt_delivered == false
      and .receipt_acceptance_recorded == false
      and .receipt_operator_approval_derived == false
      and .receipt_activation_authority_derived == false
      and .receipt_activation_command_derived == false
      and .receipt_live_execution_allowed == false
      and .receipt_status == "non_persistent_report_only"
    ))
    and (.denied_by_packet_acceptance_receipt | length) == 10
    and .packet_acceptance_receipt_recorded == false
    and .packet_acceptance_receipt_persisted == false
    and .packet_acceptance_receipt_materialized == false
    and .packet_acceptance_receipt_indexed == false
    and .packet_acceptance_receipt_delivered == false
    and .operator_acceptance_recorded == false
    and .operator_approval_recorded == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .activation_allowed == false
    and .activation_performed == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .public_release_claimed == false
    and .release_artifact_written == false
    and .external_send_performed == false
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_ROUTE_JSON"
else
  LIVE_ROUTE_JSON='null'
fi

TERMINAL_COVERAGE_JSON="$(
  capture_json_report \
    "hepta-preflight-terminal-coverage-inventory-gate" \
    scripts/hepta-preflight-terminal-coverage-inventory-gate.sh
)"
jq -e '
  .status == "ready"
  and .preflight_terminal_coverage_inventory_ready == true
  and .required_marker_count == 297
  and .present_required_marker_count == 297
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_receipt_gate_sha256="$(printf '%s' "$SOURCE_RECEIPT_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "ready" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-non-persistence" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-non-persistence --json" \
  --arg source_receipt_gate_sha256 "$source_receipt_gate_sha256" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$SOURCE_RECEIPT_JSON" \
  --argjson terminal "$TERMINAL_COVERAGE_JSON" \
  --argjson live "$LIVE_ROUTE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:$status,
    base_url:$base_url,
    gate:$gate,
    endpoint:$endpoint,
    source_command:$source_command,
    activation_mode:"full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_native_route_status",
    side_effect_free:true,
    native_route:true,
    route_enabled:true,
    source_packet_acceptance_receipt_gate_ready: $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_ready,
    source_receipt_gate_sha256:$source_receipt_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$test_log,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    live_endpoint_checked: ($live != null),
    source_route_count_expected:153,
    terminal_required_marker_count_expected:293,
    source_packet_assembly_ready: $source.source_packet_assembly_ready,
    source_packet_assembly_attempt_count: $source.source_packet_assembly_attempt_count,
    receipt_surface_count: $source.receipt_surface_count,
    receipt_generated_count: $source.receipt_generated_count,
    receipt_recorded_count: $source.receipt_recorded_count,
    receipt_persisted_count: $source.receipt_persisted_count,
    receipt_acceptance_recorded_count: $source.receipt_acceptance_recorded_count,
    receipt_activation_authority_derived_count: $source.receipt_activation_authority_derived_count,
    receipt_live_execution_allowed_count: $source.receipt_live_execution_allowed_count,
    route_source_texts_ready:true,
    terminal_coverage_ready: $terminal.preflight_terminal_coverage_inventory_ready,
    terminal_required_marker_count: $terminal.required_marker_count,
    terminal_present_required_marker_count: $terminal.present_required_marker_count,
    terminal_missing_required_marker_count: $terminal.missing_required_marker_count,
    side_effects:{
      route_gate_filesystem_written:false,
      route_gate_runtime_mutated:false,
      route_gate_service_restarted:false,
      route_gate_external_send_performed:false,
      source_gate_side_effects: $source.side_effects,
      live_route_side_effects: (if $live == null then null else $live.side_effects end)
    }
  }'

echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt non-persistence route gate passed"
