#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V14_REPORT_PATH:-}"
V13_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V13_REPORT_PATH:-}"
SCROLL_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V14_SCROLL_REPORT_PATH:-}"
SCROLL_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V14_SCREENSHOT_DIR:-}"
SCROLL_CROP_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V14_CROP_DIR:-}"
NATIVE_DIR="${HEPTA_NATIVE_FIXTURE_VISUAL_DIR:-}"
V13_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V14_V13_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v14-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v14-scroll-edge-crop-gate.json"
fi
if [[ -z "$V13_REPORT_PATH" ]]; then
  V13_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v13-geometry-occlusion-gate.json"
fi
if [[ -z "$SCROLL_REPORT_PATH" ]]; then
  SCROLL_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v14-scroll-edge-crop-census.json"
fi
if [[ -z "$SCROLL_SCREENSHOT_DIR" ]]; then
  SCROLL_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v14-scroll-edge-screenshots"
fi
if [[ -z "$SCROLL_CROP_DIR" ]]; then
  SCROLL_CROP_DIR="$READINESS_DIR/ui-harsh-v14-scroll-edge-crops"
fi
if [[ -z "$NATIVE_DIR" ]]; then
  NATIVE_DIR="$READINESS_DIR/native-fixture"
fi
if [[ -z "$V13_LOG" ]]; then
  V13_LOG="$READINESS_DIR/v13-geometry-occlusion.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$NATIVE_DIR" "$SCROLL_SCREENSHOT_DIR" "$SCROLL_CROP_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$SCROLL_REPORT_PATH")"

HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V13_REPORT_PATH="$V13_REPORT_PATH" \
HEPTA_NATIVE_FIXTURE_VISUAL_DIR="$NATIVE_DIR" \
  bash scripts/hepta-ui-harsh-top-design-referee-v13-geometry-occlusion-gate.sh "$READINESS_DIR" >"$V13_LOG" 2>&1 || {
    echo "v13 geometry/occlusion prerequisite failed" >&2
    tail -n 180 "$V13_LOG" >&2 || true
    exit 1
  }

if [[ "$(jq -r '.status' "$V13_REPORT_PATH")" != "ready" ]]; then
  echo "v13 geometry/occlusion prerequisite was not ready: $V13_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7419 7420 7421 7422 7423; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v14 referee" >&2
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
      echo "Hepta Control UI server exited before v14 scroll-edge audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$SCROLL_SCREENSHOT_DIR" "$SCROLL_CROP_DIR" >"$SCROLL_REPORT_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir, cropDir] = process.argv.slice(2);
