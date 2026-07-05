#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V18_REPORT_PATH:-}"
V17_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V17_REPORT_PATH:-}"
RESIZE_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V18_RESIZE_REPORT_PATH:-}"
RESIZE_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V18_SCREENSHOT_DIR:-}"
NATIVE_DIR="${HEPTA_NATIVE_FIXTURE_VISUAL_DIR:-}"
V17_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V18_V17_LOG:-}"
SKIP_V17="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V18_SKIP_V17:-0}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v18-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v18-resize-orientation-gate.json"
fi
if [[ -z "$V17_REPORT_PATH" ]]; then
  V17_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v17-touch-coarse-pointer-gate.json"
fi
if [[ -z "$RESIZE_REPORT_PATH" ]]; then
  RESIZE_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v18-resize-orientation-census.json"
fi
if [[ -z "$RESIZE_SCREENSHOT_DIR" ]]; then
  RESIZE_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v18-resize-orientation-screenshots"
fi
if [[ -z "$NATIVE_DIR" ]]; then
  NATIVE_DIR="$READINESS_DIR/native-fixture"
fi
if [[ -z "$V17_LOG" ]]; then
  V17_LOG="$READINESS_DIR/v17-touch-coarse-pointer.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$NATIVE_DIR" "$RESIZE_SCREENSHOT_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$RESIZE_REPORT_PATH")"

