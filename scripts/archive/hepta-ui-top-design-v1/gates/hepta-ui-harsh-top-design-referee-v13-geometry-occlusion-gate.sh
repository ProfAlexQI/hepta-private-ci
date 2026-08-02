#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

source scripts/lib/hepta-ui-rust-toolchain.sh
source scripts/lib/hepta-control-ui-runtime-fixture.sh
hepta_ui_activate_rust_toolchain

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V13_REPORT_PATH:-}"
V12_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V12_REPORT_PATH:-}"
NATIVE_DIR="${HEPTA_NATIVE_FIXTURE_VISUAL_DIR:-}"
GEOMETRY_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V13_GEOMETRY_REPORT_PATH:-}"
GEOMETRY_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V13_SCREENSHOT_DIR:-}"
V12_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V13_V12_LOG:-}"
SKIP_V12="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V13_SKIP_V12:-0}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v13-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v13-geometry-occlusion-gate.json"
fi
if [[ -z "$V12_REPORT_PATH" ]]; then
  V12_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v12-interaction-state-crop-gate.json"
fi
if [[ -z "$NATIVE_DIR" ]]; then
  NATIVE_DIR="$READINESS_DIR/native-fixture"
fi
if [[ -z "$GEOMETRY_REPORT_PATH" ]]; then
  GEOMETRY_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v13-geometry-occlusion-census.json"
fi
if [[ -z "$GEOMETRY_SCREENSHOT_DIR" ]]; then
  GEOMETRY_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v13-geometry-occlusion-screenshots"
fi
if [[ -z "$V12_LOG" ]]; then
  V12_LOG="$READINESS_DIR/v12-interaction-state-crop.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$NATIVE_DIR" "$GEOMETRY_SCREENSHOT_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$GEOMETRY_REPORT_PATH")"

