#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V26_REPORT_PATH:-}"
V26_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V26_CENSUS_PATH:-}"
V26_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V26_SCREENSHOT_DIR:-}"
V25_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V25_REPORT_PATH:-}"
V25_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V26_V25_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
SKIP_V25="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V26_SKIP_V25:-0}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v26-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v26-focus-state-gate.json"
fi
if [[ -z "$V26_CENSUS_PATH" ]]; then
  V26_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v26-focus-state-census.json"
fi
if [[ -z "$V26_SCREENSHOT_DIR" ]]; then
  V26_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v26-focus-state-screenshots"
fi
if [[ -z "$V25_REPORT_PATH" ]]; then
  V25_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v25-menu-polish-gate.json"
fi
if [[ -z "$V25_LOG" ]]; then
  V25_LOG="$READINESS_DIR/v25-menu-polish-prerequisite.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$V26_SCREENSHOT_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V26_CENSUS_PATH")"

if [[ "$SKIP_V25" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V25_REPORT_PATH="$V25_REPORT_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v25-menu-polish-gate.sh "$READINESS_DIR" >"$V25_LOG" 2>&1 || {
      echo "v25 menu-polish prerequisite failed" >&2
      tail -n 180 "$V25_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V25_REPORT_PATH")" != "ready" ]]; then
  echo "v25 menu-polish prerequisite was not ready: $V25_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7610 7611 7612 7613 7614; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v26 referee" >&2
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
      echo "Hepta Control UI server exited before v26 focus-state audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$V26_SCREENSHOT_DIR" "$READINESS_DIR" >"$V26_CENSUS_PATH" <<'NODE'
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
  v25Gate: path.join(readinessDir, "ui-harsh-top-design-referee-v25-menu-polish-gate.json"),
};

const scenarios = [
  { name: "desktop-focus-state", viewport: { width: 1365, height: 900, dpr: 2, railVisible: true, isMobile: false, hasTouch: false } },
  { name: "narrow-touch-focus-state", viewport: { width: 768, height: 900, dpr: 2, railVisible: true, isMobile: true, hasTouch: true } },
  { name: "phone320-focus-state", viewport: { width: 320, height: 700, dpr: 3, railVisible: false, isMobile: true, hasTouch: true } },
];

function missingInputs() {
  return Object.entries(paths)
    .filter(([, file]) => !fs.existsSync(file))
    .map(([key, file]) => ({ code: "missing_input", key, file }));
}

function focusTargets(profile) {
  const targets = [];
  if (profile.railVisible) {
    targets.push(
      { key: "rail-search", kind: "focus", selector: "[data-control-ui-rail-search-input='light-glass']", label: "Rail search" },
      { key: "row-menu-trigger", kind: "focus", selector: "[data-chat-row-menu-toggle='ui-chat-agent']", label: "Conversation row more" },
    );
  }
  targets.push(
    { key: "thread-tools-trigger", kind: "focus", selector: "[data-control-ui-thread-tools-trigger='light-glass']", label: "Thread tools trigger" },
    { key: "composer-attachment", kind: "focus", selector: "[data-chat-composer-popover-toggle='artifact']", label: "Composer attachment trigger" },
    { key: "composer-command", kind: "focus", selector: "[data-chat-composer-popover-toggle='command']", label: "Composer command trigger" },
    { key: "composer-input", kind: "focus", selector: "[data-chat-composer-input]", label: "Message composer" },
    { key: "composer-more", kind: "focus", selector: "[data-control-ui-composer-tools-trigger='light-glass']", label: "Composer tools trigger" },
    { key: "send-button", kind: "focus", selector: "[data-agent-chat-send]", label: "Send button" },
    {
      key: "command-palette-search",
      kind: "focus",
      selector: "[data-control-ui-command-palette-input='light-glass']",
      label: "Command palette search",
      openSelector: "[data-control-ui-command-palette-trigger='light-glass']",
      modal: true,
    },
    {
      key: "artifact-popover-search",
      kind: "focus",
      selector: "[data-control-ui-composer-popover-search='light-glass']",
      label: "Artifact popover search",
      openSelector: "[data-chat-composer-popover-toggle='artifact']",
      panelSelector: "[data-chat-composer-popover='artifact']",
    },
    {
      key: "command-popover-search",
      kind: "focus",
      selector: "[data-chat-composer-picker='command'] [data-control-ui-composer-popover-search='light-glass']",
      label: "Command popover search",
      openSelector: "[data-chat-composer-popover-toggle='command']",
      panelSelector: "[data-chat-composer-popover='command']",
    },
  );
  return targets;
}

