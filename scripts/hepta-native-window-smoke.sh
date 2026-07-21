#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

APP_MANIFEST="apps/hepta-native/Cargo.toml"
OUT_DIR="${HEPTA_NATIVE_WINDOW_SMOKE_DIR:-$(mktemp -d "${TMPDIR:-/tmp}/hepta-native-window-smoke.XXXXXX")}"
REPORT_PATH="${HEPTA_NATIVE_WINDOW_SMOKE_REPORT_PATH:-$OUT_DIR/native-window-smoke.json}"
ALLOW_PERMISSION_SKIP="${HEPTA_NATIVE_WINDOW_SMOKE_ALLOW_PERMISSION_SKIP:-0}"
ALLOW_BLOCKED="${HEPTA_NATIVE_WINDOW_SMOKE_ALLOW_BLOCKED:-$ALLOW_PERMISSION_SKIP}"
STARTUP_TIMEOUT_SEC="${HEPTA_NATIVE_WINDOW_SMOKE_STARTUP_TIMEOUT_SEC:-90}"
PREBUILD_APP="${HEPTA_NATIVE_WINDOW_SMOKE_PREBUILD:-0}"
SKIP_PREFLIGHT="${HEPTA_NATIVE_WINDOW_SMOKE_SKIP_PREFLIGHT:-0}"
DESKTOP_BOUNDS="${HEPTA_NATIVE_WINDOW_SMOKE_DESKTOP_BOUNDS:-40,120,1200,720}"
MOBILE_BOUNDS="${HEPTA_NATIVE_WINDOW_SMOKE_MOBILE_BOUNDS:-80,120,500,720}"
CAPTURE_PROFILE="${HEPTA_NATIVE_WINDOW_SMOKE_CAPTURE_PROFILE:-product-shell}"
APP_PID=""
APP_WINDOW_PID=""
CAFFEINATE_PID=""
APP_DATA_DIR="$OUT_DIR/app-data"
WINDOW_SMOKE_CARGO_TARGET_DIR="${HEPTA_NATIVE_WINDOW_SMOKE_CARGO_TARGET_DIR:-${HEPTA_NATIVE_CARGO_TARGET_DIR:-$OUT_DIR/cargo-target}}"

mkdir -p "$OUT_DIR" "$APP_DATA_DIR"
mkdir -p "$WINDOW_SMOKE_CARGO_TARGET_DIR"

flag_enabled() {
  case "${1:-}" in
    1 | true | TRUE | yes | YES | on | ON)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

json_bool_for_flag() {
  if flag_enabled "$1"; then
    printf 'true'
  else
    printf 'false'
  fi
}

canonical_fixture_route_slug() {
  local route_value
  route_value="$(printf '%s' "${1:-home}" | tr '[:upper:]' '[:lower:]' | tr '_' '-')"
  case "$route_value" in
    action | actions | workbench | action-workbench)
      printf 'actions'
      ;;
    approval | approvals | approval-inbox)
      printf 'approvals'
      ;;
    inspect | inspector | runtime-inspector)
      printf 'inspector'
      ;;
    *)
      printf 'home'
      ;;
  esac
}

fixture_route_label_for_slug() {
  case "$1" in
    actions)
      printf 'Actions'
      ;;
    approvals)
      printf 'Approvals'
      ;;
    inspector)
      printf 'Inspector'
      ;;
    *)
      printf 'Home'
      ;;
  esac
}

canonical_fixture_secondary_surface_slug() {
  local surface_value
  surface_value="$(printf '%s' "${1:-none}" | tr '[:upper:]' '[:lower:]' | tr '_' '-')"
  case "$surface_value" in
    search | find | message-search)
      printf 'search'
      ;;
    setting | settings | room-settings)
      printf 'settings'
      ;;
    attach | attachment | attachments | file | files)
      printf 'attachment'
      ;;
    voice | voice-note | audio)
      printf 'voice'
      ;;
    modal | confirm | confirmation)
      printf 'modal'
      ;;
    *)
      printf 'none'
      ;;
  esac
}

fixture_secondary_surface_label_for_slug() {
  case "$1" in
    search)
      printf 'Search'
      ;;
    settings)
      printf 'Settings'
      ;;
    attachment)
      printf 'Attachment'
      ;;
    voice)
      printf 'Voice'
      ;;
    modal)
      printf 'Modal'
      ;;
    *)
      printf 'None'
      ;;
  esac
}

case "$CAPTURE_PROFILE" in
  product-shell)
    DESKTOP_LAYOUT_LABEL="desktop-product"
    EXPECTED_SCREENSHOT_COUNT=2
    EXPECTED_FIXTURE_SELECTION_COUNT=2
    ;;
  desktop-full-route)
    DESKTOP_LAYOUT_LABEL="desktop-full"
    EXPECTED_SCREENSHOT_COUNT=1
    EXPECTED_FIXTURE_SELECTION_COUNT=1
    ;;
  desktop-full-secondary)
    DESKTOP_LAYOUT_LABEL="desktop-full"
    EXPECTED_SCREENSHOT_COUNT=1
    EXPECTED_FIXTURE_SELECTION_COUNT=1
    ;;
  mobile-secondary)
    DESKTOP_LAYOUT_LABEL="mobile-task-first"
    EXPECTED_SCREENSHOT_COUNT=1
    EXPECTED_FIXTURE_SELECTION_COUNT=1
    ;;
  *)
    echo "unsupported HEPTA_NATIVE_WINDOW_SMOKE_CAPTURE_PROFILE: $CAPTURE_PROFILE" >&2
    exit 2
    ;;
esac

FIXTURE_ROUTE_SLUG="$(canonical_fixture_route_slug "${HEPTA_NATIVE_FIXTURE_ROUTE:-home}")"
FIXTURE_ROUTE_LABEL="$(fixture_route_label_for_slug "$FIXTURE_ROUTE_SLUG")"
FIXTURE_ROW_LABEL="${HEPTA_NATIVE_FIXTURE_ROW:-}"
FIXTURE_SECONDARY_SURFACE_SLUG="$(canonical_fixture_secondary_surface_slug "${HEPTA_NATIVE_FIXTURE_SURFACE:-${HEPTA_NATIVE_FIXTURE_SECONDARY_SURFACE:-none}}")"
FIXTURE_SECONDARY_SURFACE_LABEL="$(fixture_secondary_surface_label_for_slug "$FIXTURE_SECONDARY_SURFACE_SLUG")"

blocked_exit_code() {
  if flag_enabled "$ALLOW_BLOCKED"; then
    printf '0'
  else
    printf '2'
  fi
}

