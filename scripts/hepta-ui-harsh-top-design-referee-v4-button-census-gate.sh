#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V4_REPORT_PATH:-}"
V3_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V3_REPORT_PATH:-}"
BUTTON_CENSUS_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V4_BUTTON_CENSUS_REPORT_PATH:-}"
BUTTON_CENSUS_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V4_BUTTON_CENSUS_DIR:-}"
NATIVE_REPORT_PATH="${HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v4-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"
V3_LOG="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V4_V3_LOG:-}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v4-button-census-gate.json"
fi
if [[ -z "$V3_REPORT_PATH" ]]; then
  V3_REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v3-visual-matrix-gate.json"
fi
if [[ -z "$BUTTON_CENSUS_REPORT_PATH" ]]; then
  BUTTON_CENSUS_REPORT_PATH="$READINESS_DIR/control-ui-v4-button-census.json"
fi
if [[ -z "$BUTTON_CENSUS_DIR" ]]; then
  BUTTON_CENSUS_DIR="$READINESS_DIR/control-ui-v4-button-census"
fi
if [[ -z "$NATIVE_REPORT_PATH" ]]; then
  NATIVE_REPORT_PATH="$READINESS_DIR/native-fixture/native-fixture-visual-smoke.json"
fi
if [[ -z "$V3_LOG" ]]; then
  V3_LOG="$READINESS_DIR/v3-visual-matrix.log"
fi
if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 1
fi
if [[ ! -s "$NATIVE_REPORT_PATH" ]]; then
  echo "missing native fixture visual smoke report: $NATIVE_REPORT_PATH" >&2
  exit 1
fi
jq empty "$NATIVE_REPORT_PATH" >/dev/null

mkdir -p "$READINESS_DIR" "$BUTTON_CENSUS_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$BUTTON_CENSUS_REPORT_PATH")"

HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V3_REPORT_PATH="$V3_REPORT_PATH" \
HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH="$NATIVE_REPORT_PATH" \
  scripts/hepta-ui-harsh-top-design-referee-v3-visual-matrix-gate.sh "$READINESS_DIR" >"$V3_LOG" 2>&1 || {
    echo "v3 visual matrix prerequisite failed" >&2
    tail -n 120 "$V3_LOG" >&2 || true
    exit 1
  }

if [[ "$(jq -r '.status' "$V3_REPORT_PATH")" != "ready" ]]; then
  echo "v3 visual matrix prerequisite was not ready: $V3_REPORT_PATH" >&2
  exit 1
fi

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7384 7385 7386 7387 7388; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v4 referee" >&2
  exit 1
fi

BASE_URL="http://${BIND_ADDR}"
server_pid=""
tmp_report="$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v4-final.XXXXXX")"

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
  rm -f "$tmp_report"
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
      echo "Hepta Control UI server exited before v4 button census was ready" >&2
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

node - "$CHROME_BIN" "$BASE_URL/" "$BUTTON_CENSUS_DIR" >"$BUTTON_CENSUS_REPORT_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, censusDir] = process.argv.slice(2);
const viewports = [
  { name: "desktop", width: 1365, height: 900, railVisible: true },
  { name: "narrow", width: 768, height: 900, railVisible: true },
  { name: "mobile", width: 500, height: 844, railVisible: false },
  { name: "phone320", width: 320, height: 844, railVisible: false },
];

const sanitize = (value) => String(value).replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase();

