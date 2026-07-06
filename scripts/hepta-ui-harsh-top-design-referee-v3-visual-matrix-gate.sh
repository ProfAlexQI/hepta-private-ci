#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-${1:-}}"
REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V3_REPORT_PATH:-}"
MATRIX_DIR="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V3_MATRIX_DIR:-}"
CONTROL_MATRIX_REPORT_PATH="${HEPTA_UI_HARSH_TOP_DESIGN_REFEREE_V3_CONTROL_MATRIX_REPORT_PATH:-}"
NATIVE_REPORT_PATH="${HEPTA_NATIVE_FIXTURE_VISUAL_SMOKE_REPORT_PATH:-}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
MANIFEST="codex-rs/Cargo.toml"
HOST="${HEPTA_CONTROL_UI_SMOKE_HOST:-127.0.0.1}"
BIND_ADDR="${HEPTA_CONTROL_UI_SMOKE_ADDR:-}"
SERVER_LOG="${HEPTA_CONTROL_UI_SERVER_LOG:-$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v3-server.XXXXXX")}"
STARTUP_TIMEOUT_SEC="${HEPTA_CONTROL_UI_SMOKE_STARTUP_TIMEOUT_SEC:-900}"

if [[ -z "$READINESS_DIR" ]]; then
  echo "usage: $0 <hepta-ui-product-readiness-dir>" >&2
  exit 2
fi
if [[ -z "$REPORT_PATH" ]]; then
  REPORT_PATH="$READINESS_DIR/ui-harsh-top-design-referee-v3-visual-matrix-gate.json"
fi
if [[ -z "$MATRIX_DIR" ]]; then
  MATRIX_DIR="$READINESS_DIR/control-ui-v3-visual-matrix"
fi
if [[ -z "$CONTROL_MATRIX_REPORT_PATH" ]]; then
  CONTROL_MATRIX_REPORT_PATH="$READINESS_DIR/control-ui-v3-visual-matrix.json"
fi
if [[ -z "$NATIVE_REPORT_PATH" ]]; then
  NATIVE_REPORT_PATH="$READINESS_DIR/native-fixture/native-fixture-visual-smoke.json"
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

if [[ -z "$BIND_ADDR" ]]; then
  for port in 7374 7375 7376 7377 7378; do
    if ! lsof -nP -iTCP:"$port" -sTCP:LISTEN >/dev/null 2>&1; then
      BIND_ADDR="${HOST}:${port}"
      break
    fi
  done
fi
if [[ -z "$BIND_ADDR" ]]; then
  echo "no free local port found for Hepta Control UI v3 referee" >&2
  exit 1
fi

BASE_URL="http://${BIND_ADDR}"
server_pid=""
tmp_report="$(mktemp "${TMPDIR:-/tmp}/hepta-ui-v3-final.XXXXXX")"

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
      echo "Hepta Control UI server exited before v3 matrix was ready" >&2
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

mkdir -p "$MATRIX_DIR" "$(dirname "$REPORT_PATH")" "$(dirname "$CONTROL_MATRIX_REPORT_PATH")"
start_server
wait_for_server

node - "$CHROME_BIN" "$BASE_URL/" "$MATRIX_DIR" >"$CONTROL_MATRIX_REPORT_PATH" <<'NODE'
const { chromium } = require("playwright");
const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");

const [chromeBin, baseUrl, matrixDir] = process.argv.slice(2);
const viewports = [
  { name: "desktop", width: 1365, height: 900, railVisible: true },
  { name: "narrow", width: 768, height: 900, railVisible: true },
  { name: "mobile", width: 500, height: 844, railVisible: false },
  { name: "phone320", width: 320, height: 844, railVisible: false },
];

