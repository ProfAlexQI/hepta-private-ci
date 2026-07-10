#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V8_REPORT_PATH:-}"
V7_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V7_REPORT_PATH:-}"
NATIVE_REPORT_PATH="${HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH:-}"
LIFECYCLE_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V8_LIFECYCLE_REPORT_PATH:-}"
LIFECYCLE_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V8_LIFECYCLE_DIR:-}"
V7_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V8_V7_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v8-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v8-lifecycle-gate.json"
fi
if [[ -z "$V7_REPORT_PATH" ]]; then
  V7_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v7-real-click-gate.json"
fi
if [[ -z "$NATIVE_REPORT_PATH" ]]; then
  NATIVE_REPORT_PATH="$READINESS_DIR/native-fixture/native-fixture-visual-smoke.json"
fi
if [[ -z "$LIFECYCLE_REPORT_PATH" ]]; then
  LIFECYCLE_REPORT_PATH="$READINESS_DIR/control-ui-v8-submenu-lifecycle.json"
fi
if [[ -z "$LIFECYCLE_DIR" ]]; then
  LIFECYCLE_DIR="$READINESS_DIR/control-ui-v8-submenu-lifecycle"
fi
if [[ -z "$V7_LOG" ]]; then
  V7_LOG="$READINESS_DIR/v7-real-click.log"
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

mkdir -p "$READINESS_DIR" "$LIFECYCLE_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$LIFECYCLE_REPORT_PATH")"

HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V7_REPORT_PATH="$V7_REPORT_PATH" \
HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH="$NATIVE_REPORT_PATH" \
  bash scripts/hepta-ui-harsh-top-design-referee-v7-real-click-gate.sh "$READINESS_DIR" >"$V7_LOG" 2>&1 || {
    echo "v7 real-click prerequisite failed" >&2
    tail -n 120 "$V7_LOG" >&2 || true
    exit 1
  }

if [[ "$(jq -r '.status' "$V7_REPORT_PATH")" != "ready" ]]; then
  echo "v7 real-click prerequisite was not ready: $V7_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7404 7405 7406 7407 7408; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v8 referee" >&2
  exit 1
fi

BASE_URL="http://${BIND_ADDR}"
server_pid=""
tmp_report="$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v8-final.XXXXXX")"

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
      echo "Hepta Control UI server exited before v8 lifecycle audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$LIFECYCLE_DIR" >"$LIFECYCLE_REPORT_PATH" <<'NODE'
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
const submenuSelector = ".tg-composer-popover,.tg-row-action-popover,.tg-thread-command-menu__panel,.command-palette-backdrop,.command-palette";
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
    const targetResults = [];
    for (const target of buildTargets(viewport)) {
      await page.goto(baseUrl, { waitUntil: "networkidle" });
      await page.waitForTimeout(140);

      const defaultClosed = await page.evaluate(visibleSubmenuSummary, submenuSelector);
      const pointerOpen = await openPointer(page, target);
      await page.waitForTimeout(160);
      const pointerAudit = await page.evaluate(auditOpenTarget, target);
      const pointerOpenShot = await capture(page, viewport, target, "pointer-open", outputDir);
      screenshots.push(pointerOpenShot);

      const pointerClose = await closeTarget(page, target, "pointer");
      await page.waitForTimeout(160);
      const pointerClosed = await page.evaluate(visibleSubmenuSummary, submenuSelector);
      const pointerCloseShot = await capture(page, viewport, target, "pointer-closed", outputDir);
      screenshots.push(pointerCloseShot);

      const keyboardOpen = await openKeyboardOrFocus(page, target);
      await page.waitForTimeout(160);
      const keyboardAudit = await page.evaluate(auditOpenTarget, target);
      const keyboardOpenShot = await capture(page, viewport, target, "keyboard-open", outputDir);
      screenshots.push(keyboardOpenShot);

      const keyboardClose = await closeTarget(page, target, "keyboard");
      await page.waitForTimeout(160);
      const keyboardClosed = await page.evaluate(visibleSubmenuSummary, submenuSelector);
      const keyboardCloseShot = await capture(page, viewport, target, "keyboard-closed", outputDir);
      screenshots.push(keyboardCloseShot);

      const failures = [
        ...(defaultClosed.ready ? [] : ["default_not_closed"]),
        ...(pointerOpen.ready ? [] : ["pointer_open_failed"]),
        ...(pointerAudit.ready ? [] : pointerAudit.failures.map((failure) => `pointer_${failure}`)),
        ...(pointerClose.ready ? [] : ["pointer_close_action_failed"]),
        ...(pointerClosed.ready ? [] : ["pointer_close_left_submenu_visible"]),
        ...(keyboardOpen.ready ? [] : ["keyboard_or_focus_open_failed"]),
        ...(keyboardAudit.ready ? [] : keyboardAudit.failures.map((failure) => `keyboard_${failure}`)),
        ...(keyboardClose.ready ? [] : ["keyboard_close_action_failed"]),
        ...(keyboardClosed.ready ? [] : ["keyboard_close_left_submenu_visible"]),
      ];
      targetResults.push({
        key: target.key,
        group: target.group,
        default_closed: defaultClosed,
        pointer_cycle: { open: pointerOpen, audit: pointerAudit, close: pointerClose, closed: pointerClosed },
        keyboard_focus_cycle: { open: keyboardOpen, audit: keyboardAudit, close: keyboardClose, closed: keyboardClosed },
        screenshots: [pointerOpenShot, pointerCloseShot, keyboardOpenShot, keyboardCloseShot],
        failures,
        ready: failures.length === 0,
      });
    }
    viewportResults.push({
      name: viewport.name,
      viewport: { width: viewport.width, height: viewport.height },
      target_count: targetResults.length,
      screenshot_count: targetResults.reduce((sum, target) => sum + target.screenshots.length, 0),
      targets: targetResults,
      ready: targetResults.every((target) => target.ready),
    });
    await page.close();
  }

  await browser.close();

  const failures = viewportResults.flatMap((viewport) => viewport.targets
    .filter((target) => !target.ready)
    .map((target) => ({ viewport: viewport.name, key: target.key, group: target.group, failures: target.failures })));
  const report = {
    schema_version: "hepta-ui-harsh-top-design-referee-v8-submenu-lifecycle/v0",
    standards_version: "2026-06-27-real-click-keyboard-focus-close-lifecycle-light-tempered-glass",
    status: failures.length === 0 ? "ready" : "failed",
    base_url: baseUrl,
    output_dir: outputDir,
    viewport_count: viewportResults.length,
    target_count: viewportResults.reduce((sum, viewport) => sum + viewport.target_count, 0),
    lifecycle_step_count: viewportResults.reduce((sum, viewport) => sum + viewport.target_count * 5, 0),
    screenshot_count: screenshots.length,
    failure_count: failures.length,
    viewports: viewportResults,
    failures,
    screenshots,
  };
  console.log(JSON.stringify(report, null, 2));
}

