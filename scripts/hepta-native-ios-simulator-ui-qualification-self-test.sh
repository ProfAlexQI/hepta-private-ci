#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ios-ui-qualification-self-test.XXXXXX")"
trap 'rm -rf "$TEST_DIR"' EXIT
source scripts/hepta-native-mobile-lab-cleanup-v1.sh

bash -n scripts/hepta-native-ios-simulator-ui-qualification.sh scripts/hepta-native-mobile-lab-cleanup-v1.sh
scripts/hepta-ios-login-ui-probe --help >/dev/null
scripts/hepta-native-ios-simulator-ui-qualification.sh --help >/dev/null
scripts/hepta-native-ios-simulator-ui-qualification.sh --lab-contract-only >"$TEST_DIR/lab-contract.json"
jq -e '
  .kind == "hepta-native-ios-simulator-extended-lab-source-contract"
  and .ready == true
  and .opt_in == true
  and (.modes | to_entries | all(.value == true))
  and .state_contract.snapshot_before_mutation == true
  and .state_contract.raw_orientation_snapshot == true
  and .state_contract.orientation_snapshot_backend == "simulator_ax_menu_mark"
  and .state_contract.rtl_matched_control_before_mode == true
  and .state_contract.dynamic_type_matched_control_before_mode == true
  and .state_contract.mode_specific_raster_attribution == true
  and .state_contract.semantic_layout_claims_remain_false == true
  and .state_contract.restore_to_raw_orientation == true
  and .state_contract.exact_orientation_readback == true
  and .state_contract.snapshot_failure_rejected_before_mutation == true
  and .state_contract.restore_and_readback_before_receipt == true
  and .state_contract.restore_failure_fails_closed == true
  and .state_contract.exit_cleanup_preserves_original_status == true
  and .state_contract.interrupt_cleanup_restore_and_readback == true
  and .state_contract.cleanup_failure_receipt == true
  and .claim_boundaries.simulator_only == true
  and .claim_boundaries.generic_app_wide == false
  and .claim_boundaries.real_device == false
  and .claim_boundaries.voiceover == false
  and .claim_boundaries.effective_low_power == false
  and (.forbidden_actions | to_entries | all(.value == false))
' "$TEST_DIR/lab-contract.json" >/dev/null

[[ "$(hepta_mobile_cleanup_final_exit_code 37 true true)" == 37 ]]
[[ "$(hepta_mobile_cleanup_final_exit_code 130 false false)" == 130 ]]
[[ "$(hepta_mobile_cleanup_final_exit_code 0 true false)" == 1 ]]
IOS_CLEANUP_FAILURE="$(hepta_mobile_cleanup_failure_json ios_simulator scripts/hepta-native-ios-simulator-ui-qualification.sh 130 false false)"
jq -e '
  .kind == "hepta-native-mobile-lab-cleanup-failure-receipt"
  and .status == "not_ready" and .ready == false
  and .original_exit_code == 130 and .final_exit_code == 130
  and .local_device_state_mutation_performed == true
  and .local_device_state_may_remain_mutated == true
  and (.blockers | map(.code) | index("ios_simulator_state_restore_command_failed") != null)
  and (.blockers | map(.code) | index("ios_simulator_state_restore_readback_mismatch") != null)
' >/dev/null <<<"$IOS_CLEANUP_FAILURE"