const sanitize = (value) => value.replace(/[^a-z0-9._-]+/gi, "-").replace(/^-|-$/g, "").toLowerCase();

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
    await page.goto(baseUrl, { waitUntil: "networkidle" });
    await page.waitForTimeout(250);

    const defaultAudit = await page.evaluate(() => {
      const visible = (element) => {
        if (!element) return false;
        const rect = element.getBoundingClientRect();
        const style = getComputedStyle(element);
        return style.display !== "none"
          && style.visibility !== "hidden"
          && Number(style.opacity || 1) > 0
          && rect.width > 1
          && rect.height > 1;
      };
      const submenus = Array.from(document.querySelectorAll(
        ".tg-composer-popover,.tg-row-action-popover,.tg-thread-command-menu__panel,.command-palette-backdrop,.command-palette",
      ));
      return {
        ready: submenus.every((node) => !visible(node)),
        visible_count: submenus.filter(visible).length,
        submenu_count: submenus.length,
      };
    });
    const defaultShot = await capture(page, viewport, "default-closed", matrixDir);
    screenshots.push(defaultShot);

    const targets = [];
    if (viewport.railVisible) {
      for (const key of ["ui-chat-agent", "task-queue", "operator-plane"]) {
        targets.push({
          key: `row-menu-${key}`,
          group: "row-menu",
          expectedVisibleCount: 1,
          expectedItemCount: 3,
          requiresSvg: true,
          requiresRole: true,
          targetSelectors: [`[data-chat-row-menu-panel="${key}"]`],
          surfaceSelectors: [`[data-chat-row-menu-panel="${key}"]`],
          itemSelector: "[data-chat-row-menu-item]",
          open: { type: "row-menu", key },
        });
      }
    }
    targets.push(
      {
        key: "thread-tools",
        group: "thread-tools",
        expectedVisibleCount: 1,
        expectedItemCount: 3,
        requiresSvg: true,
        requiresRole: true,
        targetSelectors: ['[data-control-ui-thread-tools-panel="light-glass"]'],
        surfaceSelectors: ['[data-control-ui-thread-tools-panel="light-glass"]'],
        itemSelector: "[data-control-ui-menu-item]",
        open: { type: "thread-tools" },
      },
      {
        key: "composer-tools",
        group: "composer-tools",
        expectedVisibleCount: 1,
        expectedItemCount: 2,
        requiresSvg: true,
        requiresRole: true,
        targetSelectors: ['[data-control-ui-composer-tools-panel="light-glass"]'],
        surfaceSelectors: ['[data-control-ui-composer-tools-panel="light-glass"]'],
        itemSelector: "[data-control-ui-menu-item]",
        open: { type: "composer-tools" },
      },
      {
        key: "composer-popover-artifact",
        group: "composer-popover",
        expectedVisibleCount: 1,
        expectedItemCount: 2,
        requiresSvg: true,
        requiresRole: true,
        targetSelectors: ['[data-chat-composer-popover="artifact"]'],
        surfaceSelectors: ['[data-chat-composer-popover="artifact"]'],
        itemSelector: ".tg-composer-popover__item",
        open: { type: "composer-popover", key: "artifact" },
      },
      {
        key: "composer-popover-command",
        group: "composer-popover",
        expectedVisibleCount: 1,
        expectedItemCount: 2,
        requiresSvg: true,
        requiresRole: true,
        targetSelectors: ['[data-chat-composer-popover="command"]'],
        surfaceSelectors: ['[data-chat-composer-popover="command"]'],
        itemSelector: ".tg-composer-popover__item",
        open: { type: "composer-popover", key: "command" },
      },
      {
        key: "command-palette",
        group: "command-palette",
        expectedVisibleCount: 2,
        expectedItemCount: 18,
        requiresSvg: false,
        requiresRole: false,
        requiresUnobscuredItems: false,
        targetSelectors: ["#command-palette", ".command-palette"],
        surfaceSelectors: [".command-palette"],
        itemSelector: "[data-control-ui-command-palette-result='light-glass']",
        open: { type: "command-palette" },
      },
    );

    const targetResults = [];
    for (const target of targets) {
      await page.goto(baseUrl, { waitUntil: "networkidle" });
      await page.waitForTimeout(100);
      await page.evaluate((openSpec) => {
        const closeAll = () => {
          document.body.removeAttribute("data-control-ui-submenu-audit-open");
          document.querySelectorAll(".tg-thread-command-menu").forEach((node) => {
            node.open = false;
          });
          document.querySelectorAll(".tg-chat-item").forEach((row) => {
            row.classList.remove("tg-chat-item--menu-open");
          });
          document.querySelectorAll(".tg-composer-popover").forEach((node) => {
            node.style.display = "";
          });
          document.querySelectorAll(".tg-composer-picker").forEach((node) => {
            node.open = false;
          });
          if (window.location.hash === "#command-palette") {
            window.location.hash = "chat";
          }
        };
        closeAll();
        if (openSpec.type === "row-menu") {
          const row = document.querySelector(`[data-chat-conversation="${openSpec.key}"]`);
          if (row) {
            const scroller = row.closest(".tg-room-rail, .tg-conversation-list, .tg-sidebar, .tg-room-list");
            if (scroller && scroller.scrollHeight > scroller.clientHeight) {
              scroller.scrollTop = Math.max(0, row.offsetTop - ((scroller.clientHeight - row.getBoundingClientRect().height) / 2));
            }
            row.scrollIntoView({ block: "center", inline: "nearest" });
          }
        }
      }, target.open);
      await page.waitForTimeout(80);
      const closedBaseline = await capture(page, viewport, `${target.key}-closed-baseline`, matrixDir);
      screenshots.push(closedBaseline);
      await page.evaluate((openSpec) => {
        const closeAll = () => {
          document.body.removeAttribute("data-control-ui-submenu-audit-open");
          document.querySelectorAll(".tg-thread-command-menu").forEach((node) => {
            node.open = false;
          });
          document.querySelectorAll(".tg-chat-item").forEach((row) => {
            row.classList.remove("tg-chat-item--menu-open");
          });
          document.querySelectorAll(".tg-composer-popover").forEach((node) => {
            node.style.display = "";
          });
          document.querySelectorAll(".tg-composer-picker").forEach((node) => {
            node.open = false;
          });
          if (window.location.hash === "#command-palette") {
            window.location.hash = "chat";
          }
        };
        closeAll();
        if (openSpec.type === "row-menu") {
          const row = document.querySelector(`[data-chat-conversation="${openSpec.key}"]`);
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
        } else if (openSpec.type === "thread-tools") {
          const node = document.querySelector('[data-thread-command-menu="true"]');
          if (node) node.open = true;
        } else if (openSpec.type === "composer-tools") {
          const node = document.querySelector("[data-control-ui-composer-more]");
          if (node) node.open = true;
        } else if (openSpec.type === "composer-popover") {
          document.body.setAttribute("data-control-ui-submenu-audit-open", "true");
          document.querySelectorAll(".tg-composer-popover").forEach((node) => {
            node.style.display = "none";
          });
          const node = document.querySelector(`[data-chat-composer-popover="${openSpec.key}"]`);
          if (node) {
            const details = node.closest(".tg-composer-picker");
            if (details) details.open = true;
            node.style.display = "grid";
          }
        } else if (openSpec.type === "command-palette") {
          window.location.hash = "command-palette";
        }
      }, target.open);
      await page.waitForTimeout(120);

      const audit = await page.evaluate((target) => {
        const visible = (element) => {
          if (!element) return false;
          const rect = element.getBoundingClientRect();
          const style = getComputedStyle(element);
          return style.display !== "none"
            && style.visibility !== "hidden"
            && Number(style.opacity || 1) > 0
            && rect.width > 1
            && rect.height > 1;
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
        const unobscuredAt = (element, x, y) => {
          if (x < 0 || y < 0 || x > window.innerWidth || y > window.innerHeight) return false;
          const hit = document.elementFromPoint(x, y);
          return hit === element || element.contains(hit);
        };
        const visuallyUnobscured = (element) => {
          const rect = element.getBoundingClientRect();
          const points = [
            [rect.left + rect.width / 2, rect.top + rect.height / 2],
            [rect.left + Math.min(rect.width - 4, Math.max(4, rect.width * 0.2)), rect.top + Math.min(rect.height - 4, Math.max(4, rect.height * 0.2))],
            [rect.right - Math.min(rect.width - 4, Math.max(4, rect.width * 0.2)), rect.top + Math.min(rect.height - 4, Math.max(4, rect.height * 0.2))],
            [rect.left + Math.min(rect.width - 4, Math.max(4, rect.width * 0.2)), rect.bottom - Math.min(rect.height - 4, Math.max(4, rect.height * 0.2))],
            [rect.right - Math.min(rect.width - 4, Math.max(4, rect.width * 0.2)), rect.bottom - Math.min(rect.height - 4, Math.max(4, rect.height * 0.2))],
          ];
          return points.every(([x, y]) => unobscuredAt(element, x, y));
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
          let color = { r: 5, g: 8, b: 11, a: 1 };
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
        const hasSvgIcon = (element) => Boolean(element?.querySelector("svg use[href^='#hepta-icon-']"));
        const allSubmenus = Array.from(document.querySelectorAll(
          ".tg-composer-popover,.tg-row-action-popover,.tg-thread-command-menu__panel,.command-palette-backdrop,.command-palette",
        )).filter(visible);
        const targetNodes = target.targetSelectors.flatMap((selector) => Array.from(document.querySelectorAll(selector))).filter(visible);
        const targetSet = new Set(targetNodes);
        const unexpectedVisible = allSubmenus.filter((node) => !targetSet.has(node));
        const surfaceNodes = target.surfaceSelectors.flatMap((selector) => Array.from(document.querySelectorAll(selector))).filter(visible);
        const surfaceDetails = surfaceNodes.map((node) => {
          const style = getComputedStyle(node);
          const bg = effectiveBackground(node);
          const luma = luminance(bg);
          const rect = rectOf(node);
          return {
            role: node.getAttribute("role") || "",
            aria_label: node.getAttribute("aria-label") || "",
            item_count: target.itemSelector ? node.querySelectorAll(target.itemSelector).length : 0,
            light_glass_ready: luma >= 0.72 && luma <= 0.98,
            effective_luminance: Number(luma.toFixed(3)),
            backdrop_filter: style.backdropFilter || style.webkitBackdropFilter || "",
            box_shadow_present: style.boxShadow && style.boxShadow !== "none",
            border_radius: Number.parseFloat(style.borderTopLeftRadius || "0") || 0,
            in_viewport: rect.left >= -1 && rect.top >= -1 && rect.right <= window.innerWidth + 1 && rect.bottom <= window.innerHeight + 1,
            visually_unobscured: visuallyUnobscured(node),
            ...rect,
          };
        });
        const itemNodes = target.itemSelector ? surfaceNodes.flatMap((node) => Array.from(node.querySelectorAll(target.itemSelector))).filter(visible) : [];
        const itemDetails = itemNodes.map((node) => {
          const style = getComputedStyle(node);
          const fg = parseColor(style.color);
          const bg = effectiveBackground(node);
          const ratio = fg ? contrast(fg, bg) : 0;
          const ariaLabel = node.getAttribute("aria-label") || "";
          const title = node.getAttribute("title") || "";
          const rect = node.getBoundingClientRect();
          return {
            label: textOf(node),
            role: node.getAttribute("role") || "",
            aria_label: ariaLabel,
            title,
            title_matches_aria_label: title === ariaLabel,
            svg_icon_present: hasSvgIcon(node),
            readable: ratio >= 4.5,
            contrast_ratio: Number(ratio.toFixed(2)),
            visually_unobscured: unobscuredAt(node, rect.left + rect.width / 2, rect.top + rect.height / 2),
            height: Math.round(rect.height),
          };
        });
        const horizontalOverflowFree = document.documentElement.scrollWidth - window.innerWidth <= 1
          && document.body.scrollWidth - window.innerWidth <= 1;
        const requiresUnobscuredItems = target.requiresUnobscuredItems !== false;
        const surfacesReady = surfaceDetails.length > 0 && surfaceDetails.every((item) => (
          item.in_viewport
          && item.visually_unobscured
          && item.light_glass_ready
          && item.effective_luminance >= 0.72
          && item.effective_luminance <= 0.98
          && String(item.backdrop_filter || "").includes("blur(")
          && item.box_shadow_present
          && item.border_radius >= 14
        ));
        const itemsReady = itemDetails.length === target.expectedItemCount && itemDetails.every((item) => (
          item.height >= 44
          && (!target.requiresSvg || item.svg_icon_present)
          && (!target.requiresRole || item.role === "menuitem")
          && item.readable
          && item.contrast_ratio >= 4.5
          && (!requiresUnobscuredItems || item.visually_unobscured)
          && item.title_matches_aria_label
          && item.label.length > 0
        ));
        return {
          target_key: target.key,
          group: target.group,
          expected_visible_count: target.expectedVisibleCount,
          visible_target_count: targetNodes.length,
          unexpected_visible_count: unexpectedVisible.length,
          horizontal_overflow_free: horizontalOverflowFree,
          expected_item_count: target.expectedItemCount,
          visible_item_count: itemDetails.length,
          requires_unobscured_items: requiresUnobscuredItems,
          surfaces_ready: surfacesReady,
          items_ready: itemsReady,
          surface_details: surfaceDetails,
          item_details: itemDetails,
          ready: targetNodes.length === target.expectedVisibleCount
            && unexpectedVisible.length === 0
            && horizontalOverflowFree
            && surfacesReady
            && itemsReady,
        };
      }, target);
      const screenshot = await capture(page, viewport, target.key, matrixDir);
      const visualDelta = {
        closed_baseline_sha256: closedBaseline.sha256,
        default_sha256: defaultShot.sha256,
        visually_distinct_from_closed_baseline: screenshot.sha256 !== closedBaseline.sha256,
        visually_distinct_from_default: screenshot.sha256 !== defaultShot.sha256,
      };
      screenshots.push(screenshot);
      targetResults.push({
        ...audit,
        visual_delta_ready: visualDelta.visually_distinct_from_closed_baseline,
        ready: audit.ready && visualDelta.visually_distinct_from_closed_baseline,
        closed_baseline_screenshot: closedBaseline,
        screenshot: { ...screenshot, ...visualDelta },
      });
    }

    viewportResults.push({
      name: viewport.name,
      viewport: { width: viewport.width, height: viewport.height },
      default_closed: defaultAudit,
      target_count: targets.length,
      targets: targetResults,
      ready: defaultAudit.ready && targetResults.every((target) => target.ready),
    });

    await page.close();
  }

  await browser.close();

  const targetScreenshotCount = viewportResults.reduce((sum, viewport) => sum + viewport.targets.length, 0);
  const report = {
    schema_version: "hepta-ui-harsh-top-design-referee-v3-visual-matrix/v0",
    standards_version: "2026-06-27-harsh-visual-matrix-light-tempered-glass",
    status: viewportResults.every((viewport) => viewport.ready) ? "ready" : "failed",
    base_url: baseUrl,
    matrix_dir: matrixDir,
    viewport_count: viewportResults.length,
    screenshot_count: screenshots.length,
    default_closed_screenshot_count: viewports.length,
    target_closed_baseline_screenshot_count: targetScreenshotCount,
    target_screenshot_count: targetScreenshotCount,
    action_state_screenshot_count: screenshots.length - viewports.length,
    viewports: viewportResults,
    screenshots,
  };
  console.log(JSON.stringify(report, null, 2));
}

