#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V41_REPORT_PATH:-}"
V41_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V41_CENSUS_PATH:-}"
V40_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V40_REPORT_PATH:-}"
V40_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V40_CENSUS_PATH:-}"
V39_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V39_CENSUS_PATH:-}"
V40_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V41_V40_LOG:-}"
SKIP_V40="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V41_SKIP_V40:-0}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v41-exhaustive-small-control-submenu-gate.json"
fi
if [[ -z "$V41_CENSUS_PATH" ]]; then
  V41_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v41-exhaustive-small-control-submenu-census.json"
fi
if [[ -z "$V40_REPORT_PATH" ]]; then
  V40_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v40-small-module-submenu-finish-gate.json"
fi
if [[ -z "$V40_CENSUS_PATH" ]]; then
  V40_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v40-small-module-submenu-finish-census.json"
fi
if [[ -z "$V39_CENSUS_PATH" ]]; then
  V39_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v39-interactive-glass-finish-census.json"
fi
if [[ -z "$V40_LOG" ]]; then
  V40_LOG="$READINESS_DIR/v40-small-module-submenu-finish-prerequisite.log"
fi

mkdir -p "$READINESS_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V41_CENSUS_PATH")"

if [[ "$SKIP_V40" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V40_REPORT_PATH="$V40_REPORT_PATH" \
    HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V40_CENSUS_PATH="$V40_CENSUS_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v40-small-module-submenu-finish-gate.sh "$READINESS_DIR" >"$V40_LOG" 2>&1 || {
      echo "v40 small-module/submenu prerequisite failed" >&2
      tail -n 180 "$V40_LOG" >&2 || true
      exit 1
    }
fi

for evidence in "$V40_REPORT_PATH" "$V40_CENSUS_PATH" "$V39_CENSUS_PATH"; do
  if [[ ! -s "$evidence" ]]; then
    echo "missing prerequisite evidence: $evidence" >&2
    exit 1
  fi
done
if [[ "$(jq -r '.status' "$V40_REPORT_PATH")" != "ready" ]]; then
  echo "v40 small-module/submenu prerequisite was not ready: $V40_REPORT_PATH" >&2
  exit 1
fi

node - "$V40_REPORT_PATH" "$V40_CENSUS_PATH" "$V39_CENSUS_PATH" >"$V41_CENSUS_PATH" <<'NODE'
const fs = require("node:fs");

const [v40ReportPath, v40CensusPath, v39CensusPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const round = (value, digits = 4) => Number(Number(value || 0).toFixed(digits));

const v40Report = readJson(v40ReportPath);
const v40Census = readJson(v40CensusPath);
const v39Census = readJson(v39CensusPath);
const crops = Array.isArray(v40Census.audited_crops) ? v40Census.audited_crops : [];
const groups = Array.isArray(v39Census.summary?.by_group) ? v39Census.summary.by_group : [];

const thresholds = {
  source_crop_count: 509,
  interactive_crop_count: 325,
  interactive_group_count: 128,
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
      finish_score_min: 1.0,
    },
    "menu-item": {
      glass_white_ratio_min: 0.9,
      mean_saturation_max: 0.11,
      dark_ratio_max: 0.075,
      edge_luma_p95_min: 254,
      edge_highlight_ratio_min: 0.3,
      cyan_edge_ratio_min: 0.16,
      texture_delta_min: 3.4,
      overbright_ratio_max: 0.74,
      edge_center_luma_delta_min: 0.6,
      finish_score_min: 0.98,
    },
    "micro-surface": {
      glass_white_ratio_min: 0.82,
      mean_saturation_max: 0.15,
      dark_ratio_max: 0.15,
      edge_luma_p95_min: 244,
      edge_highlight_ratio_min: 0.44,
      texture_delta_min: 0.25,
      overbright_ratio_max: 0.78,
      finish_score_min: 0.92,
    },
    panel: {
      glass_white_ratio_min: 0.91,
      mean_saturation_max: 0.1,
      dark_ratio_max: 0.06,
      edge_luma_p95_min: 236,
      edge_highlight_ratio_min: 0.035,
      texture_delta_min: 6,
      overbright_ratio_max: 0.76,
      finish_score_min: 1.02,
    },
  },
  group_spread: {
    control: {
      cyan_edge_ratio_spread: 0.46,
      overbright_ratio_spread: 0.63,
      edge_highlight_ratio_spread: 0.62,
      mean_saturation_spread: 0.027,
      dark_ratio_spread: 0.004,
    },
    "menu-item": {
      cyan_edge_ratio_spread: 0.34,
      overbright_ratio_spread: 0.48,
      edge_highlight_ratio_spread: 0.28,
      mean_saturation_spread: 0.036,
      dark_ratio_spread: 0.047,
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

const reasonByMetric = {
  glass_white_ratio: "v41_surface_not_bright_white_tempered_glass_enough",
  mean_saturation: "v41_surface_too_saturated_for_2026_light_glass",
  dark_ratio: "v41_surface_dark_area_too_heavy",
  edge_luma_p95: "v41_edge_luma_too_low",
  edge_highlight_ratio: "v41_edge_highlight_too_weak",
  cyan_edge_ratio: "v41_tempered_cyan_rim_too_weak",
  texture_delta: "v41_surface_too_flat_or_dead_white",
  overbright_ratio: "v41_surface_overbright_dead_white",
  edge_center_luma_delta: "v41_menu_item_edge_not_lifted_from_center",
  finish_score: "v41_finish_score_too_low",
};

const failures = [];
if (v40Report.status !== "ready") failures.push({ reason: "v40_prerequisite_not_ready", status: v40Report.status, path: v40ReportPath });
if (v40Census.status !== "ready") failures.push({ reason: "v40_census_not_ready", status: v40Census.status, path: v40CensusPath });
if (v39Census.status !== "ready") failures.push({ reason: "v39_census_not_ready", status: v39Census.status, path: v39CensusPath });
if (crops.length !== thresholds.source_crop_count) {
  failures.push({ reason: "v41_source_crop_count_changed", expected: thresholds.source_crop_count, actual: crops.length });
}
if (v40Census.summary?.v39_interactive_crop_count !== thresholds.interactive_crop_count) {
  failures.push({ reason: "v41_interactive_crop_count_changed", expected: thresholds.interactive_crop_count, actual: v40Census.summary?.v39_interactive_crop_count ?? null });
}
if (v40Census.summary?.v39_interactive_group_count !== thresholds.interactive_group_count) {
  failures.push({ reason: "v41_interactive_group_count_changed", expected: thresholds.interactive_group_count, actual: v40Census.summary?.v39_interactive_group_count ?? null });
}

const audited = crops.map((item) => {
  const metrics = { ...(item.metrics || {}) };
  metrics.finish_score = Number.isFinite(Number(metrics.finish_score)) ? Number(metrics.finish_score) : finishScore(metrics);
  const audit = { ...item, metrics, failures: [] };
  const limits = thresholds.per_crop[item.category];
  if (!limits) {
    audit.failures.push({ reason: "v41_unknown_crop_category", category: item.category, scenario: item.scenario, state: item.state, label: item.label, crop_path: item.crop_path });
  } else {
    for (const [key, limit] of Object.entries(limits)) {
      const op = key.endsWith("_max") ? "<=" : ">=";
      const metric = key.replace(/_(min|max)$/, "");
      const failure = compareMetric(audit, metric, limit, op, reasonByMetric[metric] || "v41_metric_outside_strict_threshold");
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
  if (actual !== expected) failures.push({ reason: "v41_category_coverage_changed", category, expected, actual });
}

const byState = summarizeCounts(audited, "state");
const stateCounts = Object.fromEntries(byState.map((item) => [item.state, item.crop_count]));
for (const [state, expected] of Object.entries(thresholds.state_counts)) {
  const actual = stateCounts[state] || 0;
  if (actual !== expected) failures.push({ reason: "v41_state_coverage_changed", state, expected, actual });
}

const groupFailures = [];
for (const group of groups) {
  const limits = thresholds.group_spread[group.category];
  if (!limits) continue;
  for (const [metric, limit] of Object.entries(limits)) {
    const value = Number(group[metric]);
    if (Number.isFinite(value) && value > limit) {
      groupFailures.push({
        reason: "v41_cross_viewport_small_control_or_submenu_spread_too_high",
        category: group.category,
        state: group.state,
        label: group.label,
        metric,
        value: round(value),
        expected: `<= ${limit}`,
        scenarios: group.scenarios,
      });
    }
  }
}
failures.push(...groupFailures);

const failureByReason = Object.entries(groupBy(failures, "reason"))
  .map(([reason, values]) => ({ reason, failure_count: values.length }))
  .sort((a, b) => b.failure_count - a.failure_count || a.reason.localeCompare(b.reason));
const failureByCategory = Object.entries(groupBy(failures.filter((item) => item.category), "category"))
  .map(([category, values]) => ({ category, failure_count: values.length }))
  .sort((a, b) => b.failure_count - a.failure_count || a.category.localeCompare(b.category));
const weakestCrops = [...audited]
  .sort((a, b) => {
    if (b.failures.length !== a.failures.length) return b.failures.length - a.failures.length;
    return Number(a.metrics.finish_score || 0) - Number(b.metrics.finish_score || 0);
  })
  .slice(0, 48);

const summary = {
  v40_status: v40Report.status,
  source_crop_count: crops.length,
  v39_interactive_crop_count: v40Census.summary?.v39_interactive_crop_count ?? null,
  v39_interactive_group_count: v40Census.summary?.v39_interactive_group_count ?? null,
  failure_count: failures.length,
  per_crop_failure_count: audited.reduce((sum, item) => sum + item.failures.length, 0),
  group_failure_count: groupFailures.length,
  by_category: byCategory,
  by_state: byState,
  failure_by_reason: failureByReason,
  failure_by_category: failureByCategory,
  group_failures: groupFailures,
  thresholds,
  browser_note: "Browser plugin unavailable in this run; v41 used regular Playwright/Chrome crop evidence from v35/v39/v40 and applies a stricter 2026 light tempered-glass referee over every small control, micro-surface, menu item, input, panel, and opened submenu group.",
};

const result = {
  schema: "hepta-ui-harsh-top-design-referee-v41-exhaustive-small-control-submenu-census/v1",
  status: failures.length === 0 ? "ready" : "failed",
  generated_at: new Date().toISOString(),
  inputs: { v40_report_path: v40ReportPath, v40_census_path: v40CensusPath, v39_census_path: v39CensusPath },
  summary,
  failures,
  weakest_crops: weakestCrops,
  audited_crops: audited,
};

process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
if (failures.length > 0) process.exitCode = 1;
NODE

node - "$V40_REPORT_PATH" "$V41_CENSUS_PATH" >"$REPORT_PATH" <<'NODE'
const fs = require("node:fs");
const [v40ReportPath, v41CensusPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const v40 = readJson(v40ReportPath);
const v41 = readJson(v41CensusPath);
const failures = [];
if (v40.status !== "ready") failures.push({ reason: "v40_prerequisite_not_ready", status: v40.status, path: v40ReportPath });
if (v41.status !== "ready") failures.push(...(v41.failures || []).slice(0, 640));
const report = {
  schema: "hepta-ui-harsh-top-design-referee-v41-exhaustive-small-control-submenu-gate/v1",
  status: failures.length === 0 ? "ready" : "failed",
  generated_at: new Date().toISOString(),
  summary: {
    v40_small_module_submenu_finish_referee: v40.summary?.v40_small_module_submenu_finish_referee || null,
    v41_exhaustive_small_control_submenu_referee: v41.summary || null,
  },
  failures,
};
process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
if (failures.length > 0) process.exitCode = 1;
NODE

status="$(jq -r '.status' "$REPORT_PATH")"
failure_count="$(jq -r '.summary.v41_exhaustive_small_control_submenu_referee.failure_count // 0' "$REPORT_PATH")"
if [[ "$status" != "ready" ]]; then
  echo "v41 exhaustive small-control/submenu referee failed with ${failure_count} failures: $REPORT_PATH" >&2
  exit 1
fi

echo "v41 exhaustive small-control/submenu referee ready: $REPORT_PATH"
