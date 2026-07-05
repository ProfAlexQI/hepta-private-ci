#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V23_REPORT_PATH:-}"
V23_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V23_CENSUS_PATH:-}"
V22_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V22_REPORT_PATH:-}"
V22_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V23_V22_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
SKIP_V22="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V23_SKIP_V22:-0}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v23-evidence-glass-referee-gate.json"
fi
if [[ -z "$V23_CENSUS_PATH" ]]; then
  V23_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v23-evidence-glass-referee-census.json"
fi
if [[ -z "$V22_REPORT_PATH" ]]; then
  V22_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v22-composition-referee-gate.json"
fi
if [[ -z "$V22_LOG" ]]; then
  V22_LOG="$READINESS_DIR/v22-composition-referee-prerequisite.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V23_CENSUS_PATH")"

if [[ "$SKIP_V22" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V22_REPORT_PATH="$V22_REPORT_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v22-composition-referee-gate.sh "$READINESS_DIR" >"$V22_LOG" 2>&1 || {
      echo "v22 composition-referee prerequisite failed" >&2
      tail -n 180 "$V22_LOG" >&2 || true
      exit 1
    }

  if [[ "$(jq -r '.status' "$V22_REPORT_PATH")" != "ready" ]]; then
    echo "v22 composition-referee prerequisite was not ready: $V22_REPORT_PATH" >&2
    exit 1
  fi
fi

node - "$READINESS_DIR" "$CHROME_BIN" >"$V23_CENSUS_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [readinessDir, chromeBin] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const round = (value, digits = 3) => Number(value.toFixed(digits));

const paths = {
  v20Gate: path.join(readinessDir, "ui-harsh-top-design-referee-v20-total-design-gate.json"),
  v20Census: path.join(readinessDir, "ui-harsh-top-design-referee-v20-total-design-census.json"),
  v21Gate: path.join(readinessDir, "ui-harsh-top-design-referee-v21-readable-default-gate.json"),
  v22Gate: path.join(readinessDir, "ui-harsh-top-design-referee-v22-composition-referee-gate.json"),
  v22Census: path.join(readinessDir, "ui-harsh-top-design-referee-v22-composition-referee-census.json"),
  v20Screenshots: path.join(readinessDir, "ui-harsh-v20-total-design-screenshots"),
  v21Screenshots: path.join(readinessDir, "ui-harsh-v21-readable-default-screenshots"),
  v22Screenshots: path.join(readinessDir, "ui-harsh-v22-composition-referee-screenshots"),
};

const missingInputs = Object.entries(paths)
  .filter(([key, file]) => !key.endsWith("Screenshots") && !fs.existsSync(file))
  .map(([key, file]) => ({ key, file }));
if (missingInputs.length > 0) {
  process.stdout.write(JSON.stringify({
    schema: "hepta-ui-harsh-top-design-referee-v23-evidence-glass-referee-census/v0",
    status: "failed",
    generated_at: new Date().toISOString(),
    failures: missingInputs.map((input) => ({ code: "missing_input", ...input })),
  }, null, 2));
  process.exit(0);
}

const v20Gate = readJson(paths.v20Gate);
const v20Census = readJson(paths.v20Census);
const v21Gate = readJson(paths.v21Gate);
const v22Gate = readJson(paths.v22Gate);
const v22Census = readJson(paths.v22Census);

function cropRecords() {
  const records = [];
  for (const audit of v20Census.audits?.baseline || []) {
    if (audit.crop?.metrics) {
      records.push({
        source: "v20-baseline",
        scenario: audit.scenario,
        kind: audit.kind,
        group: null,
        label: audit.label,
        crop_path: audit.crop.crop_path,
        ready: audit.ready && audit.crop.ready,
        failures: [...(audit.failures || []), ...(audit.crop.failures || [])],
        hard_clipped: Boolean(audit.info?.hard_clipped),
        metrics: audit.crop.metrics,
      });
    }
  }
  for (const menu of v20Census.audits?.menus || []) {
    if (menu.panel?.crop?.metrics) {
      records.push({
        source: "v20-opened-panel",
        scenario: menu.scenario,
        kind: "opened-panel",
        group: menu.group,
        label: menu.target,
        crop_path: menu.panel.crop.crop_path,
        ready: menu.panel.ready && menu.panel.crop.ready,
        failures: [...(menu.panel.failures || []), ...(menu.panel.crop.failures || [])],
        hard_clipped: Boolean(menu.panel.info?.hard_clipped),
        metrics: menu.panel.crop.metrics,
      });
    }
    for (const item of menu.items || []) {
      if (item.crop?.metrics) {
        records.push({
          source: "v20-opened-menu-item",
          scenario: menu.scenario,
          kind: "opened-menu-item",
          group: menu.group,
          label: item.label,
          crop_path: item.crop.crop_path,
          ready: item.ready && item.crop.ready,
          failures: [...(item.failures || []), ...(item.crop.failures || [])],
          hard_clipped: Boolean(item.info?.hard_clipped),
          metrics: item.crop.metrics,
        });
      }
    }
  }
  return records;
}

const cropThresholds = {
  mean_luma_min: 205,
  luma_p95_min: 240,
  dark_ratio_max: 0.10,
  glass_white_ratio_min: 0.74,
  mean_saturation_max: 0.14,
  texture_delta_or_stddev_min: "texture_delta >= 1.0 or luma_stddev >= 3.0",
};

function strictCropFailures(record) {
  const metrics = record.metrics;
  const failures = [];
  if (!record.ready) failures.push("predecessor_crop_not_ready");
  if (record.hard_clipped) failures.push("hard_text_or_surface_clipped");
  if (metrics.mean_luma < cropThresholds.mean_luma_min) failures.push("crop_not_bright_enough_for_v23_light_glass");
  if (metrics.luma_p95 < cropThresholds.luma_p95_min) failures.push("crop_highlights_not_bright_enough_for_v23");
  if (metrics.dark_ratio > cropThresholds.dark_ratio_max) failures.push("crop_dark_plate_ratio_too_high_for_v23");
  if (metrics.glass_white_ratio < cropThresholds.glass_white_ratio_min) failures.push("crop_light_glass_area_too_low_for_v23");
  if (metrics.mean_saturation > cropThresholds.mean_saturation_max) failures.push("crop_too_saturated_for_v23_tempered_glass");
  if (metrics.texture_delta < 1.0 && metrics.luma_stddev < 3.0) failures.push("crop_too_flat_for_v23_tempered_surface");
  return failures;
}

function screenshotFiles() {
  const dirs = [
    ["v20", paths.v20Screenshots],
    ["v21", paths.v21Screenshots],
    ["v22", paths.v22Screenshots],
  ];
  return dirs.flatMap(([source, dir]) => {
    if (!fs.existsSync(dir)) return [];
    return fs.readdirSync(dir)
      .filter((name) => name.endsWith(".png"))
      .sort()
      .map((name) => ({ source, file: path.join(dir, name), label: name }));
  });
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
    const step = Math.max(1, Math.ceil(Math.sqrt((canvas.width * canvas.height) / 10000)));
    const lumas = [];
    let darkCount = 0;
    let glassWhiteCount = 0;
    let saturationSum = 0;
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
        if (luma <= 95) darkCount += 1;
        if (luma >= 180 && saturation <= 0.34) glassWhiteCount += 1;
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
      luma_p95: percentile(0.95),
      dark_ratio: darkCount / sampleCount,
      glass_white_ratio: glassWhiteCount / sampleCount,
      mean_saturation: saturationSum / sampleCount,
    };
  }, { data });
  return Object.fromEntries(Object.entries({ ...metrics, bytes, sha256: sha256(file) }).map(([key, value]) => [
    key,
    typeof value === "number" && !Number.isInteger(value) ? round(value) : value,
  ]));
}

