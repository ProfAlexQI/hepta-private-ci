#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"
source "$ROOT_DIR/scripts/hepta-native-mobile-lab-cleanup-v1.sh"

IOS_RECEIPT=""
REPORT_PATH=""
BASELINE_SCREENSHOT=""
KEYBOARD_SCREENSHOT=""
PRODUCER="scripts/hepta-native-ios-simulator-ui-qualification.sh"
EXTENDED_LAB=false
LAB_CONTRACT_ONLY=false
LAB_EVIDENCE_DIR=""
LAB_STARTUP_SAMPLES=3

usage() {
  cat <<'EOF'
usage: scripts/hepta-native-ios-simulator-ui-qualification.sh \
  --ios-receipt /absolute/ios-simulator-receipt.json \
  --output /absolute/ui-qualification.json \
  --baseline-screenshot /absolute/login.png \
  --keyboard-screenshot /absolute/login-with-keyboard.png \
  [--extended-lab] [--lab-evidence-dir /absolute/lab-evidence]

Reinstalls the exact app artifact from a current-source iOS simulator receipt
onto that same already-booted simulator, launches the fresh unauthenticated app,
focuses the homeserver field through the real Simulator window, captures the
software keyboard, and emits source-bound login-surface safe-area/keyboard
evidence scoped to visible text anchors and conservative Login-frame clearance.
No credentials, real device, signing, upload, or publication is used.

  --extended-lab       additionally exercise reversible Simulator-only RTL,
                       Dynamic Type, rotation/keyboard, and repeated startup
                       modes. Unsupported iOS low-power emulation remains a
                       structured false claim; the mode never impersonates a
                       real-device power qualification.
  --lab-evidence-dir   external directory for extended-mode captures
  --lab-contract-only  print the side-effect-free extended-lab contract
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --ios-receipt) IOS_RECEIPT="${2:-}"; shift 2 ;;
    --output) REPORT_PATH="${2:-}"; shift 2 ;;
    --baseline-screenshot) BASELINE_SCREENSHOT="${2:-}"; shift 2 ;;
    --keyboard-screenshot) KEYBOARD_SCREENSHOT="${2:-}"; shift 2 ;;
    --extended-lab) EXTENDED_LAB=true; shift ;;
    --lab-evidence-dir) LAB_EVIDENCE_DIR="${2:-}"; shift 2 ;;
    --lab-contract-only) LAB_CONTRACT_ONLY=true; shift ;;
    --help|-h) usage; exit 0 ;;
    *) echo "error: unknown argument: $1" >&2; usage >&2; exit 64 ;;
  esac
done

for command in git jq shasum strings xcrun plutil ditto sips ruby peekaboo open osascript; do
  command -v "$command" >/dev/null 2>&1 || { echo "error: $command is required" >&2; exit 2; }
done
[[ -x /usr/bin/caffeinate ]] || { echo "error: /usr/bin/caffeinate is required" >&2; exit 2; }

if [[ "$LAB_CONTRACT_ONLY" == true ]]; then
  jq -n --arg producer "$PRODUCER" '
    {
      schema_version:1,
      kind:"hepta-native-ios-simulator-extended-lab-source-contract",
      producer:$producer,
      status:"ready",
      ready:true,
      opt_in:true,
      modes:{rtl:true,dynamic_type:true,rotation_keyboard:true,startup_performance:true,low_power:true},
      state_contract:{snapshot_before_mutation:true,raw_orientation_snapshot:true,orientation_snapshot_backend:"simulator_ax_menu_mark",rtl_matched_control_before_mode:true,dynamic_type_matched_control_before_mode:true,mode_specific_raster_attribution:true,semantic_layout_claims_remain_false:true,restore_to_raw_orientation:true,exact_orientation_readback:true,snapshot_failure_rejected_before_mutation:true,restore_and_readback_before_receipt:true,restore_failure_fails_closed:true,exit_cleanup_preserves_original_status:true,interrupt_cleanup_restore_and_readback:true,cleanup_failure_receipt:true},
      claim_boundaries:{simulator_only:true,generic_app_wide:false,real_device:false,voiceover:false,effective_low_power:false},
      forbidden_actions:{credential_supply:false,real_device_contact:false,account_connection:false,code_sign:false,upload:false,publish:false},
      external_side_effects_performed:false
    }
  '
  exit 0
fi

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

