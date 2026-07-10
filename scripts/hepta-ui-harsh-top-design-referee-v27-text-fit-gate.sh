#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V27_REPORT_PATH:-}"
V27_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V27_CENSUS_PATH:-}"
V27_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V27_SCREENSHOT_DIR:-}"
V26_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V26_REPORT_PATH:-}"
V26_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V27_V26_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
SKIP_V26="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V27_SKIP_V26:-0}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v27-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v27-text-fit-gate.json"
fi
if [[ -z "$V27_CENSUS_PATH" ]]; then
  V27_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v27-text-fit-census.json"
fi
if [[ -z "$V27_SCREENSHOT_DIR" ]]; then
  V27_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v27-text-fit-screenshots"
fi
if [[ -z "$V26_REPORT_PATH" ]]; then
  V26_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v26-focus-state-gate.json"
fi
if [[ -z "$V26_LOG" ]]; then
  V26_LOG="$READINESS_DIR/v26-focus-state-prerequisite.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$V27_SCREENSHOT_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V27_CENSUS_PATH")"

if [[ "$SKIP_V26" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V26_REPORT_PATH="$V26_REPORT_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v26-focus-state-gate.sh "$READINESS_DIR" >"$V26_LOG" 2>&1 || {
      echo "v26 focus-state prerequisite failed" >&2
      tail -n 180 "$V26_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V26_REPORT_PATH")" != "ready" ]]; then
  echo "v26 focus-state prerequisite was not ready: $V26_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7620 7621 7622 7623 7624; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v27 referee" >&2
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
      echo "Hepta Control UI server exited before v27 text-fit audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$V27_SCREENSHOT_DIR" "$READINESS_DIR" >"$V27_CENSUS_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir, readinessDir] = process.argv.slice(2);
fs.mkdirSync(screenshotDir, { recursive: true });

const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase() || "item";
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));

const paths = {
  v26Gate: path.join(readinessDir, "ui-harsh-top-design-referee-v26-focus-state-gate.json"),
};

const scenarios = [
  { name: "desktop-text-fit", viewport: { width: 1365, height: 900, dpr: 2, isMobile: false, hasTouch: false } },
  { name: "narrow-touch-text-fit", viewport: { width: 768, height: 900, dpr: 2, isMobile: true, hasTouch: true } },
  { name: "phone320-text-fit", viewport: { width: 320, height: 700, dpr: 3, isMobile: true, hasTouch: true } },
];

const states = [
  { key: "default", screenshot: true },
  { key: "command-palette", openSelector: "[data-control-ui-command-palette-trigger='light-glass']", panelSelector: "#command-palette .command-palette", screenshot: true },
  { key: "artifact-popover", openSelector: "[data-chat-composer-popover-toggle='artifact']", panelSelector: "[data-chat-composer-popover='artifact']", screenshot: true },
  { key: "command-popover", openSelector: "[data-chat-composer-popover-toggle='command']", panelSelector: "[data-chat-composer-popover='command']", screenshot: true },
  { key: "composer-tools", openSelector: "[data-control-ui-composer-tools-trigger='light-glass']", panelSelector: "[data-control-ui-composer-tools-panel='light-glass']", screenshot: false },
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
  if (!state.openSelector) return { opened: true };
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

async function auditTextFit(page, scenario, state) {
  return page.evaluate(({ scenario, state }) => {
    const canvas = document.createElement("canvas");
    const context = canvas.getContext("2d");
    const audits = [];
    const visible = (node) => {
      const rect = node.getBoundingClientRect();
      const style = window.getComputedStyle(node);
      return rect.width > 1 && rect.height > 1 && style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity) > 0.01;
    };
    const scope = state.panelSelector ? document.querySelector(state.panelSelector) : document;
    const nodes = scope ? [...scope.querySelectorAll("input, textarea, button, a, summary, [role='menuitem'], strong, small, .tg-menu-item__label, .command-palette__kind, .command-palette__copy")] : [];
    for (const node of nodes.filter(visible)) {
      const style = window.getComputedStyle(node);
      const rect = node.getBoundingClientRect();
      const tag = node.tagName.toLowerCase();
      const isTextControl = tag === "input" || tag === "textarea";
      const text = isTextControl ? (node.value || node.getAttribute("placeholder") || "") : node.textContent.trim().replace(/\s+/g, " ");
      if (!text) continue;
      context.font = style.font || `${style.fontSize} ${style.fontFamily}`;
      const paddingX = Number.parseFloat(style.paddingLeft || "0") + Number.parseFloat(style.paddingRight || "0");
      const available = Math.max(0, rect.width - paddingX - 1);
      const measured = context.measureText(text).width;
      const failures = [];
      if (isTextControl && measured > available + 1) failures.push("input_placeholder_or_value_text_clipped");
      if (!isTextControl && node.scrollWidth > node.clientWidth + 1) failures.push("visible_label_horizontal_overflow");
      if (!isTextControl && node.scrollHeight > node.clientHeight + 1) failures.push("visible_label_vertical_overflow");
      audits.push({
        scenario: scenario.name,
        state: state.key,
        tag,
        selector_hint: node.id ? `#${node.id}` : node.getAttribute("data-control-ui-command-palette-input") ? "[data-control-ui-command-palette-input]" : node.getAttribute("data-control-ui-composer-popover-search") ? "[data-control-ui-composer-popover-search]" : node.className || node.getAttribute("role") || "",
        text: text.slice(0, 140),
        rect: { width: Number(rect.width.toFixed(3)), height: Number(rect.height.toFixed(3)) },
        measured_text_width: Number(measured.toFixed(3)),
        available_text_width: Number(available.toFixed(3)),
        scroll: { scroll_width: node.scrollWidth, client_width: node.clientWidth, scroll_height: node.scrollHeight, client_height: node.clientHeight },
        failures,
      });
    }
    return audits;
  }, { scenario, state });
}

