#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V28_REPORT_PATH:-}"
V28_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V28_CENSUS_PATH:-}"
V28_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V28_SCREENSHOT_DIR:-}"
V27_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V27_REPORT_PATH:-}"
V27_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V28_V27_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
SKIP_V27="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V28_SKIP_V27:-0}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v28-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v28-scroll-completeness-gate.json"
fi
if [[ -z "$V28_CENSUS_PATH" ]]; then
  V28_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v28-scroll-completeness-census.json"
fi
if [[ -z "$V28_SCREENSHOT_DIR" ]]; then
  V28_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v28-scroll-completeness-screenshots"
fi
if [[ -z "$V27_REPORT_PATH" ]]; then
  V27_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v27-text-fit-gate.json"
fi
if [[ -z "$V27_LOG" ]]; then
  V27_LOG="$READINESS_DIR/v27-text-fit-prerequisite.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$V28_SCREENSHOT_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V28_CENSUS_PATH")"

if [[ "$SKIP_V27" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V27_REPORT_PATH="$V27_REPORT_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v27-text-fit-gate.sh "$READINESS_DIR" >"$V27_LOG" 2>&1 || {
      echo "v27 text-fit prerequisite failed" >&2
      tail -n 180 "$V27_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V27_REPORT_PATH")" != "ready" ]]; then
  echo "v27 text-fit prerequisite was not ready: $V27_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7640 7641 7642 7643 7644; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v28 referee" >&2
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
      echo "Hepta Control UI server exited before v28 scroll-completeness audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$V28_SCREENSHOT_DIR" "$READINESS_DIR" >"$V28_CENSUS_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir, readinessDir] = process.argv.slice(2);
fs.mkdirSync(screenshotDir, { recursive: true });

const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase() || "item";
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const paths = { v27Gate: path.join(readinessDir, "ui-harsh-top-design-referee-v27-text-fit-gate.json") };

const scenarios = [
  { name: "desktop-scroll-completeness", viewport: { width: 1365, height: 900, dpr: 2, isMobile: false, hasTouch: false } },
  { name: "narrow-touch-scroll-completeness", viewport: { width: 768, height: 900, dpr: 2, isMobile: true, hasTouch: true } },
  { name: "mobile-scroll-completeness", viewport: { width: 500, height: 844, dpr: 2, isMobile: true, hasTouch: true } },
  { name: "phone320-scroll-completeness", viewport: { width: 320, height: 700, dpr: 3, isMobile: true, hasTouch: true } },
];

const states = [
  {
    key: "command-palette",
    openSelector: "[data-control-ui-command-palette-trigger='light-glass']",
    panelSelector: "#command-palette .command-palette",
    containerSelector: "#command-palette-results",
    itemSelector: "#command-palette [data-control-ui-command-palette-item]",
  },
  {
    key: "artifact-popover",
    openSelector: "[data-chat-composer-popover-toggle='artifact']",
    panelSelector: "[data-chat-composer-popover='artifact']",
    containerSelector: "[data-chat-composer-popover='artifact']",
    itemSelector: "[data-chat-composer-popover='artifact'] .tg-composer-popover__item",
  },
  {
    key: "command-popover",
    openSelector: "[data-chat-composer-popover-toggle='command']",
    panelSelector: "[data-chat-composer-popover='command']",
    containerSelector: "[data-chat-composer-popover='command']",
    itemSelector: "[data-chat-composer-popover='command'] .tg-composer-popover__item",
  },
  {
    key: "composer-tools",
    openSelector: "[data-control-ui-composer-tools-trigger='light-glass']",
    panelSelector: "[data-control-ui-composer-tools-panel='light-glass']",
    containerSelector: "[data-control-ui-composer-tools-panel='light-glass']",
    itemSelector: "[data-control-ui-composer-tools-panel='light-glass'] [role='menuitem']",
  },
];

function missingInputs() {
  return Object.entries(paths)
    .filter(([, file]) => !fs.existsSync(file))
    .map(([key, file]) => ({ code: "missing_input", key, file }));
}

async function setProfile(page, profile) {
  await page.setViewportSize({ width: profile.width, height: profile.height });
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.goto(baseUrl, { waitUntil: "networkidle" });
  await page.waitForSelector('[data-rust-rendered-control-ui="true"]', { timeout: 30000 });
}

async function screenshot(page, label) {
  const file = path.join(screenshotDir, `${sanitize(label)}.png`);
  await page.screenshot({ path: file, fullPage: false });
  return { path: file, sha256: sha256(file) };
}

async function closeTransient(page) {
  await page.keyboard.press("Escape").catch(() => {});
  await page.locator("body").click({ position: { x: 4, y: 4 } }).catch(() => {});
  await page.waitForTimeout(90);
}

async function openState(page, state) {
  const trigger = page.locator(state.openSelector).first();
  if (!(await trigger.count())) return { opened: false, reason: "missing_trigger" };
  await trigger.scrollIntoViewIfNeeded().catch(() => {});
  await trigger.click({ force: true });
  await page.waitForTimeout(180);
  const panel = page.locator(state.panelSelector).first();
  if (!(await panel.count())) return { opened: false, reason: "missing_panel" };
  const visible = await panel.evaluate((element) => {
    const style = window.getComputedStyle(element);
    const rect = element.getBoundingClientRect();
    return style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity) > 0.01 && rect.width > 1 && rect.height > 1;
  }).catch(() => false);
  return { opened: visible, reason: visible ? null : "panel_not_visible" };
}

