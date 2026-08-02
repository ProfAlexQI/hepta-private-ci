#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-android-live-readback-self-test.XXXXXX")"
trap 'rm -rf "$TEST_DIR"' EXIT

# Preserve the production wrapper unchanged while placing a copy under a
# temporary source root whose trusted-adb resolver is a hermetic fixture. The
# production resolver itself has no environment/test bypass.
FAKE_ROOT="$TEST_DIR/fake-root"
FAKE_SDK="$TEST_DIR/Android/sdk"
FAKE_ADB="$FAKE_SDK/platform-tools/adb"
FAKE_APK="$TEST_DIR/installed-base.apk"
FAKE_STATE="$TEST_DIR/fake-mode"
MUTATION_LOG="$TEST_DIR/mutations.log"
PID_COUNTER="$TEST_DIR/pid-counter"
STAT_COUNTER="$TEST_DIR/stat-counter"
FOCUS_COUNTER="$TEST_DIR/focus-counter"
HELPER="$FAKE_ROOT/scripts/hepta-native-android-emulator-live-readback"
mkdir -p "$FAKE_ROOT/scripts" "$(dirname "$FAKE_ADB")"
cp scripts/hepta-native-android-emulator-live-readback "$HELPER"
chmod 0755 "$HELPER"

printf 'current source-bound apk bytes\n' >"$FAKE_APK"
printf 'ready\n' >"$FAKE_STATE"
: >"$MUTATION_LOG"
: >"$PID_COUNTER"
: >"$STAT_COUNTER"
: >"$FOCUS_COUNTER"

cat >"$FAKE_ADB" <<FAKE_ADB_SCRIPT
#!/usr/bin/env bash
set -euo pipefail

if [[ "\${1:-}" == "-L" ]]; then shift 2; fi
case "\${1:-}" in
  start-server|kill-server) exit 0 ;;
esac
[[ "\${1:-}" == "-s" ]] || { printf 'transport enumeration forbidden\n' >&2; exit 96; }
serial="\${2:-}"
shift 2
[[ "\$serial" == "emulator-5554" ]] || exit 97
mode="\$(<"$FAKE_STATE")"
case "\$*" in
  get-state) printf '%s\n' device ;;
  'emu avd name') printf '%s\n' Hepta_Pixel_API_34_arm64 OK ;;
  'shell getprop ro.boot.qemu.avd_name') printf '%s\n' Hepta_Pixel_API_34_arm64 ;;
  'shell cat /proc/sys/kernel/random/boot_id') printf '%s\n' 11111111-2222-4333-8444-555555555555 ;;
  'shell getprop sys.boot_completed') printf '%s\n' 1 ;;
  'shell getprop ro.product.cpu.abi') [[ "\$mode" == abi ]] && printf '%s\n' x86_64 || printf '%s\n' arm64-v8a ;;
  'shell uname -m') printf '%s\n' aarch64 ;;
  'shell pm path ai.hepta.nativeapp') printf '%s\n' 'package:/data/app/~~hepta==/ai.hepta.nativeapp-current==/base.apk' ;;
  'exec-out cat /data/app/~~hepta==/ai.hepta.nativeapp-current==/base.apk') command cat "$FAKE_APK" ;;
  'exec-out cat /data/local/tmp/hepta-native-smoke-0123456789abcdef01234567') printf '%s' aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee ;;
  'shell dumpsys package ai.hepta.nativeapp')
    printf '%s\n' '  versionCode=42 minSdk=26 targetSdk=35'
    [[ "\$mode" == version ]] && printf '%s\n' '  versionName=9.9.9' || printf '%s\n' '  versionName=0.1.0'
    ;;
  'shell pidof ai.hepta.nativeapp')
    count=\$(wc -l <"$PID_COUNTER" | tr -d ' ')
    printf x >>"$PID_COUNTER"; printf '\n' >>"$PID_COUNTER"
    if [[ "\$mode" == pid_pre ]]; then
      printf '%s\n' 9999
    elif [[ "\$mode" == pid_after && "\$count" -ge 1 ]]; then
      printf '%s\n' 9999
    else
      printf '%s\n' 2468
    fi
    ;;
  'shell cat /proc/2468/stat')
    count=\$(wc -l <"$STAT_COUNTER" | tr -d ' ')
    printf x >>"$STAT_COUNTER"; printf '\n' >>"$STAT_COUNTER"
    if [[ "\$mode" == process_after && "\$count" -ge 1 ]]; then start_ticks=999999; else start_ticks=123456; fi
    printf '2468 (ai.hepta.nativeapp) S 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 %s 20 21\n' "\$start_ticks"
    ;;
  'shell am start -W -n ai.hepta.nativeapp/.MakepadApp')
    printf '%s\n' am_start_credential_free >>"$MUTATION_LOG"
    if [[ "\$mode" == start_status ]]; then printf '%s\n' 'Status: error'; else printf '%s\n' 'Status: ok' 'Activity: ai.hepta.nativeapp/.MakepadApp' 'TotalTime: 3'; fi
    ;;
  'shell dumpsys activity activities')
    count=\$(wc -l <"$FOCUS_COUNTER" | tr -d ' ')
    printf x >>"$FOCUS_COUNTER"; printf '\n' >>"$FOCUS_COUNTER"
    if [[ "\$mode" == focus_pre || ("\$mode" == focus_after && "\$count" -ge 1) ]]; then
      printf '%s\n' 'topResumedActivity=ActivityRecord{abc u0 com.android.launcher/.Launcher t1}'
    else
      printf '%s\n' \
        'topResumedActivity=ActivityRecord{abc u0 ai.hepta.nativeapp/.MakepadApp t1}' \
        'mCurrentFocus=Window{abc u0 ai.hepta.nativeapp/ai.hepta.nativeapp.MakepadApp}' \
        'mFocusedApp=ActivityRecord{abc u0 ai.hepta.nativeapp/.MakepadApp t1}'
    fi
    ;;
  *) printf 'unexpected fake adb command: %s\n' "\$*" >&2; exit 98 ;;