swift -e '
  import AppKit
  import Foundation

  let background = NSColor(calibratedRed: 244/255, green: 249/255, blue: 251/255, alpha: 1)
  let accent = NSColor(calibratedRed: 20/255, green: 132/255, blue: 160/255, alpha: 1)
  let dark = NSColor(calibratedRed: 20/255, green: 42/255, blue: 50/255, alpha: 1)
  let muted = NSColor(calibratedRed: 85/255, green: 105/255, blue: 118/255, alpha: 1)

  func text(_ value: String, x: CGFloat, y: CGFloat, size: CGFloat, color: NSColor, bold: Bool = false) {
    value.draw(at: NSPoint(x: x, y: y), withAttributes: [
      .font: bold ? NSFont.boldSystemFont(ofSize: size) : NSFont.systemFont(ofSize: size),
      .foregroundColor: color
    ])
  }
  func rounded(_ rect: NSRect, radius: CGFloat, fill: NSColor? = nil, stroke: NSColor? = nil, line: CGFloat = 3) {
    let path = NSBezierPath(roundedRect: rect, xRadius: radius, yRadius: radius)
    if let fill { fill.setFill(); path.fill() }
    if let stroke { stroke.setStroke(); path.lineWidth = line; path.stroke() }
  }
  func render(width: Int, height: Int, keyboard: Bool, landscape: Bool, clipLoginBehindKeyboard: Bool, partiallyCoverLoginWithKeyboard: Bool, output: String) {
    let rep = NSBitmapImageRep(bitmapDataPlanes: nil, pixelsWide: width, pixelsHigh: height,
      bitsPerSample: 8, samplesPerPixel: 4, hasAlpha: true, isPlanar: false,
      colorSpaceName: .deviceRGB, bytesPerRow: 0, bitsPerPixel: 0)!
    NSGraphicsContext.saveGraphicsState()
    NSGraphicsContext.current = NSGraphicsContext(bitmapImageRep: rep)
    background.setFill(); NSRect(x: 0, y: 0, width: width, height: height).fill()
    if landscape {
      rounded(NSRect(x: 40, y: 36, width: 1254, height: 678), radius: 44, fill: .white, stroke: NSColor(calibratedWhite: 0.75, alpha: 1))
      rounded(NSRect(x: 80, y: 560, width: 72, height: 72), radius: 36, fill: accent)
      text("H", x: 103, y: 570, size: 44, color: .white, bold: true)
      text("Sign in to Hepta", x: 180, y: 568, size: 48, color: dark, bold: true)
      rounded(NSRect(x: 120, y: 390, width: 520, height: 92), radius: 32, fill: .white, stroke: NSColor(calibratedWhite: 0.72, alpha: 1))
      text("User ID", x: 155, y: 412, size: 34, color: muted)
      rounded(NSRect(x: 120, y: 270, width: 520, height: 92), radius: 32, fill: .white, stroke: NSColor(calibratedWhite: 0.72, alpha: 1))
      text("Password", x: 155, y: 292, size: 34, color: muted)
      rounded(NSRect(x: 690, y: 390, width: 520, height: 92), radius: 32, fill: .white, stroke: NSColor(calibratedWhite: 0.72, alpha: 1))
      text("matrix.org", x: 725, y: 412, size: 34, color: muted)
      text("Homeserver URL (optional)", x: 725, y: 340, size: 28, color: muted)
      rounded(NSRect(x: 690, y: 104, width: 520, height: 100), radius: 38, fill: accent)
      text("Login", x: 905, y: 132, size: 34, color: .white)
    } else {
      // The keyboard fixture deliberately uses a compact-height reflow. This
      // proves the probe accepts bounded, visible movement instead of requiring
      // the unauthenticated surface to remain pixel-identical under the IME.
      let compactShift: CGFloat = keyboard ? 52 : 0
      rounded(NSRect(x: 18, y: 30, width: 714, height: 1226), radius: 60, fill: .white, stroke: NSColor(calibratedWhite: 0.78, alpha: 1))
      rounded(NSRect(x: 330, y: 1135 + compactShift, width: 90, height: 90), radius: 45, fill: accent)
      text("H", x: 354, y: 1148 + compactShift, size: 54, color: .white, bold: true)
      text("Sign in to Hepta", x: 195, y: 1045 + compactShift, size: 52, color: dark, bold: true)
      rounded(NSRect(x: 116, y: 900 + compactShift, width: 518, height: 84), radius: 35, fill: .white, stroke: NSColor(calibratedWhite: 0.72, alpha: 1))
      text("User ID", x: 138, y: 920 + compactShift, size: 38, color: muted)
      rounded(NSRect(x: 116, y: 790 + compactShift, width: 518, height: 84), radius: 35, fill: .white, stroke: NSColor(calibratedWhite: 0.72, alpha: 1))
      text("Password", x: 138, y: 810 + compactShift, size: 38, color: muted)
      rounded(NSRect(x: 116, y: 680 + compactShift, width: 518, height: 84), radius: 35, fill: .white, stroke: NSColor(calibratedWhite: 0.72, alpha: 1))
      text("matrix.org", x: 138, y: 700 + compactShift, size: 38, color: muted)
      text("Homeserver URL (optional)", x: 176, y: 642 + compactShift, size: 30, color: muted)
      let loginY: CGFloat = clipLoginBehindKeyboard ? 360 : (partiallyCoverLoginWithKeyboard ? 470 : 530 + compactShift)
      rounded(NSRect(x: 120, y: loginY, width: 510, height: 96), radius: 36, fill: accent)
      text("Login", x: 326, y: loginY + 27, size: 34, color: .white)
      if keyboard {
        NSColor(calibratedRed: 190/255, green: 198/255, blue: 210/255, alpha: 1).setFill()
        NSRect(x: 0, y: 0, width: width, height: 480).fill()
        for row in 0..<4 {
          for column in 0..<10 {
            let x = 8 + column * 74
            let y = 18 + row * 112
            rounded(NSRect(x: x, y: y, width: 62, height: 86), radius: 8, fill: .white)
            text("I", x: CGFloat(x + 27), y: CGFloat(y + 26), size: 28, color: dark)
          }
        }
      }
    }
    NSGraphicsContext.restoreGraphicsState()
    let data = rep.representation(using: .png, properties: [:])!
    try! data.write(to: URL(fileURLWithPath: output), options: .atomic)
  }
  render(width: 750, height: 1334, keyboard: false, landscape: false, clipLoginBehindKeyboard: false, partiallyCoverLoginWithKeyboard: false, output: CommandLine.arguments[1])
  render(width: 750, height: 1334, keyboard: true, landscape: false, clipLoginBehindKeyboard: false, partiallyCoverLoginWithKeyboard: false, output: CommandLine.arguments[2])
  render(width: 1334, height: 750, keyboard: false, landscape: true, clipLoginBehindKeyboard: false, partiallyCoverLoginWithKeyboard: false, output: CommandLine.arguments[3])
  render(width: 750, height: 1334, keyboard: true, landscape: false, clipLoginBehindKeyboard: true, partiallyCoverLoginWithKeyboard: false, output: CommandLine.arguments[4])
  render(width: 750, height: 1334, keyboard: true, landscape: false, clipLoginBehindKeyboard: false, partiallyCoverLoginWithKeyboard: true, output: CommandLine.arguments[5])
