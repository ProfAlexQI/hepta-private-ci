#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V10_REPORT_PATH:-}"
V9_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V9_REPORT_PATH:-}"
NATIVE_REPORT_PATH="${HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH:-}"
LOCAL_CROP_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V10_LOCAL_CROP_REPORT_PATH:-}"
LOCAL_CROP_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V10_LOCAL_CROP_DIR:-}"
V9_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V10_V9_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v10-local-crop-glass-gate.json"
fi
if [[ -z "$V9_REPORT_PATH" ]]; then
  V9_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v9-switching-gate.json"
fi
if [[ -z "$NATIVE_REPORT_PATH" ]]; then
  NATIVE_REPORT_PATH="$READINESS_DIR/native-fixture/native-fixture-visual-smoke.json"
fi
if [[ -z "$LOCAL_CROP_REPORT_PATH" ]]; then
  LOCAL_CROP_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v10-local-crop-glass-census.json"
fi
if [[ -z "$LOCAL_CROP_DIR" ]]; then
  LOCAL_CROP_DIR="$READINESS_DIR/ui-harsh-v10-local-crops"
fi
if [[ -z "$V9_LOG" ]]; then
  V9_LOG="$READINESS_DIR/v9-switching.log"
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

mkdir -p "$READINESS_DIR" "$LOCAL_CROP_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$LOCAL_CROP_REPORT_PATH")"

HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V9_REPORT_PATH="$V9_REPORT_PATH" \
HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH="$NATIVE_REPORT_PATH" \
  bash scripts/hepta-ui-harsh-top-design-referee-v9-switching-gate.sh "$READINESS_DIR" >"$V9_LOG" 2>&1 || {
    echo "v9 switching prerequisite failed" >&2
    tail -n 140 "$V9_LOG" >&2 || true
    exit 1
  }

if [[ "$(jq -r '.status' "$V9_REPORT_PATH")" != "ready" ]]; then
  echo "v9 switching prerequisite was not ready: $V9_REPORT_PATH" >&2
  exit 1
fi

node - "$CHROME_BIN" "$V9_REPORT_PATH" "$NATIVE_REPORT_PATH" "$LOCAL_CROP_DIR" "$LOCAL_CROP_REPORT_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const { pathToFileURL } = require("node:url");

const [chromeBin, v9ReportPath, nativeReportPath, cropDir, outputPath] = process.argv.slice(2);

const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase();
const round = (value, digits = 3) => Number(value.toFixed(digits));

fs.mkdirSync(cropDir, { recursive: true });

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
    const step = Math.max(1, Math.ceil(Math.sqrt((canvas.width * canvas.height) / 6000)));
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
        if (luma >= 178 && saturation <= 0.28) glassWhiteCount += 1;
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

async function cropFromScreenshot(page, sourcePath, rect, outputFile) {
  const data = fs.readFileSync(sourcePath).toString("base64");
  const pngData = await page.evaluate(async ({ data, rect }) => {
    const image = new Image();
    image.src = `data:image/png;base64,${data}`;
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
  }, { data, rect });
  fs.writeFileSync(outputFile, Buffer.from(pngData, "base64"));
  return metricsForPng(page, outputFile);
}

function failuresFor(kind, rawMetrics) {
  const widthMin = kind === "surface" ? 44 : 44;
  const heightMin = kind === "surface" ? 44 : 44;
  return [
    ...(rawMetrics.bytes >= 1400 ? [] : ["too_few_bytes"]),
    ...(rawMetrics.width >= widthMin && rawMetrics.height >= heightMin ? [] : ["crop_too_small"]),
    ...(rawMetrics.mean_luma >= 216 && rawMetrics.mean_luma <= 248 ? [] : ["mean_luma_out_of_range"]),
    ...(rawMetrics.luma_stddev >= 20 ? [] : ["flat_local_luma"]),
    ...(rawMetrics.luma_p95 >= 245 ? [] : ["weak_local_highlights"]),
    ...(rawMetrics.highlight_ratio >= 0.055 ? [] : ["insufficient_local_highlight_area"]),
    ...(rawMetrics.chromatic_ratio >= 0.18 ? [] : ["insufficient_local_prismatic_chroma"]),
    ...(rawMetrics.texture_delta >= 4.0 ? [] : ["insufficient_local_caustic_texture"]),
    ...(rawMetrics.mean_saturation <= 0.13 ? [] : ["oversaturated_local_palette"]),
    ...(rawMetrics.dark_ratio <= 0.09 ? [] : ["too_much_local_dark_area"]),
    ...(rawMetrics.glass_white_ratio >= 0.86 ? [] : ["insufficient_local_light_glass_area"]),
  ];
}

