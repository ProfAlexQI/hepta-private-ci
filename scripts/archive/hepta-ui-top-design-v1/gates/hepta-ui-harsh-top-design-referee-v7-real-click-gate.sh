#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

source scripts/lib/hepta-ui-rust-toolchain.sh
source scripts/lib/hepta-control-ui-runtime-fixture.sh

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V7_REPORT_PATH:-}"
V6_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V6_REPORT_PATH:-}"
NATIVE_REPORT_PATH="${HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH:-}"
REAL_CLICK_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V7_REAL_CLICK_REPORT_PATH:-}"
REAL_CLICK_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V7_REAL_CLICK_DIR:-}"
V6_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V7_V6_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v7-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v7-real-click-gate.json"
fi
if [[ -z "$V6_REPORT_PATH" ]]; then
  V6_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v6-pixel-glass-census-gate.json"
fi
if [[ -z "$NATIVE_REPORT_PATH" ]]; then
  NATIVE_REPORT_PATH="$READINESS_DIR/native-fixture/native-fixture-visual-smoke.json"
fi
if [[ -z "$REAL_CLICK_REPORT_PATH" ]]; then
  REAL_CLICK_REPORT_PATH="$READINESS_DIR/control-ui-v7-real-click-activation.json"
fi
if [[ -z "$REAL_CLICK_DIR" ]]; then
  REAL_CLICK_DIR="$READINESS_DIR/control-ui-v7-real-click-activation"
fi
if [[ -z "$V6_LOG" ]]; then
  V6_LOG="$READINESS_DIR/v6-pixel-glass-census.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi
if [[ ! -s "$NATIVE_REPORT_PATH" ]]; then
  echo "missing native fixture visual smoke report: $NATIVE_REPORT_PATH" >&2
  exit 1
fi
jq empty "$NATIVE_REPORT_PATH" >/dev/null

mkdir -p "$READINESS_DIR" "$REAL_CLICK_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$REAL_CLICK_REPORT_PATH")"

HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V6_REPORT_PATH="$V6_REPORT_PATH" \
HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH="$NATIVE_REPORT_PATH" \
  bash scripts/hepta-ui-harsh-top-design-referee-v6-pixel-glass-census-gate.sh "$READINESS_DIR" >"$V6_LOG" 2>&1 || {
    echo "v6 pixel glass prerequisite failed" >&2
    tail -n 120 "$V6_LOG" >&2 || true
    exit 1
  }

if [[ "$(jq -r '.status' "$V6_REPORT_PATH")" != "ready" ]]; then
  echo "v6 pixel glass prerequisite was not ready: $V6_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7394 7395 7396 7397 7398; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v7 referee" >&2
  exit 1
fi

hepta_control_ui_runtime_fixture_init
BASE_URL="http://${BIND_ADDR}"
server_pid=""
tmp_report="$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v7-final.XXXXXX")"

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
  rm -f "$tmp_report"
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
      echo "Hepta Control UI server exited before v7 real-click audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$REAL_CLICK_DIR" >"$REAL_CLICK_REPORT_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, outputDir] = process.argv.slice(2);