' "$TEST_DIR/baseline.png" "$TEST_DIR/keyboard.png" "$TEST_DIR/landscape.png" "$TEST_DIR/keyboard-clipped.png" "$TEST_DIR/keyboard-partially-covered.png"

scripts/hepta-ios-login-ui-probe --locate-homeserver \
  --baseline "$TEST_DIR/baseline.png" \
  --device-name 'iPhone SE (3rd generation)' \
  --output "$TEST_DIR/homeserver-locator.json" >/dev/null
jq -e '
  .kind == "hepta-ios-homeserver-anchor-locator"
  and .status == "ready" and .ready == true
  and .source_capture.width == 750 and .source_capture.height == 1334
  and .locator.engine == "apple_vision_recognize_text"
  and .locator.match_count == 1
  and .locator.anchor.normalized_text == "matrix.org"
  and .locator.normalized_device_coordinate.x > 0.2
  and .locator.normalized_device_coordinate.x < 0.4
  and .locator.normalized_device_coordinate.y_from_top > 0.4
  and .locator.normalized_device_coordinate.y_from_top < 0.5
  and .locator.tight_hitbox_expansion_normalized == {x:0.012,y:0.012}
  and .claims.baseline_vision_homeserver_anchor_center_ready == true
  and .claims.generic_focus_ready == false
  and .claims.real_device_ready == false
' "$TEST_DIR/homeserver-locator.json" >/dev/null
LOCATOR_X="$(jq -r '.locator.normalized_device_coordinate.x' "$TEST_DIR/homeserver-locator.json")"
LOCATOR_Y="$(jq -r '.locator.normalized_device_coordinate.y_from_top' "$TEST_DIR/homeserver-locator.json")"

scripts/hepta-ios-login-ui-probe \
  --baseline "$TEST_DIR/baseline.png" \
  --keyboard "$TEST_DIR/keyboard.png" \
  --landscape "$TEST_DIR/landscape.png" \
  --device-name 'iPhone SE (3rd generation)' \
  --target-x-ratio "$LOCATOR_X" \
  --target-y-ratio "$LOCATOR_Y" \
  --keyboard-trigger-mode direct_after_vision_homeserver_anchor_click \
  --output "$TEST_DIR/ready.json" >/dev/null
