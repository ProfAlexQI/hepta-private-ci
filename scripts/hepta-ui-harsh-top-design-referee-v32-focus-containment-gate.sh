#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V32_REPORT_PATH:-}"
V32_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V32_CENSUS_PATH:-}"
V32_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V32_SCREENSHOT_DIR:-}"
V31_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V31_REPORT_PATH:-}"
V31_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V32_V31_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
SKIP_V31="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V32_SKIP_V31:-0}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v32-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v32-focus-containment-gate.json"
fi
if [[ -z "$V32_CENSUS_PATH" ]]; then
  V32_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v32-focus-containment-census.json"
fi
if [[ -z "$V32_SCREENSHOT_DIR" ]]; then
  V32_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v32-focus-containment-screenshots"
fi
if [[ -z "$V31_REPORT_PATH" ]]; then
  V31_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v31-keyboard-flow-gate.json"
fi
if [[ -z "$V31_LOG" ]]; then
  V31_LOG="$READINESS_DIR/v31-keyboard-flow-prerequisite.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$V32_SCREENSHOT_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V32_CENSUS_PATH")"

if [[ "$SKIP_V31" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V31_REPORT_PATH="$V31_REPORT_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v31-keyboard-flow-gate.sh "$READINESS_DIR" >"$V31_LOG" 2>&1 || {
      echo "v31 keyboard-flow prerequisite failed" >&2
      tail -n 180 "$V31_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V31_REPORT_PATH")" != "ready" ]]; then
  echo "v31 keyboard-flow prerequisite was not ready: $V31_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7680 7681 7682 7683 7684; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v32 referee" >&2
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
      echo "Hepta Control UI server exited before v32 focus containment audit was ready" >&2
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
node - "$CHROME_BIN" "$BASE_URL/" "$V32_SCREENSHOT_DIR" "$READINESS_DIR" "$V31_REPORT_PATH" >"$V32_CENSUS_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir, readinessDir, v31ReportPath] = process.argv.slice(2);
fs.mkdirSync(screenshotDir, { recursive: true });

const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase() || "item";
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const paths = { v31Gate: v31ReportPath || path.join(readinessDir, "ui-harsh-top-design-referee-v31-keyboard-flow-gate.json") };

const scenarios = [
  { name: "desktop-focus-containment", viewport: { width: 1365, height: 900, dpr: 2, railVisible: true, isMobile: false, hasTouch: false } },
  { name: "narrow-touch-focus-containment", viewport: { width: 768, height: 900, dpr: 2, railVisible: true, isMobile: true, hasTouch: true } },
  { name: "phone320-focus-containment", viewport: { width: 320, height: 700, dpr: 3, railVisible: false, isMobile: true, hasTouch: true } },
];

const targetGroups = [
  { key: "row-menu", railOnly: true, triggerSelector: "[data-chat-row-menu-toggle]", expectedPopup: "menu" },
  { key: "thread-tools", triggerSelector: "[data-control-ui-thread-tools-trigger='light-glass']", panelSelector: "[data-control-ui-thread-tools-panel='light-glass']", expectedPopup: "menu" },
  { key: "composer-tools", triggerSelector: "[data-control-ui-composer-tools-trigger='light-glass']", panelSelector: "[data-control-ui-composer-tools-panel='light-glass']", expectedPopup: "menu" },
  { key: "artifact-popover", triggerSelector: "[data-chat-composer-popover-toggle='artifact']", panelSelector: "[data-chat-composer-popover='artifact']", expectedPopup: "menu" },
  { key: "command-popover", triggerSelector: "[data-chat-composer-popover-toggle='command']", panelSelector: "[data-chat-composer-popover='command']", expectedPopup: "menu" },
  { key: "command-palette", triggerSelector: "[data-control-ui-command-palette-trigger='light-glass']", panelSelector: "[data-control-ui-command-palette-surface='light-glass']", expectedPopup: "dialog" },
];

const missingInputs = () => Object.entries(paths)
  .filter(([, file]) => !fs.existsSync(file))
  .map(([key, file]) => ({ code: "missing_input", key, file }));

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

async function resolvePanelSelector(trigger, group) {
  if (group.panelSelector) return group.panelSelector;
  return trigger.evaluate((node) => {
    const key = node.getAttribute("data-chat-row-menu-toggle") || "";
    return key ? `[data-chat-row-menu-panel="${key}"]` : "";
  });
}

async function stateFor(page, panelSelector, triggerSelector, triggerIndex) {
  return page.evaluate(({ panelSelector, triggerSelector, triggerIndex }) => {
    const trigger = document.querySelectorAll(triggerSelector)[triggerIndex] || null;
    const panel = panelSelector ? document.querySelector(panelSelector) : null;
    const active = document.activeElement;
    const rect = panel?.getBoundingClientRect();
    const style = panel ? window.getComputedStyle(panel) : null;
    const visible = panel && rect.width > 1 && rect.height > 1 && style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity) > 0.01;
    const activeRect = active instanceof Element ? active.getBoundingClientRect() : null;
    const activeStyle = active instanceof Element ? window.getComputedStyle(active) : null;
    const activeVisible = activeRect && activeRect.width > 1 && activeRect.height > 1 && activeStyle.display !== "none" && activeStyle.visibility !== "hidden" && Number(activeStyle.opacity) > 0.01;
    const focusableSelector = 'input:not([type="hidden"]):not([disabled]), select:not([disabled]), textarea:not([disabled]), button:not([disabled]), a[href], [tabindex]:not([tabindex="-1"])';
    const focusables = panel ? [...panel.querySelectorAll(focusableSelector)].filter((item) => {
      const itemRect = item.getBoundingClientRect();
      const itemStyle = window.getComputedStyle(item);
      return itemRect.width > 1 && itemRect.height > 1 && itemStyle.display !== "none" && itemStyle.visibility !== "hidden" && Number(itemStyle.opacity) > 0.01;
    }) : [];
    return {
      panelFound: Boolean(panel),
      panelVisible: Boolean(visible),
      activeInsidePanel: Boolean(panel && active instanceof Element && panel.contains(active)),
      activeIsTrigger: active === trigger,
      activeVisible: Boolean(activeVisible),
      activeTag: active?.tagName?.toLowerCase?.() || "",
      activeId: active?.id || "",
      activeRole: active instanceof Element ? active.getAttribute("role") || "" : "",
      activeLabel: active instanceof Element ? active.getAttribute("aria-label") || active.getAttribute("title") || active.textContent.trim().replace(/\s+/g, " ").slice(0, 80) : "",
      ariaExpanded: trigger instanceof Element ? trigger.getAttribute("aria-expanded") || "" : "",
      ariaHaspopup: trigger instanceof Element ? trigger.getAttribute("aria-haspopup") || "" : "",
      panelAriaModal: panel?.getAttribute("aria-modal") || "",
      panelFocusableCount: focusables.length,
      activeFocusableIndex: focusables.indexOf(active),
      nativePopoverTargetMatches: trigger?.popoverTargetElement === panel || Boolean(trigger?.popoverTargetElement?.contains(panel)),
      nativePopoverOpen: Boolean(trigger?.popoverTargetElement?.matches(":popover-open")),
    };
  }, { panelSelector, triggerSelector, triggerIndex });
}

