#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
source "$ROOT_DIR/scripts/hepta-native-mobile-lab-cleanup-v1.sh"
APP_DIR="$ROOT_DIR/apps/hepta-native"
PRODUCER="scripts/hepta-native-android-emulator-smoke.sh"
PACKAGE_NAME="ai.hepta.nativeapp"
APP_LABEL="Hepta"
ACTIVITY="$PACKAGE_NAME/.MakepadApp"
CARGO_PACKAGE="hepta-native"
LOGIN_TEMPLATE_DIR="$APP_DIR/packaging/android-emulator-login-template-v1"
LOGIN_TEMPLATE_MANIFEST="$LOGIN_TEMPLATE_DIR/manifest.json"
ORIENTATION_PROBE="$ROOT_DIR/scripts/hepta-android-window-orientation-probe"
SYSTEM_BAR_CONTRAST_PROBE="$ROOT_DIR/scripts/hepta-android-system-bar-contrast-probe"
HEADLESS_AVD_PROCESS_PROBE="$ROOT_DIR/scripts/hepta-android-headless-avd-process-probe"
NDK_DIRECTORY_VERSION="28.2.13676358"
NDK_RELEASE_NAME="r28b"
NDK_HOST_PREBUILT="darwin-x86_64"
MAKEPAD_ANDROID_PLATFORM="android-33-ext4"
MAKEPAD_ANDROID_BUILD_TOOLS_VERSION="33.0.1"

AVD_NAME=""
ADB_SERIAL=""
SDK_ROOT="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-}}"
REPORT_PATH=""
EVIDENCE_DIR=""
TARGET_DIR=""
CONTRACT_ONLY=false
EXTENDED_LAB=false
LAB_STARTUP_SAMPLES=3
REPORT_TEMP=""

usage() {
  cat <<'EOF'
usage: scripts/hepta-native-android-emulator-smoke.sh \
  --avd Hepta_Pixel_API_34_arm64 \
  --serial emulator-5554 \
  --sdk-root /absolute/path/to/Android/sdk \
  --output /absolute/path/report.json \
  [--evidence-dir /absolute/path/evidence] \
  [--target-dir /absolute/path/cargo-target]

Builds a fresh ARM64 APK from the current completely-clean committed HEAD with
scripts/hepta-native-mobile-cargo, removes the old package from the explicitly
named already-booted headless ARM64 AVD, installs and cold-launches the APK,
captures portrait/landscape/IME screenshots, dumps UIAutomator, and writes a
current-source-bound receipt. It never creates or boots an AVD, downloads an
SDK/runtime, supplies credentials, contacts a real device, release-signs, or
uploads anything.

  --contract-only   print the side-effect-free source contract and exit
  --extended-lab    opt in to reversible emulator-only RTL, font-scale,
                    landscape+IME, repeated startup, and simulated low-power
                    modes; generic/real-device/TalkBack claims stay false
  --help, -h        show this help
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --avd) AVD_NAME="${2:-}"; shift 2 ;;
    --serial) ADB_SERIAL="${2:-}"; shift 2 ;;
    --sdk-root) SDK_ROOT="${2:-}"; shift 2 ;;
    --output) REPORT_PATH="${2:-}"; shift 2 ;;
    --evidence-dir) EVIDENCE_DIR="${2:-}"; shift 2 ;;
    --target-dir) TARGET_DIR="${2:-}"; shift 2 ;;
    --extended-lab) EXTENDED_LAB=true; shift ;;
    --contract-only) CONTRACT_ONLY=true; shift ;;
    --help|-h) usage; exit 0 ;;
    *) echo "error: unknown argument: $1" >&2; usage >&2; exit 64 ;;
  esac
done

if [[ "$CONTRACT_ONLY" == true ]]; then
  jq -n \
    --arg producer "$PRODUCER" \
    --arg package "$PACKAGE_NAME" \
    --arg activity "$ACTIVITY" \
    --arg label "$APP_LABEL" '
      {
        schema_version:1,
        kind:"hepta-native-android-emulator-smoke-source-contract",
        status:"ready",
        producer:$producer,
        receipt:{schema_version:3,kind:"hepta-native-android-emulator-smoke-receipt"},
        identity:{package:$package,activity:$activity,label:$label,abi:"arm64-v8a"},
        build_wrapper:"scripts/hepta-native-mobile-cargo",
        requirements:{
          clean_committed_current_head:true,
          explicit_already_booted_headless_avd:true,
          explicit_adb_serial:true,
          arm64_host_and_guest:true,
          stale_package_removed:true,
          pinned_fresh_apk_build:true,
          pinned_ndk_r28b_bound:true,
          pinned_makepad_android_sdk_bound:true,
          full_head_embedded:true,
          portrait_landscape_ime_png_sha256:true,
          dumpsys_window_rotation_and_logical_geometry_ready:true,
          image_content_probe_ready:true,
          system_bar_contrast_probe_ready:true,
          system_bar_contrast_top_and_bottom_ready:true,
          system_bar_contrast_replayable_evidence_ready:true,
          versioned_login_template_probe_ready:true,
          uiautomator_xml_ready:true,
          app_foreground_and_focused:true,
          structured_current_source_receipt:true,
          extended_lab_opt_in:true,
          extended_lab_state_snapshot_restore_readback:true,
          extended_lab_mode_matched_controls:true,
          extended_lab_leaf_rehash_before_promotion:true,
          extended_lab_raw_setting_absence_restore:true,
          extended_lab_unrestorable_frozen_battery_rejected_before_mutation:true,
          exit_cleanup_preserves_original_status:true,
          interrupt_cleanup_restore_and_readback:true,
          cleanup_failure_receipt:true,
          emulator_only_power_simulation_never_real_device_claim:true
        },
        hard_boundaries:{
          accessibility:false,
          talkback:false,
          real_device:false,
          secure_credential_backend:false,
          authenticated_matrix_workflow:false,
          release_signing:false,
          public_distribution:false
        },
        forbidden_actions:{
          sdk_or_runtime_download:false,
          avd_create_or_boot:false,
          credential_supply:false,
          real_device_contact:false,
          release_sign:false,
          upload:false
        },
        external_side_effects_performed:false
      }
    '
  exit 0
fi

[[ -n "$AVD_NAME" ]] || { echo "error: --avd is required" >&2; exit 64; }
[[ -n "$ADB_SERIAL" ]] || { echo "error: --serial is required" >&2; exit 64; }
[[ -n "$SDK_ROOT" ]] || { echo "error: --sdk-root (or ANDROID_SDK_ROOT) is required" >&2; exit 64; }
[[ -n "$REPORT_PATH" ]] || { echo "error: --output is required" >&2; exit 64; }
[[ "$AVD_NAME" =~ ^[A-Za-z0-9._-]+$ ]] || { echo "error: unsafe AVD name" >&2; exit 64; }
[[ "$ADB_SERIAL" =~ ^emulator-[0-9]+$ ]] || { echo "error: --serial must be an explicit local emulator-NNNN serial" >&2; exit 64; }

for command in awk bash cp date ditto file find git grep head jq ps rg ruby sed seq shasum sips sleep sort stat strings sw_vers tail tr uname unzip uuidgen wc xmllint; do
  command -v "$command" >/dev/null 2>&1 || { echo "error: $command is required" >&2; exit 2; }
done

