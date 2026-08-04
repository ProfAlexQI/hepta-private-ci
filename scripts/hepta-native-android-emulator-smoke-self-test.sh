#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

scripts/hepta-android-system-bar-contrast-probe-self-test.sh

TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-android-emulator-smoke-self-test.XXXXXX")"
trap 'rm -rf "$TEST_DIR"' EXIT

PRODUCER="scripts/hepta-native-android-emulator-smoke.sh"
PROBE="scripts/hepta-android-login-template-probe"
ORIENTATION_PROBE="scripts/hepta-android-window-orientation-probe"
HEADLESS_AVD_PROCESS_PROBE="scripts/hepta-android-headless-avd-process-probe"
STATE_HELPER="scripts/hepta-native-android-emulator-lab-state-v1.sh"
CLEANUP_HELPER="scripts/hepta-native-mobile-lab-cleanup-v1.sh"
TEMPLATE_DIR="apps/hepta-native/packaging/android-emulator-login-template-v1"
MANIFEST="$TEMPLATE_DIR/manifest.json"

bash -n "$PRODUCER" "$PROBE" "$STATE_HELPER" "$CLEANUP_HELPER" scripts/hepta-native-mobile-readiness-gate.sh
ruby -c "$ORIENTATION_PROBE" >/dev/null
ruby -c "$HEADLESS_AVD_PROCESS_PROBE" >/dev/null
source "$STATE_HELPER"
source "$CLEANUP_HELPER"

[[ "$(hepta_mobile_cleanup_final_exit_code 29 true true)" == 29 ]]
[[ "$(hepta_mobile_cleanup_final_exit_code 143 true false)" == 143 ]]
[[ "$(hepta_mobile_cleanup_final_exit_code 0 false true)" == 1 ]]
ANDROID_CLEANUP_FAILURE="$(hepta_mobile_cleanup_failure_json android_emulator "$PRODUCER" 0 false true)"
jq -e '
  .kind == "hepta-native-mobile-lab-cleanup-failure-receipt"
  and .status == "not_ready" and .ready == false
  and .original_exit_code == 0 and .final_exit_code == 1
  and .local_device_state_mutation_performed == true
  and .local_device_state_may_remain_mutated == true
  and (.blockers | map(.code)) == ["android_emulator_state_restore_command_failed"]
' >/dev/null <<<"$ANDROID_CLEANUP_FAILURE"

NONDEFAULT_STATE='{"accelerometer_rotation":"0","user_rotation":"3","font_scale":"null","force_rtl":"null","low_power":"1","battery":{"ac":true,"counter":10000,"dock":false,"health":2,"level":100,"present":true,"scale":100,"status":2,"temp":250,"updates_stopped":false,"usb":false,"wireless":false}}'
hepta_android_emulator_lab_state_ready "$NONDEFAULT_STATE"
[[ "$(hepta_android_battery_restore_plan "$(jq -c '.battery' <<<"$NONDEFAULT_STATE")")" == reset ]]
FROZEN_STATE="$(jq -c '.battery.updates_stopped=true | .battery.level=15' <<<"$NONDEFAULT_STATE")"
if hepta_android_emulator_lab_state_ready "$FROZEN_STATE"; then
  echo "Android lab state accepted an unrestorable frozen battery snapshot" >&2
  exit 1
fi
if hepta_android_battery_restore_plan "$(jq -c '.battery' <<<"$FROZEN_STATE")" >/dev/null 2>&1; then
  echo "Android lab planned a partial frozen battery restoration" >&2
  exit 1
fi
if hepta_android_emulator_lab_state_ready "$(jq 'del(.battery.counter)' <<<"$NONDEFAULT_STATE")"; then
  echo "Android lab state accepted an incomplete battery snapshot" >&2
  exit 1
fi
FAKE_RESTORE_ADB="$TEST_DIR/fake-restore-adb"
RESTORE_LOG="$TEST_DIR/restore.log"
printf '%s\n' '#!/usr/bin/env bash' "printf '%s\\n' \"\$*\" >>$(printf %q "$RESTORE_LOG")" '[[ "$*" != *"settings delete system font_scale"* ]]' >"$FAKE_RESTORE_ADB"
chmod 0755 "$FAKE_RESTORE_ADB"
if hepta_android_emulator_lab_state_restore "$FAKE_RESTORE_ADB" emulator-5554 "$NONDEFAULT_STATE"; then
  echo "Android lab restore ignored a raw-setting restore failure" >&2
  exit 1
fi
grep -Fq 'settings put system user_rotation 3' "$RESTORE_LOG"
grep -Fq 'settings delete system font_scale' "$RESTORE_LOG"
grep -Fq 'settings delete global debug.force_rtl' "$RESTORE_LOG"
grep -Fq 'battery reset' "$RESTORE_LOG"
grep -Fq 'power set-mode 1' "$RESTORE_LOG"
: >"$RESTORE_LOG"
if hepta_android_emulator_lab_state_restore "$FAKE_RESTORE_ADB" emulator-5554 "$FROZEN_STATE"; then
  echo "Android lab restore accepted an unrestorable frozen state" >&2
  exit 1
