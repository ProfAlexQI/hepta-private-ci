#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V40_REPORT_PATH:-}"
V40_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V40_CENSUS_PATH:-}"
V39_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V39_REPORT_PATH:-}"
V39_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V39_CENSUS_PATH:-}"
V35_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V35_CENSUS_PATH:-}"
V39_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V40_V39_LOG:-}"
SKIP_V39="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V40_SKIP_V39:-0}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v40-small-module-submenu-finish-gate.json"
fi
if [[ -z "$V40_CENSUS_PATH" ]]; then
  V40_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v40-small-module-submenu-finish-census.json"
fi
if [[ -z "$V39_REPORT_PATH" ]]; then
  V39_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v39-interactive-glass-finish-gate.json"
fi
if [[ -z "$V39_CENSUS_PATH" ]]; then
  V39_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v39-interactive-glass-finish-census.json"
fi
if [[ -z "$V35_CENSUS_PATH" ]]; then
  V35_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v35-micro-polish-census.json"
fi
if [[ -z "$V39_LOG" ]]; then
  V39_LOG="$READINESS_DIR/v39-interactive-glass-finish-prerequisite.log"
fi

mkdir -p "$READINESS_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V40_CENSUS_PATH")"

if [[ "$SKIP_V39" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V39_REPORT_PATH="$V39_REPORT_PATH" \
    HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V39_CENSUS_PATH="$V39_CENSUS_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v39-interactive-glass-finish-gate.sh "$READINESS_DIR" >"$V39_LOG" 2>&1 || {
      echo "v39 interactive glass finish prerequisite failed" >&2
      tail -n 180 "$V39_LOG" >&2 || true
      exit 1
    }
fi

for evidence in "$V39_REPORT_PATH" "$V39_CENSUS_PATH" "$V35_CENSUS_PATH"; do
  if [[ ! -s "$evidence" ]]; then
    echo "missing prerequisite evidence: $evidence" >&2
    exit 1
  fi
done
if [[ "$(jq -r '.status' "$V39_REPORT_PATH")" != "ready" ]]; then
  echo "v39 interactive glass finish prerequisite was not ready: $V39_REPORT_PATH" >&2
  exit 1
fi

node - "$V39_REPORT_PATH" "$V39_CENSUS_PATH" "$V35_CENSUS_PATH" >"$V40_CENSUS_PATH" <<'NODE'
const fs = require("node:fs");

const [v39ReportPath, v39CensusPath, v35CensusPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const round = (value, digits = 4) => Number(Number(value || 0).toFixed(digits));

const v39Report = readJson(v39ReportPath);
const v39Census = readJson(v39CensusPath);
const v35Census = readJson(v35CensusPath);
const crops = Array.isArray(v35Census.crops) ? v35Census.crops : [];
const interactiveGroups = Array.isArray(v39Census.summary?.by_group) ? v39Census.summary.by_group : [];

const thresholds = {
  source_crop_count: 509,
  v39_interactive_crop_count: 325,
  v39_interactive_group_count: 128,
  category_counts: { control: 228, input: 46, "menu-item": 97, "micro-surface": 108, panel: 30 },
  state_counts: {
    default: 50,
    "opened-command-palette": 117,
    "opened-composer-popover-artifact": 56,
    "opened-composer-popover-command": 55,
    "opened-composer-tools": 55,
    "opened-row-menu-operator-plane": 37,
    "opened-row-menu-task-queue": 37,
    "opened-row-menu-ui-chat-agent": 37,
    "opened-thread-tools": 65,
  },
  per_crop: {
    control: {
      glass_white_ratio_min: 0.85,
      mean_saturation_max: 0.17,
      dark_ratio_max: 0.02,
      edge_luma_p95_min: 243,
      edge_highlight_ratio_min: 0.19,
      cyan_edge_ratio_min: 0.58,
      texture_delta_min: 0.05,
      overbright_ratio_max: 0.68,
      finish_score_min: 0.85,
    },
    input: {
      glass_white_ratio_min: 0.87,
      mean_saturation_max: 0.11,
      dark_ratio_max: 0.06,
      edge_luma_p95_min: 242,
      edge_highlight_ratio_min: 0.16,
      texture_delta_min: 0.25,
      overbright_ratio_max: 0.76,
    },
    "menu-item": {
      glass_white_ratio_min: 0.9,
      mean_saturation_max: 0.11,
      dark_ratio_max: 0.075,
      edge_luma_p95_min: 254,
      edge_highlight_ratio_min: 0.30,
      cyan_edge_ratio_min: 0.16,
      texture_delta_min: 3.4,
      overbright_ratio_max: 0.74,
      edge_center_luma_delta_min: 0.6,
    },
    "micro-surface": {
      glass_white_ratio_min: 0.82,
      mean_saturation_max: 0.15,
      dark_ratio_max: 0.15,
      edge_luma_p95_min: 244,
      edge_highlight_ratio_min: 0.44,
      texture_delta_min: 0.25,
      overbright_ratio_max: 0.78,
    },
    panel: {
      glass_white_ratio_min: 0.91,
      mean_saturation_max: 0.10,
      dark_ratio_max: 0.06,
      edge_luma_p95_min: 236,
      edge_highlight_ratio_min: 0.035,
      texture_delta_min: 6,
      overbright_ratio_max: 0.76,
    },
  },
  group_spread: {
    control: {
      edge_highlight_ratio_spread_max: 0.62,
      overbright_ratio_spread_max: 0.63,
      cyan_edge_ratio_spread_max: 0.46,
      mean_saturation_spread_max: 0.027,
      dark_ratio_spread_max: 0.004,
    },
    "menu-item": {
      cyan_edge_ratio_spread_max: 0.34,
      overbright_ratio_spread_max: 0.48,
      edge_highlight_ratio_spread_max: 0.28,
      mean_saturation_spread_max: 0.036,
      dark_ratio_spread_max: 0.047,
    },
  },
};

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
if (v39Report.status !== "ready") {
  failures.push({ reason: "v39_prerequisite_not_ready", status: v39Report.status, path: v39ReportPath });
}
if (v39Census.status !== "ready") {
  failures.push({ reason: "v39_census_not_ready", status: v39Census.status, path: v39CensusPath });
}
if (v35Census.status !== "ready") {
  failures.push({ reason: "v35_census_not_ready", status: v35Census.status, path: v35CensusPath });
}
if (crops.length !== thresholds.source_crop_count) {
  failures.push({ reason: "source_crop_count_changed", expected: thresholds.source_crop_count, actual: crops.length });
}
if (v39Census.summary?.interactive_crop_count !== thresholds.v39_interactive_crop_count) {
  failures.push({
    reason: "v39_interactive_crop_count_changed",
    expected: thresholds.v39_interactive_crop_count,
    actual: v39Census.summary?.interactive_crop_count ?? null,
  });
}
if (v39Census.summary?.interactive_group_count !== thresholds.v39_interactive_group_count) {
  failures.push({
    reason: "v39_interactive_group_count_changed",
    expected: thresholds.v39_interactive_group_count,
    actual: v39Census.summary?.interactive_group_count ?? null,
  });
}

const audited = crops.map((item) => {
  const metrics = { ...(item.metrics || {}) };
  metrics.finish_score = finishScore(metrics);
  const audit = { ...item, metrics, failures: [] };
  const limits = thresholds.per_crop[item.category];
  if (!limits) {
    audit.failures.push({
      reason: "unknown_crop_category",
      category: item.category,
      scenario: item.scenario,
      state: item.state,
      label: item.label,
      crop_path: item.crop_path,
    });
  } else {
    const checks = [
      ["glass_white_ratio", limits.glass_white_ratio_min, ">=", "small_module_not_bright_white_glass_enough"],
      ["mean_saturation", limits.mean_saturation_max, "<=", "small_module_too_saturated_for_2026_light_glass"],
      ["dark_ratio", limits.dark_ratio_max, "<=", "small_module_dark_area_too_heavy"],
      ["edge_luma_p95", limits.edge_luma_p95_min, ">=", "small_module_edge_luma_too_low"],
      ["edge_highlight_ratio", limits.edge_highlight_ratio_min, ">=", "small_module_edge_highlight_too_weak"],
      ["texture_delta", limits.texture_delta_min, ">=", "small_module_too_flat_or_dead_white"],
      ["overbright_ratio", limits.overbright_ratio_max, "<=", "small_module_overbright_dead_white"],
    ];
    if (typeof limits.cyan_edge_ratio_min === "number") {
      checks.push(["cyan_edge_ratio", limits.cyan_edge_ratio_min, ">=", "small_module_cyan_rim_too_weak"]);
    }
    if (typeof limits.edge_center_luma_delta_min === "number") {
      checks.push(["edge_center_luma_delta", limits.edge_center_luma_delta_min, ">=", "menu_item_edge_not_lifted_from_center"]);
    }
    if (typeof limits.finish_score_min === "number") {
      checks.push(["finish_score", limits.finish_score_min, ">=", "control_finish_score_too_low_for_v40"]);
    }
    for (const check of checks) {
      const failure = compareMetric(audit, ...check);
      if (failure) audit.failures.push(failure);
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
    failures.push({ reason: "category_coverage_changed", category, expected, actual });
  }
}

const byState = summarizeCounts(audited, "state");
const stateCounts = Object.fromEntries(byState.map((item) => [item.state, item.crop_count]));
for (const [state, expected] of Object.entries(thresholds.state_counts)) {
  const actual = stateCounts[state] || 0;
  if (actual !== expected) {
    failures.push({ reason: "state_coverage_changed", state, expected, actual });
  }
}

const groupFailures = [];
for (const group of interactiveGroups) {
  const limits = thresholds.group_spread[group.category];
  if (!limits) continue;
  for (const [metric, limit] of Object.entries(limits)) {
    const value = Number(group[metric.replace("_max", "")]);
    if (Number.isFinite(value) && value > limit) {
      groupFailures.push({
        reason: "small_module_interactive_group_metric_spread_too_high",
        category: group.category,
        state: group.state,
        label: group.label,
        metric: metric.replace("_max", ""),
        value: round(value),
        expected: `<= ${limit}`,
        scenarios: group.scenarios,
      });
    }
  }
}
failures.push(...groupFailures);

const weakestCrops = [...audited]
  .sort((a, b) => {
    if (a.failures.length !== b.failures.length) return b.failures.length - a.failures.length;
    return a.metrics.finish_score - b.metrics.finish_score;
  })
  .slice(0, 32);

const summary = {
  v39_status: v39Report.status,
  source_crop_count: crops.length,
  v39_interactive_crop_count: v39Census.summary?.interactive_crop_count ?? null,
  v39_interactive_group_count: v39Census.summary?.interactive_group_count ?? null,
  failure_count: failures.length,
  per_crop_failure_count: audited.reduce((sum, item) => sum + item.failures.length, 0),
  group_failure_count: groupFailures.length,
  by_category: byCategory,
  by_state: byState,
  group_failures: groupFailures,
  thresholds,
  browser_note: "Browser plugin unavailable in this run; v40 reused regular Playwright/Chrome evidence from v35/v39 and applied stricter full small-module/submenu finish thresholds.",
};

const result = {
  schema: "hepta-ui-harsh-top-design-referee-v40-small-module-submenu-finish-census/v1",
  status: failures.length === 0 ? "ready" : "failed",
  generated_at: new Date().toISOString(),
  inputs: {
    v39_report_path: v39ReportPath,
    v39_census_path: v39CensusPath,
    v35_census_path: v35CensusPath,
  },
  summary,
  failures,
  weakest_crops: weakestCrops,
  audited_crops: audited,
};

process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
if (failures.length > 0) process.exitCode = 1;
NODE

node - "$V39_REPORT_PATH" "$V40_CENSUS_PATH" >"$REPORT_PATH" <<'NODE'
const fs = require("node:fs");
const [v39ReportPath, v40CensusPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const v39 = readJson(v39ReportPath);
const v40 = readJson(v40CensusPath);
const failures = [];
if (v39.status !== "ready") {
  failures.push({ reason: "v39_prerequisite_not_ready", status: v39.status, path: v39ReportPath });
}
if (v40.status !== "ready") {
  failures.push(...(v40.failures || []).slice(0, 420));
}
const report = {
  schema: "hepta-ui-harsh-top-design-referee-v40-small-module-submenu-finish-gate/v1",
  status: failures.length === 0 ? "ready" : "failed",
  generated_at: new Date().toISOString(),
  summary: {
    v39_interactive_glass_finish_referee: v39.summary?.v39_interactive_glass_finish_referee || null,
    v40_small_module_submenu_finish_referee: v40.summary || null,
  },
  failures,
};
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
if (failures.length > 0) process.exitCode = 1;
NODE

status="$(jq -r '.status' "$REPORT_PATH")"
failure_count="$(jq -r '.summary.v40_small_module_submenu_finish_referee.failure_count // 0' "$REPORT_PATH")"
if [[ "$status" != "ready" ]]; then
  echo "v40 small module submenu finish referee failed with ${failure_count} failures: $REPORT_PATH" >&2
  exit 1
fi

echo "v40 small module submenu finish referee ready: $REPORT_PATH"
