#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ios-ui-qualification-self-test.XXXXXX")"
trap 'rm -rf "$TEST_DIR"' EXIT

scripts/hepta-ios-login-ui-probe --help >/dev/null
scripts/hepta-native-ios-simulator-ui-qualification.sh --help >/dev/null

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
  'credential_supply:false' \
  'real_device_contact:false' \
  'code_sign:false' \
  'upload:false'; do
  grep -Fq -- "$needle" scripts/hepta-native-ios-simulator-ui-qualification.sh || {
    echo "missing iOS UI qualification contract: $needle" >&2
    exit 1
  }
done

if rg -n 'security add-generic-password|xcodebuild -allowProvisioningUpdates|notarytool|stapler|curl .*upload|HEPTA_.*PASSWORD' \
    scripts/hepta-native-ios-simulator-ui-qualification.sh >/dev/null; then
  echo "iOS UI qualification contains forbidden credential/sign/upload behavior" >&2
  exit 1
fi

echo "hepta-native iOS simulator UI qualification self-test: PASS"
