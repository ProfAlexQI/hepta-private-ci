#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V35_REPORT_PATH:-}"
V35_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V35_CENSUS_PATH:-}"
V34_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V34_REPORT_PATH:-}"
V34_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V34_CENSUS_PATH:-}"
V34_CROP_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V34_CROP_DIR:-}"
V34_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V35_V34_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
SKIP_V34="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V35_SKIP_V34:-0}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v35-micro-polish-gate.json"
fi
if [[ -z "$V35_CENSUS_PATH" ]]; then
  V35_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v35-micro-polish-census.json"
fi
if [[ -z "$V34_REPORT_PATH" ]]; then
  V34_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v34-optical-crop-gate.json"
fi
if [[ -z "$V34_CENSUS_PATH" ]]; then
  V34_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v34-optical-crop-census.json"
fi
if [[ -z "$V34_CROP_DIR" ]]; then
  V34_CROP_DIR="$READINESS_DIR/ui-harsh-v34-optical-crops"
fi
if [[ -z "$V34_LOG" ]]; then
  V34_LOG="$READINESS_DIR/v34-optical-crop-prerequisite.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V35_CENSUS_PATH")"

if [[ "$SKIP_V34" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V34_REPORT_PATH="$V34_REPORT_PATH" \
    HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V34_CENSUS_PATH="$V34_CENSUS_PATH" \
    HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V34_CROP_DIR="$V34_CROP_DIR" \
    bash scripts/hepta-ui-harsh-top-design-referee-v34-optical-crop-gate.sh "$READINESS_DIR" >"$V34_LOG" 2>&1 || {
      echo "v34 optical-crop prerequisite failed" >&2
      tail -n 180 "$V34_LOG" >&2 || true
      exit 1
    }
fi

if [[ ! -f "$V34_REPORT_PATH" || ! -f "$V34_CENSUS_PATH" || ! -d "$V34_CROP_DIR" ]]; then
  echo "missing v34 optical-crop evidence under $READINESS_DIR" >&2
  exit 1
fi
if [[ "$(jq -r '.status' "$V34_REPORT_PATH")" != "ready" ]]; then
  echo "v34 optical-crop prerequisite was not ready: $V34_REPORT_PATH" >&2
  exit 1
fi

node - "$CHROME_BIN" "$V34_REPORT_PATH" "$V34_CENSUS_PATH" "$V34_CROP_DIR" >"$V35_CENSUS_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, v34ReportPath, v34CensusPath, cropDir] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const round = (value, digits = 4) => Number(Number(value || 0).toFixed(digits));

const scenarios = [
  "desktop-optical-crop",
  "narrow-touch-optical-crop",
  "mobile-optical-crop",
  "phone320-optical-crop",
];

const thresholds = {
  control: {
    min_width: 44,
    min_height: 44,
    min_bytes: 500,
    mean_luma_min: 208,
    mean_luma_max: 252.5,
    luma_p95_min: 238,
    glass_white_ratio_min: 0.79,
    dark_ratio_max: 0.055,
    mean_saturation_max: 0.19,
    texture_delta_min: 1.9,
    edge_luma_p95_min: 238,
    edge_highlight_ratio_min: 0.055,
    edge_dark_ratio_max: 0.10,
    overbright_ratio_max: 0.74,
  },
  input: {
    min_width: 44,
    min_height: 44,
    min_bytes: 700,
    mean_luma_min: 232,
    mean_luma_max: 252.5,
    luma_p95_min: 243,
    glass_white_ratio_min: 0.87,
    dark_ratio_max: 0.06,
    mean_saturation_max: 0.11,
    texture_delta_min: 1.5,
    edge_luma_p95_min: 251,
    edge_highlight_ratio_min: 0.60,
    edge_dark_ratio_max: 0.08,
    overbright_ratio_max: 0.76,
  },
  "menu-item": {
    min_width: 44,
    min_height: 32,
    min_bytes: 700,
    mean_luma_min: 216,
    mean_luma_max: 252.5,
    luma_p95_min: 250,
    glass_white_ratio_min: 0.87,
    dark_ratio_max: 0.10,
    mean_saturation_max: 0.125,
    texture_delta_min: 3.0,
    edge_luma_p95_min: 253.5,
    edge_highlight_ratio_min: 0.62,
    edge_dark_ratio_max: 0.12,
    overbright_ratio_max: 0.78,
  },
  "micro-surface": {
    min_width: 16,
    min_height: 16,
    min_bytes: 400,
    mean_luma_min: 207,
    mean_luma_max: 252.5,
    luma_p95_min: 250,
    glass_white_ratio_min: 0.82,
    dark_ratio_max: 0.15,
    mean_saturation_max: 0.15,
    texture_delta_min: 6.0,
    edge_luma_p95_min: 250,
    edge_highlight_ratio_min: 0.44,
    edge_dark_ratio_max: 0.18,
    overbright_ratio_max: 0.78,
  },
  panel: {
    min_width: 120,
    min_height: 44,
    min_bytes: 1000,
    mean_luma_min: 226,
    mean_luma_max: 252.5,
    luma_p95_min: 249,
    glass_white_ratio_min: 0.91,
    dark_ratio_max: 0.06,
    mean_saturation_max: 0.10,
    texture_delta_min: 6.0,
    edge_luma_p95_min: 249,
    edge_highlight_ratio_min: 0.55,
    edge_dark_ratio_max: 0.08,
    overbright_ratio_max: 0.76,
  },
};

function parseCrop(file) {
  const basename = path.basename(file);
  const stem = basename.replace(/\.png$/i, "");
  const scenario = scenarios.find((name) => stem.startsWith(`${name}-`));
  if (!scenario) {
    return { file, parse_error: "unknown_scenario_prefix" };
  }
  const rest = stem.slice(scenario.length + 1);
  const match = rest.match(/^(.*)-(menu-item|micro-surface|panel|input|control)-([0-9]+)-(.+)$/);
  if (!match) {
    return { file, scenario, parse_error: "missing_state_category_index_label" };
  }
  return {
    file,
    scenario,
    state: match[1],
    category: match[2],
    index: Number(match[3]),
    label: match[4],
  };
}

function listPngs(dir) {
  return fs.readdirSync(dir)
    .filter((name) => name.toLowerCase().endsWith(".png"))
    .map((name) => path.join(dir, name))
    .sort();
}

function expectedMap(items, countKey) {
  return Object.fromEntries((items || []).map((item) => [item.category || item.state, item[countKey] || 0]));
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
    const step = Math.max(1, Math.ceil(Math.sqrt((canvas.width * canvas.height) / 12000)));
    const band = Math.max(2, Math.min(10, Math.floor(Math.min(canvas.width, canvas.height) * 0.12)));
    const lumas = [];
    const edgeLumas = [];
    const centerLumas = [];
    let highlightCount = 0;
    let overbrightCount = 0;
    let darkCount = 0;
    let glassWhiteCount = 0;
    let saturationSum = 0;
    let textureSum = 0;
    let textureCount = 0;
    let edgeDarkCount = 0;
    let edgeHighlightCount = 0;
    let cyanEdgeCount = 0;

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
        const isEdge = x < band || y < band || x >= canvas.width - band || y >= canvas.height - band;
        lumas.push(luma);
        (isEdge ? edgeLumas : centerLumas).push(luma);
        saturationSum += saturation;
        if (luma >= 245) highlightCount += 1;
        if (luma >= 253) overbrightCount += 1;
        if (luma <= 92) darkCount += 1;
        if (luma >= 178 && saturation <= 0.30) glassWhiteCount += 1;
        if (isEdge) {
          if (luma <= 92) edgeDarkCount += 1;
          if (luma >= 238) edgeHighlightCount += 1;
          if (g - r >= 4 && b - r >= 4 && g >= 168 && b >= 168 && luma >= 170 && saturation <= 0.26) cyanEdgeCount += 1;
        }
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

    const summarize = (values) => {
      const sorted = [...values].sort((a, b) => a - b);
      const count = sorted.length || 1;
      const mean = sorted.reduce((sum, value) => sum + value, 0) / count;
      const variance = sorted.reduce((sum, value) => sum + ((value - mean) ** 2), 0) / count;
      const percentile = (ratio) => sorted[Math.min(count - 1, Math.max(0, Math.floor(count * ratio)))] || 0;
      return { mean, stddev: Math.sqrt(variance), p05: percentile(0.05), p50: percentile(0.50), p95: percentile(0.95) };
    };

    const all = summarize(lumas);
    const edge = summarize(edgeLumas);
    const center = summarize(centerLumas.length ? centerLumas : lumas);
    const sampleCount = lumas.length || 1;
    const edgeCount = edgeLumas.length || 1;
    return {
      width: canvas.width,
      height: canvas.height,
      sample_count: sampleCount,
      mean_luma: all.mean,
      luma_stddev: all.stddev,
      luma_p05: all.p05,
      luma_p50: all.p50,
      luma_p95: all.p95,
      highlight_ratio: highlightCount / sampleCount,
      overbright_ratio: overbrightCount / sampleCount,
      dark_ratio: darkCount / sampleCount,
      glass_white_ratio: glassWhiteCount / sampleCount,
      mean_saturation: saturationSum / sampleCount,
      texture_delta: textureSum / Math.max(1, textureCount),
      edge_mean_luma: edge.mean,
      edge_luma_p95: edge.p95,
      edge_dark_ratio: edgeDarkCount / edgeCount,
      edge_highlight_ratio: edgeHighlightCount / edgeCount,
      cyan_edge_ratio: cyanEdgeCount / edgeCount,
      center_mean_luma: center.mean,
      edge_center_luma_delta: edge.mean - center.mean,
    };
  }, { data });
  return Object.fromEntries(Object.entries({ ...metrics, bytes, sha256: sha256(file) }).map(([key, value]) => [
    key,
    typeof value === "number" ? round(value, key.endsWith("ratio") || key === "mean_saturation" ? 4 : 3) : value,
  ]));
}

