#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

IOS_RECEIPT=""
REPORT_PATH=""
BASELINE_SCREENSHOT=""
KEYBOARD_SCREENSHOT=""
PRODUCER="scripts/hepta-native-ios-simulator-ui-qualification.sh"

usage() {
  cat <<'EOF'
usage: scripts/hepta-native-ios-simulator-ui-qualification.sh \
  --ios-receipt /absolute/ios-simulator-receipt.json \
  --output /absolute/ui-qualification.json \
  --baseline-screenshot /absolute/login.png \
  --keyboard-screenshot /absolute/login-with-keyboard.png

Reinstalls the exact app artifact from a current-source iOS simulator receipt
onto that same already-booted simulator, launches the fresh unauthenticated app,
focuses the homeserver field through the real Simulator window, captures the
software keyboard, and emits source-bound login-surface safe-area/keyboard
evidence. No credentials, real device, signing, upload, or publication is used.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --ios-receipt) IOS_RECEIPT="${2:-}"; shift 2 ;;
    --output) REPORT_PATH="${2:-}"; shift 2 ;;
    --baseline-screenshot) BASELINE_SCREENSHOT="${2:-}"; shift 2 ;;
    --keyboard-screenshot) KEYBOARD_SCREENSHOT="${2:-}"; shift 2 ;;
    --help|-h) usage; exit 0 ;;
    *) echo "error: unknown argument: $1" >&2; usage >&2; exit 64 ;;
  esac
done

for command in git jq shasum strings xcrun plutil ditto sips ruby peekaboo open; do
  command -v "$command" >/dev/null 2>&1 || { echo "error: $command is required" >&2; exit 2; }
done
[[ -x /usr/bin/caffeinate ]] || { echo "error: /usr/bin/caffeinate is required" >&2; exit 2; }

