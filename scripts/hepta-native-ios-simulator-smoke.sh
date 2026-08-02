#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
APP_DIR="$ROOT_DIR/apps/hepta-native"
PRODUCER="scripts/hepta-native-ios-simulator-smoke.sh"
BUNDLE_IDENTIFIER="ai.hepta.nativeapp"
PRODUCT_NAME="Hepta"
CARGO_PACKAGE="hepta-native"

DEVICE=""
REPORT_PATH=""
SCREENSHOT_PATH=""
CONTRACT_ONLY=false

usage() {
  cat <<'EOF'
usage: scripts/hepta-native-ios-simulator-smoke.sh \
  --device booted|SIMULATOR_UDID \
  --output /absolute/path/receipt.json \
  --screenshot /absolute/path/screenshot.png

Builds the current clean Hepta Native HEAD for an already-booted iOS
simulator with the repository-pinned cargo-makepad wrapper, installs and
launches it with simctl, captures a screenshot, and emits a current-source-
bound schema-v1 receipt. The script does not download runtimes, create a
simulator or account, sign an app, contact a real device, or publish anything.

  --contract-only   print the side-effect-free source contract and exit
  --help, -h        show this help
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --device) DEVICE="${2:-}"; shift 2 ;;
    --output) REPORT_PATH="${2:-}"; shift 2 ;;
    --screenshot) SCREENSHOT_PATH="${2:-}"; shift 2 ;;
    --contract-only) CONTRACT_ONLY=true; shift ;;
    --help|-h) usage; exit 0 ;;
    *) echo "error: unknown argument: $1" >&2; usage >&2; exit 64 ;;
  esac
done

if [[ "$CONTRACT_ONLY" == true ]]; then
  jq -n \
    --arg producer "$PRODUCER" \
    --arg bundle_identifier "$BUNDLE_IDENTIFIER" \
    --arg product_name "$PRODUCT_NAME" \
    --arg executable "$CARGO_PACKAGE" '
      {
        schema_version:1,
        kind:"hepta-native-ios-simulator-smoke-source-contract",
        status:"ready",
        producer:$producer,
        build_wrapper:"scripts/hepta-native-mobile-cargo",
        receipt_kind:"hepta-native-ios-simulator-smoke-receipt",
        identity:{bundle_identifier:$bundle_identifier,display_name:$product_name,name:$product_name,executable:$executable},
        requirements:{clean_committed_source:true,already_booted_simulator:true,current_head_embedded:true,stale_bundle_removed:true,compiled_asset_catalog_ready:true,artifact_sha256:true,screenshot_sha256:true},
        forbidden_actions:{runtime_download:false,simulator_create:false,account_create:false,code_sign:false,real_device_contact:false,upload:false},
        external_side_effects_performed:false
      }
    '
  exit 0
fi

[[ -n "$DEVICE" ]] || { echo "error: --device is required" >&2; exit 64; }
[[ -n "$REPORT_PATH" ]] || { echo "error: --output is required" >&2; exit 64; }
[[ -n "$SCREENSHOT_PATH" ]] || { echo "error: --screenshot is required" >&2; exit 64; }

for command in git jq rustup shasum strings xcrun plutil ditto sips ruby; do
  command -v "$command" >/dev/null 2>&1 || { echo "error: $command is required" >&2; exit 2; }
done
[[ -x /usr/libexec/PlistBuddy ]] || { echo "error: PlistBuddy is required" >&2; exit 2; }

