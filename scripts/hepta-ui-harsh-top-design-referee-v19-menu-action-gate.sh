#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V19_REPORT_PATH:-}"
V18_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V18_REPORT_PATH:-}"
ACTION_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V19_ACTION_REPORT_PATH:-}"
ACTION_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V19_SCREENSHOT_DIR:-}"
NATIVE_DIR="${HEPTA_NATIVE_FIXTURE_VISUAL_DIR:-}"
V18_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V19_V18_LOG:-}"
SKIP_V18="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V19_SKIP_V18:-0}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v19-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v19-menu-action-gate.json"
fi
if [[ -z "$V18_REPORT_PATH" ]]; then
  V18_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v18-resize-orientation-gate.json"
fi
if [[ -z "$ACTION_REPORT_PATH" ]]; then
  ACTION_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v19-menu-action-census.json"
fi
if [[ -z "$ACTION_SCREENSHOT_DIR" ]]; then
  ACTION_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v19-menu-action-screenshots"
fi
if [[ -z "$NATIVE_DIR" ]]; then
  NATIVE_DIR="$READINESS_DIR/native-fixture"
fi
if [[ -z "$V18_LOG" ]]; then
  V18_LOG="$READINESS_DIR/v18-resize-orientation.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$NATIVE_DIR" "$ACTION_SCREENSHOT_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$ACTION_REPORT_PATH")"

if [[ "$SKIP_V18" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V18_REPORT_PATH="$V18_REPORT_PATH" \
  HEPTA_NATIVE_FIXTURE_VISUAL_DIR="$NATIVE_DIR" \
    bash scripts/hepta-ui-harsh-top-design-referee-v18-resize-orientation-gate.sh "$READINESS_DIR" >"$V18_LOG" 2>&1 || {
      echo "v18 resize/orientation prerequisite failed" >&2
      tail -n 180 "$V18_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V18_REPORT_PATH")" != "ready" ]]; then
  echo "v18 resize/orientation prerequisite was not ready: $V18_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7460 7461 7462 7463 7464; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v19 referee" >&2
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
    cargo run --manifest-path "$MANIFEST" -q -p hepta-cli --bin hepta -- --serve-ui "$BIND_ADDR" \
    >"$SERVER_LOG" 2>&1 &
  server_pid="$!"
}

wait_for_server() {
  local deadline=$((SECONDS + STARTUP_TIMEOUT_SEC))
  local root_probe=""
  until root_probe="$(curl -fsS "$BASE_URL/" 2>/dev/null)" && [[ "$root_probe" == *'data-rust-rendered-control-ui="true"'* ]]; do
    if ! kill -0 "$server_pid" 2>/dev/null; then
      echo "Hepta Control UI server exited before v19 menu action audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$ACTION_SCREENSHOT_DIR" >"$ACTION_REPORT_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir] = process.argv.slice(2);
fs.mkdirSync(screenshotDir, { recursive: true });

const scenarios = [
  {
    name: "desktop-menu-action",
    viewport: { width: 1365, height: 900, dpr: 2, railVisible: true, isMobile: false, hasTouch: false },
  },
  {
    name: "narrow-touch-menu-action",
    viewport: { width: 768, height: 900, dpr: 2, railVisible: true, isMobile: true, hasTouch: true },
  },
  {
    name: "mobile-menu-action",
    viewport: { width: 500, height: 844, dpr: 2, railVisible: false, isMobile: true, hasTouch: true },
  },
  {
    name: "phone320-menu-action",
    viewport: { width: 320, height: 700, dpr: 3, railVisible: false, isMobile: true, hasTouch: true },
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
        actionSelector: `[data-chat-row-menu-panel="${key}"] button[role="menuitem"]`,
        actionKind: "click",
      });
    }
  }
  targets.push(
    {
      key: "thread-tools",
      group: "thread-tools",
      triggerSelector: '[data-thread-command-menu="true"] [data-control-ui-thread-tools-trigger="light-glass"]',
      panelSelector: '[data-control-ui-thread-tools-panel="light-glass"]',
      actionSelector: '[data-control-ui-thread-tools-panel="light-glass"] a[role="menuitem"]',
      actionKind: "click",
    },
    {
      key: "composer-tools",
      group: "composer-tools",
      triggerSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tools-trigger="light-glass"]',
      panelSelector: '[data-control-ui-composer-tools-panel="light-glass"]',
      actionSelector: '[data-control-ui-composer-tools-panel="light-glass"] select',
      actionKind: "select",
    },
    {
      key: "composer-popover-artifact",
      group: "composer-popover",
      triggerSelector: '[data-chat-composer-popover-toggle="artifact"]',
      panelSelector: '[data-chat-composer-popover="artifact"]',
      actionSelector: '[data-chat-composer-popover="artifact"] button[role="menuitem"]',
      actionKind: "click",
    },
    {
      key: "composer-popover-command",
      group: "composer-popover",
      triggerSelector: '[data-chat-composer-popover-toggle="command"]',
      panelSelector: '[data-chat-composer-popover="command"]',
      actionSelector: '[data-chat-composer-popover="command"] button[role="menuitem"]',
      actionKind: "click",
    },
    {
      key: "command-palette",
      group: "command-palette",
      triggerSelector: '[data-control-ui-command-palette-trigger="light-glass"]',
      panelSelector: '#command-palette .command-palette',
      actionSelector: '#command-palette [data-control-ui-command-palette-item]',
      actionKind: "click",
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
    ];
    return points.every((point) => {
      if (point.x < 0 || point.y < 0 || point.x > innerWidth || point.y > innerHeight) return false;
      const top = document.elementFromPoint(point.x, point.y);
      return top === element || element.contains(top);
    });
  }).catch(() => false);
}