async function auditTrigger(page, scenario, group, triggerIndex) {
  const trigger = page.locator(group.triggerSelector).nth(triggerIndex);
  await closeTransient(page);
  await trigger.scrollIntoViewIfNeeded().catch(() => {});
  await page.waitForTimeout(50);
  await trigger.focus();
  await page.waitForTimeout(50);

  const panelSelector = await resolvePanelSelector(trigger, group);
  const label = await trigger.evaluate((node, group) => node.getAttribute("aria-label") || node.getAttribute("title") || node.textContent.trim().replace(/\s+/g, " ").slice(0, 80) || group.key, group);
  const before = await stateFor(page, panelSelector, group.triggerSelector, triggerIndex);
  const failures = [];

  const usesNativePopover = before.nativePopoverTargetMatches;
  if (usesNativePopover ? before.nativePopoverOpen : before.ariaExpanded !== "false") failures.push("initial_expanded_state_not_false");
  if (before.ariaHaspopup !== group.expectedPopup) failures.push("trigger_missing_popup_semantics");
  if (before.panelVisible) failures.push("initial_panel_visible");

  await page.keyboard.press("Enter");
  await page.waitForTimeout(220);
  const open = await stateFor(page, panelSelector, group.triggerSelector, triggerIndex);
  if (usesNativePopover ? !open.nativePopoverOpen : open.ariaExpanded !== "true") failures.push("open_expanded_state_not_true");
  if (!open.panelVisible) failures.push("open_panel_not_visible");
  if (!open.activeInsidePanel) failures.push("open_focus_not_inside_panel");
  if (open.panelFocusableCount < 1) failures.push("open_panel_has_no_focusable_controls");

  const tabStates = [];
  const focusableCount = Math.max(1, open.panelFocusableCount);
  for (let i = 0; i < focusableCount; i += 1) {
    await page.keyboard.press("Tab");
    await page.waitForTimeout(70);
    const tabState = await stateFor(page, panelSelector, group.triggerSelector, triggerIndex);
    tabStates.push({ direction: "forward", step: i + 1, ...tabState });
    if (!tabState.panelVisible) failures.push("tab_closed_panel");
    if (i < focusableCount - 1 && !tabState.activeInsidePanel) failures.push("tab_skipped_panel_control");
    if (i === focusableCount - 1 && tabState.activeInsidePanel) failures.push("tab_did_not_exit_after_last_panel_control");
    if (!tabState.activeVisible) failures.push("tab_focus_not_visible");
  }

  for (let i = 0; i < focusableCount + 1; i += 1) {
    await page.keyboard.press("Shift+Tab");
    await page.waitForTimeout(70);
    const tabState = await stateFor(page, panelSelector, group.triggerSelector, triggerIndex);
    tabStates.push({ direction: "backward", step: i + 1, ...tabState });
    if (!tabState.panelVisible) failures.push("shift_tab_closed_panel");
    if (i < focusableCount && !tabState.activeInsidePanel) failures.push("shift_tab_skipped_panel_control");
    if (i === focusableCount && tabState.activeInsidePanel) failures.push("shift_tab_did_not_exit_before_first_panel_control");
    if (!tabState.activeVisible) failures.push("shift_tab_focus_not_visible");
  }

  const shot = await screenshot(page, `${scenario.name}-${group.key}-${triggerIndex}`);

  await page.keyboard.press("Escape");
  await page.waitForTimeout(160);
  const close = await stateFor(page, panelSelector, group.triggerSelector, triggerIndex);
  if (close.panelVisible) failures.push("escape_close_panel_still_visible");
  if (usesNativePopover ? close.nativePopoverOpen : close.ariaExpanded !== "false") failures.push("escape_close_expanded_state_not_false");
  if (!close.activeVisible) failures.push("escape_left_focus_invisible");

  return {
    scenario: scenario.name,
    group: group.key,
    trigger_index: triggerIndex,
    label,
    before,
    open,
    tab_states: tabStates,
    close,
    screenshot: shot,
    failures: [...new Set(failures)],
  };
}

