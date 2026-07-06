#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

export HEPTA_AUTOLOAD=0
export HEPTA_AUTOSAVE=0
export CARGO_INCREMENTAL=0

MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_NATIVE_PACKAGING_GATE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_NATIVE_PACKAGING_GATE_ADDR:-}"
SERVER_LOG="${HEPTA_NATIVE_PACKAGING_GATE_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-native-packaging-gate-server.XXXXXX")}"
REPORT_PATH="${HEPTA_NATIVE_PACKAGING_GATE_REPORT_PATH:-}"
STARTUP_TIMEOUT_SEC="${HEPTA_NATIVE_PACKAGING_GATE_STARTUP_TIMEOUT_SEC:-900}"
APP_DIR="apps/hepta-native"
PACKAGING_DIR="$APP_DIR/packaging"
RESOURCE_DIR="$APP_DIR/resources"
PRODUCT_NAME="Hepta Native"
BUNDLE_BINARY_NAME="hepta-native"
BUNDLE_IDENTIFIER="ai.hepta.nativeapp"
BUNDLE_ICON_NAME="HeptaNative.icns"
BUNDLE_BUILD_PROFILE="${HEPTA_NATIVE_PACKAGING_BUNDLE_PROFILE:-debug}"
STAGE_UNSIGNED_APP_BUNDLE="${HEPTA_NATIVE_PACKAGING_STAGE_UNSIGNED_APP_BUNDLE:-1}"
UNSIGNED_APP_STAGE_DIR="${HEPTA_NATIVE_PACKAGING_UNSIGNED_APP_STAGE_DIR:-}"
runner_server_mode="local-loopback"
if [[ -n "${HEPTA_NATIVE_PACKAGING_CARGO_TARGET_DIR:-}" ]]; then
  SERVER_CARGO_TARGET_DIR="$HEPTA_NATIVE_PACKAGING_CARGO_TARGET_DIR"
elif [[ -n "${HEPTA_NATIVE_CARGO_TARGET_DIR:-}" ]]; then
  SERVER_CARGO_TARGET_DIR="${HEPTA_NATIVE_CARGO_TARGET_DIR%/}-packaging-server"
elif [[ -n "${HEPTA_UI_PRODUCT_READINESS_DIR:-}" ]]; then
  SERVER_CARGO_TARGET_DIR="$HEPTA_UI_PRODUCT_READINESS_DIR/cargo-target-packaging-server"
else
  SERVER_CARGO_TARGET_DIR="${TMPDIR:-/tmp}/hepta-native-packaging-gate-cargo-target"
fi
if [[ -n "${HEPTA_NATIVE_PACKAGING_BUNDLE_CARGO_TARGET_DIR:-}" ]]; then
  BUNDLE_CARGO_TARGET_DIR="$HEPTA_NATIVE_PACKAGING_BUNDLE_CARGO_TARGET_DIR"
elif [[ -n "${HEPTA_NATIVE_CARGO_TARGET_DIR:-}" ]]; then
  BUNDLE_CARGO_TARGET_DIR="$HEPTA_NATIVE_CARGO_TARGET_DIR"
elif [[ -n "${HEPTA_UI_PRODUCT_READINESS_DIR:-}" ]]; then
  BUNDLE_CARGO_TARGET_DIR="$HEPTA_UI_PRODUCT_READINESS_DIR/cargo-target/hepta-ui-native"
else
  BUNDLE_CARGO_TARGET_DIR="$APP_DIR/target"
fi