const screenshotThresholds = {
  mean_luma_min: 228,
  luma_p95_min: 250,
  dark_ratio_max: 0.05,
  glass_white_ratio_min: 0.92,
  mean_saturation_max: 0.08,
};

function screenshotFailures(record) {
  const metrics = record.metrics;
  const failures = [];
  if (metrics.mean_luma < screenshotThresholds.mean_luma_min) failures.push("screenshot_not_light_enough_for_2026_glass");
  if (metrics.luma_p95 < screenshotThresholds.luma_p95_min) failures.push("screenshot_missing_bright_tempered_highlights");
  if (metrics.dark_ratio > screenshotThresholds.dark_ratio_max) failures.push("screenshot_dark_area_too_high");
  if (metrics.glass_white_ratio < screenshotThresholds.glass_white_ratio_min) failures.push("screenshot_light_glass_area_too_low");
  if (metrics.mean_saturation > screenshotThresholds.mean_saturation_max) failures.push("screenshot_palette_too_saturated");
  return failures;
}

function quantiles(records, key) {
  const values = records.map((record) => record.metrics?.[key]).filter((value) => typeof value === "number").sort((a, b) => a - b);
  if (values.length === 0) return null;
  const at = (ratio) => round(values[Math.min(values.length - 1, Math.max(0, Math.floor(values.length * ratio)))]);
  return { min: at(0), p05: at(0.05), p50: at(0.5), p95: at(0.95), max: at(0.999) };
}

function grouped(records, key) {
  return Object.values(records.reduce((acc, record) => {
    const group = record[key] || "none";
    acc[group] ||= { [key]: group, count: 0, failure_count: 0 };
    acc[group].count += 1;
    acc[group].failure_count += record.v23_failures.length;
    return acc;
  }, {}));
}