function metricFailures(category, metrics) {
  const threshold = thresholds[category];
  const failures = [];
  if (!threshold) {
    return ["unknown_crop_category"];
  }
  const checks = [
    ["width", "<", threshold.min_width, "crop_width_below_referee_minimum"],
    ["height", "<", threshold.min_height, "crop_height_below_referee_minimum"],
    ["bytes", "<", threshold.min_bytes, "crop_byte_weight_too_low_for_real_visual_evidence"],
    ["mean_luma", "<", threshold.mean_luma_min, "mean_luma_too_low_for_light_glass"],
    ["mean_luma", ">", threshold.mean_luma_max, "mean_luma_too_high_flat_white"],
    ["luma_p95", "<", threshold.luma_p95_min, "highlight_band_too_weak"],
    ["glass_white_ratio", "<", threshold.glass_white_ratio_min, "light_glass_area_ratio_too_low"],
    ["dark_ratio", ">", threshold.dark_ratio_max, "dark_pixels_too_dominant"],
    ["mean_saturation", ">", threshold.mean_saturation_max, "palette_too_saturated_for_clear_glass"],
    ["texture_delta", "<", threshold.texture_delta_min, "tempered_texture_delta_too_low"],
    ["edge_luma_p95", "<", threshold.edge_luma_p95_min, "edge_highlight_luma_too_low"],
    ["edge_highlight_ratio", "<", threshold.edge_highlight_ratio_min, "edge_highlight_ratio_too_low"],
    ["edge_dark_ratio", ">", threshold.edge_dark_ratio_max, "edge_dark_ratio_too_high"],
    ["overbright_ratio", ">", threshold.overbright_ratio_max, "overbright_flat_white_ratio_too_high"],
  ];
  for (const [metric, op, expected, reason] of checks) {
    const value = metrics[metric];
    if ((op === "<" && value < expected) || (op === ">" && value > expected)) {
      failures.push({ reason, metric, value, expected: `${op} ${expected}` });
    }
  }
  return failures;
}