external_file_path() {
  local requested="$1" parent base resolved
  [[ "$requested" = /* ]] || { echo "error: evidence paths must be absolute: $requested" >&2; return 1; }
  parent="$(dirname "$requested")"
  base="$(basename "$requested")"
  mkdir -p "$parent"
  parent="$(cd "$parent" && pwd -P)"
  resolved="$parent/$base"
  case "$resolved" in
    "$ROOT_DIR"|"$ROOT_DIR"/*)
      echo "error: runtime evidence must stay outside the source repository: $resolved" >&2
      return 1
      ;;
  esac
  printf '%s\n' "$resolved"
}

external_directory_path() {
  local requested="$1" resolved
  [[ "$requested" = /* ]] || { echo "error: evidence directories must be absolute: $requested" >&2; return 1; }
  mkdir -p "$requested"
  resolved="$(cd "$requested" && pwd -P)"
  case "$resolved" in
    "$ROOT_DIR"|"$ROOT_DIR"/*)
      echo "error: runtime evidence must stay outside the source repository: $resolved" >&2
      return 1
      ;;
  esac
  printf '%s\n' "$resolved"
}

REPORT_PATH="$(external_file_path "$REPORT_PATH")"
if [[ -z "$EVIDENCE_DIR" ]]; then EVIDENCE_DIR="${REPORT_PATH%.json}.evidence"; fi
if [[ -z "$TARGET_DIR" ]]; then TARGET_DIR="$EVIDENCE_DIR/cargo-target"; fi
EVIDENCE_DIR="$(external_directory_path "$EVIDENCE_DIR")"
TARGET_DIR="$(external_directory_path "$TARGET_DIR")"

SDK_ROOT="$(cd "$SDK_ROOT" && pwd -P)"
ADB="$SDK_ROOT/platform-tools/adb"
EMULATOR="$SDK_ROOT/emulator/emulator"
QEMU="$SDK_ROOT/emulator/qemu/darwin-aarch64/qemu-system-aarch64-headless"
NDK_ROOT="$SDK_ROOT/ndk/$NDK_DIRECTORY_VERSION"
NDK_SOURCE_PROPERTIES="$NDK_ROOT/source.properties"
NDK_CLANG="$NDK_ROOT/toolchains/llvm/prebuilt/$NDK_HOST_PREBUILT/bin/clang"
MAKEPAD_ANDROID_JAR="$SDK_ROOT/platforms/$MAKEPAD_ANDROID_PLATFORM/android.jar"
MAKEPAD_AAPT="$SDK_ROOT/build-tools/$MAKEPAD_ANDROID_BUILD_TOOLS_VERSION/aapt"
MAKEPAD_AAPT2="$SDK_ROOT/build-tools/$MAKEPAD_ANDROID_BUILD_TOOLS_VERSION/aapt2"
MAKEPAD_D8_JAR="$SDK_ROOT/build-tools/$MAKEPAD_ANDROID_BUILD_TOOLS_VERSION/lib/d8.jar"
MAKEPAD_ZIPALIGN="$SDK_ROOT/build-tools/$MAKEPAD_ANDROID_BUILD_TOOLS_VERSION/zipalign"
MAKEPAD_APKSIGNER_JAR="$SDK_ROOT/build-tools/$MAKEPAD_ANDROID_BUILD_TOOLS_VERSION/lib/apksigner.jar"
MAKEPAD_JAVA="$SDK_ROOT/openjdk/bin/java"
MAKEPAD_JAVAC="$SDK_ROOT/openjdk/bin/javac"
[[ -x "$ADB" ]] || { echo "error: Android adb is missing: $ADB" >&2; exit 2; }
[[ -x "$EMULATOR" ]] || { echo "error: Android emulator is missing: $EMULATOR" >&2; exit 2; }
[[ -x "$QEMU" ]] || { echo "error: ARM64 headless qemu is missing: $QEMU" >&2; exit 2; }
[[ -s "$NDK_SOURCE_PROPERTIES" ]] || { echo "error: pinned Android NDK source properties are missing: $NDK_SOURCE_PROPERTIES" >&2; exit 2; }
[[ -x "$NDK_CLANG" ]] || { echo "error: pinned Android NDK clang is missing: $NDK_CLANG" >&2; exit 2; }
grep -Fxq "Pkg.ReleaseName = $NDK_RELEASE_NAME" "$NDK_SOURCE_PROPERTIES" \
  || { echo "error: pinned Android NDK release is not $NDK_RELEASE_NAME" >&2; exit 2; }
for pinned_file in "$MAKEPAD_ANDROID_JAR" "$MAKEPAD_D8_JAR" "$MAKEPAD_APKSIGNER_JAR"; do
  [[ -s "$pinned_file" ]] || { echo "error: pinned Makepad Android SDK file is missing: $pinned_file" >&2; exit 2; }
done
for pinned_executable in "$MAKEPAD_AAPT" "$MAKEPAD_AAPT2" "$MAKEPAD_ZIPALIGN" "$MAKEPAD_JAVA" "$MAKEPAD_JAVAC"; do
  [[ -x "$pinned_executable" ]] || { echo "error: pinned Makepad Android SDK executable is missing: $pinned_executable" >&2; exit 2; }
done
[[ -x "$ORIENTATION_PROBE" ]] || { echo "error: Android window orientation probe is missing: $ORIENTATION_PROBE" >&2; exit 2; }
[[ -f "$HEADLESS_AVD_PROCESS_PROBE" ]] || { echo "error: Android headless AVD process probe is missing: $HEADLESS_AVD_PROCESS_PROBE" >&2; exit 2; }

latest_build_tool() {
  local name="$1" candidate
  candidate="$(find "$SDK_ROOT/build-tools" -mindepth 2 -maxdepth 2 -type f -name "$name" -perm -111 -print 2>/dev/null \
    | ruby -e 'paths = STDIN.each_line.map(&:chomp); puts(paths.max_by { |path| File.basename(File.dirname(path)).split(/[^0-9]+/).map(&:to_i) }) unless paths.empty?')"
  [[ -n "$candidate" ]] || return 1
  printf '%s\n' "$candidate"
}
AAPT="$(latest_build_tool aapt)" || { echo "error: aapt is missing under $SDK_ROOT/build-tools" >&2; exit 2; }
APKSIGNER="$(latest_build_tool apksigner)" || { echo "error: apksigner is missing under $SDK_ROOT/build-tools" >&2; exit 2; }

jq -L "$ROOT_DIR/scripts/lib" -e '
  include "hepta-native-android-login-template-v1";
  hepta_android_login_template_v1_ready
' "$LOGIN_TEMPLATE_MANIFEST" >/dev/null
for template_key in portrait landscape ime; do
  template_relative="$(jq -r --arg key "$template_key" '.templates[$key].path' "$LOGIN_TEMPLATE_MANIFEST")"
  template_expected_sha="$(jq -r --arg key "$template_key" '.templates[$key].sha256' "$LOGIN_TEMPLATE_MANIFEST")"
  [[ -s "$ROOT_DIR/$template_relative" ]] || { echo "error: login template is missing: $template_relative" >&2; exit 1; }
  [[ "$(shasum -a 256 "$ROOT_DIR/$template_relative" | awk '{print $1}')" == "$template_expected_sha" ]] \
    || { echo "error: login template SHA drifted: $template_relative" >&2; exit 1; }
done

# This negative hook is intentionally incapable of reaching any device or
# producing a receipt. It lets the hermetic self-test prove that dirty source
# is rejected before adb, build, install, launch, or screenshot commands.
if [[ "${HEPTA_ANDROID_SMOKE_FORCE_DIRTY_SELF_TEST:-0}" == "1" ]]; then
  SOURCE_BEFORE='{"worktree_clean":false,"repository_worktree_clean":false,"dirty_path_count":1,"repository_dirty_path_count":1}'
else
  SOURCE_BEFORE="$($ROOT_DIR/scripts/hepta-ui-source-fingerprint)"
fi
if ! jq -e '
    .worktree_clean == true
    and .repository_worktree_clean == true
    and .dirty_path_count == 0
    and .repository_dirty_path_count == 0
  ' >/dev/null <<<"$SOURCE_BEFORE"; then
  echo "error: Android emulator smoke requires a completely clean committed current HEAD" >&2
  exit 1
fi
SOURCE_HEAD="$(jq -r '.head' <<<"$SOURCE_BEFORE")"
SOURCE_TREE="$(jq -r '.head_tree' <<<"$SOURCE_BEFORE")"
SOURCE_FINGERPRINT="$(jq -r '.source_fingerprint' <<<"$SOURCE_BEFORE")"

[[ "$(uname -m)" == "arm64" ]] || { echo "error: canonical Android ARM64 emulator evidence requires an arm64 macOS host" >&2; exit 1; }
file "$EMULATOR" | grep -Eq ': Mach-O 64-bit executable arm64$' || { echo "error: emulator is not an arm64 Mach-O" >&2; exit 1; }
file "$QEMU" | grep -Eq ': Mach-O 64-bit executable arm64$' || { echo "error: headless qemu is not an arm64 Mach-O" >&2; exit 1; }

TOOLCHAIN_REPORT="$($ROOT_DIR/scripts/hepta-native-mobile-cargo --print-toolchain-contract)"
jq -e '
  .status == "ready"
  and .resolved_toolchain == "1.95.0"
  and .cargo_makepad.revision == "c4335cee10b22aca768510c9d072b0ca1bba15c8"
  and .cargo_makepad.exact_revision_source_marker_ready == true
  and .cargo_makepad.custom_android_manifest_help_contract_ready == true
  and .cargo_makepad.global_cargo_makepad_used == false
  and .user_global_stable_mutated == false
' >/dev/null <<<"$TOOLCHAIN_REPORT"
TOOLS_DIR="$(dirname "$(dirname "$(jq -r '.cargo_makepad.binary' <<<"$TOOLCHAIN_REPORT")")")"

"$ADB" devices -l >"$EVIDENCE_DIR/adb-devices-preflight.txt"
grep -Eq "^${ADB_SERIAL}[[:space:]]+device([[:space:]]|$)" "$EVIDENCE_DIR/adb-devices-preflight.txt" \
  || { echo "error: $ADB_SERIAL is not exactly one online adb device" >&2; exit 1; }
[[ "$($ADB -s "$ADB_SERIAL" get-state)" == "device" ]] || { echo "error: adb serial is not online" >&2; exit 1; }
DEVICE_AVD="$($ADB -s "$ADB_SERIAL" emu avd name | tr -d '\r' | sed '/^OK$/d' | sed '/^[[:space:]]*$/d' | head -1)"
[[ "$DEVICE_AVD" == "$AVD_NAME" ]] || { echo "error: adb serial belongs to AVD '$DEVICE_AVD', not '$AVD_NAME'" >&2; exit 1; }
[[ "$($ADB -s "$ADB_SERIAL" shell getprop sys.boot_completed | tr -d '\r')" == "1" ]] \
  || { echo "error: AVD is not fully booted" >&2; exit 1; }
[[ "$($ADB -s "$ADB_SERIAL" shell getprop ro.product.cpu.abi | tr -d '\r')" == "arm64-v8a" ]] \
  || { echo "error: AVD guest ABI is not arm64-v8a" >&2; exit 1; }
[[ "$($ADB -s "$ADB_SERIAL" shell uname -m | tr -d '\r')" == "aarch64" ]] \
  || { echo "error: AVD guest machine is not aarch64" >&2; exit 1; }
DEVICE_QEMU_AVD_NAME="$($ADB -s "$ADB_SERIAL" shell getprop ro.boot.qemu.avd_name | tr -d '\r')"
[[ "$DEVICE_QEMU_AVD_NAME" == "$AVD_NAME" ]] \
  || { echo "error: ro.boot.qemu.avd_name is '$DEVICE_QEMU_AVD_NAME', not '$AVD_NAME'" >&2; exit 1; }
DEVICE_BOOT_ID="$($ADB -s "$ADB_SERIAL" shell cat /proc/sys/kernel/random/boot_id | tr -d '\r')"
[[ "$DEVICE_BOOT_ID" =~ ^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$ ]] \
  || { echo "error: emulator boot_id is missing or malformed" >&2; exit 1; }

# Record only the one relevant emulator process. A complete process listing can
# leak unrelated command-line arguments into otherwise shareable evidence.
ps -axo command= \
  | ruby "$HEADLESS_AVD_PROCESS_PROBE" "$AVD_NAME" "$QEMU" \
  >"$EVIDENCE_DIR/headless-avd-process.txt"

AVD_HOME="${ANDROID_AVD_HOME:-$HOME/.android/avd}"
AVD_CONFIG="$AVD_HOME/$AVD_NAME.avd/config.ini"
[[ -s "$AVD_CONFIG" ]] || { echo "error: AVD config is missing: $AVD_CONFIG" >&2; exit 1; }
cp "$AVD_CONFIG" "$EVIDENCE_DIR/avd-config.ini"
grep -Eq '^hw\.gpu\.mode[[:space:]]*=[[:space:]]*host$' "$AVD_CONFIG" \
  || { echo "error: AVD must use host GPU rendering" >&2; exit 1; }
IMAGE_SYSDIR="$(sed -n 's/^image\.sysdir\.1[[:space:]]*=[[:space:]]*//p' "$AVD_CONFIG" | tail -1 | tr -d '\r')"
[[ -n "$IMAGE_SYSDIR" ]] || { echo "error: AVD system image path is missing" >&2; exit 1; }
SYSTEM_IMAGE_PROPERTIES="$SDK_ROOT/$IMAGE_SYSDIR/source.properties"
[[ -s "$SYSTEM_IMAGE_PROPERTIES" ]] || { echo "error: system image source.properties is missing" >&2; exit 1; }
cp "$SYSTEM_IMAGE_PROPERTIES" "$EVIDENCE_DIR/system-image-source.properties"
grep -Eq '^SystemImage\.Abi[[:space:]]*=[[:space:]]*arm64-v8a$' "$SYSTEM_IMAGE_PROPERTIES" \
  || { echo "error: AVD system image is not arm64-v8a" >&2; exit 1; }

"$EMULATOR" -accel-check >"$EVIDENCE_DIR/emulator-accel-check.txt" 2>&1
grep -Fq 'Hypervisor.Framework' "$EVIDENCE_DIR/emulator-accel-check.txt" \
  || { echo "error: Hypervisor.Framework acceleration is not ready" >&2; exit 1; }
"$EMULATOR" -version >"$EVIDENCE_DIR/emulator-version.txt" 2>&1

BUILT_APK="$TARGET_DIR/makepad-android-apk/hepta_native/apk/hepta.apk"
APK_PATH="$EVIDENCE_DIR/Hepta-arm64-v8a.apk"
rm -rf "$TARGET_DIR/makepad-android-apk/hepta_native"
rm -f "$REPORT_PATH" "$APK_PATH"

echo "==> Building fresh current-source ARM64 APK"
CARGO_TARGET_DIR="$TARGET_DIR" \
  "$ROOT_DIR/scripts/hepta-native-mobile-cargo" android \
    --sdk-path="$SDK_ROOT" \
    --abi=aarch64 \
    --package-name="$PACKAGE_NAME" \
    --app-label="$APP_LABEL" \
    --min-sdk-version=26 \
    build -p "$CARGO_PACKAGE" --locked --release \
    >"$EVIDENCE_DIR/build.stdout.log" 2>"$EVIDENCE_DIR/build.stderr.log"
[[ -s "$BUILT_APK" ]] || { echo "error: fresh build did not create $BUILT_APK" >&2; exit 1; }
ditto "$BUILT_APK" "$APK_PATH"

SOURCE_AFTER_BUILD="$($ROOT_DIR/scripts/hepta-ui-source-fingerprint)"
jq -e --arg head "$SOURCE_HEAD" --arg tree "$SOURCE_TREE" --arg fingerprint "$SOURCE_FINGERPRINT" '
  .head == $head
  and .head_tree == $tree
  and .source_fingerprint == $fingerprint
  and .worktree_clean == true
  and .repository_worktree_clean == true
' >/dev/null <<<"$SOURCE_AFTER_BUILD" || { echo "error: source changed during Android build" >&2; exit 1; }

"$AAPT" dump badging "$APK_PATH" >"$EVIDENCE_DIR/aapt-badging.txt"
"$APKSIGNER" verify --verbose --print-certs "$APK_PATH" >"$EVIDENCE_DIR/apksigner.txt"
MANIFEST_CONTRACT="$(HEPTA_ANDROID_AAPT="$AAPT" HEPTA_NATIVE_MOBILE_TOOLS_DIR="$TOOLS_DIR" \
  ruby "$APP_DIR/tests/android_manifest_contract.rb" --apk "$APK_PATH" --aapt "$AAPT" --tools-dir "$TOOLS_DIR" --json)"
jq -e '.status == "ready" and .claims.manifest_contract_ready == true' >/dev/null <<<"$MANIFEST_CONTRACT"
printf '%s\n' "$MANIFEST_CONTRACT" >"$EVIDENCE_DIR/manifest-contract.json"

BADGING="$(<"$EVIDENCE_DIR/aapt-badging.txt")"
VERSION_CODE="$(ruby -e 'm=STDIN.read.match(/^package: .* versionCode='"'"'([0-9]+)'"'"'/); abort unless m; puts m[1]' <<<"$BADGING")"
VERSION_NAME="$(ruby -e 'm=STDIN.read.match(/^package: .* versionName='"'"'([^'"'"']+)'"'"'/); abort unless m; puts m[1]' <<<"$BADGING")"
MIN_SDK="$(sed -n "s/^sdkVersion:'\([^']*\)'$/\1/p" "$EVIDENCE_DIR/aapt-badging.txt" | tail -1)"
TARGET_SDK="$(sed -n "s/^targetSdkVersion:'\([^']*\)'$/\1/p" "$EVIDENCE_DIR/aapt-badging.txt" | tail -1)"
[[ "$MIN_SDK" == "26" && "$TARGET_SDK" == "35" ]] || { echo "error: APK SDK contract drifted" >&2; exit 1; }
grep -Fq "package: name='$PACKAGE_NAME'" "$EVIDENCE_DIR/aapt-badging.txt" || { echo "error: APK package drifted" >&2; exit 1; }
grep -Fxq "application-label:'$APP_LABEL'" "$EVIDENCE_DIR/aapt-badging.txt" || { echo "error: APK label drifted" >&2; exit 1; }
grep -Eq "^launchable-activity: name='ai\.hepta\.nativeapp\.MakepadApp' " "$EVIDENCE_DIR/aapt-badging.txt" \
  || { echo "error: APK launchable activity drifted" >&2; exit 1; }
