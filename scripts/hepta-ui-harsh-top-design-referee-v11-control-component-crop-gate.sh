#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V11_REPORT_PATH:-}"
V10_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V10_REPORT_PATH:-}"
NATIVE_REPORT_PATH="${HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH:-}"
COMPONENT_CROP_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V11_COMPONENT_CROP_REPORT_PATH:-}"
COMPONENT_CROP_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V11_COMPONENT_CROP_DIR:-}"
V10_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V11_V10_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v11-control-component-crop-gate.json"
fi
if [[ -z "$V10_REPORT_PATH" ]]; then
  V10_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v10-local-crop-glass-gate.json"
fi
if [[ -z "$NATIVE_REPORT_PATH" ]]; then
  NATIVE_REPORT_PATH="$READINESS_DIR/native-fixture/native-fixture-visual-smoke.json"
fi
if [[ -z "$COMPONENT_CROP_REPORT_PATH" ]]; then
  COMPONENT_CROP_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v11-control-component-crop-census.json"
fi
if [[ -z "$COMPONENT_CROP_DIR" ]]; then
  COMPONENT_CROP_DIR="$READINESS_DIR/ui-harsh-v11-control-component-crops"
fi
if [[ -z "$V10_LOG" ]]; then
  V10_LOG="$READINESS_DIR/v10-local-crop-glass.log"
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

mkdir -p "$READINESS_DIR" "$COMPONENT_CROP_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$COMPONENT_CROP_REPORT_PATH")"

HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V10_REPORT_PATH="$V10_REPORT_PATH" \
HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH="$NATIVE_REPORT_PATH" \
  bash scripts/hepta-ui-harsh-top-design-referee-v10-local-crop-glass-gate.sh "$READINESS_DIR" >"$V10_LOG" 2>&1 || {
    echo "v10 local crop glass prerequisite failed" >&2
    tail -n 160 "$V10_LOG" >&2 || true
    exit 1
  }

if [[ "$(jq -r '.status' "$V10_REPORT_PATH")" != "ready" ]]; then
  echo "v10 local crop glass prerequisite was not ready: $V10_REPORT_PATH" >&2
  exit 1
fi

node - "$CHROME_BIN" "$V10_REPORT_PATH" "$COMPONENT_CROP_DIR" "$COMPONENT_CROP_REPORT_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, v10ReportPath, cropDir, outputPath] = process.argv.slice(2);

const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase();
const round = (value, digits = 3) => Number(value.toFixed(digits));

fs.mkdirSync(cropDir, { recursive: true });

function resolveV4Path(v10) {
  const v9 = readJson(v10.inputs.v9_switching.path);
  const v8 = readJson(v9.inputs.v8_lifecycle.path);
  const v7 = readJson(v8.inputs.v7_real_click.path);
  const v6 = readJson(v7.inputs.v6_pixel_glass.path);
  const v5 = readJson(v6.inputs.v5_native_detail.path);
  return v5.inputs.v4_control_button_census.path;
}

async function cropMetrics(page, sourcePath, rect, outputFile) {
  const source = fs.readFileSync(sourcePath).toString("base64");
  const data = await page.evaluate(async ({ source, rect }) => {
    const image = new Image();
    image.src = `data:image/png;base64,${source}`;
    await image.decode();
    const left = Math.max(0, Math.floor(rect.left));
    const top = Math.max(0, Math.floor(rect.top));
    const width = Math.max(1, Math.min(image.naturalWidth - left, Math.ceil(rect.width || (rect.right - rect.left))));
    const height = Math.max(1, Math.min(image.naturalHeight - top, Math.ceil(rect.height || (rect.bottom - rect.top))));
    const canvas = document.createElement("canvas");
    canvas.width = width;
    canvas.height = height;
    const context = canvas.getContext("2d");
    context.drawImage(image, left, top, width, height, 0, 0, width, height);
    return canvas.toDataURL("image/png").split(",")[1];
  }, { source, rect });
  fs.writeFileSync(outputFile, Buffer.from(data, "base64"));
  return metricsForPng(page, outputFile);
}

