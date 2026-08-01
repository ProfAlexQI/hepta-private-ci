#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

source scripts/lib/hepta-ui-rust-toolchain.sh
source scripts/lib/hepta-control-ui-runtime-fixture.sh
hepta_ui_activate_rust_toolchain

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V22_REPORT_PATH:-}"
V22_CENSUS_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V22_CENSUS_PATH:-}"
V22_SCREENSHOT_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V22_SCREENSHOT_DIR:-}"
V21_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V21_REPORT_PATH:-}"
V21_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V22_V21_LOG:-}"
NATIVE_DIR="${HEPTA_NATIVE_FIXTURE_VISUAL_DIR:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v22-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"
SKIP_V21="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V22_SKIP_V21:-0}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v22-composition-referee-gate.json"
fi
if [[ -z "$V22_CENSUS_PATH" ]]; then
  V22_CENSUS_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v22-composition-referee-census.json"
fi
if [[ -z "$V22_SCREENSHOT_DIR" ]]; then
  V22_SCREENSHOT_DIR="$READINESS_DIR/ui-harsh-v22-composition-referee-screenshots"
fi
if [[ -z "$V21_REPORT_PATH" ]]; then
  V21_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v21-readable-default-gate.json"
fi
if [[ -z "$V21_LOG" ]]; then
  V21_LOG="$READINESS_DIR/v21-readable-default-prerequisite.log"
fi
if [[ -z "$NATIVE_DIR" ]]; then
  NATIVE_DIR="$READINESS_DIR/native-fixture"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi

mkdir -p "$READINESS_DIR" "$NATIVE_DIR" "$V22_SCREENSHOT_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$V22_CENSUS_PATH")"

if [[ "$SKIP_V21" != "1" ]]; then
  HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V21_REPORT_PATH="$V21_REPORT_PATH" \
  HEPTA_NATIVE_FIXTURE_VISUAL_DIR="$NATIVE_DIR" \
    bash scripts/hepta-ui-harsh-top-design-referee-v21-readable-default-gate.sh "$READINESS_DIR" >"$V21_LOG" 2>&1 || {
      echo "v21 readable-default prerequisite failed" >&2
      tail -n 180 "$V21_LOG" >&2 || true
      exit 1
    }

  if [[ "$(jq -r '.status' "$V21_REPORT_PATH")" != "ready" ]]; then
    echo "v21 readable-default prerequisite was not ready: $V21_REPORT_PATH" >&2
    exit 1
  fi
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7490 7491 7492 7493 7494; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v22 referee" >&2
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
      echo "Hepta Control UI server exited before v22 composition audit was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$V22_SCREENSHOT_DIR" >"$V22_CENSUS_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, screenshotDir] = process.argv.slice(2);
fs.mkdirSync(screenshotDir, { recursive: true });

const scenarios = [
  { name: "desktop-composition", viewport: { width: 1365, height: 900, dpr: 2, railVisible: true, isMobile: false, hasTouch: false } },
  { name: "narrow-touch-composition", viewport: { width: 768, height: 900, dpr: 2, railVisible: true, isMobile: true, hasTouch: true } },
  { name: "mobile-composition", viewport: { width: 500, height: 844, dpr: 2, railVisible: false, isMobile: true, hasTouch: true } },
  { name: "phone320-composition", viewport: { width: 320, height: 700, dpr: 3, railVisible: false, isMobile: true, hasTouch: true } },
];

const transientPanelSelector = [
  "[data-chat-row-menu-panel]",
  "[data-control-ui-thread-tools-panel]",
  "[data-control-ui-composer-tools-panel]",
  "[data-chat-composer-popover]",
  "#command-palette .command-palette",
].join(",");

const sha256 = (file) => crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase() || "item";
const round = (value, digits = 3) => Number(value.toFixed(digits));

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