async function snapshotState(page) {
  return page.evaluate((selector) => {
    const visiblePanels = [...document.querySelectorAll(selector)]
      .map((element) => {
        const rect = element.getBoundingClientRect();
        const style = getComputedStyle(element);
        return {
          visible: rect.width > 1 && rect.height > 1 && style.visibility !== "hidden" && style.display !== "none" && Number(style.opacity) > 0.01,
          hint: element.getAttribute("data-chat-row-menu-panel") ||
            element.getAttribute("data-chat-composer-popover") ||
            element.getAttribute("data-control-ui-thread-tools-panel") ||
            element.getAttribute("data-control-ui-composer-tools-panel") ||
            element.getAttribute("data-control-ui-command-palette-surface") ||
            element.id ||
            element.className,
          box: {
            left: Number(rect.left.toFixed(2)),
            top: Number(rect.top.toFixed(2)),
            right: Number(rect.right.toFixed(2)),
            bottom: Number(rect.bottom.toFixed(2)),
            width: Number(rect.width.toFixed(2)),
            height: Number(rect.height.toFixed(2)),
          },
        };
      })
      .filter((panel) => panel.visible);
    const doc = document.documentElement;
    return {
      open_tool_details_count: document.querySelectorAll('details[name="control-ui-tools-menu"][open]').length,
      open_composer_picker_count: document.querySelectorAll("details.tg-composer-picker[open]").length,
      composer_attr_open_count: document.querySelectorAll("[data-chat-composer-shell][data-chat-composer-popover-open]").length,
      row_menu_open_count: document.querySelectorAll(".tg-chat-item--menu-open").length,
      command_palette_hash_open: window.location.hash === "#command-palette",
      horizontal_overflow_px: Math.max(0, doc.scrollWidth - window.innerWidth),
      visible_panels: visiblePanels,
      visible_panel_count: visiblePanels.length,
      viewport: { width: window.innerWidth, height: window.innerHeight },
    };
  }, transientPanelSelector);
}

async function screenshot(page, name) {
  const file = path.join(screenshotDir, `${sanitize(name)}.png`);
  await page.screenshot({ path: file, fullPage: false });
  return { path: file, sha256: sha256(file) };
}

async function activate(locator, kind) {
  if (kind === "select") {
    const options = await locator.locator("option").evaluateAll((nodes) => nodes.map((option, index) => ({ index, value: option.value })));
    const target = options.length > 1 ? options[1] : options[0];
    await locator.selectOption({ index: target.index });
    return { selected_index: target.index, selected_value: target.value };
  }
  await locator.click({ force: true });
  return {};
}

