#!/usr/bin/env bash
set -euo pipefail

# Build, sign, and package the current Hepta Native source as an iOS .ipa,
# then optionally upload it to TestFlight. This lane is intentionally strict:
# a missing device/build/signing input is a hard failure and a pre-existing
# bundle is deleted before the build, so stale output can never be re-signed.
#
# Required environment:
#   IOS_PROVISIONING_PROFILE_UUID  installed .mobileprovision UUID
#
# Optional environment:
#   IOS_SIGNING_IDENTITY     certificate common name (default Apple Distribution)
#   IOS_PROVISIONING_PROFILE_PATH explicit profile path (otherwise UUID lookup)
#   IOS_DEVICE               cargo-makepad device selector (default IPhone)
#   IOS_UPLOAD_TESTFLIGHT    true to upload after packaging (default false)
#   ASC_KEY_ID / ASC_ISSUER_ID App Store Connect API identifiers for upload
#   TESTFLIGHT_BUILD_NUMBER  CFBundleVersion (default Pacific-time timestamp)
#   ORG / APP                bundle id components (default ai.hepta / nativeapp)
#   CARGO_PACKAGE            Cargo package/bin (fixed default hepta-native)
#
# cargo-makepad calls `rustup run stable` internally. The repository wrapper
# maps that symbolic channel to the pinned Rust 1.95.0 toolchain without
# changing the user's global stable toolchain.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
APP_DIR="$(cd "$SCRIPT_DIR/.." && pwd -P)"
REPO_ROOT="$(git -C "$APP_DIR" rev-parse --show-toplevel)"
cd "$APP_DIR"

ORG="${ORG:-ai.hepta}"
APP="${APP:-nativeapp}"
CARGO_PACKAGE="${CARGO_PACKAGE:-hepta-native}"
PRODUCT_NAME="Hepta"
SIGNING_IDENTITY="${IOS_SIGNING_IDENTITY:-Apple Distribution}"
PROFILE_UUID="${IOS_PROVISIONING_PROFILE_UUID:?set IOS_PROVISIONING_PROFILE_UUID}"
DEVICE="${IOS_DEVICE:-IPhone}"
BUILD_NUMBER="${TESTFLIGHT_BUILD_NUMBER:-$(TZ=America/Los_Angeles date +%Y%m%d.%H%M)}"

if [[ "$CARGO_PACKAGE" != "hepta-native" ]]; then
  echo "Error: CARGO_PACKAGE must remain hepta-native; --app controls the bundle id separately." >&2
  exit 64
fi
if [[ "$ORG.$APP" != "ai.hepta.nativeapp" ]]; then
  echo "Error: ORG and APP must remain ai.hepta and nativeapp for the canonical Hepta bundle id." >&2
  exit 64
fi

for command in git jq rustup security xcrun codesign ditto shasum strings sips plutil; do
  command -v "$command" >/dev/null 2>&1 || { echo "Error: $command is required." >&2; exit 2; }
done

if [[ -n "$(git -C "$REPO_ROOT" status --porcelain --untracked-files=all)" ]]; then
  echo "Error: TestFlight packaging requires a clean, committed worktree." >&2
  exit 1
fi
SOURCE_HEAD="$(git -C "$REPO_ROOT" rev-parse HEAD)"
SOURCE_TREE="$(git -C "$REPO_ROOT" rev-parse HEAD^{tree})"

ICON_REPORT="$($REPO_ROOT/scripts/hepta-native-ios-icons verify)"
TOOLCHAIN_REPORT="$($REPO_ROOT/scripts/hepta-native-mobile-cargo --print-toolchain-contract)"
jq -e '.status == "ready" and .app_store_marketing_icon_opaque == true and .canonical.alpha == false and (.generated | all(.alpha == false))' >/dev/null <<<"$ICON_REPORT"
jq -e '.status == "ready" and .resolved_toolchain == "1.95.0" and .cargo_makepad.revision == "c4335cee10b22aca768510c9d072b0ca1bba15c8" and .cargo_makepad.exact_revision_source_marker_ready == true and .cargo_makepad.global_cargo_makepad_used == false and .user_global_stable_mutated == false' >/dev/null <<<"$TOOLCHAIN_REPORT"