(async () => {
  const failures = missingInputs();
  const inputs = Object.fromEntries(Object.entries(paths).map(([key, file]) => [key, { path: file, sha256: fs.existsSync(file) ? sha256(file) : null }]));
  const v26Gate = failures.length ? null : readJson(paths.v26Gate);
  if (!failures.length && v26Gate.status !== "ready") failures.push({ code: "v26_gate_not_ready", status: v26Gate.status });

  const browser = await chromium.launch({ executablePath: chromeBin, headless: true, args: ["--no-sandbox", "--disable-gpu"] });
  const textFitAudits = [];
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
          failures.push({ code: "v27_state_not_opened", scenario: scenario.name, state: state.key, reason: opened.reason });
          continue;
        }
        textFitAudits.push(...await auditTextFit(page, scenario, state));
        if (state.screenshot) screenshots.push({ scenario: scenario.name, state: state.key, ...await screenshot(page, `${scenario.name}-${state.key}`) });
      }
      await context.close();
    }
  } finally {
    await browser.close();
  }

  const auditFailures = textFitAudits.filter((audit) => audit.failures.length > 0);
  for (const audit of auditFailures) failures.push({ code: "v27_text_fit_failure", audit });

  const summary = {
    scenario_count: scenarios.length,
    state_count: scenarios.length * states.length,
    text_fit_audit_count: textFitAudits.length,
    input_text_fit_audit_count: textFitAudits.filter((audit) => audit.tag === "input" || audit.tag === "textarea").length,
    screenshot_count: screenshots.length,
    input_text_clip_failure_count: auditFailures.filter((audit) => audit.failures.includes("input_placeholder_or_value_text_clipped")).length,
    label_overflow_failure_count: auditFailures.filter((audit) => audit.failures.some((failure) => failure.includes("visible_label"))).length,
    state_open_failure_count: failures.filter((failure) => failure.code === "v27_state_not_opened").length,
    failure_count: failures.length,
    thresholds: {
      visible_input_placeholder_or_value_must_fit_width: true,
      visible_labels_must_not_overflow: true,
      browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome was used.",
    },
  };

  process.stdout.write(JSON.stringify({
    schema: "hepta-ui-harsh-top-design-referee-v27-text-fit-census/v0",
    status: failures.length === 0 ? "ready" : "failed",
    generated_at: new Date().toISOString(),
    readiness_dir: readinessDir,
    base_url: baseUrl,
    summary,
    inputs,
    screenshots,
    text_fit_audits: textFitAudits,
    failures,
  }, null, 2));
})().catch((error) => {
  process.stderr.write(`${error.stack || error}\n`);
  process.exit(1);
});
NODE

node - "$V27_CENSUS_PATH" "$REPORT_PATH" "$V26_REPORT_PATH" "$SKIP_V26" <<'NODE'
const crypto = require("node:crypto");
const fs = require("node:fs");

const [censusPath, reportPath, v26ReportPath, skipV26] = process.argv.slice(2);
const sha256 = (file) => fs.existsSync(file) ? crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex") : null;
const census = JSON.parse(fs.readFileSync(censusPath, "utf8"));
const v26 = fs.existsSync(v26ReportPath) ? JSON.parse(fs.readFileSync(v26ReportPath, "utf8")) : null;
const status = census.status === "ready" && (skipV26 === "1" || v26?.status === "ready") ? "ready" : "failed";
const report = {
  schema: "hepta-ui-harsh-top-design-referee-v27-text-fit-gate/v0",
  status,
  generated_at: new Date().toISOString(),
  summary: {
    v26_focus_state_referee: v26?.summary?.v26_focus_state_referee ?? null,
    v27_text_fit_referee: census.summary,
  },
  inputs: {
    v26_focus_state_referee: fs.existsSync(v26ReportPath) ? { path: v26ReportPath, sha256: sha256(v26ReportPath), skipped: skipV26 === "1" } : { path: v26ReportPath, sha256: null, skipped: skipV26 === "1" },
    v27_text_fit_census: { path: censusPath, sha256: sha256(censusPath) },
  },
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`);
if (status !== "ready") {
  console.error(JSON.stringify(census.summary, null, 2));
  process.exit(1);
}
NODE

echo "Hepta UI harsh top-design referee v27 text-fit gate ready: $REPORT_PATH"
