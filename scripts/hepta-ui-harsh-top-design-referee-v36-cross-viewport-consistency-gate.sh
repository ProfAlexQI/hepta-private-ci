#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V36_REPORT_PATH:-}"
V36_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V36_CENSUS_PATH:-}"
V35_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V35_REPORT_PATH:-}"
V35_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V35_CENSUS_PATH:-}"
V35_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V36_V35_LOG:-}"
SKIP_V35="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V36_SKIP_V35:-0}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v36-cross-viewport-consistency-gate.json"
fi
if [[ -z "$V36_CENSUS_PATH" ]]; then
  V36_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v36-cross-viewport-consistency-census.json"
fi
if [[ -z "$V35_REPORT_PATH" ]]; then
  V35_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v35-micro-polish-gate.json"
fi
if [[ -z "$V35_CENSUS_PATH" ]]; then
  V35_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v35-micro-polish-census.json"
fi
if [[ -z "$V35_LOG" ]]; then
  V35_LOG="$READINESS_DIR/v35-micro-polish-prerequisite.log"
fi

mkdir -p "$READINESS_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V36_CENSUS_PATH")"

if [[ "$SKIP_V35" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V35_REPORT_PATH="$V35_REPORT_PATH" \
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V35_CENSUS_PATH="$V35_CENSUS_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v35-micro-polish-gate.sh "$READINESS_DIR" >"$V35_LOG" 2>&1 || {
      echo "v35 micro-polish prerequisite failed" >&2
      tail -n 180 "$V35_LOG" >&2 || true
      exit 1
    }
fi

if [[ ! -f "$V35_REPORT_PATH" || ! -f "$V35_CENSUS_PATH" ]]; then
  echo "missing v35 micro-polish evidence under $READINESS_DIR" >&2
  exit 1
fi
if [[ "$(jq -r '.status' "$V35_REPORT_PATH")" != "ready" ]]; then
  echo "v35 micro-polish prerequisite was not ready: $V35_REPORT_PATH" >&2
  exit 1
fi

node - "$V35_REPORT_PATH" "$V35_CENSUS_PATH" >"$V36_CENSUS_PATH" <<'NODE'
const fs = require("node:fs");

const [v35ReportPath, v35CensusPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const round = (value, digits = 4) => Number(Number(value || 0).toFixed(digits));

const v35Report = readJson(v35ReportPath);
const v35Census = readJson(v35CensusPath);
const crops = Array.isArray(v35Census.crops) ? v35Census.crops : [];

const scenarioMinimums = {
  "desktop-optical-crop": 160,
  "narrow-touch-optical-crop": 140,
  "mobile-optical-crop": 80,
  "phone320-optical-crop": 75,
};

const expectedCategoryCounts = {
  control: 235,
  input: 50,
  "menu-item": 78,
  "micro-surface": 94,
  panel: 30,
};

const expectedStateCounts = {
  default: 52,
  "opened-command-palette": 64,
  "opened-composer-popover-artifact": 67,
  "opened-composer-popover-command": 67,
  "opened-composer-tools": 58,
  "opened-row-menu-operator-plane": 36,
  "opened-row-menu-task-queue": 36,
  "opened-row-menu-ui-chat-agent": 38,
  "opened-thread-tools": 69,
};

const thresholds = {
  control: {
    group_mean_luma_spread_max: 30,
    group_mean_saturation_spread_max: 0.04,
    group_glass_white_ratio_spread_max: 0.20,
    group_overbright_ratio_spread_max: 0.72,
    group_dark_ratio_max: 0.055,
  },
  input: {
    group_mean_luma_spread_max: 18,
    group_mean_saturation_spread_max: 0.035,
    group_glass_white_ratio_spread_max: 0.13,
    group_overbright_ratio_spread_max: 0.16,
    group_dark_ratio_max: 0.06,
  },
  "menu-item": {
    group_mean_luma_spread_max: 15,
    group_mean_saturation_spread_max: 0.04,
    group_glass_white_ratio_spread_max: 0.16,
    group_overbright_ratio_spread_max: 0.32,
    group_dark_ratio_max: 0.10,
  },
  "micro-surface": {
    group_mean_luma_spread_max: 7,
    group_mean_saturation_spread_max: 0.025,
    group_glass_white_ratio_spread_max: 0.14,
    group_overbright_ratio_spread_max: 0.16,
    group_dark_ratio_max: 0.15,
  },
  panel: {
    group_mean_luma_spread_max: 10,
    group_mean_saturation_spread_max: 0.03,
    group_glass_white_ratio_spread_max: 0.08,
    group_overbright_ratio_spread_max: 0.24,
    group_dark_ratio_max: 0.06,
  },
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

function metricSpread(items, metric) {
  const values = items.map((item) => Number(item.metrics?.[metric] || 0));
  return round(Math.max(...values) - Math.min(...values));
}

function metricMin(items, metric) {
  return round(Math.min(...items.map((item) => Number(item.metrics?.[metric] || 0))));
}

function metricMax(items, metric) {
  return round(Math.max(...items.map((item) => Number(item.metrics?.[metric] || 0))));
}

const failures = [];

if (v35Report.status !== "ready" || v35Census.status !== "ready") {
  failures.push({
    reason: "v35_prerequisite_not_ready",
    report_status: v35Report.status,
    census_status: v35Census.status,
  });
}

if (crops.length !== 487) {
  failures.push({ reason: "crop_count_changed_from_v35_baseline", expected: 487, actual: crops.length });
}

const byScenario = summarizeCounts(crops, "scenario");
const byCategory = summarizeCounts(crops, "category");
const byState = summarizeCounts(crops, "state");
const categoryCounts = Object.fromEntries(byCategory.map((item) => [item.category, item.crop_count]));
const stateCounts = Object.fromEntries(byState.map((item) => [item.state, item.crop_count]));
const scenarioCounts = Object.fromEntries(byScenario.map((item) => [item.scenario, item.crop_count]));

for (const [scenario, expectedMin] of Object.entries(scenarioMinimums)) {
  const actual = scenarioCounts[scenario] || 0;
  if (actual < expectedMin) {
    failures.push({ reason: "scenario_crop_coverage_below_minimum", scenario, expected_min: expectedMin, actual });
  }
}

for (const [category, expected] of Object.entries(expectedCategoryCounts)) {
  const actual = categoryCounts[category] || 0;
  if (actual !== expected) {
    failures.push({ reason: "category_crop_count_changed", category, expected, actual });
  }
}

for (const [state, expected] of Object.entries(expectedStateCounts)) {
  const actual = stateCounts[state] || 0;
  if (actual !== expected) {
    failures.push({ reason: "state_crop_count_changed", state, expected, actual });
  }
}

const componentGroups = Object.entries(groupBy(crops, (item) => `${item.category}|${item.state}|${item.label}`))
  .map(([key, values]) => {
    const [category, state, label] = key.split("|");
    return {
      key,
      category,
      state,
      label,
      crop_count: values.length,
      scenarios: [...new Set(values.map((item) => item.scenario))].sort(),
      mean_luma_spread: metricSpread(values, "mean_luma"),
      mean_saturation_spread: metricSpread(values, "mean_saturation"),
      glass_white_ratio_spread: metricSpread(values, "glass_white_ratio"),
      overbright_ratio_spread: metricSpread(values, "overbright_ratio"),
      dark_ratio_max: metricMax(values, "dark_ratio"),
      edge_highlight_ratio_min: metricMin(values, "edge_highlight_ratio"),
      edge_luma_p95_min: metricMin(values, "edge_luma_p95"),
    };
  })
  .sort((a, b) => a.key.localeCompare(b.key));

const repeatedGroups = componentGroups.filter((group) => group.crop_count >= 2);
if (repeatedGroups.length < 150) {
  failures.push({ reason: "repeated_component_group_coverage_below_minimum", expected_min: 150, actual: repeatedGroups.length });
}

for (const group of repeatedGroups) {
  const threshold = thresholds[group.category];
  if (!threshold) {
    failures.push({ reason: "unknown_consistency_category", group });
    continue;
  }
  const checks = [
    ["mean_luma_spread", "group_mean_luma_spread_max", "component_lightness_varies_too_much_across_viewports"],
    ["mean_saturation_spread", "group_mean_saturation_spread_max", "component_saturation_varies_too_much_across_viewports"],
    ["glass_white_ratio_spread", "group_glass_white_ratio_spread_max", "component_glass_area_varies_too_much_across_viewports"],
    ["overbright_ratio_spread", "group_overbright_ratio_spread_max", "component_overbright_ratio_varies_too_much_across_viewports"],
  ];
  for (const [metric, limitName, reason] of checks) {
    if (group[metric] > threshold[limitName]) {
      failures.push({
        reason,
        category: group.category,
        state: group.state,
        label: group.label,
        metric,
        value: group[metric],
        expected: `<= ${threshold[limitName]}`,
        scenarios: group.scenarios,
      });
    }
  }
  if (group.dark_ratio_max > threshold.group_dark_ratio_max) {
    failures.push({
      reason: "component_dark_ratio_too_high_in_at_least_one_viewport",
      category: group.category,
      state: group.state,
      label: group.label,
      metric: "dark_ratio_max",
      value: group.dark_ratio_max,
      expected: `<= ${threshold.group_dark_ratio_max}`,
      scenarios: group.scenarios,
    });
  }
}

const worstGroups = [...repeatedGroups]
  .sort((a, b) => b.mean_luma_spread - a.mean_luma_spread)
  .slice(0, 20);

const summary = {
  v35_status: v35Report.status,
  crop_count: crops.length,
  scenario_count: byScenario.length,
  category_count: byCategory.length,
  state_count: byState.length,
  repeated_component_group_count: repeatedGroups.length,
  consistency_failure_count: failures.length,
  failure_count: failures.length,
  by_scenario: byScenario,
  by_category: byCategory,
  by_state: byState,
  thresholds,
  browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome generated the prerequisite crop evidence.",
};

const result = {
  schema: "hepta-ui-harsh-top-design-referee-v36-cross-viewport-consistency-census/v1",
  status: failures.length === 0 ? "ready" : "failed",
  generated_at: new Date().toISOString(),
  inputs: {
    v35_report_path: v35ReportPath,
    v35_census_path: v35CensusPath,
  },
  summary,
  failures,
  worst_groups: worstGroups,
  repeated_component_groups: repeatedGroups,
};

process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
NODE

node - "$V35_REPORT_PATH" "$V36_CENSUS_PATH" >"$REPORT_PATH" <<'NODE'
const fs = require("node:fs");
const [v35ReportPath, v36CensusPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const v35 = readJson(v35ReportPath);
const v36 = readJson(v36CensusPath);
const failures = [];
if (v35.status !== "ready") {
  failures.push({ reason: "v35_prerequisite_not_ready", status: v35.status, path: v35ReportPath });
}
if (v36.status !== "ready") {
  failures.push(...(v36.failures || []).slice(0, 200));
}
const report = {
  schema: "hepta-ui-harsh-top-design-referee-v36-cross-viewport-consistency-gate/v1",
  status: failures.length === 0 ? "ready" : "failed",
  generated_at: new Date().toISOString(),
  summary: {
    v35_micro_polish_referee: v35.summary?.v35_micro_polish_referee || null,
    v36_cross_viewport_consistency_referee: v36.summary || null,
  },
  failures,
};
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
if (failures.length > 0) process.exitCode = 1;
NODE

status="$(jq -r '.status' "$REPORT_PATH")"
failure_count="$(jq -r '.summary.v36_cross_viewport_consistency_referee.failure_count // 0' "$REPORT_PATH")"
if [[ "$status" != "ready" ]]; then
  echo "v36 cross-viewport consistency referee failed with ${failure_count} failures: $REPORT_PATH" >&2
  exit 1
fi

echo "v36 cross-viewport consistency referee ready: $REPORT_PATH"