async function main() {
  const browser = await chromium.launch({
    headless: true,
    executablePath: chromeBin,
    args: [
      "--disable-background-networking",
      "--disable-component-update",
      "--disable-default-apps",
      "--disable-extensions",
      "--disable-sync",
      "--hide-scrollbars",
      "--no-default-browser-check",
      "--no-first-run",
    ],
  });

  const viewportResults = [];
  const screenshots = [];

  for (const viewport of viewports) {
    const page = await browser.newPage({
      viewport: { width: viewport.width, height: viewport.height },
      deviceScaleFactor: 1,
    });
    const states = [
      { key: "default", group: "default", open: { type: "default" } },
    ];
    if (viewport.railVisible) {
      for (const key of ["ui-chat-agent", "task-queue", "operator-plane"]) {
        states.push({ key: `row-menu-${key}`, group: "row-menu", open: { type: "row-menu", key } });
      }
    }
    states.push(
      { key: "thread-tools", group: "thread-tools", open: { type: "thread-tools" } },
      { key: "composer-tools", group: "composer-tools", open: { type: "composer-tools" } },
      { key: "composer-popover-artifact", group: "composer-popover", open: { type: "composer-popover", key: "artifact" } },
      { key: "composer-popover-command", group: "composer-popover", open: { type: "composer-popover", key: "command" } },
      { key: "command-palette", group: "command-palette", open: { type: "command-palette" } },
    );

    const stateResults = [];
    for (const state of states) {
      await page.goto(baseUrl, { waitUntil: "networkidle" });
      await page.waitForTimeout(140);
      await openState(page, state.open);
      await page.waitForTimeout(140);

      const screenshot = await capture(page, viewport, state.key, censusDir);
      screenshots.push(screenshot);

      const audit = await page.evaluate((stateKey) => {
        const interactiveSelector = [
          "button",
          "a[href]",
          "summary",
          "input",
          "textarea",
          "select",
          "[role='button']",
          "[role='menuitem']",
          "[tabindex]:not([tabindex='-1'])",
        ].join(",");
        const moduleSelector = [
          ".tg-conversation-rail",
          ".tg-thread-panel",
          ".tg-thread-header",
          ".tg-compose-wrap",
          ".tg-compose-bar",
          ".tg-chat-item",
          ".tg-bubble",
          ".tg-row-action-popover",
          ".tg-thread-command-menu__panel",
          ".tg-composer-popover",
          ".command-palette",
        ].join(",");
        const insideOverflowClips = (element, centerX, centerY) => {
          for (let current = element.parentElement; current && current !== document.body; current = current.parentElement) {
            const style = getComputedStyle(current);
            const overflow = `${style.overflow} ${style.overflowX} ${style.overflowY}`;
            if (/(auto|scroll|hidden|clip)/.test(overflow)) {
              const rect = current.getBoundingClientRect();
              if (centerX < rect.left || centerX > rect.right || centerY < rect.top || centerY > rect.bottom) {
                return false;
              }
            }
          }
          return true;
        };
        const visible = (element) => {
          if (!element) return false;
          const rect = element.getBoundingClientRect();
          const style = getComputedStyle(element);
          const centerX = rect.left + rect.width / 2;
          const centerY = rect.top + rect.height / 2;
          return style.display !== "none"
            && style.visibility !== "hidden"
            && Number(style.opacity || 1) > 0
            && rect.width > 1
            && rect.height > 1
            && rect.right > 0
            && rect.bottom > 0
            && rect.left < window.innerWidth
            && rect.top < window.innerHeight
            && centerX >= 0
            && centerX <= window.innerWidth
            && centerY >= 0
            && centerY <= window.innerHeight
            && insideOverflowClips(element, centerX, centerY);
        };
        const rectOf = (element) => {
          const rect = element.getBoundingClientRect();
          return {
            left: Math.round(rect.left),
            top: Math.round(rect.top),
            right: Math.round(rect.right),
            bottom: Math.round(rect.bottom),
            width: Math.round(rect.width),
            height: Math.round(rect.height),
          };
        };
        const textOf = (element) => {
          const collect = (node) => {
            if (node.nodeType === Node.TEXT_NODE) return node.textContent || "";
            if (node.nodeType !== Node.ELEMENT_NODE) return "";
            if (node.matches("svg, svg *, .sr-only")) return "";
            return Array.from(node.childNodes).map(collect).join("");
          };
          return collect(element).replace(/\s+/g, " ").trim();
        };
        const parseColor = (value) => {
          const match = String(value || "").match(/rgba?\(([^)]+)\)/);
          if (!match) return null;
          const parts = (match[1].match(/[0-9.]+/g) || []).map((part) => Number.parseFloat(part));
          if (parts.length < 3 || parts.slice(0, 3).some((part) => Number.isNaN(part))) return null;
          return { r: parts[0], g: parts[1], b: parts[2], a: parts.length >= 4 && !Number.isNaN(parts[3]) ? parts[3] : 1 };
        };
        const blend = (fg, bg) => {
          const alpha = Math.max(0, Math.min(1, fg?.a ?? 1));
          return {
            r: (fg.r * alpha) + (bg.r * (1 - alpha)),
            g: (fg.g * alpha) + (bg.g * (1 - alpha)),
            b: (fg.b * alpha) + (bg.b * (1 - alpha)),
            a: 1,
          };
        };
        const effectiveBackground = (node) => {
          let color = { r: 245, g: 248, b: 250, a: 1 };
          const stack = [];
          for (let current = node; current && current.nodeType === Node.ELEMENT_NODE; current = current.parentElement) {
            const parsed = parseColor(getComputedStyle(current).backgroundColor);
            if (parsed && parsed.a > 0) stack.push(parsed);
          }
          stack.reverse().forEach((item) => {
            color = blend(item, color);
          });
          return color;
        };
        const luminance = (color) => {
          const channel = (value) => {
            const normalized = Math.max(0, Math.min(255, value)) / 255;
            return normalized <= 0.03928 ? normalized / 12.92 : ((normalized + 0.055) / 1.055) ** 2.4;
          };
          return (0.2126 * channel(color.r)) + (0.7152 * channel(color.g)) + (0.0722 * channel(color.b));
        };
        const contrast = (a, b) => {
          const la = luminance(a);
          const lb = luminance(b);
          return (Math.max(la, lb) + 0.05) / (Math.min(la, lb) + 0.05);
        };
        const unobscuredAt = (element, x, y) => {
          if (x < 0 || y < 0 || x > window.innerWidth || y > window.innerHeight) return false;
          const hit = document.elementFromPoint(x, y);
          return hit === element || element.contains(hit) || Boolean(hit && hit.contains(element));
        };
        const visuallyUnobscured = (element) => {
          const rect = element.getBoundingClientRect();
          return unobscuredAt(element, rect.left + rect.width / 2, rect.top + rect.height / 2);
        };
        const visuallyUnobscuredSurface = (element) => {
          const rect = element.getBoundingClientRect();
          const insetX = Math.max(4, Math.min(10, rect.width / 5));
          const insetY = Math.max(4, Math.min(10, rect.height / 5));
          return [
            [rect.left + rect.width / 2, rect.top + rect.height / 2],
            [rect.left + insetX, rect.top + insetY],
            [rect.right - insetX, rect.top + insetY],
            [rect.left + insetX, rect.bottom - insetY],
            [rect.right - insetX, rect.bottom - insetY],
          ].every(([x, y]) => unobscuredAt(element, x, y));
        };
        const visibleNodes = (selector) => Array.from(document.querySelectorAll(selector)).filter(visible);
        const hasSvgIcon = (element) => Boolean(element?.querySelector("svg use[href^='#hepta-icon-']"));
        const selectorOf = (element) => {
          const parts = [];
          if (element.id) parts.push(`#${element.id}`);
          if (element.className && typeof element.className === "string") {
            parts.push(...element.className.split(/\s+/).filter(Boolean).slice(0, 3).map((item) => `.${item}`));
          }
          for (const attr of [
            "data-control-ui-icon-button",
            "data-control-ui-menu-trigger",
            "data-control-ui-menu-item",
            "data-chat-row-menu-toggle",
            "data-chat-row-menu-item",
            "data-chat-composer-popover-toggle",
            "data-chat-composer-picker-item",
            "data-control-ui-command-palette-result",
            "data-chat-folder",
            "data-chat-conversation",
          ]) {
            const value = element.getAttribute(attr);
            if (value) parts.push(`[${attr}='${value}']`);
          }
          return parts.join("") || element.tagName.toLowerCase();
        };
        const describe = (element) => {
          const tag = element.tagName.toLowerCase();
          const role = element.getAttribute("role") || "";
          const text = textOf(element);
          const aria = element.getAttribute("aria-label") || "";
          const title = element.getAttribute("title") || "";
          const placeholder = element.getAttribute("placeholder") || "";
          const label = aria || title || placeholder || text;
          const rect = rectOf(element);
          const style = getComputedStyle(element);
          const bg = effectiveBackground(element);
          const fg = parseColor(style.color);
          const luma = luminance(bg);
          const ratio = fg ? contrast(fg, bg) : 0;
          const buttonLike = tag === "button" || tag === "summary" || role === "button" || role === "menuitem" || element.matches("[data-control-ui-icon-button],[data-control-ui-menu-trigger='icon'],.tg-icon-action,.tg-row-menu-toggle,.tg-compose-icon,.tg-send-button,.tg-menu-item,.tg-row-action,.tg-composer-popover__item");
          const fieldLike = tag === "input" || tag === "textarea" || tag === "select";
          const linkLike = tag === "a";
          const iconOnly = !fieldLike && (text.length === 0 || Boolean(element.matches("[data-control-ui-icon-button],[data-control-ui-menu-trigger='icon'],.tg-icon-action,.tg-row-menu-toggle,.tg-compose-icon,.tg-send-button")));
          const titleAriaReady = !buttonLike || (aria.length > 0 && title.length > 0 && aria === title);
          const labelReady = label.length > 0;
          const sizeReady = buttonLike
            ? rect.width >= 44 && rect.height >= 44
            : fieldLike
              ? rect.width >= (tag === "select" ? 44 : 120) && rect.height >= 32
              : linkLike
                ? rect.width >= 44 && rect.height >= 32
                : rect.width >= 24 && rect.height >= 24;
          const lightGlassReady = luma >= 0.68 && luma <= 0.99;
          const ancestorGlass = element.closest(".tg-compose-bar,.tg-search-shell,.command-palette__input-row,.tg-composer-popover,.tg-thread-command-menu__panel,.tg-row-action-popover");
          const ancestorStyle = ancestorGlass ? getComputedStyle(ancestorGlass) : null;
          const glassTreatmentReady = String(style.backdropFilter || style.webkitBackdropFilter || "").includes("blur(")
            || (style.boxShadow && style.boxShadow !== "none")
            || (style.borderTopColor && style.borderTopStyle !== "none")
            || Boolean(ancestorStyle && (
              String(ancestorStyle.backdropFilter || ancestorStyle.webkitBackdropFilter || "").includes("blur(")
              || (ancestorStyle.boxShadow && ancestorStyle.boxShadow !== "none")
            ));
          const readableReady = ratio >= 4.5;
          const iconReady = fieldLike || !iconOnly || hasSvgIcon(element);
          const inViewport = rect.left >= -1 && rect.top >= -1 && rect.right <= window.innerWidth + 1 && rect.bottom <= window.innerHeight + 1;
          return {
            selector: selectorOf(element),
            tag,
            role,
            label,
            aria_label: aria,
            title,
            placeholder,
            text,
            icon_only: iconOnly,
            button_like: buttonLike,
            field_like: fieldLike,
            link_like: linkLike,
            has_svg_icon: hasSvgIcon(element),
            title_aria_ready: titleAriaReady,
            label_ready: labelReady,
            size_ready: sizeReady,
            readable_ready: readableReady,
            contrast_ratio: Number(ratio.toFixed(2)),
            light_glass_ready: lightGlassReady,
            effective_luminance: Number(luma.toFixed(3)),
            glass_treatment_ready: glassTreatmentReady,
            in_viewport: inViewport,
            visually_unobscured: visuallyUnobscured(element),
            icon_ready: iconReady,
            ...rect,
            ready: titleAriaReady
              && labelReady
              && sizeReady
              && readableReady
            && lightGlassReady
            && glassTreatmentReady
            && inViewport
              && iconReady,
          };
        };
        const modules = visibleNodes(moduleSelector).map((element) => {
          const rect = rectOf(element);
          const style = getComputedStyle(element);
          const bg = effectiveBackground(element);
          const luma = luminance(bg);
          const borderRadius = Number.parseFloat(style.borderTopLeftRadius || "0") || 0;
          const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
          const floatingSurface = element.matches(".tg-row-action-popover,.tg-thread-command-menu__panel,.tg-composer-popover,.command-palette");
          const surfaceUnobscured = visuallyUnobscuredSurface(element);
          const ready = rect.width >= 44
            && rect.height >= 44
            && rect.left >= -1
            && rect.top >= -1
            && rect.right <= window.innerWidth + 1
            && rect.bottom <= window.innerHeight + 1
            && luma >= 0.62
            && luma <= 0.99
            && (String(backdrop).includes("blur(") || (style.boxShadow && style.boxShadow !== "none"))
            && borderRadius >= 8
            && (!floatingSurface || surfaceUnobscured);
          return {
            selector: selectorOf(element),
            role: element.getAttribute("role") || "",
            floating_surface: floatingSurface,
            light_glass_ready: luma >= 0.62 && luma <= 0.99,
            effective_luminance: Number(luma.toFixed(3)),
            backdrop_filter: backdrop,
            box_shadow_present: Boolean(style.boxShadow && style.boxShadow !== "none"),
            border_radius: borderRadius,
            visually_unobscured: surfaceUnobscured,
            ready,
            ...rect,
          };
        });
        const interactive = visibleNodes(interactiveSelector)
          .filter((element, index, array) => array.indexOf(element) === index)
          .map(describe);
        const failures = interactive.filter((item) => !item.ready);
        const moduleFailures = modules.filter((item) => !item.ready);
        const horizontalOverflowFree = document.documentElement.scrollWidth - window.innerWidth <= 1
          && document.body.scrollWidth - window.innerWidth <= 1;
        return {
          state_key: stateKey,
          viewport: `${window.innerWidth}x${window.innerHeight}`,
          horizontal_overflow_free: horizontalOverflowFree,
          interactive_count: interactive.length,
          button_like_count: interactive.filter((item) => item.button_like).length,
          field_like_count: interactive.filter((item) => item.field_like).length,
          module_count: modules.length,
          failures,
          module_failures: moduleFailures,
          interactive_sample: interactive.slice(0, 80),
          module_sample: modules.slice(0, 80),
          ready: horizontalOverflowFree && failures.length === 0 && moduleFailures.length === 0,
        };
      }, state.key);
      stateResults.push({ ...audit, group: state.group, screenshot });
    }

    viewportResults.push({
      name: viewport.name,
      viewport: { width: viewport.width, height: viewport.height },
      state_count: states.length,
      states: stateResults,
      interactive_count: stateResults.reduce((sum, state) => sum + state.interactive_count, 0),
      button_like_count: stateResults.reduce((sum, state) => sum + state.button_like_count, 0),
      module_count: stateResults.reduce((sum, state) => sum + state.module_count, 0),
      ready: stateResults.every((state) => state.ready),
    });

    await page.close();
  }

  await browser.close();

  const report = {
    schema_version: "hepta-ui-harsh-top-design-referee-v4-button-census/v0",
    standards_version: "2026-06-27-harsh-button-census-light-tempered-glass",
    status: viewportResults.every((viewport) => viewport.ready) ? "ready" : "failed",
    base_url: baseUrl,
    census_dir: censusDir,
    viewport_count: viewportResults.length,
    screenshot_count: screenshots.length,
    interactive_instance_count: viewportResults.reduce((sum, viewport) => sum + viewport.interactive_count, 0),
    button_like_instance_count: viewportResults.reduce((sum, viewport) => sum + viewport.button_like_count, 0),
    module_instance_count: viewportResults.reduce((sum, viewport) => sum + viewport.module_count, 0),
    failure_count: viewportResults.reduce((sum, viewport) => sum + viewport.states.reduce((inner, state) => inner + state.failures.length + state.module_failures.length, 0), 0),
    viewports: viewportResults,
    screenshots,
  };
  console.log(JSON.stringify(report, null, 2));
}