async function metricsForPng(page, file) {
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
    const step = Math.max(1, Math.ceil(Math.sqrt((canvas.width * canvas.height) / 5000)));
    const lumas = [];
    let highlightCount = 0;
    let darkCount = 0;
    let chromaticCount = 0;
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
        if (luma >= 245) highlightCount += 1;
        if (luma <= 95) darkCount += 1;
        if (max - min >= 10 && luma >= 140) chromaticCount += 1;
        if (luma >= 178 && saturation <= 0.30) glassWhiteCount += 1;
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
      highlight_ratio: highlightCount / sampleCount,
      dark_ratio: darkCount / sampleCount,
      chromatic_ratio: chromaticCount / sampleCount,
      glass_white_ratio: glassWhiteCount / sampleCount,
      mean_saturation: saturationSum / sampleCount,
      texture_delta: textureSum / Math.max(1, textureCount),
    };
  }, { data });
  return { ...metrics, bytes, sha256: sha256(file) };
}

function normalizeMetrics(rawMetrics) {
  return Object.fromEntries(Object.entries(rawMetrics).map(([key, value]) => [
    key,
    typeof value === "number" && !Number.isInteger(value) ? round(value) : value,
  ]));
}

function failuresFor(kind, rawMetrics, sourceReady) {
  const isModule = kind === "module";
  return [
    ...(sourceReady ? [] : ["source_dom_census_not_ready"]),
    ...(rawMetrics.bytes >= 900 ? [] : ["too_few_bytes"]),
    ...(rawMetrics.width >= 24 && rawMetrics.height >= 24 ? [] : ["crop_too_small"]),
    ...(rawMetrics.mean_luma >= 208 && rawMetrics.mean_luma <= 252 ? [] : ["mean_luma_out_of_range"]),
    ...(rawMetrics.luma_p95 >= 234 ? [] : ["weak_local_highlights"]),
    ...(rawMetrics.mean_saturation <= 0.14 ? [] : ["oversaturated_local_palette"]),
    ...(rawMetrics.dark_ratio <= 0.12 ? [] : ["too_much_local_dark_area"]),
    ...(rawMetrics.glass_white_ratio >= 0.82 ? [] : ["insufficient_local_light_glass_area"]),
    ...(rawMetrics.luma_stddev >= (isModule ? 3.0 : 2.0) ? [] : ["locally_flat_luma"]),
    ...(rawMetrics.texture_delta >= (isModule ? 0.45 : 0.15) ? [] : ["insufficient_local_texture_signal"]),
  ];
}

