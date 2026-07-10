#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V29_REPORT_PATH:-}"
V29_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V29_CENSUS_PATH:-}"
V29_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V29_SCREENSHOT_DIR:-}"
V28_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V28_REPORT_PATH:-}"
V28_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V29_V28_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
SKIP_V28="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V29_SKIP_V28:-0}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v29-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v29-reduced-motion-gate.json"
fi
if [[ -z "$V29_CENSUS_PATH" ]]; then
  V29_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v29-reduced-motion-census.json"
fi
if [[ -z "$V29_SCREENSHOT_DIR" ]]; then
  V29_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v29-reduced-motion-screenshots"
fi
if [[ -z "$V28_REPORT_PATH" ]]; then
  V28_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v28-scroll-completeness-gate.json"
fi
if [[ -z "$V28_LOG" ]]; then
  V28_LOG="$READINESS_DIR/v28-scroll-completeness-prerequisite.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$V29_SCREENSHOT_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V29_CENSUS_PATH")"

if [[ "$SKIP_V28" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V28_REPORT_PATH="$V28_REPORT_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v28-scroll-completeness-gate.sh "$READINESS_DIR" >"$V28_LOG" 2>&1 || {
      echo "v28 scroll-completeness prerequisite failed" >&2
      tail -n 180 "$V28_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V28_REPORT_PATH")" != "ready" ]]; then
  echo "v28 scroll-completeness prerequisite was not ready: $V28_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7650 7651 7652 7653 7654; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v29 referee" >&2
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
      echo "Hepta Control UI server exited before v29 reduced-motion audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$V29_SCREENSHOT_DIR" "$READINESS_DIR" >"$V29_CENSUS_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir, readinessDir] = process.argv.slice(2);
fs.mkdirSync(screenshotDir, { recursive: true });

const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase() || "item";
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const paths = { v28Gate: path.join(readinessDir, "ui-harsh-top-design-referee-v28-scroll-completeness-gate.json") };

const scenarios = [
  { name: "desktop-reduced-motion", viewport: { width: 1365, height: 900, dpr: 2, railVisible: true, isMobile: false, hasTouch: false } },
  { name: "narrow-touch-reduced-motion", viewport: { width: 768, height: 900, dpr: 2, railVisible: true, isMobile: true, hasTouch: true } },
  { name: "phone320-reduced-motion", viewport: { width: 320, height: 700, dpr: 3, railVisible: false, isMobile: true, hasTouch: true } },
];

const baseTargets = (scenario) => [
  ...(scenario.viewport.railVisible ? [
    { key: "new-conversation", selector: "[data-control-ui-icon-button='new-conversation']", label: "New conversation" },
    { key: "all-chats", selector: ".tg-folder-chip", label: "All chats", required: false },
    { key: "active-chat-row", selector: ".tg-chat-item.active", label: "Active chat row" },
    { key: "row-menu-trigger", selector: "[data-chat-row-menu-toggle='ui-chat-agent']", label: "Conversation row more" },
  ] : []),
  { key: "command-palette-trigger", selector: "[data-control-ui-command-palette-trigger='light-glass']", label: "Command palette trigger" },
  { key: "thread-tools-trigger", selector: "[data-control-ui-thread-tools-trigger='light-glass']", label: "Thread tools trigger" },
  { key: "attach-trigger", selector: "[data-chat-composer-popover-toggle='artifact']", label: "Attach trigger" },
  { key: "command-trigger", selector: "[data-chat-composer-popover-toggle='command']", label: "Command trigger" },
  { key: "send-button", selector: "[data-agent-chat-send]", label: "Send button" },
  { key: "composer-tools-trigger", selector: "[data-control-ui-composer-tools-trigger='light-glass']", label: "Composer tools trigger" },
];

const openedGroups = [
  {
    key: "row-menu",
    railOnly: true,
    openSelector: "[data-chat-row-menu-toggle='ui-chat-agent']",
    targetSelector: "[data-chat-row-menu-panel='ui-chat-agent'] [role='menuitem']",
  },
  {
    key: "thread-tools",
    openSelector: "[data-control-ui-thread-tools-trigger='light-glass']",
    targetSelector: "[data-control-ui-thread-tools-panel='light-glass'] [role='menuitem']",
  },
  {
    key: "composer-tools",
    openSelector: "[data-control-ui-composer-tools-trigger='light-glass']",
    targetSelector: "[data-control-ui-composer-tools-panel='light-glass'] [role='menuitem'], [data-control-ui-composer-tools-panel='light-glass'] select",
  },
  {
    key: "command-palette",
    openSelector: "[data-control-ui-command-palette-trigger='light-glass']",
    targetSelector: "#command-palette [data-control-ui-command-palette-item], [data-control-ui-command-palette-close='light-glass']",
    modal: true,
  },
  {
    key: "artifact-popover",
    openSelector: "[data-chat-composer-popover-toggle='artifact']",
    targetSelector: "[data-chat-composer-popover='artifact'] [role='menuitem']",
  },
  {
    key: "command-popover",
    openSelector: "[data-chat-composer-popover-toggle='command']",
    targetSelector: "[data-chat-composer-popover='command'] [role='menuitem']",
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

async function openGroup(page, group) {
  const trigger = page.locator(group.openSelector).first();
  if (!(await trigger.count())) return { opened: false, reason: "missing_trigger" };
  await trigger.scrollIntoViewIfNeeded().catch(() => {});
  await trigger.click({ force: true });
  await page.waitForTimeout(180);
  const count = await page.locator(group.targetSelector).count();
  return count > 0 ? { opened: true, target_count: count } : { opened: false, reason: "missing_targets" };
}

async function visibleElementRefs(page, selector, limit = 24) {
  return page.locator(selector).evaluateAll((nodes, limit) => {
    return nodes.map((node, index) => {
      const rect = node.getBoundingClientRect();
      const style = window.getComputedStyle(node);
      const visible = rect.width > 1 && rect.height > 1 && style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity) > 0.01;
      return {
        index,
        visible,
        label: node.getAttribute("aria-label") || node.getAttribute("title") || node.textContent.trim().replace(/\s+/g, " ").slice(0, 80) || node.tagName.toLowerCase(),
      };
    }).filter((item) => item.visible).slice(0, limit);
  }, limit);
}

async function auditHover(page, scenario, groupKey, selector, index, label) {
  const locator = page.locator(selector).nth(index);
  await locator.scrollIntoViewIfNeeded().catch(() => {});
  await page.waitForTimeout(40);
  const before = await locator.evaluate((node) => {
    const rect = node.getBoundingClientRect();
    const style = window.getComputedStyle(node);
    return {
      rect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height },
      transform: style.transform,
      transitionDuration: style.transitionDuration,
      transitionProperty: style.transitionProperty,
    };
  });
  await locator.hover({ force: true });
  await page.waitForTimeout(90);
  const after = await locator.evaluate((node) => {
    const rect = node.getBoundingClientRect();
    const style = window.getComputedStyle(node);
    return {
      rect: { x: rect.x, y: rect.y, width: rect.width, height: rect.height },
      transform: style.transform,
      transitionDuration: style.transitionDuration,
      transitionProperty: style.transitionProperty,
    };
  });
  const round = (value) => Number(value.toFixed(3));
  const dx = after.rect.x - before.rect.x;
  const dy = after.rect.y - before.rect.y;
  const dw = after.rect.width - before.rect.width;
  const dh = after.rect.height - before.rect.height;
  const maxRectDelta = Math.max(Math.abs(dx), Math.abs(dy), Math.abs(dw), Math.abs(dh));
  const isIdentityTransform = (value) => {
    if (!value || value === "none") return true;
    if (value === "matrix(1, 0, 0, 1, 0, 0)") return true;
    if (value === "matrix3d(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)") return true;
    return false;
  };
  const transformActive = !isIdentityTransform(after.transform);
  const transformTransition = String(after.transitionProperty || "").split(",").some((part) => part.trim() === "transform" || part.trim() === "all");
  const hasDuration = String(after.transitionDuration || "").split(",").some((part) => {
    const trimmed = part.trim();
    if (!trimmed || trimmed === "0s" || trimmed === "0ms") return false;
    return Number.parseFloat(trimmed) > 0;
  });
  const failures = [];
  if (transformActive) failures.push("reduced_motion_hover_transform");
  if (maxRectDelta > 0.5) failures.push("reduced_motion_hover_layout_shift");
  if (transformTransition && hasDuration) failures.push("reduced_motion_transform_transition");
  await page.mouse.move(2, 2);
  await page.waitForTimeout(40);
  return {
    scenario: scenario.name,
    group: groupKey,
    index,
    label,
    before_transform: before.transform,
    after_transform: after.transform,
    transition_property: after.transitionProperty,
    transition_duration: after.transitionDuration,
    rect_delta: { dx: round(dx), dy: round(dy), dw: round(dw), dh: round(dh), max: round(maxRectDelta) },
    failures,
  };
}

(async () => {
  const failures = missingInputs();
  const inputs = Object.fromEntries(Object.entries(paths).map(([key, file]) => [key, { path: file, sha256: fs.existsSync(file) ? sha256(file) : null }]));
  const v28Gate = failures.length ? null : readJson(paths.v28Gate);
  if (!failures.length && v28Gate.status !== "ready") failures.push({ code: "v28_gate_not_ready", status: v28Gate.status });

  const browser = await chromium.launch({ executablePath: chromeBin, headless: true, args: ["--no-sandbox", "--disable-gpu"] });
  const motionAudits = [];
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

      for (const target of baseTargets(scenario)) {
        const refs = await visibleElementRefs(page, target.selector, 1);
        if (!refs.length) {
          if (target.required !== false) failures.push({ code: "v29_missing_reduced_motion_target", scenario: scenario.name, group: "default", target });
          continue;
        }
        motionAudits.push(await auditHover(page, scenario, "default", target.selector, refs[0].index, target.label));
      }

      for (const group of openedGroups) {
        if (group.railOnly && !scenario.viewport.railVisible) continue;
        await closeTransient(page);
        const opened = await openGroup(page, group);
        if (!opened.opened) {
          failures.push({ code: "v29_group_not_opened", scenario: scenario.name, group: group.key, reason: opened.reason });
          continue;
        }
        screenshots.push({ scenario: scenario.name, state: group.key, ...await screenshot(page, `${scenario.name}-${group.key}`) });
        const refs = await visibleElementRefs(page, group.targetSelector, 12);
        if (!refs.length) failures.push({ code: "v29_missing_group_targets", scenario: scenario.name, group: group.key });
        for (const ref of refs) {
          motionAudits.push(await auditHover(page, scenario, group.key, group.targetSelector, ref.index, ref.label));
        }
      }
      await context.close();
    }
  } finally {
    await browser.close();
  }

  const auditFailures = motionAudits.filter((audit) => audit.failures?.length > 0);
  for (const audit of auditFailures) failures.push({ code: "v29_reduced_motion_failure", audit });

  const summary = {
    scenario_count: scenarios.length,
    opened_group_count: motionAudits.filter((audit) => audit.group !== "default").length,
    reduced_motion_audit_count: motionAudits.length,
    screenshot_count: screenshots.length,
    hover_transform_failure_count: auditFailures.filter((audit) => audit.failures.includes("reduced_motion_hover_transform")).length,
    hover_layout_shift_failure_count: auditFailures.filter((audit) => audit.failures.includes("reduced_motion_hover_layout_shift")).length,
    transform_transition_failure_count: auditFailures.filter((audit) => audit.failures.includes("reduced_motion_transform_transition")).length,
    failure_count: failures.length,
    thresholds: {
      prefers_reduced_motion_requires_no_hover_transform: true,
      max_hover_rect_delta_px: 0.5,
      transform_transition_duration_must_be_zero: true,
      browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome was used.",
    },
  };

  process.stdout.write(JSON.stringify({
    schema: "hepta-ui-harsh-top-design-referee-v29-reduced-motion-census/v0",
    status: failures.length === 0 ? "ready" : "failed",
    generated_at: new Date().toISOString(),
    readiness_dir: readinessDir,
    base_url: baseUrl,
    summary,
    inputs,
    motion_audits: motionAudits,
    screenshots,
    failures,
  }, null, 2));
})().catch((error) => {
  process.stdout.write(JSON.stringify({
    schema: "hepta-ui-harsh-top-design-referee-v29-reduced-motion-census/v0",
    status: "failed",
    error: String(error?.stack || error),
    readiness_dir: readinessDir,
  }, null, 2));
  process.exitCode = 1;
});
NODE

cp "$V29_CENSUS_PATH" "$REPORT_PATH"

if [[ "$(jq -r '.status' "$REPORT_PATH")" != "ready" ]]; then
  echo "v29 reduced-motion referee failed: $REPORT_PATH" >&2
  jq '.summary, (.failures[:20] // [])' "$REPORT_PATH" >&2
  exit 1
fi

echo "v29 reduced-motion referee ready: $REPORT_PATH"
jq '.summary' "$REPORT_PATH"
