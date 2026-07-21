#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V9_REPORT_PATH:-}"
V8_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V8_REPORT_PATH:-}"
NATIVE_REPORT_PATH="${HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH:-}"
SWITCH_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V9_SWITCH_REPORT_PATH:-}"
SWITCH_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V9_SWITCH_DIR:-}"
V8_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V9_V8_LOG:-}"
SKIP_V8="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V9_SKIP_V8:-0}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v9-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v9-switching-gate.json"
fi
if [[ -z "$V8_REPORT_PATH" ]]; then
  V8_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v8-lifecycle-gate.json"
fi
if [[ -z "$NATIVE_REPORT_PATH" ]]; then
  NATIVE_REPORT_PATH="$READINESS_DIR/native-fixture/native-fixture-visual-smoke.json"
fi
if [[ -z "$SWITCH_REPORT_PATH" ]]; then
  SWITCH_REPORT_PATH="$READINESS_DIR/control-ui-v9-submenu-switching.json"
fi
if [[ -z "$SWITCH_DIR" ]]; then
  SWITCH_DIR="$READINESS_DIR/control-ui-v9-submenu-switching"
fi
if [[ -z "$V8_LOG" ]]; then
  V8_LOG="$READINESS_DIR/v8-lifecycle.log"
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

mkdir -p "$READINESS_DIR" "$SWITCH_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$SWITCH_REPORT_PATH")"

if [[ "$SKIP_V8" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V8_REPORT_PATH="$V8_REPORT_PATH" \
  HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH="$NATIVE_REPORT_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v8-lifecycle-gate.sh "$READINESS_DIR" >"$V8_LOG" 2>&1 || {
      echo "v8 lifecycle prerequisite failed" >&2
      tail -n 120 "$V8_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V8_REPORT_PATH")" != "ready" ]]; then
  echo "v8 lifecycle prerequisite was not ready: $V8_REPORT_PATH" >&2
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
  echo "no free local port found for Hepta Control UI v9 referee" >&2
  exit 1
fi

BASE_URL="http://${BIND_ADDR}"
server_pid=""
tmp_report="$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v9-final.XXXXXX")"

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
      echo "Hepta Control UI server exited before v9 switching audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$SWITCH_DIR" >"$SWITCH_REPORT_PATH" <<'NODE'
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
    const sequences = buildSequences(viewport);
    const sequenceResults = [];

    for (const sequence of sequences) {
      for (const mode of ["pointer", "keyboard"]) {
        await page.goto(baseUrl, { waitUntil: "networkidle" });
        await page.waitForTimeout(140);
        const before = await page.evaluate(stateSummary);
        const steps = [];

        for (const target of sequence.targets) {
          const open = mode === "pointer" ? await openPointer(page, target) : await openKeyboardOrFocus(page, target);
          await page.waitForTimeout(160);
          const audit = await page.evaluate(auditOpenTarget, target);
          const state = await page.evaluate(stateSummary);
          const shot = await capture(page, viewport, sequence, target, mode, outputDir);
          screenshots.push(shot);
          steps.push({ target: target.key, open, audit, state, screenshot: shot, ready: open.ready && audit.ready && state.visible_count === target.expectedVisibleCount });
        }

        const close = await closeTarget(page, sequence.targets.at(-1), mode);
        await page.waitForTimeout(160);
        const after = await page.evaluate(stateSummary);
        const failures = [
          ...(before.ready ? [] : ["sequence_start_not_clean"]),
          ...steps.flatMap((step) => step.ready ? [] : [`step_failed:${step.target}`]),
          ...(close.ready ? [] : ["sequence_close_failed"]),
          ...(after.ready ? [] : ["sequence_end_not_clean"]),
        ];
        sequenceResults.push({
          key: sequence.key,
          mode,
          target_count: sequence.targets.length,
          before,
          steps,
          close,
          after,
          failures,
          ready: failures.length === 0,
        });
      }
    }

    viewportResults.push({
      name: viewport.name,
      viewport: { width: viewport.width, height: viewport.height },
      sequence_count: sequenceResults.length,
      switch_step_count: sequenceResults.reduce((sum, sequence) => sum + sequence.steps.length, 0),
      screenshot_count: sequenceResults.reduce((sum, sequence) => sum + sequence.steps.length, 0),
      sequences: sequenceResults,
      ready: sequenceResults.every((sequence) => sequence.ready),
    });
    await page.close();
  }
  await browser.close();

  const failures = viewportResults.flatMap((viewport) => viewport.sequences
    .filter((sequence) => !sequence.ready)
    .map((sequence) => ({ viewport: viewport.name, key: sequence.key, mode: sequence.mode, failures: sequence.failures })));
  const report = {
    schema_version: "hepta-ui-harsh-top-design-referee-v9-submenu-switching/v0",
    standards_version: "2026-06-27-real-session-submenu-switching-light-tempered-glass",
    status: failures.length === 0 ? "ready" : "failed",
    base_url: baseUrl,
    output_dir: outputDir,
    viewport_count: viewportResults.length,
    sequence_count: viewportResults.reduce((sum, viewport) => sum + viewport.sequence_count, 0),
    switch_step_count: viewportResults.reduce((sum, viewport) => sum + viewport.switch_step_count, 0),
    screenshot_count: screenshots.length,
    failure_count: failures.length,
    viewports: viewportResults,
    failures,
    screenshots,
  };
  console.log(JSON.stringify(report, null, 2));
}