async function main() {
  const v10 = readJson(v10ReportPath);
  const v4Path = resolveV4Path(v10);
  const v4 = readJson(v4Path).button_census;
  const browser = await chromium.launch({
    headless: true,
    executablePath: chromeBin,
    args: [
      "--disable-background-networking",
      "--disable-component-update",
      "--disable-default-apps",
      "--disable-extensions",
      "--disable-sync",
      "--hide-scrollbars",
      "--no-default-browser-check",
      "--no-first-run",
    ],
  });
  const page = await browser.newPage({ viewport: { width: 16, height: 16 }, deviceScaleFactor: 1 });
  const crops = [];
  const skippedObscured = [];

  for (const viewport of v4.viewports || []) {
    for (const state of viewport.states || []) {
      const screenshotPath = state.screenshot?.path;
      const stateKey = state.state_key || "state";
      if (!screenshotPath || !fs.existsSync(screenshotPath)) {
        crops.push({
          source: "control-v4-button-census",
          crop_kind: "state-error",
          viewport: viewport.name,
          state: stateKey,
          ready: false,
          failures: ["missing_state_screenshot"],
        });
        continue;
      }
      for (const [index, item] of (state.interactive_sample || []).entries()) {
        if (item.visually_unobscured === false) {
          skippedObscured.push({
            source: "control-v4-button-census",
            crop_kind: item.button_like ? "button" : item.field_like ? "field" : item.link_like ? "link" : "interactive",
            viewport: viewport.name,
            state: stateKey,
            selector: item.selector,
            label: item.label,
            reason: "not_topmost_visible",
          });
          continue;
        }
        const cropKind = item.button_like ? "button" : item.field_like ? "field" : item.link_like ? "link" : "interactive";
        const cropPath = path.join(cropDir, `${sanitize(viewport.name)}-${sanitize(stateKey)}-${cropKind}-${index}-${sanitize(item.label || item.selector || "interactive")}.png`);
        const rawMetrics = await cropMetrics(page, screenshotPath, item, cropPath);
        const failures = failuresFor(cropKind, rawMetrics, item.ready === true);
        crops.push({
          source: "control-v4-button-census",
          crop_kind: cropKind,
          viewport: viewport.name,
          state: stateKey,
          selector: item.selector,
          label: item.label,
          crop_path: cropPath,
          source_screenshot: screenshotPath,
          metrics: normalizeMetrics(rawMetrics),
          failures,
          ready: failures.length === 0,
        });
      }
      for (const [index, item] of (state.module_sample || []).entries()) {
        if (item.visually_unobscured === false) {
          skippedObscured.push({
            source: "control-v4-button-census",
            crop_kind: "module",
            viewport: viewport.name,
            state: stateKey,
            selector: item.selector,
            reason: "not_topmost_visible",
          });
          continue;
        }
        const cropPath = path.join(cropDir, `${sanitize(viewport.name)}-${sanitize(stateKey)}-module-${index}-${sanitize(item.selector || "module")}.png`);
        const rawMetrics = await cropMetrics(page, screenshotPath, item, cropPath);
        const failures = failuresFor("module", rawMetrics, item.ready === true);
        crops.push({
          source: "control-v4-button-census",
          crop_kind: "module",
          viewport: viewport.name,
          state: stateKey,
          selector: item.selector,
          crop_path: cropPath,
          source_screenshot: screenshotPath,
          metrics: normalizeMetrics(rawMetrics),
          failures,
          ready: failures.length === 0,
        });
      }
    }
  }

  await page.close();
  await browser.close();

  const byKind = Object.values(crops.reduce((acc, item) => {
    acc[item.crop_kind] ||= { crop_kind: item.crop_kind, crop_count: 0, failure_count: 0 };
    acc[item.crop_kind].crop_count += 1;
    if (!item.ready) acc[item.crop_kind].failure_count += 1;
    return acc;
  }, {})).sort((a, b) => a.crop_kind.localeCompare(b.crop_kind));
  const byViewport = Object.values(crops.reduce((acc, item) => {
    acc[item.viewport] ||= { viewport: item.viewport, crop_count: 0, failure_count: 0 };
    acc[item.viewport].crop_count += 1;
    if (!item.ready) acc[item.viewport].failure_count += 1;
    return acc;
  }, {})).sort((a, b) => a.viewport.localeCompare(b.viewport));
  const kindCounts = Object.fromEntries(byKind.map((item) => [item.crop_kind, item.crop_count]));
  const expectedByKind = { button: 0, field: 0, interactive: 0, link: 0, module: 0 };
  for (const viewport of v4.viewports || []) {
    for (const state of viewport.states || []) {
      for (const item of (state.interactive_sample || [])) {
        if (item.visually_unobscured === false) continue;
        const cropKind = item.button_like ? "button" : item.field_like ? "field" : item.link_like ? "link" : "interactive";
        expectedByKind[cropKind] = (expectedByKind[cropKind] || 0) + 1;
      }
      expectedByKind.module += (state.module_sample || []).filter((item) => item.visually_unobscured !== false).length;
    }
  }
  const expectedTotal = Object.values(expectedByKind).reduce((sum, value) => sum + value, 0);
  const failureCount = crops.filter((item) => !item.ready).length;
  const samplesComplete = crops.length === expectedTotal
    && Object.entries(expectedByKind).every(([kind, count]) => (kindCounts[kind] || 0) === count);
  const report = {
    schema_version: "hepta-ui-harsh-top-design-referee-v11-control-component-crop-census/v0",
    standards_version: "2026-06-27-control-topmost-button-field-link-module-local-crop-glass-census",
    status: failureCount === 0 && samplesComplete ? "ready" : "failed",
    v4_button_census_path: v4Path,
    crop_dir: cropDir,
    crop_count: crops.length,
    failure_count: failureCount,
    skipped_obscured_count: skippedObscured.length,
    skipped_obscured: skippedObscured,
    by_kind: byKind,
    by_viewport: byViewport,
    expected_counts: {
      total_crop_count: expectedTotal,
      by_kind: expectedByKind,
      source_interactive_instance_count: v4.interactive_instance_count,
      source_button_like_instance_count: v4.button_like_instance_count,
      source_module_instance_count: v4.module_instance_count,
    },
    thresholds: {
      crop_min_width: 24,
      crop_min_height: 24,
      mean_luma: "208..252",
      luma_p95_min: 234,
      mean_saturation_max: 0.14,
      dark_ratio_max: 0.12,
      glass_white_ratio_min: 0.82,
      control_luma_stddev_min: 2.0,
      module_luma_stddev_min: 3.0,
      control_texture_delta_min: 0.15,
      module_texture_delta_min: 0.45,
    },
    failures: crops.filter((item) => !item.ready),
    crops,
  };
  fs.writeFileSync(outputPath, `${JSON.stringify(report, null, 2)}\n`);
  console.log(JSON.stringify(report, null, 2));
}

