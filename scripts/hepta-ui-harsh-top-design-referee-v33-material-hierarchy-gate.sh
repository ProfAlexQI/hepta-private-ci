#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V33_REPORT_PATH:-}"
V33_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V33_CENSUS_PATH:-}"
V33_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V33_SCREENSHOT_DIR:-}"
V32_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V32_REPORT_PATH:-}"
V32_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V33_V32_LOG:-}"
NATIVE_DIR="${HEPTA_NATIVE_FIXTURE_VISUAL_DIR:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
SKIP_V32="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V33_SKIP_V32:-0}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v33-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v33-material-hierarchy-gate.json"
fi
if [[ -z "$V33_CENSUS_PATH" ]]; then
  V33_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v33-material-hierarchy-census.json"
fi
if [[ -z "$V33_SCREENSHOT_DIR" ]]; then
  V33_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v33-material-hierarchy-screenshots"
fi
if [[ -z "$V32_REPORT_PATH" ]]; then
  V32_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v32-focus-containment-gate.json"
fi
if [[ -z "$V32_LOG" ]]; then
  V32_LOG="$READINESS_DIR/v32-focus-containment-prerequisite.log"
fi
if [[ -z "$NATIVE_DIR" ]]; then
  NATIVE_DIR="$READINESS_DIR/native-fixture"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$V33_SCREENSHOT_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V33_CENSUS_PATH")"

if [[ "$SKIP_V32" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V32_REPORT_PATH="$V32_REPORT_PATH" \
  HEPTA_NATIVE_FIXTURE_VISUAL_DIR="$NATIVE_DIR" \
    bash scripts/hepta-ui-harsh-top-design-referee-v32-focus-containment-gate.sh "$READINESS_DIR" >"$V32_LOG" 2>&1 || {
      echo "v32 focus-containment prerequisite failed" >&2
      tail -n 180 "$V32_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V32_REPORT_PATH")" != "ready" ]]; then
  echo "v32 focus-containment prerequisite was not ready: $V32_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7690 7691 7692 7693 7694; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v33 referee" >&2
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
      echo "Hepta Control UI server exited before v33 material hierarchy audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$V33_SCREENSHOT_DIR" "$READINESS_DIR" "$NATIVE_DIR" "$V32_REPORT_PATH" >"$V33_CENSUS_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir, readinessDir, nativeDir, v32ReportPath] = process.argv.slice(2);
fs.mkdirSync(screenshotDir, { recursive: true });

const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const round = (value, digits = 3) => Number(Number(value || 0).toFixed(digits));
const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase() || "item";

const paths = {
  v20Census: path.join(readinessDir, "ui-harsh-top-design-referee-v20-total-design-census.json"),
  v23Gate: path.join(readinessDir, "ui-harsh-top-design-referee-v23-evidence-glass-referee-gate.json"),
  v32Gate: v32ReportPath,
  nativeReport: path.join(nativeDir, "native-fixture-visual-smoke.json"),
};

const scenarios = [
  { name: "desktop-material-hierarchy", viewport: { width: 1365, height: 900, dpr: 2, railVisible: true, isMobile: false, hasTouch: false } },
  { name: "narrow-touch-material-hierarchy", viewport: { width: 768, height: 900, dpr: 2, railVisible: true, isMobile: true, hasTouch: true } },
  { name: "mobile-material-hierarchy", viewport: { width: 500, height: 844, dpr: 2, railVisible: false, isMobile: true, hasTouch: true } },
  { name: "phone320-material-hierarchy", viewport: { width: 320, height: 700, dpr: 3, railVisible: false, isMobile: true, hasTouch: true } },
];

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
      });
    }
  }
  targets.push(
    {
      key: "thread-tools",
      group: "thread-tools",
      triggerSelector: '[data-thread-command-menu="true"] [data-control-ui-thread-tools-trigger="light-glass"]',
      panelSelector: '[data-control-ui-thread-tools-panel="light-glass"]',
    },
    {
      key: "composer-tools",
      group: "composer-tools",
      triggerSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tools-trigger="light-glass"]',
      panelSelector: '[data-control-ui-composer-tools-panel="light-glass"]',
    },
    {
      key: "composer-popover-artifact",
      group: "composer-popover",
      triggerSelector: '[data-chat-composer-popover-toggle="artifact"]',
      panelSelector: '[data-chat-composer-popover="artifact"]',
    },
    {
      key: "composer-popover-command",
      group: "composer-popover",
      triggerSelector: '[data-chat-composer-popover-toggle="command"]',
      panelSelector: '[data-chat-composer-popover="command"]',
    },
    {
      key: "command-palette",
      group: "command-palette",
      triggerSelector: '[data-control-ui-command-palette-trigger="light-glass"]',
      panelSelector: '#command-palette .command-palette',
    },
  );
  return targets;
}