const viewports = [
  { name: "desktop", width: 1365, height: 900, railVisible: true },
  { name: "narrow", width: 768, height: 900, railVisible: true },
  { name: "mobile", width: 500, height: 844, railVisible: false },
  { name: "phone320", width: 320, height: 844, railVisible: false },
];
const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase();

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
  const viewportResults = [];
  const screenshots = [];

  for (const viewport of viewports) {
    const page = await browser.newPage({ viewport: { width: viewport.width, height: viewport.height }, deviceScaleFactor: 1 });
    const targets = [];
    if (viewport.railVisible) {
      for (const key of ["ui-chat-agent", "task-queue", "operator-plane"]) {
        targets.push({
          key: `row-menu-${key}`,
          group: "row-menu",
          expectedVisibleCount: 1,
          expectedItemCount: 3,
          triggerSelector: `[data-chat-row-menu-toggle="${key}"]`,
          targetSelectors: [`[data-chat-row-menu-panel="${key}"]`],
          itemSelector: `[data-chat-row-menu-panel="${key}"] [data-chat-row-menu-item]`,
          action: { type: "row-menu", key },
        });
      }
    }
    targets.push(
      {
        key: "thread-tools",
        group: "thread-tools",
        expectedVisibleCount: 1,
        expectedItemCount: 3,
        triggerSelector: '[data-thread-command-menu="true"] [data-control-ui-thread-tools-trigger="light-glass"]',
        targetSelectors: ['[data-control-ui-thread-tools-panel="light-glass"]'],
        itemSelector: '[data-thread-command-menu="true"] [data-control-ui-menu-item]',
        action: { type: "simple-click" },
      },
      {
        key: "composer-tools",
        group: "composer-tools",
        expectedVisibleCount: 1,
        expectedItemCount: 2,
        triggerSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tools-trigger="light-glass"]',
        targetSelectors: ['[data-control-ui-composer-tools-panel="light-glass"]'],
        itemSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tool-item]',
        action: { type: "simple-click" },
      },
      {
        key: "composer-popover-artifact",
        group: "composer-popover",
        expectedVisibleCount: 1,
        expectedItemCount: 2,
        triggerSelector: '[data-chat-composer-popover-toggle="artifact"]',
        targetSelectors: ['[data-chat-composer-popover="artifact"]'],
        itemSelector: '[data-chat-composer-popover="artifact"] .tg-composer-popover__item',
        action: { type: "simple-click" },
      },
      {
        key: "composer-popover-command",
        group: "composer-popover",
        expectedVisibleCount: 1,
        expectedItemCount: 2,
        triggerSelector: '[data-chat-composer-popover-toggle="command"]',
        targetSelectors: ['[data-chat-composer-popover="command"]'],
        itemSelector: '[data-chat-composer-popover="command"] .tg-composer-popover__item',
        action: { type: "simple-click" },
      },
      {
        key: "command-palette",
        group: "command-palette",
        expectedVisibleCount: 2,
        expectedItemCount: 18,
        triggerSelector: '[data-control-ui-command-palette-trigger="light-glass"]',
        targetSelectors: ["#command-palette", ".command-palette"],
        itemSelector: "[data-control-ui-command-palette-result='light-glass']",
        action: { type: "simple-click" },
      },
    );

    const targetResults = [];
    for (const target of targets) {
      await page.goto(baseUrl, { waitUntil: "networkidle" });
      await page.waitForTimeout(140);
      const defaultClosed = await page.evaluate(visibleSubmenuSummary);
      const clickResult = await clickTarget(page, target);
      await page.waitForTimeout(160);
      const toggleCycle = await verifySecondClickToggle(page, target);
      const screenshot = await capture(page, viewport, target.key, outputDir);
      screenshots.push(screenshot);
      const audit = await page.evaluate(auditTarget, target);
      await page.mouse.click(2, 2);
      await page.waitForTimeout(100);
      const lightDismiss = await page.evaluate(visibleSubmenuSummary);
      await page.locator(target.triggerSelector).first().click({ timeout: 5000 });
      await page.waitForTimeout(100);
      await page.keyboard.press("Escape");
      await page.waitForTimeout(100);
      const escapeClose = await page.evaluate((triggerSelector) => {
        const visible = (element) => {
          if (!element) return false;
          const rect = element.getBoundingClientRect();
          const style = getComputedStyle(element);
          return style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity || 1) > 0 && rect.width > 1 && rect.height > 1;
        };
        const nodes = Array.from(document.querySelectorAll(".tg-composer-popover,.tg-row-action-popover,.tg-thread-command-menu__panel,.command-palette-backdrop,.command-palette"));
        const visibleNodes = nodes.filter(visible);
        const trigger = document.querySelector(triggerSelector);
        return {
          ready: visibleNodes.length === 0,
          visible_count: visibleNodes.length,
          total_count: nodes.length,
          focus_returned_to_trigger: document.activeElement === trigger,
        };
      }, target.triggerSelector);
      targetResults.push({
        ...target,
        default_closed: defaultClosed,
        click: clickResult,
        toggle_cycle: toggleCycle,
        audit,
        light_dismiss: lightDismiss,
        escape_close: escapeClose,
        screenshot,
        ready: defaultClosed.ready && clickResult.ready && toggleCycle.ready && audit.ready && lightDismiss.ready && escapeClose.ready && escapeClose.focus_returned_to_trigger,
      });
    }
    const mobilePaneRoutes = await auditMobilePaneRoutes(page, viewport, outputDir);
    screenshots.push(...mobilePaneRoutes.screenshots);
    const popoverSwitchSequence = await auditPopoverSwitchSequence(page, viewport);
    viewportResults.push({
      name: viewport.name,
      viewport: { width: viewport.width, height: viewport.height },
      target_count: targetResults.length,
      targets: targetResults,
      screenshot_count: targetResults.length + mobilePaneRoutes.screenshot_count,
      mobile_pane_routes: mobilePaneRoutes,
      popover_switch_sequence: popoverSwitchSequence,
      ready: targetResults.every((target) => target.ready) && mobilePaneRoutes.ready && popoverSwitchSequence.ready,
    });
    await page.close();
  }
  await browser.close();

  const failureTargets = viewportResults.flatMap((viewport) => viewport.targets.filter((target) => !target.ready).map((target) => ({ viewport: viewport.name, key: target.key, click: target.click, toggle_cycle: target.toggle_cycle, failures: target.audit.failures, default_closed: target.default_closed, light_dismiss: target.light_dismiss, escape_close: target.escape_close })));
  const failureRoutes = viewportResults.filter((viewport) => !viewport.mobile_pane_routes.ready).map((viewport) => ({ viewport: viewport.name, key: "mobile-pane-routes", details: viewport.mobile_pane_routes }));
  const failureSequences = viewportResults.filter((viewport) => !viewport.popover_switch_sequence.ready).map((viewport) => ({ viewport: viewport.name, key: "popover-switch-sequence", details: viewport.popover_switch_sequence }));
  const failures = failureTargets.concat(failureRoutes, failureSequences);
  const report = {
    schema_version: "hepta-ui-harsh-top-design-referee-v7-real-click-activation/v0",
    standards_version: "2026-06-27-real-click-submenu-activation-light-tempered-glass",
    status: failures.length === 0 ? "ready" : "failed",
    base_url: baseUrl,
    output_dir: outputDir,
    viewport_count: viewportResults.length,
    target_count: viewportResults.reduce((sum, viewport) => sum + viewport.target_count, 0),
    screenshot_count: screenshots.length,
    failure_count: failures.length,
    mobile_route_viewport_count: viewportResults.filter((viewport) => viewport.mobile_pane_routes.route_count > 0).length,
    mobile_route_count: viewportResults.reduce((sum, viewport) => sum + viewport.mobile_pane_routes.route_count, 0),
    mobile_route_screenshot_count: viewportResults.reduce((sum, viewport) => sum + viewport.mobile_pane_routes.screenshot_count, 0),
    mobile_routes_ready: viewportResults.every((viewport) => viewport.mobile_pane_routes.ready),
    popover_switch_sequence_ready: viewportResults.every((viewport) => viewport.popover_switch_sequence.ready),
    popover_switch_step_count: viewportResults.reduce((sum, viewport) => sum + viewport.popover_switch_sequence.step_count, 0),
    viewports: viewportResults,
    failures,
    screenshots,
  };
  console.log(JSON.stringify(report, null, 2));
}