fi
[[ ! -s "$RESTORE_LOG" ]] || { echo "Android lab touched adb before rejecting frozen state" >&2; exit 1; }

FAKE_SNAPSHOT_ADB="$TEST_DIR/fake-snapshot-adb"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  'case "$*" in' \
  '  *"settings get system accelerometer_rotation") echo 0 ;;' \
  '  *"settings get system user_rotation") echo 3 ;;' \
  '  *"settings get system font_scale"|*"settings get global debug.force_rtl") echo null ;;' \
  '  *"settings get global low_power") echo 1 ;;' \
  "  *\"dumpsys battery\") printf '%s\\n' 'Current Battery Service state:' '  AC powered: true' '  USB powered: false' '  Wireless powered: false' '  Dock powered: false' '  Charge counter: 10000' '  status: 2' '  health: 2' '  present: true' '  level: 100' '  scale: 100' '  temperature: 250' ;;" \
  'esac' >"$FAKE_SNAPSHOT_ADB"
chmod 0755 "$FAKE_SNAPSHOT_ADB"
RAW_SNAPSHOT="$(hepta_android_emulator_lab_state_snapshot "$FAKE_SNAPSHOT_ADB" emulator-5554)"
jq -e '.font_scale == "null" and .force_rtl == "null" and .battery.updates_stopped == false' >/dev/null <<<"$RAW_SNAPSHOT"

FAKE_ABSENCE_ADB="$TEST_DIR/fake-absence-adb"
ABSENCE_DELETE_COUNT="$TEST_DIR/absence-delete-count"
printf '%s\n' \
  '#!/usr/bin/env bash' \
  "state=$(printf %q "$ABSENCE_DELETE_COUNT")" \
  'case "$*" in' \
  '  *"settings delete system font_scale") count=0; [[ ! -f "$state" ]] || count="$(<"$state")"; printf "%s\n" "$((count + 1))" >"$state" ;;' \
  '  *"settings get system font_scale") count=0; [[ ! -f "$state" ]] || count="$(<"$state")"; if (( count < 2 )); then echo 1.0; else echo null; fi ;;' \
  'esac' >"$FAKE_ABSENCE_ADB"
chmod 0755 "$FAKE_ABSENCE_ADB"
hepta_android_restore_setting "$FAKE_ABSENCE_ADB" emulator-5554 system font_scale null
[[ "$(<"$ABSENCE_DELETE_COUNT")" == 2 ]]

# A wrapper command can contain the complete emulator invocation. Only the
# process whose executable identity is the pinned QEMU binary may be accepted.
QEMU_FIXTURE="/Users/example/Android/sdk/emulator/qemu/darwin-aarch64/qemu-system-aarch64-headless"
printf '%s\n' \
  "/bin/zsh -lc $QEMU_FIXTURE -avd Hepta_Pixel_API_34_arm64 -no-window" \
  "/usr/bin/ruby $HEADLESS_AVD_PROCESS_PROBE Hepta_Pixel_API_34_arm64 $QEMU_FIXTURE" \
  "$QEMU_FIXTURE -avd Hepta_Pixel_API_34_arm64 -port 5554 -no-window -no-audio" \
  | ruby "$HEADLESS_AVD_PROCESS_PROBE" Hepta_Pixel_API_34_arm64 "$QEMU_FIXTURE" \
  >"$TEST_DIR/headless-avd-process.txt"
[[ "$(cat "$TEST_DIR/headless-avd-process.txt")" == \
  "$QEMU_FIXTURE -avd Hepta_Pixel_API_34_arm64 -port 5554 -no-window -no-audio" ]]

if printf '%s\n' \
    "$QEMU_FIXTURE -avd Hepta_Pixel_API_34_arm64 -no-window" \
    "$QEMU_FIXTURE @Hepta_Pixel_API_34_arm64 -no-window" \
    | ruby "$HEADLESS_AVD_PROCESS_PROBE" Hepta_Pixel_API_34_arm64 "$QEMU_FIXTURE" \
      >"$TEST_DIR/headless-avd-duplicate.stdout" 2>"$TEST_DIR/headless-avd-duplicate.stderr"; then
  echo "headless AVD process probe accepted duplicate QEMU processes" >&2
  exit 1
fi
grep -Fq 'expected exactly one already-running headless AVD process' "$TEST_DIR/headless-avd-duplicate.stderr"

# Android's `wm size` reports the immutable physical panel geometry, so the
# canonical Pixel API 34 landscape state must be classified from WindowManager
# rotation plus current logical bounds instead. These excerpts mirror the
# historical ARM64 AVD's ROTATION_90 / 2220x1080 evidence.
printf '%s\n' \
  '  Display: mDisplayId=0 (organized)' \
  '    init=1080x2220 440dpi mMinSizeOfResizeableTaskDp=220 cur=2220x1080 app=2220x1014 rng=1080x948-2220x2088' \
  '  overrideConfig={ mBounds=Rect(0, 0 - 2220, 1080) mDisplayRotation=ROTATION_90 mRotation=ROTATION_90}' \
  '    mRotation=1 mDeferredRotationPauseCount=0' \
  >"$TEST_DIR/historical-hepta-pixel-api34-landscape.txt"