async function openState(page, openSpec) {
  await page.evaluate((spec) => {
    const closeAll = () => {
      document.body.removeAttribute("data-control-ui-submenu-audit-open");
      document.querySelectorAll(".tg-thread-command-menu").forEach((node) => { node.open = false; });
      document.querySelectorAll(".tg-chat-item").forEach((row) => { row.classList.remove("tg-chat-item--menu-open"); });
      document.querySelectorAll(".tg-composer-popover").forEach((node) => { node.style.display = ""; });
      document.querySelectorAll(".tg-composer-picker").forEach((node) => { node.open = false; });
      if (window.location.hash === "#command-palette") window.location.hash = "chat";
    };
    closeAll();
    if (spec.type === "row-menu") {
      const row = document.querySelector(`[data-chat-conversation="${spec.key}"]`);
      if (row) {
        const scroller = row.closest(".tg-room-rail, .tg-conversation-list, .tg-sidebar, .tg-room-list");
        if (scroller && scroller.scrollHeight > scroller.clientHeight) {
          scroller.scrollTop = Math.max(0, row.offsetTop - ((scroller.clientHeight - row.getBoundingClientRect().height) / 2));
        }
        row.scrollIntoView({ block: "center", inline: "nearest" });
        row.classList.add("tg-chat-item--menu-open");
        const toggle = row.querySelector("[data-chat-row-menu-toggle]");
        if (toggle) {
          toggle.style.opacity = "1";
          toggle.style.pointerEvents = "auto";
          toggle.style.transform = "translateX(0)";
          toggle.style.transition = "none";
        }
      }
    } else if (spec.type === "thread-tools") {
      const node = document.querySelector('[data-thread-command-menu="true"]');
      if (node) node.open = true;
    } else if (spec.type === "composer-tools") {
      const node = document.querySelector("[data-control-ui-composer-more]");
      if (node) node.open = true;
    } else if (spec.type === "composer-popover") {
      document.body.setAttribute("data-control-ui-submenu-audit-open", "true");
      document.querySelectorAll(".tg-composer-popover").forEach((node) => { node.style.display = "none"; });
      const node = document.querySelector(`[data-chat-composer-popover="${spec.key}"]`);
      if (node) {
        const details = node.closest(".tg-composer-picker");
        if (details) details.open = true;
        node.style.display = "grid";
      }
    } else if (spec.type === "command-palette") {
      window.location.hash = "command-palette";
    }
  }, openSpec);
}