const viewports = [
  { name: "desktop", width: 1365, height: 900, railVisible: true },
  { name: "narrow", width: 768, height: 900, railVisible: true },
  { name: "mobile", width: 500, height: 844, railVisible: false },
  { name: "phone320", width: 320, height: 844, railVisible: false },
];
const scrollStages = [
  { key: "scroll-top", ratio: 0 },
  { key: "scroll-mid", ratio: 0.5 },
  { key: "scroll-bottom", ratio: 1 },
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
fs.mkdirSync(screenshotDir, { recursive: true });
fs.mkdirSync(cropDir, { recursive: true });

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
      closeMode: "summary",
    },
    {
      key: "composer-tools",
      group: "composer-tools",
      triggerSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tools-trigger="light-glass"]',
      panelSelector: '[data-control-ui-composer-tools-panel="light-glass"]',
      itemSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tool-item]',
      expectedItemCount: 2,
      closeMode: "summary",
    },
    {
      key: "composer-popover-artifact",
      group: "composer-popover",
      triggerSelector: '[data-chat-composer-popover-toggle="artifact"]',
      panelSelector: '[data-chat-composer-popover="artifact"]',
      itemSelector: '[data-chat-composer-popover="artifact"] .tg-composer-popover__item',
      expectedItemCount: 2,
      closeMode: "summary",
    },
    {
      key: "composer-popover-command",
      group: "composer-popover",
      triggerSelector: '[data-chat-composer-popover-toggle="command"]',
      panelSelector: '[data-chat-composer-popover="command"]',
      itemSelector: '[data-chat-composer-popover="command"] .tg-composer-popover__item',
      expectedItemCount: 2,
      closeMode: "summary",
    },
    {
      key: "command-palette",
      group: "command-palette",
      triggerSelector: '[data-control-ui-command-palette-trigger="light-glass"]',
      panelSelector: '#command-palette .command-palette',
      itemSelector: '[data-control-ui-command-palette-result="light-glass"]',
      expectedItemCount: 18,
      closeMode: "palette-close",
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

function paddedRect(box, viewport, padding = 4) {
  const left = Math.max(0, Math.floor(box.left - padding));
  const top = Math.max(0, Math.floor(box.top - padding));
  const right = Math.min(viewport.width, Math.ceil(box.right + padding));
  const bottom = Math.min(viewport.height, Math.ceil(box.bottom + padding));
  return { left, top, width: Math.max(1, right - left), height: Math.max(1, bottom - top), right, bottom };
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

async function geometryFor(locator, viewport, kind) {
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
  const topmost = await topmostFor(locator);
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

async function scrollState(page) {
  return page.evaluate(() => {
    const describe = (selector) => {
      const element = document.querySelector(selector);
      if (!element) return null;
      return {
        selector,
        scroll_top: Math.round(element.scrollTop),
        scroll_height: Math.round(element.scrollHeight),
        client_height: Math.round(element.clientHeight),
        can_scroll: element.scrollHeight > element.clientHeight + 1,
      };
    };
    return {
      window: {
        scroll_top: Math.round(window.scrollY),
        scroll_height: Math.round(document.documentElement.scrollHeight),
        client_height: Math.round(window.innerHeight),
        can_scroll: document.documentElement.scrollHeight > window.innerHeight + 1,
      },
      rail: describe(".tg-conversation-list"),
      thread: describe(".tg-thread"),
      command_results: describe("#command-palette-results"),
    };
  });
}

async function applyScrollStage(page, stage) {
  await page.evaluate((ratio) => {
    const scrollElement = (element) => {
      if (!element) return;
      element.scrollTop = Math.max(0, (element.scrollHeight - element.clientHeight) * ratio);
    };
    scrollElement(document.querySelector(".tg-conversation-list"));
    scrollElement(document.querySelector(".tg-thread"));
    scrollElement(document.querySelector("#command-palette-results"));
    window.scrollTo(0, Math.max(0, (document.documentElement.scrollHeight - window.innerHeight) * ratio));
  }, stage.ratio);
  await page.waitForTimeout(120);
  return scrollState(page);
}

async function metricsForPng(page, file) {
  const bytes = fs.statSync(file).size;
  const data = fs.readFileSync(file).toString("base64");
  const metrics = await page.evaluate(async ({ data }) => {
    const image = new Image();
    image.src = `data:image/png;base64,${data}`;
    await image.decode();
    const canvas = document.createElement("canvas");
    canvas.width = image.naturalWidth;
    canvas.height = image.naturalHeight;
    const context = canvas.getContext("2d", { willReadFrequently: true });
    context.drawImage(image, 0, 0);
    const pixels = context.getImageData(0, 0, canvas.width, canvas.height).data;
    const step = Math.max(1, Math.ceil(Math.sqrt((canvas.width * canvas.height) / 5000)));
    const lumas = [];
    let highlightCount = 0;
    let darkCount = 0;
    let chromaticCount = 0;
    let glassWhiteCount = 0;
    let saturationSum = 0;
    let textureSum = 0;
    let textureCount = 0;
    const lumaAt = (x, y) => {
      const index = (y * canvas.width + x) * 4;
      return (0.2126 * pixels[index]) + (0.7152 * pixels[index + 1]) + (0.0722 * pixels[index + 2]);
    };
    for (let y = 0; y < canvas.height; y += step) {
      for (let x = 0; x < canvas.width; x += step) {
        const index = (y * canvas.width + x) * 4;
        const r = pixels[index];
        const g = pixels[index + 1];
        const b = pixels[index + 2];
        const luma = (0.2126 * r) + (0.7152 * g) + (0.0722 * b);
        const max = Math.max(r, g, b);
        const min = Math.min(r, g, b);
        const saturation = max > 0 ? (max - min) / max : 0;
        lumas.push(luma);
        saturationSum += saturation;
        if (luma >= 238) highlightCount += 1;
        if (luma <= 95) darkCount += 1;
        if (max - min >= 8 && luma >= 140) chromaticCount += 1;
        if (luma >= 176 && saturation <= 0.32) glassWhiteCount += 1;
        if (x + step < canvas.width) {
          textureSum += Math.abs(lumaAt(x + step, y) - luma);
          textureCount += 1;
        }
        if (y + step < canvas.height) {
          textureSum += Math.abs(lumaAt(x, y + step) - luma);
          textureCount += 1;
        }
      }
    }
    lumas.sort((a, b) => a - b);
    const sampleCount = lumas.length;
    const mean = lumas.reduce((sum, value) => sum + value, 0) / sampleCount;
    const variance = lumas.reduce((sum, value) => sum + ((value - mean) ** 2), 0) / sampleCount;
    const percentile = (ratio) => lumas[Math.min(sampleCount - 1, Math.max(0, Math.floor(sampleCount * ratio)))];
    return {
      width: canvas.width,
      height: canvas.height,
      sample_count: sampleCount,
      mean_luma: mean,
      luma_stddev: Math.sqrt(variance),
      luma_p05: percentile(0.05),
      luma_p50: percentile(0.5),
      luma_p95: percentile(0.95),
      highlight_ratio: highlightCount / sampleCount,
      dark_ratio: darkCount / sampleCount,
      chromatic_ratio: chromaticCount / sampleCount,
      glass_white_ratio: glassWhiteCount / sampleCount,
      mean_saturation: saturationSum / sampleCount,
      texture_delta: textureSum / Math.max(1, textureCount),
    };
  }, { data });
  return { ...metrics, bytes, sha256: sha256(file) };
}

function normalizeMetrics(rawMetrics) {
  return Object.fromEntries(Object.entries(rawMetrics).map(([key, value]) => [
    key,
    typeof value === "number" && !Number.isInteger(value) ? round(value) : value,
  ]));
}

function pixelFailures(kind, metrics) {
  const minHeight = kind === "menu-item" ? 32 : 44;
  return [
    ...(metrics.bytes >= 900 ? [] : ["too_few_bytes"]),
    ...(metrics.width >= 44 && metrics.height >= minHeight ? [] : ["crop_too_small"]),
    ...(metrics.mean_luma >= 198 && metrics.mean_luma <= 253 ? [] : ["mean_luma_out_of_range"]),
    ...(metrics.luma_p95 >= 232 ? [] : ["weak_local_highlights"]),
    ...(metrics.dark_ratio <= 0.16 ? [] : ["too_much_local_dark_area"]),
    ...(metrics.glass_white_ratio >= 0.76 ? [] : ["insufficient_local_light_glass_area"]),
    ...(metrics.mean_saturation <= 0.20 ? [] : ["oversaturated_local_palette"]),
    ...(metrics.luma_stddev >= 1.5 ? [] : ["locally_flat_luma"]),
    ...(metrics.texture_delta >= 0.08 ? [] : ["insufficient_local_texture_signal"]),
  ];
}

async function cropElement(page, metricPage, viewport, locator, cropKind, prefix) {
  const box = await boxFor(locator);
  if (!box) {
    return { crop_kind: cropKind, label: prefix, ready: false, failures: ["missing_box"] };
  }
  const rect = paddedRect(box, viewport, 4);
  const cropPath = path.join(cropDir, `${prefix}-${cropKind}.png`);
  await page.screenshot({
    path: cropPath,
    fullPage: false,
    clip: {
      x: rect.left,
      y: rect.top,
      width: rect.width,
      height: rect.height,
    },
  });
  const rawMetrics = await metricsForPng(metricPage, cropPath);
  const failures = pixelFailures(cropKind, rawMetrics);
  return {
    crop_kind: cropKind,
    label: prefix,
    crop_path: cropPath,
    metrics: normalizeMetrics(rawMetrics),
    failures,
    ready: failures.length === 0,
  };
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
  await trigger.scrollIntoViewIfNeeded().catch(() => {});
  await trigger.click({ timeout: 5000 });
  await page.waitForTimeout(180);
  const panel = page.locator(target.panelSelector).first();
  await panel.waitFor({ state: "visible", timeout: 5000 });
  return trigger;
}

async function closeTarget(page, viewport, target, trigger) {
  if (target.closeMode === "palette-close") {
    const close = page.locator("[data-control-ui-command-palette-close='light-glass']").first();
    await close.click({ timeout: 5000 });
  } else if (target.closeMode === "summary") {
    await trigger.click({ timeout: 5000 });
  } else {
    await page.mouse.move(viewport.width - 8, viewport.height - 8);
    await page.mouse.click(viewport.width - 8, viewport.height - 8);
  }
  await page.keyboard.press("Escape").catch(() => {});
  await page.waitForTimeout(160);
  return visibleTransientPanels(page);
}

async function auditTarget(page, metricPage, viewport, target, stage) {
  await page.goto(baseUrl, { waitUntil: "networkidle" });
  await page.waitForTimeout(150);
  const preScroll = await applyScrollStage(page, stage);
  await revealTarget(page, target);
  const trigger = page.locator(target.triggerSelector).first();
  await trigger.waitFor({ state: "visible", timeout: 5000 });
  const triggerGeometry = await geometryFor(trigger, viewport, "trigger");

  const openedTrigger = await openTarget(page, target);
  const postOpenPanels = await visibleTransientPanels(page);
  const panel = page.locator(target.panelSelector).first();
  const items = page.locator(target.itemSelector);
  const itemCount = await items.count();
  const overflow = await pageOverflow(page);
  const panelGeometry = await geometryFor(panel, viewport, "panel");
  const prefix = `${sanitize(viewport.name)}-${sanitize(stage.key)}-${sanitize(target.key)}`;
  const screenshotPath = path.join(screenshotDir, `${prefix}.png`);
  await page.screenshot({ path: screenshotPath, fullPage: false });
  const panelCrop = await cropElement(page, metricPage, viewport, panel, "panel", prefix);
  const itemGeometries = [];
  const itemCrops = [];

  for (let index = 0; index < itemCount; index += 1) {
    const item = items.nth(index);
    await item.evaluate((element) => element.scrollIntoView({ block: "center", inline: "nearest" })).catch(() => {});
    await page.waitForTimeout(35);
    const label = await item.evaluate((element, fallback) => (
      element.getAttribute("data-chat-row-menu-item")
      || element.getAttribute("data-control-ui-menu-item")
      || element.getAttribute("data-control-ui-command-palette-item")
      || element.getAttribute("data-chat-composer-picker-item")
      || element.getAttribute("aria-label")
      || element.textContent
      || `item-${fallback}`
    ), index);
    const itemPrefix = `${prefix}-item-${index}-${sanitize(String(label).trim().slice(0, 72))}`;
    itemGeometries.push({
      index,
      label: String(label).trim().slice(0, 96),
      ...(await geometryFor(item, viewport, "menu-item")),
    });
    itemCrops.push({
      index,
      label: String(label).trim().slice(0, 96),
      ...(await cropElement(page, metricPage, viewport, item, "menu-item", itemPrefix)),
    });
  }

  const postClosePanels = await closeTarget(page, viewport, target, openedTrigger);
  const failures = [
    ...(postOpenPanels.length === 1 ? [] : [`expected_one_visible_transient_panel_got_${postOpenPanels.length}`]),
    ...(itemCount === target.expectedItemCount ? [] : [`expected_${target.expectedItemCount}_items_got_${itemCount}`]),
    ...(overflow.horizontal_overflow_px <= 1 ? [] : [`horizontal_overflow_${overflow.horizontal_overflow_px}px`]),
    ...(postClosePanels.length === 0 ? [] : [`post_close_residual_panels_${postClosePanels.length}`]),
    ...triggerGeometry.failures.map((failure) => `trigger:${failure}`),
    ...panelGeometry.failures.map((failure) => `panel:${failure}`),
    ...panelCrop.failures.map((failure) => `panel_crop:${failure}`),
    ...itemGeometries.flatMap((item) => item.failures.map((failure) => `item:${item.label}:${failure}`)),
    ...itemCrops.flatMap((item) => item.failures.map((failure) => `item_crop:${item.label}:${failure}`)),
  ];

  return {
    viewport: viewport.name,
    scroll_stage: stage.key,
    target: target.key,
    group: target.group,
    pre_scroll_state: preScroll,
    expected_item_count: target.expectedItemCount,
    item_count: itemCount,
    post_open_visible_panels: postOpenPanels,
    post_close_visible_panels: postClosePanels,
    overflow,
    trigger: triggerGeometry,
    panel: panelGeometry,
    items: itemGeometries,
    panel_crop: panelCrop,
    item_crops: itemCrops,
    screenshot: { path: screenshotPath, sha256: sha256(screenshotPath) },
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
  const metricPage = await browser.newPage({ viewport: { width: 900, height: 700 }, deviceScaleFactor: 1 });
  const audits = [];

  for (const viewport of viewports) {
    const page = await browser.newPage({ viewport: { width: viewport.width, height: viewport.height }, deviceScaleFactor: 1 });
    for (const target of targetDefinitions(viewport)) {
      for (const stage of scrollStages) {
        audits.push(await auditTarget(page, metricPage, viewport, target, stage));
      }
    }
    await page.close();
  }

  await metricPage.close();
  await browser.close();

  const crops = audits.flatMap((audit) => [
    { viewport: audit.viewport, scroll_stage: audit.scroll_stage, group: audit.group, target: audit.target, ...audit.panel_crop },
    ...audit.item_crops.map((crop) => ({ viewport: audit.viewport, scroll_stage: audit.scroll_stage, group: audit.group, target: audit.target, ...crop })),
  ]);
  const failures = audits.filter((audit) => !audit.ready);
  const cropFailures = crops.filter((crop) => !crop.ready);
  const targetCount = viewports.reduce((sum, viewport) => sum + targetDefinitions(viewport).length, 0);
  const expectedAuditCount = targetCount * scrollStages.length;
  const report = {
    schema_version: "hepta-ui-harsh-top-design-referee-v14-scroll-edge-crop-census/v0",
    standards_version: "2026-06-28-control-scroll-edge-submenu-geometry-residual-local-crop-census",
    status: failures.length === 0 && cropFailures.length === 0 && audits.length === expectedAuditCount ? "ready" : "failed",
    base_url: baseUrl,
    browser_path: "Browser plugin not available; regular Playwright with local Chrome was used",
    screenshot_dir: screenshotDir,
    crop_dir: cropDir,
    viewport_count: viewports.length,
    scroll_stage_count: scrollStages.length,
    target_count: targetCount,
    audit_count: audits.length,
    expected_audit_count: expectedAuditCount,
    screenshot_count: audits.length,
    crop_count: crops.length,
    panel_crop_count: audits.length,
    menu_item_crop_count: crops.length - audits.length,
    failure_count: failures.length,
    crop_failure_count: cropFailures.length,
    thresholds: {
      trigger_min_size: "44x44",
      panel_min_size: "120x44",
      menu_item_min_size: "44x32",
      panel_clipped_ratio_min: 0.995,
      trigger_or_item_clipped_ratio_min: 0.985,
      horizontal_overflow_px_max: 1,
      visible_transient_panel_count_after_open: 1,
      visible_transient_panel_count_after_close: 0,
      local_crop_mean_luma: "198..253",
      local_crop_luma_p95_min: 232,
      local_crop_dark_ratio_max: 0.16,
      local_crop_glass_white_ratio_min: 0.76,
      local_crop_mean_saturation_max: 0.20,
      local_crop_luma_stddev_min: 1.5,
      local_crop_texture_delta_min: 0.08,
      scroll_stages: scrollStages.map((stage) => stage.key),
    },
    by_viewport: summarizeBy(audits, "viewport", "audit_count"),
    by_group: summarizeBy(audits, "group", "audit_count"),
    by_scroll_stage: summarizeBy(audits, "scroll_stage", "audit_count"),
    crop_by_viewport: summarizeBy(crops, "viewport", "crop_count"),
    crop_by_group: summarizeBy(crops, "group", "crop_count"),
    crop_by_scroll_stage: summarizeBy(crops, "scroll_stage", "crop_count"),
    failures,
    crop_failures: cropFailures,
    audits,
  };
  console.log(JSON.stringify(report, null, 2));
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
NODE

if [[ "$(jq -r '.status' "$SCROLL_REPORT_PATH")" != "ready" ]]; then
  echo "v14 scroll-edge crop census failed" >&2
  jq '.failures[:20] | map({viewport, scroll_stage, target, group, failures})' "$SCROLL_REPORT_PATH" >&2 || true
  jq '.crop_failures[:20] | map({viewport, scroll_stage, target, group, crop_kind, label, failures, metrics})' "$SCROLL_REPORT_PATH" >&2 || true
  exit 1
fi

node - "$V13_REPORT_PATH" "$SCROLL_REPORT_PATH" "$REPORT_PATH" <<'NODE'
const crypto = require("node:crypto");
const fs = require("node:fs");

const [v13Path, scrollPath, outputPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const v13 = readJson(v13Path);
const scroll = readJson(scrollPath);
const ready = v13.status === "ready"
  && scroll.status === "ready"
  && scroll.failure_count === 0
  && scroll.crop_failure_count === 0
  && scroll.audit_count === scroll.expected_audit_count
  && scroll.crop_count >= 400;
const report = {
  schema_version: "hepta-ui-harsh-top-design-referee-v14-gate/v0",
  standards_version: "2026-06-28-harsh-v13-plus-scroll-edge-residual-local-crop-census",
  status: ready ? "ready" : "failed",
  browser_path: "Browser plugin not available; regular Playwright with local Chrome was used",
  inputs: {
    v13_geometry_occlusion: { path: v13Path, sha256: sha256(v13Path) },
    scroll_edge_crop_census: { path: scrollPath, sha256: sha256(scrollPath) },
  },
  summary: {
    v13_geometry_occlusion: v13.summary?.v13_geometry_occlusion,
    v12_interaction_crop: v13.summary?.v12_interaction_crop,
    v14_scroll_edge_crop: {
      viewport_count: scroll.viewport_count,
      scroll_stage_count: scroll.scroll_stage_count,
      target_count: scroll.target_count,
      audit_count: scroll.audit_count,
      screenshot_count: scroll.screenshot_count,
      crop_count: scroll.crop_count,
      panel_crop_count: scroll.panel_crop_count,
      menu_item_crop_count: scroll.menu_item_crop_count,
      failure_count: scroll.failure_count,
      crop_failure_count: scroll.crop_failure_count,
      by_viewport: scroll.by_viewport,
      by_group: scroll.by_group,
      by_scroll_stage: scroll.by_scroll_stage,
      crop_by_viewport: scroll.crop_by_viewport,
      crop_by_group: scroll.crop_by_group,
      crop_by_scroll_stage: scroll.crop_by_scroll_stage,
      thresholds: scroll.thresholds,
    },
  },
  v13_ready: v13.status === "ready",
  scroll_edge_crop_ready: scroll.status === "ready",
  scroll_edge_crop_census: scroll,
};
fs.writeFileSync(outputPath, JSON.stringify(report, null, 2) + "\n");
console.log(JSON.stringify(report, null, 2));
NODE

cat "$REPORT_PATH"
