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
    echo "missing first-model explicit approval evidence no-invocation boundary source text: $label" >&2
    exit 1
  fi
}

SOURCE_ACTIVATION_GATE="scripts/hepta-activation-evidence-no-write-provider-router-dry-run-boundary-route-gate.sh"
SOURCE_APPROVAL_GATE="scripts/hepta-first-model-invocation-separate-approval-slice-preflight-route-gate.sh"
source "$REPO_ROOT/scripts/lib/hepta-source-set.sh"
NATIVE_GATEWAY_SOURCE="hepta-native-gateway-source-set-v1"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"
ENDPOINT="/api/hepta-first-model-invocation-explicit-approval-evidence-no-invocation-boundary"
SOURCE_COMMAND="/hepta-first-model-invocation-explicit-approval-evidence-no-invocation-boundary --json"

ACTIVATION_SOURCE_JSON="$(
  capture_json_report \
    "hepta-activation-evidence-no-write-provider-router-dry-run-boundary-route-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_EXPECTED_ROUTE_COUNT="$EXPECTED_ROUTE_COUNT" \
      "$SOURCE_ACTIVATION_GATE"
)"

jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
  .runtime == "hepta"
  and .status == "ready"
  and .expected_route_count == $expected
  and .activation_evidence_candidate_count == 8
  and .accepted_activation_evidence_candidate_count == 0
  and .denied_by_activation_evidence_no_write_provider_router_dry_run_boundary_count == 20
  and .side_effects.activation_evidence_recorded == false
  and .side_effects.activation_evidence_persisted == false
  and .side_effects.activation_evidence_materialized == false
  and .side_effects.fresh_long_soak_evidence_accepted == false
  and .side_effects.operator_approval_recorded == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.credential_read == false
  and .side_effects.live_kg_write_performed == false
  and .side_effects.memory_store_write_performed == false
  and .side_effects.external_send_performed == false
' >/dev/null <<<"$ACTIVATION_SOURCE_JSON"

APPROVAL_SOURCE_JSON="$(
  capture_json_report \
    "hepta-first-model-invocation-separate-approval-slice-preflight-route-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_EXPECTED_ROUTE_COUNT="$EXPECTED_ROUTE_COUNT" \
      "$SOURCE_APPROVAL_GATE"
)"

jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
  .runtime == "hepta"
  and .status == "ready"
  and .expected_route_count == $expected
  and .first_model_invocation_separate_approval_slice_preflight_ready == true
  and .approval_state == "requires_fresh_operator_approval_and_explicit_command"
  and .fresh_operator_approval_required == true
  and .explicit_command_required == true
  and .single_use_approval_nonce_required == true
  and .approval_packet_preview_constructed == true
  and .approval_packet_readback_audit_performed == true
  and .approval_packet_readback_hash_matched == true
  and .approval_packet_receipt_rendered == true
  and .approval_packet_accepted == false
  and .approval_packet_persisted == false
  and .candidate_provider_invocation_requested == true
  and .candidate_model_invocation_requested == true
  and .provider_invocation_authorized == false
  and .model_invocation_authorized == false
  and .provider_invocation_budget == 0
  and .model_invocation_budget == 0
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .live_kg_write_performed == false
  and .memory_store_write_performed == false
  and .external_send_performed == false
' >/dev/null <<<"$APPROVAL_SOURCE_JSON"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes explicit approval evidence no-invocation boundary"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  "HEPTA_FIRST_MODEL_INVOCATION_EXPLICIT_APPROVAL_EVIDENCE_NO_INVOCATION_BOUNDARY_ENDPOINT" \
  "explicit approval evidence no-invocation endpoint constant"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$ENDPOINT" \
  "explicit approval evidence no-invocation endpoint path"
require_source_text "$ROUTE_REGISTRY_SOURCE" "$SOURCE_COMMAND" \
  "explicit approval evidence no-invocation source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_first_model_invocation_explicit_approval_evidence_no_invocation_boundary_report" \
  "explicit approval evidence no-invocation report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"first_model_invocation_explicit_approval_evidence_no_invocation_boundary_route_enabled\"" \
  "explicit approval evidence no-invocation route enabled field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "\"denied_by_first_model_invocation_explicit_approval_evidence_no_invocation_boundary_count\"" \
  "explicit approval evidence no-invocation denial count field"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "hepta_first_model_invocation_explicit_approval_evidence_no_invocation_boundary_endpoint_blocks_approval_acceptance_and_invocation_side_effects" \
  "focused explicit approval evidence no-invocation unit test"

