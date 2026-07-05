#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V21_REPORT_PATH:-}"
V21_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V21_CENSUS_PATH:-}"
V21_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V21_SCREENSHOT_DIR:-}"
V20_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V20_REPORT_PATH:-}"
V20_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V21_V20_LOG:-}"
NATIVE_DIR="${HEPTA_NATIVE_FIXTURE_VISUAL_DIR:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v21-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"
SKIP_V20="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V21_SKIP_V20:-0}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v21-readable-default-gate.json"
fi
if [[ -z "$V21_CENSUS_PATH" ]]; then
  V21_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v21-readable-default-census.json"
fi
if [[ -z "$V21_SCREENSHOT_DIR" ]]; then
  V21_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v21-readable-default-screenshots"
fi
if [[ -z "$V20_REPORT_PATH" ]]; then
  V20_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v20-total-design-gate.json"
fi
if [[ -z "$V20_LOG" ]]; then
  V20_LOG="$READINESS_DIR/v20-total-design-prerequisite.log"
fi
if [[ -z "$NATIVE_DIR" ]]; then
  NATIVE_DIR="$READINESS_DIR/native-fixture"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$NATIVE_DIR" "$V21_SCREENSHOT_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V21_CENSUS_PATH")"

if [[ "$SKIP_V20" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V20_REPORT_PATH="$V20_REPORT_PATH" \
  HEPTA_NATIVE_FIXTURE_VISUAL_DIR="$NATIVE_DIR" \
    bash scripts/hepta-ui-harsh-top-design-referee-v20-total-design-gate.sh "$READINESS_DIR" >"$V20_LOG" 2>&1 || {
      echo "v20 total-design prerequisite failed" >&2
      tail -n 180 "$V20_LOG" >&2 || true
      exit 1
    }

  if [[ "$(jq -r '.status' "$V20_REPORT_PATH")" != "ready" ]]; then
    echo "v20 total-design prerequisite was not ready: $V20_REPORT_PATH" >&2
    exit 1
  fi
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7480 7481 7482 7483 7484; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v21 referee" >&2
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
      echo "Hepta Control UI server exited before v21 readable-default audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$V21_SCREENSHOT_DIR" >"$V21_CENSUS_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir] = process.argv.slice(2);
fs.mkdirSync(screenshotDir, { recursive: true });

const scenarios = [
  { name: "desktop-readable-default", viewport: { width: 1365, height: 900, dpr: 2, isMobile: false, hasTouch: false } },
  { name: "narrow-touch-readable-default", viewport: { width: 768, height: 900, dpr: 2, isMobile: true, hasTouch: true } },
  { name: "mobile-readable-default", viewport: { width: 500, height: 844, dpr: 2, isMobile: true, hasTouch: true } },
  { name: "phone320-readable-default", viewport: { width: 320, height: 700, dpr: 3, isMobile: true, hasTouch: true } },
];

const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const round = (value) => Number(value.toFixed(3));

function summarizeByScenario(audits) {
  return audits.map((audit) => ({
    scenario: audit.scenario,
    visible_message_count: audit.visible_message_count,
    readable_text_node_count: audit.readable_text_node_count,
    readable_text_rect_count: audit.readable_text_rect_count,
    screenshot_count: audit.screenshot_path ? 1 : 0,
    failure_count: audit.failures.length,
  }));
}

async function auditScenario(page, scenario) {
  const screenshotPath = path.join(screenshotDir, `${scenario.name}.png`);
  await page.goto(baseUrl, { waitUntil: "domcontentloaded" });
  await page.waitForSelector(".telegram-chat-shell[data-control-ui-harsh-referee]", { timeout: 30000 });
  await page.waitForTimeout(350);
  await page.screenshot({ path: screenshotPath, fullPage: false });

  const audit = await page.evaluate(() => {
    const failures = [];
    const rectFor = (element) => {
      if (!element) return null;
      const rect = element.getBoundingClientRect();
      return {
        left: rect.left,
        top: rect.top,
        right: rect.right,
        bottom: rect.bottom,
        width: rect.width,
        height: rect.height,
      };
    };
    const roundedRect = (rect) => rect && Object.fromEntries(Object.entries(rect).map(([key, value]) => [key, Number(value.toFixed(2))]));
    const intersect = (a, b) => {
      if (!a || !b) return { width: 0, height: 0, area: 0 };
      const left = Math.max(a.left, b.left);
      const right = Math.min(a.right, b.right);
      const top = Math.max(a.top, b.top);
      const bottom = Math.min(a.bottom, b.bottom);
      const width = Math.max(0, right - left);
      const height = Math.max(0, bottom - top);
      return { left, right, top, bottom, width, height, area: width * height };
    };
    const viewport = { left: 0, top: 0, right: innerWidth, bottom: innerHeight, width: innerWidth, height: innerHeight };
    const isVisible = (element) => {
      const style = getComputedStyle(element);
      const rect = rectFor(element);
      return rect && rect.width > 0 && rect.height > 0 && style.visibility !== "hidden" && style.display !== "none" && Number(style.opacity || "1") > 0.01;
    };
    const thread = document.querySelector(".tg-thread");
    const threadPanel = document.querySelector(".tg-thread-panel");
    const compose = document.querySelector(".tg-compose-wrap");
    const composeBar = document.querySelector(".tg-compose-bar");
    const footer = document.querySelector(".tg-compose-footer");
    const threadRect = rectFor(thread);
    const panelRect = rectFor(threadPanel);
    const composeRect = rectFor(compose);
    const composeBarRect = rectFor(composeBar);
    const footerRect = rectFor(footer);
    const threadComposeOverlap = intersect(threadRect, composeRect);
    const readableBottom = Math.min(
      threadRect ? threadRect.bottom : innerHeight,
      composeRect ? composeRect.top - 8 : innerHeight - 8,
      innerHeight - 8,
    );

    if (!threadRect || threadRect.width < 240 || threadRect.height < 180) {
      failures.push({ code: "thread_readable_region_missing_or_too_small", rect: roundedRect(threadRect) });
    }
    if (!composeRect || composeRect.width < 240 || composeRect.height < 44) {
      failures.push({ code: "composer_region_missing_or_too_small", rect: roundedRect(composeRect) });
    }
    if (threadComposeOverlap.area > 2) {
      failures.push({ code: "thread_and_composer_overlap", overlap: roundedRect(threadComposeOverlap), thread_rect: roundedRect(threadRect), compose_rect: roundedRect(composeRect) });
    }
    if (threadRect && composeRect && threadRect.bottom > composeRect.top - 1) {
      failures.push({ code: "thread_not_above_composer_safe_gap", thread_rect: roundedRect(threadRect), compose_rect: roundedRect(composeRect) });
    }
    if (composeBarRect && composeBarRect.height > 78 && innerWidth <= 500) {
      failures.push({ code: "mobile_composer_bar_too_tall", compose_bar_rect: roundedRect(composeBarRect) });
    }
    if (footerRect && footerRect.height > 56 && innerWidth <= 500) {
      failures.push({ code: "mobile_composer_footer_too_tall", footer_rect: roundedRect(footerRect) });
    }
    if (composeRect && composeRect.bottom > innerHeight + 1) {
      failures.push({ code: "composer_clipped_by_viewport", compose_rect: roundedRect(composeRect), viewport_height: innerHeight });
    }
    if (document.documentElement.scrollWidth > innerWidth + 1) {
      failures.push({ code: "horizontal_page_overflow", scroll_width: document.documentElement.scrollWidth, viewport_width: innerWidth });
    }

    const messageAudits = [...document.querySelectorAll(".tg-thread .tg-message")].filter(isVisible).map((message, index) => {
      const rect = rectFor(message);
      const viewportHit = intersect(rect, viewport);
      const composerHit = intersect(rect, composeRect);
      const crossesBoundary = rect && rect.top < readableBottom && rect.bottom > readableBottom + 2;
      if (crossesBoundary && viewportHit.area > 0) {
        failures.push({
          code: "visible_message_card_crosses_composer_boundary",
          index,
          text: (message.textContent || "").replace(/\s+/g, " ").trim().slice(0, 120),
          rect: roundedRect(rect),
          readable_bottom: Number(readableBottom.toFixed(2)),
          composer_intersection: roundedRect(composerHit),
        });
      }
      return { index, rect: roundedRect(rect), viewport_area: Number(viewportHit.area.toFixed(2)), composer_area: Number(composerHit.area.toFixed(2)) };
    });

    const textAudits = [];
    const walker = document.createTreeWalker(thread || document.body, NodeFilter.SHOW_TEXT, {
      acceptNode(node) {
        const text = (node.nodeValue || "").replace(/\s+/g, " ").trim();
        if (!text) return NodeFilter.FILTER_REJECT;
        const parent = node.parentElement;
        if (!parent || !parent.closest(".tg-message,.tg-thread-header,.tg-date-divider")) return NodeFilter.FILTER_REJECT;
        if (!isVisible(parent)) return NodeFilter.FILTER_REJECT;
        return NodeFilter.FILTER_ACCEPT;
      },
    });
    let node;
    let textIndex = 0;
    while ((node = walker.nextNode())) {
      const parent = node.parentElement;
      const range = document.createRange();
      range.selectNodeContents(node);
      const rects = [...range.getClientRects()].filter((rect) => rect.width >= 2 && rect.height >= 6);
      const text = (node.nodeValue || "").replace(/\s+/g, " ").trim();
      for (const rect of rects) {
        const textRect = { left: rect.left, top: rect.top, right: rect.right, bottom: rect.bottom, width: rect.width, height: rect.height };
        const viewportHit = intersect(textRect, viewport);
        const composerHit = intersect(textRect, composeRect);
        const crossesBoundary = textRect.top < readableBottom && textRect.bottom > readableBottom + 1;
        if (viewportHit.area > 0) {
          textAudits.push({
            index: textIndex,
            text: text.slice(0, 100),
            rect: roundedRect(textRect),
            viewport_area: Number(viewportHit.area.toFixed(2)),
            composer_area: Number(composerHit.area.toFixed(2)),
          });
        }
        if (crossesBoundary && viewportHit.area > 0) {
          const x = Math.min(innerWidth - 2, Math.max(2, textRect.left + Math.min(textRect.width - 1, Math.max(1, textRect.width / 2))));
          const y = Math.min(innerHeight - 2, Math.max(2, readableBottom - 2));
          const topElement = document.elementFromPoint(x, y);
          const visibleAtBoundary = topElement === parent || parent.contains(topElement) || (thread && thread.contains(topElement) && !compose?.contains(topElement));
          failures.push({
            code: "text_line_crosses_composer_boundary",
            index: textIndex,
            text: text.slice(0, 120),
            rect: roundedRect(textRect),
            readable_bottom: Number(readableBottom.toFixed(2)),
            top_element_at_boundary: topElement ? `${topElement.tagName.toLowerCase()}#${topElement.id || ""}.${String(topElement.className || "").replace(/\s+/g, ".").slice(0, 80)}` : null,
            visible_at_boundary: Boolean(visibleAtBoundary),
          });
        }
      }
      range.detach();
      textIndex += 1;
    }

    const mobileControls = [...document.querySelectorAll(".tg-compose-wrap button,.tg-compose-wrap summary,[data-control-ui-command-palette-trigger='light-glass'],[data-control-ui-thread-tools-trigger='light-glass']")]
      .filter(isVisible)
      .map((element, index) => {
        const rect = rectFor(element);
        if (innerWidth <= 500 && (rect.width < 43.5 || rect.height < 43.5)) {
          failures.push({ code: "mobile_default_control_below_44px", index, label: element.getAttribute("aria-label") || element.textContent?.trim() || element.tagName.toLowerCase(), rect: roundedRect(rect) });
        }
        return { index, rect: roundedRect(rect) };
      });

    return {
      viewport: { width: innerWidth, height: innerHeight, device_pixel_ratio: devicePixelRatio },
      thread_rect: roundedRect(threadRect),
      thread_panel_rect: roundedRect(panelRect),
      compose_rect: roundedRect(composeRect),
      compose_bar_rect: roundedRect(composeBarRect),
      footer_rect: roundedRect(footerRect),
      readable_bottom: Number(readableBottom.toFixed(2)),
      thread_compose_overlap_area: Number(threadComposeOverlap.area.toFixed(2)),
      visible_message_count: messageAudits.length,
      readable_text_node_count: textIndex,
      readable_text_rect_count: textAudits.length,
      mobile_default_control_count: mobileControls.length,
      message_audits: messageAudits,
      text_audits: textAudits,
      failures,
    };
  });

  return {
    scenario: scenario.name,
    viewport: scenario.viewport,
    screenshot_path: screenshotPath,
    screenshot_sha256: sha256(screenshotPath),
    ...audit,
  };
}

(async () => {
  const browser = await chromium.launch({
    executablePath: chromeBin,
    headless: true,
    args: ["--disable-gpu", "--no-sandbox", "--font-render-hinting=none"],
  });
  const audits = [];
  try {
    for (const scenario of scenarios) {
      const context = await browser.newContext({
        viewport: { width: scenario.viewport.width, height: scenario.viewport.height },
        deviceScaleFactor: scenario.viewport.dpr,
        isMobile: scenario.viewport.isMobile,
        hasTouch: scenario.viewport.hasTouch,
      });
      const page = await context.newPage();
      audits.push(await auditScenario(page, scenario));
      await context.close();
    }
  } finally {
    await browser.close();
  }

  const failureCount = audits.reduce((sum, audit) => sum + audit.failures.length, 0);
  const summary = {
    scenario_count: audits.length,
    visible_message_count: audits.reduce((sum, audit) => sum + audit.visible_message_count, 0),
    readable_text_node_count: audits.reduce((sum, audit) => sum + audit.readable_text_node_count, 0),
    readable_text_rect_count: audits.reduce((sum, audit) => sum + audit.readable_text_rect_count, 0),
    screenshot_count: audits.filter((audit) => audit.screenshot_path).length,
    failure_count: failureCount,
    by_scenario: summarizeByScenario(audits),
    thresholds: {
      default_thread_composer_overlap_area_max: 2,
      default_thread_bottom_must_be_above_composer: true,
      visible_message_cards_may_not_cross_composer_boundary: true,
      readable_text_lines_may_not_cross_composer_boundary: true,
      mobile_default_control_min_size: "44x44",
      mobile_composer_bar_height_max: 78,
      mobile_composer_footer_height_max: 56,
      composer_bottom_must_not_exceed_viewport: true,
      horizontal_overflow_px_max: 1,
      browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome was used.",
    },
  };

  process.stdout.write(JSON.stringify({
    schema: "hepta-ui-harsh-top-design-referee-v21-readable-default-census/v1",
    status: failureCount === 0 ? "ready" : "failed",
    generated_at: new Date().toISOString(),
    base_url: baseUrl,
    screenshot_dir: screenshotDir,
    summary,
    audits,
  }, null, 2));
})();
NODE

node - "$V21_CENSUS_PATH" "$REPORT_PATH" "$V20_REPORT_PATH" "$SKIP_V20" <<'NODE'
const crypto = require("node:crypto");
const fs = require("node:fs");

const [censusPath, reportPath, v20ReportPath, skipV20] = process.argv.slice(2);
const sha256 = (file) => fs.existsSync(file) ? crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex") : null;
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const census = readJson(censusPath);
const v20 = fs.existsSync(v20ReportPath) ? readJson(v20ReportPath) : null;
const failureCount = census.summary?.failure_count ?? 1;
const status = failureCount === 0 && (skipV20 === "1" || v20?.status === "ready") ? "ready" : "failed";
const report = {
  schema: "hepta-ui-harsh-top-design-referee-v21-readable-default-gate/v1",
  status,
  generated_at: new Date().toISOString(),
  summary: {
    v20_total_design: v20?.summary?.v20_total_design ?? null,
    v21_readable_default: census.summary,
  },
  inputs: {
    v20_total_design: fs.existsSync(v20ReportPath) ? { path: v20ReportPath, sha256: sha256(v20ReportPath), skipped: skipV20 === "1" } : { path: v20ReportPath, sha256: null, skipped: skipV20 === "1" },
    v21_readable_default_census: { path: censusPath, sha256: sha256(censusPath) },
  },
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`);
if (status !== "ready") {
  console.error(JSON.stringify(report.summary.v21_readable_default, null, 2));
  process.exit(1);
}
NODE

echo "Hepta UI harsh top-design referee v21 readable-default gate ready: $REPORT_PATH"