jq -e '
  .schema_version == 2
  and .kind == "hepta-ios-login-ui-probe"
  and .producer == "scripts/hepta-ios-login-ui-probe"
  and .status == "ready"
  and .ready == true
  and .metrics.checks.keyboard_geometry_present == true
  and .metrics.checks.upper_login_surface_stable == false
  and .metrics.checks.accent_geometry_stable == false
  and .metrics.checks.keyboard_anchor_reflow_ready == true
  and .metrics.checks.keyboard_estimated_login_control_clearance_ready == true
  and .metrics.checks.baseline_accent_region_inside_conservative_portrait_insets == true
  and .metrics.checks.small_screen_identity_ready == true
  and .metrics.checks.title_homeserver_login_visible == true
  and .metrics.checks.coordinate_targeted_keyboard_evidence_ready == true
  and .metrics.checks.portrait_visible_anchor_safe_area_geometry_ready == true
  and .metrics.checks.landscape_estimated_login_control_bottom_clearance_ready == true
  and .keyboard_reflow_geometry.vertical_order_preserved == true
  and .keyboard_reflow_geometry.minimum_spacing_preserved == true
  and .keyboard_reflow_geometry.bounded_directional_shift == true
  and .keyboard_reflow_geometry.visible_text_anchors_inside_portrait_safe_area == true
  and .keyboard_reflow_geometry.ready == true
  and .device_geometry.class == "small_phone"
  and .device_geometry.identity_ready == true
  and .coordinate_targeting_evidence.requested_coordinate_target == "baseline_homeserver_text_anchor_center"
  and .coordinate_targeting_evidence.locator == "baseline_vision_homeserver_anchor_center"
  and .coordinate_targeting_evidence.maximum_center_delta_normalized == 0.006
  and .coordinate_targeting_evidence.click_matches_anchor_center == true
  and .coordinate_targeting_evidence.keyboard_trigger_mode == "direct_after_vision_homeserver_anchor_click"
  and .coordinate_targeting_evidence.direct_keyboard_trigger_ready == true
  and .coordinate_targeting_evidence.fallback_keyboard_toggle_used == false
  and .coordinate_targeting_evidence.platform_focus_readback_performed == false
  and .coordinate_targeting_evidence.actual_focused_element == null
  and .coordinate_targeting_evidence.focus_confirmed == false
  and .coordinate_targeting_evidence.ready == true
  and .safe_area_geometry.landscape_login_control_frame_estimation.method == "ocr_text_bbox_expanded_toward_each_edge"
  and .safe_area_geometry.keyboard_login_control_frame_estimation.method == "ocr_text_bbox_expanded_toward_each_edge"
  and .safe_area_geometry.keyboard_login_control_frame_estimation.minimum_expansion_points_per_edge == 24
  and .safe_area_geometry.keyboard_login_control_frame_estimation.minimum_estimated_control_height_points == 48
  and .safe_area_geometry.keyboard_top_estimation.method == "contiguous_bottom_mid_luma_rows"
  and .safe_area_geometry.keyboard_top_estimation.normalized_y_from_top > 0.5
  and .safe_area_geometry.keyboard_estimated_login_control_clearance_points >= 8
  and .safe_area_geometry.required_keyboard_estimated_login_control_clearance_points == 8
  and .safe_area_geometry.keyboard_estimated_login_control_clearance_ready == true
  and .safe_area_geometry.landscape_login_control_frame_estimation.minimum_expansion_points_per_edge == 24
  and .safe_area_geometry.landscape_login_control_frame_estimation.minimum_estimated_control_height_points == 48
  and .safe_area_geometry.landscape_estimated_login_control_bottom_clearance_points >= 24
  and .safe_area_geometry.required_landscape_estimated_login_control_bottom_clearance_points == 24
  and .safe_area_geometry.landscape_estimated_login_control_bottom_clearance_ready == true
  and .claims.ios_simulator_login_software_keyboard_ready == true
  and .claims.ios_simulator_login_visible_anchor_safe_area_ready == true
  and .claims.ios_simulator_login_small_screen_ready == true
  and .claims.ios_simulator_login_required_controls_visible == true
  and .claims.ios_simulator_login_coordinate_targeted_keyboard_ready == true
  and .claims.ios_simulator_login_homeserver_focus_ready == false
  and .claims.ios_simulator_login_keyboard_control_clearance_ready == true
  and .claims.ios_simulator_login_landscape_control_clearance_ready == true
  and .claims.generic_software_keyboard_ready == false
  and .claims.generic_safe_area_ready == false
