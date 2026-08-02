#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

source scripts/lib/hepta-ui-rust-toolchain.sh
source scripts/lib/hepta-control-ui-runtime-fixture.sh
hepta_ui_activate_rust_toolchain

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V20_REPORT_PATH:-}"
V19_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V19_REPORT_PATH:-}"
V20_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V20_CENSUS_PATH:-}"
V20_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V20_SCREENSHOT_DIR:-}"
V20_CROP_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V20_CROP_DIR:-}"
NATIVE_DIR="${HEPTA_NATIVE_FIXTURE_VISUAL_DIR:-}"
V19_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V20_V19_LOG:-}"
SKIP_V19="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V20_SKIP_V19:-0}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v20-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v20-total-design-gate.json"
fi
if [[ -z "$V19_REPORT_PATH" ]]; then
  V19_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v19-menu-action-gate.json"
fi
if [[ -z "$V20_CENSUS_PATH" ]]; then
  V20_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v20-total-design-census.json"
fi
if [[ -z "$V20_SCREENSHOT_DIR" ]]; then
  V20_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v20-total-design-screenshots"
fi
if [[ -z "$V20_CROP_DIR" ]]; then
  V20_CROP_DIR="$READINESS_DIR/ui-harsh-v20-total-design-crops"
fi
if [[ -z "$NATIVE_DIR" ]]; then
  NATIVE_DIR="$READINESS_DIR/native-fixture"
fi
if [[ -z "$V19_LOG" ]]; then
  V19_LOG="$READINESS_DIR/v19-menu-action.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$NATIVE_DIR" "$V20_SCREENSHOT_DIR" "$V20_CROP_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V20_CENSUS_PATH")"