const materialSelector = [
  "[data-control-ui-icon-button]",
  "[data-chat-row-menu-toggle]",
  "[data-control-ui-thread-tools-trigger='light-glass']",
  "[data-control-ui-composer-tools-trigger='light-glass']",
  "[data-chat-composer-popover-toggle]",
  "[data-control-ui-command-palette-trigger='light-glass']",
  "[data-control-ui-command-palette-close='light-glass']",
  "[data-agent-chat-send]",
  "[data-agent-chat-plan]",
  "[data-control-ui-command-palette-input='light-glass']",
  "[data-control-ui-composer-popover-search='light-glass']",
  "[data-control-ui-rail-search-input='light-glass']",
  "[data-chat-composer-input]",
  ".tg-row-action",
  ".tg-menu-item",
  ".tg-composer-popover__item",
  ".command-palette__item",
  ".command-palette__kind",
  ".badge[data-control-ui-micro-surface]",
].join(",");

const panelSelector = [
  "[data-chat-row-menu-panel]",
  "[data-control-ui-thread-tools-panel]",
  "[data-control-ui-composer-tools-panel]",
  "[data-chat-composer-popover]",
  "#command-palette .command-palette",
  ".command-palette__results",
].join(",");

function parseColor(value) {
  const match = String(value || "").match(/rgba?\(([^)]+)\)/);
  if (!match) return { r: 0, g: 0, b: 0, a: 0, valid: false };
  const parts = match[1].split(",").map((part) => part.trim());
  const [r, g, b] = parts.slice(0, 3).map(Number);
  const a = parts[3] === undefined ? 1 : Number(parts[3]);
  return { r, g, b, a: Number.isFinite(a) ? a : 1, valid: true };
}

function luma(color) {
  return 0.2126 * color.r + 0.7152 * color.g + 0.0722 * color.b;
}

function contrastRatio(a, b) {
  const aRel = luma(a) / 255;
  const bRel = luma(b) / 255;
  const high = Math.max(aRel, bRel);
  const low = Math.min(aRel, bRel);
  return (high + 0.05) / (low + 0.05);
}

function clippedRatio(rect, width, height) {
  const left = Math.max(0, rect.left);
  const top = Math.max(0, rect.top);
  const right = Math.min(width, rect.right);
  const bottom = Math.min(height, rect.bottom);
  const clippedArea = Math.max(0, right - left) * Math.max(0, bottom - top);
  const area = Math.max(0, rect.width) * Math.max(0, rect.height);
  return area > 0 ? clippedArea / area : 0;
}

function summarizeBy(items, key) {
  return Object.values(items.reduce((acc, item) => {
    const value = item[key] || "unknown";
    acc[value] ||= { [key]: value, audit_count: 0, failure_count: 0 };
    acc[value].audit_count += 1;
    acc[value].failure_count += item.failures.length;
    return acc;
  }, {})).sort((a, b) => String(a[key]).localeCompare(String(b[key])));
}

async function gotoScenario(page, scenario) {
  await page.goto(baseUrl, { waitUntil: "domcontentloaded" });
  await page.waitForSelector(".telegram-chat-shell[data-control-ui-harsh-referee]", { timeout: 30000 });
  await page.waitForTimeout(250);
}

