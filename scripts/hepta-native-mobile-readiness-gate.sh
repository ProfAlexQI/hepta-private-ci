#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

REPORT_PATH=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --output) REPORT_PATH="${2:-}"; shift 2 ;;
    --help|-h)
      cat <<'EOF'
usage: scripts/hepta-native-mobile-readiness-gate.sh [--output report.json]

Validates the current-source mobile build/package contracts and emits the
known iOS/Android runtime boundaries as explicit machine-readable hard false
values. By default it never signs, uploads, boots a simulator, or contacts a
device. Supplying an Android receipt still stays read-only unless the explicit
HEPTA_NATIVE_ANDROID_EMULATOR_LIVE_READBACK=1 opt-in performs one
credential-free start probe against that exact already-running emulator.
EOF
      exit 0
      ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

for command in git jq ruby rustup shasum ditto strings plutil sips find unzip file xmllint; do
  command -v "$command" >/dev/null 2>&1 || { echo "$command is required" >&2; exit 2; }
done

SOURCE_BEFORE="$(scripts/hepta-ui-source-fingerprint)"
POLICY_PATH="apps/hepta-native/mobile-readiness-policy-v1.json"
MANIFEST_PATH="apps/hepta-native/Cargo.toml"
CREDENTIAL_PATH="apps/hepta-native/src/persistence/matrix_session_store/credential.rs"
TESTFLIGHT_PATH="apps/hepta-native/packaging/build-ios-testflight.sh"
IOS_SIMULATOR_SMOKE_PATH="scripts/hepta-native-ios-simulator-smoke.sh"
ANDROID_EMULATOR_SMOKE_PATH="scripts/hepta-native-android-emulator-smoke.sh"
ANDROID_EMULATOR_LIVE_READBACK_PATH="scripts/hepta-native-android-emulator-live-readback"
ANDROID_TRUSTED_ADB_PATH="scripts/hepta-android-trusted-adb"
ANDROID_LOGIN_TEMPLATE_PROBE_PATH="scripts/hepta-android-login-template-probe"
ANDROID_ORIENTATION_PROBE_PATH="scripts/hepta-android-window-orientation-probe"
ANDROID_LOGIN_TEMPLATE_MANIFEST_PATH="apps/hepta-native/packaging/android-emulator-login-template-v1/manifest.json"
IOS_SIMULATOR_RECEIPT="${HEPTA_NATIVE_IOS_SIMULATOR_RECEIPT:-}"
ANDROID_EMULATOR_RECEIPT="${HEPTA_NATIVE_ANDROID_EMULATOR_RECEIPT:-}"
ANDROID_EMULATOR_LIVE_READBACK_OPT_IN="${HEPTA_NATIVE_ANDROID_EMULATOR_LIVE_READBACK:-0}"
case "$ANDROID_EMULATOR_LIVE_READBACK_OPT_IN" in
  0|1) ;;
  *) echo "HEPTA_NATIVE_ANDROID_EMULATOR_LIVE_READBACK must be 0 or 1" >&2; exit 64 ;;
esac

policy_ready=false
if jq -e '
    .schema_version == 1
    and .kind == "hepta-native-mobile-readiness-policy"
    and .makepad_revision == "c4335cee10b22aca768510c9d072b0ca1bba15c8"
    and .known_upstream_boundaries.ios_accessibility_update_consumed == false
    and .known_upstream_boundaries.android_accessibility_update_consumed == false
    and .known_upstream_boundaries.observed_behavior == "CxOsOp::AccessibilityUpdate(_) => {}"
    and .downstream_boundaries.android_secure_credential_backend_supported == false
    and .downstream_boundaries.android_session_behavior == "fail_closed_relogin_required"
    and .downstream_boundaries.plaintext_credential_fallback_allowed == false
    and .downstream_boundaries.ios_bundle_identifier == "ai.hepta.nativeapp"
    and .downstream_boundaries.ios_product_name == "Hepta"
    and .downstream_boundaries.ios_executable == "hepta-native"
    and .downstream_boundaries.ios_simulator_smoke_signing_performed == false
    and .promotion_requirements.android_emulator_receipt_required == true
    and (.promotion_requirements | to_entries | all(.value == true))
  ' "$POLICY_PATH" >/dev/null 2>&1; then
  policy_ready=true
fi

makepad_pin_ready=false
if [[ "$(rg -c 'makepad-widgets = .*rev = "c4335cee10b22aca768510c9d072b0ca1bba15c8"' "$MANIFEST_PATH")" == "1" \
  && "$(rg -c 'makepad-code-editor = .*rev = "c4335cee10b22aca768510c9d072b0ca1bba15c8"' "$MANIFEST_PATH")" == "1" ]]; then
  makepad_pin_ready=true
fi

android_credential_fail_closed_ready=false
if ruby -e '
    text = File.binread(ARGV.fetch(0))
    support = text[/pub\(super\) const SYSTEM_CREDENTIAL_STORE_SUPPORTED: bool = cfg!\(any\((.*?)\)\);/m, 1]
    abort "missing support contract" unless support
    abort "Android unexpectedly declared supported" if support.include?(%q{target_os = "android"})
    abort "missing fail-closed error" unless text.include?("secure Matrix session persistence is unavailable on this platform; re-login is required")
  ' "$CREDENTIAL_PATH" >/dev/null 2>&1; then
  android_credential_fail_closed_ready=true
fi

