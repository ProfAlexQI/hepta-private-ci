#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

source scripts/lib/hepta-ui-rust-toolchain.sh
source scripts/lib/hepta-control-ui-runtime-fixture.sh

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V31_REPORT_PATH:-}"
V31_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V31_CENSUS_PATH:-}"
V31_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V31_SCREENSHOT_DIR:-}"
V30_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V30_REPORT_PATH:-}"
V30_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V31_V30_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
SKIP_V30="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V31_SKIP_V30:-0}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v31-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v31-keyboard-flow-gate.json"
fi
if [[ -z "$V31_CENSUS_PATH" ]]; then
  V31_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v31-keyboard-flow-census.json"
fi
if [[ -z "$V31_SCREENSHOT_DIR" ]]; then
  V31_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v31-keyboard-flow-screenshots"
fi
if [[ -z "$V30_REPORT_PATH" ]]; then
  V30_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v30-state-semantics-gate.json"
fi
if [[ -z "$V30_LOG" ]]; then
  V30_LOG="$READINESS_DIR/v30-state-semantics-prerequisite.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$V31_SCREENSHOT_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V31_CENSUS_PATH")"

if [[ "$SKIP_V30" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V30_REPORT_PATH="$V30_REPORT_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v30-state-semantics-gate.sh "$READINESS_DIR" >"$V30_LOG" 2>&1 || {
      echo "v30 state-semantics prerequisite failed" >&2
      tail -n 180 "$V30_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V30_REPORT_PATH")" != "ready" ]]; then
  echo "v30 state-semantics prerequisite was not ready: $V30_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7670 7671 7672 7673 7674; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v31 referee" >&2
  exit 1
fi

hepta_control_ui_runtime_fixture_init
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
      echo "Hepta Control UI server exited before v31 keyboard flow audit was ready" >&2
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

set +e
node - "$CHROME_BIN" "$BASE_URL/" "$V31_SCREENSHOT_DIR" "$READINESS_DIR" >"$V31_CENSUS_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir, readinessDir] = process.argv.slice(2);
fs.mkdirSync(screenshotDir, { recursive: true });

const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase() || "item";
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const paths = { v30Gate: path.join(readinessDir, "ui-harsh-top-design-referee-v30-state-semantics-gate.json") };

const scenarios = [
  { name: "desktop-keyboard-flow", viewport: { width: 1365, height: 900, dpr: 2, railVisible: true, isMobile: false, hasTouch: false } },
  { name: "narrow-touch-keyboard-flow", viewport: { width: 768, height: 900, dpr: 2, railVisible: true, isMobile: true, hasTouch: true } },
  { name: "phone320-keyboard-flow", viewport: { width: 320, height: 700, dpr: 3, railVisible: false, isMobile: true, hasTouch: true } },
];

const targetGroups = [
  {
    key: "row-menu",
    railOnly: true,
    triggerSelector: "[data-chat-row-menu-toggle]",
    panelForTrigger: (trigger) => `[data-chat-row-menu-panel="${trigger.getAttribute("data-chat-row-menu-toggle")}"]`,
    expectedPopup: "menu",
    expectedFocus: "menuitem",
  },
  {
    key: "thread-tools",
    triggerSelector: "[data-control-ui-thread-tools-trigger='light-glass']",
    panelSelector: "[data-control-ui-thread-tools-panel='light-glass']",
    expectedPopup: "menu",
    expectedFocus: "menuitem",
  },
  {
    key: "composer-tools",
    triggerSelector: "[data-control-ui-composer-tools-trigger='light-glass']",
    panelSelector: "[data-control-ui-composer-tools-panel='light-glass']",
    expectedPopup: "menu",
    expectedFocus: "focusable-control",
  },
  {
    key: "artifact-popover",
    triggerSelector: "[data-chat-composer-popover-toggle='artifact']",
    panelSelector: "[data-chat-composer-popover='artifact']",
    expectedPopup: "menu",
    expectedFocus: "search-or-menuitem",
  },
  {
    key: "command-popover",
    triggerSelector: "[data-chat-composer-popover-toggle='command']",
    panelSelector: "[data-chat-composer-popover='command']",
    expectedPopup: "menu",
    expectedFocus: "search-or-menuitem",
  },
  {
    key: "command-palette",
    triggerSelector: "[data-control-ui-command-palette-trigger='light-glass']",
    panelSelector: "[data-control-ui-command-palette-surface='light-glass']",
    expectedPopup: "dialog",
    expectedFocus: "search",
  },
];

function missingInputs() {
  return Object.entries(paths)
    .filter(([, file]) => !fs.existsSync(file))
    .map(([key, file]) => ({ code: "missing_input", key, file }));
}

async function setProfile(page, profile) {
  await page.setViewportSize({ width: profile.width, height: profile.height });
  await page.emulateMedia({ reducedMotion: "no-preference" });
  await page.goto(baseUrl, { waitUntil: "networkidle" });
  await page.waitForSelector('[data-rust-rendered-control-ui="true"]', { timeout: 30000 });
}

async function screenshot(page, label) {
  const file = path.join(screenshotDir, `${sanitize(label)}.png`);
  await page.screenshot({ path: file, fullPage: false });
  return { path: file, sha256: sha256(file) };
}

async function closeTransient(page) {
  await page.keyboard.press("Escape").catch(() => {});
  await page.locator("body").click({ position: { x: 4, y: 4 } }).catch(() => {});
  await page.waitForTimeout(120);
}

async function visibleTriggerIndexes(page, selector) {
  return page.locator(selector).evaluateAll((nodes) => nodes.map((node, index) => {
    const rect = node.getBoundingClientRect();
    const style = window.getComputedStyle(node);
    const visible = rect.width > 1 && rect.height > 1 && style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity) > 0.01;
    return { index, visible };
  }).filter((item) => item.visible).map((item) => item.index));
}