function compositionFailures() {
  const failures = [];
  const summary = v22Census.summary || {};
  const nonModalGroups = new Set(["row-menu", "thread-tools", "composer-tools"]);
  const modalGroups = new Set(["command-palette", "composer-popover"]);
  for (const group of summary.opened_panel_by_group || []) {
    if (nonModalGroups.has(group.group) && group.text_occlusion_count !== 0) {
      failures.push({ code: "non_modal_panel_occludes_message_text", group: group.group, text_occlusion_count: group.text_occlusion_count });
    }
    if (!nonModalGroups.has(group.group) && !modalGroups.has(group.group)) {
      failures.push({ code: "unknown_panel_group_in_v22", group: group.group });
    }
  }
  const allowedModalOcclusionCount = (summary.opened_panel_by_group || [])
    .filter((group) => modalGroups.has(group.group))
    .reduce((sum, group) => sum + group.text_occlusion_count, 0);
  if (allowedModalOcclusionCount > 24) {
    failures.push({ code: "modal_overlay_text_occlusion_count_exceeds_v23_budget", allowed_modal_text_occlusion_count: allowedModalOcclusionCount, max: 24 });
  }
  if ((summary.failure_count || 0) !== 0) failures.push({ code: "v22_composition_has_failures", failure_count: summary.failure_count });
  return failures;
}

(async () => {
  const crops = cropRecords().map((record) => ({ ...record, v23_failures: strictCropFailures(record) }));
  const browser = await chromium.launch({ executablePath: chromeBin, headless: true, args: ["--no-sandbox", "--disable-gpu"] });
  const page = await browser.newPage();
  const screenshots = [];
  try {
    for (const item of screenshotFiles()) {
      const metrics = await metricsForPng(page, item.file);
      screenshots.push({ ...item, metrics, sha256: metrics.sha256 });
    }
  } finally {
    await browser.close();
  }
  const screenshotAudits = screenshots.map((record) => ({ ...record, v23_failures: screenshotFailures(record) }));

  const inputFailures = [];
  if (v20Gate.status !== "ready") inputFailures.push({ code: "v20_gate_not_ready", status: v20Gate.status });
  if (v20Census.status !== "ready") inputFailures.push({ code: "v20_census_not_ready", status: v20Census.status });
  if (v21Gate.status !== "ready") inputFailures.push({ code: "v21_gate_not_ready", status: v21Gate.status });
  if (v22Gate.status !== "ready") inputFailures.push({ code: "v22_gate_not_ready", status: v22Gate.status });
  if (v22Census.status !== "ready") inputFailures.push({ code: "v22_census_not_ready", status: v22Census.status });
  if (crops.length !== (v20Census.summary?.crop_count || 0)) inputFailures.push({ code: "v20_crop_count_mismatch", observed: crops.length, expected: v20Census.summary?.crop_count });
  if (screenshotAudits.length < 64) inputFailures.push({ code: "screenshot_evidence_count_below_v23_minimum", observed: screenshotAudits.length, expected_minimum: 64 });

  const cropFailures = crops.filter((record) => record.v23_failures.length > 0);
  const screenshotAuditFailures = screenshotAudits.filter((record) => record.v23_failures.length > 0);
  const compositionAuditFailures = compositionFailures();
  const failures = [
    ...inputFailures,
    ...cropFailures.map((record) => ({ code: "v23_crop_failure", source: record.source, kind: record.kind, group: record.group, label: record.label, failures: record.v23_failures, metrics: record.metrics, crop_path: record.crop_path })),
    ...screenshotAuditFailures.map((record) => ({ code: "v23_screenshot_failure", source: record.source, label: record.label, failures: record.v23_failures, metrics: record.metrics, path: record.file })),
    ...compositionAuditFailures,
  ];

  const summary = {
    v20_crop_audit_count: crops.length,
    v20_crop_failure_count: cropFailures.length,
    v20_crop_by_source: grouped(crops, "source"),
    v20_crop_by_kind: grouped(crops, "kind"),
    screenshot_audit_count: screenshotAudits.length,
    screenshot_failure_count: screenshotAuditFailures.length,
    screenshot_by_source: grouped(screenshotAudits, "source"),
    v22_opened_panel_audit_count: v22Census.summary?.opened_panel_audit_count || 0,
    v22_non_modal_text_occlusion_failure_count: compositionAuditFailures.filter((failure) => failure.code === "non_modal_panel_occludes_message_text").length,
    v22_allowed_modal_text_occlusion_count: (v22Census.summary?.opened_panel_by_group || [])
      .filter((group) => ["command-palette", "composer-popover"].includes(group.group))
      .reduce((sum, group) => sum + group.text_occlusion_count, 0),
    failure_count: failures.length,
    crop_metric_quantiles: {
      mean_luma: quantiles(crops, "mean_luma"),
      luma_p95: quantiles(crops, "luma_p95"),
      dark_ratio: quantiles(crops, "dark_ratio"),
      glass_white_ratio: quantiles(crops, "glass_white_ratio"),
      mean_saturation: quantiles(crops, "mean_saturation"),
    },
    screenshot_metric_quantiles: {
      mean_luma: quantiles(screenshotAudits, "mean_luma"),
      luma_p95: quantiles(screenshotAudits, "luma_p95"),
      dark_ratio: quantiles(screenshotAudits, "dark_ratio"),
      glass_white_ratio: quantiles(screenshotAudits, "glass_white_ratio"),
      mean_saturation: quantiles(screenshotAudits, "mean_saturation"),
    },
    thresholds: {
      crop: cropThresholds,
      screenshot: screenshotThresholds,
      v22_non_modal_text_occlusion_allowed: false,
      v22_modal_overlay_text_occlusion_budget_max: 24,
      screenshot_evidence_count_minimum: 64,
      browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome was used.",
    },
  };

  process.stdout.write(JSON.stringify({
    schema: "hepta-ui-harsh-top-design-referee-v23-evidence-glass-referee-census/v0",
    status: failures.length === 0 ? "ready" : "failed",
    generated_at: new Date().toISOString(),
    readiness_dir: readinessDir,
    summary,
    inputs: {
      v20_gate: { path: paths.v20Gate, sha256: sha256(paths.v20Gate) },
      v20_census: { path: paths.v20Census, sha256: sha256(paths.v20Census) },
      v21_gate: { path: paths.v21Gate, sha256: sha256(paths.v21Gate) },
      v22_gate: { path: paths.v22Gate, sha256: sha256(paths.v22Gate) },
      v22_census: { path: paths.v22Census, sha256: sha256(paths.v22Census) },
    },
    failures,
    worst_crops: {
      lowest_mean_luma: [...crops].sort((a, b) => a.metrics.mean_luma - b.metrics.mean_luma).slice(0, 10),
      highest_dark_ratio: [...crops].sort((a, b) => b.metrics.dark_ratio - a.metrics.dark_ratio).slice(0, 10),
      lowest_glass_white_ratio: [...crops].sort((a, b) => a.metrics.glass_white_ratio - b.metrics.glass_white_ratio).slice(0, 10),
    },
    worst_screenshots: {
      lowest_mean_luma: [...screenshotAudits].sort((a, b) => a.metrics.mean_luma - b.metrics.mean_luma).slice(0, 10),
      highest_dark_ratio: [...screenshotAudits].sort((a, b) => b.metrics.dark_ratio - a.metrics.dark_ratio).slice(0, 10),
      lowest_glass_white_ratio: [...screenshotAudits].sort((a, b) => a.metrics.glass_white_ratio - b.metrics.glass_white_ratio).slice(0, 10),
    },
  }, null, 2));
})().catch((error) => {
  process.stderr.write(`${error.stack || error}\n`);
  process.exit(1);
});
NODE