function normalizeMetrics(rawMetrics) {
  return Object.fromEntries(Object.entries(rawMetrics).map(([key, value]) => [
    key,
    typeof value === "number" && !Number.isInteger(value) ? round(value) : value,
  ]));
}

async function collectControlCrops(metricPage, v9) {
  const results = [];
  const switchReport = v9.control_submenu_switching;
  for (const viewport of switchReport.viewports || []) {
    for (const sequence of viewport.sequences || []) {
      for (const step of sequence.steps || []) {
        const screenshot = step.screenshot?.path;
        if (!screenshot || !fs.existsSync(screenshot)) {
          results.push({
            source: "control-v9-submenu-switching",
            crop_kind: "missing-screenshot",
            viewport: viewport.name,
            sequence: sequence.key,
            target: step.target,
            ready: false,
            failures: ["missing_source_screenshot"],
          });
          continue;
        }
        for (const [index, surface] of (step.audit?.surface_details || []).entries()) {
          const cropPath = path.join(cropDir, `${sanitize(viewport.name)}-${sanitize(sequence.key)}-${sanitize(step.target)}-surface-${index}.png`);
          const rawMetrics = await cropFromScreenshot(metricPage, screenshot, surface, cropPath);
          const failures = failuresFor("surface", rawMetrics);
          results.push({
            source: "control-v9-submenu-switching",
            crop_kind: "surface",
            viewport: viewport.name,
            sequence: sequence.key,
            target: step.target,
            crop_path: cropPath,
            source_screenshot: screenshot,
            metrics: normalizeMetrics(rawMetrics),
            failures,
            ready: failures.length === 0,
          });
        }
        for (const [index, item] of (step.audit?.item_details || []).entries()) {
          const cropPath = path.join(cropDir, `${sanitize(viewport.name)}-${sanitize(sequence.key)}-${sanitize(step.target)}-item-${index}-${sanitize(item.text || "item")}.png`);
          const rawMetrics = await cropFromScreenshot(metricPage, screenshot, item, cropPath);
          const failures = failuresFor("action", rawMetrics);
          results.push({
            source: "control-v9-submenu-switching",
            crop_kind: "action",
            viewport: viewport.name,
            sequence: sequence.key,
            target: step.target,
            label: item.text || "",
            crop_path: cropPath,
            source_screenshot: screenshot,
            metrics: normalizeMetrics(rawMetrics),
            failures,
            ready: failures.length === 0,
          });
        }
      }
    }
  }
  return results;
}