(async () => {
  const failures = missingInputs();
  const inputs = Object.fromEntries(Object.entries(paths).map(([key, file]) => [key, { path: file, sha256: fs.existsSync(file) ? sha256(file) : null }]));
  const v31Gate = failures.length ? null : readJson(paths.v31Gate);
  if (v31Gate && v31Gate.status !== "ready") failures.push({ code: "v31_gate_not_ready", status: v31Gate.status });

  const browser = await chromium.launch({ executablePath: chromeBin, headless: true, args: ["--no-sandbox", "--disable-gpu"] });
  const audits = [];
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
          failures.push({ code: "missing_visible_focus_containment_trigger", scenario: scenario.name, group: group.key, selector: group.triggerSelector });
          continue;
        }
        for (const index of indexes.slice(0, group.key === "row-menu" ? 3 : 1)) {
          const audit = await auditTrigger(page, scenario, group, index);
          audits.push(audit);
          for (const failure of audit.failures) failures.push({ code: failure, audit });
        }
      }
      await context.close();
    }
  } finally {
    await browser.close();
  }

  const count = (codes) => failures.filter((failure) => codes.includes(failure.code)).length;
  const output = {
    status: failures.length ? "failed" : "ready",
    generated_at: new Date().toISOString(),
    inputs,
    summary: {
      scenario_count: scenarios.length,
      focus_containment_audit_count: audits.length,
      screenshot_count: audits.filter((audit) => audit.screenshot).length,
      tab_order_failure_count: count(["tab_skipped_panel_control", "tab_did_not_exit_after_last_panel_control", "shift_tab_skipped_panel_control", "shift_tab_did_not_exit_before_first_panel_control"]),
      tab_panel_close_failure_count: count(["tab_closed_panel", "shift_tab_closed_panel"]),
      tab_focus_visibility_failure_count: count(["tab_focus_not_visible", "shift_tab_focus_not_visible"]),
      escape_close_failure_count: count(["escape_close_panel_still_visible", "escape_close_expanded_state_not_false"]),
      escape_focus_visibility_failure_count: count(["escape_left_focus_invisible"]),
      missing_trigger_failure_count: count(["missing_visible_focus_containment_trigger"]),
      failure_count: failures.length,
      thresholds: {
        native_nonmodal_popovers_must_expose_every_panel_control_in_natural_tab_order: true,
        tab_may_leave_only_after_the_last_panel_control: true,
        shift_tab_may_leave_only_before_the_first_panel_control: true,
        tab_may_not_close_the_visible_popup: true,
        escape_must_close_panel_without_making_current_focus_invisible: true,
        browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome was used.",
      },
    },
    audits,
    failures,
  };
  console.log(JSON.stringify(output, null, 2));
  process.exitCode = failures.length ? 1 : 0;
})();
NODE
node_status=$?
set -e
cp "$V32_CENSUS_PATH" "$REPORT_PATH"
if [[ "$node_status" -ne 0 ]]; then
  echo "v32 focus containment referee failed: $REPORT_PATH" >&2
  jq '.summary, (.failures[:24])' "$REPORT_PATH" >&2 || true
  exit "$node_status"
fi

echo "v32 focus containment referee ready: $REPORT_PATH"
jq '.summary' "$REPORT_PATH"
