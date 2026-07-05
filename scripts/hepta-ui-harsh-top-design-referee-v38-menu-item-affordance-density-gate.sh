#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V38_REPORT_PATH:-}"
V38_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V38_CENSUS_PATH:-}"
V37_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V37_REPORT_PATH:-}"
V35_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V35_CENSUS_PATH:-}"
V37_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V38_V37_LOG:-}"
SKIP_V37="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V38_SKIP_V37:-0}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v38-menu-item-affordance-density-gate.json"
fi
if [[ -z "$V38_CENSUS_PATH" ]]; then
  V38_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v38-menu-item-affordance-density-census.json"
fi
if [[ -z "$V37_REPORT_PATH" ]]; then
  V37_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v37-open-composer-trigger-parity-gate.json"
fi
if [[ -z "$V35_CENSUS_PATH" ]]; then
  V35_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v35-micro-polish-census.json"
fi
if [[ -z "$V37_LOG" ]]; then
  V37_LOG="$READINESS_DIR/v37-open-composer-trigger-parity-prerequisite.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V38_CENSUS_PATH")"

if [[ "$SKIP_V37" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V37_REPORT_PATH="$V37_REPORT_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v37-open-composer-trigger-parity-gate.sh "$READINESS_DIR" >"$V37_LOG" 2>&1 || {
      echo "v37 open composer trigger parity prerequisite failed" >&2
      tail -n 180 "$V37_LOG" >&2 || true
      exit 1
    }
fi

if [[ ! -f "$V37_REPORT_PATH" || ! -f "$V35_CENSUS_PATH" ]]; then
  echo "missing v37/v35 evidence under $READINESS_DIR" >&2
  exit 1
fi
if [[ "$(jq -r '.status' "$V37_REPORT_PATH")" != "ready" ]]; then
  echo "v37 open composer trigger parity prerequisite was not ready: $V37_REPORT_PATH" >&2
  exit 1
fi

V38_CENSUS_EXIT=0
node - "$CHROME_BIN" "$V37_REPORT_PATH" "$V35_CENSUS_PATH" >"$V38_CENSUS_PATH" <<'NODE' || V38_CENSUS_EXIT=$?
const { chromium } = require("playwright");
const fs = require("node:fs");

const [chromeBin, v37ReportPath, v35CensusPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const round = (value, digits = 4) => Number(Number(value || 0).toFixed(digits));

const v37Report = readJson(v37ReportPath);
const v35Census = readJson(v35CensusPath);
const crops = Array.isArray(v35Census.crops) ? v35Census.crops : [];
const menuItems = crops.filter((item) => item.category === "menu-item");

const expectedStateCounts = {
  "opened-command-palette": 24,
  "opened-composer-popover-artifact": 8,
  "opened-composer-popover-command": 8,
  "opened-composer-tools": 8,
  "opened-row-menu-operator-plane": 6,
  "opened-row-menu-task-queue": 6,
  "opened-row-menu-ui-chat-agent": 6,
  "opened-thread-tools": 12,
};

const expectedScenarioMinimums = {
  "desktop-optical-crop": 24,
  "narrow-touch-optical-crop": 24,
  "mobile-optical-crop": 12,
  "phone320-optical-crop": 12,
};

const thresholds = {
  source_crop_count: 487,
  menu_item_crop_count: 78,
  menu_item_group_count: 24,
  glass_white_ratio_min: 0.9,
  mean_saturation_max: 0.115,
  dark_ratio_max: 0.085,
  edge_highlight_ratio_min: 0.64,
  edge_luma_p95_min: 254,
  leading_dark_ratio_min: 0.009,
  leading_texture_min: 3.0,
  leading_anchor_delta_min: 1.4,
  rim_highlight_ratio_min: 0.35,
  group_glass_white_ratio_spread_max: 0.08,
  group_mean_saturation_spread_max: 0.06,
  group_leading_anchor_delta_spread_max: 42,
};

function groupBy(items, key) {
  return items.reduce((acc, item) => {
    const value = typeof key === "function" ? key(item) : item[key];
    acc[value] ||= [];
    acc[value].push(item);
    return acc;
  }, {});
}

function summarizeCounts(items, keyName) {
  return Object.entries(groupBy(items, keyName))
    .map(([key, values]) => ({
      [keyName]: key,
      crop_count: values.length,
      failure_count: values.filter((item) => (item.failures || []).length > 0).length,
    }))
    .sort((a, b) => String(a[keyName]).localeCompare(String(b[keyName])));
}

function spread(items, accessor) {
  const values = items.map(accessor).map(Number).filter(Number.isFinite);
  if (values.length === 0) return 0;
  return round(Math.max(...values) - Math.min(...values));
}

async function measureAffordance(page, cropPath) {
  const data = fs.readFileSync(cropPath).toString("base64");
  return page.evaluate(async ({ data }) => {
    const image = new Image();
    image.src = `data:image/png;base64,${data}`;
    await image.decode();
    const canvas = document.createElement("canvas");
    canvas.width = image.naturalWidth;
    canvas.height = image.naturalHeight;
    const context = canvas.getContext("2d", { willReadFrequently: true });
    context.drawImage(image, 0, 0);
    const pixels = context.getImageData(0, 0, canvas.width, canvas.height).data;
    const lumaAt = (x, y) => {
      const index = (y * canvas.width + x) * 4;
      return (0.2126 * pixels[index]) + (0.7152 * pixels[index + 1]) + (0.0722 * pixels[index + 2]);
    };
    const saturationAt = (x, y) => {
      const index = (y * canvas.width + x) * 4;
      const r = pixels[index];
      const g = pixels[index + 1];
      const b = pixels[index + 2];
      const max = Math.max(r, g, b);
      const min = Math.min(r, g, b);
      return max === 0 ? 0 : (max - min) / max;
    };
    const sampleBand = (x0, x1, y0, y1) => {
      const width = Math.max(1, x1 - x0);
      const height = Math.max(1, y1 - y0);
      const step = Math.max(1, Math.ceil(Math.sqrt((width * height) / 4000)));
      let count = 0;
      let lumaSum = 0;
      let saturationSum = 0;
      let darkCount = 0;
      let highlightCount = 0;
      let textureSum = 0;
      let textureCount = 0;
      for (let y = y0; y < y1; y += step) {
        for (let x = x0; x < x1; x += step) {
          const luma = lumaAt(x, y);
          count += 1;
          lumaSum += luma;
          saturationSum += saturationAt(x, y);
          if (luma < 125) darkCount += 1;
          if (luma > 245) highlightCount += 1;
          if (x + step < x1) {
            textureSum += Math.abs(luma - lumaAt(x + step, y));
            textureCount += 1;
          }
          if (y + step < y1) {
            textureSum += Math.abs(luma - lumaAt(x, y + step));
            textureCount += 1;
          }
        }
      }
      return {
        mean_luma: lumaSum / count,
        mean_saturation: saturationSum / count,
        dark_ratio: darkCount / count,
        highlight_ratio: highlightCount / count,
        texture_delta: textureSum / Math.max(1, textureCount),
      };
    };
    const width = canvas.width;
    const height = canvas.height;
    const y0 = Math.floor(height * 0.12);
    const y1 = Math.max(y0 + 1, Math.floor(height * 0.88));
    const leadingWidth = Math.max(48, Math.min(96, Math.floor(width * 0.18)));
    const leading = sampleBand(0, leadingWidth, y0, y1);
    const body = sampleBand(Math.floor(width * 0.32), Math.max(Math.floor(width * 0.82), Math.floor(width * 0.32) + 1), y0, y1);
    const rim = sampleBand(0, width, 0, Math.max(2, Math.floor(height * 0.16)));
    return {
      width,
      height,
      leading_dark_ratio: leading.dark_ratio,
      leading_texture_delta: leading.texture_delta,
      leading_mean_saturation: leading.mean_saturation,
      leading_body_luma_delta: Math.abs(leading.mean_luma - body.mean_luma),
      leading_body_saturation_delta: Math.abs(leading.mean_saturation - body.mean_saturation),
      rim_highlight_ratio: rim.highlight_ratio,
      rim_texture_delta: rim.texture_delta,
    };
  }, { data });
}

(async () => {
  const failures = [];
  if (v37Report.status !== "ready") {
    failures.push({ reason: "v37_prerequisite_not_ready", status: v37Report.status });
  }
  if (v35Census.status !== "ready") {
    failures.push({ reason: "v35_census_not_ready", status: v35Census.status });
  }
  if (crops.length !== thresholds.source_crop_count) {
    failures.push({ reason: "source_crop_count_changed", expected: thresholds.source_crop_count, actual: crops.length });
  }
  if (menuItems.length !== thresholds.menu_item_crop_count) {
    failures.push({ reason: "menu_item_crop_count_changed", expected: thresholds.menu_item_crop_count, actual: menuItems.length });
  }

  const browser = await chromium.launch({ executablePath: chromeBin, headless: true });
  const page = await browser.newPage();
  const audited = [];
  try {
    for (const item of menuItems) {
      const affordance = await measureAffordance(page, item.crop_path);
      const metrics = item.metrics || {};
      const audit = {
        scenario: item.scenario,
        state: item.state,
        label: item.label,
        crop_path: item.crop_path,
        metrics: {
          glass_white_ratio: round(metrics.glass_white_ratio),
          mean_saturation: round(metrics.mean_saturation),
          dark_ratio: round(metrics.dark_ratio),
          edge_highlight_ratio: round(metrics.edge_highlight_ratio),
          edge_luma_p95: round(metrics.edge_luma_p95),
          leading_dark_ratio: round(affordance.leading_dark_ratio),
          leading_texture_delta: round(affordance.leading_texture_delta),
          leading_body_luma_delta: round(affordance.leading_body_luma_delta),
          leading_body_saturation_delta: round(affordance.leading_body_saturation_delta),
          rim_highlight_ratio: round(affordance.rim_highlight_ratio),
          rim_texture_delta: round(affordance.rim_texture_delta),
          width: affordance.width,
          height: affordance.height,
        },
        failures: [],
      };
      const checks = [
        ["glass_white_ratio", thresholds.glass_white_ratio_min, ">=", "menu_item_not_bright_white_glass_enough"],
        ["mean_saturation", thresholds.mean_saturation_max, "<=", "menu_item_too_saturated_for_neutral_light_glass"],
        ["dark_ratio", thresholds.dark_ratio_max, "<=", "menu_item_dark_area_too_heavy"],
        ["edge_highlight_ratio", thresholds.edge_highlight_ratio_min, ">=", "menu_item_edge_highlight_too_weak"],
        ["edge_luma_p95", thresholds.edge_luma_p95_min, ">=", "menu_item_edge_luma_too_low"],
        ["leading_dark_ratio", thresholds.leading_dark_ratio_min, ">=", "menu_item_leading_icon_ink_too_faint"],
        ["leading_texture_delta", thresholds.leading_texture_min, ">=", "menu_item_leading_icon_texture_too_flat"],
        ["leading_body_luma_delta", thresholds.leading_anchor_delta_min, ">=", "menu_item_leading_anchor_not_distinct_from_body"],
        ["rim_highlight_ratio", thresholds.rim_highlight_ratio_min, ">=", "menu_item_rim_highlight_too_weak"],
      ];
      for (const [metric, limit, op, reason] of checks) {
        const value = Number(audit.metrics[metric] || 0);
        const pass = op === ">=" ? value >= limit : value <= limit;
        if (!pass) {
          audit.failures.push({
            reason,
            scenario: item.scenario,
            state: item.state,
            label: item.label,
            metric,
            value: round(value),
            expected: `${op} ${limit}`,
            crop_path: item.crop_path,
          });
        }
      }
      failures.push(...audit.failures);
      audited.push({ ...audit, ready: audit.failures.length === 0 });
    }
  } finally {
    await browser.close();
  }

  const byState = summarizeCounts(audited, "state");
  const byScenario = summarizeCounts(audited, "scenario");
  const stateCounts = Object.fromEntries(byState.map((item) => [item.state, item.crop_count]));
  const scenarioCounts = Object.fromEntries(byScenario.map((item) => [item.scenario, item.crop_count]));
  for (const [state, expected] of Object.entries(expectedStateCounts)) {
    const actual = stateCounts[state] || 0;
    if (actual !== expected) {
      failures.push({ reason: "menu_item_state_coverage_changed", state, expected, actual });
    }
  }
  for (const [scenario, expectedMinimum] of Object.entries(expectedScenarioMinimums)) {
    const actual = scenarioCounts[scenario] || 0;
    if (actual < expectedMinimum) {
      failures.push({ reason: "menu_item_scenario_coverage_below_minimum", scenario, expected_min: expectedMinimum, actual });
    }
  }

  const groups = Object.entries(groupBy(audited, (item) => `${item.state}|${item.label}`))
    .map(([key, values]) => {
      const [state, label] = key.split("|");
      return {
        state,
        label,
        crop_count: values.length,
        scenarios: values.map((item) => item.scenario).sort(),
        glass_white_ratio_spread: spread(values, (item) => item.metrics.glass_white_ratio),
        mean_saturation_spread: spread(values, (item) => item.metrics.mean_saturation),
        leading_anchor_delta_spread: spread(values, (item) => item.metrics.leading_body_luma_delta),
        failure_count: values.filter((item) => item.failures.length > 0).length,
      };
    })
    .sort((a, b) => `${a.state}|${a.label}`.localeCompare(`${b.state}|${b.label}`));

  if (groups.length !== thresholds.menu_item_group_count) {
    failures.push({ reason: "menu_item_group_count_changed", expected: thresholds.menu_item_group_count, actual: groups.length });
  }
  for (const group of groups) {
    if (group.glass_white_ratio_spread > thresholds.group_glass_white_ratio_spread_max) {
      failures.push({
        reason: "menu_item_group_white_glass_varies_too_much",
        state: group.state,
        label: group.label,
        metric: "glass_white_ratio_spread",
        value: group.glass_white_ratio_spread,
        expected: `<= ${thresholds.group_glass_white_ratio_spread_max}`,
        scenarios: group.scenarios,
      });
    }
    if (group.mean_saturation_spread > thresholds.group_mean_saturation_spread_max) {
      failures.push({
        reason: "menu_item_group_saturation_varies_too_much",
        state: group.state,
        label: group.label,
        metric: "mean_saturation_spread",
        value: group.mean_saturation_spread,
        expected: `<= ${thresholds.group_mean_saturation_spread_max}`,
        scenarios: group.scenarios,
      });
    }
    if (group.leading_anchor_delta_spread > thresholds.group_leading_anchor_delta_spread_max) {
      failures.push({
        reason: "menu_item_group_leading_anchor_varies_too_much",
        state: group.state,
        label: group.label,
        metric: "leading_anchor_delta_spread",
        value: group.leading_anchor_delta_spread,
        expected: `<= ${thresholds.group_leading_anchor_delta_spread_max}`,
        scenarios: group.scenarios,
      });
    }
  }

  const weakestMenuItems = [...audited]
    .sort((a, b) => {
      const aScore = a.metrics.glass_white_ratio - a.metrics.mean_saturation - a.metrics.dark_ratio + (a.metrics.edge_highlight_ratio / 10);
      const bScore = b.metrics.glass_white_ratio - b.metrics.mean_saturation - b.metrics.dark_ratio + (b.metrics.edge_highlight_ratio / 10);
      return aScore - bScore;
    })
    .slice(0, 20);

  const summary = {
    v37_status: v37Report.status,
    v35_status: v35Census.status,
    source_crop_count: crops.length,
    menu_item_crop_count: menuItems.length,
    menu_item_group_count: groups.length,
    failure_count: failures.length,
    by_state: byState,
    by_scenario: byScenario,
    by_group: groups,
    thresholds,
    browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome measured menu-item crop affordance density.",
  };

  const result = {
    schema: "hepta-ui-harsh-top-design-referee-v38-menu-item-affordance-density-census/v1",
    status: failures.length === 0 ? "ready" : "failed",
    generated_at: new Date().toISOString(),
    inputs: { v37_report_path: v37ReportPath, v35_census_path: v35CensusPath },
    summary,
    failures,
    weakest_menu_items: weakestMenuItems,
    menu_items: audited,
  };

  process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
  if (failures.length > 0) process.exitCode = 1;
})().catch((error) => {
  console.error(error);
  process.exit(1);
});
NODE

if [[ "$V38_CENSUS_EXIT" -ne 0 && ! -s "$V38_CENSUS_PATH" ]]; then
  echo "v38 menu item affordance density census failed before writing JSON: $V38_CENSUS_PATH" >&2
  exit "$V38_CENSUS_EXIT"
fi

node - "$V37_REPORT_PATH" "$V38_CENSUS_PATH" >"$REPORT_PATH" <<'NODE'
const fs = require("node:fs");
const [v37ReportPath, v38CensusPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const v37 = readJson(v37ReportPath);
const v38 = readJson(v38CensusPath);
const failures = [];
if (v37.status !== "ready") {
  failures.push({ reason: "v37_prerequisite_not_ready", status: v37.status, path: v37ReportPath });
}
if (v38.status !== "ready") {
  failures.push(...(v38.failures || []).slice(0, 240));
}
const report = {
  schema: "hepta-ui-harsh-top-design-referee-v38-menu-item-affordance-density-gate/v1",
  status: failures.length === 0 ? "ready" : "failed",
  generated_at: new Date().toISOString(),
  summary: {
    v37_open_composer_trigger_parity_referee: v37.summary?.v37_open_composer_trigger_parity_referee || null,
    v38_menu_item_affordance_density_referee: v38.summary || null,
  },
  failures,
};
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
if (failures.length > 0) process.exitCode = 1;
NODE

status="$(jq -r '.status' "$REPORT_PATH")"
failure_count="$(jq -r '.summary.v38_menu_item_affordance_density_referee.failure_count // 0' "$REPORT_PATH")"
if [[ "$status" != "ready" ]]; then
  echo "v38 menu item affordance density referee failed with ${failure_count} failures: $REPORT_PATH" >&2
  exit 1
fi

echo "v38 menu item affordance density referee ready: $REPORT_PATH"
