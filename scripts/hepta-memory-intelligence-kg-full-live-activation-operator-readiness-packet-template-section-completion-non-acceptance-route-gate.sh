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
    echo "missing operator readiness packet template section-completion route source text: $label" >&2
    exit 1
  fi
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"
if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

SECTION_COMPLETION_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-section-completion-non-acceptance-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-section-completion-non-acceptance-gate.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_ready == true
  and .source_field_validation_ready == true
  and .source_operator_packet_section_count == 10
  and .source_operator_packet_required_field_count == 43
  and .source_required_field_count == 43
  and .source_missing_field_count == 43
  and .section_completion_matrix_count == 10
  and .section_completion_attempt_count == 10
  and .section_complete_count == 0
  and .section_ready_count == 0
  and .section_recorded_count == 0
  and .section_persisted_count == 0
  and .section_accepted_count == 0
  and .section_operator_approval_derived_count == 0
  and .section_activation_authority_derived_count == 0
  and .section_live_execution_allowed_count == 0
  and (.section_completion_matrix | all(
    .required_field_count > 0
    and .missing_field_count == .required_field_count
    and .present_field_count == 0
    and .recorded_field_count == 0
    and .accepted_field_count == 0
    and .authority_derived_field_count == 0
    and .live_execution_allowed_field_count == 0
    and .section_completion_attempted == true
    and .section_complete == false
    and .section_ready == false
    and .section_recorded == false
    and .section_persisted == false
    and .section_accepted == false
    and .section_operator_approval_derived == false
    and .section_activation_authority_derived == false
    and .section_live_execution_allowed == false
    and .completion_status == "completion_denied_missing_required_fields"
  ))
  and (.denied_by_section_completion | length) == 8
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .activation_authority_derived == false
  and .activation_command_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SECTION_COMPLETION_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/cli/src/native_gateway.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = 119;' \
  "native gateway route/source command count includes operator readiness packet template section-completion route"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_SECTION_COMPLETION_NON_ACCEPTANCE_ENDPOINT' \
  "native gateway operator readiness packet template section-completion endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-section-completion-non-acceptance' \
  "native gateway operator readiness packet template section-completion endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-section-completion-non-acceptance --json' \
  "native gateway operator readiness packet template section-completion source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_report' \
  "native gateway operator readiness packet template section-completion report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_route_enabled": true' \
  "operator readiness packet template section-completion route enabled"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"section_completion_matrix_count": section_completion_matrix_count' \
  "section completion matrix count emitted"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"section_ready_promoted": false' \
  "section ready promotion denied"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '"section_activation_authority_derived": false' \
  "section activation authority derivation denied"

TEST_LOG="$(mktemp /tmp/hepta-operator-readiness-packet-template-section-completion-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --lib \
  hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_endpoint_blocks_acceptance_authority \
  -- --nocapture >"$TEST_LOG"

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_ROUTE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-section-completion-non-acceptance"
  )"
  jq -e '
    .status == "ready"
    and .route_count == 119
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_route_enabled == true
    and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_ready == true
    and .source_field_validation_ready == true
    and .source_operator_packet_section_count == 10
    and .source_operator_packet_required_field_count == 43
    and .source_required_field_count == 43
    and .source_missing_field_count == 43
    and .section_completion_matrix_count == 10
    and .section_completion_attempt_count == 10
    and .section_complete_count == 0
    and .section_ready_count == 0
    and .section_recorded_count == 0
    and .section_persisted_count == 0
    and .section_accepted_count == 0
    and .section_operator_approval_derived_count == 0
    and .section_activation_authority_derived_count == 0
    and .section_live_execution_allowed_count == 0
    and (.section_completion_matrix | all(
      .required_field_count > 0
      and .missing_field_count == .required_field_count
      and .present_field_count == 0
      and .recorded_field_count == 0
      and .accepted_field_count == 0
      and .authority_derived_field_count == 0
      and .live_execution_allowed_field_count == 0
      and .section_completion_attempted == true
      and .section_complete == false
      and .section_ready == false
      and .section_recorded == false
      and .section_persisted == false
      and .section_accepted == false
      and .section_operator_approval_derived == false
      and .section_activation_authority_derived == false
      and .section_live_execution_allowed == false
      and .completion_status == "completion_denied_missing_required_fields"
    ))
    and (.denied_by_section_completion | length) == 8
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
  and .required_marker_count == 259
  and .present_required_marker_count == 259
  and .missing_required_marker_count == 0
  and .duplicate_required_marker_count == 0
  and .out_of_order_required_marker_count == 0
