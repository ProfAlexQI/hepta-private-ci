#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V6_REPORT_PATH:-}"
V5_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V5_REPORT_PATH:-}"
NATIVE_REPORT_PATH="${HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH:-}"
PIXEL_CENSUS_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V6_PIXEL_CENSUS_REPORT_PATH:-}"
V5_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V6_V5_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v6-pixel-glass-census-gate.json"
fi
if [[ -z "$V5_REPORT_PATH" ]]; then
  V5_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v5-native-detail-census-gate.json"
fi
if [[ -z "$NATIVE_REPORT_PATH" ]]; then
  NATIVE_REPORT_PATH="$READINESS_DIR/native-fixture/native-fixture-visual-smoke.json"
fi
if [[ -z "$PIXEL_CENSUS_REPORT_PATH" ]]; then
  PIXEL_CENSUS_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v6-pixel-glass-census.json"
fi
if [[ -z "$V5_LOG" ]]; then
  V5_LOG="$READINESS_DIR/v5-native-detail-census.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi
if [[ ! -s "$NATIVE_REPORT_PATH" ]]; then
  echo "missing native fixture visual smoke report: $NATIVE_REPORT_PATH" >&2
  exit 1
fi
jq empty "$NATIVE_REPORT_PATH" >/dev/null

mkdir -p "$READINESS_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$PIXEL_CENSUS_REPORT_PATH")"

HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V5_REPORT_PATH="$V5_REPORT_PATH" \
HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH="$NATIVE_REPORT_PATH" \
  bash scripts/hepta-ui-harsh-top-design-referee-v5-native-detail-census-gate.sh "$READINESS_DIR" >"$V5_LOG" 2>&1 || {
    echo "v5 native detail prerequisite failed" >&2
    tail -n 120 "$V5_LOG" >&2 || true
    exit 1
  }

if [[ "$(jq -r '.status' "$V5_REPORT_PATH")" != "ready" ]]; then
  echo "v5 native detail prerequisite was not ready: $V5_REPORT_PATH" >&2
  exit 1
fi

node - "$CHROME_BIN" "$V5_REPORT_PATH" "$NATIVE_REPORT_PATH" "$PIXEL_CENSUS_REPORT_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, v5ReportPath, nativeReportPath, outputPath] = process.argv.slice(2);

const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const existsFile = (file) => Boolean(file && fs.existsSync(file) && fs.statSync(file).isFile());
const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const round = (value, digits = 3) => Number(value.toFixed(digits));

function collectScreenshots() {
  const v5 = readJson(v5ReportPath);
  const native = readJson(nativeReportPath);
  const v4 = readJson(v5.inputs.v4_control_button_census.path);
  const v3 = readJson(v4.inputs.v3_visual_matrix.path);
  const screenshots = [];

  for (const shot of v3.control_matrix.screenshots || []) {
    screenshots.push({ source: "control-v3-visual-matrix", viewport: shot.viewport, name: shot.name, path: shot.path });
  }
  for (const shot of v4.button_census.screenshots || []) {
    screenshots.push({ source: "control-v4-button-census", viewport: shot.viewport, name: shot.name, path: shot.path });
  }
  const nativeDir = native.output_dir;
  for (const filename of fs.readdirSync(nativeDir).filter((item) => item.endsWith(".png") && !item.startsWith("hepta-glass"))) {
    screenshots.push({ source: "native-fixture", viewport: filename.startsWith("mobile") ? "mobile" : "desktop", name: filename.replace(/\.png$/, ""), path: path.join(nativeDir, filename) });
  }

  const seen = new Set();
  return screenshots.filter((shot) => {
    if (!existsFile(shot.path) || seen.has(shot.path)) return false;
    seen.add(shot.path);
    return true;
  });
}