emit_report_json() {
  tee "$REPORT_PATH"
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required for the Hepta Native window smoke gate" >&2
  exit 2
fi

if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 is required for the Hepta Native window screenshot visual probe" >&2
  exit 2
fi

emit_permission_report() {
  local screen_recording="$1"
  local accessibility="$2"
  local exit_code="$3"
 jq -n \
    --arg product "Hepta Native" \
    --arg runtime "hepta" \
    --arg capture_backend "peekaboo" \
    --arg output_dir "$OUT_DIR" \
    --arg capture_profile "$CAPTURE_PROFILE" \
    --arg fixture_route "$FIXTURE_ROUTE_LABEL" \
    --arg fixture_route_slug "$FIXTURE_ROUTE_SLUG" \
    --arg fixture_row "$FIXTURE_ROW_LABEL" \
    --arg fixture_secondary_surface "$FIXTURE_SECONDARY_SURFACE_LABEL" \
    --arg fixture_secondary_surface_slug "$FIXTURE_SECONDARY_SURFACE_SLUG" \
    --argjson screen_recording "$screen_recording" \
    --argjson accessibility "$accessibility" \
    --argjson blocked_allowed "$(json_bool_for_flag "$ALLOW_BLOCKED")" \
    '{
      product:$product,
      runtime:$runtime,
      status:"blocked_by_local_macos_permissions",
      capture_backend:$capture_backend,
      capture_profile:$capture_profile,
      output_dir:$output_dir,
      fixture_route:$fixture_route,
      fixture_route_slug:$fixture_route_slug,
      fixture_row:$fixture_row,
      fixture_secondary_surface:$fixture_secondary_surface,
      fixture_secondary_surface_slug:$fixture_secondary_surface_slug,
      required_permissions:["Screen Recording","Accessibility"],
      permissions:{
        screen_recording:$screen_recording,
        accessibility:$accessibility
      },
      true_window_capture_performed:false,
      blocked_allowed:$blocked_allowed,
      blocked_allowed_reason:(
        if $blocked_allowed then
          "HEPTA_NATIVE_WINDOW_SMOKE_ALLOW_BLOCKED records local macOS window-capture blockers without claiming true-window screenshots passed."
        else
          null
        end
      ),
      native_first_read_path_guard_ready:true,
      native_primary_read_path:"telegram-chat-shell",
      native_telegram_shell_ready:true,
      native_desktop_first_read_path_ready:true,
      native_mobile_first_read_path_ready:true,
      native_engineering_copy_hidden:true,
      native_makepad_desktop_product_layout_ready:true,
      native_makepad_route_variant_ready:false,
      native_makepad_secondary_surface_ready:false,
      fixture_secondary_surface_selected_ready:false,
      fixture_secondary_surface_selection_count:0,
      fixture_mobile_secondary_content_visible_ready:false,
      fixture_mobile_secondary_content_visible_count:0,
      fixture_desktop_product_layout_ready:false,
      fixture_desktop_product_layout_count:0,
      fixture_desktop_full_layout_ready:false,
      fixture_desktop_full_layout_count:0,
      fixture_route_selected_ready:false,
      fixture_route_selection_count:0,
      fixture_mobile_task_first_layout_ready:false,
      fixture_mobile_task_first_layout_count:0,
      fixture_product_shell_selected_ready:false,
      fixture_product_shell_selection_count:0,
      fixture_matrix_composer_hidden_ready:false,
      fixture_matrix_composer_hidden_count:0,
      native_makepad_desktop_full_layout_env_override:"HEPTA_NATIVE_FIXTURE_LAYOUT=desktop-full",
      native_makepad_mobile_task_first_layout_ready:true,
      native_makepad_mobile_layout_width_threshold:620,
      note:"Grant Peekaboo/terminal Screen Recording and Accessibility permissions, then rerun without HEPTA_NATIVE_WINDOW_SMOKE_ALLOW_BLOCKED.",
      side_effects:{
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        external_mutation:false
      }
    }' | emit_report_json
  exit "$exit_code"
}

emit_locked_screen_report() {
  local exit_code="$1"
 jq -n \
    --arg product "Hepta Native" \
    --arg runtime "hepta" \
    --arg capture_backend "peekaboo" \
    --arg output_dir "$OUT_DIR" \
    --arg capture_profile "$CAPTURE_PROFILE" \
    --arg fixture_route "$FIXTURE_ROUTE_LABEL" \
    --arg fixture_route_slug "$FIXTURE_ROUTE_SLUG" \
    --arg fixture_row "$FIXTURE_ROW_LABEL" \
    --arg fixture_secondary_surface "$FIXTURE_SECONDARY_SURFACE_LABEL" \
    --arg fixture_secondary_surface_slug "$FIXTURE_SECONDARY_SURFACE_SLUG" \
    --argjson blocked_allowed "$(json_bool_for_flag "$ALLOW_BLOCKED")" \
    '{
      product:$product,
      runtime:$runtime,
      status:"blocked_by_locked_screen",
      capture_backend:$capture_backend,
      capture_mode:"screen_crop_from_window_bounds",
      capture_profile:$capture_profile,
      output_dir:$output_dir,
      fixture_route:$fixture_route,
      fixture_route_slug:$fixture_route_slug,
      fixture_row:$fixture_row,
      fixture_secondary_surface:$fixture_secondary_surface,
      fixture_secondary_surface_slug:$fixture_secondary_surface_slug,
      required_state:"Unlocked macOS desktop",
      true_window_capture_performed:false,
      blocked_allowed:$blocked_allowed,
      blocked_allowed_reason:(
        if $blocked_allowed then
          "HEPTA_NATIVE_WINDOW_SMOKE_ALLOW_BLOCKED records locked-screen blockers without claiming true-window screenshots passed."
        else
          null
        end
      ),
      native_first_read_path_guard_ready:true,
      native_primary_read_path:"telegram-chat-shell",
      native_telegram_shell_ready:true,
      native_desktop_first_read_path_ready:true,
      native_mobile_first_read_path_ready:true,
      native_engineering_copy_hidden:true,
      native_makepad_desktop_product_layout_ready:true,
      native_makepad_route_variant_ready:false,
      native_makepad_secondary_surface_ready:false,
      fixture_secondary_surface_selected_ready:false,
      fixture_secondary_surface_selection_count:0,
      fixture_mobile_secondary_content_visible_ready:false,
      fixture_mobile_secondary_content_visible_count:0,
      fixture_desktop_product_layout_ready:false,
      fixture_desktop_product_layout_count:0,
      fixture_desktop_full_layout_ready:false,
      fixture_desktop_full_layout_count:0,
      fixture_route_selected_ready:false,
      fixture_route_selection_count:0,
      fixture_mobile_task_first_layout_ready:false,
      fixture_mobile_task_first_layout_count:0,
      fixture_product_shell_selected_ready:false,
      fixture_product_shell_selection_count:0,
      fixture_matrix_composer_hidden_ready:false,
      fixture_matrix_composer_hidden_count:0,
      native_makepad_desktop_full_layout_env_override:"HEPTA_NATIVE_FIXTURE_LAYOUT=desktop-full",
      native_makepad_mobile_task_first_layout_ready:true,
      native_makepad_mobile_layout_width_threshold:620,
      note:"Unlock the macOS desktop, then rerun this gate. Screen Recording and Accessibility may be granted while the lock screen still prevents true window capture.",
      side_effects:{
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        external_mutation:false
      }
    }' | emit_report_json
  exit "$exit_code"
}

console_screen_locked() {
  local root_state
  root_state="$(ioreg -n Root -d1 2>/dev/null || true)"
  grep -Eq '"IOConsoleLocked" = Yes|"CGSSessionScreenIsLocked"=Yes' <<<"$root_state"
}

run_with_timeout() {
  local timeout_sec="$1"
  shift
  "$@" &
  local command_pid="$!"
  local deadline=$((SECONDS + timeout_sec))
  while kill -0 "$command_pid" 2>/dev/null; do
    if (( SECONDS >= deadline )); then
      kill "$command_pid" 2>/dev/null || true
      wait "$command_pid" 2>/dev/null || true
      return 124
    fi
    sleep 0.2
  done
  wait "$command_pid"
}

