#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

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
      const screenshot = await capture(page, viewport, target.key, outputDir);
      screenshots.push(screenshot);
      const audit = await page.evaluate(auditTarget, target);
      targetResults.push({ ...target, default_closed: defaultClosed, click: clickResult, audit, screenshot, ready: defaultClosed.ready && clickResult.ready && audit.ready });
    }
    viewportResults.push({
      name: viewport.name,
      viewport: { width: viewport.width, height: viewport.height },
      target_count: targetResults.length,
      targets: targetResults,
      screenshot_count: targetResults.length,
      ready: targetResults.every((target) => target.ready),
    });
    await page.close();
  }
  await browser.close();

  const failureTargets = viewportResults.flatMap((viewport) => viewport.targets.filter((target) => !target.ready).map((target) => ({ viewport: viewport.name, key: target.key, click: target.click, failures: target.audit.failures, default_closed: target.default_closed })));
  const report = {
    schema_version: "hepta-ui-harsh-top-design-referee-v7-real-click-activation/v0",
    standards_version: "2026-06-27-real-click-submenu-activation-light-tempered-glass",
    status: failureTargets.length === 0 ? "ready" : "failed",
    base_url: baseUrl,
    output_dir: outputDir,
    viewport_count: viewportResults.length,
    target_count: viewportResults.reduce((sum, viewport) => sum + viewport.target_count, 0),
    screenshot_count: screenshots.length,
    failure_count: failureTargets.length,
    viewports: viewportResults,
    failures: failureTargets,
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
    return { ready: true, trigger_selector: target.triggerSelector, trigger_box: box };
  } catch (error) {
    return { ready: false, trigger_selector: target.triggerSelector, error: String(error?.message || error) };
  }
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
  const items = Array.from(document.querySelectorAll(target.itemSelector)).filter(visible);
  const surfaceDetails = surfaces.map((node) => {
    const style = getComputedStyle(node);
    const rect = rectOf(node);
    const luma = luminance(effectiveBackground(node));
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    return {
      selector: target.targetSelectors.find((selector) => node.matches(selector)) || node.tagName.toLowerCase(),
      ...rect,
      light_glass_ready: luma >= 0.62 && luma <= 0.99,
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
      and $real.screenshot_count == 26
      and $real.failure_count == 0
      and ($real.viewports | all(.ready == true and (.targets | all(.ready == true and .click.ready == true and .default_closed.ready == true and .audit.ready == true))));
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
          viewports:($real.viewports | map({name, target_count, screenshot_count, ready}))
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