' >/dev/null <<<"$TERMINAL_COVERAGE_JSON"

native_gateway_sha256="$(sha256_file "$NATIVE_GATEWAY_SOURCE")"
source_section_completion_gate_sha256="$(printf '%s' "$SECTION_COMPLETION_JSON" | shasum -a 256 | awk '{print $1}')"
terminal_coverage_sha256="$(printf '%s' "$TERMINAL_COVERAGE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "ready" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_route_gate" \
  --arg endpoint "/api/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-section-completion-non-acceptance" \
  --arg source_command "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-section-completion-non-acceptance --json" \
  --arg source_section_completion_gate_sha256 "$source_section_completion_gate_sha256" \
  --arg native_gateway_sha256 "$native_gateway_sha256" \
  --arg test_log "$TEST_LOG" \
  --arg terminal_coverage_sha256 "$terminal_coverage_sha256" \
  --argjson source "$SECTION_COMPLETION_JSON" \
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
    activation_mode:"full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_native_route_status",
    source_section_completion_gate:$source.gate,
    source_section_completion_gate_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_ready,
    source_section_completion_gate_sha256:$source_section_completion_gate_sha256,
    source_route_wired:true,
    source_route_count_expected:119,
    native_gateway_source:"codex-rs/cli/src/native_gateway.rs",
    native_gateway_sha256:$native_gateway_sha256,
    native_gateway_unit_test_log:$test_log,
    live_endpoint_required:($live != null),
    live_endpoint_ready:(if $live == null then false else ($live.status == "ready") end),
    source_operator_packet_section_count:$source.source_operator_packet_section_count,
    source_operator_packet_required_field_count:$source.source_operator_packet_required_field_count,
    section_completion_matrix_count:$source.section_completion_matrix_count,
    section_completion_attempt_count:$source.section_completion_attempt_count,
    section_complete_count:0,
    section_ready_count:0,
    section_recorded_count:0,
    section_persisted_count:0,
    section_accepted_count:0,
    section_operator_approval_derived_count:0,
    section_activation_authority_derived_count:0,
    section_live_execution_allowed_count:0,
    operator_acceptance_recorded:false,
    operator_approval_recorded:false,
    activation_authority_derived:false,
    activation_command_derived:false,
    activation_allowed:false,
    activation_performed:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    live_kg_write_performed:false,
    provider_invoked:false,
    model_invoked:false,
    credential_read:false,
    install_executed:false,
    active_binary_mutated:false,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    terminal_required_marker_count:$terminal.required_marker_count,
    terminal_present_required_marker_count:$terminal.present_required_marker_count,
    terminal_missing_required_marker_count:$terminal.missing_required_marker_count,
    terminal_duplicate_required_marker_count:$terminal.duplicate_required_marker_count,
    terminal_out_of_order_required_marker_count:$terminal.out_of_order_required_marker_count,
    side_effects:{
      section_completion_recorded:false,
      section_completion_persisted:false,
      section_completion_materialized:false,
      section_completion_accepted:false,
      section_ready_promoted:false,
      section_operator_approval_derived:false,
      section_activation_authority_derived:false,
      section_live_execution_allowed:false,
      packet_template_recorded:false,
      packet_template_persisted:false,
      operator_acceptance_recorded:false,
      operator_approval_recorded:false,
      activation_authority_derived:false,
      activation_allowed:false,
      activation_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      live_kg_write_performed:false,
      provider_invoked:false,
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

echo "Hepta Memory/Intelligence/KG full live activation operator readiness packet template section completion non-acceptance route gate passed"
