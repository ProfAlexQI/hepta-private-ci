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
values. It never signs, uploads, boots a simulator, or contacts a device.
EOF
      exit 0
      ;;
    *) echo "unknown argument: $1" >&2; exit 64 ;;
  esac
done

for command in git jq ruby rustup shasum ditto strings plutil sips find unzip file; do
  command -v "$command" >/dev/null 2>&1 || { echo "$command is required" >&2; exit 2; }
done

SOURCE_BEFORE="$(scripts/hepta-ui-source-fingerprint)"
POLICY_PATH="apps/hepta-native/mobile-readiness-policy-v1.json"
MANIFEST_PATH="apps/hepta-native/Cargo.toml"
CREDENTIAL_PATH="apps/hepta-native/src/persistence/matrix_session_store/credential.rs"
TESTFLIGHT_PATH="apps/hepta-native/packaging/build-ios-testflight.sh"
IOS_SIMULATOR_SMOKE_PATH="scripts/hepta-native-ios-simulator-smoke.sh"
IOS_SIMULATOR_RECEIPT="${HEPTA_NATIVE_IOS_SIMULATOR_RECEIPT:-}"
ANDROID_EMULATOR_RECEIPT="${HEPTA_NATIVE_ANDROID_EMULATOR_RECEIPT:-}"

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
android_emulator_receipt_summary="$(jq -n \
  --arg path "$ANDROID_EMULATOR_RECEIPT" \
  '{supplied:false,path:$path,status:"missing",ready:false,claims:{runtime:false,visual:false,rotation:false,ime:false}}')"

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
  sdk_root="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}"
  [[ -d "$sdk_root/build-tools" ]] || return 1
  candidate="$(find "$sdk_root/build-tools" -mindepth 2 -maxdepth 2 -type f -name "$tool" -perm -111 -print 2>/dev/null | sort | tail -1)"
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
  sdk_root="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}"
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
    && jq -e \
      --arg head "$(jq -r '.head' <<<"$SOURCE_AFTER")" \
      --arg tree "$(jq -r '.head_tree' <<<"$SOURCE_AFTER")" \
      --arg fingerprint "$(jq -r '.source_fingerprint' <<<"$SOURCE_AFTER")" '
        .schema_version == 2
        and .kind == "hepta-native-android-arm64-emulator-runtime-receipt"
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
        and .host_toolchain.emulator_binary_architecture == "arm64"
        and (.host_toolchain.emulator_binary_sha256 | test("^[0-9a-f]{64}$"))
        and .host_toolchain.qemu_binary_architecture == "arm64"
        and (.host_toolchain.qemu_binary_sha256 | test("^[0-9a-f]{64}$"))
        and .host_toolchain.accelerator == "Hypervisor.Framework"
        and .avd.guest_abi == "arm64-v8a"
        and .avd.guest_uname_machine == "aarch64"
        and (.avd.system_image | contains("arm64-v8a"))
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
        and .runtime.foreground == true
        and .runtime.top_resumed == true
        and .runtime.current_focus == true
        and .runtime.focused_app == true
        and .runtime.fatal_marker_count == 0
        and .runtime.anr_marker_count == 0
        and .runtime.login_bgra_gl_error_count == 0
        and .runtime.crash_buffer_empty == true
        and .runtime.matrix_state == "unauthenticated_waiting_for_login"
        and .visual_inspection.inspected_with_original_resolution_viewer == true
        and (. as $receipt | ["portrait","landscape_top","landscape_scrolled","ime"] | all(. as $key |
          ($receipt.visual_inspection[$key].format == "png")
          and ($receipt.visual_inspection[$key].path | type == "string" and startswith("/") and (contains("/../") | not))
          and ($receipt.visual_inspection[$key].sha256 | test("^[0-9a-f]{64}$"))
          and ($receipt.visual_inspection[$key].width | type == "number" and . >= 320)
          and ($receipt.visual_inspection[$key].height | type == "number" and . >= 320)
          and ($receipt.visual_inspection[$key].content_probe.status == "ready")
          and ($receipt.visual_inspection[$key].content_probe.ready == true)
        ))
        and ([.visual_inspection.portrait.path,.visual_inspection.landscape_top.path,.visual_inspection.landscape_scrolled.path,.visual_inspection.ime.path] | unique | length == 4)
        and ([.visual_inspection.portrait.sha256,.visual_inspection.landscape_top.sha256,.visual_inspection.landscape_scrolled.sha256,.visual_inspection.ime.sha256] | unique | length == 4)
        and .visual_inspection.portrait.width < .visual_inspection.portrait.height
        and .visual_inspection.portrait.form_fits_viewport == true
        and .visual_inspection.landscape_top.width > .visual_inspection.landscape_top.height
        and .visual_inspection.landscape_top.app_remains_foreground == true
        and .visual_inspection.landscape_top.top_content_visible == true
        and .visual_inspection.landscape_scrolled.width > .visual_inspection.landscape_scrolled.height
        and .visual_inspection.landscape_scrolled.app_remains_foreground == true
        and .visual_inspection.landscape_scrolled.lower_content_visible == true
        and .visual_inspection.landscape_scrolled.sign_in_action_visible == true
        and .visual_inspection.ime.width < .visual_inspection.ime.height
        and .visual_inspection.ime.input_shown == true
        and .visual_inspection.ime.focused_field_visible == true
        and .visual_inspection.ime.soft_input_mode == "ADJUST_NOTHING_WITH_MAKEPAD_KEYBOARD_VIEW"
        and .visual_inspection.ime.manifest_soft_input_mode == "STATE_UNCHANGED|ADJUST_NOTHING"
        and .visual_inspection.ime.manifest_soft_input_contract_ready == true
        and .visual_inspection.ime.lower_form_covered == false
        and .asset_rendering.brand_mark_correct == true
        and .asset_rendering.sso_provider_marks_correct == true
        and .asset_rendering.provider_marks_texture_free == true
        and .asset_rendering.black_rectangle_regression_absent == true
        and .claims.android_arm64_debug_apk_installable == true
        and .claims.android_emulator_environment_ready == true
        and .claims.android_emulator_runtime_ready == true
        and .claims.android_emulator_login_surface_visual_ready == true
        and .claims.android_login_rotation_ready == true
        and .claims.android_login_ime_ready == true
        and .claims.android_login_asset_rendering_ready == true
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
      ' "$ANDROID_EMULATOR_RECEIPT" >/dev/null 2>&1; then
    android_receipt_apk_path="$(jq -r '.artifact.path' "$ANDROID_EMULATOR_RECEIPT")"
    android_receipt_apk_sha256="$(jq -r '.artifact.sha256' "$ANDROID_EMULATOR_RECEIPT")"
    android_receipt_apk_size="$(jq -r '.artifact.size_bytes' "$ANDROID_EMULATOR_RECEIPT")"
    if [[ "$(shasum -a 256 "$android_receipt_apk_path" 2>/dev/null | awk '{print $1}')" == "$android_receipt_apk_sha256" \
      && "$(stat -f %z "$android_receipt_apk_path" 2>/dev/null)" == "$android_receipt_apk_size" ]] \
      && verify_android_emulator_artifact "$ANDROID_EMULATOR_RECEIPT" "$android_receipt_apk_path" \
      && verify_android_emulator_host_tools "$ANDROID_EMULATOR_RECEIPT" \
      && verify_android_emulator_screenshot "$ANDROID_EMULATOR_RECEIPT" portrait \
      && verify_android_emulator_screenshot "$ANDROID_EMULATOR_RECEIPT" landscape_top \
      && verify_android_emulator_screenshot "$ANDROID_EMULATOR_RECEIPT" landscape_scrolled \
      && verify_android_emulator_screenshot "$ANDROID_EMULATOR_RECEIPT" ime; then
      android_emulator_receipt_ready=true
      android_emulator_receipt_status="ready"
    fi
  fi
  android_emulator_receipt_summary="$(jq -n \
    --arg path "$ANDROID_EMULATOR_RECEIPT" \
    --arg status "$android_emulator_receipt_status" \
    --argjson ready "$android_emulator_receipt_ready" \
    '{supplied:true,path:$path,status:$status,ready:$ready,claims:{runtime:$ready,visual:$ready,rotation:$ready,ime:$ready}}')"
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
    android_emulator_receipt_summary="$(jq -n \
      --arg path "$ANDROID_EMULATOR_RECEIPT" \
      '{supplied:true,path:$path,status:"invalid",ready:false,claims:{runtime:false,visual:false,rotation:false,ime:false}}')"
  fi