if [[ "$SKIP_V12" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V12_REPORT_PATH="$V12_REPORT_PATH" \
  HEPTA_NATIVE_FIXTURE_VISUAL_DIR="$NATIVE_DIR" \
    bash scripts/hepta-ui-harsh-top-design-referee-v12-interaction-state-crop-gate.sh "$READINESS_DIR" >"$V12_LOG" 2>&1 || {
      echo "v12 interaction-state crop prerequisite failed" >&2
      tail -n 180 "$V12_LOG" >&2 || true
      exit 1
    }
fi

if [[ ! -s "$V12_REPORT_PATH" ]]; then
  echo "missing v12 interaction-state crop prerequisite evidence: $V12_REPORT_PATH" >&2
  exit 1
fi
if [[ "$(jq -r '.status' "$V12_REPORT_PATH")" != "ready" ]]; then
  echo "v12 interaction-state crop prerequisite was not ready: $V12_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7414 7415 7416 7417 7418; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v13 referee" >&2
  exit 1
fi

BASE_URL="http://${BIND_ADDR}"
server_pid=""
tmp_report="$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v13-final.XXXXXX")"

cleanup() {
  if [[ -n "${server_pid:-}" ]] && kill -0 "$server_pid" 2>/dev/null; then
    kill "$server_pid" 2>/dev/null || true
    for _ in {1..20}; do
      if ! kill -0 "$server_pid" 2>/dev/null; then
        wait "$server_pid" 2>/dev/null || true
        break
      fi
      sleep 0.2
    done
    kill -9 "$server_pid" 2>/dev/null || true
    wait "$server_pid" 2>/dev/null || true
  fi
  rm -f "$tmp_report"
  hepta_control_ui_runtime_fixture_cleanup
}
trap cleanup EXIT

start_server() {
  hepta_control_ui_runtime_fixture_start_server "$MANIFEST" "$BIND_ADDR" "$SERVER_LOG"
}

wait_for_server() {
  local deadline=$((SECONDS + STARTUP_TIMEOUT_SEC))
  local root_probe=""
  until root_probe="$(curl -fsS "$BASE_URL/" 2>/dev/null)" && [[ "$root_probe" == *'data-rust-rendered-control-ui="true"'* ]]; do
    if ! kill -0 "$server_pid" 2>/dev/null; then
      echo "Hepta Control UI server exited before v13 geometry audit was ready" >&2
      tail -n 80 "$SERVER_LOG" >&2 || true
      return 1
    fi
    if [[ "$SECONDS" -ge "$deadline" ]]; then
      echo "timed out waiting for Hepta Control UI server at $BASE_URL" >&2
      tail -n 80 "$SERVER_LOG" >&2 || true
      return 1
    fi
    sleep 1
  done
}

start_server
wait_for_server

node - "$CHROME_BIN" "$BASE_URL/" "$GEOMETRY_SCREENSHOT_DIR" >"$GEOMETRY_REPORT_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir] = process.argv.slice(2);
const viewports = [
  { name: "desktop", width: 1365, height: 900, railVisible: true },
  { name: "narrow", width: 768, height: 900, railVisible: true },
  { name: "mobile", width: 500, height: 844, railVisible: false },
  { name: "phone320", width: 320, height: 844, railVisible: false },
];
const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase();
const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const round = (value, digits = 3) => Number(value.toFixed(digits));
fs.mkdirSync(screenshotDir, { recursive: true });

function targetDefinitions(viewport) {
  const targets = [];
  if (viewport.railVisible) {
    for (const key of ["ui-chat-agent", "task-queue", "operator-plane"]) {
      targets.push({
        key: `row-menu-${key}`,
        group: "row-menu",
        triggerSelector: `[data-chat-row-menu-toggle="${key}"]`,
        revealSelector: `[data-chat-conversation="${key}"]`,
        panelSelector: `[data-chat-row-menu-panel="${key}"]`,
        itemSelector: `[data-chat-row-menu-panel="${key}"] [data-chat-row-menu-item]`,
        expectedItemCount: 3,
      });
    }
  }
  targets.push(
    {
      key: "thread-tools",
      group: "thread-tools",
      triggerSelector: '[data-thread-command-menu="true"] [data-control-ui-thread-tools-trigger="light-glass"]',
      panelSelector: '[data-control-ui-thread-tools-panel="light-glass"]',
      itemSelector: '[data-thread-command-menu="true"] [data-control-ui-menu-item]',
      expectedItemCount: 3,
    },
    {
      key: "composer-tools",
      group: "composer-tools",
      triggerSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tools-trigger="light-glass"]',
      panelSelector: '[data-control-ui-composer-tools-panel="light-glass"]',
      itemSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tool-item]',
      expectedItemCount: 2,
    },
    {
      key: "composer-popover-artifact",
      group: "composer-popover",
      triggerSelector: '[data-chat-composer-popover-toggle="artifact"]',
      panelSelector: '[data-chat-composer-popover="artifact"]',
      itemSelector: '[data-chat-composer-popover="artifact"] .tg-composer-popover__item',
      expectedItemCount: 2,
    },
    {
      key: "composer-popover-command",
      group: "composer-popover",
      triggerSelector: '[data-chat-composer-popover-toggle="command"]',
      panelSelector: '[data-chat-composer-popover="command"]',
      itemSelector: '[data-chat-composer-popover="command"] .tg-composer-popover__item',
      expectedItemCount: 2,
    },
    {
      key: "command-palette",
      group: "command-palette",
      triggerSelector: '[data-control-ui-command-palette-trigger="light-glass"]',
      panelSelector: '#command-palette .command-palette',
      itemSelector: '[data-control-ui-command-palette-result="light-glass"]',
      expectedItemCount: 18,
    },
  );
  return targets;
}

async function revealTarget(page, target) {
  if (!target.revealSelector) return;
  const row = page.locator(target.revealSelector).first();
  await row.scrollIntoViewIfNeeded();
  await row.hover({ position: { x: 16, y: 16 } });
  await page.waitForTimeout(80);
}