async function collectNativeCrops(browser, metricPage, native) {
  const results = [];
  const htmlPath = native.html;
  if (!htmlPath || !fs.existsSync(htmlPath)) {
    return [{
      source: "native-secondary-surfaces",
      crop_kind: "missing-html",
      ready: false,
      failures: ["missing_native_fixture_html"],
    }];
  }
  const cases = native.secondary_product_surfaces?.results || [];
  for (const item of cases) {
    const viewport = item.viewport || { width: 390, height: 844 };
    const page = await browser.newPage({
      viewport: { width: viewport.width, height: viewport.height },
      deviceScaleFactor: 1,
    });
    try {
      const url = `${pathToFileURL(htmlPath).href}?route=Home&selected=0&surface=${encodeURIComponent(item.surface)}`;
      await page.goto(url, { waitUntil: "load" });
      await page.waitForTimeout(320);
      const surfaceLocator = page.locator(`[data-secondary-surface="${item.surface}"]`).first();
      const surfacePath = path.join(cropDir, `native-${viewport.width}x${viewport.height}-${sanitize(item.surface)}-surface.png`);
      await surfaceLocator.screenshot({ path: surfacePath });
      const surfaceMetrics = await metricsForPng(metricPage, surfacePath);
      const surfaceFailures = failuresFor("surface", surfaceMetrics);
      results.push({
        source: "native-secondary-surfaces",
        crop_kind: "surface",
        viewport: `${viewport.width}x${viewport.height}`,
        surface: item.surface,
        crop_path: surfacePath,
        metrics: normalizeMetrics(surfaceMetrics),
        failures: surfaceFailures,
        ready: surfaceFailures.length === 0,
      });

      const actionLocators = await page.locator(`[data-secondary-surface="${item.surface}"] .surface-actions button[data-secondary-action]`).all();
      for (const [index, locator] of actionLocators.entries()) {
        const actionName = await locator.getAttribute("data-secondary-action") || `action-${index}`;
        const actionPath = path.join(cropDir, `native-${viewport.width}x${viewport.height}-${sanitize(item.surface)}-action-${index}-${sanitize(actionName)}.png`);
        await locator.screenshot({ path: actionPath });
        const actionMetrics = await metricsForPng(metricPage, actionPath);
        const actionFailures = failuresFor("action", actionMetrics);
        results.push({
          source: "native-secondary-surfaces",
          crop_kind: "action",
          viewport: `${viewport.width}x${viewport.height}`,
          surface: item.surface,
          action: actionName,
          crop_path: actionPath,
          metrics: normalizeMetrics(actionMetrics),
          failures: actionFailures,
          ready: actionFailures.length === 0,
        });
      }
    } catch (error) {
      results.push({
        source: "native-secondary-surfaces",
        crop_kind: "case-error",
        viewport: `${viewport.width}x${viewport.height}`,
        surface: item.surface,
        ready: false,
        failures: ["native_crop_capture_failed"],
        error: String(error?.message || error),
      });
    } finally {
      await page.close();
    }
  }
  return results;
}

function summarize(results) {
  const totals = {
    crop_count: results.length,
    failure_count: results.filter((item) => !item.ready).length,
    by_source: {},
    by_kind: {},
  };
  for (const item of results) {
    totals.by_source[item.source] ||= { source: item.source, crop_count: 0, failure_count: 0 };
    totals.by_source[item.source].crop_count += 1;
    if (!item.ready) totals.by_source[item.source].failure_count += 1;
    totals.by_kind[item.crop_kind] ||= { crop_kind: item.crop_kind, crop_count: 0, failure_count: 0 };
    totals.by_kind[item.crop_kind].crop_count += 1;
    if (!item.ready) totals.by_kind[item.crop_kind].failure_count += 1;
  }
  totals.by_source = Object.values(totals.by_source).sort((a, b) => a.source.localeCompare(b.source));
  totals.by_kind = Object.values(totals.by_kind).sort((a, b) => a.crop_kind.localeCompare(b.crop_kind));
  return totals;
}