function buildSequences(viewport) {
  const rowTargets = ["ui-chat-agent", "task-queue", "operator-plane"].map((key) => ({
    key: `row-menu-${key}`,
    group: "row-menu",
    expectedVisibleCount: 1,
    expectedItemCount: 3,
    triggerSelector: `[data-chat-row-menu-toggle="${key}"]`,
    targetSelectors: [`[data-chat-row-menu-panel="${key}"]`],
    itemSelector: `[data-chat-row-menu-panel="${key}"] [data-chat-row-menu-item]`,
    action: { type: "row-menu", key },
  }));
  const toolTargets = [
    {
      key: "thread-tools",
      group: "thread-tools",
      expectedVisibleCount: 1,
      expectedItemCount: 3,
      triggerSelector: '[data-thread-command-menu="true"] [data-control-ui-thread-tools-trigger="light-glass"]',
      targetSelectors: ['[data-control-ui-thread-tools-panel="light-glass"]'],
      itemSelector: '[data-thread-command-menu="true"] [data-control-ui-menu-item]',
      action: { type: "popover" },
    },
    {
      key: "composer-tools",
      group: "composer-tools",
      expectedVisibleCount: 1,
      expectedItemCount: 2,
      triggerSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tools-trigger="light-glass"]',
      targetSelectors: ['[data-control-ui-composer-tools-panel="light-glass"]'],
      itemSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tool-item]',
      action: { type: "popover" },
    },
  ];
  const composerTargets = [
    {
      key: "composer-popover-artifact",
      group: "composer-popover",
      expectedVisibleCount: 1,
      expectedItemCount: 2,
      triggerSelector: '[data-chat-composer-popover-toggle="artifact"]',
      targetSelectors: ['[data-chat-composer-popover="artifact"]'],
      itemSelector: '[data-chat-composer-popover="artifact"] .tg-composer-popover__item',
      action: { type: "popover" },
    },
    {
      key: "composer-popover-command",
      group: "composer-popover",
      expectedVisibleCount: 1,
      expectedItemCount: 2,
      triggerSelector: '[data-chat-composer-popover-toggle="command"]',
      targetSelectors: ['[data-chat-composer-popover="command"]'],
      itemSelector: '[data-chat-composer-popover="command"] .tg-composer-popover__item',
      action: { type: "popover" },
    },
  ];
  const sequences = [
    { key: "tool-details-switch", targets: toolTargets },
    { key: "composer-popover-switch", targets: composerTargets },
  ];
  if (viewport.railVisible) {
    sequences.unshift({ key: "row-menu-switch", targets: rowTargets });
  }
  return sequences;
}