function hoverTargets(profile) {
  const targets = [];
  if (profile.railVisible) {
    targets.push({
      key: "row-action-pin",
      kind: "hover",
      selector: "[data-chat-row-menu-panel='ui-chat-agent'] [data-chat-row-menu-item='pin']",
      label: "Row menu Pin",
      openSelector: "[data-chat-row-menu-toggle='ui-chat-agent']",
    });
  }
  targets.push(
    {
      key: "thread-tools-history",
      kind: "hover",
      selector: "[data-thread-command-menu='true'] [data-control-ui-menu-item='history']",
      label: "Thread History item",
      openSelector: "[data-control-ui-thread-tools-trigger='light-glass']",
    },
    {
      key: "composer-tools-reply",
      kind: "hover",
      selector: "[data-control-ui-composer-tool-item='reply-mode']",
      label: "Composer Reply mode item",
      openSelector: "[data-control-ui-composer-tools-trigger='light-glass']",
    },
    {
      key: "command-palette-result",
      kind: "hover",
      selector: "[data-control-ui-command-palette-result='light-glass']",
      label: "Command palette result",
      openSelector: "[data-control-ui-command-palette-trigger='light-glass']",
      modal: true,
    },
  );
  return targets;
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

async function openTarget(page, target) {
  if (!target.openSelector) return { opened: true };
  const trigger = page.locator(target.openSelector).first();
  if (!(await trigger.count())) return { opened: false, reason: "missing_trigger" };
  await trigger.scrollIntoViewIfNeeded().catch(() => {});
  await trigger.click({ force: true });
  await page.waitForTimeout(180);
  if (!target.panelSelector && !target.modal) return { opened: true };
  const panelSelector = target.panelSelector || "#command-palette .command-palette";
  const panel = page.locator(panelSelector).first();
  if (!(await panel.count())) return { opened: false, reason: "missing_panel" };
  const visible = await panel.evaluate((element) => {
    const style = window.getComputedStyle(element);
    const rect = element.getBoundingClientRect();
    return style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity) > 0.01 && rect.width > 1 && rect.height > 1;
  }).catch(() => false);
  return { opened: visible, reason: visible ? null : "panel_not_visible" };
}

function stateAuditScript({ scenario, target, beforeRect }) {
  return ({ scenario, target, beforeRect }) => {
    const node = document.querySelector(target.selector);
    if (!node) return { missing: true, failures: ["state_target_missing"] };
    const style = window.getComputedStyle(node);
    const rect = node.getBoundingClientRect();
    const colorText = [
      style.borderTopColor,
      style.borderRightColor,
      style.outlineColor,
      style.boxShadow,
      style.backgroundColor,
    ].join(" ");
    const colors = [...colorText.matchAll(/rgba?\(([^)]+)\)/g)].map((match) => {
      const [r, g, b, a = "1"] = match[1].split(",").map((part) => Number.parseFloat(part.trim()));
      return { r, g, b, a };
    }).filter((color) => Number.isFinite(color.r) && Number.isFinite(color.g) && Number.isFinite(color.b) && color.a !== 0);
    const alertLikeColors = colors.filter(({ r, g, b }) => {
      const red = r > 150 && g < 105 && b < 135;
      const purple = b > 175 && r > 90 && g < 140;
      return red || purple;
    });
    const failures = [];
    const hasFocusVisible =
      target.kind !== "focus" ||
      style.outlineStyle !== "none" && Number.parseFloat(style.outlineWidth) >= 1 ||
      style.boxShadow !== "none" ||
      style.borderTopColor !== "rgba(0, 0, 0, 0)";
    if (!hasFocusVisible) failures.push("focus_state_lacks_visible_ring_or_glass_glow");
    if (alertLikeColors.length > 0 && !node.matches(".tg-row-action--danger,[data-danger],.danger,[aria-label*='Archive']")) {
      failures.push("non_danger_state_uses_alert_red_or_purple_focus_chroma");
    }
    if (beforeRect) {
      const dx = Math.abs(rect.left - beforeRect.left);
      const dy = Math.abs(rect.top - beforeRect.top);
      const dw = Math.abs(rect.width - beforeRect.width);
      const dh = Math.abs(rect.height - beforeRect.height);
      if (dx > 1.5 || dy > 1.5 || dw > 1.5 || dh > 1.5) failures.push("hover_or_focus_state_causes_layout_shift");
    }
    if (target.kind === "hover" && style.cursor !== "pointer") failures.push("hover_target_cursor_not_pointer");
    return {
      scenario: scenario.name,
      key: target.key,
      kind: target.kind,
      label: target.label,
      tag: node.tagName.toLowerCase(),
      text: node.textContent.trim().replace(/\s+/g, " ").slice(0, 120),
      rect: {
        left: Number(rect.left.toFixed(3)),
        top: Number(rect.top.toFixed(3)),
        width: Number(rect.width.toFixed(3)),
        height: Number(rect.height.toFixed(3)),
      },
      style: {
        cursor: style.cursor,
        border_top_color: style.borderTopColor,
        outline_color: style.outlineColor,
        outline_style: style.outlineStyle,
        outline_width: style.outlineWidth,
        box_shadow: style.boxShadow,
        background_color: style.backgroundColor,
      },
      alert_like_colors: alertLikeColors,
      failures,
    };
  };
}

