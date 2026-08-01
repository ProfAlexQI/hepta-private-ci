#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

source scripts/lib/hepta-ui-rust-toolchain.sh
source scripts/lib/hepta-control-ui-runtime-fixture.sh
hepta_ui_activate_rust_toolchain

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V25_REPORT_PATH:-}"
V25_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V25_CENSUS_PATH:-}"
V25_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V25_SCREENSHOT_DIR:-}"
V24_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V24_REPORT_PATH:-}"
V24_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V25_V24_LOG:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
SKIP_V24="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V25_SKIP_V24:-0}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v25-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v25-menu-polish-gate.json"
fi
if [[ -z "$V25_CENSUS_PATH" ]]; then
  V25_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v25-menu-polish-census.json"
fi
if [[ -z "$V25_SCREENSHOT_DIR" ]]; then
  V25_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v25-menu-polish-screenshots"
fi
if [[ -z "$V24_REPORT_PATH" ]]; then
  V24_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v24-edge-affordance-gate.json"
fi
if [[ -z "$V24_LOG" ]]; then
  V24_LOG="$READINESS_DIR/v24-edge-affordance-prerequisite.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$V25_SCREENSHOT_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V25_CENSUS_PATH")"

if [[ "$SKIP_V24" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V24_REPORT_PATH="$V24_REPORT_PATH" \
    bash scripts/hepta-ui-harsh-top-design-referee-v24-edge-affordance-gate.sh "$READINESS_DIR" >"$V24_LOG" 2>&1 || {
      echo "v24 edge-affordance prerequisite failed" >&2
      tail -n 180 "$V24_LOG" >&2 || true
      exit 1
    }
fi

if [[ "$(jq -r '.status' "$V24_REPORT_PATH")" != "ready" ]]; then
  echo "v24 edge-affordance prerequisite was not ready: $V24_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7600 7601 7602 7603 7604; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v25 referee" >&2
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
      echo "Hepta Control UI server exited before v25 menu-polish audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$V25_SCREENSHOT_DIR" "$READINESS_DIR" >"$V25_CENSUS_PATH" <<'NODE'
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
  v20Gate: path.join(readinessDir, "ui-harsh-top-design-referee-v20-total-design-gate.json"),
  v20Census: path.join(readinessDir, "ui-harsh-top-design-referee-v20-total-design-census.json"),
  v24Gate: path.join(readinessDir, "ui-harsh-top-design-referee-v24-edge-affordance-gate.json"),
};

const scenarios = [
  { name: "desktop-menu-polish", viewport: { width: 1365, height: 900, dpr: 2, railVisible: true, isMobile: false, hasTouch: false } },
  { name: "narrow-touch-menu-polish", viewport: { width: 768, height: 900, dpr: 2, railVisible: true, isMobile: true, hasTouch: true } },
  { name: "mobile-menu-polish", viewport: { width: 500, height: 844, dpr: 2, railVisible: false, isMobile: true, hasTouch: true } },
  { name: "phone320-menu-polish", viewport: { width: 320, height: 700, dpr: 3, railVisible: false, isMobile: true, hasTouch: true } },
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

async function auditOpenTarget(page, scenario, target) {
  return page.evaluate(({ scenario, target }) => {
    const panel = document.querySelector(target.panelSelector);
    const audits = [];
    const symbolIds = new Set([...document.querySelectorAll("symbol[id]")].map((symbol) => symbol.id));
    const decorationTargets = (item) => [item, ...item.querySelectorAll(":scope :is(strong, small, span, b, code)")];
    const itemNodes = panel ? [...panel.querySelectorAll(target.itemSelector)] : [];
    const panelNodes = panel ? [panel] : [];
    const svgUseNodes = panel ? [...panel.querySelectorAll("svg use")] : [];

    for (const [index, item] of itemNodes.entries()) {
      const style = window.getComputedStyle(item);
      const rect = item.getBoundingClientRect();
      const failures = [];
      const decorationLines = decorationTargets(item).map((node) => ({
        tag: node.tagName.toLowerCase(),
        class_name: node.className || "",
        text: node.textContent.trim().slice(0, 80),
        text_decoration_line: window.getComputedStyle(node).textDecorationLine,
      }));
      if (decorationLines.some((line) => line.text_decoration_line && line.text_decoration_line !== "none")) {
        failures.push("menu_item_uses_browser_link_text_decoration");
      }
      if (!["input", "select", "textarea"].includes(item.tagName.toLowerCase()) && style.cursor !== "pointer") {
        failures.push("small_action_cursor_not_pointer");
      }
      if (rect.width > 1 && item.scrollWidth > item.clientWidth + 1) failures.push("small_action_horizontal_text_overflow");
      if (rect.height > 1 && item.scrollHeight > item.clientHeight + 1) failures.push("small_action_vertical_text_overflow");
      audits.push({
        scenario: scenario.name,
        group: target.group,
        target: target.key,
        kind: "menu-item",
        index,
        text: item.textContent.trim().replace(/\s+/g, " ").slice(0, 140),
        tag: item.tagName.toLowerCase(),
        role: item.getAttribute("role") || "",
        cursor: style.cursor,
        rect: { width: Number(rect.width.toFixed(3)), height: Number(rect.height.toFixed(3)) },
        scroll: { scroll_width: item.scrollWidth, client_width: item.clientWidth, scroll_height: item.scrollHeight, client_height: item.clientHeight },
        decoration_lines: decorationLines,
        failures,
      });
    }

    for (const node of [...panelNodes, ...itemNodes]) {
      const style = window.getComputedStyle(node);
      const failures = [];
      if (!String(style.backdropFilter || "").includes("blur") && !String(style.webkitBackdropFilter || "").includes("blur")) {
        failures.push("polished_glass_surface_lacks_backdrop_blur");
      }
      audits.push({
        scenario: scenario.name,
        group: target.group,
        target: target.key,
        kind: node === panel ? "panel-surface" : "menu-item-surface",
        text: node.textContent.trim().replace(/\s+/g, " ").slice(0, 140),
        tag: node.tagName.toLowerCase(),
        backdrop_filter: style.backdropFilter || style.webkitBackdropFilter || "",
        failures,
      });
    }

    for (const useNode of svgUseNodes) {
      const href = useNode.getAttribute("href") || useNode.getAttribute("xlink:href") || "";
      const id = href.startsWith("#") ? href.slice(1) : "";
      const failures = [];
      if (!id || !symbolIds.has(id)) failures.push("svg_use_references_missing_symbol");
      audits.push({
        scenario: scenario.name,
        group: target.group,
        target: target.key,
        kind: "svg-use",
        href,
        resolved: Boolean(id && symbolIds.has(id)),
        failures,
      });
    }
    return audits;
  }, { scenario, target });
}

(async () => {
  const failures = missingInputs();
  const inputs = Object.fromEntries(Object.entries(paths).map(([key, file]) => [key, { path: file, sha256: fs.existsSync(file) ? sha256(file) : null }]));
  const v20Gate = failures.length ? null : readJson(paths.v20Gate);
  const v20Census = failures.length ? null : readJson(paths.v20Census);
  const v24Gate = failures.length ? null : readJson(paths.v24Gate);
  if (!failures.length) {
    for (const [name, report] of [["v20", v20Gate], ["v24", v24Gate]]) {
      if (report.status !== "ready") failures.push({ code: `${name}_gate_not_ready`, status: report.status });
    }
  }

  const browser = await chromium.launch({ executablePath: chromeBin, headless: true, args: ["--no-sandbox", "--disable-gpu"] });
  const polishAudits = [];
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
      for (const target of targetDefinitions(scenario.viewport)) {
        await closeTransient(page);
        const opened = await openTarget(page, target);
        if (!opened.opened) {
          failures.push({ code: "v25_target_not_opened", scenario: scenario.name, target, reason: opened.reason });
          continue;
        }
        polishAudits.push(...await auditOpenTarget(page, scenario, target));
        screenshots.push({ scenario: scenario.name, state: target.key, ...await screenshot(page, `${scenario.name}-${target.key}`) });
      }
      await context.close();
    }
  } finally {
    await browser.close();
  }

  const auditFailures = polishAudits.filter((audit) => audit.failures.length > 0);
  for (const audit of auditFailures) failures.push({ code: "v25_menu_polish_failure", audit });

  const v20MenuItemCount = v20Census?.audits?.menus?.flatMap((menu) => menu.items || []).length ?? 0;
  const summary = {
    scenario_count: scenarios.length,
    target_count: scenarios.reduce((count, scenario) => count + targetDefinitions(scenario.viewport).length, 0),
    screenshot_count: screenshots.length,
    opened_menu_item_audit_count: polishAudits.filter((audit) => audit.kind === "menu-item").length,
    surface_glass_audit_count: polishAudits.filter((audit) => audit.kind.endsWith("surface")).length,
    svg_use_audit_count: polishAudits.filter((audit) => audit.kind === "svg-use").length,
    text_decoration_failure_count: auditFailures.filter((audit) => audit.failures.includes("menu_item_uses_browser_link_text_decoration")).length,
    cursor_failure_count: auditFailures.filter((audit) => audit.failures.includes("small_action_cursor_not_pointer")).length,
    overflow_failure_count: auditFailures.filter((audit) => audit.failures.some((failure) => failure.includes("overflow"))).length,
    svg_resolution_failure_count: auditFailures.filter((audit) => audit.failures.includes("svg_use_references_missing_symbol")).length,
    backdrop_failure_count: auditFailures.filter((audit) => audit.failures.includes("polished_glass_surface_lacks_backdrop_blur")).length,
    v20_opened_menu_item_count: v20MenuItemCount,
    failure_count: failures.length,
    by_group: Object.values(polishAudits.reduce((acc, audit) => {
      acc[audit.group] ||= { group: audit.group, audit_count: 0, failure_count: 0 };
      acc[audit.group].audit_count += 1;
      if (audit.failures.length) acc[audit.group].failure_count += 1;
      return acc;
    }, {})),
    thresholds: {
      opened_menu_items_may_not_use_browser_link_underlines: true,
      small_action_cursor_must_be_pointer: true,
      menu_item_text_overflow_allowed_px: 1,
      svg_use_references_must_resolve: true,
      panels_and_menu_items_require_backdrop_blur: true,
      browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome was used.",
    },
  };

  process.stdout.write(JSON.stringify({
    schema: "hepta-ui-harsh-top-design-referee-v25-menu-polish-census/v0",
    status: failures.length === 0 ? "ready" : "failed",
    generated_at: new Date().toISOString(),
    readiness_dir: readinessDir,
    base_url: baseUrl,
    summary,
    inputs,
    screenshots,
    polish_audits: polishAudits,
    failures,
  }, null, 2));
})().catch((error) => {
  process.stderr.write(`${error.stack || error}\n`);
  process.exit(1);
});
NODE

