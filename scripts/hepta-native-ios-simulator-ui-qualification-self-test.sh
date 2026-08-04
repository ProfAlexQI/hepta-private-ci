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

ruby -e '
  width = 640
  height = 1280
  background = [244, 249, 251]
  accent = [20, 132, 160]
  dark = [20, 42, 50]
  pixels = Array.new(width * height) { background.dup }
  draw = lambda do |x0, y0, x1, y1, color|
    (y0...y1).each do |y|
      (x0...x1).each { |x| pixels[y * width + x] = color }
    end
  end
  draw.call(280, 130, 360, 220, accent)
  draw.call(150, 300, 490, 320, dark)
  draw.call(120, 500, 520, 610, accent)
  draw.call(180, 650, 460, 670, dark)
  draw.call(240, 700, 400, 780, accent)
  File.open(ARGV.fetch(0), "wb") do |file|
    file.write("P6\n#{width} #{height}\n255\n")
    pixels.each { |pixel| file.write(pixel.pack("C3")) }
  end
  keyboard = pixels.map(&:dup)
  draw_keyboard = lambda do |x0, y0, x1, y1, color|
    (y0...y1).each do |y|
      (x0...x1).each { |x| keyboard[y * width + x] = color }
    end
  end
  draw_keyboard.call(0, 800, width, height, [190, 198, 210])
  4.times do |row|
    10.times do |column|
      x0 = 8 + column * 63
      y0 = 820 + row * 105
      draw_keyboard.call(x0, y0, [x0 + 54, width].min, [y0 + 82, height].min, [248, 249, 251])
      draw_keyboard.call(x0 + 23, y0 + 26, [x0 + 31, width].min, [y0 + 58, height].min, [40, 44, 50])
    end
  end
  File.open(ARGV.fetch(1), "wb") do |file|
    file.write("P6\n#{width} #{height}\n255\n")
    keyboard.each { |pixel| file.write(pixel.pack("C3")) }
  end
' "$TEST_DIR/baseline.ppm" "$TEST_DIR/keyboard.ppm"
sips -s format png "$TEST_DIR/baseline.ppm" --out "$TEST_DIR/baseline.png" >/dev/null
sips -s format png "$TEST_DIR/keyboard.ppm" --out "$TEST_DIR/keyboard.png" >/dev/null

scripts/hepta-ios-login-ui-probe \
  --baseline "$TEST_DIR/baseline.png" \
  --keyboard "$TEST_DIR/keyboard.png" \
  --output "$TEST_DIR/ready.json" >/dev/null
jq -e '
  .schema_version == 1
  and .kind == "hepta-ios-login-ui-probe"
  and .producer == "scripts/hepta-ios-login-ui-probe"
  and .status == "ready"
  and .ready == true
  and .metrics.checks.keyboard_geometry_present == true
  and .metrics.checks.upper_login_surface_stable == true
  and .metrics.checks.login_interactives_inside_safe_area == true
  and .claims.ios_simulator_login_software_keyboard_ready == true
  and .claims.ios_simulator_login_safe_area_ready == true
  and .claims.generic_software_keyboard_ready == false
  and .claims.generic_safe_area_ready == false
' "$TEST_DIR/ready.json" >/dev/null

cp "$TEST_DIR/baseline.png" "$TEST_DIR/no-keyboard.png"
if scripts/hepta-ios-login-ui-probe \
  --baseline "$TEST_DIR/baseline.png" \
  --keyboard "$TEST_DIR/no-keyboard.png" \
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
  'peekaboo click --no-remote --coords "$CLICK_X,$CLICK_Y" --no-auto-focus' \
  "--keys 'cmd,k'" \
  'scripts/hepta-ios-login-ui-probe --baseline "$BASELINE_SCREENSHOT"' \
  'ios_simulator_login_software_keyboard_ready:true' \
  'ios_simulator_login_safe_area_ready:true' \
  'generic_software_keyboard_ready:false' \
  'generic_safe_area_ready:false' \
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