async function openTarget(page, target) {
  await revealTarget(page, target);
  const trigger = page.locator(target.triggerSelector).first();
  await trigger.waitFor({ state: "visible", timeout: 5000 });
  await trigger.click({ timeout: 5000 });
  await page.waitForTimeout(180);
  const panel = page.locator(target.panelSelector).first();
  await panel.waitFor({ state: "visible", timeout: 5000 });
}

async function boxFor(locator) {
  const box = await locator.boundingBox().catch(() => null);
  if (!box) return null;
  return {
    left: box.x,
    top: box.y,
    right: box.x + box.width,
    bottom: box.y + box.height,
    width: box.width,
    height: box.height,
    area: box.width * box.height,
  };
}

function clippedRatio(box, viewport) {
  if (!box || box.area <= 0) return 0;
  const left = Math.max(0, box.left);
  const top = Math.max(0, box.top);
  const right = Math.min(viewport.width, box.right);
  const bottom = Math.min(viewport.height, box.bottom);
  const width = Math.max(0, right - left);
  const height = Math.max(0, bottom - top);
  return (width * height) / box.area;
}

function roundedBox(box) {
  if (!box) return null;
  return {
    left: round(box.left),
    top: round(box.top),
    right: round(box.right),
    bottom: round(box.bottom),
    width: round(box.width),
    height: round(box.height),
  };
}

async function geometryFor(page, locator, viewport, kind) {
  const box = await boxFor(locator);
  if (!box) {
    return { kind, box: null, clipped_ratio: 0, topmost: false, failures: ["missing_box"], ready: false };
  }
  const clipped = clippedRatio(box, viewport);
  const failures = [];
  const below = (value, minimum) => value + 0.5 < minimum;
  if (kind === "trigger" && (below(box.width, 44) || below(box.height, 44))) failures.push("trigger_hit_target_below_44px");
  if (kind === "panel" && (below(box.width, 120) || below(box.height, 44))) failures.push("panel_too_small");
  if (kind === "menu-item" && (below(box.width, 44) || below(box.height, 32))) failures.push("menu_item_touch_target_too_small");
  if (clipped < (kind === "panel" ? 0.995 : 0.985)) failures.push(`${kind}_clipped_by_viewport`);

  const topmost = await locator.evaluate((element) => {
    const rect = element.getBoundingClientRect();
    const points = [
      { x: rect.left + rect.width / 2, y: rect.top + rect.height / 2 },
      { x: rect.left + Math.min(16, rect.width / 2), y: rect.top + Math.min(16, rect.height / 2) },
      { x: rect.right - Math.min(16, rect.width / 2), y: rect.bottom - Math.min(16, rect.height / 2) },
    ].filter((point) => point.x >= 0 && point.y >= 0 && point.x <= window.innerWidth && point.y <= window.innerHeight);
    const hits = points.map((point) => document.elementFromPoint(point.x, point.y));
    return hits.length > 0 && hits.every((hit) => hit && (hit === element || element.contains(hit)));
  }).catch(() => false);
  if (!topmost) failures.push(`${kind}_not_topmost_at_sample_points`);

  return {
    kind,
    box: roundedBox(box),
    clipped_ratio: round(clipped, 4),
    topmost,
    failures,
    ready: failures.length === 0,
  };
}

async function pageOverflow(page) {
  return page.evaluate(() => ({
    inner_width: window.innerWidth,
    inner_height: window.innerHeight,
    document_scroll_width: document.documentElement.scrollWidth,
    body_scroll_width: document.body?.scrollWidth || 0,
    horizontal_overflow_px: Math.max(0, document.documentElement.scrollWidth - window.innerWidth, (document.body?.scrollWidth || 0) - window.innerWidth),
  }));
}