testflight_source_contract_ready=false
if ruby -e '
    text = File.binread(ARGV.fetch(0))
    required = [
      %q{APP="${APP:-nativeapp}"},
      %q{CARGO_PACKAGE="${CARGO_PACKAGE:-hepta-native}"},
      %q{PRODUCT_NAME="Hepta"},
      %q{APP_BUNDLE="$BUILD_DIR/${CARGO_PACKAGE}.app"},
      %q{rm -rf "$APP_BUNDLE" "$SCENT"},
      %q{run-device -p "$CARGO_PACKAGE" --locked --release},
      %q{chmod 0755 "$BINARY"},
      %q{scripts/hepta-native-mobile-cargo},
      %q{https://github.com/ProfAlexQI/Hepta/commit/$SOURCE_HEAD},
      %q{set_or_add CFBundleDisplayName string "$PRODUCT_NAME"},
      %q{set_or_add CFBundleName string "$PRODUCT_NAME"},
      %q{[[ "$BUILT_IDENTIFIER" == "ai.hepta.nativeapp" ]]},
      %q{[[ "$BUILT_EXECUTABLE" == "hepta-native" ]]},
      %q{[[ "$BUILT_DISPLAY_NAME" == "$PRODUCT_NAME" ]]},
      %q{[[ "$BUILT_BUNDLE_NAME" == "$PRODUCT_NAME" ]]},
      %q{compiled_asset_catalog_ready:true},
      %q{stale_artifact_accepted:false},
    ]
    abort "missing fail-closed TestFlight contract" unless required.all? { |needle| text.include?(needle) }
    abort "TestFlight script still suppresses a command failure" if text.include?("|| true")
  ' "$TESTFLIGHT_PATH" >/dev/null 2>&1; then
  testflight_source_contract_ready=true
fi

ios_simulator_smoke_source_shape_ready=false
if ruby -e '
    text = File.binread(ARGV.fetch(0))
    required = [
      %q{BUNDLE_IDENTIFIER="ai.hepta.nativeapp"},
      %q{PRODUCT_NAME="Hepta"},
      %q{CARGO_PACKAGE="hepta-native"},
      %q{scripts/hepta-native-mobile-cargo},
      %q{rm -rf "$APP_BUNDLE" "$SCENT"},
      %q{build -p "$CARGO_PACKAGE" --locked --release},
      %q{https://github.com/ProfAlexQI/Hepta/commit/$SOURCE_HEAD},
      %q{compiled_asset_catalog_ready:true},
      %q{xcrun simctl install},
      %q{xcrun simctl launch --terminate-running-process},
      %q{xcrun simctl io},
      %q{scripts/hepta-image-content-probe},
      %q{visual_content:$visual_content},
      %q{signing:{performed:false}},
      %q{ios_real_device_verified:false},
      %q{safe_area_verified:false},
      %q{software_keyboard_verified:false},
      %q{voiceover_verified:false},
      %q{rtl_verified:false},
      %q{dynamic_type_verified:false},
    ]
    abort "missing iOS simulator source contract" unless required.all? { |needle| text.include?(needle) }
    forbidden_commands = [
      /^\s*xcodebuild\s+.*-download/m,
      /^\s*xcrun\s+simctl\s+create\b/m,
      /^\s*codesign\b/m,
      /^\s*security\b/m,
    ]
    abort "iOS simulator script contains a forbidden mutation command" if forbidden_commands.any? { |pattern| text.match?(pattern) }
  ' "$IOS_SIMULATOR_SMOKE_PATH" >/dev/null 2>&1; then
  ios_simulator_smoke_source_shape_ready=true
fi

ios_simulator_smoke_source_contract='{"status":"not_ready"}'
ios_simulator_smoke_source_contract_ready=false
if [[ "$ios_simulator_smoke_source_shape_ready" == true ]] \
  && ios_simulator_smoke_source_contract="$($IOS_SIMULATOR_SMOKE_PATH --contract-only 2>/dev/null)" \
  && jq -e '
    .schema_version == 1
    and .kind == "hepta-native-ios-simulator-smoke-source-contract"
    and .status == "ready"
    and .producer == "scripts/hepta-native-ios-simulator-smoke.sh"
    and .build_wrapper == "scripts/hepta-native-mobile-cargo"
    and .receipt_kind == "hepta-native-ios-simulator-smoke-receipt"
    and .identity.bundle_identifier == "ai.hepta.nativeapp"
    and .identity.display_name == "Hepta"
    and .identity.name == "Hepta"
    and .identity.executable == "hepta-native"
    and (.requirements | to_entries | all(.value == true))
    and (.forbidden_actions | to_entries | all(.value == false))
    and .external_side_effects_performed == false
  ' >/dev/null <<<"$ios_simulator_smoke_source_contract"; then
  ios_simulator_smoke_source_contract_ready=true
fi

android_emulator_smoke_source_shape_ready=false
if ruby -e '
    text = File.binread(ARGV.fetch(0))
    required = [
      %q{PACKAGE_NAME="ai.hepta.nativeapp"},
      %q{APP_LABEL="Hepta"},
      %q{ACTIVITY="$PACKAGE_NAME/.MakepadApp"},
      %q{scripts/hepta-native-mobile-cargo},
      %q{--abi=aarch64},
      %q{build -p "$CARGO_PACKAGE" --locked --release},
      %q{uninstall "$PACKAGE_NAME"},
      %q{install --no-incremental -r "$APK_PATH"},
      %q{shell am start -W -S -n "$ACTIVITY"},
      %q{exec-out screencap -p},
      %q{shell uiautomator dump},
      %q{scripts/hepta-image-content-probe},
      %q{scripts/hepta-android-login-template-probe},
      %q{scripts/hepta-android-window-orientation-probe},
      %q{android-emulator-login-template-v1},
      %q{/proc/sys/kernel/random/boot_id},
      %q{ro.boot.qemu.avd_name},
      %q{session_probe},
      %q{FINAL_QEMU_AVD_NAME},
      %q{FINAL_BOOT_ID},
      %q{process_start_time_ticks},
      %q{source changed during Android emulator smoke},
      %q{accessibility_verified:false},
      %q{talkback_verified:false},
      %q{real_device_verified:false},
      %q{secure_credential_backend_verified:false},
      %q{release_signed:false},
      %q{public_distribution_ready:false},
    ]
    abort "missing Android emulator producer contract" unless required.all? { |needle| text.include?(needle) }
    forbidden_commands = [
      /^\s*avdmanager\s+create\b/m,
      /^\s*emulator\s+.*(?:-avd|@)/m,
      /^\s*sdkmanager\b/m,
      /^\s*codesign\b/m,
      /^\s*security\b/m,
    ]
    abort "Android producer contains a forbidden mutation command" if forbidden_commands.any? { |pattern| text.match?(pattern) }
  ' "$ANDROID_EMULATOR_SMOKE_PATH" >/dev/null 2>&1; then
  android_emulator_smoke_source_shape_ready=true
fi

android_emulator_live_readback_source_contract_ready=false
if bash -n "$ANDROID_EMULATOR_LIVE_READBACK_PATH" "$ANDROID_TRUSTED_ADB_PATH" \
  && ruby -e '
    text = File.binread(ARGV.fetch(0))
    required = [
      %q{receipt serial is not an emulator serial},
      %q{scripts/hepta-android-trusted-adb},
      %q{caller_routing_environment_trusted:false},
      %q{all_transports_enumerated:false},
      %q{/proc/sys/kernel/random/boot_id},
      %q{ro.boot.qemu.avd_name},
      %q{exec-out cat "$LIVE_PACKAGE_PATH"},
      %q{session nonce is absent or changed},
      %q{shell am start -W -n ai.hepta.nativeapp/.MakepadApp},
      %q{PRE_PROCESS_START_TICKS},
      %q{POST_PROCESS_START_TICKS},
      %q{credentials_supplied:false},
      %q{unauthenticated_login_surface_rotation:false},
      %q{unauthenticated_login_surface_ime:false},
      %q{real_device_contacted:false},
    ]
    abort "missing fail-closed Android live readback contract" unless required.all? { |needle| text.include?(needle) }
  ' "$ANDROID_EMULATOR_LIVE_READBACK_PATH" >/dev/null 2>&1 \
  && ruby -e '
    text = File.binread(ARGV.fetch(0))
    required = [
      %q{Etc.getpwuid(Process.uid).dir},
      %q{PATH=/usr/bin:/bin:/usr/sbin:/sbin},
      %q{codesign --verify --strict},
      %q{anchor apple generic},
      %q{EQHXZ8M8AV},
      %q{Identifier=adb},
      %q{caller_environment_trusted:false},
    ]
    abort "missing canonical signed adb trust contract" unless required.all? { |needle| text.include?(needle) }
    abort "trusted adb resolver consumes caller SDK environment" if text.match?(/\$\{?ANDROID_(?:SDK_ROOT|HOME)/)
  ' "$ANDROID_TRUSTED_ADB_PATH" >/dev/null 2>&1 \
  && ruby -c "$ANDROID_ORIENTATION_PROBE_PATH" >/dev/null 2>&1; then
  android_emulator_live_readback_source_contract_ready=true
fi

android_login_template_contract_ready=false
android_login_template_manifest_sha256=""
if jq -L "$ROOT_DIR/scripts/lib" -e '
    include "hepta-native-android-login-template-v1";
    hepta_android_login_template_v1_ready
  ' "$ANDROID_LOGIN_TEMPLATE_MANIFEST_PATH" >/dev/null 2>&1 \
  && [[ "$(shasum -a 256 "$(jq -r '.templates.portrait.path' "$ANDROID_LOGIN_TEMPLATE_MANIFEST_PATH")" | awk '{print $1}')" == "$(jq -r '.templates.portrait.sha256' "$ANDROID_LOGIN_TEMPLATE_MANIFEST_PATH")" ]] \
  && [[ "$(shasum -a 256 "$(jq -r '.templates.landscape.path' "$ANDROID_LOGIN_TEMPLATE_MANIFEST_PATH")" | awk '{print $1}')" == "$(jq -r '.templates.landscape.sha256' "$ANDROID_LOGIN_TEMPLATE_MANIFEST_PATH")" ]] \
  && [[ "$(shasum -a 256 "$(jq -r '.templates.ime.path' "$ANDROID_LOGIN_TEMPLATE_MANIFEST_PATH")" | awk '{print $1}')" == "$(jq -r '.templates.ime.sha256' "$ANDROID_LOGIN_TEMPLATE_MANIFEST_PATH")" ]] \
  && ruby -e '
    text = File.binread(ARGV.fetch(0))
    required = ["normalized_rgb_luma_edge_and_color_mask_v1", "sign_in_to_hepta_title", "homeserver_input_focused", "same_login_form_above_ime", "ime_keyboard", "dark_text_and_provider_geometry"]
    abort unless required.all? { |needle| text.include?(needle) }
  ' "$ANDROID_LOGIN_TEMPLATE_PROBE_PATH" >/dev/null 2>&1; then
  android_login_template_contract_ready=true
  android_login_template_manifest_sha256="$(shasum -a 256 "$ANDROID_LOGIN_TEMPLATE_MANIFEST_PATH" | awk '{print $1}')"
fi

android_emulator_smoke_source_contract='{"status":"not_ready"}'
android_emulator_smoke_source_contract_ready=false
if [[ "$android_emulator_smoke_source_shape_ready" == true ]] \
  && android_emulator_smoke_source_contract="$($ANDROID_EMULATOR_SMOKE_PATH --contract-only 2>/dev/null)" \
  && jq -e '
    .schema_version == 1
    and .kind == "hepta-native-android-emulator-smoke-source-contract"
    and .status == "ready"
    and .producer == "scripts/hepta-native-android-emulator-smoke.sh"
    and .receipt.schema_version == 3
    and .receipt.kind == "hepta-native-android-emulator-smoke-receipt"
    and .identity.package == "ai.hepta.nativeapp"
    and .identity.activity == "ai.hepta.nativeapp/.MakepadApp"
    and .identity.label == "Hepta"
    and .identity.abi == "arm64-v8a"
    and .build_wrapper == "scripts/hepta-native-mobile-cargo"
    and (.requirements | to_entries | all(.value == true))
    and (.hard_boundaries | to_entries | all(.value == false))
    and (.forbidden_actions | to_entries | all(.value == false))
    and .external_side_effects_performed == false
  ' >/dev/null <<<"$android_emulator_smoke_source_contract"; then
  android_emulator_smoke_source_contract_ready=true
fi

toolchain_report='{"status":"not_ready"}'
toolchain_wrapper_ready=false
if toolchain_report="$(scripts/hepta-native-mobile-cargo --print-toolchain-contract 2>/dev/null)" \
  && jq -e '
    .schema_version == 1
    and .kind == "hepta-native-mobile-cargo-toolchain-contract"
    and .status == "ready"
    and .cargo_makepad_requested_channel == "stable"
    and .resolved_toolchain == "1.95.0"
    and (.rustc | startswith("rustc 1.95.0 "))
    and .cargo_makepad.repository == "https://github.com/kevinaboos/makepad.git"
    and .cargo_makepad.revision == "c4335cee10b22aca768510c9d072b0ca1bba15c8"
    and (.cargo_makepad.binary_sha256 | test("^[0-9a-f]{64}$"))
    and .cargo_makepad.exact_revision_source_marker_ready == true
    and .cargo_makepad.custom_android_manifest_help_contract_ready == true
    and .cargo_makepad.global_cargo_makepad_used == false
    and .user_global_stable_mutated == false
  ' >/dev/null <<<"$toolchain_report"; then
  toolchain_wrapper_ready=true
fi

icon_report='{"status":"not_ready"}'
ios_icon_contract_ready=false
if icon_report="$(scripts/hepta-native-ios-icons verify 2>/dev/null)" \
  && jq -e '
    .schema_version == 1
    and .kind == "hepta-native-ios-icon-contract"
    and .status == "ready"
    and .canonical.path == "apps/hepta-native/resources/icon_1024.png"
    and .canonical.pixels == 1024
    and .canonical.png_color_type == 2
    and .canonical.alpha == false
    and .app_store_marketing_icon_opaque == true
    and (.generated | length == 5)
    and (.generated | all(.png_color_type == 2 and .alpha == false and .canonical_source == "apps/hepta-native/resources/icon_1024.png"))
  ' >/dev/null <<<"$icon_report"; then
  ios_icon_contract_ready=true
fi

installed_targets="$(rustup target list --toolchain 1.95.0 --installed 2>/dev/null || true)"
ios_toolchain_targets_ready=false
if grep -Fxq aarch64-apple-ios <<<"$installed_targets" \
  && grep -Fxq aarch64-apple-ios-sim <<<"$installed_targets"; then
  ios_toolchain_targets_ready=true
fi
android_toolchain_target_ready=false
if grep -Fxq aarch64-linux-android <<<"$installed_targets"; then android_toolchain_target_ready=true; fi

ios_distribution_identity_available=false
ios_distribution_identity_count=0
if command -v security >/dev/null 2>&1; then
  ios_distribution_identity_count="$(security find-identity -v -p codesigning 2>/dev/null | awk '/Apple Distribution/ {count++} END {print count+0}')"
  [[ "$ios_distribution_identity_count" -gt 0 ]] && ios_distribution_identity_available=true
fi

SOURCE_AFTER="$(scripts/hepta-ui-source-fingerprint)"
source_stable=false
if [[ "$(jq -r '.head' <<<"$SOURCE_BEFORE")" == "$(jq -r '.head' <<<"$SOURCE_AFTER")" \
  && "$(jq -r '.head_tree' <<<"$SOURCE_BEFORE")" == "$(jq -r '.head_tree' <<<"$SOURCE_AFTER")" \
  && "$(jq -r '.source_fingerprint' <<<"$SOURCE_BEFORE")" == "$(jq -r '.source_fingerprint' <<<"$SOURCE_AFTER")" ]]; then
  source_stable=true
fi

ios_simulator_receipt_supplied=false
ios_simulator_receipt_ready=false
ios_simulator_receipt_status="missing"
ios_simulator_receipt_summary="$(jq -n \
  --arg path "$IOS_SIMULATOR_RECEIPT" \
  '{supplied:false,path:$path,status:"missing",ready:false}')"
android_emulator_receipt_supplied=false
android_emulator_receipt_ready=false
android_emulator_receipt_status="missing"
android_emulator_live_readback_performed=false
android_emulator_live_readback_ready=false
android_emulator_live_readback_report='null'
android_emulator_login_visual_ready=false
android_emulator_login_rotation_ready=false
android_emulator_login_ime_ready=false
android_emulator_receipt_summary="$(jq -n \
  --arg path "$ANDROID_EMULATOR_RECEIPT" \
  --argjson live_opt_in "$ANDROID_EMULATOR_LIVE_READBACK_OPT_IN" \
  '{supplied:false,path:$path,status:"missing",ready:false,scope:null,live_readback:{opt_in:($live_opt_in == 1),performed:false,ready:false},claims:{runtime:false,unauthenticated_login_surface_visual:false,unauthenticated_login_surface_rotation:false,unauthenticated_login_surface_ime:false,visual:false,rotation:false,ime:false},deprecated_generic_claims_hard_false:true}')"

verify_ios_simulator_artifact() {
  local receipt="$1" archive="$2" extract_root app_bundle_count app_bundle plist binary mode evidence_path evidence_sha
  # Reject absolute paths, parent traversal, backslashes, and control bytes
  # before extraction. The smoke producer emits only portable slash-separated
  # app-bundle members, so these shapes are never needed by valid evidence.
  if ! unzip -Z -1 "$archive" 2>/dev/null | ruby -e '
      entries = STDIN.each_line.map(&:chomp)
      abort if entries.empty?
      entries.each do |entry|
        abort if entry.empty? || entry.start_with?("/") || entry.include?("\\")
        abort if entry.bytes.any? { |byte| byte < 0x20 || byte == 0x7f }
        abort if entry.split("/").include?("..")
      end
    '; then
    return 1
  fi

  extract_root="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ios-sim-receipt.XXXXXX")"
  if ! ditto -x -k "$archive" "$extract_root" >/dev/null 2>&1; then
    rm -rf "$extract_root"
    return 1
  fi
  # No archive member may redirect later hash/metadata reads outside the
  # extraction root. This is checked before discovering or opening the app.
  if [[ -n "$(find "$extract_root" -type l -print -quit)" ]]; then
    rm -rf "$extract_root"
    return 1
  fi
  app_bundle_count="$(find "$extract_root" -maxdepth 2 -type d -name 'hepta-native.app' | wc -l | tr -d '[:space:]')"
  [[ "$app_bundle_count" == "1" ]] || { rm -rf "$extract_root"; return 1; }
  app_bundle="$(find "$extract_root" -maxdepth 2 -type d -name 'hepta-native.app' -print -quit)"
  plist="$app_bundle/Info.plist"
  binary="$app_bundle/hepta-native"
  if [[ ! -s "$plist" || ! -x "$binary" ]] \
    || ! plutil -lint "$plist" >/dev/null 2>&1 \
    || [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$plist" 2>/dev/null)" != "ai.hepta.nativeapp" ]] \
    || [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleDisplayName' "$plist" 2>/dev/null)" != "Hepta" ]] \
    || [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleName' "$plist" 2>/dev/null)" != "Hepta" ]] \
    || [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$plist" 2>/dev/null)" != "hepta-native" ]] \
    || ! strings "$binary" | grep -F "https://github.com/ProfAlexQI/Hepta/commit/$(jq -r '.source_binding.head' "$receipt")" >/dev/null; then
    rm -rf "$extract_root"
    return 1
  fi

  mode="$(jq -r '.asset_catalog.mode' "$receipt")"
  evidence_path="$(jq -r '.asset_catalog.evidence.path' "$receipt")"
  evidence_sha="$(jq -r '.asset_catalog.evidence.sha256' "$receipt")"
  if [[ ! -s "$app_bundle/$evidence_path" \
    || "$(shasum -a 256 "$app_bundle/$evidence_path" | awk '{print $1}')" != "$evidence_sha" ]]; then
    rm -rf "$extract_root"
    return 1
  fi
  if [[ "$mode" == "actool_info_and_opaque_icon_outputs" ]]; then
    if ! plutil -lint "$app_bundle/actool-Info.plist" >/dev/null 2>&1 \
      || [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIcons:CFBundlePrimaryIcon:CFBundleIconName' "$app_bundle/actool-Info.plist" 2>/dev/null)" != "AppIcon" ]] \
      || [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIcons~ipad:CFBundlePrimaryIcon:CFBundleIconName' "$app_bundle/actool-Info.plist" 2>/dev/null)" != "AppIcon" ]] \
      || [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIcons:CFBundlePrimaryIcon:CFBundleIconName' "$plist" 2>/dev/null)" != "AppIcon" ]] \
      || [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIcons~ipad:CFBundlePrimaryIcon:CFBundleIconName' "$plist" 2>/dev/null)" != "AppIcon" ]]; then
      rm -rf "$extract_root"
      return 1
    fi
    while IFS=$'\t' read -r icon_path icon_sha icon_width icon_height; do
      if [[ ! -s "$app_bundle/$icon_path" \
        || "$(shasum -a 256 "$app_bundle/$icon_path" | awk '{print $1}')" != "$icon_sha" \
        || "$(sips -g pixelWidth "$app_bundle/$icon_path" 2>/dev/null | awk '/pixelWidth:/ {print $2}')" != "$icon_width" \
        || "$(sips -g pixelHeight "$app_bundle/$icon_path" 2>/dev/null | awk '/pixelHeight:/ {print $2}')" != "$icon_height" \
        || "$(sips -g hasAlpha "$app_bundle/$icon_path" 2>/dev/null | awk '/hasAlpha:/ {print $2}')" != "no" ]]; then
        rm -rf "$extract_root"
        return 1
      fi
    done < <(jq -r '.asset_catalog.icon_outputs[] | [.path,.sha256,(.width|tostring),(.height|tostring)] | @tsv' "$receipt")
  fi
  rm -rf "$extract_root"
}