async function auditTrigger(page, scenario, group, triggerIndex) {
  const trigger = page.locator(group.triggerSelector).nth(triggerIndex);
  await closeTransient(page);
  await trigger.scrollIntoViewIfNeeded().catch(() => {});
  await page.waitForTimeout(50);
  await trigger.focus();
  await page.waitForTimeout(50);

  const panelSelectorForTrigger = group.panelSelector || await trigger.evaluate((node) => {
    const key = node.getAttribute("data-chat-row-menu-toggle") || "";
    return key ? `[data-chat-row-menu-panel="${key}"]` : "";
  });
  const wireGroup = { ...group, panelSelector: panelSelectorForTrigger };
  delete wireGroup.panelForTrigger;

  const before = await trigger.evaluate((node, group) => {
    const active = document.activeElement;
    const panel = group.panelSelector ? document.querySelector(group.panelSelector) : null;
    const panelRect = panel?.getBoundingClientRect();
    const panelStyle = panel ? window.getComputedStyle(panel) : null;
    const panelVisible = panel && panelRect.width > 1 && panelRect.height > 1 && panelStyle.display !== "none" && panelStyle.visibility !== "hidden" && Number(panelStyle.opacity) > 0.01;
    return {
      label: node.getAttribute("aria-label") || node.getAttribute("title") || node.textContent.trim().replace(/\s+/g, " ").slice(0, 80) || group.key,
      triggerId: node.id || "",
      triggerTag: node.tagName.toLowerCase(),
      triggerFocused: active === node,
      ariaExpanded: node.getAttribute("aria-expanded") || "",
      nativePopoverTargetMatches: node.popoverTargetElement === panel || Boolean(node.popoverTargetElement?.contains(panel)),
      nativePopoverOpen: Boolean(node.popoverTargetElement?.matches(":popover-open")),
      ariaHaspopup: node.getAttribute("aria-haspopup") || "",
      ariaControls: node.getAttribute("aria-controls") || "",
      panelFound: Boolean(panel),
      panelRole: panel?.getAttribute("role") || "",
      panelVisible: Boolean(panelVisible),
    };
  }, wireGroup);

  const failures = [];
  if (!before.triggerFocused) failures.push("trigger_not_keyboard_focusable");
  const usesNativePopover = before.nativePopoverTargetMatches;
  if (usesNativePopover ? before.nativePopoverOpen : before.ariaExpanded !== "false") failures.push("initial_expanded_state_not_false");
  if (before.ariaHaspopup !== group.expectedPopup) failures.push("trigger_missing_popup_semantics");
  if (!before.panelFound) failures.push("controlled_panel_missing");
  if (before.panelVisible) failures.push("initial_keyboard_focus_panel_visible");

  await page.keyboard.press("Enter");
  await page.waitForTimeout(220);

  const openState = await trigger.evaluate((node, group) => {
    const panel = group.panelSelector ? document.querySelector(group.panelSelector) : null;
    const active = document.activeElement;
    const panelRect = panel?.getBoundingClientRect();
    const panelStyle = panel ? window.getComputedStyle(panel) : null;
    const panelVisible = panel && panelRect.width > 1 && panelRect.height > 1 && panelStyle.display !== "none" && panelStyle.visibility !== "hidden" && Number(panelStyle.opacity) > 0.01;
    const focusableSelector = 'input:not([type="hidden"]), select, textarea, button:not([disabled]), a[href], [tabindex]:not([tabindex="-1"]), [role="menuitem"]';
    const firstFocusable = panel?.querySelector(focusableSelector) || null;
    const menuItem = active instanceof Element ? active.closest('[role="menuitem"]') : null;
    const activeRect = active instanceof Element ? active.getBoundingClientRect() : null;
    const activeStyle = active instanceof Element ? window.getComputedStyle(active) : null;
    const activeVisible = activeRect && activeRect.width > 1 && activeRect.height > 1 && activeStyle.display !== "none" && activeStyle.visibility !== "hidden" && Number(activeStyle.opacity) > 0.01;
    return {
      ariaExpanded: node.getAttribute("aria-expanded") || "",
      nativePopoverTargetMatches: node.popoverTargetElement === panel || Boolean(node.popoverTargetElement?.contains(panel)),
      nativePopoverOpen: Boolean(node.popoverTargetElement?.matches(":popover-open")),
      panelVisible: Boolean(panelVisible),
      activeTag: active?.tagName?.toLowerCase?.() || "",
      activeId: active?.id || "",
      activeRole: active instanceof Element ? active.getAttribute("role") || "" : "",
      activeType: active instanceof HTMLInputElement ? active.type : "",
      activeLabel: active instanceof Element ? active.getAttribute("aria-label") || active.getAttribute("title") || active.textContent.trim().replace(/\s+/g, " ").slice(0, 80) : "",
      activeInsidePanel: Boolean(panel && active instanceof Element && panel.contains(active)),
      activeIsTrigger: active === node,
      activeVisible: Boolean(activeVisible),
      activeClosestMenuItemLabel: menuItem?.getAttribute("aria-label") || menuItem?.getAttribute("title") || menuItem?.textContent.trim().replace(/\s+/g, " ").slice(0, 80) || "",
      firstFocusableTag: firstFocusable?.tagName?.toLowerCase?.() || "",
      firstFocusableRole: firstFocusable?.getAttribute("role") || "",
      firstFocusableLabel: firstFocusable?.getAttribute("aria-label") || firstFocusable?.getAttribute("title") || firstFocusable?.textContent.trim().replace(/\s+/g, " ").slice(0, 80) || "",
    };
  }, wireGroup);

  if (usesNativePopover ? !openState.nativePopoverOpen : openState.ariaExpanded !== "true") failures.push("keyboard_open_expanded_state_not_true");
  if (!openState.panelVisible) failures.push("keyboard_open_panel_not_visible");
  if (!openState.activeInsidePanel) failures.push("keyboard_open_focus_not_inside_panel");
  if (openState.activeIsTrigger) failures.push("keyboard_open_focus_stayed_on_trigger");
  if (!openState.activeVisible) failures.push("keyboard_open_focus_not_visible");
  if (group.expectedFocus === "search" && !(openState.activeTag === "input" && openState.activeType === "search")) failures.push("keyboard_open_focus_not_search_input");
  if (group.expectedFocus === "menuitem" && !openState.activeClosestMenuItemLabel) failures.push("keyboard_open_focus_not_menuitem");
  if (group.expectedFocus === "search-or-menuitem" && !(openState.activeTag === "input" || openState.activeClosestMenuItemLabel)) failures.push("keyboard_open_focus_not_search_or_menuitem");
  if (group.expectedFocus === "focusable-control" && !["input", "select", "textarea", "button", "a"].includes(openState.activeTag) && !openState.activeClosestMenuItemLabel) failures.push("keyboard_open_focus_not_control");

  const shot = await screenshot(page, `${scenario.name}-${group.key}-${triggerIndex}`);

  await page.keyboard.press("Escape");
  await page.waitForTimeout(220);
  const closeState = await trigger.evaluate((node, group) => {
    const panel = group.panelSelector ? document.querySelector(group.panelSelector) : null;
    const active = document.activeElement;
    const panelRect = panel?.getBoundingClientRect();
    const panelStyle = panel ? window.getComputedStyle(panel) : null;
    const panelVisible = panel && panelRect.width > 1 && panelRect.height > 1 && panelStyle.display !== "none" && panelStyle.visibility !== "hidden" && Number(panelStyle.opacity) > 0.01;
    return {
      ariaExpanded: node.getAttribute("aria-expanded") || "",
      nativePopoverTargetMatches: node.popoverTargetElement === panel || Boolean(node.popoverTargetElement?.contains(panel)),
      nativePopoverOpen: Boolean(node.popoverTargetElement?.matches(":popover-open")),
      panelVisible: Boolean(panelVisible),
      focusReturnedToTrigger: active === node,
      activeTag: active?.tagName?.toLowerCase?.() || "",
      activeId: active?.id || "",
      activeLabel: active instanceof Element ? active.getAttribute("aria-label") || active.getAttribute("title") || active.textContent.trim().replace(/\s+/g, " ").slice(0, 80) : "",
    };
  }, wireGroup);

  if (usesNativePopover ? closeState.nativePopoverOpen : closeState.ariaExpanded !== "false") failures.push("escape_close_expanded_state_not_false");
  if (closeState.panelVisible) failures.push("escape_close_panel_still_visible");
  if (!closeState.focusReturnedToTrigger) failures.push("escape_focus_not_returned_to_trigger");

  return {
    scenario: scenario.name,
    group: group.key,
    trigger_index: triggerIndex,
    label: before.label,
    before,
    open_state: openState,
    close_state: closeState,
    screenshot: shot,
    failures: [...new Set(failures)],
  };
}

