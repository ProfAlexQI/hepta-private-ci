#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

source scripts/lib/hepta-ui-rust-toolchain.sh
source scripts/lib/hepta-control-ui-runtime-fixture.sh
hepta_ui_activate_rust_toolchain

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V24_REPORT_PATH:-}"
V24_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V24_CENSUS_PATH:-}"
V24_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V24_SCREENSHOT_DIR:-}"
V23_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V23_REPORT_PATH:-}"
V23_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V24_V23_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
SKIP_V23="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V24_SKIP_V23:-0}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v24-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v24-edge-affordance-gate.json"
fi
if [[ -z "$V24_CENSUS_PATH" ]]; then
  V24_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v24-edge-affordance-census.json"
fi
if [[ -z "$V24_SCREENSHOT_DIR" ]]; then
  V24_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v24-edge-affordance-screenshots"
fi
if [[ -z "$V23_REPORT_PATH" ]]; then
  V23_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v23-evidence-glass-referee-gate.json"
fi
if [[ -z "$V23_LOG" ]]; then
  V23_LOG="$READINESS_DIR/v23-evidence-glass-referee-prerequisite.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$V24_SCREENSHOT_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V24_CENSUS_PATH")"

if [[ "$SKIP_V23" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V23_REPORT_PATH="$V23_REPORT_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v23-evidence-glass-referee-gate.sh "$READINESS_DIR" >"$V23_LOG" 2>&1 || {
      echo "v23 evidence-glass prerequisite failed" >&2
      tail -n 180 "$V23_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V23_REPORT_PATH")" != "ready" ]]; then
  echo "v23 evidence-glass prerequisite was not ready: $V23_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7500 7501 7502 7503 7504; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v24 referee" >&2
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
      echo "Hepta Control UI server exited before v24 edge-affordance audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$V24_SCREENSHOT_DIR" "$READINESS_DIR" >"$V24_CENSUS_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir, readinessDir] = process.argv.slice(2);
fs.mkdirSync(screenshotDir, { recursive: true });

const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const round = (value, digits = 3) => Number(value.toFixed(digits));
const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase() || "item";
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));

const paths = {
  v20Gate: path.join(readinessDir, "ui-harsh-top-design-referee-v20-total-design-gate.json"),
  v20Census: path.join(readinessDir, "ui-harsh-top-design-referee-v20-total-design-census.json"),
  v21Gate: path.join(readinessDir, "ui-harsh-top-design-referee-v21-readable-default-gate.json"),
  v22Gate: path.join(readinessDir, "ui-harsh-top-design-referee-v22-composition-referee-gate.json"),
  v23Gate: path.join(readinessDir, "ui-harsh-top-design-referee-v23-evidence-glass-referee-gate.json"),
};

const scenarios = [
  { name: "desktop-edge-affordance", viewport: { width: 1365, height: 900, dpr: 2, railVisible: true, isMobile: false, hasTouch: false } },
  { name: "narrow-touch-edge-affordance", viewport: { width: 768, height: 900, dpr: 2, railVisible: true, isMobile: true, hasTouch: true } },
  { name: "mobile-edge-affordance", viewport: { width: 500, height: 844, dpr: 2, railVisible: false, isMobile: true, hasTouch: true } },
  { name: "phone320-edge-affordance", viewport: { width: 320, height: 700, dpr: 3, railVisible: false, isMobile: true, hasTouch: true } },
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
  );
  return targets;
}

function missingInputs() {
  return Object.entries(paths)
    .filter(([, file]) => !fs.existsSync(file))
    .map(([key, file]) => ({ code: "missing_input", key, file }));
}

function semanticAffordanceAudits(v20Census) {
  const controlAudits = [];
  const menuItemAudits = [];
  for (const audit of v20Census.audits?.baseline || []) {
    if (audit.kind !== "control") continue;
    const info = audit.info || {};
    const actionable = ["button", "summary", "a"].includes(info.tag);
    const toolish = /command|tools?|actions?|attach|insert|send|conversation|new/i.test(info.accessible_name || audit.label || "");
    const failures = [];
    if (actionable && toolish && !info.has_svg_or_img) failures.push("tool_action_lacks_icon_or_svg_affordance");
    if (actionable && info.is_icon_only && !info.has_svg_or_img) failures.push("icon_only_action_lacks_vector_or_image");
    if (actionable && String(info.accessible_name || "").trim().length < 2) failures.push("action_lacks_accessible_name");
    controlAudits.push({
      scenario: audit.scenario,
      label: audit.label,
      tag: info.tag,
      text: info.text,
      accessible_name: info.accessible_name,
      is_icon_only: Boolean(info.is_icon_only),
      has_svg_or_img: Boolean(info.has_svg_or_img),
      failures,
    });
  }
  for (const menu of v20Census.audits?.menus || []) {
    for (const item of menu.items || []) {
      const info = item.info || {};
      const failures = [];
      if (!info.has_svg_or_img) failures.push("opened_menu_item_lacks_icon_or_marker");
      if (String(info.accessible_name || "").trim().length < 2) failures.push("opened_menu_item_lacks_accessible_name");
      menuItemAudits.push({
        scenario: menu.scenario,
        group: menu.group,
        target: menu.target,
        label: item.label,
        text: info.text,
        accessible_name: info.accessible_name,
        has_svg_or_img: Boolean(info.has_svg_or_img),
        failures,
      });
    }
  }
  return { controlAudits, menuItemAudits };
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
  if (target.revealSelector) {
    const reveal = page.locator(target.revealSelector).first();
    if (await reveal.count()) await reveal.scrollIntoViewIfNeeded().catch(() => {});
  }
  const trigger = page.locator(target.triggerSelector).first();
  if (!(await trigger.count())) return { opened: false, reason: "missing_trigger" };
  await trigger.scrollIntoViewIfNeeded().catch(() => {});
  await trigger.click({ force: true });
  await page.waitForTimeout(160);
  const panel = page.locator(target.panelSelector).first();
  if (!(await panel.count())) return { opened: false, reason: "missing_panel" };
  const visible = await panel.evaluate((element) => {
    const style = window.getComputedStyle(element);
    const rect = element.getBoundingClientRect();
    return style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity) > 0.01 && rect.width > 1 && rect.height > 1;
  }).catch(() => false);
  return { opened: visible, reason: visible ? null : "panel_not_visible" };
}