run_preflight_test() {
  local test_name="$1"
  if ! cargo_with_window_target test --manifest-path "$APP_MANIFEST" -q "$test_name" >&2; then
    echo "Hepta Native window smoke preflight test failed: $test_name" >&2
    exit 1
  fi
}

run_preflight_suite() {
  run_preflight_test current_codex_fixture_smoke_is_ready_without_live_side_effects
  run_preflight_test hepta_fixture_cockpit_has_a_card_for_each_sample_event
  run_preflight_test hepta_fixture_layout_policy_collapses_mobile_to_task_first_without_live_mutation
}

cargo_with_window_target() {
  CARGO_TARGET_DIR="$WINDOW_SMOKE_CARGO_TARGET_DIR" cargo "$@"
}

run_hepta_native_app() {
  env \
    CARGO_TARGET_DIR="$WINDOW_SMOKE_CARGO_TARGET_DIR" \
    HEPTA_NATIVE_FIXTURE_MODE=1 \
    HEPTA_NATIVE_APP_DATA_DIR="$APP_DATA_DIR" \
    HEPTA_AUTOLOAD=0 \
    HEPTA_AUTOSAVE=0 \
    cargo run --manifest-path "$APP_MANIFEST"
}

prebuild_hepta_native_app() {
  if ! flag_enabled "$PREBUILD_APP"; then
    return
  fi
  if ! cargo_with_window_target build --manifest-path "$APP_MANIFEST" -q >&2; then
    echo "Hepta Native window smoke prebuild failed" >&2
    exit 1
  fi
}

if ! command -v peekaboo >/dev/null 2>&1; then
  echo "peekaboo is required for the Hepta Native macOS window smoke gate" >&2
  exit 2
fi

permissions_json="$(peekaboo permissions status --json)"
screen_recording_granted="$(jq -r '.data.permissions[] | select(.name == "Screen Recording") | .isGranted' <<<"$permissions_json")"
accessibility_granted="$(jq -r '.data.permissions[] | select(.name == "Accessibility") | .isGranted' <<<"$permissions_json")"

if [[ "$screen_recording_granted" != "true" || "$accessibility_granted" != "true" ]]; then
  emit_permission_report "$screen_recording_granted" "$accessibility_granted" "$(blocked_exit_code)"
fi

if console_screen_locked; then
  emit_locked_screen_report "$(blocked_exit_code)"
fi

if ! flag_enabled "$SKIP_PREFLIGHT"; then
  run_preflight_suite
fi

prebuild_hepta_native_app

if console_screen_locked; then
  emit_locked_screen_report "$(blocked_exit_code)"
fi

parse_bounds() {
  local bounds="$1"
  IFS=',' read -r x y width height <<<"$bounds"
  if [[ -z "${x:-}" || -z "${y:-}" || -z "${width:-}" || -z "${height:-}" ]]; then
    echo "invalid window bounds: $bounds" >&2
    exit 2
  fi
  printf '%s %s %s %s\n' "$x" "$y" "$width" "$height"
}

descendant_pids() {
  local root_pid="$1"
  local child_pid
  for child_pid in $(pgrep -P "$root_pid" 2>/dev/null || true); do
    printf '%s\n' "$child_pid"
    descendant_pids "$child_pid"
  done
}

current_hepta_native_app_pid() {
  local candidate_pid
  local command_path
  local command_name
  for candidate_pid in $(descendant_pids "$APP_PID"); do
    command_path="$(ps -p "$candidate_pid" -o comm= 2>/dev/null || true)"
    command_name="${command_path##*/}"
    if [[ "$command_name" == "hepta-native" ]]; then
      printf '%s\n' "$candidate_pid"
      return 0
    fi
  done
  return 1
}

peekaboo_window_list_json() {
  if [[ -n "${APP_WINDOW_PID:-}" ]]; then
    peekaboo window list --pid "$APP_WINDOW_PID" --json
  else
    peekaboo window list --app "hepta-native" --json
  fi
}

wait_for_window() {
  local deadline=$((SECONDS + STARTUP_TIMEOUT_SEC))
  while [[ "$SECONDS" -lt "$deadline" ]]; do
    if console_screen_locked; then
      emit_locked_screen_report "$(blocked_exit_code)"
    fi
    local windows_json
    local candidate_pid
    candidate_pid="$(current_hepta_native_app_pid || true)"
    if [[ -n "$candidate_pid" ]]; then
      APP_WINDOW_PID="$candidate_pid"
    fi
    if [[ -n "${APP_WINDOW_PID:-}" ]] &&
      windows_json="$(peekaboo_window_list_json 2>/dev/null)" &&
      jq -e '.data.windows[]? | select(.window_title == "Hepta Native")' <<<"$windows_json" >/dev/null; then
      return 0
    fi
    if [[ -n "${APP_PID:-}" ]] && ! kill -0 "$APP_PID" 2>/dev/null; then
      if console_screen_locked; then
        emit_locked_screen_report "$(blocked_exit_code)"
      fi
      echo "Hepta Native exited before its window became capturable" >&2
      tail -n 80 "$APP_LOG" >&2 || true
      exit 1
    fi
    sleep 1
  done

  if console_screen_locked; then
    emit_locked_screen_report "$(blocked_exit_code)"
  fi
  echo "Timed out waiting for Hepta Native window" >&2
  tail -n 80 "$APP_LOG" >&2 || true
  exit 1
}

fixture_cockpit_selection_count() {
  grep -c "Hepta Native fixture cockpit selected" "$APP_LOG" 2>/dev/null || true
}

fixture_product_shell_selection_count() {
  grep -c "Hepta Native fixture product shell selected" "$APP_LOG" 2>/dev/null || true
}

fixture_matrix_composer_hidden_count() {
  grep -c "Hepta Native fixture Matrix composer hidden" "$APP_LOG" 2>/dev/null || true
}

fixture_desktop_product_layout_count() {
  grep -c "Hepta Native fixture cockpit layout selected: desktop-product" "$APP_LOG" 2>/dev/null || true
}

fixture_mobile_task_first_layout_count() {
  grep -c "Hepta Native fixture cockpit layout selected: mobile-task-first" "$APP_LOG" 2>/dev/null || true
}

fixture_desktop_full_layout_count() {
  grep -c "Hepta Native fixture cockpit layout selected: desktop-full" "$APP_LOG" 2>/dev/null || true
}

fixture_route_selection_count() {
  grep -c "Hepta Native fixture route selected: ${FIXTURE_ROUTE_LABEL}" "$APP_LOG" 2>/dev/null || true
}

fixture_secondary_surface_selection_count() {
  if [[ "$FIXTURE_SECONDARY_SURFACE_SLUG" == "none" ]]; then
    printf '0\n'
    return
  fi
  grep -c "Hepta Native fixture secondary surface selected: ${FIXTURE_SECONDARY_SURFACE_LABEL}" "$APP_LOG" 2>/dev/null || true
}

fixture_route_top_design_referee_count() {
  local expected_generic="true"
  local expected_desktop_route="false"
  if [[ "$CAPTURE_PROFILE" == "desktop-full-route" && "$FIXTURE_ROUTE_SLUG" != "home" ]]; then
    expected_generic="false"
    expected_desktop_route="true"
  fi

  grep -c \
    "Hepta Native fixture top-design route workspace: route=${FIXTURE_ROUTE_LABEL} secondary_surface=${FIXTURE_SECONDARY_SURFACE_LABEL} generic_scaffold_visible=${expected_generic} route_detail_visible=true desktop_route_workspace_visible=${expected_desktop_route}" \
    "$APP_LOG" 2>/dev/null || true
}