async function clickTarget(page, target) {
  try {
    if (target.action.type === "row-menu") {
      const row = page.locator(`[data-chat-conversation="${target.action.key}"]`).first();
      await row.scrollIntoViewIfNeeded();
      await row.hover();
      await page.waitForTimeout(80);
    }
    const trigger = page.locator(target.triggerSelector).first();
    await trigger.scrollIntoViewIfNeeded();
    const box = await trigger.boundingBox();
    await trigger.click({ timeout: 5000 });
    const semantics = await trigger.evaluate((node) => ({
      tag_name: node.tagName.toLowerCase(),
      popover_target: node.getAttribute("popovertarget") || "",
      aria_has_popup: node.getAttribute("aria-haspopup") || "",
      aria_controls: node.getAttribute("aria-controls") || "",
    }));
    return {
      ready: semantics.tag_name === "button"
        && semantics.popover_target.length > 0
        && semantics.aria_controls === semantics.popover_target
        && (target.group !== "command-palette" || semantics.aria_has_popup === "dialog"),
      trigger_selector: target.triggerSelector,
      trigger_box: box,
      semantics,
    };
  } catch (error) {
    return { ready: false, trigger_selector: target.triggerSelector, error: String(error?.message || error) };
  }
}

async function verifySecondClickToggle(page, target) {
  try {
    const trigger = page.locator(target.triggerSelector).first();
    let closeMethod = "trigger-second-click";
    if (target.group === "command-palette") {
      const closeButton = page.locator('[data-control-ui-command-palette-close="light-glass"]').first();
      await closeButton.click({ timeout: 5000 });
      closeMethod = "explicit-close-button";
    } else {
      await trigger.click({ timeout: 5000 });
    }
    await page.waitForTimeout(100);
    const closed = await page.evaluate(visibleSubmenuSummary);
    await trigger.click({ timeout: 5000 });
    await page.waitForTimeout(100);
    const reopened = await page.evaluate((selectors) => {
      const visible = (element) => {
        if (!element) return false;
        const rect = element.getBoundingClientRect();
        const style = getComputedStyle(element);
        return style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity || 1) > 0 && rect.width > 1 && rect.height > 1;
      };
      const nodes = selectors.flatMap((selector) => Array.from(document.querySelectorAll(selector))).filter(visible);
      return {
        ready: nodes.length > 0 && nodes.every((node) => !node.hasAttribute("popover") || node.matches(":popover-open")),
        visible_target_count: nodes.length,
      };
    }, target.targetSelectors);
    return { ready: closed.ready && reopened.ready, close_method: closeMethod, closed, reopened };
  } catch (error) {
    return { ready: false, error: String(error?.message || error) };
  }
}

