#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V37_REPORT_PATH:-}"
V37_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V37_CENSUS_PATH:-}"
V36_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V36_REPORT_PATH:-}"
V35_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V35_CENSUS_PATH:-}"
V36_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V37_V36_LOG:-}"
SKIP_V36="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V37_SKIP_V36:-0}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v37-open-composer-trigger-parity-gate.json"
fi
if [[ -z "$V37_CENSUS_PATH" ]]; then
  V37_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v37-open-composer-trigger-parity-census.json"
fi
if [[ -z "$V36_REPORT_PATH" ]]; then
  V36_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v36-cross-viewport-consistency-gate.json"
fi
if [[ -z "$V35_CENSUS_PATH" ]]; then
  V35_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v35-micro-polish-census.json"
fi
if [[ -z "$V36_LOG" ]]; then
  V36_LOG="$READINESS_DIR/v36-cross-viewport-consistency-prerequisite.log"
fi

mkdir -p "$READINESS_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V37_CENSUS_PATH")"

if [[ "$SKIP_V36" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V36_REPORT_PATH="$V36_REPORT_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v36-cross-viewport-consistency-gate.sh "$READINESS_DIR" >"$V36_LOG" 2>&1 || {
      echo "v36 cross-viewport consistency prerequisite failed" >&2
      tail -n 180 "$V36_LOG" >&2 || true
      exit 1
    }
fi

if [[ ! -f "$V36_REPORT_PATH" || ! -f "$V35_CENSUS_PATH" ]]; then
  echo "missing v36/v35 evidence under $READINESS_DIR" >&2
  exit 1
fi
if [[ "$(jq -r '.status' "$V36_REPORT_PATH")" != "ready" ]]; then
  echo "v36 cross-viewport prerequisite was not ready: $V36_REPORT_PATH" >&2
  exit 1
fi

node - "$V36_REPORT_PATH" "$V35_CENSUS_PATH" >"$V37_CENSUS_PATH" <<'NODE'
const fs = require("node:fs");

const [v36ReportPath, v35CensusPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const round = (value, digits = 4) => Number(Number(value || 0).toFixed(digits));

const v36Report = readJson(v36ReportPath);
const v35Census = readJson(v35CensusPath);
const crops = Array.isArray(v35Census.crops) ? v35Census.crops : [];

const expected = [
  { state: "opened-composer-popover-artifact", label: "attach-local-context" },
  { state: "opened-composer-popover-command", label: "insert-command" },
];
const scenarioMinimums = {
  "desktop-optical-crop": 2,
  "narrow-touch-optical-crop": 2,
  "mobile-optical-crop": 2,
  "phone320-optical-crop": 2,
};
const thresholds = {
  open_composer_trigger_count: 8,
  glass_white_ratio_min: 0.88,
  mean_saturation_max: 0.13,
  edge_highlight_ratio_min: 0.16,
  edge_luma_p95_min: 241,
  dark_ratio_max: 0.04,
  metric_spread_by_label: {
    glass_white_ratio_spread_max: 0.08,
    mean_saturation_spread_max: 0.045,
    edge_highlight_ratio_spread_max: 0.75,
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

function spread(items, metric) {
  const values = items.map((item) => Number(item.metrics?.[metric] || 0));
  return round(Math.max(...values) - Math.min(...values));
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

const failures = [];
if (v36Report.status !== "ready") {
  failures.push({ reason: "v36_prerequisite_not_ready", status: v36Report.status });
}
if (crops.length !== 487) {
  failures.push({ reason: "v35_crop_count_changed", expected: 487, actual: crops.length });
}

const openTriggerCrops = crops.filter((item) =>
  item.category === "control" &&
  expected.some((candidate) => candidate.state === item.state && candidate.label === item.label)
);

if (openTriggerCrops.length !== thresholds.open_composer_trigger_count) {
  failures.push({
    reason: "open_composer_trigger_crop_count_changed",
    expected: thresholds.open_composer_trigger_count,
    actual: openTriggerCrops.length,
  });
}

const byScenario = summarizeCounts(openTriggerCrops, "scenario");
const scenarioCounts = Object.fromEntries(byScenario.map((item) => [item.scenario, item.crop_count]));
for (const [scenario, expectedMin] of Object.entries(scenarioMinimums)) {
  const actual = scenarioCounts[scenario] || 0;
  if (actual < expectedMin) {
    failures.push({ reason: "scenario_open_composer_trigger_coverage_below_minimum", scenario, expected_min: expectedMin, actual });
  }
}

const audited = openTriggerCrops.map((item) => {
  const metrics = item.metrics || {};
  const itemFailures = [];
  const checks = [
    ["glass_white_ratio", thresholds.glass_white_ratio_min, ">=", "opened_composer_trigger_not_white_glass_enough"],
    ["mean_saturation", thresholds.mean_saturation_max, "<=", "opened_composer_trigger_too_saturated"],
    ["edge_highlight_ratio", thresholds.edge_highlight_ratio_min, ">=", "opened_composer_trigger_edge_highlight_too_weak"],
    ["edge_luma_p95", thresholds.edge_luma_p95_min, ">=", "opened_composer_trigger_edge_luma_too_low"],
    ["dark_ratio", thresholds.dark_ratio_max, "<=", "opened_composer_trigger_dark_ratio_too_high"],
  ];
  for (const [metric, limit, op, reason] of checks) {
    const value = Number(metrics[metric] || 0);
    const pass = op === ">=" ? value >= limit : value <= limit;
    if (!pass) {
      itemFailures.push({
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
  failures.push(...itemFailures);
  return { ...item, failures: itemFailures, ready: itemFailures.length === 0 };
});

const byLabelGroups = Object.entries(groupBy(openTriggerCrops, (item) => `${item.state}|${item.label}`))
  .map(([key, values]) => {
    const [state, label] = key.split("|");
    return {
      state,
      label,
      crop_count: values.length,
      scenarios: values.map((item) => item.scenario).sort(),
      glass_white_ratio_spread: spread(values, "glass_white_ratio"),
      mean_saturation_spread: spread(values, "mean_saturation"),
      edge_highlight_ratio_spread: spread(values, "edge_highlight_ratio"),
    };
  })
  .sort((a, b) => `${a.state}|${a.label}`.localeCompare(`${b.state}|${b.label}`));

for (const group of byLabelGroups) {
  const spreadThresholds = thresholds.metric_spread_by_label;
  for (const [metric, limit] of Object.entries(spreadThresholds)) {
    const value = group[metric.replace("_max", "")];
    if (typeof value === "number" && value > limit) {
      failures.push({
        reason: "opened_composer_trigger_metric_spread_too_high",
        state: group.state,
        label: group.label,
        metric: metric.replace("_max", ""),
        value,
        expected: `<= ${limit}`,
        scenarios: group.scenarios,
      });
    }
  }
}

const summary = {
  v36_status: v36Report.status,
  source_crop_count: crops.length,
  open_composer_trigger_crop_count: openTriggerCrops.length,
  scenario_count: byScenario.length,
  trigger_group_count: byLabelGroups.length,
  failure_count: failures.length,
  by_scenario: byScenario,
  by_label: byLabelGroups,
  thresholds,
  browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome generated the prerequisite crop evidence.",
};

const result = {
  schema: "hepta-ui-harsh-top-design-referee-v37-open-composer-trigger-parity-census/v1",
  status: failures.length === 0 ? "ready" : "failed",
  generated_at: new Date().toISOString(),
  inputs: { v36_report_path: v36ReportPath, v35_census_path: v35CensusPath },
  summary,
  failures,
  crops: audited,
};

process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
NODE

node - "$V36_REPORT_PATH" "$V37_CENSUS_PATH" >"$REPORT_PATH" <<'NODE'
const fs = require("node:fs");
const [v36ReportPath, v37CensusPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const v36 = readJson(v36ReportPath);
const v37 = readJson(v37CensusPath);
const failures = [];
if (v36.status !== "ready") {
  failures.push({ reason: "v36_prerequisite_not_ready", status: v36.status, path: v36ReportPath });
}
if (v37.status !== "ready") {
  failures.push(...(v37.failures || []).slice(0, 200));
}
const report = {
  schema: "hepta-ui-harsh-top-design-referee-v37-open-composer-trigger-parity-gate/v1",
  status: failures.length === 0 ? "ready" : "failed",
  generated_at: new Date().toISOString(),
  summary: {
    v36_cross_viewport_consistency_referee: v36.summary?.v36_cross_viewport_consistency_referee || null,
    v37_open_composer_trigger_parity_referee: v37.summary || null,
  },
  failures,
};
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
if (failures.length > 0) process.exitCode = 1;
NODE

status="$(jq -r '.status' "$REPORT_PATH")"
failure_count="$(jq -r '.summary.v37_open_composer_trigger_parity_referee.failure_count // 0' "$REPORT_PATH")"
if [[ "$status" != "ready" ]]; then
  echo "v37 open composer trigger parity referee failed with ${failure_count} failures: $REPORT_PATH" >&2
  exit 1
fi

echo "v37 open composer trigger parity referee ready: $REPORT_PATH"