CERT_SHA1="$(security find-identity -v -p codesigning \
  | awk -v id="$SIGNING_IDENTITY" 'index($0, id) {print $2; exit}')"
if [[ ! "$CERT_SHA1" =~ ^[0-9A-Fa-f]{40}$ ]]; then
  echo "Error: no code-signing identity matching '$SIGNING_IDENTITY' in the keychain." >&2
  exit 1
fi

PROFILE_PATH="${IOS_PROVISIONING_PROFILE_PATH:-}"
if [[ -z "$PROFILE_PATH" ]]; then
  for candidate in \
    "$HOME/Library/Developer/Xcode/UserData/Provisioning Profiles/${PROFILE_UUID}.mobileprovision" \
    "$HOME/Library/MobileDevice/Provisioning Profiles/${PROFILE_UUID}.mobileprovision"; do
    if [[ -f "$candidate" ]]; then PROFILE_PATH="$candidate"; break; fi
  done
fi
if [[ ! -f "$PROFILE_PATH" ]]; then
  echo "Error: provisioning profile $PROFILE_UUID was not found in an Xcode profile directory." >&2
  exit 1
fi

if [[ -n "${CARGO_TARGET_DIR:-}" ]]; then
  if [[ "$CARGO_TARGET_DIR" = /* ]]; then
    APPLE_TARGET_ROOT="$CARGO_TARGET_DIR"
  else
    APPLE_TARGET_ROOT="$APP_DIR/$CARGO_TARGET_DIR"
  fi
else
  APPLE_TARGET_ROOT="$APP_DIR/target/apple"
fi
BUILD_DIR="$APPLE_TARGET_ROOT/makepad-apple-app/aarch64-apple-ios/release"
APP_BUNDLE="$BUILD_DIR/${CARGO_PACKAGE}.app"
BINARY="$APP_BUNDLE/$CARGO_PACKAGE"
SCENT="$BUILD_DIR/${CARGO_PACKAGE}.scent"

echo "==> Source:   $SOURCE_HEAD"
echo "==> Identity: $SIGNING_IDENTITY ($CERT_SHA1)"
echo "==> Profile:  $PROFILE_UUID"
echo "==> Build:    $BUILD_NUMBER"

# run-device is the only cargo-makepad command that builds the arm64 device
# target. It must complete successfully; a missing device is not treated as a
# successful package build. Delete both possible reusable outputs first.
rm -rf "$APP_BUNDLE" "$SCENT"
CARGO_PROFILE_RELEASE_DEBUG=false \
CARGO_PROFILE_RELEASE_STRIP=symbols \
CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 \
CARGO_PROFILE_RELEASE_LTO=fat \
TESTFLIGHT_BUILD_NUMBER="$BUILD_NUMBER" \
  "$REPO_ROOT/scripts/hepta-native-mobile-cargo" apple ios --stable \
    --org="$ORG" \
    --app="$APP" \
    --profile="$PROFILE_UUID" \
    --cert="$CERT_SHA1" \
    --device="$DEVICE" \
    run-device -p "$CARGO_PACKAGE" --locked --release

[[ -d "$APP_BUNDLE" ]] || { echo "Error: current build did not create $APP_BUNDLE" >&2; exit 1; }
[[ -f "$BINARY" ]] || { echo "Error: current build did not create $BINARY" >&2; exit 1; }
# The pinned cargo-makepad copy step currently preserves the executable bytes
# but can drop their mode inside the generated .app. Restore the canonical app
# executable mode before signing or packaging, then fail closed if it did not
# take effect.
chmod 0755 "$BINARY"
[[ -x "$BINARY" ]] || { echo "Error: current build did not create executable $BINARY" >&2; exit 1; }
if [[ "$(git -C "$REPO_ROOT" rev-parse HEAD)" != "$SOURCE_HEAD" \
  || "$(git -C "$REPO_ROOT" rev-parse HEAD^{tree})" != "$SOURCE_TREE" \
  || -n "$(git -C "$REPO_ROOT" status --porcelain --untracked-files=all)" ]]; then
  echo "Error: source changed during the iOS build." >&2
  exit 1
fi

PLIST="$APP_BUNDLE/Info.plist"
[[ -s "$PLIST" ]] || { echo "Error: built bundle has no Info.plist." >&2; exit 1; }
BUILT_IDENTIFIER="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$PLIST")"
BUILT_EXECUTABLE="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$PLIST")"
[[ "$BUILT_IDENTIFIER" == "$ORG.$APP" ]] || { echo "Error: bundle id drifted to $BUILT_IDENTIFIER" >&2; exit 1; }
[[ "$BUILT_EXECUTABLE" == "$CARGO_PACKAGE" ]] || { echo "Error: bundle executable drifted to $BUILT_EXECUTABLE" >&2; exit 1; }
if ! strings "$BINARY" | grep -F "https://github.com/ProfAlexQI/Hepta/commit/$SOURCE_HEAD" >/dev/null; then
  echo "Error: built binary is not bound to current source HEAD $SOURCE_HEAD." >&2
  exit 1
fi

SDK_VER="$(xcrun vtool -show-build "$BINARY" 2>/dev/null | awk '$1=="sdk"{print $2; exit}')"
if [[ ! "$SDK_VER" =~ ^[0-9]+([.][0-9]+)*$ ]]; then
  echo "Error: could not read the linked iOS SDK version from $BINARY." >&2
  exit 1
fi
if [[ "${SDK_VER%%.*}" -lt 26 ]]; then
  echo "Error: built against iOS SDK $SDK_VER; this release lane requires 26+. Active Xcode: $(xcode-select -p)" >&2
  exit 1
fi
echo "==> Linked iOS SDK: $SDK_VER"

TEMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ios-testflight.XXXXXX")"
trap 'rm -rf "$TEMP_DIR"' EXIT
ASSET_INFO="$TEMP_DIR/AssetInfo.plist"
PROFILE_PLIST="$TEMP_DIR/profile.plist"
ENTITLEMENTS="$TEMP_DIR/entitlements.plist"

xcrun actool ./packaging/ios/icons/Assets.xcassets \
  --compile "$APP_BUNDLE" \
  --platform iphoneos \
  --minimum-deployment-target 15.0 \
  --app-icon AppIcon \
  --output-partial-info-plist "$ASSET_INFO"
/usr/libexec/PlistBuddy -c "Merge $ASSET_INFO" "$PLIST"

set_or_add() {
  local key="$1" type="$2" value="$3"
  if ! /usr/libexec/PlistBuddy -c "Add :$key $type $value" "$PLIST" 2>/dev/null; then
    /usr/libexec/PlistBuddy -c "Set :$key $value" "$PLIST"
  fi
}
set_or_add CFBundlePackageType string APPL
set_or_add CFBundleDisplayName string "$PRODUCT_NAME"
set_or_add CFBundleName string "$PRODUCT_NAME"
set_or_add CFBundleIconName string AppIcon
if ! /usr/libexec/PlistBuddy -c "Add :UILaunchScreen dict" "$PLIST" 2>/dev/null; then
  /usr/libexec/PlistBuddy -c "Print :UILaunchScreen" "$PLIST" >/dev/null
fi
set_or_add UILaunchScreen:UIImageName string AppIcon60x60
set_or_add UILaunchScreen:UIColorName string LaunchScreenBackground
set_or_add ITSAppUsesNonExemptEncryption bool false

[[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIconName' "$PLIST")" == "AppIcon" ]] \
  || { echo "Error: built bundle does not select the compiled AppIcon catalog." >&2; exit 1; }
ASSET_CAR="$APP_BUNDLE/Assets.car"
ASSET_CATALOG_REPORT='{}'
if [[ -s "$ASSET_CAR" ]]; then
  ASSET_CATALOG_REPORT="$(jq -n \
    --arg sha256 "$(shasum -a 256 "$ASSET_CAR" | awk '{print $1}')" \
    '{compiled_asset_catalog_ready:true,mode:"assets_car",evidence:{path:"Assets.car",sha256:$sha256}}')"
else
  plutil -lint "$ASSET_INFO" >/dev/null
  [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIcons:CFBundlePrimaryIcon:CFBundleIconName' "$ASSET_INFO")" == "AppIcon" ]] \
    || { echo "Error: actool phone icon contract is missing." >&2; exit 1; }
  [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIcons~ipad:CFBundlePrimaryIcon:CFBundleIconName' "$ASSET_INFO")" == "AppIcon" ]] \
    || { echo "Error: actool iPad icon contract is missing." >&2; exit 1; }
  ICON_OUTPUTS='[]'
  for specification in \
    'AppIcon60x60@2x.png:120' \
    'AppIcon60x60@3x.png:180' \
    'AppIcon76x76@2x~ipad.png:152' \
    'AppIcon83.5x83.5@2x~ipad.png:167'; do
    icon_name="${specification%%:*}"
    expected_pixels="${specification##*:}"
    icon_path="$APP_BUNDLE/$icon_name"
    [[ -s "$icon_path" ]] || { echo "Error: actool output is missing $icon_name." >&2; exit 1; }
    icon_width="$(sips -g pixelWidth "$icon_path" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
    icon_height="$(sips -g pixelHeight "$icon_path" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"
    icon_alpha="$(sips -g hasAlpha "$icon_path" 2>/dev/null | awk '/hasAlpha:/ {print $2}')"
    [[ "$icon_width" == "$expected_pixels" && "$icon_height" == "$expected_pixels" && "$icon_alpha" == "no" ]] \
      || { echo "Error: invalid compiled icon output $icon_name." >&2; exit 1; }
    ICON_OUTPUTS="$(jq \
      --arg path "$icon_name" \
      --arg sha256 "$(shasum -a 256 "$icon_path" | awk '{print $1}')" \
      --argjson pixels "$expected_pixels" \
      '. + [{path:$path,sha256:$sha256,width:$pixels,height:$pixels,alpha:false}]' <<<"$ICON_OUTPUTS")"
  done
  ASSET_CATALOG_REPORT="$(jq -n \
    --arg sha256 "$(shasum -a 256 "$ASSET_INFO" | awk '{print $1}')" \
    --argjson outputs "$ICON_OUTPUTS" \
    '{compiled_asset_catalog_ready:true,mode:"actool_info_and_opaque_icon_outputs",evidence:{path:"actool-partial-info.plist",sha256:$sha256},icon_outputs:$outputs}')"
fi
jq -e '.compiled_asset_catalog_ready == true and (.evidence.sha256 | test("^[0-9a-f]{64}$"))' >/dev/null <<<"$ASSET_CATALOG_REPORT"

VERSION="$(cargo metadata --locked --offline --no-deps --format-version 1 \
  | jq -r --arg package "$CARGO_PACKAGE" '.packages[] | select(.name == $package) | .version' \
  | sed 's/-.*$//')"
[[ "$VERSION" =~ ^[0-9]+[.][0-9]+[.][0-9]+$ ]] || { echo "Error: invalid release version '$VERSION'." >&2; exit 1; }
/usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $VERSION" "$PLIST"
/usr/libexec/PlistBuddy -c "Set :CFBundleVersion $BUILD_NUMBER" "$PLIST"

# cargo-makepad derives the generated bundle name from --app and currently
# emits "Nativeapp". Branding is package metadata, not the executable or
# bundle-id component, so rewrite it deterministically and reject any drift
# before signing. A signed Nativeapp-branded payload must never be emitted.
BUILT_IDENTIFIER="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$PLIST")"
BUILT_EXECUTABLE="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$PLIST")"
BUILT_DISPLAY_NAME="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleDisplayName' "$PLIST")"
BUILT_BUNDLE_NAME="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleName' "$PLIST")"
[[ "$BUILT_IDENTIFIER" == "ai.hepta.nativeapp" ]] || { echo "Error: final bundle id drifted to $BUILT_IDENTIFIER" >&2; exit 1; }
[[ "$BUILT_IDENTIFIER" == "$ORG.$APP" ]] || { echo "Error: requested bundle id drifted to $BUILT_IDENTIFIER" >&2; exit 1; }
[[ "$BUILT_EXECUTABLE" == "hepta-native" ]] || { echo "Error: final executable drifted to $BUILT_EXECUTABLE" >&2; exit 1; }
[[ "$BUILT_EXECUTABLE" == "$CARGO_PACKAGE" ]] || { echo "Error: requested executable drifted to $BUILT_EXECUTABLE" >&2; exit 1; }
[[ "$BUILT_DISPLAY_NAME" == "$PRODUCT_NAME" ]] || { echo "Error: CFBundleDisplayName drifted to $BUILT_DISPLAY_NAME" >&2; exit 1; }
[[ "$BUILT_BUNDLE_NAME" == "$PRODUCT_NAME" ]] || { echo "Error: CFBundleName drifted to $BUILT_BUNDLE_NAME" >&2; exit 1; }

security cms -D -i "$PROFILE_PATH" >"$PROFILE_PLIST"
/usr/libexec/PlistBuddy -x -c "Print :Entitlements" "$PROFILE_PLIST" >"$ENTITLEMENTS"
codesign --force --sign "$CERT_SHA1" \
  --entitlements "$ENTITLEMENTS" \
  --timestamp=none \
  "$APP_BUNDLE"
codesign --verify --deep --strict --verbose=3 "$APP_BUNDLE"
codesign -d --entitlements :- "$APP_BUNDLE" >/dev/null

IPA_NAME="Hepta-${VERSION}-ios.ipa"
IPA_PATH="$BUILD_DIR/$IPA_NAME"
PAYLOAD_ROOT="$TEMP_DIR/Payload"
mkdir -p "$PAYLOAD_ROOT"
ditto "$APP_BUNDLE" "$PAYLOAD_ROOT/Hepta.app"
rm -f "$IPA_PATH"
(cd "$TEMP_DIR" && ditto -c -k --sequesterRsrc --keepParent Payload "$IPA_PATH")
[[ -s "$IPA_PATH" ]] || { echo "Error: IPA packaging did not produce $IPA_PATH" >&2; exit 1; }

UPLOADED=false
if [[ "${IOS_UPLOAD_TESTFLIGHT:-false}" == "true" ]]; then
  : "${ASC_KEY_ID:?set ASC_KEY_ID to upload}"
  : "${ASC_ISSUER_ID:?set ASC_ISSUER_ID to upload}"
  xcrun altool --upload-app --type ios \
    --file "$IPA_PATH" \
    --apiKey "$ASC_KEY_ID" \
    --apiIssuer "$ASC_ISSUER_ID"
  UPLOADED=true
fi

IPA_SHA256="$(shasum -a 256 "$IPA_PATH" | awk '{print $1}')"
RECEIPT_PATH="$IPA_PATH.receipt.json"
jq -n \
  --arg source_head "$SOURCE_HEAD" \
  --arg source_tree "$SOURCE_TREE" \
  --arg artifact_path "$IPA_PATH" \
  --arg artifact_sha256 "$IPA_SHA256" \
  --arg bundle_identifier "$BUILT_IDENTIFIER" \
  --arg executable "$BUILT_EXECUTABLE" \
  --arg display_name "$BUILT_DISPLAY_NAME" \
  --arg bundle_name "$BUILT_BUNDLE_NAME" \
  --arg sdk_version "$SDK_VER" \
  --arg signing_identity "$SIGNING_IDENTITY" \
  --arg signing_identity_sha1 "$CERT_SHA1" \
  --arg provisioning_profile_uuid "$PROFILE_UUID" \
  --argjson toolchain "$TOOLCHAIN_REPORT" \
  --argjson icons "$ICON_REPORT" \
  --argjson asset_catalog "$ASSET_CATALOG_REPORT" \
  --argjson uploaded "$UPLOADED" \
  '{schema_version:1,kind:"hepta-native-ios-testflight-package",status:"ready",source_binding:{head:$source_head,head_tree:$source_tree,worktree_clean:true},artifact:{path:$artifact_path,sha256:$artifact_sha256},bundle:{identifier:$bundle_identifier,display_name:$display_name,name:$bundle_name,executable:$executable,linked_sdk:$sdk_version},toolchain:$toolchain,icons:$icons,asset_catalog:$asset_catalog,signing:{performed:true,identity:$signing_identity,identity_sha1:$signing_identity_sha1,provisioning_profile_uuid:$provisioning_profile_uuid},testflight_uploaded:$uploaded,public_distribution_authorized:false,stale_artifact_accepted:false}' \
  >"$RECEIPT_PATH"

ls -lh "$IPA_PATH"
echo "==> Packaged current source: $IPA_PATH"
echo "==> Receipt: $RECEIPT_PATH"
if [[ "$UPLOADED" == "true" ]]; then
  echo "==> Uploaded to TestFlight."
else
  echo "==> Skipping TestFlight upload (IOS_UPLOAD_TESTFLIGHT != true)."
fi