function buildTargets(viewport) {
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
      action: { type: "details" },
    },
    {
      key: "composer-tools",
      group: "composer-tools",
      expectedVisibleCount: 1,
      expectedItemCount: 2,
      triggerSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tools-trigger="light-glass"]',
      targetSelectors: ['[data-control-ui-composer-tools-panel="light-glass"]'],
      itemSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tool-item]',
      action: { type: "details" },
    },
    {
      key: "composer-popover-artifact",
      group: "composer-popover",
      expectedVisibleCount: 1,
      expectedItemCount: 2,
      triggerSelector: '[data-chat-composer-popover-toggle="artifact"]',
      targetSelectors: ['[data-chat-composer-popover="artifact"]'],
      itemSelector: '[data-chat-composer-popover="artifact"] .tg-composer-popover__item',
      action: { type: "details" },
    },
    {
      key: "composer-popover-command",
      group: "composer-popover",
      expectedVisibleCount: 1,
      expectedItemCount: 2,
      triggerSelector: '[data-chat-composer-popover-toggle="command"]',
      targetSelectors: ['[data-chat-composer-popover="command"]'],
      itemSelector: '[data-chat-composer-popover="command"] .tg-composer-popover__item',
      action: { type: "details" },
    },
    {
      key: "command-palette",
      group: "command-palette",
      expectedVisibleCount: 2,
      expectedItemCount: 18,
      triggerSelector: '[data-control-ui-command-palette-trigger="light-glass"]',
      targetSelectors: ["#command-palette", ".command-palette"],
      itemSelector: "[data-control-ui-command-palette-result='light-glass']",
      action: { type: "hash-dialog" },
    },
  );
  return targets;
}

async function openPointer(page, target) {
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
    return { ready: true, method: "pointer", trigger_selector: target.triggerSelector, trigger_box: box };
  } catch (error) {
    return { ready: false, method: "pointer", trigger_selector: target.triggerSelector, error: String(error?.message || error) };
  }
}