async function openTarget(page, scenario, target) {
  if (target.revealSelector) {
    const reveal = page.locator(target.revealSelector).first();
    await reveal.scrollIntoViewIfNeeded().catch(() => {});
    if (!scenario.viewport.hasTouch) await reveal.hover({ force: true }).catch(() => {});
  }
  const trigger = page.locator(target.triggerSelector).first();
  await trigger.scrollIntoViewIfNeeded().catch(() => {});
  const box = await trigger.boundingBox().catch(() => null);
  if (scenario.viewport.hasTouch && box) {
    await page.touchscreen.tap(box.x + box.width / 2, box.y + box.height / 2);
  } else {
    await trigger.click({ force: true });
  }
  await page.waitForTimeout(260);
}

async function screenshot(page, name) {
  const file = path.join(screenshotDir, `${sanitize(name)}.png`);
  await page.screenshot({ path: file, fullPage: false });
  return { path: file, sha256: sha256(file) };
}

async function auditMaterialState(page, scenario, state) {
  return page.evaluate(({ materialSelector, panelSelector, scenarioName, state }) => {
    const visible = (element) => {
      const rect = element.getBoundingClientRect();
      const style = getComputedStyle(element);
      if (rect.width <= 1 || rect.height <= 1 || style.display === "none" || style.visibility === "hidden" || Number(style.opacity || "1") <= 0.01) return false;
      const left = Math.max(0, rect.left);
      const top = Math.max(0, rect.top);
      const right = Math.min(innerWidth, rect.right);
      const bottom = Math.min(innerHeight, rect.bottom);
      const visibleArea = Math.max(0, right - left) * Math.max(0, bottom - top);
      const area = Math.max(1, rect.width * rect.height);
      if (visibleArea / area < 0.985) return false;
      const centerX = Math.min(innerWidth - 1, Math.max(1, rect.left + rect.width / 2));
      const centerY = Math.min(innerHeight - 1, Math.max(1, rect.top + rect.height / 2));
      const topElement = document.elementFromPoint(centerX, centerY);
      return Boolean(topElement && (topElement === element || element.contains(topElement)));
    };
    const rectFor = (element) => {
      const rect = element.getBoundingClientRect();
      return {
        left: Number(rect.left.toFixed(3)),
        top: Number(rect.top.toFixed(3)),
        right: Number(rect.right.toFixed(3)),
        bottom: Number(rect.bottom.toFixed(3)),
        width: Number(rect.width.toFixed(3)),
        height: Number(rect.height.toFixed(3)),
      };
    };
    const cssFor = (element) => {
      const style = getComputedStyle(element);
      return {
        background_color: style.backgroundColor,
        border_top_color: style.borderTopColor,
        border_top_width: style.borderTopWidth,
        color: style.color,
        box_shadow: style.boxShadow,
        backdrop_filter: style.backdropFilter || style.webkitBackdropFilter || "",
        border_radius: style.borderRadius,
        outline_style: style.outlineStyle,
        outline_width: style.outlineWidth,
        text_shadow: style.textShadow,
      };
    };
    const categoryFor = (element) => {
      if (element.matches(panelSelector)) return "panel";
      if (element.matches(".command-palette__kind,.badge[data-control-ui-micro-surface]")) return "micro-surface";
      if (element.matches(".tg-row-action,.tg-menu-item,.tg-composer-popover__item,.command-palette__item")) return "menu-item";
      if (element.matches("input,textarea")) return "input";
      return "control";
    };
    const infoFor = (element) => {
      const text = (element.textContent || "").replace(/\s+/g, " ").trim();
      const label = element.getAttribute("aria-label") || element.getAttribute("title") || text.slice(0, 80) || element.getAttribute("data-control-ui-icon-button") || element.tagName.toLowerCase();
      return {
        tag: element.tagName.toLowerCase(),
        class_name: String(element.className || ""),
        role: element.getAttribute("role") || "",
        label,
        text,
        aria_label: element.getAttribute("aria-label") || "",
        title: element.getAttribute("title") || "",
        has_icon: Boolean(element.querySelector("svg,img")),
        icon_only: Boolean(element.querySelector("svg,img")) && text.length <= 2,
      };
    };
    const records = [];
    for (const element of [...document.querySelectorAll(materialSelector), ...document.querySelectorAll(panelSelector)]) {
      if (!visible(element)) continue;
      records.push({
        scenario: scenarioName,
        state,
        category: categoryFor(element),
        rect: rectFor(element),
        css: cssFor(element),
        info: infoFor(element),
        viewport: { width: innerWidth, height: innerHeight, device_pixel_ratio: devicePixelRatio },
      });
    }
    return records;
  }, { materialSelector, panelSelector, scenarioName: scenario.name, state });
}