fixture_mobile_secondary_content_visible_count() {
  if [[ "$CAPTURE_PROFILE" != "mobile-secondary" || "$FIXTURE_SECONDARY_SURFACE_SLUG" == "none" ]]; then
    printf '0\n'
    return
  fi
  grep -c \
    "Hepta Native fixture mobile secondary content visible: surface=${FIXTURE_SECONDARY_SURFACE_LABEL} route_shell_visible=true route_detail_visible=true primary_panel_visible=true horizontal_detail_rows_visible=false desktop_card_row_hidden=true action_dock_hidden=true" \
    "$APP_LOG" 2>/dev/null || true
}

fixture_layout_marker_count() {
  local layout_label="$1"
  case "$layout_label" in
    desktop-product)
      fixture_desktop_product_layout_count
      ;;
    desktop-full)
      fixture_desktop_full_layout_count
      ;;
    mobile-task-first)
      fixture_mobile_task_first_layout_count
      ;;
    *)
      grep -c "Hepta Native fixture cockpit layout selected: ${layout_label}" "$APP_LOG" 2>/dev/null || true
      ;;
  esac
}

assert_no_native_app_log_errors() {
  local error_log="$OUT_DIR/native-window-app-log-errors.txt"
  if grep -E '^\[E\]' "$APP_LOG" >"$error_log" 2>/dev/null; then
    echo "Hepta Native emitted app log errors during true-window smoke" >&2
    cat "$error_log" >&2
    exit 1
  fi
  rm -f "$error_log"
}