external_directory_path() {
  local requested="$1"
  local resolved
  [[ "$requested" == /* ]] || { echo "error: evidence directories must be absolute" >&2; return 1; }
  mkdir -p "$requested"
  resolved="$(cd "$requested" && pwd -P)"
  case "$resolved" in
    "$ROOT_DIR"|"$ROOT_DIR"/*)
      echo "error: runtime evidence must be outside the source repository" >&2
      return 1
      ;;
  esac
  printf '%s\n' "$resolved"
}

[[ "$IOS_RECEIPT" == /* && -s "$IOS_RECEIPT" && ! -L "$IOS_RECEIPT" ]] \
  || { echo "error: --ios-receipt must be an absolute non-symlink file" >&2; exit 64; }
IOS_RECEIPT="$(cd "$(dirname "$IOS_RECEIPT")" && pwd -P)/$(basename "$IOS_RECEIPT")"
REPORT_PATH="$(external_path "$REPORT_PATH")"
BASELINE_SCREENSHOT="$(external_path "$BASELINE_SCREENSHOT")"
KEYBOARD_SCREENSHOT="$(external_path "$KEYBOARD_SCREENSHOT")"
if [[ "$EXTENDED_LAB" == true ]]; then
  [[ -n "$LAB_EVIDENCE_DIR" ]] || LAB_EVIDENCE_DIR="${REPORT_PATH%.json}.extended-lab"
  LAB_EVIDENCE_DIR="$(external_directory_path "$LAB_EVIDENCE_DIR")"
fi
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
LAB_STATE_MUTATED=false
LAB_ORIENTATION_MUTATED=false
ORIGINAL_CONTENT_SIZE=""
ORIGINAL_ORIENTATION=""
simulator_orientation() {
  /usr/bin/osascript -l JavaScript <<'JXA'
const systemEvents = Application('System Events');
const simulator = systemEvents.processes.byName('Simulator');
if (!simulator.exists()) throw new Error('Simulator process unavailable');
const menu = simulator.menuBars[0].menuBarItems.byName('Device').menus[0].menuItems.byName('Orientation').menus[0];
const names = ['Portrait', 'Landscape Right', 'Portrait Upside Down', 'Landscape Left', 'Face Up', 'Face Down'];
function mark(item) {
  const attributes = item.attributes();
  for (let index = 0; index < attributes.length; index++) {
    if (attributes[index].name() === 'AXMenuItemMarkChar') return attributes[index].value();
  }
  return null;
}
const selected = names.filter(name => mark(menu.menuItems.byName(name)) !== null);
if (selected.length !== 1) throw new Error('expected exactly one selected Simulator orientation');
selected[0];
JXA
}
wait_for_simulator_orientation() {
  local wanted="$1" observed
  for _ in {1..20}; do
    observed="$(simulator_orientation 2>/dev/null || true)"
    [[ "$observed" == "$wanted" ]] && return 0
    sleep 0.25
  done
  return 1
}
restore_ios_lab_state() {
  local failed=false
  if [[ "$LAB_ORIENTATION_MUTATED" == true ]]; then
    peekaboo menu click --no-remote --app Simulator \
      --path "Device > Orientation > $ORIGINAL_ORIENTATION" --json >/dev/null 2>&1 || failed=true
    wait_for_simulator_orientation "$ORIGINAL_ORIENTATION" || failed=true
  fi
  if [[ -n "$ORIGINAL_CONTENT_SIZE" ]]; then
    xcrun simctl ui "$UDID" content_size "$ORIGINAL_CONTENT_SIZE" >/dev/null 2>&1 || failed=true
  fi
  [[ "$failed" == false ]]
}
ios_lab_state_readback_ready() {
  local content_size orientation
  content_size="$(xcrun simctl ui "$UDID" content_size 2>/dev/null | tail -1 | tr -d '\r')" || return 1
  orientation="$(simulator_orientation 2>/dev/null)" || return 1
  [[ "$content_size" == "$ORIGINAL_CONTENT_SIZE" && "$orientation" == "$ORIGINAL_ORIENTATION" ]]
}
write_ios_cleanup_failure_receipt() {
  local original_exit="$1" restore_ready="$2" readback_ready="$3" temporary
  temporary="$(mktemp "$(dirname "$REPORT_PATH")/.hepta-ios-cleanup-failure.XXXXXX")" || return 1
  if ! hepta_mobile_cleanup_failure_json ios_simulator "$PRODUCER" "$original_exit" "$restore_ready" "$readback_ready" >"$temporary"; then
    rm -f -- "$temporary"
    return 1
  fi
  mv -f -- "$temporary" "$REPORT_PATH"
}
cleanup() {
  local original_exit=$? restore_ready=true readback_ready=true final_exit
  trap - EXIT HUP INT TERM
  set +e
  if [[ "$LAB_STATE_MUTATED" == true ]]; then
    restore_ios_lab_state || restore_ready=false
    ios_lab_state_readback_ready || readback_ready=false
    if [[ "$restore_ready" != true || "$readback_ready" != true ]]; then
      write_ios_cleanup_failure_receipt "$original_exit" "$restore_ready" "$readback_ready" \
        || echo "error: failed to write iOS cleanup failure receipt" >&2
    fi
  fi
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
  final_exit="$(hepta_mobile_cleanup_final_exit_code "$original_exit" "$restore_ready" "$readback_ready")" || final_exit=1
  exit "$final_exit"
}
trap cleanup EXIT
trap 'exit 129' HUP
trap 'exit 130' INT
trap 'exit 143' TERM

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

BASELINE_HOMESERVER_LOCATOR="$TMP_DIR/baseline-homeserver-locator.json"
scripts/hepta-ios-login-ui-probe --locate-homeserver \
  --baseline "$BASELINE_SCREENSHOT" --device-name "$DEVICE_NAME" \
  --output "$BASELINE_HOMESERVER_LOCATOR" >/dev/null
jq -e '
  .kind == "hepta-ios-homeserver-anchor-locator"
  and .ready == true
  and .locator.engine == "apple_vision_recognize_text"
  and .locator.match_count == 1
  and .claims.baseline_vision_homeserver_anchor_center_ready == true
  and .claims.generic_focus_ready == false
' "$BASELINE_HOMESERVER_LOCATOR" >/dev/null \
  || { echo "error: unique baseline homeserver anchor was not located" >&2; exit 1; }
TARGET_X_RATIO="$(jq -r '.locator.normalized_device_coordinate.x' "$BASELINE_HOMESERVER_LOCATOR")"
TARGET_Y_RATIO="$(jq -r '.locator.normalized_device_coordinate.y_from_top' "$BASELINE_HOMESERVER_LOCATOR")"
CLICK_X="$(ruby -e 'puts(ARGV[0].to_i + (ARGV[1].to_i * ARGV[2].to_f).round)' "$WINDOW_X" "$WINDOW_WIDTH" "$TARGET_X_RATIO")"
CLICK_Y="$(ruby -e 'puts(ARGV[0].to_i + (ARGV[1].to_i * ARGV[2].to_f).round)' "$WINDOW_Y" "$WINDOW_HEIGHT" "$TARGET_Y_RATIO")"
KEYBOARD_TRIGGER_MODE="direct_after_vision_homeserver_anchor_click"
peekaboo click --no-remote --coords "$CLICK_X,$CLICK_Y" --no-auto-focus --json >/dev/null
sleep 1
CANDIDATE_SCREENSHOT="$TMP_DIR/keyboard-candidate.png"
CANDIDATE_PROBE="$TMP_DIR/keyboard-candidate-probe.json"
xcrun simctl io "$UDID" screenshot --type=png "$CANDIDATE_SCREENSHOT" >/dev/null
scripts/hepta-ios-login-ui-probe --baseline "$BASELINE_SCREENSHOT" \
  --keyboard "$CANDIDATE_SCREENSHOT" --device-name "$DEVICE_NAME" \
  --target-x-ratio "$TARGET_X_RATIO" --target-y-ratio "$TARGET_Y_RATIO" \
  --keyboard-trigger-mode "$KEYBOARD_TRIGGER_MODE" \
  --output "$CANDIDATE_PROBE" >/dev/null \
  || { echo "error: OCR-located homeserver click did not directly produce qualified keyboard evidence" >&2; exit 1; }
cp "$CANDIDATE_SCREENSHOT" "$KEYBOARD_SCREENSHOT"
KEYBOARD_CAPTURE_MODE="$KEYBOARD_TRIGGER_MODE"

KEYBOARD_CONTENT_PROBE="$TMP_DIR/keyboard-content.json"
UI_PROBE="$TMP_DIR/ios-login-ui-probe.json"
scripts/hepta-image-content-probe --image "$KEYBOARD_SCREENSHOT" --output "$KEYBOARD_CONTENT_PROBE" >/dev/null
jq -e '.ready == true' "$KEYBOARD_CONTENT_PROBE" >/dev/null \
  || { echo "error: keyboard screenshot content is not ready" >&2; exit 1; }
scripts/hepta-ios-login-ui-probe --baseline "$BASELINE_SCREENSHOT" \
  --keyboard "$KEYBOARD_SCREENSHOT" --device-name "$DEVICE_NAME" \
  --target-x-ratio "$TARGET_X_RATIO" --target-y-ratio "$TARGET_Y_RATIO" \
  --keyboard-trigger-mode "$KEYBOARD_TRIGGER_MODE" \
  --output "$UI_PROBE" >/dev/null

LAB_RESULT='{"requested":false,"status":"not_requested","ready":false}'
if [[ "$EXTENDED_LAB" == true ]]; then
  lab_capture() {
    local path="$1" probe="$2"
    xcrun simctl io "$UDID" screenshot --type=png "$path" >/dev/null
    scripts/hepta-image-content-probe --image "$path" --output "$probe" >/dev/null
    jq -e '.ready == true' "$probe" >/dev/null
  }
  lab_launch() {
    local output pid
    output="$(xcrun simctl launch --terminate-running-process "$UDID" "$BUNDLE_ID")"
    pid="$(awk '{print $NF}' <<<"$output")"
    [[ "$pid" =~ ^[1-9][0-9]*$ ]]
    printf '%s\n' "$pid"
  }

  ORIGINAL_CONTENT_SIZE="$(xcrun simctl ui "$UDID" content_size | tail -1 | tr -d '\r')"
  case "$ORIGINAL_CONTENT_SIZE" in
    extra-small|small|medium|large|extra-large|extra-extra-large|extra-extra-extra-large|accessibility-medium|accessibility-large|accessibility-extra-large|accessibility-extra-extra-large|accessibility-extra-extra-extra-large) ;;
    *) echo "error: unsupported Simulator content-size readback: $ORIGINAL_CONTENT_SIZE" >&2; exit 1 ;;
  esac
  ORIGINAL_ORIENTATION="$(simulator_orientation)" \
    || { echo "error: Simulator orientation snapshot failed before extended-lab mutation" >&2; exit 1; }
  case "$ORIGINAL_ORIENTATION" in
    Portrait|Landscape\ Right|Portrait\ Upside\ Down|Landscape\ Left|Face\ Up|Face\ Down) ;;
    *) echo "error: unsupported Simulator orientation snapshot: $ORIGINAL_ORIENTATION" >&2; exit 1 ;;
  esac
  BASELINE_LAB_WIDTH="$(sips -g pixelWidth "$BASELINE_SCREENSHOT" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
  BASELINE_LAB_HEIGHT="$(sips -g pixelHeight "$BASELINE_SCREENSHOT" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"
  [[ "$BASELINE_LAB_WIDTH" =~ ^[1-9][0-9]*$ && "$BASELINE_LAB_HEIGHT" =~ ^[1-9][0-9]*$ ]] \
    || { echo "error: Simulator orientation baseline dimensions are unreadable" >&2; exit 1; }
  LAB_STATE_MUTATED=true

  SIMCTL_CHILD_APPLE_LANGUAGES='(en)' SIMCTL_CHILD_APPLE_LOCALE='en_US' \
    SIMCTL_CHILD_NSForceRightToLeftWritingDirection='NO' SIMCTL_CHILD_AppleTextDirection='NO' \
    lab_launch >/dev/null
  sleep 2
  RTL_CONTROL_PATH="$LAB_EVIDENCE_DIR/rtl-control-ltr.png"
  lab_capture "$RTL_CONTROL_PATH" "$LAB_EVIDENCE_DIR/rtl-control.content-probe.json"
  RTL_CONTROL_SHA="$(shasum -a 256 "$RTL_CONTROL_PATH" | awk '{print $1}')"
  RTL_CONTROL_WIDTH="$(sips -g pixelWidth "$RTL_CONTROL_PATH" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
  RTL_CONTROL_HEIGHT="$(sips -g pixelHeight "$RTL_CONTROL_PATH" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"

  RTL_PATH="$LAB_EVIDENCE_DIR/rtl-ar-SA.png"
  SIMCTL_CHILD_APPLE_LANGUAGES='(ar)' SIMCTL_CHILD_APPLE_LOCALE='ar_SA' \
    SIMCTL_CHILD_NSForceRightToLeftWritingDirection='YES' SIMCTL_CHILD_AppleTextDirection='YES' \
    xcrun simctl launch --terminate-running-process "$UDID" "$BUNDLE_ID" >/dev/null
  sleep 2
  lab_capture "$RTL_PATH" "$LAB_EVIDENCE_DIR/rtl.content-probe.json"
  RTL_SHA="$(shasum -a 256 "$RTL_PATH" | awk '{print $1}')"
  RTL_WIDTH="$(sips -g pixelWidth "$RTL_PATH" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
  RTL_HEIGHT="$(sips -g pixelHeight "$RTL_PATH" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"
  [[ "$RTL_SHA" != "$RTL_CONTROL_SHA" ]] && RTL_RASTER_CHANGED=true || RTL_RASTER_CHANGED=false
  [[ "$RTL_WIDTH" == "$RTL_CONTROL_WIDTH" && "$RTL_HEIGHT" == "$RTL_CONTROL_HEIGHT" ]] \
    && RTL_SAME_CANVAS=true || RTL_SAME_CANVAS=false

  lab_launch >/dev/null
  sleep 2
  DYNAMIC_CONTROL_PATH="$LAB_EVIDENCE_DIR/dynamic-type-control-$ORIGINAL_CONTENT_SIZE.png"
  lab_capture "$DYNAMIC_CONTROL_PATH" "$LAB_EVIDENCE_DIR/dynamic-type-control.content-probe.json"
  DYNAMIC_CONTROL_SHA="$(shasum -a 256 "$DYNAMIC_CONTROL_PATH" | awk '{print $1}')"
  DYNAMIC_CONTROL_WIDTH="$(sips -g pixelWidth "$DYNAMIC_CONTROL_PATH" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
  DYNAMIC_CONTROL_HEIGHT="$(sips -g pixelHeight "$DYNAMIC_CONTROL_PATH" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"

  DYNAMIC_TYPE_SIZE="accessibility-extra-extra-extra-large"
  xcrun simctl ui "$UDID" content_size "$DYNAMIC_TYPE_SIZE" >/dev/null
  [[ "$(xcrun simctl ui "$UDID" content_size | tail -1 | tr -d '\r')" == "$DYNAMIC_TYPE_SIZE" ]] \
    || { echo "error: Simulator Dynamic Type setting did not apply" >&2; exit 1; }
  lab_launch >/dev/null
  sleep 2
  DYNAMIC_PATH="$LAB_EVIDENCE_DIR/dynamic-type-axxxl.png"
  lab_capture "$DYNAMIC_PATH" "$LAB_EVIDENCE_DIR/dynamic-type.content-probe.json"
  DYNAMIC_SHA="$(shasum -a 256 "$DYNAMIC_PATH" | awk '{print $1}')"
  DYNAMIC_WIDTH="$(sips -g pixelWidth "$DYNAMIC_PATH" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
  DYNAMIC_HEIGHT="$(sips -g pixelHeight "$DYNAMIC_PATH" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"
  [[ "$DYNAMIC_SHA" != "$DYNAMIC_CONTROL_SHA" ]] && DYNAMIC_RASTER_CHANGED=true || DYNAMIC_RASTER_CHANGED=false
  [[ "$DYNAMIC_WIDTH" == "$DYNAMIC_CONTROL_WIDTH" && "$DYNAMIC_HEIGHT" == "$DYNAMIC_CONTROL_HEIGHT" ]] \
    && DYNAMIC_SAME_CANVAS=true || DYNAMIC_SAME_CANVAS=false
  xcrun simctl ui "$UDID" content_size "$ORIGINAL_CONTENT_SIZE" >/dev/null
  [[ "$(xcrun simctl ui "$UDID" content_size | tail -1 | tr -d '\r')" == "$ORIGINAL_CONTENT_SIZE" ]] \
    || { echo "error: Simulator content size did not restore" >&2; exit 1; }

  lab_launch >/dev/null
  sleep 1
  LAB_ORIENTATION_MUTATED=true
  peekaboo menu click --no-remote --app Simulator \
    --path 'Device > Orientation > Landscape Right' --json >/dev/null
  sleep 3
  LANDSCAPE_PATH="$LAB_EVIDENCE_DIR/landscape.png"
  lab_capture "$LANDSCAPE_PATH" "$LAB_EVIDENCE_DIR/landscape.content-probe.json"
  LANDSCAPE_WIDTH="$(sips -g pixelWidth "$LANDSCAPE_PATH" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
  LANDSCAPE_HEIGHT="$(sips -g pixelHeight "$LANDSCAPE_PATH" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"
  if (( LANDSCAPE_WIDTH > LANDSCAPE_HEIGHT )); then
    ROTATION_TRANSITION_READY=true
    peekaboo list windows --no-remote --app Simulator --include-details bounds,ids --json >"$WINDOW_LIST"
    LAB_WINDOW_X="$(jq -r --argjson id "$SIMULATOR_WINDOW_ID" '.data.windows[] | select(.windowID == $id) | .bounds[0][0]' "$WINDOW_LIST")"
    LAB_WINDOW_Y="$(jq -r --argjson id "$SIMULATOR_WINDOW_ID" '.data.windows[] | select(.windowID == $id) | .bounds[0][1]' "$WINDOW_LIST")"
    LAB_WINDOW_WIDTH="$(jq -r --argjson id "$SIMULATOR_WINDOW_ID" '.data.windows[] | select(.windowID == $id) | .bounds[1][0]' "$WINDOW_LIST")"
    LAB_WINDOW_HEIGHT="$(jq -r --argjson id "$SIMULATOR_WINDOW_ID" '.data.windows[] | select(.windowID == $id) | .bounds[1][1]' "$WINDOW_LIST")"
    peekaboo click --no-remote \
      --coords "$((LAB_WINDOW_X + LAB_WINDOW_WIDTH / 2)),$((LAB_WINDOW_Y + LAB_WINDOW_HEIGHT * 42 / 100))" \
      --no-auto-focus --json >/dev/null
    sleep 2
    LANDSCAPE_KEYBOARD_PATH="$LAB_EVIDENCE_DIR/landscape-keyboard.png"
    lab_capture "$LANDSCAPE_KEYBOARD_PATH" "$LAB_EVIDENCE_DIR/landscape-keyboard.content-probe.json"
    if [[ "$(shasum -a 256 "$LANDSCAPE_KEYBOARD_PATH" | awk '{print $1}')" == "$(shasum -a 256 "$LANDSCAPE_PATH" | awk '{print $1}')" ]]; then
      peekaboo hotkey --no-remote --app Simulator --window-id "$SIMULATOR_WINDOW_ID" --keys 'cmd,k' --json >/dev/null
      KEYBOARD_TOGGLED=true
      sleep 2
      lab_capture "$LANDSCAPE_KEYBOARD_PATH" "$LAB_EVIDENCE_DIR/landscape-keyboard.content-probe.json"
    fi
    [[ "$(shasum -a 256 "$LANDSCAPE_KEYBOARD_PATH" | awk '{print $1}')" != "$(shasum -a 256 "$LANDSCAPE_PATH" | awk '{print $1}')" ]] \
      && LANDSCAPE_KEYBOARD_CAPTURE_READY=true || LANDSCAPE_KEYBOARD_CAPTURE_READY=false
  else
    ROTATION_TRANSITION_READY=false
    LANDSCAPE_KEYBOARD_CAPTURE_READY=false
    LANDSCAPE_KEYBOARD_PATH=""
  fi

  peekaboo menu click --no-remote --app Simulator \
    --path 'Device > Orientation > Portrait' --json >/dev/null
  sleep 3
  if [[ "$KEYBOARD_TOGGLED" == true ]]; then
    peekaboo hotkey --no-remote --app Simulator --window-id "$SIMULATOR_WINDOW_ID" --keys 'cmd,k' --json >/dev/null
    KEYBOARD_TOGGLED=false
  fi

  STARTUP_SAMPLES='[]'
  for ((sample = 1; sample <= LAB_STARTUP_SAMPLES; sample++)); do
    xcrun simctl terminate "$UDID" "$BUNDLE_ID" >/dev/null 2>&1 || true
    START_MS="$(ruby -e 'puts((Process.clock_gettime(Process::CLOCK_MONOTONIC) * 1000).round)')"
    SAMPLE_PID="$(lab_launch)"
    END_MS="$(ruby -e 'puts((Process.clock_gettime(Process::CLOCK_MONOTONIC) * 1000).round)')"
    STARTUP_SAMPLES="$(jq -c --argjson sample "$sample" --argjson pid "$SAMPLE_PID" --argjson elapsed "$((END_MS - START_MS))" \
      '. + [{sample:$sample,pid:$pid,simctl_launch_elapsed_ms:$elapsed}]' <<<"$STARTUP_SAMPLES")"
  done
  sleep 2
  STARTUP_PATH="$LAB_EVIDENCE_DIR/startup-final.png"
  lab_capture "$STARTUP_PATH" "$LAB_EVIDENCE_DIR/startup-final.content-probe.json"
  STARTUP_STATS="$(jq -c '([.[].simctl_launch_elapsed_ms] | sort) as $v | {samples:length,min_ms:$v[0],median_ms:$v[(length/2|floor)],max_ms:$v[-1]}' <<<"$STARTUP_SAMPLES")"
  jq -e --argjson count "$LAB_STARTUP_SAMPLES" 'length == $count and all(.[]; .pid > 0 and .simctl_launch_elapsed_ms >= 0 and .simctl_launch_elapsed_ms <= 5000)' \
    >/dev/null <<<"$STARTUP_SAMPLES" && STARTUP_MODE_READY=true || STARTUP_MODE_READY=false

  RESTORE_COMMAND_READY=true
  RESTORE_READBACK_READY=true
  restore_ios_lab_state || RESTORE_COMMAND_READY=false
  sleep 2
  ios_lab_state_readback_ready || RESTORE_READBACK_READY=false
  if [[ "$RESTORE_COMMAND_READY" != true || "$RESTORE_READBACK_READY" != true ]]; then
    write_ios_cleanup_failure_receipt 1 "$RESTORE_COMMAND_READY" "$RESTORE_READBACK_READY" \
      || echo "error: failed to write iOS cleanup failure receipt" >&2
    echo "error: Simulator extended-lab state restoration failed" >&2
    exit 1
  fi
  RESTORED_CONTENT_SIZE="$(xcrun simctl ui "$UDID" content_size | tail -1 | tr -d '\r')"
  RESTORED_ORIENTATION="$(simulator_orientation)"
  RESTORED_ORIENTATION_PATH="$LAB_EVIDENCE_DIR/state-restored.png"
  lab_capture "$RESTORED_ORIENTATION_PATH" "$LAB_EVIDENCE_DIR/state-restored.content-probe.json"
  RESTORED_WIDTH="$(sips -g pixelWidth "$RESTORED_ORIENTATION_PATH" 2>/dev/null | awk '/pixelWidth:/ {print $2}')"
  RESTORED_HEIGHT="$(sips -g pixelHeight "$RESTORED_ORIENTATION_PATH" 2>/dev/null | awk '/pixelHeight:/ {print $2}')"
  if [[ "$RESTORED_CONTENT_SIZE" != "$ORIGINAL_CONTENT_SIZE" || "$RESTORED_ORIENTATION" != "$ORIGINAL_ORIENTATION" \
    || "$RESTORED_WIDTH" != "$BASELINE_LAB_WIDTH" || "$RESTORED_HEIGHT" != "$BASELINE_LAB_HEIGHT" ]]; then
    write_ios_cleanup_failure_receipt 1 true false || echo "error: failed to write iOS cleanup failure receipt" >&2
    echo "error: Simulator extended-lab state readback did not restore" >&2
    exit 1
  fi
  LAB_ORIENTATION_MUTATED=false
  LAB_STATE_MUTATED=false

  scripts/hepta-ios-login-ui-probe --baseline "$BASELINE_SCREENSHOT" \
    --keyboard "$KEYBOARD_SCREENSHOT" --landscape "$LANDSCAPE_PATH" \
    --device-name "$DEVICE_NAME" --target-x-ratio "$TARGET_X_RATIO" \
    --target-y-ratio "$TARGET_Y_RATIO" --keyboard-trigger-mode "$KEYBOARD_TRIGGER_MODE" \
    --output "$UI_PROBE" >/dev/null

  LAB_RESULT="$(jq -n \
    --arg evidence_root "$LAB_EVIDENCE_DIR" --arg original_size "$ORIGINAL_CONTENT_SIZE" \
    --arg original_orientation "$ORIGINAL_ORIENTATION" --arg restored_orientation "$RESTORED_ORIENTATION" \
    --arg dynamic_size "$DYNAMIC_TYPE_SIZE" --arg rtl_control_path "$RTL_CONTROL_PATH" --arg rtl_control_sha "$RTL_CONTROL_SHA" \
    --arg rtl_path "$RTL_PATH" --arg rtl_sha "$RTL_SHA" \
    --arg dynamic_control_path "$DYNAMIC_CONTROL_PATH" --arg dynamic_control_sha "$DYNAMIC_CONTROL_SHA" \
    --arg dynamic_path "$DYNAMIC_PATH" --arg dynamic_sha "$DYNAMIC_SHA" \
    --arg landscape_path "$LANDSCAPE_PATH" --arg landscape_keyboard_path "$LANDSCAPE_KEYBOARD_PATH" \
    --arg startup_path "$STARTUP_PATH" --argjson rtl_changed "$RTL_RASTER_CHANGED" --argjson rtl_same_canvas "$RTL_SAME_CANVAS" \
    --argjson dynamic_changed "$DYNAMIC_RASTER_CHANGED" --argjson dynamic_same_canvas "$DYNAMIC_SAME_CANVAS" --argjson rotation "$ROTATION_TRANSITION_READY" \
    --argjson landscape_keyboard "$LANDSCAPE_KEYBOARD_CAPTURE_READY" --argjson startup_ready "$STARTUP_MODE_READY" \
    --argjson startup_samples "$STARTUP_SAMPLES" --argjson startup_stats "$STARTUP_STATS" '
      {
        requested:true,status:"not_ready",ready:false,evidence_root:$evidence_root,state_restore_verified:true,
        state_snapshot:{content_size:$original_size,raw_orientation:$original_orientation,restored_orientation:$restored_orientation,exact_orientation_readback:true},
        modes:{
          rtl:{executed:true,environment:{languages:["ar"],locale:"ar_SA",forced_writing_direction:"right_to_left"},matched_control:{path:$rtl_control_path,sha256:$rtl_control_sha,languages:["en"],locale:"en_US",writing_direction:"left_to_right"},capture:{path:$rtl_path,sha256:$rtl_sha},raster_changed:$rtl_changed,mode_attributable_raster_change:$rtl_changed,geometry_comparison:{same_canvas:$rtl_same_canvas,semantic_layout_verified:false},ready:false},
          dynamic_type:{executed:true,requested_content_size:$dynamic_size,original_content_size:$original_size,setting_readback_ready:true,matched_control:{path:$dynamic_control_path,sha256:$dynamic_control_sha,content_size:$original_size},capture:{path:$dynamic_path,sha256:$dynamic_sha},raster_changed:$dynamic_changed,mode_attributable_raster_change:$dynamic_changed,geometry_comparison:{same_canvas:$dynamic_same_canvas,semantic_text_reflow_verified:false},ready:false},
          rotation_keyboard:{executed:true,landscape_transition_observed:$rotation,landscape_capture:$landscape_path,landscape_keyboard_capture:$landscape_keyboard_path,keyboard_raster_change_observed:$landscape_keyboard,ready:false},
          startup_performance:{executed:true,scope:"simctl_launch_command_to_pid_on_unauthenticated_simulator",samples:$startup_samples,statistics:$startup_stats,ready:$startup_ready},
          low_power:{executed:false,supported_by_ios_simulator:false,effective_low_power_mode:false,ready:false}
        },
        claims:{rtl_ready:false,dynamic_type_ready:false,generic_safe_area_ready:false,generic_software_keyboard_ready:false,rotation_ready:false,effective_low_power_performance_ready:false,ios_real_device_ready:false,voiceover_ready:false},
        blockers:[
          {code:"ios_rtl_semantic_layout_verification_missing",requires:"semantic mirrored layout and interaction evidence",observed:{simulator_launch_environment:true,matched_control:true,mode_attributable_raster_change:$rtl_changed,same_canvas:$rtl_same_canvas}},
          {code:"ios_dynamic_type_semantic_response_verification_missing",requires:"accessible text reflow and interaction evidence",observed:{content_size_setting_applied:true,matched_control:true,mode_attributable_raster_change:$dynamic_changed,same_canvas:$dynamic_same_canvas}},
          {code:"ios_generic_safe_area_keyboard_scope_missing",requires:"authenticated app-wide portrait and landscape coverage",observed:{login_landscape_transition:$rotation,login_landscape_keyboard_capture:$landscape_keyboard}},
          {code:"ios_simulator_effective_low_power_mode_unsupported",requires:"real-device effective Low Power Mode",observed:{simulator_support:false}},
          {code:"ios_real_device_receipt_missing",requires:"explicit physical-device lab receipt",observed:false},
          {code:"voiceover_receipt_missing",requires:"real provider/action roundtrip on selected device",observed:false}
        ],
        forbidden_actions_performed:{credential_supply:false,real_device_contact:false,account_connection:false,code_sign:false,upload:false,publish:false},
        external_side_effects_performed:false
      }
    ')"
fi

SOURCE_FINAL="$(scripts/hepta-ui-source-fingerprint)"
jq -e --arg head "$SOURCE_HEAD" --arg tree "$SOURCE_TREE" --arg fingerprint "$SOURCE_FINGERPRINT" '
  .head == $head and .head_tree == $tree and .source_fingerprint == $fingerprint
  and .worktree_clean == true and .repository_worktree_clean == true
' >/dev/null <<<"$SOURCE_FINAL" || { echo "error: source changed during iOS UI qualification" >&2; exit 1; }

IOS_RECEIPT_SHA256="$(shasum -a 256 "$IOS_RECEIPT" | awk '{print $1}')"
BASELINE_SHA256="$(shasum -a 256 "$BASELINE_SCREENSHOT" | awk '{print $1}')"
KEYBOARD_SHA256="$(shasum -a 256 "$KEYBOARD_SCREENSHOT" | awk '{print $1}')"
UI_PROBE_JSON="$(cat "$UI_PROBE")"
BASELINE_HOMESERVER_LOCATOR_JSON="$(cat "$BASELINE_HOMESERVER_LOCATOR")"
UI_SMALL_SCREEN_READY="$(jq -r '.claims.ios_simulator_login_small_screen_ready' "$UI_PROBE")"
UI_REQUIRED_CONTROLS_READY="$(jq -r '.claims.ios_simulator_login_required_controls_visible' "$UI_PROBE")"
UI_COORDINATE_TARGETED_KEYBOARD_READY="$(jq -r '.claims.ios_simulator_login_coordinate_targeted_keyboard_ready' "$UI_PROBE")"
UI_VISIBLE_ANCHOR_SAFE_AREA_READY="$(jq -r '.claims.ios_simulator_login_visible_anchor_safe_area_ready' "$UI_PROBE")"
UI_KEYBOARD_CONTROL_CLEARANCE_READY="$(jq -r '.claims.ios_simulator_login_keyboard_control_clearance_ready' "$UI_PROBE")"
UI_LANDSCAPE_CONTROL_CLEARANCE_READY="$(jq -r '.claims.ios_simulator_login_landscape_control_clearance_ready' "$UI_PROBE")"
REPORT="$(jq -n \
  --arg generated_at_utc "$(date -u +%Y-%m-%dT%H:%M:%SZ)" --arg producer "$PRODUCER" \
  --argjson source_binding "$SOURCE_FINAL" --arg ios_receipt "$IOS_RECEIPT" \
  --arg ios_receipt_sha "$IOS_RECEIPT_SHA256" --arg artifact "$ARTIFACT_PATH" \
  --arg artifact_sha "$ARTIFACT_SHA256" --arg udid "$UDID" --arg device_name "$DEVICE_NAME" \
  --arg runtime "$RUNTIME_IDENTIFIER" --argjson device "$DEVICE_REPORT" \
  --arg launch_output "$LAUNCH_OUTPUT" --argjson launch_pid "$LAUNCH_PID" \
  --argjson window_id "$SIMULATOR_WINDOW_ID" --arg window_bounds "$WINDOW_X,$WINDOW_Y,$WINDOW_WIDTH,$WINDOW_HEIGHT" \
  --argjson click_x "$CLICK_X" --argjson click_y "$CLICK_Y" \
  --argjson target_x_ratio "$TARGET_X_RATIO" --argjson target_y_ratio "$TARGET_Y_RATIO" \
  --arg keyboard_capture_mode "$KEYBOARD_CAPTURE_MODE" \
  --arg baseline "$BASELINE_SCREENSHOT" --arg baseline_sha "$BASELINE_SHA256" \
  --arg keyboard "$KEYBOARD_SCREENSHOT" --arg keyboard_sha "$KEYBOARD_SHA256" \
  --argjson homeserver_locator "$BASELINE_HOMESERVER_LOCATOR_JSON" \
  --argjson ui_probe "$UI_PROBE_JSON" --argjson extended_lab "$LAB_RESULT" \
  --argjson small_screen_ready "$UI_SMALL_SCREEN_READY" \
  --argjson required_controls_ready "$UI_REQUIRED_CONTROLS_READY" \
  --argjson coordinate_targeted_keyboard_ready "$UI_COORDINATE_TARGETED_KEYBOARD_READY" \
  --argjson visible_anchor_safe_area_ready "$UI_VISIBLE_ANCHOR_SAFE_AREA_READY" \
  --argjson keyboard_control_clearance_ready "$UI_KEYBOARD_CONTROL_CLEARANCE_READY" \
  --argjson landscape_control_clearance_ready "$UI_LANDSCAPE_CONTROL_CLEARANCE_READY" '
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
      simulator_window:{app:"Simulator",window_id:$window_id,exact_device_title_match_count:1,bounds:$window_bounds,coordinate_targeting:{ready:$coordinate_targeted_keyboard_ready,requested_target:"baseline_homeserver_text_anchor_center",locator:$homeserver_locator,click_coordinate:{x:$click_x,y:$click_y,normalized_to_device:{x:$target_x_ratio,y_from_top:$target_y_ratio}},keyboard_trigger_mode:$keyboard_capture_mode,keyboard_toggle_fallback_used:false,platform_focus_readback_performed:false,actual_focused_element:null,focus_confirmed:false},display_wake_backend:"/usr/bin/caffeinate"},
      captures:{baseline:{path:$baseline,sha256:$baseline_sha},software_keyboard:{path:$keyboard,sha256:$keyboard_sha,capture_mode:$keyboard_capture_mode},landscape:$ui_probe.captures.landscape},
      ui_probe:$ui_probe,
      extended_lab:$extended_lab,
      claims:{
        ios_simulator_login_software_keyboard_ready:true,
        ios_simulator_login_visible_anchor_safe_area_ready:$visible_anchor_safe_area_ready,
        ios_simulator_login_small_screen_ready:$small_screen_ready,
        ios_simulator_login_required_controls_visible:$required_controls_ready,
        ios_simulator_login_coordinate_targeted_keyboard_ready:$coordinate_targeted_keyboard_ready,
        ios_simulator_login_homeserver_focus_ready:false,
        ios_simulator_login_keyboard_control_clearance_ready:$keyboard_control_clearance_ready,
        ios_simulator_login_landscape_control_clearance_ready:$landscape_control_clearance_ready,
        generic_software_keyboard_ready:false,
        generic_safe_area_ready:false,
        generic_rotation_ready:false,
        ios_real_device_ready:false,
        voiceover_ready:false,
        rtl_ready:false,
        dynamic_type_ready:false,
        ios_low_power_performance_ready:false,
        public_distribution_ready:false
      },
      device_lab:{
        real_device_selected:false,voiceover_session_selected:false,
        blockers:[
          {code:"ios_real_device_receipt_missing",claim:"ios_real_device_ready",observed:false},
          {code:"voiceover_receipt_missing",claim:"voiceover_ready",observed:false}
        ]
      },
      local_simulator_side_effects:{fresh_app_install:true,app_launch:true,coordinate_targeted_click:true,platform_focus_readback:false,software_keyboard_toggle:false,screenshot_capture:true,extended_lab_requested:$extended_lab.requested},
      forbidden_actions_performed:{credential_supply:false,real_device_contact:false,account_connection:false,code_sign:false,upload:false,publish:false},
      external_side_effects_performed:false
    }
  ')"

temporary="$(mktemp "$(dirname "$REPORT_PATH")/.hepta-ios-ui-qualification.XXXXXX")"
printf '%s\n' "$REPORT" >"$temporary"
mv -f -- "$temporary" "$REPORT_PATH"
printf '%s\n' "$REPORT"
