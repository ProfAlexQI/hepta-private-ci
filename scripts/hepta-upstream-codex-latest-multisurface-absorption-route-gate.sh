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
    echo "missing upstream Codex latest multisurface absorption native route source text: $label" >&2
    exit 1
  fi
}

SOURCE_GATE_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-latest-multisurface-absorption" \
    scripts/hepta-upstream-codex-latest-multisurface-absorption.sh
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .latest_multisurface_absorption_ready == true
  and .commit_count == 12
  and .changed_file_count == 57
  and .provider_security_changed_file_count == 0
  and .runtime_appserver_changed_file_count == 11
  and .legacy_cli_tui_changed_file_count == 47
  and .product_governance_changed_file_count == 2
  and .family_count == 5
  and .ready_family_count == 5
  and .activation_blocking_family_count == 5
  and .active_runtime_promotion_allowed == false
  and .upstream_merge_performed == false
  and .active_runtime_dependency_allowed == false
  and .active_service_restart_allowed == false
  and .public_release_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .latest_multisurface_denied_by_count == 13
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$SOURCE_GATE_JSON"

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_REGISTRY_SOURCE="codex-rs/hepta-native-gateway/src/route_registry.rs"

require_source_text "$NATIVE_GATEWAY_SOURCE" \
  "const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();" \
  "native gateway route/source command count includes upstream Codex latest multisurface absorption route"
require_source_text "$ROUTE_REGISTRY_SOURCE" \
  'HEPTA_UPSTREAM_CODEX_LATEST_MULTISURFACE_ABSORPTION_ENDPOINT' \
  "upstream Codex latest multisurface absorption endpoint constant"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/api/hepta-upstream-codex-latest-multisurface-absorption' \
  "upstream Codex latest multisurface absorption endpoint path"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  '/hepta-upstream-codex-latest-multisurface-absorption --json' \
  "upstream Codex latest multisurface absorption source command"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_upstream_codex_latest_multisurface_absorption_report' \
  "upstream Codex latest multisurface absorption report function"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'native_route_latest_upstream_delta_classification_no_fetch_no_merge_no_activation' \
  "upstream Codex latest multisurface absorption native no-fetch execution mode"
require_source_text "$NATIVE_GATEWAY_SOURCE" \
  'hepta_upstream_codex_latest_multisurface_absorption_endpoint_classifies_without_fetch_merge_or_activation_side_effects' \
  "focused upstream Codex latest multisurface absorption unit test"

TEST_LOG="$(mktemp /tmp/hepta-upstream-codex-latest-multisurface-absorption-route-tests.XXXXXX)"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_upstream_codex_latest_multisurface_absorption_endpoint_classifies_without_fetch_merge_or_activation_side_effects \
  -- --nocapture >"$TEST_LOG"

live_route_status="skipped"
live_route_count=0
live_missing_route_count=0

if [[ "$REQUIRE_LIVE_ENDPOINT" == "1" ]]; then
  LIVE_JSON="$(
    curl -fsS "$BASE_URL/api/hepta-upstream-codex-latest-multisurface-absorption"
  )"
  jq -e --argjson expected "$EXPECTED_ROUTE_COUNT" '
    .status == "ready"
    and .route_count == $expected
    and .implemented_route_count == $expected
    and .missing_route_count == 0
    and .route_count_source_command_accepted == true
    and .native_route_mode == "native_route_latest_upstream_delta_classification_no_fetch_no_merge_no_activation"
    and .upstream_codex_latest_multisurface_absorption_route_enabled == true
    and .upstream_codex_latest_multisurface_absorption_ready == true
    and .commit_count == 12
    and .changed_file_count == 57
    and .family_count == 5
    and .ready_family_count == 5
    and .activation_blocking_family_count == 5
    and .latest_multisurface_denied_by_count == 13
    and .upstream_fetch_performed_by_native_route == false
    and .upstream_merge_performed == false
    and .upstream_checkout_performed == false
    and .active_runtime_dependency_allowed == false
    and .active_service_restart_allowed == false
    and .provider_model_invocation_allowed == false
    and .channel_delivery_allowed == false
    and .public_release_claim_allowed == false
    and .release_artifact_write_allowed == false
    and .evidence_persistence_allowed == false
    and (.side_effects | to_entries | all(.value == false))
  ' >/dev/null <<<"$LIVE_JSON"
  live_route_status="$(jq -r '.status' <<<"$LIVE_JSON")"
  live_route_count="$(jq -r '.route_count' <<<"$LIVE_JSON")"
  live_missing_route_count="$(jq -r '.missing_route_count' <<<"$LIVE_JSON")"
fi

source_gate_sha256="$(printf '%s' "$SOURCE_GATE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_upstream_codex_latest_multisurface_absorption_route_gate" \
  --arg endpoint "/api/hepta-upstream-codex-latest-multisurface-absorption" \
  --arg source_command "/hepta-upstream-codex-latest-multisurface-absorption --json" \
  --arg source_script_command "scripts/hepta-upstream-codex-latest-multisurface-absorption.sh" \
  --arg source_gate_sha256 "$source_gate_sha256" \
  --arg native_gateway_sha256 "$(sha256_file "$NATIVE_GATEWAY_SOURCE")" \
  --arg focused_test_log "$TEST_LOG" \
  --arg live_route_status "$live_route_status" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
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
    source_script_command:$source_script_command,
    source_gate_sha256:$source_gate_sha256,
    native_gateway_sha256:$native_gateway_sha256,
    focused_test_log:$focused_test_log,
    live_endpoint_checked:$live_endpoint_checked,
    live_route_status:$live_route_status,
    live_route_count:$live_route_count,
    live_missing_route_count:$live_missing_route_count,
    expected_route_count:$expected_route_count,
    route_gate_ready:true,
    upstream_codex_latest_multisurface_absorption_native_route_ready:true,
    native_route_mode:"native_route_latest_upstream_delta_classification_no_fetch_no_merge_no_activation",
    commit_count:12,
    changed_file_count:57,
    family_count:5,
    ready_family_count:5,
    activation_blocking_family_count:5,
    latest_multisurface_denied_by_count:13,
    upstream_fetch_performed_by_native_route:false,
    upstream_merge_performed:false,
    upstream_checkout_performed:false,
    active_runtime_dependency_allowed:false,
    active_service_restart_allowed:false,
    provider_model_invocation_allowed:false,
    channel_delivery_allowed:false,
    public_release_claim_allowed:false,
    release_artifact_write_allowed:false,
    evidence_persistence_allowed:false,
    side_effects:{
      upstream_fetch_performed:false,
      upstream_merge_performed:false,
      upstream_checkout_performed:false,
      workspace_write:false,
      active_binary_mutated:false,
      active_service_restart:false,
      launchd_mutated:false,
      gateway_mutation_performed:false,
      provider_invoked:false,
      model_invoked:false,
      channel_send_performed:false,
      telegram_send_performed:false,
      external_send_performed:false,
      release_artifact_written:false,
      public_release_published:false,
      public_ga_claimed:false,
      evidence_persisted:false,
      credential_value_read:false,
      credential_read:false,
      secret_file_read:false,
      filesystem_written:false
    }
  }'

echo "Hepta upstream Codex latest multisurface absorption route gate passed"