"$ORIENTATION_PROBE" "$TEST_DIR/historical-hepta-pixel-api34-landscape.txt" >"$TEST_DIR/orientation-landscape.json"
jq -e '.ready == true and .orientation == "landscape" and .rotation == 1 and .display_rotation_degrees == 90 and .logical_width == 2220 and .logical_height == 1080' \
  "$TEST_DIR/orientation-landscape.json" >/dev/null

printf '%s\n' \
  '  Display: mDisplayId=0 (organized)' \
  '    init=1080x2220 440dpi mMinSizeOfResizeableTaskDp=220 cur=1080x2220 app=1080x2154 rng=1080x948-2220x2088' \
  '  overrideConfig={ mBounds=Rect(0, 0 - 1080, 2220) mDisplayRotation=ROTATION_0 mRotation=ROTATION_0}' \
  '    mRotation=0 mDeferredRotationPauseCount=0' \
  >"$TEST_DIR/hepta-pixel-api34-portrait.txt"
"$ORIENTATION_PROBE" "$TEST_DIR/hepta-pixel-api34-portrait.txt" >"$TEST_DIR/orientation-portrait.json"
jq -e '.ready == true and .orientation == "portrait" and .rotation == 0 and .logical_width == 1080 and .logical_height == 2220' \
  "$TEST_DIR/orientation-portrait.json" >/dev/null

printf '%s\n' \
  '  Display: mDisplayId=0 (organized)' \
  '    init=1080x2220 440dpi cur=1080x2220 app=1080x2154' \
  '  overrideConfig={ mBounds=Rect(0, 0 - 1080, 2220) mDisplayRotation=ROTATION_90 mRotation=ROTATION_90}' \
  '    mRotation=1 mDeferredRotationPauseCount=0' \
  >"$TEST_DIR/inconsistent-landscape.txt"
if "$ORIENTATION_PROBE" "$TEST_DIR/inconsistent-landscape.txt" >"$TEST_DIR/orientation-inconsistent.json" 2>/dev/null; then
  echo "orientation probe accepted ROTATION_90 with portrait logical geometry" >&2
  exit 1
fi
jq -e '.ready == false and .checks.rotation_matches_logical_geometry == false' "$TEST_DIR/orientation-inconsistent.json" >/dev/null

# A conflicting secondary display must never poison the default-display
# rotation/geometry selection.
printf '%s\n' \
  '  Display: mDisplayId=0 (organized)' \
  '    init=1080x2220 440dpi cur=2220x1080 app=2220x1014' \
  '  overrideConfig={ mBounds=Rect(0, 0 - 2220, 1080) mDisplayRotation=ROTATION_90 mRotation=ROTATION_90}' \
  '    mRotation=1 mDeferredRotationPauseCount=0' \
  '  Display: mDisplayId=1 (organized)' \
  '    init=800x1200 320dpi cur=800x1200 app=800x1134' \
  '  overrideConfig={ mBounds=Rect(0, 0 - 800, 1200) mDisplayRotation=ROTATION_0 mRotation=ROTATION_0}' \
  '    mRotation=0 mDeferredRotationPauseCount=0' \
  >"$TEST_DIR/multi-display.txt"
"$ORIENTATION_PROBE" "$TEST_DIR/multi-display.txt" >"$TEST_DIR/orientation-multi-display.json"
jq -e '
  .ready == true
  and .display_id == 0
  and .orientation == "landscape"
  and .rotation == 1
  and .logical_width == 2220
  and .logical_height == 1080
  and .checks.exactly_one_default_display_block == true
' "$TEST_DIR/orientation-multi-display.json" >/dev/null

contract="$($PRODUCER --contract-only)"
jq -e '
  .schema_version == 1
  and .kind == "hepta-native-android-emulator-smoke-source-contract"
  and .status == "ready"
  and .receipt.schema_version == 3
  and .receipt.kind == "hepta-native-android-emulator-smoke-receipt"
  and (.requirements | to_entries | all(.value == true))
  and .requirements.extended_lab_opt_in == true
  and .requirements.extended_lab_state_snapshot_restore_readback == true
  and .requirements.extended_lab_mode_matched_controls == true
  and .requirements.extended_lab_leaf_rehash_before_promotion == true
  and .requirements.extended_lab_raw_setting_absence_restore == true
  and .requirements.extended_lab_unrestorable_frozen_battery_rejected_before_mutation == true
  and .requirements.exit_cleanup_preserves_original_status == true
  and .requirements.interrupt_cleanup_restore_and_readback == true
  and .requirements.cleanup_failure_receipt == true
  and .requirements.system_bar_contrast_probe_ready == true
  and .requirements.emulator_only_power_simulation_never_real_device_claim == true
  and (.hard_boundaries | to_entries | all(.value == false))
  and (.forbidden_actions | to_entries | all(.value == false))
  and .external_side_effects_performed == false
' >/dev/null <<<"$contract"