async function capture(page, viewport, key, matrixDir) {
  const filename = `${sanitize(viewport.name)}-${sanitize(key)}.png`;
  const outputPath = path.join(matrixDir, filename);
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

control_sha="$(shasum -a 256 "$CONTROL_MATRIX_REPORT_PATH" | awk '{print $1}')"
native_sha="$(shasum -a 256 "$NATIVE_REPORT_PATH" | awk '{print $1}')"
native_output_dir="$(jq -r '.output_dir // empty' "$NATIVE_REPORT_PATH")"
native_png_count=0
if [[ -n "$native_output_dir" && -d "$native_output_dir" ]]; then
  native_png_count="$(find "$native_output_dir" -maxdepth 1 -type f -name '*.png' | wc -l | tr -d ' ')"
fi

jq -n \
  --arg control_path "$CONTROL_MATRIX_REPORT_PATH" \
  --arg native_path "$NATIVE_REPORT_PATH" \
  --arg control_sha "$control_sha" \
  --arg native_sha "$native_sha" \
  --argjson native_png_count "$native_png_count" \
  --slurpfile control_file "$CONTROL_MATRIX_REPORT_PATH" \
  --slurpfile native_file "$NATIVE_REPORT_PATH" '
  ($control_file[0]) as $control
  | ($native_file[0]) as $native
  | def control_ready:
      $control.status == "ready"
      and $control.viewport_count == 4
      and $control.screenshot_count == 56
      and $control.default_closed_screenshot_count == 4
      and $control.target_closed_baseline_screenshot_count == 26
      and $control.target_screenshot_count == 26
      and $control.action_state_screenshot_count == 52
      and ($control.screenshots | all(.bytes_ready == true))
      and ($control.viewports | all(
        .ready == true
        and .default_closed.ready == true
        and (.targets | all(
          .ready == true
          and .unexpected_visible_count == 0
          and .horizontal_overflow_free == true
          and .visible_target_count == .expected_visible_count
          and .visible_item_count == .expected_item_count
          and .visual_delta_ready == true
          and .closed_baseline_screenshot.bytes_ready == true
          and .screenshot.bytes_ready == true
          and .screenshot.visually_distinct_from_closed_baseline == true
          and .screenshot.visually_distinct_from_default == true
          and (.surface_details | all(.visually_unobscured == true))
          and ((.requires_unobscured_items == false) or (.item_details | all(.visually_unobscured == true)))
        ))
      ))
      and ([$control.viewports[] | select(.name == "desktop") | .target_count][0] == 8)
      and ([$control.viewports[] | select(.name == "narrow") | .target_count][0] == 8)
      and ([$control.viewports[] | select(.name == "mobile") | .target_count][0] == 5)
      and ([$control.viewports[] | select(.name == "phone320") | .target_count][0] == 5)
      and ([$control.viewports[].targets[] | select(.group == "command-palette") | .visible_item_count] | all(. == 18));
    def native_ready:
      $native.status == "ready"
      and ($native.screenshot_count // 0) >= 41
      and $native.native_top_design_referee_ready == true
      and $native.native_tempered_glass_visual_contract_ready == true
      and $native.native_secondary_harsh_action_matrix_ready == true
      and $native.native_320_reflow_ready == true
      and $native.native_mobile_touch_target_preferred_ready == true
      and $native.native_telegram_mobile_safe_area_keyboard_ready == true
      and $native.native_readability_contrast_clip_ready == true
      and $native.native_visible_audit_failure_count == 0
      and ($native.tempered_glass_visual_contract.min_contrast_ratio // 0) >= 4.8
      and $native.secondary_product_surfaces.status == "ready"
      and $native.secondary_product_surfaces.case_count == 15
      and $native.secondary_product_surfaces.total_action_instance_count == 57
      and $native.secondary_product_surfaces.harsh_action_failure_count == 0
      and $native.secondary_product_surfaces.icon_text_placeholder_failure_count == 0
      and $native.secondary_product_surfaces.title_tooltip_failure_count == 0
      and $native.secondary_product_surfaces.label_layout_failure_count == 0
      and $native_png_count >= 41;
    {
      schema_version:"hepta-ui-harsh-top-design-referee-v3-gate/v0",
      standards_version:"2026-06-27-harsh-visual-matrix-light-tempered-glass",
      status:(if (control_ready and native_ready) then "ready" else "failed" end),
      inputs:{
        control_visual_matrix:{path:$control_path, sha256:$control_sha},
        native_fixture:{path:$native_path, sha256:$native_sha, png_count:$native_png_count}
      },
      summary:{
        control:{
          viewport_count:$control.viewport_count,
          screenshot_count:$control.screenshot_count,
          target_closed_baseline_screenshot_count:$control.target_closed_baseline_screenshot_count,
          target_screenshot_count:$control.target_screenshot_count,
          viewports:[$control.viewports[] | {
            name,
            default_closed_ready:.default_closed.ready,
            target_count,
            target_items:([.targets[].visible_item_count] | add),
            visually_distinct_targets:([.targets[] | select(.visual_delta_ready == true)] | length),
            command_palette_items:([.targets[] | select(.group == "command-palette") | .visible_item_count][0]),
            ready
          }]
        },
        native:{
          screenshot_count:$native.screenshot_count,
          png_count:$native_png_count,
          secondary_case_count:$native.secondary_product_surfaces.case_count,
          secondary_action_instance_count:$native.secondary_product_surfaces.total_action_instance_count,
          min_contrast:$native.tempered_glass_visual_contract.min_contrast_ratio,
          visible_audit_failure_count:$native.native_visible_audit_failure_count
        }
      },
      control_matrix:$control,
      native_ready:native_ready
    }
  ' >"$tmp_report"

cp "$tmp_report" "$REPORT_PATH"
cat "$REPORT_PATH"

if [[ "$(jq -r '.status' "$REPORT_PATH")" != "ready" ]]; then
  echo "Hepta UI harsh top-design referee v3 visual matrix failed" >&2
  exit 1
fi

echo "$REPORT_PATH"