esac
FAKE_ADB_SCRIPT
chmod 0755 "$FAKE_ADB"

ADB_SHA="$(shasum -a 256 "$FAKE_ADB" | awk '{print $1}')"
cat >"$FAKE_ROOT/scripts/hepta-android-trusted-adb" <<FAKE_RESOLVER
#!/usr/bin/env bash
set -euo pipefail
jq -n --arg adb "$FAKE_ADB" --arg sha "$ADB_SHA" --arg sdk "$FAKE_SDK" '
  {schema_version:1,kind:"hepta-android-trusted-adb",status:"ready",ready:true,account_home:"/tmp",sdk_root:\$sdk,adb:{path:\$adb,sha256:\$sha,architecture:"arm64",identifier:"adb",authority:"Developer ID Application: Google LLC (EQHXZ8M8AV)",team_identifier:"EQHXZ8M8AV",strict_codesign_verified:true},caller_environment_trusted:false}
'
FAKE_RESOLVER
chmod 0755 "$FAKE_ROOT/scripts/hepta-android-trusted-adb"

APK_SHA="$(shasum -a 256 "$FAKE_APK" | awk '{print $1}')"
SESSION_NONCE="aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee"
SESSION_SHA="$(printf '%s' "$SESSION_NONCE" | shasum -a 256 | awk '{print $1}')"
RECEIPT="$TEST_DIR/receipt.json"
jq -n \
  --arg adb "$FAKE_ADB" --arg adb_sha "$ADB_SHA" --arg apk_sha "$APK_SHA" \
  --arg session_nonce "$SESSION_NONCE" --arg session_sha "$SESSION_SHA" '
  {
    host_toolchain:{adb_binary_path:$adb,adb_binary_sha256:$adb_sha},
    device:{adb_serial:"emulator-5554",avd_name:"Hepta_Pixel_API_34_arm64",qemu_avd_name:"Hepta_Pixel_API_34_arm64",boot_id:"11111111-2222-4333-8444-555555555555"},
    artifact:{sha256:$apk_sha,version_code:42,version_name:"0.1.0"},
    runtime:{installed_package_path:"/data/app/~~hepta==/ai.hepta.nativeapp-current==/base.apk",pid:2468,process_start_time_ticks:123456},
    session_probe:{path:"/data/local/tmp/hepta-native-smoke-0123456789abcdef01234567",nonce:$session_nonce,sha256:$session_sha,boot_id:"11111111-2222-4333-8444-555555555555"}
  }
' >"$RECEIPT"

reset_fake() {
  printf '%s\n' "${1:-ready}" >"$FAKE_STATE"
  : >"$PID_COUNTER"
  : >"$STAT_COUNTER"
  : >"$FOCUS_COUNTER"
}
start_count() {
  grep -c '^am_start_credential_free$' "$MUTATION_LOG" || true
}

bash -n "$HELPER"
if rg -n '(^|[[:space:]])devices[[:space:]]+-l([[:space:]]|$)' scripts/hepta-native-android-emulator-live-readback >/dev/null; then
  echo "production live readback enumerates all adb transports" >&2
  exit 1