function materialFailures(record) {
  const failures = [];
  const rect = record.rect;
  const bg = parseColor(record.css.background_color);
  const border = parseColor(record.css.border_top_color);
  const fg = parseColor(record.css.color);
  const bgLuma = bg.valid ? luma(bg) : 0;
  const borderLuma = border.valid ? luma(border) : 0;
  const shadow = record.css.box_shadow && record.css.box_shadow !== "none";
  const insetShadow = shadow && record.css.box_shadow.includes("inset");
  const filter = record.css.backdrop_filter && record.css.backdrop_filter !== "none";
  const borderWidth = parseFloat(record.css.border_top_width || "0") || 0;
  const clip = clippedRatio(rect, record.viewport.width, record.viewport.height);
  const contrast = fg.valid && bg.valid ? contrastRatio(fg, bg) : 0;

  const minSize = {
    control: { width: 43.5, height: 43.5 },
    input: { width: 80, height: 39.5 },
    "menu-item": { width: 43.5, height: 31.5 },
    panel: { width: 120, height: 43.5 },
    "micro-surface": { width: 20, height: 18 },
  }[record.category] || { width: 43.5, height: 31.5 };

  if (rect.width < minSize.width || rect.height < minSize.height) failures.push("surface_below_v33_minimum_hit_or_read_size");
  if (clip < 0.985) failures.push("surface_clipped_by_viewport");
  if (bg.valid && bgLuma < 204 && record.category !== "micro-surface") failures.push("surface_not_bright_enough_for_light_glass");
  if (bg.valid && bg.a < 0.48 && !filter && record.category !== "micro-surface") failures.push("surface_too_transparent_without_blur");
  if (borderWidth < 0.75 || !border.valid || border.a < 0.12) failures.push("missing_visible_tempered_edge");
  if (border.valid && borderLuma < 185 && record.category !== "micro-surface") failures.push("edge_not_light_enough_for_bright_glass");
  if (!shadow && !filter && record.category !== "micro-surface") failures.push("missing_depth_or_backdrop_blur");
  if (!insetShadow && record.category !== "micro-surface" && record.category !== "input") failures.push("missing_inner_highlight_edge");
  if (contrast > 0 && contrast < 4.5 && !["micro-surface", "panel"].includes(record.category)) failures.push("text_contrast_below_wcag_on_glass");
  if (record.info.icon_only && (!record.info.has_icon || (!record.info.aria_label && !record.info.title))) failures.push("icon_only_control_missing_accessible_name");
  if (["control", "menu-item"].includes(record.category) && !record.info.icon_only && !record.info.text && !record.info.aria_label) failures.push("interactive_surface_missing_label");
  return {
    ...record,
    metrics: {
      bg_luma: round(bgLuma),
      bg_alpha: round(bg.a),
      border_luma: round(borderLuma),
      border_alpha: round(border.a),
      clipped_ratio: round(clip, 4),
      contrast_ratio: round(contrast, 2),
      has_shadow: Boolean(shadow),
      has_inset_shadow: Boolean(insetShadow),
      has_backdrop_filter: Boolean(filter),
    },
    failures,
    ready: failures.length === 0,
  };
}