template_manifest_predicate() {
  jq -L "$ROOT_DIR/scripts/lib" -e '
    include "hepta-native-android-login-template-v1";
    hepta_android_login_template_v1_ready
  ' "$1" >/dev/null
}
template_manifest_predicate "$MANIFEST"

expect_manifest_failure() {
  local label="$1" filter="$2"
  jq "$filter" "$MANIFEST" >"$TEST_DIR/manifest-$label.json"
  if template_manifest_predicate "$TEST_DIR/manifest-$label.json"; then
    echo "template manifest unexpectedly accepted $label" >&2
    exit 1
  fi
}
expect_manifest_failure absolute_source_receipt '.source_receipt = "/Users/example/private/report.json"'
expect_manifest_failure missing_external_boundary '.source_evidence.receipt.external_not_committed = false'
expect_manifest_failure source_head_drift '.source_evidence.head = "0000000000000000000000000000000000000000"'
expect_manifest_failure captured_frame_drift '.source_evidence.captured_frames.ime_sha256 = "bad"'

for key in portrait landscape ime; do
  template="$(jq -r --arg key "$key" '.templates[$key].path' "$MANIFEST")"
  expected_sha="$(jq -r --arg key "$key" '.templates[$key].sha256' "$MANIFEST")"
  [[ "$(shasum -a 256 "$template" | awk '{print $1}')" == "$expected_sha" ]]
  "$PROBE" --image "$template" --template "$template" --mode "$key" \
    --output "$TEST_DIR/$key.positive.json" >/dev/null
  jq -e --arg key "$key" --arg sha "$expected_sha" '
    .status == "ready"
    and .ready == true
    and .mode == $key
    and .image.sha256 == $sha
    and .template.sha256 == $sha
    and (.detections | all(.ready == true))
    and (.detections | any(.name == "sign_in_to_hepta_title" and .ready == true))
    and (.detections | any((.name == "homeserver_input" or .name == "homeserver_input_focused") and .ready == true))
  ' "$TEST_DIR/$key.positive.json" >/dev/null
done
jq -e '
  (.detections | any(.name == "same_login_form_above_ime" and .ready == true))
  and (.detections | any(.name == "ime_keyboard" and .ready == true))
' "$TEST_DIR/ime.positive.json" >/dev/null

expect_probe_failure() {
  local label="$1" image="$2" template="$3" mode="$4"
  if "$PROBE" --image "$image" --template "$template" --mode "$mode" \
      --output "$TEST_DIR/$label.json" >/dev/null 2>&1; then
    echo "template probe unexpectedly accepted $label" >&2
    exit 1
  fi
  jq -e '.status == "not_ready" and .ready == false and (.detections | any(.ready == false))' \
    "$TEST_DIR/$label.json" >/dev/null
}

expect_probe_failure \
  cross_state_portrait_as_ime \
  "$TEMPLATE_DIR/portrait.png" \
  "$TEMPLATE_DIR/ime.png" \
  ime
expect_probe_failure \
  unrelated_login_generation \
  docs/architecture/assets/hepta-native-robrix-main-2026-08-01/mobile-login-390x828@2x.png \
  "$TEMPLATE_DIR/portrait.png" \
  portrait

ruby -rzlib -e '
  path = ARGV.fetch(0)
  width = 64
  height = 128
  chunk = ->(name, data) { [data.bytesize].pack("N") + name + data + [Zlib.crc32(name + data)].pack("N") }
  raw = ("\x00".b + "\xff\xff\xff".b * width) * height
  png = "\x89PNG\r\n\x1a\n".b +
    chunk.call("IHDR", [width, height, 8, 2, 0, 0, 0].pack("NNC5")) +
    chunk.call("IDAT", Zlib.deflate(raw)) +
    chunk.call("IEND", "".b)
  File.binwrite(path, png)
' "$TEST_DIR/blank.png"
expect_probe_failure blank "$TEST_DIR/blank.png" "$TEMPLATE_DIR/portrait.png" portrait

# The forced-dirty hook is negative-only. Fake every SDK executable and prove
# the producer rejects source before calling adb, emulator, aapt, or apksigner.
FAKE_SDK="$TEST_DIR/fake-sdk"
FAKE_NDK="$FAKE_SDK/ndk/28.2.13676358"
mkdir -p "$FAKE_SDK/platform-tools" "$FAKE_SDK/emulator/qemu/darwin-aarch64" "$FAKE_SDK/build-tools/35.0.0" \
  "$FAKE_SDK/platforms/android-33-ext4" "$FAKE_SDK/build-tools/33.0.1/lib" "$FAKE_SDK/openjdk/bin" \
  "$FAKE_NDK/toolchains/llvm/prebuilt/darwin-x86_64/bin"