async function edgeAudit(page, scenario, target = null) {
  return page.evaluate(({ scenario, target }) => {
    const round = (value) => Number(value.toFixed(3));
    const viewportRect = { left: 0, top: 0, right: window.innerWidth, bottom: window.innerHeight, width: window.innerWidth, height: window.innerHeight };
    const rectObj = (rect) => ({ left: round(rect.left), top: round(rect.top), right: round(rect.right), bottom: round(rect.bottom), width: round(rect.width), height: round(rect.height) });
    const intersect = (a, b) => {
      const left = Math.max(a.left, b.left);
      const top = Math.max(a.top, b.top);
      const right = Math.min(a.right, b.right);
      const bottom = Math.min(a.bottom, b.bottom);
      return { left, top, right, bottom, width: Math.max(0, right - left), height: Math.max(0, bottom - top) };
    };
    const isVisible = (element) => {
      const style = window.getComputedStyle(element);
      const rect = element.getBoundingClientRect();
      return style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity) > 0.01 && rect.width > 1 && rect.height > 1;
    };
    const clippedAudit = (element, container, label, group, minRatio = 0.995) => {
      if (!isVisible(element)) return null;
      const rect = element.getBoundingClientRect();
      const containerRect = container ? intersect(container.getBoundingClientRect(), viewportRect) : viewportRect;
      const visibleRect = intersect(rect, containerRect);
      const area = rect.width * rect.height;
      const visibleArea = visibleRect.width * visibleRect.height;
      const ratio = area > 0 ? visibleArea / area : 0;
      const failures = [];
      if (ratio > 0.025 && ratio < minRatio) failures.push("partial_visible_edge_clipping");
      return {
        scenario,
        target,
        group,
        label,
        clipped_ratio: round(ratio),
        rect: rectObj(rect),
        container_rect: rectObj(containerRect),
        failures,
      };
    };

    const audits = [];
    const railList = document.querySelector(".tg-conversation-list");
    if (!target && railList) {
      document.querySelectorAll(".tg-conversation-list .tg-chat-item").forEach((item, index) => {
        const label = item.getAttribute("data-chat-conversation") || item.getAttribute("aria-label") || `conversation-${index}`;
        const audit = clippedAudit(item, railList, label, "conversation-rail");
        if (audit) audits.push(audit);
      });
    }

    if (target?.panelSelector) {
      const panel = document.querySelector(target.panelSelector);
      if (panel) {
        const panelAudit = clippedAudit(panel, null, target.key, target.group);
        if (panelAudit) audits.push(panelAudit);
        if (!["command-palette"].includes(target.group)) {
          panel.querySelectorAll('[role="menuitem"], button, select, [data-chat-composer-popover-item], .tg-menu-item, .tg-composer-popover__item').forEach((item, index) => {
            const label = item.getAttribute("aria-label") || item.textContent.trim() || `${target.key}-item-${index}`;
            const audit = clippedAudit(item, panel, label, `${target.group}-item`);
            if (audit) audits.push(audit);
          });
        }
      }
    }
    return audits;
  }, { scenario: scenario.name, target });
}

