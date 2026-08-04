#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
PROBE="$ROOT_DIR/scripts/hepta-android-system-bar-contrast-probe"
VERIFY="$ROOT_DIR/scripts/hepta-android-system-bar-contrast-replay-verify"
TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-android-system-bar-contrast-self-test.XXXXXX")"
trap 'rm -rf "$TEST_DIR"' EXIT

make_fixture() {
  local ppm="$1" png="$2" bottom_icon="$3"
  ruby -e '
    width = 200
    height = 400
    bottom_icon = ARGV.fetch(1) == "true"
    pixels = String.new(capacity: width * height * 3, encoding: Encoding::BINARY)
    height.times do |y|
      width.times do |x|
        system_bar = y < 10 || y >= height - 10
        icon = (y.between?(2, 7) && x.between?(88, 111)) ||
          (bottom_icon && y.between?(393, 396) && x.between?(68, 131))
        value = icon ? 24 : (system_bar ? 238 : 230)
        pixels << value.chr << value.chr << value.chr
      end
    end
    File.binwrite(ARGV.fetch(0), "P6\n#{width} #{height}\n255\n".b + pixels)
  ' "$ppm" "$bottom_icon"
  sips -s format png "$ppm" --out "$png" >/dev/null
}

make_fixture "$TEST_DIR/good.ppm" "$TEST_DIR/good.png" true
"$PROBE" --image "$TEST_DIR/good.png" --output "$TEST_DIR/evidence.json"
jq -e '
  .ready == true
  and .regions.status_bar.edge == "top"
  and .regions.status_bar.ready == true
  and .regions.navigation_bar.edge == "bottom"
  and .regions.navigation_bar.ready == true
' "$TEST_DIR/evidence.json" >/dev/null

IMAGE_SHA="$(shasum -a 256 "$TEST_DIR/good.png" | awk '{print $1}')"
EVIDENCE_SHA="$(shasum -a 256 "$TEST_DIR/evidence.json" | awk '{print $1}')"
jq --arg image "$TEST_DIR/good.png" --arg image_sha "$IMAGE_SHA" \
  --arg evidence "$TEST_DIR/evidence.json" --arg evidence_sha "$EVIDENCE_SHA" \
  '{visual_inspection:{portrait:{path:$image,sha256:$image_sha},system_bar_contrast:(. + {evidence_path:$evidence,evidence_sha256:$evidence_sha})}}' \
  "$TEST_DIR/evidence.json" >"$TEST_DIR/receipt.json"
"$VERIFY" --receipt "$TEST_DIR/receipt.json" >/dev/null

jq '.visual_inspection.system_bar_contrast.regions.status_bar.sample.luma_max = 200' \
  "$TEST_DIR/receipt.json" >"$TEST_DIR/tampered-fields.json"
if "$VERIFY" --receipt "$TEST_DIR/tampered-fields.json" >/dev/null 2>&1; then
  echo 'replay verifier accepted tampered embedded luma fields' >&2
  exit 1
fi

cp "$TEST_DIR/evidence.json" "$TEST_DIR/evidence-original.json"
jq '.regions.navigation_bar.sample.luma_span = 200' "$TEST_DIR/evidence.json" >"$TEST_DIR/evidence-tampered.json"
mv -f "$TEST_DIR/evidence-tampered.json" "$TEST_DIR/evidence.json"
if "$VERIFY" --receipt "$TEST_DIR/receipt.json" >/dev/null 2>&1; then
  echo 'replay verifier accepted tampered evidence bytes' >&2
  exit 1
fi
mv -f "$TEST_DIR/evidence-original.json" "$TEST_DIR/evidence.json"

make_fixture "$TEST_DIR/bad-bottom.ppm" "$TEST_DIR/bad-bottom.png" false
if "$PROBE" --image "$TEST_DIR/bad-bottom.png" --output "$TEST_DIR/bad-bottom.json" >/dev/null 2>&1; then
  echo 'contrast probe accepted a navigation bar without dark icon pixels' >&2
  exit 1
fi
jq -e '.ready == false and .regions.status_bar.ready == true and .regions.navigation_bar.ready == false' \
  "$TEST_DIR/bad-bottom.json" >/dev/null

echo 'hepta Android system-bar contrast probe self-test: PASS'