async function imageMetrics(page, file) {
  const bytes = fs.statSync(file).size;
  const data = fs.readFileSync(file).toString("base64");
  const metrics = await page.evaluate(async ({ data }) => {
    const image = new Image();
    image.src = `data:image/png;base64,${data}`;
    await image.decode();
    const canvas = document.createElement("canvas");
    canvas.width = image.naturalWidth;
    canvas.height = image.naturalHeight;
    const context = canvas.getContext("2d", { willReadFrequently: true });
    context.drawImage(image, 0, 0);
    const pixels = context.getImageData(0, 0, canvas.width, canvas.height).data;
    const step = Math.max(1, Math.ceil(Math.sqrt((canvas.width * canvas.height) / 14000)));
    const lumas = [];
    let brightCount = 0;
    let highlightCount = 0;
    let darkCount = 0;
    let chromaticCount = 0;
    let coolCount = 0;
    let glassWhiteCount = 0;
    let saturationSum = 0;
    let textureSum = 0;
    let textureCount = 0;

    const lumaAt = (x, y) => {
      const index = (y * canvas.width + x) * 4;
      return (0.2126 * pixels[index]) + (0.7152 * pixels[index + 1]) + (0.0722 * pixels[index + 2]);
    };

    for (let y = 0; y < canvas.height; y += step) {
      for (let x = 0; x < canvas.width; x += step) {
        const index = (y * canvas.width + x) * 4;
        const r = pixels[index];
        const g = pixels[index + 1];
        const b = pixels[index + 2];
        const luma = (0.2126 * r) + (0.7152 * g) + (0.0722 * b);
        const max = Math.max(r, g, b);
        const min = Math.min(r, g, b);
        const saturation = max > 0 ? (max - min) / max : 0;
        lumas.push(luma);
        saturationSum += saturation;
        if (luma >= 220) brightCount += 1;
        if (luma >= 245) highlightCount += 1;
        if (luma <= 80) darkCount += 1;
        if (max - min >= 12 && luma >= 145) chromaticCount += 1;
        if (b - r >= 10 && luma >= 140) coolCount += 1;
        if (luma >= 178 && saturation <= 0.24) glassWhiteCount += 1;
        if (x + step < canvas.width) {
          textureSum += Math.abs(lumaAt(x + step, y) - luma);
          textureCount += 1;
        }
        if (y + step < canvas.height) {
          textureSum += Math.abs(lumaAt(x, y + step) - luma);
          textureCount += 1;
        }
      }
    }

    lumas.sort((a, b) => a - b);
    const sampleCount = lumas.length;
    const mean = lumas.reduce((sum, value) => sum + value, 0) / sampleCount;
    const variance = lumas.reduce((sum, value) => sum + ((value - mean) ** 2), 0) / sampleCount;
    const percentile = (ratio) => lumas[Math.min(sampleCount - 1, Math.max(0, Math.floor(sampleCount * ratio)))];

    return {
      width: canvas.width,
      height: canvas.height,
      sample_count: sampleCount,
      mean_luma: mean,
      luma_stddev: Math.sqrt(variance),
      luma_p05: percentile(0.05),
      luma_p50: percentile(0.5),
      luma_p95: percentile(0.95),
      bright_ratio: brightCount / sampleCount,
      highlight_ratio: highlightCount / sampleCount,
      dark_ratio: darkCount / sampleCount,
      chromatic_ratio: chromaticCount / sampleCount,
      cool_prismatic_ratio: coolCount / sampleCount,
      glass_white_ratio: glassWhiteCount / sampleCount,
      mean_saturation: saturationSum / sampleCount,
      texture_delta: textureSum / Math.max(1, textureCount),
    };
  }, { data });

  return { ...metrics, bytes, sha256: sha256(file) };
}

function failuresFor(shot, metrics) {
  const minBytes = shot.source === "native-fixture"
    ? 30000
    : shot.viewport === "desktop" || shot.viewport === "narrow"
      ? 80000
      : 45000;
  return [
    ...(metrics.bytes >= minBytes ? [] : ["too_few_bytes"]),
    ...(metrics.width >= 300 && metrics.height >= 760 ? [] : ["unexpected_dimensions"]),
    ...(metrics.mean_luma >= 230 && metrics.mean_luma <= 250 ? [] : ["mean_luma_out_of_range"]),
    ...(metrics.luma_stddev >= 16 ? [] : ["flat_luma"]),
    ...(metrics.luma_p95 >= 246 ? [] : ["weak_highlights"]),
    ...(metrics.highlight_ratio >= 0.1 ? [] : ["insufficient_bright_highlight_area"]),
    ...(metrics.chromatic_ratio >= 0.075 ? [] : ["insufficient_environment_chroma"]),
    ...(metrics.texture_delta >= 4.5 ? [] : ["insufficient_caustic_texture"]),
    ...(metrics.mean_saturation <= 0.12 ? [] : ["oversaturated_one_note_palette"]),
    ...(metrics.dark_ratio <= 0.03 ? [] : ["too_much_dark_area"]),
    ...(metrics.glass_white_ratio >= 0.48 ? [] : ["insufficient_light_glass_area"]),
  ];
}