grep -Fxq "native-code: 'arm64-v8a'" "$EVIDENCE_DIR/aapt-badging.txt" || { echo "error: APK is not ARM64-only" >&2; exit 1; }
if grep -Fxq 'application-debuggable' "$EVIDENCE_DIR/aapt-badging.txt"; then
  echo "error: canonical emulator APK must come from the non-debuggable release profile" >&2
  exit 1
fi
grep -Fxq 'Verifies' "$EVIDENCE_DIR/apksigner.txt"
grep -Fxq 'Verified using v2 scheme (APK Signature Scheme v2): true' "$EVIDENCE_DIR/apksigner.txt"
grep -Fxq 'Verified using v3 scheme (APK Signature Scheme v3): true' "$EVIDENCE_DIR/apksigner.txt"
grep -Fxq 'Number of signers: 1' "$EVIDENCE_DIR/apksigner.txt"
grep -Eq '^Signer #1 certificate DN: .*CN=Android Debug(,|$)' "$EVIDENCE_DIR/apksigner.txt"
CERTIFICATE_DN="$(sed -n 's/^Signer #1 certificate DN: //p' "$EVIDENCE_DIR/apksigner.txt" | head -1)"
CERTIFICATE_SHA256="$(sed -n 's/^Signer #1 certificate SHA-256 digest: //p' "$EVIDENCE_DIR/apksigner.txt" | head -1)"
[[ "$CERTIFICATE_SHA256" =~ ^[0-9a-f]{64}$ ]] || { echo "error: invalid debug certificate digest" >&2; exit 1; }

unzip -Z -1 "$APK_PATH" >"$EVIDENCE_DIR/archive-list.txt"
ruby -e '
  entries = File.readlines(ARGV.fetch(0), chomp: true)
  abort unless entries.count("lib/arm64-v8a/libmakepad.so") == 1
  abort unless entries.grep(%r{\Alib/[^/]+/libmakepad\.so\z}) == ["lib/arm64-v8a/libmakepad.so"]
' "$EVIDENCE_DIR/archive-list.txt"
unzip -p "$APK_PATH" lib/arm64-v8a/libmakepad.so >"$EVIDENCE_DIR/libmakepad.so"
strings "$EVIDENCE_DIR/libmakepad.so" >"$EVIDENCE_DIR/libmakepad.strings.txt"
grep -Fq "https://github.com/ProfAlexQI/Hepta/commit/$SOURCE_HEAD" "$EVIDENCE_DIR/libmakepad.strings.txt" \
  || { echo "error: APK native library is not bound to current HEAD $SOURCE_HEAD" >&2; exit 1; }

APK_SHA256="$(shasum -a 256 "$APK_PATH" | awk '{print $1}')"
APK_SIZE="$(stat -f %z "$APK_PATH")"
ADB_SHA256="$(shasum -a 256 "$ADB" | awk '{print $1}')"
EMULATOR_SHA256="$(shasum -a 256 "$EMULATOR" | awk '{print $1}')"
QEMU_SHA256="$(shasum -a 256 "$QEMU" | awk '{print $1}')"
NDK_SOURCE_PROPERTIES_SHA256="$(shasum -a 256 "$NDK_SOURCE_PROPERTIES" | awk '{print $1}')"
NDK_CLANG_SHA256="$(shasum -a 256 "$NDK_CLANG" | awk '{print $1}')"
MAKEPAD_ANDROID_JAR_SHA256="$(shasum -a 256 "$MAKEPAD_ANDROID_JAR" | awk '{print $1}')"
MAKEPAD_AAPT_SHA256="$(shasum -a 256 "$MAKEPAD_AAPT" | awk '{print $1}')"
MAKEPAD_AAPT2_SHA256="$(shasum -a 256 "$MAKEPAD_AAPT2" | awk '{print $1}')"
MAKEPAD_D8_JAR_SHA256="$(shasum -a 256 "$MAKEPAD_D8_JAR" | awk '{print $1}')"
MAKEPAD_ZIPALIGN_SHA256="$(shasum -a 256 "$MAKEPAD_ZIPALIGN" | awk '{print $1}')"
MAKEPAD_APKSIGNER_JAR_SHA256="$(shasum -a 256 "$MAKEPAD_APKSIGNER_JAR" | awk '{print $1}')"
MAKEPAD_JAVA_SHA256="$(shasum -a 256 "$MAKEPAD_JAVA" | awk '{print $1}')"
MAKEPAD_JAVAC_SHA256="$(shasum -a 256 "$MAKEPAD_JAVAC" | awk '{print $1}')"

set +e
"$ADB" -s "$ADB_SERIAL" uninstall "$PACKAGE_NAME" >"$EVIDENCE_DIR/adb-uninstall.txt" 2>&1
UNINSTALL_EXIT=$?
set -e
if [[ "$UNINSTALL_EXIT" -ne 0 ]] && ! grep -Eq 'Unknown package|not installed for 0' "$EVIDENCE_DIR/adb-uninstall.txt"; then
  echo "error: stale package removal failed" >&2
  exit 1
fi
"$ADB" -s "$ADB_SERIAL" shell pm path "$PACKAGE_NAME" >"$EVIDENCE_DIR/pm-path-after-uninstall.txt" 2>&1 || true
if grep -Fq 'package:' "$EVIDENCE_DIR/pm-path-after-uninstall.txt"; then
  echo "error: stale package remains installed after uninstall" >&2
  exit 1
fi

"$ADB" -s "$ADB_SERIAL" install --no-incremental -r "$APK_PATH" >"$EVIDENCE_DIR/adb-install.txt"
grep -Fxq 'Success' "$EVIDENCE_DIR/adb-install.txt" || { echo "error: APK install did not return Success" >&2; exit 1; }
"$ADB" -s "$ADB_SERIAL" shell pm path "$PACKAGE_NAME" >"$EVIDENCE_DIR/pm-path.txt"
grep -Fq 'package:' "$EVIDENCE_DIR/pm-path.txt" || { echo "error: installed package path is missing" >&2; exit 1; }
INSTALLED_PACKAGE_PATH="$(ruby -e '
  paths = File.readlines(ARGV.fetch(0), chomp: true).map { |line| line.delete_suffix("\r")[/\Apackage:(\/.*)\z/, 1] }.compact
  abort "expected exactly one installed base APK path" unless paths.length == 1 && paths.first.end_with?("/base.apk")
  puts paths.first
' "$EVIDENCE_DIR/pm-path.txt")"
[[ "$INSTALLED_PACKAGE_PATH" =~ ^/data/app/[0-9A-Za-z._~=/+-]+/base\.apk$ ]] \
  || { echo "error: installed package path has an unsafe shape" >&2; exit 1; }
"$ADB" -s "$ADB_SERIAL" logcat -c
"$ADB" -s "$ADB_SERIAL" shell am force-stop "$PACKAGE_NAME"
"$ADB" -s "$ADB_SERIAL" shell am start -W -S -n "$ACTIVITY" >"$EVIDENCE_DIR/am-start.txt"
grep -Fxq 'Status: ok' "$EVIDENCE_DIR/am-start.txt" || { echo "error: cold launch did not report Status: ok" >&2; exit 1; }
COLD_LAUNCH_TOTAL_MS="$(sed -n 's/^TotalTime: //p' "$EVIDENCE_DIR/am-start.txt" | tail -1)"
[[ "$COLD_LAUNCH_TOTAL_MS" =~ ^[0-9]+$ ]] || { echo "error: cold launch timing is missing" >&2; exit 1; }

APP_PID=""
for _ in $(seq 1 30); do
  APP_PID="$($ADB -s "$ADB_SERIAL" shell pidof "$PACKAGE_NAME" 2>/dev/null | tr -d '\r' | awk '{print $1}')"
  [[ "$APP_PID" =~ ^[1-9][0-9]*$ ]] && break
  sleep 1
done
[[ "$APP_PID" =~ ^[1-9][0-9]*$ ]] || { echo "error: app process did not stay alive" >&2; exit 1; }
PROCESS_START_TIME_TICKS="$($ADB -s "$ADB_SERIAL" shell cat "/proc/$APP_PID/stat" | tr -d '\r' | ruby -e '
  text = STDIN.read.strip
  match = text.match(/\A\d+ \(.*\) (.*)\z/)
  abort unless match
  fields = match[1].split
  value = Integer(fields.fetch(19), 10)
  abort unless value.positive?
  puts value
')"
[[ "$PROCESS_START_TIME_TICKS" =~ ^[1-9][0-9]*$ ]] \
  || { echo "error: app process start time is unavailable" >&2; exit 1; }

# This nonce binds delayed consumer readback to this exact emulator boot and
# producer session. It contains no credentials and deliberately remains on the
# explicitly selected emulator so a later opt-in consumer can read it back.
SESSION_PROBE_NONCE="$(uuidgen | tr '[:upper:]' '[:lower:]')"
SESSION_PROBE_SHA256="$(printf '%s' "$SESSION_PROBE_NONCE" | shasum -a 256 | awk '{print $1}')"
SESSION_PROBE_PATH="/data/local/tmp/hepta-native-smoke-${SESSION_PROBE_SHA256:0:24}"
SESSION_PROBE_LOCAL_PATH="$EVIDENCE_DIR/session-probe-nonce.txt"
printf '%s' "$SESSION_PROBE_NONCE" >"$SESSION_PROBE_LOCAL_PATH"
"$ADB" -s "$ADB_SERIAL" push "$SESSION_PROBE_LOCAL_PATH" "$SESSION_PROBE_PATH" >"$EVIDENCE_DIR/adb-session-probe-push.txt"
SESSION_PROBE_READBACK="$($ADB -s "$ADB_SERIAL" exec-out cat "$SESSION_PROBE_PATH")"
[[ "$SESSION_PROBE_READBACK" == "$SESSION_PROBE_NONCE" ]] \
  || { echo "error: emulator session nonce readback did not match" >&2; exit 1; }