TEST_LOG="$(mktemp /tmp/hepta-first-model-explicit-approval-evidence-no-invocation-boundary-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_first_model_invocation_explicit_approval_evidence_no_invocation_boundary_endpoint_blocks_approval_acceptance_and_invocation_side_effects \
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
    and .first_model_invocation_explicit_approval_evidence_no_invocation_boundary_route_enabled == true
    and .first_model_invocation_explicit_approval_evidence_no_invocation_boundary_ready == true
    and .first_model_invocation_explicit_approval_evidence_no_invocation_boundary_status == "blocked_report_only"
    and .source_activation_evidence_no_write_provider_router_dry_run_boundary_ready == true
    and .source_first_model_invocation_separate_approval_slice_preflight_ready == true
    and .approval_evidence_candidate_count == 10
    and .accepted_approval_evidence_candidate_count == 0
    and (.approval_evidence_candidates | length) == 10
    and (.approval_evidence_candidates | all(.accepted == false))
    and .required_approval_evidence_field_count == 18
    and .recorded_approval_evidence_field_count == 0
    and .fresh_operator_approval_required == true
    and .explicit_invocation_command_required == true
    and .single_use_approval_nonce_required == true
    and .operator_identity_session_binding_required == true
    and .fresh_long_soak_evidence_required == true
    and .fresh_operator_approval_artifact_present == false
    and .fresh_operator_approval_artifact_verified == false
    and .operator_approval_artifact_accepted == false
    and .operator_approval_recorded == false
    and .single_use_approval_nonce_verified == false
    and .single_use_approval_nonce_consumed == false
    and .operator_identity_session_binding_verified == false
    and .operator_identity_session_bound == false
    and .explicit_invocation_command_accepted == false
    and .explicit_invocation_command_consumed == false
    and .fresh_long_soak_evidence_accepted == false
    and .explicit_approval_evidence_recorded == false
    and .explicit_approval_evidence_persisted == false
    and .explicit_approval_evidence_accepted == false
    and .explicit_approval_evidence_filesystem_written == false
    and .approval_authority_derived == false
    and .activation_authority_derived == false
    and .candidate_provider_invocation_requested == true
    and .candidate_model_invocation_requested == true
    and .provider_invocation_authorized == false
    and .model_invocation_authorized == false
    and .provider_invocation_budget == 0
    and .model_invocation_budget == 0
    and .provider_router_live_envelope_executed == false
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
    and .public_release_claimed == false
    and .public_ga_claimed == false
    and .denied_by_first_model_invocation_explicit_approval_evidence_no_invocation_boundary_count == 24
    and (.denied_by_first_model_invocation_explicit_approval_evidence_no_invocation_boundary | length) == 24
    and (.boundary_steps | length) == 5
    and .allowed_next_actions[0].action == "first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight"
    and .allowed_next_actions[0].uses_activation_evidence_no_write_provider_router_dry_run_boundary == true
    and .allowed_next_actions[0].requires_fresh_operator_approval == true
    and .allowed_next_actions[0].requires_explicit_command == true
    and .allowed_next_actions[0].invokes_provider == false
    and .allowed_next_actions[0].invokes_model == false
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
activation_source_gate_sha256="$(sha256_file "$SOURCE_ACTIVATION_GATE")"
approval_source_gate_sha256="$(sha256_file "$SOURCE_APPROVAL_GATE")"
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
  --arg activation_source_gate_sha256 "$activation_source_gate_sha256" \
  --arg approval_source_gate_sha256 "$approval_source_gate_sha256" \
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
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:"hepta_first_model_invocation_explicit_approval_evidence_no_invocation_boundary_route_gate",
    endpoint:$endpoint,
    source_command:$source_command,
    native_gateway_sha256:$native_gateway_sha256,
    activation_source_gate_sha256:$activation_source_gate_sha256,
    approval_source_gate_sha256:$approval_source_gate_sha256,
    focused_test_log:$test_log,
    live_endpoint_checked:$live_endpoint_checked,
    live_route_status:$live_route_status,
    live_route_count:$live_route_count,
    live_missing_route_count:$live_missing_route_count,
    expected_route_count:$expected_route_count,
    route_gate_ready:true,
    first_model_invocation_explicit_approval_evidence_no_invocation_boundary_ready:true,
    approval_evidence_candidate_count:10,
    accepted_approval_evidence_candidate_count:0,
    required_approval_evidence_field_count:18,
    recorded_approval_evidence_field_count:0,
    denied_by_first_model_invocation_explicit_approval_evidence_no_invocation_boundary_count:24,
    terminal_coverage_sha256:$terminal_coverage_sha256,
    terminal_required_marker_count:$terminal_required_marker_count,
    terminal_present_required_marker_count:$terminal_present_required_marker_count,
    terminal_missing_required_marker_count:$terminal_missing_required_marker_count,
    source_activation_evidence_no_write_provider_router_dry_run_boundary_ready:true,
    source_first_model_invocation_separate_approval_slice_preflight_ready:true,
    fresh_operator_approval_required:true,
    explicit_invocation_command_required:true,
    single_use_approval_nonce_required:true,
    operator_identity_session_binding_required:true,
    fresh_long_soak_evidence_required:true,
    explicit_approval_evidence_accepted:false,
    explicit_approval_evidence_persisted:false,
    fresh_operator_approval_artifact_verified:false,
    explicit_invocation_command_accepted:false,
    single_use_approval_nonce_consumed:false,
    provider_invocation_authorized:false,
    model_invocation_authorized:false,
    provider_invocation_budget:0,
    model_invocation_budget:0,
    provider_invoked:false,
    model_invoked:false,
    credential_read:false,
    live_kg_write_performed:false,
    memory_store_write_performed:false,
    channel_send_performed:false,
    external_send_performed:false,
    install_executed:false,
    service_restarted:false,
    active_binary_mutated:false,
    side_effects:{
      explicit_approval_evidence_recorded:false,
      explicit_approval_evidence_persisted:false,
      explicit_approval_evidence_accepted:false,
      operator_approval_recorded:false,
      single_use_approval_nonce_consumed:false,
      explicit_invocation_command_accepted:false,
      provider_invocation_authorized:false,
      model_invocation_authorized:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      live_kg_write_performed:false,
      memory_store_write_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      install_executed:false,
      service_restarted:false,
      active_binary_mutated:false
    }
  }'

echo "Hepta first model invocation explicit approval evidence no-invocation boundary route gate passed"