' "$TEST_DIR/ready.json" >/dev/null
scripts/hepta-ios-login-ui-probe \
  --baseline "$TEST_DIR/baseline.png" \
  --keyboard "$TEST_DIR/keyboard.png" \
  --landscape "$TEST_DIR/landscape.png" \
  --device-name 'iPhone SE (3rd generation)' \
  --target-x-ratio "$LOCATOR_X" \
  --target-y-ratio "$LOCATOR_Y" \
  --keyboard-trigger-mode direct_after_vision_homeserver_anchor_click >"$TEST_DIR/replay.json"
[[ "$(jq -S -c . "$TEST_DIR/replay.json")" == "$(jq -S -c . "$TEST_DIR/ready.json")" ]] \
  || { echo "iOS login UI probe replay is not deterministic" >&2; exit 1; }

cp "$TEST_DIR/baseline.png" "$TEST_DIR/no-keyboard.png"
cp "$TEST_DIR/baseline.png" "$TEST_DIR/portrait-as-landscape.png"
if scripts/hepta-ios-login-ui-probe \
  --baseline "$TEST_DIR/baseline.png" \
  --keyboard "$TEST_DIR/no-keyboard.png" \
  --device-name 'iPhone SE (3rd generation)' \
  --target-x-ratio "$LOCATOR_X" \
  --target-y-ratio "$LOCATOR_Y" \
  --keyboard-trigger-mode direct_after_vision_homeserver_anchor_click \
  --output "$TEST_DIR/not-ready.json" >/dev/null 2>&1; then
  echo "iOS login UI probe accepted a capture without a software keyboard" >&2
  exit 1
fi
jq -e '
  .status == "not_ready"
  and .ready == false
  and (.blockers | index("captures_not_distinct") != null)
  and (.blockers | index("software_keyboard_geometry_missing") != null)
' "$TEST_DIR/not-ready.json" >/dev/null

if scripts/hepta-ios-login-ui-probe \
    --baseline "$TEST_DIR/baseline.png" \
    --keyboard "$TEST_DIR/keyboard-clipped.png" \
    --landscape "$TEST_DIR/landscape.png" \
    --device-name 'iPhone SE (3rd generation)' \
    --target-x-ratio "$LOCATOR_X" \
    --target-y-ratio "$LOCATOR_Y" \
    --keyboard-trigger-mode direct_after_vision_homeserver_anchor_click \
    --output "$TEST_DIR/clipped-anchor.json" >/dev/null 2>&1; then
  echo "iOS login UI probe accepted a Login anchor swallowed by the software keyboard" >&2
  exit 1
fi
jq -e '
  .ready == false
  and .metrics.checks.keyboard_geometry_present == true
  and .metrics.checks.title_homeserver_login_visible == false
  and .metrics.checks.keyboard_anchor_reflow_ready == false
  and (.blockers | index("required_login_text_anchor_missing") != null)
  and (.blockers | index("keyboard_anchor_reflow_invalid") != null)
' "$TEST_DIR/clipped-anchor.json" >/dev/null

if scripts/hepta-ios-login-ui-probe \
    --baseline "$TEST_DIR/baseline.png" \
    --keyboard "$TEST_DIR/keyboard-partially-covered.png" \
    --landscape "$TEST_DIR/landscape.png" \
    --device-name 'iPhone SE (3rd generation)' \
    --target-x-ratio "$LOCATOR_X" \
    --target-y-ratio "$LOCATOR_Y" \
    --keyboard-trigger-mode direct_after_vision_homeserver_anchor_click \
    --output "$TEST_DIR/partially-covered-control.json" >/dev/null 2>&1; then
  echo "iOS login UI probe accepted a Login action frame partially covered by the software keyboard" >&2
  exit 1
fi
jq -e '
  .ready == false
  and .metrics.checks.keyboard_geometry_present == true
  and .metrics.checks.title_homeserver_login_visible == true
  and .metrics.checks.keyboard_estimated_login_control_clearance_ready == false
  and .safe_area_geometry.keyboard_estimated_login_control_clearance_points < 8
  and (.blockers | index("required_login_text_anchor_missing") == null)
  and (.blockers | index("keyboard_login_control_clearance_insufficient") != null)
' "$TEST_DIR/partially-covered-control.json" >/dev/null