async function auditFocus(page, scenario, target) {
  const opened = await openTarget(page, target);
  if (!opened.opened) return { scenario: scenario.name, key: target.key, kind: target.kind, failures: [`target_open_failed:${opened.reason}`] };
  const locator = page.locator(target.selector).first();
  if (!(await locator.count())) return { scenario: scenario.name, key: target.key, kind: target.kind, failures: ["state_target_missing"] };
  await locator.scrollIntoViewIfNeeded().catch(() => {});
  await locator.focus().catch(() => {});
  await page.waitForTimeout(120);
  return page.evaluate(stateAuditScript({ scenario, target, beforeRect: null }), { scenario, target, beforeRect: null });
}

async function auditHover(page, scenario, target) {
  const opened = await openTarget(page, target);
  if (!opened.opened) return { scenario: scenario.name, key: target.key, kind: target.kind, failures: [`target_open_failed:${opened.reason}`] };
  const locator = page.locator(target.selector).first();
  if (!(await locator.count())) return { scenario: scenario.name, key: target.key, kind: target.kind, failures: ["state_target_missing"] };
  await locator.scrollIntoViewIfNeeded().catch(() => {});
  const beforeRect = await locator.evaluate((node) => {
    const rect = node.getBoundingClientRect();
    return { left: rect.left, top: rect.top, width: rect.width, height: rect.height };
  });
  await locator.hover({ force: true }).catch(() => {});
  await page.waitForTimeout(120);
  return page.evaluate(stateAuditScript({ scenario, target, beforeRect }), { scenario, target, beforeRect });
}