async function imageMetrics(page, file) {
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
    const step = Math.max(1, Math.ceil(Math.sqrt((canvas.width * canvas.height) / 12000)));
    const lumas = [];
    let darkCount = 0;
    let glassWhiteCount = 0;
    let saturationSum = 0;
    for (let y = 0; y < canvas.height; y += step) {
      for (let x = 0; x < canvas.width; x += step) {
        const index = (y * canvas.width + x) * 4;
        const r = pixels[index];
        const g = pixels[index + 1];
        const b = pixels[index + 2];
        const lumaValue = (0.2126 * r) + (0.7152 * g) + (0.0722 * b);
        const max = Math.max(r, g, b);
        const min = Math.min(r, g, b);
        const saturation = max > 0 ? (max - min) / max : 0;
        lumas.push(lumaValue);
        saturationSum += saturation;
        if (lumaValue <= 95) darkCount += 1;
        if (lumaValue >= 180 && saturation <= 0.34) glassWhiteCount += 1;
      }
    }
    lumas.sort((a, b) => a - b);
    const sampleCount = lumas.length;
    const mean = lumas.reduce((sum, value) => sum + value, 0) / sampleCount;
    const variance = lumas.reduce((sum, value) => sum + ((value - mean) ** 2), 0) / sampleCount;
    return {
      width: canvas.width,
      height: canvas.height,
      sample_count: sampleCount,
      mean_luma: mean,
      luma_stddev: Math.sqrt(variance),
      luma_p05: lumas[Math.floor(sampleCount * 0.05)],
      luma_p95: lumas[Math.floor(sampleCount * 0.95)],
      dark_ratio: darkCount / sampleCount,
      glass_white_ratio: glassWhiteCount / sampleCount,
      mean_saturation: saturationSum / sampleCount,
    };
  }, { data });
  return Object.fromEntries(Object.entries({ ...metrics, bytes, sha256: sha256(file) }).map(([key, value]) => [
    key,
    typeof value === "number" ? round(value, key.endsWith("ratio") || key === "mean_saturation" ? 4 : 3) : value,
  ]));
}

function nativeScreenshotFailures(metrics) {
  const failures = [];
  if (metrics.width < 300 || metrics.height < 500) failures.push("native_screenshot_too_small");
  if (metrics.mean_luma < 208 || metrics.mean_luma > 240) failures.push("native_global_luma_outside_bright_glass_band");
  if (metrics.luma_p95 < 248) failures.push("native_missing_bright_tempered_highlights");
  if (metrics.dark_ratio > 0.025) failures.push("native_dark_area_too_high_for_light_surface");
  if (metrics.glass_white_ratio < 0.88) failures.push("native_light_glass_area_too_low");
  if (metrics.mean_saturation > 0.11) failures.push("native_palette_too_saturated_for_clear_glass");
  if (metrics.luma_stddev < 14) failures.push("native_surface_too_flat_for_tempered_glass");
  if (metrics.luma_stddev > 46) failures.push("native_background_too_noisy_for_readability");
  return failures;
}