node - "$V25_CENSUS_PATH" "$REPORT_PATH" "$V24_REPORT_PATH" "$SKIP_V24" <<'NODE'
const crypto = require("node:crypto");
const fs = require("node:fs");

const [censusPath, reportPath, v24ReportPath, skipV24] = process.argv.slice(2);
const sha256 = (file) => fs.existsSync(file) ? crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex") : null;
const census = JSON.parse(fs.readFileSync(censusPath, "utf8"));
const v24 = fs.existsSync(v24ReportPath) ? JSON.parse(fs.readFileSync(v24ReportPath, "utf8")) : null;
const status = census.status === "ready" && (skipV24 === "1" || v24?.status === "ready") ? "ready" : "failed";
const report = {
  schema: "hepta-ui-harsh-top-design-referee-v25-menu-polish-gate/v0",
  status,
  generated_at: new Date().toISOString(),
  summary: {
    v24_edge_affordance_referee: v24?.summary?.v24_edge_affordance_referee ?? null,
    v25_menu_polish_referee: census.summary,
  },
  inputs: {
    v24_edge_affordance_referee: fs.existsSync(v24ReportPath) ? { path: v24ReportPath, sha256: sha256(v24ReportPath), skipped: skipV24 === "1" } : { path: v24ReportPath, sha256: null, skipped: skipV24 === "1" },
    v25_menu_polish_census: { path: censusPath, sha256: sha256(censusPath) },
  },
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`);
if (status !== "ready") {
  console.error(JSON.stringify(census.summary, null, 2));
  process.exit(1);
}
NODE

echo "Hepta UI harsh top-design referee v25 menu-polish gate ready: $REPORT_PATH"