fi

source_contract_ready=false
if [[ "$source_stable" == true \
  && "$policy_ready" == true \
  && "$makepad_pin_ready" == true \
  && "$android_credential_fail_closed_ready" == true \
  && "$testflight_source_contract_ready" == true \
  && "$ios_simulator_smoke_source_contract_ready" == true \
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
  --argjson ios_simulator_receipt_supplied "$ios_simulator_receipt_supplied" \
  --argjson ios_simulator_receipt_ready "$ios_simulator_receipt_ready" \
  --argjson ios_simulator_receipt_summary "$ios_simulator_receipt_summary" \
  --argjson android_emulator_receipt_supplied "$android_emulator_receipt_supplied" \
  --argjson android_emulator_receipt_ready "$android_emulator_receipt_ready" \
  --argjson android_emulator_receipt_summary "$android_emulator_receipt_summary" \
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
        ios_opaque_canonical_icon_contract_ready:$icons_ready,
        android_credential_fail_closed_contract_ready:$credential_ready,
        ios_pinned_toolchain_targets_installed:$ios_targets,
        android_pinned_toolchain_target_installed:$android_target
      },
      toolchain:$toolchain,
      ios_icons:$icons,
      ios_simulator_smoke_source_contract:$ios_simulator_smoke_source_contract,
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
        android_emulator_visual_verified:$android_emulator_receipt_ready,
        android_emulator_rotation_verified:$android_emulator_receipt_ready,
        android_emulator_ime_verified:$android_emulator_receipt_ready,
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
      external_side_effects_performed:false,
      blockers:([if $source_stable then empty else "source_changed_during_mobile_gate" end,if $policy_ready then empty else "mobile_policy_contract_not_ready" end,if $makepad_pin_ready then empty else "makepad_revision_not_pinned" end,if $toolchain_ready then empty else "cargo_makepad_exact_toolchain_wrapper_not_ready" end,if $testflight_ready then empty else "testflight_current_source_fail_closed_contract_not_ready" end,if $ios_simulator_smoke_source_ready then empty else "ios_simulator_smoke_source_contract_not_ready" end,if $icons_ready then empty else "ios_opaque_icon_contract_not_ready" end,if $credential_ready then empty else "android_credential_fail_closed_contract_not_ready" end,if $ios_targets then empty else "ios_1_95_targets_not_installed" end,if $android_target then empty else "android_1_95_target_not_installed" end,if $identity_available then empty else "apple_distribution_identity_not_available" end,"pinned_makepad_ios_accessibility_update_discarded","pinned_makepad_android_accessibility_update_discarded","android_secure_credential_backend_not_supported",if $ios_simulator_receipt_ready then empty elif $ios_simulator_receipt_supplied then "ios_simulator_receipt_invalid" else "ios_simulator_receipt_missing" end,if $android_emulator_receipt_ready then empty elif $android_emulator_receipt_supplied then "android_emulator_receipt_invalid" else "android_emulator_receipt_missing" end,"ios_real_device_receipt_missing","android_real_device_receipt_missing","voiceover_receipt_missing","talkback_receipt_missing","software_keyboard_receipt_missing","safe_area_receipt_missing","rtl_receipt_missing","dynamic_type_or_font_scale_receipt_missing"])
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
