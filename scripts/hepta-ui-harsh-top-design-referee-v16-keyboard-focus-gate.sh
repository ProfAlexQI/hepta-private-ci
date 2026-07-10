#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V16_REPORT_PATH:-}"
V15_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V15_REPORT_PATH:-}"
KEYBOARD_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V16_KEYBOARD_REPORT_PATH:-}"
KEYBOARD_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V16_SCREENSHOT_DIR:-}"
KEYBOARD_CROP_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V16_CROP_DIR:-}"
NATIVE_DIR="${HEPTA_NATIVE_FIXTURE_VISUAL_DIR:-}"
V15_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V16_V15_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v16-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v16-keyboard-focus-gate.json"
fi
if [[ -z "$V15_REPORT_PATH" ]]; then
  V15_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v15-text-zoom-squeeze-gate.json"
fi
if [[ -z "$KEYBOARD_REPORT_PATH" ]]; then
  KEYBOARD_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v16-keyboard-focus-census.json"
fi
if [[ -z "$KEYBOARD_SCREENSHOT_DIR" ]]; then
  KEYBOARD_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v16-keyboard-focus-screenshots"
fi
if [[ -z "$KEYBOARD_CROP_DIR" ]]; then
  KEYBOARD_CROP_DIR="$READINESS_DIR/ui-harsh-v16-keyboard-focus-crops"
fi
if [[ -z "$NATIVE_DIR" ]]; then
  NATIVE_DIR="$READINESS_DIR/native-fixture"
fi
if [[ -z "$V15_LOG" ]]; then
  V15_LOG="$READINESS_DIR/v15-text-zoom-squeeze.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$NATIVE_DIR" "$KEYBOARD_SCREENSHOT_DIR" "$KEYBOARD_CROP_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$KEYBOARD_REPORT_PATH")"

HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V15_REPORT_PATH="$V15_REPORT_PATH" \
HEPTA_NATIVE_FIXTURE_VISUAL_DIR="$NATIVE_DIR" \
  bash scripts/hepta-ui-harsh-top-design-referee-v15-text-zoom-squeeze-gate.sh "$READINESS_DIR" >"$V15_LOG" 2>&1 || {
    echo "v15 text-zoom squeeze prerequisite failed" >&2
    tail -n 180 "$V15_LOG" >&2 || true
    exit 1
  }

if [[ "$(jq -r '.status' "$V15_REPORT_PATH")" != "ready" ]]; then
  echo "v15 text-zoom squeeze prerequisite was not ready: $V15_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7434 7435 7436 7437 7438; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v16 referee" >&2
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
      echo "Hepta Control UI server exited before v16 keyboard/focus audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$KEYBOARD_SCREENSHOT_DIR" "$KEYBOARD_CROP_DIR" >"$KEYBOARD_REPORT_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir, cropDir] = process.argv.slice(2);