stage_unsigned_app_bundle() {
  if [[ "$STAGE_UNSIGNED_APP_BUNDLE" != "1" ]]; then
    jq -n \
      --arg status "skipped" \
      '{
        ready:false,
        status:$status,
        skipped:true,
        reason:"HEPTA_NATIVE_PACKAGING_STAGE_UNSIGNED_APP_BUNDLE disabled"
      }'
    return 0
  fi

  local stage_dir="$UNSIGNED_APP_STAGE_DIR"
  if [[ -z "$stage_dir" ]]; then
    if [[ -n "$REPORT_PATH" ]]; then
      stage_dir="$(dirname "$REPORT_PATH")/native-packaging-unsigned-app"
    else
      stage_dir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-native-packaging-unsigned-app.XXXXXX")"
    fi
  fi

  local profile_dir="$BUNDLE_BUILD_PROFILE"
  local cargo_build_args=(build --manifest-path "$APP_DIR/Cargo.toml" --bin "$BUNDLE_BINARY_NAME" -q)
  case "$BUNDLE_BUILD_PROFILE" in
    debug)
      profile_dir="debug"
      ;;
    release)
      profile_dir="release"
      cargo_build_args+=(--release)
      ;;
    *)
      cargo_build_args+=(--profile "$BUNDLE_BUILD_PROFILE")
      ;;
  esac

  mkdir -p "$BUNDLE_CARGO_TARGET_DIR"
  CARGO_TARGET_DIR="$BUNDLE_CARGO_TARGET_DIR" cargo "${cargo_build_args[@]}"

  local binary_path="$BUNDLE_CARGO_TARGET_DIR/$profile_dir/$BUNDLE_BINARY_NAME"
  if [[ ! -x "$binary_path" ]]; then
    echo "missing built Hepta Native binary for unsigned app bundle probe: $binary_path" >&2
    exit 1
  fi

  local app_bundle_dir="$stage_dir/${PRODUCT_NAME}.app"
  local contents_dir="$app_bundle_dir/Contents"
  local macos_dir="$contents_dir/MacOS"
  local resources_dir="$contents_dir/Resources"
  rm -rf "$app_bundle_dir"
  mkdir -p "$macos_dir" "$resources_dir"
  cp "$PACKAGING_DIR/Info.plist" "$contents_dir/Info.plist"
  cp "$PACKAGING_DIR/$BUNDLE_ICON_NAME" "$resources_dir/$BUNDLE_ICON_NAME"
  cp "$binary_path" "$macos_dir/$BUNDLE_BINARY_NAME"
  chmod 755 "$macos_dir/$BUNDLE_BINARY_NAME"

  plutil -lint "$contents_dir/Info.plist" >/dev/null

  local plist_identifier plist_executable plist_name plist_package_type plist_icon plist_minimum_system_version
  local plist_url_scheme_hepta plist_url_scheme_matrix
  plist_identifier="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$contents_dir/Info.plist")"
  plist_executable="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$contents_dir/Info.plist")"
  plist_name="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleName' "$contents_dir/Info.plist")"
  plist_package_type="$(/usr/libexec/PlistBuddy -c 'Print :CFBundlePackageType' "$contents_dir/Info.plist")"
  plist_icon="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIconFile' "$contents_dir/Info.plist")"
  plist_minimum_system_version="$(/usr/libexec/PlistBuddy -c 'Print :LSMinimumSystemVersion' "$contents_dir/Info.plist")"
  plist_url_scheme_hepta="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleURLTypes:0:CFBundleURLSchemes:0' "$contents_dir/Info.plist")"
  plist_url_scheme_matrix="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleURLTypes:0:CFBundleURLSchemes:1' "$contents_dir/Info.plist")"

  [[ "$plist_identifier" == "$BUNDLE_IDENTIFIER" ]] || {
    echo "unsigned app bundle probe found unexpected bundle id: $plist_identifier" >&2
    exit 1
  }
  [[ "$plist_executable" == "$BUNDLE_BINARY_NAME" ]] || {
    echo "unsigned app bundle probe found unexpected executable: $plist_executable" >&2
    exit 1
  }
  [[ "$plist_name" == "$PRODUCT_NAME" ]] || {
    echo "unsigned app bundle probe found unexpected bundle name: $plist_name" >&2
    exit 1
  }
  [[ "$plist_package_type" == "APPL" ]] || {
    echo "unsigned app bundle probe found unexpected package type: $plist_package_type" >&2
    exit 1
  }
  [[ "$plist_icon" == "$BUNDLE_ICON_NAME" ]] || {
    echo "unsigned app bundle probe found unexpected icon file: $plist_icon" >&2
    exit 1
  }
  [[ "$plist_url_scheme_hepta" == "hepta-native" && "$plist_url_scheme_matrix" == "matrix" ]] || {
    echo "unsigned app bundle probe found unexpected URL schemes: $plist_url_scheme_hepta, $plist_url_scheme_matrix" >&2
    exit 1
  }

  local binary_file_output binary_sha256 info_sha256 icon_sha256 file_count bundle_bytes codesign_status
  binary_file_output="$(file -b "$macos_dir/$BUNDLE_BINARY_NAME")"
  case "$binary_file_output" in
    Mach-O\ *executable*)
      ;;
    *)
      echo "unsigned app bundle probe binary is not a Mach-O executable: $binary_file_output" >&2
      exit 1
      ;;
  esac
  binary_sha256="$(shasum -a 256 "$macos_dir/$BUNDLE_BINARY_NAME" | awk '{print $1}')"
  info_sha256="$(shasum -a 256 "$contents_dir/Info.plist" | awk '{print $1}')"
  icon_sha256="$(shasum -a 256 "$resources_dir/$BUNDLE_ICON_NAME" | awk '{print $1}')"
  file_count="$(find "$app_bundle_dir" -type f | wc -l | tr -d '[:space:]')"
  bundle_bytes="$(find "$app_bundle_dir" -type f -exec stat -f '%z' {} \; | awk '{sum += $1} END {print sum + 0}')"
  if codesign --verify --deep --strict "$app_bundle_dir" >/dev/null 2>&1; then
    codesign_status="signed_unexpected"
  else
    codesign_status="unsigned_expected"
  fi

  jq -n \
    --arg status "ready" \
    --arg stage_dir "$stage_dir" \
    --arg app_bundle_path "$app_bundle_dir" \
    --arg cargo_target_dir "$BUNDLE_CARGO_TARGET_DIR" \
    --arg build_profile "$BUNDLE_BUILD_PROFILE" \
    --arg source_binary_path "$binary_path" \
    --arg binary_file_output "$binary_file_output" \
    --arg bundle_identifier "$plist_identifier" \
    --arg bundle_executable "$plist_executable" \
    --arg bundle_name "$plist_name" \
    --arg bundle_package_type "$plist_package_type" \
    --arg bundle_icon_file "$plist_icon" \
    --arg minimum_system_version "$plist_minimum_system_version" \
    --arg url_scheme_hepta "$plist_url_scheme_hepta" \
    --arg url_scheme_matrix "$plist_url_scheme_matrix" \
    --arg binary_sha256 "$binary_sha256" \
    --arg info_sha256 "$info_sha256" \
    --arg icon_sha256 "$icon_sha256" \
    --arg codesign_status "$codesign_status" \
    --argjson file_count "$file_count" \
    --argjson bundle_bytes "$bundle_bytes" \
    '{
      ready:true,
      status:$status,
      skipped:false,
      stage_dir:$stage_dir,
      app_bundle_path:$app_bundle_path,
      cargo_target_dir:$cargo_target_dir,
      build_profile:$build_profile,
      source_binary_path:$source_binary_path,
      binary_file_output:$binary_file_output,
      file_count:$file_count,
      bundle_bytes:$bundle_bytes,
      bundle_identifier:$bundle_identifier,
      bundle_executable:$bundle_executable,
      bundle_name:$bundle_name,
      bundle_package_type:$bundle_package_type,
      bundle_icon_file:$bundle_icon_file,
      minimum_system_version:$minimum_system_version,
      url_schemes:[$url_scheme_hepta, $url_scheme_matrix],
      binary_sha256:$binary_sha256,
      info_sha256:$info_sha256,
      icon_sha256:$icon_sha256,
      codesign_status:$codesign_status,
      info_plist_ready:true,
      icon_ready:true,
      mach_o_binary_ready:true,
      distribution_signed:false,
      distribution_notarized:false,
      distribution_stapled:false,
      public_distribution_artifact_written:false,
      local_filesystem_written:true
    }'
}

