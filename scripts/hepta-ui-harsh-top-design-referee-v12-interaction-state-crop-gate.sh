#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V12_REPORT_PATH:-}"
V11_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V11_REPORT_PATH:-}"
NATIVE_DIR="${HEPTA_NATIVE_FIXTURE_VISUAL_DIR:-}"
NATIVE_REPORT_PATH="${HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH:-}"
INTERACTION_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V12_INTERACTION_REPORT_PATH:-}"
INTERACTION_CROP_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V12_INTERACTION_CROP_DIR:-}"
NATIVE_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V12_NATIVE_LOG:-}"
V11_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V12_V11_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v12-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"
SKIP_V11="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V12_SKIP_V11:-0}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v12-interaction-state-crop-gate.json"
fi
if [[ -z "$V11_REPORT_PATH" ]]; then
  V11_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v11-control-component-crop-gate.json"
fi
if [[ -z "$NATIVE_DIR" ]]; then
  NATIVE_DIR="$READINESS_DIR/native-fixture"
fi
if [[ -z "$NATIVE_REPORT_PATH" ]]; then
  NATIVE_REPORT_PATH="$NATIVE_DIR/native-fixture-visual-smoke.json"
fi
if [[ -z "$INTERACTION_REPORT_PATH" ]]; then
  INTERACTION_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v12-interaction-state-crop-census.json"
fi
if [[ -z "$INTERACTION_CROP_DIR" ]]; then
  INTERACTION_CROP_DIR="$READINESS_DIR/ui-harsh-v12-interaction-state-crops"
fi
if [[ -z "$NATIVE_LOG" ]]; then
  NATIVE_LOG="$READINESS_DIR/native-fixture-visual-smoke.log"
fi
if [[ -z "$V11_LOG" ]]; then
  V11_LOG="$READINESS_DIR/v11-control-component-crop.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$NATIVE_DIR" "$INTERACTION_CROP_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$INTERACTION_REPORT_PATH")"

HEPTA_NATIVE_FIXTURE_VISUAL_DIR="$NATIVE_DIR" \
  bash scripts/hepta-native-fixture-visual-smoke.sh >"$NATIVE_LOG" 2>&1 || {
    echo "fresh Native fixture visual smoke prerequisite failed" >&2
    tail -n 160 "$NATIVE_LOG" >&2 || true
    exit 1
  }

if [[ "$(jq -r '.status' "$NATIVE_REPORT_PATH")" != "ready" ]]; then
  echo "fresh Native fixture visual smoke was not ready: $NATIVE_REPORT_PATH" >&2
  exit 1
fi

if [[ "$SKIP_V11" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V11_REPORT_PATH="$V11_REPORT_PATH" \
  HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH="$NATIVE_REPORT_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v11-control-component-crop-gate.sh "$READINESS_DIR" >"$V11_LOG" 2>&1 || {
      echo "v11 control component crop prerequisite failed" >&2
      tail -n 180 "$V11_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V11_REPORT_PATH")" != "ready" ]]; then
  echo "v11 control component crop prerequisite was not ready: $V11_REPORT_PATH" >&2
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
  echo "no free local port found for Hepta Control UI v12 referee" >&2
  exit 1
fi

BASE_URL="http://${BIND_ADDR}"
server_pid=""
tmp_report="$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v12-final.XXXXXX")"

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
    cargo run --manifest-path "$MANIFEST" -q -p codex-cli --bin hepta -- --serve-ui "$BIND_ADDR" \
    >"$SERVER_LOG" 2>&1 &
  server_pid="$!"
}