async function main() {
  const v9 = readJson(v9ReportPath);
  const native = readJson(nativeReportPath);
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
  const metricPage = await browser.newPage({ viewport: { width: 16, height: 16 }, deviceScaleFactor: 1 });
  const controlCrops = await collectControlCrops(metricPage, v9);
  const nativeCrops = await collectNativeCrops(browser, metricPage, native);
  await metricPage.close();
  await browser.close();

  const crops = [...controlCrops, ...nativeCrops];
  const summary = summarize(crops);
  const sourceCounts = Object.fromEntries(summary.by_source.map((item) => [item.source, item.crop_count]));
  const kindCounts = Object.fromEntries(summary.by_kind.map((item) => [item.crop_kind, item.crop_count]));
  const expectedReady = summary.failure_count === 0
    && sourceCounts["control-v9-submenu-switching"] === 152
    && sourceCounts["native-secondary-surfaces"] === 72
    && kindCounts.surface === 59
    && kindCounts.action === 165;
  const report = {
    schema_version: "hepta-ui-harsh-top-design-referee-v10-local-crop-glass-census/v0",
    standards_version: "2026-06-27-local-crop-submenu-native-surface-pixel-glass-census",
    status: expectedReady ? "ready" : "failed",
    thresholds: {
      crop_min_width: 44,
      crop_min_height: 44,
      mean_luma: "216..248",
      luma_stddev_min: 20,
      luma_p95_min: 245,
      highlight_ratio_min: 0.055,
      chromatic_ratio_min: 0.18,
      texture_delta_min: 4.0,
      mean_saturation_max: 0.13,
      dark_ratio_max: 0.09,
      glass_white_ratio_min: 0.86,
    },
    crop_dir: cropDir,
    crop_count: summary.crop_count,
    failure_count: summary.failure_count,
    by_source: summary.by_source,
    by_kind: summary.by_kind,
    expected_counts: {
      control_v9_crop_count: 152,
      native_secondary_surface_crop_count: 72,
      surface_crop_count: 59,
      action_crop_count: 165,
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

local_crop_sha="$(shasum -a 256 "$LOCAL_CROP_REPORT_PATH" | awk '{print $1}')"
v9_sha="$(shasum -a 256 "$V9_REPORT_PATH" | awk '{print $1}')"
native_sha="$(shasum -a 256 "$NATIVE_REPORT_PATH" | awk '{print $1}')"
tmp_report="$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v10-final.XXXXXX")"

jq -n \
  --arg v9_path "$V9_REPORT_PATH" \
  --arg native_path "$NATIVE_REPORT_PATH" \
  --arg local_crop_path "$LOCAL_CROP_REPORT_PATH" \
  --arg v9_sha "$v9_sha" \
  --arg native_sha "$native_sha" \
  --arg local_crop_sha "$local_crop_sha" \
  --slurpfile v9_file "$V9_REPORT_PATH" \
  --slurpfile local_crop_file "$LOCAL_CROP_REPORT_PATH" '
  ($v9_file[0]) as $v9
  | ($local_crop_file[0]) as $local_crop
  | def v9_ready:
      $v9.status == "ready"
      and $v9.v8_ready == true
      and $v9.switching_ready == true
      and $v9.summary.control_submenu_switching.failure_count == 0;
    def local_crop_ready:
      $local_crop.status == "ready"
      and $local_crop.failure_count == 0
      and $local_crop.crop_count == 224
      and (($local_crop.by_source // []) | any(.source == "control-v9-submenu-switching" and .crop_count == 152 and .failure_count == 0))
      and (($local_crop.by_source // []) | any(.source == "native-secondary-surfaces" and .crop_count == 72 and .failure_count == 0))
      and (($local_crop.by_kind // []) | any(.crop_kind == "surface" and .crop_count == 59 and .failure_count == 0))
      and (($local_crop.by_kind // []) | any(.crop_kind == "action" and .crop_count == 165 and .failure_count == 0));
    {
      schema_version:"hepta-ui-harsh-top-design-referee-v10-gate/v0",
      standards_version:"2026-06-27-harsh-v9-plus-local-crop-pixel-glass-census",
      status:(if (v9_ready and local_crop_ready) then "ready" else "failed" end),
      inputs:{
        v9_switching:{path:$v9_path, sha256:$v9_sha},
        native_fixture:{path:$native_path, sha256:$native_sha},
        local_crop_glass_census:{path:$local_crop_path, sha256:$local_crop_sha}
      },
      summary:{
        control_visual_matrix:$v9.summary.control_visual_matrix,
        control_button_census:$v9.summary.control_button_census,
        native_fixture:$v9.summary.native_fixture,
        native_detail_census:$v9.summary.native_detail_census,
        pixel_glass_census:$v9.summary.pixel_glass_census,
        control_real_click_activation:$v9.summary.control_real_click_activation,
        control_submenu_lifecycle:$v9.summary.control_submenu_lifecycle,
        control_submenu_switching:$v9.summary.control_submenu_switching,
        local_crop_glass_census:{
          crop_count:$local_crop.crop_count,
          failure_count:$local_crop.failure_count,
          by_source:$local_crop.by_source,
          by_kind:$local_crop.by_kind,
          thresholds:$local_crop.thresholds
        }
      },
      v9_ready:v9_ready,
      local_crop_glass_ready:local_crop_ready,
      local_crop_glass_census:$local_crop
    }
  ' >"$tmp_report"

cp "$tmp_report" "$REPORT_PATH"
rm -f "$tmp_report"
cat "$REPORT_PATH"

if [[ "$(jq -r '.status' "$REPORT_PATH")" != "ready" ]]; then
  echo "Hepta UI harsh top-design referee v10 local crop glass gate failed" >&2
  exit 1
fi

echo "$REPORT_PATH"