if [[ -n "${HEPTA_LIVE_URL:-}" ]]; then
  BASE_URL="$HEPTA_LIVE_URL"
  runner_server_mode="provided-live-url"
else
  if [[ -z "$BIND_ADDR" ]]; then
    for port in 7380 7381 7382 7383 7384; do
      if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
        BIND_ADDR="${HOST}:${port}"
        break
      fi
    done
  fi

  if [[ -z "$BIND_ADDR" ]]; then
    echo "no free local port found for Hepta Native packaging gate" >&2
    exit 1
  fi

  BASE_URL="http://${BIND_ADDR}"

  mkdir -p "$SERVER_CARGO_TARGET_DIR"
  CARGO_TARGET_DIR="$SERVER_CARGO_TARGET_DIR" cargo build --manifest-path "$MANIFEST" -q -p codex-cli --bin hepta
  SERVER_BINARY_PATH="$SERVER_CARGO_TARGET_DIR/debug/hepta"
  if [[ ! -x "$SERVER_BINARY_PATH" ]]; then
    echo "missing built Hepta serve-ui binary for native packaging gate: $SERVER_BINARY_PATH" >&2
    exit 1
  fi
  "$SERVER_BINARY_PATH" --serve-ui "$BIND_ADDR" >"$SERVER_LOG" 2>&1 &
  server_pid="$!"

  cleanup() {
    if kill -0 "$server_pid" 2>/dev/null; then
      kill "$server_pid" 2>/dev/null || true
      wait "$server_pid" 2>/dev/null || true
    fi
  }
  trap cleanup EXIT

  deadline=$((SECONDS + STARTUP_TIMEOUT_SEC))
  until curl -fsS "$BASE_URL/health" >/dev/null 2>&1; do
    if ! kill -0 "$server_pid" 2>/dev/null; then
      echo "Hepta Native packaging gate server exited before readiness checks" >&2
      echo "server binary: ${SERVER_BINARY_PATH:-not_built}" >&2
      tail -n 80 "$SERVER_LOG" >&2 || true
      exit 1
    fi
    if [[ "$SECONDS" -ge "$deadline" ]]; then
      echo "timed out waiting for Hepta Native packaging gate server at $BASE_URL" >&2
      echo "server binary: ${SERVER_BINARY_PATH:-not_built}" >&2
      tail -n 80 "$SERVER_LOG" >&2 || true
      exit 1
    fi
    sleep 1
  done