async function visibleCount(page, selector) {
  return page.locator(selector).evaluateAll((elements) => elements.filter((element) => {
    const rect = element.getBoundingClientRect();
    const style = window.getComputedStyle(element);
    return rect.width > 0 && rect.height > 0 && style.visibility !== "hidden" && style.display !== "none";
  }).length);
}

async function auditTarget(page, viewport, target, pass) {
  await page.goto(baseUrl, { waitUntil: "networkidle" });
  await page.waitForTimeout(150);
  await revealTarget(page, target);
  const trigger = page.locator(target.triggerSelector).first();
  await trigger.waitFor({ state: "visible", timeout: 5000 });
  const triggerGeometry = await geometryFor(page, trigger, viewport, "trigger");

  await openTarget(page, target);

  const panel = page.locator(target.panelSelector).first();
  const items = page.locator(target.itemSelector);
  const itemCount = await items.count();
  const visiblePanels = await visibleCount(page, target.panelSelector);
  const overflow = await pageOverflow(page);
  const panelGeometry = await geometryFor(page, panel, viewport, "panel");
  const itemGeometries = [];

  for (let index = 0; index < itemCount; index += 1) {
    const item = items.nth(index);
    await item.evaluate((element) => {
      element.scrollIntoView({ block: "center", inline: "nearest" });
    }).catch(() => {});
    await page.waitForTimeout(40);
    const label = await item.evaluate((element, fallback) => (
      element.getAttribute("data-chat-row-menu-item")
      || element.getAttribute("data-control-ui-menu-item")
      || element.getAttribute("data-control-ui-command-palette-item")
      || element.getAttribute("data-chat-composer-picker-item")
      || element.getAttribute("aria-label")
      || element.textContent
      || `item-${fallback}`
    ), index);
    itemGeometries.push({
      index,
      label: String(label).trim().slice(0, 96),
      ...(await geometryFor(page, item, viewport, "menu-item")),
    });
  }

  const screenshotPath = path.join(screenshotDir, `${sanitize(viewport.name)}-${sanitize(target.key)}-${pass}.png`);
  await page.screenshot({ path: screenshotPath, fullPage: false });

  const failures = [
    ...(visiblePanels === 1 ? [] : [`expected_one_visible_panel_got_${visiblePanels}`]),
    ...(itemCount === target.expectedItemCount ? [] : [`expected_${target.expectedItemCount}_items_got_${itemCount}`]),
    ...(overflow.horizontal_overflow_px <= 1 ? [] : [`horizontal_overflow_${overflow.horizontal_overflow_px}px`]),
    ...triggerGeometry.failures.map((failure) => `trigger:${failure}`),
    ...panelGeometry.failures.map((failure) => `panel:${failure}`),
    ...itemGeometries.flatMap((item) => item.failures.map((failure) => `item:${item.label}:${failure}`)),
  ];

  return {
    viewport: viewport.name,
    target: target.key,
    group: target.group,
    pass,
    expected_item_count: target.expectedItemCount,
    item_count: itemCount,
    visible_panel_count: visiblePanels,
    overflow,
    trigger: triggerGeometry,
    panel: panelGeometry,
    items: itemGeometries,
    screenshot: { path: screenshotPath, sha256: sha256(screenshotPath) },
    failures,
    ready: failures.length === 0,
  };
}

