#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

source scripts/lib/hepta-ui-rust-toolchain.sh
source scripts/lib/hepta-control-ui-runtime-fixture.sh

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V30_REPORT_PATH:-}"
V30_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V30_CENSUS_PATH:-}"
V30_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V30_SCREENSHOT_DIR:-}"
V29_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V29_REPORT_PATH:-}"
V29_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V30_V29_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
SKIP_V29="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V30_SKIP_V29:-0}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v30-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v30-state-semantics-gate.json"
fi
if [[ -z "$V30_CENSUS_PATH" ]]; then
  V30_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v30-state-semantics-census.json"
fi
if [[ -z "$V30_SCREENSHOT_DIR" ]]; then
  V30_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v30-state-semantics-screenshots"
fi
if [[ -z "$V29_REPORT_PATH" ]]; then
  V29_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v29-reduced-motion-gate.json"
fi
if [[ -z "$V29_LOG" ]]; then
  V29_LOG="$READINESS_DIR/v29-reduced-motion-prerequisite.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$V30_SCREENSHOT_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V30_CENSUS_PATH")"

if [[ "$SKIP_V29" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V29_REPORT_PATH="$V29_REPORT_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v29-reduced-motion-gate.sh "$READINESS_DIR" >"$V29_LOG" 2>&1 || {
      echo "v29 reduced-motion prerequisite failed" >&2
      tail -n 180 "$V29_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V29_REPORT_PATH")" != "ready" ]]; then
  echo "v29 reduced-motion prerequisite was not ready: $V29_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7660 7661 7662 7663 7664; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v30 referee" >&2
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
      echo "Hepta Control UI server exited before v30 state semantics audit was ready" >&2
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
node - "$CHROME_BIN" "$BASE_URL/" "$V30_SCREENSHOT_DIR" "$READINESS_DIR" >"$V30_CENSUS_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir, readinessDir] = process.argv.slice(2);
fs.mkdirSync(screenshotDir, { recursive: true });

const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase() || "item";
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const paths = { v29Gate: path.join(readinessDir, "ui-harsh-top-design-referee-v29-reduced-motion-gate.json") };

const scenarios = [
  { name: "desktop-state-semantics", viewport: { width: 1365, height: 900, dpr: 2, railVisible: true, isMobile: false, hasTouch: false } },
  { name: "narrow-touch-state-semantics", viewport: { width: 768, height: 900, dpr: 2, railVisible: true, isMobile: true, hasTouch: true } },
  { name: "phone320-state-semantics", viewport: { width: 320, height: 700, dpr: 3, railVisible: false, isMobile: true, hasTouch: true } },
];

const targetGroups = [
  {
    key: "row-menu",
    railOnly: true,
    triggerSelector: "[data-chat-row-menu-toggle]",
    panelForTrigger: (trigger) => `[data-chat-row-menu-panel="${trigger.getAttribute("data-chat-row-menu-toggle")}"]`,
    expectedPopup: "",
    expectedPanelRole: "group",
  },
  {
    key: "thread-tools",
    triggerSelector: "[data-control-ui-thread-tools-trigger='light-glass']",
    panelSelector: "[data-control-ui-thread-tools-panel='light-glass']",
    expectedPopup: "",
    expectedPanelRole: "group",
  },
  {
    key: "composer-tools",
    triggerSelector: "[data-control-ui-composer-tools-trigger='light-glass']",
    panelSelector: "[data-control-ui-composer-tools-panel='light-glass']",
    expectedPopup: "",
    expectedPanelRole: "group",
  },
  {
    key: "artifact-popover",
    triggerSelector: "[data-chat-composer-popover-toggle='artifact']",
    panelSelector: "[data-chat-composer-popover='artifact']",
    expectedPopup: "",
    expectedPanelRole: "group",
  },
  {
    key: "command-popover",
    triggerSelector: "[data-chat-composer-popover-toggle='command']",
    panelSelector: "[data-chat-composer-popover='command']",
    expectedPopup: "",
    expectedPanelRole: "group",
  },
  {
    key: "command-palette",
    triggerSelector: "[data-control-ui-command-palette-trigger='light-glass']",
    panelSelector: "[data-control-ui-command-palette-surface='light-glass']",
    expectedPopup: "dialog",
    expectedPanelRole: "dialog",
    expectedAriaModal: "false",
  },
];

function missingInputs() {
  return Object.entries(paths)
    .filter(([, file]) => !fs.existsSync(file))
    .map(([key, file]) => ({ code: "missing_input", key, file }));
}

async function setProfile(page, profile) {
  await page.setViewportSize({ width: profile.width, height: profile.height });
  await page.emulateMedia({ reducedMotion: "reduce" });
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
  await page.waitForTimeout(100);
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
  if (group.key === "row-menu") {
    await trigger.evaluate((node) => {
      const row = node.closest(".tg-chat-item");
      const scroller = row?.closest(".tg-room-rail, .tg-conversation-list, .tg-sidebar, .tg-room-list");
      if (row && scroller && scroller.scrollHeight > scroller.clientHeight) {
        scroller.scrollTop = Math.max(0, row.offsetTop - ((scroller.clientHeight - row.getBoundingClientRect().height) / 2));
      }
      row?.scrollIntoView({ block: "center", inline: "nearest" });
    }).catch(() => {});
  }
  await trigger.scrollIntoViewIfNeeded().catch(() => {});
  await page.waitForTimeout(50);
  const panelSelectorForTrigger = group.panelSelector || await trigger.evaluate((node) => {
    const key = node.getAttribute("data-chat-row-menu-toggle") || "";
    return key ? `[data-chat-row-menu-panel="${key}"]` : "";
  });
  const wireGroup = { ...group, panelSelector: panelSelectorForTrigger };
  delete wireGroup.panelForTrigger;
  const initial = await trigger.evaluate((node, group) => {
    const panelSelector = group.panelSelector;
    const panel = panelSelector ? document.querySelector(panelSelector) : null;
    const rect = node.getBoundingClientRect();
    const style = window.getComputedStyle(node);
    const panelVisible = panel ? (() => {
      const panelRect = panel.getBoundingClientRect();
      const panelStyle = window.getComputedStyle(panel);
      return panelRect.width > 1 && panelRect.height > 1 && panelStyle.display !== "none" && panelStyle.visibility !== "hidden" && Number(panelStyle.opacity) > 0.01;
    })() : false;
    return {
      label: node.getAttribute("aria-label") || node.getAttribute("title") || node.textContent.trim().replace(/\s+/g, " ").slice(0, 80) || group.key,
      tag: node.tagName.toLowerCase(),
      role: node.getAttribute("role") || "",
      width: rect.width,
      height: rect.height,
      display: style.display,
      ariaHaspopup: node.getAttribute("aria-haspopup") || "",
      ariaControls: node.getAttribute("aria-controls") || "",
      ariaExpanded: node.getAttribute("aria-expanded") || "",
      nativePopoverTargetMatches: node.popoverTargetElement === panel || Boolean(node.popoverTargetElement?.contains(panel)),
      nativePopoverOpen: Boolean(node.popoverTargetElement?.matches(":popover-open")),
      panelSelector,
      panelFound: Boolean(panel),
      panelId: panel?.id || "",
      panelRole: panel?.getAttribute("role") || "",
      panelAriaLabel: panel?.getAttribute("aria-label") || "",
      panelAriaModal: panel?.getAttribute("aria-modal") || "",
      panelVisible,
    };
  }, wireGroup);

  const failures = [];
  if (initial.ariaHaspopup !== group.expectedPopup) failures.push("trigger_missing_or_wrong_aria_haspopup");
  if (!initial.ariaControls) failures.push("trigger_missing_aria_controls");
  if (initial.ariaControls && initial.panelId && initial.ariaControls !== initial.panelId) failures.push("aria_controls_does_not_reference_panel_id");
  if (!initial.panelFound) failures.push("aria_controlled_panel_missing");
  if (initial.panelFound && initial.panelRole !== group.expectedPanelRole) failures.push("controlled_panel_wrong_role");
  if (initial.panelFound && String(initial.panelAriaLabel || "").trim().length < 2) failures.push("controlled_panel_missing_accessible_name");
  if (group.expectedPanelRole === "dialog" && initial.panelAriaModal !== group.expectedAriaModal) failures.push("dialog_wrong_aria_modal_state");
  const usesNativePopover = initial.nativePopoverTargetMatches;
  if (usesNativePopover ? initial.nativePopoverOpen : initial.ariaExpanded !== "false") failures.push("initial_expanded_state_not_false");
  if (initial.panelVisible) failures.push("initial_controlled_panel_visible");

  await trigger.click({ force: true });
  await page.waitForTimeout(180);
  const openState = await trigger.evaluate((node, group) => {
    const panelSelector = group.panelSelector;
    const panel = panelSelector ? document.querySelector(panelSelector) : null;
    const panelRect = panel?.getBoundingClientRect();
    const panelStyle = panel ? window.getComputedStyle(panel) : null;
    const panelVisible = panel && panelRect.width > 1 && panelRect.height > 1 && panelStyle.display !== "none" && panelStyle.visibility !== "hidden" && Number(panelStyle.opacity) > 0.01;
    return {
      ariaExpanded: node.getAttribute("aria-expanded") || "",
      nativePopoverTargetMatches: node.popoverTargetElement === panel || Boolean(node.popoverTargetElement?.contains(panel)),
      nativePopoverOpen: Boolean(node.popoverTargetElement?.matches(":popover-open")),
      panelVisible: Boolean(panelVisible),
      panelRole: panel?.getAttribute("role") || "",
      panelAriaLabel: panel?.getAttribute("aria-label") || "",
    };
  }, wireGroup);
  if (usesNativePopover ? !openState.nativePopoverOpen : openState.ariaExpanded !== "true") failures.push("open_expanded_state_not_true");
  if (!openState.panelVisible) failures.push("open_controlled_panel_not_visible");
  if (openState.panelRole !== group.expectedPanelRole) failures.push("open_controlled_panel_wrong_role");
  if (String(openState.panelAriaLabel || "").trim().length < 2) failures.push("open_controlled_panel_missing_accessible_name");

  const shot = await screenshot(page, `${scenario.name}-${group.key}-${triggerIndex}`);
  await closeTransient(page);
  const closeState = await trigger.evaluate((node, group) => {
    const panelSelector = group.panelSelector;
    const panel = panelSelector ? document.querySelector(panelSelector) : null;
    const panelRect = panel?.getBoundingClientRect();
    const panelStyle = panel ? window.getComputedStyle(panel) : null;
    const panelVisible = panel && panelRect.width > 1 && panelRect.height > 1 && panelStyle.display !== "none" && panelStyle.visibility !== "hidden" && Number(panelStyle.opacity) > 0.01;
    return {
      ariaExpanded: node.getAttribute("aria-expanded") || "",
      nativePopoverTargetMatches: node.popoverTargetElement === panel || Boolean(node.popoverTargetElement?.contains(panel)),
      nativePopoverOpen: Boolean(node.popoverTargetElement?.matches(":popover-open")),
      panelVisible: Boolean(panelVisible),
    };
  }, wireGroup);
  if (usesNativePopover ? closeState.nativePopoverOpen : closeState.ariaExpanded !== "false") failures.push("closed_expanded_state_not_false");
  if (closeState.panelVisible) failures.push("closed_controlled_panel_still_visible");

  return {
    scenario: scenario.name,
    group: group.key,
    trigger_index: triggerIndex,
    label: initial.label,
    initial,
    open_state: openState,
    close_state: closeState,
    screenshot: shot,
    failures: [...new Set(failures)],
  };
}

(async () => {
  const failures = missingInputs();
  const inputs = Object.fromEntries(Object.entries(paths).map(([key, file]) => [key, { path: file, sha256: fs.existsSync(file) ? sha256(file) : null }]));
  const v29Gate = failures.length ? null : readJson(paths.v29Gate);
  if (v29Gate && v29Gate.status !== "ready") failures.push({ code: "v29_gate_not_ready", status: v29Gate.status });

  const browser = await chromium.launch({ executablePath: chromeBin, headless: true, args: ["--no-sandbox", "--disable-gpu"] });
  const triggerAudits = [];
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
          failures.push({ code: "missing_visible_trigger", scenario: scenario.name, group: group.key, selector: group.triggerSelector });
          continue;
        }
        for (const index of indexes.slice(0, group.key === "row-menu" ? 3 : 1)) {
          const audit = await auditTrigger(page, scenario, group, index);
          triggerAudits.push(audit);
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
      trigger_audit_count: triggerAudits.length,
      screenshot_count: triggerAudits.filter((audit) => audit.screenshot).length,
      aria_haspopup_failure_count: countFailures(["aria_haspopup"]),
      aria_controls_failure_count: countFailures(["aria_controls", "aria_controlled"]),
      expanded_state_failure_count: countFailures(["expanded_state"]),
      controlled_panel_semantics_failure_count: countFailures(["controlled_panel_wrong_role", "controlled_panel_missing_accessible_name", "dialog_wrong_aria_modal_state"]),
      controlled_panel_visibility_failure_count: countFailures(["controlled_panel_not_visible", "controlled_panel_still_visible", "controlled_panel_visible"]),
      failure_count: failures.length,
      thresholds: {
        generic_action_popovers_do_not_overclaim_aria_menu: true,
        command_palette_trigger_requires_aria_haspopup_dialog: true,
        popup_triggers_require_aria_controls_to_existing_panel_id: true,
      popup_triggers_require_expanded_state_sync: "explicit aria-expanded or native popover implicit state",
        controlled_action_popovers_require_role_group_and_name: true,
        controlled_dialogs_require_role_name_and_truthful_aria_modal_state: true,
        browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome was used.",
      },
    },
    audits: triggerAudits,
    failures,
  };
  console.log(JSON.stringify(output, null, 2));
  process.exitCode = failures.length ? 1 : 0;
})();
NODE
node_status=$?
set -e
cp "$V30_CENSUS_PATH" "$REPORT_PATH"
if [[ "$node_status" -ne 0 ]]; then
  echo "v30 state semantics referee failed: $REPORT_PATH" >&2
  jq '.summary, (.failures[:24])' "$REPORT_PATH" >&2 || true
  exit "$node_status"
fi

echo "v30 state semantics referee ready: $REPORT_PATH"
jq '.summary' "$REPORT_PATH"