main().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
NODE

component_crop_sha="$(shasum -a 256 "$COMPONENT_CROP_REPORT_PATH" | awk '{print $1}')"
v10_sha="$(shasum -a 256 "$V10_REPORT_PATH" | awk '{print $1}')"
native_sha="$(shasum -a 256 "$NATIVE_REPORT_PATH" | awk '{print $1}')"
tmp_report="$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v11-final.XXXXXX")"

jq -n \
  --arg v10_path "$V10_REPORT_PATH" \
  --arg native_path "$NATIVE_REPORT_PATH" \
  --arg component_crop_path "$COMPONENT_CROP_REPORT_PATH" \
  --arg v10_sha "$v10_sha" \
  --arg native_sha "$native_sha" \
  --arg component_crop_sha "$component_crop_sha" \
  --slurpfile v10_file "$V10_REPORT_PATH" \
  --slurpfile component_crop_file "$COMPONENT_CROP_REPORT_PATH" '
  ($v10_file[0]) as $v10
  | ($component_crop_file[0]) as $component_crop
  | def v10_ready:
      $v10.status == "ready"
      and $v10.v9_ready == true
      and $v10.local_crop_glass_ready == true
      and $v10.summary.local_crop_glass_census.failure_count == 0;
    def component_crop_ready:
      $component_crop.status == "ready"
      and $component_crop.failure_count == 0
      and $component_crop.crop_count == $component_crop.expected_counts.total_crop_count
      and (($component_crop.by_kind // []) | any(.crop_kind == "button" and .crop_count == ($component_crop.expected_counts.by_kind.button // 0) and .failure_count == 0))
      and (($component_crop.by_kind // []) | any(.crop_kind == "field" and .crop_count == ($component_crop.expected_counts.by_kind.field // 0) and .failure_count == 0))
      and (($component_crop.by_kind // []) | any(.crop_kind == "interactive" and .crop_count == ($component_crop.expected_counts.by_kind.interactive // 0) and .failure_count == 0))
      and (($component_crop.by_kind // []) | any(.crop_kind == "link" and .crop_count == ($component_crop.expected_counts.by_kind.link // 0) and .failure_count == 0))
      and (($component_crop.by_kind // []) | any(.crop_kind == "module" and .crop_count == ($component_crop.expected_counts.by_kind.module // 0) and .failure_count == 0));
    {
      schema_version:"hepta-ui-harsh-top-design-referee-v11-gate/v0",
      standards_version:"2026-06-27-harsh-v10-plus-control-topmost-component-local-crop-census",
      status:(if (v10_ready and component_crop_ready) then "ready" else "failed" end),
      inputs:{
        v10_local_crop_glass:{path:$v10_path, sha256:$v10_sha},
        native_fixture:{path:$native_path, sha256:$native_sha},
        control_component_crop_census:{path:$component_crop_path, sha256:$component_crop_sha}
      },
      summary:{
        control_visual_matrix:$v10.summary.control_visual_matrix,
        control_button_census:$v10.summary.control_button_census,
        native_fixture:$v10.summary.native_fixture,
        native_detail_census:$v10.summary.native_detail_census,
        pixel_glass_census:$v10.summary.pixel_glass_census,
        control_real_click_activation:$v10.summary.control_real_click_activation,
        control_submenu_lifecycle:$v10.summary.control_submenu_lifecycle,
        control_submenu_switching:$v10.summary.control_submenu_switching,
        local_crop_glass_census:$v10.summary.local_crop_glass_census,
        control_component_crop_census:{
          crop_count:$component_crop.crop_count,
          failure_count:$component_crop.failure_count,
          skipped_obscured_count:$component_crop.skipped_obscured_count,
          by_kind:$component_crop.by_kind,
          by_viewport:$component_crop.by_viewport,
          thresholds:$component_crop.thresholds
        }
      },
      v10_ready:v10_ready,
      control_component_crop_ready:component_crop_ready,
      control_component_crop_census:$component_crop
    }
  ' >"$tmp_report"

cp "$tmp_report" "$REPORT_PATH"
rm -f "$tmp_report"
cat "$REPORT_PATH"

if [[ "$(jq -r '.status' "$REPORT_PATH")" != "ready" ]]; then
  echo "Hepta UI harsh top-design referee v11 control component crop gate failed" >&2
  exit 1
fi

echo "$REPORT_PATH"