fi
reset_fake ready
"$HELPER" --receipt "$RECEIPT" --output "$TEST_DIR/readback.json" >/dev/null
jq -e '
  .status == "ready"
  and .ready == true
  and .trusted_adb.adb.team_identifier == "EQHXZ8M8AV"
  and .adb_server.caller_routing_environment_trusted == false
  and .adb_server.private_socket == true
  and .adb_server.all_transports_enumerated == false
  and .device.serial == "emulator-5554"
  and .device.real_device_contacted == false
  and .package.apk_sha256 == $apk_sha
  and .package.pid == 2468
  and .package.process_start_time_ticks == 123456
  and .credential_free_current_session_probe.pid_before == 2468
  and .credential_free_current_session_probe.pid_after == 2468
  and .credential_free_current_session_probe.pid_unchanged == true
  and .credential_free_current_session_probe.process_instance_unchanged == true
  and .credential_free_current_session_probe.focus_ready_before == true
  and .credential_free_current_session_probe.focus_ready_after == true
  and .independently_verified_claims.emulator_runtime == true
  and .independently_verified_claims.unauthenticated_login_surface_visual == false
  and .independently_verified_claims.unauthenticated_login_surface_rotation == false
  and .independently_verified_claims.unauthenticated_login_surface_ime == false
  and (.forbidden_actions_performed | to_entries | all(.value == false))
' --arg apk_sha "$APK_SHA" "$TEST_DIR/readback.json" >/dev/null
[[ "$(start_count)" == "1" ]]

expect_failure() {
  local label="$1" mode="$2" filter="$3" expected_start_delta="$4" before
  reset_fake "$mode"
  before="$(start_count)"
  jq "$filter" "$RECEIPT" >"$TEST_DIR/$label.json"
  if "$HELPER" --receipt "$TEST_DIR/$label.json" >"$TEST_DIR/$label.stdout" 2>"$TEST_DIR/$label.stderr"; then
    echo "live readback accepted forged/runtime-invalid receipt: $label" >&2
    exit 1
  fi
  [[ "$(( $(start_count) - before ))" == "$expected_start_delta" ]] || {
    echo "unexpected start-probe count while rejecting: $label" >&2
    exit 1
  }
}

expect_failure nonexistent_serial ready '.device.adb_serial = "emulator-5999"' 0
grep -Fq 'not an online device' "$TEST_DIR/nonexistent_serial.stderr"
expect_failure real_device_serial ready '.device.adb_serial = "R58M123456A"' 0
grep -Fq 'not an emulator serial' "$TEST_DIR/real_device_serial.stderr"
expect_failure boot_session ready '.device.boot_id = "99999999-2222-4333-8444-555555555555" | .session_probe.boot_id = .device.boot_id' 0
expect_failure avd_identity ready '.device.avd_name = "forged_avd" | .device.qemu_avd_name = .device.avd_name' 0
expect_failure session_nonce ready '.session_probe.nonce = "ffffffff-bbbb-4ccc-8ddd-eeeeeeeeeeee" | .session_probe.sha256 = "642527897b46843e2892f6a7de459406036379ef4285eb42441b5dca9e17f0b1"' 0
expect_failure apk_bytes ready '.artifact.sha256 = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"' 0
expect_failure adb_bytes ready '.host_toolchain.adb_binary_sha256 = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"' 0
expect_failure abi abi '.' 0
expect_failure version version '.' 0
expect_failure pid_before pid_pre '.' 0
expect_failure focus_before focus_pre '.' 0
expect_failure start_status start_status '.' 1
expect_failure pid_after pid_after '.' 1
expect_failure process_after process_after '.' 1
expect_failure focus_after focus_after '.' 1

# The production resolver has no fake-SDK switch and must reject a hostile
# caller environment while still resolving the OS account's signed adb.
HOSTILE_SDK="$TEST_DIR/hostile-sdk"
mkdir -p "$HOSTILE_SDK/platform-tools"
printf '#!/usr/bin/env bash\nexit 0\n' >"$HOSTILE_SDK/platform-tools/adb"
chmod 0755 "$HOSTILE_SDK/platform-tools/adb"
env -i PATH=/usr/bin:/bin:/usr/sbin:/sbin \
  ANDROID_SDK_ROOT="$HOSTILE_SDK" ANDROID_HOME="$HOSTILE_SDK" HOME="$TEST_DIR/forged-home" \
  scripts/hepta-android-trusted-adb >"$TEST_DIR/trusted-adb.json"
jq -e --arg hostile "$HOSTILE_SDK/platform-tools/adb" '
  .ready == true
  and .caller_environment_trusted == false
  and .adb.path != $hostile
  and .adb.team_identifier == "EQHXZ8M8AV"
  and .adb.strict_codesign_verified == true
' "$TEST_DIR/trusted-adb.json" >/dev/null

echo "hepta-native Android emulator live readback self-test: PASS"