fi

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
  and .rust_source_file_count >= 125
  and .packaging_resource_file_count >= 111
  and .rust_source_file_count_policy == "minimum_floor_from_reviewed_manifest"
  and .packaging_resource_file_count_policy == "minimum_floor_from_reviewed_manifest"
  and .ui_iteration_file_count_flexible == true
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

while IFS= read -r required; do
  [[ -f "$required" ]] || {
    echo "missing required native packaging file: $required" >&2
    exit 1
  }
done < <(jq -r '.required_metadata_files[]' <<<"$GATE_JSON")

rust_count="$(find "$APP_DIR/src" -type f -name '*.rs' | wc -l | tr -d '[:space:]')"
resource_count="$(find "$PACKAGING_DIR" "$RESOURCE_DIR" -type f | wc -l | tr -d '[:space:]')"
rust_count_floor="$(jq -r '.rust_source_file_count' <<<"$GATE_JSON")"
resource_count_floor="$(jq -r '.packaging_resource_file_count' <<<"$GATE_JSON")"
if (( rust_count < rust_count_floor )); then
  echo "Hepta Native Rust source count fell below manifest floor: $rust_count < $rust_count_floor" >&2
  exit 1
fi
if (( resource_count < resource_count_floor )); then
  echo "Hepta Native packaging/resource count fell below manifest floor: $resource_count < $resource_count_floor" >&2
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

unsigned_app_bundle_json="$(stage_unsigned_app_bundle)"
jq -e '
  .ready == true
  and .status == "ready"
  and .skipped == false
  and .info_plist_ready == true
  and .icon_ready == true
  and .mach_o_binary_ready == true
  and .bundle_identifier == "ai.hepta.nativeapp"
  and .bundle_executable == "hepta-native"
  and .bundle_name == "Hepta Native"
  and .bundle_package_type == "APPL"
  and (.url_schemes | index("hepta-native") != null)
  and (.url_schemes | index("matrix") != null)
  and (.binary_sha256 | test("^[0-9a-f]{64}$"))
  and (.info_sha256 | test("^[0-9a-f]{64}$"))
  and (.icon_sha256 | test("^[0-9a-f]{64}$"))
  and .file_count >= 3
  and .bundle_bytes > 1000000
  and .distribution_signed == false
  and .distribution_notarized == false
  and .distribution_stapled == false
  and .public_distribution_artifact_written == false
  and .local_filesystem_written == true