async function openPointer(page, target) {
  try {
    if (target.action.type === "row-menu") {
      const row = page.locator(`[data-chat-conversation="${target.action.key}"]`).first();
      await row.scrollIntoViewIfNeeded();
    }
    const trigger = page.locator(target.triggerSelector).first();
    if (target.action.type === "popover") {
      const unobscured = await trigger.evaluate((node) => {
        const rect = node.getBoundingClientRect();
        const hit = document.elementFromPoint(rect.left + rect.width / 2, rect.top + rect.height / 2);
        return Boolean(hit && (hit === node || node.contains(hit)));
      });
      if (!unobscured) {
        await page.mouse.click(8, 8);
        await page.waitForTimeout(80);
      }
    }
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
      const trigger = page.locator(target.triggerSelector).first();
      await trigger.focus();
      await page.keyboard.press("Enter");
      return { ready: true, method: "keyboard-enter-row-menu", trigger_selector: target.triggerSelector };
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
    if (target.action.type === "popover") {
      if (mode === "keyboard") {
        await page.keyboard.press("Escape");
      } else {
        await page.mouse.click(8, 8);
      }
      return { ready: true, method: mode === "keyboard" ? "keyboard-escape-popover" : "pointer-light-dismiss-popover" };
    }
    if (target.action.type === "details" || target.action.type === "row-menu") {
      const trigger = page.locator(target.triggerSelector).first();
      await trigger.scrollIntoViewIfNeeded();
      if (mode === "keyboard") {
        await trigger.focus();
        await page.keyboard.press("Enter");
      } else {
        await trigger.click({ timeout: 5000 });
      }
      return { ready: true, method: `${mode}-toggle-trigger` };
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

function stateSummary() {
  const visible = (element) => {
    if (!element) return false;
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    return style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity || 1) > 0 && rect.width > 1 && rect.height > 1;
  };
  const visibleSubmenus = Array.from(document.querySelectorAll(".tg-composer-popover,.tg-row-action-popover,.tg-thread-command-menu__panel,.command-palette-backdrop,.command-palette")).filter(visible);
  const openDetails = Array.from(document.querySelectorAll("details[open]")).map((node) => node.getAttribute("data-thread-command-menu") === "true" ? "thread-tools" : node.getAttribute("data-control-ui-composer-more") ? "composer-tools" : node.className || node.tagName.toLowerCase());
  return {
    ready: visibleSubmenus.length === 0 && openDetails.length === 0 && location.hash !== "#command-palette",
    visible_count: visibleSubmenus.length,
    open_details: openDetails,
    hash: location.hash,
    active_element: document.activeElement?.getAttribute("data-chat-composer-popover-toggle") || document.activeElement?.getAttribute("data-control-ui-thread-tools-trigger") || document.activeElement?.getAttribute("data-control-ui-composer-tools-trigger") || document.activeElement?.tagName?.toLowerCase() || "",
  };
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

async function capture(page, viewport, sequence, target, mode, outputDir) {
  const filename = `${sanitize(viewport.name)}-${sanitize(sequence.key)}-${sanitize(mode)}-${sanitize(target.key)}.png`;
  const outputPath = path.join(outputDir, filename);
  await page.screenshot({ path: outputPath, fullPage: false });
  const bytes = fs.statSync(outputPath).size;
  const digest = crypto.createHash("sha256").update(fs.readFileSync(outputPath)).digest("hex");
  return { name: target.key, sequence: sequence.key, mode, viewport: viewport.name, viewport_size: `${viewport.width}x${viewport.height}`, path: outputPath, bytes, sha256: digest };
}

main().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
NODE

switch_sha="$(shasum -a 256 "$SWITCH_REPORT_PATH" | awk '{print $1}')"
v8_sha="$(shasum -a 256 "$V8_REPORT_PATH" | awk '{print $1}')"
native_sha="$(shasum -a 256 "$NATIVE_REPORT_PATH" | awk '{print $1}')"

jq -n \
  --arg v8_path "$V8_REPORT_PATH" \
  --arg native_path "$NATIVE_REPORT_PATH" \
  --arg switch_path "$SWITCH_REPORT_PATH" \
  --arg v8_sha "$v8_sha" \
  --arg native_sha "$native_sha" \
  --arg switch_sha "$switch_sha" \
  --slurpfile v8_file "$V8_REPORT_PATH" \
  --slurpfile switch_file "$SWITCH_REPORT_PATH" '
  ($v8_file[0]) as $v8
  | ($switch_file[0]) as $switch
  | def v8_ready:
      $v8.status == "ready"
      and $v8.v7_ready == true
      and $v8.lifecycle_ready == true
      and $v8.summary.control_submenu_lifecycle.failure_count == 0;
    def switching_ready:
      $switch.status == "ready"
      and $switch.viewport_count == 4
      and $switch.sequence_count == 20
      and $switch.switch_step_count == 44
      and $switch.screenshot_count == 44
      and $switch.failure_count == 0
      and ($switch.viewports | all(.ready == true and (.sequences | all(.ready == true))));
    {
      schema_version:"hepta-ui-harsh-top-design-referee-v9-gate/v0",
      standards_version:"2026-06-27-harsh-v8-plus-real-session-submenu-switching",
      status:(if (v8_ready and switching_ready) then "ready" else "failed" end),
      inputs:{
        v8_lifecycle:{path:$v8_path, sha256:$v8_sha},
        native_fixture:{path:$native_path, sha256:$native_sha},
        control_submenu_switching:{path:$switch_path, sha256:$switch_sha}
      },
      summary:{
        control_visual_matrix:$v8.summary.control_visual_matrix,
        control_button_census:$v8.summary.control_button_census,
        native_fixture:$v8.summary.native_fixture,
        native_detail_census:$v8.summary.native_detail_census,
        pixel_glass_census:$v8.summary.pixel_glass_census,
        control_real_click_activation:$v8.summary.control_real_click_activation,
        control_submenu_lifecycle:$v8.summary.control_submenu_lifecycle,
        control_submenu_switching:{
          viewport_count:$switch.viewport_count,
          sequence_count:$switch.sequence_count,
          switch_step_count:$switch.switch_step_count,
          screenshot_count:$switch.screenshot_count,
          failure_count:$switch.failure_count,
          viewports:($switch.viewports | map({name, sequence_count, switch_step_count, screenshot_count, ready}))
        }
      },
      v8_ready:v8_ready,
      switching_ready:switching_ready,
      control_submenu_switching:$switch
    }
  ' >"$tmp_report"

cp "$tmp_report" "$REPORT_PATH"
rm -f "$tmp_report"
cat "$REPORT_PATH"

if [[ "$(jq -r '.status' "$REPORT_PATH")" != "ready" ]]; then
  echo "Hepta UI harsh top-design referee v9 switching gate failed" >&2
  exit 1
fi

echo "$REPORT_PATH"
