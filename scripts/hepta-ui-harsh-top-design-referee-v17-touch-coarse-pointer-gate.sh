#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V17_REPORT_PATH:-}"
V16_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V16_REPORT_PATH:-}"
TOUCH_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V17_TOUCH_REPORT_PATH:-}"
TOUCH_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V17_SCREENSHOT_DIR:-}"
TOUCH_CROP_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V17_CROP_DIR:-}"
NATIVE_DIR="${HEPTA_NATIVE_FIXTURE_VISUAL_DIR:-}"
V16_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V17_V16_LOG:-}"
SKIP_V16="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V17_SKIP_V16:-0}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v17-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v17-touch-coarse-pointer-gate.json"
fi
if [[ -z "$V16_REPORT_PATH" ]]; then
  V16_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v16-keyboard-focus-gate.json"
fi
if [[ -z "$TOUCH_REPORT_PATH" ]]; then
  TOUCH_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v17-touch-coarse-pointer-census.json"
fi
if [[ -z "$TOUCH_SCREENSHOT_DIR" ]]; then
  TOUCH_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v17-touch-coarse-pointer-screenshots"
fi
if [[ -z "$TOUCH_CROP_DIR" ]]; then
  TOUCH_CROP_DIR="$READINESS_DIR/ui-harsh-v17-touch-coarse-pointer-crops"
fi
if [[ -z "$NATIVE_DIR" ]]; then
  NATIVE_DIR="$READINESS_DIR/native-fixture"
fi
if [[ -z "$V16_LOG" ]]; then
  V16_LOG="$READINESS_DIR/v16-keyboard-focus.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$NATIVE_DIR" "$TOUCH_SCREENSHOT_DIR" "$TOUCH_CROP_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$TOUCH_REPORT_PATH")"