safe_absolute_regular_file() {
  ruby -rpathname -e '
    path = ARGV.fetch(0)
    abort unless path.start_with?("/")
    abort unless path.bytes.none? { |byte| byte < 0x20 || byte == 0x7f }
    abort if path.include?("\\")
    abort unless Pathname.new(path).cleanpath.to_s == path
    stat = File.lstat(path)
    abort unless stat.file? && !stat.symlink?
    abort unless File.realpath(path) == path
  ' "$1" >/dev/null 2>&1
}

android_build_tool() {
  local tool="$1" sdk_root candidate
  sdk_root="$(env -i PATH=/usr/bin:/bin:/usr/sbin:/sbin "$ANDROID_TRUSTED_ADB_PATH" | jq -er 'select(.ready == true) | .sdk_root')" || return 1
  [[ -d "$sdk_root/build-tools" ]] || return 1
  candidate="$(find "$sdk_root/build-tools" -mindepth 2 -maxdepth 2 -type f -name "$tool" -perm -111 -print 2>/dev/null \
    | ruby -e 'paths = STDIN.each_line.map(&:chomp); puts(paths.max_by { |path| File.basename(File.dirname(path)).split(/[^0-9]+/).map(&:to_i) }) unless paths.empty?')"
  [[ -n "$candidate" ]] || return 1
  printf '%s\n' "$candidate"
}

