#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V39_REPORT_PATH:-}"
V39_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V39_CENSUS_PATH:-}"
V38_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V38_REPORT_PATH:-}"
V38_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V38_CENSUS_PATH:-}"
V35_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V35_CENSUS_PATH:-}"
V38_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V39_V38_LOG:-}"
SKIP_V38="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V39_SKIP_V38:-0}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v39-interactive-glass-finish-gate.json"
fi
if [[ -z "$V39_CENSUS_PATH" ]]; then
  V39_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v39-interactive-glass-finish-census.json"
fi
if [[ -z "$V38_REPORT_PATH" ]]; then
  V38_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v38-menu-item-affordance-density-gate.json"
fi
if [[ -z "$V38_CENSUS_PATH" ]]; then
  V38_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v38-menu-item-affordance-density-census.json"
fi
if [[ -z "$V35_CENSUS_PATH" ]]; then
  V35_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v35-micro-polish-census.json"
fi
if [[ -z "$V38_LOG" ]]; then
  V38_LOG="$READINESS_DIR/v38-menu-item-affordance-density-prerequisite.log"
fi

mkdir -p "$READINESS_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V39_CENSUS_PATH")"

if [[ "$SKIP_V38" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V38_REPORT_PATH="$V38_REPORT_PATH" \
    HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V38_CENSUS_PATH="$V38_CENSUS_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v38-menu-item-affordance-density-gate.sh "$READINESS_DIR" >"$V38_LOG" 2>&1 || {
      echo "v38 menu item affordance density prerequisite failed" >&2
      tail -n 180 "$V38_LOG" >&2 || true
      exit 1
    }
fi

for evidence in "$V38_REPORT_PATH" "$V38_CENSUS_PATH" "$V35_CENSUS_PATH"; do
  if [[ ! -s "$evidence" ]]; then
    echo "missing prerequisite evidence: $evidence" >&2
    exit 1
  fi
done
if [[ "$(jq -r '.status' "$V38_REPORT_PATH")" != "ready" ]]; then
  echo "v38 menu item affordance density prerequisite was not ready: $V38_REPORT_PATH" >&2
  exit 1
fi

node - "$V38_REPORT_PATH" "$V38_CENSUS_PATH" "$V35_CENSUS_PATH" >"$V39_CENSUS_PATH" <<'NODE'
const fs = require("node:fs");

const [v38ReportPath, v38CensusPath, v35CensusPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const round = (value, digits = 4) => Number(Number(value || 0).toFixed(digits));

const v38Report = readJson(v38ReportPath);
const v38Census = readJson(v38CensusPath);
const v35Census = readJson(v35CensusPath);
const crops = Array.isArray(v35Census.crops) ? v35Census.crops : [];
const menuAffordanceItems = Array.isArray(v38Census.menu_items) ? v38Census.menu_items : [];
const auditedCategories = new Set(["control", "menu-item"]);
const interactiveCrops = crops.filter((item) => auditedCategories.has(item.category));

const thresholds = {
  source_crop_count: 487,
  interactive_crop_count: 313,
  category_counts: { control: 235, "menu-item": 78 },
  per_crop: {
    control: {
      glass_white_ratio_min: 0.887,
      mean_saturation_max: 0.122,
      dark_ratio_max: 0.038,
      edge_luma_p95_min: 241,
      edge_highlight_ratio_min: 0.12,
      cyan_edge_ratio_min: 0.38,
      texture_delta_min: 2.15,
      overbright_ratio_max: 0.635,
      mean_luma_min: 216,
      mean_luma_max: 246,
      luma_stddev_min: 22.4,
      luma_stddev_max: 41,
      finish_score_min: 0.9,
    },
    "menu-item": {
      glass_white_ratio_min: 0.902,
      mean_saturation_max: 0.094,
      dark_ratio_max: 0.068,
      edge_luma_p95_min: 254,
      edge_highlight_ratio_min: 0.65,
      cyan_edge_ratio_min: 0.26,
      texture_delta_min: 3.25,
      overbright_ratio_max: 0.682,
      mean_luma_min: 232,
      mean_luma_max: 247,
      luma_stddev_min: 26,
      luma_stddev_max: 57,
      edge_center_luma_delta_min: 1.5,
      finish_score_min: 1.035,
    },
  },
  menu_affordance: {
    leading_dark_ratio_min: 0.009,
    leading_texture_delta_min: 3,
    leading_body_luma_delta_min: 1.4,
    rim_highlight_ratio_min: 0.35,
  },
  group_spread: {
    control: {
      glass_white_ratio_spread_max: 0.065,
      mean_saturation_spread_max: 0.027,
      dark_ratio_spread_max: 0.004,
      edge_highlight_ratio_spread_max: 0.82,
      cyan_edge_ratio_spread_max: 0.46,
      overbright_ratio_spread_max: 0.63,
    },
    "menu-item": {
      glass_white_ratio_spread_max: 0.053,
      mean_saturation_spread_max: 0.036,
      dark_ratio_spread_max: 0.047,
      edge_highlight_ratio_spread_max: 0.28,
      cyan_edge_ratio_spread_max: 0.212,
      overbright_ratio_spread_max: 0.14,
    },
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

function spread(items, metric) {
  const values = items.map((item) => Number(item.metrics?.[metric])).filter(Number.isFinite);
  if (values.length === 0) return 0;
  return round(Math.max(...values) - Math.min(...values));
}

function finishScore(metrics) {
  return round(
    Number(metrics.glass_white_ratio || 0) +
      (Number(metrics.edge_highlight_ratio || 0) * 0.16) +
      (Math.min(Number(metrics.texture_delta || 0), 10) * 0.01) +
      (Math.min(Number(metrics.cyan_edge_ratio || 0), 1) * 0.05) -
      (Number(metrics.mean_saturation || 0) * 0.8) -
      (Number(metrics.dark_ratio || 0) * 0.8) -
      (Math.max(Number(metrics.overbright_ratio || 0) - 0.66, 0) * 0.3)
  );
}

function compareMetric(item, metric, limit, op, reason) {
  const value = Number(item.metrics?.[metric]);
  const pass = op === ">=" ? value >= limit : value <= limit;
  if (Number.isFinite(value) && pass) return null;
  return {
    reason,
    category: item.category,
    scenario: item.scenario,
    state: item.state,
    label: item.label,
    metric,
    value: Number.isFinite(value) ? round(value) : null,
    expected: `${op} ${limit}`,
    crop_path: item.crop_path,
  };
}

const failures = [];
if (v38Report.status !== "ready") {
  failures.push({ reason: "v38_prerequisite_not_ready", status: v38Report.status, path: v38ReportPath });
}
if (v38Census.status !== "ready") {
  failures.push({ reason: "v38_census_not_ready", status: v38Census.status, path: v38CensusPath });
}
if (v35Census.status !== "ready") {
  failures.push({ reason: "v35_census_not_ready", status: v35Census.status, path: v35CensusPath });
}
if (crops.length !== thresholds.source_crop_count) {
  failures.push({ reason: "source_crop_count_changed", expected: thresholds.source_crop_count, actual: crops.length });
}
if (interactiveCrops.length !== thresholds.interactive_crop_count) {
  failures.push({ reason: "interactive_crop_count_changed", expected: thresholds.interactive_crop_count, actual: interactiveCrops.length });
}

const menuAffordanceByKey = new Map(
  menuAffordanceItems.map((item) => [`${item.scenario}|${item.state}|${item.label}`, item])
);

const audited = interactiveCrops.map((item) => {
  const metrics = { ...(item.metrics || {}) };
  metrics.finish_score = finishScore(metrics);
  const audit = { ...item, metrics, failures: [] };
  const limits = thresholds.per_crop[item.category];
  const checks = [
    ["glass_white_ratio", limits.glass_white_ratio_min, ">=", "interactive_surface_not_bright_white_glass_enough"],
    ["mean_saturation", limits.mean_saturation_max, "<=", "interactive_surface_too_saturated_for_neutral_glass"],
    ["dark_ratio", limits.dark_ratio_max, "<=", "interactive_surface_dark_area_too_heavy"],
    ["edge_luma_p95", limits.edge_luma_p95_min, ">=", "interactive_surface_edge_luma_too_low"],
    ["edge_highlight_ratio", limits.edge_highlight_ratio_min, ">=", "interactive_surface_edge_highlight_too_weak"],
    ["cyan_edge_ratio", limits.cyan_edge_ratio_min, ">=", "interactive_surface_cyan_rim_too_weak"],
    ["texture_delta", limits.texture_delta_min, ">=", "interactive_surface_too_flat_or_dead_white"],
    ["overbright_ratio", limits.overbright_ratio_max, "<=", "interactive_surface_overbright_dead_white"],
    ["mean_luma", limits.mean_luma_min, ">=", "interactive_surface_too_gray"],
    ["mean_luma", limits.mean_luma_max, "<=", "interactive_surface_too_flat_bright"],
    ["luma_stddev", limits.luma_stddev_min, ">=", "interactive_surface_not_enough_tempered_texture"],
    ["luma_stddev", limits.luma_stddev_max, "<=", "interactive_surface_too_noisy_or_dirty"],
    ["finish_score", limits.finish_score_min, ">=", "interactive_surface_finish_score_too_low"],
  ];
  if (item.category === "menu-item") {
    checks.push(["edge_center_luma_delta", limits.edge_center_luma_delta_min, ">=", "menu_item_edge_not_lifted_from_center"]);
  }
  for (const check of checks) {
    const failure = compareMetric(audit, ...check);
    if (failure) audit.failures.push(failure);
  }

  if (item.category === "menu-item") {
    const affordance = menuAffordanceByKey.get(`${item.scenario}|${item.state}|${item.label}`);
    if (!affordance) {
      audit.failures.push({
        reason: "menu_item_missing_v38_leading_affordance_measurement",
        scenario: item.scenario,
        state: item.state,
        label: item.label,
        crop_path: item.crop_path,
      });
    } else {
      audit.metrics.leading_dark_ratio = affordance.metrics?.leading_dark_ratio;
      audit.metrics.leading_texture_delta = affordance.metrics?.leading_texture_delta;
      audit.metrics.leading_body_luma_delta = affordance.metrics?.leading_body_luma_delta;
      audit.metrics.rim_highlight_ratio = affordance.metrics?.rim_highlight_ratio;
      for (const [metric, limit, op, reason] of [
        ["leading_dark_ratio", thresholds.menu_affordance.leading_dark_ratio_min, ">=", "menu_item_leading_icon_ink_too_faint"],
        ["leading_texture_delta", thresholds.menu_affordance.leading_texture_delta_min, ">=", "menu_item_leading_icon_texture_too_flat"],
        ["leading_body_luma_delta", thresholds.menu_affordance.leading_body_luma_delta_min, ">=", "menu_item_leading_anchor_not_distinct_from_body"],
        ["rim_highlight_ratio", thresholds.menu_affordance.rim_highlight_ratio_min, ">=", "menu_item_rim_highlight_too_weak"],
      ]) {
        const failure = compareMetric(audit, metric, limit, op, reason);
        if (failure) audit.failures.push(failure);
      }
    }
  }

  failures.push(...audit.failures);
  return { ...audit, ready: audit.failures.length === 0 };
});

const byCategory = summarizeCounts(audited, "category");
const categoryCounts = Object.fromEntries(byCategory.map((item) => [item.category, item.crop_count]));
for (const [category, expected] of Object.entries(thresholds.category_counts)) {
  const actual = categoryCounts[category] || 0;
  if (actual !== expected) {
    failures.push({ reason: "interactive_category_count_changed", category, expected, actual });
  }
}

const groups = Object.entries(groupBy(audited, (item) => `${item.category}|${item.state}|${item.label}`))
  .map(([key, items]) => {
    const [category, state, label] = key.split("|");
    const group = {
      category,
      state,
      label,
      crop_count: items.length,
      scenarios: items.map((item) => item.scenario).sort(),
      failure_count: items.filter((item) => item.failures.length > 0).length,
    };
    for (const metric of [
      "glass_white_ratio",
      "mean_saturation",
      "dark_ratio",
      "edge_highlight_ratio",
      "cyan_edge_ratio",
      "overbright_ratio",
    ]) {
      group[`${metric}_spread`] = spread(items, metric);
    }
    return group;
  })
  .sort((a, b) => `${a.category}|${a.state}|${a.label}`.localeCompare(`${b.category}|${b.state}|${b.label}`));

for (const group of groups) {
  const limits = thresholds.group_spread[group.category];
  for (const [metric, limit] of Object.entries(limits)) {
    const value = group[metric.replace("_max", "")];
    if (typeof value === "number" && value > limit) {
      failures.push({
        reason: "interactive_surface_group_metric_spread_too_high",
        category: group.category,
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

const weakestInteractiveCrops = [...audited]
  .sort((a, b) => a.metrics.finish_score - b.metrics.finish_score)
  .slice(0, 24);

const summary = {
  v38_status: v38Report.status,
  v35_status: v35Census.status,
  source_crop_count: crops.length,
  interactive_crop_count: interactiveCrops.length,
  interactive_group_count: groups.length,
  failure_count: failures.length,
  by_category: byCategory,
  by_scenario: summarizeCounts(audited, "scenario"),
  by_state: summarizeCounts(audited, "state"),
  by_group: groups,
  thresholds,
  browser_note: "Browser plugin unavailable in this run; v39 used regular Playwright/Chrome crop evidence from v35/v38 and applied a stricter interactive tempered-glass finish referee.",
};

const result = {
  schema: "hepta-ui-harsh-top-design-referee-v39-interactive-glass-finish-census/v1",
  status: failures.length === 0 ? "ready" : "failed",
  generated_at: new Date().toISOString(),
  inputs: {
    v38_report_path: v38ReportPath,
    v38_census_path: v38CensusPath,
    v35_census_path: v35CensusPath,
  },
  summary,
  failures,
  weakest_interactive_crops: weakestInteractiveCrops,
  interactive_crops: audited,
};

process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
if (failures.length > 0) process.exitCode = 1;
NODE

node - "$V38_REPORT_PATH" "$V39_CENSUS_PATH" >"$REPORT_PATH" <<'NODE'
const fs = require("node:fs");
const [v38ReportPath, v39CensusPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const v38 = readJson(v38ReportPath);
const v39 = readJson(v39CensusPath);
const failures = [];
if (v38.status !== "ready") {
  failures.push({ reason: "v38_prerequisite_not_ready", status: v38.status, path: v38ReportPath });
}
if (v39.status !== "ready") {
  failures.push(...(v39.failures || []).slice(0, 320));
}
const report = {
  schema: "hepta-ui-harsh-top-design-referee-v39-interactive-glass-finish-gate/v1",
  status: failures.length === 0 ? "ready" : "failed",
  generated_at: new Date().toISOString(),
  summary: {
    v38_menu_item_affordance_density_referee: v38.summary?.v38_menu_item_affordance_density_referee || null,
    v39_interactive_glass_finish_referee: v39.summary || null,
  },
  failures,
};
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
if (failures.length > 0) process.exitCode = 1;
NODE

status="$(jq -r '.status' "$REPORT_PATH")"
failure_count="$(jq -r '.summary.v39_interactive_glass_finish_referee.failure_count // 0' "$REPORT_PATH")"
if [[ "$status" != "ready" ]]; then
  echo "v39 interactive glass finish referee failed with ${failure_count} failures: $REPORT_PATH" >&2
  exit 1
fi

echo "v39 interactive glass finish referee ready: $REPORT_PATH"