printf '%s\n' 'Pkg.Desc = Android NDK' 'Pkg.ReleaseName = r28b' >"$FAKE_NDK/source.properties"
printf 'android jar\n' >"$FAKE_SDK/platforms/android-33-ext4/android.jar"
printf 'd8 jar\n' >"$FAKE_SDK/build-tools/33.0.1/lib/d8.jar"
printf 'apksigner jar\n' >"$FAKE_SDK/build-tools/33.0.1/lib/apksigner.jar"
SENTINEL="$TEST_DIR/external-tool-called"
for tool in \
  "$FAKE_SDK/platform-tools/adb" \
  "$FAKE_SDK/emulator/emulator" \
  "$FAKE_SDK/emulator/qemu/darwin-aarch64/qemu-system-aarch64-headless" \
  "$FAKE_SDK/build-tools/35.0.0/aapt" \
  "$FAKE_SDK/build-tools/35.0.0/apksigner" \
  "$FAKE_SDK/build-tools/33.0.1/aapt" \
  "$FAKE_SDK/build-tools/33.0.1/aapt2" \
  "$FAKE_SDK/build-tools/33.0.1/zipalign" \
  "$FAKE_SDK/openjdk/bin/java" \
  "$FAKE_SDK/openjdk/bin/javac" \
  "$FAKE_NDK/toolchains/llvm/prebuilt/darwin-x86_64/bin/clang"; do
  printf '#!/usr/bin/env bash\nprintf called >>%q\nexit 99\n' "$SENTINEL" >"$tool"
  chmod 0755 "$tool"
done
if HEPTA_ANDROID_SMOKE_FORCE_DIRTY_SELF_TEST=1 \
  "$PRODUCER" \
    --avd Hepta_Pixel_API_34_arm64 \
    --serial emulator-5554 \
    --sdk-root "$FAKE_SDK" \
    --output "$TEST_DIR/dirty/report.json" \
    --evidence-dir "$TEST_DIR/dirty/evidence" \
    --target-dir "$TEST_DIR/dirty/target" \
    --extended-lab \
    >"$TEST_DIR/dirty.stdout" 2>"$TEST_DIR/dirty.stderr"; then
  echo "forced-dirty producer run unexpectedly succeeded" >&2
  exit 1
fi
grep -Fq 'requires a completely clean committed current HEAD' "$TEST_DIR/dirty.stderr"
[[ ! -e "$SENTINEL" ]] || { echo "producer touched an external tool before rejecting dirty source" >&2; exit 1; }
[[ ! -e "$TEST_DIR/dirty/report.json" ]] || { echo "dirty producer run wrote a receipt" >&2; exit 1; }