if scripts/hepta-ios-login-ui-probe \
    --baseline "$TEST_DIR/baseline.png" \
    --keyboard "$TEST_DIR/keyboard.png" \
    --landscape "$TEST_DIR/landscape.png" \
    --device-name 'iPhone 15 Pro' \
    --target-x-ratio "$LOCATOR_X" \
    --target-y-ratio "$LOCATOR_Y" \
    --keyboard-trigger-mode direct_after_vision_homeserver_anchor_click \
    --output "$TEST_DIR/wrong-device.json" >/dev/null 2>&1; then
  echo "iOS login UI probe accepted a non-small-screen device identity" >&2
  exit 1
fi
jq -e '
  .ready == false
  and .device_geometry.identity_ready == false
  and (.blockers | index("small_screen_device_identity_invalid") != null)
' "$TEST_DIR/wrong-device.json" >/dev/null

if scripts/hepta-ios-login-ui-probe \
    --baseline "$TEST_DIR/baseline.png" \
    --keyboard "$TEST_DIR/keyboard.png" \
    --landscape "$TEST_DIR/portrait-as-landscape.png" \
    --device-name 'iPhone SE (3rd generation)' \
    --target-x-ratio "$LOCATOR_X" \
    --target-y-ratio "$LOCATOR_Y" \
    --keyboard-trigger-mode direct_after_vision_homeserver_anchor_click \
    --output "$TEST_DIR/wrong-landscape.json" >/dev/null 2>&1; then
  echo "iOS login UI probe accepted portrait pixels as landscape evidence" >&2
  exit 1
fi
jq -e '
  .ready == false
  and .safe_area_geometry.landscape_estimated_login_control_bottom_clearance_ready == false
  and (.blockers | index("landscape_canvas_invalid") != null)
' "$TEST_DIR/wrong-landscape.json" >/dev/null

if scripts/hepta-ios-login-ui-probe \
    --baseline "$TEST_DIR/baseline.png" \
    --keyboard "$TEST_DIR/keyboard.png" \
    --device-name 'iPhone SE (3rd generation)' \
    --target-x-ratio "$LOCATOR_X" \
    --target-y-ratio 0.1 \
    --keyboard-trigger-mode direct_after_vision_homeserver_anchor_click \
    --output "$TEST_DIR/wrong-focus.json" >/dev/null 2>&1; then
  echo "iOS login UI probe accepted a target coordinate outside the homeserver anchor band" >&2
  exit 1
fi
jq -e '
  .ready == false
  and .coordinate_targeting_evidence.ready == false
  and (.blockers | index("coordinate_targeted_keyboard_evidence_missing") != null)
' "$TEST_DIR/wrong-focus.json" >/dev/null