if [[ "$SKIP_V17" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V17_REPORT_PATH="$V17_REPORT_PATH" \
  HEPTA_NATIVE_FIXTURE_VISUAL_DIR="$NATIVE_DIR" \
    bash scripts/hepta-ui-harsh-top-design-referee-v17-touch-coarse-pointer-gate.sh "$READINESS_DIR" >"$V17_LOG" 2>&1 || {
      echo "v17 touch/coarse-pointer prerequisite failed" >&2
      tail -n 180 "$V17_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V17_REPORT_PATH")" != "ready" ]]; then
  echo "v17 touch/coarse-pointer prerequisite was not ready: $V17_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7444 7445 7446 7447 7448; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v18 referee" >&2
  exit 1
fi

BASE_URL="http://${BIND_ADDR}"
server_pid=""

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
}
trap cleanup EXIT

start_server() {
  : >"$SERVER_LOG"
  HEPTA_AUTOLOAD=0 HEPTA_AUTOSAVE=0 CARGO_INCREMENTAL=0 \
    cargo run --manifest-path "$MANIFEST" -q -p codex-cli --bin hepta -- --serve-ui "$BIND_ADDR" \
    >"$SERVER_LOG" 2>&1 &
  server_pid="$!"
}

wait_for_server() {
  local deadline=$((SECONDS + STARTUP_TIMEOUT_SEC))
  local root_probe=""
  until root_probe="$(curl -fsS "$BASE_URL/" 2>/dev/null)" && [[ "$root_probe" == *'data-rust-rendered-control-ui="true"'* ]]; do
    if ! kill -0 "$server_pid" 2>/dev/null; then
      echo "Hepta Control UI server exited before v18 resize/orientation audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$RESIZE_SCREENSHOT_DIR" >"$RESIZE_REPORT_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir] = process.argv.slice(2);
fs.mkdirSync(screenshotDir, { recursive: true });

const scenarios = [
  {
    name: "desktop-wide-to-narrow-short",
    initial: { width: 1365, height: 900, dpr: 2, railVisible: true, isMobile: false, hasTouch: false },
    resized: { width: 980, height: 640 },
  },
  {
    name: "narrow-to-phone-portrait",
    initial: { width: 768, height: 900, dpr: 2, railVisible: true, isMobile: true, hasTouch: true },
    resized: { width: 390, height: 844 },
  },
  {
    name: "mobile-portrait-to-landscape",
    initial: { width: 500, height: 844, dpr: 2, railVisible: false, isMobile: true, hasTouch: true },
    resized: { width: 844, height: 390 },
  },
  {
    name: "phone320-keyboard-shrink",
    initial: { width: 320, height: 700, dpr: 3, railVisible: false, isMobile: true, hasTouch: true },
    resized: { width: 320, height: 520 },
  },
];

const transientPanelSelector = [
  "[data-chat-row-menu-panel]",
  "[data-control-ui-thread-tools-panel]",
  "[data-control-ui-composer-tools-panel]",
  "[data-chat-composer-popover]",
  "#command-palette .command-palette",
].join(",");

const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase();
const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const round = (value, digits = 3) => Number(value.toFixed(digits));

function targetDefinitions(profile) {
  const targets = [];
  if (profile.railVisible) {
    for (const key of ["ui-chat-agent", "task-queue", "operator-plane"]) {
      targets.push({
        key: `row-menu-${key}`,
        group: "row-menu",
        triggerSelector: `[data-chat-row-menu-toggle="${key}"]`,
        revealSelector: `[data-chat-conversation="${key}"]`,
        panelSelector: `[data-chat-row-menu-panel="${key}"]`,
      });
    }
  }
  targets.push(
    {
      key: "thread-tools",
      group: "thread-tools",
      triggerSelector: '[data-thread-command-menu="true"] [data-control-ui-thread-tools-trigger="light-glass"]',
      panelSelector: '[data-control-ui-thread-tools-panel="light-glass"]',
    },
    {
      key: "composer-tools",
      group: "composer-tools",
      triggerSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tools-trigger="light-glass"]',
      panelSelector: '[data-control-ui-composer-tools-panel="light-glass"]',
    },
    {
      key: "composer-popover-artifact",
      group: "composer-popover",
      triggerSelector: '[data-chat-composer-popover-toggle="artifact"]',
      panelSelector: '[data-chat-composer-popover="artifact"]',
    },
    {
      key: "composer-popover-command",
      group: "composer-popover",
      triggerSelector: '[data-chat-composer-popover-toggle="command"]',
      panelSelector: '[data-chat-composer-popover="command"]',
    },
    {
      key: "command-palette",
      group: "command-palette",
      triggerSelector: '[data-control-ui-command-palette-trigger="light-glass"]',
      panelSelector: '#command-palette .command-palette',
    },
  );
  return targets;
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

async function topmostFor(locator) {
  return locator.evaluate((element) => {
    const rect = element.getBoundingClientRect();
    const insetX = Math.max(4, Math.min(16, rect.width / 3));
    const insetY = Math.max(4, Math.min(16, rect.height / 3));
    const points = [
      { x: rect.left + rect.width / 2, y: rect.top + rect.height / 2 },
      { x: rect.left + insetX, y: rect.top + insetY },
      { x: rect.right - insetX, y: rect.bottom - insetY },
    ].filter((point) => point.x >= 0 && point.y >= 0 && point.x <= window.innerWidth && point.y <= window.innerHeight);
    const hits = points.map((point) => document.elementFromPoint(point.x, point.y));
    return hits.length > 0 && hits.every((hit) => hit && (hit === element || element.contains(hit)));
  }).catch(() => false);
}

async function triggerGeometry(trigger, viewport) {
  const box = await boxFor(trigger);
  const failures = [];
  if (!box) {
    return { box: null, topmost: false, clipped_ratio: 0, failures: ["missing_trigger_box"], ready: false };
  }
  if (box.width + 0.5 < 44 || box.height + 0.5 < 44) failures.push("trigger_hit_target_below_44px");
  const clipped = clippedRatio(box, viewport);
  if (clipped < 0.985) failures.push("trigger_clipped_by_viewport");
  const topmost = await topmostFor(trigger);
  if (!topmost) failures.push("trigger_not_topmost_at_sample_points");
  return {
    box: roundedBox(box),
    clipped_ratio: round(clipped, 4),
    topmost,
    failures,
    ready: failures.length === 0,
  };
}

async function visibleTransientPanels(page) {
  return page.locator(transientPanelSelector).evaluateAll((elements) => elements.flatMap((element) => {
    const rect = element.getBoundingClientRect();
    const style = window.getComputedStyle(element);
    const visible = rect.width > 0
      && rect.height > 0
      && style.visibility !== "hidden"
      && style.display !== "none"
      && Number(style.opacity || 1) !== 0;
    if (!visible) return [];
    return [{
      selector_hint: element.getAttribute("data-chat-row-menu-panel")
        || element.getAttribute("data-control-ui-thread-tools-panel")
        || element.getAttribute("data-control-ui-composer-tools-panel")
        || element.getAttribute("data-chat-composer-popover")
        || element.getAttribute("data-control-ui-command-palette-surface")
        || element.id
        || element.className,
      box: {
        left: Number(rect.left.toFixed(2)),
        top: Number(rect.top.toFixed(2)),
        right: Number(rect.right.toFixed(2)),
        bottom: Number(rect.bottom.toFixed(2)),
        width: Number(rect.width.toFixed(2)),
        height: Number(rect.height.toFixed(2)),
      },
    }];
  }));
}

async function transientState(page) {
  const visiblePanels = await visibleTransientPanels(page);
  const state = await page.evaluate(() => ({
    open_tool_details_count: document.querySelectorAll('details[name="control-ui-tools-menu"][open]').length,
    open_composer_picker_count: document.querySelectorAll('details.tg-composer-picker[open]').length,
    composer_attr_open_count: document.querySelectorAll('[data-chat-composer-shell][data-chat-composer-popover-open]').length,
    row_menu_open_count: document.querySelectorAll('.tg-chat-item--menu-open').length,
    command_palette_hash_open: window.location.hash === "#command-palette",
    horizontal_overflow_px: Math.max(0, document.documentElement.scrollWidth - window.innerWidth),
    viewport: { width: window.innerWidth, height: window.innerHeight },
  }));
  return { ...state, visible_panels: visiblePanels, visible_panel_count: visiblePanels.length };
}

function tapPointForBox(box, viewport) {
  return {
    x: Math.max(1, Math.min(viewport.width - 1, round(box.left + box.width / 2, 1))),
    y: Math.max(1, Math.min(viewport.height - 1, round(box.top + box.height / 2, 1))),
  };
}

async function revealTarget(page, target) {
  if (!target.revealSelector) return;
  const row = page.locator(target.revealSelector).first();
  await row.scrollIntoViewIfNeeded();
  await page.waitForTimeout(70);
}

async function openTarget(page, scenario, target) {
  await revealTarget(page, target);
  const trigger = page.locator(target.triggerSelector).first();
  await trigger.waitFor({ state: "visible", timeout: 5000 });
  await trigger.evaluate((element) => element.scrollIntoView({ block: "center", inline: "center" })).catch(() => {});
  await page.waitForTimeout(140);
  const triggerBox = await boxFor(trigger);
  const geometry = await triggerGeometry(trigger, scenario.initial);
  const point = triggerBox ? tapPointForBox(triggerBox, scenario.initial) : null;
  if (point && scenario.initial.hasTouch) {
    await page.touchscreen.tap(point.x, point.y);
  } else if (point) {
    await page.mouse.click(point.x, point.y);
  }
  await page.waitForTimeout(260);
  const panel = page.locator(target.panelSelector).first();
  const panelVisible = await panel.waitFor({ state: "visible", timeout: 5000 }).then(() => true).catch(() => false);
  return { trigger_geometry: geometry, tap_point: point, panel_visible: panelVisible };
}

async function auditResizeTarget(browser, scenario, target) {
  const page = await browser.newPage({
    viewport: { width: scenario.initial.width, height: scenario.initial.height },
    deviceScaleFactor: scenario.initial.dpr,
    isMobile: scenario.initial.isMobile,
    hasTouch: scenario.initial.hasTouch,
    colorScheme: "light",
    reducedMotion: "reduce",
  });
  await page.goto(baseUrl, { waitUntil: "networkidle" });
  await page.waitForTimeout(180);
  const open = await openTarget(page, scenario, target);
  const beforeResizeState = await transientState(page);
  const beforePath = path.join(screenshotDir, `${sanitize(scenario.name)}-${sanitize(target.key)}-before-resize.png`);
  await page.screenshot({ path: beforePath, fullPage: false });
  await page.setViewportSize({ width: scenario.resized.width, height: scenario.resized.height });
  await page.waitForTimeout(460);
  const afterResizeState = await transientState(page);
  const afterPath = path.join(screenshotDir, `${sanitize(scenario.name)}-${sanitize(target.key)}-after-resize.png`);
  await page.screenshot({ path: afterPath, fullPage: false });
  await page.close();

  const failures = [
    ...open.trigger_geometry.failures.map((failure) => `trigger:${failure}`),
    ...(open.tap_point ? [] : ["missing_trigger_activation_point"]),
    ...(open.panel_visible ? [] : ["panel_not_visible_before_resize"]),
    ...(beforeResizeState.visible_panel_count === 1 ? [] : [`before_resize_expected_one_visible_panel_got_${beforeResizeState.visible_panel_count}`]),
    ...(afterResizeState.visible_panel_count === 0 ? [] : [`after_resize_residual_visible_panels_${afterResizeState.visible_panel_count}`]),
    ...(afterResizeState.open_tool_details_count === 0 ? [] : [`after_resize_open_tool_details_${afterResizeState.open_tool_details_count}`]),
    ...(afterResizeState.open_composer_picker_count === 0 ? [] : [`after_resize_open_composer_pickers_${afterResizeState.open_composer_picker_count}`]),
    ...(afterResizeState.composer_attr_open_count === 0 ? [] : [`after_resize_composer_attr_open_${afterResizeState.composer_attr_open_count}`]),
    ...(afterResizeState.row_menu_open_count === 0 ? [] : [`after_resize_row_menu_open_${afterResizeState.row_menu_open_count}`]),
    ...(!afterResizeState.command_palette_hash_open ? [] : ["after_resize_command_palette_hash_still_open"]),
    ...(afterResizeState.horizontal_overflow_px <= 1 ? [] : [`after_resize_horizontal_overflow_${afterResizeState.horizontal_overflow_px}`]),
  ];

  return {
    scenario: scenario.name,
    group: target.group,
    target: target.key,
    initial_viewport: scenario.initial,
    resized_viewport: scenario.resized,
    trigger: open.trigger_geometry,
    activation_point: open.tap_point,
    panel_visible_before_resize: open.panel_visible,
    before_resize_state: beforeResizeState,
    after_resize_state: afterResizeState,
    screenshots: {
      before_resize: { path: beforePath, sha256: sha256(beforePath) },
      after_resize: { path: afterPath, sha256: sha256(afterPath) },
    },
    failures,
    ready: failures.length === 0,
  };
}

function summarizeBy(items, key, countKey) {
  return Object.values(items.reduce((acc, item) => {
    const value = item[key];
    acc[value] ||= { [key]: value, [countKey]: 0, failure_count: 0 };
    acc[value][countKey] += 1;
    if (!item.ready) acc[value].failure_count += 1;
    return acc;
  }, {})).sort((a, b) => String(a[key]).localeCompare(String(b[key])));
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
  for (const scenario of scenarios) {
    for (const target of targetDefinitions(scenario.initial)) {
      audits.push(await auditResizeTarget(browser, scenario, target));
    }
  }
  await browser.close();

  const failures = audits.filter((audit) => !audit.ready);
  const expectedAuditCount = scenarios.reduce((sum, scenario) => sum + targetDefinitions(scenario.initial).length, 0);
  const ready = failures.length === 0 && audits.length === expectedAuditCount;
  const report = {
    schema_version: "hepta-ui-harsh-top-design-referee-v18-resize-orientation-census/v0",
    standards_version: "2026-06-29-control-open-transient-resize-orientation-zero-residual-census",
    status: ready ? "ready" : "failed",
    base_url: baseUrl,
    browser_path: "Browser plugin not available; regular Playwright with local Chrome was used",
    screenshot_dir: screenshotDir,
    scenario_count: scenarios.length,
    target_count: audits.length,
    expected_target_count: expectedAuditCount,
    screenshot_count: audits.length * 2,
    failure_count: failures.length,
    by_scenario: summarizeBy(audits, "scenario", "resize_audit_count"),
    by_group: summarizeBy(audits, "group", "resize_audit_count"),
    thresholds: {
      resize_scenarios: scenarios.map((scenario) => ({
        name: scenario.name,
        initial: scenario.initial,
        resized: scenario.resized,
      })),
      before_resize_visible_transient_panel_count: 1,
      after_resize_visible_transient_panel_count: 0,
      after_resize_open_details_count: 0,
      after_resize_open_composer_picker_count: 0,
      after_resize_row_menu_open_count: 0,
      after_resize_command_palette_hash_open: false,
      after_resize_horizontal_overflow_px_max: 1,
      trigger_min_size: "44x44",
      trigger_clipped_ratio_min: 0.985,
      trigger_topmost_sample_points: "center + diagonal inset",
    },
    failures,
    resize_audits: audits,
  };
  console.log(JSON.stringify(report, null, 2));
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
NODE

if [[ "$(jq -r '.status' "$RESIZE_REPORT_PATH")" != "ready" ]]; then
  echo "v18 resize/orientation census failed" >&2
  jq '.failures[:30] | map({scenario, target, group, before_resize_state, after_resize_state, failures})' "$RESIZE_REPORT_PATH" >&2 || true
  exit 1
fi

node - "$V17_REPORT_PATH" "$RESIZE_REPORT_PATH" "$REPORT_PATH" <<'NODE'
const crypto = require("node:crypto");
const fs = require("node:fs");

const [v17Path, resizePath, outputPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const v17 = readJson(v17Path);
const resize = readJson(resizePath);
const ready = v17.status === "ready"
  && resize.status === "ready"
  && resize.failure_count === 0
  && resize.target_count === resize.expected_target_count;
const report = {
  schema_version: "hepta-ui-harsh-top-design-referee-v18-gate/v0",
  standards_version: "2026-06-29-harsh-v17-plus-open-transient-resize-orientation-zero-residual-census",
  status: ready ? "ready" : "failed",
  browser_path: "Browser plugin not available; regular Playwright with local Chrome was used",
  inputs: {
    v17_touch_coarse_pointer: { path: v17Path, sha256: sha256(v17Path) },
    resize_orientation_census: { path: resizePath, sha256: sha256(resizePath) },
  },
  summary: {
    v17_touch_coarse_pointer: v17.summary?.v17_touch_coarse_pointer,
    v16_keyboard_focus: v17.summary?.v16_keyboard_focus,
    v15_text_zoom_squeeze: v17.summary?.v15_text_zoom_squeeze,
    v14_scroll_edge_crop: v17.summary?.v14_scroll_edge_crop,
    v13_geometry_occlusion: v17.summary?.v13_geometry_occlusion,
    v12_interaction_crop: v17.summary?.v12_interaction_crop,
    v18_resize_orientation: {
      scenario_count: resize.scenario_count,
      target_count: resize.target_count,
      expected_target_count: resize.expected_target_count,
      screenshot_count: resize.screenshot_count,
      failure_count: resize.failure_count,
      by_scenario: resize.by_scenario,
      by_group: resize.by_group,
      thresholds: resize.thresholds,
    },
  },
  v17_ready: v17.status === "ready",
  resize_orientation_ready: resize.status === "ready",
  resize_orientation_census: resize,
};
fs.writeFileSync(outputPath, JSON.stringify(report, null, 2) + "\n");
console.log(JSON.stringify(report, null, 2));
NODE