if [[ "$SKIP_V16" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V16_REPORT_PATH="$V16_REPORT_PATH" \
  HEPTA_NATIVE_FIXTURE_VISUAL_DIR="$NATIVE_DIR" \
    bash scripts/hepta-ui-harsh-top-design-referee-v16-keyboard-focus-gate.sh "$READINESS_DIR" >"$V16_LOG" 2>&1 || {
      echo "v16 keyboard/focus prerequisite failed" >&2
      tail -n 180 "$V16_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V16_REPORT_PATH")" != "ready" ]]; then
  echo "v16 keyboard/focus prerequisite was not ready: $V16_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7439 7440 7441 7442 7443; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v17 referee" >&2
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
      echo "Hepta Control UI server exited before v17 touch/coarse-pointer audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$TOUCH_SCREENSHOT_DIR" "$TOUCH_CROP_DIR" >"$TOUCH_REPORT_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir, cropDir] = process.argv.slice(2);
const viewports = [
  { name: "desktop-touch-dpr2", width: 1365, height: 900, dpr: 2, railVisible: true },
  { name: "narrow-touch-dpr2", width: 768, height: 900, dpr: 2, railVisible: true },
  { name: "mobile-touch-dpr2", width: 500, height: 844, dpr: 2, railVisible: false },
  { name: "phone320-touch-dpr3", width: 320, height: 700, dpr: 3, railVisible: false },
];
const tapSampleNames = ["center", "inset-start", "inset-end"];
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
    },
    {
      key: "composer-tools",
      group: "composer-tools",
      triggerSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tools-trigger="light-glass"]',
      panelSelector: '[data-control-ui-composer-tools-panel="light-glass"]',
      itemSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tool-item]',
      expectedItemCount: 2,
    },
    {
      key: "composer-popover-artifact",
      group: "composer-popover",
      triggerSelector: '[data-chat-composer-popover-toggle="artifact"]',
      panelSelector: '[data-chat-composer-popover="artifact"]',
      itemSelector: '[data-chat-composer-popover="artifact"] .tg-composer-popover__item',
      expectedItemCount: 2,
    },
    {
      key: "composer-popover-command",
      group: "composer-popover",
      triggerSelector: '[data-chat-composer-popover-toggle="command"]',
      panelSelector: '[data-chat-composer-popover="command"]',
      itemSelector: '[data-chat-composer-popover="command"] .tg-composer-popover__item',
      expectedItemCount: 2,
    },
    {
      key: "command-palette",
      group: "command-palette",
      triggerSelector: '[data-control-ui-command-palette-trigger="light-glass"]',
      panelSelector: '#command-palette .command-palette',
      itemSelector: '[data-control-ui-command-palette-result="light-glass"]',
      expectedItemCount: 18,
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

async function comparePngs(page, beforeFile, afterFile) {
  const before = fs.readFileSync(beforeFile).toString("base64");
  const after = fs.readFileSync(afterFile).toString("base64");
  return page.evaluate(async ({ before, after }) => {
    const load = async (data) => {
      const image = new Image();
      image.src = `data:image/png;base64,${data}`;
      await image.decode();
      return image;
    };
    const beforeImage = await load(before);
    const afterImage = await load(after);
    const width = Math.min(beforeImage.naturalWidth, afterImage.naturalWidth);
    const height = Math.min(beforeImage.naturalHeight, afterImage.naturalHeight);
    const canvas = document.createElement("canvas");
    canvas.width = width;
    canvas.height = height;
    const context = canvas.getContext("2d", { willReadFrequently: true });
    context.drawImage(beforeImage, 0, 0);
    const beforePixels = context.getImageData(0, 0, width, height).data;
    context.clearRect(0, 0, width, height);
    context.drawImage(afterImage, 0, 0);
    const afterPixels = context.getImageData(0, 0, width, height).data;
    const step = Math.max(1, Math.ceil(Math.sqrt((width * height) / 5000)));
    let rgbDeltaSum = 0;
    let lumaDeltaSum = 0;
    let count = 0;
    for (let y = 0; y < height; y += step) {
      for (let x = 0; x < width; x += step) {
        const index = (y * width + x) * 4;
        const br = beforePixels[index];
        const bg = beforePixels[index + 1];
        const bb = beforePixels[index + 2];
        const ar = afterPixels[index];
        const ag = afterPixels[index + 1];
        const ab = afterPixels[index + 2];
        const beforeLuma = (0.2126 * br) + (0.7152 * bg) + (0.0722 * bb);
        const afterLuma = (0.2126 * ar) + (0.7152 * ag) + (0.0722 * ab);
        rgbDeltaSum += (Math.abs(ar - br) + Math.abs(ag - bg) + Math.abs(ab - bb)) / 3;
        lumaDeltaSum += Math.abs(afterLuma - beforeLuma);
        count += 1;
      }
    }
    return {
      width,
      height,
      sample_count: count,
      mean_abs_rgb_delta: rgbDeltaSum / count,
      mean_abs_luma_delta: lumaDeltaSum / count,
    };
  }, { before, after });
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

async function saveCrop(page, viewport, locator, cropKind, prefix) {
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
  return { crop_path: cropPath, crop_kind: cropKind, label: prefix };
}

async function cropElement(page, metricPage, viewport, locator, cropKind, prefix) {
  const crop = await saveCrop(page, viewport, locator, cropKind, prefix);
  if (!crop.crop_path) return crop;
  const rawMetrics = await metricsForPng(metricPage, crop.crop_path);
  const failures = pixelFailures(cropKind, rawMetrics);
  return {
    ...crop,
    metrics: normalizeMetrics(rawMetrics),
    failures,
    ready: failures.length === 0,
  };
}

async function revealTarget(page, target) {
  if (!target.revealSelector) return;
  const row = page.locator(target.revealSelector).first();
  await row.scrollIntoViewIfNeeded();
  await page.waitForTimeout(60);
}

async function pointerProfile(page) {
  return page.evaluate(() => ({
    pointer_coarse: window.matchMedia("(pointer: coarse)").matches,
    pointer_fine: window.matchMedia("(pointer: fine)").matches,
    hover_hover: window.matchMedia("(hover: hover)").matches,
    hover_none: window.matchMedia("(hover: none)").matches,
    any_hover_hover: window.matchMedia("(any-hover: hover)").matches,
    any_pointer_coarse: window.matchMedia("(any-pointer: coarse)").matches,
    max_touch_points: navigator.maxTouchPoints || 0,
  }));
}

function tapPointForBox(box, sampleName, viewport) {
  const insetX = Math.max(8, Math.min(18, box.width / 3));
  const insetY = Math.max(8, Math.min(18, box.height / 3));
  const points = {
    center: { x: box.left + box.width / 2, y: box.top + box.height / 2 },
    "inset-start": { x: box.left + insetX, y: box.top + box.height / 2 },
    "inset-end": { x: box.right - insetX, y: box.bottom - insetY },
  };
  const point = points[sampleName] || points.center;
  return {
    x: Math.max(1, Math.min(viewport.width - 1, round(point.x, 1))),
    y: Math.max(1, Math.min(viewport.height - 1, round(point.y, 1))),
  };
}

function isOutsideBox(point, box, padding = 8) {
  if (!box) return true;
  return point.x < box.left - padding
    || point.x > box.right + padding
    || point.y < box.top - padding
    || point.y > box.bottom + padding;
}

function outsideTapPoint(viewport, panelBox) {
  const candidates = [
    { x: 8, y: 8 },
    { x: viewport.width - 8, y: 8 },
    { x: 8, y: viewport.height - 8 },
    { x: viewport.width - 8, y: viewport.height - 8 },
    { x: viewport.width / 2, y: viewport.height - 8 },
    { x: viewport.width / 2, y: 8 },
  ];
  return candidates.find((point) => isOutsideBox(point, panelBox)) || null;
}

async function openWithTouch(page, viewport, target, sampleName) {
  await revealTarget(page, target);
  const trigger = page.locator(target.triggerSelector).first();
  await trigger.waitFor({ state: "visible", timeout: 5000 });
  await trigger.evaluate((element) => element.scrollIntoView({ block: "center", inline: "center" })).catch(() => {});
  await page.waitForTimeout(120);
  const triggerBox = await boxFor(trigger);
  const triggerGeometry = await geometryFor(trigger, viewport, "trigger");
  const tapPoint = triggerBox ? tapPointForBox(triggerBox, sampleName, viewport) : null;
  if (tapPoint) {
    await page.touchscreen.tap(tapPoint.x, tapPoint.y);
  }
  await page.waitForTimeout(240);
  const panel = page.locator(target.panelSelector).first();
  const panelVisible = await panel.waitFor({ state: "visible", timeout: 5000 }).then(() => true).catch(() => false);
  return { trigger, triggerGeometry, triggerBox: roundedBox(triggerBox), tapPoint, panel, panelVisible };
}

async function closeWithOutsideTap(page, viewport, panelBox) {
  const tapPoint = outsideTapPoint(viewport, panelBox);
  if (tapPoint) {
    await page.touchscreen.tap(tapPoint.x, tapPoint.y);
    await page.waitForTimeout(260);
  }
  return {
    tap_point: tapPoint,
    visible_panels_after_close: await visibleTransientPanels(page),
  };
}

async function auditMenuItems(page, metricPage, viewport, target) {
  const items = page.locator(target.itemSelector);
  const itemCount = await items.count();
  const itemAudits = [];
  for (let index = 0; index < itemCount; index += 1) {
    const item = items.nth(index);
    await item.evaluate((element) => element.scrollIntoView({ block: "center", inline: "nearest" })).catch(() => {});
    await page.waitForTimeout(70);
    const label = await item.evaluate((element, fallback) => (
      element.getAttribute("data-chat-row-menu-item")
      || element.getAttribute("data-control-ui-menu-item")
      || element.getAttribute("data-control-ui-command-palette-item")
      || element.getAttribute("data-chat-composer-picker-item")
      || element.getAttribute("aria-label")
      || element.textContent
      || `item-${fallback}`
    ), index);
    const textLabel = String(label).trim().replace(/\s+/g, " ").slice(0, 96);
    const prefix = `${sanitize(viewport.name)}-${sanitize(target.key)}-item-${index}-${sanitize(textLabel.slice(0, 72))}`;
    const geometry = await geometryFor(item, viewport, "menu-item");
    const crop = await cropElement(page, metricPage, viewport, item, "menu-item", prefix);
    const failures = [
      ...geometry.failures.map((failure) => `geometry:${failure}`),
      ...((crop.failures || []).map((failure) => `crop:${failure}`)),
    ];
    itemAudits.push({
      index,
      label: textLabel,
      geometry,
      crop,
      failures,
      ready: failures.length === 0,
    });
  }
  return { item_count: itemCount, item_audits: itemAudits };
}

async function auditTouchTarget(page, metricPage, viewport, target) {
  await page.goto(baseUrl, { waitUntil: "networkidle" });
  await page.waitForTimeout(180);
  const media = await pointerProfile(page);
  const prefix = `${sanitize(viewport.name)}-${sanitize(target.key)}`;
  const firstOpen = await openWithTouch(page, viewport, target, "center");
  const postOpenPanels = await visibleTransientPanels(page);
  const panelGeometry = firstOpen.panelVisible
    ? await geometryFor(firstOpen.panel, viewport, "panel")
    : { kind: "panel", box: null, clipped_ratio: 0, topmost: false, failures: ["panel_not_visible_after_center_touch"], ready: false };
  const panelBox = await boxFor(firstOpen.panel).catch(() => null);
  const panelCrop = firstOpen.panelVisible
    ? await cropElement(page, metricPage, viewport, firstOpen.panel, "panel", `${prefix}-panel`)
    : { crop_kind: "panel", label: `${prefix}-panel`, failures: ["panel_not_visible_after_center_touch"], ready: false };
  const menuItems = firstOpen.panelVisible
    ? await auditMenuItems(page, metricPage, viewport, target)
    : { item_count: 0, item_audits: [] };
  const screenshotPath = path.join(screenshotDir, `${prefix}-open.png`);
  await page.screenshot({ path: screenshotPath, fullPage: false });
  const closeResult = await closeWithOutsideTap(page, viewport, panelBox);
  const tapHitAudits = [{
    sample: "center",
    trigger_geometry: firstOpen.triggerGeometry,
    trigger_box: firstOpen.triggerBox,
    tap_point: firstOpen.tapPoint,
    panel_visible: firstOpen.panelVisible,
    post_open_visible_panel_count: postOpenPanels.length,
    post_close_visible_panel_count: closeResult.visible_panels_after_close.length,
    close_tap_point: closeResult.tap_point,
    failures: [
      ...(firstOpen.tapPoint ? [] : ["missing_trigger_tap_point"]),
      ...firstOpen.triggerGeometry.failures.map((failure) => `trigger:${failure}`),
      ...(firstOpen.panelVisible ? [] : ["panel_not_visible_after_touch"]),
      ...(postOpenPanels.length === 1 ? [] : [`expected_one_visible_transient_panel_got_${postOpenPanels.length}`]),
      ...(closeResult.tap_point ? [] : ["no_safe_outside_tap_point"]),
      ...(closeResult.visible_panels_after_close.length === 0 ? [] : [`outside_tap_residual_panels_${closeResult.visible_panels_after_close.length}`]),
    ],
  }];

  for (const sampleName of tapSampleNames.filter((name) => name !== "center")) {
    await page.goto(baseUrl, { waitUntil: "networkidle" });
    await page.waitForTimeout(140);
    const open = await openWithTouch(page, viewport, target, sampleName);
    const panels = await visibleTransientPanels(page);
    const samplePanelBox = await boxFor(open.panel).catch(() => null);
    const sampleClose = await closeWithOutsideTap(page, viewport, samplePanelBox);
    tapHitAudits.push({
      sample: sampleName,
      trigger_geometry: open.triggerGeometry,
      trigger_box: open.triggerBox,
      tap_point: open.tapPoint,
      panel_visible: open.panelVisible,
      post_open_visible_panel_count: panels.length,
      post_close_visible_panel_count: sampleClose.visible_panels_after_close.length,
      close_tap_point: sampleClose.tap_point,
      failures: [
        ...(open.tapPoint ? [] : ["missing_trigger_tap_point"]),
        ...open.triggerGeometry.failures.map((failure) => `trigger:${failure}`),
        ...(open.panelVisible ? [] : ["panel_not_visible_after_touch"]),
        ...(panels.length === 1 ? [] : [`expected_one_visible_transient_panel_got_${panels.length}`]),
        ...(sampleClose.tap_point ? [] : ["no_safe_outside_tap_point"]),
        ...(sampleClose.visible_panels_after_close.length === 0 ? [] : [`outside_tap_residual_panels_${sampleClose.visible_panels_after_close.length}`]),
      ],
    });
  }

  const failures = [
    ...(media.pointer_coarse ? [] : ["media_pointer_not_coarse"]),
    ...(media.hover_none ? [] : ["media_hover_not_none"]),
    ...(media.max_touch_points >= 1 ? [] : ["navigator_max_touch_points_missing"]),
    ...(postOpenPanels.length === 1 ? [] : [`expected_one_visible_transient_panel_got_${postOpenPanels.length}`]),
    ...(menuItems.item_count === target.expectedItemCount ? [] : [`expected_${target.expectedItemCount}_items_got_${menuItems.item_count}`]),
    ...(closeResult.visible_panels_after_close.length === 0 ? [] : [`outside_tap_residual_panels_${closeResult.visible_panels_after_close.length}`]),
    ...firstOpen.triggerGeometry.failures.map((failure) => `trigger:${failure}`),
    ...panelGeometry.failures.map((failure) => `panel:${failure}`),
    ...((panelCrop.failures || []).map((failure) => `panel_crop:${failure}`)),
    ...tapHitAudits.flatMap((audit) => audit.failures.map((failure) => `tap_hit:${audit.sample}:${failure}`)),
    ...menuItems.item_audits.flatMap((item) => item.failures.map((failure) => `item:${item.label}:${failure}`)),
  ];
  return {
    viewport: viewport.name,
    viewport_size: { width: viewport.width, height: viewport.height, dpr: viewport.dpr },
    target: target.key,
    group: target.group,
    pointer_profile: media,
    expected_item_count: target.expectedItemCount,
    item_count: menuItems.item_count,
    tap_hit_audits: tapHitAudits.map((audit) => ({ ...audit, ready: audit.failures.length === 0 })),
    post_open_visible_panels: postOpenPanels,
    post_outside_tap_visible_panels: closeResult.visible_panels_after_close,
    trigger: firstOpen.triggerGeometry,
    panel: panelGeometry,
    panel_crop: panelCrop,
    menu_items: menuItems.item_audits,
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
  const touchAudits = [];

  for (const viewport of viewports) {
    const page = await browser.newPage({
      viewport: { width: viewport.width, height: viewport.height },
      deviceScaleFactor: viewport.dpr,
      isMobile: true,
      hasTouch: true,
      colorScheme: "light",
      reducedMotion: "reduce",
    });
    for (const target of targetDefinitions(viewport)) {
      touchAudits.push(await auditTouchTarget(page, metricPage, viewport, target));
    }
    await page.close();
  }

  await metricPage.close();
  await browser.close();

  const tapHitAudits = touchAudits.flatMap((audit) => audit.tap_hit_audits.map((tapAudit) => ({
    viewport: audit.viewport,
    group: audit.group,
    target: audit.target,
    ...tapAudit,
  })));
  const menuItems = touchAudits.flatMap((audit) => audit.menu_items.map((item) => ({
    viewport: audit.viewport,
    group: audit.group,
    target: audit.target,
    ...item,
  })));
  const crops = [
    ...touchAudits.map((audit) => ({
      viewport: audit.viewport,
      group: audit.group,
      target: audit.target,
      ...audit.panel_crop,
    })),
    ...menuItems.map((item) => ({
      viewport: item.viewport,
      group: item.group,
      target: item.target,
      ...item.crop,
    })),
  ];
  const failures = touchAudits.filter((audit) => !audit.ready);
  const tapHitFailures = tapHitAudits.filter((audit) => !audit.ready);
  const itemFailures = menuItems.filter((item) => !item.ready);
  const cropFailures = crops.filter((crop) => !crop.ready);
  const expectedTouchAuditCount = viewports.reduce((sum, viewport) => sum + targetDefinitions(viewport).length, 0);
  const expectedTapHitAuditCount = expectedTouchAuditCount * tapSampleNames.length;
  const ready = failures.length === 0
    && tapHitFailures.length === 0
    && itemFailures.length === 0
    && cropFailures.length === 0
    && touchAudits.length === expectedTouchAuditCount
    && tapHitAudits.length === expectedTapHitAuditCount;
  const report = {
    schema_version: "hepta-ui-harsh-top-design-referee-v17-touch-coarse-pointer-census/v0",
    standards_version: "2026-06-28-control-touch-coarse-pointer-dpr-nohover-glass-census",
    status: ready ? "ready" : "failed",
    base_url: baseUrl,
    browser_path: "Browser plugin not available; regular Playwright with local Chrome was used",
    screenshot_dir: screenshotDir,
    crop_dir: cropDir,
    viewport_count: viewports.length,
    target_count: touchAudits.length,
    expected_target_count: expectedTouchAuditCount,
    tap_sample_count: tapSampleNames.length,
    tap_hit_audit_count: tapHitAudits.length,
    expected_tap_hit_audit_count: expectedTapHitAuditCount,
    menu_item_audit_count: menuItems.length,
    crop_count: crops.length,
    screenshot_count: touchAudits.length,
    failure_count: failures.length,
    tap_hit_failure_count: tapHitFailures.length,
    item_failure_count: itemFailures.length,
    crop_failure_count: cropFailures.length,
    thresholds: {
      touch_profiles: viewports.map(({ name, width, height, dpr }) => ({ name, width, height, dpr, has_touch: true, is_mobile: true })),
      tap_samples: tapSampleNames,
      media_pointer: "(pointer: coarse) must match",
      media_hover: "(hover: none) must match",
      navigator_max_touch_points_min: 1,
      trigger_min_size: "44x44",
      panel_min_size: "120x44",
      menu_item_min_size: "44x32",
      panel_clipped_ratio_min: 0.995,
      trigger_or_item_clipped_ratio_min: 0.985,
      visible_transient_panel_count_after_touch_open: 1,
      visible_transient_panel_count_after_outside_tap: 0,
      local_crop_mean_luma: "198..253",
      local_crop_luma_p95_min: 232,
      local_crop_dark_ratio_max: 0.16,
      local_crop_glass_white_ratio_min: 0.76,
      local_crop_mean_saturation_max: 0.20,
      local_crop_luma_stddev_min: 1.5,
      local_crop_texture_delta_min: 0.08,
    },
    by_viewport: summarizeBy(touchAudits, "viewport", "touch_audit_count"),
    by_group: summarizeBy(touchAudits, "group", "touch_audit_count"),
    tap_hit_by_viewport: summarizeBy(tapHitAudits, "viewport", "tap_hit_audit_count"),
    tap_hit_by_group: summarizeBy(tapHitAudits, "group", "tap_hit_audit_count"),
    crop_by_viewport: summarizeBy(crops, "viewport", "crop_count"),
    crop_by_group: summarizeBy(crops, "group", "crop_count"),
    failures,
    tap_hit_failures: tapHitFailures,
    item_failures: itemFailures,
    crop_failures: cropFailures,
    touch_audits: touchAudits,
  };
  console.log(JSON.stringify(report, null, 2));
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
NODE

if [[ "$(jq -r '.status' "$TOUCH_REPORT_PATH")" != "ready" ]]; then
  echo "v17 touch/coarse-pointer census failed" >&2
  jq '.failures[:20] | map({viewport, target, group, pointer_profile, failures})' "$TOUCH_REPORT_PATH" >&2 || true
  jq '.tap_hit_failures[:20] | map({viewport, target, group, sample, tap_point, failures})' "$TOUCH_REPORT_PATH" >&2 || true
  jq '.item_failures[:20] | map({viewport, target, group, label, failures})' "$TOUCH_REPORT_PATH" >&2 || true
  jq '.crop_failures[:20] | map({viewport, target, group, label, crop_kind, failures, metrics})' "$TOUCH_REPORT_PATH" >&2 || true
  exit 1
fi

node - "$V16_REPORT_PATH" "$TOUCH_REPORT_PATH" "$REPORT_PATH" <<'NODE'
const crypto = require("node:crypto");
const fs = require("node:fs");

const [v16Path, touchPath, outputPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const v16 = readJson(v16Path);
const touch = readJson(touchPath);
const ready = v16.status === "ready"
  && touch.status === "ready"
  && touch.failure_count === 0
  && touch.tap_hit_failure_count === 0
  && touch.item_failure_count === 0
  && touch.crop_failure_count === 0
  && touch.target_count === touch.expected_target_count
  && touch.tap_hit_audit_count === touch.expected_tap_hit_audit_count;
const report = {
  schema_version: "hepta-ui-harsh-top-design-referee-v17-gate/v0",
  standards_version: "2026-06-28-harsh-v16-plus-touch-coarse-pointer-dpr-nohover-glass-census",
  status: ready ? "ready" : "failed",
  browser_path: "Browser plugin not available; regular Playwright with local Chrome was used",
  inputs: {
    v16_keyboard_focus: { path: v16Path, sha256: sha256(v16Path) },
    touch_coarse_pointer_census: { path: touchPath, sha256: sha256(touchPath) },
  },
  summary: {
    v16_keyboard_focus: v16.summary?.v16_keyboard_focus,
    v15_text_zoom_squeeze: v16.summary?.v15_text_zoom_squeeze,
    v14_scroll_edge_crop: v16.summary?.v14_scroll_edge_crop,
    v13_geometry_occlusion: v16.summary?.v13_geometry_occlusion,
    v12_interaction_crop: v16.summary?.v12_interaction_crop,
    v17_touch_coarse_pointer: {
      viewport_count: touch.viewport_count,
      target_count: touch.target_count,
      tap_sample_count: touch.tap_sample_count,
      tap_hit_audit_count: touch.tap_hit_audit_count,
      menu_item_audit_count: touch.menu_item_audit_count,
      crop_count: touch.crop_count,
      screenshot_count: touch.screenshot_count,
      failure_count: touch.failure_count,
      tap_hit_failure_count: touch.tap_hit_failure_count,
      item_failure_count: touch.item_failure_count,
      crop_failure_count: touch.crop_failure_count,
      by_viewport: touch.by_viewport,
      by_group: touch.by_group,
      tap_hit_by_viewport: touch.tap_hit_by_viewport,
      tap_hit_by_group: touch.tap_hit_by_group,
      crop_by_viewport: touch.crop_by_viewport,
      crop_by_group: touch.crop_by_group,
      thresholds: touch.thresholds,
    },
  },
  v16_ready: v16.status === "ready",
  touch_coarse_pointer_ready: touch.status === "ready",
  touch_coarse_pointer_census: touch,
};
fs.writeFileSync(outputPath, JSON.stringify(report, null, 2) + "\n");
console.log(JSON.stringify(report, null, 2));
NODE