' <<<"$unsigned_app_bundle_json" >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg runner_server_mode "$runner_server_mode" \
  --arg server_log "$SERVER_LOG" \
  --arg server_cargo_target_dir "$SERVER_CARGO_TARGET_DIR" \
  --argjson startup_timeout_sec "$STARTUP_TIMEOUT_SEC" \
  --argjson gate "$GATE_JSON" \
  --argjson ga "$GA_JSON" \
  --argjson merge "$MERGE_JSON" \
  --argjson unsigned_app_bundle "$unsigned_app_bundle_json" \
  --argjson rust_count "$rust_count" \
  --argjson resource_count "$resource_count" \
  --argjson rust_count_floor "$rust_count_floor" \
  --argjson resource_count_floor "$resource_count_floor" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    packaging_evidence_mode:"metadata_plus_local_unsigned_app_bundle_probe",
    base_url:$base_url,
    runner:{
      server_mode:$runner_server_mode,
      server_log:$server_log,
      server_cargo_target_dir:$server_cargo_target_dir,
      server_binary_path:($server_cargo_target_dir + "/debug/hepta"),
      bundle_cargo_target_dir:$unsigned_app_bundle.cargo_target_dir,
      startup_timeout_sec:$startup_timeout_sec,
      local_loopback_spawned:($runner_server_mode == "local-loopback"),
      provided_live_url:($runner_server_mode == "provided-live-url"),
      local_unsigned_app_bundle_probe_created:$unsigned_app_bundle.ready
    },
    endpoint:"/api/hepta-native-packaging-gate",
    current_hepta_codex_script_total:$gate.current_hepta_codex_script_total,
    native_gateway_source_command_count:$gate.native_gateway_source_command_count,
    route_count:$gate.route_count,
    missing_route_count:$gate.missing_route_count,
    rust_source_file_count:$rust_count,
    packaging_resource_file_count:$resource_count,
    rust_source_file_count_floor:$rust_count_floor,
    packaging_resource_file_count_floor:$resource_count_floor,
    rust_source_file_count_policy:$gate.rust_source_file_count_policy,
    packaging_resource_file_count_policy:$gate.packaging_resource_file_count_policy,
    ui_iteration_file_count_flexible:$gate.ui_iteration_file_count_flexible,
    rust_source_file_count_delta:($rust_count - $rust_count_floor),
    packaging_resource_file_count_delta:($resource_count - $resource_count_floor),
    local_packaging_gate_ready:$gate.local_packaging_gate_ready,
    local_unsigned_app_bundle_probe_ready:$unsigned_app_bundle.ready,
    local_unsigned_app_bundle:$unsigned_app_bundle,
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
    script_side_effects:{
      local_loopback_server_spawned:($runner_server_mode == "local-loopback"),
      local_unsigned_app_bundle_written:$unsigned_app_bundle.local_filesystem_written,
      public_distribution_artifact_written:false,
      distribution_signed:false,
      distribution_notarized:false,
      distribution_stapled:false,
      credential_read:false,
      external_network_read:false,
      provider_invoked:false,
      channel_send_performed:false,
      gateway_mutation_performed:false
    },
    side_effects:$gate.side_effects
  }')"

printf '%s\n' "$report"

if [[ -n "$REPORT_PATH" ]]; then
  mkdir -p "$(dirname "$REPORT_PATH")"
  printf '%s\n' "$report" >"$REPORT_PATH"
fi

if [[ "$(jq -r '.reports_synchronized' <<<"$report")" != "true" ]]; then
  echo "Hepta Native packaging, public GA, and merge-completion reports are out of sync" >&2
  exit 1
fi

if [[ "$(jq -r '.local_unsigned_app_bundle_probe_ready' <<<"$report")" != "true" ]]; then
  echo "Hepta Native local unsigned app bundle probe did not pass" >&2
  exit 1
fi

echo "Hepta native packaging gate passed with local unsigned app bundle probe"
