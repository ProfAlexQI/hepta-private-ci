#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V34_REPORT_PATH:-}"
V34_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V34_CENSUS_PATH:-}"
V34_CROP_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V34_CROP_DIR:-}"
V34_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V34_SCREENSHOT_DIR:-}"
V33_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V33_REPORT_PATH:-}"
V33_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V34_V33_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
SKIP_V33="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V34_SKIP_V33:-0}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v34-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v34-optical-crop-gate.json"
fi
if [[ -z "$V34_CENSUS_PATH" ]]; then
  V34_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v34-optical-crop-census.json"
fi
if [[ -z "$V34_CROP_DIR" ]]; then
  V34_CROP_DIR="$READINESS_DIR/ui-harsh-v34-optical-crops"
fi
if [[ -z "$V34_SCREENSHOT_DIR" ]]; then
  V34_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v34-optical-screenshots"
fi
if [[ -z "$V33_REPORT_PATH" ]]; then
  V33_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v33-material-hierarchy-gate.json"
fi
if [[ -z "$V33_LOG" ]]; then
  V33_LOG="$READINESS_DIR/v33-material-hierarchy-prerequisite.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

for evidence_dir in "$V34_CROP_DIR" "$V34_SCREENSHOT_DIR"; do
  case "$evidence_dir" in
    "$READINESS_DIR"/*)
      rm -rf "$evidence_dir"
      ;;
    *)
      echo "refusing to clean v34 evidence dir outside readiness dir: $evidence_dir" >&2
      exit 1
      ;;
  esac
done

mkdir -p "$READINESS_DIR" "$V34_CROP_DIR" "$V34_SCREENSHOT_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V34_CENSUS_PATH")"

if [[ "$SKIP_V33" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V33_REPORT_PATH="$V33_REPORT_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v33-material-hierarchy-gate.sh "$READINESS_DIR" >"$V33_LOG" 2>&1 || {
      echo "v33 material-hierarchy prerequisite failed" >&2
      tail -n 180 "$V33_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V33_REPORT_PATH")" != "ready" ]]; then
  echo "v33 material-hierarchy prerequisite was not ready: $V33_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7700 7701 7702 7703 7704; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v34 referee" >&2
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
      echo "Hepta Control UI server exited before v34 optical crop audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$V34_CROP_DIR" "$V34_SCREENSHOT_DIR" "$READINESS_DIR" "$V33_REPORT_PATH" >"$V34_CENSUS_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, cropDir, screenshotDir, readinessDir, v33ReportPath] = process.argv.slice(2);
fs.mkdirSync(cropDir, { recursive: true });
fs.mkdirSync(screenshotDir, { recursive: true });

const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase() || "item";
const round = (value, digits = 3) => Number(Number(value || 0).toFixed(digits));

const scenarios = [
  { name: "desktop-optical-crop", viewport: { width: 1365, height: 900, dpr: 2, railVisible: true, isMobile: false, hasTouch: false } },
  { name: "narrow-touch-optical-crop", viewport: { width: 768, height: 900, dpr: 2, railVisible: true, isMobile: true, hasTouch: true } },
  { name: "mobile-optical-crop", viewport: { width: 500, height: 844, dpr: 2, railVisible: false, isMobile: true, hasTouch: true } },
  { name: "phone320-optical-crop", viewport: { width: 320, height: 700, dpr: 3, railVisible: false, isMobile: true, hasTouch: true } },
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
      });
    }
  }
  targets.push(
    { key: "thread-tools", group: "thread-tools", triggerSelector: '[data-thread-command-menu="true"] [data-control-ui-thread-tools-trigger="light-glass"]' },
    { key: "composer-tools", group: "composer-tools", triggerSelector: '[data-control-ui-composer-more] [data-control-ui-composer-tools-trigger="light-glass"]' },
    { key: "composer-popover-artifact", group: "composer-popover", triggerSelector: '[data-chat-composer-popover-toggle="artifact"]' },
    { key: "composer-popover-command", group: "composer-popover", triggerSelector: '[data-chat-composer-popover-toggle="command"]' },
    { key: "command-palette", group: "command-palette", triggerSelector: '[data-control-ui-command-palette-trigger="light-glass"]' },
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

async function gotoScenario(page) {
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

async function captureScreenshot(page, label) {
  const file = path.join(screenshotDir, `${sanitize(label)}.png`);
  await page.screenshot({ path: file, fullPage: false });
  return { path: file, sha256: sha256(file) };
}

async function auditSurfaceRecords(page, scenario, state) {
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
      return { label, text, tag: element.tagName.toLowerCase(), class_name: String(element.className || "") };
    };
    const nodes = [...new Set([...document.querySelectorAll(materialSelector), ...document.querySelectorAll(panelSelector)])];
    const records = [];
    const seen = new Set();
    for (const element of nodes) {
      if (!visible(element)) continue;
      const rect = element.getBoundingClientRect();
      const record = {
        scenario: scenarioName,
        state,
        category: categoryFor(element),
        rect: {
          left: Number(rect.left.toFixed(3)),
          top: Number(rect.top.toFixed(3)),
          right: Number(rect.right.toFixed(3)),
          bottom: Number(rect.bottom.toFixed(3)),
          width: Number(rect.width.toFixed(3)),
          height: Number(rect.height.toFixed(3)),
        },
        viewport: { width: innerWidth, height: innerHeight, device_pixel_ratio: devicePixelRatio },
        info: infoFor(element),
      };
      const key = `${record.category}:${record.info.label}:${record.rect.left}:${record.rect.top}:${record.rect.width}:${record.rect.height}`;
      if (seen.has(key)) continue;
      seen.add(key);
      records.push(record);
    }
    return records;
  }, { materialSelector, panelSelector, scenarioName: scenario.name, state });
}

async function cropMetrics(page, screenshotPath, record, outputFile) {
  const source = fs.readFileSync(screenshotPath).toString("base64");
  const data = await page.evaluate(async ({ source, record }) => {
    const image = new Image();
    image.src = `data:image/png;base64,${source}`;
    await image.decode();
    const scaleX = image.naturalWidth / record.viewport.width;
    const scaleY = image.naturalHeight / record.viewport.height;
    const left = Math.max(0, Math.floor(record.rect.left * scaleX));
    const top = Math.max(0, Math.floor(record.rect.top * scaleY));
    const width = Math.max(1, Math.min(image.naturalWidth - left, Math.ceil(record.rect.width * scaleX)));
    const height = Math.max(1, Math.min(image.naturalHeight - top, Math.ceil(record.rect.height * scaleY)));
    const canvas = document.createElement("canvas");
    canvas.width = width;
    canvas.height = height;
    const context = canvas.getContext("2d");
    context.drawImage(image, left, top, width, height, 0, 0, width, height);
    return canvas.toDataURL("image/png").split(",")[1];
  }, { source, record });
  fs.writeFileSync(outputFile, Buffer.from(data, "base64"));
  return metricsForPng(page, outputFile);
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
    const step = Math.max(1, Math.ceil(Math.sqrt((canvas.width * canvas.height) / 9000)));
    const band = Math.max(2, Math.min(8, Math.floor(Math.min(canvas.width, canvas.height) * 0.12)));
    const lumas = [];
    const edgeLumas = [];
    const centerLumas = [];
    let highlightCount = 0;
    let overbrightCount = 0;
    let darkCount = 0;
    let glassWhiteCount = 0;
    let saturationSum = 0;
    let textureSum = 0;
    let textureCount = 0;
    let edgeDarkCount = 0;
    let edgeHighlightCount = 0;

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
        const isEdge = x < band || y < band || x >= canvas.width - band || y >= canvas.height - band;
        lumas.push(luma);
        (isEdge ? edgeLumas : centerLumas).push(luma);
        saturationSum += saturation;
        if (luma >= 245) highlightCount += 1;
        if (luma >= 253) overbrightCount += 1;
        if (luma <= 92) darkCount += 1;
        if (luma >= 178 && saturation <= 0.30) glassWhiteCount += 1;
        if (isEdge) {
          if (luma <= 92) edgeDarkCount += 1;
          if (luma >= 238) edgeHighlightCount += 1;
        }
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
    const summarize = (values) => {
      const sorted = [...values].sort((a, b) => a - b);
      const count = sorted.length || 1;
      const mean = sorted.reduce((sum, value) => sum + value, 0) / count;
      const variance = sorted.reduce((sum, value) => sum + ((value - mean) ** 2), 0) / count;
      const percentile = (ratio) => sorted[Math.min(count - 1, Math.max(0, Math.floor(count * ratio)))] || 0;
      return { mean, stddev: Math.sqrt(variance), p05: percentile(0.05), p50: percentile(0.50), p95: percentile(0.95) };
    };
    const all = summarize(lumas);
    const edge = summarize(edgeLumas);
    const center = summarize(centerLumas.length ? centerLumas : lumas);
    const sampleCount = lumas.length;
    const edgeCount = edgeLumas.length || 1;
    return {
      width: canvas.width,
      height: canvas.height,
      sample_count: sampleCount,
      mean_luma: all.mean,
      luma_stddev: all.stddev,
      luma_p05: all.p05,
      luma_p50: all.p50,
      luma_p95: all.p95,
      highlight_ratio: highlightCount / sampleCount,
      overbright_ratio: overbrightCount / sampleCount,
      dark_ratio: darkCount / sampleCount,
      glass_white_ratio: glassWhiteCount / sampleCount,
      mean_saturation: saturationSum / sampleCount,
      texture_delta: textureSum / Math.max(1, textureCount),
      edge_mean_luma: edge.mean,
      edge_luma_p95: edge.p95,
      edge_dark_ratio: edgeDarkCount / edgeCount,
      edge_highlight_ratio: edgeHighlightCount / edgeCount,
      center_mean_luma: center.mean,
      edge_center_luma_delta: edge.mean - center.mean,
    };
  }, { data });
  return Object.fromEntries(Object.entries({ ...metrics, bytes, sha256: sha256(file) }).map(([key, value]) => [
    key,
    typeof value === "number" ? round(value, key.endsWith("ratio") || key === "mean_saturation" ? 4 : 3) : value,
  ]));
}

function opticalFailures(category, metrics) {
  const failures = [];
  const isMicro = category === "micro-surface";
  const isPanel = category === "panel";
  const minBytes = isMicro ? 500 : 800;
  const minSide = isMicro ? 16 : 24;
  if (metrics.bytes < minBytes) failures.push("crop_byte_weight_too_low_for_optical_proof");
  if (metrics.width < minSide || metrics.height < minSide) failures.push("crop_too_small_for_optical_proof");
  if (isMicro) {
    if (metrics.luma_p95 < 188) failures.push("micro_surface_missing_visible_highlight");
    if (metrics.dark_ratio > 0.42) failures.push("micro_surface_too_dark_or_ink_heavy");
    if (metrics.overbright_ratio > 0.78) failures.push("micro_surface_overexposed_flat_white");
    if (metrics.mean_saturation > 0.38) failures.push("micro_surface_palette_too_saturated");
    if (metrics.luma_stddev < 0.9) failures.push("micro_surface_optically_flat");
    return failures;
  }
  const meanMin = isPanel ? 200 : 196;
  const glassMin = isPanel ? 0.62 : 0.56;
  const darkMax = isPanel ? 0.18 : 0.24;
  const edgeP95Min = isPanel ? 226 : 218;
  if (metrics.mean_luma < meanMin || metrics.mean_luma > 253) failures.push("surface_mean_luma_outside_light_glass_band");
  if (metrics.luma_p95 < 232) failures.push("surface_missing_tempered_highlights");
  if (metrics.overbright_ratio > 0.74) failures.push("surface_overexposed_flat_white");
  if (metrics.dark_ratio > darkMax) failures.push("surface_dark_ink_or_shadow_area_too_high");
  if (metrics.glass_white_ratio < glassMin) failures.push("surface_light_glass_area_too_low");
  if (metrics.mean_saturation > 0.30) failures.push("surface_palette_too_saturated_for_clear_glass");
  if (metrics.luma_stddev < 1.5) failures.push("surface_optically_flat");
  if (metrics.texture_delta < 0.035) failures.push("surface_missing_subtle_tempered_texture");
  if (metrics.edge_luma_p95 < edgeP95Min) failures.push("surface_edge_highlight_too_weak");
  if (metrics.edge_dark_ratio > 0.28) failures.push("surface_edge_too_dark_or_dirty");
  if (metrics.edge_center_luma_delta < -22) failures.push("surface_edge_darker_than_body");
  return failures;
}

(async () => {
  const v33 = readJson(v33ReportPath);
  const browser = await chromium.launch({
    executablePath: chromeBin,
    headless: true,
    args: ["--no-sandbox", "--disable-gpu", "--font-render-hinting=none"],
  });

  const crops = [];
  const screenshotEvidence = [];
  try {
    for (const scenario of scenarios) {
      const context = await browser.newContext({
        viewport: { width: scenario.viewport.width, height: scenario.viewport.height },
        deviceScaleFactor: scenario.viewport.dpr,
        isMobile: scenario.viewport.isMobile,
        hasTouch: scenario.viewport.hasTouch,
      });
      const page = await context.newPage();
      const metricPage = await context.newPage();
      async function auditState(stateLabel) {
        const screenshot = await captureScreenshot(page, `${scenario.name}-${stateLabel}`);
        screenshotEvidence.push({ scenario: scenario.name, state: stateLabel, ...screenshot });
        const records = await auditSurfaceRecords(page, scenario, stateLabel);
        for (const [index, record] of records.entries()) {
          const cropPath = path.join(cropDir, `${sanitize(scenario.name)}-${sanitize(stateLabel)}-${sanitize(record.category)}-${index}-${sanitize(record.info.label)}.png`);
          const metrics = await cropMetrics(metricPage, screenshot.path, record, cropPath);
          const failures = opticalFailures(record.category, metrics);
          crops.push({
            scenario: scenario.name,
            state: stateLabel,
            category: record.category,
            label: record.info.label,
            rect: record.rect,
            crop_path: cropPath,
            source_screenshot: screenshot.path,
            metrics,
            failures,
            ready: failures.length === 0,
          });
        }
      }

      await gotoScenario(page);
      await auditState("default");
      for (const target of targetDefinitions(scenario.viewport)) {
        await gotoScenario(page);
        await openTarget(page, scenario, target);
        await auditState(`opened-${target.key}`);
      }
      await metricPage.close();
      await context.close();
    }
  } finally {
    await browser.close();
  }

  const failures = crops.filter((item) => item.failures.length > 0);
  const byCategory = Object.values(crops.reduce((acc, item) => {
    acc[item.category] ||= { category: item.category, crop_count: 0, failure_count: 0 };
    acc[item.category].crop_count += 1;
    if (item.failures.length > 0) acc[item.category].failure_count += 1;
    return acc;
  }, {})).sort((a, b) => a.category.localeCompare(b.category));
  const byState = Object.values(crops.reduce((acc, item) => {
    acc[item.state] ||= { state: item.state, crop_count: 0, failure_count: 0 };
    acc[item.state].crop_count += 1;
    if (item.failures.length > 0) acc[item.state].failure_count += 1;
    return acc;
  }, {})).sort((a, b) => a.state.localeCompare(b.state));
  const prerequisiteFailures = [];
  if (v33.status !== "ready") prerequisiteFailures.push({ code: "v33_gate_not_ready", status: v33.status });
  if (crops.length < 420) prerequisiteFailures.push({ code: "v34_surface_crop_count_below_minimum", observed: crops.length, expected_minimum: 420 });
  if (screenshotEvidence.length < 30) prerequisiteFailures.push({ code: "v34_screenshot_count_below_minimum", observed: screenshotEvidence.length, expected_minimum: 30 });
  const failureCount = failures.length + prerequisiteFailures.length;
  const summary = {
    scenario_count: scenarios.length,
    screenshot_count: screenshotEvidence.length,
    optical_crop_count: crops.length,
    optical_failure_count: failures.length,
    prerequisite_failure_count: prerequisiteFailures.length,
    failure_count: failureCount,
    by_category: byCategory,
    by_state: byState,
    thresholds: {
      crop_count_min: 420,
      screenshot_count_min: 30,
      non_micro_mean_luma_min: "panel 200, other 196",
      non_micro_mean_luma_max: 253,
      non_micro_luma_p95_min: 232,
      non_micro_overbright_ratio_max: 0.74,
      non_micro_dark_ratio_max: "panel 0.18, other 0.24",
      non_micro_glass_white_ratio_min: "panel 0.62, other 0.56",
      non_micro_mean_saturation_max: 0.30,
      non_micro_luma_stddev_min: 1.5,
      non_micro_texture_delta_min: 0.035,
      non_micro_edge_luma_p95_min: "panel 226, other 218",
      non_micro_edge_dark_ratio_max: 0.28,
      non_micro_edge_center_luma_delta_min: -22,
      micro_luma_p95_min: 188,
      micro_dark_ratio_max: 0.42,
      micro_overbright_ratio_max: 0.78,
      browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome was used.",
    },
  };

  process.stdout.write(JSON.stringify({
    schema: "hepta-ui-harsh-top-design-referee-v34-optical-crop-census/v1",
    status: failureCount === 0 ? "ready" : "failed",
    generated_at: new Date().toISOString(),
    base_url: baseUrl,
    crop_dir: cropDir,
    screenshot_dir: screenshotDir,
    summary,
    inputs: {
      v33_material_hierarchy: { path: v33ReportPath, sha256: sha256(v33ReportPath) },
    },
    screenshot_evidence: screenshotEvidence,
    failures: failures.slice(0, 160),
    prerequisite_failures: prerequisiteFailures,
    worst_crops: {
      lowest_luma_p95: [...crops].sort((a, b) => a.metrics.luma_p95 - b.metrics.luma_p95).slice(0, 10),
      highest_dark_ratio: [...crops].sort((a, b) => b.metrics.dark_ratio - a.metrics.dark_ratio).slice(0, 10),
      lowest_glass_white_ratio: [...crops].sort((a, b) => a.metrics.glass_white_ratio - b.metrics.glass_white_ratio).slice(0, 10),
      weakest_edge_highlight: [...crops].filter((item) => item.category !== "micro-surface").sort((a, b) => a.metrics.edge_luma_p95 - b.metrics.edge_luma_p95).slice(0, 10),
    },
  }, null, 2));
})();
NODE

node - "$V34_CENSUS_PATH" "$REPORT_PATH" "$V33_REPORT_PATH" "$SKIP_V33" <<'NODE'
const crypto = require("node:crypto");
const fs = require("node:fs");

const [censusPath, reportPath, v33ReportPath, skipV33] = process.argv.slice(2);
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const sha256 = (file) => fs.existsSync(file) ? crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex") : null;
const census = readJson(censusPath);
const v33 = fs.existsSync(v33ReportPath) ? readJson(v33ReportPath) : null;
const status = census.status === "ready" && v33?.status === "ready" ? "ready" : "failed";
const report = {
  schema: "hepta-ui-harsh-top-design-referee-v34-optical-crop-gate/v1",
  status,
  generated_at: new Date().toISOString(),
  summary: {
    v33_material_hierarchy_referee: v33?.summary?.v33_material_hierarchy_referee ?? null,
    v34_optical_crop_referee: census.summary,
  },
  inputs: {
    v33_material_hierarchy: fs.existsSync(v33ReportPath) ? { path: v33ReportPath, sha256: sha256(v33ReportPath), skipped: skipV33 === "1" } : { path: v33ReportPath, sha256: null, skipped: skipV33 === "1" },
    v34_optical_crop_census: { path: censusPath, sha256: sha256(censusPath) },
  },
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`);
if (status !== "ready") {
  console.error(JSON.stringify(report.summary.v34_optical_crop_referee, null, 2));
  const failures = census.failures || [];
  if (failures.length > 0) console.error(JSON.stringify(failures.slice(0, 24), null, 2));
  process.exit(1);
}
NODE

echo "Hepta UI harsh top-design referee v34 optical crop gate ready: $REPORT_PATH"