(async () => {
  const failures = missingInputs();
  const inputs = Object.fromEntries(Object.entries(paths).map(([key, file]) => [key, { path: file, sha256: fs.existsSync(file) ? sha256(file) : null }]));
  const v25Gate = failures.length ? null : readJson(paths.v25Gate);
  if (!failures.length && v25Gate.status !== "ready") failures.push({ code: "v25_gate_not_ready", status: v25Gate.status });

  const browser = await chromium.launch({ executablePath: chromeBin, headless: true, args: ["--no-sandbox", "--disable-gpu"] });
  const stateAudits = [];
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
      screenshots.push({ scenario: scenario.name, state: "default", ...await screenshot(page, `${scenario.name}-default`) });
      for (const target of focusTargets(scenario.viewport)) {
        await closeTransient(page);
        stateAudits.push(await auditFocus(page, scenario, target));
        if (["command-palette-search", "artifact-popover-search", "command-popover-search", "composer-input"].includes(target.key)) {
          screenshots.push({ scenario: scenario.name, state: target.key, ...await screenshot(page, `${scenario.name}-${target.key}`) });
        }
      }
      for (const target of hoverTargets(scenario.viewport)) {
        await closeTransient(page);
        stateAudits.push(await auditHover(page, scenario, target));
        if (["command-palette-result", "composer-tools-reply"].includes(target.key)) {
          screenshots.push({ scenario: scenario.name, state: target.key, ...await screenshot(page, `${scenario.name}-${target.key}`) });
        }
      }
      await context.close();
    }
  } finally {
    await browser.close();
  }

  const auditFailures = stateAudits.filter((audit) => audit.failures.length > 0);
  for (const audit of auditFailures) failures.push({ code: "v26_focus_state_failure", audit });

  const summary = {
    scenario_count: scenarios.length,
    focus_target_count: scenarios.reduce((count, scenario) => count + focusTargets(scenario.viewport).length, 0),
    hover_target_count: scenarios.reduce((count, scenario) => count + hoverTargets(scenario.viewport).length, 0),
    state_audit_count: stateAudits.length,
    screenshot_count: screenshots.length,
    alert_chroma_failure_count: auditFailures.filter((audit) => audit.failures.includes("non_danger_state_uses_alert_red_or_purple_focus_chroma")).length,
    missing_focus_failure_count: auditFailures.filter((audit) => audit.failures.includes("focus_state_lacks_visible_ring_or_glass_glow")).length,
    layout_shift_failure_count: auditFailures.filter((audit) => audit.failures.includes("hover_or_focus_state_causes_layout_shift")).length,
    cursor_failure_count: auditFailures.filter((audit) => audit.failures.includes("hover_target_cursor_not_pointer")).length,
    target_open_failure_count: auditFailures.filter((audit) => audit.failures.some((failure) => failure.startsWith("target_open_failed"))).length,
    failure_count: failures.length,
    thresholds: {
      non_danger_focus_and_hover_states_may_not_use_alert_red_or_purple_chroma: true,
      focus_states_need_visible_ring_or_glass_glow: true,
      hover_and_focus_layout_shift_allowed_px: 1.5,
      hover_targets_must_keep_pointer_cursor: true,
      browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome was used.",
    },
  };

  process.stdout.write(JSON.stringify({
    schema: "hepta-ui-harsh-top-design-referee-v26-focus-state-census/v0",
    status: failures.length === 0 ? "ready" : "failed",
    generated_at: new Date().toISOString(),
    readiness_dir: readinessDir,
    base_url: baseUrl,
    summary,
    inputs,
    screenshots,
    state_audits: stateAudits,
    failures,
  }, null, 2));
})().catch((error) => {
  process.stderr.write(`${error.stack || error}\n`);
  process.exit(1);
});
NODE

node - "$V26_CENSUS_PATH" "$REPORT_PATH" "$V25_REPORT_PATH" "$SKIP_V25" <<'NODE'
const crypto = require("node:crypto");
const fs = require("node:fs");

const [censusPath, reportPath, v25ReportPath, skipV25] = process.argv.slice(2);
const sha256 = (file) => fs.existsSync(file) ? crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex") : null;
const census = JSON.parse(fs.readFileSync(censusPath, "utf8"));
const v25 = fs.existsSync(v25ReportPath) ? JSON.parse(fs.readFileSync(v25ReportPath, "utf8")) : null;
const status = census.status === "ready" && (skipV25 === "1" || v25?.status === "ready") ? "ready" : "failed";
const report = {
  schema: "hepta-ui-harsh-top-design-referee-v26-focus-state-gate/v0",
  status,
  generated_at: new Date().toISOString(),
  summary: {
    v25_menu_polish_referee: v25?.summary?.v25_menu_polish_referee ?? null,
    v26_focus_state_referee: census.summary,
  },
  inputs: {
    v25_menu_polish_referee: fs.existsSync(v25ReportPath) ? { path: v25ReportPath, sha256: sha256(v25ReportPath), skipped: skipV25 === "1" } : { path: v25ReportPath, sha256: null, skipped: skipV25 === "1" },
    v26_focus_state_census: { path: censusPath, sha256: sha256(censusPath) },
  },
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`);
if (status !== "ready") {
  console.error(JSON.stringify(census.summary, null, 2));
  process.exit(1);
}
NODE

echo "Hepta UI harsh top-design referee v26 focus-state gate ready: $REPORT_PATH"