(async () => {
  const failures = missingInputs();
  const inputs = Object.fromEntries(Object.entries(paths).map(([key, file]) => [key, { path: file, sha256: fs.existsSync(file) ? sha256(file) : null }]));
  const v30Gate = failures.length ? null : readJson(paths.v30Gate);
  if (v30Gate && v30Gate.status !== "ready") failures.push({ code: "v30_gate_not_ready", status: v30Gate.status });

  const browser = await chromium.launch({ executablePath: chromeBin, headless: true, args: ["--no-sandbox", "--disable-gpu"] });
  const keyboardAudits = [];
  try {
    for (const scenario of scenarios) {
      const context = await browser.newContext({
        viewport: { width: scenario.viewport.width, height: scenario.viewport.height },
        deviceScaleFactor: scenario.viewport.dpr,
        isMobile: scenario.viewport.isMobile,
        hasTouch: scenario.viewport.hasTouch,
      });
      const page = await context.newPage();
      await setProfile(page, scenario.viewport);
      for (const group of targetGroups) {
        if (group.railOnly && !scenario.viewport.railVisible) continue;
        const indexes = await visibleTriggerIndexes(page, group.triggerSelector);
        if (!indexes.length) {
          failures.push({ code: "missing_visible_keyboard_trigger", scenario: scenario.name, group: group.key, selector: group.triggerSelector });
          continue;
        }
        for (const index of indexes.slice(0, group.key === "row-menu" ? 3 : 1)) {
          const audit = await auditTrigger(page, scenario, group, index);
          keyboardAudits.push(audit);
          for (const failure of audit.failures) failures.push({ code: failure, audit });
        }
      }
      await context.close();
    }
  } finally {
    await browser.close();
  }

  const countFailures = (prefixes) => failures.filter((failure) => prefixes.some((prefix) => String(failure.code || "").includes(prefix))).length;
  const output = {
    status: failures.length ? "failed" : "ready",
    generated_at: new Date().toISOString(),
    inputs,
    summary: {
      scenario_count: scenarios.length,
      keyboard_flow_audit_count: keyboardAudits.length,
      screenshot_count: keyboardAudits.filter((audit) => audit.screenshot).length,
      trigger_focusable_failure_count: countFailures(["trigger_not_keyboard_focusable"]),
      keyboard_open_failure_count: countFailures(["keyboard_open_"]),
      initial_focus_failure_count: countFailures(["initial_"]),
      escape_close_failure_count: countFailures(["escape_close_"]),
      focus_return_failure_count: countFailures(["escape_focus_"]),
      missing_trigger_failure_count: countFailures(["missing_visible_keyboard_trigger"]),
      failure_count: failures.length,
      thresholds: {
        keyboard_open_must_focus_control_inside_panel: true,
        menus_must_focus_a_menuitem_or_panel_control: true,
        dialogs_must_focus_search_input: true,
        escape_must_close_panel_and_restore_trigger_focus: true,
        browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome was used.",
      },
    },
    audits: keyboardAudits,
    failures,
  };
  console.log(JSON.stringify(output, null, 2));
  process.exitCode = failures.length ? 1 : 0;
})();
NODE
node_status=$?
set -e
cp "$V31_CENSUS_PATH" "$REPORT_PATH"
if [[ "$node_status" -ne 0 ]]; then
  echo "v31 keyboard flow referee failed: $REPORT_PATH" >&2
  jq '.summary, (.failures[:24])' "$REPORT_PATH" >&2 || true
  exit "$node_status"
fi

echo "v31 keyboard flow referee ready: $REPORT_PATH"
jq '.summary' "$REPORT_PATH"