node - "$V23_CENSUS_PATH" "$REPORT_PATH" "$V22_REPORT_PATH" "$SKIP_V22" <<'NODE'
const crypto = require("node:crypto");
const fs = require("node:fs");

const [censusPath, reportPath, v22ReportPath, skipV22] = process.argv.slice(2);
const sha256 = (file) => fs.existsSync(file) ? crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex") : null;
const census = JSON.parse(fs.readFileSync(censusPath, "utf8"));
const v22 = fs.existsSync(v22ReportPath) ? JSON.parse(fs.readFileSync(v22ReportPath, "utf8")) : null;
const status = census.status === "ready" && (skipV22 === "1" || v22?.status === "ready") ? "ready" : "failed";
const report = {
  schema: "hepta-ui-harsh-top-design-referee-v23-evidence-glass-referee-gate/v0",
  status,
  generated_at: new Date().toISOString(),
  summary: {
    v22_composition_referee: v22?.summary?.v22_composition_referee ?? null,
    v23_evidence_glass_referee: census.summary,
  },
  inputs: {
    v22_composition_referee: fs.existsSync(v22ReportPath) ? { path: v22ReportPath, sha256: sha256(v22ReportPath), skipped: skipV22 === "1" } : { path: v22ReportPath, sha256: null, skipped: skipV22 === "1" },
    v23_evidence_glass_referee_census: { path: censusPath, sha256: sha256(censusPath) },
  },
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`);
if (status !== "ready") {
  console.error(JSON.stringify(census.summary, null, 2));
  process.exit(1);
}
NODE

echo "Hepta UI harsh top-design referee v23 evidence-glass-referee gate ready: $REPORT_PATH"