async function openTarget(page, scenario, target) {
  if (target.revealSelector) {
    const reveal = page.locator(target.revealSelector).first();
    await reveal.scrollIntoViewIfNeeded().catch(() => {});
    if (!scenario.viewport.hasTouch) await reveal.hover({ force: true }).catch(() => {});
  }
  const trigger = page.locator(target.triggerSelector).first();
  await trigger.scrollIntoViewIfNeeded().catch(() => {});
  const triggerBox = await boxFor(trigger);
  const triggerFailures = [];
  if (!triggerBox) {
    triggerFailures.push("trigger_missing_box");
  } else {
    if (triggerBox.width < 44 || triggerBox.height < 44) triggerFailures.push("trigger_too_small_for_44px_hit_target");
    if (clippedRatio(triggerBox, scenario.viewport) < 0.985) triggerFailures.push("trigger_clipped");
    if (!(await topmostFor(trigger))) triggerFailures.push("trigger_not_topmost_at_sample_points");
  }
  if (scenario.viewport.hasTouch && triggerBox) {
    await page.touchscreen.tap(triggerBox.left + triggerBox.width / 2, triggerBox.top + triggerBox.height / 2);
  } else {
    await trigger.click({ force: true });
  }
  await page.waitForTimeout(280);
  return { triggerBox, triggerFailures };
}

async function auditOne(page, scenario, target, actionIndex) {
  await page.goto(baseUrl, { waitUntil: "domcontentloaded" });
  await page.waitForTimeout(220);
  await page.mouse.move(5, 5).catch(() => {});
  const { triggerBox, triggerFailures } = await openTarget(page, scenario, target);
  const beforeState = await snapshotState(page);
  const action = page.locator(target.actionSelector).nth(actionIndex);
  const actionCount = await page.locator(target.actionSelector).count();
  await action.scrollIntoViewIfNeeded().catch(() => {});
  await page.waitForTimeout(120);
  const actionBox = await boxFor(action);
  const actionText = await action.evaluate((element) => (
    element.getAttribute("aria-label") ||
    element.getAttribute("title") ||
    element.textContent ||
    element.getAttribute("data-control-ui-command-palette-item") ||
    element.getAttribute("data-chat-row-menu-item") ||
    element.getAttribute("data-control-ui-menu-item") ||
    element.getAttribute("data-chat-composer-picker-item") ||
    element.getAttribute("data-control-ui-composer-tool-item") ||
    ""
  ).trim()).catch(() => "");
  const actionFailures = [];
  if (!actionBox) {
    actionFailures.push("action_item_missing_box");
  } else {
    if (actionBox.width < 44 || actionBox.height < 32) actionFailures.push("action_item_too_small_for_menu_target");
    if (clippedRatio(actionBox, scenario.viewport) < 0.985) actionFailures.push("action_item_clipped");
    if (!(await topmostFor(action))) actionFailures.push("action_item_not_topmost_at_sample_points");
  }
  const beforeScreenshot = await screenshot(page, `${scenario.name}-${target.key}-${actionIndex}-before-action`);
  let activation = {};
  if (!actionFailures.length) activation = await activate(action, target.actionKind);
  await page.waitForTimeout(520);
  const afterState = await snapshotState(page);
  const afterScreenshot = await screenshot(page, `${scenario.name}-${target.key}-${actionIndex}-after-action`);
  await page.keyboard.press("Escape");
  await page.waitForTimeout(180);
  const postDismissState = await snapshotState(page);
  const failures = [...triggerFailures, ...actionFailures];
  if (actionCount <= actionIndex) failures.push("action_item_missing");
  if (beforeState.visible_panel_count !== 1) failures.push(`before_action_visible_panels_${beforeState.visible_panel_count}`);
  if (afterState.visible_panel_count !== 1) failures.push(`after_action_expected_one_visible_panel_got_${afterState.visible_panel_count}`);
  if (postDismissState.visible_panel_count !== 0) failures.push(`escape_after_action_residual_visible_panels_${postDismissState.visible_panel_count}`);
  if (afterState.open_tool_details_count !== 0) failures.push(`after_action_open_tool_details_${afterState.open_tool_details_count}`);
  if (afterState.open_composer_picker_count !== 0) failures.push(`after_action_open_composer_pickers_${afterState.open_composer_picker_count}`);
  if (afterState.composer_attr_open_count !== 0) failures.push(`after_action_composer_attr_open_${afterState.composer_attr_open_count}`);
  if (afterState.row_menu_open_count !== 0) failures.push(`after_action_row_menu_open_${afterState.row_menu_open_count}`);
  if (afterState.command_palette_hash_open) failures.push("after_action_command_palette_hash_still_open");
  if (afterState.horizontal_overflow_px > 1) failures.push(`after_action_horizontal_overflow_${afterState.horizontal_overflow_px}`);
  return {
    scenario: scenario.name,
    group: target.group,
    target: target.key,
    action_kind: target.actionKind,
    action_index: actionIndex,
    action_text: actionText,
    action_count: actionCount,
    viewport: scenario.viewport,
    trigger: {
      box: roundedBox(triggerBox),
      clipped_ratio: triggerBox ? round(clippedRatio(triggerBox, scenario.viewport), 4) : 0,
      failures: triggerFailures,
      ready: triggerFailures.length === 0,
    },
    action_item: {
      box: roundedBox(actionBox),
      clipped_ratio: actionBox ? round(clippedRatio(actionBox, scenario.viewport), 4) : 0,
      failures: actionFailures,
      ready: actionFailures.length === 0,
    },
    activation,
    before_action_state: beforeState,
    after_action_state: afterState,
    post_escape_state: postDismissState,
    screenshots: { before_action: beforeScreenshot, after_action: afterScreenshot },
    failures,
    ready: failures.length === 0,
  };
}