# Shared consumer structure predicate: start with a valid minimal critical
# fixture, then mutate every promotion-sensitive class independently.
HEAD="$(printf 'a%.0s' {1..40})"
TREE="$(printf 'b%.0s' {1..40})"
FINGERPRINT="$(printf 'c%.0s' {1..64})"
SHA="$(printf 'd%.0s' {1..64})"
BOOT_ID="11111111-2222-3333-4444-555555555555"
SESSION_NONCE="aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee"
MANIFEST_ABS="$ROOT_DIR/$MANIFEST"
MANIFEST_SHA="$(shasum -a 256 "$MANIFEST" | awk '{print $1}')"
jq -n \
  --arg head "$HEAD" --arg tree "$TREE" --arg fingerprint "$FINGERPRINT" \
  --arg sha "$SHA" --arg boot_id "$BOOT_ID" --arg session_nonce "$SESSION_NONCE" \
  --arg manifest "$MANIFEST_ABS" --arg manifest_sha "$MANIFEST_SHA" '
  {
    schema_version:3,kind:"hepta-native-android-emulator-smoke-receipt",producer:"scripts/hepta-native-android-emulator-smoke.sh",status:"ready",ready:true,
    scope:"unauthenticated_android_login_surface_on_arm64_emulator",
    source_binding:{head:$head,head_tree:$tree,source_fingerprint:$fingerprint,worktree_clean:true,repository_worktree_clean:true},
    artifact:{path:"/tmp/Hepta.apk",sha256:$sha,stale_artifact_accepted:false,full_head_embedded:true,artifact_source_bound:true},
    host_toolchain:{
      adb_binary_path:"/tmp/Android/sdk/platform-tools/adb",adb_binary_sha256:$sha,emulator_binary_sha256:$sha,qemu_binary_sha256:$sha,
      ndk:{directory_version:"28.2.13676358",release_name:"r28b",root_path:"/tmp/Android/sdk/ndk/28.2.13676358",source_properties_path:"/tmp/Android/sdk/ndk/28.2.13676358/source.properties",source_properties_sha256:$sha,host_prebuilt:"darwin-x86_64",clang_binary_path:"/tmp/Android/sdk/ndk/28.2.13676358/toolchains/llvm/prebuilt/darwin-x86_64/bin/clang",clang_binary_sha256:$sha},
      makepad_android_sdk:{platform:"android-33-ext4",build_tools_version:"33.0.1",android_jar_path:"/tmp/Android/sdk/platforms/android-33-ext4/android.jar",android_jar_sha256:$sha,aapt_path:"/tmp/Android/sdk/build-tools/33.0.1/aapt",aapt_sha256:$sha,aapt2_path:"/tmp/Android/sdk/build-tools/33.0.1/aapt2",aapt2_sha256:$sha,d8_jar_path:"/tmp/Android/sdk/build-tools/33.0.1/lib/d8.jar",d8_jar_sha256:$sha,zipalign_path:"/tmp/Android/sdk/build-tools/33.0.1/zipalign",zipalign_sha256:$sha,apksigner_jar_path:"/tmp/Android/sdk/build-tools/33.0.1/lib/apksigner.jar",apksigner_jar_sha256:$sha,java_path:"/tmp/Android/sdk/openjdk/bin/java",java_sha256:$sha,javac_path:"/tmp/Android/sdk/openjdk/bin/javac",javac_sha256:$sha}
    },
    device:{adb_serial:"emulator-5554",avd_name:"Hepta_Pixel_API_34_arm64",qemu_avd_name:"Hepta_Pixel_API_34_arm64",boot_id:$boot_id},
    avd:{name:"Hepta_Pixel_API_34_arm64"},
    runtime:{pid:2468,process_start_time_ticks:123456,installed_package_path:"/data/app/~~hepta==/ai.hepta.nativeapp-current==/base.apk"},
    session_probe:{path:"/data/local/tmp/hepta-native-smoke-0123456789abcdef01234567",nonce:$session_nonce,sha256:$sha,boot_id:$boot_id,created_by_producer:true,readback_matched:true,no_credentials_supplied:true},
    login_surface_template:{manifest_path:$manifest,manifest_sha256:$manifest_sha,all_states_ready:true},
    visual_inspection:{
      system_bar_contrast:{
        schema_version:2,kind:"hepta-android-system-bar-contrast-probe",status:"ready",ready:true,requested_icon_tint:"dark",
        evidence_path:"/tmp/status-bar.json",evidence_sha256:$sha,image:{path:"/tmp/portrait.png",sha256:$sha,width:1080,height:2400},
        regions:{
          status_bar:{edge:"top",requested_icon_tint:"dark",ready:true,sample:{vertical_fraction:0.025,horizontal_fraction:0.96,pixels:51840,step:1,background_median_luma:238,luma_min:18,luma_max:248,luma_span:230,dark_pixel_ratio:0.01},thresholds:{min_background_median_luma:176,max_dark_icon_luma:112,min_luma_span:72,min_dark_pixel_ratio:0.001}},
          navigation_bar:{edge:"bottom",requested_icon_tint:"dark",ready:true,sample:{vertical_fraction:0.025,horizontal_fraction:0.96,pixels:51840,step:1,background_median_luma:238,luma_min:16,luma_max:248,luma_span:232,dark_pixel_ratio:0.02},thresholds:{min_background_median_luma:176,max_dark_icon_luma:112,min_luma_span:72,min_dark_pixel_ratio:0.001}}
        }
      },
      portrait:{path:"/tmp/portrait.png",sha256:$sha,width:1080,height:2400,content_probe:{ready:true},login_template_probe:{ready:true},login_surface_template_ready:true},
      landscape:{path:"/tmp/landscape.png",sha256:$sha,content_probe:{ready:true},login_template_probe:{ready:true},login_surface_template_ready:true},
      ime:{path:"/tmp/ime.png",sha256:$sha,content_probe:{ready:true},login_template_probe:{ready:true},login_surface_template_ready:true}
    },
    uiautomator:{path:"/tmp/ui.xml",sha256:$sha,xml_ready:true,semantic_accessibility_ready:false,talkback_ready:false},
    claims:{
      android_emulator_login_surface_visual_ready:true,android_login_rotation_ready:true,android_login_ime_ready:true,
      android_rotation_ready:false,android_ime_ready:false,android_accessibility_ready:false,talkback_ready:false,
      android_real_device_ready:false,android_secure_credential_backend_ready:false,authenticated_matrix_workflow_ready:false,
      release_signed:false,public_distribution_ready:false,full_product_ready:false,public_ga_ready:false
    },
    hard_boundaries:{accessibility_verified:false,talkback_verified:false,real_device_verified:false},
    forbidden_actions_performed:{sdk_or_runtime_download:false,avd_create_or_boot:false,credential_supply:false,real_device_contact:false,release_sign:false,upload:false}
  }
' >"$TEST_DIR/receipt-valid.json"

receipt_predicate() {
  jq -L "$ROOT_DIR/scripts/lib" -e \
    --arg head "$HEAD" --arg tree "$TREE" --arg fingerprint "$FINGERPRINT" \
    --arg manifest "$MANIFEST_ABS" --arg manifest_sha "$MANIFEST_SHA" '
      include "hepta-native-android-emulator-receipt-v3";
      hepta_android_emulator_receipt_v3_ready($head; $tree; $fingerprint; $manifest; $manifest_sha)
    ' "$1" >/dev/null
}
receipt_predicate "$TEST_DIR/receipt-valid.json"