verify_android_emulator_artifact() {
  local receipt="$1" apk="$2" extract_root lib_path head aapt apksigner badging signing cert_sha version_code version_name min_sdk target_sdk
  safe_absolute_regular_file "$apk" || return 1
  ruby -e 'abort unless File.binread(ARGV.fetch(0), 4).start_with?("PK")' "$apk" >/dev/null 2>&1 || return 1
  unzip -t "$apk" >/dev/null 2>&1 || return 1
  if ! unzip -Z -1 "$apk" 2>/dev/null | ruby -e '
      entries = STDIN.each_line.map(&:chomp)
      abort if entries.empty?
      entries.each do |entry|
        abort if entry.empty? || entry.start_with?("/") || entry.include?("\\")
        abort if entry.bytes.any? { |byte| byte < 0x20 || byte == 0x7f }
        abort if entry.split("/").include?("..")
      end
      abort unless entries.count("lib/arm64-v8a/libmakepad.so") == 1
      abort unless entries.grep(%r{\Alib/[^/]+/libmakepad\.so\z}) == ["lib/arm64-v8a/libmakepad.so"]
    '; then
    return 1
  fi

  extract_root="$(mktemp -d "${TMPDIR:-/tmp}/hepta-android-emulator-receipt.XXXXXX")"
  if ! unzip -qq "$apk" -d "$extract_root" >/dev/null 2>&1 \
    || [[ -n "$(find "$extract_root" -type l -print -quit)" ]]; then
    rm -rf "$extract_root"
    return 1
  fi
  lib_path="$extract_root/lib/arm64-v8a/libmakepad.so"
  head="$(jq -r '.source_binding.head' "$receipt")"
  if [[ ! -s "$lib_path" ]] \
    || ! strings "$lib_path" | grep -F "https://github.com/ProfAlexQI/Hepta/commit/$head" >/dev/null; then
    rm -rf "$extract_root"
    return 1
  fi
  rm -rf "$extract_root"

  aapt="$(android_build_tool aapt)" || return 1
  apksigner="$(android_build_tool apksigner)" || return 1
  badging="$($aapt dump badging "$apk" 2>/dev/null)" || return 1
  version_code="$(jq -r '.artifact.version_code' "$receipt")"
  version_name="$(jq -r '.artifact.version_name' "$receipt")"
  min_sdk="$(jq -r '.artifact.min_sdk' "$receipt")"
  target_sdk="$(jq -r '.artifact.target_sdk' "$receipt")"
  grep -Fq "package: name='ai.hepta.nativeapp' versionCode='$version_code' versionName='$version_name' " <<<"$badging" || return 1
  grep -Fxq "sdkVersion:'$min_sdk'" <<<"$badging" || return 1
  grep -Fxq "targetSdkVersion:'$target_sdk'" <<<"$badging" || return 1
  grep -Fxq "application-label:'Hepta'" <<<"$badging" || return 1
  grep -Eq "^launchable-activity: name='ai\.hepta\.nativeapp\.MakepadApp' " <<<"$badging" || return 1
  grep -Fxq "native-code: 'arm64-v8a'" <<<"$badging" || return 1

  signing="$($apksigner verify --verbose --print-certs "$apk" 2>/dev/null)" || return 1
  cert_sha="$(jq -r '.signing.certificate_sha256' "$receipt")"
  grep -Fxq "Verifies" <<<"$signing" || return 1
  grep -Fxq "Verified using v2 scheme (APK Signature Scheme v2): true" <<<"$signing" || return 1
  grep -Fxq "Verified using v3 scheme (APK Signature Scheme v3): true" <<<"$signing" || return 1
  grep -Fxq "Number of signers: 1" <<<"$signing" || return 1
  grep -Eq '^Signer #1 certificate DN: .*CN=Android Debug(,|$)' <<<"$signing" || return 1
  grep -Fxq "Signer #1 certificate SHA-256 digest: $cert_sha" <<<"$signing" || return 1
}

verify_android_emulator_host_tools() {
  local receipt="$1" sdk_root emulator qemu emulator_sha qemu_sha
  sdk_root="$(env -i PATH=/usr/bin:/bin:/usr/sbin:/sbin "$ANDROID_TRUSTED_ADB_PATH" | jq -er 'select(.ready == true) | .sdk_root')" || return 1
  emulator="$sdk_root/emulator/emulator"
  qemu="$sdk_root/emulator/qemu/darwin-aarch64/qemu-system-aarch64-headless"
  safe_absolute_regular_file "$emulator" || return 1
  safe_absolute_regular_file "$qemu" || return 1
  file "$emulator" | grep -Eq ': Mach-O 64-bit executable arm64$' || return 1
  file "$qemu" | grep -Eq ': Mach-O 64-bit executable arm64$' || return 1
  emulator_sha="$(jq -r '.host_toolchain.emulator_binary_sha256' "$receipt")"
  qemu_sha="$(jq -r '.host_toolchain.qemu_binary_sha256' "$receipt")"
  [[ "$(shasum -a 256 "$emulator" | awk '{print $1}')" == "$emulator_sha" ]] || return 1
  [[ "$(shasum -a 256 "$qemu" | awk '{print $1}')" == "$qemu_sha" ]] || return 1
}

verify_android_emulator_screenshot() {
  local receipt="$1" key="$2" path sha width height probe
  path="$(jq -r --arg key "$key" '.visual_inspection[$key].path' "$receipt")"
  sha="$(jq -r --arg key "$key" '.visual_inspection[$key].sha256' "$receipt")"
  width="$(jq -r --arg key "$key" '.visual_inspection[$key].width' "$receipt")"
  height="$(jq -r --arg key "$key" '.visual_inspection[$key].height' "$receipt")"
  safe_absolute_regular_file "$path" || return 1
  [[ "$(shasum -a 256 "$path" | awk '{print $1}')" == "$sha" ]] || return 1
  ruby -e 'abort unless File.binread(ARGV.fetch(0), 8) == "\x89PNG\r\n\x1a\n".b' "$path" >/dev/null 2>&1 || return 1
  [[ "$(sips -g pixelWidth "$path" 2>/dev/null | awk '/pixelWidth:/ {print $2}')" == "$width" ]] || return 1
  [[ "$(sips -g pixelHeight "$path" 2>/dev/null | awk '/pixelHeight:/ {print $2}')" == "$height" ]] || return 1
  probe="$(scripts/hepta-image-content-probe --image "$path" 2>/dev/null)" || return 1
  jq -e --arg sha "$sha" --argjson width "$width" --argjson height "$height" '
    .schema_version == 1
    and .kind == "hepta-image-content-probe"
    and .status == "ready"
    and .ready == true
    and .image.sha256 == $sha
    and .image.width == $width
    and .image.height == $height
    and .sample.non_black_ratio >= .thresholds.min_non_black_ratio
    and .sample.luma_span >= .thresholds.min_luma_span
    and .sample.luma_bucket_count >= .thresholds.min_luma_buckets
  ' >/dev/null <<<"$probe"
}