async function auditInitialVisibleItems(page, scenario, state) {
  return page.evaluate(({ scenario, state }) => {
    const round = (value) => Number(value.toFixed(3));
    const rectObj = (rect) => ({ x: round(rect.x), y: round(rect.y), width: round(rect.width), height: round(rect.height), top: round(rect.top), right: round(rect.right), bottom: round(rect.bottom), left: round(rect.left) });
    const intersect = (a, b) => {
      const left = Math.max(a.left, b.left);
      const top = Math.max(a.top, b.top);
      const right = Math.min(a.right, b.right);
      const bottom = Math.min(a.bottom, b.bottom);
      return { left, top, right, bottom, width: Math.max(0, right - left), height: Math.max(0, bottom - top), x: left, y: top };
    };
    const visible = (element) => {
      const style = window.getComputedStyle(element);
      const rect = element.getBoundingClientRect();
      return style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity) > 0.01 && rect.width > 1 && rect.height > 1;
    };
    const container = document.querySelector(state.containerSelector);
    if (!container) return [{ scenario: scenario.name, state: state.key, code: "missing_container", failures: ["missing_container"] }];
    container.scrollTop = 0;
    const viewport = { left: 0, top: 0, right: window.innerWidth, bottom: window.innerHeight, width: window.innerWidth, height: window.innerHeight, x: 0, y: 0 };
    const containerRect = intersect(container.getBoundingClientRect(), viewport);
    const items = [...document.querySelectorAll(state.itemSelector)].filter(visible);
    return items.map((item, index) => {
      const rect = item.getBoundingClientRect();
      const visibleRect = intersect(rect, containerRect);
      const area = rect.width * rect.height;
      const visibleArea = visibleRect.width * visibleRect.height;
      const ratio = area > 0 ? visibleArea / area : 0;
      const verticalRatio = rect.height > 0 ? visibleRect.height / rect.height : 0;
      const isPartiallyVisible = ratio > 0.025 && ratio < 0.995;
      const failures = [];
      if (isPartiallyVisible) failures.push("partial_visible_scroll_item");
      return {
        scenario: scenario.name,
        state: state.key,
        index,
        label: item.getAttribute("aria-label") || item.textContent.trim().replace(/\s+/g, " ").slice(0, 120),
        clipped_ratio: round(ratio),
        vertical_visible_ratio: round(verticalRatio),
        item_rect: rectObj(rect),
        container_rect: rectObj(containerRect),
        failures,
      };
    });
  }, { scenario, state });
}