function groupSummary(crops, groupKey) {
  return Object.values(crops.reduce((acc, item) => {
    const key = item[groupKey];
    acc[key] ||= { [groupKey]: key, crop_count: 0, failure_count: 0 };
    acc[key].crop_count += 1;
    if (item.failures.length > 0) acc[key].failure_count += 1;
    return acc;
  }, {})).sort((a, b) => String(a[groupKey]).localeCompare(String(b[groupKey])));
}

function minimaByCategory(crops) {
  const fields = [
    "mean_luma",
    "luma_p95",
    "glass_white_ratio",
    "texture_delta",
    "edge_luma_p95",
    "edge_highlight_ratio",
    "cyan_edge_ratio",
  ];
  const maximumFields = ["dark_ratio", "mean_saturation", "overbright_ratio"];
  const result = {};
  for (const crop of crops) {
    result[crop.category] ||= { category: crop.category, crop_count: 0 };
    const bucket = result[crop.category];
    bucket.crop_count += 1;
    for (const field of fields) {
      bucket[`${field}_min`] = bucket[`${field}_min`] === undefined ? crop.metrics[field] : Math.min(bucket[`${field}_min`], crop.metrics[field]);
    }
    for (const field of maximumFields) {
      bucket[`${field}_max`] = bucket[`${field}_max`] === undefined ? crop.metrics[field] : Math.max(bucket[`${field}_max`], crop.metrics[field]);
    }
  }
  return Object.values(result).sort((a, b) => a.category.localeCompare(b.category));
}