async function capture(page, viewport, key, censusDir) {
  const filename = `${sanitize(viewport.name)}-${sanitize(key)}.png`;
  const outputPath = path.join(censusDir, filename);
  await page.screenshot({ path: outputPath, fullPage: false });
  const bytes = fs.statSync(outputPath).size;
  const sha256 = crypto.createHash("sha256").update(fs.readFileSync(outputPath)).digest("hex");
  const minBytes = viewport.width >= 700 ? 80000 : 45000;
  return {
    name: key,
    viewport: viewport.name,
    viewport_size: `${viewport.width}x${viewport.height}`,
    path: outputPath,
    bytes,
    sha256,
    min_bytes: minBytes,
    bytes_ready: bytes >= minBytes,
  };
}

main().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
NODE

button_census_sha="$(shasum -a 256 "$BUTTON_CENSUS_REPORT_PATH" | awk '{print $1}')"
v3_sha="$(shasum -a 256 "$V3_REPORT_PATH" | awk '{print $1}')"
native_sha="$(shasum -a 256 "$NATIVE_REPORT_PATH" | awk '{print $1}')"

jq -n \
  --arg v3_path "$V3_REPORT_PATH" \
  --arg button_census_path "$BUTTON_CENSUS_REPORT_PATH" \
  --arg native_path "$NATIVE_REPORT_PATH" \
  --arg v3_sha "$v3_sha" \
  --arg button_census_sha "$button_census_sha" \
  --arg native_sha "$native_sha" \
  --slurpfile v3_file "$V3_REPORT_PATH" \
  --slurpfile button_file "$BUTTON_CENSUS_REPORT_PATH" \
  --slurpfile native_file "$NATIVE_REPORT_PATH" '
  ($v3_file[0]) as $v3
  | ($button_file[0]) as $button
  | ($native_file[0]) as $native
  | def v3_ready:
      $v3.status == "ready"
      and $v3.standards_version == "2026-06-27-harsh-visual-matrix-light-tempered-glass"
      and $v3.summary.control.viewport_count == 4
      and $v3.summary.control.screenshot_count == 56
      and $v3.summary.native.screenshot_count >= 41
      and $v3.native_ready == true;
    def button_ready:
      $button.status == "ready"
      and $button.standards_version == "2026-06-27-harsh-button-census-light-tempered-glass"
      and $button.viewport_count == 4
      and $button.screenshot_count == 30
      and $button.failure_count == 0
      and $button.interactive_instance_count >= 220
      and $button.button_like_instance_count >= 160
      and $button.module_instance_count >= 240
      and ($button.screenshots | all(.bytes_ready == true))
      and ($button.viewports | all(
        .ready == true
        and .interactive_count >= 30
        and .button_like_count >= 20
        and .module_count >= 20
        and (.states | all(.ready == true and .horizontal_overflow_free == true and (.failures | length) == 0 and (.module_failures | length) == 0))
      ))
      and ([$button.viewports[] | select(.name == "desktop") | .state_count][0] == 9)
      and ([$button.viewports[] | select(.name == "narrow") | .state_count][0] == 9)
      and ([$button.viewports[] | select(.name == "mobile") | .state_count][0] == 6)
      and ([$button.viewports[] | select(.name == "phone320") | .state_count][0] == 6);
    def native_ready:
      $native.status == "ready"
      and $native.native_top_design_referee_ready == true
      and $native.native_tempered_glass_visual_contract_ready == true
      and $native.native_secondary_harsh_action_matrix_ready == true
      and $native.native_visible_audit_failure_count == 0
      and $native.secondary_product_surfaces.case_count == 15
      and $native.secondary_product_surfaces.total_action_instance_count == 57;
    {
      schema_version:"hepta-ui-harsh-top-design-referee-v4-gate/v0",
      standards_version:"2026-06-27-harsh-button-census-plus-visual-matrix-light-tempered-glass",
      status:(if (v3_ready and button_ready and native_ready) then "ready" else "failed" end),
      inputs:{
        v3_visual_matrix:{path:$v3_path, sha256:$v3_sha},
        control_button_census:{path:$button_census_path, sha256:$button_census_sha},
        native_fixture:{path:$native_path, sha256:$native_sha}
      },
      summary:{
        control_visual_matrix:$v3.summary.control,
        control_button_census:{
          viewport_count:$button.viewport_count,
          screenshot_count:$button.screenshot_count,
          interactive_instance_count:$button.interactive_instance_count,
          button_like_instance_count:$button.button_like_instance_count,
          module_instance_count:$button.module_instance_count,
          failure_count:$button.failure_count,
          viewports:[$button.viewports[] | {
            name,
            state_count,
            interactive_count,
            button_like_count,
            module_count,
            ready
          }]
        },
        native:$v3.summary.native
      },
      v3_ready:v3_ready,
      button_census_ready:button_ready,
      native_ready:native_ready,
      button_census:$button
    }
  ' >"$tmp_report"

cp "$tmp_report" "$REPORT_PATH"
cat "$REPORT_PATH"

if [[ "$(jq -r '.status' "$REPORT_PATH")" != "ready" ]]; then
  echo "Hepta UI harsh top-design referee v4 button census failed" >&2
  exit 1
fi

echo "$REPORT_PATH"