verify_android_emulator_uiautomator() {
  local receipt="$1" path sha expected_nodes expected_visible expected_package expected_labeled actual
  path="$(jq -r '.uiautomator.path' "$receipt")"
  sha="$(jq -r '.uiautomator.sha256' "$receipt")"
  expected_nodes="$(jq -r '.uiautomator.node_count' "$receipt")"
  expected_visible="$(jq -r '.uiautomator.visible_node_count' "$receipt")"
  expected_package="$(jq -r '.uiautomator.package_node_count' "$receipt")"
  expected_labeled="$(jq -r '.uiautomator.labeled_node_count' "$receipt")"
  safe_absolute_regular_file "$path" || return 1
  [[ "$sha" =~ ^[0-9a-f]{64}$ ]] || return 1
  [[ "$(shasum -a 256 "$path" | awk '{print $1}')" == "$sha" ]] || return 1
  xmllint --noout "$path" >/dev/null 2>&1 || return 1
  actual="$(ruby -rrexml/document -e '
    doc = REXML::Document.new(File.binread(ARGV.fetch(0)))
    nodes = []
    REXML::XPath.each(doc, "//node") { |node| nodes << node }
    visible = nodes.count do |node|
      match = node.attributes["bounds"].to_s.match(/\[(\d+),(\d+)\]\[(\d+),(\d+)\]/)
      match && match[3].to_i > match[1].to_i && match[4].to_i > match[2].to_i
    end
    package_nodes = nodes.count { |node| node.attributes["package"] == "ai.hepta.nativeapp" }
    labeled = nodes.count { |node| [node.attributes["text"], node.attributes["content-desc"]].any? { |value| !value.to_s.strip.empty? } }
    puts [nodes.length, visible, package_nodes, labeled].join(" ")
  ' "$path")" || return 1
  [[ "$actual" == "$expected_nodes $expected_visible $expected_package $expected_labeled" ]]
}

verify_android_login_template() {
  local receipt="$1" key="$2" mode="$3" image template evidence_path expected_template expected_sha replay embedded persisted
  image="$(jq -r --arg key "$key" '.visual_inspection[$key].path' "$receipt")"
  template="$(jq -r --arg key "$key" '.visual_inspection[$key].login_template_probe.template.path' "$receipt")"
  evidence_path="$(jq -r --arg key "$key" '.visual_inspection[$key].login_template_probe.evidence_path' "$receipt")"
  expected_template="$ROOT_DIR/$(jq -r --arg key "$key" '.templates[$key].path' "$ANDROID_LOGIN_TEMPLATE_MANIFEST_PATH")"
  expected_sha="$(jq -r --arg key "$key" '.templates[$key].sha256' "$ANDROID_LOGIN_TEMPLATE_MANIFEST_PATH")"
  [[ "$template" == "$expected_template" ]] || return 1
  [[ "$(shasum -a 256 "$template" | awk '{print $1}')" == "$expected_sha" ]] || return 1
  safe_absolute_regular_file "$evidence_path" || return 1
  jq -e \
    --arg mode "$mode" \
    --arg image "$(jq -r --arg key "$key" '.visual_inspection[$key].sha256' "$receipt")" \
    --arg template "$expected_sha" '
      .schema_version == 1
      and .kind == "hepta-android-login-template-probe"
      and .status == "ready"
      and .ready == true
      and .mode == $mode
      and .image.sha256 == $image
      and .template.sha256 == $template
      and .algorithm == "normalized_rgb_luma_edge_and_color_mask_v1"
      and (.detections | length >= 12)
      and (.detections | all(.ready == true))
      and (.detections | any(.name == "sign_in_to_hepta_title" and .ready == true))
      and (.detections | any((.name == "homeserver_input" or .name == "homeserver_input_focused") and .ready == true))
      and (if $mode == "ime" then
        (.detections | any(.name == "same_login_form_above_ime" and .ready == true))
        and (.detections | any(.name == "ime_keyboard" and .ready == true))
      else true end)
    ' "$evidence_path" >/dev/null || return 1
  embedded="$(jq -S -c --arg key "$key" '.visual_inspection[$key].login_template_probe | del(.evidence_path)' "$receipt")"
  persisted="$(jq -S -c . "$evidence_path")"
  [[ "$embedded" == "$persisted" ]] || return 1
  replay="$($ANDROID_LOGIN_TEMPLATE_PROBE_PATH --image "$image" --template "$template" --mode "$mode" 2>/dev/null)" || return 1
  jq -e --arg expected_sha "$expected_sha" '
    .status == "ready"
    and .ready == true
    and .template.sha256 == $expected_sha
    and (.detections | all(.ready == true))
  ' >/dev/null <<<"$replay"
}