async function auditMobilePaneRoutes(page, viewport, outputDir) {
  await page.goto(baseUrl, { waitUntil: "networkidle" });
  await page.waitForTimeout(120);
  const tabsVisible = await page.locator("[data-chat-mobile-pane-tabs]").isVisible();
  if (viewport.width > 700) {
    return { ready: !tabsVisible, expected_visible: false, route_count: 0, screenshot_count: 0, routes: [], screenshots: [] };
  }
  const routes = [];
  const routeScreenshots = [];
  for (const pane of ["chats", "thread", "room"]) {
    const link = page.locator(`[data-chat-mobile-pane-tab="${pane}"]`).first();
    await link.click({ timeout: 5000 });
    await page.waitForTimeout(100);
    const detail = await page.evaluate((paneName) => {
      const visible = (element) => {
        if (!element) return false;
        const rect = element.getBoundingClientRect();
        const style = getComputedStyle(element);
        return style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity || 1) > 0 && rect.width > 1 && rect.height > 1;
      };
      const panes = Array.from(document.querySelectorAll("[data-chat-mobile-pane]"));
      const visiblePanes = panes.filter(visible);
      const target = document.querySelector(`[data-chat-mobile-pane="${paneName}"]`);
      const linkNode = document.querySelector(`[data-chat-mobile-pane-tab="${paneName}"]`);
      const rect = target?.getBoundingClientRect();
      const roomContent = paneName === "room" ? target?.querySelector(".hepta-right-sidebar") : null;
      const roomContentRect = roomContent?.getBoundingClientRect();
      const roomText = (roomContent?.innerText || "").replace(/\s+/g, " ").trim();
      return {
        pane: paneName,
        hash: location.hash,
        focused: document.activeElement === linkNode,
        target_focused: document.activeElement === target,
        route_focus_ready: document.activeElement === linkNode || document.activeElement === target,
        visible_panes: visiblePanes.map((node) => node.getAttribute("data-chat-mobile-pane") || ""),
        target_visible: Boolean(target && visible(target)),
        target_in_viewport: Boolean(rect && rect.left >= -1 && rect.top >= -1 && rect.right <= innerWidth + 1 && rect.bottom <= innerHeight + 1),
        pane_content_ready: Boolean(rect && rect.height >= 240) && (paneName !== "room" || Boolean(
          roomContent
          && visible(roomContent)
          && roomContentRect
          && roomContentRect.height >= 240
          && roomText.length >= 120
          && roomContent.querySelectorAll("a").length >= 3
        )),
        room_content_visible: paneName !== "room" || Boolean(roomContent && visible(roomContent)),
        room_content_height: roomContentRect ? Math.round(roomContentRect.height) : 0,
        room_content_text_length: roomText.length,
        room_content_link_count: roomContent ? roomContent.querySelectorAll("a").length : 0,
        horizontal_overflow_free: document.documentElement.scrollWidth - innerWidth <= 1 && document.body.scrollWidth - innerWidth <= 1,
      };
    }, pane);
    let rowMenu = { ready: true, tested: false };
    if (pane === "chats") {
      const trigger = page.locator('[data-chat-row-menu-toggle="ui-chat-agent"]').first();
      await trigger.click({ timeout: 5000 });
      await page.waitForTimeout(100);
      rowMenu = await page.evaluate(() => {
        const trigger = document.querySelector('[data-chat-row-menu-toggle="ui-chat-agent"]');
        const panel = document.querySelector("#row-menu-ui-chat-agent");
        const triggerRect = trigger?.getBoundingClientRect();
        const panelRect = panel?.getBoundingClientRect();
        const open = Boolean(panel?.matches(":popover-open"));
        return {
          tested: true,
          open,
          trigger_rect: triggerRect ? { left: Math.round(triggerRect.left), top: Math.round(triggerRect.top), right: Math.round(triggerRect.right), bottom: Math.round(triggerRect.bottom) } : null,
          panel_rect: panelRect ? { left: Math.round(panelRect.left), top: Math.round(panelRect.top), right: Math.round(panelRect.right), bottom: Math.round(panelRect.bottom) } : null,
          ready: Boolean(open && triggerRect && panelRect
            && panelRect.left >= 8
            && panelRect.right <= innerWidth - 8
            && panelRect.top >= triggerRect.bottom - 1
            && panelRect.top - triggerRect.bottom <= 18
            && Math.abs(panelRect.right - triggerRect.right) <= 3),
        };
      });
    }
    const screenshot = await capture(page, viewport, `mobile-pane-${pane}`, outputDir);
    routeScreenshots.push(screenshot);
    if (pane === "chats") {
      await page.keyboard.press("Escape");
      await page.waitForTimeout(80);
      rowMenu.escape_ready = await page.evaluate(() => {
        const panel = document.querySelector("#row-menu-ui-chat-agent");
        const trigger = document.querySelector('[data-chat-row-menu-toggle="ui-chat-agent"]');
        return !panel?.matches(":popover-open") && document.activeElement === trigger;
      });
      rowMenu.ready = rowMenu.ready && rowMenu.escape_ready;
    }
    detail.row_menu = rowMenu;
    detail.ready = detail.hash === `#chat-${pane === "chats" ? "list" : pane}`
      && detail.route_focus_ready
      && detail.visible_panes.length === 1
      && detail.visible_panes[0] === pane
      && detail.target_visible
      && detail.target_in_viewport
      && detail.pane_content_ready
      && detail.horizontal_overflow_free
      && rowMenu.ready;
    routes.push(detail);
  }
  return {
    ready: tabsVisible && routes.length === 3 && routes.every((route) => route.ready),
    expected_visible: true,
    route_count: routes.length,
    screenshot_count: routeScreenshots.length,
    routes,
    screenshots: routeScreenshots,
  };
}

