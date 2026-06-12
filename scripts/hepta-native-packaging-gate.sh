#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
APP_DIR="apps/hepta-native"
PACKAGING_DIR="$APP_DIR/packaging"
RESOURCE_DIR="$APP_DIR/resources"

GATE_JSON="$(curl -fsS "$BASE_URL/api/hepta-native-packaging-gate")"
GA_JSON="$(curl -fsS "$BASE_URL/api/hepta-public-ga-readiness")"
MERGE_JSON="$(curl -fsS "$BASE_URL/api/hepta-merge-completion")"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .compatibility_mode == "native_app_packaging_readiness_gate"
  and .side_effect_free == true
  and .current_hepta_codex_script_total >= 17
  and .native_gateway_source_command_count >= 69
  and .missing_route_count == 0
  and .rust_source_file_count == 125
  and .packaging_resource_file_count == 111
  and .required_metadata_file_count == 9
  and .cargo_metadata_gate_ready == true
  and .package_metadata_ready == true
  and .icon_resource_matrix_ready == true
  and .dmg_helper_script_ready == true
  and .android_resource_matrix_ready == true
  and .ios_icon_matrix_ready == true
  and .local_bridge_fixture_smoke_ready == true
  and .local_native_test_gate_ready == true
  and .signing_notarization_deferred == true
  and .public_distribution_artifact_written == false
  and .local_packaging_gate_ready == true
  and .side_effects.process_spawned == false
  and .side_effects.filesystem_read == false
  and .side_effects.filesystem_written == false
  and .side_effects.release_artifact_written == false
  and .side_effects.app_signed == false
  and .side_effects.app_notarized == false
  and .side_effects.app_stapled == false
  and .side_effects.credential_read == false
  and .side_effects.external_network_read == false
  and .side_effects.external_send_performed == false
  and .side_effects.provider_invoked == false
  and .side_effects.model_invoked == false
  and .side_effects.channel_read_performed == false
  and .side_effects.channel_send_performed == false
  and .side_effects.telegram_owner_handoff_performed == false
  and .side_effects.native_post_mutation_performed == false
  and .side_effects.gateway_mutation_performed == false
' <<<"$GATE_JSON" >/dev/null

for required in \
  "$APP_DIR/Cargo.toml" \
  "$APP_DIR/Cargo.lock" \
  "$APP_DIR/README.md" \
  "$APP_DIR/LICENSE-MIT" \
  "$APP_DIR/License Attributions.md" \
  "$PACKAGING_DIR/Info.plist" \
  "$PACKAGING_DIR/Entitlements.plist" \
  "$PACKAGING_DIR/HeptaNative.icns" \
  "$PACKAGING_DIR/build-macos-dmg.sh"; do
  [[ -f "$required" ]] || {
    echo "missing required native packaging file: $required" >&2
    exit 1
  }
done

rust_count="$(find "$APP_DIR/src" -type f -name '*.rs' | wc -l | tr -d '[:space:]')"
resource_count="$(find "$PACKAGING_DIR" "$RESOURCE_DIR" -type f | wc -l | tr -d '[:space:]')"
if [[ "$rust_count" != "125" ]]; then
  echo "unexpected Hepta Native Rust source count: $rust_count" >&2
  exit 1
fi
if [[ "$resource_count" != "111" ]]; then
  echo "unexpected Hepta Native packaging/resource file count: $resource_count" >&2
  exit 1
fi

cargo metadata --manifest-path "$APP_DIR/Cargo.toml" --no-deps --format-version 1 >/dev/null
bash -n "$PACKAGING_DIR/build-macos-dmg.sh"
bash -n "$PACKAGING_DIR/fix-dmg-applications-icon.sh"
plutil -lint "$PACKAGING_DIR/Info.plist" "$PACKAGING_DIR/Entitlements.plist" >/dev/null

if [[ "${HEPTA_NATIVE_PACKAGING_RUN_CARGO:-0}" == "1" ]]; then
  target_dir="${HEPTA_NATIVE_TARGET_DIR:-apps/hepta-native/target}"
  CARGO_TARGET_DIR="$target_dir" cargo check --manifest-path "$APP_DIR/Cargo.toml"
  CARGO_TARGET_DIR="$target_dir" cargo test --manifest-path "$APP_DIR/Cargo.toml" hepta_ -- --nocapture
fi

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --argjson gate "$GATE_JSON" \
  --argjson ga "$GA_JSON" \
  --argjson merge "$MERGE_JSON" \
  --argjson rust_count "$rust_count" \
  --argjson resource_count "$resource_count" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    endpoint:"/api/hepta-native-packaging-gate",
    current_hepta_codex_script_total:$gate.current_hepta_codex_script_total,
    native_gateway_source_command_count:$gate.native_gateway_source_command_count,
    route_count:$gate.route_count,
    missing_route_count:$gate.missing_route_count,
    rust_source_file_count:$rust_count,
    packaging_resource_file_count:$resource_count,
    local_packaging_gate_ready:$gate.local_packaging_gate_ready,
    signing_notarization_deferred:$gate.signing_notarization_deferred,
    public_distribution_artifact_written:$gate.public_distribution_artifact_written,
    public_ga_ready:$ga.public_ga_ready,
    hepta_native_release_packaging_ready:$ga.hepta_native_release_packaging_ready,
    reports_synchronized: (
      $gate.current_hepta_codex_script_total == $ga.current_hepta_codex_script_total
      and $gate.native_gateway_source_command_count == $ga.native_gateway_source_command_count
      and $gate.current_hepta_codex_script_total == $merge.current_hepta_codex_script_total
      and $gate.native_gateway_source_command_count == $merge.native_gateway_source_command_count
      and $gate.missing_route_count == $ga.missing_route_count
      and $gate.missing_route_count == $merge.missing_route_count
    ),
    side_effects:$gate.side_effects
  }')"

printf '%s\n' "$report"

if [[ "$(jq -r '.reports_synchronized' <<<"$report")" != "true" ]]; then
  echo "Hepta Native packaging, public GA, and merge-completion reports are out of sync" >&2
  exit 1
fi

echo "Hepta native packaging gate passed"
