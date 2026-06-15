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
    echo "missing full live activation operator readiness packet template route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

PACKET_TEMPLATE_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_gate"
  and .operator_readiness_packet_template_schema_version == "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_v1"
  and .operator_readiness_packet_template_mode == "report_only_template_no_acceptance_no_activation"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_ready == true
  and .source_readiness_index_replay_idempotency_denial_ready == true
  and .source_full_live_activation_enabled == false
  and .source_full_live_activation_status == "blocked_report_only"
  and .source_replay_allowed == false
  and .source_activation_authority_derived == false
  and .required_operator_packet_section_count == 10
  and .operator_packet_section_count == 10
  and .missing_operator_packet_section_count == 10
  and .accepted_operator_packet_section_count == 0
  and .recorded_operator_packet_section_count == 0
  and .operator_packet_required_field_count == 43
  and .operator_packet_recorded_field_count == 0
  and .operator_packet_accepted_field_count == 0
  and (.operator_packet_sections | length) == 10
  and (.operator_packet_sections | all(
    .status == "missing"
    and .operator_input_required == true
    and .template_only == true
    and .report_only == true
    and .recorded == false
    and .persisted == false
    and .materialized == false
    and .accepted == false
    and .delivered == false
    and .activation_authority == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .attaches_intelligence_context == false
    and .invokes_provider == false
    and .reads_credentials == false
    and .installs_or_restarts == false
    and .publishes_artifacts == false
    and .sends_external == false
  ))
  and .packet_template_recorded == false
  and .packet_template_persisted == false
  and .packet_template_materialized == false
  and .packet_template_delivered == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .activation_authority_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and (.allowed_next_actions | all(.status == "allowed_report_only"))
  and (.denied_by_operator_readiness_packet_template | length) == 12
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .hepta_intelligence_context_attached == false
  and .prompt_preview_rendered == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .external_kg_adapter_read_performed == false
  and .external_adapter_client_constructed == false
  and .network_call_performed == false
  and .external_db_write_performed == false
  and .live_kg_write_performed == false
  and .credential_read == false
  and .secret_file_read == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and .public_artifact_written == false
  and .external_send_performed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$PACKET_TEMPLATE_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 127;' \
  "native gateway route/source command count includes operator readiness packet template route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_ENDPOINT' \
  "native gateway operator readiness packet template endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template' \
  "native gateway operator readiness packet template endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template --json' \
  "native gateway operator readiness packet template source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_report' \
  "native gateway operator readiness packet template report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_route_enabled": true' \
  "operator readiness packet template route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_packet_sections"' \
  "operator packet sections exposed"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"packet_template_persisted": false' \
  "packet template persistence denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"operator_acceptance_recorded": false' \
  "operator acceptance recording denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"activation_authority_derived": false' \
  "activation authority derivation denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-readiness-packet-template-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_endpoint_blocks_authority \
  -- --nocapture >"$TEST_LOG"

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 127
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_ready == true
    and .source_readiness_index_replay_idempotency_denial_ready == true
    and .source_full_live_activation_enabled == false
    and .source_full_live_activation_status == "blocked_report_only"
    and .source_replay_allowed == false
    and .source_activation_authority_derived == false
    and .required_operator_packet_section_count == 10
    and .operator_packet_section_count == 10
    and .missing_operator_packet_section_count == 10
    and .accepted_operator_packet_section_count == 0
    and .recorded_operator_packet_section_count == 0
    and .operator_packet_required_field_count == 43
    and .operator_packet_recorded_field_count == 0
    and .operator_packet_accepted_field_count == 0
    and (.operator_packet_sections | length) == 10
    and .packet_template_recorded == false
    and .packet_template_persisted == false
    and .packet_template_materialized == false
    and .packet_template_delivered == false
    and .operator_acceptance_recorded == false
    and .operator_approval_recorded == false
    and .activation_authority_derived == false
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
  and .required_marker_count == 267
  and .present_required_marker_count == 267
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_packet_template_gate_sha256="$(printf '%s' "$PACKET_TEMPLATE_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "ready" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template --json" \
  --arg source_packet_template_gate_sha256 "$source_packet_template_gate_sha256" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$PACKET_TEMPLATE_JSON" \
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
    activation_mode:"full_live_activation_operator_readiness_packet_template_native_route_status",
    source_packet_template_gate:$source.gate,
    source_packet_template_gate_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_ready,
    source_packet_template_gate_sha256:$source_packet_template_gate_sha256,
    source_route_wired:true,
    source_route_count_expected:127,
    native_gateway_source:"codex-rs/cli/src/native_gateway.rs",
    native_gateway_sha256:$native_gateway_sha256,
    native_gateway_unit_test_log:$test_log,
    live_endpoint_required:($live != null),
    live_endpoint_ready:(if $live == null then false else ($live.status == "ready") end),
    operator_packet_section_count:$source.operator_packet_section_count,
    missing_operator_packet_section_count:$source.missing_operator_packet_section_count,
    accepted_operator_packet_section_count:$source.accepted_operator_packet_section_count,
    recorded_operator_packet_section_count:$source.recorded_operator_packet_section_count,
    operator_packet_required_field_count:$source.operator_packet_required_field_count,
    operator_packet_recorded_field_count:$source.operator_packet_recorded_field_count,
    operator_packet_accepted_field_count:$source.operator_packet_accepted_field_count,
    packet_template_recorded:false,
    packet_template_persisted:false,
    packet_template_materialized:false,
    packet_template_delivered:false,
    operator_acceptance_recorded:false,
    operator_approval_recorded:false,
    activation_authority_derived:false,
    activation_allowed:false,
    activation_performed:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    live_kg_write_performed:false,
    provider_invoked:false,
    model_invoked:false,
    credential_read:false,
    install_executed:false,
    service_restarted:false,
    active_binary_mutated:false,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    terminal_required_marker_count:$terminal.required_marker_count,
    terminal_present_required_marker_count:$terminal.present_required_marker_count,
    terminal_missing_required_marker_count:$terminal.missing_required_marker_count,
    terminal_duplicate_required_marker_count:$terminal.duplicate_required_marker_count,
    terminal_out_of_order_required_marker_count:$terminal.out_of_order_required_marker_count,
    side_effects:{
      packet_template_recorded:false,
      packet_template_persisted:false,
      packet_template_materialized:false,
      packet_template_delivered:false,
      operator_acceptance_recorded:false,
      operator_approval_recorded:false,
      activation_authority_derived:false,
      activation_allowed:false,
      activation_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      public_release_claimed:false,
      release_artifact_written:false,
      external_send_performed:false,
      filesystem_written:false
    }
  }'

echo "Hepta Memory/Intelligence/KG full live activation operator readiness packet template route gate passed"