async function auditPopoverSwitchSequence(page, viewport) {
  await page.goto(baseUrl, { waitUntil: "networkidle" });
  await page.waitForTimeout(120);
  const specs = [];
  if (viewport.railVisible) {
    for (const key of ["ui-chat-agent", "task-queue", "operator-plane"]) {
      specs.push({ key: `row-${key}`, trigger: `[data-chat-row-menu-toggle="${key}"]`, target: `row-menu-${key}` });
    }
  }
  specs.push(
    { key: "thread-tools", trigger: '[data-control-ui-thread-tools-trigger="light-glass"]', target: "thread-tools-popover" },
    { key: "composer-tools", trigger: '[data-control-ui-composer-tools-trigger="light-glass"]', target: "composer-tools-popover" },
    { key: "attachment", trigger: '[data-chat-composer-popover-toggle="artifact"]', target: "composer-popover-artifact" },
    { key: "command", trigger: '[data-chat-composer-popover-toggle="command"]', target: "composer-popover-command" },
    { key: "command-palette", trigger: '[data-control-ui-command-palette-trigger="light-glass"]', target: "command-palette" },
  );
  const steps = [];
  for (const spec of specs) {
    await page.mouse.click(2, 2);
    await page.waitForTimeout(60);
    const priorClosed = await page.evaluate(() => document.querySelectorAll("[popover]:popover-open").length === 0);
    const trigger = page.locator(spec.trigger).first();
    if (spec.key.startsWith("row-")) {
      await trigger.scrollIntoViewIfNeeded();
      await trigger.hover();
    }
    await trigger.click({ timeout: 5000 });
    await page.waitForTimeout(90);
    steps.push(await page.evaluate(({ specKey, targetId, triggerSelector, priorClosed }) => {
      const open = Array.from(document.querySelectorAll("[popover]:popover-open"));
      const target = document.getElementById(targetId);
      const triggerNode = document.querySelector(triggerSelector);
      return {
        key: specKey,
        prior_closed: priorClosed,
        open_ids: open.map((node) => node.id),
        expected_open: Boolean(target?.matches(":popover-open")),
        focus_contained: Boolean(target && (target === document.activeElement || target.contains(document.activeElement))),
        trigger_target_matches: triggerNode?.getAttribute("popovertarget") === targetId,
        ready: priorClosed
          && open.length === 1
          && open[0].id === targetId
          && Boolean(target && (target === document.activeElement || target.contains(document.activeElement)))
          && triggerNode?.getAttribute("popovertarget") === targetId,
      };
    }, { specKey: spec.key, targetId: spec.target, triggerSelector: spec.trigger, priorClosed }));
  }
  const last = specs[specs.length - 1];
  await page.keyboard.press("Escape");
  await page.waitForTimeout(90);
  const escape = await page.evaluate((triggerSelector) => ({
    open_count: document.querySelectorAll("[popover]:popover-open").length,
    focus_returned: document.activeElement === document.querySelector(triggerSelector),
  }), last.trigger);
  escape.ready = escape.open_count === 0 && escape.focus_returned;
  return {
    ready: steps.length === specs.length && steps.every((step) => step.ready) && escape.ready,
    expected_step_count: specs.length,
    step_count: steps.length,
    steps,
    escape,
  };
}