source "$ROOT_DIR/scripts/hepta-native-android-emulator-lab-state-v1.sh"
ORIGINAL_ACCELEROMETER="$($ADB -s "$ADB_SERIAL" shell settings get system accelerometer_rotation | tr -d '\r')"
ORIGINAL_ROTATION="$($ADB -s "$ADB_SERIAL" shell settings get system user_rotation | tr -d '\r')"
EMULATOR_STATE_SNAPSHOT=""
if [[ "$EXTENDED_LAB" == true ]]; then
  EMULATOR_STATE_SNAPSHOT="$(hepta_android_emulator_lab_state_snapshot "$ADB" "$ADB_SERIAL")"
  hepta_android_emulator_lab_state_ready "$EMULATOR_STATE_SNAPSHOT" \
    || { echo "error: Android extended-lab rejects unreadable or frozen original battery state before mutation" >&2; exit 1; }
  ORIGINAL_FONT_SCALE="$(jq -r '.font_scale' <<<"$EMULATOR_STATE_SNAPSHOT")"
  ORIGINAL_FORCE_RTL="$(jq -r '.force_rtl' <<<"$EMULATOR_STATE_SNAPSHOT")"
  EFFECTIVE_ORIGINAL_FONT_SCALE="$ORIGINAL_FONT_SCALE"
  EFFECTIVE_ORIGINAL_FORCE_RTL="$ORIGINAL_FORCE_RTL"
  if [[ "$EFFECTIVE_ORIGINAL_FONT_SCALE" == null ]]; then EFFECTIVE_ORIGINAL_FONT_SCALE=1.0; fi
  if [[ "$EFFECTIVE_ORIGINAL_FORCE_RTL" == null ]]; then EFFECTIVE_ORIGINAL_FORCE_RTL=0; fi
fi
restore_emulator_state() {
  local failed=false
  "$ADB" -s "$ADB_SERIAL" shell input keyevent KEYCODE_BACK >/dev/null 2>&1 || failed=true
  if [[ "$EXTENDED_LAB" == true ]]; then
    hepta_android_emulator_lab_state_restore "$ADB" "$ADB_SERIAL" "$EMULATOR_STATE_SNAPSHOT" >/dev/null 2>&1 || failed=true
  else
    hepta_android_restore_setting "$ADB" "$ADB_SERIAL" system accelerometer_rotation "$ORIGINAL_ACCELEROMETER" >/dev/null 2>&1 || failed=true
    hepta_android_restore_setting "$ADB" "$ADB_SERIAL" system user_rotation "$ORIGINAL_ROTATION" >/dev/null 2>&1 || failed=true
  fi
  [[ "$failed" == false ]]
}
android_emulator_state_readback_ready() {
  if [[ "$EXTENDED_LAB" == true ]]; then
    [[ "$(hepta_android_emulator_lab_state_snapshot "$ADB" "$ADB_SERIAL" | jq -Sc .)" == "$(jq -Sc . <<<"$EMULATOR_STATE_SNAPSHOT")" ]]
  else
    [[ "$($ADB -s "$ADB_SERIAL" shell settings get system accelerometer_rotation | tr -d '\r')" == "$ORIGINAL_ACCELEROMETER" \
      && "$($ADB -s "$ADB_SERIAL" shell settings get system user_rotation | tr -d '\r')" == "$ORIGINAL_ROTATION" ]]
  fi
}
write_android_cleanup_failure_receipt() {
  local original_exit="$1" restore_ready="$2" readback_ready="$3" temporary
  temporary="$(mktemp "$(dirname "$REPORT_PATH")/.hepta-android-cleanup-failure.XXXXXX")" || return 1
  if ! hepta_mobile_cleanup_failure_json android_emulator "$PRODUCER" "$original_exit" "$restore_ready" "$readback_ready" >"$temporary"; then
    rm -f -- "$temporary"
    return 1
  fi
  mv -f -- "$temporary" "$REPORT_PATH"
}
android_cleanup() {
  local original_exit=$? restore_ready=true readback_ready=true final_exit
  trap - EXIT HUP INT TERM
  set +e
  restore_emulator_state || restore_ready=false
  android_emulator_state_readback_ready || readback_ready=false
  if [[ "$restore_ready" != true || "$readback_ready" != true ]]; then
    write_android_cleanup_failure_receipt "$original_exit" "$restore_ready" "$readback_ready" \
      || echo "error: failed to write Android cleanup failure receipt" >&2
  fi
  if [[ -n "$REPORT_TEMP" ]]; then rm -f -- "$REPORT_TEMP"; fi
  final_exit="$(hepta_mobile_cleanup_final_exit_code "$original_exit" "$restore_ready" "$readback_ready")" || final_exit=1
  exit "$final_exit"
}
trap android_cleanup EXIT
trap 'exit 129' HUP
trap 'exit 130' INT
trap 'exit 143' TERM

app_foreground() {
  local activity_dump="$1"
  "$ADB" -s "$ADB_SERIAL" shell dumpsys activity activities >"$activity_dump"
  grep -Eq "topResumedActivity=.*${PACKAGE_NAME}/\.MakepadApp" "$activity_dump" \
    && grep -Eq "mCurrentFocus=.*${PACKAGE_NAME}/${PACKAGE_NAME}\.MakepadApp" "$activity_dump" \
    && grep -Eq "mFocusedApp=.*${PACKAGE_NAME}/\.MakepadApp" "$activity_dump"
}

wait_for_orientation() {
  local wanted="$1" output="$2" probe
  for _ in $(seq 1 30); do
    "$ADB" -s "$ADB_SERIAL" shell dumpsys window displays >"$output"
    probe="$($ORIENTATION_PROBE "$output" 2>/dev/null || true)"
    if jq -e --arg wanted "$wanted" '.ready == true and .orientation == $wanted' >/dev/null 2>&1 <<<"$probe"; then
      printf '%s\n' "$probe" >"$output.orientation.json"
      return 0
    fi
    sleep 1
  done
  return 1
}

capture_png() {
  local key="$1" wanted="$2" path="$3" probe_path="$4" attempts=0 width height probe
  for attempts in $(seq 1 30); do
    "$ADB" -s "$ADB_SERIAL" exec-out screencap -p >"$path"
    if [[ -s "$path" ]] \
      && probe="$($ROOT_DIR/scripts/hepta-image-content-probe --image "$path" 2>/dev/null)" \
      && jq -e '.ready == true' >/dev/null <<<"$probe"; then
      width="$(sips -g pixelWidth "$path" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
      height="$(sips -g pixelHeight "$path" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"
      if [[ "$wanted" == portrait && "$width" -lt "$height" ]] || [[ "$wanted" == landscape && "$width" -gt "$height" ]]; then
        printf '%s\n' "$probe" >"$probe_path"
        printf '%s\n' "$attempts" >"$EVIDENCE_DIR/${key}-capture-attempts.txt"
        return 0
      fi
    fi
    sleep 1
  done
  echo "error: $key screenshot never produced ready $wanted content" >&2
  return 1
}

"$ADB" -s "$ADB_SERIAL" shell settings put system accelerometer_rotation 0
"$ADB" -s "$ADB_SERIAL" shell settings put system user_rotation 0
wait_for_orientation portrait "$EVIDENCE_DIR/dumpsys-window-displays-portrait.txt"
app_foreground "$EVIDENCE_DIR/dumpsys-activity-portrait.txt" || { echo "error: app lost focus in portrait" >&2; exit 1; }
PORTRAIT_PATH="$EVIDENCE_DIR/screenshot-portrait.png"
PORTRAIT_PROBE_PATH="$EVIDENCE_DIR/screenshot-portrait.content-probe.json"
capture_png portrait portrait "$PORTRAIT_PATH" "$PORTRAIT_PROBE_PATH"
STATUS_BAR_CONTRAST_PATH="$EVIDENCE_DIR/screenshot-portrait.system-bar-contrast.json"
"$SYSTEM_BAR_CONTRAST_PROBE" --image "$PORTRAIT_PATH" --output "$STATUS_BAR_CONTRAST_PATH" \
  || { echo "error: Android top/bottom system-bar light-icon contrast probe failed" >&2; exit 1; }
PORTRAIT_TEMPLATE_PATH="$LOGIN_TEMPLATE_DIR/portrait.png"
PORTRAIT_TEMPLATE_REPORT_PATH="$EVIDENCE_DIR/screenshot-portrait.login-template-probe.json"
"$ROOT_DIR/scripts/hepta-android-login-template-probe" \
  --image "$PORTRAIT_PATH" --template "$PORTRAIT_TEMPLATE_PATH" --mode portrait \
  --output "$PORTRAIT_TEMPLATE_REPORT_PATH" >/dev/null