(async () => {
  const failures = missingInputs();
  const inputs = Object.fromEntries(Object.entries(paths).map(([key, file]) => [key, { path: file, sha256: fs.existsSync(file) ? sha256(file) : null }]));
  const v27Gate = failures.length ? null : readJson(paths.v27Gate);
  if (!failures.length && v27Gate.status !== "ready") failures.push({ code: "v27_gate_not_ready", status: v27Gate.status });

  const browser = await chromium.launch({ executablePath: chromeBin, headless: true, args: ["--no-sandbox", "--disable-gpu"] });
  const scrollItemAudits = [];
  const screenshots = [];
  try {
    for (const scenario of scenarios) {
      const context = await browser.newContext({
        viewport: { width: scenario.viewport.width, height: scenario.viewport.height },
        deviceScaleFactor: scenario.viewport.dpr,
        isMobile: scenario.viewport.isMobile,
        hasTouch: scenario.viewport.hasTouch,
      });
      const page = await context.newPage();
      await setProfile(page, scenario.viewport);
      for (const state of states) {
        await closeTransient(page);
        const opened = await openState(page, state);
        if (!opened.opened) {
          failures.push({ code: "v28_state_not_opened", scenario: scenario.name, state: state.key, reason: opened.reason });
          continue;
        }
        scrollItemAudits.push(...await auditInitialVisibleItems(page, scenario, state));
        screenshots.push({ scenario: scenario.name, state: state.key, ...await screenshot(page, `${scenario.name}-${state.key}`) });
      }
      await context.close();
    }
  } finally {
    await browser.close();
  }

  const auditFailures = scrollItemAudits.filter((audit) => audit.failures?.length > 0);
  for (const audit of auditFailures) failures.push({ code: "v28_scroll_completeness_failure", audit });

  const summary = {
    scenario_count: scenarios.length,
    opened_state_count: scenarios.length * states.length,
    scroll_item_audit_count: scrollItemAudits.length,
    screenshot_count: screenshots.length,
    partial_item_failure_count: auditFailures.filter((audit) => audit.failures.includes("partial_visible_scroll_item")).length,
    state_open_failure_count: failures.filter((failure) => failure.code === "v28_state_not_opened").length,
    failure_count: failures.length,
    thresholds: {
      opened_scroll_items_may_not_be_partially_visible_in_initial_view: true,
      partial_visible_clipped_ratio_min: 0.995,
      partial_visible_lower_noise_floor: 0.025,
      browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome was used.",
    },
  };

  process.stdout.write(JSON.stringify({
    schema: "hepta-ui-harsh-top-design-referee-v28-scroll-completeness-census/v0",
    status: failures.length === 0 ? "ready" : "failed",
    generated_at: new Date().toISOString(),
    readiness_dir: readinessDir,
    base_url: baseUrl,
    summary,
    inputs,
    scroll_item_audits: scrollItemAudits,
    screenshots,
    failures,
  }, null, 2));
})().catch((error) => {
  process.stdout.write(JSON.stringify({
    schema: "hepta-ui-harsh-top-design-referee-v28-scroll-completeness-census/v0",
    status: "failed",
    error: String(error?.stack || error),
    readiness_dir: readinessDir,
  }, null, 2));
  process.exitCode = 1;
});
NODE

cp "$V28_CENSUS_PATH" "$REPORT_PATH"

if [[ "$(jq -r '.status' "$REPORT_PATH")" != "ready" ]]; then
  echo "v28 scroll-completeness referee failed: $REPORT_PATH" >&2
  jq '.summary, (.failures[:20] // [])' "$REPORT_PATH" >&2
  exit 1
fi

echo "v28 scroll-completeness referee ready: $REPORT_PATH"
jq '.summary' "$REPORT_PATH"