(async () => {
  const v34Report = readJson(v34ReportPath);
  const v34Census = readJson(v34CensusPath);
  const v34Summary = v34Report.summary?.v34_optical_crop_referee || {};
  const expectedCategories = expectedMap(v34Summary.by_category, "crop_count");
  const expectedStates = expectedMap(v34Summary.by_state, "crop_count");
  const files = listPngs(cropDir);
  const parsed = files.map(parseCrop);
  const parseFailures = parsed.filter((item) => item.parse_error).map((item) => ({
    reason: item.parse_error,
    crop_path: item.file,
  }));

  const browser = await chromium.launch({
    executablePath: chromeBin,
    headless: true,
    args: ["--no-sandbox", "--disable-gpu", "--font-render-hinting=none"],
  });

  const crops = [];
  try {
    const context = await browser.newContext({ viewport: { width: 320, height: 240 }, deviceScaleFactor: 1 });
    const page = await context.newPage();
    for (const item of parsed.filter((entry) => !entry.parse_error)) {
      const metrics = await metricsForPng(page, item.file);
      const failures = metricFailures(item.category, metrics);
      crops.push({
        scenario: item.scenario,
        state: item.state,
        category: item.category,
        index: item.index,
        label: item.label,
        crop_path: item.file,
        metrics,
        failures,
        ready: failures.length === 0,
      });
    }
    await context.close();
  } finally {
    await browser.close();
  }

  const byCategory = groupSummary(crops, "category");
  const byState = groupSummary(crops, "state");
  const byScenario = groupSummary(crops, "scenario");
  const countFailures = [];
  const actualCategoryCounts = Object.fromEntries(byCategory.map((item) => [item.category, item.crop_count]));
  const actualStateCounts = Object.fromEntries(byState.map((item) => [item.state, item.crop_count]));
  for (const [category, expected] of Object.entries(expectedCategories)) {
    if (actualCategoryCounts[category] !== expected) {
      countFailures.push({ reason: "category_crop_count_mismatch", category, expected, actual: actualCategoryCounts[category] || 0 });
    }
  }
  for (const [state, expected] of Object.entries(expectedStates)) {
    if (actualStateCounts[state] !== expected) {
      countFailures.push({ reason: "state_crop_count_mismatch", state, expected, actual: actualStateCounts[state] || 0 });
    }
  }
  if (files.length !== Number(v34Summary.optical_crop_count || 0)) {
    countFailures.push({ reason: "total_crop_count_mismatch", expected: v34Summary.optical_crop_count, actual: files.length });
  }
  if (byScenario.length !== Number(v34Summary.scenario_count || 0)) {
    countFailures.push({ reason: "scenario_count_mismatch", expected: v34Summary.scenario_count, actual: byScenario.length });
  }
  if ((v34Census.screenshot_evidence || []).length < Number(v34Summary.screenshot_count || 30)) {
    countFailures.push({ reason: "v34_screenshot_evidence_count_below_expected", expected: v34Summary.screenshot_count || 30, actual: (v34Census.screenshot_evidence || []).length });
  }

  const metricFailureCrops = crops.filter((item) => item.failures.length > 0);
  const smallControlCount = crops.filter((item) => item.category === "control" && item.metrics.width <= 170 && item.metrics.height <= 170).length;
  if (smallControlCount < 120) {
    countFailures.push({ reason: "small_button_crop_coverage_below_minimum", expected: ">= 120", actual: smallControlCount });
  }

  const failures = [
    ...parseFailures,
    ...countFailures,
    ...metricFailureCrops.map((crop) => ({
      reason: "crop_failed_micro_polish_threshold",
      crop_path: crop.crop_path,
      scenario: crop.scenario,
      state: crop.state,
      category: crop.category,
      label: crop.label,
      failures: crop.failures,
      metrics: crop.metrics,
    })),
  ];

  const weakestCrops = [...crops]
    .sort((a, b) => {
      const aScore = a.metrics.edge_highlight_ratio + (a.metrics.luma_p95 / 255) + a.metrics.glass_white_ratio - a.metrics.dark_ratio;
      const bScore = b.metrics.edge_highlight_ratio + (b.metrics.luma_p95 / 255) + b.metrics.glass_white_ratio - b.metrics.dark_ratio;
      return aScore - bScore;
    })
    .slice(0, 24)
    .map((crop) => ({
      scenario: crop.scenario,
      state: crop.state,
      category: crop.category,
      label: crop.label,
      crop_path: crop.crop_path,
      metrics: crop.metrics,
    }));

  const summary = {
    v34_status: v34Report.status,
    scenario_count: byScenario.length,
    screenshot_count: (v34Census.screenshot_evidence || []).length,
    crop_count: crops.length,
    small_control_crop_count: smallControlCount,
    parse_failure_count: parseFailures.length,
    count_failure_count: countFailures.length,
    micro_polish_failure_count: metricFailureCrops.length,
    failure_count: failures.length,
    by_category: byCategory,
    by_state: byState,
    by_scenario: byScenario,
    category_worst_metrics: minimaByCategory(crops),
    thresholds,
    browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome was used.",
  };

  const result = {
    schema: "hepta-ui-harsh-top-design-referee-v35-micro-polish-census/v1",
    status: failures.length === 0 ? "ready" : "failed",
    generated_at: new Date().toISOString(),
    inputs: {
      v34_report_path: v34ReportPath,
      v34_census_path: v34CensusPath,
      v34_crop_dir: cropDir,
      chrome_bin: chromeBin,
    },
    summary,
    failures,
    weakest_crops: weakestCrops,
    crops,
  };
  process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
})().catch((error) => {
  const result = {
    schema: "hepta-ui-harsh-top-design-referee-v35-micro-polish-census/v1",
    status: "failed",
    generated_at: new Date().toISOString(),
    summary: { failure_count: 1 },
    failures: [{ reason: "v35_micro_polish_exception", message: String(error && error.stack || error) }],
  };
  process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
  process.exitCode = 1;
});
NODE