(async () => {
  const missingInputs = Object.entries(paths)
    .filter(([, file]) => !fs.existsSync(file))
    .map(([key, file]) => ({ code: "missing_input", key, file }));
  if (missingInputs.length > 0) {
    process.stdout.write(JSON.stringify({
      schema: "hepta-ui-harsh-top-design-referee-v33-material-hierarchy-census/v1",
      status: "failed",
      generated_at: new Date().toISOString(),
      failures: missingInputs,
    }, null, 2));
    return;
  }

  const v20Census = readJson(paths.v20Census);
  const v23Gate = readJson(paths.v23Gate);
  const v32Gate = readJson(paths.v32Gate);
  const nativeReport = readJson(paths.nativeReport);

  const browser = await chromium.launch({
    executablePath: chromeBin,
    headless: true,
    args: ["--no-sandbox", "--disable-gpu", "--font-render-hinting=none"],
  });

  const materialAudits = [];
  const screenshotEvidence = [];
  const nativeScreenshotAudits = [];
  try {
    for (const scenario of scenarios) {
      const context = await browser.newContext({
        viewport: { width: scenario.viewport.width, height: scenario.viewport.height },
        deviceScaleFactor: scenario.viewport.dpr,
        isMobile: scenario.viewport.isMobile,
        hasTouch: scenario.viewport.hasTouch,
      });
      const page = await context.newPage();
      await gotoScenario(page, scenario);
      screenshotEvidence.push({ scenario: scenario.name, state: "default", ...(await screenshot(page, `${scenario.name}-default`)) });
      materialAudits.push(...(await auditMaterialState(page, scenario, "default")).map(materialFailures));

      for (const target of targetDefinitions(scenario.viewport)) {
        await gotoScenario(page, scenario);
        await openTarget(page, scenario, target);
        screenshotEvidence.push({ scenario: scenario.name, state: `opened-${target.key}`, ...(await screenshot(page, `${scenario.name}-opened-${target.key}`)) });
        materialAudits.push(...(await auditMaterialState(page, scenario, `opened-${target.group}`)).map(materialFailures));
      }
      await context.close();
    }

    const metricPage = await browser.newPage();
    const nativePngs = fs.readdirSync(nativeDir)
      .filter((name) => name.endsWith(".png"))
      .filter((name) => !name.startsWith("hepta-glass-") && !name.startsWith("hepta-tempered-glass-bg"))
      .sort()
      .map((name) => path.join(nativeDir, name));
    for (const file of nativePngs) {
      const metrics = await imageMetrics(metricPage, file);
      const failures = nativeScreenshotFailures(metrics);
      nativeScreenshotAudits.push({
        file,
        label: path.basename(file),
        metrics,
        failures,
        ready: failures.length === 0,
      });
    }
    await metricPage.close();
  } finally {
    await browser.close();
  }

  const materialFailuresAll = materialAudits.filter((audit) => audit.failures.length > 0);
  const nativeFailures = nativeScreenshotAudits.filter((audit) => audit.failures.length > 0);
  const prerequisiteFailures = [];
  if (v20Census.status !== "ready") prerequisiteFailures.push({ code: "v20_census_not_ready", status: v20Census.status });
  if (v23Gate.status !== "ready") prerequisiteFailures.push({ code: "v23_gate_not_ready", status: v23Gate.status });
  if (v32Gate.status !== "ready") prerequisiteFailures.push({ code: "v32_gate_not_ready", status: v32Gate.status });
  if (nativeReport.status !== "ready") prerequisiteFailures.push({ code: "native_fixture_not_ready", status: nativeReport.status });
  if ((nativeReport.screenshot_count || 0) < 40) prerequisiteFailures.push({ code: "native_fixture_screenshot_count_below_v33_minimum", observed: nativeReport.screenshot_count || 0, expected_minimum: 40 });
  if (nativeScreenshotAudits.length < 40) prerequisiteFailures.push({ code: "native_screenshot_audit_count_below_v33_minimum", observed: nativeScreenshotAudits.length, expected_minimum: 40 });

  const failureCount = materialFailuresAll.length + nativeFailures.length + prerequisiteFailures.length;
  const summary = {
    scenario_count: scenarios.length,
    material_surface_audit_count: materialAudits.length,
    material_failure_count: materialFailuresAll.length,
    material_by_category: summarizeBy(materialAudits, "category"),
    material_by_state: summarizeBy(materialAudits, "state"),
    screenshot_count: screenshotEvidence.length,
    native_screenshot_audit_count: nativeScreenshotAudits.length,
    native_screenshot_failure_count: nativeFailures.length,
    prerequisite_failure_count: prerequisiteFailures.length,
    failure_count: failureCount,
    thresholds: {
      control_min_size: "44x44",
      menu_item_min_size: "44x32",
      panel_min_size: "120x44",
      clipped_ratio_min: 0.985,
      non_micro_surface_bg_luma_min: 204,
      non_micro_surface_bg_alpha_min_unless_backdrop: 0.48,
      border_width_min: 0.75,
      edge_luma_min: 185,
      depth_requires_shadow_or_backdrop: true,
      non_input_surface_requires_inner_highlight_edge: true,
      text_contrast_min: 4.5,
      native_global_mean_luma: "208..240",
      native_luma_p95_min: 248,
      native_dark_ratio_max: 0.025,
      native_glass_white_ratio_min: 0.88,
      native_mean_saturation_max: 0.11,
      native_luma_stddev: "14..46",
      browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome was used.",
    },
  };

  process.stdout.write(JSON.stringify({
    schema: "hepta-ui-harsh-top-design-referee-v33-material-hierarchy-census/v1",
    status: failureCount === 0 ? "ready" : "failed",
    generated_at: new Date().toISOString(),
    base_url: baseUrl,
    screenshot_dir: screenshotDir,
    summary,
    inputs: {
      v20_census: { path: paths.v20Census, sha256: sha256(paths.v20Census) },
      v23_gate: { path: paths.v23Gate, sha256: sha256(paths.v23Gate) },
      v32_gate: { path: paths.v32Gate, sha256: sha256(paths.v32Gate) },
      native_fixture: { path: paths.nativeReport, sha256: sha256(paths.nativeReport) },
    },
    screenshot_evidence: screenshotEvidence,
    material_failures: materialFailuresAll.slice(0, 120),
    native_screenshot_failures: nativeFailures.slice(0, 80),
    prerequisite_failures: prerequisiteFailures,
    worst_native_screenshots: {
      lowest_mean_luma: [...nativeScreenshotAudits].sort((a, b) => a.metrics.mean_luma - b.metrics.mean_luma).slice(0, 8),
      highest_dark_ratio: [...nativeScreenshotAudits].sort((a, b) => b.metrics.dark_ratio - a.metrics.dark_ratio).slice(0, 8),
      lowest_glass_white_ratio: [...nativeScreenshotAudits].sort((a, b) => a.metrics.glass_white_ratio - b.metrics.glass_white_ratio).slice(0, 8),
      highest_luma_stddev: [...nativeScreenshotAudits].sort((a, b) => b.metrics.luma_stddev - a.metrics.luma_stddev).slice(0, 8),
    },
  }, null, 2));
})();
NODE