(async () => {
  const failures = missingInputs();
  const inputs = Object.fromEntries(Object.entries(paths).map(([key, file]) => [key, { path: file, sha256: fs.existsSync(file) ? sha256(file) : null }]));
  const v20Census = failures.length ? null : readJson(paths.v20Census);
  const v20Gate = failures.length ? null : readJson(paths.v20Gate);
  const v21Gate = failures.length ? null : readJson(paths.v21Gate);
  const v22Gate = failures.length ? null : readJson(paths.v22Gate);
  const v23Gate = failures.length ? null : readJson(paths.v23Gate);
  if (!failures.length) {
    for (const [name, report] of [["v20", v20Gate], ["v21", v21Gate], ["v22", v22Gate], ["v23", v23Gate]]) {
      if (report.status !== "ready") failures.push({ code: `${name}_gate_not_ready`, status: report.status });
    }
  }

  const semantic = v20Census ? semanticAffordanceAudits(v20Census) : { controlAudits: [], menuItemAudits: [] };
  for (const audit of [...semantic.controlAudits, ...semantic.menuItemAudits]) {
    for (const failure of audit.failures) failures.push({ code: failure, audit });
  }

  const browser = await chromium.launch({ executablePath: chromeBin, headless: true, args: ["--no-sandbox", "--disable-gpu"] });
  const edgeAudits = [];
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
      edgeAudits.push(...await edgeAudit(page, scenario));
      screenshots.push({ scenario: scenario.name, state: "default", ...await screenshot(page, `${scenario.name}-default`) });
      for (const target of targetDefinitions(scenario.viewport)) {
        await closeTransient(page);
        const opened = await openTarget(page, target);
        if (!opened.opened) {
          failures.push({ code: "v24_target_not_opened", scenario: scenario.name, target, reason: opened.reason });
          continue;
        }
        edgeAudits.push(...await edgeAudit(page, scenario, target));
        screenshots.push({ scenario: scenario.name, state: target.key, ...await screenshot(page, `${scenario.name}-${target.key}`) });
      }
      await context.close();
    }
  } finally {
    await browser.close();
  }

  const edgeFailures = edgeAudits.filter((audit) => audit.failures.length > 0);
  for (const audit of edgeFailures) failures.push({ code: "v24_edge_clipping_failure", audit });

  const summary = {
    scenario_count: scenarios.length,
    default_edge_audit_count: edgeAudits.filter((audit) => !audit.target).length,
    opened_edge_audit_count: edgeAudits.filter((audit) => audit.target).length,
    screenshot_count: screenshots.length,
    semantic_control_audit_count: semantic.controlAudits.length,
    semantic_menu_item_audit_count: semantic.menuItemAudits.length,
    edge_failure_count: edgeFailures.length,
    semantic_failure_count: [...semantic.controlAudits, ...semantic.menuItemAudits].filter((audit) => audit.failures.length > 0).length,
    failure_count: failures.length,
    by_group: Object.values(edgeAudits.reduce((acc, audit) => {
      acc[audit.group] ||= { group: audit.group, audit_count: 0, failure_count: 0 };
      acc[audit.group].audit_count += 1;
      if (audit.failures.length) acc[audit.group].failure_count += 1;
      return acc;
    }, {})),
    thresholds: {
      partial_visible_clipped_ratio_min: 0.995,
      partial_visible_lower_noise_floor: 0.025,
      tool_actions_require_icon_or_svg_affordance: true,
      opened_menu_items_require_icon_or_marker: true,
      browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome was used.",
    },
  };

  process.stdout.write(JSON.stringify({
    schema: "hepta-ui-harsh-top-design-referee-v24-edge-affordance-census/v0",
    status: failures.length === 0 ? "ready" : "failed",
    generated_at: new Date().toISOString(),
    readiness_dir: readinessDir,
    base_url: baseUrl,
    summary,
    inputs,
    screenshots,
    edge_audits: edgeAudits,
    semantic_affordance_audits: semantic,
    failures,
  }, null, 2));
})().catch((error) => {
  process.stderr.write(`${error.stack || error}\n`);
  process.exit(1);
});
NODE

node - "$V24_CENSUS_PATH" "$REPORT_PATH" "$V23_REPORT_PATH" "$SKIP_V23" <<'NODE'
const crypto = require("node:crypto");
const fs = require("node:fs");

const [censusPath, reportPath, v23ReportPath, skipV23] = process.argv.slice(2);
const sha256 = (file) => fs.existsSync(file) ? crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex") : null;
const census = JSON.parse(fs.readFileSync(censusPath, "utf8"));
const v23 = fs.existsSync(v23ReportPath) ? JSON.parse(fs.readFileSync(v23ReportPath, "utf8")) : null;
const status = census.status === "ready" && (skipV23 === "1" || v23?.status === "ready") ? "ready" : "failed";
const report = {
  schema: "hepta-ui-harsh-top-design-referee-v24-edge-affordance-gate/v0",
  status,
  generated_at: new Date().toISOString(),
  summary: {
    v23_evidence_glass_referee: v23?.summary?.v23_evidence_glass_referee ?? null,
    v24_edge_affordance_referee: census.summary,
  },
  inputs: {
    v23_evidence_glass_referee: fs.existsSync(v23ReportPath) ? { path: v23ReportPath, sha256: sha256(v23ReportPath), skipped: skipV23 === "1" } : { path: v23ReportPath, sha256: null, skipped: skipV23 === "1" },
    v24_edge_affordance_census: { path: censusPath, sha256: sha256(censusPath) },
  },
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`);
if (status !== "ready") {
  console.error(JSON.stringify(census.summary, null, 2));
  process.exit(1);
}
NODE

echo "Hepta UI harsh top-design referee v24 edge-affordance gate ready: $REPORT_PATH"