async function openKeyboardOrFocus(page, target) {
  try {
    if (target.action.type === "row-menu") {
      const row = page.locator(`[data-chat-conversation="${target.action.key}"]`).first();
      await row.scrollIntoViewIfNeeded();
      await row.focus();
      await page.keyboard.press("Enter");
      return { ready: true, method: "focus-visible-row", trigger_selector: `[data-chat-conversation="${target.action.key}"]` };
    }
    const trigger = page.locator(target.triggerSelector).first();
    await trigger.scrollIntoViewIfNeeded();
    await trigger.focus();
    if (target.action.type === "focus-popover") {
      return { ready: true, method: "focus-visible-button", trigger_selector: target.triggerSelector };
    }
    await page.keyboard.press("Enter");
    return { ready: true, method: "keyboard-enter", trigger_selector: target.triggerSelector };
  } catch (error) {
    return { ready: false, method: "keyboard-or-focus", trigger_selector: target.triggerSelector, error: String(error?.message || error) };
  }
}

async function closeTarget(page, target, mode) {
  try {
    if (target.action.type === "details") {
      const trigger = page.locator(target.triggerSelector).first();
      await trigger.scrollIntoViewIfNeeded();
      if (mode === "keyboard") {
        await trigger.focus();
        await page.keyboard.press("Enter");
      } else {
        await trigger.click({ timeout: 5000 });
      }
      return { ready: true, method: `${mode}-toggle-summary` };
    }
    if (target.action.type === "hash-dialog") {
      const close = page.locator('[data-control-ui-command-palette-close="light-glass"]').first();
      if (mode === "keyboard") {
        await close.focus();
        await page.keyboard.press("Enter");
      } else {
        await close.click({ timeout: 5000 });
      }
      return { ready: true, method: `${mode}-close-link` };
    }
    const textarea = page.locator("[data-chat-composer-input]").first();
    if (await textarea.count()) {
      await textarea.focus();
    } else {
      await page.mouse.move(12, 12);
      await page.locator("body").click({ position: { x: 12, y: 12 }, timeout: 5000 });
    }
    await page.mouse.move(12, Math.max(12, Math.floor((page.viewportSize()?.height || 844) / 2)));
    return { ready: true, method: `${mode}-focus-outside` };
  } catch (error) {
    return { ready: false, method: `${mode}-close`, error: String(error?.message || error) };
  }
}

function visibleSubmenuSummary(selector) {
  const visible = (element) => {
    if (!element) return false;
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    return style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity || 1) > 0 && rect.width > 1 && rect.height > 1;
  };
  const visibleNodes = Array.from(document.querySelectorAll(selector)).filter(visible);
  return { ready: visibleNodes.length === 0, visible_count: visibleNodes.length, total_count: document.querySelectorAll(selector).length };
}

function auditOpenTarget(target) {
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
  const items = Array.from(document.querySelectorAll(target.itemSelector)).filter(visible);
  const surfaceDetails = surfaces.map((node) => {
    const style = getComputedStyle(node);
    const rect = rectOf(node);
    const luma = luminance(effectiveBackground(node));
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    return {
      ...rect,
      effective_luminance: Number(luma.toFixed(3)),
      backdrop_filter: backdrop,
      box_shadow_present: Boolean(style.boxShadow && style.boxShadow !== "none"),
      in_viewport: rect.left >= -1 && rect.top >= -1 && rect.right <= window.innerWidth + 1 && rect.bottom <= window.innerHeight + 1,
      unobscured: fivePointUnobscured(node),
      ready: rect.width >= 44 && rect.height >= 44 && luma >= 0.62 && luma <= 0.99 && (String(backdrop).includes("blur(") || (style.boxShadow && style.boxShadow !== "none")) && rect.left >= -1 && rect.top >= -1 && rect.right <= window.innerWidth + 1 && rect.bottom <= window.innerHeight + 1 && fivePointUnobscured(node),
    };
  });
  const itemDetails = items.map((node) => {
    const rect = rectOf(node);
    const aria = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const role = node.getAttribute("role") || "";
    const svg = Boolean(node.querySelector("svg use[href^='#hepta-icon-']"));
    const text = textOf(node);
    return {
      text,
      role,
      aria_label: aria,
      title,
      svg_icon_present: svg,
      ...rect,
      ready: rect.width >= 44 && rect.height >= 44 && aria.length > 0 && title.length > 0 && aria === title && (target.group === "command-palette" || role === "menuitem") && (target.group === "command-palette" || svg),
    };
  });
  const failures = [
    ...(visibleSubmenus.length === target.expectedVisibleCount ? [] : ["visible_submenu_count"]),
    ...(surfaces.length === target.expectedVisibleCount ? [] : ["target_surface_count"]),
    ...(items.length === target.expectedItemCount ? [] : ["visible_item_count"]),
    ...(surfaceDetails.every((item) => item.ready) ? [] : ["surface_geometry_or_glass"]),
    ...(itemDetails.every((item) => item.ready) ? [] : ["item_accessibility_or_icon"]),
    ...(document.documentElement.scrollWidth - window.innerWidth <= 1 && document.body.scrollWidth - window.innerWidth <= 1 ? [] : ["horizontal_overflow"]),
  ];
  return {
    ready: failures.length === 0,
    failures,
    viewport: `${window.innerWidth}x${window.innerHeight}`,
    visible_submenu_count: visibleSubmenus.length,
    target_surface_count: surfaces.length,
    visible_item_count: items.length,
    surface_details: surfaceDetails,
    item_details: itemDetails,
  };
}