function visibleSubmenuSummary() {
  const visible = (element) => {
    if (!element) return false;
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    return style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity || 1) > 0 && rect.width > 1 && rect.height > 1;
  };
  const nodes = Array.from(document.querySelectorAll(".tg-composer-popover,.tg-row-action-popover,.tg-thread-command-menu__panel,.command-palette-backdrop,.command-palette"));
  const visibleNodes = nodes.filter(visible);
  return { ready: visibleNodes.length === 0, visible_count: visibleNodes.length, total_count: nodes.length };
}

function auditTarget(target) {
  const visible = (element) => {
    if (!element) return false;
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    return style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity || 1) > 0 && rect.width > 1 && rect.height > 1;
  };
  const rectOf = (element) => {
    const rect = element.getBoundingClientRect();
    return { left: Math.round(rect.left), top: Math.round(rect.top), right: Math.round(rect.right), bottom: Math.round(rect.bottom), width: Math.round(rect.width), height: Math.round(rect.height) };
  };
  const parseColor = (value) => {
    const match = String(value || "").match(/rgba?\(([^)]+)\)/);
    if (!match) return null;
    const parts = (match[1].match(/[0-9.]+/g) || []).map((part) => Number.parseFloat(part));
    if (parts.length < 3 || parts.slice(0, 3).some((part) => Number.isNaN(part))) return null;
    return { r: parts[0], g: parts[1], b: parts[2], a: parts.length >= 4 && !Number.isNaN(parts[3]) ? parts[3] : 1 };
  };
  const blend = (fg, bg) => {
    const alpha = Math.max(0, Math.min(1, fg?.a ?? 1));
    return { r: (fg.r * alpha) + (bg.r * (1 - alpha)), g: (fg.g * alpha) + (bg.g * (1 - alpha)), b: (fg.b * alpha) + (bg.b * (1 - alpha)), a: 1 };
  };
  const effectiveBackground = (node) => {
    let color = { r: 245, g: 248, b: 250, a: 1 };
    const stack = [];
    for (let current = node; current && current.nodeType === Node.ELEMENT_NODE; current = current.parentElement) {
      const parsed = parseColor(getComputedStyle(current).backgroundColor);
      if (parsed && parsed.a > 0) stack.push(parsed);
    }
    stack.reverse().forEach((item) => {
      color = blend(item, color);
    });
    return color;
  };
  const luminance = (color) => {
    const channel = (value) => {
      const normalized = Math.max(0, Math.min(255, value)) / 255;
      return normalized <= 0.03928 ? normalized / 12.92 : ((normalized + 0.055) / 1.055) ** 2.4;
    };
    return (0.2126 * channel(color.r)) + (0.7152 * channel(color.g)) + (0.0722 * channel(color.b));
  };
  const unobscuredAt = (element, x, y) => {
    if (x < 0 || y < 0 || x > window.innerWidth || y > window.innerHeight) return false;
    const hit = document.elementFromPoint(x, y);
    return hit === element || element.contains(hit);
  };
  const fivePointUnobscured = (element) => {
    const rect = element.getBoundingClientRect();
    const insetX = Math.max(4, Math.min(10, rect.width / 5));
    const insetY = Math.max(4, Math.min(10, rect.height / 5));
    return [
      [rect.left + rect.width / 2, rect.top + rect.height / 2],
      [rect.left + insetX, rect.top + insetY],
      [rect.right - insetX, rect.top + insetY],
      [rect.left + insetX, rect.bottom - insetY],
      [rect.right - insetX, rect.bottom - insetY],
    ].every(([x, y]) => unobscuredAt(element, x, y));
  };
  const textOf = (element) => {
    const collect = (node) => {
      if (node.nodeType === Node.TEXT_NODE) return node.textContent || "";
      if (node.nodeType !== Node.ELEMENT_NODE) return "";
      if (node.matches("svg,svg *,.sr-only")) return "";
      return Array.from(node.childNodes).map(collect).join("");
    };
    return collect(element).replace(/\s+/g, " ").trim();
  };
  const allSubmenus = Array.from(document.querySelectorAll(".tg-composer-popover,.tg-row-action-popover,.tg-thread-command-menu__panel,.command-palette-backdrop,.command-palette"));
  const visibleSubmenus = allSubmenus.filter(visible);
  const surfaces = target.targetSelectors.flatMap((selector) => Array.from(document.querySelectorAll(selector))).filter(visible);
  const trigger = document.querySelector(target.triggerSelector);
  const triggerRect = trigger && visible(trigger) ? rectOf(trigger) : null;
  const items = Array.from(document.querySelectorAll(target.itemSelector)).filter(visible);
  const surfaceDetails = surfaces.map((node) => {
    const style = getComputedStyle(node);
    const rect = rectOf(node);
    const luma = luminance(effectiveBackground(node));
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const surfaceRole = node.getAttribute("role") || "";
    const surfaceRoleReady = target.group === "command-palette" ? ["", "dialog"].includes(surfaceRole) : surfaceRole === "group";
    return {
      selector: target.targetSelectors.find((selector) => node.matches(selector)) || node.tagName.toLowerCase(),
      ...rect,
      light_glass_ready: luma >= 0.62 && luma <= 0.99,
      effective_luminance: Number(luma.toFixed(3)),
      backdrop_filter: backdrop,
      box_shadow_present: Boolean(style.boxShadow && style.boxShadow !== "none"),
      native_popover_open: !node.hasAttribute("popover") || node.matches(":popover-open"),
      role: surfaceRole,
      role_ready: surfaceRoleReady,
      in_viewport: rect.left >= -1 && rect.top >= -1 && rect.right <= window.innerWidth + 1 && rect.bottom <= window.innerHeight + 1,
      trigger_block_gap: triggerRect ? Math.round(rect.top - triggerRect.bottom) : null,
      trigger_inline_end_delta: triggerRect ? Math.round(rect.right - triggerRect.right) : null,
      trigger_geometry_ready: target.group !== "row-menu" || Boolean(triggerRect
        && rect.top >= triggerRect.bottom - 1
        && rect.top - triggerRect.bottom <= 18
        && Math.abs(rect.right - triggerRect.right) <= 3),
      unobscured: fivePointUnobscured(node),
      ready: rect.width >= 44 && rect.height >= 44 && luma >= 0.62 && luma <= 0.99 && (String(backdrop).includes("blur(") || (style.boxShadow && style.boxShadow !== "none")) && rect.left >= -1 && rect.top >= -1 && rect.right <= window.innerWidth + 1 && rect.bottom <= window.innerHeight + 1 && fivePointUnobscured(node) && (!node.hasAttribute("popover") || node.matches(":popover-open")) && surfaceRoleReady && (target.group !== "row-menu" || (triggerRect && rect.top >= triggerRect.bottom - 1 && rect.top - triggerRect.bottom <= 18 && Math.abs(rect.right - triggerRect.right) <= 3)),
    };
  });
  const itemDetails = items.map((node) => {
    const rect = rectOf(node);
    const aria = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const role = node.getAttribute("role") || "";
    const svg = Boolean(node.querySelector("svg use[href^='#hepta-icon-']"));
    const text = textOf(node);
    const nativeInteractive = node.matches("button,a[href],input,select") || Boolean(node.querySelector("input,select,button,a[href]"));
    return {
      text,
      role,
      aria_label: aria,
      title,
      svg_icon_present: svg,
      native_interactive: nativeInteractive,
      ...rect,
      ready: rect.width >= 44 && rect.height >= 44 && aria.length > 0 && title.length > 0 && aria === title && nativeInteractive && (target.group === "command-palette" || svg),
    };
  });
  const failures = [
    ...(visibleSubmenus.length === target.expectedVisibleCount ? [] : ["visible_submenu_count"]),
    ...(surfaces.length === target.expectedVisibleCount ? [] : ["target_surface_count"]),
    ...(items.length === target.expectedItemCount ? [] : ["visible_item_count"]),
    ...(surfaceDetails.every((item) => item.ready) ? [] : ["surface_geometry_or_glass"]),
    ...(itemDetails.every((item) => item.ready) ? [] : ["item_accessibility_or_icon"]),
    ...(surfaces.some((node) => node === document.activeElement || node.contains(document.activeElement)) ? [] : ["focus_not_contained_in_popover"]),
    ...(document.documentElement.scrollWidth - window.innerWidth <= 1 && document.body.scrollWidth - window.innerWidth <= 1 ? [] : ["horizontal_overflow"]),
  ];
  return {
    ready: failures.length === 0,
    failures,
    viewport: `${window.innerWidth}x${window.innerHeight}`,
    visible_submenu_count: visibleSubmenus.length,
    target_surface_count: surfaces.length,
    visible_item_count: items.length,
    focus_contained: surfaces.some((node) => node === document.activeElement || node.contains(document.activeElement)),
    surface_details: surfaceDetails,
    item_details: itemDetails,
  };
}