external_path() {
  local requested="$1" parent base resolved
  [[ "$requested" = /* ]] || { echo "error: evidence paths must be absolute: $requested" >&2; return 1; }
  parent="$(dirname "$requested")"
  base="$(basename "$requested")"
  mkdir -p "$parent"
  parent="$(cd "$parent" && pwd -P)"
  resolved="$parent/$base"
  case "$resolved" in
    "$ROOT_DIR"|"$ROOT_DIR"/*)
      echo "error: runtime evidence must be written outside the source repository: $resolved" >&2
      return 1
      ;;
  esac
  printf '%s\n' "$resolved"
}

REPORT_PATH="$(external_path "$REPORT_PATH")"
SCREENSHOT_PATH="$(external_path "$SCREENSHOT_PATH")"
if [[ "$REPORT_PATH" == *.json ]]; then
  ARTIFACT_PATH="${REPORT_PATH%.json}.app.zip"
else
  ARTIFACT_PATH="$REPORT_PATH.app.zip"
fi
ARTIFACT_PATH="$(external_path "$ARTIFACT_PATH")"
[[ "$REPORT_PATH" != "$SCREENSHOT_PATH" && "$REPORT_PATH" != "$ARTIFACT_PATH" && "$SCREENSHOT_PATH" != "$ARTIFACT_PATH" ]] \
  || { echo "error: output, screenshot, and artifact paths must be distinct" >&2; exit 64; }

SOURCE_BEFORE="$($ROOT_DIR/scripts/hepta-ui-source-fingerprint)"
if ! jq -e '.worktree_clean == true and .repository_worktree_clean == true and .dirty_path_count == 0 and .repository_dirty_path_count == 0' >/dev/null <<<"$SOURCE_BEFORE"; then
  echo "error: iOS simulator smoke requires a completely clean committed worktree" >&2
  exit 1
fi
SOURCE_HEAD="$(jq -r '.head' <<<"$SOURCE_BEFORE")"
SOURCE_TREE="$(jq -r '.head_tree' <<<"$SOURCE_BEFORE")"
SOURCE_FINGERPRINT="$(jq -r '.source_fingerprint' <<<"$SOURCE_BEFORE")"

TOOLCHAIN_REPORT="$($ROOT_DIR/scripts/hepta-native-mobile-cargo --print-toolchain-contract)"
jq -e '
  .status == "ready"
  and .resolved_toolchain == "1.95.0"
  and .cargo_makepad.revision == "c4335cee10b22aca768510c9d072b0ca1bba15c8"
  and .cargo_makepad.exact_revision_source_marker_ready == true
  and .cargo_makepad.global_cargo_makepad_used == false
  and .user_global_stable_mutated == false
' >/dev/null <<<"$TOOLCHAIN_REPORT"

DEVICE_INVENTORY="$(xcrun simctl list devices --json)"
if [[ "$DEVICE" == "booted" ]]; then
  DEVICE_MATCHES="$(jq '[.devices | to_entries[] as $runtime | $runtime.value[] | select(.isAvailable == true and .state == "Booted") | {udid,name,state,is_available:.isAvailable,runtime_identifier:$runtime.key}]' <<<"$DEVICE_INVENTORY")"
else
  DEVICE_MATCHES="$(jq --arg udid "$DEVICE" '[.devices | to_entries[] as $runtime | $runtime.value[] | select(.udid == $udid and .isAvailable == true and .state == "Booted") | {udid,name,state,is_available:.isAvailable,runtime_identifier:$runtime.key}]' <<<"$DEVICE_INVENTORY")"
fi
[[ "$(jq 'length' <<<"$DEVICE_MATCHES")" == "1" ]] || {
  echo "error: --device must identify exactly one available, already-booted simulator" >&2
  exit 1
}
DEVICE_REPORT="$(jq '.[0]' <<<"$DEVICE_MATCHES")"
DEVICE_UDID="$(jq -r '.udid' <<<"$DEVICE_REPORT")"

if [[ -n "${CARGO_TARGET_DIR:-}" ]]; then
  if [[ "$CARGO_TARGET_DIR" = /* ]]; then
    APPLE_TARGET_ROOT="$CARGO_TARGET_DIR"
  else
    APPLE_TARGET_ROOT="$APP_DIR/$CARGO_TARGET_DIR"
  fi
else
  APPLE_TARGET_ROOT="$APP_DIR/target/apple"
fi
case "$(uname -m)" in
  arm64|aarch64) SIM_TARGET="aarch64-apple-ios-sim" ;;
  x86_64) SIM_TARGET="x86_64-apple-ios" ;;
  *) echo "error: unsupported macOS host architecture: $(uname -m)" >&2; exit 1 ;;
esac
BUILD_DIR="$APPLE_TARGET_ROOT/makepad-apple-app/$SIM_TARGET/release"
APP_BUNDLE="$BUILD_DIR/$CARGO_PACKAGE.app"
BINARY="$APP_BUNDLE/$CARGO_PACKAGE"
SCENT="$BUILD_DIR/$CARGO_PACKAGE.scent"
PLIST="$APP_BUNDLE/Info.plist"

# Remove every reusable output before invoking the builder. The receipt and
# screenshot are also removed so an interrupted prior run cannot be promoted.
rm -rf "$APP_BUNDLE" "$SCENT"
rm -f "$REPORT_PATH" "$SCREENSHOT_PATH" "$ARTIFACT_PATH"

CARGO_PROFILE_RELEASE_DEBUG=false \
CARGO_PROFILE_RELEASE_STRIP=symbols \
CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 \
  "$ROOT_DIR/scripts/hepta-native-mobile-cargo" apple ios --stable \
    --org=ai.hepta \
    --app=nativeapp \
    build -p "$CARGO_PACKAGE" --locked --release

[[ -d "$APP_BUNDLE" ]] || { echo "error: current build did not create $APP_BUNDLE" >&2; exit 1; }
[[ -f "$BINARY" ]] || { echo "error: current build did not create $BINARY" >&2; exit 1; }
# cargo-makepad's generated-bundle copy can lose the executable bit even when
# the freshly-linked target binary is valid. Normalize the bundle payload
# before archiving or simctl installation and prove the mode was restored.
chmod 0755 "$BINARY"
[[ -x "$BINARY" ]] || { echo "error: current build did not create executable $BINARY" >&2; exit 1; }
[[ -s "$PLIST" ]] || { echo "error: current build did not create Info.plist" >&2; exit 1; }
plutil -lint "$PLIST" >/dev/null

set_or_add() {
  local key="$1" type="$2" value="$3"
  if ! /usr/libexec/PlistBuddy -c "Add :$key $type $value" "$PLIST" 2>/dev/null; then
    /usr/libexec/PlistBuddy -c "Set :$key $value" "$PLIST"
  fi
}
set_or_add CFBundleDisplayName string "$PRODUCT_NAME"
set_or_add CFBundleName string "$PRODUCT_NAME"

BUILT_IDENTIFIER="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$PLIST")"
BUILT_DISPLAY_NAME="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleDisplayName' "$PLIST")"
BUILT_BUNDLE_NAME="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleName' "$PLIST")"
BUILT_EXECUTABLE="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$PLIST")"
[[ "$BUILT_IDENTIFIER" == "$BUNDLE_IDENTIFIER" ]] || { echo "error: bundle id drifted to $BUILT_IDENTIFIER" >&2; exit 1; }
[[ "$BUILT_DISPLAY_NAME" == "$PRODUCT_NAME" ]] || { echo "error: CFBundleDisplayName drifted to $BUILT_DISPLAY_NAME" >&2; exit 1; }
[[ "$BUILT_BUNDLE_NAME" == "$PRODUCT_NAME" ]] || { echo "error: CFBundleName drifted to $BUILT_BUNDLE_NAME" >&2; exit 1; }
[[ "$BUILT_EXECUTABLE" == "$CARGO_PACKAGE" ]] || { echo "error: executable drifted to $BUILT_EXECUTABLE" >&2; exit 1; }
if ! strings "$BINARY" | grep -F "https://github.com/ProfAlexQI/Hepta/commit/$SOURCE_HEAD" >/dev/null; then
  echo "error: simulator binary is not bound to current source HEAD $SOURCE_HEAD" >&2
  exit 1
fi

# A cargo-makepad subprocess can print an actool/runtime warning while still
# exiting zero. Require actual compiled catalog output from this just-built
# bundle. Newer actool versions emit Assets.car; iOS 17.x may instead emit the
# strict legacy icon set plus actool-Info.plist. Both branches validate bytes
# produced by actool, not merely the source asset catalog.
[[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIconName' "$PLIST")" == "AppIcon" ]] \
  || { echo "error: built bundle does not select the compiled AppIcon catalog" >&2; exit 1; }
ASSET_CAR="$APP_BUNDLE/Assets.car"
ACTOOL_INFO="$APP_BUNDLE/actool-Info.plist"
ASSET_CATALOG_REPORT='{}'
if [[ -s "$ASSET_CAR" ]]; then
  ASSET_CAR_SHA256="$(shasum -a 256 "$ASSET_CAR" | awk '{print $1}')"
  ASSET_CATALOG_REPORT="$(jq -n \
    --arg path "Assets.car" \
    --arg sha256 "$ASSET_CAR_SHA256" \
    '{compiled_asset_catalog_ready:true,mode:"assets_car",evidence:{path:$path,sha256:$sha256}}')"
else
  [[ -s "$ACTOOL_INFO" ]] || { echo "error: build emitted neither Assets.car nor actool-Info.plist" >&2; exit 1; }
  plutil -lint "$ACTOOL_INFO" >/dev/null
  [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIcons:CFBundlePrimaryIcon:CFBundleIconName' "$ACTOOL_INFO")" == "AppIcon" ]] \
    || { echo "error: actool phone icon contract is missing" >&2; exit 1; }
  [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIcons~ipad:CFBundlePrimaryIcon:CFBundleIconName' "$ACTOOL_INFO")" == "AppIcon" ]] \
    || { echo "error: actool iPad icon contract is missing" >&2; exit 1; }
  ACTOOL_INFO_SHA256="$(shasum -a 256 "$ACTOOL_INFO" | awk '{print $1}')"
  ICON_OUTPUTS='[]'
  for specification in \
    'AppIcon60x60@2x.png:120' \
    'AppIcon60x60@3x.png:180' \
    'AppIcon76x76@2x~ipad.png:152' \
    'AppIcon83.5x83.5@2x~ipad.png:167'; do
    icon_name="${specification%%:*}"
    expected_pixels="${specification##*:}"
    icon_path="$APP_BUNDLE/$icon_name"
    [[ -s "$icon_path" ]] || { echo "error: actool output is missing $icon_name" >&2; exit 1; }
    icon_width="$(sips -g pixelWidth "$icon_path" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
    icon_height="$(sips -g pixelHeight "$icon_path" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"
    icon_alpha="$(sips -g hasAlpha "$icon_path" 2>/dev/null | awk '/hasAlpha:/ {print $2}')"
    [[ "$icon_width" == "$expected_pixels" && "$icon_height" == "$expected_pixels" && "$icon_alpha" == "no" ]] \
      || { echo "error: invalid compiled icon output $icon_name" >&2; exit 1; }
    icon_sha256="$(shasum -a 256 "$icon_path" | awk '{print $1}')"
    ICON_OUTPUTS="$(jq \
      --arg path "$icon_name" \
      --arg sha256 "$icon_sha256" \
      --argjson pixels "$expected_pixels" \
      '. + [{path:$path,sha256:$sha256,width:$pixels,height:$pixels,alpha:false}]' <<<"$ICON_OUTPUTS")"
  done
  ASSET_CATALOG_REPORT="$(jq -n \
    --arg path "actool-Info.plist" \
    --arg sha256 "$ACTOOL_INFO_SHA256" \
    --argjson outputs "$ICON_OUTPUTS" \
    '{compiled_asset_catalog_ready:true,mode:"actool_info_and_opaque_icon_outputs",evidence:{path:$path,sha256:$sha256},icon_outputs:$outputs}')"

  # A newer Xcode SDK can return non-zero against an older simulator runtime
  # after still producing the complete legacy icon set and partial plist. The
  # pinned cargo-makepad warning path leaves that plist unmerged, so merge it
  # only after validating every emitted icon byte above.
  /usr/libexec/PlistBuddy -c "Merge $ACTOOL_INFO" "$PLIST"
  [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIcons:CFBundlePrimaryIcon:CFBundleIconName' "$PLIST")" == "AppIcon" ]] \
    || { echo "error: fallback phone icon metadata was not merged" >&2; exit 1; }
  [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIcons~ipad:CFBundlePrimaryIcon:CFBundleIconName' "$PLIST")" == "AppIcon" ]] \
    || { echo "error: fallback iPad icon metadata was not merged" >&2; exit 1; }
fi
jq -e '
  .compiled_asset_catalog_ready == true
  and (.mode == "assets_car" or .mode == "actool_info_and_opaque_icon_outputs")
  and (.evidence.sha256 | test("^[0-9a-f]{64}$"))
' >/dev/null <<<"$ASSET_CATALOG_REPORT"

SOURCE_AFTER_BUILD="$($ROOT_DIR/scripts/hepta-ui-source-fingerprint)"
if ! jq -e \
  --arg head "$SOURCE_HEAD" \
  --arg tree "$SOURCE_TREE" \
  --arg fingerprint "$SOURCE_FINGERPRINT" '
    .head == $head
    and .head_tree == $tree
    and .source_fingerprint == $fingerprint
    and .worktree_clean == true
    and .repository_worktree_clean == true
  ' >/dev/null <<<"$SOURCE_AFTER_BUILD"; then
  echo "error: source changed during the iOS simulator build" >&2
  exit 1
fi

ditto -c -k --sequesterRsrc --keepParent "$APP_BUNDLE" "$ARTIFACT_PATH"
[[ -s "$ARTIFACT_PATH" ]] || { echo "error: simulator artifact archive was not created" >&2; exit 1; }
ARTIFACT_SHA256="$(shasum -a 256 "$ARTIFACT_PATH" | awk '{print $1}')"

if xcrun simctl get_app_container "$DEVICE_UDID" "$BUNDLE_IDENTIFIER" app >/dev/null 2>&1; then
  xcrun simctl terminate "$DEVICE_UDID" "$BUNDLE_IDENTIFIER" >/dev/null 2>&1 || :
  xcrun simctl uninstall "$DEVICE_UDID" "$BUNDLE_IDENTIFIER"
fi
xcrun simctl install "$DEVICE_UDID" "$APP_BUNDLE"
CONTAINER_PATH="$(xcrun simctl get_app_container "$DEVICE_UDID" "$BUNDLE_IDENTIFIER" app)"
[[ -d "$CONTAINER_PATH" ]] || { echo "error: simctl returned no installed app container" >&2; exit 1; }
INSTALLED_PLIST="$CONTAINER_PATH/Info.plist"
[[ -s "$INSTALLED_PLIST" ]] || { echo "error: installed simulator app has no Info.plist" >&2; exit 1; }
[[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleDisplayName' "$INSTALLED_PLIST")" == "$PRODUCT_NAME" ]] \
  || { echo "error: installed simulator app lost Hepta display branding" >&2; exit 1; }
[[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleName' "$INSTALLED_PLIST")" == "$PRODUCT_NAME" ]] \
  || { echo "error: installed simulator app lost Hepta bundle branding" >&2; exit 1; }

LAUNCH_OUTPUT="$(xcrun simctl launch --terminate-running-process "$DEVICE_UDID" "$BUNDLE_IDENTIFIER")"
LAUNCH_PID="$(awk -v bundle="$BUNDLE_IDENTIFIER" '$1 == bundle ":" && $2 ~ /^[0-9]+$/ {print $2; exit}' <<<"$LAUNCH_OUTPUT")"
[[ "$LAUNCH_PID" =~ ^[0-9]+$ ]] || { echo "error: could not parse a launch pid from: $LAUNCH_OUTPUT" >&2; exit 1; }
sleep 2
# Simulator runtimes do not guarantee that the userland `ps` utility is
# present (the iOS 17.5 runtime is one such case). launchd is the process
# authority for Simulator apps, so verify the exact UIKitApplication job and
# PID through launchctl instead of depending on an optional binary.
PROCESS_REPORT="$(xcrun simctl spawn "$DEVICE_UDID" launchctl list)"
if ! awk -v pid="$LAUNCH_PID" -v bundle="$BUNDLE_IDENTIFIER" '
  $1 == pid && index($3, "UIKitApplication:" bundle "[") == 1 {found=1}
  END {exit(found ? 0 : 1)}
' <<<"$PROCESS_REPORT"; then
  echo "error: Hepta process $LAUNCH_PID exited before screenshot capture" >&2
  exit 1
fi

xcrun simctl io "$DEVICE_UDID" screenshot --type=png "$SCREENSHOT_PATH" >/dev/null
[[ -s "$SCREENSHOT_PATH" ]] || { echo "error: simctl did not create a screenshot" >&2; exit 1; }
SCREENSHOT_WIDTH="$(sips -g pixelWidth "$SCREENSHOT_PATH" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
SCREENSHOT_HEIGHT="$(sips -g pixelHeight "$SCREENSHOT_PATH" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"
[[ "$SCREENSHOT_WIDTH" =~ ^[1-9][0-9]*$ && "$SCREENSHOT_HEIGHT" =~ ^[1-9][0-9]*$ ]] \
  || { echo "error: screenshot dimensions are invalid" >&2; exit 1; }
SCREENSHOT_SHA256="$(shasum -a 256 "$SCREENSHOT_PATH" | awk '{print $1}')"

SOURCE_FINAL="$($ROOT_DIR/scripts/hepta-ui-source-fingerprint)"
if ! jq -e \
  --arg head "$SOURCE_HEAD" \
  --arg tree "$SOURCE_TREE" \
  --arg fingerprint "$SOURCE_FINGERPRINT" '
    .head == $head
    and .head_tree == $tree
    and .source_fingerprint == $fingerprint
    and .worktree_clean == true
    and .repository_worktree_clean == true
  ' >/dev/null <<<"$SOURCE_FINAL"; then
  echo "error: source changed during the iOS simulator smoke" >&2
  exit 1
fi

jq -n \
  --arg generated_at_utc "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg producer "$PRODUCER" \
  --argjson source_binding "$SOURCE_FINAL" \
  --argjson device "$DEVICE_REPORT" \
  --arg artifact_path "$ARTIFACT_PATH" \
  --arg artifact_sha256 "$ARTIFACT_SHA256" \
  --arg screenshot_path "$SCREENSHOT_PATH" \
  --arg screenshot_sha256 "$SCREENSHOT_SHA256" \
  --argjson screenshot_width "$SCREENSHOT_WIDTH" \
  --argjson screenshot_height "$SCREENSHOT_HEIGHT" \
  --arg bundle_identifier "$BUILT_IDENTIFIER" \
  --arg display_name "$BUILT_DISPLAY_NAME" \
  --arg bundle_name "$BUILT_BUNDLE_NAME" \
  --arg executable "$BUILT_EXECUTABLE" \
  --argjson asset_catalog "$ASSET_CATALOG_REPORT" \
  --arg launch_output "$LAUNCH_OUTPUT" \
  --argjson launch_pid "$LAUNCH_PID" \
  --arg app_container "$CONTAINER_PATH" \
  --argjson toolchain "$TOOLCHAIN_REPORT" '
    {
      schema_version:1,
      kind:"hepta-native-ios-simulator-smoke-receipt",
      producer:$producer,
      status:"ready",
      ready:true,
      generated_at_utc:$generated_at_utc,
      source_binding:$source_binding,
      device:$device,
      artifact:{path:$artifact_path,sha256:$artifact_sha256,format:"zip",stale_artifact_accepted:false},
      screenshot:{path:$screenshot_path,sha256:$screenshot_sha256,format:"png",width:$screenshot_width,height:$screenshot_height},
      bundle:{identifier:$bundle_identifier,display_name:$display_name,name:$bundle_name,executable:$executable},
      asset_catalog:$asset_catalog,
      launch:{ready:true,install_succeeded:true,launch_succeeded:true,pid:$launch_pid,output:$launch_output,app_container:$app_container},
      toolchain:$toolchain,
      signing:{performed:false},
      local_simulator_side_effects:{app_install:true,app_launch:true,screenshot_capture:true},
      forbidden_actions_performed:{runtime_download:false,simulator_create:false,account_create:false,code_sign:false,real_device_contact:false,upload:false},
      hard_boundaries:{
        ios_real_device_verified:false,
        safe_area_verified:false,
        software_keyboard_verified:false,
        voiceover_verified:false,
        rtl_verified:false,
        dynamic_type_verified:false,
        public_distribution_ready:false
      }
    }
  ' >"$REPORT_PATH"

jq -e '.schema_version == 1 and .kind == "hepta-native-ios-simulator-smoke-receipt" and .status == "ready" and .ready == true' "$REPORT_PATH" >/dev/null
echo "==> iOS simulator runtime verified for current source $SOURCE_HEAD"
echo "==> Receipt:    $REPORT_PATH"
echo "==> Artifact:   $ARTIFACT_PATH"
echo "==> Screenshot: $SCREENSHOT_PATH"