const viewports = [
  { name: "desktop", width: 1365, height: 900, railVisible: true },
  { name: "narrow", width: 768, height: 900, railVisible: true },
  { name: "mobile", width: 500, height: 844, railVisible: false },
  { name: "phone320", width: 320, height: 700, railVisible: false },
];
const activationKeys = ["Enter", "Space"];
const transientPanelSelector = [
  "[data-chat-row-menu-panel]",
  "[data-control-ui-thread-tools-panel]",
  "[data-control-ui-composer-tools-panel]",
  "[data-chat-composer-popover]",
  "#command-palette .command-palette",
].join(",");
const focusableSelector = [
  "button",
  "a[href]",
  "summary",
  "input",
  "select",
  "textarea",
  "[role='button']",
  "[role='menuitem']",
  "[tabindex]:not([tabindex='-1'])",
].join(",");
const nativeFocusableSelector = [
  "button",
  "a[href]",
  "summary",
  "input",
  "select",
  "textarea",
  "[tabindex]:not([tabindex='-1'])",
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

async function focusSignals(locator) {
  return locator.evaluate((element) => {
    const active = document.activeElement;
    const style = window.getComputedStyle(element);
    const rect = element.getBoundingClientRect();
    const outlineWidth = Number.parseFloat(style.outlineWidth || "0") || 0;
    const boxShadow = style.boxShadow || "none";
    const visible = rect.width > 0
      && rect.height > 0
      && style.display !== "none"
      && style.visibility !== "hidden"
      && Number(style.opacity || 1) > 0.01;
    const disabled = element.matches(":disabled,[aria-disabled='true']");
    const nativeFocusable = element.matches("button,a[href],summary,input,select,textarea,[tabindex]");
    return {
      active: Boolean(active && (active === element || element.contains(active))),
      visible,
      disabled,
      tab_index: element.tabIndex,
      native_focusable: nativeFocusable,
      focus_style_visible: outlineWidth >= 1 || (boxShadow !== "none" && boxShadow.length > 0),
      outline_width: outlineWidth,
      outline_style: style.outlineStyle,
      box_shadow: boxShadow,
      opacity: Number(style.opacity || 1),
    };
  }).catch((error) => ({ error: String(error?.message || error), active: false, visible: false, disabled: true, focus_style_visible: false }));
}

async function focusableFor(item) {
  const nestedNative = item.locator(nativeFocusableSelector).first();
  if (await nestedNative.count().catch(() => 0)) return nestedNative;
  const selfMatches = await item.evaluate((element, selector) => element.matches(selector), focusableSelector).catch(() => false);
  if (selfMatches) return item;
  const nested = item.locator(focusableSelector).first();
  if (await nested.count().catch(() => 0)) return nested;
  return item;
}

async function focusAudit(page, metricPage, viewport, locator, cropKind, prefix) {
  const before = await saveCrop(page, viewport, locator, cropKind, `${prefix}-before`).catch((error) => ({
    ready: false,
    failures: [`before_crop_failed:${String(error?.message || error).slice(0, 120)}`],
  }));
  const focusTarget = await focusableFor(locator);
  await focusTarget.focus({ timeout: 5000 }).catch(async () => {
    await locator.focus({ timeout: 5000 });
  });
  await page.waitForTimeout(120);
  const signals = await focusSignals(locator);
  const focusTargetSignals = await focusSignals(focusTarget);
  const geometry = await geometryFor(locator, viewport, cropKind === "trigger" ? "trigger" : "menu-item");
  const focusCrop = await cropElement(page, metricPage, viewport, locator, cropKind, `${prefix}-focus`);
  const delta = before.crop_path && focusCrop.crop_path
    ? normalizeMetrics(await comparePngs(metricPage, before.crop_path, focusCrop.crop_path))
    : null;
  const deltaReady = signals.focus_style_visible || (delta && (delta.mean_abs_luma_delta >= 0.20 || delta.mean_abs_rgb_delta >= 0.20));
  const keyboardFocusable = signals.native_focusable || signals.tab_index >= 0 || focusTargetSignals.native_focusable || focusTargetSignals.tab_index >= 0;
  const failures = [
    ...(signals.active ? [] : ["focus_not_active_or_contained"]),
    ...(signals.visible ? [] : ["focused_element_not_visible"]),
    ...(!signals.disabled ? [] : ["focused_element_disabled"]),
    ...(keyboardFocusable ? [] : ["element_not_keyboard_focusable"]),
    ...(signals.focus_style_visible ? [] : ["focus_style_not_visible"]),
    ...(deltaReady ? [] : ["focus_delta_below_threshold"]),
    ...geometry.failures.map((failure) => `geometry:${failure}`),
    ...((focusCrop.failures || []).map((failure) => `focus_crop:${failure}`)),
  ];
  return {
    crop_kind: cropKind,
    label: prefix,
    signals,
    focus_target_signals: focusTargetSignals,
    geometry,
    before_crop_path: before.crop_path || null,
    focus_crop: focusCrop,
    focus_delta: delta,
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

async function keyboardOpen(page, target, key) {
  await revealTarget(page, target);
  const trigger = page.locator(target.triggerSelector).first();
  await trigger.waitFor({ state: "attached", timeout: 5000 });
  await trigger.evaluate((element) => element.scrollIntoView({ block: "center", inline: "center" })).catch(() => {});
  await trigger.focus({ timeout: 5000 });
  await page.waitForTimeout(140);
  await page.keyboard.press(key);
  await page.waitForTimeout(220);
  const panel = page.locator(target.panelSelector).first();
  await panel.waitFor({ state: "visible", timeout: 5000 });
  return trigger;
}

async function closeWithEscape(page) {
  await page.keyboard.press("Escape");
  await page.waitForTimeout(180);
  return visibleTransientPanels(page);
}

async function auditKeyboardActivation(page, metricPage, viewport, target, key) {
  await page.goto(baseUrl, { waitUntil: "networkidle" });
  await page.waitForTimeout(160);
  await revealTarget(page, target);
  const trigger = page.locator(target.triggerSelector).first();
  await trigger.waitFor({ state: "attached", timeout: 5000 });
  const prefix = `${sanitize(viewport.name)}-${sanitize(target.key)}-${sanitize(key)}`;
  const triggerFocus = await focusAudit(page, metricPage, viewport, trigger, "trigger", `${prefix}-trigger`);
  await page.keyboard.press(key);
  await page.waitForTimeout(220);
  const panel = page.locator(target.panelSelector).first();
  const postOpenPanels = await visibleTransientPanels(page);
  const itemCount = await page.locator(target.itemSelector).count();
  const panelGeometry = await geometryFor(panel, viewport, "panel");
  const screenshotPath = path.join(screenshotDir, `${prefix}-open.png`);
  await page.screenshot({ path: screenshotPath, fullPage: false });
  const postClosePanels = await closeWithEscape(page);
  const failures = [
    ...(postOpenPanels.length === 1 ? [] : [`expected_one_visible_transient_panel_got_${postOpenPanels.length}`]),
    ...(itemCount === target.expectedItemCount ? [] : [`expected_${target.expectedItemCount}_items_got_${itemCount}`]),
    ...(postClosePanels.length === 0 ? [] : [`escape_close_residual_panels_${postClosePanels.length}`]),
    ...triggerFocus.failures.map((failure) => `trigger_focus:${failure}`),
    ...panelGeometry.failures.map((failure) => `panel:${failure}`),
  ];
  return {
    viewport: viewport.name,
    target: target.key,
    group: target.group,
    activation_key: key,
    expected_item_count: target.expectedItemCount,
    item_count: itemCount,
    post_open_visible_panels: postOpenPanels,
    post_escape_visible_panels: postClosePanels,
    trigger_focus: triggerFocus,
    panel: panelGeometry,
    screenshot: { path: screenshotPath, sha256: sha256(screenshotPath) },
    failures,
    ready: failures.length === 0,
  };
}

async function auditItemFocus(page, metricPage, viewport, target) {
  await page.goto(baseUrl, { waitUntil: "networkidle" });
  await page.waitForTimeout(160);
  await keyboardOpen(page, target, "Enter");
  const items = page.locator(target.itemSelector);
  const itemCount = await items.count();
  const itemFocuses = [];
  for (let index = 0; index < itemCount; index += 1) {
    const item = items.nth(index);
    await item.evaluate((element) => element.scrollIntoView({ block: "center", inline: "nearest" })).catch(() => {});
    await page.waitForTimeout(45);
    const label = await item.evaluate((element, fallback) => (
      element.getAttribute("data-chat-row-menu-item")
      || element.getAttribute("data-control-ui-menu-item")
      || element.getAttribute("data-control-ui-command-palette-item")
      || element.getAttribute("data-chat-composer-picker-item")
      || element.getAttribute("aria-label")
      || element.textContent
      || `item-${fallback}`
    ), index);
    const textLabel = String(label).trim().slice(0, 96);
    const focusTarget = await focusableFor(item);
    await focusTarget.focus({ timeout: 5000 }).catch(() => {});
    await page.waitForTimeout(100);
    itemFocuses.push({
      index,
      label: textLabel,
      ...(await focusAudit(page, metricPage, viewport, item, "menu-item", `${sanitize(viewport.name)}-${sanitize(target.key)}-item-${index}-${sanitize(textLabel.slice(0, 72))}`)),
    });
  }
  const postClosePanels = await closeWithEscape(page);
  const failures = [
    ...(itemCount === target.expectedItemCount ? [] : [`expected_${target.expectedItemCount}_items_got_${itemCount}`]),
    ...(postClosePanels.length === 0 ? [] : [`escape_after_item_focus_residual_panels_${postClosePanels.length}`]),
    ...itemFocuses.flatMap((item) => item.failures.map((failure) => `item:${item.label}:${failure}`)),
  ];
  return {
    viewport: viewport.name,
    target: target.key,
    group: target.group,
    expected_item_count: target.expectedItemCount,
    item_count: itemCount,
    item_focuses: itemFocuses,
    post_escape_visible_panels: postClosePanels,
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
  const activationAudits = [];
  const itemFocusAudits = [];

  for (const viewport of viewports) {
    const page = await browser.newPage({ viewport: { width: viewport.width, height: viewport.height }, deviceScaleFactor: 1 });
    for (const target of targetDefinitions(viewport)) {
      for (const key of activationKeys) {
        activationAudits.push(await auditKeyboardActivation(page, metricPage, viewport, target, key));
      }
      itemFocusAudits.push(await auditItemFocus(page, metricPage, viewport, target));
    }
    await page.close();
  }

  await metricPage.close();
  await browser.close();

  const triggerFocuses = activationAudits.map((audit) => ({
    viewport: audit.viewport,
    group: audit.group,
    target: audit.target,
    activation_key: audit.activation_key,
    ...audit.trigger_focus,
  }));
  const itemFocuses = itemFocusAudits.flatMap((audit) => audit.item_focuses.map((item) => ({
    viewport: audit.viewport,
    group: audit.group,
    target: audit.target,
    ...item,
  })));
  const focusCrops = [
    ...triggerFocuses.map((focus) => ({ ...focus, ...focus.focus_crop })),
    ...itemFocuses.map((focus) => ({ ...focus, ...focus.focus_crop })),
  ];
  const failures = [
    ...activationAudits.filter((audit) => !audit.ready),
    ...itemFocusAudits.filter((audit) => !audit.ready),
  ];
  const focusFailures = [
    ...triggerFocuses.filter((focus) => !focus.ready),
    ...itemFocuses.filter((focus) => !focus.ready),
  ];
  const cropFailures = focusCrops.filter((crop) => !crop.ready);
  const expectedActivationAuditCount = viewports.reduce((sum, viewport) => (
    sum + targetDefinitions(viewport).length * activationKeys.length
  ), 0);
  const expectedItemFocusAuditCount = viewports.reduce((sum, viewport) => sum + targetDefinitions(viewport).length, 0);
  const ready = failures.length === 0
    && focusFailures.length === 0
    && cropFailures.length === 0
    && activationAudits.length === expectedActivationAuditCount
    && itemFocusAudits.length === expectedItemFocusAuditCount;
  const report = {
    schema_version: "hepta-ui-harsh-top-design-referee-v16-keyboard-focus-census/v0",
    standards_version: "2026-06-28-control-keyboard-focus-submenu-glass-census",
    status: ready ? "ready" : "failed",
    base_url: baseUrl,
    browser_path: "Browser plugin not available; regular Playwright with local Chrome was used",
    screenshot_dir: screenshotDir,
    crop_dir: cropDir,
    viewport_count: viewports.length,
    target_count: expectedItemFocusAuditCount,
    activation_key_count: activationKeys.length,
    activation_audit_count: activationAudits.length,
    expected_activation_audit_count: expectedActivationAuditCount,
    item_focus_audit_count: itemFocusAudits.length,
    expected_item_focus_audit_count: expectedItemFocusAuditCount,
    trigger_focus_count: triggerFocuses.length,
    menu_item_focus_count: itemFocuses.length,
    focus_crop_count: focusCrops.length,
    screenshot_count: activationAudits.length,
    failure_count: failures.length,
    focus_failure_count: focusFailures.length,
    crop_failure_count: cropFailures.length,
    thresholds: {
      activation_keys: activationKeys,
      trigger_min_size: "44x44",
      panel_min_size: "120x44",
      menu_item_min_size: "44x32",
      panel_clipped_ratio_min: 0.995,
      trigger_or_item_clipped_ratio_min: 0.985,
      visible_transient_panel_count_after_keyboard_open: 1,
      visible_transient_panel_count_after_escape: 0,
      focus_style_visible: "outline >= 1px or non-none box-shadow",
      focus_delta_min: "mean_abs_luma_delta >= 0.20 or mean_abs_rgb_delta >= 0.20 or visible focus style",
      local_crop_mean_luma: "198..253",
      local_crop_luma_p95_min: 232,
      local_crop_dark_ratio_max: 0.16,
      local_crop_glass_white_ratio_min: 0.76,
      local_crop_mean_saturation_max: 0.20,
      local_crop_luma_stddev_min: 1.5,
      local_crop_texture_delta_min: 0.08,
    },
    by_viewport: summarizeBy(activationAudits, "viewport", "activation_audit_count"),
    by_group: summarizeBy(activationAudits, "group", "activation_audit_count"),
    by_activation_key: summarizeBy(activationAudits, "activation_key", "activation_audit_count"),
    item_focus_by_viewport: summarizeBy(itemFocusAudits, "viewport", "item_focus_audit_count"),
    item_focus_by_group: summarizeBy(itemFocusAudits, "group", "item_focus_audit_count"),
    focus_crop_by_viewport: summarizeBy(focusCrops, "viewport", "focus_crop_count"),
    focus_crop_by_group: summarizeBy(focusCrops, "group", "focus_crop_count"),
    failures,
    focus_failures: focusFailures,
    crop_failures: cropFailures,
    activation_audits: activationAudits,
    item_focus_audits: itemFocusAudits,
  };
  console.log(JSON.stringify(report, null, 2));
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
NODE

if [[ "$(jq -r '.status' "$KEYBOARD_REPORT_PATH")" != "ready" ]]; then
  echo "v16 keyboard/focus census failed" >&2
  jq '.failures[:20] | map({viewport, target, group, activation_key, failures})' "$KEYBOARD_REPORT_PATH" >&2 || true
  jq '.focus_failures[:20] | map({viewport, target, group, activation_key, label, failures, signals, focus_delta})' "$KEYBOARD_REPORT_PATH" >&2 || true
  jq '.crop_failures[:20] | map({viewport, target, group, activation_key, label, failures, metrics})' "$KEYBOARD_REPORT_PATH" >&2 || true
  exit 1
fi

node - "$V15_REPORT_PATH" "$KEYBOARD_REPORT_PATH" "$REPORT_PATH" <<'NODE'
const crypto = require("node:crypto");
const fs = require("node:fs");

const [v15Path, keyboardPath, outputPath] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const v15 = readJson(v15Path);
const keyboard = readJson(keyboardPath);
const ready = v15.status === "ready"
  && keyboard.status === "ready"
  && keyboard.failure_count === 0
  && keyboard.focus_failure_count === 0
  && keyboard.crop_failure_count === 0
  && keyboard.activation_audit_count === keyboard.expected_activation_audit_count
  && keyboard.item_focus_audit_count === keyboard.expected_item_focus_audit_count;
const report = {
  schema_version: "hepta-ui-harsh-top-design-referee-v16-gate/v0",
  standards_version: "2026-06-28-harsh-v15-plus-keyboard-focus-glass-census",
  status: ready ? "ready" : "failed",
  browser_path: "Browser plugin not available; regular Playwright with local Chrome was used",
  inputs: {
    v15_text_zoom_squeeze: { path: v15Path, sha256: sha256(v15Path) },
    keyboard_focus_census: { path: keyboardPath, sha256: sha256(keyboardPath) },
  },
  summary: {
    v15_text_zoom_squeeze: v15.summary?.v15_text_zoom_squeeze,
    v14_scroll_edge_crop: v15.summary?.v14_scroll_edge_crop,
    v13_geometry_occlusion: v15.summary?.v13_geometry_occlusion,
    v12_interaction_crop: v15.summary?.v12_interaction_crop,
    v16_keyboard_focus: {
      viewport_count: keyboard.viewport_count,
      target_count: keyboard.target_count,
      activation_key_count: keyboard.activation_key_count,
      activation_audit_count: keyboard.activation_audit_count,
      item_focus_audit_count: keyboard.item_focus_audit_count,
      trigger_focus_count: keyboard.trigger_focus_count,
      menu_item_focus_count: keyboard.menu_item_focus_count,
      focus_crop_count: keyboard.focus_crop_count,
      screenshot_count: keyboard.screenshot_count,
      failure_count: keyboard.failure_count,
      focus_failure_count: keyboard.focus_failure_count,
      crop_failure_count: keyboard.crop_failure_count,
      by_viewport: keyboard.by_viewport,
      by_group: keyboard.by_group,
      by_activation_key: keyboard.by_activation_key,
      item_focus_by_viewport: keyboard.item_focus_by_viewport,
      item_focus_by_group: keyboard.item_focus_by_group,
      focus_crop_by_viewport: keyboard.focus_crop_by_viewport,
      focus_crop_by_group: keyboard.focus_crop_by_group,
      thresholds: keyboard.thresholds,
    },
  },
  v15_ready: v15.status === "ready",
  keyboard_focus_ready: keyboard.status === "ready",
  keyboard_focus_census: keyboard,
};
fs.writeFileSync(outputPath, JSON.stringify(report, null, 2) + "\n");
console.log(JSON.stringify(report, null, 2));
NODE