async function capture(page, viewport, key, outputDir) {
  const filename = `${sanitize(viewport.name)}-${sanitize(key)}.png`;
  const outputPath = path.join(outputDir, filename);
  await page.screenshot({ path: outputPath, fullPage: false });
  const bytes = fs.statSync(outputPath).size;
  const digest = crypto.createHash("sha256").update(fs.readFileSync(outputPath)).digest("hex");
  return { name: key, viewport: viewport.name, viewport_size: `${viewport.width}x${viewport.height}`, path: outputPath, bytes, sha256: digest };
}

main().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
NODE

real_click_sha="$(shasum -a 256 "$REAL_CLICK_REPORT_PATH" | awk '{print $1}')"
v6_sha="$(shasum -a 256 "$V6_REPORT_PATH" | awk '{print $1}')"
native_sha="$(shasum -a 256 "$NATIVE_REPORT_PATH" | awk '{print $1}')"

jq -n \
  --arg v6_path "$V6_REPORT_PATH" \
  --arg native_path "$NATIVE_REPORT_PATH" \
  --arg real_click_path "$REAL_CLICK_REPORT_PATH" \
  --arg v6_sha "$v6_sha" \
  --arg native_sha "$native_sha" \
  --arg real_click_sha "$real_click_sha" \
  --slurpfile v6_file "$V6_REPORT_PATH" \
  --slurpfile real_click_file "$REAL_CLICK_REPORT_PATH" '
  ($v6_file[0]) as $v6
  | ($real_click_file[0]) as $real
  | def v6_ready:
      $v6.status == "ready"
      and $v6.v5_ready == true
      and $v6.pixel_glass_ready == true
      and $v6.summary.pixel_glass_census.failure_count == 0;
    def real_click_ready:
      $real.status == "ready"
      and $real.viewport_count == 4
      and $real.target_count == 26
      and $real.screenshot_count == 32
      and $real.failure_count == 0
      and $real.mobile_route_viewport_count == 2
      and $real.mobile_route_count == 6
      and $real.mobile_route_screenshot_count == 6
      and $real.mobile_routes_ready == true
      and $real.popover_switch_sequence_ready == true
      and $real.popover_switch_step_count == 26
      and ($real.viewports | all(.ready == true
        and .mobile_pane_routes.ready == true
        and .popover_switch_sequence.ready == true
        and (.targets | all(.ready == true
          and .click.ready == true
          and .default_closed.ready == true
          and .toggle_cycle.ready == true
          and .audit.ready == true
          and .light_dismiss.ready == true
          and .escape_close.ready == true
          and .escape_close.focus_returned_to_trigger == true))));
    {
      schema_version:"hepta-ui-harsh-top-design-referee-v7-gate/v0",
      standards_version:"2026-06-27-harsh-v6-plus-real-click-submenu-activation",
      status:(if (v6_ready and real_click_ready) then "ready" else "failed" end),
      inputs:{
        v6_pixel_glass:{path:$v6_path, sha256:$v6_sha},
        native_fixture:{path:$native_path, sha256:$native_sha},
        control_real_click_activation:{path:$real_click_path, sha256:$real_click_sha}
      },
      summary:{
        control_visual_matrix:$v6.summary.control_visual_matrix,
        control_button_census:$v6.summary.control_button_census,
        native_fixture:$v6.summary.native_fixture,
        native_detail_census:$v6.summary.native_detail_census,
        pixel_glass_census:$v6.summary.pixel_glass_census,
        control_real_click_activation:{
          viewport_count:$real.viewport_count,
          target_count:$real.target_count,
          screenshot_count:$real.screenshot_count,
          failure_count:$real.failure_count,
          mobile_route_viewport_count:$real.mobile_route_viewport_count,
          mobile_route_count:$real.mobile_route_count,
          mobile_route_screenshot_count:$real.mobile_route_screenshot_count,
          mobile_routes_ready:$real.mobile_routes_ready,
          popover_switch_sequence_ready:$real.popover_switch_sequence_ready,
          popover_switch_step_count:$real.popover_switch_step_count,
          viewports:($real.viewports | map({name, target_count, screenshot_count, mobile_pane_routes, popover_switch_sequence, ready}))
        }
      },
      v6_ready:v6_ready,
      real_click_ready:real_click_ready,
      control_real_click_activation:$real
    }
  ' >"$tmp_report"

cp "$tmp_report" "$REPORT_PATH"
rm -f "$tmp_report"
cat "$REPORT_PATH"

if [[ "$(jq -r '.status' "$REPORT_PATH")" != "ready" ]]; then
  echo "Hepta UI harsh top-design referee v7 real-click gate failed" >&2
  exit 1
fi

echo "$REPORT_PATH"