function groupBy(items, key) {
  const map = new Map();
  for (const item of items) {
    const value = item[key];
    const current = map.get(value) || { [key]: value, item_action_audit_count: 0, failure_count: 0 };
    current.item_action_audit_count += 1;
    if (!item.ready) current.failure_count += 1;
    map.set(value, current);
  }
  return [...map.values()].sort((a, b) => String(a[key]).localeCompare(String(b[key])));
}

(async () => {
  const browser = await chromium.launch({
    executablePath: chromeBin,
    headless: true,
    args: [
      "--no-sandbox",
      "--disable-background-networking",
      "--disable-component-update",
      "--disable-default-apps",
      "--disable-extensions",
        "--disable-sync",
        "--hide-scrollbars",
        "--blink-settings=primaryHoverType=2,availableHoverTypes=2,primaryPointerType=4,availablePointerTypes=4",
        "--no-default-browser-check",
        "--no-first-run",
    ],
  });
  const records = [];
  try {
    for (const scenario of scenarios) {
      const context = await browser.newContext({
        viewport: { width: scenario.viewport.width, height: scenario.viewport.height },
        deviceScaleFactor: scenario.viewport.dpr,
        isMobile: scenario.viewport.isMobile,
        hasTouch: scenario.viewport.hasTouch,
      });
      const page = await context.newPage();
      for (const target of targetDefinitions(scenario.viewport)) {
        await page.goto(baseUrl, { waitUntil: "domcontentloaded" });
        await page.waitForTimeout(180);
        await page.mouse.move(5, 5).catch(() => {});
        await openTarget(page, scenario, target);
        const actionCount = await page.locator(target.actionSelector).count();
        await page.goto(baseUrl, { waitUntil: "domcontentloaded" });
        const boundedCount = Math.min(actionCount, 32);
        if (boundedCount === 0) {
          records.push({
            scenario: scenario.name,
            group: target.group,
            target: target.key,
            action_kind: target.actionKind,
            action_index: 0,
            action_count: 0,
            viewport: scenario.viewport,
            failures: ["no_action_items_found"],
            ready: false,
          });
        }
        for (let index = 0; index < boundedCount; index += 1) {
          records.push(await auditOne(page, scenario, target, index));
        }
      }
      await context.close();
    }
  } finally {
    await browser.close();
  }
  const failures = records.filter((record) => !record.ready);
  const summary = {
    scenario_count: scenarios.length,
    target_count: scenarios.reduce((sum, scenario) => sum + targetDefinitions(scenario.viewport).length, 0),
    item_action_audit_count: records.length,
    screenshot_count: records.reduce((sum, record) => sum + (record.screenshots ? 2 : 0), 0),
    failure_count: failures.length,
    by_scenario: groupBy(records, "scenario"),
    by_group: groupBy(records, "group"),
    by_action_kind: groupBy(records, "action_kind"),
    thresholds: {
      before_action_visible_transient_panel_count: 1,
      after_action_visible_transient_panel_count: 1,
      escape_after_action_visible_transient_panel_count: 0,
      after_action_open_details_count: 0,
      after_action_open_composer_picker_count: 0,
      after_action_row_menu_open_count: 0,
      after_action_command_palette_hash_open: false,
      after_action_horizontal_overflow_px_max: 1,
      trigger_min_size: "44x44",
      action_item_min_size: "44x32",
      trigger_or_item_clipped_ratio_min: 0.985,
      topmost_sample_points: "center + diagonal inset",
    },
  };
  console.log(JSON.stringify({
    schema_version: "hepta-ui-harsh-top-design-referee-v19-menu-action-census/v0",
    standards_version: "2026-07-11-harsh-v18-plus-popover-item-action-escape-dismiss-census",
    status: failures.length === 0 ? "ready" : "failed",
    browser_path: "Browser plugin not available; regular Playwright with local Chrome was used",
    summary,
    failures,
    records,
  }, null, 2));
})().catch((error) => {
  console.error(error);
  process.exit(1);
});
NODE