external_path() {
  local requested="$1"
  local parent
  local base
  [[ "$requested" == /* ]] || { echo "error: evidence paths must be absolute" >&2; return 1; }
  [[ ! -L "$requested" ]] || { echo "error: evidence paths must not be symlinks" >&2; return 1; }
  parent="$(dirname "$requested")"
  base="$(basename "$requested")"
  mkdir -p "$parent"
  parent="$(cd "$parent" && pwd -P)"
  case "$parent/$base" in
    "$ROOT_DIR"|"$ROOT_DIR"/*)
      echo "error: runtime evidence must be outside the source repository" >&2
      return 1
      ;;
  esac
  printf '%s/%s\n' "$parent" "$base"
}

[[ "$IOS_RECEIPT" == /* && -s "$IOS_RECEIPT" && ! -L "$IOS_RECEIPT" ]] \
  || { echo "error: --ios-receipt must be an absolute non-symlink file" >&2; exit 64; }
IOS_RECEIPT="$(cd "$(dirname "$IOS_RECEIPT")" && pwd -P)/$(basename "$IOS_RECEIPT")"
REPORT_PATH="$(external_path "$REPORT_PATH")"
BASELINE_SCREENSHOT="$(external_path "$BASELINE_SCREENSHOT")"
KEYBOARD_SCREENSHOT="$(external_path "$KEYBOARD_SCREENSHOT")"
[[ "$REPORT_PATH" != "$BASELINE_SCREENSHOT" && "$REPORT_PATH" != "$KEYBOARD_SCREENSHOT" \
  && "$BASELINE_SCREENSHOT" != "$KEYBOARD_SCREENSHOT" ]] \
  || { echo "error: output paths must be distinct" >&2; exit 64; }

SOURCE_BEFORE="$(scripts/hepta-ui-source-fingerprint)"
jq -e '.worktree_clean == true and .repository_worktree_clean == true' >/dev/null <<<"$SOURCE_BEFORE" \
  || { echo "error: iOS UI qualification requires a clean committed worktree" >&2; exit 1; }
SOURCE_HEAD="$(jq -r '.head' <<<"$SOURCE_BEFORE")"
SOURCE_TREE="$(jq -r '.head_tree' <<<"$SOURCE_BEFORE")"
SOURCE_FINGERPRINT="$(jq -r '.source_fingerprint' <<<"$SOURCE_BEFORE")"

jq -e \
  --arg head "$SOURCE_HEAD" --arg tree "$SOURCE_TREE" --arg fingerprint "$SOURCE_FINGERPRINT" '
    .schema_version == 1
    and .kind == "hepta-native-ios-simulator-smoke-receipt"
    and .producer == "scripts/hepta-native-ios-simulator-smoke.sh"
    and .status == "ready"
    and .ready == true
    and .source_binding.head == $head
    and .source_binding.head_tree == $tree
    and .source_binding.source_fingerprint == $fingerprint
    and .bundle.identifier == "ai.hepta.nativeapp"
    and .bundle.executable == "hepta-native"
    and .launch.install_succeeded == true
    and .launch.launch_succeeded == true
    and .forbidden_actions_performed.real_device_contact == false
    and .forbidden_actions_performed.code_sign == false
    and .forbidden_actions_performed.upload == false
  ' "$IOS_RECEIPT" >/dev/null || { echo "error: invalid or stale iOS simulator receipt" >&2; exit 1; }

ARTIFACT_PATH="$(jq -r '.artifact.path' "$IOS_RECEIPT")"
ARTIFACT_SHA256="$(jq -r '.artifact.sha256' "$IOS_RECEIPT")"
UDID="$(jq -r '.device.udid' "$IOS_RECEIPT")"
DEVICE_NAME="$(jq -r '.device.name' "$IOS_RECEIPT")"
RUNTIME_IDENTIFIER="$(jq -r '.device.runtime_identifier' "$IOS_RECEIPT")"
[[ "$ARTIFACT_PATH" == /* && -s "$ARTIFACT_PATH" && ! -L "$ARTIFACT_PATH" ]] \
  || { echo "error: iOS artifact is missing or unsafe" >&2; exit 1; }
[[ "$(shasum -a 256 "$ARTIFACT_PATH" | awk '{print $1}')" == "$ARTIFACT_SHA256" ]] \
  || { echo "error: iOS artifact hash mismatch" >&2; exit 1; }

DEVICE_REPORT="$(xcrun simctl list devices -j | jq -c --arg udid "$UDID" --arg runtime "$RUNTIME_IDENTIFIER" '
  [.devices[$runtime][]? | select(.udid == $udid)] | if length == 1 then .[0] else null end
')"
jq -e '.state == "Booted" and .isAvailable == true' >/dev/null <<<"$DEVICE_REPORT" \
  || { echo "error: receipt simulator is not uniquely booted and available" >&2; exit 1; }

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ios-ui-qualification.XXXXXX")"
DISPLAY_WAKE_PID=""
KEYBOARD_TOGGLED=false
SIMULATOR_WINDOW_ID=""
cleanup() {
  if [[ "$KEYBOARD_TOGGLED" == true && -n "$SIMULATOR_WINDOW_ID" ]]; then
    peekaboo hotkey --no-remote --app Simulator --window-id "$SIMULATOR_WINDOW_ID" \
      --keys 'cmd,k' --json >/dev/null 2>&1 || true
  fi
  xcrun simctl terminate "$UDID" ai.hepta.nativeapp >/dev/null 2>&1 || true
  if [[ -n "$DISPLAY_WAKE_PID" ]]; then
    kill "$DISPLAY_WAKE_PID" >/dev/null 2>&1 || true
    wait "$DISPLAY_WAKE_PID" >/dev/null 2>&1 || true
  fi
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

/usr/bin/caffeinate -dimsu -w "$$" >/dev/null 2>&1 &
DISPLAY_WAKE_PID=$!
sleep 1
kill -0 "$DISPLAY_WAKE_PID" >/dev/null 2>&1 \
  || { echo "error: display wake assertion did not start" >&2; exit 1; }

ditto -x -k "$ARTIFACT_PATH" "$TMP_DIR/artifact"
APP_PATH="$(find "$TMP_DIR/artifact" -type d -name '*.app' -maxdepth 3 -print | head -1)"
[[ -n "$APP_PATH" && -f "$APP_PATH/Info.plist" ]] \
  || { echo "error: exact iOS app bundle was not found in the receipt artifact" >&2; exit 1; }
[[ "$(find "$TMP_DIR/artifact" -type d -name '*.app' -maxdepth 3 -print | wc -l | tr -d ' ')" == "1" ]] \
  || { echo "error: iOS artifact must contain exactly one app bundle" >&2; exit 1; }
BUNDLE_ID="$(plutil -extract CFBundleIdentifier raw -o - "$APP_PATH/Info.plist")"
EXECUTABLE="$(plutil -extract CFBundleExecutable raw -o - "$APP_PATH/Info.plist")"
[[ "$BUNDLE_ID" == "ai.hepta.nativeapp" && "$EXECUTABLE" == "hepta-native" ]] \
  || { echo "error: staged iOS app identity mismatch" >&2; exit 1; }
strings "$APP_PATH/$EXECUTABLE" \
  | grep -F "https://github.com/ProfAlexQI/Hepta/commit/$SOURCE_HEAD" >/dev/null \
  || { echo "error: staged iOS executable does not embed the exact current HEAD" >&2; exit 1; }

xcrun simctl uninstall "$UDID" "$BUNDLE_ID" >/dev/null 2>&1 || true
xcrun simctl install "$UDID" "$APP_PATH"
LAUNCH_OUTPUT="$(xcrun simctl launch "$UDID" "$BUNDLE_ID")"
LAUNCH_PID="$(awk '{print $NF}' <<<"$LAUNCH_OUTPUT")"
[[ "$LAUNCH_PID" =~ ^[1-9][0-9]*$ ]] || { echo "error: iOS launch did not return a PID" >&2; exit 1; }

open -a Simulator --args -CurrentDeviceUDID "$UDID"
WINDOW_LIST="$TMP_DIR/simulator-windows.json"
for _ in {1..30}; do
  peekaboo list windows --no-remote --app Simulator --include-details bounds,ids --json >"$WINDOW_LIST" 2>/dev/null || true
  SIMULATOR_WINDOW_ID="$(jq -r --arg name "$DEVICE_NAME" '
    [.data.windows[]? | select((.title // "") | startswith($name + " "))][0].windowID // empty
  ' "$WINDOW_LIST" 2>/dev/null || true)"
  [[ -z "$SIMULATOR_WINDOW_ID" ]] || break
  sleep 0.5
done
[[ -n "$SIMULATOR_WINDOW_ID" ]] || { echo "error: unique Simulator device window was not found" >&2; exit 1; }
WINDOW_MATCH_COUNT="$(jq --arg name "$DEVICE_NAME" '[.data.windows[]? | select((.title // "") | startswith($name + " "))] | length' "$WINDOW_LIST")"
[[ "$WINDOW_MATCH_COUNT" == "1" ]] || { echo "error: Simulator device window was not unique" >&2; exit 1; }
peekaboo window focus --no-remote --app Simulator --window-id "$SIMULATOR_WINDOW_ID" --json >/dev/null
sleep 2
peekaboo list windows --no-remote --app Simulator --include-details bounds,ids --json >"$WINDOW_LIST"
WINDOW_X="$(jq -r --argjson id "$SIMULATOR_WINDOW_ID" '.data.windows[] | select(.windowID == $id) | .bounds[0][0]' "$WINDOW_LIST")"
WINDOW_Y="$(jq -r --argjson id "$SIMULATOR_WINDOW_ID" '.data.windows[] | select(.windowID == $id) | .bounds[0][1]' "$WINDOW_LIST")"
WINDOW_WIDTH="$(jq -r --argjson id "$SIMULATOR_WINDOW_ID" '.data.windows[] | select(.windowID == $id) | .bounds[1][0]' "$WINDOW_LIST")"
WINDOW_HEIGHT="$(jq -r --argjson id "$SIMULATOR_WINDOW_ID" '.data.windows[] | select(.windowID == $id) | .bounds[1][1]' "$WINDOW_LIST")"
for value in "$WINDOW_X" "$WINDOW_Y" "$WINDOW_WIDTH" "$WINDOW_HEIGHT"; do
  [[ "$value" =~ ^[0-9]+$ ]] || { echo "error: invalid Simulator window bounds" >&2; exit 1; }
done
(( WINDOW_WIDTH >= 300 && WINDOW_HEIGHT >= 600 )) \
  || { echo "error: Simulator device window is too small for qualification" >&2; exit 1; }

xcrun simctl io "$UDID" screenshot --type=png "$BASELINE_SCREENSHOT" >/dev/null
BASELINE_PROBE="$TMP_DIR/baseline-content.json"
scripts/hepta-image-content-probe --image "$BASELINE_SCREENSHOT" --output "$BASELINE_PROBE" >/dev/null
jq -e '.ready == true' "$BASELINE_PROBE" >/dev/null || { echo "error: baseline screenshot content is not ready" >&2; exit 1; }

CLICK_X=$((WINDOW_X + WINDOW_WIDTH / 2))
CLICK_Y=$((WINDOW_Y + WINDOW_HEIGHT * 42 / 100))
peekaboo click --no-remote --coords "$CLICK_X,$CLICK_Y" --no-auto-focus --json >/dev/null
sleep 1
CANDIDATE_SCREENSHOT="$TMP_DIR/keyboard-candidate.png"
CANDIDATE_PROBE="$TMP_DIR/keyboard-candidate-probe.json"
xcrun simctl io "$UDID" screenshot --type=png "$CANDIDATE_SCREENSHOT" >/dev/null
if scripts/hepta-ios-login-ui-probe --baseline "$BASELINE_SCREENSHOT" \
    --keyboard "$CANDIDATE_SCREENSHOT" --output "$CANDIDATE_PROBE" >/dev/null 2>&1; then
  cp "$CANDIDATE_SCREENSHOT" "$KEYBOARD_SCREENSHOT"
  KEYBOARD_CAPTURE_MODE="already_visible_after_field_focus"
else
  peekaboo hotkey --no-remote --app Simulator --window-id "$SIMULATOR_WINDOW_ID" \
    --keys 'cmd,k' --json >/dev/null
  KEYBOARD_TOGGLED=true
  sleep 2
  xcrun simctl io "$UDID" screenshot --type=png "$KEYBOARD_SCREENSHOT" >/dev/null
  KEYBOARD_CAPTURE_MODE="simulator_software_keyboard_toggle"
fi

KEYBOARD_CONTENT_PROBE="$TMP_DIR/keyboard-content.json"
UI_PROBE="$TMP_DIR/ios-login-ui-probe.json"
scripts/hepta-image-content-probe --image "$KEYBOARD_SCREENSHOT" --output "$KEYBOARD_CONTENT_PROBE" >/dev/null
jq -e '.ready == true' "$KEYBOARD_CONTENT_PROBE" >/dev/null \
  || { echo "error: keyboard screenshot content is not ready" >&2; exit 1; }
scripts/hepta-ios-login-ui-probe --baseline "$BASELINE_SCREENSHOT" \
  --keyboard "$KEYBOARD_SCREENSHOT" --output "$UI_PROBE" >/dev/null

SOURCE_FINAL="$(scripts/hepta-ui-source-fingerprint)"
jq -e --arg head "$SOURCE_HEAD" --arg tree "$SOURCE_TREE" --arg fingerprint "$SOURCE_FINGERPRINT" '
  .head == $head and .head_tree == $tree and .source_fingerprint == $fingerprint
  and .worktree_clean == true and .repository_worktree_clean == true
' >/dev/null <<<"$SOURCE_FINAL" || { echo "error: source changed during iOS UI qualification" >&2; exit 1; }

IOS_RECEIPT_SHA256="$(shasum -a 256 "$IOS_RECEIPT" | awk '{print $1}')"
BASELINE_SHA256="$(shasum -a 256 "$BASELINE_SCREENSHOT" | awk '{print $1}')"
KEYBOARD_SHA256="$(shasum -a 256 "$KEYBOARD_SCREENSHOT" | awk '{print $1}')"
UI_PROBE_JSON="$(cat "$UI_PROBE")"
REPORT="$(jq -n \
  --arg generated_at_utc "$(date -u +%Y-%m-%dT%H:%M:%SZ)" --arg producer "$PRODUCER" \
  --argjson source_binding "$SOURCE_FINAL" --arg ios_receipt "$IOS_RECEIPT" \
  --arg ios_receipt_sha "$IOS_RECEIPT_SHA256" --arg artifact "$ARTIFACT_PATH" \
  --arg artifact_sha "$ARTIFACT_SHA256" --arg udid "$UDID" --arg device_name "$DEVICE_NAME" \
  --arg runtime "$RUNTIME_IDENTIFIER" --argjson device "$DEVICE_REPORT" \
  --arg launch_output "$LAUNCH_OUTPUT" --argjson launch_pid "$LAUNCH_PID" \
  --argjson window_id "$SIMULATOR_WINDOW_ID" --arg window_bounds "$WINDOW_X,$WINDOW_Y,$WINDOW_WIDTH,$WINDOW_HEIGHT" \
  --argjson click_x "$CLICK_X" --argjson click_y "$CLICK_Y" --arg keyboard_capture_mode "$KEYBOARD_CAPTURE_MODE" \
  --arg baseline "$BASELINE_SCREENSHOT" --arg baseline_sha "$BASELINE_SHA256" \
  --arg keyboard "$KEYBOARD_SCREENSHOT" --arg keyboard_sha "$KEYBOARD_SHA256" \
  --argjson ui_probe "$UI_PROBE_JSON" '
    {
      schema_version:1,
      kind:"hepta-native-ios-simulator-ui-qualification",
      producer:$producer,
      generated_at_utc:$generated_at_utc,
      status:"ready",
      ready:true,
      source_binding:$source_binding,
      source_stable_during_run:true,
      scope:"unauthenticated_ios_simulator_login_surface",
      input_receipt:{path:$ios_receipt,sha256:$ios_receipt_sha,artifact:{path:$artifact,sha256:$artifact_sha}},
      device:{udid:$udid,name:$device_name,runtime_identifier:$runtime,state:$device.state,is_available:$device.isAvailable,real_device:false},
      launch:{fresh_uninstall_install:true,ready:true,pid:$launch_pid,output:$launch_output,credentials_supplied:false},
      simulator_window:{app:"Simulator",window_id:$window_id,exact_device_title_match_count:1,bounds:$window_bounds,focus_ready:true,field_focus_coordinate:{x:$click_x,y:$click_y},display_wake_backend:"/usr/bin/caffeinate"},
      captures:{baseline:{path:$baseline,sha256:$baseline_sha},software_keyboard:{path:$keyboard,sha256:$keyboard_sha,capture_mode:$keyboard_capture_mode}},
      ui_probe:$ui_probe,
      claims:{
        ios_simulator_login_software_keyboard_ready:true,
        ios_simulator_login_safe_area_ready:true,
        generic_software_keyboard_ready:false,
        generic_safe_area_ready:false,
        ios_real_device_ready:false,
        voiceover_ready:false,
        rtl_ready:false,
        dynamic_type_ready:false,
        public_distribution_ready:false
      },
      local_simulator_side_effects:{fresh_app_install:true,app_launch:true,field_focus:true,software_keyboard_toggle:($keyboard_capture_mode == "simulator_software_keyboard_toggle"),screenshot_capture:true},
      forbidden_actions_performed:{credential_supply:false,real_device_contact:false,code_sign:false,upload:false,publish:false},
      external_side_effects_performed:false
    }
  ')"

temporary="$(mktemp "$(dirname "$REPORT_PATH")/.hepta-ios-ui-qualification.XXXXXX")"
printf '%s\n' "$REPORT" >"$temporary"
mv -f -- "$temporary" "$REPORT_PATH"
printf '%s\n' "$REPORT"