wait_for_server() {
  local deadline=$((SECONDS + STARTUP_TIMEOUT_SEC))
  local root_probe=""
  until root_probe="$(curl -fsS "$BASE_URL/" 2>/dev/null)" && [[ "$root_probe" == *'data-rust-rendered-control-ui="true"'* ]]; do
    if ! kill -0 "$server_pid" 2>/dev/null; then
      echo "Hepta Control UI server exited before v12 interaction-state audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$INTERACTION_CROP_DIR" >"$INTERACTION_REPORT_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, cropDir] = process.argv.slice(2);
const viewports = [
  { name: "desktop", width: 1365, height: 900, railVisible: true },
  { name: "narrow", width: 768, height: 900, railVisible: true },
  { name: "mobile", width: 500, height: 844, railVisible: false },
  { name: "phone320", width: 320, height: 844, railVisible: false },
];
const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase();
const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const round = (value, digits = 3) => Number(value.toFixed(digits));
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

async function visibleBox(locator) {
  await locator.waitFor({ state: "visible", timeout: 5000 });
  const box = await locator.boundingBox();
  if (!box) return null;
  return {
    x: box.x,
    y: box.y,
    width: box.width,
    height: box.height,
    left: box.x,
    top: box.y,
    right: box.x + box.width,
    bottom: box.y + box.height,
  };
}

function paddedRect(box, viewport, padding = 8) {
  const left = Math.max(0, Math.floor(box.left - padding));
  const top = Math.max(0, Math.floor(box.top - padding));
  const right = Math.min(viewport.width, Math.ceil(box.right + padding));
  const bottom = Math.min(viewport.height, Math.ceil(box.bottom + padding));
  return { left, top, width: Math.max(1, right - left), height: Math.max(1, bottom - top), right, bottom };
}

async function cropFromPage(page, metricPage, rect, outputFile) {
  await page.screenshot({
    path: outputFile,
    fullPage: false,
    clip: {
      x: Math.max(0, rect.left),
      y: Math.max(0, rect.top),
      width: Math.max(1, rect.width),
      height: Math.max(1, rect.height),
    },
  });
  return metricsForPng(metricPage, outputFile);
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
        if (luma >= 245) highlightCount += 1;
        if (luma <= 95) darkCount += 1;
        if (max - min >= 10 && luma >= 140) chromaticCount += 1;
        if (luma >= 178 && saturation <= 0.30) glassWhiteCount += 1;
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

async function imageDelta(page, baselineFile, stateFile) {
  const baseline = fs.readFileSync(baselineFile).toString("base64");
  const state = fs.readFileSync(stateFile).toString("base64");
  return page.evaluate(async ({ baseline, state }) => {
    const load = async (data) => {
      const image = new Image();
      image.src = `data:image/png;base64,${data}`;
      await image.decode();
      const canvas = document.createElement("canvas");
      canvas.width = image.naturalWidth;
      canvas.height = image.naturalHeight;
      const context = canvas.getContext("2d", { willReadFrequently: true });
      context.drawImage(image, 0, 0);
      return { canvas, pixels: context.getImageData(0, 0, canvas.width, canvas.height).data };
    };
    const a = await load(baseline);
    const b = await load(state);
    const width = Math.min(a.canvas.width, b.canvas.width);
    const height = Math.min(a.canvas.height, b.canvas.height);
    const step = Math.max(1, Math.ceil(Math.sqrt((width * height) / 5000)));
    let count = 0;
    let lumaSum = 0;
    let rgbSum = 0;
    for (let y = 0; y < height; y += step) {
      for (let x = 0; x < width; x += step) {
        const ai = (y * a.canvas.width + x) * 4;
        const bi = (y * b.canvas.width + x) * 4;
        const ar = a.pixels[ai], ag = a.pixels[ai + 1], ab = a.pixels[ai + 2];
        const br = b.pixels[bi], bg = b.pixels[bi + 1], bb = b.pixels[bi + 2];
        const al = (0.2126 * ar) + (0.7152 * ag) + (0.0722 * ab);
        const bl = (0.2126 * br) + (0.7152 * bg) + (0.0722 * bb);
        lumaSum += Math.abs(al - bl);
        rgbSum += (Math.abs(ar - br) + Math.abs(ag - bg) + Math.abs(ab - bb)) / 3;
        count += 1;
      }
    }
    return {
      sample_count: count,
      mean_abs_luma_delta: lumaSum / Math.max(1, count),
      mean_abs_rgb_delta: rgbSum / Math.max(1, count),
    };
  }, { baseline, state });
}

async function computedSignature(locator) {
  return locator.evaluate((element) => {
    const target = element.matches("label") ? (element.querySelector("select") || element) : element;
    const style = window.getComputedStyle(element);
    const focusStyle = window.getComputedStyle(target);
    return {
      active_element_matches: document.activeElement === element || element.contains(document.activeElement),
      background_color: style.backgroundColor,
      background_image: style.backgroundImage,
      border_color: style.borderColor,
      box_shadow: style.boxShadow,
      filter: style.filter,
      outline_color: focusStyle.outlineColor,
      outline_style: focusStyle.outlineStyle,
      outline_width: focusStyle.outlineWidth,
      transform: style.transform,
    };
  });
}

function signatureChanged(a, b) {
  return [
    "background_color",
    "background_image",
    "border_color",
    "box_shadow",
    "filter",
    "outline_color",
    "outline_style",
    "outline_width",
    "transform",
  ].some((key) => (a?.[key] || "") !== (b?.[key] || ""));
}

function normalizeMetrics(rawMetrics) {
  return Object.fromEntries(Object.entries(rawMetrics).map(([key, value]) => [
    key,
    typeof value === "number" && !Number.isInteger(value) ? round(value) : value,
  ]));
}

function failuresFor({ kind, state, rawMetrics, delta, signature_delta, focused }) {
  const base = [
    ...(rawMetrics.bytes >= 900 ? [] : ["too_few_bytes"]),
    ...(rawMetrics.width >= 32 && rawMetrics.height >= 32 ? [] : ["crop_too_small"]),
    ...(rawMetrics.mean_luma >= 204 && rawMetrics.mean_luma <= 252 ? [] : ["mean_luma_out_of_range"]),
    ...(rawMetrics.luma_p95 >= 232 ? [] : ["weak_local_highlights"]),
    ...(rawMetrics.dark_ratio <= 0.14 ? [] : ["too_much_local_dark_area"]),
    ...(rawMetrics.glass_white_ratio >= 0.78 ? [] : ["insufficient_local_light_glass_area"]),
    ...(rawMetrics.mean_saturation <= 0.16 ? [] : ["oversaturated_local_palette"]),
    ...(rawMetrics.luma_stddev >= 2.0 ? [] : ["locally_flat_luma"]),
    ...(rawMetrics.texture_delta >= 0.12 ? [] : ["insufficient_local_texture_signal"]),
  ];
  if (state === "focus") {
    base.push(...(focused ? [] : ["focus_not_landed_on_target"]));
  }
  if (state !== "normal" && state !== "revealed") {
    const deltaReady = (delta?.mean_abs_luma_delta || 0) >= 0.20 || (delta?.mean_abs_rgb_delta || 0) >= 0.20 || signature_delta === true;
    if (!deltaReady) base.push("interaction_state_has_no_visible_affordance_delta");
  }
  if (kind === "trigger" && state === "active") {
    const activeDeltaReady = (delta?.mean_abs_luma_delta || 0) >= 0.35 || (delta?.mean_abs_rgb_delta || 0) >= 0.35 || signature_delta === true;
    if (!activeDeltaReady) base.push("active_state_delta_too_weak");
  }
  return base;
}

function boxContainsPoint(box, point, padding = 12) {
  if (!box || !point) return false;
  return (
    point.x >= box.left - padding &&
    point.x <= box.right + padding &&
    point.y >= box.top - padding &&
    point.y <= box.bottom + padding
  );
}

async function resetPointer(page, avoidBox = null) {
  await page.mouse.up().catch(() => {});
  const viewport = page.viewportSize() || { width: 1200, height: 800 };
  const candidates = [
    { x: viewport.width - 4, y: viewport.height - 4 },
    { x: 4, y: viewport.height - 4 },
    { x: viewport.width - 4, y: 4 },
    { x: Math.floor(viewport.width / 2), y: viewport.height - 4 },
    { x: 4, y: 4 },
  ];
  const point = candidates.find((candidate) => !boxContainsPoint(avoidBox, candidate)) || candidates[0];
  await page.mouse.move(point.x, point.y);
  await page.waitForTimeout(40);
}

async function closeTransientPanels(page) {
  await page.mouse.up().catch(() => {});
  await page.keyboard.press("Escape").catch(() => {});
  await page.evaluate(() => {
    if (document.activeElement instanceof HTMLElement) {
      document.activeElement.blur();
    }
    for (const details of document.querySelectorAll("details[open]")) {
      details.removeAttribute("open");
    }
    for (const row of document.querySelectorAll(".tg-chat-item--menu-open")) {
      row.classList.remove("tg-chat-item--menu-open");
    }
    if (window.location.hash === "#command-palette") {
      window.history.replaceState(null, "", "#commands");
    }
  });
  await page.mouse.move(2, 2);
  await page.waitForTimeout(90);
}

async function revealTarget(page, target) {
  if (!target.revealSelector) return;
  const row = page.locator(target.revealSelector).first();
  await row.scrollIntoViewIfNeeded();
  await row.hover({ position: { x: 16, y: 16 } });
  await page.waitForTimeout(90);
}

async function focusLocator(locator) {
  const tagName = await locator.evaluate((element) => element.tagName.toLowerCase());
  if (tagName === "label") {
    await locator.evaluate((element) => {
      const target = element.querySelector("select") || element.querySelector("input") || element;
      target.focus({ preventScroll: true });
    });
    return;
  }
  await locator.focus({ timeout: 5000 });
  const focused = await locator.evaluate((element) => document.activeElement === element || element.contains(document.activeElement));
  if (!focused) {
    await locator.evaluate((element) => {
      if (!element.hasAttribute("tabindex")) element.setAttribute("tabindex", "0");
      element.focus({ preventScroll: true });
    });
  }
}

async function ensurePanelOpen(page, target) {
  if (!target.panelSelector) return;
  const panel = page.locator(target.panelSelector).first();
  if (await panel.isVisible().catch(() => false)) return;
  await openTarget(page, target);
}

async function auditElementStates(page, metricPage, viewport, context, locator, kind, label, baseStateName = "normal", options = {}) {
  const results = [];
  if (options.keepPanelOpen) {
    await resetPointer(page);
    await ensurePanelOpen(page, context);
  } else {
    await closeTransientPanels(page);
    await revealTarget(page, context);
  }
  await locator.scrollIntoViewIfNeeded().catch(() => {});
  await page.waitForTimeout(40);
  let baseBox = await visibleBox(locator);
  if (!baseBox) {
    return [{
      viewport: viewport.name,
      target: context.key,
      group: context.group,
      kind,
      label,
      state: baseStateName,
      ready: false,
      failures: ["missing_visible_box"],
    }];
  }
  if (options.keepPanelOpen) {
    await resetPointer(page, baseBox);
    await ensurePanelOpen(page, context);
    await locator.scrollIntoViewIfNeeded().catch(() => {});
    await page.waitForTimeout(40);
    baseBox = await visibleBox(locator);
  }
  const rect = paddedRect(baseBox, viewport, 8);
  const basePath = path.join(cropDir, `${sanitize(viewport.name)}-${sanitize(context.key)}-${sanitize(kind)}-${sanitize(label)}-${baseStateName}.png`);
  const baseMetrics = await cropFromPage(page, metricPage, rect, basePath);
  const baseSignature = await computedSignature(locator);
  let baseFailures = failuresFor({ kind, state: baseStateName, rawMetrics: baseMetrics, delta: null, signature_delta: false, focused: baseSignature.active_element_matches });
  results.push({
    viewport: viewport.name,
    target: context.key,
    group: context.group,
    kind,
    label,
    state: baseStateName,
    crop_path: basePath,
    metrics: normalizeMetrics(baseMetrics),
    computed: baseSignature,
    delta_from_base: null,
    failures: baseFailures,
    ready: baseFailures.length === 0,
  });

  const states = ["hover", "focus", "active"];
  for (const state of states) {
    await resetPointer(page);
    await revealTarget(page, context);
    if (options.keepPanelOpen) {
      await ensurePanelOpen(page, context);
    } else {
      await closeTransientPanels(page);
      await revealTarget(page, context);
    }
    await locator.scrollIntoViewIfNeeded().catch(() => {});
    if (state === "hover") {
      await locator.hover();
    } else if (state === "focus") {
      await focusLocator(locator);
    } else if (state === "active") {
      await locator.hover();
      const box = await visibleBox(locator);
      await page.mouse.move(box.left + (box.width / 2), box.top + (box.height / 2));
      await page.mouse.down();
    }
    await page.waitForTimeout(90);
    const statePath = path.join(cropDir, `${sanitize(viewport.name)}-${sanitize(context.key)}-${sanitize(kind)}-${sanitize(label)}-${state}.png`);
    const rawMetrics = await cropFromPage(page, metricPage, rect, statePath);
    const signature = await computedSignature(locator);
    const delta = await imageDelta(metricPage, basePath, statePath);
    const signatureDelta = signatureChanged(baseSignature, signature);
    const failures = failuresFor({
      kind,
      state,
      rawMetrics,
      delta,
      signature_delta: signatureDelta,
      focused: signature.active_element_matches,
    });
    results.push({
      viewport: viewport.name,
      target: context.key,
      group: context.group,
      kind,
      label,
      state,
      crop_path: statePath,
      metrics: normalizeMetrics(rawMetrics),
      computed: signature,
      delta_from_base: normalizeMetrics(delta),
      signature_delta: signatureDelta,
      failures,
      ready: failures.length === 0,
    });
    if (state === "active") {
      await page.mouse.up();
    }
  }
  await resetPointer(page);
  return results;
}

async function openTarget(page, target) {
  await revealTarget(page, target);
  const trigger = page.locator(target.triggerSelector).first();
  await trigger.waitFor({ state: "visible", timeout: 5000 });
  await trigger.click({ timeout: 5000 });
  await page.waitForTimeout(180);
  const panel = page.locator(target.panelSelector).first();
  await panel.waitFor({ state: "visible", timeout: 5000 });
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
  const allCrops = [];
  const targetSummaries = [];
  const metricPage = await browser.newPage({ viewport: { width: 16, height: 16 }, deviceScaleFactor: 1 });

  for (const viewport of viewports) {
    const page = await browser.newPage({ viewport: { width: viewport.width, height: viewport.height }, deviceScaleFactor: 1 });
    for (const target of targetDefinitions(viewport)) {
      await page.goto(baseUrl, { waitUntil: "networkidle" });
      await page.waitForTimeout(180);
      await revealTarget(page, target);
      const trigger = page.locator(target.triggerSelector).first();
      const triggerStateName = target.group === "row-menu" ? "revealed" : "normal";
      const triggerCrops = await auditElementStates(page, metricPage, viewport, target, trigger, "trigger", target.key, triggerStateName);
      allCrops.push(...triggerCrops);

      await page.goto(baseUrl, { waitUntil: "networkidle" });
      await page.waitForTimeout(180);
      await openTarget(page, target);
      const panelVisible = await page.locator(target.panelSelector).count()
        .then((count) => count > 0)
        .catch(() => false);
      const items = page.locator(target.itemSelector);
      const itemCount = await items.count();
      const itemLimitFailures = itemCount === target.expectedItemCount ? [] : [`expected_${target.expectedItemCount}_items_got_${itemCount}`];
      const itemCrops = [];
      for (let index = 0; index < itemCount; index += 1) {
        const item = items.nth(index);
        const label = await item.evaluate((element, index) => (
          element.getAttribute("data-chat-row-menu-item")
          || element.getAttribute("data-control-ui-menu-item")
          || element.getAttribute("data-control-ui-command-palette-item")
          || element.getAttribute("data-chat-composer-picker-item")
          || element.getAttribute("aria-label")
          || element.textContent
          || `item-${index}`
        ), index);
        const crops = await auditElementStates(page, metricPage, viewport, target, item, "menu-item", label || `item-${index}`, "normal", { keepPanelOpen: true });
        itemCrops.push(...crops);
        allCrops.push(...crops);
      }
      targetSummaries.push({
        viewport: viewport.name,
        target: target.key,
        group: target.group,
        panel_visible: panelVisible,
        expected_item_count: target.expectedItemCount,
        item_count: itemCount,
        item_count_failures: itemLimitFailures,
        trigger_crop_count: triggerCrops.length,
        item_crop_count: itemCrops.length,
        failure_count: [...triggerCrops, ...itemCrops].filter((item) => !item.ready).length + itemLimitFailures.length,
      });
    }
    await page.close();
  }

  await metricPage.close();
  await browser.close();

  const failures = allCrops.filter((item) => !item.ready);
  const targetFailures = targetSummaries.filter((item) => item.failure_count > 0 || item.panel_visible !== true);
  const byViewport = Object.values(allCrops.reduce((acc, item) => {
    acc[item.viewport] ||= { viewport: item.viewport, crop_count: 0, failure_count: 0 };
    acc[item.viewport].crop_count += 1;
    if (!item.ready) acc[item.viewport].failure_count += 1;
    return acc;
  }, {})).sort((a, b) => a.viewport.localeCompare(b.viewport));
  const byState = Object.values(allCrops.reduce((acc, item) => {
    acc[item.state] ||= { state: item.state, crop_count: 0, failure_count: 0 };
    acc[item.state].crop_count += 1;
    if (!item.ready) acc[item.state].failure_count += 1;
    return acc;
  }, {})).sort((a, b) => a.state.localeCompare(b.state));
  const byGroup = Object.values(allCrops.reduce((acc, item) => {
    acc[item.group] ||= { group: item.group, crop_count: 0, failure_count: 0 };
    acc[item.group].crop_count += 1;
    if (!item.ready) acc[item.group].failure_count += 1;
    return acc;
  }, {})).sort((a, b) => a.group.localeCompare(b.group));
  const triggerCount = allCrops.filter((item) => item.kind === "trigger").length;
  const menuItemCount = allCrops.filter((item) => item.kind === "menu-item").length;
  const expectedTargets = viewports.reduce((sum, viewport) => sum + targetDefinitions(viewport).length, 0);
  const report = {
    schema_version: "hepta-ui-harsh-top-design-referee-v12-interaction-state-crop-census/v0",
    standards_version: "2026-06-27-control-real-hover-focus-active-local-crop-glass-census",
    status: failures.length === 0 && targetFailures.length === 0 && targetSummaries.length === expectedTargets ? "ready" : "failed",
    base_url: baseUrl,
    crop_dir: cropDir,
    viewport_count: viewports.length,
    target_count: targetSummaries.length,
    expected_target_count: expectedTargets,
    crop_count: allCrops.length,
    trigger_crop_count: triggerCount,
    menu_item_crop_count: menuItemCount,
    failure_count: failures.length + targetFailures.length,
    by_viewport: byViewport,
    by_state: byState,
    by_group: byGroup,
    thresholds: {
      crop_min_width: 32,
      crop_min_height: 32,
      mean_luma: "204..252",
      luma_p95_min: 232,
      dark_ratio_max: 0.14,
      glass_white_ratio_min: 0.78,
      mean_saturation_max: 0.16,
      luma_stddev_min: 2.0,
      texture_delta_min: 0.12,
      interaction_delta_min: "mean_abs_luma_delta >= 0.20 or mean_abs_rgb_delta >= 0.20 or computed signature changed",
      active_trigger_delta_min: "mean_abs_luma_delta >= 0.35 or mean_abs_rgb_delta >= 0.35 or computed signature changed",
    },
    target_summaries: targetSummaries,
    target_failures: targetFailures,
    failures,
    crops: allCrops,
  };
  console.log(JSON.stringify(report, null, 2));
}

main().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
NODE

if [[ "$(jq -r '.status' "$INTERACTION_REPORT_PATH")" != "ready" ]]; then
  echo "v12 interaction-state crop census failed" >&2
  jq '.failures[:20], .target_failures[:20]' "$INTERACTION_REPORT_PATH" >&2 || true
  exit 1
fi

interaction_sha="$(shasum -a 256 "$INTERACTION_REPORT_PATH" | awk '{print $1}')"
v11_sha="$(shasum -a 256 "$V11_REPORT_PATH" | awk '{print $1}')"
native_sha="$(shasum -a 256 "$NATIVE_REPORT_PATH" | awk '{print $1}')"

jq -n \
  --arg v11_path "$V11_REPORT_PATH" \
  --arg native_path "$NATIVE_REPORT_PATH" \
  --arg interaction_path "$INTERACTION_REPORT_PATH" \
  --arg v11_sha "$v11_sha" \
  --arg native_sha "$native_sha" \
  --arg interaction_sha "$interaction_sha" \
  --slurpfile v11_file "$V11_REPORT_PATH" \
  --slurpfile interaction_file "$INTERACTION_REPORT_PATH" '
  ($v11_file[0]) as $v11
  | ($interaction_file[0]) as $interaction
  | def v11_ready:
      $v11.status == "ready"
      and $v11.v10_ready == true
      and $v11.control_component_crop_ready == true
      and $v11.summary.control_component_crop_census.failure_count == 0;
    def interaction_ready:
      $interaction.status == "ready"
      and $interaction.viewport_count == 4
      and $interaction.target_count == $interaction.expected_target_count
      and $interaction.crop_count >= 300
      and $interaction.trigger_crop_count >= 92
      and $interaction.menu_item_crop_count >= 240
      and $interaction.failure_count == 0
      and (($interaction.by_state // []) | any(.state == "hover" and .crop_count >= 80 and .failure_count == 0))
      and (($interaction.by_state // []) | any(.state == "focus" and .crop_count >= 80 and .failure_count == 0))
      and (($interaction.by_state // []) | any(.state == "active" and .crop_count >= 80 and .failure_count == 0));
    {
      schema_version:"hepta-ui-harsh-top-design-referee-v12-gate/v0",
      standards_version:"2026-06-27-harsh-v11-plus-real-hover-focus-active-local-crop-census",
      status:(if (v11_ready and interaction_ready) then "ready" else "failed" end),
      browser_path:"Browser plugin not available; regular Playwright with local Chrome was used",
      inputs:{
        v11_control_component_crop:{path:$v11_path, sha256:$v11_sha},
        fresh_native_fixture:{path:$native_path, sha256:$native_sha},
        control_interaction_state_crop_census:{path:$interaction_path, sha256:$interaction_sha}
      },
      summary:{
        control_visual_matrix:$v11.summary.control_visual_matrix,
        control_button_census:$v11.summary.control_button_census,
        native_fixture:$v11.summary.native_fixture,
        native_detail_census:$v11.summary.native_detail_census,
        pixel_glass_census:$v11.summary.pixel_glass_census,
        control_real_click_activation:$v11.summary.control_real_click_activation,
        control_submenu_lifecycle:$v11.summary.control_submenu_lifecycle,
        control_submenu_switching:$v11.summary.control_submenu_switching,
        local_crop_glass_census:$v11.summary.local_crop_glass_census,
        control_component_crop_census:$v11.summary.control_component_crop_census,
        control_interaction_state_crop_census:{
          viewport_count:$interaction.viewport_count,
          target_count:$interaction.target_count,
          crop_count:$interaction.crop_count,
          trigger_crop_count:$interaction.trigger_crop_count,
          menu_item_crop_count:$interaction.menu_item_crop_count,
          failure_count:$interaction.failure_count,
          by_viewport:$interaction.by_viewport,
          by_state:$interaction.by_state,
          by_group:$interaction.by_group,
          thresholds:$interaction.thresholds
        }
      },
      v11_ready:v11_ready,
      control_interaction_state_crop_ready:interaction_ready,
      control_interaction_state_crop_census:$interaction
    }
  ' >"$tmp_report"

cp "$tmp_report" "$REPORT_PATH"
rm -f "$tmp_report"
cat "$REPORT_PATH"

if [[ "$(jq -r '.status' "$REPORT_PATH")" != "ready" ]]; then
  echo "Hepta UI harsh top-design referee v12 interaction-state crop gate failed" >&2
  exit 1
fi

echo "$REPORT_PATH"