node - "$V34_REPORT_PATH" "$V35_CENSUS_PATH" >"$REPORT_PATH" <<'NODE'
const fs = require("node:fs");
const [v34ReportPath, v35CensusPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const v34 = readJson(v34ReportPath);
const v35 = readJson(v35CensusPath);
const failures = [];
if (v34.status !== "ready") {
  failures.push({ reason: "v34_prerequisite_not_ready", status: v34.status, path: v34ReportPath });
}
if (v35.status !== "ready") {
  failures.push(...(v35.failures || []).slice(0, 200));
}
const report = {
  schema: "hepta-ui-harsh-top-design-referee-v35-micro-polish-gate/v1",
  status: failures.length === 0 ? "ready" : "failed",
  generated_at: new Date().toISOString(),
  summary: {
    v34_optical_crop_referee: v34.summary?.v34_optical_crop_referee || null,
    v35_micro_polish_referee: v35.summary || null,
  },
  failures,
};
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
if (failures.length > 0) process.exitCode = 1;
NODE

status="$(jq -r '.status' "$REPORT_PATH")"
failure_count="$(jq -r '.summary.v35_micro_polish_referee.failure_count // 0' "$REPORT_PATH")"
if [[ "$status" != "ready" ]]; then
  echo "v35 micro-polish referee failed with ${failure_count} failures: $REPORT_PATH" >&2
  exit 1
fi

echo "v35 micro-polish referee ready: $REPORT_PATH"