if [[ "$SKIP_V19" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V19_REPORT_PATH="$V19_REPORT_PATH" \
  HEPTA_NATIVE_FIXTURE_VISUAL_DIR="$NATIVE_DIR" \
    bash scripts/hepta-ui-harsh-top-design-referee-v19-menu-action-gate.sh "$READINESS_DIR" >"$V19_LOG" 2>&1 || {
      echo "v19 menu-action prerequisite failed" >&2
      tail -n 180 "$V19_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V19_REPORT_PATH")" != "ready" ]]; then
  echo "v19 menu-action prerequisite was not ready: $V19_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7470 7471 7472 7473 7474; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v20 referee" >&2
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
      echo "Hepta Control UI server exited before v20 total-design audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$V20_SCREENSHOT_DIR" "$V20_CROP_DIR" >"$V20_CENSUS_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir, cropDir] = process.argv.slice(2);
fs.mkdirSync(screenshotDir, { recursive: true });
fs.mkdirSync(cropDir, { recursive: true });

const scenarios = [
  { name: "desktop-total-design", viewport: { width: 1365, height: 900, dpr: 2, railVisible: true, isMobile: false, hasTouch: false } },
  { name: "narrow-touch-total-design", viewport: { width: 768, height: 900, dpr: 2, railVisible: true, isMobile: true, hasTouch: true } },
  { name: "mobile-total-design", viewport: { width: 500, height: 844, dpr: 2, railVisible: false, isMobile: true, hasTouch: true } },
  { name: "phone320-total-design", viewport: { width: 320, height: 700, dpr: 3, railVisible: false, isMobile: true, hasTouch: true } },
];

const moduleSelector = [
  ".tg-conversation-rail",
  ".tg-thread-panel",
  ".tg-room-panel",
  ".tg-chat-item",
  ".tg-thread-header",
  ".tg-message",
  ".tg-bubble",
  ".tg-compose-wrap",
  ".tg-compose-bar",
  ".tg-room-section",
  ".hepta-route-surface",
  ".command-palette",
  ".command-palette__item",
  "[data-control-ui-micro-surface]",
].join(",");

const controlSelector = [
  "button",
  "a[href]",
  "summary",
  "input",
  "select",
  "textarea",
  "[role='button']",
  "[role='menuitem']",
  "[data-control-ui-icon-button]",
  "[data-control-ui-row-menu-trigger]",
  "[data-open-command-palette]",
].join(",");

const transientPanelSelector = [
  "[data-chat-row-menu-panel]",
  "[data-control-ui-thread-tools-panel]",
  "[data-control-ui-composer-tools-panel]",
  "[data-chat-composer-popover]",
  "#command-palette .command-palette",
].join(",");

const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase() || "item";
const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const round = (value, digits = 3) => Number(value.toFixed(digits));
const below = (value, target) => value + 0.01 < target;

function targetDefinitions(profile) {
  const targets = [];
  if (profile.railVisible) {
    for (const key of ["ui-chat-agent", "task-queue", "operator-plane"]) {
      targets.push({
        key: `row-menu-${key}`,
        group: "row-menu",
        triggerSelector: `[data-chat-row-menu-toggle="${key}"]`,
        revealSelector: `[data-chat-conversation="${key}"]`,
        panelSelector: `[data-chat-row-menu-panel="${key}"]`,
        itemSelector: `[data-chat-row-menu-panel="${key}"] button[role="menuitem"]`,
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
    },
    {
      key: "composer-tools",
      group: "composer-tools",
      triggerSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tools-trigger="light-glass"]',
      panelSelector: '[data-control-ui-composer-tools-panel="light-glass"]',
      itemSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tool-item]',
    },
    {
      key: "composer-popover-artifact",
      group: "composer-popover",
      triggerSelector: '[data-chat-composer-popover-toggle="artifact"]',
      panelSelector: '[data-chat-composer-popover="artifact"]',
      itemSelector: '[data-chat-composer-popover="artifact"] .tg-composer-popover__item',
    },
    {
      key: "composer-popover-command",
      group: "composer-popover",
      triggerSelector: '[data-chat-composer-popover-toggle="command"]',
      panelSelector: '[data-chat-composer-popover="command"]',
      itemSelector: '[data-chat-composer-popover="command"] .tg-composer-popover__item',
    },
    {
      key: "command-palette",
      group: "command-palette",
      triggerSelector: '[data-control-ui-command-palette-trigger="light-glass"]',
      panelSelector: '#command-palette .command-palette',
      itemSelector: '#command-palette [data-control-ui-command-palette-item]',
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
  return { left, top, width: Math.max(1, right - left), height: Math.max(1, bottom - top) };
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
    ];
    const hits = points.filter((point) => {
      if (point.x < 0 || point.y < 0 || point.x > innerWidth || point.y > innerHeight) return false;
      const top = document.elementFromPoint(point.x, point.y);
      return top === element || element.contains(top);
    });
    return hits.length >= 2;
  }).catch(() => false);
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
    const step = Math.max(1, Math.ceil(Math.sqrt((canvas.width * canvas.height) / 6000)));
    const lumas = [];
    let highlightCount = 0;
    let darkCount = 0;
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
        if (luma >= 240) highlightCount += 1;
        if (luma <= 95) darkCount += 1;
        if (luma >= 180 && saturation <= 0.34) glassWhiteCount += 1;
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
  const isModule = kind === "module";
  const isMenuItem = kind === "menu-item";
  const minHeight = isMenuItem ? 32 : 44;
  const minMean = isModule ? 190 : 200;
  const minP95 = isModule ? 228 : 235;
  const maxDark = isModule ? 0.22 : 0.16;
  const minGlass = isModule ? 0.68 : 0.76;
  const maxSaturation = isModule ? 0.34 : 0.30;
  const minTexture = isModule ? 0.015 : 0.035;
  return [
    ...(metrics.bytes >= 900 ? [] : ["too_few_bytes"]),
    ...(metrics.width >= 44 && metrics.height >= minHeight ? [] : ["crop_too_small"]),
    ...(metrics.mean_luma >= minMean && metrics.mean_luma <= 254 ? [] : ["mean_luma_out_of_2026_light_glass_range"]),
    ...(metrics.luma_p95 >= minP95 ? [] : ["weak_bright_glass_highlights"]),
    ...(metrics.dark_ratio <= maxDark ? [] : ["too_much_dark_plate_area"]),
    ...(metrics.glass_white_ratio >= minGlass ? [] : ["insufficient_light_glass_area"]),
    ...(metrics.mean_saturation <= maxSaturation ? [] : ["oversaturated_for_tempered_glass"]),
    ...(metrics.texture_delta >= minTexture || metrics.luma_stddev >= 1.5 ? [] : ["flat_non_tempered_surface"]),
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
    clip: { x: rect.left, y: rect.top, width: rect.width, height: rect.height },
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

async function elementInfo(locator) {
  return locator.evaluate((element) => {
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    const visibleText = (element.textContent || "").replace(/\s+/g, " ").trim();
    const aria = element.getAttribute("aria-label") || "";
    const title = element.getAttribute("title") || "";
    const role = element.getAttribute("role") || element.tagName.toLowerCase();
    const accessibleName = (aria || title || visibleText || element.getAttribute("placeholder") || element.getAttribute("value") || "").trim();
    const hardClipped = element.scrollWidth > element.clientWidth + 2 || element.scrollHeight > element.clientHeight + 2;
    const isIconOnly = visibleText.length <= 1 || /^[+xX.\-_*#\s]*$/.test(visibleText);
    const hasGlass = (candidate) => {
      const candidateStyle = getComputedStyle(candidate);
      return !/rgba?\(0,\s*0,\s*0,\s*0\)/.test(candidateStyle.backgroundColor || "")
        || (candidateStyle.boxShadow && candidateStyle.boxShadow !== "none")
        || ((candidateStyle.backdropFilter || candidateStyle.webkitBackdropFilter || "none") !== "none")
        || Number.parseFloat(candidateStyle.borderTopWidth || "0") > 0;
    };
    let ancestorGlassTreatment = false;
    let parent = element.parentElement;
    for (let depth = 0; parent && depth < 4; depth += 1, parent = parent.parentElement) {
      if (hasGlass(parent)) {
        ancestorGlassTreatment = true;
        break;
      }
    }
    return {
      tag: element.tagName.toLowerCase(),
      role,
      class_name: element.className || "",
      id: element.id || "",
      text: visibleText.slice(0, 120),
      aria_label: aria,
      title,
      accessible_name: accessibleName,
      is_icon_only: isIconOnly,
      has_svg_or_img: Boolean(element.querySelector("svg,img")),
      ancestor_glass_treatment: ancestorGlassTreatment,
      style: {
        background_color: style.backgroundColor,
        border_top_color: style.borderTopColor,
        border_top_width: style.borderTopWidth,
        box_shadow: style.boxShadow,
        backdrop_filter: style.backdropFilter || style.webkitBackdropFilter || "none",
        color: style.color,
        border_radius: style.borderRadius,
        font_size: style.fontSize,
        font_weight: style.fontWeight,
        display: style.display,
        visibility: style.visibility,
        opacity: style.opacity,
      },
      hard_clipped: hardClipped,
      scroll: {
        scroll_width: element.scrollWidth,
        client_width: element.clientWidth,
        scroll_height: element.scrollHeight,
        client_height: element.clientHeight,
      },
      rect: {
        left: Number(rect.left.toFixed(2)),
        top: Number(rect.top.toFixed(2)),
        width: Number(rect.width.toFixed(2)),
        height: Number(rect.height.toFixed(2)),
      },
    };
  }).catch(() => null);
}

function hasGlassTreatment(info) {
  if (!info) return false;
  const style = info.style || {};
  return !/rgba?\(0,\s*0,\s*0,\s*0\)/.test(style.background_color || "")
    || (style.box_shadow && style.box_shadow !== "none")
    || (style.backdrop_filter && style.backdrop_filter !== "none")
    || Number.parseFloat(style.border_top_width || "0") > 0;
}

function isFormField(info) {
  if (!info) return false;
  return ["input", "textarea", "select"].includes(info.tag)
    || ["textbox", "searchbox", "combobox"].includes(info.role);
}

function hasEffectiveGlassTreatment(info) {
  if (!info) return false;
  return hasGlassTreatment(info) || (isFormField(info) && info.ancestor_glass_treatment);
}

function isPassiveMicroModule(kind, info, box) {
  if (kind !== "module" || !info || !box) return false;
  if (info.tag === "button" || info.role === "button" || info.role === "menuitem") return false;
  if (box.height > 32) return false;
  const className = String(info.class_name || "");
  return info.tag === "span"
    || className.includes("badge")
    || className.includes("pill")
    || className.includes("chip")
    || className.includes("micro");
}

function isMenuActionCoveredTrigger(kind, info) {
  if (kind !== "control" || !info) return false;
  const className = String(info.class_name || "");
  return className.includes("tg-row-menu-toggle");
}

async function screenshot(page, name) {
  const file = path.join(screenshotDir, `${sanitize(name)}.png`);
  await page.screenshot({ path: file, fullPage: false });
  return { path: file, sha256: sha256(file) };
}

async function visibleTransientPanels(page) {
  return page.evaluate((selector) => {
    return [...document.querySelectorAll(selector)]
      .map((element) => {
        const rect = element.getBoundingClientRect();
        const style = getComputedStyle(element);
        const visible = rect.width > 1 && rect.height > 1 && style.visibility !== "hidden" && style.display !== "none" && Number(style.opacity) > 0.01;
        return {
          visible,
          hint: element.getAttribute("data-chat-row-menu-panel")
            || element.getAttribute("data-chat-composer-popover")
            || element.getAttribute("data-control-ui-thread-tools-panel")
            || element.getAttribute("data-control-ui-composer-tools-panel")
            || element.getAttribute("data-control-ui-command-palette-surface")
            || element.id
            || String(element.className),
          box: {
            left: Number(rect.left.toFixed(2)),
            top: Number(rect.top.toFixed(2)),
            right: Number(rect.right.toFixed(2)),
            bottom: Number(rect.bottom.toFixed(2)),
            width: Number(rect.width.toFixed(2)),
            height: Number(rect.height.toFixed(2)),
          },
        };
      })
      .filter((panel) => panel.visible);
  }, transientPanelSelector);
}

async function openTarget(page, scenario, target) {
  if (target.revealSelector) {
    const reveal = page.locator(target.revealSelector).first();
    await reveal.scrollIntoViewIfNeeded().catch(() => {});
    if (!scenario.viewport.hasTouch) await reveal.hover({ force: true }).catch(() => {});
  }
  const trigger = page.locator(target.triggerSelector).first();
  await trigger.scrollIntoViewIfNeeded().catch(() => {});
  const triggerBox = await boxFor(trigger);
  if (scenario.viewport.hasTouch && triggerBox) {
    await page.touchscreen.tap(triggerBox.left + triggerBox.width / 2, triggerBox.top + triggerBox.height / 2);
  } else {
    await trigger.click({ force: true });
  }
  await page.waitForTimeout(260);
  return trigger;
}

async function auditLocator({ page, metricPage, scenario, locator, kind, label, minHeight = 44, requireTopmost = true }) {
  const info = await elementInfo(locator);
  const box = await boxFor(locator);
  const geometryFailures = [];
  if (!box) {
    geometryFailures.push("missing_box");
  } else {
    const passiveMicroModule = isPassiveMicroModule(kind, info, box);
    const minWidth = passiveMicroModule ? 24 : 44;
    const effectiveMinHeight = passiveMicroModule ? 20 : minHeight;
    if (below(box.width, minWidth) || below(box.height, effectiveMinHeight)) geometryFailures.push(`target_below_${minWidth}x${effectiveMinHeight}`);
    if (clippedRatio(box, scenario.viewport) < 0.985) geometryFailures.push("clipped_by_viewport");
    if (requireTopmost && !isMenuActionCoveredTrigger(kind, info) && !(await topmostFor(locator))) geometryFailures.push("not_topmost_at_sample_points");
  }
  const designFailures = [];
  if (kind === "control" || kind === "menu-item") {
    if (!info || !info.accessible_name) designFailures.push("missing_accessible_name");
    if (info && info.is_icon_only && !isFormField(info) && !info.has_svg_or_img) designFailures.push("icon_only_control_without_svg_or_image");
    if (info && info.hard_clipped && !["input", "textarea", "select"].includes(info.tag)) designFailures.push("hard_text_clipping");
  }
  if (info && !hasEffectiveGlassTreatment(info) && kind !== "module") designFailures.push("missing_glass_surface_treatment");
  const crop = box ? await cropElement(page, metricPage, scenario.viewport, locator, kind, `${sanitize(scenario.name)}-${sanitize(label)}`) : { failures: ["missing_box"], ready: false };
  const failures = [
    ...geometryFailures.map((failure) => `geometry:${failure}`),
    ...designFailures.map((failure) => `design:${failure}`),
    ...((crop.failures || []).map((failure) => `crop:${failure}`)),
  ];
  return {
    scenario: scenario.name,
    kind,
    label,
    box: roundedBox(box),
    clipped_ratio: box ? round(clippedRatio(box, scenario.viewport), 4) : 0,
    info,
    crop,
    failures,
    ready: failures.length === 0,
  };
}

async function auditVisibleSet(page, metricPage, scenario, selector, kind, minHeight) {
  const count = await page.locator(selector).count();
  const audits = [];
  for (let index = 0; index < count; index += 1) {
    const locator = page.locator(selector).nth(index);
    const box = await boxFor(locator);
    const visibleRatio = box ? clippedRatio(box, scenario.viewport) : 0;
    if (!box || visibleRatio < 0.65) continue;
    if (kind === "module" && visibleRatio < 0.985) continue;
    const info = await elementInfo(locator);
    if (!info || info.style.visibility === "hidden" || info.style.display === "none" || Number(info.style.opacity) <= 0.01) continue;
    const readable = info.accessible_name || info.text || info.id || info.class_name || `${kind}-${index}`;
    const label = `${kind}-${index}-${sanitize(readable.slice(0, 56))}`;
    audits.push(await auditLocator({ page, metricPage, scenario, locator, kind, label, minHeight, requireTopmost: kind === "control" }));
  }
  return audits;
}

async function auditMenuTarget(page, metricPage, scenario, target) {
  await page.goto(baseUrl, { waitUntil: "domcontentloaded" });
  await page.waitForTimeout(220);
  await openTarget(page, scenario, target);
  const panels = await visibleTransientPanels(page);
  const panel = page.locator(target.panelSelector).first();
  const panelAudit = await auditLocator({
    page,
    metricPage,
    scenario,
    locator: panel,
    kind: "module",
    label: `opened-panel-${target.key}`,
    minHeight: 44,
    requireTopmost: true,
  });
  const itemAudits = [];
  const itemCount = await page.locator(target.itemSelector).count();
  for (let index = 0; index < itemCount; index += 1) {
    const item = page.locator(target.itemSelector).nth(index);
    await item.scrollIntoViewIfNeeded().catch(() => {});
    const info = await elementInfo(item);
    const labelSource = info?.accessible_name || info?.text || `${target.key}-item-${index}`;
    itemAudits.push(await auditLocator({
      page,
      metricPage,
      scenario,
      locator: item,
      kind: "menu-item",
      label: `opened-item-${target.key}-${index}-${sanitize(labelSource.slice(0, 56))}`,
      minHeight: 32,
      requireTopmost: true,
    }));
  }
  await page.keyboard.press("Escape").catch(() => {});
  await page.waitForTimeout(180);
  const afterClosePanels = await visibleTransientPanels(page);
  const lifecycleFailures = [
    ...(panels.length === 1 ? [] : [`opened_visible_panel_count_${panels.length}`]),
    ...(itemCount > 0 ? [] : ["opened_menu_has_no_items"]),
    ...(afterClosePanels.length === 0 ? [] : [`escape_residual_panel_count_${afterClosePanels.length}`]),
  ];
  const failures = [
    ...lifecycleFailures,
    ...panelAudit.failures.map((failure) => `panel:${failure}`),
    ...itemAudits.flatMap((item) => item.failures.map((failure) => `item:${item.label}:${failure}`)),
  ];
  return {
    scenario: scenario.name,
    group: target.group,
    target: target.key,
    visible_panels_after_open: panels,
    visible_panels_after_escape: afterClosePanels,
    item_count: itemCount,
    panel: panelAudit,
    items: itemAudits,
    failures,
    ready: failures.length === 0,
  };
}

function summarizeBy(items, key, countKey = "audit_count") {
  return Object.values(items.reduce((acc, item) => {
    const value = item[key] || "unknown";
    acc[value] ||= { [key]: value, [countKey]: 0, failure_count: 0 };
    acc[value][countKey] += 1;
    if (!item.ready) acc[value].failure_count += 1;
    return acc;
  }, {})).sort((a, b) => String(a[key]).localeCompare(String(b[key])));
}

(async () => {
  const browser = await chromium.launch({
    executablePath: chromeBin,
    headless: true,
    args: [
      "--no-sandbox",
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
  const metricContext = await browser.newContext({ viewport: { width: 900, height: 900 }, deviceScaleFactor: 1, colorScheme: "light" });
  const metricPage = await metricContext.newPage();
  const baselineAudits = [];
  const menuAudits = [];
  const screenshots = [];
  try {
    for (const scenario of scenarios) {
      const context = await browser.newContext({
        viewport: { width: scenario.viewport.width, height: scenario.viewport.height },
        deviceScaleFactor: scenario.viewport.dpr,
        isMobile: scenario.viewport.isMobile,
        hasTouch: scenario.viewport.hasTouch,
        colorScheme: "light",
      });
      const page = await context.newPage();
      await page.goto(baseUrl, { waitUntil: "domcontentloaded" });
      await page.waitForTimeout(260);
      screenshots.push({ scenario: scenario.name, state: "default", ...(await screenshot(page, `${scenario.name}-default`)) });
      const moduleAudits = await auditVisibleSet(page, metricPage, scenario, moduleSelector, "module", 44);
      const controlAudits = await auditVisibleSet(page, metricPage, scenario, controlSelector, "control", 44);
      baselineAudits.push(...moduleAudits, ...controlAudits);
      for (const target of targetDefinitions(scenario.viewport)) {
        menuAudits.push(await auditMenuTarget(page, metricPage, scenario, target));
        screenshots.push({ scenario: scenario.name, state: `opened-${target.key}`, ...(await screenshot(page, `${scenario.name}-after-${target.key}`)) });
      }
      await context.close();
    }
  } finally {
    await metricContext.close();
    await browser.close();
  }

  const baselineFailures = baselineAudits.filter((audit) => !audit.ready);
  const menuFailures = menuAudits.filter((audit) => !audit.ready);
  const panelAudits = menuAudits.map((audit) => audit.panel);
  const menuItemAudits = menuAudits.flatMap((audit) => audit.items);
  const cropAudits = [
    ...baselineAudits.map((audit) => audit.crop),
    ...panelAudits.map((audit) => audit.crop),
    ...menuItemAudits.map((audit) => audit.crop),
  ].filter(Boolean);
  const cropFailures = cropAudits.filter((crop) => !crop.ready);
  const ready = baselineFailures.length === 0 && menuFailures.length === 0 && cropFailures.length === 0;

  console.log(JSON.stringify({
    schema_version: "hepta-ui-harsh-top-design-referee-v20-total-design-census/v0",
    standards_version: "2026-06-29-harsh-v19-plus-total-module-small-control-submenu-top-design-census",
    status: ready ? "ready" : "failed",
    browser_path: "Browser plugin not available; regular Playwright with local Chrome was used",
    flow_under_test: "Control UI loads -> every visible module/small control is audited -> every small trigger opens its submenu -> every opened panel/item remains light bright tempered glass with zero residual panels on Escape",
    summary: {
      scenario_count: scenarios.length,
      baseline_audit_count: baselineAudits.length,
      module_audit_count: baselineAudits.filter((audit) => audit.kind === "module").length,
      control_audit_count: baselineAudits.filter((audit) => audit.kind === "control").length,
      menu_target_count: menuAudits.length,
      opened_panel_audit_count: panelAudits.length,
      opened_menu_item_audit_count: menuItemAudits.length,
      crop_count: cropAudits.length,
      screenshot_count: screenshots.length,
      failure_count: baselineFailures.length + menuFailures.length + cropFailures.length,
      baseline_failure_count: baselineFailures.length,
      menu_failure_count: menuFailures.length,
      crop_failure_count: cropFailures.length,
      by_scenario: summarizeBy([...baselineAudits, ...menuAudits], "scenario"),
      baseline_by_kind: summarizeBy(baselineAudits, "kind"),
      menu_by_group: summarizeBy(menuAudits, "group", "menu_target_count"),
      thresholds: {
        control_min_size: "44x44",
        menu_item_min_size: "44x32",
        module_min_size: "44x44 crop, 120x44 practical surface target in prior v13-v19 gates",
        clipped_ratio_min: 0.985,
        topmost_sample_points: "center + diagonal inset",
        opened_visible_transient_panel_count: 1,
        escape_visible_transient_panel_count: 0,
        accessible_name_required_for_controls_and_menu_items: true,
        icon_only_controls_require_svg_or_image: true,
        hard_text_clipping_allowed: false,
        control_crop_mean_luma: "200..254",
        control_crop_luma_p95_min: 235,
        control_crop_dark_ratio_max: 0.16,
        control_crop_glass_white_ratio_min: 0.76,
        control_crop_mean_saturation_max: 0.30,
        module_crop_mean_luma_min: 190,
        module_crop_luma_p95_min: 228,
        module_crop_dark_ratio_max: 0.22,
        module_crop_glass_white_ratio_min: 0.68,
      },
    },
    failures: {
      baseline: baselineFailures.slice(0, 80),
      menus: menuFailures.slice(0, 80),
      crops: cropFailures.slice(0, 80),
    },
    screenshots,
    audits: {
      baseline: baselineAudits,
      menus: menuAudits,
    },
  }, null, 2));
})().catch((error) => {
  console.error(error);
  process.exit(1);
});
NODE

V20_STATUS="$(jq -r '.status' "$V20_CENSUS_PATH")"

jq -n \
  --arg v19_path "$V19_REPORT_PATH" \
  --arg v19_sha "$(shasum -a 256 "$V19_REPORT_PATH" | awk '{print $1}')" \
  --arg v20_path "$V20_CENSUS_PATH" \
  --arg v20_sha "$(shasum -a 256 "$V20_CENSUS_PATH" | awk '{print $1}')" \
  --slurpfile v19 "$V19_REPORT_PATH" \
  --slurpfile v20 "$V20_CENSUS_PATH" \
  '{
    schema_version: "hepta-ui-harsh-top-design-referee-v20-gate/v0",
    standards_version: "2026-06-29-harsh-v19-plus-total-module-small-control-submenu-top-design-census",
    status: (if $v20[0].status == "ready" then "ready" else "failed" end),
    browser_path: "Browser plugin not available; regular Playwright with local Chrome was used",
    inputs: {
      v19_menu_action: { path: $v19_path, sha256: $v19_sha },
      v20_total_design_census: { path: $v20_path, sha256: $v20_sha }
    },
    summary: {
      v19_menu_action: $v19[0].summary.v19_menu_action,
      v18_resize_orientation: $v19[0].summary.v18_resize_orientation,
      v17_touch_coarse_pointer: $v19[0].summary.v17_touch_coarse_pointer,
      v16_keyboard_focus: $v19[0].summary.v16_keyboard_focus,
      v15_text_zoom_squeeze: $v19[0].summary.v15_text_zoom_squeeze,
      v14_scroll_edge_crop: $v19[0].summary.v14_scroll_edge_crop,
      v13_geometry_occlusion: $v19[0].summary.v13_geometry_occlusion,
      v12_interaction_state_crop: $v19[0].summary.v12_interaction_state_crop,
      v20_total_design: $v20[0].summary
    },
    total_design_census: $v20[0]
  }' >"$REPORT_PATH"

cat "$REPORT_PATH"

if [[ "$V20_STATUS" != "ready" ]]; then
  echo "v20 total-design audit failed: $V20_CENSUS_PATH" >&2
  jq '.summary, .failures' "$V20_CENSUS_PATH" >&2 || true
  exit 1
fi