jq '.extended_lab = {
  requested:true,status:"executed_with_product_claim_blockers",execution_ready:true,ready:false,state_restore_verified:true,
  modes:{
    rtl:{executed:true,force_rtl_readback:true,matched_control:{path:"/tmp/rtl-control.png",sha256:"dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",force_rtl:0,writing_direction:"left_to_right"},capture:{path:"/tmp/rtl.png",sha256:"dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"},raster_changed:false,mode_attributable_raster_change:false,geometry_comparison:{same_canvas:true,semantic_layout_verified:false},ready:false},
    font_scale:{executed:true,setting_readback_ready:true,matched_control:{path:"/tmp/font-control.png",sha256:"dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",font_scale:1.0},capture:{path:"/tmp/font.png",sha256:"dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"},raster_changed:false,mode_attributable_raster_change:false,geometry_comparison:{same_canvas:true,semantic_text_reflow_verified:false},ready:false},
    rotation_ime:{executed:true,scope:"unauthenticated_login_surface",generic_app_wide_ready:false},
    startup_performance:{executed:true,ready:true},
    low_power:{executed:true,emulator_only:true,real_low_power_qualification:false,ready:false}
  },
  promotion:{eligible:false,canonical_leaf_artifacts_rehashed:false,matched_control_leaf_artifacts_rehashed:false},
  claims:{android_rtl_ready:false,android_dynamic_type_ready:false,android_safe_area_ready:false,android_rotation_ready:false,android_ime_ready:false,android_low_power_performance_ready:false,android_real_device_ready:false,talkback_ready:false},
  blockers:[
    {code:"android_extended_lab_leaf_artifact_rehash_missing"},
    {code:"android_real_device_low_power_performance_receipt_missing"},
    {code:"android_real_device_receipt_missing"},
    {code:"talkback_receipt_missing"}
  ],
  forbidden_actions_performed:{credential_supply:false,real_device_contact:false,account_connection:false,sdk_or_runtime_download:false,avd_create_or_boot:false,release_sign:false,upload:false}
}' "$TEST_DIR/receipt-valid.json" >"$TEST_DIR/receipt-extended-valid.json"
receipt_predicate "$TEST_DIR/receipt-extended-valid.json"
for filter in \
  '.extended_lab.state_restore_verified = false' \
  '.extended_lab.modes.rtl.geometry_comparison.same_canvas = false' \
  '.extended_lab.modes.font_scale.mode_attributable_raster_change = true' \
  '.extended_lab.modes.low_power.real_low_power_qualification = true' \
  '.extended_lab.promotion.eligible = true' \
  '.extended_lab.promotion.canonical_leaf_artifacts_rehashed = true' \
  '.extended_lab.promotion.matched_control_leaf_artifacts_rehashed = true' \
  '.extended_lab.claims.android_low_power_performance_ready = true' \
  '.extended_lab.blockers = []'; do
  jq "$filter" "$TEST_DIR/receipt-extended-valid.json" >"$TEST_DIR/receipt-extended-invalid.json"
  if receipt_predicate "$TEST_DIR/receipt-extended-invalid.json"; then
    echo "consumer predicate accepted invalid extended-lab receipt: $filter" >&2
    exit 1
  fi
done

expect_receipt_failure() {
  local label="$1" filter="$2"
  jq "$filter" "$TEST_DIR/receipt-valid.json" >"$TEST_DIR/receipt-$label.json"
  if receipt_predicate "$TEST_DIR/receipt-$label.json"; then
    echo "consumer predicate accepted tampered receipt: $label" >&2
    exit 1
  fi
}
expect_receipt_failure artifact_hash '.artifact.sha256 = "bad"'
expect_receipt_failure screenshot_hash '.visual_inspection.portrait.sha256 = "bad"'
expect_receipt_failure screenshot_path '.visual_inspection.landscape.path = "../landscape.png"'
expect_receipt_failure system_bar_icon_tint '.visual_inspection.system_bar_contrast.requested_icon_tint = "light"'
expect_receipt_failure navigation_bar_icon_tint '.visual_inspection.system_bar_contrast.regions.navigation_bar.requested_icon_tint = "light"'
expect_receipt_failure status_bar_light_surface '.visual_inspection.system_bar_contrast.regions.status_bar.sample.background_median_luma = 175'
expect_receipt_failure navigation_bar_dark_icons '.visual_inspection.system_bar_contrast.regions.navigation_bar.sample.dark_pixel_ratio = 0'
expect_receipt_failure system_bar_thresholds '.visual_inspection.system_bar_contrast.regions.status_bar.thresholds.max_dark_icon_luma = 113'
expect_receipt_failure template_claim '.visual_inspection.ime.login_template_probe.ready = false'
expect_receipt_failure uiautomator_hash '.uiautomator.sha256 = "bad"'
expect_receipt_failure host_tool_hash '.host_toolchain.emulator_binary_sha256 = "bad"'
expect_receipt_failure adb_path '.host_toolchain.adb_binary_path = "platform-tools/adb"'
expect_receipt_failure ndk_release '.host_toolchain.ndk.release_name = "r28c"'
expect_receipt_failure ndk_source_hash '.host_toolchain.ndk.source_properties_sha256 = "bad"'
expect_receipt_failure ndk_clang_path '.host_toolchain.ndk.clang_binary_path = "/tmp/clang"'
expect_receipt_failure android_jar_hash '.host_toolchain.makepad_android_sdk.android_jar_sha256 = "bad"'
expect_receipt_failure build_tools_version '.host_toolchain.makepad_android_sdk.build_tools_version = "34.0.0"'
expect_receipt_failure javac_path '.host_toolchain.makepad_android_sdk.javac_path = "/tmp/javac"'
expect_receipt_failure real_device_serial '.device.adb_serial = "R58M123456A"'
expect_receipt_failure avd_identity '.device.qemu_avd_name = "forged-avd"'
expect_receipt_failure boot_session '.session_probe.boot_id = "99999999-2222-3333-4444-555555555555"'
expect_receipt_failure session_probe_path '.session_probe.path = "/data/local/tmp/forged"'
expect_receipt_failure installed_apk_path '.runtime.installed_package_path = "/sdcard/Hepta.apk"'
expect_receipt_failure process_instance '.runtime.process_start_time_ticks = 0'
expect_receipt_failure accessibility_overclaim '.claims.android_accessibility_ready = true'
expect_receipt_failure boundary_overclaim '.hard_boundaries.accessibility_verified = true'