"$ADB" -s "$ADB_SERIAL" shell uiautomator dump /sdcard/hepta-ui-uiautomator.xml >"$EVIDENCE_DIR/uiautomator-command.txt"
"$ADB" -s "$ADB_SERIAL" exec-out cat /sdcard/hepta-ui-uiautomator.xml >"$EVIDENCE_DIR/uiautomator-portrait.xml"
"$ADB" -s "$ADB_SERIAL" shell rm -f /sdcard/hepta-ui-uiautomator.xml
xmllint --noout "$EVIDENCE_DIR/uiautomator-portrait.xml"
read -r UI_NODE_COUNT UI_VISIBLE_NODE_COUNT UI_PACKAGE_NODE_COUNT UI_LABELED_NODE_COUNT < <(ruby -rrexml/document -e '
  doc = REXML::Document.new(File.binread(ARGV.fetch(0)))
  nodes = []
  REXML::XPath.each(doc, "//node") { |node| nodes << node }
  visible = nodes.count do |node|
    match = node.attributes["bounds"].to_s.match(/\[(\d+),(\d+)\]\[(\d+),(\d+)\]/)
    match && match[3].to_i > match[1].to_i && match[4].to_i > match[2].to_i
  end
  package_nodes = nodes.count { |node| node.attributes["package"] == "ai.hepta.nativeapp" }
  labeled = nodes.count { |node| [node.attributes["text"], node.attributes["content-desc"]].any? { |value| !value.to_s.strip.empty? } }
  abort unless nodes.length > 0 && visible > 0 && package_nodes > 0
  puts [nodes.length, visible, package_nodes, labeled].join(" ")
' "$EVIDENCE_DIR/uiautomator-portrait.xml")
UIAUTOMATOR_SHA256="$(shasum -a 256 "$EVIDENCE_DIR/uiautomator-portrait.xml" | awk '{print $1}')"

"$ADB" -s "$ADB_SERIAL" shell settings put system user_rotation 1
wait_for_orientation landscape "$EVIDENCE_DIR/dumpsys-window-displays-landscape.txt"
app_foreground "$EVIDENCE_DIR/dumpsys-activity-landscape.txt" || { echo "error: app lost focus in landscape" >&2; exit 1; }
LANDSCAPE_PATH="$EVIDENCE_DIR/screenshot-landscape.png"
LANDSCAPE_PROBE_PATH="$EVIDENCE_DIR/screenshot-landscape.content-probe.json"
capture_png landscape landscape "$LANDSCAPE_PATH" "$LANDSCAPE_PROBE_PATH"
LANDSCAPE_TEMPLATE_PATH="$LOGIN_TEMPLATE_DIR/landscape.png"
LANDSCAPE_TEMPLATE_REPORT_PATH="$EVIDENCE_DIR/screenshot-landscape.login-template-probe.json"
"$ROOT_DIR/scripts/hepta-android-login-template-probe" \
  --image "$LANDSCAPE_PATH" --template "$LANDSCAPE_TEMPLATE_PATH" --mode landscape \
  --output "$LANDSCAPE_TEMPLATE_REPORT_PATH" >/dev/null

"$ADB" -s "$ADB_SERIAL" shell settings put system user_rotation 0
wait_for_orientation portrait "$EVIDENCE_DIR/dumpsys-window-displays-ime.txt"
PORTRAIT_WIDTH="$(sips -g pixelWidth "$PORTRAIT_PATH" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
PORTRAIT_HEIGHT="$(sips -g pixelHeight "$PORTRAIT_PATH" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"
TAP_X=$((PORTRAIT_WIDTH / 2))
TAP_Y=$((PORTRAIT_HEIGHT * 32 / 100))
"$ADB" -s "$ADB_SERIAL" shell input tap "$TAP_X" "$TAP_Y"
IME_READY=false
for _ in $(seq 1 30); do
  "$ADB" -s "$ADB_SERIAL" shell dumpsys input_method >"$EVIDENCE_DIR/dumpsys-input-method-ime.txt"
  if grep -Fq 'mIsInputViewShown=true' "$EVIDENCE_DIR/dumpsys-input-method-ime.txt" \
    && grep -Eq "(focusedWindowName|requestWindowName)=.*${PACKAGE_NAME}/${PACKAGE_NAME}\.MakepadApp" "$EVIDENCE_DIR/dumpsys-input-method-ime.txt"; then
    IME_READY=true
    break
  fi
  sleep 1
done
[[ "$IME_READY" == true ]] || { echo "error: Makepad homeserver tap did not show an IME targeted at Hepta" >&2; exit 1; }
app_foreground "$EVIDENCE_DIR/dumpsys-activity-ime.txt" || { echo "error: app lost focus while IME was shown" >&2; exit 1; }
IME_PATH="$EVIDENCE_DIR/screenshot-portrait-ime.png"
IME_PROBE_PATH="$EVIDENCE_DIR/screenshot-portrait-ime.content-probe.json"
capture_png ime portrait "$IME_PATH" "$IME_PROBE_PATH"
IME_TEMPLATE_PATH="$LOGIN_TEMPLATE_DIR/ime.png"
IME_TEMPLATE_REPORT_PATH="$EVIDENCE_DIR/screenshot-portrait-ime.login-template-probe.json"
"$ROOT_DIR/scripts/hepta-android-login-template-probe" \
  --image "$IME_PATH" --template "$IME_TEMPLATE_PATH" --mode ime \
  --output "$IME_TEMPLATE_REPORT_PATH" >/dev/null

LAB_RESULT='{"requested":false,"status":"not_requested","ready":false}'
if [[ "$EXTENDED_LAB" == true ]]; then
  cold_launch_samples() {
    local label="$1" samples='[]' output total pid
    for ((sample = 1; sample <= LAB_STARTUP_SAMPLES; sample++)); do
      output="$EVIDENCE_DIR/lab-${label}-startup-${sample}.txt"
      "$ADB" -s "$ADB_SERIAL" shell am force-stop "$PACKAGE_NAME"
      "$ADB" -s "$ADB_SERIAL" shell am start -W -S -n "$ACTIVITY" >"$output"
      grep -Fxq 'Status: ok' "$output"
      total="$(sed -n 's/^TotalTime: //p' "$output" | tail -1)"
      pid="$($ADB -s "$ADB_SERIAL" shell pidof "$PACKAGE_NAME" | tr -d '\r')"
      [[ "$total" =~ ^[0-9]+$ && "$pid" =~ ^[1-9][0-9]*$ ]]
      app_foreground "$EVIDENCE_DIR/lab-${label}-startup-${sample}.activity.txt"
      samples="$(jq -c --argjson sample "$sample" --argjson pid "$pid" --argjson total "$total" \
        '. + [{sample:$sample,pid:$pid,total_time_ms:$total}]' <<<"$samples")"
    done
    printf '%s\n' "$samples"
  }
  startup_stats() {
    jq -c '([.[].total_time_ms] | sort) as $v | {samples:length,min_ms:$v[0],median_ms:$v[(length/2|floor)],max_ms:$v[-1]}' <<<"$1"
  }

  "$ADB" -s "$ADB_SERIAL" shell input keyevent KEYCODE_BACK
  "$ADB" -s "$ADB_SERIAL" shell settings put system user_rotation 0
  wait_for_orientation portrait "$EVIDENCE_DIR/lab-dumpsys-window-portrait.txt"

  "$ADB" -s "$ADB_SERIAL" shell settings put global debug.force_rtl 0
  [[ "$($ADB -s "$ADB_SERIAL" shell settings get global debug.force_rtl | tr -d '\r')" == 0 ]] \
    || { echo "error: Android matched-control LTR setting did not apply" >&2; exit 1; }
  "$ADB" -s "$ADB_SERIAL" shell am force-stop "$PACKAGE_NAME"
  "$ADB" -s "$ADB_SERIAL" shell am start -W -S -n "$ACTIVITY" >"$EVIDENCE_DIR/lab-rtl-control-start.txt"
  grep -Fxq 'Status: ok' "$EVIDENCE_DIR/lab-rtl-control-start.txt"
  RTL_CONTROL_PATH="$EVIDENCE_DIR/lab-rtl-control-ltr.png"
  capture_png lab-rtl-control portrait "$RTL_CONTROL_PATH" "$EVIDENCE_DIR/lab-rtl-control.content-probe.json"
  RTL_CONTROL_SHA="$(shasum -a 256 "$RTL_CONTROL_PATH" | awk '{print $1}')"
  RTL_CONTROL_WIDTH="$(sips -g pixelWidth "$RTL_CONTROL_PATH" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
  RTL_CONTROL_HEIGHT="$(sips -g pixelHeight "$RTL_CONTROL_PATH" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"

  "$ADB" -s "$ADB_SERIAL" shell settings put global debug.force_rtl 1
  [[ "$($ADB -s "$ADB_SERIAL" shell settings get global debug.force_rtl | tr -d '\r')" == 1 ]] \
    || { echo "error: Android force-RTL setting did not apply" >&2; exit 1; }
  "$ADB" -s "$ADB_SERIAL" shell am force-stop "$PACKAGE_NAME"
  "$ADB" -s "$ADB_SERIAL" shell am start -W -S -n "$ACTIVITY" >"$EVIDENCE_DIR/lab-rtl-start.txt"
  grep -Fxq 'Status: ok' "$EVIDENCE_DIR/lab-rtl-start.txt"
  RTL_PATH="$EVIDENCE_DIR/lab-rtl.png"
  capture_png lab-rtl portrait "$RTL_PATH" "$EVIDENCE_DIR/lab-rtl.content-probe.json"
  RTL_SHA="$(shasum -a 256 "$RTL_PATH" | awk '{print $1}')"
  RTL_WIDTH="$(sips -g pixelWidth "$RTL_PATH" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
  RTL_HEIGHT="$(sips -g pixelHeight "$RTL_PATH" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"
  [[ "$RTL_SHA" != "$RTL_CONTROL_SHA" ]] && RTL_RASTER_CHANGED=true || RTL_RASTER_CHANGED=false
  [[ "$RTL_WIDTH" == "$RTL_CONTROL_WIDTH" && "$RTL_HEIGHT" == "$RTL_CONTROL_HEIGHT" ]] \
    && RTL_SAME_CANVAS=true || RTL_SAME_CANVAS=false
  hepta_android_restore_setting "$ADB" "$ADB_SERIAL" global debug.force_rtl "$ORIGINAL_FORCE_RTL"
  [[ "$($ADB -s "$ADB_SERIAL" shell settings get global debug.force_rtl | tr -d '\r')" == "$ORIGINAL_FORCE_RTL" ]] \
    || { echo "error: Android force-RTL setting did not restore" >&2; exit 1; }

  "$ADB" -s "$ADB_SERIAL" shell settings put system font_scale 1.0
  [[ "$($ADB -s "$ADB_SERIAL" shell settings get system font_scale | tr -d '\r')" == 1.0 ]] \
    || { echo "error: Android matched-control font scale did not apply" >&2; exit 1; }
  "$ADB" -s "$ADB_SERIAL" shell am force-stop "$PACKAGE_NAME"
  "$ADB" -s "$ADB_SERIAL" shell am start -W -S -n "$ACTIVITY" >"$EVIDENCE_DIR/lab-font-scale-control-start.txt"
  grep -Fxq 'Status: ok' "$EVIDENCE_DIR/lab-font-scale-control-start.txt"
  FONT_SCALE_CONTROL_PATH="$EVIDENCE_DIR/lab-font-scale-control.png"
  capture_png lab-font-scale-control portrait "$FONT_SCALE_CONTROL_PATH" "$EVIDENCE_DIR/lab-font-scale-control.content-probe.json"
  FONT_SCALE_CONTROL_SHA="$(shasum -a 256 "$FONT_SCALE_CONTROL_PATH" | awk '{print $1}')"
  FONT_SCALE_CONTROL_WIDTH="$(sips -g pixelWidth "$FONT_SCALE_CONTROL_PATH" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
  FONT_SCALE_CONTROL_HEIGHT="$(sips -g pixelHeight "$FONT_SCALE_CONTROL_PATH" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"

  FONT_SCALE="1.5"
  "$ADB" -s "$ADB_SERIAL" shell settings put system font_scale "$FONT_SCALE"
  [[ "$($ADB -s "$ADB_SERIAL" shell settings get system font_scale | tr -d '\r')" == "$FONT_SCALE" ]] \
    || { echo "error: Android font-scale setting did not apply" >&2; exit 1; }
  "$ADB" -s "$ADB_SERIAL" shell am force-stop "$PACKAGE_NAME"
  "$ADB" -s "$ADB_SERIAL" shell am start -W -S -n "$ACTIVITY" >"$EVIDENCE_DIR/lab-font-scale-start.txt"
  grep -Fxq 'Status: ok' "$EVIDENCE_DIR/lab-font-scale-start.txt"
  FONT_SCALE_PATH="$EVIDENCE_DIR/lab-font-scale-1.5.png"
  capture_png lab-font-scale portrait "$FONT_SCALE_PATH" "$EVIDENCE_DIR/lab-font-scale.content-probe.json"
  FONT_SCALE_SHA="$(shasum -a 256 "$FONT_SCALE_PATH" | awk '{print $1}')"
  FONT_SCALE_WIDTH="$(sips -g pixelWidth "$FONT_SCALE_PATH" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
  FONT_SCALE_HEIGHT="$(sips -g pixelHeight "$FONT_SCALE_PATH" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"
  [[ "$FONT_SCALE_SHA" != "$FONT_SCALE_CONTROL_SHA" ]] && FONT_SCALE_RASTER_CHANGED=true || FONT_SCALE_RASTER_CHANGED=false
  [[ "$FONT_SCALE_WIDTH" == "$FONT_SCALE_CONTROL_WIDTH" && "$FONT_SCALE_HEIGHT" == "$FONT_SCALE_CONTROL_HEIGHT" ]] \
    && FONT_SCALE_SAME_CANVAS=true || FONT_SCALE_SAME_CANVAS=false
  hepta_android_restore_setting "$ADB" "$ADB_SERIAL" system font_scale "$ORIGINAL_FONT_SCALE"
  [[ "$($ADB -s "$ADB_SERIAL" shell settings get system font_scale | tr -d '\r')" == "$ORIGINAL_FONT_SCALE" ]] \
    || { echo "error: Android font-scale setting did not restore" >&2; exit 1; }

  "$ADB" -s "$ADB_SERIAL" shell cmd battery reset >/dev/null
  "$ADB" -s "$ADB_SERIAL" shell cmd power set-mode 0 >/dev/null
  [[ "$($ADB -s "$ADB_SERIAL" shell settings get global low_power | tr -d '\r')" == 0 ]]
  NORMAL_STARTUP_SAMPLES="$(cold_launch_samples normal)"
  jq -e --argjson count "$LAB_STARTUP_SAMPLES" 'length == $count and all(.[]; .total_time_ms >= 0 and .total_time_ms <= 10000)' \
    >/dev/null <<<"$NORMAL_STARTUP_SAMPLES"
  "$ADB" -s "$ADB_SERIAL" shell cmd battery unplug >/dev/null
  "$ADB" -s "$ADB_SERIAL" shell cmd battery set level 15 >/dev/null
  "$ADB" -s "$ADB_SERIAL" shell cmd power set-mode 1 >/dev/null
  [[ "$($ADB -s "$ADB_SERIAL" shell settings get global low_power | tr -d '\r')" == 1 ]] \
    || { echo "error: Android emulator low-power simulation did not apply" >&2; exit 1; }
  SIMULATED_BATTERY_STATE="$(hepta_android_battery_state_json "$ADB" "$ADB_SERIAL")"
  jq -e '.level == 15' >/dev/null <<<"$SIMULATED_BATTERY_STATE"
  LOW_POWER_STARTUP_SAMPLES="$(cold_launch_samples simulated-low-power)"
  jq -e --argjson count "$LAB_STARTUP_SAMPLES" 'length == $count and all(.[]; .total_time_ms >= 0 and .total_time_ms <= 10000)' \
    >/dev/null <<<"$LOW_POWER_STARTUP_SAMPLES"
  LOW_POWER_PATH="$EVIDENCE_DIR/lab-simulated-low-power.png"
  capture_png lab-simulated-low-power portrait "$LOW_POWER_PATH" "$EVIDENCE_DIR/lab-simulated-low-power.content-probe.json"

  LAB_RESTORE_COMMAND_READY=true
  LAB_RESTORE_READBACK_READY=true
  restore_emulator_state || LAB_RESTORE_COMMAND_READY=false
  android_emulator_state_readback_ready || LAB_RESTORE_READBACK_READY=false
  if [[ "$LAB_RESTORE_COMMAND_READY" != true || "$LAB_RESTORE_READBACK_READY" != true ]]; then
    write_android_cleanup_failure_receipt 1 "$LAB_RESTORE_COMMAND_READY" "$LAB_RESTORE_READBACK_READY" \
      || echo "error: failed to write Android cleanup failure receipt" >&2
    echo "error: Android extended-lab state restoration/readback failed" >&2
    exit 1
  fi

  "$ADB" -s "$ADB_SERIAL" shell am force-stop "$PACKAGE_NAME"
  "$ADB" -s "$ADB_SERIAL" shell am start -W -S -n "$ACTIVITY" >"$EVIDENCE_DIR/lab-restored-final-start.txt"
  grep -Fxq 'Status: ok' "$EVIDENCE_DIR/lab-restored-final-start.txt"
  APP_PID="$($ADB -s "$ADB_SERIAL" shell pidof "$PACKAGE_NAME" | tr -d '\r')"
  PROCESS_START_TIME_TICKS="$($ADB -s "$ADB_SERIAL" shell cat "/proc/$APP_PID/stat" | tr -d '\r' | ruby -e '
    text = STDIN.read.strip; match = text.match(/\A\d+ \(.*\) (.*)\z/); abort unless match
    value = Integer(match[1].split.fetch(19), 10); abort unless value.positive?; puts value
  ')"
  app_foreground "$EVIDENCE_DIR/lab-restored-final.activity.txt"

  LAB_RESULT="$(jq -n \
    --arg rtl_control_path "$RTL_CONTROL_PATH" --arg rtl_control_sha "$RTL_CONTROL_SHA" \
    --arg rtl_path "$RTL_PATH" --arg rtl_sha "$RTL_SHA" \
    --arg font_control_path "$FONT_SCALE_CONTROL_PATH" --arg font_control_sha "$FONT_SCALE_CONTROL_SHA" \
    --arg font_path "$FONT_SCALE_PATH" --arg font_sha "$FONT_SCALE_SHA" \
    --arg low_power_path "$LOW_POWER_PATH" --arg font_scale "$FONT_SCALE" \
    --arg original_rtl_raw "$ORIGINAL_FORCE_RTL" --arg original_rtl_effective "$EFFECTIVE_ORIGINAL_FORCE_RTL" \
    --arg original_font_raw "$ORIGINAL_FONT_SCALE" --arg original_font_effective "$EFFECTIVE_ORIGINAL_FONT_SCALE" \
    --argjson rtl_changed "$RTL_RASTER_CHANGED" --argjson rtl_same_canvas "$RTL_SAME_CANVAS" \
    --argjson font_changed "$FONT_SCALE_RASTER_CHANGED" --argjson font_same_canvas "$FONT_SCALE_SAME_CANVAS" \
    --argjson normal_samples "$NORMAL_STARTUP_SAMPLES" --argjson normal_stats "$(startup_stats "$NORMAL_STARTUP_SAMPLES")" \
    --argjson low_samples "$LOW_POWER_STARTUP_SAMPLES" --argjson low_stats "$(startup_stats "$LOW_POWER_STARTUP_SAMPLES")" \
    --argjson simulated_battery "$SIMULATED_BATTERY_STATE" '
      {
        requested:true,status:"executed_with_product_claim_blockers",execution_ready:true,ready:false,state_restore_verified:true,
        modes:{
          rtl:{executed:true,original_setting:{raw:$original_rtl_raw,effective:$original_rtl_effective},force_rtl_readback:true,matched_control:{path:$rtl_control_path,sha256:$rtl_control_sha,force_rtl:0,writing_direction:"left_to_right"},capture:{path:$rtl_path,sha256:$rtl_sha},raster_changed:$rtl_changed,mode_attributable_raster_change:$rtl_changed,geometry_comparison:{same_canvas:$rtl_same_canvas,semantic_layout_verified:false},ready:false},
          font_scale:{executed:true,original_setting:{raw:$original_font_raw,effective:$original_font_effective},requested_scale:($font_scale|tonumber),setting_readback_ready:true,matched_control:{path:$font_control_path,sha256:$font_control_sha,font_scale:1.0},capture:{path:$font_path,sha256:$font_sha},raster_changed:$font_changed,mode_attributable_raster_change:$font_changed,geometry_comparison:{same_canvas:$font_same_canvas,semantic_text_reflow_verified:false},ready:false},
          rotation_ime:{executed:true,scope:"unauthenticated_login_surface",portrait_landscape_portrait:true,ime_targeted_at_hepta:true,ready:true,generic_app_wide_ready:false},
          startup_performance:{executed:true,scope:"am_start_total_time_on_unauthenticated_emulator",normal:{samples:$normal_samples,statistics:$normal_stats},ready:true},
          low_power:{executed:true,simulation_backend:"adb_cmd_battery_plus_cmd_power",emulator_only:true,real_low_power_qualification:false,battery_state:$simulated_battery,artifact:$low_power_path,startup:{samples:$low_samples,statistics:$low_stats},ready:false}
        },
        promotion:{
          eligible:false,
          canonical_leaf_artifacts_rehashed:false,
          matched_control_leaf_artifacts_rehashed:false
        },
        claims:{android_rtl_ready:false,android_dynamic_type_ready:false,android_safe_area_ready:false,android_rotation_ready:false,android_ime_ready:false,android_low_power_performance_ready:false,android_real_device_ready:false,talkback_ready:false},
        blockers:[
          {code:"android_extended_lab_leaf_artifact_rehash_missing",requires:"canonical consumer rehashes every mode capture and matched-control artifact before promotion",observed:false},
          {code:"android_rtl_semantic_layout_verification_missing",requires:"semantic mirrored layout and interaction evidence",observed:{force_rtl:true,matched_control:true,mode_attributable_raster_change:$rtl_changed,same_canvas:$rtl_same_canvas}},
          {code:"android_font_scale_semantic_response_verification_missing",requires:"accessible text reflow and interaction evidence",observed:{font_scale:($font_scale|tonumber),matched_control:true,mode_attributable_raster_change:$font_changed,same_canvas:$font_same_canvas}},
          {code:"android_generic_safe_area_rotation_ime_scope_missing",requires:"authenticated app-wide coverage",observed:{unauthenticated_login_rotation_ime:true}},
          {code:"android_real_device_low_power_performance_receipt_missing",requires:"physical-device effective low-power evidence",observed:{emulator_power_simulation_only:true}},
          {code:"android_real_device_receipt_missing",requires:"explicit physical-device lab receipt",observed:false},
          {code:"talkback_receipt_missing",requires:"real provider/action roundtrip on selected device",observed:false}
        ],
        forbidden_actions_performed:{credential_supply:false,real_device_contact:false,account_connection:false,sdk_or_runtime_download:false,avd_create_or_boot:false,release_sign:false,upload:false},
        external_side_effects_performed:false
      }
    ')"
fi

PORTRAIT_SHA256="$(shasum -a 256 "$PORTRAIT_PATH" | awk '{print $1}')"
LANDSCAPE_SHA256="$(shasum -a 256 "$LANDSCAPE_PATH" | awk '{print $1}')"
IME_SHA256="$(shasum -a 256 "$IME_PATH" | awk '{print $1}')"
[[ "$(printf '%s\n' "$PORTRAIT_SHA256" "$LANDSCAPE_SHA256" "$IME_SHA256" | sort -u | wc -l | tr -d ' ')" == "3" ]] \
  || { echo "error: portrait, landscape, and IME screenshots must be byte-distinct" >&2; exit 1; }
LANDSCAPE_WIDTH="$(sips -g pixelWidth "$LANDSCAPE_PATH" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
LANDSCAPE_HEIGHT="$(sips -g pixelHeight "$LANDSCAPE_PATH" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"
IME_WIDTH="$(sips -g pixelWidth "$IME_PATH" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
IME_HEIGHT="$(sips -g pixelHeight "$IME_PATH" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"
PORTRAIT_ATTEMPTS="$(<"$EVIDENCE_DIR/portrait-capture-attempts.txt")"
LANDSCAPE_ATTEMPTS="$(<"$EVIDENCE_DIR/landscape-capture-attempts.txt")"
IME_ATTEMPTS="$(<"$EVIDENCE_DIR/ime-capture-attempts.txt")"

"$ADB" -s "$ADB_SERIAL" shell dumpsys SurfaceFlinger >"$EVIDENCE_DIR/dumpsys-surfaceflinger.txt"
GLES_LINE="$(grep -m1 '^GLES:' "$EVIDENCE_DIR/dumpsys-surfaceflinger.txt" || true)"
[[ "$GLES_LINE" == *'Google (Apple)'* && "$GLES_LINE" == *'Android Emulator OpenGL ES Translator (Apple'* && "$GLES_LINE" == *'Metal'* ]] \
  || { echo "error: emulator renderer is not the canonical Apple host/Metal path" >&2; exit 1; }
RENDERER_VENDOR="$(ruby -e 'm=STDIN.read.match(/^GLES:\s*([^,]+(?:\([^)]*\))?),\s*([^,]+),\s*(.*)$/); abort unless m; puts m[1]' <<<"$GLES_LINE")"
RENDERER_ADAPTER="$(ruby -e 'm=STDIN.read.match(/^GLES:\s*([^,]+(?:\([^)]*\))?),\s*([^,]+),\s*(.*)$/); abort unless m; puts m[2]' <<<"$GLES_LINE")"
RENDERER_API_BACKEND="$(ruby -e 'm=STDIN.read.match(/^GLES:\s*([^,]+(?:\([^)]*\))?),\s*([^,]+),\s*(.*)$/); abort unless m; puts m[3]' <<<"$GLES_LINE")"

"$ADB" -s "$ADB_SERIAL" shell dumpsys activity activities >"$EVIDENCE_DIR/dumpsys-activity-final.txt"
app_foreground "$EVIDENCE_DIR/dumpsys-activity-final-check.txt" || { echo "error: app was not foreground/focused at final verification" >&2; exit 1; }
"$ADB" -s "$ADB_SERIAL" logcat -d >"$EVIDENCE_DIR/logcat-final.txt"
"$ADB" -s "$ADB_SERIAL" logcat -b crash -d >"$EVIDENCE_DIR/logcat-crash-buffer.txt"
FATAL_COUNT="$(rg -c 'FATAL EXCEPTION' "$EVIDENCE_DIR/logcat-final.txt" || true)"
ANR_COUNT="$(rg -c "ANR in $PACKAGE_NAME" "$EVIDENCE_DIR/logcat-final.txt" || true)"
BGRA_COUNT="$(rg -ci 'GL_BGRA|bgra.*(invalid|error)|invalid.*bgra' "$EVIDENCE_DIR/logcat-final.txt" || true)"
[[ -z "$FATAL_COUNT" ]] && FATAL_COUNT=0
[[ -z "$ANR_COUNT" ]] && ANR_COUNT=0
[[ -z "$BGRA_COUNT" ]] && BGRA_COUNT=0
[[ "$FATAL_COUNT" == 0 && "$ANR_COUNT" == 0 && "$BGRA_COUNT" == 0 ]] || { echo "error: fatal/ANR/BGRA runtime marker found" >&2; exit 1; }
[[ ! -s "$EVIDENCE_DIR/logcat-crash-buffer.txt" ]] || { echo "error: Android crash buffer is not empty" >&2; exit 1; }

ACCESSIBILITY_ENABLED="$($ADB -s "$ADB_SERIAL" shell settings get secure accessibility_enabled | tr -d '\r')"
ACCESSIBILITY_SERVICES="$($ADB -s "$ADB_SERIAL" shell settings get secure enabled_accessibility_services | tr -d '\r')"
PHYSICAL_SIZE="$($ADB -s "$ADB_SERIAL" shell wm size | tr -d '\r')"
PHYSICAL_DENSITY="$($ADB -s "$ADB_SERIAL" shell wm density | tr -d '\r')"
DEVICE_MODEL="$($ADB -s "$ADB_SERIAL" shell getprop ro.product.model | tr -d '\r')"
ANDROID_VERSION="$($ADB -s "$ADB_SERIAL" shell getprop ro.build.version.release | tr -d '\r')"
API_LEVEL="$($ADB -s "$ADB_SERIAL" shell getprop ro.build.version.sdk | tr -d '\r')"
read -r PHYSICAL_WIDTH PHYSICAL_HEIGHT < <(ruby -e 'm=STDIN.read.match(/Physical size:\s*(\d+)x(\d+)/); abort unless m; puts "#{m[1]} #{m[2]}"' <<<"$PHYSICAL_SIZE")
DENSITY_DPI="$(ruby -e 'm=STDIN.read.match(/Physical density:\s*(\d+)/); abort unless m; puts m[1]' <<<"$PHYSICAL_DENSITY")"
SYSTEM_IMAGE_DESCRIPTION="$(sed -n 's/^Pkg\.Desc[[:space:]]*=[[:space:]]*//p' "$SYSTEM_IMAGE_PROPERTIES" | tail -1)"
[[ "$SYSTEM_IMAGE_DESCRIPTION" == *"ARM 64"* || "$SYSTEM_IMAGE_DESCRIPTION" == *"arm64"* ]] \
  || SYSTEM_IMAGE_DESCRIPTION="$SYSTEM_IMAGE_DESCRIPTION arm64-v8a"

# The build/capture phase can be long. Rebind the ready receipt to the same
# emulator boot and exact app process immediately before source/receipt output.
FINAL_DEVICE_STATE="$($ADB -s "$ADB_SERIAL" get-state | tr -d '\r')"
FINAL_DEVICE_AVD="$($ADB -s "$ADB_SERIAL" emu avd name | tr -d '\r' | sed '/^OK$/d' | sed '/^[[:space:]]*$/d')"
FINAL_QEMU_AVD_NAME="$($ADB -s "$ADB_SERIAL" shell getprop ro.boot.qemu.avd_name | tr -d '\r')"
FINAL_BOOT_ID="$($ADB -s "$ADB_SERIAL" shell cat /proc/sys/kernel/random/boot_id | tr -d '\r')"
FINAL_APP_PID="$($ADB -s "$ADB_SERIAL" shell pidof "$PACKAGE_NAME" | tr -d '\r')"
[[ "$FINAL_APP_PID" =~ ^[1-9][0-9]*$ ]] \
  || { echo "error: app does not have exactly one primary PID at final verification" >&2; exit 1; }
FINAL_PROCESS_START_TIME_TICKS="$($ADB -s "$ADB_SERIAL" shell cat "/proc/$FINAL_APP_PID/stat" | tr -d '\r' | ruby -e '
  text = STDIN.read.strip
  match = text.match(/\A\d+ \(.*\) (.*)\z/)
  abort unless match
  fields = match[1].split
  value = Integer(fields.fetch(19), 10)
  abort unless value.positive?
  puts value
')"
[[ "$FINAL_DEVICE_STATE" == "device" \
  && "$FINAL_DEVICE_AVD" == "$DEVICE_AVD" \
  && "$FINAL_QEMU_AVD_NAME" == "$DEVICE_QEMU_AVD_NAME" \
  && "$FINAL_BOOT_ID" == "$DEVICE_BOOT_ID" \
  && "$FINAL_APP_PID" == "$APP_PID" \
  && "$FINAL_PROCESS_START_TIME_TICKS" == "$PROCESS_START_TIME_TICKS" ]] \
  || { echo "error: emulator boot or Hepta process changed during Android emulator smoke" >&2; exit 1; }

SOURCE_FINAL="$($ROOT_DIR/scripts/hepta-ui-source-fingerprint)"
jq -e --arg head "$SOURCE_HEAD" --arg tree "$SOURCE_TREE" --arg fingerprint "$SOURCE_FINGERPRINT" '
  .head == $head
  and .head_tree == $tree
  and .source_fingerprint == $fingerprint
  and .worktree_clean == true
  and .repository_worktree_clean == true
' >/dev/null <<<"$SOURCE_FINAL" || { echo "error: source changed during Android emulator smoke" >&2; exit 1; }

PORTRAIT_PROBE="$(jq '{path:$path,status,ready,non_black_ratio:.sample.non_black_ratio,luma_span:.sample.luma_span,luma_bucket_count:.sample.luma_bucket_count}' --arg path "$PORTRAIT_PROBE_PATH" "$PORTRAIT_PROBE_PATH")"
STATUS_BAR_CONTRAST_SHA256="$(shasum -a 256 "$STATUS_BAR_CONTRAST_PATH" | awk '{print $1}')"
STATUS_BAR_CONTRAST="$(jq --arg path "$STATUS_BAR_CONTRAST_PATH" --arg sha "$STATUS_BAR_CONTRAST_SHA256" '. + {evidence_path:$path,evidence_sha256:$sha}' "$STATUS_BAR_CONTRAST_PATH")"
LANDSCAPE_PROBE="$(jq '{path:$path,status,ready,non_black_ratio:.sample.non_black_ratio,luma_span:.sample.luma_span,luma_bucket_count:.sample.luma_bucket_count}' --arg path "$LANDSCAPE_PROBE_PATH" "$LANDSCAPE_PROBE_PATH")"
IME_PROBE="$(jq '{path:$path,status,ready,non_black_ratio:.sample.non_black_ratio,luma_span:.sample.luma_span,luma_bucket_count:.sample.luma_bucket_count}' --arg path "$IME_PROBE_PATH" "$IME_PROBE_PATH")"
PORTRAIT_TEMPLATE_PROBE="$(jq --arg path "$PORTRAIT_TEMPLATE_REPORT_PATH" '. + {evidence_path:$path}' "$PORTRAIT_TEMPLATE_REPORT_PATH")"
LANDSCAPE_TEMPLATE_PROBE="$(jq --arg path "$LANDSCAPE_TEMPLATE_REPORT_PATH" '. + {evidence_path:$path}' "$LANDSCAPE_TEMPLATE_REPORT_PATH")"
IME_TEMPLATE_PROBE="$(jq --arg path "$IME_TEMPLATE_REPORT_PATH" '. + {evidence_path:$path}' "$IME_TEMPLATE_REPORT_PATH")"
LOGIN_TEMPLATE_MANIFEST_SHA256="$(shasum -a 256 "$LOGIN_TEMPLATE_MANIFEST" | awk '{print $1}')"
HOST_OS="macOS $(sw_vers -productVersion) ($(sw_vers -buildVersion))"
REPORT_TEMP="$(mktemp "$(dirname "$REPORT_PATH")/.hepta-android-emulator-smoke.XXXXXX")"

jq -n \
  --arg captured_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg producer "$PRODUCER" \
  --argjson source_binding "$SOURCE_FINAL" \
  --arg artifact_path "$APK_PATH" \
  --arg artifact_sha256 "$APK_SHA256" \
  --argjson artifact_size "$APK_SIZE" \
  --argjson version_code "$VERSION_CODE" \
  --arg version_name "$VERSION_NAME" \
  --arg certificate_dn "$CERTIFICATE_DN" \
  --arg certificate_sha256 "$CERTIFICATE_SHA256" \
  --arg adb_path "$ADB" \
  --arg adb_sha256 "$ADB_SHA256" \
  --arg serial "$ADB_SERIAL" \
  --arg avd_name "$AVD_NAME" \
  --arg qemu_avd_name "$DEVICE_QEMU_AVD_NAME" \
  --arg boot_id "$DEVICE_BOOT_ID" \
  --arg device_model "$DEVICE_MODEL" \
  --arg android_version "$ANDROID_VERSION" \
  --argjson api_level "$API_LEVEL" \
  --arg system_image "$SYSTEM_IMAGE_DESCRIPTION" \
  --argjson physical_width "$PHYSICAL_WIDTH" \
  --argjson physical_height "$PHYSICAL_HEIGHT" \
  --argjson density_dpi "$DENSITY_DPI" \
  --arg renderer_vendor "$RENDERER_VENDOR" \
  --arg renderer_adapter "$RENDERER_ADAPTER" \
  --arg renderer_api_backend "$RENDERER_API_BACKEND" \
  --arg emulator_version "$(head -1 "$EVIDENCE_DIR/emulator-version.txt")" \
  --arg host_os "$HOST_OS" \
  --arg emulator_sha256 "$EMULATOR_SHA256" \
  --arg qemu_sha256 "$QEMU_SHA256" \
  --arg ndk_directory_version "$NDK_DIRECTORY_VERSION" \
  --arg ndk_release_name "$NDK_RELEASE_NAME" \
  --arg ndk_root "$NDK_ROOT" \
  --arg ndk_source_properties "$NDK_SOURCE_PROPERTIES" \
  --arg ndk_source_properties_sha256 "$NDK_SOURCE_PROPERTIES_SHA256" \
  --arg ndk_host_prebuilt "$NDK_HOST_PREBUILT" \
  --arg ndk_clang "$NDK_CLANG" \
  --arg ndk_clang_sha256 "$NDK_CLANG_SHA256" \
  --arg makepad_android_platform "$MAKEPAD_ANDROID_PLATFORM" \
  --arg makepad_android_build_tools_version "$MAKEPAD_ANDROID_BUILD_TOOLS_VERSION" \
  --arg makepad_android_jar "$MAKEPAD_ANDROID_JAR" \
  --arg makepad_android_jar_sha256 "$MAKEPAD_ANDROID_JAR_SHA256" \
  --arg makepad_aapt "$MAKEPAD_AAPT" \
  --arg makepad_aapt_sha256 "$MAKEPAD_AAPT_SHA256" \
  --arg makepad_aapt2 "$MAKEPAD_AAPT2" \
  --arg makepad_aapt2_sha256 "$MAKEPAD_AAPT2_SHA256" \
  --arg makepad_d8_jar "$MAKEPAD_D8_JAR" \
  --arg makepad_d8_jar_sha256 "$MAKEPAD_D8_JAR_SHA256" \
  --arg makepad_zipalign "$MAKEPAD_ZIPALIGN" \
  --arg makepad_zipalign_sha256 "$MAKEPAD_ZIPALIGN_SHA256" \
  --arg makepad_apksigner_jar "$MAKEPAD_APKSIGNER_JAR" \
  --arg makepad_apksigner_jar_sha256 "$MAKEPAD_APKSIGNER_JAR_SHA256" \
  --arg makepad_java "$MAKEPAD_JAVA" \
  --arg makepad_java_sha256 "$MAKEPAD_JAVA_SHA256" \
  --arg makepad_javac "$MAKEPAD_JAVAC" \
  --arg makepad_javac_sha256 "$MAKEPAD_JAVAC_SHA256" \
  --argjson pid "$APP_PID" \
  --argjson process_start_time_ticks "$PROCESS_START_TIME_TICKS" \
  --arg installed_package_path "$INSTALLED_PACKAGE_PATH" \
  --arg session_probe_path "$SESSION_PROBE_PATH" \
  --arg session_probe_nonce "$SESSION_PROBE_NONCE" \
  --arg session_probe_sha256 "$SESSION_PROBE_SHA256" \
  --argjson cold_launch_ms "$COLD_LAUNCH_TOTAL_MS" \
  --argjson fatal_count "$FATAL_COUNT" \
  --argjson anr_count "$ANR_COUNT" \
  --argjson bgra_count "$BGRA_COUNT" \
  --arg portrait_path "$PORTRAIT_PATH" \
  --arg portrait_sha "$PORTRAIT_SHA256" \
  --argjson portrait_width "$PORTRAIT_WIDTH" \
  --argjson portrait_height "$PORTRAIT_HEIGHT" \
  --argjson portrait_attempts "$PORTRAIT_ATTEMPTS" \
  --argjson portrait_probe "$PORTRAIT_PROBE" \
  --argjson status_bar_contrast "$STATUS_BAR_CONTRAST" \
  --argjson portrait_template_probe "$PORTRAIT_TEMPLATE_PROBE" \
  --arg landscape_path "$LANDSCAPE_PATH" \
  --arg landscape_sha "$LANDSCAPE_SHA256" \
  --argjson landscape_width "$LANDSCAPE_WIDTH" \
  --argjson landscape_height "$LANDSCAPE_HEIGHT" \
  --argjson landscape_attempts "$LANDSCAPE_ATTEMPTS" \
  --argjson landscape_probe "$LANDSCAPE_PROBE" \
  --argjson landscape_template_probe "$LANDSCAPE_TEMPLATE_PROBE" \
  --arg ime_path "$IME_PATH" \
  --arg ime_sha "$IME_SHA256" \
  --argjson ime_width "$IME_WIDTH" \
  --argjson ime_height "$IME_HEIGHT" \
  --argjson ime_attempts "$IME_ATTEMPTS" \
  --argjson ime_probe "$IME_PROBE" \
  --argjson ime_template_probe "$IME_TEMPLATE_PROBE" \
  --arg uiautomator_path "$EVIDENCE_DIR/uiautomator-portrait.xml" \
  --arg uiautomator_sha256 "$UIAUTOMATOR_SHA256" \
  --argjson node_count "$UI_NODE_COUNT" \
  --argjson visible_node_count "$UI_VISIBLE_NODE_COUNT" \
  --argjson package_node_count "$UI_PACKAGE_NODE_COUNT" \
  --argjson labeled_node_count "$UI_LABELED_NODE_COUNT" \
  --arg accessibility_enabled "$ACCESSIBILITY_ENABLED" \
  --arg accessibility_services "$ACCESSIBILITY_SERVICES" \
  --arg manifest_contract_path "$EVIDENCE_DIR/manifest-contract.json" \
  --argjson manifest_contract "$MANIFEST_CONTRACT" \
  --argjson toolchain "$TOOLCHAIN_REPORT" \
  --arg login_template_manifest "$LOGIN_TEMPLATE_MANIFEST" \
  --arg login_template_manifest_sha256 "$LOGIN_TEMPLATE_MANIFEST_SHA256" \
  --arg evidence_root "$EVIDENCE_DIR" --argjson extended_lab "$LAB_RESULT" '
    {
      schema_version:3,
      kind:"hepta-native-android-emulator-smoke-receipt",
      producer:$producer,
      status:"ready",
      ready:true,
      captured_at:$captured_at,
      scope:"unauthenticated_android_login_surface_on_arm64_emulator",
      scope_note:"Ready proves a fresh source-bound ARM64 debug-key APK on the explicitly named already-booted headless emulator, including foreground launch, portrait/landscape/IME nonblank content, and a valid UIAutomator tree. It is not a semantic accessibility, real-device, authenticated Matrix, secure credential, release-signing, or public-distribution claim.",
      source_binding:$source_binding,
      artifact:{
        format:"apk",stale_artifact_accepted:false,path:$artifact_path,size_bytes:$artifact_size,sha256:$artifact_sha256,
        package:"ai.hepta.nativeapp",activity:"ai.hepta.nativeapp/.MakepadApp",launchable_activity:"ai.hepta.nativeapp.MakepadApp",label:"Hepta",
        version_code:$version_code,version_name:$version_name,min_sdk:26,target_sdk:35,primary_cpu_abi:"arm64-v8a",application_debuggable:false,
        install_result:"Success",install_success:true,full_head_embedded:true,artifact_source_bound:true,
        manifest_contract:{status:$manifest_contract.status,ready:$manifest_contract.claims.manifest_contract_ready,path:$manifest_contract_path}
      },
      signing:{kind:"android_debug",verified:true,v2:true,v3:true,certificate_dn:$certificate_dn,certificate_sha256:$certificate_sha256,release_signed:false},
      toolchain:$toolchain,
      host_toolchain:{
        host_os:$host_os,host_architecture:"arm64",
        adb_binary_path:$adb_path,adb_binary_sha256:$adb_sha256,
        emulator_version:$emulator_version,emulator_binary_architecture:"arm64",emulator_binary_sha256:$emulator_sha256,
        qemu_binary_architecture:"arm64",qemu_binary_sha256:$qemu_sha256,accelerator:"Hypervisor.Framework",
        ndk:{
          directory_version:$ndk_directory_version,release_name:$ndk_release_name,root_path:$ndk_root,
          source_properties_path:$ndk_source_properties,source_properties_sha256:$ndk_source_properties_sha256,
          host_prebuilt:$ndk_host_prebuilt,clang_binary_path:$ndk_clang,clang_binary_sha256:$ndk_clang_sha256
        },
        makepad_android_sdk:{
          platform:$makepad_android_platform,build_tools_version:$makepad_android_build_tools_version,
          android_jar_path:$makepad_android_jar,android_jar_sha256:$makepad_android_jar_sha256,
          aapt_path:$makepad_aapt,aapt_sha256:$makepad_aapt_sha256,
          aapt2_path:$makepad_aapt2,aapt2_sha256:$makepad_aapt2_sha256,
          d8_jar_path:$makepad_d8_jar,d8_jar_sha256:$makepad_d8_jar_sha256,
          zipalign_path:$makepad_zipalign,zipalign_sha256:$makepad_zipalign_sha256,
          apksigner_jar_path:$makepad_apksigner_jar,apksigner_jar_sha256:$makepad_apksigner_jar_sha256,
          java_path:$makepad_java,java_sha256:$makepad_java_sha256,
          javac_path:$makepad_javac,javac_sha256:$makepad_javac_sha256
        }
      },
      device:{adb_serial:$serial,state:"device",boot_completed:true,avd_name:$avd_name,qemu_avd_name:$qemu_avd_name,avd_name_match:true,boot_id:$boot_id,model:$device_model},
      avd:{
        name:$avd_name,system_image:$system_image,android_version:$android_version,api_level:$api_level,guest_abi:"arm64-v8a",guest_uname_machine:"aarch64",
        physical_width:$physical_width,physical_height:$physical_height,physical_density_dpi:$density_dpi,headless:true,hardware_accelerated:true,
        renderer:{mode:"host",vendor:$renderer_vendor,adapter:$renderer_adapter,guest_api:($renderer_api_backend | split(" (")[0]),host_backend:($renderer_api_backend | capture("\\((?<backend>.*Metal.*)\\)").backend)}
      },
      runtime:{
        stale_package_removed:true,install_success:true,cold_launch_success:true,cold_launch_total_ms:$cold_launch_ms,process_alive:true,pid:$pid,
        process_start_time_ticks:$process_start_time_ticks,
        foreground:true,top_resumed:true,current_focus:true,focused_app:true,fatal_marker_count:$fatal_count,anr_marker_count:$anr_count,
        login_bgra_gl_error_count:$bgra_count,crash_buffer_empty:true,fresh_install_without_supplied_credentials:true,
        installed_package_path:$installed_package_path
      },
      session_probe:{path:$session_probe_path,nonce:$session_probe_nonce,sha256:$session_probe_sha256,boot_id:$boot_id,created_by_producer:true,readback_matched:true,no_credentials_supplied:true},
      login_surface_template:{manifest_path:$login_template_manifest,manifest_sha256:$login_template_manifest_sha256,version:1,all_states_ready:true},
      visual_inspection:{
        machine_verified_original_dimensions:true,
        system_bar_contrast:$status_bar_contrast,
        portrait:{format:"png",path:$portrait_path,sha256:$portrait_sha,width:$portrait_width,height:$portrait_height,capture_attempts:$portrait_attempts,content_probe:$portrait_probe,login_template_probe:$portrait_template_probe,app_remains_foreground:true,login_surface_template_ready:true},
        landscape:{format:"png",path:$landscape_path,sha256:$landscape_sha,width:$landscape_width,height:$landscape_height,capture_attempts:$landscape_attempts,content_probe:$landscape_probe,login_template_probe:$landscape_template_probe,app_remains_foreground:true,login_surface_template_ready:true},
        ime:{format:"png",path:$ime_path,sha256:$ime_sha,width:$ime_width,height:$ime_height,capture_attempts:$ime_attempts,content_probe:$ime_probe,login_template_probe:$ime_template_probe,app_remains_foreground:true,input_shown:true,input_view_shown:true,focused_surface:"homeserver_input_template_anchor",focused_surface_visible:true,manifest_soft_input_mode:"STATE_UNCHANGED|ADJUST_NOTHING",manifest_soft_input_contract_ready:true,login_surface_template_ready:true}
      },
      uiautomator:{xml_ready:true,path:$uiautomator_path,sha256:$uiautomator_sha256,node_count:$node_count,visible_node_count:$visible_node_count,package_node_count:$package_node_count,labeled_node_count:$labeled_node_count,semantic_accessibility_ready:false,talkback_ready:false},
      accessibility:{android_accessibility_enabled:($accessibility_enabled == "1"),enabled_services:(if $accessibility_services == "null" then null else $accessibility_services end),semantic_accessibility_ready:false,talkback_ready:false},
      extended_lab:$extended_lab,
      claims:{
        android_arm64_debug_apk_installable:true,android_emulator_environment_ready:true,android_emulator_runtime_ready:true,
        android_emulator_login_surface_visual_ready:true,android_login_rotation_ready:true,android_login_ime_ready:true,
        android_rotation_ready:false,android_ime_ready:false,android_accessibility_ready:false,talkback_ready:false,
        android_safe_area_ready:false,android_rtl_ready:false,android_dynamic_type_ready:false,android_low_power_performance_ready:false,
        android_real_device_ready:false,android_secure_credential_backend_ready:false,authenticated_matrix_workflow_ready:false,
        post_login_raster_media_ready:false,release_signed:false,public_distribution_ready:false,full_product_ready:false,public_ga_ready:false
      },
      hard_boundaries:{
        accessibility_verified:false,talkback_verified:false,real_device_verified:false,secure_credential_backend_verified:false,
        authenticated_matrix_workflow_verified:false,post_login_raster_media_verified:false,effective_low_power_performance_verified:false,release_signed:false,public_distribution_ready:false
      },
      device_lab:{
        real_device_selected:false,talkback_session_selected:false,
        blockers:[
          {code:"android_real_device_receipt_missing",claim:"android_real_device_ready",observed:false},
          {code:"talkback_receipt_missing",claim:"talkback_ready",observed:false}
        ]
      },
      local_emulator_side_effects:{old_package_removed:true,apk_installed:true,app_launched:true,orientation_changed_temporarily:true,ime_shown:true,screenshots_captured:true,session_probe_written:true,extended_lab_requested:$extended_lab.requested,power_state_simulated_temporarily:($extended_lab.requested == true)},
      forbidden_actions_performed:{sdk_or_runtime_download:false,avd_create_or_boot:false,credential_supply:false,real_device_contact:false,account_connection:false,release_sign:false,upload:false},
      evidence:{root:$evidence_root}
    }
  ' >"$REPORT_TEMP"
jq -e '
  .schema_version == 3
  and .kind == "hepta-native-android-emulator-smoke-receipt"
  and .producer == "scripts/hepta-native-android-emulator-smoke.sh"
  and .status == "ready"
  and .ready == true
  and (.hard_boundaries | to_entries | all(.value == false))
  and (.forbidden_actions_performed | to_entries | all(.value == false))
' "$REPORT_TEMP" >/dev/null

FINAL_RESTORE_COMMAND_READY=true
FINAL_RESTORE_READBACK_READY=true
restore_emulator_state || FINAL_RESTORE_COMMAND_READY=false
android_emulator_state_readback_ready || FINAL_RESTORE_READBACK_READY=false
if [[ "$FINAL_RESTORE_COMMAND_READY" != true || "$FINAL_RESTORE_READBACK_READY" != true ]]; then
  write_android_cleanup_failure_receipt 1 "$FINAL_RESTORE_COMMAND_READY" "$FINAL_RESTORE_READBACK_READY" \
    || echo "error: failed to write Android cleanup failure receipt" >&2
  echo "error: Android final state restoration/readback failed" >&2
  exit 1
fi
trap - EXIT HUP INT TERM
mv -f -- "$REPORT_TEMP" "$REPORT_PATH"
REPORT_TEMP=""
echo "==> Android ARM64 emulator smoke verified for current source $SOURCE_HEAD"
echo "==> Receipt: $REPORT_PATH"
echo "==> APK:     $APK_PATH"
echo "==> Evidence: $EVIDENCE_DIR"