async function capture(page, viewport, target, phase, outputDir) {
  const filename = `${sanitize(viewport.name)}-${sanitize(target.key)}-${sanitize(phase)}.png`;
  const outputPath = path.join(outputDir, filename);
  await page.screenshot({ path: outputPath, fullPage: false });
  const bytes = fs.statSync(outputPath).size;
  const digest = crypto.createHash("sha256").update(fs.readFileSync(outputPath)).digest("hex");
  return { name: target.key, phase, viewport: viewport.name, viewport_size: `${viewport.width}x${viewport.height}`, path: outputPath, bytes, sha256: digest };
}

main().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
NODE

lifecycle_sha="$(shasum -a 256 "$LIFECYCLE_REPORT_PATH" | awk '{print $1}')"
v7_sha="$(shasum -a 256 "$V7_REPORT_PATH" | awk '{print $1}')"
native_sha="$(shasum -a 256 "$NATIVE_REPORT_PATH" | awk '{print $1}')"

jq -n \
  --arg v7_path "$V7_REPORT_PATH" \
  --arg native_path "$NATIVE_REPORT_PATH" \
  --arg lifecycle_path "$LIFECYCLE_REPORT_PATH" \
  --arg v7_sha "$v7_sha" \
  --arg native_sha "$native_sha" \
  --arg lifecycle_sha "$lifecycle_sha" \
  --slurpfile v7_file "$V7_REPORT_PATH" \
  --slurpfile lifecycle_file "$LIFECYCLE_REPORT_PATH" '
  ($v7_file[0]) as $v7
  | ($lifecycle_file[0]) as $life
  | def v7_ready:
      $v7.status == "ready"
      and $v7.v6_ready == true
      and $v7.real_click_ready == true
      and $v7.summary.control_real_click_activation.failure_count == 0;
    def lifecycle_ready:
      $life.status == "ready"
      and $life.viewport_count == 4
      and $life.target_count == 26
      and $life.lifecycle_step_count == 130
      and $life.screenshot_count == 104
      and $life.failure_count == 0
      and ($life.viewports | all(.ready == true and (.targets | all(.ready == true))));
    {
      schema_version:"hepta-ui-harsh-top-design-referee-v8-gate/v0",
      standards_version:"2026-06-27-harsh-v7-plus-submenu-lifecycle-keyboard-focus-close",
      status:(if (v7_ready and lifecycle_ready) then "ready" else "failed" end),
      inputs:{
        v7_real_click:{path:$v7_path, sha256:$v7_sha},
        native_fixture:{path:$native_path, sha256:$native_sha},
        control_submenu_lifecycle:{path:$lifecycle_path, sha256:$lifecycle_sha}
      },
      summary:{
        control_visual_matrix:$v7.summary.control_visual_matrix,
        control_button_census:$v7.summary.control_button_census,
        native_fixture:$v7.summary.native_fixture,
        native_detail_census:$v7.summary.native_detail_census,
        pixel_glass_census:$v7.summary.pixel_glass_census,
        control_real_click_activation:$v7.summary.control_real_click_activation,
        control_submenu_lifecycle:{
          viewport_count:$life.viewport_count,
          target_count:$life.target_count,
          lifecycle_step_count:$life.lifecycle_step_count,
          screenshot_count:$life.screenshot_count,
          failure_count:$life.failure_count,
          viewports:($life.viewports | map({name, target_count, screenshot_count, ready}))
        }
      },
      v7_ready:v7_ready,
      lifecycle_ready:lifecycle_ready,
      control_submenu_lifecycle:$life
    }
  ' >"$tmp_report"

cp "$tmp_report" "$REPORT_PATH"
rm -f "$tmp_report"
cat "$REPORT_PATH"

if [[ "$(jq -r '.status' "$REPORT_PATH")" != "ready" ]]; then
  echo "Hepta UI harsh top-design referee v8 lifecycle gate failed" >&2
  exit 1
fi

echo "$REPORT_PATH"