async function visibleTransientPanels(page) {
  return page.evaluate((selector) => {
    const visible = (element) => {
      const rect = element.getBoundingClientRect();
      const style = getComputedStyle(element);
      return rect.width > 0 && rect.height > 0 && style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity || "1") > 0.01;
    };
    return [...document.querySelectorAll(selector)].filter(visible).map((element) => {
      const rect = element.getBoundingClientRect();
      return {
        label: element.getAttribute("data-chat-row-menu-panel")
          || element.getAttribute("data-chat-composer-popover")
          || element.getAttribute("data-control-ui-thread-tools-panel")
          || element.getAttribute("data-control-ui-composer-tools-panel")
          || element.getAttribute("data-control-ui-command-palette-surface")
          || element.id
          || String(element.className),
        box: {
          left: Number(rect.left.toFixed(3)),
          top: Number(rect.top.toFixed(3)),
          right: Number(rect.right.toFixed(3)),
          bottom: Number(rect.bottom.toFixed(3)),
          width: Number(rect.width.toFixed(3)),
          height: Number(rect.height.toFixed(3)),
        },
      };
    });
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

async function auditDefault(page, scenario) {
  const screenshotPath = path.join(screenshotDir, `${scenario.name}-default.png`);
  await page.goto(baseUrl, { waitUntil: "domcontentloaded" });
  await page.waitForSelector(".telegram-chat-shell[data-control-ui-harsh-referee]", { timeout: 30000 });
  await page.waitForTimeout(350);
  await page.screenshot({ path: screenshotPath, fullPage: false });

  const audit = await page.evaluate((scenarioName) => {
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
    const intersects = (a, b) => {
      if (!a || !b) return { width: 0, height: 0, area: 0 };
      const left = Math.max(a.left, b.left);
      const right = Math.min(a.right, b.right);
      const top = Math.max(a.top, b.top);
      const bottom = Math.min(a.bottom, b.bottom);
      const width = Math.max(0, right - left);
      const height = Math.max(0, bottom - top);
      return { left, right, top, bottom, width, height, area: width * height };
    };
    const isVisible = (element) => {
      if (!element) return false;
      const rect = rectFor(element);
      const style = getComputedStyle(element);
      return rect && rect.width > 0 && rect.height > 0 && style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity || "1") > 0.01;
    };
    const rail = document.querySelector(".tg-conversation-rail");
    const panel = document.querySelector(".tg-thread-panel");
    const header = document.querySelector(".tg-thread-header");
    const headerMain = document.querySelector(".tg-thread-header__main");
    const headerTitle = document.querySelector(".tg-thread-header__main h2");
    const headerStatus = document.querySelector(".tg-thread-status");
    const headerSubtitle = document.querySelector(".tg-thread-header p");
    const compose = document.querySelector(".tg-compose-wrap");
    const railRect = rectFor(rail);
    const panelRect = rectFor(panel);
    const headerRect = rectFor(header);
    const headerMainRect = rectFor(headerMain);
    const headerTitleRect = rectFor(headerTitle);
    const headerStatusRect = rectFor(headerStatus);
    const subtitleRect = rectFor(headerSubtitle);
    const composeRect = rectFor(compose);
    const railVisible = isVisible(rail) && railRect.width > 24 && railRect.height > 24;
    const railPanelGap = railVisible && panelRect ? panelRect.top - railRect.bottom : null;
    const headerMainStatusGap = headerTitleRect && headerStatusRect ? headerStatusRect.left - headerTitleRect.right : null;
    const panelViewportGap = panelRect ? Math.min(panelRect.left, panelRect.top, innerWidth - panelRect.right, innerHeight - panelRect.bottom) : null;
    const composeViewportGap = composeRect ? Math.min(composeRect.left, innerWidth - composeRect.right, innerHeight - composeRect.bottom) : null;

    if (innerWidth >= 701 && innerWidth <= 980 && (!railVisible || railPanelGap < 12)) {
      failures.push({ code: "narrow_rail_thread_gap_below_12px", rail_panel_gap: railPanelGap, rail_rect: roundedRect(railRect), panel_rect: roundedRect(panelRect) });
    }
    if (innerWidth <= 500 && isVisible(headerSubtitle)) {
      failures.push({ code: "mobile_header_duplicate_subtitle_visible", subtitle_rect: roundedRect(subtitleRect), text: headerSubtitle.textContent.trim() });
    }
    if (innerWidth <= 500 && (headerMainStatusGap === null || headerMainStatusGap < 12)) {
      failures.push({ code: "mobile_header_title_status_gap_below_12px", header_main_status_gap: headerMainStatusGap, title_rect: roundedRect(headerTitleRect), main_rect: roundedRect(headerMainRect), status_rect: roundedRect(headerStatusRect) });
    }
    if (panelViewportGap !== null && panelViewportGap < 6) {
      failures.push({ code: "thread_panel_too_close_to_viewport_edge", panel_viewport_gap: panelViewportGap, panel_rect: roundedRect(panelRect) });
    }
    if (composeViewportGap !== null && composeViewportGap < 6) {
      failures.push({ code: "composer_too_close_to_viewport_edge", compose_viewport_gap: composeViewportGap, compose_rect: roundedRect(composeRect) });
    }
    if (headerRect && panelRect && intersects(headerRect, panelRect).area < headerRect.width * headerRect.height * 0.98) {
      failures.push({ code: "header_not_visually_contained_in_thread_panel", header_rect: roundedRect(headerRect), panel_rect: roundedRect(panelRect) });
    }

    return {
      scenario: scenarioName,
      viewport: { width: innerWidth, height: innerHeight, device_pixel_ratio: devicePixelRatio },
      rail_visible: railVisible,
      rail_rect: roundedRect(railRect),
      panel_rect: roundedRect(panelRect),
      header_rect: roundedRect(headerRect),
      header_main_rect: roundedRect(headerMainRect),
      header_title_rect: roundedRect(headerTitleRect),
      header_status_rect: roundedRect(headerStatusRect),
      header_subtitle_rect: roundedRect(subtitleRect),
      compose_rect: roundedRect(composeRect),
      rail_panel_gap: railPanelGap === null ? null : Number(railPanelGap.toFixed(3)),
      header_main_status_gap: headerMainStatusGap === null ? null : Number(headerMainStatusGap.toFixed(3)),
      panel_viewport_gap: panelViewportGap === null ? null : Number(panelViewportGap.toFixed(3)),
      compose_viewport_gap: composeViewportGap === null ? null : Number(composeViewportGap.toFixed(3)),
      failures,
    };
  }, scenario.name);

  return {
    ...audit,
    screenshot_path: screenshotPath,
    screenshot_sha256: sha256(screenshotPath),
  };
}

async function auditOpenedPanel(page, scenario, target) {
  await page.goto(baseUrl, { waitUntil: "domcontentloaded" });
  await page.waitForSelector(".telegram-chat-shell[data-control-ui-harsh-referee]", { timeout: 30000 });
  await page.waitForTimeout(220);
  const trigger = await openTarget(page, scenario, target);
  const screenshotPath = path.join(screenshotDir, `${scenario.name}-opened-${sanitize(target.key)}.png`);
  await page.screenshot({ path: screenshotPath, fullPage: false });

  const triggerBox = await boxFor(trigger);
  const panel = page.locator(target.panelSelector).first();
  const panelBox = await boxFor(panel);
  const visiblePanels = await visibleTransientPanels(page);
  const textOcclusions = await page.evaluate((panelSelector) => {
    const panel = document.querySelector(panelSelector);
    if (!panel) return [];
    const panelRect = panel.getBoundingClientRect();
    const intersections = [];
    const visible = (element) => {
      if (!element) return false;
      const rect = element.getBoundingClientRect();
      const style = getComputedStyle(element);
      return rect.width > 0 && rect.height > 0 && style.display !== "none" && style.visibility !== "hidden" && Number(style.opacity || "1") > 0.01;
    };
    const intersect = (rect) => {
      const left = Math.max(rect.left, panelRect.left);
      const right = Math.min(rect.right, panelRect.right);
      const top = Math.max(rect.top, panelRect.top);
      const bottom = Math.min(rect.bottom, panelRect.bottom);
      const width = Math.max(0, right - left);
      const height = Math.max(0, bottom - top);
      return { left, right, top, bottom, width, height, area: width * height };
    };
    const walker = document.createTreeWalker(document.querySelector(".tg-thread") || document.body, NodeFilter.SHOW_TEXT, {
      acceptNode(node) {
        const text = (node.nodeValue || "").replace(/\s+/g, " ").trim();
        if (!text) return NodeFilter.FILTER_REJECT;
        const parent = node.parentElement;
        if (!parent || !parent.closest(".tg-bubble p")) return NodeFilter.FILTER_REJECT;
        if (!visible(parent)) return NodeFilter.FILTER_REJECT;
        return NodeFilter.FILTER_ACCEPT;
      },
    });
    let node;
    while ((node = walker.nextNode())) {
      const range = document.createRange();
      range.selectNodeContents(node);
      const text = (node.nodeValue || "").replace(/\s+/g, " ").trim();
      for (const rect of [...range.getClientRects()].filter((r) => r.width >= 2 && r.height >= 6)) {
        const hit = intersect(rect);
        if (hit.area <= 2) continue;
        const x = Math.max(1, Math.min(innerWidth - 1, hit.left + hit.width / 2));
        const y = Math.max(1, Math.min(innerHeight - 1, hit.top + hit.height / 2));
        const top = document.elementFromPoint(x, y);
        const panelOwnsTop = top === panel || panel.contains(top);
        if (panelOwnsTop) {
          intersections.push({
            text: text.slice(0, 120),
            intersection: {
              left: Number(hit.left.toFixed(2)),
              top: Number(hit.top.toFixed(2)),
              right: Number(hit.right.toFixed(2)),
              bottom: Number(hit.bottom.toFixed(2)),
              width: Number(hit.width.toFixed(2)),
              height: Number(hit.height.toFixed(2)),
              area: Number(hit.area.toFixed(2)),
            },
          });
        }
      }
      range.detach();
    }
    return intersections;
  }, target.panelSelector);

  const failures = [];
  if (!triggerBox || triggerBox.width < 43.5 || triggerBox.height < 43.5) failures.push("trigger_below_44x44");
  if (!panelBox || panelBox.width < 120 || panelBox.height < 44) failures.push("opened_panel_too_small_or_missing");
  if (panelBox && clippedRatio(panelBox, scenario.viewport) < 0.995) failures.push("opened_panel_clipped_by_viewport");
  if (visiblePanels.length !== 1) failures.push(`opened_visible_panel_count_${visiblePanels.length}`);
  const modalOverlayGroups = new Set(["command-palette", "composer-popover"]);
  if (textOcclusions.length > 0 && !modalOverlayGroups.has(target.group)) {
    failures.push(`opened_panel_occludes_message_text_${textOcclusions.length}`);
  }

  await page.keyboard.press("Escape").catch(() => {});
  await page.waitForTimeout(160);
  const afterClosePanels = await visibleTransientPanels(page);
  if (afterClosePanels.length !== 0) failures.push(`escape_residual_panel_count_${afterClosePanels.length}`);

  return {
    scenario: scenario.name,
    target: target.key,
    group: target.group,
    trigger_box: roundedBox(triggerBox),
    panel_box: roundedBox(panelBox),
    panel_clipped_ratio: panelBox ? round(clippedRatio(panelBox, scenario.viewport), 4) : 0,
    visible_panels_after_open: visiblePanels,
    visible_panels_after_escape: afterClosePanels,
    text_occlusions: textOcclusions,
    screenshot_path: screenshotPath,
    screenshot_sha256: sha256(screenshotPath),
    failures,
    ready: failures.length === 0,
  };
}

function summarizeDefault(audits) {
  return audits.map((audit) => ({
    scenario: audit.scenario,
    rail_panel_gap: audit.rail_panel_gap,
    header_main_status_gap: audit.header_main_status_gap,
    failure_count: audit.failures.length,
  }));
}

function summarizeOpened(audits) {
  return Object.values(audits.reduce((acc, audit) => {
    acc[audit.group] ||= { group: audit.group, opened_panel_audit_count: 0, text_occlusion_count: 0, failure_count: 0 };
    acc[audit.group].opened_panel_audit_count += 1;
    acc[audit.group].text_occlusion_count += audit.text_occlusions.length;
    acc[audit.group].failure_count += audit.failures.length;
    return acc;
  }, {}));
}

(async () => {
  const browser = await chromium.launch({
    executablePath: chromeBin,
    headless: true,
    args: ["--disable-gpu", "--no-sandbox", "--font-render-hinting=none"],
  });
  const defaultAudits = [];
  const openedPanelAudits = [];
  try {
    for (const scenario of scenarios) {
      const context = await browser.newContext({
        viewport: { width: scenario.viewport.width, height: scenario.viewport.height },
        deviceScaleFactor: scenario.viewport.dpr,
        isMobile: scenario.viewport.isMobile,
        hasTouch: scenario.viewport.hasTouch,
      });
      const page = await context.newPage();
      defaultAudits.push(await auditDefault(page, scenario));
      for (const target of targetDefinitions(scenario.viewport)) {
        openedPanelAudits.push(await auditOpenedPanel(page, scenario, target));
      }
      await context.close();
    }
  } finally {
    await browser.close();
  }

  const failureCount = defaultAudits.reduce((sum, audit) => sum + audit.failures.length, 0)
    + openedPanelAudits.reduce((sum, audit) => sum + audit.failures.length, 0);
  const summary = {
    scenario_count: scenarios.length,
    default_composition_audit_count: defaultAudits.length,
    opened_panel_audit_count: openedPanelAudits.length,
    screenshot_count: defaultAudits.length + openedPanelAudits.length,
    text_occlusion_count: openedPanelAudits.reduce((sum, audit) => sum + audit.text_occlusions.length, 0),
    failure_count: failureCount,
    default_by_scenario: summarizeDefault(defaultAudits),
    opened_panel_by_group: summarizeOpened(openedPanelAudits),
    thresholds: {
      narrow_rail_thread_gap_min_px: 12,
      mobile_header_main_status_gap_min_px: 12,
      mobile_header_duplicate_subtitle_visible: false,
      panel_viewport_gap_min_px: 6,
      composer_viewport_gap_min_px: 6,
      opened_panel_clipped_ratio_min: 0.995,
      opened_visible_transient_panel_count: 1,
      escape_visible_transient_panel_count: 0,
      opened_panel_text_occlusion_allowed: false,
      opened_panel_text_occlusion_modal_overlay_groups: ["command-palette", "composer-popover"],
      browser_note: "Browser plugin unavailable in this run; regular Playwright with local Chrome was used.",
    },
  };

  process.stdout.write(JSON.stringify({
    schema: "hepta-ui-harsh-top-design-referee-v22-composition-referee-census/v1",
    status: failureCount === 0 ? "ready" : "failed",
    generated_at: new Date().toISOString(),
    base_url: baseUrl,
    screenshot_dir: screenshotDir,
    summary,
    default_audits: defaultAudits,
    opened_panel_audits: openedPanelAudits,
  }, null, 2));
})();
NODE

node - "$V22_CENSUS_PATH" "$REPORT_PATH" "$V21_REPORT_PATH" "$SKIP_V21" <<'NODE'
const crypto = require("node:crypto");
const fs = require("node:fs");

const [censusPath, reportPath, v21ReportPath, skipV21] = process.argv.slice(2);
const sha256 = (file) => fs.existsSync(file) ? crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex") : null;
const readJson = (file) => JSON.parse(fs.readFileSync(file, "utf8"));
const census = readJson(censusPath);
const v21 = fs.existsSync(v21ReportPath) ? readJson(v21ReportPath) : null;
const failureCount = census.summary?.failure_count ?? 1;
const status = failureCount === 0 && (skipV21 === "1" || v21?.status === "ready") ? "ready" : "failed";
const report = {
  schema: "hepta-ui-harsh-top-design-referee-v22-composition-referee-gate/v1",
  status,
  generated_at: new Date().toISOString(),
  summary: {
    v21_readable_default: v21?.summary?.v21_readable_default ?? null,
    v22_composition_referee: census.summary,
  },
  inputs: {
    v21_readable_default: fs.existsSync(v21ReportPath) ? { path: v21ReportPath, sha256: sha256(v21ReportPath), skipped: skipV21 === "1" } : { path: v21ReportPath, sha256: null, skipped: skipV21 === "1" },
    v22_composition_referee_census: { path: censusPath, sha256: sha256(censusPath) },
  },
};
fs.writeFileSync(reportPath, `${JSON.stringify(report, null, 2)}\n`);
if (status !== "ready") {
  console.error(JSON.stringify(report.summary.v22_composition_referee, null, 2));
  process.exit(1);
}
NODE

echo "Hepta UI harsh top-design referee v22 composition-referee gate ready: $REPORT_PATH"