if [[ -n "$IOS_SIMULATOR_RECEIPT" ]]; then
  ios_simulator_receipt_supplied=true
  ios_simulator_receipt_status="invalid"
  if [[ -s "$IOS_SIMULATOR_RECEIPT" ]] \
    && jq -e \
      --arg head "$(jq -r '.head' <<<"$SOURCE_AFTER")" \
      --arg tree "$(jq -r '.head_tree' <<<"$SOURCE_AFTER")" \
      --arg fingerprint "$(jq -r '.source_fingerprint' <<<"$SOURCE_AFTER")" '
        .schema_version == 1
        and .kind == "hepta-native-ios-simulator-smoke-receipt"
        and .producer == "scripts/hepta-native-ios-simulator-smoke.sh"
        and .status == "ready"
        and .ready == true
        and .source_binding.head == $head
        and .source_binding.head_tree == $tree
        and .source_binding.source_fingerprint == $fingerprint
        and .source_binding.worktree_clean == true
        and .source_binding.repository_worktree_clean == true
        and .device.state == "Booted"
        and .device.is_available == true
        and (.device.udid | type == "string" and length > 0)
        and .artifact.format == "zip"
        and .artifact.stale_artifact_accepted == false
        and (.artifact.path | type == "string" and startswith("/"))
        and (.artifact.sha256 | test("^[0-9a-f]{64}$"))
        and .screenshot.format == "png"
        and (.screenshot.path | type == "string" and startswith("/"))
        and (.screenshot.sha256 | test("^[0-9a-f]{64}$"))
        and .screenshot.width > 0
        and .screenshot.height > 0
        and .visual_content.schema_version == 1
        and .visual_content.kind == "hepta-image-content-probe"
        and .visual_content.status == "ready"
        and .visual_content.ready == true
        and .visual_content.image.sha256 == .screenshot.sha256
        and .visual_content.image.width == .screenshot.width
        and .visual_content.image.height == .screenshot.height
        and .visual_content.sample.non_black_ratio >= .visual_content.thresholds.min_non_black_ratio
        and .visual_content.sample.luma_span >= .visual_content.thresholds.min_luma_span
        and .visual_content.sample.luma_bucket_count >= .visual_content.thresholds.min_luma_buckets
        and (.visual_content.capture_attempts >= 1 and .visual_content.capture_attempts <= 30)
        and .bundle.identifier == "ai.hepta.nativeapp"
        and .bundle.display_name == "Hepta"
        and .bundle.name == "Hepta"
        and .bundle.executable == "hepta-native"
        and .asset_catalog.compiled_asset_catalog_ready == true
        and (.asset_catalog.evidence.sha256 | test("^[0-9a-f]{64}$"))
        and (
          if .asset_catalog.mode == "assets_car" then
            .asset_catalog.evidence.path == "Assets.car"
            and ((.asset_catalog.icon_outputs // []) | length == 0)
          elif .asset_catalog.mode == "actool_info_and_opaque_icon_outputs" then
            .asset_catalog.evidence.path == "actool-Info.plist"
            and (.asset_catalog.icon_outputs | length == 4)
            and (.asset_catalog.icon_outputs | map(.path) | unique | length == 4)
            and (.asset_catalog.icon_outputs | map({path,width,height,alpha}) | sort_by(.path)) == ([
              {path:"AppIcon60x60@2x.png",width:120,height:120,alpha:false},
              {path:"AppIcon60x60@3x.png",width:180,height:180,alpha:false},
              {path:"AppIcon76x76@2x~ipad.png",width:152,height:152,alpha:false},
              {path:"AppIcon83.5x83.5@2x~ipad.png",width:167,height:167,alpha:false}
            ] | sort_by(.path))
            and (.asset_catalog.icon_outputs | all(.sha256 | test("^[0-9a-f]{64}$")))
          else false end
        )
        and .launch.ready == true
        and .launch.install_succeeded == true
        and .launch.launch_succeeded == true
        and .launch.pid > 0
        and (.launch.app_container | type == "string" and length > 0)
        and .signing.performed == false
        and (.forbidden_actions_performed | to_entries | all(.value == false))
        and (.hard_boundaries | to_entries | all(.value == false))
        and .toolchain.status == "ready"
        and .toolchain.resolved_toolchain == "1.95.0"
        and .toolchain.cargo_makepad.revision == "c4335cee10b22aca768510c9d072b0ca1bba15c8"
      ' "$IOS_SIMULATOR_RECEIPT" >/dev/null 2>&1; then
    receipt_artifact_path="$(jq -r '.artifact.path' "$IOS_SIMULATOR_RECEIPT")"
    receipt_artifact_sha256="$(jq -r '.artifact.sha256' "$IOS_SIMULATOR_RECEIPT")"
    receipt_screenshot_path="$(jq -r '.screenshot.path' "$IOS_SIMULATOR_RECEIPT")"
    receipt_screenshot_sha256="$(jq -r '.screenshot.sha256' "$IOS_SIMULATOR_RECEIPT")"
    receipt_visual_probe="$(scripts/hepta-image-content-probe --image "$receipt_screenshot_path" 2>/dev/null || true)"
    if [[ -s "$receipt_artifact_path" && -s "$receipt_screenshot_path" \
      && "$(shasum -a 256 "$receipt_artifact_path" | awk '{print $1}')" == "$receipt_artifact_sha256" \
      && "$(shasum -a 256 "$receipt_screenshot_path" | awk '{print $1}')" == "$receipt_screenshot_sha256" ]] \
      && ruby -e 'abort unless File.binread(ARGV.fetch(0), 4).start_with?("PK")' "$receipt_artifact_path" >/dev/null 2>&1 \
      && ruby -e 'abort unless File.binread(ARGV.fetch(0), 8) == "\x89PNG\r\n\x1a\n".b' "$receipt_screenshot_path" >/dev/null 2>&1 \
      && jq -e --arg sha256 "$receipt_screenshot_sha256" '
        .schema_version == 1
        and .kind == "hepta-image-content-probe"
        and .status == "ready"
        and .ready == true
        and .image.sha256 == $sha256
      ' >/dev/null <<<"$receipt_visual_probe" \
      && verify_ios_simulator_artifact "$IOS_SIMULATOR_RECEIPT" "$receipt_artifact_path"; then
      ios_simulator_receipt_ready=true
      ios_simulator_receipt_status="ready"
    fi
  fi
  ios_simulator_receipt_summary="$(jq -n \
    --arg path "$IOS_SIMULATOR_RECEIPT" \
    --arg status "$ios_simulator_receipt_status" \
    --argjson ready "$ios_simulator_receipt_ready" \
    '{supplied:true,path:$path,status:$status,ready:$ready}')"
fi

if [[ -n "$ANDROID_EMULATOR_RECEIPT" ]]; then
  android_emulator_receipt_supplied=true
  android_emulator_receipt_status="invalid"
  if [[ -s "$ANDROID_EMULATOR_RECEIPT" ]] \
    && [[ "$(jq -r '.worktree_clean' <<<"$SOURCE_AFTER")" == true ]] \
    && [[ "$(jq -r '.repository_worktree_clean' <<<"$SOURCE_AFTER")" == true ]] \
    && jq -L "$ROOT_DIR/scripts/lib" -e \
      --arg head "$(jq -r '.head' <<<"$SOURCE_AFTER")" \
      --arg tree "$(jq -r '.head_tree' <<<"$SOURCE_AFTER")" \
      --arg fingerprint "$(jq -r '.source_fingerprint' <<<"$SOURCE_AFTER")" \
      --arg login_manifest "$ROOT_DIR/$ANDROID_LOGIN_TEMPLATE_MANIFEST_PATH" \
      --arg login_manifest_sha "$android_login_template_manifest_sha256" '
        include "hepta-native-android-emulator-receipt-v3";
        hepta_android_emulator_receipt_v3_ready($head; $tree; $fingerprint; $login_manifest; $login_manifest_sha)
        and .schema_version == 3
        and .kind == "hepta-native-android-emulator-smoke-receipt"
        and .producer == "scripts/hepta-native-android-emulator-smoke.sh"
        and .scope == "unauthenticated_android_login_surface_on_arm64_emulator"
        and .status == "ready"
        and .ready == true
        and .source_binding.head == $head
        and .source_binding.head_tree == $tree
        and .source_binding.source_fingerprint == $fingerprint
        and .source_binding.worktree_clean == true
        and .source_binding.repository_worktree_clean == true
        and .artifact.format == "apk"
        and .artifact.stale_artifact_accepted == false
        and (.artifact.path | type == "string" and startswith("/") and (contains("/../") | not))
        and (.artifact.size_bytes | type == "number" and . > 0)
        and (.artifact.sha256 | test("^[0-9a-f]{64}$"))
        and .artifact.package == "ai.hepta.nativeapp"
        and .artifact.activity == "ai.hepta.nativeapp/.MakepadApp"
        and .artifact.label == "Hepta"
        and (.artifact.version_code | type == "number" and . == floor and . > 0)
        and (.artifact.version_name | test("^[0-9A-Za-z][0-9A-Za-z._+-]{0,63}$"))
        and .artifact.min_sdk == 26
        and .artifact.target_sdk == 35
        and .artifact.primary_cpu_abi == "arm64-v8a"
        and .artifact.install_result == "Success"
        and .artifact.install_success == true
        and .artifact.full_head_embedded == true
        and .artifact.artifact_source_bound == true
        and .artifact.application_debuggable == false
        and .artifact.manifest_contract.status == "ready"
        and .artifact.manifest_contract.ready == true
        and .signing.kind == "android_debug"
        and .signing.verified == true
        and (.signing.certificate_dn | type == "string" and contains("CN=Android Debug"))
        and (.signing.certificate_sha256 | test("^[0-9a-f]{64}$"))
        and .signing.v2 == true
        and .signing.v3 == true
        and .signing.release_signed == false
        and .host_toolchain.host_architecture == "arm64"
        and (.host_toolchain.adb_binary_path | type == "string" and startswith("/") and endswith("/platform-tools/adb"))
        and (.host_toolchain.adb_binary_sha256 | test("^[0-9a-f]{64}$"))
        and .host_toolchain.emulator_binary_architecture == "arm64"
        and (.host_toolchain.emulator_binary_sha256 | test("^[0-9a-f]{64}$"))
        and .host_toolchain.qemu_binary_architecture == "arm64"
        and (.host_toolchain.qemu_binary_sha256 | test("^[0-9a-f]{64}$"))
        and .host_toolchain.accelerator == "Hypervisor.Framework"
        and .device.state == "device"
        and .device.boot_completed == true
        and (.device.adb_serial | test("^emulator-[0-9]+$"))
        and (.device.avd_name | type == "string" and length > 0)
        and .device.avd_name == .avd.name
        and .device.qemu_avd_name == .avd.name
        and .device.avd_name_match == true
        and (.device.boot_id | test("^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"))
        and .avd.guest_abi == "arm64-v8a"
        and .avd.guest_uname_machine == "aarch64"
        and (.avd.system_image | test("ARM 64|arm64"; "i"))
        and .avd.headless == true
        and .avd.hardware_accelerated == true
        and .avd.renderer.mode == "host"
        and (.avd.renderer.vendor | contains("Apple"))
        and (.avd.renderer.adapter | contains("Apple"))
        and (.avd.renderer.host_backend | contains("Metal"))
        and .runtime.install_success == true
        and .runtime.cold_launch_success == true
        and .runtime.process_alive == true
        and .runtime.pid > 0
        and (.runtime.process_start_time_ticks | type == "number" and . == floor and . > 0)
        and .runtime.foreground == true
        and .runtime.top_resumed == true
        and .runtime.current_focus == true
        and .runtime.focused_app == true
        and .runtime.fatal_marker_count == 0
        and .runtime.anr_marker_count == 0
        and .runtime.login_bgra_gl_error_count == 0
        and .runtime.crash_buffer_empty == true
        and .runtime.fresh_install_without_supplied_credentials == true
        and .runtime.stale_package_removed == true
        and (.runtime.installed_package_path | test("^/data/app/[0-9A-Za-z._~=/+-]+/base\\.apk$"))
        and (.session_probe.path | test("^/data/local/tmp/hepta-native-smoke-[0-9a-f]{24}$"))
        and (.session_probe.nonce | test("^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"))
        and (.session_probe.sha256 | test("^[0-9a-f]{64}$"))
        and .session_probe.boot_id == .device.boot_id
        and .session_probe.created_by_producer == true
        and .session_probe.readback_matched == true
        and .session_probe.no_credentials_supplied == true
        and .login_surface_template.version == 1
        and .login_surface_template.all_states_ready == true
        and .login_surface_template.manifest_path == $login_manifest
        and .login_surface_template.manifest_sha256 == $login_manifest_sha
        and .visual_inspection.machine_verified_original_dimensions == true
        and (. as $receipt | ["portrait","landscape","ime"] | all(. as $key |
          ($receipt.visual_inspection[$key].format == "png")
          and ($receipt.visual_inspection[$key].path | type == "string" and startswith("/") and (contains("/../") | not))
          and ($receipt.visual_inspection[$key].sha256 | test("^[0-9a-f]{64}$"))
          and ($receipt.visual_inspection[$key].width | type == "number" and . >= 320)
          and ($receipt.visual_inspection[$key].height | type == "number" and . >= 320)
          and ($receipt.visual_inspection[$key].content_probe.status == "ready")
          and ($receipt.visual_inspection[$key].content_probe.ready == true)
          and ($receipt.visual_inspection[$key].login_template_probe.status == "ready")
          and ($receipt.visual_inspection[$key].login_template_probe.ready == true)
          and ($receipt.visual_inspection[$key].login_surface_template_ready == true)
          and ($receipt.visual_inspection[$key].app_remains_foreground == true)
        ))
        and ([.visual_inspection.portrait.path,.visual_inspection.landscape.path,.visual_inspection.ime.path] | unique | length == 3)
        and ([.visual_inspection.portrait.sha256,.visual_inspection.landscape.sha256,.visual_inspection.ime.sha256] | unique | length == 3)
        and .visual_inspection.portrait.width < .visual_inspection.portrait.height
        and .visual_inspection.landscape.width > .visual_inspection.landscape.height
        and .visual_inspection.ime.width < .visual_inspection.ime.height
        and .visual_inspection.ime.input_shown == true
        and .visual_inspection.ime.input_view_shown == true
        and .visual_inspection.ime.focused_surface == "homeserver_input_template_anchor"
        and .visual_inspection.ime.focused_surface_visible == true
        and .visual_inspection.ime.manifest_soft_input_mode == "STATE_UNCHANGED|ADJUST_NOTHING"
        and .visual_inspection.ime.manifest_soft_input_contract_ready == true
        and .uiautomator.xml_ready == true
        and (.uiautomator.path | type == "string" and startswith("/") and (contains("/../") | not))
        and (.uiautomator.sha256 | test("^[0-9a-f]{64}$"))
        and .uiautomator.node_count > 0
        and .uiautomator.visible_node_count > 0
        and .uiautomator.package_node_count > 0
        and .uiautomator.semantic_accessibility_ready == false
        and .uiautomator.talkback_ready == false
        and .accessibility.semantic_accessibility_ready == false
        and .accessibility.talkback_ready == false
        and .claims.android_arm64_debug_apk_installable == true
        and .claims.android_emulator_environment_ready == true
        and .claims.android_emulator_runtime_ready == true
        and .claims.android_emulator_login_surface_visual_ready == true
        and .claims.android_login_rotation_ready == true
        and .claims.android_login_ime_ready == true
        and .claims.android_rotation_ready == false
        and .claims.android_ime_ready == false
        and .claims.android_accessibility_ready == false
        and .claims.talkback_ready == false
        and .claims.android_safe_area_ready == false
        and .claims.android_rtl_ready == false
        and .claims.android_dynamic_type_ready == false
        and .claims.android_low_power_performance_ready == false
        and .claims.android_real_device_ready == false
        and .claims.android_secure_credential_backend_ready == false
        and .claims.authenticated_matrix_workflow_ready == false
        and .claims.post_login_raster_media_ready == false
        and .claims.release_signed == false
        and .claims.public_distribution_ready == false
        and .claims.full_product_ready == false
        and .claims.public_ga_ready == false
        and (.hard_boundaries | to_entries | all(.value == false))
        and (.forbidden_actions_performed | to_entries | all(.value == false))
      ' "$ANDROID_EMULATOR_RECEIPT" >/dev/null 2>&1; then
    android_receipt_apk_path="$(jq -r '.artifact.path' "$ANDROID_EMULATOR_RECEIPT")"
    android_receipt_apk_sha256="$(jq -r '.artifact.sha256' "$ANDROID_EMULATOR_RECEIPT")"
    android_receipt_apk_size="$(jq -r '.artifact.size_bytes' "$ANDROID_EMULATOR_RECEIPT")"
    if [[ "$(shasum -a 256 "$android_receipt_apk_path" 2>/dev/null | awk '{print $1}')" == "$android_receipt_apk_sha256" \
      && "$(stat -f %z "$android_receipt_apk_path" 2>/dev/null)" == "$android_receipt_apk_size" ]] \
      && verify_android_emulator_artifact "$ANDROID_EMULATOR_RECEIPT" "$android_receipt_apk_path" \
      && verify_android_emulator_host_tools "$ANDROID_EMULATOR_RECEIPT" \
      && verify_android_emulator_screenshot "$ANDROID_EMULATOR_RECEIPT" portrait \
      && verify_android_emulator_screenshot "$ANDROID_EMULATOR_RECEIPT" landscape \
      && verify_android_emulator_screenshot "$ANDROID_EMULATOR_RECEIPT" ime \
      && verify_android_login_template "$ANDROID_EMULATOR_RECEIPT" portrait portrait \
      && verify_android_login_template "$ANDROID_EMULATOR_RECEIPT" landscape landscape \
      && verify_android_login_template "$ANDROID_EMULATOR_RECEIPT" ime ime \
      && verify_android_emulator_uiautomator "$ANDROID_EMULATOR_RECEIPT"; then
      if [[ "$ANDROID_EMULATOR_LIVE_READBACK_OPT_IN" == "1" ]]; then
        android_emulator_live_readback_performed=true
        if android_emulator_live_readback_report="$(
            env -i PATH=/usr/bin:/bin:/usr/sbin:/sbin "$ANDROID_EMULATOR_LIVE_READBACK_PATH" \
              --receipt "$ANDROID_EMULATOR_RECEIPT" 2>/dev/null
          )" \
          && jq -e '
            .schema_version == 1
            and .kind == "hepta-native-android-emulator-live-readback"
            and .status == "ready"
            and .ready == true
            and .trusted_adb.adb.strict_codesign_verified == true
            and .trusted_adb.adb.team_identifier == "EQHXZ8M8AV"
            and .adb_server.caller_routing_environment_trusted == false
            and .adb_server.all_device_commands_explicitly_serial_scoped == true
            and .adb_server.all_transports_enumerated == false
            and .device.real_device_contacted == false
            and .credential_free_current_session_probe.performed == true
            and .credential_free_current_session_probe.credentials_supplied == false
            and .credential_free_current_session_probe.pid_unchanged == true
            and .credential_free_current_session_probe.process_instance_unchanged == true
            and .credential_free_current_session_probe.focus_ready_before == true
            and .credential_free_current_session_probe.focus_ready_after == true
            and .independently_verified_claims.emulator_runtime == true
            and .independently_verified_claims.unauthenticated_login_surface_visual == false
            and .independently_verified_claims.unauthenticated_login_surface_rotation == false
            and .independently_verified_claims.unauthenticated_login_surface_ime == false
            and (.forbidden_actions_performed | to_entries | all(.value == false))
          ' >/dev/null <<<"$android_emulator_live_readback_report"; then
          android_emulator_live_readback_ready=true
          android_emulator_receipt_ready=true
          android_emulator_receipt_status="ready"
        fi
        if [[ "$android_emulator_live_readback_ready" != true ]]; then
          android_emulator_live_readback_report="$(jq -n \
            --arg receipt "$ANDROID_EMULATOR_RECEIPT" \
            '{schema_version:1,kind:"hepta-native-android-emulator-live-readback",status:"not_ready",ready:false,receipt:$receipt,reason:"trusted_live_readback_failed",independently_verified_claims:{emulator_runtime:false,unauthenticated_login_surface_visual:false,unauthenticated_login_surface_rotation:false,unauthenticated_login_surface_ime:false}}')"
        fi
      fi
    fi
  fi
  android_emulator_receipt_summary="$(jq -n \
    --arg path "$ANDROID_EMULATOR_RECEIPT" \
    --arg status "$android_emulator_receipt_status" \
    --argjson ready "$android_emulator_receipt_ready" \
    --argjson login_visual "$android_emulator_login_visual_ready" \
    --argjson login_rotation "$android_emulator_login_rotation_ready" \
    --argjson login_ime "$android_emulator_login_ime_ready" \
    --argjson live_opt_in "$ANDROID_EMULATOR_LIVE_READBACK_OPT_IN" \
    --argjson live_performed "$android_emulator_live_readback_performed" \
    --argjson live_ready "$android_emulator_live_readback_ready" \
    --argjson live_report "$android_emulator_live_readback_report" \
    '{supplied:true,path:$path,status:$status,ready:$ready,scope:"unauthenticated_android_login_surface_on_arm64_emulator",live_readback:{opt_in:($live_opt_in == 1),performed:$live_performed,ready:$live_ready,report:$live_report},claims:{runtime:$ready,unauthenticated_login_surface_visual:$login_visual,unauthenticated_login_surface_rotation:$login_rotation,unauthenticated_login_surface_ime:$login_ime,visual:false,rotation:false,ime:false},deprecated_generic_claims_hard_false:true}')"
fi

SOURCE_FINAL="$(scripts/hepta-ui-source-fingerprint)"
if [[ "$(jq -r '.head' <<<"$SOURCE_AFTER")" != "$(jq -r '.head' <<<"$SOURCE_FINAL")" \
  || "$(jq -r '.head_tree' <<<"$SOURCE_AFTER")" != "$(jq -r '.head_tree' <<<"$SOURCE_FINAL")" \
  || "$(jq -r '.source_fingerprint' <<<"$SOURCE_AFTER")" != "$(jq -r '.source_fingerprint' <<<"$SOURCE_FINAL")" ]]; then
  source_stable=false
  if [[ "$ios_simulator_receipt_supplied" == true ]]; then
    ios_simulator_receipt_ready=false
    ios_simulator_receipt_status="invalid"
    ios_simulator_receipt_summary="$(jq -n \
      --arg path "$IOS_SIMULATOR_RECEIPT" \
      '{supplied:true,path:$path,status:"invalid",ready:false}')"
  fi
  if [[ "$android_emulator_receipt_supplied" == true ]]; then
    android_emulator_receipt_ready=false
    android_emulator_receipt_status="invalid"
    android_emulator_live_readback_ready=false
    android_emulator_login_visual_ready=false
    android_emulator_login_rotation_ready=false
    android_emulator_login_ime_ready=false
    android_emulator_receipt_summary="$(jq -n \
      --arg path "$ANDROID_EMULATOR_RECEIPT" \
      --argjson live_opt_in "$ANDROID_EMULATOR_LIVE_READBACK_OPT_IN" \
      --argjson live_performed "$android_emulator_live_readback_performed" \
      '{supplied:true,path:$path,status:"invalid",ready:false,scope:null,live_readback:{opt_in:($live_opt_in == 1),performed:$live_performed,ready:false},claims:{runtime:false,unauthenticated_login_surface_visual:false,unauthenticated_login_surface_rotation:false,unauthenticated_login_surface_ime:false,visual:false,rotation:false,ime:false},deprecated_generic_claims_hard_false:true}')"
  fi
fi

source_contract_ready=false
if [[ "$source_stable" == true \
  && "$policy_ready" == true \
  && "$makepad_pin_ready" == true \
  && "$android_credential_fail_closed_ready" == true \
  && "$testflight_source_contract_ready" == true \
  && "$ios_simulator_smoke_source_contract_ready" == true \
  && "$android_emulator_smoke_source_contract_ready" == true \
  && "$android_emulator_live_readback_source_contract_ready" == true \
  && "$android_login_template_contract_ready" == true \
  && "$toolchain_wrapper_ready" == true \
  && "$ios_icon_contract_ready" == true \
  && "$ios_toolchain_targets_ready" == true \
  && "$android_toolchain_target_ready" == true ]]; then
  source_contract_ready=true
fi

report="$(jq -n \
  --arg generated_at_utc "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --argjson source_binding "$SOURCE_FINAL" \
  --argjson source_stable "$source_stable" \
  --argjson policy_ready "$policy_ready" \
  --argjson makepad_pin_ready "$makepad_pin_ready" \
  --argjson credential_ready "$android_credential_fail_closed_ready" \
  --argjson testflight_ready "$testflight_source_contract_ready" \
  --argjson ios_simulator_smoke_source_ready "$ios_simulator_smoke_source_contract_ready" \
  --argjson ios_simulator_smoke_source_contract "$ios_simulator_smoke_source_contract" \
  --argjson android_emulator_smoke_source_ready "$android_emulator_smoke_source_contract_ready" \
  --argjson android_emulator_smoke_source_contract "$android_emulator_smoke_source_contract" \
  --argjson android_emulator_live_readback_source_ready "$android_emulator_live_readback_source_contract_ready" \
  --argjson android_login_template_ready "$android_login_template_contract_ready" \
  --argjson ios_simulator_receipt_supplied "$ios_simulator_receipt_supplied" \
  --argjson ios_simulator_receipt_ready "$ios_simulator_receipt_ready" \
  --argjson ios_simulator_receipt_summary "$ios_simulator_receipt_summary" \
  --argjson android_emulator_receipt_supplied "$android_emulator_receipt_supplied" \
  --argjson android_emulator_receipt_ready "$android_emulator_receipt_ready" \
  --argjson android_emulator_receipt_summary "$android_emulator_receipt_summary" \
  --argjson android_live_readback_performed "$android_emulator_live_readback_performed" \
  --argjson android_live_readback_opt_in "$ANDROID_EMULATOR_LIVE_READBACK_OPT_IN" \
  --argjson android_login_visual_ready "$android_emulator_login_visual_ready" \
  --argjson android_login_rotation_ready "$android_emulator_login_rotation_ready" \
  --argjson android_login_ime_ready "$android_emulator_login_ime_ready" \
  --argjson toolchain_ready "$toolchain_wrapper_ready" \
  --argjson toolchain "$toolchain_report" \
  --argjson icons_ready "$ios_icon_contract_ready" \
  --argjson icons "$icon_report" \
  --argjson ios_targets "$ios_toolchain_targets_ready" \
  --argjson android_target "$android_toolchain_target_ready" \
  --argjson identity_available "$ios_distribution_identity_available" \
  --argjson identity_count "$ios_distribution_identity_count" \
  --argjson source_ready "$source_contract_ready" '
    {
      schema_version:1,
      kind:"hepta-native-mobile-readiness-gate",
      producer:"scripts/hepta-native-mobile-readiness-gate.sh",
      generated_at_utc:$generated_at_utc,
      status:(if $source_ready then "source_contract_ready" else "not_ready" end),
      source_binding:$source_binding,
      source_stable_during_run:$source_stable,
      mobile_source_contract_ready:$source_ready,
      checks:{
        policy_contract_ready:$policy_ready,
        pinned_makepad_revision_ready:$makepad_pin_ready,
        cargo_makepad_exact_toolchain_wrapper_ready:$toolchain_ready,
        testflight_fail_closed_current_source_contract_ready:$testflight_ready,
        ios_simulator_smoke_source_contract_ready:$ios_simulator_smoke_source_ready,
        android_emulator_smoke_source_contract_ready:$android_emulator_smoke_source_ready,
        android_emulator_live_readback_source_contract_ready:$android_emulator_live_readback_source_ready,
        android_login_template_contract_ready:$android_login_template_ready,
        ios_opaque_canonical_icon_contract_ready:$icons_ready,
        android_credential_fail_closed_contract_ready:$credential_ready,
        ios_pinned_toolchain_targets_installed:$ios_targets,
        android_pinned_toolchain_target_installed:$android_target
      },
      toolchain:$toolchain,
      ios_icons:$icons,
      ios_simulator_smoke_source_contract:$ios_simulator_smoke_source_contract,
      android_emulator_smoke_source_contract:$android_emulator_smoke_source_contract,
      ios_simulator_runtime_evidence:$ios_simulator_receipt_summary,
      android_emulator_runtime_evidence:$android_emulator_receipt_summary,
      signing_preflight:{apple_distribution_identity_available:$identity_available,apple_distribution_identity_count:$identity_count,signing_performed:false},
      hard_boundaries:{
        ios_accessibility_update_consumed:false,
        android_accessibility_update_consumed:false,
        android_secure_session_persistence_ready:false,
        plaintext_credential_fallback_allowed:false,
        ios_simulator_runtime_verified:$ios_simulator_receipt_ready,
        android_emulator_runtime_verified:$android_emulator_receipt_ready,
        android_emulator_unauthenticated_login_surface_visual_verified:$android_login_visual_ready,
        android_emulator_unauthenticated_login_surface_rotation_verified:$android_login_rotation_ready,
        android_emulator_unauthenticated_login_surface_ime_verified:$android_login_ime_ready,
        android_emulator_visual_verified:false,
        android_emulator_rotation_verified:false,
        android_emulator_ime_verified:false,
        deprecated_generic_android_emulator_claims_hard_false:true,
        ios_real_device_verified:false,
        android_real_device_verified:false,
        voiceover_verified:false,
        talkback_verified:false,
        software_keyboard_verified:false,
        safe_area_verified:false,
        rtl_verified:false,
        dynamic_type_or_font_scale_verified:false,
        mobile_full_product_ready:false,
        mobile_public_ga_ready:false
      },
      local_emulator_side_effects_performed:$android_live_readback_performed,
      external_side_effects_performed:false,
      blockers:([if $source_stable then empty else "source_changed_during_mobile_gate" end,if $policy_ready then empty else "mobile_policy_contract_not_ready" end,if $makepad_pin_ready then empty else "makepad_revision_not_pinned" end,if $toolchain_ready then empty else "cargo_makepad_exact_toolchain_wrapper_not_ready" end,if $testflight_ready then empty else "testflight_current_source_fail_closed_contract_not_ready" end,if $ios_simulator_smoke_source_ready then empty else "ios_simulator_smoke_source_contract_not_ready" end,if $android_emulator_smoke_source_ready then empty else "android_emulator_smoke_source_contract_not_ready" end,if $android_emulator_live_readback_source_ready then empty else "android_emulator_live_readback_source_contract_not_ready" end,if $android_login_template_ready then empty else "android_login_template_contract_not_ready" end,if $icons_ready then empty else "ios_opaque_icon_contract_not_ready" end,if $credential_ready then empty else "android_credential_fail_closed_contract_not_ready" end,if $ios_targets then empty else "ios_1_95_targets_not_installed" end,if $android_target then empty else "android_1_95_target_not_installed" end,if $identity_available then empty else "apple_distribution_identity_not_available" end,"pinned_makepad_ios_accessibility_update_discarded","pinned_makepad_android_accessibility_update_discarded","android_secure_credential_backend_not_supported",if $ios_simulator_receipt_ready then empty elif $ios_simulator_receipt_supplied then "ios_simulator_receipt_invalid" else "ios_simulator_receipt_missing" end,if $android_emulator_receipt_supplied and $android_live_readback_opt_in != 1 then "android_emulator_live_readback_opt_in_missing" elif $android_emulator_receipt_ready then empty elif $android_emulator_receipt_supplied then "android_emulator_receipt_invalid" else "android_emulator_receipt_missing" end,"ios_real_device_receipt_missing","android_real_device_receipt_missing","voiceover_receipt_missing","talkback_receipt_missing","software_keyboard_receipt_missing","safe_area_receipt_missing","rtl_receipt_missing","dynamic_type_or_font_scale_receipt_missing"])
    }
  ')"

if [[ -n "$REPORT_PATH" ]]; then
  mkdir -p "$(dirname "$REPORT_PATH")"
  printf '%s\n' "$report" >"$REPORT_PATH"
fi
printf '%s\n' "$report"
if [[ "$ios_simulator_receipt_supplied" == true && "$ios_simulator_receipt_ready" != true ]]; then
  exit 1
fi
if [[ "$android_emulator_receipt_supplied" == true && "$android_emulator_receipt_ready" != true ]]; then
  exit 1
fi
jq -e '.mobile_source_contract_ready == true' <<<"$report" >/dev/null