ACTION_STATUS="$(jq -r '.status' "$ACTION_REPORT_PATH")"

jq -n \
  --arg v18_path "$V18_REPORT_PATH" \
  --arg v18_sha "$(shasum -a 256 "$V18_REPORT_PATH" | awk '{print $1}')" \
  --arg action_path "$ACTION_REPORT_PATH" \
  --arg action_sha "$(shasum -a 256 "$ACTION_REPORT_PATH" | awk '{print $1}')" \
  --slurpfile v18 "$V18_REPORT_PATH" \
  --slurpfile action "$ACTION_REPORT_PATH" \
  '{
    schema_version: "hepta-ui-harsh-top-design-referee-v19-gate/v0",
    standards_version: "2026-06-29-harsh-v18-plus-submenu-item-action-zero-residual-census",
    status: (if $action[0].status == "ready" then "ready" else "failed" end),
    browser_path: "Browser plugin not available; regular Playwright with local Chrome was used",
    inputs: {
      v18_resize_orientation: { path: $v18_path, sha256: $v18_sha },
      menu_action_census: { path: $action_path, sha256: $action_sha }
    },
    summary: {
      v18_resize_orientation: $v18[0].summary.v18_resize_orientation,
      v17_touch_coarse_pointer: $v18[0].summary.v17_touch_coarse_pointer,
      v16_keyboard_focus: $v18[0].summary.v16_keyboard_focus,
      v15_text_zoom_squeeze: $v18[0].summary.v15_text_zoom_squeeze,
      v14_scroll_edge_crop: $v18[0].summary.v14_scroll_edge_crop,
      v13_geometry_occlusion: $v18[0].summary.v13_geometry_occlusion,
      v12_interaction_state_crop: $v18[0].summary.v12_interaction_state_crop,
      v19_menu_action: $action[0].summary
    },
    menu_action_census: $action[0]
  }' >"$REPORT_PATH"

cat "$REPORT_PATH"

if [[ "$ACTION_STATUS" != "ready" ]]; then
  echo "v19 menu action lifecycle audit failed: $ACTION_REPORT_PATH" >&2
  jq '.summary, .failures[0:12]' "$ACTION_REPORT_PATH" >&2 || true
  exit 1
fi