node - "$V33_CENSUS_PATH" "$REPORT_PATH" "$V32_REPORT_PATH" "$SKIP_V32" <<'NODE'
const crypto = require("node:crypto");
const fs = require("node:fs");

const [censusPath, reportPath, v32ReportPath, skipV32] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const sha256 = (file) => fs.existsSync(file) ? crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex") : null;
const census = readJson(censusPath);
const v32 = fs.existsSync(v32ReportPath) ? readJson(v32ReportPath) : null;
const status = census.status === "ready" && v32?.status === "ready" ? "ready" : "failed";
const report = {
  schema: "hepta-ui-harsh-top-design-referee-v33-material-hierarchy-gate/v1",
  status,
  generated_at: new Date().toISOString(),
  summary: {
    v32_focus_containment_referee: v32?.summary?.v32_focus_containment_referee ?? null,
    v33_material_hierarchy_referee: census.summary,
  },
  inputs: {
    v32_focus_containment: fs.existsSync(v32ReportPath) ? { path: v32ReportPath, sha256: sha256(v32ReportPath), skipped: skipV32 === "1" } : { path: v32ReportPath, sha256: null, skipped: skipV32 === "1" },
    v33_material_hierarchy_census: { path: censusPath, sha256: sha256(censusPath) },
  },
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`);
if (status !== "ready") {
  console.error(JSON.stringify(report.summary.v33_material_hierarchy_referee, null, 2));
  process.exit(1);
}
NODE

echo "Hepta UI harsh top-design referee v33 material hierarchy gate ready: $REPORT_PATH"