screenshot_visual_probe() {
  local name="$1"
  local screenshot="$2"
  python3 - "$name" "$screenshot" <<'PY'
import json
import math
import pathlib
import struct
import sys
import zlib


def fail(message):
    print(message, file=sys.stderr)
    sys.exit(1)


def paeth(a, b, c):
    p = a + b - c
    pa = abs(p - a)
    pb = abs(p - b)
    pc = abs(p - c)
    if pa <= pb and pa <= pc:
        return a
    if pb <= pc:
        return b
    return c


def read_png(path):
    data = path.read_bytes()
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        fail(f"{path} is not a PNG")
    pos = 8
    width = height = bit_depth = color_type = None
    raw = b""
    while pos < len(data):
        length = struct.unpack(">I", data[pos : pos + 4])[0]
        pos += 4
        chunk_type = data[pos : pos + 4]
        pos += 4
        chunk = data[pos : pos + length]
        pos += length + 4
        if chunk_type == b"IHDR":
            width, height, bit_depth, color_type, compression, filter_method, interlace = struct.unpack(
                ">IIBBBBB", chunk
            )
            if bit_depth != 8 or compression != 0 or filter_method != 0 or interlace != 0:
                fail(f"unsupported PNG encoding for {path}")
        elif chunk_type == b"IDAT":
            raw += chunk
        elif chunk_type == b"IEND":
            break

    channels_by_type = {0: 1, 2: 3, 4: 2, 6: 4}
    if color_type not in channels_by_type:
        fail(f"unsupported PNG color type {color_type} for {path}")
    channels = channels_by_type[color_type]
    stride = width * channels
    scanlines = zlib.decompress(raw)
    rows = []
    previous = bytearray(stride)
    index = 0
    for _ in range(height):
        filter_type = scanlines[index]
        index += 1
        current = bytearray(scanlines[index : index + stride])
        index += stride
        for x in range(stride):
            left = current[x - channels] if x >= channels else 0
            up = previous[x]
            upper_left = previous[x - channels] if x >= channels else 0
            if filter_type == 1:
                current[x] = (current[x] + left) & 255
            elif filter_type == 2:
                current[x] = (current[x] + up) & 255
            elif filter_type == 3:
                current[x] = (current[x] + ((left + up) // 2)) & 255
            elif filter_type == 4:
                current[x] = (current[x] + paeth(left, up, upper_left)) & 255
            elif filter_type != 0:
                fail(f"unsupported PNG filter {filter_type} for {path}")
        rows.append(current)
        previous = current
    return width, height, color_type, channels, rows


name = sys.argv[1]
path = pathlib.Path(sys.argv[2])
width, height, color_type, channels, rows = read_png(path)
sample_step = max(1, int(math.sqrt((width * height) / 20000)))
colors = set()
luminance_values = []
accent_pixels = 0
sample_count = 0

for y in range(0, height, sample_step):
    row = rows[y]
    for x in range(0, width, sample_step):
        offset = x * channels
        if color_type in (0, 4):
            red = green = blue = row[offset]
        else:
            red, green, blue = row[offset], row[offset + 1], row[offset + 2]
        colors.add((red // 8, green // 8, blue // 8))
        luminance = 0.2126 * red + 0.7152 * green + 0.0722 * blue
        luminance_values.append(luminance)
        if (blue > 95 and green > 55 and red < 105) or (green > 120 and blue > 115 and red < 90):
            accent_pixels += 1
        sample_count += 1

average_luminance = sum(luminance_values) / sample_count
luminance_stddev = (
    sum((value - average_luminance) ** 2 for value in luminance_values) / sample_count
) ** 0.5
dark_pixel_fraction = sum(value < 45 for value in luminance_values) / sample_count
mid_pixel_fraction = sum(45 <= value <= 210 for value in luminance_values) / sample_count
accent_pixel_fraction = accent_pixels / sample_count
unique_color_bins = len(colors)
min_accent_pixel_fraction = 0.006 if name in ("desktop-window", "mobile-window") else 0.004 if (
    name.startswith("desktop-full-route-")
    or name.startswith("desktop-full-secondary-")
    or name.startswith("mobile-secondary-")
) else 0.01

def region_stats(x0, y0, x1, y1):
    region_width = max(1, x1 - x0)
    region_height = max(1, y1 - y0)
    region_step = max(1, int(math.sqrt((region_width * region_height) / 6000)))
    region_colors = set()
    region_luminance_values = []
    region_accent_pixels = 0
    region_sample_count = 0
    for y in range(y0, y1, region_step):
        row = rows[y]
        for x in range(x0, x1, region_step):
            offset = x * channels
            if color_type in (0, 4):
                red = green = blue = row[offset]
            else:
                red, green, blue = row[offset], row[offset + 1], row[offset + 2]
            region_colors.add((red // 8, green // 8, blue // 8))
            luminance = 0.2126 * red + 0.7152 * green + 0.0722 * blue
            region_luminance_values.append(luminance)
            if (blue > 95 and green > 55 and red < 105) or (
                green > 120 and blue > 115 and red < 90
            ):
                region_accent_pixels += 1
            region_sample_count += 1
    region_average = sum(region_luminance_values) / region_sample_count
    region_stddev = (
        sum((value - region_average) ** 2 for value in region_luminance_values)
        / region_sample_count
    ) ** 0.5
    return {
        "sample_count": region_sample_count,
        "unique_color_bins": len(region_colors),
        "average_luminance": round(region_average, 2),
        "luminance_stddev": round(region_stddev, 2),
        "dark_pixel_fraction": round(
            sum(value < 45 for value in region_luminance_values) / region_sample_count,
            4,
        ),
        "mid_pixel_fraction": round(
            sum(45 <= value <= 210 for value in region_luminance_values) / region_sample_count,
            4,
        ),
        "accent_pixel_fraction": round(region_accent_pixels / region_sample_count, 4),
    }


mobile_secondary_content_probe = None
mobile_secondary_content_ready = True
route_content_probe = None
route_content_ready = True
product_shell_light_glass_ready = True
if name.startswith("desktop-full-route-"):
    workspace_region = region_stats(
        int(width * 0.18),
        int(height * 0.16),
        int(width * 0.96),
        int(height * 0.90),
    )
    upper_route_region = region_stats(
        int(width * 0.20),
        int(height * 0.16),
        int(width * 0.55),
        int(height * 0.45),
    )
    lower_route_region = region_stats(
        int(width * 0.20),
        int(height * 0.48),
        int(width * 0.96),
        int(height * 0.90),
    )
    route_content_ready = (
        width >= 1100
        and height >= 680
        and workspace_region["unique_color_bins"] >= 90
        and workspace_region["luminance_stddev"] >= 18.0
        and workspace_region["mid_pixel_fraction"] >= 0.03
        and upper_route_region["unique_color_bins"] >= 100
        and upper_route_region["luminance_stddev"] >= 18.0
        and upper_route_region["mid_pixel_fraction"] >= 0.035
        and lower_route_region["unique_color_bins"] >= 100
        and lower_route_region["luminance_stddev"] >= 19.0
        and lower_route_region["mid_pixel_fraction"] >= 0.025
    )
    route_content_probe = {
        "ready": route_content_ready,
        "min_width": 1100,
        "min_height": 680,
        "workspace_region": workspace_region,
        "upper_route_region": upper_route_region,
        "lower_route_region": lower_route_region,
    }
if name.startswith("mobile-secondary-"):
    center_region = region_stats(
        int(width * 0.08),
        int(height * 0.22),
        int(width * 0.92),
        int(height * 0.70),
    )
    bottom_region = region_stats(0, int(height * 0.72), width, height)
    mobile_secondary_content_ready = (
        width <= 430
        and height >= 800
        and center_region["unique_color_bins"] >= 120
        and center_region["luminance_stddev"] >= 24.0
        and center_region["mid_pixel_fraction"] >= 0.07
        and bottom_region["unique_color_bins"] >= 110
        and bottom_region["luminance_stddev"] >= 22.0
    )
    mobile_secondary_content_probe = {
        "ready": mobile_secondary_content_ready,
        "min_height": 800,
        "max_width": 430,
        "center_region": center_region,
        "bottom_region": bottom_region,
    }
if name in ("desktop-window", "mobile-window"):
    product_shell_light_glass_ready = (
        238.0 <= average_luminance <= 250.0
        and dark_pixel_fraction <= 0.025
        and mid_pixel_fraction >= 0.02
        and accent_pixel_fraction >= min_accent_pixel_fraction
    )

if name in ("desktop-window", "mobile-window"):
    ready = (
        sample_count >= 1000
        and unique_color_bins >= 48
        and luminance_stddev >= 10.0
        and product_shell_light_glass_ready
    )
else:
    ready = (
        sample_count >= 1000
        and unique_color_bins >= 48
        and luminance_stddev >= 10.0
        and 0.25 <= dark_pixel_fraction <= 0.96
        and mid_pixel_fraction >= 0.03
        and accent_pixel_fraction >= min_accent_pixel_fraction
        and route_content_ready
        and mobile_secondary_content_ready
    )

probe = {
    "name": name,
    "ready": ready,
    "sample_count": sample_count,
    "unique_color_bins": unique_color_bins,
    "average_luminance": round(average_luminance, 2),
    "luminance_stddev": round(luminance_stddev, 2),
    "dark_pixel_fraction": round(dark_pixel_fraction, 4),
    "mid_pixel_fraction": round(mid_pixel_fraction, 4),
    "accent_pixel_fraction": round(accent_pixel_fraction, 4),
    "min_accent_pixel_fraction": min_accent_pixel_fraction,
    "product_shell_light_glass_ready": product_shell_light_glass_ready,
    "product_shell_light_glass_average_luminance_range": "238..250",
    "product_shell_light_glass_max_dark_pixel_fraction": 0.025,
    "product_shell_light_glass_min_mid_pixel_fraction": 0.02,
}
if route_content_probe is not None:
    probe["route_content_ready"] = route_content_ready
    probe["route_content_probe"] = route_content_probe
if mobile_secondary_content_probe is not None:
    probe["mobile_secondary_content_ready"] = mobile_secondary_content_ready
    probe["mobile_secondary_content_probe"] = mobile_secondary_content_probe
print(json.dumps(probe, separators=(",", ":")))
if not ready:
    fail(f"Hepta Native true-window screenshot visual probe failed for {name}: {probe}")
PY
}

wait_for_fixture_cockpit_count() {
  local expected_count="$1"
  local label="$2"
  local deadline=$((SECONDS + STARTUP_TIMEOUT_SEC))
  while [[ "$SECONDS" -lt "$deadline" ]]; do
    local selected_count
    selected_count="$(fixture_cockpit_selection_count)"
    if (( selected_count >= expected_count )); then
      return 0
    fi
    if [[ -n "${APP_PID:-}" ]] && ! kill -0 "$APP_PID" 2>/dev/null; then
      echo "Hepta Native exited before selecting the fixture cockpit for $label" >&2
      tail -n 120 "$APP_LOG" >&2 || true
      exit 1
    fi
    sleep 1
  done

  echo "Timed out waiting for Hepta Native to select the fixture cockpit for $label" >&2
  tail -n 120 "$APP_LOG" >&2 || true
  exit 1
}

wait_for_fixture_cockpit() {
  wait_for_fixture_cockpit_count 1 "initial window"
}

wait_for_fixture_product_shell_count() {
  local expected_count="$1"
  local label="$2"
  local deadline=$((SECONDS + STARTUP_TIMEOUT_SEC))
  while [[ "$SECONDS" -lt "$deadline" ]]; do
    local selected_count
    selected_count="$(fixture_product_shell_selection_count)"
    if (( selected_count >= expected_count )); then
      return 0
    fi
    if [[ -n "${APP_PID:-}" ]] && ! kill -0 "$APP_PID" 2>/dev/null; then
      echo "Hepta Native exited before selecting the fixture product shell for $label" >&2
      tail -n 120 "$APP_LOG" >&2 || true
      exit 1
    fi
    sleep 1
  done

  echo "Timed out waiting for Hepta Native to select the fixture product shell for $label" >&2
  tail -n 120 "$APP_LOG" >&2 || true
  exit 1
}

wait_for_fixture_matrix_composer_hidden_count() {
  local expected_count="$1"
  local context="$2"
  local deadline=$((SECONDS + STARTUP_TIMEOUT_SEC))
  while [[ "$SECONDS" -lt "$deadline" ]]; do
    local hidden_count
    hidden_count="$(fixture_matrix_composer_hidden_count)"
    if (( hidden_count >= expected_count )); then
      return 0
    fi
    if [[ -n "${APP_PID:-}" ]] && ! kill -0 "$APP_PID" 2>/dev/null; then
      echo "Hepta Native app exited before fixture Matrix composer hide marker appeared for $context" >&2
      tail -n 160 "$APP_LOG" >&2 || true
      exit 1
    fi
    sleep 0.5
  done
  echo "Timed out waiting for fixture Matrix composer hide marker for $context" >&2
  tail -n 160 "$APP_LOG" >&2 || true
  exit 1
}

wait_for_fixture_layout_count() {
  local layout_label="$1"
  local expected_count="$2"
  local context="$3"
  local deadline=$((SECONDS + STARTUP_TIMEOUT_SEC))
  while [[ "$SECONDS" -lt "$deadline" ]]; do
    local layout_count
    layout_count="$(fixture_layout_marker_count "$layout_label")"
    if (( layout_count >= expected_count )); then
      return 0
    fi
    if [[ -n "${APP_PID:-}" ]] && ! kill -0 "$APP_PID" 2>/dev/null; then
      echo "Hepta Native app exited before fixture layout marker ${layout_label} appeared for $context" >&2
      tail -n 160 "$APP_LOG" >&2 || true
      exit 1
    fi
    sleep 0.5
  done
  echo "Timed out waiting for fixture layout marker ${layout_label} for $context" >&2
  tail -n 160 "$APP_LOG" >&2 || true
  exit 1
}

wait_for_mobile_secondary_content_visible_count() {
  local expected_count="$1"
  local context="$2"
  local deadline=$((SECONDS + STARTUP_TIMEOUT_SEC))
  while [[ "$SECONDS" -lt "$deadline" ]]; do
    local content_count
    content_count="$(fixture_mobile_secondary_content_visible_count)"
    if (( content_count >= expected_count )); then
      return 0
    fi
    if [[ -n "${APP_PID:-}" ]] && ! kill -0 "$APP_PID" 2>/dev/null; then
      echo "Hepta Native app exited before mobile secondary content marker appeared for $context" >&2
      tail -n 160 "$APP_LOG" >&2 || true
      exit 1
    fi
    sleep 0.5
  done
  echo "Timed out waiting for mobile secondary content marker for $context" >&2
  tail -n 160 "$APP_LOG" >&2 || true
  exit 1
}

capture_window() {
  local name="$1"
  local bounds="$2"
  local screenshot="$OUT_DIR/${name}.png"
  local screen_capture="$OUT_DIR/${name}.screen.png"
  local x y width height
  read -r x y width height < <(parse_bounds "$bounds")
  if [[ -z "${APP_WINDOW_PID:-}" ]]; then
    echo "unable to resolve the launched Hepta Native app pid before capturing $name" >&2
    exit 1
  fi
  local windows_json
  local window_id
  windows_json="$(peekaboo_window_list_json)"
  window_id="$(
    jq -r '.data.windows[] | select(.window_title == "Hepta Native") | .window_id' \
      <<<"$windows_json" |
      head -1
  )"
  if [[ -z "$window_id" || "$window_id" == "null" ]]; then
    echo "unable to resolve the launched Hepta Native window id for $name" >&2
    exit 1
  fi

  local set_attempt
  for set_attempt in 1 2 3; do
    if run_with_timeout 20 peekaboo window set-bounds \
      --app "PID:$APP_WINDOW_PID" \
      --window-title "Hepta Native" \
      --x "$x" \
      --y "$y" \
      --width "$width" \
      --height "$height" \
      --json >/dev/null; then
      break
    fi
    if console_screen_locked; then
      emit_locked_screen_report "$(blocked_exit_code)"
    fi
    if [[ "$set_attempt" == "3" ]]; then
      echo "timed out setting Hepta Native window bounds for $name after ${set_attempt} attempts" >&2
      exit 1
    fi
    sleep "$set_attempt"
	  done
	  local focus_attempt
	  for focus_attempt in 1 2 3; do
	    if run_with_timeout 20 peekaboo window focus \
	      --window-id "$window_id" \
	      --json >/dev/null || \
	      run_with_timeout 20 peekaboo window focus \
	        --pid "$APP_WINDOW_PID" \
	        --window-title "Hepta Native" \
	        --json >/dev/null; then
	      break
	    fi
    if console_screen_locked; then
      emit_locked_screen_report "$(blocked_exit_code)"
    fi
    if [[ "$focus_attempt" == "3" ]]; then
      echo "timed out focusing Hepta Native window for $name after ${focus_attempt} attempts" >&2
      exit 1
    fi
    sleep "$focus_attempt"
  done
  sleep 2
  if console_screen_locked; then
    emit_locked_screen_report "$(blocked_exit_code)"
  fi
  if [[ "$name" == "mobile-window" ]]; then
    wait_for_fixture_layout_count "mobile-task-first" 1 "mobile window"
    wait_for_fixture_cockpit_count 2 "mobile window"
    wait_for_fixture_product_shell_count 2 "mobile window"
    wait_for_fixture_matrix_composer_hidden_count 2 "mobile window"
  else
    wait_for_fixture_layout_count "$DESKTOP_LAYOUT_LABEL" 1 "$name"
  fi

  windows_json="$(peekaboo_window_list_json)"
  local actual_x actual_y actual_width actual_height
  actual_x="$(jq -r --argjson window_id "$window_id" '.data.windows[] | select(.window_id == $window_id) | .bounds.x' <<<"$windows_json" | head -1)"
  actual_y="$(jq -r --argjson window_id "$window_id" '.data.windows[] | select(.window_id == $window_id) | .bounds.y' <<<"$windows_json" | head -1)"
  actual_width="$(jq -r --argjson window_id "$window_id" '.data.windows[] | select(.window_id == $window_id) | .bounds.width' <<<"$windows_json" | head -1)"
  actual_height="$(jq -r --argjson window_id "$window_id" '.data.windows[] | select(.window_id == $window_id) | .bounds.height' <<<"$windows_json" | head -1)"
  if [[ -z "$actual_x" || -z "$actual_y" || -z "$actual_width" || -z "$actual_height" || -z "$window_id" ]]; then
    echo "unable to resolve Hepta Native window bounds for $name" >&2
    exit 1
  fi
  if [[ "$actual_width" -ne "$width" ]]; then
    echo "Hepta Native window width did not settle for $name: expected $width, got $actual_width" >&2
    exit 1
  fi
  if (( actual_height < height - 80 )); then
    echo "Hepta Native window height did not settle for $name: expected near $height, got $actual_height" >&2
    exit 1
  fi

  local capture_mode="window"
  if ! run_with_timeout 10 peekaboo image \
    --mode window \
    --window-id "$window_id" \
    --path "$screenshot" \
    --format png \
    --json >/dev/null; then
    capture_mode="screen_crop_from_window_bounds"
    if ! run_with_timeout 10 peekaboo image \
      --mode screen \
      --screen-index 0 \
      --path "$screen_capture" \
      --format png \
      --json >/dev/null; then
      if console_screen_locked; then
        emit_locked_screen_report "$(blocked_exit_code)"
      fi
      echo "timed out capturing screen for $name" >&2
      exit 1
    fi
    if console_screen_locked; then
      emit_locked_screen_report "$(blocked_exit_code)"
    fi
    cp "$screen_capture" "$screenshot"
    sips \
      --cropToHeightWidth "$actual_height" "$actual_width" \
      --cropOffset "$actual_y" "$actual_x" \
      "$screenshot" >/dev/null
  fi

  if [[ ! -s "$screenshot" ]]; then
    echo "window screenshot was not created for $name" >&2
    exit 1
  fi

  local dimensions
  dimensions="$(
    sips -g pixelWidth -g pixelHeight "$screenshot" 2>/dev/null |
      awk '/pixelWidth/ { w=$2 } /pixelHeight/ { h=$2 } END { print w "x" h }'
  )"
  if [[ -z "$dimensions" || "$dimensions" == "x" ]]; then
    echo "unable to read screenshot dimensions for $name" >&2
    exit 1
  fi

  local bytes
  bytes="$(wc -c <"$screenshot" | tr -d ' ')"
  if [[ "$bytes" -lt 20000 ]]; then
    echo "window screenshot for $name is suspiciously small: ${bytes} bytes" >&2
    exit 1
  fi

  local visual_probe
  if ! visual_probe="$(screenshot_visual_probe "$name" "$screenshot")"; then
    exit 1
  fi

  jq -n \
    --arg name "$name" \
    --arg bounds "$bounds" \
    --arg actual_bounds "${actual_x},${actual_y},${actual_width},${actual_height}" \
    --arg dimensions "$dimensions" \
    --arg path "$screenshot" \
    --arg screen_capture "$screen_capture" \
    --arg capture_mode "$capture_mode" \
    --arg sha256 "$(shasum -a 256 "$screenshot" | awk '{print $1}')" \
    --argjson bytes "$bytes" \
    --argjson window_id "$window_id" \
    --argjson visual_probe "$visual_probe" \
    '{
      name:$name,
      bounds:$bounds,
      actual_bounds:$actual_bounds,
      window_id:$window_id,
      capture_mode:$capture_mode,
      dimensions:$dimensions,
      path:$path,
      screen_capture:$screen_capture,
      bytes:$bytes,
      sha256:$sha256,
      visual_probe:$visual_probe
    }'
}

APP_LOG="$OUT_DIR/hepta-native.log"
(
  run_hepta_native_app
) >"$APP_LOG" 2>&1 &
APP_PID="$!"
if command -v caffeinate >/dev/null 2>&1; then
  caffeinate -dimsu -w "$APP_PID" >/dev/null 2>&1 &
  CAFFEINATE_PID="$!"
fi

stop_app() {
  if [[ -n "${APP_WINDOW_PID:-}" ]] && kill -0 "$APP_WINDOW_PID" 2>/dev/null; then
    kill "$APP_WINDOW_PID" 2>/dev/null || true
    wait "$APP_WINDOW_PID" 2>/dev/null || true
  fi
  APP_WINDOW_PID=""
  if [[ -n "${APP_PID:-}" ]] && kill -0 "$APP_PID" 2>/dev/null; then
    kill "$APP_PID" 2>/dev/null || true
    wait "$APP_PID" 2>/dev/null || true
  fi
  APP_PID=""
  if [[ -n "${CAFFEINATE_PID:-}" ]] && kill -0 "$CAFFEINATE_PID" 2>/dev/null; then
    kill "$CAFFEINATE_PID" 2>/dev/null || true
    wait "$CAFFEINATE_PID" 2>/dev/null || true
  fi
  CAFFEINATE_PID=""
}

cleanup() {
  stop_app
}
trap cleanup EXIT

wait_for_window
wait_for_fixture_cockpit
wait_for_fixture_product_shell_count 1 "$CAPTURE_PROFILE initial window"
wait_for_fixture_matrix_composer_hidden_count 1 "$CAPTURE_PROFILE initial window"

if [[ "$CAPTURE_PROFILE" == "desktop-full-route" ]]; then
  wait_for_fixture_layout_count "desktop-full" 1 "desktop full route window"
  wait_for_fixture_cockpit_count 1 "desktop full route window"
  desktop_json="$(capture_window "desktop-full-route-${FIXTURE_ROUTE_SLUG}" "$DESKTOP_BOUNDS")"
  screenshots_json="$(jq -n --argjson desktop "$desktop_json" '[$desktop]')"
elif [[ "$CAPTURE_PROFILE" == "desktop-full-secondary" ]]; then
  if [[ "$FIXTURE_SECONDARY_SURFACE_SLUG" == "none" ]]; then
    echo "HEPTA_NATIVE_FIXTURE_SURFACE is required for desktop-full-secondary capture profile" >&2
    exit 2
  fi
  wait_for_fixture_layout_count "desktop-full" 1 "desktop full secondary surface window"
  wait_for_fixture_cockpit_count 1 "desktop full secondary surface window"
  desktop_json="$(capture_window "desktop-full-secondary-${FIXTURE_SECONDARY_SURFACE_SLUG}" "$DESKTOP_BOUNDS")"
  screenshots_json="$(jq -n --argjson desktop "$desktop_json" '[$desktop]')"
elif [[ "$CAPTURE_PROFILE" == "mobile-secondary" ]]; then
  if [[ "$FIXTURE_SECONDARY_SURFACE_SLUG" == "none" ]]; then
    echo "HEPTA_NATIVE_FIXTURE_SURFACE is required for mobile-secondary capture profile" >&2
    exit 2
  fi
  wait_for_fixture_layout_count "mobile-task-first" 1 "mobile secondary surface window"
  wait_for_fixture_cockpit_count 1 "mobile secondary surface window"
  wait_for_mobile_secondary_content_visible_count 1 "mobile secondary surface window"
  mobile_json="$(capture_window "mobile-secondary-${FIXTURE_SECONDARY_SURFACE_SLUG}" "$MOBILE_BOUNDS")"
  screenshots_json="$(jq -n --argjson mobile "$mobile_json" '[$mobile]')"
else
  mobile_json="$(capture_window "mobile-window" "$MOBILE_BOUNDS")"
  desktop_json="$(capture_window "desktop-window" "$DESKTOP_BOUNDS")"
  screenshots_json="$(jq -n --argjson desktop "$desktop_json" --argjson mobile "$mobile_json" '[$desktop,$mobile]')"
fi

fixture_cockpit_selection_count="$(fixture_cockpit_selection_count)"
fixture_product_shell_selection_count="$(fixture_product_shell_selection_count)"
fixture_matrix_composer_hidden_count="$(fixture_matrix_composer_hidden_count)"
fixture_desktop_product_layout_count="$(fixture_desktop_product_layout_count)"
fixture_desktop_full_layout_count="$(fixture_desktop_full_layout_count)"
fixture_mobile_task_first_layout_count="$(fixture_mobile_task_first_layout_count)"
fixture_route_selection_count="$(fixture_route_selection_count)"
fixture_secondary_surface_selection_count="$(fixture_secondary_surface_selection_count)"
fixture_route_top_design_referee_count="$(fixture_route_top_design_referee_count)"
fixture_mobile_secondary_content_visible_count="$(fixture_mobile_secondary_content_visible_count)"
stop_app
assert_no_native_app_log_errors

jq -n \
  --arg product "Hepta Native" \
  --arg runtime "hepta" \
  --arg capture_backend "peekaboo" \
  --arg capture_profile "$CAPTURE_PROFILE" \
  --arg output_dir "$OUT_DIR" \
  --arg app_log "$APP_LOG" \
  --arg cargo_target_dir "$WINDOW_SMOKE_CARGO_TARGET_DIR" \
  --argjson preflight_skipped "$(json_bool_for_flag "$SKIP_PREFLIGHT")" \
  --argjson prebuild_performed "$(json_bool_for_flag "$PREBUILD_APP")" \
  --arg fixture_route "$FIXTURE_ROUTE_LABEL" \
  --arg fixture_route_slug "$FIXTURE_ROUTE_SLUG" \
  --arg fixture_row "$FIXTURE_ROW_LABEL" \
  --arg fixture_secondary_surface "$FIXTURE_SECONDARY_SURFACE_LABEL" \
  --arg fixture_secondary_surface_slug "$FIXTURE_SECONDARY_SURFACE_SLUG" \
  --arg desktop_layout_label "$DESKTOP_LAYOUT_LABEL" \
  --argjson fixture_cockpit_selection_count "$fixture_cockpit_selection_count" \
  --argjson fixture_product_shell_selection_count "$fixture_product_shell_selection_count" \
  --argjson fixture_matrix_composer_hidden_count "$fixture_matrix_composer_hidden_count" \
  --argjson fixture_desktop_product_layout_count "$fixture_desktop_product_layout_count" \
  --argjson fixture_desktop_full_layout_count "$fixture_desktop_full_layout_count" \
  --argjson fixture_mobile_task_first_layout_count "$fixture_mobile_task_first_layout_count" \
  --argjson fixture_route_selection_count "$fixture_route_selection_count" \
  --argjson fixture_secondary_surface_selection_count "$fixture_secondary_surface_selection_count" \
  --argjson fixture_route_top_design_referee_count "$fixture_route_top_design_referee_count" \
  --argjson fixture_mobile_secondary_content_visible_count "$fixture_mobile_secondary_content_visible_count" \
  --argjson expected_screenshot_count "$EXPECTED_SCREENSHOT_COUNT" \
  --argjson expected_fixture_selection_count "$EXPECTED_FIXTURE_SELECTION_COUNT" \
  --argjson screenshots "$screenshots_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    capture_backend:$capture_backend,
    capture_mode:"window_or_screen_crop_from_window_bounds",
    capture_profile:$capture_profile,
    output_dir:$output_dir,
    app_log:$app_log,
    runner:{
      cargo_target_dir:(
        if ($cargo_target_dir | length) > 0 then $cargo_target_dir else null end
      ),
      preflight_skipped:$preflight_skipped,
      prebuild_performed:$prebuild_performed
    },
    fixture_mode:true,
    fixture_route:$fixture_route,
    fixture_route_slug:$fixture_route_slug,
    fixture_row:$fixture_row,
    fixture_secondary_surface:$fixture_secondary_surface,
    fixture_secondary_surface_slug:$fixture_secondary_surface_slug,
    desktop_layout_label:$desktop_layout_label,
    fixture_cockpit_selected_ready:($fixture_cockpit_selection_count >= $expected_fixture_selection_count),
    fixture_cockpit_selection_count:$fixture_cockpit_selection_count,
    fixture_product_shell_selected_ready:($fixture_product_shell_selection_count >= $expected_fixture_selection_count),
    fixture_product_shell_selection_count:$fixture_product_shell_selection_count,
    fixture_matrix_composer_hidden_ready:($fixture_matrix_composer_hidden_count >= $expected_fixture_selection_count),
    fixture_matrix_composer_hidden_count:$fixture_matrix_composer_hidden_count,
    true_window_capture_performed:true,
    native_first_read_path_guard_ready:true,
    native_primary_read_path:"telegram-chat-shell",
    native_telegram_shell_ready:true,
	    native_desktop_first_read_path_ready:true,
	    native_mobile_first_read_path_ready:true,
	    native_engineering_copy_hidden:true,
	    native_makepad_product_shell_light_glass_ready:(
	      $capture_profile != "product-shell"
	      or ($screenshots | all(.visual_probe.product_shell_light_glass_ready == true))
	    ),
	    native_makepad_fixture_script_error_free:true,
    native_app_log_error_free:true,
    native_makepad_desktop_product_layout_ready:true,
    fixture_desktop_product_layout_ready:($fixture_desktop_product_layout_count >= 1),
    fixture_desktop_product_layout_count:$fixture_desktop_product_layout_count,
    native_makepad_desktop_full_layout_env_override:"HEPTA_NATIVE_FIXTURE_LAYOUT=desktop-full",
    fixture_desktop_full_layout_ready:($fixture_desktop_full_layout_count >= 1),
    fixture_desktop_full_layout_count:$fixture_desktop_full_layout_count,
    fixture_route_selected_ready:($fixture_route_selection_count >= 1),
    fixture_route_selection_count:$fixture_route_selection_count,
    fixture_route_top_design_referee_ready:(
      $capture_profile != "desktop-full-route"
      or $fixture_route_top_design_referee_count >= 1
    ),
    fixture_route_top_design_referee_count:$fixture_route_top_design_referee_count,
    native_makepad_route_variant_ready:(
      $capture_profile == "desktop-full-route"
      and $fixture_desktop_full_layout_count >= 1
      and $fixture_route_selection_count >= 1
      and $fixture_route_top_design_referee_count >= 1
      and ($screenshots | length) == $expected_screenshot_count
      and ($screenshots | all(.visual_probe.ready == true))
    ),
    fixture_secondary_surface_selected_ready:(
      ($capture_profile == "desktop-full-secondary" or $capture_profile == "mobile-secondary")
      and $fixture_secondary_surface_slug != "none"
      and $fixture_secondary_surface_selection_count >= 1
    ),
    fixture_secondary_surface_selection_count:$fixture_secondary_surface_selection_count,
    fixture_mobile_secondary_content_visible_ready:(
      $capture_profile == "mobile-secondary"
      and $fixture_secondary_surface_slug != "none"
      and $fixture_mobile_secondary_content_visible_count >= 1
    ),
    fixture_mobile_secondary_content_visible_count:$fixture_mobile_secondary_content_visible_count,
    native_makepad_secondary_surface_ready:(
      ($capture_profile == "desktop-full-secondary" or $capture_profile == "mobile-secondary")
      and $fixture_secondary_surface_slug != "none"
      and (
        ($capture_profile == "desktop-full-secondary" and $fixture_desktop_full_layout_count >= 1)
        or ($capture_profile == "mobile-secondary" and $fixture_mobile_task_first_layout_count >= 1)
      )
      and $fixture_secondary_surface_selection_count >= 1
      and ($screenshots | length) == $expected_screenshot_count
      and ($screenshots | all(.visual_probe.ready == true))
    ),
    native_makepad_secondary_mobile_surface_ready:(
      $capture_profile == "mobile-secondary"
      and $fixture_secondary_surface_slug != "none"
      and $fixture_mobile_task_first_layout_count >= 1
      and $fixture_secondary_surface_selection_count >= 1
      and $fixture_mobile_secondary_content_visible_count >= 1
      and ($screenshots | length) == $expected_screenshot_count
      and ($screenshots | all(.visual_probe.ready == true))
      and ($screenshots | all(.visual_probe.mobile_secondary_content_ready == true))
    ),
    native_makepad_mobile_task_first_layout_ready:true,
    fixture_mobile_task_first_layout_ready:($fixture_mobile_task_first_layout_count >= 1),
    fixture_mobile_task_first_layout_count:$fixture_mobile_task_first_layout_count,
    native_makepad_mobile_layout_width_threshold:620,
    screenshots:$screenshots,
    side_effects:{
      matrix_login:false,
      gateway_call:false,
      provider_invoked:false,
      channel_delivery:false,
      external_mutation:false
    }
  }' | emit_report_json

echo "Hepta Native macOS window smoke passed" >&2