# File-byte tampering is checked outside jq by the canonical consumer. Keep
# these literals pinned so a later refactor cannot silently drop readback.
grep -Fq 'shasum -a 256 "$android_receipt_apk_path"' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq 'shasum -a 256 "$path"' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq 'verify_android_emulator_uiautomator' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq 'verify_android_emulator_host_tools' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq 'verify_android_login_template' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq 'trusted_live_readback_failed' scripts/hepta-native-mobile-readiness-gate.sh
grep -Fq 'FINAL_BOOT_ID' scripts/hepta-native-android-emulator-smoke.sh
grep -Fq 'FINAL_QEMU_AVD_NAME' scripts/hepta-native-android-emulator-smoke.sh
grep -Fq 'process_start_time_ticks' scripts/hepta-native-android-emulator-smoke.sh
grep -Fq -- '--extended-lab' scripts/hepta-native-android-emulator-smoke.sh
grep -Fq 'hepta-android-system-bar-contrast-probe' scripts/hepta-native-android-emulator-smoke.sh
grep -Fq 'SystemBarAppearance::DarkIcons' apps/hepta-native/src/app.rs
grep -Fq 'restore_emulator_state || LAB_RESTORE_COMMAND_READY=false' scripts/hepta-native-android-emulator-smoke.sh
grep -Fq 'cmd battery reset' "$STATE_HELPER"
grep -Fq 'frozen battery state is not exactly restorable' "$STATE_HELPER"
grep -Fq 'Android extended-lab rejects unreadable or frozen original battery state before mutation' scripts/hepta-native-android-emulator-smoke.sh
grep -Fq 'trap android_cleanup EXIT' scripts/hepta-native-android-emulator-smoke.sh
grep -Fq 'android_emulator_state_readback_ready' scripts/hepta-native-android-emulator-smoke.sh
grep -Fq 'write_android_cleanup_failure_receipt' scripts/hepta-native-android-emulator-smoke.sh
grep -Fq 'hepta_mobile_cleanup_final_exit_code "$original_exit"' scripts/hepta-native-android-emulator-smoke.sh
grep -Fq 'REPORT_TEMP="$(mktemp ' scripts/hepta-native-android-emulator-smoke.sh
grep -Fq 'real_low_power_qualification:false' scripts/hepta-native-android-emulator-smoke.sh
grep -Fq 'android_real_device_receipt_missing' scripts/hepta-native-android-emulator-smoke.sh
grep -Fq 'talkback_receipt_missing' scripts/hepta-native-android-emulator-smoke.sh
ruby -e '
  source = File.read(ARGV.fetch(0))
  snapshot = source.index(%q{EMULATOR_STATE_SNAPSHOT="$(hepta_android_emulator_lab_state_snapshot}) or abort "lab snapshot missing"
  rejection = source.index("hepta_android_emulator_lab_state_ready", snapshot) or abort "lab rejection missing"
  mutation = source.index(%q{shell settings put system accelerometer_rotation 0}, rejection) or abort "first emulator mutation missing"
  abort "frozen battery rejection is not before mutation" unless snapshot < rejection && rejection < mutation
' scripts/hepta-native-android-emulator-smoke.sh
grep -Fq 'grep -Fq "https://github.com/ProfAlexQI/Hepta/commit/$SOURCE_HEAD"' scripts/hepta-native-android-emulator-smoke.sh
if grep -Fq 'grep -Fxq "https://github.com/ProfAlexQI/Hepta/commit/$SOURCE_HEAD"' scripts/hepta-native-android-emulator-smoke.sh; then
  echo "Android producer requires the embedded HEAD URL to occupy a whole strings(1) line" >&2
  exit 1
fi
grep -Fq 'hepta_android_emulator_receipt_v3_ready($head; $tree; $fingerprint; $login_manifest; $login_manifest_sha)' scripts/hepta-native-mobile-readiness-gate.sh
[[ "$(rg -c -- '--arg login_manifest ' scripts/hepta-native-mobile-readiness-gate.sh)" == "1" ]] || {
  echo "Android receipt consumer must pass the login manifest through one canonical jq predicate" >&2
  exit 1
}

echo "hepta-native Android emulator smoke self-test: PASS"