for needle in \
  'scripts/hepta-ui-source-fingerprint' \
  'hepta-native-ios-simulator-smoke-receipt' \
  'scripts/hepta-native-ios-simulator-smoke.sh' \
  'shasum -a 256 "$ARTIFACT_PATH"' \
  'xcrun simctl uninstall "$UDID" "$BUNDLE_ID"' \
  'xcrun simctl install "$UDID" "$APP_PATH"' \
  'grep -F "https://github.com/ProfAlexQI/Hepta/commit/$SOURCE_HEAD"' \
  '/usr/bin/caffeinate -dimsu -w "$$"' \
  'peekaboo window focus --no-remote --app Simulator' \
  'peekaboo see --no-remote --app Simulator --window-id "$SIMULATOR_WINDOW_ID"' \
  'DEVICE_CANVAS_FRAME_JSON="$(jq -cer' \
  'peekaboo_ax_unique_device_aspect_frame' \
  '"$DEVICE_CANVAS_X" "$DEVICE_CANVAS_WIDTH" "$TARGET_X_RATIO"' \
  '"$DEVICE_CANVAS_Y" "$DEVICE_CANVAS_HEIGHT" "$TARGET_Y_RATIO"' \
  'peekaboo click --no-remote --coords "$CLICK_X,$CLICK_Y" --no-auto-focus' \
  "--keys 'cmd,k'" \
  'scripts/hepta-ios-login-ui-probe --baseline "$BASELINE_SCREENSHOT"' \
  '--device-name "$DEVICE_NAME"' \
  '--target-x-ratio "$TARGET_X_RATIO"' \
  '--landscape "$LANDSCAPE_PATH"' \
  'normalize_landscape_capture' \
  'simctl_portrait_canvas_rotated_clockwise_90' \
  'landscape_capture_normalization:$landscape_capture_normalization' \
  'ios_simulator_login_software_keyboard_ready:true' \
  'ios_simulator_login_visible_anchor_safe_area_ready:$visible_anchor_safe_area_ready' \
  'ios_simulator_login_small_screen_ready:$small_screen_ready' \
  'ios_simulator_login_required_controls_visible:$required_controls_ready' \
  'ios_simulator_login_coordinate_targeted_keyboard_ready:$coordinate_targeted_keyboard_ready' \
  'ios_simulator_login_homeserver_focus_ready:false' \
  'ios_simulator_login_keyboard_control_clearance_ready:$keyboard_control_clearance_ready' \
  'ios_simulator_login_landscape_control_clearance_ready:$landscape_control_clearance_ready' \
  'generic_software_keyboard_ready:false' \
  'generic_safe_area_ready:false' \
  'generic_rotation_ready:false' \
  '--extended-lab' \
  "--path 'Device > Orientation > Landscape Right'" \
  'xcrun simctl ui "$UDID" content_size "$DYNAMIC_TYPE_SIZE"' \
  'RTL_CONTROL_PATH="$LAB_EVIDENCE_DIR/rtl-control-ltr.png"' \
  'DYNAMIC_CONTROL_PATH="$LAB_EVIDENCE_DIR/dynamic-type-control-$ORIGINAL_CONTENT_SIZE.png"' \
  'mode_attributable_raster_change:$rtl_changed' \
  'mode_attributable_raster_change:$dynamic_changed' \
  'semantic_layout_verified:false' \
  'semantic_text_reflow_verified:false' \
  'restore_ios_lab_state' \
  'AXMenuItemMarkChar' \
  'ORIGINAL_ORIENTATION="$(simulator_orientation)"' \
  '--path "Device > Orientation > $ORIGINAL_ORIENTATION"' \
  'wait_for_simulator_orientation "$ORIGINAL_ORIENTATION"' \
  'RESTORED_ORIENTATION="$(simulator_orientation)"' \
  'ios_lab_state_readback_ready' \
  'write_ios_cleanup_failure_receipt' \
  'hepta_mobile_cleanup_final_exit_code "$original_exit"' \
  "trap 'exit 130' INT" \
  'Simulator extended-lab state restoration failed' \
  'ios_simulator_effective_low_power_mode_unsupported' \
  'effective_low_power_mode:false' \
  'ios_real_device_receipt_missing' \
  'voiceover_receipt_missing' \
  'account_connection:false' \
  'credential_supply:false' \
  'real_device_contact:false' \
  'code_sign:false' \
  'upload:false'; do
  grep -Fq -- "$needle" scripts/hepta-native-ios-simulator-ui-qualification.sh || {
    echo "missing iOS UI qualification contract: $needle" >&2
    exit 1
  }
done

if grep -Fq 'restore_ios_lab_state || true' scripts/hepta-native-ios-simulator-ui-qualification.sh; then
  echo "iOS cleanup still swallows state restoration failure" >&2
  exit 1
fi

ruby -e '
  source = File.read(ARGV.fetch(0))
  snapshot = source.index(%q{ORIGINAL_ORIENTATION="$(simulator_orientation)"}) or abort "orientation snapshot missing"
  mutation = source.index("LAB_STATE_MUTATED=true", snapshot) or abort "lab mutation missing"
  orientation_flag = source.index("LAB_ORIENTATION_MUTATED=true", snapshot) or abort "orientation mutation flag missing"
  landscape_click = source.index(%q{--path '\''Device > Orientation > Landscape Right'\''}, orientation_flag) or abort "landscape mutation missing"
  readback = source.index(%q{RESTORED_ORIENTATION="$(simulator_orientation)"}, landscape_click) or abort "orientation readback missing"
  cleared = source.index("LAB_STATE_MUTATED=false", readback) or abort "restore success flag missing"
  abort "iOS lab state ordering is not fail-closed" unless snapshot < mutation && orientation_flag < landscape_click && readback < cleared
' scripts/hepta-native-ios-simulator-ui-qualification.sh

if rg -n 'security add-generic-password|xcodebuild -allowProvisioningUpdates|notarytool|stapler|curl .*upload|HEPTA_.*PASSWORD|xctrace list devices' \
    scripts/hepta-native-ios-simulator-ui-qualification.sh >/dev/null; then
  echo "iOS UI qualification contains forbidden credential/sign/upload behavior" >&2
  exit 1
fi

echo "hepta-native iOS simulator UI qualification self-test: PASS"