async function main() {
  const browser = await chromium.launch({
    headless: true,
    executablePath: chromeBin,
    args: [
      "--disable-background-networking",
      "--disable-component-update",
      "--disable-default-apps",
      "--disable-extensions",
      "--disable-sync",
      "--no-default-browser-check",
      "--no-first-run",
    ],
  });
  const page = await browser.newPage({ viewport: { width: 16, height: 16 } });
  const screenshots = collectScreenshots();
  const results = [];

  for (const shot of screenshots) {
    const rawMetrics = await imageMetrics(page, shot.path);
    const metrics = Object.fromEntries(Object.entries(rawMetrics).map(([key, value]) => [key, typeof value === "number" && !Number.isInteger(value) ? round(value) : value]));
    const failures = failuresFor(shot, rawMetrics);
    results.push({ ...shot, metrics, failures, ready: failures.length === 0 });
  }

  await browser.close();

  const bySource = Object.values(results.reduce((acc, item) => {
    acc[item.source] ||= { source: item.source, screenshot_count: 0, failure_count: 0 };
    acc[item.source].screenshot_count += 1;
    if (!item.ready) acc[item.source].failure_count += 1;
    return acc;
  }, {})).sort((a, b) => a.source.localeCompare(b.source));

  const failureCount = results.filter((item) => !item.ready).length;
  const sourceCounts = Object.fromEntries(bySource.map((item) => [item.source, item.screenshot_count]));
  const status = failureCount === 0
    && sourceCounts["control-v3-visual-matrix"] === 56
    && sourceCounts["control-v4-button-census"] === 30
    && (sourceCounts["native-fixture"] || 0) >= 40
    ? "ready"
    : "failed";

  const report = {
    schema_version: "hepta-ui-harsh-top-design-referee-v6-pixel-glass-census/v0",
    standards_version: "2026-07-11-shallow-light-tempered-glass-screenshot-census",
    status,
    thresholds: {
      mean_luma: "230..250",
      luma_stddev_min: 16,
      luma_p95_min: 246,
      highlight_ratio_min: 0.1,
      chromatic_ratio_min: 0.075,
      texture_delta_min: 4.5,
      mean_saturation_max: 0.12,
      dark_ratio_max: 0.03,
      glass_white_ratio_min: 0.48,
    },
    screenshot_count: results.length,
    failure_count: failureCount,
    by_source: bySource,
    failures: results.filter((item) => !item.ready),
    screenshots: results,
  };

  fs.writeFileSync(outputPath, `${JSON.stringify(report, null, 2)}\n`);
  console.log(JSON.stringify(report, null, 2));
}

main().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
NODE

pixel_sha="$(shasum -a 256 "$PIXEL_CENSUS_REPORT_PATH" | awk '{print $1}')"
v5_sha="$(shasum -a 256 "$V5_REPORT_PATH" | awk '{print $1}')"
native_sha="$(shasum -a 256 "$NATIVE_REPORT_PATH" | awk '{print $1}')"
tmp_report="$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v6-final.XXXXXX")"

jq -n \
  --arg v5_path "$V5_REPORT_PATH" \
  --arg native_path "$NATIVE_REPORT_PATH" \
  --arg pixel_path "$PIXEL_CENSUS_REPORT_PATH" \
  --arg v5_sha "$v5_sha" \
  --arg native_sha "$native_sha" \
  --arg pixel_sha "$pixel_sha" \
  --slurpfile v5_file "$V5_REPORT_PATH" \
  --slurpfile pixel_file "$PIXEL_CENSUS_REPORT_PATH" '
  ($v5_file[0]) as $v5
  | ($pixel_file[0]) as $pixel
  | def v5_ready:
      $v5.status == "ready"
      and $v5.v4_ready == true
      and $v5.native_detail_ready == true
      and $v5.summary.control_button_census.failure_count == 0
      and $v5.summary.native_detail_census.case_failure_count == 0
      and $v5.summary.native_detail_census.action_failure_count == 0;
    def pixel_ready:
      $pixel.status == "ready"
      and $pixel.failure_count == 0
      and $pixel.screenshot_count >= 126
      and (($pixel.by_source // []) | any(.source == "control-v3-visual-matrix" and .screenshot_count == 56 and .failure_count == 0))
      and (($pixel.by_source // []) | any(.source == "control-v4-button-census" and .screenshot_count == 30 and .failure_count == 0))
      and (($pixel.by_source // []) | any(.source == "native-fixture" and .screenshot_count >= 40 and .failure_count == 0));
    {
      schema_version:"hepta-ui-harsh-top-design-referee-v6-gate/v0",
      standards_version:"2026-06-27-harsh-control-native-detail-plus-pixel-glass-census",
      status:(if (v5_ready and pixel_ready) then "ready" else "failed" end),
      inputs:{
        v5_native_detail:{path:$v5_path, sha256:$v5_sha},
        native_fixture:{path:$native_path, sha256:$native_sha},
        pixel_glass_census:{path:$pixel_path, sha256:$pixel_sha}
      },
      summary:{
        control_visual_matrix:$v5.summary.control_visual_matrix,
        control_button_census:$v5.summary.control_button_census,
        native_fixture:$v5.summary.native_fixture,
        native_detail_census:$v5.summary.native_detail_census,
        pixel_glass_census:{
          screenshot_count:$pixel.screenshot_count,
          failure_count:$pixel.failure_count,
          by_source:$pixel.by_source,
          thresholds:$pixel.thresholds
        }
      },
      v5_ready:v5_ready,
      pixel_glass_ready:pixel_ready,
      pixel_glass_census:$pixel
    }
  ' >"$tmp_report"

cp "$tmp_report" "$REPORT_PATH"
rm -f "$tmp_report"
cat "$REPORT_PATH"

if [[ "$(jq -r '.status' "$REPORT_PATH")" != "ready" ]]; then
  echo "Hepta UI harsh top-design referee v6 pixel glass census failed" >&2
  exit 1
fi

echo "$REPORT_PATH"