async function main() {
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
  const audits = [];

  for (const viewport of viewports) {
    const page = await browser.newPage({ viewport: { width: viewport.width, height: viewport.height }, deviceScaleFactor: 1 });
    for (const target of targetDefinitions(viewport)) {
      audits.push(await auditTarget(page, viewport, target, "first-open"));
      audits.push(await auditTarget(page, viewport, target, "reopen"));
    }
    await page.close();
  }

  await browser.close();

  const failures = audits.filter((audit) => !audit.ready);
  const byViewport = Object.values(audits.reduce((acc, audit) => {
    acc[audit.viewport] ||= { viewport: audit.viewport, audit_count: 0, failure_count: 0 };
    acc[audit.viewport].audit_count += 1;
    if (!audit.ready) acc[audit.viewport].failure_count += 1;
    return acc;
  }, {})).sort((a, b) => a.viewport.localeCompare(b.viewport));
  const byGroup = Object.values(audits.reduce((acc, audit) => {
    acc[audit.group] ||= { group: audit.group, audit_count: 0, failure_count: 0 };
    acc[audit.group].audit_count += 1;
    if (!audit.ready) acc[audit.group].failure_count += 1;
    return acc;
  }, {})).sort((a, b) => a.group.localeCompare(b.group));
  const targetCount = viewports.reduce((sum, viewport) => sum + targetDefinitions(viewport).length, 0);
  const report = {
    schema_version: "hepta-ui-harsh-top-design-referee-v13-geometry-occlusion-census/v0",
    standards_version: "2026-06-28-control-real-panel-geometry-topmost-viewport-fit",
    status: failures.length === 0 && audits.length === targetCount * 2 ? "ready" : "failed",
    base_url: baseUrl,
    screenshot_dir: screenshotDir,
    viewport_count: viewports.length,
    target_count: targetCount,
    audit_count: audits.length,
    expected_audit_count: targetCount * 2,
    screenshot_count: audits.length,
    failure_count: failures.length,
    thresholds: {
      trigger_min_size: "44x44",
      panel_min_size: "120x44",
      menu_item_min_size: "44x32",
      panel_clipped_ratio_min: 0.995,
      trigger_or_item_clipped_ratio_min: 0.985,
      horizontal_overflow_px_max: 1,
      topmost_sample_points: "center plus two diagonal interior points",
      reopen_passes_per_target: 2,
    },
    by_viewport: byViewport,
    by_group: byGroup,
    failures,
    audits,
  };
  console.log(JSON.stringify(report, null, 2));
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
NODE

if [[ "$(jq -r '.status' "$GEOMETRY_REPORT_PATH")" != "ready" ]]; then
  echo "v13 geometry/occlusion census failed" >&2
  jq '.failures[:20] | map({viewport, target, group, pass, failures})' "$GEOMETRY_REPORT_PATH" >&2 || true
  exit 1
fi

node - "$V12_REPORT_PATH" "$GEOMETRY_REPORT_PATH" "$REPORT_PATH" <<'NODE'
const crypto = require("node:crypto");
const fs = require("node:fs");

const [v12Path, geometryPath, outputPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const v12 = readJson(v12Path);
const geometry = readJson(geometryPath);
const report = {
  schema_version: "hepta-ui-harsh-top-design-referee-v13-gate/v0",
  standards_version: "2026-06-28-harsh-v12-plus-real-geometry-topmost-occlusion-reopen-census",
  status: v12.status === "ready" && geometry.status === "ready" ? "ready" : "failed",
  browser_path: "Browser plugin not available; regular Playwright with local Chrome was used",
  inputs: {
    v12_interaction_state_crop: { path: v12Path, sha256: sha256(v12Path) },
    geometry_occlusion_census: { path: geometryPath, sha256: sha256(geometryPath) },
  },
  summary: {
    v12_interaction_crop: v12.summary?.control_interaction_state_crop_census,
    v13_geometry_occlusion: {
      viewport_count: geometry.viewport_count,
      target_count: geometry.target_count,
      audit_count: geometry.audit_count,
      screenshot_count: geometry.screenshot_count,
      failure_count: geometry.failure_count,
      by_viewport: geometry.by_viewport,
      by_group: geometry.by_group,
      thresholds: geometry.thresholds,
    },
  },
  v12_ready: v12.status === "ready",
  geometry_occlusion_ready: geometry.status === "ready",
  geometry_occlusion_census: geometry,
};
fs.writeFileSync(outputPath, JSON.stringify(report, null, 2) + "\n");
console.log(JSON.stringify(report, null, 2));
NODE

cat "$REPORT_PATH"
