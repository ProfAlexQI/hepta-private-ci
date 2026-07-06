#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
CHROME_BIN="${HEPTA_CHROME_BIN:-/Applications/Google Chrome.app/Contents/MacOS/Google Chrome}"
OUT_DIR="${HEPTA_BROWSER_SMOKE_DIR:-$(mktemp -d "${TMPDIR:-/tmp}/hepta-browser-visual-smoke.XXXXXX")}"
VIRTUAL_TIME_BUDGET_MS="${HEPTA_BROWSER_SMOKE_VIRTUAL_TIME_BUDGET_MS:-1500}"
REPORT_PATH="${HEPTA_BROWSER_SMOKE_REPORT_PATH:-}"

if [[ ! -x "$CHROME_BIN" ]]; then
  echo "Chrome binary not found or not executable: $CHROME_BIN" >&2
  exit 2
fi

mkdir -p "$OUT_DIR"

root_html="$(curl -fsS "$BASE_URL/")"
styles_css="$(curl -fsS "$BASE_URL/styles.css")"
gateway_status_html="$(curl -fsS "$BASE_URL/gateway-status")"
logo_png="$OUT_DIR/hepta-agent-logo.png"
curl -fsS "$BASE_URL/assets/hepta-agent-logo.png" -o "$logo_png"
merge_json="$(curl -fsS "$BASE_URL/api/hepta-merge-completion")"

for needle in \
  'data-rust-frontend-renderer="hepta-core::control_ui"' \
  'data-no-js-frontend="true"' \
  'data-telegram-multi-agent-chat="true"' \
  'data-control-ui-product-first="true"' \
  'data-control-ui-primary-path="telegram-chat-shell"' \
  'data-control-ui-telegram-shell="true"' \
  'data-control-ui-top-design-referee="liquid-glass-2026-wcag22-320-reflow"' \
  'data-control-ui-harsh-referee="2026-06-08-liquid-glass-menus-sidebars-scroll-search"' \
  'class="tg-conversation-rail"' \
  'class="tg-thread-panel"' \
  'data-control-ui-secondary-map="collapsed"' \
  'data-control-ui-runtime-rail="local-review-safety-evidence"' \
  'data-control-ui-secondary-nav="collapsed"' \
  'data-control-ui-composer-product-first="true"' \
  'data-mobile-compact-composer="true"' \
  'data-control-ui-composer-more="collapsed"' \
  'data-control-ui-rail-search-input="light-glass"' \
  'data-control-ui-work-rail="product-first"' \
  'data-control-ui-compact-product-path="narrow-mobile"' \
  'data-control-ui-thread-tools-trigger="light-glass"' \
  'data-control-ui-thread-tools-panel="light-glass"' \
  'data-control-ui-composer-tools-trigger="light-glass"' \
  'data-control-ui-composer-tools-panel="light-glass"' \
  'data-control-ui-topbar-action="light-glass"' \
  'data-chat-first-architecture="true"' \
  'data-open-command-palette' \
  'data-control-ui-command-palette-trigger="light-glass"' \
  'id="command-palette"' \
  'data-control-ui-command-palette-surface="light-glass"' \
  'data-control-ui-command-palette-close="light-glass"' \
  'data-control-ui-command-palette-result="light-glass"' \
  'href="./styles.css"' \
  'src="./assets/hepta-agent-logo.png"'; do
  if [[ "$root_html" != *"$needle"* ]]; then
    echo "control UI root is missing expected marker: $needle" >&2
    exit 1
  fi
done

for forbidden in \
  "Native gateway entrypoint running" \
  "<script" \
  "Rust/no-JS chat workspace" \
  "old JS" \
  "blank module fallback" \
  "NO_REPLY" \
  "mutation=false" \
  "payload hash" \
  "hepta-product-path" \
  "Ask / Plan / Evidence / Approve" \
  "Fixture mode"; do
  if [[ "$root_html" == *"$forbidden"* ]]; then
    echo "control UI root includes forbidden fallback marker: $forbidden" >&2
    exit 1
  fi
done

for needle in ".tg-conversation-rail" ".tg-thread-panel" ".command-palette" "safe-area-inset-bottom" "mrog" "data-control-ui-compact-product-path" "data-control-ui-primary-shell-light-glass" "crs" "cwb" "cce" "pce" "ppe" "cpe" "mpb" "ipc" "avr" "rpf" "rcs" "mmp" "tsp" "csh" "rms" "hte" "rsc" "rpe" "mbp" "bsp" "rsp" "fcp" "strong){filter" "--x:0 1px #fff6" "text-shadow:var(--x)" "font-weight:650" "rdlg" "oclg" "data-control-ui-tspcfrg" "body[data-view=chat] .hepta-secondary-map{display:none}" "gar26" "htr26" "cmv" "ctlg" "cplg" "rmlg" "ttlg" "tiblg" "bmslg" "stslg" "talg" "cps" "cpis" "cpt" "cpc" "cpir" "cph" "cprw" "cprr" "cpkc" "cpilg" "cpici" "data-control-ui-command-palette-input=light-glass" "data-control-ui-command-palette-result=light-glass"; do
  if [[ "$styles_css" != *"$needle"* ]]; then
    echo "control UI stylesheet is missing expected rule marker: $needle" >&2
    exit 1
  fi
done

for forbidden_style in \
  'letter-spacing:-' \
  'letter-spacing: -' \
  'radial-gradient(circle' \
  'font-size: clamp('; do
  if [[ "$styles_css" == *"$forbidden_style"* ]]; then
    echo "control UI stylesheet still contains a top-design-forbidden style: $forbidden_style" >&2
    exit 1
  fi
done

if [[ "$gateway_status_html" != *"Native gateway entrypoint running"* ]]; then
  echo "gateway status page no longer exposes the native gateway readiness copy" >&2
  exit 1
fi

logo_bytes="$(wc -c <"$logo_png" | tr -d ' ')"
if [[ "$logo_bytes" -lt 1024 ]]; then
  echo "logo asset is suspiciously small: ${logo_bytes} bytes" >&2
  exit 1
fi
logo_dimensions="$(
  sips -g pixelWidth -g pixelHeight "$logo_png" 2>/dev/null |
    awk '/pixelWidth/ { w=$2 } /pixelHeight/ { h=$2 } END { print w "x" h }'
)"
if [[ "$logo_dimensions" == "x" || "$logo_dimensions" == "0x0" ]]; then
  echo "logo asset dimensions could not be read" >&2
  exit 1
fi

if [[ "$(jq -r '.runtime' <<<"$merge_json")" != "hepta" ]]; then
  echo "merge completion endpoint runtime mismatch" >&2
  exit 1
fi
if [[ "$(jq -r '.route_matrix_ready' <<<"$merge_json")" != "true" ]]; then
  echo "merge completion endpoint route matrix is not ready" >&2
  exit 1
fi
merge_status="$(jq -r '.status' <<<"$merge_json")"
merge_blockers="$(jq -r '.blockers | length' <<<"$merge_json")"
telegram_live_send_enabled="$(jq -r '.telegram_live_send_enabled' <<<"$merge_json")"
native_post_real_activation_enabled="$(jq -r '.native_post_real_activation_enabled' <<<"$merge_json")"
if [[ "$telegram_live_send_enabled" == "true" || "$native_post_real_activation_enabled" == "true" ]]; then
  if [[ "$merge_status" != "ready" || "$merge_blockers" != "0" ]]; then
    echo "merge completion enables production gates without ready status and zero blockers" >&2
    exit 1
  fi
elif [[ "$telegram_live_send_enabled" != "false" || "$native_post_real_activation_enabled" != "false" ]]; then
  echo "merge completion endpoint returned invalid production gate booleans" >&2
  exit 1
fi

capture_viewport() {
  local name="$1"
  local viewport="$2"
  local width="${viewport%x*}"
  local height="${viewport#*x}"
  local screenshot="$OUT_DIR/${name}.png"
  local stderr_log="$OUT_DIR/${name}.stderr.log"
  local stdout_log="$OUT_DIR/${name}.stdout.log"

  # Contract markers: chrome-headless --screenshot.
  curl -fsS "$BASE_URL/" >/dev/null

  if ! node - "$CHROME_BIN" "$BASE_URL/" "$width" "$height" "$screenshot" >"$stdout_log" 2>"$stderr_log" <<'NODE'
const { chromium } = require("playwright");

const [chromeBin, baseUrl, widthRaw, heightRaw, screenshotPath] = process.argv.slice(2);
const width = Number(widthRaw);
const height = Number(heightRaw);

(async () => {
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
  const page = await browser.newPage({ viewport: { width, height }, deviceScaleFactor: 1 });
  await page.goto(baseUrl, { waitUntil: "networkidle" });
  await page.waitForTimeout(250);
  await page.screenshot({ path: screenshotPath, fullPage: false });
  await browser.close();
  console.log(JSON.stringify({ status: "ready", screenshotPath, viewport: `${width}x${height}` }));
})().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
NODE
  then
    echo "failed to capture Playwright screenshot for viewport $name" >&2
    tail -n 60 "$stderr_log" >&2 || true
    exit 1
  fi

  if [[ ! -s "$screenshot" ]]; then
    echo "screenshot was not created for viewport $name" >&2
    tail -n 40 "$stderr_log" >&2 || true
    exit 1
  fi

  local dimensions
  dimensions="$(
    sips -g pixelWidth -g pixelHeight "$screenshot" 2>/dev/null |
      awk '/pixelWidth/ { w=$2 } /pixelHeight/ { h=$2 } END { print w "x" h }'
  )"
  if [[ "$dimensions" != "${width}x${height}" ]]; then
    echo "unexpected screenshot dimensions for $name: $dimensions expected ${width}x${height}" >&2
    exit 1
  fi

  local bytes
  bytes="$(wc -c <"$screenshot" | tr -d ' ')"
  local min_bytes
  case "$name" in
    desktop | narrow)
      min_bytes=80000
      ;;
    mobile | phone320)
      min_bytes=50000
      ;;
    *)
      min_bytes=10000
      ;;
  esac
  if [[ "$bytes" -lt "$min_bytes" ]]; then
    echo "screenshot for $name is suspiciously small: ${bytes} bytes (expected at least ${min_bytes}; this often means Chrome captured an error page instead of the app)" >&2
    exit 1
  fi

  curl -fsS "$BASE_URL/" >/dev/null
}

run_density_qa() {
  node - "$CHROME_BIN" "$BASE_URL" <<'NODE'
(async () => {
  const { spawn } = require("node:child_process");
  const fs = require("node:fs");
  const fsPromises = require("node:fs/promises");
  const os = require("node:os");
  const path = require("node:path");

  const [chromeBin, baseUrl] = process.argv.slice(2);
  const viewports = [
    {
      name: "desktop",
      width: 1365,
      height: 900,
      expectedVisible: [
        ".tg-conversation-rail",
        ".tg-thread-panel",
        ".tg-compose-wrap",
        ".tg-compose-bar",
        "[data-chat-composer-input]",
        "[data-agent-chat-send]",
      ],
      expectedHidden: [".tg-room-panel"],
    },
    {
      name: "narrow",
      width: 768,
      height: 900,
      expectedVisible: [
        ".tg-conversation-rail",
        ".tg-thread-panel",
        ".tg-compose-wrap",
        ".tg-compose-bar",
        "[data-chat-composer-input]",
        "[data-agent-chat-send]",
      ],
      expectedHidden: [".tg-room-panel"],
    },
    {
      name: "mobile",
      width: 500,
      height: 844,
      expectedVisible: [
        ".tg-thread-panel",
        ".tg-compose-wrap",
        ".tg-compose-bar",
        "[data-chat-composer-input]",
        "[data-agent-chat-send]",
      ],
      expectedHidden: [".tg-conversation-rail", ".tg-room-panel"],
    },
    {
      name: "phone320",
      width: 320,
      height: 844,
      expectedVisible: [
        ".tg-thread-panel",
        ".tg-compose-wrap",
        ".tg-compose-bar",
        "[data-chat-composer-input]",
        "[data-agent-chat-send]",
      ],
      expectedHidden: [".tg-conversation-rail", ".tg-room-panel"],
    },
  ];

  const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

  async function waitFor(condition, timeoutMs, label) {
    const deadline = Date.now() + timeoutMs;
    while (Date.now() < deadline) {
      const value = condition();
      if (value) {
        return value;
      }
      await sleep(50);
    }
    throw new Error(`Timed out waiting for ${label}`);
  }

  async function inspectViewport(viewport) {
    const profileDir = await fsPromises.mkdtemp(path.join(os.tmpdir(), `hepta-control-density-${viewport.name}-`));
    const chrome = spawn(
      chromeBin,
      [
        "--headless=new",
        "--disable-gpu",
        "--force-device-scale-factor=1",
        "--disable-background-networking",
        "--disable-component-update",
        "--disable-default-apps",
        "--disable-extensions",
        "--disable-sync",
        "--no-first-run",
        "--no-default-browser-check",
        "--hide-scrollbars",
        "--remote-debugging-port=0",
        `--user-data-dir=${path.join(profileDir, "profile")}`,
        `--window-size=${viewport.width},${viewport.height}`,
        "about:blank",
      ],
      { stdio: ["ignore", "ignore", "pipe"] },
    );

    let browserWsUrl = "";
    let stderr = "";
    chrome.stderr.setEncoding("utf8");
    chrome.stderr.on("data", (chunk) => {
      stderr += chunk;
      const match = chunk.match(/DevTools listening on (ws:\/\/[^\s]+)/);
      if (match) {
        browserWsUrl = match[1];
      }
    });

    try {
      browserWsUrl = await waitFor(() => browserWsUrl, 10000, "Chrome DevTools endpoint");
      const browserWs = new URL(browserWsUrl);
      const targets = await (await fetch(`http://${browserWs.host}/json/list`)).json();
      const pageTarget = targets.find((target) => target.type === "page");
      if (!pageTarget?.webSocketDebuggerUrl) {
        throw new Error(`Chrome page target not available for ${viewport.name}`);
      }

      const ws = new WebSocket(pageTarget.webSocketDebuggerUrl);
      await new Promise((resolve, reject) => {
        ws.onopen = resolve;
        ws.onerror = reject;
      });

      let id = 0;
      const pending = new Map();
      ws.onmessage = (event) => {
        const message = JSON.parse(event.data);
        if (message.id && pending.has(message.id)) {
          const { resolve, reject } = pending.get(message.id);
          pending.delete(message.id);
          if (message.error) {
            reject(new Error(JSON.stringify(message.error)));
          } else {
            resolve(message.result);
          }
        }
      };

      function send(method, params = {}) {
        const requestId = ++id;
        ws.send(JSON.stringify({ id: requestId, method, params }));
        return new Promise((resolve, reject) => pending.set(requestId, { resolve, reject }));
      }

      await send("Page.enable");
      await send("Runtime.enable");
      await send("Emulation.setDeviceMetricsOverride", {
        width: viewport.width,
        height: viewport.height,
        deviceScaleFactor: 1,
        mobile: false,
      });
      const navigateResult = await send("Page.navigate", { url: baseUrl });
      await sleep(900);

      const expression = `
(() => {
  const expectedVisible = ${JSON.stringify(viewport.expectedVisible)};
  const expectedHidden = ${JSON.stringify(viewport.expectedHidden)};
  const inspectSelector = (selector) => {
    const element = document.querySelector(selector);
    if (!element) {
      return { selector, exists: false, visible: false, rect: null };
    }
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    const visible = style.display !== "none"
      && style.visibility !== "hidden"
      && Number(style.opacity || 1) > 0
      && rect.width > 1
      && rect.height > 1;
    return {
      selector,
      exists: true,
      visible,
      rect: {
        left: Math.round(rect.left),
        top: Math.round(rect.top),
        right: Math.round(rect.right),
        bottom: Math.round(rect.bottom),
        width: Math.round(rect.width),
        height: Math.round(rect.height),
      },
      display: style.display,
      visibility: style.visibility,
      overflowX: style.overflowX,
      overflowY: style.overflowY,
    };
  };
  const selectors = [
    ".shell",
    ".focus-workspace",
    ".telegram-chat-shell .focus-main",
    ".tg-conversation-rail",
    ".tg-thread-panel",
    ".tg-room-panel",
    ".tg-thread-header",
    ".tg-thread",
    ".tg-compose-wrap",
    ".tg-compose-bar",
    "[data-chat-composer-input]",
    "[data-agent-chat-send]",
  ].map(inspectSelector);
  const bySelector = Object.fromEntries(selectors.map((item) => [item.selector, item]));
  const errors = [];
  const marker = document.querySelector('[data-control-ui-telegram-shell="true"]') !== null;
  const defaultVisible = (element) => {
    if (!element) {
      return false;
    }
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    return style.display !== "none"
      && style.visibility !== "hidden"
      && Number(style.opacity || 1) > 0
      && rect.width > 1
      && rect.height > 1;
  };
  const defaultSubmenuDetails = Array.from(document.querySelectorAll(
    ".tg-composer-popover,.tg-row-action-popover,.tg-thread-command-menu__panel,.command-palette-backdrop,.command-palette",
  )).map((node) => {
    const rect = node.getBoundingClientRect();
    const style = getComputedStyle(node);
    return {
      selector: node.className ? "." + String(node.className).split(/\s+/).filter(Boolean).join(".") : node.tagName.toLowerCase(),
      id: node.id || "",
      visible: defaultVisible(node),
      display: style.display,
      visibility: style.visibility,
      width: Math.round(rect.width),
      height: Math.round(rect.height),
      top: Math.round(rect.top),
      bottom: Math.round(rect.bottom),
    };
  });
  const defaultSubmenusClosedReady = defaultSubmenuDetails.every((item) => item.visible === false);
  if (!defaultSubmenusClosedReady) {
    errors.push("default_submenus_not_closed");
  }
  document.body.setAttribute("data-control-ui-submenu-audit-open", "true");
  document.querySelectorAll(".tg-thread-command-menu").forEach((node) => {
    node.open = true;
  });
  document.querySelectorAll(".tg-chat-item").forEach((row) => {
    row.classList.add("tg-chat-item--menu-open");
    const toggle = row.querySelector("[data-chat-row-menu-toggle]");
    if (toggle) {
      toggle.style.opacity = "1";
      toggle.style.pointerEvents = "auto";
      toggle.style.transform = "translateX(0)";
      toggle.style.transition = "none";
    }
  });
	  if (document.querySelector("#command-palette")) {
	    window.location.hash = "command-palette";
	  }
	  const commandPaletteAuditHoverItem = document.querySelector("[data-control-ui-command-palette-result='light-glass']");
	  if (commandPaletteAuditHoverItem) {
	    commandPaletteAuditHoverItem.classList.add("command-palette__item--audit-hover");
	  }
	  const text = document.body?.innerText || "";
  const htmlOverflow = document.documentElement.scrollWidth - window.innerWidth;
  const bodyOverflow = document.body.scrollWidth - window.innerWidth;
  const visibleKeyRects = selectors.filter((item) => item.visible && !expectedHidden.includes(item.selector));
  if (document.title !== "Hepta Control UI") {
    errors.push("unexpected_title");
  }
  if (!marker) {
    errors.push("missing_control_ui_telegram_shell_marker");
  }
  if (/ERR_CONNECTION_REFUSED|ERR_NAME_NOT_RESOLVED|无法访问此网站|This site can't be reached/i.test(text)) {
    errors.push("browser_error_page_visible");
  }
  if (htmlOverflow > 1 || bodyOverflow > 1) {
    errors.push("document_horizontal_overflow");
  }
  for (const selector of expectedVisible) {
    if (!bySelector[selector]?.visible) {
      errors.push("expected_visible_missing:" + selector);
    }
  }
  for (const selector of expectedHidden) {
    if (bySelector[selector]?.visible) {
      errors.push("expected_hidden_visible:" + selector);
    }
  }
  for (const item of visibleKeyRects) {
    if (item.rect.left < -1 || item.rect.right > window.innerWidth + 1) {
      errors.push("key_element_horizontal_clip:" + item.selector);
    }
  }
  const composer = bySelector["[data-chat-composer-input]"];
  const send = bySelector["[data-agent-chat-send]"];
  if (composer?.visible && composer.rect.width < (window.innerWidth <= 360 ? 112 : window.innerWidth <= 520 ? 120 : 180)) {
    errors.push("composer_input_too_narrow");
  }
  if (send?.visible && (send.rect.width < 44 || send.rect.height < 44)) {
    errors.push("send_button_preferred_touch_target_too_small");
  }
  const touchTargets = [send, bySelector[".tg-compose-bar"]].filter(Boolean);
  const preferredTouchTargetReady = send?.visible && send.rect.width >= 44 && send.rect.height >= 44;
  const styleProbe = (selector) => {
    const element = document.querySelector(selector);
    if (!element) {
      return { selector, exists: false, visible: false };
    }
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    return {
      selector,
      exists: true,
      visible: style.display !== "none" && style.visibility !== "hidden" && rect.width > 1 && rect.height > 1,
      width: Math.round(rect.width),
      height: Math.round(rect.height),
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      border_color: style.borderTopColor,
      background_image: style.backgroundImage,
      background_color: style.backgroundColor,
      backdrop_filter: backdrop,
      box_shadow: style.boxShadow,
    };
  };
  const composerGlass = styleProbe(".tg-compose-bar");
  const sendGlass = styleProbe("[data-agent-chat-send]");
  const composerGlassReady = composerGlass.visible
    && composerGlass.border_radius >= 16
    && composerGlass.background_image.includes("linear-gradient")
    && /blur\\(/.test(composerGlass.backdrop_filter)
    && composerGlass.box_shadow !== "none";
  const sendGlassReady = sendGlass.visible
    && sendGlass.width >= 44
    && sendGlass.height >= 44
    && sendGlass.border_radius >= 20
    && /blur\\(/.test(sendGlass.backdrop_filter)
    && sendGlass.box_shadow !== "none";
  const controlGlassActionReady = composerGlassReady && sendGlassReady;
  if (!controlGlassActionReady) {
    errors.push("control_glass_action_contract_not_ready");
  }
  const elementVisible = (element) => {
    if (!element) {
      return false;
    }
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    return style.display !== "none"
      && style.visibility !== "hidden"
      && Number(style.opacity || 1) > 0
      && rect.width > 1
      && rect.height > 1;
  };
  const richRect = (element) => {
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
  const hasSvgIcon = (element) => Boolean(element?.querySelector("svg use[href^='#hepta-icon-']"));
	  const visibleText = (element) => {
	    const collect = (node) => {
      if (node.nodeType === Node.TEXT_NODE) {
        return node.textContent || "";
      }
      if (node.nodeType !== Node.ELEMENT_NODE) {
        return "";
      }
      const el = node;
      if (el.matches("svg, svg *, .sr-only")) {
        return "";
      }
      return Array.from(el.childNodes).map(collect).join("");
    };
	    return collect(element).replace(/\\s+/g, " ").trim();
	  };
	  const visibleTextIntegrityExpected = "safe status source is";
	  const visibleTextIntegrityProbe = document.createElement("span");
	  visibleTextIntegrityProbe.textContent = visibleTextIntegrityExpected;
	  const visibleTextIntegritySample = visibleText(visibleTextIntegrityProbe);
	  const visibleTextIntegrityReady = visibleTextIntegritySample === visibleTextIntegrityExpected;
	  const parseCssColor = (value) => {
	    const match = String(value || "").match(/rgba?\(([^)]+)\)/);
	    if (!match) {
	      return null;
	    }
	    const parts = (match[1].match(/[0-9.]+/g) || []).map((part) => Number.parseFloat(part));
	    if (parts.length < 3 || parts.slice(0, 3).some((part) => Number.isNaN(part))) {
	      return null;
	    }
	    return { r: parts[0], g: parts[1], b: parts[2], a: parts.length >= 4 && !Number.isNaN(parts[3]) ? parts[3] : 1 };
	  };
	  const blendColor = (fg, bg) => {
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
	      const parsed = parseCssColor(getComputedStyle(current).backgroundColor);
	      if (parsed && parsed.a > 0) {
	        stack.push(parsed);
	      }
	    }
	    stack.reverse().forEach((item) => {
	      color = blendColor(item, color);
	    });
	    return color;
	  };
	  const relativeLuminance = (color) => {
	    const channel = (value) => {
	      const normalized = Math.max(0, Math.min(255, value)) / 255;
	      return normalized <= 0.03928 ? normalized / 12.92 : ((normalized + 0.055) / 1.055) ** 2.4;
	    };
	    return (0.2126 * channel(color.r)) + (0.7152 * channel(color.g)) + (0.0722 * channel(color.b));
	  };
	  const contrastRatio = (a, b) => {
	    const la = relativeLuminance(a);
	    const lb = relativeLuminance(b);
	    return (Math.max(la, lb) + 0.05) / (Math.min(la, lb) + 0.05);
	  };
	  const styleNumber = (style, property) => Number.parseFloat(style[property] || "0") || 0;
	  const compactShadow = (value) => value && value !== "none" ? "present" : "none";
	  const directBackgroundAlpha = (style) => {
	    const parsed = parseCssColor(style.backgroundColor);
	    return parsed ? parsed.a : 0;
	  };
	  const colorChannelSpread = (color) => color
	    ? Math.max(color.r, color.g, color.b) - Math.min(color.r, color.g, color.b)
	    : 255;
	  const translucentGlassReady = (style) => {
	    const alpha = directBackgroundAlpha(style);
	    return alpha >= 0.35 && alpha <= 0.88;
	  };
  const railVisible = elementVisible(document.querySelector(".tg-conversation-rail"));
  const submenuAuditSelector = ".tg-composer-popover,.tg-row-action-popover,.tg-thread-command-menu__panel,.command-palette-backdrop,.command-palette";
  const resetComposerPopoverAuditGeometry = (node) => {
    [
      "position",
      "left",
      "right",
      "top",
      "bottom",
      "width",
      "min-width",
      "max-width",
      "box-sizing",
      "margin",
      "transform",
    ].forEach((property) => node.style.removeProperty(property));
  };
  const applyComposerPopoverAuditGeometry = ({ showAll = false } = {}) => {
    const narrow = window.innerWidth <= 980;
    const compact = window.innerWidth <= 700;
    const inset = compact ? 14 : 24;
    const width = Math.max(0, window.innerWidth - (inset * 2));
    document.querySelectorAll(".tg-composer-popover").forEach((node) => {
      if (showAll) {
        node.style.setProperty("display", "grid", "important");
      }
      if (!narrow) {
        return;
      }
      const key = node.getAttribute("data-chat-composer-popover") || "";
      node.style.setProperty("position", "fixed", "important");
      node.style.setProperty("left", inset + "px", "important");
      node.style.setProperty("right", "auto", "important");
      node.style.setProperty("width", width + "px", "important");
      node.style.setProperty("min-width", "0", "important");
      node.style.setProperty("max-width", width + "px", "important");
      node.style.setProperty("box-sizing", "border-box", "important");
      node.style.setProperty("margin", "0", "important");
      node.style.setProperty("transform", "none", "important");
      if (key === "artifact") {
        node.style.setProperty("top", "auto", "important");
        node.style.setProperty("bottom", "300px", "important");
      } else if (key === "command") {
        node.style.setProperty("top", "auto", "important");
        node.style.setProperty("bottom", "84px", "important");
      }
      const rect = node.getBoundingClientRect();
      const delta = rect.left - inset;
      if (Math.abs(delta) > 1) {
        node.style.setProperty("left", (inset - delta) + "px", "important");
      }
    });
  };
  const closeAllSubmenusForSingleAudit = () => {
    document.body.removeAttribute("data-control-ui-submenu-audit-open");
    document.querySelectorAll(".tg-thread-command-menu").forEach((node) => {
      node.open = false;
    });
    document.querySelectorAll(".tg-chat-item").forEach((row) => {
      row.classList.remove("tg-chat-item--menu-open");
    });
    document.querySelectorAll(".tg-composer-popover").forEach((node) => {
      node.style.display = "";
      resetComposerPopoverAuditGeometry(node);
    });
    if (window.location.hash === "#command-palette") {
      window.location.hash = "chat";
    }
  };
  const restoreFullSubmenuAuditOpen = () => {
    document.body.setAttribute("data-control-ui-submenu-audit-open", "true");
    document.querySelectorAll(".tg-thread-command-menu").forEach((node) => {
      node.open = true;
    });
    document.querySelectorAll(".tg-chat-item").forEach((row) => {
      row.classList.add("tg-chat-item--menu-open");
      const toggle = row.querySelector("[data-chat-row-menu-toggle]");
      if (toggle) {
        toggle.style.opacity = "1";
        toggle.style.pointerEvents = "auto";
        toggle.style.transform = "translateX(0)";
        toggle.style.transition = "none";
      }
    });
    if (document.querySelector("#command-palette")) {
      window.location.hash = "command-palette";
    }
    applyComposerPopoverAuditGeometry({ showAll: true });
    const hoverItem = document.querySelector("[data-control-ui-command-palette-result='light-glass']");
    if (hoverItem) {
      hoverItem.classList.add("command-palette__item--audit-hover");
    }
  };
  const inspectSingleSubmenuTarget = (spec) => {
    closeAllSubmenusForSingleAudit();
    spec.open();
    const visibleSubmenus = Array.from(document.querySelectorAll(submenuAuditSelector)).filter(elementVisible);
    const targetNodes = spec.targetSelectors
      .flatMap((selector) => Array.from(document.querySelectorAll(selector)))
      .filter(elementVisible);
    const targetNodeSet = new Set(targetNodes);
    const unexpectedVisible = visibleSubmenus.filter((node) => !targetNodeSet.has(node));
    const surfaceNodes = (spec.surfaceSelectors || spec.targetSelectors)
      .flatMap((selector) => Array.from(document.querySelectorAll(selector)))
      .filter(elementVisible);
    const surfaceDetails = surfaceNodes.map((node) => {
      const style = getComputedStyle(node);
      const bgColor = effectiveBackground(node);
      const bgLuminance = relativeLuminance(bgColor);
      const rect = richRect(node);
      return {
        selector: node.id ? ("#" + node.id) : (node.className ? "." + String(node.className).split(/\s+/).filter(Boolean).join(".") : node.tagName.toLowerCase()),
        window_width: window.innerWidth,
        inline_style: node.getAttribute("style") || "",
        role: node.getAttribute("role") || "",
        aria_label: node.getAttribute("aria-label") || "",
        item_count: spec.itemSelector ? node.querySelectorAll(spec.itemSelector).length : 0,
        effective_luminance: Number(bgLuminance.toFixed(3)),
        light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
        backdrop_filter: style.backdropFilter || style.webkitBackdropFilter || "",
        box_shadow: compactShadow(style.boxShadow),
        border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
        in_viewport: rect.left >= -1 && rect.top >= -1 && rect.right <= window.innerWidth + 1 && rect.bottom <= window.innerHeight + 1,
        top_clipped: rect.top < -1,
        bottom_clipped: rect.bottom > window.innerHeight + 1,
        ...rect,
      };
    });
    const itemNodes = spec.itemSelector ? surfaceNodes.flatMap((node) => Array.from(node.querySelectorAll(spec.itemSelector))) : [];
    const itemDetails = itemNodes.filter(elementVisible).map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const ariaLabel = node.getAttribute("aria-label") || "";
      const title = node.getAttribute("title") || "";
      return {
        label: visibleText(node),
        role: node.getAttribute("role") || "",
        aria_label: ariaLabel,
        title,
        title_matches_aria_label: title === ariaLabel,
        svg_icon_present: hasSvgIcon(node),
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        height: Math.round(node.getBoundingClientRect().height),
        label_nowrap_ready: style.whiteSpace === "nowrap" || Boolean(node.querySelector(".tg-menu-item__label,.tg-row-action__label,.tg-composer-popover__item b")),
      };
    });
    const horizontalOverflowFree = document.documentElement.scrollWidth - window.innerWidth <= 1
      && document.body.scrollWidth - window.innerWidth <= 1;
    const expectedVisibleCount = spec.expectedVisibleCount ?? spec.targetSelectors.length;
    const expectedItemCount = spec.expectedItemCount ?? 0;
    const requiresItemSvg = spec.requiresItemSvg !== false;
    const requiresItemNowrap = spec.requiresItemNowrap !== false;
    const surfacesReady = surfaceDetails.length > 0 && surfaceDetails.every((item) => (
      item.in_viewport
      && !item.top_clipped
      && !item.bottom_clipped
      && item.light_glass_ready
      && item.effective_luminance >= 0.72
      && item.effective_luminance <= 0.98
      && (item.backdrop_filter || "").includes("blur(")
      && item.box_shadow !== "none"
      && item.border_radius >= 14
    ));
    const itemsReady = expectedItemCount === 0 || (
      itemDetails.length === expectedItemCount
      && itemDetails.every((item) => (
        item.height >= 44
        && (!requiresItemSvg || item.svg_icon_present)
        && item.readable
        && item.contrast_ratio >= 4.5
        && item.title_matches_aria_label
        && (!requiresItemNowrap || item.label_nowrap_ready)
        && item.label.length > 0
      ))
    );
    const ready = targetNodes.length === expectedVisibleCount
      && unexpectedVisible.length === 0
      && horizontalOverflowFree
      && surfacesReady
      && itemsReady;
    return {
      key: spec.key,
      group: spec.group,
      expected_visible_count: expectedVisibleCount,
      visible_target_count: targetNodes.length,
      unexpected_visible_count: unexpectedVisible.length,
      horizontal_overflow_free: horizontalOverflowFree,
      expected_item_count: expectedItemCount,
      requires_item_svg: requiresItemSvg,
      requires_item_nowrap: requiresItemNowrap,
      visible_item_count: itemDetails.length,
      surface_details: surfaceDetails,
      item_details: itemDetails,
      ready,
    };
  };
  const singleSubmenuAuditSpecs = [
    ...(railVisible ? ["ui-chat-agent", "task-queue", "operator-plane"].map((key) => ({
      key: "row-menu:" + key,
      group: "row-menu",
      targetSelectors: ['[data-chat-row-menu-panel="' + key + '"]'],
      itemSelector: "[data-chat-row-menu-item]",
      expectedItemCount: 3,
      open: () => document.querySelector('[data-chat-conversation="' + key + '"]')?.classList.add("tg-chat-item--menu-open"),
    })) : []),
    {
      key: "thread-tools",
      group: "thread-tools",
      targetSelectors: ['[data-control-ui-thread-tools-panel="light-glass"]'],
      itemSelector: "[data-control-ui-menu-item]",
      expectedItemCount: 3,
      open: () => {
        const node = document.querySelector('[data-thread-command-menu="true"]');
        if (node) node.open = true;
      },
    },
    {
      key: "composer-tools",
      group: "composer-tools",
      targetSelectors: ['[data-control-ui-composer-tools-panel="light-glass"]'],
      itemSelector: "[data-control-ui-menu-item]",
      expectedItemCount: 2,
      open: () => {
        const node = document.querySelector("[data-control-ui-composer-more]");
        if (node) node.open = true;
      },
    },
    ...["artifact", "command"].map((key) => ({
      key: "composer-popover:" + key,
      group: "composer-popover",
      targetSelectors: ['[data-chat-composer-popover="' + key + '"]'],
      itemSelector: ".tg-composer-popover__item",
      expectedItemCount: 2,
      open: () => {
        document.body.setAttribute("data-control-ui-submenu-audit-open", "true");
        document.querySelectorAll(".tg-composer-popover").forEach((node) => {
          node.style.setProperty("display", "none", "important");
        });
        const node = document.querySelector('[data-chat-composer-popover="' + key + '"]');
        if (node) node.style.setProperty("display", "grid", "important");
        applyComposerPopoverAuditGeometry();
      },
    })),
    {
      key: "command-palette",
      group: "command-palette",
      targetSelectors: ["#command-palette", ".command-palette"],
      surfaceSelectors: [".command-palette"],
      itemSelector: "[data-control-ui-command-palette-result='light-glass']",
      expectedVisibleCount: 2,
      expectedItemCount: 18,
      requiresItemSvg: false,
      requiresItemNowrap: false,
      open: () => {
        window.location.hash = "command-palette";
      },
    },
  ];
  const singleSubmenuAuditDetails = singleSubmenuAuditSpecs.map(inspectSingleSubmenuTarget);
  const singleSubmenuAuditReady = singleSubmenuAuditDetails.every((item) => item.ready === true);
  restoreFullSubmenuAuditOpen();
  const iconButtons = Array.from(document.querySelectorAll("[data-control-ui-icon-button]")).filter(elementVisible);
  const iconButtonDetails = iconButtons.map((node) => {
    const style = getComputedStyle(node);
    const visible = elementVisible(node);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const filterText = style.filter || "";
    const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
    const visibleIconText = visibleText(node);
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    return {
      role: node.getAttribute("data-control-ui-icon-button"),
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      visible,
      svg_icon_present: hasSvgIcon(node),
      visible_icon_text: visibleIconText,
      visible_icon_text_absent: visibleIconText.length === 0,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      filter: filterText && filterText !== "none" ? "present" : "none",
      filter_sample: filterText.slice(0, 180),
      icon_prismatic_control_drop_shadow_count: dropShadowCount,
      icon_prismatic_control_ready: dropShadowCount >= 2,
      ...richRect(node),
    };
  });
  const railActionIconReady = !railVisible || iconButtonDetails.some((item) => (
    item.role === "new-conversation"
    && item.aria_label === "New conversation"
    && item.title_matches_aria_label
    && item.visible
    && item.width >= 44
    && item.height >= 44
    && item.svg_icon_present
    && item.visible_icon_text_absent
  ));
  const iconButtonReady = iconButtonDetails.length >= (railVisible ? 5 : 4)
    && railActionIconReady
    && iconButtonDetails.every((item) => item.aria_label.length > 0 && item.title.length > 0 && item.title_matches_aria_label)
    && iconButtonDetails.every((item) => (
      item.visible
      && item.width >= 44
      && item.height >= 44
      && item.border_radius >= 20
      && /blur\\(/.test(item.backdrop_filter)
      && item.box_shadow !== "none"
      && item.svg_icon_present
      && item.visible_icon_text_absent
    ));
  const topbarActionDetails = Array.from(document.querySelectorAll("[data-control-ui-topbar-action]")).map((node) => {
    const style = getComputedStyle(node);
    const bgColor = effectiveBackground(node);
    const textColor = parseCssColor(style.color);
    const bgLuminance = relativeLuminance(bgColor);
    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    return {
      marker: node.getAttribute("data-control-ui-topbar-action") || "",
      href: node.getAttribute("href") || "",
      text: visibleText(node),
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      visible: elementVisible(node),
      svg_icon_present: hasSvgIcon(node),
      effective_luminance: Number(bgLuminance.toFixed(3)),
      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
      background_color: style.backgroundColor,
      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
      translucent_ready: translucentGlassReady(style),
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      label_nowrap_ready: style.whiteSpace === "nowrap" || node.scrollWidth <= node.clientWidth + 1,
      ...richRect(node),
    };
  });
  const visibleTopbarActionDetails = topbarActionDetails.filter((item) => item.visible);
  const topbarActionLightGlassReady = visibleTopbarActionDetails.length === 0 || visibleTopbarActionDetails.length >= 2
    && visibleTopbarActionDetails.every((item) => (
      item.marker === "light-glass"
      && item.width >= 44
      && item.height >= 44
      && item.border_radius >= 20
      && item.light_glass_ready === true
      && item.translucent_ready === true
      && (item.backdrop_filter || "").includes("blur(")
      && item.box_shadow !== "none"
      && item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label === true
      && item.svg_icon_present === true
      && item.readable === true
      && item.contrast_ratio >= 4.5
      && item.label_nowrap_ready === true
    ));
  const chromeBarTranslucencyDetails = Array.from(document.querySelectorAll(".tg-thread-header,.tg-compose-wrap"))
    .filter(elementVisible)
    .map((node) => {
      const style = getComputedStyle(node);
      const bgColor = effectiveBackground(node);
      const bgLuminance = relativeLuminance(bgColor);
      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
      const backgroundImageText = style.backgroundImage || "";
      const backgroundLayerCount = (backgroundImageText.match(/gradient\\(/g) || []).length;
      const backgroundRepeatingLayerCount = (backgroundImageText.match(/repeating-linear-gradient/g) || []).length;
      const specularLayerCount = (style.boxShadow.match(/\\binset\\b/g) || []).length;
      const directAlpha = directBackgroundAlpha(style);
      const directColor = parseCssColor(style.backgroundColor);
      const directChannelSpread = colorChannelSpread(directColor);
      const blurPx = Number.parseFloat((backdrop.split("blur(")[1] || "").split("px")[0] || "0");
      const translucentChromeReady = directAlpha >= 0.42 && directAlpha <= 0.72
        && bgLuminance >= 0.72 && bgLuminance <= 0.98
        && backdrop.includes("blur(")
        && blurPx >= 20;
      return {
        selector: node.className || node.tagName.toLowerCase(),
        background_color: style.backgroundColor,
        background_image: backgroundImageText && backgroundImageText !== "none" ? "present" : "none",
        background_image_sample: backgroundImageText.slice(0, 180),
        chrome_refraction_layer_count: backgroundLayerCount,
        chrome_refraction_repeating_layer_count: backgroundRepeatingLayerCount,
        background_alpha: Number(directAlpha.toFixed(2)),
        background_channel_spread: Number(directChannelSpread.toFixed(1)),
        clear_white_balance_ready: directChannelSpread <= 10,
        effective_luminance: Number(bgLuminance.toFixed(3)),
        backdrop_filter: backdrop,
        backdrop_blur_px: Number((Number.isFinite(blurPx) ? blurPx : 0).toFixed(2)),
        specular_layer_count: specularLayerCount,
        box_shadow: compactShadow(style.boxShadow),
        border_color: style.borderTopColor,
        translucent_chrome_ready: translucentChromeReady,
        refractive_chrome_ready: translucentChromeReady
          && backgroundLayerCount >= 2
          && backgroundRepeatingLayerCount >= 1
          && backgroundImageText.includes("255, 255, 255")
          && specularLayerCount >= 2,
        ...richRect(node),
      };
    });
  const chromeBarTranslucencyLightGlassReady = chromeBarTranslucencyDetails.length >= 2
    && chromeBarTranslucencyDetails.every((item) => item.translucent_chrome_ready === true);
  const chromeRefractiveSkinLightGlassReady = chromeBarTranslucencyLightGlassReady
    && chromeBarTranslucencyDetails.every((item) => item.refractive_chrome_ready === true);
  const primaryShellSurfaceDetails = Array.from(document.querySelectorAll(".tg-conversation-rail,.tg-thread-panel,.tg-compose-bar,.tg-bubble"))
    .filter(elementVisible)
    .map((node) => {
	      const style = getComputedStyle(node);
	      const bgColor = effectiveBackground(node);
	      const textColor = parseCssColor(style.color);
	      const bgLuminance = relativeLuminance(bgColor);
	      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	      const directColor = parseCssColor(style.backgroundColor);
	      const directChannelSpread = colorChannelSpread(directColor);
	      const backgroundImageText = style.backgroundImage || "";
	      const backgroundBlendModeText = style.backgroundBlendMode || "";
	      const backgroundSizeText = style.backgroundSize || "";
	      const backgroundPositionText = style.backgroundPosition || "";
	      const gradientLayerCount = (backgroundImageText.match(/gradient\\(/g) || []).length;
	      const lensScaleLayers = backgroundSizeText.split(",").map((item) => item.trim()).filter(Boolean);
	      const layerScaleParallaxSizeCount = new Set(lensScaleLayers).size;
	      const layerScaleParallaxReady = gradientLayerCount >= 6 && lensScaleLayers.length >= 2 && layerScaleParallaxSizeCount >= 2;
	      const phasePositionLayers = backgroundPositionText.split(",").map((item) => item.trim()).filter(Boolean);
	      const phasePositionCount = new Set(phasePositionLayers).size;
	      const phaseYAxisCount = new Set(phasePositionLayers.map((item) => {
	        const parts = item.split(/\\s+/).filter(Boolean);
	        return parts.length > 1 ? parts.slice(1).join(" ") : "50%";
	      })).size;
	      const phaseSeparatedRefractionReady = gradientLayerCount >= 6 && phasePositionCount >= 6;
	      const twoAxisPhaseRefractionReady = phaseSeparatedRefractionReady && phaseYAxisCount >= 3;
	      const biaxialMagnificationReady = gradientLayerCount >= 6 && /\\d+% \\d+%/.test(backgroundSizeText);
	      const anisotropicMagnificationReady = gradientLayerCount >= 6 && (
	        backgroundSizeText.includes("128% 132%") || backgroundSizeText.includes("126% 134%")
	      );
	      const microRefractionLineCount = (backgroundImageText.match(/repeating-linear-gradient/g) || []).length;
	      const microRefractionAngles = Array.from(backgroundImageText.matchAll(/repeating-linear-gradient\\(([-\\d.]+)deg/g))
	        .map((match) => Number(match[1]))
	        .filter(Number.isFinite);
	      const microRefractionReady = microRefractionLineCount >= 1
	        && microRefractionAngles.some((angle) => angle >= 90 && angle <= 110);
	      const sparkleGlintCount = (backgroundImageText.match(/radial-gradient/g) || []).length;
	      const sparkleGlintReady = sparkleGlintCount >= 1 && backgroundImageText.includes("255, 255, 255");
	      const lensBloomReady = sparkleGlintCount >= 2 && (
	        backgroundImageText.includes("223, 255, 233") || backgroundImageText.includes("223 255 233")
	      );
	      const radialFocalSignatures = Array.from(backgroundImageText.matchAll(/radial-gradient\\(at\\s+([^,]+),/g))
	        .map((match) => match[1].trim().replace(/\\s+/g, " "));
	      const radialFocalSignature = radialFocalSignatures.join("|");
	      const radialFocalCount = new Set(radialFocalSignatures).size;
	      const blurPx = Number.parseFloat((backdrop.split("blur(")[1] || "").split("px")[0] || "0");
	      const causticLayerCount = (backgroundImageText.match(/linear-gradient/g) || []).length;
	      const causticHighlightPresent = backgroundImageText.includes("255, 255, 255");
	      const prismaticPinkPresent = backgroundImageText.includes("255, 223, 244") || backgroundImageText.includes("255 223 244");
	      const prismaticMintPresent = backgroundImageText.includes("223, 255, 233") || backgroundImageText.includes("223 255 233");
	      const facetedReflectionAngles = Array.from(backgroundImageText.matchAll(/linear-gradient\\(([-\\d.]+)deg/g))
	        .map((match) => Number(match[1]))
	        .filter(Number.isFinite);
	      const facetedReflectionAngleCount = new Set(facetedReflectionAngles.map((angle) => Math.round(angle))).size;
	      const spectralAngleSignature = Array.from(new Set(facetedReflectionAngles.map((angle) => Math.round(angle)))).join("/");
	      const specularLayerCount = (style.boxShadow.match(/\\binset\\b/g) || []).length;
	      const shadowColorLayerCount = (style.boxShadow.match(/rgba?\\(/g) || []).length;
	      const specularOutlineColor = parseCssColor(style.outlineColor);
	      const specularOutlineWidth = Number.parseFloat(style.outlineWidth || "0");
	      const specularOutlineOffset = Number.parseFloat(style.outlineOffset || "0");
	      const specularOutlineReady = style.outlineStyle === "solid"
	        && specularOutlineWidth >= 1
	        && specularOutlineOffset <= -1
	        && !!specularOutlineColor
	        && specularOutlineColor.a >= 0.35;
	      return {
	        selector: node.className || node.tagName.toLowerCase(),
	        text: visibleText(node).slice(0, 80),
        visible: elementVisible(node),
		        background_color: style.backgroundColor,
		        background_image: backgroundImageText && backgroundImageText !== "none" ? "present" : "none",
		        background_image_sample: backgroundImageText.slice(0, 220),
		        background_position: backgroundPositionText,
		        phase_position_count: phasePositionCount,
		        phase_y_axis_count: phaseYAxisCount,
		        phase_separated_refraction_ready: phaseSeparatedRefractionReady,
		        two_axis_phase_refraction_ready: twoAxisPhaseRefractionReady,
		        background_size: backgroundSizeText,
	        micro_refraction_line_count: microRefractionLineCount,
	        micro_refraction_angles: microRefractionAngles.map((angle) => Math.round(angle)),
	        micro_refraction_ready: microRefractionReady,
	        sparkle_glint_count: sparkleGlintCount,
	        sparkle_glint_ready: sparkleGlintReady,
	        lens_bloom_count: sparkleGlintCount,
	        lens_bloom_ready: lensBloomReady,
	        radial_focal_signature: radialFocalSignature,
	        radial_focal_layer_count: radialFocalSignatures.length,
	        radial_focal_count: radialFocalCount,
	        spectral_fusion_layer_count: gradientLayerCount,
	        spectral_fusion_blend_mode: backgroundBlendModeText,
	        spectral_fusion_ready: gradientLayerCount >= 6 && backgroundBlendModeText.includes("screen"),
	        optical_magnification_size: backgroundSizeText,
	        optical_magnification_ready: gradientLayerCount >= 6 && backgroundSizeText.includes("%"),
	        biaxial_magnification_size: backgroundSizeText,
	        biaxial_magnification_ready: biaxialMagnificationReady,
	        anisotropic_magnification_size: backgroundSizeText,
	        anisotropic_magnification_ready: anisotropicMagnificationReady,
	        lens_scale_layer_count: lensScaleLayers.length,
	        lens_scale_parallax_size_count: layerScaleParallaxSizeCount,
	        layer_scale_parallax_ready: layerScaleParallaxReady,
		        spectral_angle_signature: spectralAngleSignature,
		        spectral_angle_layer_count: facetedReflectionAngles.length,
		        spectral_angle_count: facetedReflectionAngleCount,
		        caustic_layer_count: causticLayerCount,
		        caustic_highlight_present: causticHighlightPresent,
		        caustic_highlight_ready: causticLayerCount >= 2 && causticHighlightPresent,
		        faceted_reflection_angles: facetedReflectionAngles.map((angle) => Math.round(angle)),
		        faceted_reflection_angle_count: facetedReflectionAngleCount,
		        faceted_reflection_ready: facetedReflectionAngleCount >= 3 && causticHighlightPresent && prismaticPinkPresent && prismaticMintPresent,
	        prismatic_pink_present: prismaticPinkPresent,
	        prismatic_mint_present: prismaticMintPresent,
	        prismatic_dispersion_ready: prismaticPinkPresent && prismaticMintPresent,
	        background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
	        background_channel_spread: Number(directChannelSpread.toFixed(1)),
	        clear_white_balance_ready: directChannelSpread <= 10,
        translucent_ready: translucentGlassReady(style),
        effective_luminance: Number(bgLuminance.toFixed(3)),
        light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
	        border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	        backdrop_filter: backdrop,
	        backdrop_blur_px: Number((Number.isFinite(blurPx) ? blurPx : 0).toFixed(2)),
	        box_shadow: compactShadow(style.boxShadow),
	        specular_layer_count: specularLayerCount,
	        beveled_rim_layer_count: shadowColorLayerCount,
	        beveled_rim_ready: shadowColorLayerCount >= 5 && specularOutlineReady,
	        specular_outline_width: Number((Number.isFinite(specularOutlineWidth) ? specularOutlineWidth : 0).toFixed(2)),
	        specular_outline_offset: Number((Number.isFinite(specularOutlineOffset) ? specularOutlineOffset : 0).toFixed(2)),
	        specular_outline_alpha: specularOutlineColor ? Number(specularOutlineColor.a.toFixed(2)) : 0,
	        specular_edge_ready: specularLayerCount >= 2 || specularOutlineReady,
	        contrast_ratio: Number(ratio.toFixed(2)),
	        readable: ratio >= 4.5 || visibleText(node).length === 0,
	        ...richRect(node),
      };
    });
  const primaryShellLightGlassReady = primaryShellSurfaceDetails.length >= 3
    && primaryShellSurfaceDetails.every((item) => (
      item.visible
      && item.light_glass_ready === true
      && item.translucent_ready === true
      && item.effective_luminance >= 0.72
      && item.effective_luminance <= 0.98
      && item.border_radius >= 10
      && (item.backdrop_filter || "").includes("blur(")
	      && item.box_shadow !== "none"
	      && item.readable === true
	    ));
	  const bodyDirectColor = parseCssColor(getComputedStyle(document.body).backgroundColor);
	  const bodyChannelSpread = colorChannelSpread(bodyDirectColor);
	  const primaryClearWhiteBalanceReady = primaryShellSurfaceDetails.length >= 3
	    && primaryShellSurfaceDetails.every((item) => item.clear_white_balance_ready === true);
	  const chromeClearWhiteBalanceReady = chromeBarTranslucencyDetails.length >= 2
	    && chromeBarTranslucencyDetails.every((item) => item.clear_white_balance_ready === true);
	  const clearWhiteBalanceLightGlassReady = bodyChannelSpread <= 10
	    && primaryClearWhiteBalanceReady
	    && chromeClearWhiteBalanceReady;
	  const clearWhiteBalanceDetails = {
	    body_background_color: getComputedStyle(document.body).backgroundColor,
	    body_background_channel_spread: Number(bodyChannelSpread.toFixed(1)),
	    primary_surface_channel_spread_max: Math.max(...primaryShellSurfaceDetails.map((item) => item.background_channel_spread ?? 255)),
	    chrome_channel_spread_max: Math.max(...chromeBarTranslucencyDetails.map((item) => item.background_channel_spread ?? 255)),
	    body_clear_white_ready: bodyChannelSpread <= 10,
	    primary_clear_white_ready: primaryClearWhiteBalanceReady,
	    chrome_clear_white_ready: chromeClearWhiteBalanceReady,
	  };
	  const chamferCutEdgeSurfaceDetails = Array.from(document.querySelectorAll(".tg-bubble,.tg-chat-item.active"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const clipPath = style.clipPath || style.webkitClipPath || "";
	      const filterText = style.filter || "";
	      const polygonReady = clipPath.includes("polygon(") && clipPath.includes("9px") && clipPath.includes("calc(");
	      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	      const prismaticCutEdgeReady = polygonReady && dropShadowCount >= 2;
	      return {
	        selector: node.className || node.tagName.toLowerCase(),
	        text: visibleText(node).slice(0, 80),
	        clip_path: clipPath && clipPath !== "none" ? "present" : "none",
	        clip_path_sample: clipPath.slice(0, 160),
	        polygon_clip_ready: polygonReady,
	        filter: filterText && filterText !== "none" ? "present" : "none",
	        filter_sample: filterText.slice(0, 180),
	        cut_edge_drop_shadow_count: dropShadowCount,
	        prismatic_cut_edge_ready: prismaticCutEdgeReady,
	        border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	        box_shadow: compactShadow(style.boxShadow),
	        ...richRect(node),
	      };
	    });
	  const chamferBubbleCount = chamferCutEdgeSurfaceDetails.filter((item) => item.selector.includes("tg-bubble")).length;
	  const chamferActiveCardCount = chamferCutEdgeSurfaceDetails.filter((item) => item.selector.includes("tg-chat-item") && item.selector.includes("active")).length;
	  const chamferCutEdgeLightGlassReady = chamferBubbleCount >= 3
	    && (!railVisible || chamferActiveCardCount >= 1)
	    && chamferCutEdgeSurfaceDetails.every((item) => item.polygon_clip_ready === true && item.box_shadow !== "none");
	  const prismaticCutEdgeLightGlassReady = chamferCutEdgeLightGlassReady
	    && chamferCutEdgeSurfaceDetails.every((item) => item.prismatic_cut_edge_ready === true);
	  const panePrismaticPerimeterDetails = Array.from(document.querySelectorAll(".tg-conversation-rail,.tg-thread-panel,.tg-room-panel"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const filterText = style.filter || "";
	      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	      return {
	        selector: node.className || node.tagName.toLowerCase(),
	        filter: filterText && filterText !== "none" ? "present" : "none",
	        filter_sample: filterText.slice(0, 180),
	        perimeter_drop_shadow_count: dropShadowCount,
	        pane_prismatic_perimeter_ready: dropShadowCount >= 2,
	        border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	        box_shadow: compactShadow(style.boxShadow),
	        ...richRect(node),
	      };
	    });
	  const panePrismaticPerimeterLightGlassReady = panePrismaticPerimeterDetails.length >= 1
	    && panePrismaticPerimeterDetails.every((item) => item.pane_prismatic_perimeter_ready === true && item.box_shadow !== "none");
	  const composerPrismaticControlDetails = Array.from(document.querySelectorAll(".tg-compose-bar,[data-agent-chat-send]"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const filterText = style.filter || "";
	      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	      return {
	        selector: node.className || node.getAttribute("data-agent-chat-send") || node.tagName.toLowerCase(),
	        filter: filterText && filterText !== "none" ? "present" : "none",
	        filter_sample: filterText.slice(0, 180),
	        control_drop_shadow_count: dropShadowCount,
	        composer_prismatic_control_ready: dropShadowCount >= 2,
	        backdrop_filter: backdrop,
	        border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	        box_shadow: compactShadow(style.boxShadow),
	        ...richRect(node),
	      };
	    });
	  const composerPrismaticControlLightGlassReady = composerPrismaticControlDetails.length >= 2
	    && composerPrismaticControlDetails.every((item) => (
	      item.composer_prismatic_control_ready === true
	      && item.box_shadow !== "none"
	      && (item.backdrop_filter || "").includes("blur(")
	    ));
	  const specularEdgeDetails = primaryShellSurfaceDetails.map((item) => ({
	    selector: item.selector,
	    specular_layer_count: item.specular_layer_count,
	    specular_outline_width: item.specular_outline_width,
	    specular_outline_offset: item.specular_outline_offset,
	    specular_outline_alpha: item.specular_outline_alpha,
	    specular_edge_ready: item.specular_edge_ready,
	    box_shadow: item.box_shadow,
	    width: item.width,
	    height: item.height,
	  }));
	  const specularEdgeLightGlassReady = primaryShellSurfaceDetails.length >= 3
	    && primaryShellSurfaceDetails.every((item) => item.specular_edge_ready === true);
	  const prismaticDispersionDetails = primaryShellSurfaceDetails.map((item) => ({
	    selector: item.selector,
	    prismatic_pink_present: item.prismatic_pink_present,
	    prismatic_mint_present: item.prismatic_mint_present,
	    prismatic_dispersion_ready: item.prismatic_dispersion_ready,
	    background_image: item.background_image,
	    background_image_sample: item.background_image_sample,
	    width: item.width,
	    height: item.height,
	  }));
	  const prismaticDispersionLightGlassReady = primaryShellSurfaceDetails.length >= 3
	    && primaryShellSurfaceDetails.every((item) => item.prismatic_dispersion_ready === true);
	  const causticHighlightDetails = primaryShellSurfaceDetails.map((item) => ({
	    selector: item.selector,
	    caustic_layer_count: item.caustic_layer_count,
	    caustic_highlight_present: item.caustic_highlight_present,
	    caustic_highlight_ready: item.caustic_highlight_ready,
	    background_image: item.background_image,
	    background_image_sample: item.background_image_sample,
	    width: item.width,
	    height: item.height,
	  }));
		  const causticHighlightLightGlassReady = primaryShellSurfaceDetails.length >= 3
		    && primaryShellSurfaceDetails.every((item) => item.caustic_highlight_ready === true);
		  const causticDepthShiftDetails = primaryShellSurfaceDetails.map((item) => ({
		    selector: item.selector,
		    caustic_highlight_ready: item.caustic_highlight_ready,
		    background_position: item.background_position,
		    phase_position_count: item.phase_position_count,
		    phase_y_axis_count: item.phase_y_axis_count,
		    phase_separated_refraction_ready: item.phase_separated_refraction_ready,
		    two_axis_phase_refraction_ready: item.two_axis_phase_refraction_ready,
		    width: item.width,
		    height: item.height,
		  }));
		  const causticDepthShiftKeyCount = new Set(causticDepthShiftDetails.map((item) => item.background_position)).size;
		  const phaseSeparatedRefractionLightGlassReady = primaryShellSurfaceDetails.length >= 3
		    && primaryShellSurfaceDetails.every((item) => item.phase_separated_refraction_ready === true);
		  const causticDepthShiftLightGlassReady = primaryShellSurfaceDetails.length >= 3
		    && (causticDepthShiftKeyCount >= 2 || phaseSeparatedRefractionLightGlassReady)
		    && primaryShellSurfaceDetails.every((item) => item.caustic_highlight_ready === true);
		  const opticalThicknessTierDetails = primaryShellSurfaceDetails.map((item) => ({
		    selector: item.selector,
		    background_alpha: item.background_alpha,
		    backdrop_blur_px: item.backdrop_blur_px,
		    background_position: item.background_position,
		    background_size: item.background_size,
		    caustic_highlight_ready: item.caustic_highlight_ready,
		    width: item.width,
		    height: item.height,
		  }));
		  const opticalThicknessBlurTierCount = new Set(opticalThicknessTierDetails.map((item) => item.backdrop_blur_px)).size;
		  const opticalThicknessAlphaTierCount = new Set(opticalThicknessTierDetails.map((item) => item.background_alpha)).size;
		  const opticalThicknessTiersLightGlassReady = primaryShellSurfaceDetails.length >= 3
		    && opticalThicknessBlurTierCount >= 3
		    && opticalThicknessAlphaTierCount >= 3
		    && primaryShellSurfaceDetails.every((item) => item.caustic_highlight_ready === true && item.background_alpha >= 0.38 && item.background_alpha <= 0.49);
		  const facetedReflectionDetails = primaryShellSurfaceDetails.map((item) => ({
		    selector: item.selector,
		    faceted_reflection_angles: item.faceted_reflection_angles,
		    faceted_reflection_angle_count: item.faceted_reflection_angle_count,
		    faceted_reflection_ready: item.faceted_reflection_ready,
		    background_image: item.background_image,
		    background_image_sample: item.background_image_sample,
		    width: item.width,
		    height: item.height,
		  }));
		  const facetedReflectionLightGlassReady = primaryShellSurfaceDetails.length >= 3
		    && primaryShellSurfaceDetails.every((item) => item.faceted_reflection_ready === true);
		  const beveledRimDetails = primaryShellSurfaceDetails.map((item) => ({
		    selector: item.selector,
		    beveled_rim_layer_count: item.beveled_rim_layer_count,
		    beveled_rim_ready: item.beveled_rim_ready,
		    box_shadow: item.box_shadow,
		    specular_outline_width: item.specular_outline_width,
		    specular_outline_offset: item.specular_outline_offset,
		    width: item.width,
		    height: item.height,
		  }));
			  const beveledRimLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.beveled_rim_ready === true);
			  const microRefractionDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    micro_refraction_line_count: item.micro_refraction_line_count,
			    micro_refraction_ready: item.micro_refraction_ready,
			    background_image: item.background_image,
			    background_image_sample: item.background_image_sample,
			    width: item.width,
			    height: item.height,
			  }));
			  const microRefractionLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.micro_refraction_ready === true);
			  const sparkleGlintDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    sparkle_glint_count: item.sparkle_glint_count,
			    sparkle_glint_ready: item.sparkle_glint_ready,
			    background_image: item.background_image,
			    background_image_sample: item.background_image_sample,
			    width: item.width,
			    height: item.height,
			  }));
			  const sparkleGlintLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.sparkle_glint_ready === true);
			  const lensBloomDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    lens_bloom_count: item.lens_bloom_count,
			    lens_bloom_ready: item.lens_bloom_ready,
			    background_image: item.background_image,
			    background_image_sample: item.background_image_sample,
			    width: item.width,
			    height: item.height,
			  }));
			  const lensBloomLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.lens_bloom_ready === true);
			  const spectralFusionDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    spectral_fusion_layer_count: item.spectral_fusion_layer_count,
			    spectral_fusion_blend_mode: item.spectral_fusion_blend_mode,
			    spectral_fusion_ready: item.spectral_fusion_ready,
			    background_image: item.background_image,
			    background_image_sample: item.background_image_sample,
			    width: item.width,
			    height: item.height,
			  }));
			  const spectralFusionLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.spectral_fusion_ready === true);
			  const opticalMagnificationDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    optical_magnification_size: item.optical_magnification_size,
			    optical_magnification_ready: item.optical_magnification_ready,
			    spectral_fusion_layer_count: item.spectral_fusion_layer_count,
			    background_image: item.background_image,
			    background_image_sample: item.background_image_sample,
			    width: item.width,
			    height: item.height,
			  }));
			  const opticalMagnificationLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.optical_magnification_ready === true);
			  const biaxialMagnificationDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    biaxial_magnification_size: item.biaxial_magnification_size,
			    biaxial_magnification_ready: item.biaxial_magnification_ready,
			    spectral_fusion_layer_count: item.spectral_fusion_layer_count,
			    background_image: item.background_image,
			    background_image_sample: item.background_image_sample,
			    width: item.width,
			    height: item.height,
			  }));
			  const biaxialMagnificationLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.biaxial_magnification_ready === true);
			  const anisotropicMagnificationDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    anisotropic_magnification_size: item.anisotropic_magnification_size,
			    anisotropic_magnification_ready: item.anisotropic_magnification_ready,
			    spectral_fusion_layer_count: item.spectral_fusion_layer_count,
			    background_image: item.background_image,
			    background_image_sample: item.background_image_sample,
			    width: item.width,
			    height: item.height,
			  }));
			  const anisotropicMagnificationLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.anisotropic_magnification_ready === true);
			  const phaseSeparatedRefractionDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    phase_position_count: item.phase_position_count,
			    phase_y_axis_count: item.phase_y_axis_count,
			    phase_separated_refraction_ready: item.phase_separated_refraction_ready,
			    background_position: item.background_position,
			    spectral_fusion_layer_count: item.spectral_fusion_layer_count,
			    width: item.width,
			    height: item.height,
			  }));
			  const twoAxisPhaseRefractionDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    phase_position_count: item.phase_position_count,
			    phase_y_axis_count: item.phase_y_axis_count,
			    two_axis_phase_refraction_ready: item.two_axis_phase_refraction_ready,
			    background_position: item.background_position,
			    spectral_fusion_layer_count: item.spectral_fusion_layer_count,
			    width: item.width,
			    height: item.height,
			  }));
			  const twoAxisPhaseRefractionLightGlassReady = primaryShellSurfaceDetails.length >= 3
			    && primaryShellSurfaceDetails.every((item) => item.two_axis_phase_refraction_ready === true);
			  const surfacePhaseDriftPositionCount = new Set(primaryShellSurfaceDetails.map((item) => item.background_position)).size;
			  const surfacePhaseDriftDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    background_position: item.background_position,
			    surface_phase_drift_position_count: surfacePhaseDriftPositionCount,
			    two_axis_phase_refraction_ready: item.two_axis_phase_refraction_ready,
			    width: item.width,
			    height: item.height,
			  }));
			  const surfacePhaseDriftLightGlassReady = twoAxisPhaseRefractionLightGlassReady
			    && surfacePhaseDriftPositionCount >= 2;
			  const surfaceLensScaleDriftSizeCount = new Set(primaryShellSurfaceDetails.map((item) => item.background_size)).size;
			  const surfaceLensScaleDriftDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    background_size: item.background_size,
			    surface_lens_scale_drift_size_count: surfaceLensScaleDriftSizeCount,
			    anisotropic_magnification_ready: item.anisotropic_magnification_ready,
			    two_axis_phase_refraction_ready: item.two_axis_phase_refraction_ready,
			    width: item.width,
			    height: item.height,
			  }));
			  const surfaceLensScaleDriftLightGlassReady = surfacePhaseDriftLightGlassReady
			    && surfaceLensScaleDriftSizeCount >= 2
			    && primaryShellSurfaceDetails.every((item) => item.anisotropic_magnification_ready === true);
			  const layerScaleParallaxDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    background_size: item.background_size,
			    lens_scale_layer_count: item.lens_scale_layer_count,
			    lens_scale_parallax_size_count: item.lens_scale_parallax_size_count,
			    layer_scale_parallax_ready: item.layer_scale_parallax_ready,
			    surface_lens_scale_drift_size_count: surfaceLensScaleDriftSizeCount,
			    width: item.width,
			    height: item.height,
			  }));
			  const layerScaleParallaxLightGlassReady = surfaceLensScaleDriftLightGlassReady
			    && primaryShellSurfaceDetails.every((item) => item.layer_scale_parallax_ready === true);
			  const surfaceSpectralAngleDriftSignatureCount = new Set(primaryShellSurfaceDetails.map((item) => item.spectral_angle_signature)).size;
			  const surfaceSpectralAngleDriftDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    spectral_angle_signature: item.spectral_angle_signature,
			    spectral_angle_layer_count: item.spectral_angle_layer_count,
			    spectral_angle_count: item.spectral_angle_count,
			    surface_spectral_angle_drift_signature_count: surfaceSpectralAngleDriftSignatureCount,
			    layer_scale_parallax_ready: item.layer_scale_parallax_ready,
			    width: item.width,
			    height: item.height,
			  }));
			  const surfaceSpectralAngleDriftLightGlassReady = layerScaleParallaxLightGlassReady
			    && surfaceSpectralAngleDriftSignatureCount >= 2
			    && primaryShellSurfaceDetails.every((item) => item.spectral_angle_layer_count >= 4 && item.spectral_angle_count >= 4);
			  const surfaceGlintFocalDriftSignatureCount = new Set(primaryShellSurfaceDetails.map((item) => item.radial_focal_signature)).size;
			  const surfaceGlintFocalDriftDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    radial_focal_signature: item.radial_focal_signature,
			    radial_focal_layer_count: item.radial_focal_layer_count,
			    radial_focal_count: item.radial_focal_count,
			    surface_glint_focal_drift_signature_count: surfaceGlintFocalDriftSignatureCount,
			    surface_spectral_angle_drift_ready: surfaceSpectralAngleDriftLightGlassReady,
			    width: item.width,
			    height: item.height,
			  }));
			  const surfaceGlintFocalDriftLightGlassReady = surfaceSpectralAngleDriftLightGlassReady
			    && surfaceGlintFocalDriftSignatureCount >= 2
			    && primaryShellSurfaceDetails.every((item) => item.radial_focal_layer_count >= 2 && item.radial_focal_count >= 2);
			  const threadGlintFocalSignatures = new Set(primaryShellSurfaceDetails
			    .filter((item) => item.selector.includes("tg-thread-panel"))
			    .map((item) => item.radial_focal_signature));
			  const composerGlintFocalDecouplingDetails = primaryShellSurfaceDetails
			    .filter((item) => item.selector.includes("tg-compose-bar"))
			    .map((item) => ({
			      selector: item.selector,
			      radial_focal_signature: item.radial_focal_signature,
			      radial_focal_layer_count: item.radial_focal_layer_count,
			      radial_focal_count: item.radial_focal_count,
			      thread_radial_focal_signatures: Array.from(threadGlintFocalSignatures),
			      surface_glint_focal_drift_signature_count: surfaceGlintFocalDriftSignatureCount,
			      composer_focal_decoupled: !threadGlintFocalSignatures.has(item.radial_focal_signature),
			      width: item.width,
			      height: item.height,
			    }));
			  const composerGlintFocalDecouplingLightGlassReady = surfaceGlintFocalDriftLightGlassReady
			    && surfaceGlintFocalDriftSignatureCount >= 3
			    && composerGlintFocalDecouplingDetails.length >= 1
			    && composerGlintFocalDecouplingDetails.every((item) => item.composer_focal_decoupled === true
			      && item.radial_focal_layer_count >= 2
			      && item.radial_focal_count >= 2);
			  const threadSpectralAngleSignatures = new Set(primaryShellSurfaceDetails
			    .filter((item) => item.selector.includes("tg-thread-panel"))
			    .map((item) => item.spectral_angle_signature));
			  const composerSpectralAngleDecouplingDetails = primaryShellSurfaceDetails
			    .filter((item) => item.selector.includes("tg-compose-bar"))
			    .map((item) => ({
			      selector: item.selector,
			      spectral_angle_signature: item.spectral_angle_signature,
			      spectral_angle_layer_count: item.spectral_angle_layer_count,
			      spectral_angle_count: item.spectral_angle_count,
			      thread_spectral_angle_signatures: Array.from(threadSpectralAngleSignatures),
			      surface_spectral_angle_drift_signature_count: surfaceSpectralAngleDriftSignatureCount,
			      composer_spectral_angle_decoupled: !threadSpectralAngleSignatures.has(item.spectral_angle_signature),
			      width: item.width,
			      height: item.height,
			    }));
			  const composerSpectralAngleDecouplingLightGlassReady = composerGlintFocalDecouplingLightGlassReady
			    && surfaceSpectralAngleDriftSignatureCount >= 3
			    && composerSpectralAngleDecouplingDetails.length >= 1
			    && composerSpectralAngleDecouplingDetails.every((item) => item.composer_spectral_angle_decoupled === true
			      && item.spectral_angle_layer_count >= 4
			      && item.spectral_angle_count >= 4);
			  const threadPhaseSignatures = new Set(primaryShellSurfaceDetails
			    .filter((item) => item.selector.includes("tg-thread-panel"))
			    .map((item) => item.background_position));
			  const composerPhaseDecouplingDetails = primaryShellSurfaceDetails
			    .filter((item) => item.selector.includes("tg-compose-bar"))
			    .map((item) => ({
			      selector: item.selector,
			      background_position: item.background_position,
			      phase_position_count: item.phase_position_count,
			      phase_y_axis_count: item.phase_y_axis_count,
			      thread_phase_signatures: Array.from(threadPhaseSignatures),
			      surface_phase_drift_position_count: surfacePhaseDriftPositionCount,
			      composer_phase_decoupled: !threadPhaseSignatures.has(item.background_position),
			      width: item.width,
			      height: item.height,
			    }));
			  const composerPhaseDecouplingLightGlassReady = composerSpectralAngleDecouplingLightGlassReady
			    && surfacePhaseDriftPositionCount >= 3
			    && composerPhaseDecouplingDetails.length >= 1
			    && composerPhaseDecouplingDetails.every((item) => item.composer_phase_decoupled === true
			      && item.phase_position_count >= 6
			      && item.phase_y_axis_count >= 3);
			  const threadLayerScaleSignatures = new Set(primaryShellSurfaceDetails
			    .filter((item) => item.selector.includes("tg-thread-panel"))
			    .map((item) => item.background_size));
			  const composerLayerScaleDecouplingDetails = primaryShellSurfaceDetails
			    .filter((item) => item.selector.includes("tg-compose-bar"))
			    .map((item) => ({
			      selector: item.selector,
			      background_size: item.background_size,
			      lens_scale_layer_count: item.lens_scale_layer_count,
			      lens_scale_parallax_size_count: item.lens_scale_parallax_size_count,
			      thread_layer_scale_signatures: Array.from(threadLayerScaleSignatures),
			      surface_lens_scale_drift_size_count: surfaceLensScaleDriftSizeCount,
			      composer_layer_scale_decoupled: !threadLayerScaleSignatures.has(item.background_size),
			      width: item.width,
			      height: item.height,
			    }));
			  const composerLayerScaleDecouplingLightGlassReady = composerPhaseDecouplingLightGlassReady
			    && surfaceLensScaleDriftSizeCount >= 3
			    && composerLayerScaleDecouplingDetails.length >= 1
			    && composerLayerScaleDecouplingDetails.every((item) => item.composer_layer_scale_decoupled === true
			      && item.lens_scale_layer_count >= 2
			      && item.lens_scale_parallax_size_count >= 2);
			  const menuTriggers = Array.from(document.querySelectorAll("summary[data-control-ui-menu-trigger='icon']"));
  const menuTriggerDetails = menuTriggers.map((node) => {
    const icon = node.querySelector(".tg-menu-icon");
    const label = node.querySelector(".sr-only");
    const style = getComputedStyle(node);
    const visible = elementVisible(node);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const filterText = style.filter || "";
    const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
    const visibleIconText = visibleText(node);
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    return {
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      icon_present: Boolean(icon),
      svg_icon_present: hasSvgIcon(node),
      sr_label_present: Boolean(label && (label.textContent || "").trim().length > 0),
      visible,
      visible_icon_text: visibleIconText,
      visible_icon_text_absent: visibleIconText.length === 0,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      filter: filterText && filterText !== "none" ? "present" : "none",
      filter_sample: filterText.slice(0, 180),
      icon_prismatic_control_drop_shadow_count: dropShadowCount,
      icon_prismatic_control_ready: dropShadowCount >= 2,
      ...richRect(node),
    };
  });
  const menuTriggerReady = menuTriggerDetails.length >= 2
    && menuTriggerDetails.every((item) => (
      item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.icon_present
      && item.sr_label_present
    ))
    && menuTriggerDetails.every((item) => (
      item.visible
      && item.width >= 44
      && item.height >= 44
      && item.border_radius >= 20
      && /blur\\(/.test(item.backdrop_filter)
      && item.box_shadow !== "none"
      && item.svg_icon_present
      && item.visible_icon_text_absent
    ));
  const folderChips = Array.from(document.querySelectorAll(".tg-folder-chip")).filter(elementVisible);
  const folderChipDetails = folderChips.map((node) => {
    const style = getComputedStyle(node);
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const ariaPressed = node.getAttribute("aria-pressed") || "";
    const active = node.classList.contains("active");
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const filterText = style.filter || "";
    const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
    const textShadow = style.textShadow || "";
    const textShadowCount = textShadow && textShadow !== "none" ? ((textShadow.match(/rgb/g) || []).length || 1) : 0;
    const textColor = parseCssColor(style.color);
    const bgColor = effectiveBackground(node);
    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
    return {
      key: node.getAttribute("data-chat-folder") || "",
      text: visibleText(node),
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      aria_pressed: ariaPressed,
      active,
      active_state_matches_aria_pressed: active ? ariaPressed === "true" : ariaPressed === "false",
      visible: true,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      filter: filterText && filterText !== "none" ? "present" : "none",
      filter_sample: filterText.slice(0, 180),
      rail_filter_drop_shadow_count: dropShadowCount,
      rail_prismatic_filter_ready: dropShadowCount >= 2,
      text_shadow: textShadow && textShadow !== "none" ? "present" : "none",
      text_shadow_sample: textShadow.slice(0, 180),
      folder_chip_label_text_shadow_count: textShadowCount,
      folder_chip_label_prismatic_etch_ready: textShadowCount >= 2,
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      ...richRect(node),
    };
  });
  const folderChipTouchReady = folderChipDetails.length === 0 || (
    folderChipDetails.length >= 3
    && folderChipDetails.every((item) => (
      item.key.length > 0
      && item.text.length > 0
      && item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.active_state_matches_aria_pressed
      && item.width >= 44
      && item.height >= 44
      && item.border_radius >= 20
      && /blur\\(/.test(item.backdrop_filter)
      && item.box_shadow !== "none"
    ))
  );
  const folderChipLabelPrismaticEtchLightGlassReady = railVisible
    ? folderChipDetails.length >= 3
      && folderChipDetails.every((item) => (
        item.visible
        && item.text.length > 0
        && item.width >= 44
        && item.height >= 44
        && item.text_shadow === "present"
        && item.folder_chip_label_prismatic_etch_ready === true
        && item.folder_chip_label_text_shadow_count >= 2
        && item.readable === true
        && item.contrast_ratio >= 4.5
      ))
    : folderChipDetails.length === 0;
  const rowMenuToggles = Array.from(document.querySelectorAll("[data-chat-row-menu-toggle]"));
  const rowMenuToggleDetails = rowMenuToggles.map((node) => {
    const style = getComputedStyle(node);
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const visibleIconText = visibleText(node);
    const row = node.closest("[data-chat-conversation]");
    return {
      owner_key: row?.getAttribute("data-chat-conversation") || "",
      toggle_key: node.getAttribute("data-chat-row-menu-toggle") || "",
      marker: node.getAttribute("data-control-ui-row-menu-trigger") || "",
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      visible: elementVisible(node),
      svg_icon_present: hasSvgIcon(node),
      visible_icon_text: visibleIconText,
      visible_icon_text_absent: visibleIconText.length === 0,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      ...richRect(node),
    };
  });
  const rowMenuPanels = Array.from(document.querySelectorAll("[data-chat-row-menu-panel]"));
  const rowMenuPanelDetails = rowMenuPanels.map((node) => {
    const style = getComputedStyle(node);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const rect = richRect(node);
    const bgColor = effectiveBackground(node);
    const bgLuminance = relativeLuminance(bgColor);
    const horizontalInViewport = rect.left >= -1 && rect.right <= window.innerWidth + 1 && rect.width <= window.innerWidth - 16;
    const verticalInViewport = rect.top >= -1 && rect.bottom <= window.innerHeight + 1 && rect.height <= window.innerHeight - 16;
    const row = node.closest("[data-chat-conversation]");
    return {
      owner_key: row?.getAttribute("data-chat-conversation") || "",
      panel_key: node.getAttribute("data-chat-row-menu-panel") || "",
      marker: node.getAttribute("data-control-ui-row-menu-panel") || "",
      visible: elementVisible(node),
      item_count: node.querySelectorAll("[data-chat-row-menu-item]").length,
      background_color: style.backgroundColor,
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      effective_luminance: Number(bgLuminance.toFixed(3)),
      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      horizontal_in_viewport: horizontalInViewport,
      vertical_in_viewport: verticalInViewport,
      in_viewport: horizontalInViewport && verticalInViewport,
      top_clipped: rect.top < -1,
      bottom_clipped: rect.bottom > window.innerHeight + 1,
      ...rect,
    };
  });
  const rowMenuItemDetails = Array.from(document.querySelectorAll("[data-chat-row-menu-item]")).map((node) => {
    const labelNode = node.querySelector(".tg-row-action__label");
    const iconNode = node.querySelector(".tg-row-action__icon");
    const style = getComputedStyle(node);
    const labelStyle = labelNode ? getComputedStyle(labelNode) : null;
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const textColor = parseCssColor(style.color);
    const bgColor = effectiveBackground(node);
    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
    const row = node.closest("[data-chat-conversation]");
    return {
      owner_key: row?.getAttribute("data-chat-conversation") || "",
      key: node.getAttribute("data-chat-row-menu-item") || "",
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      color: style.color,
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      icon_present: Boolean(iconNode),
      icon_svg_present: hasSvgIcon(iconNode),
      label: (labelNode?.textContent || "").replace(/\\s+/g, " ").trim(),
      visible: elementVisible(node),
      height: Math.round(node.getBoundingClientRect().height),
      label_white_space: labelStyle?.whiteSpace || "",
      label_nowrap_ready: labelStyle?.whiteSpace === "nowrap",
      ...richRect(node),
    };
  });
  const expectedVisibleRowMenuPanelCount = railVisible ? 3 : 0;
  const visibleRowMenuToggleDetails = rowMenuToggleDetails.filter((item) => item.visible);
  const visibleRowMenuPanelDetails = rowMenuPanelDetails.filter((item) => item.visible);
  const visibleRowMenuItemDetails = rowMenuItemDetails.filter((item) => item.visible);
  const rowMenuPanelKeys = new Set(visibleRowMenuPanelDetails.map((item) => item.owner_key));
  const rowMenuAllRowsReady = !railVisible || (
    visibleRowMenuToggleDetails.length === expectedVisibleRowMenuPanelCount
    && visibleRowMenuPanelDetails.length === expectedVisibleRowMenuPanelCount
    && visibleRowMenuItemDetails.length === expectedVisibleRowMenuPanelCount * 3
    && visibleRowMenuToggleDetails.every((item) => (
      item.owner_key.length > 0
      && item.toggle_key === item.owner_key
      && item.marker === "light-glass"
      && rowMenuPanelKeys.has(item.owner_key)
    ))
    && visibleRowMenuPanelDetails.every((item) => (
      item.owner_key.length > 0
      && item.panel_key === item.owner_key
      && item.marker === "light-glass"
      && item.item_count === 3
    ))
  );
	  const rowMenuTouchReady = !railVisible || (
    rowMenuAllRowsReady
    && visibleRowMenuToggleDetails.every((item) => (
      item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.width >= 44
      && item.height >= 44
      && item.border_radius >= 20
      && /blur\\(/.test(item.backdrop_filter)
      && item.box_shadow !== "none"
      && item.svg_icon_present
      && item.visible_icon_text_absent
    ))
    && visibleRowMenuPanelDetails.every((item) => (
      item.visible
      && item.item_count >= 3
      && item.width >= 180
      && item.height >= 132
      && item.border_radius >= 16
      && /blur\\(/.test(item.backdrop_filter)
      && item.box_shadow !== "none"
      && item.in_viewport
    ))
    && visibleRowMenuItemDetails.every((item) => (
      item.owner_key.length > 0
      && item.key.length > 0
      && item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.icon_present
      && item.icon_svg_present
      && item.label.length > 0
      && item.visible
      && item.height >= 44
      && item.label_nowrap_ready
	    ))
	  );
	  const rowMenuLightGlassReady = !railVisible || (
	    rowMenuAllRowsReady
	    && visibleRowMenuPanelDetails.every((item) => (
	      item.visible
	      && item.light_glass_ready
	      && item.effective_luminance >= 0.72
	      && item.effective_luminance <= 0.98
	      && /blur\\(/.test(item.backdrop_filter)
	      && item.box_shadow !== "none"
	    ))
	    && visibleRowMenuItemDetails.every((item) => (
	      item.visible
	      && item.readable
	      && item.contrast_ratio >= 4.5
	    ))
	  );
	  const commandPalettePanel = document.querySelector("[data-control-ui-command-palette-surface]");
	  const commandPalettePanelDetails = (() => {
	    if (!commandPalettePanel) {
	      return { exists: false, visible: false };
	    }
	    const style = getComputedStyle(commandPalettePanel);
	    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	    const filter = style.filter || "";
	    const dropShadowCount = (filter.match(/drop-shadow/g) || []).length;
	    const rect = richRect(commandPalettePanel);
	    const bgColor = effectiveBackground(commandPalettePanel);
	    const bgLuminance = relativeLuminance(bgColor);
	    const horizontalInViewport = rect.left >= -1 && rect.right <= window.innerWidth + 1 && rect.width <= window.innerWidth - 16;
	    const verticalInViewport = rect.top >= -1 && rect.bottom <= window.innerHeight + 1 && rect.height <= window.innerHeight - 16;
	    return {
	      exists: true,
	      visible: elementVisible(commandPalettePanel),
	      marker: commandPalettePanel.getAttribute("data-control-ui-command-palette-surface") || "",
	      role: commandPalettePanel.getAttribute("role") || "",
	      aria_modal: commandPalettePanel.getAttribute("aria-modal") || "",
	      aria_label: commandPalettePanel.getAttribute("aria-label") || "",
	      background_color: style.backgroundColor,
	      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
	      translucent_ready: translucentGlassReady(style),
	      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
	      effective_luminance: Number(bgLuminance.toFixed(3)),
	      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
	      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	      backdrop_filter: backdrop,
	      box_shadow: compactShadow(style.boxShadow),
	      filter: filter && filter !== "none" ? "present" : "none",
	      filter_sample: filter.slice(0, 180),
	      command_palette_surface_drop_shadow_count: dropShadowCount,
	      command_palette_surface_prismatic_perimeter_ready: dropShadowCount >= 2,
	      horizontal_in_viewport: horizontalInViewport,
	      vertical_in_viewport: verticalInViewport,
	      in_viewport: horizontalInViewport && verticalInViewport,
	      ...rect,
	    };
	  })();
	  const commandPaletteBackdrop = document.querySelector(".command-palette-backdrop");
	  const commandPaletteBackdropDetails = (() => {
	    if (!commandPaletteBackdrop) {
	      return { exists: false, visible: false };
	    }
	    const style = getComputedStyle(commandPaletteBackdrop);
	    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	    const blurPx = Number.parseFloat((backdrop.split("blur(")[1] || "").split("px")[0] || "0");
	    const backgroundImage = style.backgroundImage || "";
	    const repeatingLayerCount = (backgroundImage.match(/repeating-linear-gradient/g) || []).length;
	    const rect = richRect(commandPaletteBackdrop);
	    return {
	      exists: true,
	      visible: elementVisible(commandPaletteBackdrop),
	      background_color: style.backgroundColor,
	      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
	      background_image: backgroundImage && backgroundImage !== "none" ? "present" : "none",
	      background_image_sample: backgroundImage.slice(0, 180),
	      backdrop_filter: backdrop,
	      backdrop_blur_px: Number((Number.isFinite(blurPx) ? blurPx : 0).toFixed(2)),
	      command_palette_backdrop_repeating_layer_count: repeatingLayerCount,
	      command_palette_backdrop_caustic_veil_ready: repeatingLayerCount >= 1 && blurPx >= 10,
	      covers_viewport: rect.width >= window.innerWidth - 1 && rect.height >= window.innerHeight - 1,
	      ...rect,
	    };
	  })();
	  const commandPaletteInputRow = document.querySelector("[data-control-ui-command-palette-surface] .command-palette__input-row");
	  const commandPaletteInputRowDetails = (() => {
	    if (!commandPaletteInputRow) {
	      return { exists: false, visible: false };
	    }
	    const style = getComputedStyle(commandPaletteInputRow);
	    const borderColor = parseCssColor(style.borderBottomColor);
	    const boxShadow = style.boxShadow || "";
	    const shadowCount = boxShadow && boxShadow !== "none" ? ((boxShadow.match(/rgb/g) || []).length || 1) : 0;
	    return {
	      exists: true,
	      visible: elementVisible(commandPaletteInputRow),
	      border_bottom_color: style.borderBottomColor,
	      border_bottom_alpha: borderColor ? Number(borderColor.a.toFixed(2)) : 0,
	      box_shadow: compactShadow(boxShadow),
	      box_shadow_sample: boxShadow.slice(0, 180),
	      command_palette_input_row_separator_shadow_count: shadowCount,
	      command_palette_input_row_prismatic_separator_ready: shadowCount >= 2 && !!borderColor && borderColor.a >= 0.25,
	      ...richRect(commandPaletteInputRow),
	    };
	  })();
	  const commandPaletteResultsWell = document.querySelector("[data-control-ui-command-palette-surface] .command-palette__results");
	  const commandPaletteResultsWellDetails = (() => {
	    if (!commandPaletteResultsWell) {
	      return { exists: false, visible: false };
	    }
	    const style = getComputedStyle(commandPaletteResultsWell);
	    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	    const blurPx = Number.parseFloat((backdrop.split("blur(")[1] || "").split("px")[0] || "0");
	    const bgColor = effectiveBackground(commandPaletteResultsWell);
	    const bgLuminance = relativeLuminance(bgColor);
	    const alpha = directBackgroundAlpha(style);
	    const borderColor = parseCssColor(style.borderTopColor);
	    const boxShadow = style.boxShadow || "";
	    const shadowCount = boxShadow && boxShadow !== "none" ? ((boxShadow.match(/rgb/g) || []).length || 1) : 0;
	    const radius = Number.parseFloat(style.borderTopLeftRadius || "0");
	    return {
	      exists: true,
	      visible: elementVisible(commandPaletteResultsWell),
	      background_color: style.backgroundColor,
	      background_alpha: Number(alpha.toFixed(2)),
	      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
	      effective_luminance: Number(bgLuminance.toFixed(3)),
	      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
	      border_color: style.borderTopColor,
	      border_alpha: borderColor ? Number(borderColor.a.toFixed(2)) : 0,
	      border_radius: radius,
	      box_shadow: compactShadow(boxShadow),
	      box_shadow_sample: boxShadow.slice(0, 180),
	      backdrop_filter: backdrop,
	      backdrop_blur_px: Number((Number.isFinite(blurPx) ? blurPx : 0).toFixed(2)),
	      command_palette_results_well_rim_shadow_count: shadowCount,
	      command_palette_results_well_light_glass_ready: alpha >= 0.1 && alpha <= 0.4 && blurPx >= 10,
	      command_palette_results_well_prismatic_rim_ready: shadowCount >= 2 && !!borderColor && borderColor.a >= 0.25 && radius >= 12,
	      ...richRect(commandPaletteResultsWell),
	    };
	  })();
	  const commandPaletteClose = document.querySelector("[data-control-ui-command-palette-close]");
	  const commandPaletteCloseDetails = (() => {
	    if (!commandPaletteClose) {
	      return { exists: false, visible: false };
	    }
	    const style = getComputedStyle(commandPaletteClose);
	    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	    const textColor = parseCssColor(style.color);
	    const bgColor = effectiveBackground(commandPaletteClose);
	    const bgLuminance = relativeLuminance(bgColor);
	    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	    const filter = style.filter || "";
	    const dropShadowCount = (filter.match(/drop-shadow/g) || []).length;
	    const ariaLabel = commandPaletteClose.getAttribute("aria-label") || "";
	    const title = commandPaletteClose.getAttribute("title") || "";
	    return {
	      exists: true,
	      marker: commandPaletteClose.getAttribute("data-control-ui-command-palette-close") || "",
	      href: commandPaletteClose.getAttribute("href") || "",
	      visible: elementVisible(commandPaletteClose),
	      aria_label: ariaLabel,
	      title,
	      title_matches_aria_label: title === ariaLabel,
	      svg_icon_present: hasSvgIcon(commandPaletteClose),
	      visible_icon_text: visibleText(commandPaletteClose),
	      visible_icon_text_absent: visibleText(commandPaletteClose).length === 0,
	      background_color: style.backgroundColor,
	      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
	      translucent_ready: translucentGlassReady(style),
	      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
	      effective_luminance: Number(bgLuminance.toFixed(3)),
	      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
	      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	      backdrop_filter: backdrop,
	      box_shadow: compactShadow(style.boxShadow),
	      filter: filter && filter !== "none" ? "present" : "none",
	      filter_sample: filter.slice(0, 180),
	      command_palette_close_drop_shadow_count: dropShadowCount,
	      command_palette_close_prismatic_icon_ready: dropShadowCount >= 2,
	      contrast_ratio: Number(ratio.toFixed(2)),
	      readable: ratio >= 4.5,
	      ...richRect(commandPaletteClose),
	    };
	  })();
	  const commandPaletteInput = document.querySelector("[data-control-ui-command-palette-input]");
	  const commandPaletteInputDetails = (() => {
	    if (!commandPaletteInput) {
	      return { exists: false, visible: false };
	    }
	    const style = getComputedStyle(commandPaletteInput);
	    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	    const textColor = parseCssColor(style.color);
	    const bgColor = effectiveBackground(commandPaletteInput);
	    const bgLuminance = relativeLuminance(bgColor);
	    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	    const ariaLabel = commandPaletteInput.getAttribute("aria-label") || "";
	    const title = commandPaletteInput.getAttribute("title") || "";
	    const marker = commandPaletteInput.getAttribute("data-control-ui-command-palette-input") || "";
	    const textShadow = style.textShadow || "";
	    const textShadowCount = textShadow && textShadow !== "none" ? ((textShadow.match(/rgb/g) || []).length || 1) : 0;
	    const placeholderStyle = getComputedStyle(commandPaletteInput, "::placeholder");
	    const placeholderColor = parseCssColor(placeholderStyle.color);
	    const placeholderRatio = placeholderColor ? contrastRatio(placeholderColor, bgColor) : 0;
	    const placeholderTextShadow = placeholderStyle.textShadow || "";
	    const placeholderTextShadowCount = placeholderTextShadow && placeholderTextShadow !== "none" ? ((placeholderTextShadow.match(/rgb/g) || []).length || 1) : 0;
	    const placeholderFontWeight = Number.parseFloat(placeholderStyle.fontWeight || "0") || 0;
	    return {
	      exists: true,
	      visible: elementVisible(commandPaletteInput),
	      marker,
	      aria_label: ariaLabel,
	      title,
	      title_matches_aria_label: title === ariaLabel,
	      type: commandPaletteInput.getAttribute("type") || "",
	      placeholder: commandPaletteInput.getAttribute("placeholder") || "",
	      background_color: style.backgroundColor,
	      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
	      translucent_ready: translucentGlassReady(style),
	      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
	      effective_luminance: Number(bgLuminance.toFixed(3)),
	      light_glass_ready: marker === "light-glass" && bgLuminance >= 0.72 && bgLuminance <= 0.98,
	      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	      backdrop_filter: backdrop,
	      box_shadow: compactShadow(style.boxShadow),
	      text_shadow: textShadow && textShadow !== "none" ? "present" : "none",
	      text_shadow_sample: textShadow.slice(0, 180),
	      command_palette_input_text_shadow_count: textShadowCount,
	      command_palette_input_prismatic_etch_ready: textShadowCount >= 2,
	      placeholder_color: placeholderStyle.color,
	      placeholder_text_shadow: placeholderTextShadow && placeholderTextShadow !== "none" ? "present" : "none",
	      placeholder_text_shadow_sample: placeholderTextShadow.slice(0, 180),
	      command_palette_input_placeholder_text_shadow_count: placeholderTextShadowCount,
	      command_palette_input_placeholder_font_weight: placeholderFontWeight,
	      command_palette_input_placeholder_prismatic_etch_ready: placeholderTextShadowCount >= 2 && placeholderFontWeight >= 600,
	      placeholder_contrast_ratio: Number(placeholderRatio.toFixed(2)),
	      placeholder_readable: placeholderRatio >= 4.5,
	      contrast_ratio: Number(ratio.toFixed(2)),
	      readable: ratio >= 4.5,
	      ...richRect(commandPaletteInput),
	    };
	  })();
	  const commandPaletteInputIcon = document.querySelector(".command-palette__input-row > span");
	  const commandPaletteInputIconDetails = (() => {
	    if (!commandPaletteInputIcon) {
	      return { exists: false, visible: false };
	    }
	    const style = getComputedStyle(commandPaletteInputIcon);
	    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	    const filter = style.filter || "";
	    const dropShadowCount = (filter.match(/drop-shadow/g) || []).length;
	    const textColor = parseCssColor(style.color);
	    const bgColor = effectiveBackground(commandPaletteInputIcon);
	    const bgLuminance = relativeLuminance(bgColor);
	    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	    return {
	      exists: true,
	      visible: elementVisible(commandPaletteInputIcon),
	      svg_icon_present: hasSvgIcon(commandPaletteInputIcon),
	      visible_icon_text: visibleText(commandPaletteInputIcon),
	      visible_icon_text_absent: visibleText(commandPaletteInputIcon).length === 0,
	      background_color: style.backgroundColor,
	      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
	      translucent_ready: translucentGlassReady(style),
	      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
	      effective_luminance: Number(bgLuminance.toFixed(3)),
	      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
	      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	      backdrop_filter: backdrop,
	      box_shadow: compactShadow(style.boxShadow),
	      filter: filter && filter !== "none" ? "present" : "none",
	      filter_sample: filter.slice(0, 180),
	      command_palette_input_icon_drop_shadow_count: dropShadowCount,
	      command_palette_input_icon_prismatic_ready: dropShadowCount >= 2,
	      contrast_ratio: Number(ratio.toFixed(2)),
	      readable: ratio >= 4.5,
	      ...richRect(commandPaletteInputIcon),
	    };
	  })();
	  const commandPaletteItemDetails = Array.from(document.querySelectorAll("[data-control-ui-command-palette-item]")).map((node) => {
	    const style = getComputedStyle(node);
	    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	    const borderColor = parseCssColor(style.borderTopColor);
	    const itemShadowCount = style.boxShadow && style.boxShadow !== "none" ? ((style.boxShadow.match(/rgb/g) || []).length || 1) : 0;
	    const kindNode = node.querySelector(".command-palette__kind");
	    const strong = node.querySelector("strong");
	    const detailNode = node.querySelector("small");
	    const kindStyle = kindNode ? getComputedStyle(kindNode) : style;
	    const strongStyle = strong ? getComputedStyle(strong) : style;
	    const detailStyle = detailNode ? getComputedStyle(detailNode) : style;
	    const kindColor = parseCssColor(kindStyle.color);
	    const textColor = parseCssColor(strongStyle.color);
	    const detailColor = parseCssColor(detailStyle.color);
	    const bgColor = effectiveBackground(node);
	    const kindBgColor = kindNode ? effectiveBackground(kindNode) : bgColor;
	    const bgLuminance = relativeLuminance(bgColor);
	    const kindBgLuminance = relativeLuminance(kindBgColor);
	    const kindRatio = kindColor ? contrastRatio(kindColor, kindBgColor) : 0;
	    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	    const detailRatio = detailColor ? contrastRatio(detailColor, bgColor) : 0;
	    const kindBackdrop = kindStyle.backdropFilter || kindStyle.webkitBackdropFilter || "";
	    const kindBorderColor = parseCssColor(kindStyle.borderTopColor);
	    const kindShadow = kindStyle.boxShadow || "";
	    const kindShadowCount = kindShadow && kindShadow !== "none" ? ((kindShadow.match(/rgb/g) || []).length || 1) : 0;
	    const kindRect = kindNode ? richRect(kindNode) : { width: 0, height: 0 };
	    const kindTextShadow = kindStyle.textShadow || "";
	    const labelTextShadow = strongStyle.textShadow || "";
	    const detailTextShadow = detailStyle.textShadow || "";
	    const kindTextShadowCount = kindTextShadow && kindTextShadow !== "none" ? ((kindTextShadow.match(/rgb/g) || []).length || 1) : 0;
	    const labelTextShadowCount = labelTextShadow && labelTextShadow !== "none" ? ((labelTextShadow.match(/rgb/g) || []).length || 1) : 0;
	    const detailTextShadowCount = detailTextShadow && detailTextShadow !== "none" ? ((detailTextShadow.match(/rgb/g) || []).length || 1) : 0;
	    const ariaLabel = node.getAttribute("aria-label") || "";
	    const title = node.getAttribute("title") || "";
	    return {
	      key: node.getAttribute("data-control-ui-command-palette-item") || "",
	      marker: node.getAttribute("data-control-ui-command-palette-result") || "",
	      kind: visibleText(kindNode),
	      label: visibleText(strong),
	      detail: visibleText(detailNode),
	      text: visibleText(node),
	      aria_label: ariaLabel,
	      title,
	      title_matches_aria_label: title === ariaLabel,
	      visible: elementVisible(node),
	      audit_hover: node.classList.contains("command-palette__item--audit-hover"),
	      background_color: style.backgroundColor,
	      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
	      translucent_ready: translucentGlassReady(style),
	      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
	      effective_luminance: Number(bgLuminance.toFixed(3)),
	      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
	      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	      border_color: style.borderTopColor,
	      border_alpha: borderColor ? Number(borderColor.a.toFixed(2)) : 0,
	      backdrop_filter: backdrop,
	      box_shadow: compactShadow(style.boxShadow),
	      command_palette_item_hover_shadow_count: itemShadowCount,
	      command_palette_item_hover_prismatic_ready: node.classList.contains("command-palette__item--audit-hover") && itemShadowCount >= 2 && !!borderColor && borderColor.a >= 0.25,
	      command_palette_item_rim_shadow_count: itemShadowCount,
	      command_palette_item_prismatic_rim_ready: itemShadowCount >= 2 && !!borderColor && borderColor.a >= 0.25,
	      kind_background_color: kindStyle.backgroundColor,
	      kind_background_alpha: Number(directBackgroundAlpha(kindStyle).toFixed(2)),
	      kind_effective_luminance: Number(kindBgLuminance.toFixed(3)),
	      kind_border_alpha: kindBorderColor ? Number(kindBorderColor.a.toFixed(2)) : 0,
	      kind_border_radius: Number.parseFloat(kindStyle.borderTopLeftRadius || "0"),
	      kind_width: kindRect.width || 0,
	      kind_height: kindRect.height || 0,
	      kind_backdrop_filter: kindBackdrop,
	      kind_box_shadow: compactShadow(kindShadow),
	      command_palette_kind_chip_shadow_count: kindShadowCount,
	      command_palette_kind_chip_light_glass_ready: Boolean(kindNode) && kindRect.width >= 44 && kindRect.height >= 22 && directBackgroundAlpha(kindStyle) >= 0.25 && directBackgroundAlpha(kindStyle) <= 0.75 && kindBgLuminance >= 0.72 && kindBgLuminance <= 0.98 && kindShadowCount >= 2 && !!kindBorderColor && kindBorderColor.a >= 0.25 && Number.parseFloat(kindStyle.borderTopLeftRadius || "0") >= 20 && kindBackdrop.includes("blur("),
	      kind_text_shadow: kindTextShadow && kindTextShadow !== "none" ? "present" : "none",
	      kind_text_shadow_sample: kindTextShadow.slice(0, 180),
	      label_text_shadow: labelTextShadow && labelTextShadow !== "none" ? "present" : "none",
	      label_text_shadow_sample: labelTextShadow.slice(0, 180),
	      detail_text_shadow: detailTextShadow && detailTextShadow !== "none" ? "present" : "none",
	      detail_text_shadow_sample: detailTextShadow.slice(0, 180),
	      command_palette_item_kind_text_shadow_count: kindTextShadowCount,
	      command_palette_item_label_text_shadow_count: labelTextShadowCount,
	      command_palette_item_detail_text_shadow_count: detailTextShadowCount,
	      command_palette_item_label_prismatic_etch_ready: kindTextShadowCount >= 2 && labelTextShadowCount >= 2 && detailTextShadowCount >= 2,
	      kind_contrast_ratio: Number(kindRatio.toFixed(2)),
	      kind_readable: kindRatio >= 4.5,
	      contrast_ratio: Number(ratio.toFixed(2)),
	      readable: ratio >= 4.5,
	      detail_contrast_ratio: Number(detailRatio.toFixed(2)),
	      detail_readable: detailRatio >= 4.5,
	      ...richRect(node),
	    };
	  });
	  const commandPaletteTriggerDetails = Array.from(document.querySelectorAll("[data-control-ui-command-palette-trigger]")).map((node) => {
	    const style = getComputedStyle(node);
	    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	    const filterText = style.filter || "";
	    const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	    const textColor = parseCssColor(style.color);
	    const bgColor = effectiveBackground(node);
	    const bgLuminance = relativeLuminance(bgColor);
	    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	    const ariaLabel = node.getAttribute("aria-label") || "";
	    const title = node.getAttribute("title") || "";
	    const visibleTextValue = visibleText(node);
	    return {
	      marker: node.getAttribute("data-control-ui-command-palette-trigger") || "",
	      href: node.getAttribute("href") || "",
	      aria_label: ariaLabel,
	      title,
	      title_matches_aria_label: title === ariaLabel,
	      svg_icon_present: hasSvgIcon(node),
	      visible_icon_text: visibleTextValue,
	      visible_icon_text_absent: visibleTextValue.length === 0,
	      visible: elementVisible(node),
	      background_color: style.backgroundColor,
	      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
	      translucent_ready: translucentGlassReady(style),
	      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
	      effective_luminance: Number(bgLuminance.toFixed(3)),
	      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
	      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	      backdrop_filter: backdrop,
	      box_shadow: compactShadow(style.boxShadow),
	      filter: filterText && filterText !== "none" ? "present" : "none",
	      filter_sample: filterText.slice(0, 180),
	      icon_prismatic_control_drop_shadow_count: dropShadowCount,
	      icon_prismatic_control_ready: dropShadowCount >= 2,
	      contrast_ratio: Number(ratio.toFixed(2)),
	      readable: ratio >= 4.5,
	      ...richRect(node),
	    };
	  });
	  const visibleCommandPaletteTriggerDetails = commandPaletteTriggerDetails.filter((item) => item.visible);
	  const commandPaletteTriggerLightGlassReady = visibleCommandPaletteTriggerDetails.length === 1
	    && visibleCommandPaletteTriggerDetails.every((item) => (
	      item.marker === "light-glass"
	      && item.href === "#command-palette"
	      && item.width >= 44
	      && item.height >= 44
	      && item.border_radius >= 20
	      && item.light_glass_ready === true
	      && item.translucent_ready === true
	      && item.effective_luminance >= 0.72
	      && item.effective_luminance <= 0.98
	      && (item.backdrop_filter || "").includes("blur(")
	      && item.box_shadow !== "none"
	      && item.aria_label.length > 0
	      && item.title.length > 0
	      && item.title_matches_aria_label === true
	      && item.svg_icon_present === true
	      && item.visible_icon_text_absent === true
	      && item.readable === true
	      && item.contrast_ratio >= 4.5
	    ));
	  const iconPrismaticControlDetails = [
	    ...iconButtonDetails.map((item) => ({ group: "icon-button", ...item })),
	    ...menuTriggerDetails.map((item) => ({ group: "menu-trigger", ...item })),
	    ...visibleCommandPaletteTriggerDetails.map((item) => ({ group: "command-palette-trigger", ...item })),
	  ];
	  const iconPrismaticControlLightGlassReady = iconButtonReady
	    && menuTriggerReady
	    && commandPaletteTriggerLightGlassReady
	    && iconPrismaticControlDetails.length >= (railVisible ? 8 : 7)
	    && iconPrismaticControlDetails.every((item) => (
	      item.visible === true
	      && item.width >= 44
	      && item.height >= 44
	      && item.border_radius >= 20
	      && (item.backdrop_filter || "").includes("blur(")
	      && item.box_shadow !== "none"
	      && item.icon_prismatic_control_ready === true
	      && item.icon_prismatic_control_drop_shadow_count >= 2
	    ));
	  const commandPaletteReady = commandPalettePanelDetails.exists === true
	    && commandPalettePanelDetails.visible === true
	    && commandPalettePanelDetails.marker === "light-glass"
	    && commandPalettePanelDetails.role === "dialog"
	    && commandPalettePanelDetails.aria_modal === "true"
	    && commandPalettePanelDetails.aria_label === "Command palette"
	    && commandPalettePanelDetails.light_glass_ready === true
	    && commandPalettePanelDetails.translucent_ready === true
	    && commandPalettePanelDetails.effective_luminance >= 0.72
	    && commandPalettePanelDetails.effective_luminance <= 0.98
	    && commandPalettePanelDetails.border_radius >= 18
	    && /blur\\(/.test(commandPalettePanelDetails.backdrop_filter || "")
	    && commandPalettePanelDetails.box_shadow !== "none"
	    && commandPalettePanelDetails.in_viewport === true
	    && commandPaletteCloseDetails.exists === true
	    && commandPaletteCloseDetails.visible === true
	    && commandPaletteCloseDetails.width >= 44
	    && commandPaletteCloseDetails.height >= 44
	    && commandPaletteCloseDetails.title_matches_aria_label === true
	    && commandPaletteCloseDetails.svg_icon_present === true
	    && commandPaletteCloseDetails.visible_icon_text_absent === true
	    && /blur\\(/.test(commandPaletteCloseDetails.backdrop_filter || "")
	    && commandPaletteCloseDetails.box_shadow !== "none"
	    && commandPaletteInputDetails.exists === true
	    && commandPaletteInputDetails.visible === true
	    && commandPaletteInputDetails.height >= 44
	    && commandPaletteInputDetails.title_matches_aria_label === true
	    && commandPaletteInputDetails.readable === true
	    && commandPaletteItemDetails.length >= 1
	    && commandPaletteItemDetails.every((item) => (
	      item.visible
	      && item.width >= 180
	      && item.height >= 44
	      && item.text.length > 0
	      && item.aria_label.length > 0
	      && item.title.length > 0
	      && item.title_matches_aria_label
	      && item.readable
	      && item.contrast_ratio >= 4.5
	    ));
	  const commandPaletteSurfaceLightGlassReady = commandPalettePanelDetails.exists === true
	    && commandPalettePanelDetails.visible === true
	    && commandPalettePanelDetails.marker === "light-glass"
	    && commandPalettePanelDetails.role === "dialog"
	    && commandPalettePanelDetails.aria_modal === "true"
	    && commandPalettePanelDetails.aria_label === "Command palette"
	    && commandPalettePanelDetails.width >= 274
	    && commandPalettePanelDetails.height >= 132
	    && commandPalettePanelDetails.border_radius >= 18
	    && commandPalettePanelDetails.light_glass_ready === true
	    && commandPalettePanelDetails.translucent_ready === true
	    && commandPalettePanelDetails.effective_luminance >= 0.72
	    && commandPalettePanelDetails.effective_luminance <= 0.98
	    && /blur\\(/.test(commandPalettePanelDetails.backdrop_filter || "")
	    && commandPalettePanelDetails.box_shadow !== "none"
	    && commandPalettePanelDetails.in_viewport === true;
	  const commandPaletteSurfacePrismaticPerimeterLightGlassReady = commandPaletteSurfaceLightGlassReady
	    && commandPalettePanelDetails.filter === "present"
	    && commandPalettePanelDetails.command_palette_surface_prismatic_perimeter_ready === true
	    && commandPalettePanelDetails.command_palette_surface_drop_shadow_count >= 2;
	  const commandPaletteBackdropCausticVeilLightGlassReady = commandPaletteBackdropDetails.exists === true
	    && commandPaletteBackdropDetails.visible === true
	    && commandPaletteBackdropDetails.background_alpha >= 0.2
	    && commandPaletteBackdropDetails.background_alpha <= 0.6
	    && commandPaletteBackdropDetails.background_image === "present"
	    && commandPaletteBackdropDetails.command_palette_backdrop_caustic_veil_ready === true
	    && commandPaletteBackdropDetails.command_palette_backdrop_repeating_layer_count >= 1
	    && commandPaletteBackdropDetails.backdrop_blur_px >= 10
	    && commandPaletteBackdropDetails.covers_viewport === true;
	  const commandPaletteInputRowPrismaticSeparatorLightGlassReady = commandPaletteInputRowDetails.exists === true
	    && commandPaletteInputRowDetails.visible === true
	    && commandPaletteInputRowDetails.width >= 274
	    && commandPaletteInputRowDetails.height >= 60
	    && commandPaletteInputRowDetails.border_bottom_alpha >= 0.25
	    && commandPaletteInputRowDetails.box_shadow !== "none"
	    && commandPaletteInputRowDetails.command_palette_input_row_separator_shadow_count >= 2
	    && commandPaletteInputRowDetails.command_palette_input_row_prismatic_separator_ready === true;
	  const commandPaletteResultsWellLightGlassReady = commandPaletteResultsWellDetails.exists === true
	    && commandPaletteResultsWellDetails.visible === true
	    && commandPaletteResultsWellDetails.width >= 274
	    && commandPaletteResultsWellDetails.height >= 58
	    && commandPaletteResultsWellDetails.background_alpha >= 0.1
	    && commandPaletteResultsWellDetails.background_alpha <= 0.4
	    && commandPaletteResultsWellDetails.light_glass_ready === true
	    && String(commandPaletteResultsWellDetails.backdrop_filter || "").includes("blur(")
	    && commandPaletteResultsWellDetails.backdrop_blur_px >= 10
	    && commandPaletteResultsWellDetails.command_palette_results_well_light_glass_ready === true;
	  const commandPaletteResultsWellPrismaticRimLightGlassReady = commandPaletteResultsWellLightGlassReady
	    && commandPaletteResultsWellDetails.border_alpha >= 0.25
	    && commandPaletteResultsWellDetails.border_radius >= 12
	    && commandPaletteResultsWellDetails.box_shadow !== "none"
	    && commandPaletteResultsWellDetails.command_palette_results_well_rim_shadow_count >= 2
	    && commandPaletteResultsWellDetails.command_palette_results_well_prismatic_rim_ready === true;
	  const commandPaletteCloseLightGlassReady = commandPaletteCloseDetails.exists === true
	    && commandPaletteCloseDetails.visible === true
	    && commandPaletteCloseDetails.marker === "light-glass"
	    && commandPaletteCloseDetails.href === "#commands"
	    && commandPaletteCloseDetails.width >= 44
	    && commandPaletteCloseDetails.height >= 44
	    && commandPaletteCloseDetails.border_radius >= 20
	    && commandPaletteCloseDetails.light_glass_ready === true
	    && commandPaletteCloseDetails.translucent_ready === true
	    && commandPaletteCloseDetails.effective_luminance >= 0.72
	    && commandPaletteCloseDetails.effective_luminance <= 0.98
	    && /blur\\(/.test(commandPaletteCloseDetails.backdrop_filter || "")
	    && commandPaletteCloseDetails.box_shadow !== "none"
	    && commandPaletteCloseDetails.aria_label === "Close command palette"
	    && commandPaletteCloseDetails.title_matches_aria_label === true
	    && commandPaletteCloseDetails.svg_icon_present === true
	    && commandPaletteCloseDetails.visible_icon_text_absent === true
	    && commandPaletteCloseDetails.readable === true
	    && commandPaletteCloseDetails.contrast_ratio >= 4.5;
	  const commandPaletteClosePrismaticIconLightGlassReady = commandPaletteCloseLightGlassReady
	    && commandPaletteCloseDetails.filter === "present"
	    && commandPaletteCloseDetails.command_palette_close_prismatic_icon_ready === true
	    && commandPaletteCloseDetails.command_palette_close_drop_shadow_count >= 2;
	  const commandPaletteItemLightGlassReady = commandPaletteItemDetails.length >= 1
	    && commandPaletteItemDetails.every((item) => (
	      item.visible
	      && item.marker === "light-glass"
	      && item.key.length > 0
	      && item.kind.length > 0
	      && item.label.length > 0
	      && item.detail.length > 0
	      && item.width >= 180
	      && item.height >= 44
	      && item.border_radius >= 8
	      && item.light_glass_ready === true
	      && item.translucent_ready === true
	      && item.effective_luminance >= 0.72
	      && item.effective_luminance <= 0.98
	      && /blur\\(/.test(item.backdrop_filter || "")
	      && item.box_shadow !== "none"
	      && item.aria_label.length > 0
	      && item.title.length > 0
	      && item.title_matches_aria_label === true
	      && item.readable === true
	      && item.contrast_ratio >= 4.5
	    ));
	  const commandPaletteItemPrismaticRimLightGlassReady = commandPaletteItemLightGlassReady
	    && commandPaletteItemDetails.every((item) => (
	      item.border_alpha >= 0.25
	      && item.box_shadow !== "none"
	      && item.command_palette_item_rim_shadow_count >= 2
	      && item.command_palette_item_prismatic_rim_ready === true
	    ));
	  const commandPaletteKindChipLightGlassReady = commandPaletteItemLightGlassReady
	    && commandPaletteItemDetails.every((item) => (
	      item.kind_width >= 44
	      && item.kind_height >= 22
	      && item.kind_background_alpha >= 0.25
	      && item.kind_background_alpha <= 0.75
	      && item.kind_effective_luminance >= 0.72
	      && item.kind_effective_luminance <= 0.98
	      && item.kind_border_alpha >= 0.25
	      && item.kind_border_radius >= 20
	      && (item.kind_backdrop_filter || "").includes("blur(")
	      && item.kind_box_shadow !== "none"
	      && item.command_palette_kind_chip_shadow_count >= 2
	      && item.command_palette_kind_chip_light_glass_ready === true
	      && item.kind_readable === true
	      && item.kind_contrast_ratio >= 4.5
	    ));
	  const commandPaletteItemHoverDetails = commandPaletteItemDetails.filter((item) => item.audit_hover === true);
	  const commandPaletteItemHoverPrismaticLightGlassReady = commandPaletteItemLightGlassReady
	    && commandPaletteItemHoverDetails.length >= 1
	    && commandPaletteItemHoverDetails.every((item) => (
	      item.command_palette_item_hover_prismatic_ready === true
	      && item.command_palette_item_hover_shadow_count >= 2
	      && item.border_alpha >= 0.25
	      && item.box_shadow !== "none"
	    ));
	  const commandPaletteItemLabelPrismaticEtchLightGlassReady = commandPaletteItemLightGlassReady
	    && commandPaletteItemDetails.every((item) => (
	      item.kind_text_shadow === "present"
	      && item.label_text_shadow === "present"
	      && item.detail_text_shadow === "present"
	      && item.command_palette_item_label_prismatic_etch_ready === true
	      && item.command_palette_item_kind_text_shadow_count >= 2
	      && item.command_palette_item_label_text_shadow_count >= 2
	      && item.command_palette_item_detail_text_shadow_count >= 2
	      && item.kind_readable
	      && item.readable
	      && item.detail_readable
	      && item.kind_contrast_ratio >= 4.5
	      && item.contrast_ratio >= 4.5
	      && item.detail_contrast_ratio >= 4.5
	    ));
	  const commandPaletteInputLightGlassReady = commandPaletteInputDetails.exists === true
	    && commandPaletteInputDetails.visible === true
	    && commandPaletteInputDetails.marker === "light-glass"
	    && commandPaletteInputDetails.type === "search"
	    && commandPaletteInputDetails.placeholder.length > 0
	    && commandPaletteInputDetails.height >= 44
	    && commandPaletteInputDetails.border_radius >= 10
	    && commandPaletteInputDetails.light_glass_ready === true
	    && commandPaletteInputDetails.translucent_ready === true
	    && commandPaletteInputDetails.effective_luminance >= 0.72
	    && commandPaletteInputDetails.effective_luminance <= 0.98
	    && /blur\\(/.test(commandPaletteInputDetails.backdrop_filter || "")
	    && commandPaletteInputDetails.box_shadow !== "none"
	    && commandPaletteInputDetails.aria_label.length > 0
	    && commandPaletteInputDetails.title.length > 0
	    && commandPaletteInputDetails.title_matches_aria_label === true
	    && commandPaletteInputDetails.readable === true
	    && commandPaletteInputDetails.contrast_ratio >= 4.5;
	  const commandPaletteInputTextPrismaticEtchLightGlassReady = commandPaletteInputLightGlassReady
	    && commandPaletteInputDetails.text_shadow === "present"
	    && commandPaletteInputDetails.command_palette_input_prismatic_etch_ready === true
	    && commandPaletteInputDetails.command_palette_input_text_shadow_count >= 2
	    && commandPaletteInputDetails.readable === true
	    && commandPaletteInputDetails.contrast_ratio >= 4.5;
	  const commandPaletteInputPlaceholderPrismaticEtchLightGlassReady = commandPaletteInputLightGlassReady
	    && commandPaletteInputDetails.placeholder.length > 0
	    && commandPaletteInputDetails.placeholder_text_shadow === "present"
	    && commandPaletteInputDetails.command_palette_input_placeholder_prismatic_etch_ready === true
	    && commandPaletteInputDetails.command_palette_input_placeholder_text_shadow_count >= 2
	    && commandPaletteInputDetails.command_palette_input_placeholder_font_weight >= 600
	    && commandPaletteInputDetails.placeholder_readable === true
	    && commandPaletteInputDetails.placeholder_contrast_ratio >= 4.5;
	  const commandPaletteInputIconLightGlassReady = commandPaletteInputIconDetails.exists === true
	    && commandPaletteInputIconDetails.visible === true
	    && commandPaletteInputIconDetails.width >= 44
	    && commandPaletteInputIconDetails.height >= 44
	    && commandPaletteInputIconDetails.border_radius >= 20
	    && commandPaletteInputIconDetails.light_glass_ready === true
	    && commandPaletteInputIconDetails.translucent_ready === true
	    && commandPaletteInputIconDetails.effective_luminance >= 0.72
	    && commandPaletteInputIconDetails.effective_luminance <= 0.98
	    && /blur\\(/.test(commandPaletteInputIconDetails.backdrop_filter || "")
	    && commandPaletteInputIconDetails.box_shadow !== "none"
	    && commandPaletteInputIconDetails.svg_icon_present === true
	    && commandPaletteInputIconDetails.visible_icon_text_absent === true
	    && commandPaletteInputIconDetails.readable === true
	    && commandPaletteInputIconDetails.contrast_ratio >= 4.5;
	  const commandPaletteInputIconPrismaticLightGlassReady = commandPaletteInputIconLightGlassReady
	    && commandPaletteInputIconDetails.filter === "present"
	    && commandPaletteInputIconDetails.command_palette_input_icon_prismatic_ready === true
	    && commandPaletteInputIconDetails.command_palette_input_icon_drop_shadow_count >= 2;
	  const controlFormControlDetails = Array.from(document.querySelectorAll("[data-chat-search],[data-chat-composer-input],[data-chat-routing-mode],[data-chat-autoscroll-mode]"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const textColor = parseCssColor(style.color);
	      const formBackgroundHost = node.matches("[data-control-ui-rail-search-input]") ? node : (node.closest(".tg-search-shell,.tg-compose-bar,.tg-menu-item,.command-palette") || node);
	      const bgColor = effectiveBackground(formBackgroundHost);
	      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	      const ariaLabel = node.getAttribute("aria-label") || "";
	      const title = node.getAttribute("title") || "";
	      return {
	        role: node.getAttribute("data-chat-search") !== null ? "chat-search"
	          : node.getAttribute("data-chat-composer-input") !== null ? "chat-composer-input"
	          : node.getAttribute("data-chat-routing-mode") !== null ? "chat-routing-mode"
	          : node.getAttribute("data-chat-autoscroll-mode") !== null ? "chat-autoscroll-mode"
	          : node.tagName.toLowerCase(),
	        tag: node.tagName.toLowerCase(),
	        aria_label: ariaLabel,
	        title,
	        title_matches_aria_label: title === ariaLabel,
	        placeholder: node.getAttribute("placeholder") || "",
	        color: style.color,
	        effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
	        contrast_ratio: Number(ratio.toFixed(2)),
	        readable: ratio >= 4.5,
	        ...richRect(node),
	      };
	    });
	  const expectedVisibleFormControlCount = railVisible ? 4 : 1;
		  const controlFormControlReady = controlFormControlDetails.length >= expectedVisibleFormControlCount
		    && controlFormControlDetails.every((item) => (
		      item.aria_label.length > 0
		      && item.title.length > 0
		      && item.title_matches_aria_label
		      && item.height >= 44
		      && item.readable
		      && item.contrast_ratio >= 4.5
		    ));
		  const chatRowOptionDetails = Array.from(document.querySelectorAll("[data-chat-conversation]"))
		    .filter(elementVisible)
		    .map((node) => {
		      const style = getComputedStyle(node);
		      const ariaLabel = node.getAttribute("aria-label") || "";
		      const title = node.getAttribute("title") || "";
		      const active = node.classList.contains("active");
		      const ariaSelected = node.getAttribute("aria-selected") || "";
		      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
		      const filterText = style.filter || "";
		      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
		      return {
		        key: node.getAttribute("data-chat-conversation") || "",
		        role: node.getAttribute("role") || "",
		        aria_selected: ariaSelected,
		        aria_label: ariaLabel,
		        title,
		        title_matches_aria_label: title === ariaLabel,
		        tabindex: node.getAttribute("tabindex") || "",
		        active,
		        visible: elementVisible(node),
		        active_state_matches_aria_selected: (active ? "true" : "false") === ariaSelected,
		        border_radius: styleNumber(style, "borderTopLeftRadius"),
		        box_shadow: compactShadow(style.boxShadow),
		        backdrop_filter: backdrop,
		        filter: filterText && filterText !== "none" ? "present" : "none",
		        filter_sample: filterText.slice(0, 180),
		        chat_row_drop_shadow_count: dropShadowCount,
		        chat_row_prismatic_slab_ready: dropShadowCount >= 2,
		        ...richRect(node),
		      };
		    });
		  const expectedVisibleChatRowOptionCount = railVisible ? 3 : 0;
		  const chatRowOptionSemanticTouchReady = chatRowOptionDetails.length === expectedVisibleChatRowOptionCount
		    && chatRowOptionDetails.every((item) => (
		      item.key.length > 0
		      && item.role === "option"
		      && item.width >= 44
		      && item.height >= 64
		      && item.aria_label.length > 0
		      && item.title.length > 0
		      && item.title_matches_aria_label
		      && item.tabindex === "0"
		      && item.active_state_matches_aria_selected
		      && item.border_radius >= 18
		    ));
		  const railChatRowPrismaticSlabLightGlassReady = railVisible
		    ? (
		      chatRowOptionDetails.length >= 3
		      && chatRowOptionDetails.every((item) => (
		        item.visible !== false
		        && item.width >= 44
		        && item.height >= 64
		        && item.border_radius >= 18
		        && item.box_shadow !== "none"
		        && (item.backdrop_filter || "").includes("blur(")
		        && item.chat_row_prismatic_slab_ready === true
		        && item.chat_row_drop_shadow_count >= 2
		      ))
		    )
		    : chatRowOptionDetails.length === 0;
		  const menuItems = Array.from(document.querySelectorAll("[data-control-ui-menu-item]"));
  const menuItemDetails = menuItems.map((node) => {
    const labelNode = node.querySelector(".tg-menu-item__label");
    const iconNode = node.querySelector(".tg-menu-item__icon");
    const style = getComputedStyle(node);
    const labelStyle = labelNode ? getComputedStyle(labelNode) : null;
    const label = (labelNode?.textContent || "").replace(/\\s+/g, " ").trim();
    return {
      key: node.getAttribute("data-control-ui-menu-item") || "",
      icon_present: Boolean(iconNode),
      icon_svg_present: hasSvgIcon(iconNode),
      label,
      label_ready: Boolean(labelNode && label.length > 0),
      visible: elementVisible(node),
      min_height: styleNumber(style, "minHeight"),
      height: Math.round(node.getBoundingClientRect().height),
      label_white_space: labelStyle?.whiteSpace || "",
      label_overflow: labelStyle?.overflow || "",
      label_text_overflow: labelStyle?.textOverflow || "",
      label_nowrap_ready: labelStyle?.whiteSpace === "nowrap",
    };
  });
  const menuItemIconReady = menuItemDetails.length >= 5
    && menuItemDetails.every((item) => (
      item.key.length > 0
      && item.icon_present
      && item.icon_svg_present
      && item.label_ready
      && item.visible
      && item.height >= 36
      && item.label_nowrap_ready
    ));
  const menuSurfaces = Array.from(document.querySelectorAll(".tg-thread-command-menu__panel"));
  const menuSurfaceDetails = menuSurfaces.map((node) => {
    const style = getComputedStyle(node);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const rect = richRect(node);
    const horizontalInViewport = rect.left >= -1 && rect.right <= window.innerWidth + 1 && rect.width <= window.innerWidth - 16;
    const verticalInViewport = rect.top >= -1 && rect.bottom <= window.innerHeight + 1 && rect.height <= window.innerHeight - 16;
    return {
      visible: elementVisible(node),
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      background_image: style.backgroundImage && style.backgroundImage !== "none" ? "present" : "none",
      background_color: style.backgroundColor,
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      overflow_x: style.overflowX,
      item_count: node.querySelectorAll("[data-control-ui-menu-item]").length,
      horizontal_in_viewport: horizontalInViewport,
      vertical_in_viewport: verticalInViewport,
      in_viewport: horizontalInViewport && verticalInViewport,
      top_clipped: rect.top < -1,
      bottom_clipped: rect.bottom > window.innerHeight + 1,
      viewport_height: window.innerHeight,
      viewport_width: window.innerWidth,
      ...rect,
    };
  });
  const menuSurfaceReady = menuSurfaceDetails.length >= 2
    && menuSurfaceDetails.every((item) => (
      item.visible
      && item.item_count >= 1
      && item.width >= 180
      && item.height >= 44
      && item.border_radius >= 16
      && /blur\\(/.test(item.backdrop_filter)
      && item.box_shadow !== "none"
      && item.in_viewport
    ));
  const threadToolsTrigger = document.querySelector('[data-thread-command-menu="true"] [data-control-ui-thread-tools-trigger="light-glass"]');
  const threadToolsTriggerDetails = (() => {
    if (!threadToolsTrigger) {
      return { exists: false, visible: false };
    }
    const style = getComputedStyle(threadToolsTrigger);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const bgColor = effectiveBackground(threadToolsTrigger);
    const fgColor = parseCssColor(style.color);
    const bgLuminance = relativeLuminance(bgColor);
    const ratio = fgColor ? contrastRatio(fgColor, bgColor) : 0;
    const ariaLabel = threadToolsTrigger.getAttribute("aria-label") || "";
    const title = threadToolsTrigger.getAttribute("title") || "";
    return {
      exists: true,
      marker: threadToolsTrigger.getAttribute("data-control-ui-thread-tools-trigger") || "",
      visible: elementVisible(threadToolsTrigger),
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      svg_icon_present: hasSvgIcon(threadToolsTrigger),
      visible_icon_text: visibleText(threadToolsTrigger),
      visible_icon_text_absent: visibleText(threadToolsTrigger).length === 0,
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      effective_luminance: Number(bgLuminance.toFixed(3)),
      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      ...richRect(threadToolsTrigger),
    };
  })();
  const threadToolsTriggerLightGlassReady = threadToolsTriggerDetails.exists === true
    && threadToolsTriggerDetails.marker === "light-glass"
    && threadToolsTriggerDetails.visible === true
    && threadToolsTriggerDetails.width >= 44
    && threadToolsTriggerDetails.height >= 44
    && threadToolsTriggerDetails.border_radius >= 20
    && threadToolsTriggerDetails.light_glass_ready === true
    && /blur\\(/.test(threadToolsTriggerDetails.backdrop_filter || "")
    && threadToolsTriggerDetails.box_shadow !== "none"
    && threadToolsTriggerDetails.title_matches_aria_label === true
    && threadToolsTriggerDetails.svg_icon_present === true
    && threadToolsTriggerDetails.visible_icon_text_absent === true
    && threadToolsTriggerDetails.readable === true
    && threadToolsTriggerDetails.contrast_ratio >= 4.5;
  const threadToolsPanel = document.querySelector('[data-thread-command-menu="true"] [data-control-ui-thread-tools-panel="light-glass"]');
  const threadToolsPanelDetails = (() => {
    if (!threadToolsPanel) {
      return { exists: false, visible: false };
    }
    const style = getComputedStyle(threadToolsPanel);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const rect = richRect(threadToolsPanel);
    const bgColor = effectiveBackground(threadToolsPanel);
    const bgLuminance = relativeLuminance(bgColor);
    const horizontalInViewport = rect.left >= -1 && rect.right <= window.innerWidth + 1 && rect.width <= window.innerWidth - 16;
    const verticalInViewport = rect.top >= -1 && rect.bottom <= window.innerHeight + 1 && rect.height <= window.innerHeight - 16;
    return {
      exists: true,
      visible: elementVisible(threadToolsPanel),
      role: threadToolsPanel.getAttribute("role") || "",
      aria_label: threadToolsPanel.getAttribute("aria-label") || "",
      background_color: style.backgroundColor,
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      effective_luminance: Number(bgLuminance.toFixed(3)),
      marker: threadToolsPanel.getAttribute("data-control-ui-thread-tools-panel") || "",
      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      item_count: threadToolsPanel.querySelectorAll("[data-control-ui-menu-item]").length,
      horizontal_in_viewport: horizontalInViewport,
      vertical_in_viewport: verticalInViewport,
      in_viewport: horizontalInViewport && verticalInViewport,
      top_clipped: rect.top < -1,
      bottom_clipped: rect.bottom > window.innerHeight + 1,
      viewport_height: window.innerHeight,
      viewport_width: window.innerWidth,
      ...rect,
    };
  })();
  const threadToolsItemDetails = Array.from(document.querySelectorAll('[data-thread-command-menu="true"] [data-control-ui-menu-item]')).map((node) => {
    const labelNode = node.querySelector(".tg-menu-item__label");
    const iconNode = node.querySelector(".tg-menu-item__icon");
    const style = getComputedStyle(node);
    const labelStyle = labelNode ? getComputedStyle(labelNode) : null;
    const labelTextStyle = labelNode ? getComputedStyle(labelNode) : style;
    const textColor = parseCssColor(labelTextStyle.color);
    const bgColor = effectiveBackground(node);
    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
    const label = (labelNode?.textContent || "").replace(/\\s+/g, " ").trim();
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    return {
      key: node.getAttribute("data-control-ui-menu-item") || "",
      role: node.getAttribute("role") || "",
      text: visibleText(node),
      label,
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      icon_present: Boolean(iconNode),
      icon_svg_present: hasSvgIcon(iconNode),
      label_ready: Boolean(labelNode && label.length > 0),
      visible: elementVisible(node),
      min_height: styleNumber(style, "minHeight"),
      height: Math.round(node.getBoundingClientRect().height),
      label_white_space: labelStyle?.whiteSpace || "",
      label_overflow: labelStyle?.overflow || "",
      label_text_overflow: labelStyle?.textOverflow || "",
      label_nowrap_ready: labelStyle?.whiteSpace === "nowrap",
      color: labelTextStyle.color,
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      ...richRect(node),
    };
  });
  const expectedThreadToolsKeys = ["history", "tasks", "sessions"];
  const threadToolsMenuReady = threadToolsPanelDetails.exists === true
    && threadToolsPanelDetails.visible === true
    && threadToolsPanelDetails.role === "menu"
    && threadToolsPanelDetails.aria_label === "Thread tools"
    && threadToolsPanelDetails.item_count === 3
    && threadToolsPanelDetails.width >= 180
    && threadToolsPanelDetails.height >= 44
    && threadToolsPanelDetails.border_radius >= 16
    && threadToolsTriggerLightGlassReady === true
    && threadToolsPanelDetails.marker === "light-glass"
    && threadToolsPanelDetails.light_glass_ready === true
    && threadToolsPanelDetails.effective_luminance >= 0.72
    && threadToolsPanelDetails.effective_luminance <= 0.98
    && /blur\\(/.test(threadToolsPanelDetails.backdrop_filter || "")
    && threadToolsPanelDetails.box_shadow !== "none"
    && threadToolsPanelDetails.in_viewport === true
    && threadToolsPanelDetails.top_clipped === false
    && threadToolsPanelDetails.bottom_clipped === false
    && threadToolsItemDetails.length === 3
    && expectedThreadToolsKeys.every((key) => threadToolsItemDetails.some((item) => item.key === key))
    && threadToolsItemDetails.every((item) => (
      item.visible
      && item.role === "menuitem"
      && item.key.length > 0
      && item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.icon_present
      && item.icon_svg_present
      && item.label_ready
      && item.height >= 44
      && item.label_nowrap_ready
      && item.readable
      && item.contrast_ratio >= 4.5
    ));
  const composerToolsTrigger = document.querySelector('[data-control-ui-composer-more] [data-control-ui-composer-tools-trigger="light-glass"]');
  const composerToolsTriggerDetails = (() => {
    if (!composerToolsTrigger) {
      return { exists: false, visible: false };
    }
    const style = getComputedStyle(composerToolsTrigger);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const bgColor = effectiveBackground(composerToolsTrigger);
    const fgColor = parseCssColor(style.color);
    const bgLuminance = relativeLuminance(bgColor);
    const ratio = fgColor ? contrastRatio(fgColor, bgColor) : 0;
    const ariaLabel = composerToolsTrigger.getAttribute("aria-label") || "";
    const title = composerToolsTrigger.getAttribute("title") || "";
    return {
      exists: true,
      marker: composerToolsTrigger.getAttribute("data-control-ui-composer-tools-trigger") || "",
      visible: elementVisible(composerToolsTrigger),
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      svg_icon_present: hasSvgIcon(composerToolsTrigger),
      visible_icon_text: visibleText(composerToolsTrigger),
      visible_icon_text_absent: visibleText(composerToolsTrigger).length === 0,
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      effective_luminance: Number(bgLuminance.toFixed(3)),
      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      ...richRect(composerToolsTrigger),
    };
  })();
  const composerToolsTriggerLightGlassReady = composerToolsTriggerDetails.exists === true
    && composerToolsTriggerDetails.marker === "light-glass"
    && composerToolsTriggerDetails.visible === true
    && composerToolsTriggerDetails.width >= 44
    && composerToolsTriggerDetails.height >= 44
    && composerToolsTriggerDetails.border_radius >= 20
    && composerToolsTriggerDetails.light_glass_ready === true
    && /blur\\(/.test(composerToolsTriggerDetails.backdrop_filter || "")
    && composerToolsTriggerDetails.box_shadow !== "none"
    && composerToolsTriggerDetails.title_matches_aria_label === true
    && composerToolsTriggerDetails.svg_icon_present === true
    && composerToolsTriggerDetails.visible_icon_text_absent === true
    && composerToolsTriggerDetails.readable === true
    && composerToolsTriggerDetails.contrast_ratio >= 4.5;
  const composerToolsPanel = document.querySelector('[data-control-ui-composer-more] [data-control-ui-composer-tools-panel="light-glass"]');
  const composerToolsPanelDetails = (() => {
    if (!composerToolsPanel) {
      return { exists: false, visible: false };
    }
    const style = getComputedStyle(composerToolsPanel);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const rect = richRect(composerToolsPanel);
    const bgColor = effectiveBackground(composerToolsPanel);
    const bgLuminance = relativeLuminance(bgColor);
    const horizontalInViewport = rect.left >= -1 && rect.right <= window.innerWidth + 1 && rect.width <= window.innerWidth - 16;
    const verticalInViewport = rect.top >= -1 && rect.bottom <= window.innerHeight + 1 && rect.height <= window.innerHeight - 16;
    return {
      exists: true,
      visible: elementVisible(composerToolsPanel),
      role: composerToolsPanel.getAttribute("role") || "",
      aria_label: composerToolsPanel.getAttribute("aria-label") || "",
      background_color: style.backgroundColor,
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      effective_luminance: Number(bgLuminance.toFixed(3)),
      marker: composerToolsPanel.getAttribute("data-control-ui-composer-tools-panel") || "",
      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      item_count: composerToolsPanel.querySelectorAll("[data-control-ui-composer-tool-item]").length,
      horizontal_in_viewport: horizontalInViewport,
      vertical_in_viewport: verticalInViewport,
      in_viewport: horizontalInViewport && verticalInViewport,
      top_clipped: rect.top < -1,
      bottom_clipped: rect.bottom > window.innerHeight + 1,
      viewport_height: window.innerHeight,
      viewport_width: window.innerWidth,
      ...rect,
    };
  })();
  const composerToolsItemDetails = Array.from(document.querySelectorAll('[data-control-ui-composer-more] [data-control-ui-composer-tool-item]')).map((node) => {
    const labelNode = node.querySelector(".tg-menu-item__label");
    const iconNode = node.querySelector(".tg-menu-item__icon");
    const selectNode = node.querySelector("select");
    const style = getComputedStyle(node);
    const labelStyle = labelNode ? getComputedStyle(labelNode) : null;
    const labelTextStyle = labelNode ? getComputedStyle(labelNode) : style;
    const textColor = parseCssColor(labelTextStyle.color);
    const bgColor = effectiveBackground(node);
    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
    const label = (labelNode?.textContent || "").replace(/\\s+/g, " ").trim();
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const selectAriaLabel = selectNode?.getAttribute("aria-label") || "";
    const selectTitle = selectNode?.getAttribute("title") || "";
    const selectStyle = selectNode ? getComputedStyle(selectNode) : null;
    const selectTextColor = selectStyle ? parseCssColor(selectStyle.color) : null;
    const selectBgColor = selectNode ? effectiveBackground(selectNode) : null;
    const selectRatio = selectTextColor && selectBgColor ? contrastRatio(selectTextColor, selectBgColor) : 0;
    return {
      key: node.getAttribute("data-control-ui-composer-tool-item") || "",
      role: node.getAttribute("role") || "",
      text: visibleText(node),
      label,
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      icon_present: Boolean(iconNode),
      icon_svg_present: hasSvgIcon(iconNode),
      label_ready: Boolean(labelNode && label.length > 0),
      select_present: Boolean(selectNode),
      select_visible: Boolean(selectNode && elementVisible(selectNode)),
      select_aria_label: selectAriaLabel,
      select_title: selectTitle,
      select_title_matches_aria_label: selectTitle === selectAriaLabel,
      select_height: selectNode ? Math.round(selectNode.getBoundingClientRect().height) : 0,
      select_color: selectStyle?.color || "",
      select_effective_background: selectBgColor ? "rgb(" + Math.round(selectBgColor.r) + ", " + Math.round(selectBgColor.g) + ", " + Math.round(selectBgColor.b) + ")" : "",
      select_contrast_ratio: Number(selectRatio.toFixed(2)),
      select_readable: selectRatio >= 4.5,
      visible: elementVisible(node),
      min_height: styleNumber(style, "minHeight"),
      height: Math.round(node.getBoundingClientRect().height),
      label_white_space: labelStyle?.whiteSpace || "",
      label_overflow: labelStyle?.overflow || "",
      label_text_overflow: labelStyle?.textOverflow || "",
      label_nowrap_ready: labelStyle?.whiteSpace === "nowrap",
      color: labelTextStyle.color,
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      ...richRect(node),
    };
  });
  const expectedComposerToolsKeys = ["reply-mode", "scroll-mode"];
  const composerToolsMenuReady = composerToolsPanelDetails.exists === true
    && composerToolsPanelDetails.visible === true
    && composerToolsPanelDetails.role === "menu"
    && composerToolsPanelDetails.aria_label === "Composer tools"
    && composerToolsPanelDetails.item_count === 2
    && composerToolsPanelDetails.width >= 180
    && composerToolsPanelDetails.height >= 44
    && composerToolsPanelDetails.border_radius >= 16
    && composerToolsTriggerLightGlassReady === true
    && composerToolsPanelDetails.marker === "light-glass"
    && composerToolsPanelDetails.light_glass_ready === true
    && composerToolsPanelDetails.effective_luminance >= 0.72
    && composerToolsPanelDetails.effective_luminance <= 0.98
    && /blur\\(/.test(composerToolsPanelDetails.backdrop_filter || "")
    && composerToolsPanelDetails.box_shadow !== "none"
    && composerToolsPanelDetails.in_viewport === true
    && composerToolsPanelDetails.top_clipped === false
    && composerToolsPanelDetails.bottom_clipped === false
    && composerToolsItemDetails.length === 2
    && expectedComposerToolsKeys.every((key) => composerToolsItemDetails.some((item) => item.key === key))
    && composerToolsItemDetails.every((item) => (
      item.visible
      && item.role === "menuitem"
      && item.key.length > 0
      && item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.icon_present
      && item.icon_svg_present
      && item.label_ready
      && item.select_present
      && item.select_visible
      && item.select_aria_label.length > 0
      && item.select_title.length > 0
      && item.select_title_matches_aria_label
      && item.select_height >= 44
      && item.select_readable
      && item.select_contrast_ratio >= 4.5
      && item.height >= 44
      && item.label_nowrap_ready
      && item.readable
      && item.contrast_ratio >= 4.5
    ));
  const composerPopoverToggleDetails = Array.from(document.querySelectorAll("[data-chat-composer-popover-toggle]")).map((node) => {
    const style = getComputedStyle(node);
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    return {
      key: node.getAttribute("data-chat-composer-popover-toggle") || "",
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      aria_haspopup: node.getAttribute("aria-haspopup") || "",
      aria_controls: node.getAttribute("aria-controls") || "",
      visible: elementVisible(node),
      svg_icon_present: hasSvgIcon(node),
      visible_icon_text_absent: visibleText(node).length === 0,
      ...richRect(node),
    };
  });
  const composerPopoverPanelDetails = Array.from(document.querySelectorAll('[data-control-ui-composer-popover-panel="light-glass"]')).map((node) => {
    const style = getComputedStyle(node);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const rect = richRect(node);
    const bgColor = effectiveBackground(node);
    const bgLuminance = relativeLuminance(bgColor);
    const horizontalInViewport = rect.left >= -1 && rect.right <= window.innerWidth + 1 && rect.width <= window.innerWidth - 16;
    const verticalInViewport = rect.top >= -1 && rect.bottom <= window.innerHeight + 1 && rect.height <= window.innerHeight - 16;
    return {
      key: node.getAttribute("data-chat-composer-popover") || "",
      window_width: window.innerWidth,
      inline_style: node.getAttribute("style") || "",
      role: node.getAttribute("role") || "",
      marker: node.getAttribute("data-control-ui-composer-popover-panel") || "",
      aria_label: node.getAttribute("aria-label") || "",
      visible: elementVisible(node),
      search_count: node.querySelectorAll("[data-chat-composer-picker-search]").length,
      item_count: node.querySelectorAll("[data-chat-composer-picker-item]").length,
      background_color: style.backgroundColor,
      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
      translucent_ready: translucentGlassReady(style),
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      effective_luminance: Number(bgLuminance.toFixed(3)),
      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      horizontal_in_viewport: horizontalInViewport,
      vertical_in_viewport: verticalInViewport,
      in_viewport: horizontalInViewport && verticalInViewport,
      top_clipped: rect.top < -1,
      bottom_clipped: rect.bottom > window.innerHeight + 1,
      ...rect,
    };
  });
  const composerPopoverHeaderDetails = Array.from(document.querySelectorAll('[data-control-ui-composer-popover-panel="light-glass"] .tg-composer-popover__header')).map((node) => {
    const panel = node.closest("[data-control-ui-composer-popover-panel]");
    const labelNode = node.querySelector("strong");
    const statusNode = node.querySelector("span");
    const labelStyle = labelNode ? getComputedStyle(labelNode) : null;
    const statusStyle = statusNode ? getComputedStyle(statusNode) : null;
    const bgColor = effectiveBackground(panel || node);
    const labelColor = parseCssColor((labelStyle || getComputedStyle(node)).color);
    const statusColor = parseCssColor((statusStyle || getComputedStyle(node)).color);
    const labelRatio = labelColor ? contrastRatio(labelColor, bgColor) : 0;
    const statusRatio = statusColor ? contrastRatio(statusColor, bgColor) : 0;
    const labelTextShadow = labelStyle?.textShadow || "";
    const statusTextShadow = statusStyle?.textShadow || "";
    const labelTextShadowCount = labelTextShadow && labelTextShadow !== "none" ? ((labelTextShadow.match(/rgb/g) || []).length || 1) : 0;
    const statusTextShadowCount = statusTextShadow && statusTextShadow !== "none" ? ((statusTextShadow.match(/rgb/g) || []).length || 1) : 0;
    return {
      key: panel?.getAttribute("data-chat-composer-popover") || "",
      label: (labelNode?.textContent || "").replace(/\\s+/g, " ").trim(),
      status: (statusNode?.textContent || "").replace(/\\s+/g, " ").trim(),
      visible: elementVisible(node),
      label_visible: Boolean(labelNode) && elementVisible(labelNode),
      status_visible: Boolean(statusNode) && elementVisible(statusNode),
      label_text_shadow: labelTextShadow && labelTextShadow !== "none" ? "present" : "none",
      label_text_shadow_sample: labelTextShadow.slice(0, 180),
      status_text_shadow: statusTextShadow && statusTextShadow !== "none" ? "present" : "none",
      status_text_shadow_sample: statusTextShadow.slice(0, 180),
      composer_popover_header_label_text_shadow_count: labelTextShadowCount,
      composer_popover_header_status_text_shadow_count: statusTextShadowCount,
      composer_popover_header_prismatic_etch_ready: labelTextShadowCount >= 2 && statusTextShadowCount >= 2,
      label_contrast_ratio: Number(labelRatio.toFixed(2)),
      status_contrast_ratio: Number(statusRatio.toFixed(2)),
      label_readable: labelRatio >= 4.5,
      status_readable: statusRatio >= 4.5,
      ...richRect(node),
    };
  });
  const composerPopoverSearchDetails = Array.from(document.querySelectorAll("[data-chat-composer-picker-search]")).map((node) => {
    const style = getComputedStyle(node);
    const placeholderStyle = getComputedStyle(node, "::placeholder");
    const textColorValue = style.webkitTextFillColor || style.color;
    const textColor = parseCssColor(textColorValue);
    const placeholderColor = parseCssColor(placeholderStyle.color);
    const bgColor = effectiveBackground(node);
    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
    const placeholderRatio = placeholderColor ? contrastRatio(placeholderColor, bgColor) : 0;
    const placeholderTextShadow = placeholderStyle.textShadow || "";
    const placeholderTextShadowCount = placeholderTextShadow && placeholderTextShadow !== "none" ? ((placeholderTextShadow.match(/rgb/g) || []).length || 1) : 0;
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const bgLuminance = relativeLuminance(bgColor);
    const marker = node.getAttribute("data-control-ui-composer-popover-search") || "";
    return {
      key: node.getAttribute("data-chat-composer-picker-search") || "",
      marker,
      placeholder: node.getAttribute("placeholder") || "",
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      visible: elementVisible(node),
      background_color: style.backgroundColor,
      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
      translucent_ready: translucentGlassReady(style),
      color: textColorValue,
      effective_luminance: Number(bgLuminance.toFixed(3)),
      light_glass_ready: marker === "light-glass" && bgLuminance >= 0.72 && bgLuminance <= 0.98,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      placeholder_text_shadow: placeholderTextShadow && placeholderTextShadow !== "none" ? "present" : "none",
      placeholder_text_shadow_sample: placeholderTextShadow.slice(0, 180),
      composer_popover_search_placeholder_text_shadow_count: placeholderTextShadowCount,
      composer_popover_search_placeholder_prismatic_etch_ready: placeholderTextShadowCount >= 2,
      placeholder_contrast_ratio: Number(placeholderRatio.toFixed(2)),
      placeholder_readable: placeholderRatio >= 4.5,
      ...richRect(node),
    };
  });
  const composerPopoverItemDetails = Array.from(document.querySelectorAll("[data-chat-composer-picker-item]")).map((node) => {
    const labelNode = node.querySelector("b");
    const smallNode = node.querySelector("small");
    const iconNode = node.querySelector(".tg-composer-popover__icon");
    const style = getComputedStyle(node);
    const labelStyle = labelNode ? getComputedStyle(labelNode) : null;
    const detailStyle = smallNode ? getComputedStyle(smallNode) : null;
    const textColor = parseCssColor((labelStyle || style).color);
    const detailColor = parseCssColor((detailStyle || style).color);
    const bgColor = effectiveBackground(node);
    const bgLuminance = relativeLuminance(bgColor);
    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
    const detailRatio = detailColor ? contrastRatio(detailColor, bgColor) : 0;
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const labelTextShadow = labelStyle?.textShadow || "";
    const detailTextShadow = detailStyle?.textShadow || "";
    const labelTextShadowCount = labelTextShadow && labelTextShadow !== "none" ? ((labelTextShadow.match(/rgb/g) || []).length || 1) : 0;
    const detailTextShadowCount = detailTextShadow && detailTextShadow !== "none" ? ((detailTextShadow.match(/rgb/g) || []).length || 1) : 0;
    return {
      key: node.getAttribute("data-chat-composer-picker-item") || "",
      role: node.getAttribute("role") || "",
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      label: (labelNode?.textContent || "").replace(/\\s+/g, " ").trim(),
      detail: (smallNode?.textContent || "").replace(/\\s+/g, " ").trim(),
      visible: elementVisible(node),
      icon_present: Boolean(iconNode),
      icon_svg_present: hasSvgIcon(iconNode),
      label_nowrap_ready: labelStyle?.whiteSpace === "nowrap",
      detail_nowrap_ready: detailStyle?.whiteSpace === "nowrap",
      background_color: style.backgroundColor,
      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
      translucent_ready: translucentGlassReady(style),
      effective_luminance: Number(bgLuminance.toFixed(3)),
      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      detail_contrast_ratio: Number(detailRatio.toFixed(2)),
      detail_readable: detailRatio >= 4.5,
      label_text_shadow: labelTextShadow && labelTextShadow !== "none" ? "present" : "none",
      label_text_shadow_sample: labelTextShadow.slice(0, 180),
      detail_text_shadow: detailTextShadow && detailTextShadow !== "none" ? "present" : "none",
      detail_text_shadow_sample: detailTextShadow.slice(0, 180),
      composer_popover_item_label_text_shadow_count: labelTextShadowCount,
      composer_popover_item_detail_text_shadow_count: detailTextShadowCount,
      composer_popover_item_label_prismatic_etch_ready: labelTextShadowCount >= 2 && detailTextShadowCount >= 2,
      ...richRect(node),
    };
  });
  const expectedComposerPopoverKeys = ["artifact", "command"];
  const composerPopoverReady = composerPopoverToggleDetails.length === 2
    && composerPopoverToggleDetails.every((item) => (
      item.visible
      && expectedComposerPopoverKeys.includes(item.key)
      && item.width >= 44
      && item.height >= 44
      && item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.aria_haspopup === "menu"
      && item.aria_controls.length > 0
      && item.svg_icon_present
      && item.visible_icon_text_absent
    ))
    && composerPopoverPanelDetails.length === 2
    && expectedComposerPopoverKeys.every((key) => composerPopoverPanelDetails.some((item) => item.key === key))
    && composerPopoverPanelDetails.every((item) => (
      item.visible
      && item.role === "menu"
      && item.aria_label.length > 0
      && item.search_count === 1
      && item.item_count === 2
      && item.width >= 180
      && item.height >= 132
      && item.border_radius >= 16
      && item.marker === "light-glass"
      && item.translucent_ready === true
      && item.light_glass_ready
      && item.effective_luminance >= 0.72
      && item.effective_luminance <= 0.98
      && /blur\\(/.test(item.backdrop_filter || "")
      && item.box_shadow !== "none"
      && item.in_viewport
      && item.top_clipped === false
      && item.bottom_clipped === false
    ))
    && composerPopoverSearchDetails.length === 2
    && composerPopoverSearchDetails.every((item) => (
      item.visible
      && expectedComposerPopoverKeys.includes(item.key)
      && item.height >= 44
      && item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.readable
      && item.contrast_ratio >= 4.5
    ))
    && composerPopoverItemDetails.length === 4
    && composerPopoverItemDetails.every((item) => (
      item.visible
      && item.role === "menuitem"
      && item.key.length > 0
      && item.width >= 120
      && item.height >= 44
      && item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.label.length > 0
      && item.detail.length > 0
      && item.icon_present
      && item.icon_svg_present
      && item.light_glass_ready
      && item.translucent_ready === true
      && item.label_nowrap_ready
      && item.readable
      && item.contrast_ratio >= 4.5
    ));
  const composerPopoverItemLabelPrismaticEtchLightGlassReady = composerPopoverReady
    && composerPopoverItemDetails.every((item) => (
      item.visible
      && item.label.length > 0
      && item.detail.length > 0
      && item.label_text_shadow === "present"
      && item.detail_text_shadow === "present"
      && item.composer_popover_item_label_prismatic_etch_ready === true
      && item.composer_popover_item_label_text_shadow_count >= 2
      && item.composer_popover_item_detail_text_shadow_count >= 2
      && item.readable
      && item.detail_readable
      && item.contrast_ratio >= 4.5
      && item.detail_contrast_ratio >= 4.5
      && item.label_nowrap_ready
      && item.detail_nowrap_ready
    ));
  const composerPopoverHeaderPrismaticEtchLightGlassReady = composerPopoverReady
    && composerPopoverHeaderDetails.length === 2
    && expectedComposerPopoverKeys.every((key) => composerPopoverHeaderDetails.some((item) => item.key === key))
    && composerPopoverHeaderDetails.every((item) => (
      item.visible
      && item.label_visible
      && item.status_visible
      && item.label.length > 0
      && item.status.length > 0
      && item.label_text_shadow === "present"
      && item.status_text_shadow === "present"
      && item.composer_popover_header_prismatic_etch_ready === true
      && item.composer_popover_header_label_text_shadow_count >= 2
      && item.composer_popover_header_status_text_shadow_count >= 2
      && item.label_readable
      && item.status_readable
      && item.label_contrast_ratio >= 4.5
      && item.status_contrast_ratio >= 4.5
    ));
  const composerPopoverSearchLightGlassReady = composerPopoverSearchDetails.length === 2
    && expectedComposerPopoverKeys.every((key) => composerPopoverSearchDetails.some((item) => item.key === key))
    && composerPopoverSearchDetails.every((item) => (
      item.visible
      && item.marker === "light-glass"
      && item.height >= 44
      && item.border_radius >= 10
      && item.light_glass_ready
      && item.translucent_ready === true
      && item.effective_luminance >= 0.72
      && item.effective_luminance <= 0.98
      && (item.backdrop_filter || "").includes("blur(")
      && item.box_shadow !== "none"
      && item.readable
      && item.contrast_ratio >= 4.5
    ));
  const composerPopoverSearchPlaceholderPrismaticEtchLightGlassReady = composerPopoverSearchLightGlassReady
    && composerPopoverSearchDetails.every((item) => (
      item.placeholder.length > 0
      && item.placeholder_text_shadow === "present"
      && item.composer_popover_search_placeholder_prismatic_etch_ready === true
      && item.composer_popover_search_placeholder_text_shadow_count >= 2
      && item.placeholder_readable === true
      && item.placeholder_contrast_ratio >= 4.5
    ));
  const railSearchNodes = Array.from(document.querySelectorAll("[data-control-ui-rail-search-input]"));
  const railSearchDetails = railSearchNodes.map((node) => {
    const style = getComputedStyle(node);
    const placeholderStyle = getComputedStyle(node, "::placeholder");
    const textColor = parseCssColor(style.webkitTextFillColor || style.color);
    const placeholderColor = parseCssColor(placeholderStyle.color);
    const bgColor = effectiveBackground(node);
    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
    const placeholderRatio = placeholderColor ? contrastRatio(placeholderColor, bgColor) : 0;
    const placeholderTextShadow = placeholderStyle.textShadow || "";
    const placeholderTextShadowCount = placeholderTextShadow && placeholderTextShadow !== "none" ? ((placeholderTextShadow.split("rgba(").length - 1) + (placeholderTextShadow.split("rgb(").length - 1) || 1) : 0;
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const rect = richRect(node);
    const effectiveLuminance = relativeLuminance(bgColor);
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const filterText = style.filter || "";
    const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
    return {
      marker: node.getAttribute("data-control-ui-rail-search-input") || "",
      placeholder: node.getAttribute("placeholder") || "",
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      visible: elementVisible(node),
      type: node.getAttribute("type") || "",
      border_radius: styleNumber(style, "borderTopLeftRadius"),
      background_color: style.backgroundColor,
      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
      translucent_ready: translucentGlassReady(style),
      effective_luminance: Number(effectiveLuminance.toFixed(3)),
      light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      color: style.color,
      text_fill_color: style.webkitTextFillColor || "",
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      placeholder_color: placeholderStyle.color,
      placeholder_text_shadow: placeholderTextShadow && placeholderTextShadow !== "none" ? "present" : "none",
      placeholder_text_shadow_sample: placeholderTextShadow.slice(0, 180),
      rail_search_placeholder_text_shadow_count: placeholderTextShadowCount,
      rail_search_placeholder_prismatic_etch_ready: placeholderTextShadowCount >= 2,
      placeholder_contrast_ratio: Number(placeholderRatio.toFixed(2)),
      placeholder_readable: placeholderRatio >= 4.5,
      filter: filterText && filterText !== "none" ? "present" : "none",
      filter_sample: filterText.slice(0, 180),
      rail_filter_drop_shadow_count: dropShadowCount,
      rail_prismatic_filter_ready: dropShadowCount >= 2,
      ...rect,
    };
  });
  const visibleRailSearchDetails = railSearchDetails.filter((item) => item.visible);
  const railSearchPlaceholderPrismaticEtchDetails = visibleRailSearchDetails.map((item) => ({
    placeholder: item.placeholder,
    visible: item.visible,
    width: item.width,
    height: item.height,
    placeholder_text_shadow: item.placeholder_text_shadow,
    placeholder_text_shadow_sample: item.placeholder_text_shadow_sample,
    rail_search_placeholder_text_shadow_count: item.rail_search_placeholder_text_shadow_count,
    rail_search_placeholder_prismatic_etch_ready: item.rail_search_placeholder_prismatic_etch_ready,
    placeholder_color: item.placeholder_color,
    placeholder_contrast_ratio: item.placeholder_contrast_ratio,
    placeholder_readable: item.placeholder_readable,
  }));
  const railSearchPlaceholderPrismaticEtchLightGlassReady = railVisible
    ? visibleRailSearchDetails.length === 1
      && visibleRailSearchDetails.every((item) => (
        item.placeholder.length > 0
        && item.width >= 180
        && item.height >= 44
        && item.placeholder_text_shadow === "present"
        && item.rail_search_placeholder_prismatic_etch_ready === true
        && item.rail_search_placeholder_text_shadow_count >= 2
        && item.placeholder_readable
        && item.placeholder_contrast_ratio >= 4.5
      ))
    : visibleRailSearchDetails.length === 0;
  const railSearchLightGlassReady = railVisible
    ? (
      railSearchNodes.length === 1
      && visibleRailSearchDetails.length === 1
      && visibleRailSearchDetails.every((item) => (
        item.marker === "light-glass"
        && item.type === "search"
        && item.placeholder.length > 0
        && item.aria_label === "Search chats"
        && item.title === "Search chats"
        && item.title_matches_aria_label
        && item.width >= 180
        && item.height >= 44
        && item.border_radius >= 12
        && item.light_glass_ready
        && item.translucent_ready === true
        && item.effective_luminance >= 0.72
        && item.effective_luminance <= 0.98
        && (item.backdrop_filter || "").includes("blur(")
        && item.box_shadow !== "none"
        && item.readable
        && item.contrast_ratio >= 4.5
        && item.placeholder_readable
        && item.placeholder_contrast_ratio >= 4.5
      ))
    )
    : (railSearchNodes.length === 1 && visibleRailSearchDetails.length === 0);
  const railPrismaticFilterDetails = [
    ...visibleRailSearchDetails.map((item) => ({ kind: "search", ...item })),
    ...folderChipDetails.map((item) => ({ kind: "folder-chip", ...item })),
  ];
  const railPrismaticFilterLightGlassReady = railVisible
    ? (
      railPrismaticFilterDetails.length >= 4
      && railPrismaticFilterDetails.every((item) => (
        item.visible
        && item.width >= 44
        && item.height >= 44
        && item.border_radius >= 12
        && item.box_shadow !== "none"
        && (item.backdrop_filter || "").includes("blur(")
        && item.rail_prismatic_filter_ready === true
        && item.rail_filter_drop_shadow_count >= 2
      ))
    )
    : railPrismaticFilterDetails.length === 0;
  const microSurfaceDetails = Array.from(document.querySelectorAll("[data-control-ui-micro-surface]"))
    .filter(elementVisible)
    .map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const rect = richRect(node);
      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
      const filterText = style.filter || "";
      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
      const textShadow = style.textShadow || "";
      const textShadowCount = textShadow && textShadow !== "none" ? ((textShadow.match(/rgb/g) || []).length || 1) : 0;
      const background = parseCssColor(style.backgroundColor);
      const effectiveLuminance = relativeLuminance(bgColor);
      const label = visibleText(node);
      return {
        key: node.getAttribute("data-control-ui-micro-surface") || "",
        text: label,
        visible: true,
        min_height: styleNumber(style, "minHeight"),
        border_radius: styleNumber(style, "borderTopLeftRadius"),
        background_color: style.backgroundColor,
        background_alpha: background ? background.a : 0,
        translucent_ready: translucentGlassReady(style),
        effective_luminance: Number(effectiveLuminance.toFixed(3)),
        light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
        backdrop_filter: backdrop,
        box_shadow: compactShadow(style.boxShadow),
        filter: filterText && filterText !== "none" ? "present" : "none",
        filter_sample: filterText.slice(0, 180),
        micro_prismatic_badge_drop_shadow_count: dropShadowCount,
        micro_prismatic_badge_ready: dropShadowCount >= 2,
        text_shadow: textShadow && textShadow !== "none" ? "present" : "none",
        text_shadow_sample: textShadow.slice(0, 180),
        micro_badge_label_text_shadow_count: textShadowCount,
        micro_badge_label_prismatic_etch_ready: textShadowCount >= 2,
        color: style.color,
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        label_nowrap_ready: style.whiteSpace === "nowrap",
        ...rect,
      };
    });
  const engineeringSessionChipDetails = Array.from(document.querySelectorAll(".tg-session-state")).map((node) => ({
    text: visibleText(node),
    visible: elementVisible(node),
    display: getComputedStyle(node).display,
    ...richRect(node),
  }));
  const engineeringSessionChipsSuppressedReady = engineeringSessionChipDetails.every((item) => item.visible === false);
  const expectedMicroSurfaceKeys = railVisible
    ? ["unread-count", "thread-status-local", "thread-status-safe-review", "date-divider", "routing-safe-preview", "routing-local-only", "composer-status-ready"]
    : ["thread-status-local", "thread-status-safe-review", "date-divider", "routing-safe-preview", "routing-local-only"];
  const threadIntroBadgeDetails = microSurfaceDetails.filter((item) => item.key.startsWith("thread-intro-"));
  const threadIntroStrip = document.querySelector(".tg-thread-intro");
  const threadIntroVisible = Boolean(threadIntroStrip && elementVisible(threadIntroStrip));
  const expectedThreadIntroBadgeKeys = [
    "thread-intro-telegram-shell",
    "thread-intro-message-workflow",
    "thread-intro-evidence-inline",
    "thread-intro-approval-chat",
  ];
  const threadIntroBadgeNodes = Array.from(document.querySelectorAll("[data-control-ui-thread-intro-badge]"));
  const threadIntroBadgeLightGlassReady = threadIntroVisible
    ? (
      threadIntroBadgeDetails.length === 4
      && expectedThreadIntroBadgeKeys.every((key) => threadIntroBadgeDetails.some((item) => item.key === key))
      && threadIntroBadgeNodes.length === 4
      && threadIntroBadgeNodes.every((node) => {
        const key = node.getAttribute("data-control-ui-thread-intro-badge") || "";
        const ariaLabel = node.getAttribute("aria-label") || "";
        const title = node.getAttribute("title") || "";
        return key.length > 0 && ariaLabel.length > 0 && title.length > 0 && title === ariaLabel;
      })
      && threadIntroBadgeDetails.every((item) => (
        item.visible
        && item.text.length > 0
        && item.height >= 22
        && item.border_radius >= 10
        && item.light_glass_ready
        && item.translucent_ready === true
        && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
        && (item.backdrop_filter || "").includes("blur(")
        && item.box_shadow !== "none"
        && item.readable
        && item.contrast_ratio >= 4.5
        && item.label_nowrap_ready
      ))
    )
    : threadIntroBadgeDetails.length === 0;
  const statusTrustStrip = document.querySelector("[data-control-ui-status-trust-strip]");
  const statusTrustStripVisible = Boolean(statusTrustStrip && elementVisible(statusTrustStrip));
  const expectedStatusTrustBadgeKeys = ["local", "safe-review"];
  const statusTrustBadgeNodes = Array.from(document.querySelectorAll("[data-control-ui-status-trust-badge]"));
  const statusTrustBadgeDetails = statusTrustBadgeNodes
    .filter(elementVisible)
    .map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const rect = richRect(node);
      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
      const background = parseCssColor(style.backgroundColor);
      const effectiveLuminance = relativeLuminance(bgColor);
      const ariaLabel = node.getAttribute("aria-label") || "";
      const title = node.getAttribute("title") || "";
      return {
        key: node.getAttribute("data-control-ui-status-trust-badge") || "",
        micro_surface_key: node.getAttribute("data-control-ui-micro-surface") || "",
        text: visibleText(node),
        visible: true,
        min_height: styleNumber(style, "minHeight"),
        border_radius: styleNumber(style, "borderTopLeftRadius"),
        background_color: style.backgroundColor,
        background_alpha: background ? background.a : 0,
        translucent_ready: translucentGlassReady(style),
        effective_luminance: Number(effectiveLuminance.toFixed(3)),
        light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
        backdrop_filter: backdrop,
        box_shadow: compactShadow(style.boxShadow),
        color: style.color,
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        label_nowrap_ready: style.whiteSpace === "nowrap",
        aria_label: ariaLabel,
        title,
        title_matches_aria_label: title === ariaLabel,
        ...rect,
      };
    });
  const statusTrustStripLightGlassReady = statusTrustStripVisible
    && statusTrustStrip.getAttribute("data-control-ui-status-trust-strip") === "local-safe-review"
    && statusTrustStrip.getAttribute("role") === "group"
    && statusTrustStrip.getAttribute("aria-label") === "Thread status trust"
    && statusTrustBadgeNodes.length === 2
    && statusTrustBadgeDetails.length === 2
    && expectedStatusTrustBadgeKeys.every((key) => statusTrustBadgeDetails.some((item) => item.key === key))
    && statusTrustBadgeDetails.every((item) => (
      item.visible
      && item.text.length > 0
      && item.height >= 22
      && item.border_radius >= 10
      && item.light_glass_ready
      && item.translucent_ready === true
      && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
      && (item.backdrop_filter || "").includes("blur(")
      && item.box_shadow !== "none"
      && item.readable
      && item.contrast_ratio >= 4.5
      && item.label_nowrap_ready
      && item.title.length > 0
      && item.aria_label.length > 0
      && item.title_matches_aria_label
    ));
  const routingBadgeDetails = microSurfaceDetails.filter((item) => item.key === "routing-safe-preview" || item.key === "routing-local-only");
  const messageRoutingBadgeLightGlassReady = routingBadgeDetails.length === 2
    && ["routing-safe-preview", "routing-local-only"].every((key) => routingBadgeDetails.some((item) => item.key === key))
    && routingBadgeDetails.every((item) => (
      item.visible
      && item.text.length > 0
      && item.height >= 22
      && item.border_radius >= 10
      && item.light_glass_ready
      && item.translucent_ready === true
      && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
      && (item.backdrop_filter || "").includes("blur(")
      && item.box_shadow !== "none"
      && item.readable
      && item.contrast_ratio >= 4.5
      && item.label_nowrap_ready
    ));
  const microSurfaceLightGlassReady = microSurfaceDetails.length >= expectedMicroSurfaceKeys.length
    && expectedMicroSurfaceKeys.every((key) => microSurfaceDetails.some((item) => item.key === key))
    && microSurfaceDetails.every((item) => (
      item.key.length > 0
      && item.text.length > 0
      && item.height >= 22
      && item.border_radius >= 10
      && item.light_glass_ready
      && item.translucent_ready === true
      && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
      && (item.backdrop_filter || "").includes("blur(")
      && item.box_shadow !== "none"
      && item.readable
      && item.contrast_ratio >= 4.5
      && item.label_nowrap_ready
    ));
  const microPrismaticBadgeLightGlassReady = microSurfaceLightGlassReady
    && microSurfaceDetails.every((item) => (
      item.micro_prismatic_badge_ready === true
      && item.micro_prismatic_badge_drop_shadow_count >= 2
      && item.box_shadow !== "none"
      && (item.backdrop_filter || "").includes("blur(")
    ));
  const microBadgeLabelPrismaticEtchLightGlassReady = microSurfaceLightGlassReady
    && microSurfaceDetails.every((item) => (
      item.text_shadow === "present"
      && item.micro_badge_label_prismatic_etch_ready === true
      && item.micro_badge_label_text_shadow_count >= 2
      && item.readable === true
      && item.contrast_ratio >= 4.5
    ));
  const messageMetadataPrismaticDetails = Array.from(document.querySelectorAll(".tg-message small"))
    .filter(elementVisible)
    .map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
      const filterText = style.filter || "";
      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
      const background = parseCssColor(style.backgroundColor);
      const effectiveLuminance = relativeLuminance(bgColor);
      return {
        text: visibleText(node),
        visible: true,
        height: richRect(node).height,
        width: richRect(node).width,
        border_radius: styleNumber(style, "borderTopLeftRadius"),
        background_color: style.backgroundColor,
        background_alpha: background ? background.a : 0,
        translucent_ready: translucentGlassReady(style),
        effective_luminance: Number(effectiveLuminance.toFixed(3)),
        light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
        backdrop_filter: backdrop,
        box_shadow: compactShadow(style.boxShadow),
        filter: filterText && filterText !== "none" ? "present" : "none",
        filter_sample: filterText.slice(0, 180),
        message_metadata_drop_shadow_count: dropShadowCount,
        message_metadata_prismatic_ready: dropShadowCount >= 2,
        color: style.color,
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        label_nowrap_ready: style.whiteSpace === "nowrap",
      };
    });
  const messageMetadataPrismaticLightGlassReady = messageMetadataPrismaticDetails.length >= 3
    && messageMetadataPrismaticDetails.every((item) => (
      item.visible
      && item.text.length > 0
      && item.height >= 22
      && item.border_radius >= 10
      && item.light_glass_ready
      && item.translucent_ready === true
      && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
      && (item.backdrop_filter || "").includes("blur(")
      && item.box_shadow !== "none"
      && item.message_metadata_prismatic_ready === true
      && item.message_metadata_drop_shadow_count >= 2
      && item.readable
      && item.contrast_ratio >= 4.5
      && item.label_nowrap_ready
    ));
  const threadSubtitlePrismaticDetails = Array.from(document.querySelectorAll(".tg-thread-header__main p"))
    .filter(elementVisible)
    .map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
      const filterText = style.filter || "";
      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
      const background = parseCssColor(style.backgroundColor);
      const effectiveLuminance = relativeLuminance(bgColor);
      return {
        text: visibleText(node),
        visible: true,
        border_radius: styleNumber(style, "borderTopLeftRadius"),
        background_color: style.backgroundColor,
        background_alpha: background ? background.a : 0,
        translucent_ready: translucentGlassReady(style),
        effective_luminance: Number(effectiveLuminance.toFixed(3)),
        light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
        backdrop_filter: backdrop,
        box_shadow: compactShadow(style.boxShadow),
        filter: filterText && filterText !== "none" ? "present" : "none",
        filter_sample: filterText.slice(0, 180),
        thread_subtitle_drop_shadow_count: dropShadowCount,
        thread_subtitle_prismatic_ready: dropShadowCount >= 2,
        color: style.color,
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        label_nowrap_ready: style.whiteSpace === "nowrap",
        ...richRect(node),
      };
    });
  const threadSubtitlePrismaticLightGlassReady = threadSubtitlePrismaticDetails.length >= 1
    && threadSubtitlePrismaticDetails.every((item) => (
      item.visible
      && item.text.length > 0
      && item.height >= 22
      && item.border_radius >= 10
      && item.light_glass_ready
      && item.translucent_ready === true
      && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
      && (item.backdrop_filter || "").includes("blur(")
      && item.box_shadow !== "none"
      && item.thread_subtitle_prismatic_ready === true
      && item.thread_subtitle_drop_shadow_count >= 2
      && item.readable
      && item.contrast_ratio >= 4.5
      && item.label_nowrap_ready
    ));
  const composerShortcutHintPrismaticDetails = Array.from(document.querySelectorAll("[data-chat-shortcut-hint]"))
    .filter(elementVisible)
    .map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
      const filterText = style.filter || "";
      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
      const background = parseCssColor(style.backgroundColor);
      const effectiveLuminance = relativeLuminance(bgColor);
      return {
        text: visibleText(node),
        visible: true,
        border_radius: styleNumber(style, "borderTopLeftRadius"),
        background_color: style.backgroundColor,
        background_alpha: background ? background.a : 0,
        translucent_ready: translucentGlassReady(style),
        effective_luminance: Number(effectiveLuminance.toFixed(3)),
        light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
        backdrop_filter: backdrop,
        box_shadow: compactShadow(style.boxShadow),
        filter: filterText && filterText !== "none" ? "present" : "none",
        filter_sample: filterText.slice(0, 180),
        composer_shortcut_hint_drop_shadow_count: dropShadowCount,
        composer_shortcut_hint_prismatic_ready: dropShadowCount >= 2,
        color: style.color,
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        label_nowrap_ready: style.whiteSpace === "nowrap",
        ...richRect(node),
      };
    });
  const composerShortcutHintExpectedVisible = window.innerWidth > 700;
  const composerShortcutHintPrismaticLightGlassReady = composerShortcutHintExpectedVisible
    ? composerShortcutHintPrismaticDetails.length >= 1
      && composerShortcutHintPrismaticDetails.every((item) => (
        item.visible
        && item.text.length > 0
        && item.height >= 22
        && item.border_radius >= 10
        && item.light_glass_ready
        && item.translucent_ready === true
        && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
        && (item.backdrop_filter || "").includes("blur(")
        && item.box_shadow !== "none"
        && item.composer_shortcut_hint_prismatic_ready === true
        && item.composer_shortcut_hint_drop_shadow_count >= 2
        && item.readable
        && item.contrast_ratio >= 4.5
        && item.label_nowrap_ready
      ))
    : composerShortcutHintPrismaticDetails.length === 0;
  const railMetadataChipPrismaticDetails = Array.from(document.querySelectorAll(".tg-conversation-rail .tg-chat-item__topline span"))
    .filter(elementVisible)
    .map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
      const filterText = style.filter || "";
      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
      const background = parseCssColor(style.backgroundColor);
      const effectiveLuminance = relativeLuminance(bgColor);
      return {
        text: visibleText(node),
        visible: true,
        border_radius: styleNumber(style, "borderTopLeftRadius"),
        background_color: style.backgroundColor,
        background_alpha: background ? background.a : 0,
        translucent_ready: translucentGlassReady(style),
        effective_luminance: Number(effectiveLuminance.toFixed(3)),
        light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
        backdrop_filter: backdrop,
        box_shadow: compactShadow(style.boxShadow),
        filter: filterText && filterText !== "none" ? "present" : "none",
        filter_sample: filterText.slice(0, 180),
        rail_metadata_chip_drop_shadow_count: dropShadowCount,
        rail_metadata_chip_prismatic_ready: dropShadowCount >= 2,
        color: style.color,
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        label_nowrap_ready: style.whiteSpace === "nowrap",
        ...richRect(node),
      };
    });
  const railMetadataChipExpectedVisible = window.innerWidth > 700;
  const railMetadataChipPrismaticLightGlassReady = railMetadataChipExpectedVisible
    ? railMetadataChipPrismaticDetails.length >= 3
      && railMetadataChipPrismaticDetails.every((item) => (
        item.visible
        && item.text.length > 0
        && item.height >= 22
        && item.border_radius >= 10
        && item.light_glass_ready
        && item.translucent_ready === true
        && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
        && (item.backdrop_filter || "").includes("blur(")
        && item.box_shadow !== "none"
        && item.rail_metadata_chip_prismatic_ready === true
        && item.rail_metadata_chip_drop_shadow_count >= 2
        && item.readable
        && item.contrast_ratio >= 4.5
        && item.label_nowrap_ready
      ))
    : railMetadataChipPrismaticDetails.length === 0;
  const railStatusCountPrismaticDetails = Array.from(document.querySelectorAll(".tg-conversation-rail .tg-rail-status__item"))
    .filter(elementVisible)
    .map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
      const filterText = style.filter || "";
      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
      const background = parseCssColor(style.backgroundColor);
      const effectiveLuminance = relativeLuminance(bgColor);
      return {
        text: visibleText(node),
        visible: true,
        border_radius: styleNumber(style, "borderTopLeftRadius"),
        background_color: style.backgroundColor,
        background_alpha: background ? background.a : 0,
        translucent_ready: translucentGlassReady(style),
        effective_luminance: Number(effectiveLuminance.toFixed(3)),
        light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
        backdrop_filter: backdrop,
        box_shadow: compactShadow(style.boxShadow),
        filter: filterText && filterText !== "none" ? "present" : "none",
        filter_sample: filterText.slice(0, 180),
        rail_status_count_drop_shadow_count: dropShadowCount,
        rail_status_count_prismatic_ready: dropShadowCount >= 2,
        color: style.color,
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        label_nowrap_ready: style.whiteSpace === "nowrap",
        ...richRect(node),
      };
    });
	  const railStatusCountExpectedVisible = window.innerWidth > 700;
	  const railStatusCountPrismaticLightGlassReady = railStatusCountExpectedVisible
	    ? railStatusCountPrismaticDetails.length >= 1
      && railStatusCountPrismaticDetails.every((item) => (
        item.visible
        && item.text.length > 0
        && item.height >= 22
        && item.border_radius >= 10
        && item.light_glass_ready
        && item.translucent_ready === true
        && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
        && (item.backdrop_filter || "").includes("blur(")
        && item.box_shadow !== "none"
        && item.rail_status_count_prismatic_ready === true
        && item.rail_status_count_drop_shadow_count >= 2
        && item.readable
        && item.contrast_ratio >= 4.5
        && item.label_nowrap_ready
	      ))
	    : railStatusCountPrismaticDetails.length === 0;
	  const railPreviewPrismaticEtchDetails = Array.from(document.querySelectorAll(".tg-conversation-rail .tg-chat-item__body p"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const textColor = parseCssColor(style.color);
	      const bgColor = effectiveBackground(node);
	      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	      const filterText = style.filter || "";
	      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	      return {
	        text: visibleText(node),
	        visible: true,
	        filter: filterText && filterText !== "none" ? "present" : "none",
	        filter_sample: filterText.slice(0, 180),
	        rail_preview_drop_shadow_count: dropShadowCount,
	        rail_preview_prismatic_etch_ready: dropShadowCount >= 2,
	        color: style.color,
	        contrast_ratio: Number(ratio.toFixed(2)),
	        readable: ratio >= 4.5,
	        font_weight: style.fontWeight,
	        ...richRect(node),
	      };
	    });
	  const railPreviewExpectedVisible = window.innerWidth > 700;
	  const railPreviewPrismaticEtchLightGlassReady = railPreviewExpectedVisible
	    ? railPreviewPrismaticEtchDetails.length >= 3
	      && railPreviewPrismaticEtchDetails.every((item) => (
	        item.visible
	        && item.text.length > 0
	        && item.width > 20
	        && item.height >= 14
	        && item.filter === "present"
	        && item.rail_preview_prismatic_etch_ready === true
	        && item.rail_preview_drop_shadow_count >= 2
	        && item.readable
	        && item.contrast_ratio >= 4.5
	      ))
	    : railPreviewPrismaticEtchDetails.length === 0;
	  const railChatTitlePrismaticEtchDetails = Array.from(document.querySelectorAll(".tg-conversation-rail .tg-chat-item__topline strong"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const textColor = parseCssColor(style.color);
	      const bgColor = effectiveBackground(node);
	      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	      const filterText = style.filter || "";
	      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	      return {
	        text: visibleText(node),
	        visible: true,
	        filter: filterText && filterText !== "none" ? "present" : "none",
	        filter_sample: filterText.slice(0, 180),
	        rail_chat_title_drop_shadow_count: dropShadowCount,
	        rail_chat_title_prismatic_etch_ready: dropShadowCount >= 2,
	        color: style.color,
	        contrast_ratio: Number(ratio.toFixed(2)),
	        readable: ratio >= 4.5,
	        font_weight: style.fontWeight,
	        ...richRect(node),
	      };
	    });
	  const railChatTitleExpectedVisible = window.innerWidth > 700;
	  const railChatTitlePrismaticEtchLightGlassReady = railChatTitleExpectedVisible
	    ? railChatTitlePrismaticEtchDetails.length >= 3
	      && railChatTitlePrismaticEtchDetails.every((item) => (
	        item.visible
	        && item.text.length > 0
	        && item.width > 20
	        && item.height >= 14
	        && item.filter === "present"
	        && item.rail_chat_title_prismatic_etch_ready === true
	        && item.rail_chat_title_drop_shadow_count >= 2
	        && item.readable
	        && item.contrast_ratio >= 4.5
	      ))
	    : railChatTitlePrismaticEtchDetails.length === 0;
	  const messageBodyPrismaticEtchDetails = Array.from(document.querySelectorAll(".tg-thread .tg-bubble p"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const textColor = parseCssColor(style.color);
	      const bgColor = effectiveBackground(node);
	      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	      const filterText = style.filter || "";
	      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	      return {
	        text: visibleText(node),
	        visible: true,
	        filter: filterText && filterText !== "none" ? "present" : "none",
	        filter_sample: filterText.slice(0, 180),
	        message_body_drop_shadow_count: dropShadowCount,
	        message_body_prismatic_etch_ready: dropShadowCount >= 2,
	        color: style.color,
	        contrast_ratio: Number(ratio.toFixed(2)),
	        readable: ratio >= 4.5,
	        font_weight: style.fontWeight,
	        ...richRect(node),
	      };
	    });
	  const messageBodyPrismaticEtchLightGlassReady = messageBodyPrismaticEtchDetails.length >= 3
	    && messageBodyPrismaticEtchDetails.every((item) => (
	      item.visible
	      && item.text.length > 0
	      && item.width > 20
	      && item.height >= 16
	      && item.filter === "present"
	      && item.message_body_prismatic_etch_ready === true
	      && item.message_body_drop_shadow_count >= 2
	      && item.readable
	      && item.contrast_ratio >= 4.5
	    ));
	  const messageSpeakerPrismaticChipDetails = Array.from(document.querySelectorAll(".tg-thread .tg-bubble>span"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const textColor = parseCssColor(style.color);
	      const bgColor = effectiveBackground(node);
	      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
	      const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
	      const filterText = style.filter || "";
	      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	      const background = parseCssColor(style.backgroundColor);
	      const effectiveLuminance = relativeLuminance(bgColor);
	      return {
	        text: visibleText(node),
	        visible: true,
	        border_radius: styleNumber(style, "borderTopLeftRadius"),
	        background_color: style.backgroundColor,
	        background_alpha: background ? background.a : 0,
	        translucent_ready: translucentGlassReady(style),
	        effective_luminance: Number(effectiveLuminance.toFixed(3)),
	        light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
	        backdrop_filter: backdrop,
	        box_shadow: compactShadow(style.boxShadow),
	        filter: filterText && filterText !== "none" ? "present" : "none",
	        filter_sample: filterText.slice(0, 180),
	        message_speaker_chip_drop_shadow_count: dropShadowCount,
	        message_speaker_prismatic_chip_ready: dropShadowCount >= 2,
	        color: style.color,
	        contrast_ratio: Number(ratio.toFixed(2)),
	        readable: ratio >= 4.5,
	        label_nowrap_ready: style.whiteSpace === "nowrap",
	        ...richRect(node),
	      };
	    });
	  const messageSpeakerPrismaticChipLightGlassReady = messageSpeakerPrismaticChipDetails.length >= 3
	    && messageSpeakerPrismaticChipDetails.every((item) => (
	      item.visible
	      && item.text.length > 0
	      && item.height >= 22
	      && item.border_radius >= 10
	      && item.light_glass_ready
	      && item.translucent_ready === true
	      && item.effective_luminance >= 0.72 && item.effective_luminance <= 0.98
	      && (item.backdrop_filter || "").includes("blur(")
	      && item.box_shadow !== "none"
	      && item.filter === "present"
	      && item.message_speaker_prismatic_chip_ready === true
	      && item.message_speaker_chip_drop_shadow_count >= 2
	      && item.readable
	      && item.contrast_ratio >= 4.5
	      && item.label_nowrap_ready
	    ));
	  const composerPlaceholderPrismaticEtchDetails = Array.from(document.querySelectorAll(".tg-compose-bar textarea[placeholder]"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const placeholderStyle = getComputedStyle(node, "::placeholder");
	      const placeholderColor = parseCssColor(placeholderStyle.color);
	      const bgColor = effectiveBackground(node);
	      const ratio = placeholderColor ? contrastRatio(placeholderColor, bgColor) : 0;
	      const textShadow = placeholderStyle.textShadow || "";
	      const textShadowCount = textShadow && textShadow !== "none" ? ((textShadow.split("rgba(").length - 1) + (textShadow.split("rgb(").length - 1) || 1) : 0;
	      return {
	        placeholder: node.getAttribute("placeholder") || "",
	        visible: true,
	        placeholder_text_shadow: textShadow && textShadow !== "none" ? "present" : "none",
	        placeholder_text_shadow_sample: textShadow.slice(0, 180),
	        composer_placeholder_text_shadow_count: textShadowCount,
	        composer_placeholder_prismatic_etch_ready: textShadowCount >= 2,
	        color: placeholderStyle.color,
	        contrast_ratio: Number(ratio.toFixed(2)),
	        readable: ratio >= 4.5,
	        font_weight: style.fontWeight,
	        ...richRect(node),
	      };
	    });
	  const composerPlaceholderPrismaticEtchLightGlassReady = composerPlaceholderPrismaticEtchDetails.length >= 1
	    && composerPlaceholderPrismaticEtchDetails.every((item) => (
	      item.visible
	      && item.placeholder.length > 0
	      && item.width >= 100
	      && item.height >= 44
	      && item.placeholder_text_shadow === "present"
	      && item.composer_placeholder_prismatic_etch_ready === true
	      && item.composer_placeholder_text_shadow_count >= 2
	      && item.readable
	      && item.contrast_ratio >= 4.5
	    ));
	  const headerTitlePrismaticEtchDetails = Array.from(document.querySelectorAll(".tg-rail-header h2,.tg-thread-header h2"))
	    .filter(elementVisible)
	    .map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const filterText = style.filter || "";
      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
      return {
        text: visibleText(node),
        visible: true,
        filter: filterText && filterText !== "none" ? "present" : "none",
        filter_sample: filterText.slice(0, 180),
        header_title_drop_shadow_count: dropShadowCount,
        header_title_prismatic_etch_ready: dropShadowCount >= 2,
        color: style.color,
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        font_weight: style.fontWeight,
        ...richRect(node),
      };
    });
  const headerTitleExpectedCount = window.innerWidth > 700 ? 2 : 1;
  const headerTitlePrismaticEtchLightGlassReady = headerTitlePrismaticEtchDetails.length >= headerTitleExpectedCount
    && headerTitlePrismaticEtchDetails.every((item) => (
      item.visible
      && item.text.length > 0
      && item.width > 20
      && item.height >= 16
      && item.filter === "present"
      && item.header_title_prismatic_etch_ready === true
      && item.header_title_drop_shadow_count >= 2
      && item.readable
      && item.contrast_ratio >= 4.5
    ));
  const navIconReady = Array.from(document.querySelectorAll(".nav .hepta-ui-icon")).length >= 4
    && Array.from(document.querySelectorAll(".nav a")).every((node) => hasSvgIcon(node.querySelector(".hepta-ui-icon")));
  const threadPanel = document.querySelector(".tg-thread-panel");
  const threadPanelAfter = threadPanel ? getComputedStyle(threadPanel, "::after") : null;
  const threadStyle = document.querySelector(".tg-thread") ? getComputedStyle(document.querySelector(".tg-thread")) : null;
  const scrollEdgeReady = Boolean(
    document.body.getAttribute("data-control-ui-harsh-referee")
    && threadPanelAfter
    && threadPanelAfter.content !== "none"
    && threadStyle
    && threadStyle.overscrollBehaviorY === "contain"
  );
  const microcopySelectors = [
    ".badge",
    ".tg-bubble p",
    ".tg-agent-reply-group",
    ".tg-agent-reply-card",
    ".tile",
    ".mini-card",
    ".row-card",
    ".timeline-item",
    ".empty-state",
    ".panel",
    ".card",
    ".tg-room-section",
    ".tg-room-panel__header",
    ".tg-thread-details__grid article",
    ".tg-menu-item__label",
    ".tg-chat-item__topline strong",
  ];
  const microcopyWrapDetails = microcopySelectors.flatMap((selector) =>
    Array.from(document.querySelectorAll(selector))
      .filter(elementVisible)
      .slice(0, 20)
      .map((node) => {
        const style = getComputedStyle(node);
        return {
          selector,
          text: visibleText(node).slice(0, 80),
          overflow_wrap: style.overflowWrap,
          word_break: style.wordBreak,
          white_space: style.whiteSpace,
          ...richRect(node),
        };
      })
  );
	  const microcopyWrapReady = microcopyWrapDetails.length >= 6
	    && microcopyWrapDetails.every((item) => (
	      item.overflow_wrap !== "anywhere"
	      && item.word_break !== "break-word"
	      && item.word_break !== "break-all"
	    ));
	  const logoClipDetails = Array.from(document.querySelectorAll('[data-hepta-agent-logo="true"]'))
	    .filter(elementVisible)
	    .map((node) => {
	      const img = node.querySelector("img");
	      const style = getComputedStyle(node);
	      const rect = richRect(node);
	      const imgRect = img ? richRect(img) : null;
	      return {
	        selector: node.className || node.tagName.toLowerCase(),
	        flex_shrink: style.flexShrink,
	        min_width: style.minWidth,
	        visible: elementVisible(node),
	        image_present: Boolean(img),
	        image_width: imgRect?.width || 0,
	        image_height: imgRect?.height || 0,
	        image_fills_container: Boolean(
	          imgRect
	          && imgRect.width >= Math.min(rect.width, rect.height) * 0.9
	          && imgRect.height >= Math.min(rect.width, rect.height) * 0.9
	        ),
	        ...rect,
	      };
	    });
		  const logoClipReady = logoClipDetails.length >= 1
		    && logoClipDetails.every((item) => (
		      item.visible
	      && item.image_present
	      && item.width >= 32
	      && item.height >= 32
		      && item.image_fills_container
		    ));
	  const avatarPrismaticRimDetails = Array.from(document.querySelectorAll(".tg-chat-item__avatar,.tg-thread-avatar"))
	    .filter(elementVisible)
	    .map((node) => {
	      const style = getComputedStyle(node);
	      const filterText = style.filter || "";
	      const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
	      const img = node.querySelector("img");
	      return {
	        selector: node.className || node.tagName.toLowerCase(),
	        text: visibleText(node),
	        visible: true,
	        image_present: Boolean(img),
	        border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
	        box_shadow: compactShadow(style.boxShadow),
	        filter: filterText && filterText !== "none" ? "present" : "none",
	        filter_sample: filterText.slice(0, 180),
	        avatar_rim_drop_shadow_count: dropShadowCount,
	        avatar_prismatic_rim_ready: dropShadowCount >= 2,
	        ...richRect(node),
	      };
	    });
	  const avatarPrismaticRimLightGlassReady = avatarPrismaticRimDetails.length >= (railVisible ? 4 : 1)
	    && avatarPrismaticRimDetails.every((item) => (
	      item.visible
	      && item.width >= 40
	      && item.height >= 40
	      && item.border_radius >= 16
	      && (item.image_present || item.text.length > 0)
	      && item.box_shadow !== "none"
	      && item.avatar_prismatic_rim_ready === true
	      && item.avatar_rim_drop_shadow_count >= 2
	    ));
		  const readabilityHost = (node) => node.matches("[data-control-ui-rail-search-input]") ? node : (node.closest(".tg-chat-item,.tg-thread-header,.tg-search-shell,.tg-compose-bar,.tg-thread-hepta-controls,.tg-folder-chip,.tg-menu-item,.tg-compose-footer") || node);
		  const readabilityDetail = (node) => {
		      const style = getComputedStyle(node);
		      const textColor = parseCssColor(style.color);
		      const bgColor = effectiveBackground(readabilityHost(node));
		      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
		      return {
		        selector: node.tagName.toLowerCase() + (node.className ? "." + String(node.className).replace(/\\s+/g, ".") : ""),
		        text: visibleText(node).slice(0, 80),
		        color: style.color,
		        effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
		        contrast_ratio: Number(ratio.toFixed(2)),
		        readable: ratio >= 4.5,
		        ...richRect(node),
		      };
		    };
		  const placeholderReadabilityDetail = (node) => {
		    const style = getComputedStyle(node, "::placeholder");
		    const textColor = parseCssColor(style.color);
		    const bgColor = effectiveBackground(readabilityHost(node));
		    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
		    return {
		      selector: node.tagName.toLowerCase() + "[placeholder]::placeholder",
		      text: (node.getAttribute("placeholder") || "").slice(0, 80),
		      color: style.color,
		      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
		      contrast_ratio: Number(ratio.toFixed(2)),
		      readable: ratio >= 4.5,
		      ...richRect(node),
		    };
		  };
		  const chatRowReadabilityDetails = Array.from(document.querySelectorAll(".tg-chat-item :is(.tg-chat-item__topline strong,.tg-chat-item__topline span,.tg-chat-item__body p)"))
		    .filter(elementVisible)
		    .map(readabilityDetail);
		  const threadHeaderReadabilityDetails = Array.from(document.querySelectorAll(".tg-thread-header__main p"))
		    .filter(elementVisible)
		    .map(readabilityDetail);
		  const composeFooterReadabilityDetails = Array.from(document.querySelectorAll(".tg-compose-footer [data-chat-shortcut-hint]"))
		    .filter(elementVisible)
		    .map(readabilityDetail);
		  const messageMetaReadabilityDetails = Array.from(document.querySelectorAll(".tg-message small"))
		    .filter(elementVisible)
		    .map(readabilityDetail);
		  const placeholderReadabilityDetails = Array.from(document.querySelectorAll(".tg-search-shell input[placeholder],.tg-compose-bar textarea[placeholder]"))
		    .filter(elementVisible)
		    .map(placeholderReadabilityDetail);
		  const smallControlReadabilityDetails = Array.from(document.querySelectorAll(".tg-folder-chip,.tg-folder-chip small,.tg-thread-hepta-controls span,.tg-thread-hepta-controls select,.tg-autoscroll-select,.tg-autoscroll-select select,.tg-menu-item__label"))
		    .filter(elementVisible)
		    .map(readabilityDetail);
		  const activeChatReadabilityDetails = chatRowReadabilityDetails.concat(threadHeaderReadabilityDetails, composeFooterReadabilityDetails, messageMetaReadabilityDetails, placeholderReadabilityDetails, smallControlReadabilityDetails);
		  const placeholderReadabilityReady = placeholderReadabilityDetails.length >= 1 && placeholderReadabilityDetails.every((item) => item.readable);
		  const smallControlReadabilityReady = smallControlReadabilityDetails.every((item) => item.readable);
		  const activeChatReadabilityReady = threadHeaderReadabilityDetails.length >= 1
		    && (chatRowReadabilityDetails.length === 0 || chatRowReadabilityDetails.length >= 9)
		    && placeholderReadabilityReady
		    && smallControlReadabilityReady
		    && activeChatReadabilityDetails.every((item) => item.readable);
		  const translucentGlassDetails = [
		    ...primaryShellSurfaceDetails.map((item) => ({ group: "primary-shell", ...item })),
		    ...visibleTopbarActionDetails.map((item) => ({ group: "topbar-action", ...item })),
		    ...visibleRailSearchDetails.map((item) => ({ group: "rail-search", ...item })),
		    ...microSurfaceDetails.map((item) => ({ group: "micro-surface", ...item })),
		    ...(commandPalettePanelDetails.exists ? [{ group: "command-palette-panel", ...commandPalettePanelDetails }] : []),
		    ...(commandPaletteCloseDetails.exists ? [{ group: "command-palette-close", ...commandPaletteCloseDetails }] : []),
		    ...(commandPaletteInputDetails.exists ? [{ group: "command-palette-input", ...commandPaletteInputDetails }] : []),
		    ...commandPaletteItemDetails.map((item) => ({ group: "command-palette-item", ...item })),
		    ...composerPopoverPanelDetails.map((item) => ({ group: "composer-popover-panel", ...item })),
		    ...composerPopoverSearchDetails.map((item) => ({ group: "composer-popover-search", ...item })),
		    ...composerPopoverItemDetails.map((item) => ({ group: "composer-popover-item", ...item })),
		  ].filter((item) => item.visible === true);
			  const translucentShellLightGlassReady = translucentGlassDetails.length >= 18
			    && translucentGlassDetails.every((item) => (
			      item.translucent_ready === true
			      && item.background_alpha >= 0.35
			      && item.background_alpha <= 0.88
			      && (item.backdrop_filter || "").includes("blur(")
			      && item.box_shadow !== "none"
			    ));
			  const bodyStyle = getComputedStyle(document.body);
			  const bodyBeforeStyle = getComputedStyle(document.body, "::before");
			  const bodyBackgroundImage = bodyStyle.backgroundImage || "";
			  const bodyBeforeBackgroundImage = bodyBeforeStyle.backgroundImage || "";
			  const bodyBeforeOpacity = Number.parseFloat(bodyBeforeStyle.opacity || "0");
			  const bodyBackgroundLayerCount = bodyBackgroundImage.split("gradient(").length - 1;
			  const bodyBackgroundRepeatingLayerCount = (bodyBackgroundImage.match(/repeating-linear-gradient/g) || []).length;
			  const bodyBackgroundAngles = Array.from(bodyBackgroundImage.matchAll(/(?:repeating-)?linear-gradient\\(([-\\d.]+)deg/g))
			    .map((match) => Number(match[1]))
			    .filter(Number.isFinite);
			  const bodyBackgroundAngleCount = new Set(bodyBackgroundAngles.map((angle) => Math.round(angle))).size;
			  const refractiveDepthDetails = {
			    body_background_image: bodyBackgroundImage && bodyBackgroundImage !== "none" ? "present" : "none",
			    body_background_translucent_layer: bodyBackgroundImage.includes("rgba("),
			    body_background_layer_count: bodyBackgroundLayerCount,
			    body_background_repeating_layer_count: bodyBackgroundRepeatingLayerCount,
			    body_background_angles: bodyBackgroundAngles.map((angle) => Math.round(angle)),
			    body_background_angle_count: bodyBackgroundAngleCount,
			    before_background_image: bodyBeforeBackgroundImage && bodyBeforeBackgroundImage !== "none" ? "present" : "none",
			    before_opacity: Number((Number.isFinite(bodyBeforeOpacity) ? bodyBeforeOpacity : 0).toFixed(2)),
			    primary_shell_gradient_count: primaryShellSurfaceDetails.filter((item) => item.background_image === "present").length,
			    primary_shell_low_alpha_count: primaryShellSurfaceDetails.filter((item) => item.background_alpha >= 0.38 && item.background_alpha <= 0.58).length,
			    primary_shell_clear_alpha_count: primaryShellSurfaceDetails.filter((item) => item.background_alpha >= 0.38 && item.background_alpha <= 0.49).length,
			  };
			  const primaryShellAlphaValues = primaryShellSurfaceDetails.map((item) => item.background_alpha).filter(Number.isFinite);
			  const primaryShellAlphaMax = primaryShellAlphaValues.length > 0 ? Math.max(...primaryShellAlphaValues) : 1;
			  const primaryShellAlphaMin = primaryShellAlphaValues.length > 0 ? Math.min(...primaryShellAlphaValues) : 1;
			  const primaryShellAlphaAverage = primaryShellAlphaValues.length > 0
			    ? primaryShellAlphaValues.reduce((sum, alpha) => sum + alpha, 0) / primaryShellAlphaValues.length
			    : 1;
			  const primaryShellAlphaBelow045Count = primaryShellAlphaValues.filter((alpha) => alpha < 0.45).length;
			  const substrateCausticFieldDetails = {
			    body_background_image: refractiveDepthDetails.body_background_image,
			    body_background_translucent_layer: refractiveDepthDetails.body_background_translucent_layer,
			    body_background_layer_count: bodyBackgroundLayerCount,
			    body_background_repeating_layer_count: bodyBackgroundRepeatingLayerCount,
			    body_background_angle_count: bodyBackgroundAngleCount,
			    body_background_angles: bodyBackgroundAngles.map((angle) => Math.round(angle)),
			    before_background_image: refractiveDepthDetails.before_background_image,
			    before_opacity: refractiveDepthDetails.before_opacity,
			  };
			  const refractiveDepthLightGlassReady = refractiveDepthDetails.body_background_image === "present"
			    && refractiveDepthDetails.before_background_image === "present"
			    && refractiveDepthDetails.before_opacity >= 0.12
			    && refractiveDepthDetails.primary_shell_gradient_count >= 3
			    && refractiveDepthDetails.primary_shell_low_alpha_count >= 3;
			  const opticalClarityLightGlassReady = refractiveDepthLightGlassReady
			    && refractiveDepthDetails.body_background_translucent_layer === true
			    && refractiveDepthDetails.body_background_layer_count >= 3
			    && refractiveDepthDetails.before_opacity >= 0.2
			    && refractiveDepthDetails.primary_shell_clear_alpha_count >= 3;
			  const surfaceClearAlphaDetails = primaryShellSurfaceDetails.map((item) => ({
			    selector: item.selector,
			    background_alpha: item.background_alpha,
			    effective_luminance: item.effective_luminance,
			    backdrop_blur_px: item.backdrop_blur_px,
			    surface_alpha_max: Number(primaryShellAlphaMax.toFixed(2)),
			    surface_alpha_min: Number(primaryShellAlphaMin.toFixed(2)),
			    surface_alpha_average: Number(primaryShellAlphaAverage.toFixed(3)),
			    surface_alpha_below_045_count: primaryShellAlphaBelow045Count,
			    surface_count: primaryShellAlphaValues.length,
			    clear_alpha_ready: item.background_alpha >= 0.38 && item.background_alpha <= 0.49,
			    readable: item.readable,
			    width: item.width,
			    height: item.height,
			  }));
			  const surfaceClearAlphaLightGlassReady = opticalClarityLightGlassReady
			    && primaryShellAlphaValues.length >= 3
			    && primaryShellAlphaMax <= 0.49
			    && primaryShellAlphaAverage <= 0.44
			    && primaryShellAlphaMin <= 0.4
			    && primaryShellAlphaBelow045Count >= Math.max(1, primaryShellAlphaValues.length - 1)
			    && primaryShellSurfaceDetails.every((item) => item.background_alpha >= 0.38 && item.background_alpha <= 0.49 && item.readable === true);
			  const substrateCausticFieldLightGlassReady = opticalClarityLightGlassReady
			    && bodyBackgroundLayerCount >= 4
			    && bodyBackgroundRepeatingLayerCount >= 2
			    && bodyBackgroundAngleCount >= 4
			    && refractiveDepthDetails.body_background_translucent_layer === true
			    && refractiveDepthDetails.before_opacity >= 0.2;
	  const harshRefereeReady = iconButtonReady
	    && iconPrismaticControlLightGlassReady
	    && defaultSubmenusClosedReady
	    && engineeringSessionChipsSuppressedReady
		    && translucentShellLightGlassReady
		    && refractiveDepthLightGlassReady
		    && opticalClarityLightGlassReady
		    && surfaceClearAlphaLightGlassReady
		    && substrateCausticFieldLightGlassReady
		    && chromeRefractiveSkinLightGlassReady
		    && clearWhiteBalanceLightGlassReady
		    && chamferCutEdgeLightGlassReady
			    && specularEdgeLightGlassReady
			    && prismaticDispersionLightGlassReady
			    && causticHighlightLightGlassReady
			    && causticDepthShiftLightGlassReady
				    && opticalThicknessTiersLightGlassReady
				    && facetedReflectionLightGlassReady
				    && beveledRimLightGlassReady
				    && microRefractionLightGlassReady
				    && sparkleGlintLightGlassReady
				    && lensBloomLightGlassReady
				    && spectralFusionLightGlassReady
				    && opticalMagnificationLightGlassReady
				    && biaxialMagnificationLightGlassReady
				    && anisotropicMagnificationLightGlassReady
				    && phaseSeparatedRefractionLightGlassReady
				    && twoAxisPhaseRefractionLightGlassReady
				    && surfacePhaseDriftLightGlassReady
				    && surfaceLensScaleDriftLightGlassReady
				    && layerScaleParallaxLightGlassReady
				    && surfaceSpectralAngleDriftLightGlassReady
				    && surfaceGlintFocalDriftLightGlassReady
				    && composerGlintFocalDecouplingLightGlassReady
				    && topbarActionLightGlassReady
	    && primaryShellLightGlassReady
	    && menuTriggerReady
	    && folderChipTouchReady
	    && folderChipLabelPrismaticEtchLightGlassReady
	    && rowMenuTouchReady
	    && rowMenuAllRowsReady
		    && rowMenuLightGlassReady
		    && commandPaletteReady
		    && commandPaletteTriggerLightGlassReady
			    && commandPaletteCloseLightGlassReady
				    && commandPaletteInputLightGlassReady
				    && commandPaletteInputPlaceholderPrismaticEtchLightGlassReady
					    && commandPaletteInputRowPrismaticSeparatorLightGlassReady
				    && commandPaletteResultsWellLightGlassReady
				    && commandPaletteResultsWellPrismaticRimLightGlassReady
					    && commandPaletteInputIconPrismaticLightGlassReady
					    && commandPaletteItemLightGlassReady
					    && commandPaletteItemPrismaticRimLightGlassReady
				    && commandPaletteKindChipLightGlassReady
			    && commandPaletteItemHoverPrismaticLightGlassReady
			    && commandPaletteItemLabelPrismaticEtchLightGlassReady
		    && controlFormControlReady
		    && chatRowOptionSemanticTouchReady
		    && railChatRowPrismaticSlabLightGlassReady
		    && menuItemIconReady
    && menuSurfaceReady
	    && threadToolsMenuReady
	    && composerToolsMenuReady
	    && composerPopoverReady
	    && composerPopoverItemLabelPrismaticEtchLightGlassReady
	    && composerPopoverSearchLightGlassReady
	    && composerPopoverSearchPlaceholderPrismaticEtchLightGlassReady
	    && railSearchLightGlassReady
	    && railSearchPlaceholderPrismaticEtchLightGlassReady
	    && railPrismaticFilterLightGlassReady
	    && microSurfaceLightGlassReady
	    && microPrismaticBadgeLightGlassReady
	    && microBadgeLabelPrismaticEtchLightGlassReady
	    && messageMetadataPrismaticLightGlassReady
	    && threadSubtitlePrismaticLightGlassReady
	    && composerShortcutHintPrismaticLightGlassReady
		    && railMetadataChipPrismaticLightGlassReady
		    && railStatusCountPrismaticLightGlassReady
		    && railPreviewPrismaticEtchLightGlassReady
		    && railChatTitlePrismaticEtchLightGlassReady
		    && messageBodyPrismaticEtchLightGlassReady
		    && messageSpeakerPrismaticChipLightGlassReady
		    && composerPlaceholderPrismaticEtchLightGlassReady
		    && messageRoutingBadgeLightGlassReady
	    && threadIntroBadgeLightGlassReady
	    && statusTrustStripLightGlassReady
		    && navIconReady
		    && scrollEdgeReady
		    && microcopyWrapReady
		    && logoClipReady
		    && avatarPrismaticRimLightGlassReady
		    && activeChatReadabilityReady
		    && visibleTextIntegrityReady;
	  if (!harshRefereeReady) {
	    errors.push("control_ui_harsh_2026_referee_not_ready");
	  }
	  if (!iconPrismaticControlLightGlassReady) {
	    errors.push("icon_prismatic_control_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteReady) {
	    errors.push("command_palette_touch_guard_not_ready");
	  }
	  if (!commandPaletteTriggerLightGlassReady) {
	    errors.push("command_palette_trigger_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteCloseLightGlassReady) {
	    errors.push("command_palette_close_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteClosePrismaticIconLightGlassReady) {
	    errors.push("command_palette_close_prismatic_icon_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteSurfaceLightGlassReady) {
	    errors.push("command_palette_surface_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteSurfacePrismaticPerimeterLightGlassReady) {
	    errors.push("command_palette_surface_prismatic_perimeter_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteBackdropCausticVeilLightGlassReady) {
	    errors.push("command_palette_backdrop_caustic_veil_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteInputLightGlassReady) {
	    errors.push("command_palette_input_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteInputTextPrismaticEtchLightGlassReady) {
	    errors.push("command_palette_input_text_prismatic_etch_light_glass_guard_not_ready");
	  }
	  if (!commandPaletteInputPlaceholderPrismaticEtchLightGlassReady) {
	    errors.push("command_palette_input_placeholder_prismatic_etch_light_glass_guard_not_ready");
	  }
		  if (!commandPaletteInputRowPrismaticSeparatorLightGlassReady) {
		    errors.push("command_palette_input_row_prismatic_separator_light_glass_guard_not_ready");
		  }
		  if (!commandPaletteResultsWellLightGlassReady) {
		    errors.push("command_palette_results_well_light_glass_guard_not_ready");
		  }
		  if (!commandPaletteResultsWellPrismaticRimLightGlassReady) {
		    errors.push("command_palette_results_well_prismatic_rim_light_glass_guard_not_ready");
		  }
		  if (!commandPaletteInputIconLightGlassReady) {
		    errors.push("command_palette_input_icon_light_glass_guard_not_ready");
	  }
		  if (!commandPaletteInputIconPrismaticLightGlassReady) {
		    errors.push("command_palette_input_icon_prismatic_light_glass_guard_not_ready");
	  }
		  if (!commandPaletteItemLightGlassReady) {
		    errors.push("command_palette_item_light_glass_guard_not_ready");
		  }
		  if (!commandPaletteItemPrismaticRimLightGlassReady) {
		    errors.push("command_palette_item_prismatic_rim_light_glass_guard_not_ready");
		  }
		  if (!commandPaletteKindChipLightGlassReady) {
		    errors.push("command_palette_kind_chip_light_glass_guard_not_ready");
		  }
		  if (!commandPaletteItemHoverPrismaticLightGlassReady) {
		    errors.push("command_palette_item_hover_prismatic_light_glass_guard_not_ready");
		  }
		  if (!commandPaletteItemLabelPrismaticEtchLightGlassReady) {
		    errors.push("command_palette_item_label_prismatic_etch_light_glass_guard_not_ready");
		  }
		  if (!controlFormControlReady) {
		    errors.push("control_form_control_title_touch_guard_not_ready");
		  }
		  if (!chatRowOptionSemanticTouchReady) {
		    errors.push("chat_row_option_semantic_touch_guard_not_ready");
		  }
		  if (!railChatRowPrismaticSlabLightGlassReady) {
		    errors.push("rail_chat_row_prismatic_slab_light_glass_guard_not_ready");
		  }
		  if (!composerPopoverSearchLightGlassReady) {
		    errors.push("composer_popover_search_light_glass_guard_not_ready");
		  }
		  if (!composerPopoverSearchPlaceholderPrismaticEtchLightGlassReady) {
		    errors.push("composer_popover_search_placeholder_prismatic_etch_light_glass_guard_not_ready");
		  }
		  if (!railSearchLightGlassReady) {
		    errors.push("rail_search_light_glass_guard_not_ready");
		  }
		  if (!railSearchPlaceholderPrismaticEtchLightGlassReady) {
		    errors.push("rail_search_placeholder_prismatic_etch_light_glass_guard_not_ready");
		  }
		  if (!railPrismaticFilterLightGlassReady) {
		    errors.push("rail_prismatic_filter_light_glass_guard_not_ready");
		  }
		  if (!folderChipLabelPrismaticEtchLightGlassReady) {
		    errors.push("folder_chip_label_prismatic_etch_light_glass_guard_not_ready");
		  }
		  if (!rowMenuAllRowsReady) {
		    errors.push("row_menu_all_rows_guard_not_ready");
		  }
		  if (!threadToolsMenuReady) {
		    errors.push("thread_tools_menu_light_glass_guard_not_ready");
		  }
		  if (!composerToolsMenuReady) {
		    errors.push("composer_tools_menu_light_glass_guard_not_ready");
		  }
		  if (!composerPopoverReady) {
		    errors.push("composer_popover_light_glass_guard_not_ready");
		  }
		  if (!composerPopoverItemLabelPrismaticEtchLightGlassReady) {
		    errors.push("composer_popover_item_label_prismatic_etch_light_glass_guard_not_ready");
		  }
		  if (!composerPopoverHeaderPrismaticEtchLightGlassReady) {
		    errors.push("composer_popover_header_prismatic_etch_light_glass_guard_not_ready");
		  }
		  if (!microSurfaceLightGlassReady) {
		    errors.push("micro_surface_light_glass_guard_not_ready");
		  }
		  if (!microPrismaticBadgeLightGlassReady) {
		    errors.push("micro_prismatic_badge_light_glass_guard_not_ready");
		  }
		  if (!microBadgeLabelPrismaticEtchLightGlassReady) {
		    errors.push("micro_badge_label_prismatic_etch_light_glass_guard_not_ready");
		  }
		  if (!messageMetadataPrismaticLightGlassReady) {
		    errors.push("message_metadata_prismatic_light_glass_guard_not_ready");
		  }
		  if (!threadSubtitlePrismaticLightGlassReady) {
		    errors.push("thread_subtitle_prismatic_light_glass_guard_not_ready");
		  }
		  if (!composerShortcutHintPrismaticLightGlassReady) {
		    errors.push("composer_shortcut_hint_prismatic_light_glass_guard_not_ready");
		  }
		  if (!railMetadataChipPrismaticLightGlassReady) {
		    errors.push("rail_metadata_chip_prismatic_light_glass_guard_not_ready");
		  }
			  if (!railStatusCountPrismaticLightGlassReady) {
			    errors.push("rail_status_count_prismatic_light_glass_guard_not_ready");
			  }
			  if (!railPreviewPrismaticEtchLightGlassReady) {
			    errors.push("rail_preview_prismatic_etch_light_glass_guard_not_ready");
			  }
			  if (!railChatTitlePrismaticEtchLightGlassReady) {
			    errors.push("rail_chat_title_prismatic_etch_light_glass_guard_not_ready");
			  }
			  if (!messageBodyPrismaticEtchLightGlassReady) {
			    errors.push("message_body_prismatic_etch_light_glass_guard_not_ready");
			  }
			  if (!messageSpeakerPrismaticChipLightGlassReady) {
			    errors.push("message_speaker_prismatic_chip_light_glass_guard_not_ready");
			  }
			  if (!composerPlaceholderPrismaticEtchLightGlassReady) {
			    errors.push("composer_placeholder_prismatic_etch_light_glass_guard_not_ready");
			  }
			  if (!headerTitlePrismaticEtchLightGlassReady) {
			    errors.push("header_title_prismatic_etch_light_glass_guard_not_ready");
			  }
		  if (!messageRoutingBadgeLightGlassReady) {
		    errors.push("message_routing_badge_light_glass_guard_not_ready");
		  }
		  if (!threadIntroBadgeLightGlassReady) {
		    errors.push("thread_intro_badge_light_glass_guard_not_ready");
		  }
		  if (!statusTrustStripLightGlassReady) {
		    errors.push("status_trust_strip_light_glass_guard_not_ready");
		  }
		  if (!avatarPrismaticRimLightGlassReady) {
		    errors.push("avatar_prismatic_rim_light_glass_guard_not_ready");
		  }
		  if (!topbarActionLightGlassReady) {
		    errors.push("topbar_action_light_glass_guard_not_ready");
		  }
	  if (!primaryShellLightGlassReady) {
	    errors.push("primary_shell_light_glass_guard_not_ready");
	  }
	  if (!translucentShellLightGlassReady) {
	    errors.push("translucent_shell_light_glass_guard_not_ready");
	  }
	  if (!refractiveDepthLightGlassReady) {
	    errors.push("refractive_depth_light_glass_guard_not_ready");
	  }
		  if (!opticalClarityLightGlassReady) {
		    errors.push("optical_clarity_light_glass_guard_not_ready");
		  }
			  if (!specularEdgeLightGlassReady) {
			    errors.push("specular_edge_light_glass_guard_not_ready");
			  }
			  if (!prismaticDispersionLightGlassReady) {
			    errors.push("prismatic_dispersion_light_glass_guard_not_ready");
			  }
				  if (!causticHighlightLightGlassReady) {
				    errors.push("caustic_highlight_light_glass_guard_not_ready");
				  }
				  if (!causticDepthShiftLightGlassReady) {
				    errors.push("caustic_depth_shift_light_glass_guard_not_ready");
				  }
				  if (!opticalThicknessTiersLightGlassReady) {
				    errors.push("optical_thickness_tiers_light_glass_guard_not_ready");
				  }
				  if (!facetedReflectionLightGlassReady) {
				    errors.push("faceted_reflection_light_glass_guard_not_ready");
				  }
					  if (!beveledRimLightGlassReady) {
					    errors.push("beveled_rim_light_glass_guard_not_ready");
					  }
					  if (!microRefractionLightGlassReady) {
					    errors.push("micro_refraction_light_glass_guard_not_ready");
					  }
					  if (!sparkleGlintLightGlassReady) {
					    errors.push("sparkle_glint_light_glass_guard_not_ready");
					  }
					  if (!lensBloomLightGlassReady) {
					    errors.push("lens_bloom_light_glass_guard_not_ready");
					  }
					  if (!spectralFusionLightGlassReady) {
					    errors.push("spectral_fusion_light_glass_guard_not_ready");
					  }
					  if (!opticalMagnificationLightGlassReady) {
					    errors.push("optical_magnification_light_glass_guard_not_ready");
					  }
					  if (!biaxialMagnificationLightGlassReady) {
					    errors.push("biaxial_magnification_light_glass_guard_not_ready");
					  }
					  if (!anisotropicMagnificationLightGlassReady) {
					    errors.push("anisotropic_magnification_light_glass_guard_not_ready");
					  }
					  if (!phaseSeparatedRefractionLightGlassReady) {
					    errors.push("phase_separated_refraction_light_glass_guard_not_ready");
					  }
					  if (!twoAxisPhaseRefractionLightGlassReady) {
					    errors.push("two_axis_phase_refraction_light_glass_guard_not_ready");
					  }
					  if (!surfacePhaseDriftLightGlassReady) {
					    errors.push("surface_phase_drift_light_glass_guard_not_ready");
					  }
					  if (!surfaceLensScaleDriftLightGlassReady) {
					    errors.push("surface_lens_scale_drift_light_glass_guard_not_ready");
					  }
					  if (!layerScaleParallaxLightGlassReady) {
					    errors.push("layer_scale_parallax_light_glass_guard_not_ready");
					  }
					  if (!surfaceSpectralAngleDriftLightGlassReady) {
					    errors.push("surface_spectral_angle_drift_light_glass_guard_not_ready");
					  }
					  if (!surfaceGlintFocalDriftLightGlassReady) {
					    errors.push("surface_glint_focal_drift_light_glass_guard_not_ready");
					  }
					  if (!composerGlintFocalDecouplingLightGlassReady) {
					    errors.push("composer_glint_focal_decoupling_light_glass_guard_not_ready");
					  }
					  if (!composerSpectralAngleDecouplingLightGlassReady) {
					    errors.push("composer_spectral_angle_decoupling_light_glass_guard_not_ready");
					  }
					  if (!composerPhaseDecouplingLightGlassReady) {
					    errors.push("composer_phase_decoupling_light_glass_guard_not_ready");
					  }
					  if (!composerLayerScaleDecouplingLightGlassReady) {
					    errors.push("composer_layer_scale_decoupling_light_glass_guard_not_ready");
					  }
					  if (!chromeBarTranslucencyLightGlassReady) {
					    errors.push("chrome_bar_translucency_light_glass_guard_not_ready");
					  }
					  if (!chromeRefractiveSkinLightGlassReady) {
					    errors.push("chrome_refractive_skin_light_glass_guard_not_ready");
					  }
					  if (!clearWhiteBalanceLightGlassReady) {
					    errors.push("clear_white_balance_light_glass_guard_not_ready");
					  }
					  if (!chamferCutEdgeLightGlassReady) {
					    errors.push("chamfer_cut_edge_light_glass_guard_not_ready");
					  }
					  if (!prismaticCutEdgeLightGlassReady) {
					    errors.push("prismatic_cut_edge_light_glass_guard_not_ready");
					  }
					  if (!panePrismaticPerimeterLightGlassReady) {
					    errors.push("pane_prismatic_perimeter_light_glass_guard_not_ready");
					  }
					  if (!composerPrismaticControlLightGlassReady) {
					    errors.push("composer_prismatic_control_light_glass_guard_not_ready");
					  }
					  if (!surfaceClearAlphaLightGlassReady) {
					    errors.push("surface_clear_alpha_light_glass_guard_not_ready");
					  }
					  if (!substrateCausticFieldLightGlassReady) {
					    errors.push("substrate_caustic_field_light_glass_guard_not_ready");
					  }
		  if (!microcopyWrapReady) {
	    errors.push("microcopy_word_split_guard_not_ready");
	  }
		  if (!logoClipReady) {
		    errors.push("logo_clip_guard_not_ready");
		  }
		  if (!activeChatReadabilityReady) {
		    errors.push("active_chat_readability_guard_not_ready");
		  }
	  if (!visibleTextIntegrityReady) {
	    errors.push("visible_text_integrity_guard_not_ready");
	  }
	  if (!defaultSubmenusClosedReady) {
	    errors.push("default_submenus_closed_guard_not_ready");
	  }
	  if (!singleSubmenuAuditReady) {
	    errors.push("single_submenu_audit_guard_not_ready");
	  }
	  if (!engineeringSessionChipsSuppressedReady) {
	    errors.push("engineering_session_chips_suppressed_guard_not_ready");
	  }
	  return {
	    errors,
	    title: document.title,
	    marker,
    viewport: { width: window.innerWidth, height: window.innerHeight },
    document_scroll_width: document.documentElement.scrollWidth,
    body_scroll_width: document.body.scrollWidth,
    horizontal_overflow_free: htmlOverflow <= 1 && bodyOverflow <= 1,
    default_submenus_closed_ready: defaultSubmenusClosedReady,
    default_submenus_closed_details: defaultSubmenuDetails,
    single_submenu_audit_ready: singleSubmenuAuditReady,
    single_submenu_audit_target_count: singleSubmenuAuditDetails.length,
    single_submenu_audit_details: singleSubmenuAuditDetails,
    engineering_session_chips_suppressed_ready: engineeringSessionChipsSuppressedReady,
    engineering_session_chip_details: engineeringSessionChipDetails,
    preferred_touch_target_ready: preferredTouchTargetReady,
    control_glass_action_ready: controlGlassActionReady,
    harsh_referee_ready: harshRefereeReady,
    rail_visible: railVisible,
    rail_action_icon_ready: railActionIconReady,
    icon_button_ready: iconButtonReady,
    icon_button_details: iconButtonDetails,
    icon_prismatic_control_light_glass_ready: iconPrismaticControlLightGlassReady,
    icon_prismatic_control_details: iconPrismaticControlDetails,
    topbar_action_light_glass_ready: topbarActionLightGlassReady,
    topbar_action_details: visibleTopbarActionDetails,
    chrome_bar_translucency_light_glass_ready: chromeBarTranslucencyLightGlassReady,
    chrome_bar_translucency_details: chromeBarTranslucencyDetails,
    chrome_refractive_skin_light_glass_ready: chromeRefractiveSkinLightGlassReady,
    chrome_refractive_skin_details: chromeBarTranslucencyDetails,
    clear_white_balance_light_glass_ready: clearWhiteBalanceLightGlassReady,
    clear_white_balance_details: clearWhiteBalanceDetails,
    chamfer_cut_edge_light_glass_ready: chamferCutEdgeLightGlassReady,
    chamfer_cut_edge_details: chamferCutEdgeSurfaceDetails,
    prismatic_cut_edge_light_glass_ready: prismaticCutEdgeLightGlassReady,
    prismatic_cut_edge_details: chamferCutEdgeSurfaceDetails,
    pane_prismatic_perimeter_light_glass_ready: panePrismaticPerimeterLightGlassReady,
    pane_prismatic_perimeter_details: panePrismaticPerimeterDetails,
    composer_prismatic_control_light_glass_ready: composerPrismaticControlLightGlassReady,
    composer_prismatic_control_details: composerPrismaticControlDetails,
    primary_shell_light_glass_ready: primaryShellLightGlassReady,
    primary_shell_surface_details: primaryShellSurfaceDetails,
    translucent_shell_light_glass_ready: translucentShellLightGlassReady,
    translucent_glass_details: translucentGlassDetails,
	    refractive_depth_light_glass_ready: refractiveDepthLightGlassReady,
	    optical_clarity_light_glass_ready: opticalClarityLightGlassReady,
	    surface_clear_alpha_light_glass_ready: surfaceClearAlphaLightGlassReady,
	    substrate_caustic_field_light_glass_ready: substrateCausticFieldLightGlassReady,
		    specular_edge_light_glass_ready: specularEdgeLightGlassReady,
		    prismatic_dispersion_light_glass_ready: prismaticDispersionLightGlassReady,
		    caustic_highlight_light_glass_ready: causticHighlightLightGlassReady,
		    caustic_depth_shift_light_glass_ready: causticDepthShiftLightGlassReady,
		    caustic_depth_shift_key_count: causticDepthShiftKeyCount,
		    optical_thickness_tiers_light_glass_ready: opticalThicknessTiersLightGlassReady,
		    optical_thickness_blur_tier_count: opticalThicknessBlurTierCount,
		    optical_thickness_alpha_tier_count: opticalThicknessAlphaTierCount,
		    faceted_reflection_light_glass_ready: facetedReflectionLightGlassReady,
		    beveled_rim_light_glass_ready: beveledRimLightGlassReady,
		    refractive_depth_details: refractiveDepthDetails,
		    substrate_caustic_field_details: substrateCausticFieldDetails,
		    specular_edge_details: specularEdgeDetails,
		    prismatic_dispersion_details: prismaticDispersionDetails,
		    caustic_highlight_details: causticHighlightDetails,
		    caustic_depth_shift_details: causticDepthShiftDetails,
		    optical_thickness_tier_details: opticalThicknessTierDetails,
		    faceted_reflection_details: facetedReflectionDetails,
		    beveled_rim_details: beveledRimDetails,
		    surface_clear_alpha_details: surfaceClearAlphaDetails,
		    micro_refraction_light_glass_ready: microRefractionLightGlassReady,
		    micro_refraction_details: microRefractionDetails,
		    sparkle_glint_light_glass_ready: sparkleGlintLightGlassReady,
		    sparkle_glint_details: sparkleGlintDetails,
		    lens_bloom_light_glass_ready: lensBloomLightGlassReady,
		    lens_bloom_details: lensBloomDetails,
		    spectral_fusion_light_glass_ready: spectralFusionLightGlassReady,
		    spectral_fusion_details: spectralFusionDetails,
		    optical_magnification_light_glass_ready: opticalMagnificationLightGlassReady,
		    optical_magnification_details: opticalMagnificationDetails,
		    biaxial_magnification_light_glass_ready: biaxialMagnificationLightGlassReady,
		    biaxial_magnification_details: biaxialMagnificationDetails,
		    anisotropic_magnification_light_glass_ready: anisotropicMagnificationLightGlassReady,
		    anisotropic_magnification_details: anisotropicMagnificationDetails,
		    phase_separated_refraction_light_glass_ready: phaseSeparatedRefractionLightGlassReady,
		    phase_separated_refraction_details: phaseSeparatedRefractionDetails,
		    two_axis_phase_refraction_light_glass_ready: twoAxisPhaseRefractionLightGlassReady,
		    two_axis_phase_refraction_details: twoAxisPhaseRefractionDetails,
		    surface_phase_drift_light_glass_ready: surfacePhaseDriftLightGlassReady,
		    surface_phase_drift_position_count: surfacePhaseDriftPositionCount,
		    surface_phase_drift_details: surfacePhaseDriftDetails,
		    surface_lens_scale_drift_light_glass_ready: surfaceLensScaleDriftLightGlassReady,
		    surface_lens_scale_drift_size_count: surfaceLensScaleDriftSizeCount,
		    surface_lens_scale_drift_details: surfaceLensScaleDriftDetails,
		    layer_scale_parallax_light_glass_ready: layerScaleParallaxLightGlassReady,
		    layer_scale_parallax_details: layerScaleParallaxDetails,
		    surface_spectral_angle_drift_light_glass_ready: surfaceSpectralAngleDriftLightGlassReady,
		    surface_spectral_angle_drift_signature_count: surfaceSpectralAngleDriftSignatureCount,
		    surface_spectral_angle_drift_details: surfaceSpectralAngleDriftDetails,
		    surface_glint_focal_drift_light_glass_ready: surfaceGlintFocalDriftLightGlassReady,
		    surface_glint_focal_drift_signature_count: surfaceGlintFocalDriftSignatureCount,
		    surface_glint_focal_drift_details: surfaceGlintFocalDriftDetails,
		    composer_glint_focal_decoupling_light_glass_ready: composerGlintFocalDecouplingLightGlassReady,
		    composer_glint_focal_decoupling_details: composerGlintFocalDecouplingDetails,
		    composer_spectral_angle_decoupling_light_glass_ready: composerSpectralAngleDecouplingLightGlassReady,
		    composer_spectral_angle_decoupling_details: composerSpectralAngleDecouplingDetails,
		    composer_phase_decoupling_light_glass_ready: composerPhaseDecouplingLightGlassReady,
		    composer_phase_decoupling_details: composerPhaseDecouplingDetails,
		    composer_layer_scale_decoupling_light_glass_ready: composerLayerScaleDecouplingLightGlassReady,
		    composer_layer_scale_decoupling_details: composerLayerScaleDecouplingDetails,
	      menu_trigger_ready: menuTriggerReady,
    menu_trigger_details: menuTriggerDetails,
	    folder_chip_touch_ready: folderChipTouchReady,
	    folder_chip_label_prismatic_etch_light_glass_ready: folderChipLabelPrismaticEtchLightGlassReady,
	    folder_chip_details: folderChipDetails,
	    row_menu_touch_ready: rowMenuTouchReady,
	    row_menu_all_rows_ready: rowMenuAllRowsReady,
	    row_menu_light_glass_ready: rowMenuLightGlassReady,
	    row_menu_toggle_details: railVisible ? rowMenuToggleDetails : [],
    row_menu_panel_details: railVisible ? visibleRowMenuPanelDetails : [],
    row_menu_visible_item_count: visibleRowMenuItemDetails.length,
    row_menu_item_details: railVisible ? visibleRowMenuItemDetails : [],
	    command_palette_ready: commandPaletteReady,
	    command_palette_surface_light_glass_ready: commandPaletteSurfaceLightGlassReady,
	    command_palette_surface_prismatic_perimeter_light_glass_ready: commandPaletteSurfacePrismaticPerimeterLightGlassReady,
	    command_palette_backdrop_caustic_veil_light_glass_ready: commandPaletteBackdropCausticVeilLightGlassReady,
	    command_palette_trigger_light_glass_ready: commandPaletteTriggerLightGlassReady,
	    command_palette_close_light_glass_ready: commandPaletteCloseLightGlassReady,
	    command_palette_close_prismatic_icon_light_glass_ready: commandPaletteClosePrismaticIconLightGlassReady,
			    command_palette_input_light_glass_ready: commandPaletteInputLightGlassReady,
			    command_palette_input_text_prismatic_etch_light_glass_ready: commandPaletteInputTextPrismaticEtchLightGlassReady,
			    command_palette_input_placeholder_prismatic_etch_light_glass_ready: commandPaletteInputPlaceholderPrismaticEtchLightGlassReady,
			    command_palette_input_row_prismatic_separator_light_glass_ready: commandPaletteInputRowPrismaticSeparatorLightGlassReady,
			    command_palette_results_well_light_glass_ready: commandPaletteResultsWellLightGlassReady,
			    command_palette_results_well_prismatic_rim_light_glass_ready: commandPaletteResultsWellPrismaticRimLightGlassReady,
				    command_palette_input_icon_light_glass_ready: commandPaletteInputIconLightGlassReady,
				    command_palette_input_icon_prismatic_light_glass_ready: commandPaletteInputIconPrismaticLightGlassReady,
			    command_palette_item_light_glass_ready: commandPaletteItemLightGlassReady,
			    command_palette_item_prismatic_rim_light_glass_ready: commandPaletteItemPrismaticRimLightGlassReady,
		    command_palette_kind_chip_light_glass_ready: commandPaletteKindChipLightGlassReady,
		    command_palette_item_hover_prismatic_light_glass_ready: commandPaletteItemHoverPrismaticLightGlassReady,
		    command_palette_item_label_prismatic_etch_light_glass_ready: commandPaletteItemLabelPrismaticEtchLightGlassReady,
	    command_palette_panel_details: commandPalettePanelDetails,
	    command_palette_backdrop_details: commandPaletteBackdropDetails,
	    command_palette_close_details: commandPaletteCloseDetails,
	    command_palette_trigger_details: visibleCommandPaletteTriggerDetails,
		    command_palette_input_details: commandPaletteInputDetails,
		    command_palette_input_row_details: commandPaletteInputRowDetails,
		    command_palette_results_well_details: commandPaletteResultsWellDetails,
		    command_palette_input_icon_details: commandPaletteInputIconDetails,
		    command_palette_item_details: commandPaletteItemDetails.slice(0, 2),
		    control_form_control_title_touch_ready: controlFormControlReady,
		    control_form_control_details: controlFormControlDetails,
		    chat_row_option_semantic_touch_ready: chatRowOptionSemanticTouchReady,
		    chat_row_option_details: chatRowOptionDetails,
		    rail_chat_row_prismatic_slab_light_glass_ready: railChatRowPrismaticSlabLightGlassReady,
		    rail_chat_row_prismatic_slab_details: chatRowOptionDetails.map((item) => ({
		      key: item.key,
		      active: item.active,
		      visible: item.visible,
		      width: item.width,
		      height: item.height,
		      border_radius: item.border_radius,
		      box_shadow: item.box_shadow,
		      backdrop_filter: item.backdrop_filter,
		      filter: item.filter,
		      filter_sample: item.filter_sample,
		      chat_row_drop_shadow_count: item.chat_row_drop_shadow_count,
		      chat_row_prismatic_slab_ready: item.chat_row_prismatic_slab_ready,
		    })),
	    menu_item_icon_ready: menuItemIconReady,
    menu_item_details: menuItemDetails,
    menu_surface_ready: menuSurfaceReady,
    menu_surface_details: menuSurfaceDetails,
    thread_tools_menu_ready: threadToolsMenuReady,
    thread_tools_trigger_details: threadToolsTriggerDetails,
    thread_tools_panel_details: threadToolsPanelDetails,
    thread_tools_item_details: threadToolsItemDetails,
    composer_tools_menu_ready: composerToolsMenuReady,
    composer_tools_trigger_light_glass_ready: composerToolsTriggerLightGlassReady,
    composer_tools_trigger_details: composerToolsTriggerDetails,
    composer_tools_panel_details: composerToolsPanelDetails,
    composer_tools_item_details: composerToolsItemDetails,
    composer_popover_ready: composerPopoverReady,
    composer_popover_item_label_prismatic_etch_light_glass_ready: composerPopoverItemLabelPrismaticEtchLightGlassReady,
    composer_popover_header_prismatic_etch_light_glass_ready: composerPopoverHeaderPrismaticEtchLightGlassReady,
    composer_popover_header_prismatic_etch_details: composerPopoverHeaderDetails,
    composer_popover_search_light_glass_ready: composerPopoverSearchLightGlassReady,
    composer_popover_search_placeholder_prismatic_etch_light_glass_ready: composerPopoverSearchPlaceholderPrismaticEtchLightGlassReady,
    rail_search_light_glass_ready: railSearchLightGlassReady,
    rail_search_placeholder_prismatic_etch_light_glass_ready: railSearchPlaceholderPrismaticEtchLightGlassReady,
    rail_search_placeholder_prismatic_etch_details: railSearchPlaceholderPrismaticEtchDetails,
    rail_prismatic_filter_light_glass_ready: railPrismaticFilterLightGlassReady,
    rail_prismatic_filter_details: railPrismaticFilterDetails.map((item) => ({
      kind: item.kind,
      key: item.key || item.marker || "",
      text: item.text || item.placeholder || "",
      visible: item.visible,
      width: item.width,
      height: item.height,
      border_radius: item.border_radius,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      filter: item.filter,
      filter_sample: item.filter_sample,
      rail_filter_drop_shadow_count: item.rail_filter_drop_shadow_count,
      rail_prismatic_filter_ready: item.rail_prismatic_filter_ready,
    })),
    rail_search_visible_count: visibleRailSearchDetails.length,
    rail_search_details: railSearchDetails.map((item) => ({
      marker: item.marker,
      visible: item.visible,
      type: item.type,
      placeholder: item.placeholder,
      width: item.width,
      height: item.height,
      border_radius: item.border_radius,
      effective_luminance: item.effective_luminance,
      light_glass_ready: item.light_glass_ready,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      aria_label: item.aria_label,
      title: item.title,
      title_matches_aria_label: item.title_matches_aria_label,
      readable: item.readable,
      contrast_ratio: item.contrast_ratio,
      placeholder_readable: item.placeholder_readable,
      placeholder_contrast_ratio: item.placeholder_contrast_ratio,
      placeholder_text_shadow: item.placeholder_text_shadow,
      rail_search_placeholder_text_shadow_count: item.rail_search_placeholder_text_shadow_count,
      rail_search_placeholder_prismatic_etch_ready: item.rail_search_placeholder_prismatic_etch_ready,
    })),
    composer_popover_toggle_details: composerPopoverToggleDetails.map((item) => ({
      key: item.key,
      visible: item.visible,
      width: item.width,
      height: item.height,
      aria_label: item.aria_label,
      title: item.title,
      title_matches_aria_label: item.title_matches_aria_label,
      aria_haspopup: item.aria_haspopup,
      aria_controls: item.aria_controls,
      svg_icon_present: item.svg_icon_present,
      visible_icon_text_absent: item.visible_icon_text_absent,
    })),
    composer_popover_panel_details: composerPopoverPanelDetails.map((item) => ({
      key: item.key,
      role: item.role,
      aria_label: item.aria_label,
      visible: item.visible,
      search_count: item.search_count,
      item_count: item.item_count,
      width: item.width,
      height: item.height,
      border_radius: item.border_radius,
      effective_luminance: item.effective_luminance,
      light_glass_ready: item.light_glass_ready,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      in_viewport: item.in_viewport,
      top_clipped: item.top_clipped,
      bottom_clipped: item.bottom_clipped,
    })),
    composer_popover_search_details: composerPopoverSearchDetails.map((item) => ({
      key: item.key,
      marker: item.marker,
      placeholder: item.placeholder,
      visible: item.visible,
      height: item.height,
      border_radius: item.border_radius,
      effective_luminance: item.effective_luminance,
      light_glass_ready: item.light_glass_ready,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      aria_label: item.aria_label,
      title: item.title,
      title_matches_aria_label: item.title_matches_aria_label,
      readable: item.readable,
      contrast_ratio: item.contrast_ratio,
      placeholder_readable: item.placeholder_readable,
      placeholder_contrast_ratio: item.placeholder_contrast_ratio,
      placeholder_text_shadow: item.placeholder_text_shadow,
      placeholder_text_shadow_sample: item.placeholder_text_shadow_sample,
      composer_popover_search_placeholder_text_shadow_count: item.composer_popover_search_placeholder_text_shadow_count,
      composer_popover_search_placeholder_prismatic_etch_ready: item.composer_popover_search_placeholder_prismatic_etch_ready,
    })),
    composer_popover_item_details: composerPopoverItemDetails.map((item) => ({
      key: item.key,
      role: item.role,
      visible: item.visible,
      width: item.width,
      height: item.height,
      aria_label: item.aria_label,
      title: item.title,
      title_matches_aria_label: item.title_matches_aria_label,
      label: item.label,
      detail: item.detail,
      icon_svg_present: item.icon_svg_present,
      label_nowrap_ready: item.label_nowrap_ready,
      detail_nowrap_ready: item.detail_nowrap_ready,
      background_alpha: item.background_alpha,
      translucent_ready: item.translucent_ready,
      effective_luminance: item.effective_luminance,
      light_glass_ready: item.light_glass_ready,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      readable: item.readable,
      contrast_ratio: item.contrast_ratio,
      detail_readable: item.detail_readable,
      detail_contrast_ratio: item.detail_contrast_ratio,
      label_text_shadow: item.label_text_shadow,
      label_text_shadow_sample: item.label_text_shadow_sample,
      detail_text_shadow: item.detail_text_shadow,
      detail_text_shadow_sample: item.detail_text_shadow_sample,
      composer_popover_item_label_text_shadow_count: item.composer_popover_item_label_text_shadow_count,
      composer_popover_item_detail_text_shadow_count: item.composer_popover_item_detail_text_shadow_count,
      composer_popover_item_label_prismatic_etch_ready: item.composer_popover_item_label_prismatic_etch_ready,
    })),
    composer_popover_header_prismatic_etch_light_glass_ready: composerPopoverHeaderPrismaticEtchLightGlassReady,
    composer_popover_header_prismatic_etch_details: composerPopoverHeaderDetails.map((item) => ({
      key: item.key,
      label: item.label,
      status: item.status,
      visible: item.visible,
      label_visible: item.label_visible,
      status_visible: item.status_visible,
      label_text_shadow: item.label_text_shadow,
      label_text_shadow_sample: item.label_text_shadow_sample,
      status_text_shadow: item.status_text_shadow,
      status_text_shadow_sample: item.status_text_shadow_sample,
      composer_popover_header_label_text_shadow_count: item.composer_popover_header_label_text_shadow_count,
      composer_popover_header_status_text_shadow_count: item.composer_popover_header_status_text_shadow_count,
      composer_popover_header_prismatic_etch_ready: item.composer_popover_header_prismatic_etch_ready,
      label_readable: item.label_readable,
      status_readable: item.status_readable,
      label_contrast_ratio: item.label_contrast_ratio,
      status_contrast_ratio: item.status_contrast_ratio,
      width: item.width,
      height: item.height,
    })),
    micro_surface_light_glass_ready: microSurfaceLightGlassReady,
    micro_prismatic_badge_light_glass_ready: microPrismaticBadgeLightGlassReady,
    micro_badge_label_prismatic_etch_light_glass_ready: microBadgeLabelPrismaticEtchLightGlassReady,
    message_metadata_prismatic_light_glass_ready: messageMetadataPrismaticLightGlassReady,
    message_metadata_prismatic_details: messageMetadataPrismaticDetails,
    thread_subtitle_prismatic_light_glass_ready: threadSubtitlePrismaticLightGlassReady,
    thread_subtitle_prismatic_details: threadSubtitlePrismaticDetails,
    composer_shortcut_hint_prismatic_light_glass_ready: composerShortcutHintPrismaticLightGlassReady,
    composer_shortcut_hint_expected_visible: composerShortcutHintExpectedVisible,
    composer_shortcut_hint_prismatic_details: composerShortcutHintPrismaticDetails,
    rail_metadata_chip_prismatic_light_glass_ready: railMetadataChipPrismaticLightGlassReady,
    rail_metadata_chip_expected_visible: railMetadataChipExpectedVisible,
    rail_metadata_chip_prismatic_details: railMetadataChipPrismaticDetails,
	    rail_status_count_prismatic_light_glass_ready: railStatusCountPrismaticLightGlassReady,
	    rail_status_count_expected_visible: railStatusCountExpectedVisible,
	      rail_status_count_prismatic_details: railStatusCountPrismaticDetails,
	      rail_preview_prismatic_etch_light_glass_ready: railPreviewPrismaticEtchLightGlassReady,
	      rail_preview_expected_visible: railPreviewExpectedVisible,
	      rail_preview_prismatic_etch_details: railPreviewPrismaticEtchDetails,
	      rail_chat_title_prismatic_etch_light_glass_ready: railChatTitlePrismaticEtchLightGlassReady,
	      rail_chat_title_expected_visible: railChatTitleExpectedVisible,
	      rail_chat_title_prismatic_etch_details: railChatTitlePrismaticEtchDetails,
	      message_body_prismatic_etch_light_glass_ready: messageBodyPrismaticEtchLightGlassReady,
	    message_body_prismatic_etch_details: messageBodyPrismaticEtchDetails,
	    message_speaker_prismatic_chip_light_glass_ready: messageSpeakerPrismaticChipLightGlassReady,
	    message_speaker_prismatic_chip_details: messageSpeakerPrismaticChipDetails,
	    composer_placeholder_prismatic_etch_light_glass_ready: composerPlaceholderPrismaticEtchLightGlassReady,
	    composer_placeholder_prismatic_etch_details: composerPlaceholderPrismaticEtchDetails,
	    header_title_prismatic_etch_light_glass_ready: headerTitlePrismaticEtchLightGlassReady,
    header_title_expected_count: headerTitleExpectedCount,
    header_title_prismatic_etch_details: headerTitlePrismaticEtchDetails,
    micro_surface_details: microSurfaceDetails.map((item) => ({
      key: item.key,
      text: item.text,
      visible: item.visible,
      width: item.width,
      height: item.height,
      border_radius: item.border_radius,
      effective_luminance: item.effective_luminance,
      light_glass_ready: item.light_glass_ready,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      filter: item.filter,
      filter_sample: item.filter_sample,
      micro_prismatic_badge_drop_shadow_count: item.micro_prismatic_badge_drop_shadow_count,
      micro_prismatic_badge_ready: item.micro_prismatic_badge_ready,
      text_shadow: item.text_shadow,
      text_shadow_sample: item.text_shadow_sample,
      micro_badge_label_text_shadow_count: item.micro_badge_label_text_shadow_count,
      micro_badge_label_prismatic_etch_ready: item.micro_badge_label_prismatic_etch_ready,
      readable: item.readable,
      contrast_ratio: item.contrast_ratio,
      label_nowrap_ready: item.label_nowrap_ready,
    })),
    message_routing_badge_light_glass_ready: messageRoutingBadgeLightGlassReady,
    message_routing_badge_details: routingBadgeDetails.map((item) => ({
      key: item.key,
      text: item.text,
      visible: item.visible,
      width: item.width,
      height: item.height,
      border_radius: item.border_radius,
      effective_luminance: item.effective_luminance,
      light_glass_ready: item.light_glass_ready,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      readable: item.readable,
      contrast_ratio: item.contrast_ratio,
      label_nowrap_ready: item.label_nowrap_ready,
    })),
    thread_intro_badge_light_glass_ready: threadIntroBadgeLightGlassReady,
    thread_intro_badge_visible: threadIntroVisible,
    thread_intro_badge_details: threadIntroBadgeDetails.map((item) => ({
      key: item.key,
      text: item.text,
      visible: item.visible,
      width: item.width,
      height: item.height,
      border_radius: item.border_radius,
      effective_luminance: item.effective_luminance,
      light_glass_ready: item.light_glass_ready,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      readable: item.readable,
      contrast_ratio: item.contrast_ratio,
      label_nowrap_ready: item.label_nowrap_ready,
    })),
    status_trust_strip_light_glass_ready: statusTrustStripLightGlassReady,
    status_trust_strip_visible: statusTrustStripVisible,
    status_trust_badge_details: statusTrustBadgeDetails.map((item) => ({
      key: item.key,
      micro_surface_key: item.micro_surface_key,
      text: item.text,
      visible: item.visible,
      width: item.width,
      height: item.height,
      border_radius: item.border_radius,
      effective_luminance: item.effective_luminance,
      light_glass_ready: item.light_glass_ready,
      backdrop_filter: item.backdrop_filter,
      box_shadow: item.box_shadow,
      aria_label: item.aria_label,
      title: item.title,
      title_matches_aria_label: item.title_matches_aria_label,
      readable: item.readable,
      contrast_ratio: item.contrast_ratio,
      label_nowrap_ready: item.label_nowrap_ready,
    })),
    nav_icon_ready: navIconReady,
	    scroll_edge_ready: scrollEdgeReady,
	    microcopy_word_split_guard_ready: microcopyWrapReady,
	    microcopy_wrap_details: microcopyWrapDetails.slice(0, 8),
		    logo_clip_guard_ready: logoClipReady,
		    logo_clip_details: logoClipDetails,
		    avatar_prismatic_rim_light_glass_ready: avatarPrismaticRimLightGlassReady,
		    avatar_prismatic_rim_details: avatarPrismaticRimDetails,
		    active_chat_readability_ready: activeChatReadabilityReady,
			    active_chat_readability_details: activeChatReadabilityDetails.slice(0, 4),
		    placeholder_readability_ready: placeholderReadabilityReady,
			    placeholder_readability_details: placeholderReadabilityDetails.slice(0, 2),
		    small_control_readability_ready: smallControlReadabilityReady,
			    small_control_readability_details: smallControlReadabilityDetails.slice(0, 4),
		    visible_text_integrity_ready: visibleTextIntegrityReady,
		    visible_text_integrity_probe: {
		      expected: visibleTextIntegrityExpected,
		      actual: visibleTextIntegritySample,
		    },
		    message_speaker_prismatic_chip_light_glass_ready: messageSpeakerPrismaticChipLightGlassReady,
		    message_speaker_prismatic_chip_details: messageSpeakerPrismaticChipDetails.map((item) => ({
		      text: item.text,
		      visible: item.visible,
		      width: item.width,
		      height: item.height,
		      border_radius: item.border_radius,
		      background_alpha: item.background_alpha,
		      effective_luminance: item.effective_luminance,
		      backdrop_filter: item.backdrop_filter,
		      box_shadow: item.box_shadow,
		      filter: item.filter,
		      message_speaker_chip_drop_shadow_count: item.message_speaker_chip_drop_shadow_count,
		      message_speaker_prismatic_chip_ready: item.message_speaker_prismatic_chip_ready,
		      readable: item.readable,
		      contrast_ratio: item.contrast_ratio,
		      label_nowrap_ready: item.label_nowrap_ready,
		    })),
		    composer_glass_ready: composerGlassReady,
    send_glass_ready: sendGlassReady,
    selectors: selectors.filter((item) => [
      "[data-agent-chat-send]",
      "[data-chat-composer-input]",
      ".tg-compose-bar",
    ].includes(item.selector)),
    errors,
  };
})()
      `;

      const evaluation = await send("Runtime.evaluate", {
        expression,
        returnByValue: true,
        awaitPromise: false,
      });
      ws.close();
      if (evaluation.exceptionDetails) {
        const message =
          evaluation.exceptionDetails.exception?.description ||
          evaluation.exceptionDetails.text ||
          "runtime_evaluate_exception";
        return {
          name: viewport.name,
          viewport: `${viewport.width}x${viewport.height}`,
          status: "failed",
          navigation_error: navigateResult.errorText || null,
          errors: [String(message).slice(0, 240)],
        };
      }
      const value = evaluation.result?.value || { errors: ["runtime_evaluate_no_value"] };
      if (!Array.isArray(value.errors)) {
        value.errors = [];
      }
      return {
        name: viewport.name,
        viewport: `${viewport.width}x${viewport.height}`,
        status: value.errors.length === 0 ? "ready" : "failed",
        navigation_error: navigateResult.errorText || null,
        ...value,
      };
    } finally {
      if (!chrome.killed) {
        chrome.kill("SIGTERM");
      }
      setTimeout(() => {
        if (!chrome.killed) {
          chrome.kill("SIGKILL");
        }
      }, 1000).unref();
    }
  }

  const results = [];
  for (const viewport of viewports) {
    results.push(await inspectViewport(viewport));
  }

	  const failures = results.flatMap((result) =>
	    result.status === "ready" ? [] : (result.errors || []).map((error) => `${result.name}:${error}`),
	  );

  const report = {
    gate: "control_ui_visual_density_qa",
    status: failures.length === 0 ? "ready" : "failed",
    control_ui_visual_density_qa_ready: failures.length === 0,
    viewport_count: results.length,
    phone320_ready: results.some((result) => result.name === "phone320" && result.status === "ready"),
    default_submenus_closed_ready: results.every((result) => result.default_submenus_closed_ready === true),
    single_submenu_audit_ready: results.every((result) => result.single_submenu_audit_ready === true),
    engineering_session_chips_suppressed_ready: results.every((result) => result.engineering_session_chips_suppressed_ready === true),
    preferred_touch_targets_ready: results.every((result) => result.preferred_touch_target_ready === true),
    control_glass_action_ready: results.every((result) => result.control_glass_action_ready === true),
    harsh_referee_ready: results.every((result) => result.harsh_referee_ready === true),
    rail_action_icon_ready: results.every((result) => result.rail_action_icon_ready === true),
    icon_button_ready: results.every((result) => result.icon_button_ready === true),
    icon_prismatic_control_light_glass_ready: results.every((result) => result.icon_prismatic_control_light_glass_ready === true),
    topbar_action_light_glass_ready: results.every((result) => result.topbar_action_light_glass_ready === true),
    chrome_bar_translucency_light_glass_ready: results.every((result) => result.chrome_bar_translucency_light_glass_ready === true),
    chrome_refractive_skin_light_glass_ready: results.every((result) => result.chrome_refractive_skin_light_glass_ready === true),
    clear_white_balance_light_glass_ready: results.every((result) => result.clear_white_balance_light_glass_ready === true),
    chamfer_cut_edge_light_glass_ready: results.every((result) => result.chamfer_cut_edge_light_glass_ready === true),
    prismatic_cut_edge_light_glass_ready: results.every((result) => result.prismatic_cut_edge_light_glass_ready === true),
    pane_prismatic_perimeter_light_glass_ready: results.every((result) => result.pane_prismatic_perimeter_light_glass_ready === true),
    composer_prismatic_control_light_glass_ready: results.every((result) => result.composer_prismatic_control_light_glass_ready === true),
    primary_shell_light_glass_ready: results.every((result) => result.primary_shell_light_glass_ready === true),
	    translucent_shell_light_glass_ready: results.every((result) => result.translucent_shell_light_glass_ready === true),
	    refractive_depth_light_glass_ready: results.every((result) => result.refractive_depth_light_glass_ready === true),
	    optical_clarity_light_glass_ready: results.every((result) => result.optical_clarity_light_glass_ready === true),
	    surface_clear_alpha_light_glass_ready: results.every((result) => result.surface_clear_alpha_light_glass_ready === true),
	    substrate_caustic_field_light_glass_ready: results.every((result) => result.substrate_caustic_field_light_glass_ready === true),
		    specular_edge_light_glass_ready: results.every((result) => result.specular_edge_light_glass_ready === true),
		    prismatic_dispersion_light_glass_ready: results.every((result) => result.prismatic_dispersion_light_glass_ready === true),
		    caustic_highlight_light_glass_ready: results.every((result) => result.caustic_highlight_light_glass_ready === true),
		    caustic_depth_shift_light_glass_ready: results.every((result) => result.caustic_depth_shift_light_glass_ready === true),
			    optical_thickness_tiers_light_glass_ready: results.every((result) => result.optical_thickness_tiers_light_glass_ready === true),
			    faceted_reflection_light_glass_ready: results.every((result) => result.faceted_reflection_light_glass_ready === true),
			    beveled_rim_light_glass_ready: results.every((result) => result.beveled_rim_light_glass_ready === true),
			    micro_refraction_light_glass_ready: results.every((result) => result.micro_refraction_light_glass_ready === true),
			    sparkle_glint_light_glass_ready: results.every((result) => result.sparkle_glint_light_glass_ready === true),
			    lens_bloom_light_glass_ready: results.every((result) => result.lens_bloom_light_glass_ready === true),
			    spectral_fusion_light_glass_ready: results.every((result) => result.spectral_fusion_light_glass_ready === true),
			    optical_magnification_light_glass_ready: results.every((result) => result.optical_magnification_light_glass_ready === true),
			    biaxial_magnification_light_glass_ready: results.every((result) => result.biaxial_magnification_light_glass_ready === true),
			    anisotropic_magnification_light_glass_ready: results.every((result) => result.anisotropic_magnification_light_glass_ready === true),
			    phase_separated_refraction_light_glass_ready: results.every((result) => result.phase_separated_refraction_light_glass_ready === true),
			    two_axis_phase_refraction_light_glass_ready: results.every((result) => result.two_axis_phase_refraction_light_glass_ready === true),
			    surface_phase_drift_light_glass_ready: results.every((result) => result.surface_phase_drift_light_glass_ready === true),
			    surface_lens_scale_drift_light_glass_ready: results.every((result) => result.surface_lens_scale_drift_light_glass_ready === true),
			    layer_scale_parallax_light_glass_ready: results.every((result) => result.layer_scale_parallax_light_glass_ready === true),
			    surface_spectral_angle_drift_light_glass_ready: results.every((result) => result.surface_spectral_angle_drift_light_glass_ready === true),
			    surface_glint_focal_drift_light_glass_ready: results.every((result) => result.surface_glint_focal_drift_light_glass_ready === true),
			    composer_glint_focal_decoupling_light_glass_ready: results.every((result) => result.composer_glint_focal_decoupling_light_glass_ready === true),
			    composer_spectral_angle_decoupling_light_glass_ready: results.every((result) => result.composer_spectral_angle_decoupling_light_glass_ready === true),
			    composer_phase_decoupling_light_glass_ready: results.every((result) => result.composer_phase_decoupling_light_glass_ready === true),
			    composer_layer_scale_decoupling_light_glass_ready: results.every((result) => result.composer_layer_scale_decoupling_light_glass_ready === true),
			    menu_trigger_ready: results.every((result) => result.menu_trigger_ready === true),
		    folder_chip_touch_ready: results.every((result) => result.folder_chip_touch_ready === true),
		    folder_chip_label_prismatic_etch_light_glass_ready: results.every((result) => result.folder_chip_label_prismatic_etch_light_glass_ready === true),
		    row_menu_touch_ready: results.every((result) => result.row_menu_touch_ready === true),
			    row_menu_all_rows_ready: results.every((result) => result.row_menu_all_rows_ready === true),
		    row_menu_light_glass_ready: results.every((result) => result.row_menu_light_glass_ready === true),
		    command_palette_ready: results.every((result) => result.command_palette_ready === true),
		    command_palette_surface_light_glass_ready: results.every((result) => result.command_palette_surface_light_glass_ready === true),
		    command_palette_surface_prismatic_perimeter_light_glass_ready: results.every((result) => result.command_palette_surface_prismatic_perimeter_light_glass_ready === true),
		    command_palette_backdrop_caustic_veil_light_glass_ready: results.every((result) => result.command_palette_backdrop_caustic_veil_light_glass_ready === true),
		    command_palette_trigger_light_glass_ready: results.every((result) => result.command_palette_trigger_light_glass_ready === true),
		    command_palette_close_light_glass_ready: results.every((result) => result.command_palette_close_light_glass_ready === true),
		    command_palette_close_prismatic_icon_light_glass_ready: results.every((result) => result.command_palette_close_prismatic_icon_light_glass_ready === true),
				    command_palette_input_light_glass_ready: results.every((result) => result.command_palette_input_light_glass_ready === true),
				    command_palette_input_text_prismatic_etch_light_glass_ready: results.every((result) => result.command_palette_input_text_prismatic_etch_light_glass_ready === true),
				    command_palette_input_placeholder_prismatic_etch_light_glass_ready: results.every((result) => result.command_palette_input_placeholder_prismatic_etch_light_glass_ready === true),
				    command_palette_input_row_prismatic_separator_light_glass_ready: results.every((result) => result.command_palette_input_row_prismatic_separator_light_glass_ready === true),
				    command_palette_results_well_light_glass_ready: results.every((result) => result.command_palette_results_well_light_glass_ready === true),
				    command_palette_results_well_prismatic_rim_light_glass_ready: results.every((result) => result.command_palette_results_well_prismatic_rim_light_glass_ready === true),
					    command_palette_input_icon_light_glass_ready: results.every((result) => result.command_palette_input_icon_light_glass_ready === true),
					    command_palette_input_icon_prismatic_light_glass_ready: results.every((result) => result.command_palette_input_icon_prismatic_light_glass_ready === true),
				    command_palette_item_light_glass_ready: results.every((result) => result.command_palette_item_light_glass_ready === true),
				    command_palette_item_prismatic_rim_light_glass_ready: results.every((result) => result.command_palette_item_prismatic_rim_light_glass_ready === true),
			    command_palette_kind_chip_light_glass_ready: results.every((result) => result.command_palette_kind_chip_light_glass_ready === true),
			    command_palette_item_hover_prismatic_light_glass_ready: results.every((result) => result.command_palette_item_hover_prismatic_light_glass_ready === true),
			    command_palette_item_label_prismatic_etch_light_glass_ready: results.every((result) => result.command_palette_item_label_prismatic_etch_light_glass_ready === true),
		    control_form_control_title_touch_ready: results.every((result) => result.control_form_control_title_touch_ready === true),
		    chat_row_option_semantic_touch_ready: results.every((result) => result.chat_row_option_semantic_touch_ready === true),
		    rail_chat_row_prismatic_slab_light_glass_ready: results.every((result) => result.rail_chat_row_prismatic_slab_light_glass_ready === true),
		    menu_item_icon_ready: results.every((result) => result.menu_item_icon_ready === true),
    icon_button_title_match_ready: results.every((result) => (result.icon_button_details || []).every((item) => item.title_matches_aria_label === true)),
    menu_trigger_title_match_ready: results.every((result) => (result.menu_trigger_details || []).every((item) => item.title_matches_aria_label === true)),
    menu_surface_ready: results.every((result) => result.menu_surface_ready === true),
    thread_tools_menu_ready: results.every((result) => result.thread_tools_menu_ready === true),
    composer_tools_menu_ready: results.every((result) => result.composer_tools_menu_ready === true),
    composer_tools_trigger_light_glass_ready: results.every((result) => result.composer_tools_trigger_light_glass_ready === true),
    composer_popover_ready: results.every((result) => result.composer_popover_ready === true),
    composer_popover_item_label_prismatic_etch_light_glass_ready: results.every((result) => result.composer_popover_item_label_prismatic_etch_light_glass_ready === true),
    composer_popover_header_prismatic_etch_light_glass_ready: results.every((result) => result.composer_popover_header_prismatic_etch_light_glass_ready === true),
    composer_popover_search_light_glass_ready: results.every((result) => result.composer_popover_search_light_glass_ready === true),
    composer_popover_search_placeholder_prismatic_etch_light_glass_ready: results.every((result) => result.composer_popover_search_placeholder_prismatic_etch_light_glass_ready === true),
    rail_search_light_glass_ready: results.every((result) => result.rail_search_light_glass_ready === true),
    rail_search_placeholder_prismatic_etch_light_glass_ready: results.every((result) => result.rail_search_placeholder_prismatic_etch_light_glass_ready === true),
    rail_prismatic_filter_light_glass_ready: results.every((result) => result.rail_prismatic_filter_light_glass_ready === true),
    micro_surface_light_glass_ready: results.every((result) => result.micro_surface_light_glass_ready === true),
    micro_prismatic_badge_light_glass_ready: results.every((result) => result.micro_prismatic_badge_light_glass_ready === true),
    micro_badge_label_prismatic_etch_light_glass_ready: results.every((result) => result.micro_badge_label_prismatic_etch_light_glass_ready === true),
    message_metadata_prismatic_light_glass_ready: results.every((result) => result.message_metadata_prismatic_light_glass_ready === true),
    thread_subtitle_prismatic_light_glass_ready: results.every((result) => result.thread_subtitle_prismatic_light_glass_ready === true),
    composer_shortcut_hint_prismatic_light_glass_ready: results.every((result) => result.composer_shortcut_hint_prismatic_light_glass_ready === true),
	    rail_metadata_chip_prismatic_light_glass_ready: results.every((result) => result.rail_metadata_chip_prismatic_light_glass_ready === true),
	    rail_status_count_prismatic_light_glass_ready: results.every((result) => result.rail_status_count_prismatic_light_glass_ready === true),
	    rail_preview_prismatic_etch_light_glass_ready: results.every((result) => result.rail_preview_prismatic_etch_light_glass_ready === true),
	    rail_chat_title_prismatic_etch_light_glass_ready: results.every((result) => result.rail_chat_title_prismatic_etch_light_glass_ready === true),
	    message_body_prismatic_etch_light_glass_ready: results.every((result) => result.message_body_prismatic_etch_light_glass_ready === true),
	    message_speaker_prismatic_chip_light_glass_ready: results.every((result) => result.message_speaker_prismatic_chip_light_glass_ready === true),
	    composer_placeholder_prismatic_etch_light_glass_ready: results.every((result) => result.composer_placeholder_prismatic_etch_light_glass_ready === true),
	    header_title_prismatic_etch_light_glass_ready: results.every((result) => result.header_title_prismatic_etch_light_glass_ready === true),
    message_routing_badge_light_glass_ready: results.every((result) => result.message_routing_badge_light_glass_ready === true),
    thread_intro_badge_light_glass_ready: results.every((result) => result.thread_intro_badge_light_glass_ready === true),
    status_trust_strip_light_glass_ready: results.every((result) => result.status_trust_strip_light_glass_ready === true),
    menu_surface_viewport_guard_ready: results.every((result) => (result.menu_surface_details || []).every((item) =>
      item.in_viewport === true
      && item.vertical_in_viewport === true
      && item.top_clipped === false
      && item.bottom_clipped === false
    )),
    nav_icon_ready: results.every((result) => result.nav_icon_ready === true),
	    scroll_edge_ready: results.every((result) => result.scroll_edge_ready === true),
	    microcopy_word_split_guard_ready: results.every((result) => result.microcopy_word_split_guard_ready === true),
	    logo_clip_guard_ready: results.every((result) => result.logo_clip_guard_ready === true),
	    avatar_prismatic_rim_light_glass_ready: results.every((result) => result.avatar_prismatic_rim_light_glass_ready === true),
	    active_chat_readability_ready: results.every((result) => result.active_chat_readability_ready === true),
	    placeholder_readability_ready: results.every((result) => result.placeholder_readability_ready === true),
	    small_control_readability_ready: results.every((result) => result.small_control_readability_ready === true)
	      && results.reduce((count, result) => count + (result.small_control_readability_details || []).length, 0) >= 8,
	    visible_text_integrity_ready: results.every((result) => result.visible_text_integrity_ready === true),
	    horizontal_overflow_free: results.every((result) => result.horizontal_overflow_free === true),
	    browser_error_page_absent: results.every((result) => !(result.errors || []).includes("browser_error_page_visible")),
    results: results.map((result) => ({
      name: result.name,
      viewport: result.viewport,
      status: result.status,
      errors: result.errors || [],
      default_submenus_closed_ready: result.default_submenus_closed_ready,
      default_submenus_closed_details: result.default_submenus_closed_details,
      single_submenu_audit_ready: result.single_submenu_audit_ready,
      single_submenu_audit_target_count: result.single_submenu_audit_target_count,
      single_submenu_audit_details: result.single_submenu_audit_details,
      engineering_session_chips_suppressed_ready: result.engineering_session_chips_suppressed_ready,
      engineering_session_chip_details: result.engineering_session_chip_details,
      preferred_touch_target_ready: result.preferred_touch_target_ready,
      control_glass_action_ready: result.control_glass_action_ready,
      harsh_referee_ready: result.harsh_referee_ready,
      rail_visible: result.rail_visible,
      rail_action_icon_ready: result.rail_action_icon_ready,
      icon_button_ready: result.icon_button_ready,
      icon_button_details: result.icon_button_details,
      icon_prismatic_control_light_glass_ready: result.icon_prismatic_control_light_glass_ready,
      icon_prismatic_control_details: result.icon_prismatic_control_details,
      topbar_action_light_glass_ready: result.topbar_action_light_glass_ready,
      topbar_action_details: result.topbar_action_details,
      chrome_bar_translucency_light_glass_ready: result.chrome_bar_translucency_light_glass_ready,
      chrome_bar_translucency_details: result.chrome_bar_translucency_details,
      chrome_refractive_skin_light_glass_ready: result.chrome_refractive_skin_light_glass_ready,
      chrome_refractive_skin_details: result.chrome_refractive_skin_details,
      clear_white_balance_light_glass_ready: result.clear_white_balance_light_glass_ready,
      clear_white_balance_details: result.clear_white_balance_details,
      chamfer_cut_edge_light_glass_ready: result.chamfer_cut_edge_light_glass_ready,
      chamfer_cut_edge_details: result.chamfer_cut_edge_details,
      prismatic_cut_edge_light_glass_ready: result.prismatic_cut_edge_light_glass_ready,
      prismatic_cut_edge_details: result.prismatic_cut_edge_details,
      pane_prismatic_perimeter_light_glass_ready: result.pane_prismatic_perimeter_light_glass_ready,
      pane_prismatic_perimeter_details: result.pane_prismatic_perimeter_details,
      composer_prismatic_control_light_glass_ready: result.composer_prismatic_control_light_glass_ready,
      composer_prismatic_control_details: result.composer_prismatic_control_details,
      primary_shell_light_glass_ready: result.primary_shell_light_glass_ready,
      primary_shell_surface_details: result.primary_shell_surface_details,
      translucent_shell_light_glass_ready: result.translucent_shell_light_glass_ready,
      translucent_glass_details: result.translucent_glass_details,
	      refractive_depth_light_glass_ready: result.refractive_depth_light_glass_ready,
	      optical_clarity_light_glass_ready: result.optical_clarity_light_glass_ready,
	      surface_clear_alpha_light_glass_ready: result.surface_clear_alpha_light_glass_ready,
	      substrate_caustic_field_light_glass_ready: result.substrate_caustic_field_light_glass_ready,
		      specular_edge_light_glass_ready: result.specular_edge_light_glass_ready,
		      prismatic_dispersion_light_glass_ready: result.prismatic_dispersion_light_glass_ready,
		      caustic_highlight_light_glass_ready: result.caustic_highlight_light_glass_ready,
		      caustic_depth_shift_light_glass_ready: result.caustic_depth_shift_light_glass_ready,
		      caustic_depth_shift_key_count: result.caustic_depth_shift_key_count,
		      optical_thickness_tiers_light_glass_ready: result.optical_thickness_tiers_light_glass_ready,
		      optical_thickness_blur_tier_count: result.optical_thickness_blur_tier_count,
		      optical_thickness_alpha_tier_count: result.optical_thickness_alpha_tier_count,
		      faceted_reflection_light_glass_ready: result.faceted_reflection_light_glass_ready,
		      beveled_rim_light_glass_ready: result.beveled_rim_light_glass_ready,
		      refractive_depth_details: result.refractive_depth_details,
		      substrate_caustic_field_details: result.substrate_caustic_field_details,
		      specular_edge_details: result.specular_edge_details,
		      prismatic_dispersion_details: result.prismatic_dispersion_details,
		      caustic_highlight_details: result.caustic_highlight_details,
		      caustic_depth_shift_details: result.caustic_depth_shift_details,
			      optical_thickness_tier_details: result.optical_thickness_tier_details,
			      faceted_reflection_details: result.faceted_reflection_details,
			      beveled_rim_details: result.beveled_rim_details,
			      surface_clear_alpha_details: result.surface_clear_alpha_details,
			      micro_refraction_light_glass_ready: result.micro_refraction_light_glass_ready,
			      micro_refraction_details: result.micro_refraction_details,
			      sparkle_glint_light_glass_ready: result.sparkle_glint_light_glass_ready,
			      sparkle_glint_details: result.sparkle_glint_details,
			      lens_bloom_light_glass_ready: result.lens_bloom_light_glass_ready,
			      lens_bloom_details: result.lens_bloom_details,
			      spectral_fusion_light_glass_ready: result.spectral_fusion_light_glass_ready,
			      spectral_fusion_details: result.spectral_fusion_details,
			      optical_magnification_light_glass_ready: result.optical_magnification_light_glass_ready,
			      optical_magnification_details: result.optical_magnification_details,
			      biaxial_magnification_light_glass_ready: result.biaxial_magnification_light_glass_ready,
			      biaxial_magnification_details: result.biaxial_magnification_details,
			      anisotropic_magnification_light_glass_ready: result.anisotropic_magnification_light_glass_ready,
			      anisotropic_magnification_details: result.anisotropic_magnification_details,
			      phase_separated_refraction_light_glass_ready: result.phase_separated_refraction_light_glass_ready,
			      phase_separated_refraction_details: result.phase_separated_refraction_details,
			      two_axis_phase_refraction_light_glass_ready: result.two_axis_phase_refraction_light_glass_ready,
			      two_axis_phase_refraction_details: result.two_axis_phase_refraction_details,
			      surface_phase_drift_light_glass_ready: result.surface_phase_drift_light_glass_ready,
			      surface_phase_drift_position_count: result.surface_phase_drift_position_count,
			      surface_phase_drift_details: result.surface_phase_drift_details,
			      surface_lens_scale_drift_light_glass_ready: result.surface_lens_scale_drift_light_glass_ready,
			      surface_lens_scale_drift_size_count: result.surface_lens_scale_drift_size_count,
			      surface_lens_scale_drift_details: result.surface_lens_scale_drift_details,
			      layer_scale_parallax_light_glass_ready: result.layer_scale_parallax_light_glass_ready,
			      layer_scale_parallax_details: result.layer_scale_parallax_details,
			      surface_spectral_angle_drift_light_glass_ready: result.surface_spectral_angle_drift_light_glass_ready,
			      surface_spectral_angle_drift_signature_count: result.surface_spectral_angle_drift_signature_count,
			      surface_spectral_angle_drift_details: result.surface_spectral_angle_drift_details,
			      surface_glint_focal_drift_light_glass_ready: result.surface_glint_focal_drift_light_glass_ready,
			      surface_glint_focal_drift_signature_count: result.surface_glint_focal_drift_signature_count,
			      surface_glint_focal_drift_details: result.surface_glint_focal_drift_details,
			      composer_glint_focal_decoupling_light_glass_ready: result.composer_glint_focal_decoupling_light_glass_ready,
			      composer_glint_focal_decoupling_details: result.composer_glint_focal_decoupling_details,
			      composer_spectral_angle_decoupling_light_glass_ready: result.composer_spectral_angle_decoupling_light_glass_ready,
			      composer_spectral_angle_decoupling_details: result.composer_spectral_angle_decoupling_details,
			      composer_phase_decoupling_light_glass_ready: result.composer_phase_decoupling_light_glass_ready,
			      composer_phase_decoupling_details: result.composer_phase_decoupling_details,
			      composer_layer_scale_decoupling_light_glass_ready: result.composer_layer_scale_decoupling_light_glass_ready,
			      composer_layer_scale_decoupling_details: result.composer_layer_scale_decoupling_details,
			      menu_trigger_ready: result.menu_trigger_ready,
      menu_trigger_details: result.menu_trigger_details,
      folder_chip_touch_ready: result.folder_chip_touch_ready,
      folder_chip_label_prismatic_etch_light_glass_ready: result.folder_chip_label_prismatic_etch_light_glass_ready,
      folder_chip_details: result.folder_chip_details,
      row_menu_touch_ready: result.row_menu_touch_ready,
      row_menu_all_rows_ready: result.row_menu_all_rows_ready,
      row_menu_light_glass_ready: result.row_menu_light_glass_ready,
      row_menu_toggle_details: result.row_menu_toggle_details,
      row_menu_panel_details: result.row_menu_panel_details,
      row_menu_visible_item_count: result.row_menu_visible_item_count,
      row_menu_item_details: result.row_menu_item_details,
      menu_item_icon_ready: result.menu_item_icon_ready,
      menu_item_details: result.menu_item_details,
      menu_surface_ready: result.menu_surface_ready,
      menu_surface_details: result.menu_surface_details,
      command_palette_ready: result.command_palette_ready,
      command_palette_surface_light_glass_ready: result.command_palette_surface_light_glass_ready,
      command_palette_surface_prismatic_perimeter_light_glass_ready: result.command_palette_surface_prismatic_perimeter_light_glass_ready,
      command_palette_backdrop_caustic_veil_light_glass_ready: result.command_palette_backdrop_caustic_veil_light_glass_ready,
      command_palette_panel_details: result.command_palette_panel_details,
      command_palette_backdrop_details: result.command_palette_backdrop_details,
      command_palette_trigger_light_glass_ready: result.command_palette_trigger_light_glass_ready,
      command_palette_close_light_glass_ready: result.command_palette_close_light_glass_ready,
      command_palette_close_prismatic_icon_light_glass_ready: result.command_palette_close_prismatic_icon_light_glass_ready,
      command_palette_close_details: result.command_palette_close_details,
      command_palette_trigger_details: result.command_palette_trigger_details,
		      command_palette_input_light_glass_ready: result.command_palette_input_light_glass_ready,
		      command_palette_input_text_prismatic_etch_light_glass_ready: result.command_palette_input_text_prismatic_etch_light_glass_ready,
		      command_palette_input_placeholder_prismatic_etch_light_glass_ready: result.command_palette_input_placeholder_prismatic_etch_light_glass_ready,
		      command_palette_input_row_prismatic_separator_light_glass_ready: result.command_palette_input_row_prismatic_separator_light_glass_ready,
		      command_palette_results_well_light_glass_ready: result.command_palette_results_well_light_glass_ready,
			      command_palette_input_icon_light_glass_ready: result.command_palette_input_icon_light_glass_ready,
			      command_palette_input_icon_prismatic_light_glass_ready: result.command_palette_input_icon_prismatic_light_glass_ready,
		      command_palette_input_icon_details: result.command_palette_input_icon_details,
	      command_palette_item_light_glass_ready: result.command_palette_item_light_glass_ready,
	      command_palette_item_prismatic_rim_light_glass_ready: result.command_palette_item_prismatic_rim_light_glass_ready,
	      command_palette_item_hover_prismatic_light_glass_ready: result.command_palette_item_hover_prismatic_light_glass_ready,
	      command_palette_item_label_prismatic_etch_light_glass_ready: result.command_palette_item_label_prismatic_etch_light_glass_ready,
      command_palette_item_details: result.command_palette_item_details,
	      command_palette_input_details: result.command_palette_input_details,
	      command_palette_input_row_details: result.command_palette_input_row_details,
	      command_palette_results_well_details: result.command_palette_results_well_details,
      control_form_control_title_touch_ready: result.control_form_control_title_touch_ready,
      control_form_control_details: result.control_form_control_details,
      chat_row_option_semantic_touch_ready: result.chat_row_option_semantic_touch_ready,
      chat_row_option_details: result.chat_row_option_details,
      rail_chat_row_prismatic_slab_light_glass_ready: result.rail_chat_row_prismatic_slab_light_glass_ready,
      rail_chat_row_prismatic_slab_details: result.rail_chat_row_prismatic_slab_details,
      thread_tools_menu_ready: result.thread_tools_menu_ready,
      thread_tools_trigger_details: result.thread_tools_trigger_details,
      thread_tools_panel_details: result.thread_tools_panel_details,
      thread_tools_item_details: result.thread_tools_item_details,
      composer_tools_menu_ready: result.composer_tools_menu_ready,
      composer_tools_trigger_light_glass_ready: result.composer_tools_trigger_light_glass_ready,
      composer_tools_trigger_details: result.composer_tools_trigger_details,
      composer_tools_panel_details: result.composer_tools_panel_details,
      composer_tools_item_details: result.composer_tools_item_details,
      composer_popover_ready: result.composer_popover_ready,
      composer_popover_item_label_prismatic_etch_light_glass_ready: result.composer_popover_item_label_prismatic_etch_light_glass_ready,
      composer_popover_header_prismatic_etch_light_glass_ready: result.composer_popover_header_prismatic_etch_light_glass_ready,
      composer_popover_header_prismatic_etch_details: result.composer_popover_header_prismatic_etch_details,
      composer_popover_search_light_glass_ready: result.composer_popover_search_light_glass_ready,
      composer_popover_search_placeholder_prismatic_etch_light_glass_ready: result.composer_popover_search_placeholder_prismatic_etch_light_glass_ready,
      rail_search_light_glass_ready: result.rail_search_light_glass_ready,
      rail_search_placeholder_prismatic_etch_light_glass_ready: result.rail_search_placeholder_prismatic_etch_light_glass_ready,
      rail_search_placeholder_prismatic_etch_details: result.rail_search_placeholder_prismatic_etch_details,
      rail_prismatic_filter_light_glass_ready: result.rail_prismatic_filter_light_glass_ready,
      rail_prismatic_filter_details: result.rail_prismatic_filter_details,
      rail_search_visible_count: result.rail_search_visible_count,
      rail_search_details: result.rail_search_details,
      composer_popover_panel_details: result.composer_popover_panel_details,
      composer_popover_search_details: result.composer_popover_search_details,
      composer_popover_item_details: result.composer_popover_item_details,
      micro_surface_light_glass_ready: result.micro_surface_light_glass_ready,
      micro_prismatic_badge_light_glass_ready: result.micro_prismatic_badge_light_glass_ready,
      micro_badge_label_prismatic_etch_light_glass_ready: result.micro_badge_label_prismatic_etch_light_glass_ready,
      micro_surface_details: result.micro_surface_details,
      message_metadata_prismatic_light_glass_ready: result.message_metadata_prismatic_light_glass_ready,
      message_metadata_prismatic_details: result.message_metadata_prismatic_details,
      thread_subtitle_prismatic_light_glass_ready: result.thread_subtitle_prismatic_light_glass_ready,
      thread_subtitle_prismatic_details: result.thread_subtitle_prismatic_details,
      composer_shortcut_hint_prismatic_light_glass_ready: result.composer_shortcut_hint_prismatic_light_glass_ready,
      composer_shortcut_hint_expected_visible: result.composer_shortcut_hint_expected_visible,
      composer_shortcut_hint_prismatic_details: result.composer_shortcut_hint_prismatic_details,
      rail_metadata_chip_prismatic_light_glass_ready: result.rail_metadata_chip_prismatic_light_glass_ready,
      rail_metadata_chip_expected_visible: result.rail_metadata_chip_expected_visible,
      rail_metadata_chip_prismatic_details: result.rail_metadata_chip_prismatic_details,
	      rail_status_count_prismatic_light_glass_ready: result.rail_status_count_prismatic_light_glass_ready,
	      rail_status_count_expected_visible: result.rail_status_count_expected_visible,
	      rail_status_count_prismatic_details: result.rail_status_count_prismatic_details,
	      rail_preview_prismatic_etch_light_glass_ready: result.rail_preview_prismatic_etch_light_glass_ready,
	      rail_preview_expected_visible: result.rail_preview_expected_visible,
	      rail_preview_prismatic_etch_details: result.rail_preview_prismatic_etch_details,
	      rail_chat_title_prismatic_etch_light_glass_ready: result.rail_chat_title_prismatic_etch_light_glass_ready,
	      rail_chat_title_expected_visible: result.rail_chat_title_expected_visible,
	      rail_chat_title_prismatic_etch_details: result.rail_chat_title_prismatic_etch_details,
	      message_body_prismatic_etch_light_glass_ready: result.message_body_prismatic_etch_light_glass_ready,
	      message_body_prismatic_etch_details: result.message_body_prismatic_etch_details,
	      message_speaker_prismatic_chip_light_glass_ready: result.message_speaker_prismatic_chip_light_glass_ready,
	      message_speaker_prismatic_chip_details: result.message_speaker_prismatic_chip_details,
	      composer_placeholder_prismatic_etch_light_glass_ready: result.composer_placeholder_prismatic_etch_light_glass_ready,
	      composer_placeholder_prismatic_etch_details: result.composer_placeholder_prismatic_etch_details,
	      header_title_prismatic_etch_light_glass_ready: result.header_title_prismatic_etch_light_glass_ready,
      header_title_expected_count: result.header_title_expected_count,
      header_title_prismatic_etch_details: result.header_title_prismatic_etch_details,
      message_routing_badge_light_glass_ready: result.message_routing_badge_light_glass_ready,
      thread_intro_badge_light_glass_ready: result.thread_intro_badge_light_glass_ready,
      thread_intro_badge_visible: result.thread_intro_badge_visible,
      thread_intro_badge_details: result.thread_intro_badge_details,
      status_trust_strip_light_glass_ready: result.status_trust_strip_light_glass_ready,
      status_trust_strip_visible: result.status_trust_strip_visible,
      status_trust_badge_details: result.status_trust_badge_details,
      nav_icon_ready: result.nav_icon_ready,
      scroll_edge_ready: result.scroll_edge_ready,
      microcopy_word_split_guard_ready: result.microcopy_word_split_guard_ready,
      microcopy_wrap_details: result.microcopy_wrap_details,
      logo_clip_guard_ready: result.logo_clip_guard_ready,
      logo_clip_details: result.logo_clip_details,
      avatar_prismatic_rim_light_glass_ready: result.avatar_prismatic_rim_light_glass_ready,
      avatar_prismatic_rim_details: result.avatar_prismatic_rim_details,
      active_chat_readability_ready: result.active_chat_readability_ready,
      active_chat_readability_details: result.active_chat_readability_details,
      placeholder_readability_ready: result.placeholder_readability_ready,
      placeholder_readability_details: result.placeholder_readability_details,
      small_control_readability_ready: result.small_control_readability_ready,
      small_control_readability_details: result.small_control_readability_details,
      visible_text_integrity_ready: result.visible_text_integrity_ready,
      visible_text_integrity_probe: result.visible_text_integrity_probe,
      composer_glass_ready: result.composer_glass_ready,
      send_glass_ready: result.send_glass_ready,
      horizontal_overflow_free: result.horizontal_overflow_free,
      selectors: result.selectors,
    })),
    failures,
  };

  fs.writeSync(1, JSON.stringify(report) + "\n");
  if (failures.length > 0) {
    process.exit(1);
  }
})().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
NODE
}

capture_viewport "desktop" "1365x900"
capture_viewport "narrow" "768x900"
capture_viewport "mobile" "500x844"
capture_viewport "phone320" "320x844"

density_qa_status=0
density_qa_json="$(run_density_qa)" || density_qa_status="$?"
printf '%s\n' "$density_qa_json" >"$OUT_DIR/density-qa.json"

if [[ "$density_qa_status" != "0" ]] || ! jq -e '
  .status == "ready"
  and .control_ui_visual_density_qa_ready == true
  and .viewport_count == 4
  and .phone320_ready == true
  and .default_submenus_closed_ready == true
  and .single_submenu_audit_ready == true
  and .engineering_session_chips_suppressed_ready == true
  and .preferred_touch_targets_ready == true
  and .control_glass_action_ready == true
  and .harsh_referee_ready == true
  and .rail_action_icon_ready == true
  and .icon_button_ready == true
  and .icon_prismatic_control_light_glass_ready == true
	  and .topbar_action_light_glass_ready == true
	  and .primary_shell_light_glass_ready == true
	  and .translucent_shell_light_glass_ready == true
	  and .refractive_depth_light_glass_ready == true
	  and .optical_clarity_light_glass_ready == true
	  and .surface_clear_alpha_light_glass_ready == true
	  and .substrate_caustic_field_light_glass_ready == true
	  and .specular_edge_light_glass_ready == true
	  and .prismatic_dispersion_light_glass_ready == true
	  and .caustic_highlight_light_glass_ready == true
	  and .caustic_depth_shift_light_glass_ready == true
		  and .optical_thickness_tiers_light_glass_ready == true
		  and .faceted_reflection_light_glass_ready == true
		  and .beveled_rim_light_glass_ready == true
		  and .micro_refraction_light_glass_ready == true
		  and .sparkle_glint_light_glass_ready == true
		  and .lens_bloom_light_glass_ready == true
		  and .spectral_fusion_light_glass_ready == true
		  and .optical_magnification_light_glass_ready == true
		  and .biaxial_magnification_light_glass_ready == true
		  and .anisotropic_magnification_light_glass_ready == true
		  and .phase_separated_refraction_light_glass_ready == true
		  and .two_axis_phase_refraction_light_glass_ready == true
		  and .surface_phase_drift_light_glass_ready == true
		  and .surface_lens_scale_drift_light_glass_ready == true
		  and .layer_scale_parallax_light_glass_ready == true
		  and .surface_spectral_angle_drift_light_glass_ready == true
		  and .surface_glint_focal_drift_light_glass_ready == true
		  and .composer_glint_focal_decoupling_light_glass_ready == true
		  and .composer_spectral_angle_decoupling_light_glass_ready == true
		  and .composer_phase_decoupling_light_glass_ready == true
		  and .composer_layer_scale_decoupling_light_glass_ready == true
		  and .chrome_bar_translucency_light_glass_ready == true
		  and .chrome_refractive_skin_light_glass_ready == true
		  and .clear_white_balance_light_glass_ready == true
		  and .chamfer_cut_edge_light_glass_ready == true
		  and .prismatic_cut_edge_light_glass_ready == true
		  and .pane_prismatic_perimeter_light_glass_ready == true
		  and .composer_prismatic_control_light_glass_ready == true
		  and .menu_trigger_ready == true
	  and .folder_chip_touch_ready == true
	  and .folder_chip_label_prismatic_etch_light_glass_ready == true
	  and .row_menu_touch_ready == true
		  and .row_menu_all_rows_ready == true
		  and .row_menu_light_glass_ready == true
	  and .command_palette_ready == true
	  and .command_palette_surface_light_glass_ready == true
	  and .command_palette_surface_prismatic_perimeter_light_glass_ready == true
	  and .command_palette_backdrop_caustic_veil_light_glass_ready == true
	  and .command_palette_trigger_light_glass_ready == true
	  and .command_palette_close_light_glass_ready == true
	  and .command_palette_close_prismatic_icon_light_glass_ready == true
	  and .command_palette_input_light_glass_ready == true
		  and .command_palette_input_text_prismatic_etch_light_glass_ready == true
		  and .command_palette_input_placeholder_prismatic_etch_light_glass_ready == true
			  and .command_palette_input_row_prismatic_separator_light_glass_ready == true
				  and .command_palette_results_well_light_glass_ready == true
				  and .command_palette_results_well_prismatic_rim_light_glass_ready == true
				  and .command_palette_input_icon_light_glass_ready == true
				  and .command_palette_input_icon_prismatic_light_glass_ready == true
				  and .command_palette_item_light_glass_ready == true
				  and .command_palette_item_prismatic_rim_light_glass_ready == true
			  and .command_palette_kind_chip_light_glass_ready == true
			  and .command_palette_item_hover_prismatic_light_glass_ready == true
			  and .command_palette_item_label_prismatic_etch_light_glass_ready == true
		  and .control_form_control_title_touch_ready == true
		  and .chat_row_option_semantic_touch_ready == true
		  and .rail_chat_row_prismatic_slab_light_glass_ready == true
		  and .menu_item_icon_ready == true
  and .menu_surface_ready == true
	  and .thread_tools_menu_ready == true
	  and .composer_tools_menu_ready == true
	  and .composer_popover_ready == true
	  and .composer_popover_item_label_prismatic_etch_light_glass_ready == true
	  and .composer_popover_header_prismatic_etch_light_glass_ready == true
	  and .composer_popover_search_light_glass_ready == true
	  and .composer_popover_search_placeholder_prismatic_etch_light_glass_ready == true
	  and .rail_search_light_glass_ready == true
	  and .rail_search_placeholder_prismatic_etch_light_glass_ready == true
	  and .rail_prismatic_filter_light_glass_ready == true
	  and .micro_surface_light_glass_ready == true
	  and .micro_prismatic_badge_light_glass_ready == true
	  and .micro_badge_label_prismatic_etch_light_glass_ready == true
	  and .message_metadata_prismatic_light_glass_ready == true
	  and .thread_subtitle_prismatic_light_glass_ready == true
	  and .composer_shortcut_hint_prismatic_light_glass_ready == true
		  and .rail_metadata_chip_prismatic_light_glass_ready == true
	  and .rail_status_count_prismatic_light_glass_ready == true
	  and .rail_preview_prismatic_etch_light_glass_ready == true
	  and .rail_chat_title_prismatic_etch_light_glass_ready == true
	  and .message_body_prismatic_etch_light_glass_ready == true
		  and .message_speaker_prismatic_chip_light_glass_ready == true
		  and .composer_placeholder_prismatic_etch_light_glass_ready == true
		  and .header_title_prismatic_etch_light_glass_ready == true
	  and .message_routing_badge_light_glass_ready == true
	  and .thread_intro_badge_light_glass_ready == true
	  and .status_trust_strip_light_glass_ready == true
	  and .nav_icon_ready == true
		  and .scroll_edge_ready == true
		  and .microcopy_word_split_guard_ready == true
		  and .logo_clip_guard_ready == true
		  and .avatar_prismatic_rim_light_glass_ready == true
		  and .active_chat_readability_ready == true
		  and .placeholder_readability_ready == true
		  and .small_control_readability_ready == true
		  and .visible_text_integrity_ready == true
		  and .horizontal_overflow_free == true
	  and .browser_error_page_absent == true
  and (.results | length) == 4
  and (.results | all(.status == "ready"))
' <<<"$density_qa_json" >/dev/null; then
  echo "control UI density QA failed" >&2
  jq '{
    status,
    failures,
    viewport_count,
    phone320_ready,
    default_submenus_closed_ready,
    single_submenu_audit_ready,
    engineering_session_chips_suppressed_ready,
    preferred_touch_targets_ready,
    control_glass_action_ready,
    harsh_referee_ready,
    rail_action_icon_ready,
    icon_button_ready,
    icon_prismatic_control_light_glass_ready,
    topbar_action_light_glass_ready,
    primary_shell_light_glass_ready,
	    translucent_shell_light_glass_ready,
	    refractive_depth_light_glass_ready,
	    optical_clarity_light_glass_ready,
	    surface_clear_alpha_light_glass_ready,
	    substrate_caustic_field_light_glass_ready,
	    specular_edge_light_glass_ready,
		    prismatic_dispersion_light_glass_ready,
		    caustic_highlight_light_glass_ready,
		    caustic_depth_shift_light_glass_ready,
		    menu_trigger_ready,
    folder_chip_touch_ready,
    folder_chip_label_prismatic_etch_light_glass_ready,
	    row_menu_touch_ready,
		    row_menu_all_rows_ready,
		    row_menu_light_glass_ready,
		    command_palette_ready,
		    command_palette_surface_prismatic_perimeter_light_glass_ready,
		    command_palette_trigger_light_glass_ready,
	    command_palette_close_light_glass_ready,
		        command_palette_input_light_glass_ready,
		        command_palette_input_text_prismatic_etch_light_glass_ready,
		        command_palette_input_placeholder_prismatic_etch_light_glass_ready,
		        command_palette_input_row_prismatic_separator_light_glass_ready,
	    command_palette_item_light_glass_ready,
	    control_form_control_title_touch_ready,
	    chat_row_option_semantic_touch_ready,
	    rail_chat_row_prismatic_slab_light_glass_ready,
	    menu_item_icon_ready,
    menu_surface_ready,
    thread_tools_menu_ready,
    composer_tools_menu_ready,
    composer_popover_ready,
    rail_search_light_glass_ready,
    rail_prismatic_filter_light_glass_ready,
    micro_surface_light_glass_ready,
    micro_prismatic_badge_light_glass_ready,
    micro_badge_label_prismatic_etch_light_glass_ready,
    message_metadata_prismatic_light_glass_ready,
    thread_subtitle_prismatic_light_glass_ready,
    composer_shortcut_hint_prismatic_light_glass_ready,
    rail_metadata_chip_prismatic_light_glass_ready,
    rail_chat_title_prismatic_etch_light_glass_ready,
    message_routing_badge_light_glass_ready,
    thread_intro_badge_light_glass_ready,
	    status_trust_strip_light_glass_ready,
	    faceted_reflection_light_glass_ready,
	    beveled_rim_light_glass_ready,
	    micro_refraction_light_glass_ready,
	    sparkle_glint_light_glass_ready,
	    lens_bloom_light_glass_ready,
	    spectral_fusion_light_glass_ready,
	    optical_magnification_light_glass_ready,
	    biaxial_magnification_light_glass_ready,
	    anisotropic_magnification_light_glass_ready,
	    phase_separated_refraction_light_glass_ready,
	    two_axis_phase_refraction_light_glass_ready,
	    surface_phase_drift_light_glass_ready,
	    surface_lens_scale_drift_light_glass_ready,
	    layer_scale_parallax_light_glass_ready,
	    surface_spectral_angle_drift_light_glass_ready,
	    surface_glint_focal_drift_light_glass_ready,
	    composer_glint_focal_decoupling_light_glass_ready,
	    composer_spectral_angle_decoupling_light_glass_ready,
	    composer_phase_decoupling_light_glass_ready,
	    composer_layer_scale_decoupling_light_glass_ready,
	    chrome_bar_translucency_light_glass_ready,
	    chrome_refractive_skin_light_glass_ready,
	    clear_white_balance_light_glass_ready,
	    nav_icon_ready,
	    scroll_edge_ready,
	    microcopy_word_split_guard_ready,
	    logo_clip_guard_ready,
	    avatar_prismatic_rim_light_glass_ready,
	    active_chat_readability_ready,
		    placeholder_readability_ready,
		    small_control_readability_ready,
		    visible_text_integrity_ready,
		    horizontal_overflow_free,
    browser_error_page_absent,
    bad_viewports: [
      .results[] | select(.status != "ready" or .harsh_referee_ready != true) | {
        name,
        status,
        errors,
        default_submenus_closed_ready,
        default_submenus_closed_details,
        single_submenu_audit_ready,
        single_submenu_audit_target_count,
        single_submenu_audit_details,
        engineering_session_chips_suppressed_ready,
        engineering_session_chip_details,
        icon_button_ready,
        topbar_action_light_glass_ready,
        primary_shell_light_glass_ready,
	        translucent_shell_light_glass_ready,
	        refractive_depth_light_glass_ready,
	        optical_clarity_light_glass_ready,
	        surface_clear_alpha_light_glass_ready,
	        substrate_caustic_field_light_glass_ready,
	        chrome_refractive_skin_light_glass_ready,
	        clear_white_balance_light_glass_ready,
		        specular_edge_light_glass_ready,
		        prismatic_dispersion_light_glass_ready,
		        caustic_highlight_light_glass_ready,
		        caustic_depth_shift_light_glass_ready,
			        caustic_depth_shift_key_count,
			        faceted_reflection_light_glass_ready,
			        beveled_rim_light_glass_ready,
			        micro_refraction_light_glass_ready,
			        sparkle_glint_light_glass_ready,
			        lens_bloom_light_glass_ready,
			        spectral_fusion_light_glass_ready,
			        optical_magnification_light_glass_ready,
			        biaxial_magnification_light_glass_ready,
			        anisotropic_magnification_light_glass_ready,
			        phase_separated_refraction_light_glass_ready,
			        two_axis_phase_refraction_light_glass_ready,
			        surface_phase_drift_light_glass_ready,
			        surface_lens_scale_drift_light_glass_ready,
			        layer_scale_parallax_light_glass_ready,
			        surface_spectral_angle_drift_light_glass_ready,
			        surface_glint_focal_drift_light_glass_ready,
			        composer_glint_focal_decoupling_light_glass_ready,
			        refractive_depth_details,
			        surface_clear_alpha_details,
			        substrate_caustic_field_details,
		        specular_edge_details,
		        prismatic_dispersion_details,
		        caustic_highlight_details,
			        caustic_depth_shift_details,
			        faceted_reflection_details,
			        beveled_rim_details,
			        micro_refraction_details,
			        sparkle_glint_details,
			        lens_bloom_details,
			        spectral_fusion_details,
			        optical_magnification_details,
			        biaxial_magnification_details,
			        anisotropic_magnification_details,
			        phase_separated_refraction_details,
			        two_axis_phase_refraction_details,
			        surface_phase_drift_position_count,
			        surface_phase_drift_details,
			        surface_lens_scale_drift_size_count,
			        surface_lens_scale_drift_details,
			        layer_scale_parallax_details,
			        surface_spectral_angle_drift_details,
			        surface_glint_focal_drift_details,
			        composer_glint_focal_decoupling_details,
			        chrome_refractive_skin_details,
			        clear_white_balance_details,
			        chamfer_cut_edge_details,
			        menu_trigger_ready,
	        folder_chip_touch_ready,
	        folder_chip_label_prismatic_etch_light_glass_ready,
	        folder_chip_details,
	        row_menu_touch_ready,
	        row_menu_light_glass_ready,
	        command_palette_ready,
	        command_palette_surface_prismatic_perimeter_light_glass_ready,
	        command_palette_trigger_light_glass_ready,
	        command_palette_close_light_glass_ready,
	        command_palette_close_prismatic_icon_light_glass_ready,
	        command_palette_input_light_glass_ready,
	        command_palette_input_text_prismatic_etch_light_glass_ready,
	        command_palette_input_row_prismatic_separator_light_glass_ready,
	        command_palette_input_icon_light_glass_ready,
		        command_palette_item_light_glass_ready,
		        command_palette_item_label_prismatic_etch_light_glass_ready,
		        control_form_control_title_touch_ready,
		        chat_row_option_semantic_touch_ready,
		        rail_chat_row_prismatic_slab_light_glass_ready,
		        menu_item_icon_ready,
        thread_tools_menu_ready,
        composer_popover_item_label_prismatic_etch_light_glass_ready,
        composer_popover_search_light_glass_ready,
        composer_popover_search_placeholder_prismatic_etch_light_glass_ready,
        rail_search_light_glass_ready,
        rail_search_placeholder_prismatic_etch_light_glass_ready,
        rail_search_placeholder_prismatic_etch_details,
        rail_prismatic_filter_light_glass_ready,
        message_metadata_prismatic_light_glass_ready,
        message_metadata_prismatic_details,
        thread_subtitle_prismatic_light_glass_ready,
        thread_subtitle_prismatic_details,
        composer_shortcut_hint_prismatic_light_glass_ready,
        composer_shortcut_hint_prismatic_details,
        rail_metadata_chip_prismatic_light_glass_ready,
	        rail_metadata_chip_prismatic_details,
	        rail_status_count_prismatic_light_glass_ready,
	        rail_status_count_prismatic_details,
	        rail_preview_prismatic_etch_light_glass_ready,
	        rail_preview_prismatic_etch_details,
	        rail_chat_title_prismatic_etch_light_glass_ready,
	        rail_chat_title_prismatic_etch_details,
	        message_body_prismatic_etch_light_glass_ready,
	        message_body_prismatic_etch_details,
	        message_speaker_prismatic_chip_light_glass_ready,
	        message_speaker_prismatic_chip_details,
	        composer_placeholder_prismatic_etch_light_glass_ready,
	        composer_placeholder_prismatic_etch_details,
	        header_title_prismatic_etch_light_glass_ready,
        header_title_prismatic_etch_details,
        status_trust_strip_light_glass_ready,
        icon_button_title_match_ready,
        menu_trigger_title_match_ready,
        menu_surface_ready,
        nav_icon_ready,
        scroll_edge_ready,
        bad_icon_buttons: [
          (.icon_button_details // [])[] | select(.visible != true or .width < 44 or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .svg_icon_present != true or .visible_icon_text_absent != true)
        ],
        bad_icon_prismatic_controls: [
          (.icon_prismatic_control_details // [])[] | select(.icon_prismatic_control_ready != true or (.icon_prismatic_control_drop_shadow_count // 0) < 2 or .box_shadow == "none" or ((.backdrop_filter // "") | contains("blur(") | not))
        ],
        bad_translucent_glass: [
          (.translucent_glass_details // [])[] | select(.translucent_ready != true or .background_alpha < 0.35 or .background_alpha > 0.88 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none")
        ],
        bad_refractive_depth: (
          .refractive_depth_details // {} | select(.body_background_image != "present" or .before_background_image != "present" or .before_opacity < 0.12 or .primary_shell_gradient_count < 3 or .primary_shell_low_alpha_count < 3)
        ),
	        bad_optical_clarity: (
	          .refractive_depth_details // {} | select(.body_background_translucent_layer != true or .body_background_layer_count < 3 or .before_opacity < 0.2 or .primary_shell_clear_alpha_count < 3)
	        ),
	        bad_surface_clear_alpha: [
	          (.surface_clear_alpha_details // [])[] | select(.clear_alpha_ready != true or (.surface_alpha_max // 1) > 0.49 or (.surface_alpha_average // 1) > 0.44 or (.surface_alpha_min // 1) > 0.4 or (.surface_alpha_below_045_count // 0) < ((.surface_count // 1) - 1) or .readable != true)
	        ],
	        bad_substrate_caustic_field: (
	          .substrate_caustic_field_details // {} | select(.body_background_layer_count < 4 or .body_background_repeating_layer_count < 2 or .body_background_angle_count < 4 or .body_background_translucent_layer != true or .before_opacity < 0.2)
	        ),
	        bad_chrome_refractive_skin: [
	          (.chrome_refractive_skin_details // [])[] | select(.refractive_chrome_ready != true or (.chrome_refraction_layer_count // 0) < 2 or (.chrome_refraction_repeating_layer_count // 0) < 1 or (.specular_layer_count // 0) < 2)
	        ],
	        bad_clear_white_balance: (
	          .clear_white_balance_details // {} | select(.body_clear_white_ready != true or .primary_clear_white_ready != true or .chrome_clear_white_ready != true or (.body_background_channel_spread // 255) > 10 or (.primary_surface_channel_spread_max // 255) > 10 or (.chrome_channel_spread_max // 255) > 10)
	        ),
	        bad_chamfer_cut_edge: [
	          (.chamfer_cut_edge_details // [])[] | select(.polygon_clip_ready != true or .box_shadow == "none")
	        ],
	        bad_prismatic_cut_edge: [
	          (.prismatic_cut_edge_details // [])[] | select(.prismatic_cut_edge_ready != true or (.cut_edge_drop_shadow_count // 0) < 2)
	        ],
	        bad_pane_prismatic_perimeter: [
	          (.pane_prismatic_perimeter_details // [])[] | select(.pane_prismatic_perimeter_ready != true or (.perimeter_drop_shadow_count // 0) < 2 or .box_shadow == "none")
	        ],
	        bad_composer_prismatic_control: [
	          (.composer_prismatic_control_details // [])[] | select(.composer_prismatic_control_ready != true or (.control_drop_shadow_count // 0) < 2 or .box_shadow == "none" or ((.backdrop_filter // "") | contains("blur(") | not))
	        ],
	        bad_specular_edge: [
	          (.specular_edge_details // [])[] | select(.specular_edge_ready != true)
	        ],
	        bad_prismatic_dispersion: [
	          (.prismatic_dispersion_details // [])[] | select(.prismatic_dispersion_ready != true)
	        ],
		        bad_caustic_highlight: [
		          (.caustic_highlight_details // [])[] | select(.caustic_highlight_ready != true)
		        ],
		        bad_caustic_depth_shift: (
		          if (.caustic_depth_shift_key_count // 0) < 2
		          then (.caustic_depth_shift_details // [])
		          else [(.caustic_depth_shift_details // [])[] | select(.caustic_highlight_ready != true)]
		          end
		        ),
		        bad_faceted_reflection: [
		          (.faceted_reflection_details // [])[] | select(.faceted_reflection_ready != true or .faceted_reflection_angle_count < 3)
		        ],
			        bad_beveled_rim: [
			          (.beveled_rim_details // [])[] | select(.beveled_rim_ready != true or .beveled_rim_layer_count < 5)
			        ],
			        bad_micro_refraction: [
			          (.micro_refraction_details // [])[] | select(.micro_refraction_ready != true or .micro_refraction_line_count < 1)
			        ],
			        bad_sparkle_glint: [
			          (.sparkle_glint_details // [])[] | select(.sparkle_glint_ready != true or .sparkle_glint_count < 1)
			        ],
			        bad_lens_bloom: [
			          (.lens_bloom_details // [])[] | select(.lens_bloom_ready != true or .lens_bloom_count < 2)
			        ],
			        bad_spectral_fusion: [
			          (.spectral_fusion_details // [])[] | select(.spectral_fusion_ready != true or .spectral_fusion_layer_count < 6 or ((.spectral_fusion_blend_mode // "") | contains("screen") | not))
			        ],
			        bad_optical_magnification: [
			          (.optical_magnification_details // [])[] | select(.optical_magnification_ready != true or ((.optical_magnification_size // "") | contains("%") | not))
			        ],
			        bad_biaxial_magnification: [
			          (.biaxial_magnification_details // [])[] | select(.biaxial_magnification_ready != true or ((.biaxial_magnification_size // "") | test("[0-9]+% [0-9]+%") | not))
			        ],
			        bad_anisotropic_magnification: [
			          (.anisotropic_magnification_details // [])[] | select(.anisotropic_magnification_ready != true or (((.anisotropic_magnification_size // "") | contains("128% 132%")) or ((.anisotropic_magnification_size // "") | contains("126% 134%")) | not))
			        ],
			        bad_phase_separated_refraction: [
			          (.phase_separated_refraction_details // [])[] | select(.phase_separated_refraction_ready != true or (.phase_position_count // 0) < 6)
			        ],
			        bad_two_axis_phase_refraction: [
			          (.two_axis_phase_refraction_details // [])[] | select(.two_axis_phase_refraction_ready != true or (.phase_position_count // 0) < 6 or (.phase_y_axis_count // 0) < 3)
			        ],
			        bad_surface_phase_drift: [
			          (.surface_phase_drift_details // [])[] | select(.two_axis_phase_refraction_ready != true or (.surface_phase_drift_position_count // 0) < 2)
			        ],
			        bad_surface_lens_scale_drift: [
			          (.surface_lens_scale_drift_details // [])[] | select(.anisotropic_magnification_ready != true or (.surface_lens_scale_drift_size_count // 0) < 2)
			        ],
			        bad_layer_scale_parallax: [
			          (.layer_scale_parallax_details // [])[] | select(.layer_scale_parallax_ready != true or (.lens_scale_layer_count // 0) < 2 or (.lens_scale_parallax_size_count // 0) < 2)
			        ],
			        bad_surface_spectral_angle_drift: [
			          (.surface_spectral_angle_drift_details // [])[] | select(.layer_scale_parallax_ready != true or (.surface_spectral_angle_drift_signature_count // 0) < 2 or (.spectral_angle_layer_count // 0) < 4 or (.spectral_angle_count // 0) < 4)
			        ],
			        bad_surface_glint_focal_drift: [
			          (.surface_glint_focal_drift_details // [])[] | select(.surface_spectral_angle_drift_ready != true or (.surface_glint_focal_drift_signature_count // 0) < 2 or (.radial_focal_layer_count // 0) < 2 or (.radial_focal_count // 0) < 2)
			        ],
			        bad_composer_glint_focal_decoupling: [
			          (.composer_glint_focal_decoupling_details // [])[] | select(.composer_focal_decoupled != true or (.surface_glint_focal_drift_signature_count // 0) < 3 or (.radial_focal_layer_count // 0) < 2 or (.radial_focal_count // 0) < 2)
			        ],
			        bad_composer_spectral_angle_decoupling: [
			          (.composer_spectral_angle_decoupling_details // [])[] | select(.composer_spectral_angle_decoupled != true or (.surface_spectral_angle_drift_signature_count // 0) < 3 or (.spectral_angle_layer_count // 0) < 4 or (.spectral_angle_count // 0) < 4)
			        ],
			        bad_composer_phase_decoupling: [
			          (.composer_phase_decoupling_details // [])[] | select(.composer_phase_decoupled != true or (.surface_phase_drift_position_count // 0) < 3 or (.phase_position_count // 0) < 6 or (.phase_y_axis_count // 0) < 3)
			        ],
			        bad_composer_layer_scale_decoupling: [
			          (.composer_layer_scale_decoupling_details // [])[] | select(.composer_layer_scale_decoupled != true or (.surface_lens_scale_drift_size_count // 0) < 3 or (.lens_scale_layer_count // 0) < 2 or (.lens_scale_parallax_size_count // 0) < 2)
			        ],
			        bad_chrome_bar_translucency: [
			          (.chrome_bar_translucency_details // [])[] | select(.translucent_chrome_ready != true or (.background_alpha // 1) > 0.72 or (.backdrop_blur_px // 0) < 20)
			        ],
			        bad_menu_triggers: [
          (.menu_trigger_details // [])[] | select(.visible != true or .width < 44 or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .svg_icon_present != true or .visible_icon_text_absent != true)
        ],
        bad_folder_chips: [
          (.folder_chip_details // [])[] | select(.visible != true or .width < 44 or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .active_state_matches_aria_pressed != true or .box_shadow == "none")
        ],
        bad_folder_chip_label_prismatic_etch: [
          (.folder_chip_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .width < 44 or .height < 44 or .text_shadow != "present" or .folder_chip_label_prismatic_etch_ready != true or (.folder_chip_label_text_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5)
        ],
        bad_row_menu_toggles: [
          (.row_menu_toggle_details // [])[] | select(.marker != "light-glass" or .width < 44 or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .svg_icon_present != true or .visible_icon_text_absent != true or .box_shadow == "none")
        ],
	        bad_row_menu_panels: [
	          (.row_menu_panel_details // [])[] | select(.visible != true or .item_count < 3 or .width < 180 or .height < 132 or .border_radius < 16 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .in_viewport != true or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98)
	        ],
	        bad_row_menu_items: [
	          (.row_menu_item_details // [])[] | select(.visible != true or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .icon_svg_present != true or .label_nowrap_ready != true or .readable != true or .contrast_ratio < 4.5)
	        ],
	        bad_command_palette: {
	          trigger: [(.command_palette_trigger_details // [])[] | select(.visible != true or .marker != "light-glass" or .href != "#command-palette" or .width < 44 or .height < 44 or .border_radius < 20 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none" or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .svg_icon_present != true or .visible_icon_text_absent != true or .readable != true or .contrast_ratio < 4.5)],
	          panel: (.command_palette_panel_details // {} | select(.visible != true or .marker != "light-glass" or .role != "dialog" or .aria_modal != "true" or .aria_label != "Command palette" or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or .border_radius < 18 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .in_viewport != true)),
	          backdrop: (.command_palette_backdrop_details // {} | select(.visible != true or .background_alpha < 0.2 or .background_alpha > 0.6 or .background_image != "present" or (.command_palette_backdrop_repeating_layer_count // 0) < 1 or .command_palette_backdrop_caustic_veil_ready != true or (.backdrop_blur_px // 0) < 10 or .covers_viewport != true)),
	          close: (.command_palette_close_details // {} | select(.visible != true or .marker != "light-glass" or .href != "#commands" or .width < 44 or .height < 44 or .border_radius < 20 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.aria_label // "") != "Close command palette") or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .svg_icon_present != true or .visible_icon_text_absent != true or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .readable != true or .contrast_ratio < 4.5)),
	          input: (.command_palette_input_details // {} | select(.visible != true or .marker != "light-glass" or .type != "search" or ((.placeholder // "") | length) == 0 or .height < 44 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .readable != true or .contrast_ratio < 4.5)),
	          items: [(.command_palette_item_details // [])[] | select(.visible != true or .marker != "light-glass" or .width < 180 or .height < 44 or .border_radius < 8 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or ((.key // "") | length) == 0 or ((.kind // "") | length) == 0 or ((.label // "") | length) == 0 or ((.detail // "") | length) == 0 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .readable != true or .contrast_ratio < 4.5)]
	        },
	        bad_command_palette_surface_prismatic_perimeter: (.command_palette_panel_details // {} | select(.filter != "present" or .command_palette_surface_prismatic_perimeter_ready != true or (.command_palette_surface_drop_shadow_count // 0) < 2 or .box_shadow == "none")),
	        bad_command_palette_item_label_prismatic_etch: [
	          (.command_palette_item_details // [])[] | select(.kind_text_shadow != "present" or .label_text_shadow != "present" or .detail_text_shadow != "present" or .command_palette_item_label_prismatic_etch_ready != true or (.command_palette_item_kind_text_shadow_count // 0) < 2 or (.command_palette_item_label_text_shadow_count // 0) < 2 or (.command_palette_item_detail_text_shadow_count // 0) < 2 or .kind_readable != true or .readable != true or .detail_readable != true or .kind_contrast_ratio < 4.5 or .contrast_ratio < 4.5 or .detail_contrast_ratio < 4.5)
	        ],
	        bad_command_palette_kind_chip: [
	          (.command_palette_item_details // [])[] | select((.kind_width // 0) < 44 or (.kind_height // 0) < 22 or (.kind_background_alpha // 0) < 0.25 or (.kind_background_alpha // 0) > 0.75 or (.kind_effective_luminance // 0) < 0.72 or (.kind_effective_luminance // 0) > 0.98 or (.kind_border_alpha // 0) < 0.25 or (.kind_border_radius // 0) < 20 or ((.kind_backdrop_filter // "") | contains("blur(") | not) or .kind_box_shadow == "none" or (.command_palette_kind_chip_shadow_count // 0) < 2 or .command_palette_kind_chip_light_glass_ready != true or .kind_readable != true or .kind_contrast_ratio < 4.5)
	        ],
	        bad_command_palette_item_hover_prismatic: [
	          (.command_palette_item_details // [])[] | select(.audit_hover == true and ((.command_palette_item_hover_prismatic_ready != true) or ((.command_palette_item_hover_shadow_count // 0) < 2) or ((.border_alpha // 0) < 0.25) or .box_shadow == "none"))
	        ],
	        bad_command_palette_item_prismatic_rim: [
	          (.command_palette_item_details // [])[] | select(.command_palette_item_prismatic_rim_ready != true or (.command_palette_item_rim_shadow_count // 0) < 2 or (.border_alpha // 0) < 0.25 or .box_shadow == "none")
	        ],
		        bad_command_palette_close_prismatic_icon: (.command_palette_close_details // {} | select(.filter != "present" or .command_palette_close_prismatic_icon_ready != true or (.command_palette_close_drop_shadow_count // 0) < 2 or .svg_icon_present != true or .visible_icon_text_absent != true)),
			        bad_command_palette_input_text_prismatic_etch: (.command_palette_input_details // {} | select(.text_shadow != "present" or .command_palette_input_prismatic_etch_ready != true or (.command_palette_input_text_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5)),
			        bad_command_palette_input_placeholder_prismatic_etch: (.command_palette_input_details // {} | select(((.placeholder // "") | length) == 0 or .placeholder_text_shadow != "present" or .command_palette_input_placeholder_prismatic_etch_ready != true or (.command_palette_input_placeholder_text_shadow_count // 0) < 2 or (.command_palette_input_placeholder_font_weight // 0) < 600 or .placeholder_readable != true or .placeholder_contrast_ratio < 4.5)),
			        bad_command_palette_input_row_prismatic_separator: (.command_palette_input_row_details // {} | select(.visible != true or .width < 274 or .height < 60 or (.border_bottom_alpha // 0) < 0.25 or .box_shadow == "none" or (.command_palette_input_row_separator_shadow_count // 0) < 2 or .command_palette_input_row_prismatic_separator_ready != true)),
		        bad_command_palette_results_well: (.command_palette_results_well_details // {} | select(.visible != true or .width < 274 or .height < 58 or (.background_alpha // 0) < 0.1 or (.background_alpha // 0) > 0.4 or .light_glass_ready != true or ((.backdrop_filter // "") | contains("blur(") | not) or (.backdrop_blur_px // 0) < 10 or .command_palette_results_well_light_glass_ready != true)),
		        bad_command_palette_results_well_prismatic_rim: (.command_palette_results_well_details // {} | select((.border_alpha // 0) < 0.25 or (.border_radius // 0) < 12 or .box_shadow == "none" or (.command_palette_results_well_rim_shadow_count // 0) < 2 or .command_palette_results_well_prismatic_rim_ready != true)),
		        bad_command_palette_input_icon: (.command_palette_input_icon_details // {} | select(.visible != true or .width < 44 or .height < 44 or .border_radius < 20 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none" or .svg_icon_present != true or .visible_icon_text_absent != true or .readable != true or .contrast_ratio < 4.5)),
		        bad_command_palette_input_icon_prismatic: (.command_palette_input_icon_details // {} | select(.filter != "present" or .command_palette_input_icon_prismatic_ready != true or (.command_palette_input_icon_drop_shadow_count // 0) < 2 or .svg_icon_present != true or .visible_icon_text_absent != true)),
		        bad_form_controls: [
		          (.control_form_control_details // [])[] | select(.height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .readable != true or .contrast_ratio < 4.5)
		        ],
		        bad_chat_row_options: [
		          (.chat_row_option_details // [])[] | select(.role != "option" or .width < 44 or .height < 64 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .tabindex != "0" or .active_state_matches_aria_selected != true or .border_radius < 18)
		        ],
		        bad_rail_chat_row_prismatic_slabs: [
		          (.rail_chat_row_prismatic_slab_details // [])[] | select(.visible != true or .width < 44 or .height < 64 or .border_radius < 18 or .chat_row_prismatic_slab_ready != true or (.chat_row_drop_shadow_count // 0) < 2 or .box_shadow == "none" or ((.backdrop_filter // "") | contains("blur(") | not))
		        ],
        bad_menu_items: [
          (.menu_item_details // [])[] | select(.icon_present != true or .icon_svg_present != true or .label_ready != true or .visible != true or .height < 36 or .label_nowrap_ready != true)
        ],
        bad_menu_surfaces: [
	          (.menu_surface_details // [])[] | select(.visible != true or .item_count < 1 or .width < 180 or .height < 44 or .border_radius < 16 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .in_viewport != true)
        ],
		        bad_thread_tools_trigger: (
		          .thread_tools_trigger_details // {} | select(.exists != true or .marker != "light-glass" or .visible != true or .width < 44 or .height < 44 or .border_radius < 20 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .title_matches_aria_label != true or .svg_icon_present != true or .visible_icon_text_absent != true or .readable != true or .contrast_ratio < 4.5)
		        ),
		        bad_thread_tools_panel: (
		          .thread_tools_panel_details // {} | select(.exists != true or .marker != "light-glass" or .visible != true or .role != "menu" or .aria_label != "Thread tools" or .item_count != 3 or .width < 180 or .height < 44 or .border_radius < 16 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .in_viewport != true or .top_clipped != false or .bottom_clipped != false)
		        ),
		        bad_thread_tools_items: [
		          (.thread_tools_item_details // [])[] | select(.visible != true or .role != "menuitem" or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .icon_svg_present != true or .label_nowrap_ready != true or .readable != true or .contrast_ratio < 4.5)
		        ],
		        bad_composer_tools_panel: (
		          .composer_tools_panel_details // {} | select(.exists != true or .visible != true or .role != "menu" or .aria_label != "Composer tools" or .item_count != 2 or .width < 180 or .height < 44 or .border_radius < 16 or .marker != "light-glass" or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .in_viewport != true or .top_clipped != false or .bottom_clipped != false)
		        ),
		        bad_composer_tools_items: [
		          (.composer_tools_item_details // [])[] | select(.visible != true or .role != "menuitem" or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .icon_svg_present != true or .label_nowrap_ready != true or .select_present != true or .select_visible != true or .select_height < 44 or ((.select_aria_label // "") | length) == 0 or ((.select_title // "") | length) == 0 or .select_title_matches_aria_label != true or .select_readable != true or .select_contrast_ratio < 4.5 or .readable != true or .contrast_ratio < 4.5)
		        ],
		        bad_composer_popover_toggles: [
		          (.composer_popover_toggle_details // [])[] | select(.visible != true or .width < 44 or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .aria_haspopup != "menu" or ((.aria_controls // "") | length) == 0 or .svg_icon_present != true or .visible_icon_text_absent != true)
		        ],
		        bad_composer_popover_panels: [
		          (.composer_popover_panel_details // [])[] | select(.visible != true or .role != "menu" or ((.aria_label // "") | length) == 0 or .search_count != 1 or .item_count != 2 or .width < 180 or .height < 132 or .border_radius < 16 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .in_viewport != true or .top_clipped != false or .bottom_clipped != false)
		        ],
		        bad_composer_popover_search: [
		          (.composer_popover_search_details // [])[] | select(.visible != true or .marker != "light-glass" or .height < 44 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .readable != true or .contrast_ratio < 4.5)
		        ],
		        bad_composer_popover_search_placeholder_prismatic_etch: [
		          (.composer_popover_search_details // [])[] | select(.visible != true or ((.placeholder // "") | length) == 0 or .height < 44 or .placeholder_text_shadow != "present" or .composer_popover_search_placeholder_prismatic_etch_ready != true or (.composer_popover_search_placeholder_text_shadow_count // 0) < 2 or .placeholder_readable != true or .placeholder_contrast_ratio < 4.5)
		        ],
        bad_rail_search: [
          (.rail_search_details // [])[] | select(.visible == true and (.marker != "light-glass" or .type != "search" or ((.placeholder // "") | length) == 0 or .width < 180 or .height < 44 or .border_radius < 12 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or .readable != true or .contrast_ratio < 4.5 or .placeholder_readable != true or .placeholder_contrast_ratio < 4.5))
        ],
        bad_rail_search_placeholder_prismatic_etch: [
          (.rail_search_placeholder_prismatic_etch_details // [])[] | select(.visible != true or ((.placeholder // "") | length) == 0 or .width < 180 or .height < 44 or .placeholder_text_shadow != "present" or .rail_search_placeholder_prismatic_etch_ready != true or (.rail_search_placeholder_text_shadow_count // 0) < 2 or .placeholder_readable != true or .placeholder_contrast_ratio < 4.5)
        ],
	        bad_rail_prismatic_filters: [
	          (.rail_prismatic_filter_details // [])[] | select(.visible != true or .width < 44 or .height < 44 or .border_radius < 12 or .rail_prismatic_filter_ready != true or (.rail_filter_drop_shadow_count // 0) < 2 or .box_shadow == "none" or ((.backdrop_filter // "") | contains("blur(") | not))
	        ],
		        bad_composer_popover_items: [
		          (.composer_popover_item_details // [])[] | select(.visible != true or .role != "menuitem" or .width < 120 or .height < 44 or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true or ((.label // "") | length) == 0 or ((.detail // "") | length) == 0 or .icon_svg_present != true or .label_nowrap_ready != true or .readable != true or .contrast_ratio < 4.5)
		        ],
		        bad_composer_popover_item_label_prismatic_etch: [
		          (.composer_popover_item_details // [])[] | select(.visible != true or ((.label // "") | length) == 0 or ((.detail // "") | length) == 0 or .label_text_shadow != "present" or .detail_text_shadow != "present" or .composer_popover_item_label_prismatic_etch_ready != true or (.composer_popover_item_label_text_shadow_count // 0) < 2 or (.composer_popover_item_detail_text_shadow_count // 0) < 2 or .readable != true or .detail_readable != true or .contrast_ratio < 4.5 or .detail_contrast_ratio < 4.5 or .label_nowrap_ready != true or .detail_nowrap_ready != true)
		        ],
		        bad_composer_popover_header_prismatic_etch: [
		          (.composer_popover_header_prismatic_etch_details // [])[] | select(.visible != true or .label_visible != true or .status_visible != true or ((.label // "") | length) == 0 or ((.status // "") | length) == 0 or .label_text_shadow != "present" or .status_text_shadow != "present" or .composer_popover_header_prismatic_etch_ready != true or (.composer_popover_header_label_text_shadow_count // 0) < 2 or (.composer_popover_header_status_text_shadow_count // 0) < 2 or .label_readable != true or .status_readable != true or .label_contrast_ratio < 4.5 or .status_contrast_ratio < 4.5)
		        ],
		        bad_micro_surfaces: [
		          (.micro_surface_details // [])[] | select(.visible != true or ((.key // "") | length) == 0 or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
		        ],
		        bad_micro_prismatic_badges: [
		          (.micro_surface_details // [])[] | select(.micro_prismatic_badge_ready != true or (.micro_prismatic_badge_drop_shadow_count // 0) < 2 or .box_shadow == "none" or ((.backdrop_filter // "") | contains("blur(") | not))
		        ],
		        bad_micro_badge_label_prismatic_etch: [
		          (.micro_surface_details // [])[] | select(.text_shadow != "present" or .micro_badge_label_prismatic_etch_ready != true or (.micro_badge_label_text_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5)
		        ],
		        bad_message_metadata_prismatic: [
		          (.message_metadata_prismatic_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none" or .message_metadata_prismatic_ready != true or (.message_metadata_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
		        ],
		        bad_thread_subtitle_prismatic: [
		          (.thread_subtitle_prismatic_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none" or .thread_subtitle_prismatic_ready != true or (.thread_subtitle_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
		        ],
		        bad_composer_shortcut_hint_prismatic: [
		          (.composer_shortcut_hint_prismatic_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none" or .composer_shortcut_hint_prismatic_ready != true or (.composer_shortcut_hint_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
		        ],
		        bad_rail_metadata_chip_prismatic: [
		          (.rail_metadata_chip_prismatic_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none" or .rail_metadata_chip_prismatic_ready != true or (.rail_metadata_chip_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
		        ],
			        bad_rail_status_count_prismatic: [
			          (.rail_status_count_prismatic_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none" or .rail_status_count_prismatic_ready != true or (.rail_status_count_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
			        ],
			        bad_rail_preview_prismatic_etch: [
			          (.rail_preview_prismatic_etch_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .width <= 20 or .height < 14 or .filter != "present" or .rail_preview_prismatic_etch_ready != true or (.rail_preview_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5)
			        ],
			        bad_rail_chat_title_prismatic_etch: [
			          (.rail_chat_title_prismatic_etch_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .width <= 20 or .height < 14 or .filter != "present" or .rail_chat_title_prismatic_etch_ready != true or (.rail_chat_title_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5)
			        ],
			        bad_message_body_prismatic_etch: [
			          (.message_body_prismatic_etch_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .width <= 20 or .height < 16 or .filter != "present" or .message_body_prismatic_etch_ready != true or (.message_body_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5)
			        ],
			        bad_message_speaker_prismatic_chip: [
			          (.message_speaker_prismatic_chip_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or ((.backdrop_filter // "") | contains("blur(") | not) or .box_shadow == "none" or .filter != "present" or .message_speaker_prismatic_chip_ready != true or (.message_speaker_chip_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
			        ],
			        bad_composer_placeholder_prismatic_etch: [
			          (.composer_placeholder_prismatic_etch_details // [])[] | select(.visible != true or ((.placeholder // "") | length) == 0 or .width < 100 or .height < 44 or .placeholder_text_shadow != "present" or .composer_placeholder_prismatic_etch_ready != true or (.composer_placeholder_text_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5)
			        ],
			        bad_header_title_prismatic_etch: [
		          (.header_title_prismatic_etch_details // [])[] | select(.visible != true or ((.text // "") | length) == 0 or .width <= 20 or .height < 16 or .filter != "present" or .header_title_prismatic_etch_ready != true or (.header_title_drop_shadow_count // 0) < 2 or .readable != true or .contrast_ratio < 4.5)
		        ],
		        bad_message_routing_badges: [
		          (.message_routing_badge_details // [])[] | select(.visible != true or ((.key // "") | length) == 0 or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
		        ],
		        bad_thread_intro_badges: [
		          (.thread_intro_badge_details // [])[] | select(.visible != true or ((.key // "") | length) == 0 or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true)
		        ],
		        bad_status_trust_badges: [
		          (.status_trust_badge_details // [])[] | select(.visible != true or ((.key // "") | length) == 0 or ((.text // "") | length) == 0 or .height < 22 or .border_radius < 10 or .light_glass_ready != true or .effective_luminance < 0.72 or .effective_luminance > 0.98 or (.backdrop_filter | contains("blur(") | not) or .box_shadow == "none" or .readable != true or .contrast_ratio < 4.5 or .label_nowrap_ready != true or ((.aria_label // "") | length) == 0 or ((.title // "") | length) == 0 or .title_matches_aria_label != true)
		        ],
	        bad_microcopy_wrap: [
	          (.microcopy_wrap_details // [])[] | select(.overflow_wrap == "anywhere" or .word_break == "break-word" or .word_break == "break-all")
	        ],
		        bad_logo_clip: [
		          (.logo_clip_details // [])[] | select(.visible != true or .image_present != true or .width < 32 or .height < 32 or .image_fills_container != true)
		        ],
		        bad_avatar_prismatic_rims: [
		          (.avatar_prismatic_rim_details // [])[] | select(.visible != true or .width < 40 or .height < 40 or .border_radius < 16 or .avatar_prismatic_rim_ready != true or (.avatar_rim_drop_shadow_count // 0) < 2 or .box_shadow == "none")
		        ],
		        bad_active_chat_readability: [
		          (.active_chat_readability_details // [])[] | select(.readable != true)
		        ],
		        bad_placeholder_readability: [
		          (.placeholder_readability_details // [])[] | select(.readable != true)
		        ],
		        bad_small_control_readability: [
		          (.small_control_readability_details // [])[] | select(.readable != true)
		        ],
		        visible_text_integrity_probe
	      }
    ]
  }' <<<"$density_qa_json" >&2 || true
  exit 1
fi

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg output_dir "$OUT_DIR" \
  --argjson telegram_live_send_enabled "$telegram_live_send_enabled" \
  --argjson native_post_real_activation_enabled "$native_post_real_activation_enabled" \
  --arg logo_dimensions "$logo_dimensions" \
  --arg logo_sha "$(shasum -a 256 "$logo_png" | awk '{print $1}')" \
  --arg desktop_sha "$(shasum -a 256 "$OUT_DIR/desktop.png" | awk '{print $1}')" \
  --arg narrow_sha "$(shasum -a 256 "$OUT_DIR/narrow.png" | awk '{print $1}')" \
  --arg mobile_sha "$(shasum -a 256 "$OUT_DIR/mobile.png" | awk '{print $1}')" \
  --arg phone320_sha "$(shasum -a 256 "$OUT_DIR/phone320.png" | awk '{print $1}')" \
  --argjson phone320_bytes "$(wc -c <"$OUT_DIR/phone320.png" | tr -d ' ')" \
  --argjson density_qa "$density_qa_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    output_dir:$output_dir,
    browser:"playwright-system-chrome",
    checked_text:[
      "data-rust-frontend-renderer=\"hepta-core::control_ui\"",
      "data-no-js-frontend=\"true\"",
      "data-telegram-multi-agent-chat=\"true\"",
      "data-control-ui-product-first=\"true\"",
      "data-control-ui-primary-path=\"telegram-chat-shell\"",
      "data-control-ui-telegram-shell=\"true\"",
      "data-control-ui-top-design-referee=\"liquid-glass-2026-wcag22-320-reflow\"",
      "data-control-ui-harsh-referee=\"2026-06-08-liquid-glass-menus-sidebars-scroll-search\"",
      "data-control-ui-secondary-map=\"collapsed\"",
      "data-control-ui-runtime-rail=\"local-review-safety-evidence\"",
      "data-control-ui-secondary-nav=\"collapsed\"",
      "data-control-ui-composer-product-first=\"true\"",
      "data-mobile-compact-composer=\"true\"",
      "data-control-ui-composer-more=\"collapsed\"",
      "data-control-ui-composer-tools-trigger=\"light-glass\"",
      "data-control-ui-composer-tools-panel=\"light-glass\"",
      "data-control-ui-composer-popover-panel=\"light-glass\"",
      "data-control-ui-topbar-action=\"light-glass\"",
      "data-control-ui-micro-surface",
      "data-control-ui-thread-intro-badge",
      "data-control-ui-status-trust-badge",
      "data-control-ui-work-rail=\"product-first\"",
      "data-control-ui-compact-product-path=\"narrow-mobile\"",
      "data-control-ui-thread-tools-trigger=\"light-glass\"",
      "data-control-ui-thread-tools-panel=\"light-glass\"",
      "data-open-command-palette",
      "id=\"command-palette\"",
      "data-control-ui-command-palette-surface=\"light-glass\"",
      "data-control-ui-command-palette-input=\"light-glass\"",
      "data-control-ui-command-palette-close=\"light-glass\"",
      "data-control-ui-command-palette-result=\"light-glass\""
    ],
    control_ui_product_first_ready:true,
    control_ui_primary_path:"telegram-chat-shell",
    control_ui_telegram_shell_ready:true,
    control_ui_dashboard_cards_hidden:true,
    control_ui_secondary_map_collapsed:true,
    control_ui_runtime_rail_product_first_ready:true,
    control_ui_secondary_nav_collapsed:true,
    control_ui_composer_product_first_ready:true,
    control_ui_work_rail_product_first_ready:true,
    control_ui_compact_product_path_ready:true,
    control_ui_engineering_copy_hidden:true,
    control_ui_top_design_referee_ready:true,
    control_ui_320_reflow_ready:$density_qa.phone320_ready,
    control_ui_default_submenus_closed_ready:$density_qa.default_submenus_closed_ready,
    control_ui_single_submenu_audit_ready:$density_qa.single_submenu_audit_ready,
    control_ui_engineering_status_chips_suppressed_ready:$density_qa.engineering_session_chips_suppressed_ready,
    control_ui_preferred_touch_targets_ready:$density_qa.preferred_touch_targets_ready,
    control_ui_glass_action_contract_ready:$density_qa.control_glass_action_ready,
    control_ui_harsh_2026_ready:$density_qa.harsh_referee_ready,
    control_ui_rail_action_icon_ready:$density_qa.rail_action_icon_ready,
    control_ui_icon_buttons_ready:$density_qa.icon_button_ready,
    control_ui_icon_prismatic_control_light_glass_ready:$density_qa.icon_prismatic_control_light_glass_ready,
    control_ui_topbar_action_light_glass_ready:$density_qa.topbar_action_light_glass_ready,
    control_ui_primary_shell_light_glass_ready:$density_qa.primary_shell_light_glass_ready,
	    control_ui_translucent_shell_light_glass_ready:$density_qa.translucent_shell_light_glass_ready,
	    control_ui_refractive_depth_light_glass_ready:$density_qa.refractive_depth_light_glass_ready,
		    control_ui_optical_clarity_light_glass_ready:$density_qa.optical_clarity_light_glass_ready,
		    control_ui_surface_clear_alpha_light_glass_ready:$density_qa.surface_clear_alpha_light_glass_ready,
		    control_ui_substrate_caustic_field_light_glass_ready:$density_qa.substrate_caustic_field_light_glass_ready,
		    control_ui_specular_edge_light_glass_ready:$density_qa.specular_edge_light_glass_ready,
		    control_ui_prismatic_dispersion_light_glass_ready:$density_qa.prismatic_dispersion_light_glass_ready,
		    control_ui_caustic_highlight_light_glass_ready:$density_qa.caustic_highlight_light_glass_ready,
		    control_ui_caustic_depth_shift_light_glass_ready:$density_qa.caustic_depth_shift_light_glass_ready,
			    control_ui_optical_thickness_tiers_light_glass_ready:$density_qa.optical_thickness_tiers_light_glass_ready,
			    control_ui_faceted_reflection_light_glass_ready:$density_qa.faceted_reflection_light_glass_ready,
			    control_ui_beveled_rim_light_glass_ready:$density_qa.beveled_rim_light_glass_ready,
			    control_ui_micro_refraction_light_glass_ready:$density_qa.micro_refraction_light_glass_ready,
			    control_ui_sparkle_glint_light_glass_ready:$density_qa.sparkle_glint_light_glass_ready,
			    control_ui_lens_bloom_light_glass_ready:$density_qa.lens_bloom_light_glass_ready,
			    control_ui_spectral_fusion_light_glass_ready:$density_qa.spectral_fusion_light_glass_ready,
			    control_ui_optical_magnification_light_glass_ready:$density_qa.optical_magnification_light_glass_ready,
			    control_ui_biaxial_magnification_light_glass_ready:$density_qa.biaxial_magnification_light_glass_ready,
			    control_ui_anisotropic_magnification_light_glass_ready:$density_qa.anisotropic_magnification_light_glass_ready,
			    control_ui_phase_separated_refraction_light_glass_ready:$density_qa.phase_separated_refraction_light_glass_ready,
			    control_ui_two_axis_phase_refraction_light_glass_ready:$density_qa.two_axis_phase_refraction_light_glass_ready,
			    control_ui_surface_phase_drift_light_glass_ready:$density_qa.surface_phase_drift_light_glass_ready,
			    control_ui_surface_lens_scale_drift_light_glass_ready:$density_qa.surface_lens_scale_drift_light_glass_ready,
			    control_ui_layer_scale_parallax_light_glass_ready:$density_qa.layer_scale_parallax_light_glass_ready,
			    control_ui_surface_spectral_angle_drift_light_glass_ready:$density_qa.surface_spectral_angle_drift_light_glass_ready,
			    control_ui_surface_glint_focal_drift_light_glass_ready:$density_qa.surface_glint_focal_drift_light_glass_ready,
			    control_ui_composer_glint_focal_decoupling_light_glass_ready:$density_qa.composer_glint_focal_decoupling_light_glass_ready,
			    control_ui_composer_spectral_angle_decoupling_light_glass_ready:$density_qa.composer_spectral_angle_decoupling_light_glass_ready,
			    control_ui_composer_phase_decoupling_light_glass_ready:$density_qa.composer_phase_decoupling_light_glass_ready,
			    control_ui_composer_layer_scale_decoupling_light_glass_ready:$density_qa.composer_layer_scale_decoupling_light_glass_ready,
			    control_ui_chrome_bar_translucency_light_glass_ready:$density_qa.chrome_bar_translucency_light_glass_ready,
			    control_ui_chrome_refractive_skin_light_glass_ready:$density_qa.chrome_refractive_skin_light_glass_ready,
			    control_ui_clear_white_balance_light_glass_ready:$density_qa.clear_white_balance_light_glass_ready,
			    control_ui_chamfer_cut_edge_light_glass_ready:$density_qa.chamfer_cut_edge_light_glass_ready,
			    control_ui_prismatic_cut_edge_light_glass_ready:$density_qa.prismatic_cut_edge_light_glass_ready,
			    control_ui_pane_prismatic_perimeter_light_glass_ready:$density_qa.pane_prismatic_perimeter_light_glass_ready,
			    control_ui_composer_prismatic_control_light_glass_ready:$density_qa.composer_prismatic_control_light_glass_ready,
		    control_ui_menu_triggers_ready:$density_qa.menu_trigger_ready,
	    control_ui_folder_chip_touch_ready:$density_qa.folder_chip_touch_ready,
	    control_ui_folder_chip_label_prismatic_etch_light_glass_ready:$density_qa.folder_chip_label_prismatic_etch_light_glass_ready,
	    control_ui_row_menu_touch_ready:$density_qa.row_menu_touch_ready,
	    control_ui_row_menu_all_rows_ready:$density_qa.row_menu_all_rows_ready,
		    control_ui_row_menu_light_glass_ready:$density_qa.row_menu_light_glass_ready,
	    control_ui_command_palette_ready:$density_qa.command_palette_ready,
	    control_ui_command_palette_surface_light_glass_ready:$density_qa.command_palette_surface_light_glass_ready,
	    control_ui_command_palette_surface_prismatic_perimeter_light_glass_ready:$density_qa.command_palette_surface_prismatic_perimeter_light_glass_ready,
	    control_ui_command_palette_backdrop_caustic_veil_light_glass_ready:$density_qa.command_palette_backdrop_caustic_veil_light_glass_ready,
	    control_ui_command_palette_trigger_light_glass_ready:$density_qa.command_palette_trigger_light_glass_ready,
	    control_ui_command_palette_close_light_glass_ready:$density_qa.command_palette_close_light_glass_ready,
	    control_ui_command_palette_close_prismatic_icon_light_glass_ready:$density_qa.command_palette_close_prismatic_icon_light_glass_ready,
	    control_ui_command_palette_input_light_glass_ready:$density_qa.command_palette_input_light_glass_ready,
			    control_ui_command_palette_input_text_prismatic_etch_light_glass_ready:$density_qa.command_palette_input_text_prismatic_etch_light_glass_ready,
			    control_ui_command_palette_input_placeholder_prismatic_etch_light_glass_ready:$density_qa.command_palette_input_placeholder_prismatic_etch_light_glass_ready,
			    control_ui_command_palette_input_row_prismatic_separator_light_glass_ready:$density_qa.command_palette_input_row_prismatic_separator_light_glass_ready,
			    control_ui_command_palette_results_well_light_glass_ready:$density_qa.command_palette_results_well_light_glass_ready,
			    control_ui_command_palette_results_well_prismatic_rim_light_glass_ready:$density_qa.command_palette_results_well_prismatic_rim_light_glass_ready,
			    control_ui_command_palette_input_icon_light_glass_ready:$density_qa.command_palette_input_icon_light_glass_ready,
			    control_ui_command_palette_input_icon_prismatic_light_glass_ready:$density_qa.command_palette_input_icon_prismatic_light_glass_ready,
			    control_ui_command_palette_item_light_glass_ready:$density_qa.command_palette_item_light_glass_ready,
			    control_ui_command_palette_item_prismatic_rim_light_glass_ready:$density_qa.command_palette_item_prismatic_rim_light_glass_ready,
			    control_ui_command_palette_kind_chip_light_glass_ready:$density_qa.command_palette_kind_chip_light_glass_ready,
			    control_ui_command_palette_item_hover_prismatic_light_glass_ready:$density_qa.command_palette_item_hover_prismatic_light_glass_ready,
			    control_ui_command_palette_item_label_prismatic_etch_light_glass_ready:$density_qa.command_palette_item_label_prismatic_etch_light_glass_ready,
		    control_ui_form_control_title_touch_ready:$density_qa.control_form_control_title_touch_ready,
		    control_ui_chat_row_option_semantic_touch_ready:$density_qa.chat_row_option_semantic_touch_ready,
		    control_ui_rail_chat_row_prismatic_slab_light_glass_ready:$density_qa.rail_chat_row_prismatic_slab_light_glass_ready,
		    control_ui_icon_button_title_match_ready:$density_qa.icon_button_title_match_ready,
    control_ui_menu_trigger_title_match_ready:$density_qa.menu_trigger_title_match_ready,
    control_ui_menu_item_icons_ready:$density_qa.menu_item_icon_ready,
    control_ui_menu_surfaces_ready:$density_qa.menu_surface_ready,
    control_ui_thread_tools_menu_ready:$density_qa.thread_tools_menu_ready,
    control_ui_composer_tools_menu_ready:$density_qa.composer_tools_menu_ready,
    control_ui_composer_tools_trigger_light_glass_ready:$density_qa.composer_tools_trigger_light_glass_ready,
    control_ui_composer_popover_ready:$density_qa.composer_popover_ready,
    control_ui_composer_popover_item_label_prismatic_etch_light_glass_ready:$density_qa.composer_popover_item_label_prismatic_etch_light_glass_ready,
    control_ui_composer_popover_header_prismatic_etch_light_glass_ready:$density_qa.composer_popover_header_prismatic_etch_light_glass_ready,
    control_ui_composer_popover_search_light_glass_ready:$density_qa.composer_popover_search_light_glass_ready,
    control_ui_composer_popover_search_placeholder_prismatic_etch_light_glass_ready:$density_qa.composer_popover_search_placeholder_prismatic_etch_light_glass_ready,
    control_ui_rail_search_light_glass_ready:$density_qa.rail_search_light_glass_ready,
    control_ui_rail_search_placeholder_prismatic_etch_light_glass_ready:$density_qa.rail_search_placeholder_prismatic_etch_light_glass_ready,
    control_ui_rail_prismatic_filter_light_glass_ready:$density_qa.rail_prismatic_filter_light_glass_ready,
    control_ui_micro_surface_light_glass_ready:$density_qa.micro_surface_light_glass_ready,
    control_ui_micro_prismatic_badge_light_glass_ready:$density_qa.micro_prismatic_badge_light_glass_ready,
    control_ui_micro_badge_label_prismatic_etch_light_glass_ready:$density_qa.micro_badge_label_prismatic_etch_light_glass_ready,
    control_ui_message_metadata_prismatic_light_glass_ready:$density_qa.message_metadata_prismatic_light_glass_ready,
    control_ui_thread_subtitle_prismatic_light_glass_ready:$density_qa.thread_subtitle_prismatic_light_glass_ready,
	    control_ui_composer_shortcut_hint_prismatic_light_glass_ready:$density_qa.composer_shortcut_hint_prismatic_light_glass_ready,
	    control_ui_rail_metadata_chip_prismatic_light_glass_ready:$density_qa.rail_metadata_chip_prismatic_light_glass_ready,
	    control_ui_rail_status_count_prismatic_light_glass_ready:$density_qa.rail_status_count_prismatic_light_glass_ready,
	    control_ui_rail_preview_prismatic_etch_light_glass_ready:$density_qa.rail_preview_prismatic_etch_light_glass_ready,
	    control_ui_rail_chat_title_prismatic_etch_light_glass_ready:$density_qa.rail_chat_title_prismatic_etch_light_glass_ready,
	    control_ui_message_body_prismatic_etch_light_glass_ready:$density_qa.message_body_prismatic_etch_light_glass_ready,
	    control_ui_message_speaker_prismatic_chip_light_glass_ready:$density_qa.message_speaker_prismatic_chip_light_glass_ready,
	    control_ui_composer_placeholder_prismatic_etch_light_glass_ready:$density_qa.composer_placeholder_prismatic_etch_light_glass_ready,
	    control_ui_header_title_prismatic_etch_light_glass_ready:$density_qa.header_title_prismatic_etch_light_glass_ready,
    control_ui_message_routing_badge_light_glass_ready:$density_qa.message_routing_badge_light_glass_ready,
    control_ui_thread_intro_badge_light_glass_ready:$density_qa.thread_intro_badge_light_glass_ready,
    control_ui_status_trust_strip_light_glass_ready:$density_qa.status_trust_strip_light_glass_ready,
    control_ui_menu_surface_viewport_guard_ready:$density_qa.menu_surface_viewport_guard_ready,
    control_ui_navigation_icons_ready:$density_qa.nav_icon_ready,
	    control_ui_scroll_edge_ready:$density_qa.scroll_edge_ready,
	    control_ui_microcopy_word_split_guard_ready:$density_qa.microcopy_word_split_guard_ready,
	    control_ui_logo_clip_guard_ready:$density_qa.logo_clip_guard_ready,
	    control_ui_avatar_prismatic_rim_light_glass_ready:$density_qa.avatar_prismatic_rim_light_glass_ready,
	    control_ui_active_chat_readability_ready:$density_qa.active_chat_readability_ready,
	    control_ui_placeholder_readability_ready:$density_qa.placeholder_readability_ready,
	    control_ui_small_control_readability_ready:$density_qa.small_control_readability_ready,
	    control_ui_visible_text_integrity_ready:$density_qa.visible_text_integrity_ready,
	    control_ui_visual_density_qa_ready:true,
    control_ui_browser_error_page_absent:true,
    control_ui_horizontal_overflow_free:true,
    density_qa:$density_qa,
    checked_assets:[
		      {path:"/styles.css", markers:[".tg-conversation-rail",".tg-thread-panel",".command-palette","safe-area-inset-bottom","mrog","data-control-ui-compact-product-path","data-control-ui-primary-shell-light-glass","crs","cwb","cce","pce","ppe","cpe","mpb","ipc","avr","rpf","rcs","mmp","tsp","csh","rms","hte","rsc","rpe","mbp","bsp","rsp","fcp","strong){filter","--x:0 1px #fff6","text-shadow:var(--x)","rdlg","oclg","data-control-ui-tspcfrg","dsc","mecs","cmv","ctlg","cplg","cpsg","rmlg","ttlg","bmslg","mslg","tiblg","stslg","talg","body[data-view=chat] .hepta-secondary-map{display:none}","gar26","cps","cpis","cpt","cpc","cpir","cph","cprw","cprr","cpkc","cpilg","data-control-ui-command-palette-input=light-glass","data-control-ui-command-palette-result=light-glass"]},
      {path:"/assets/hepta-agent-logo.png", dimensions:$logo_dimensions, sha256:$logo_sha}
    ],
    telegram_live_send_enabled:$telegram_live_send_enabled,
    native_post_real_activation_enabled:$native_post_real_activation_enabled,
    screenshots:[
      {name:"desktop", viewport:"1365x900", sha256:$desktop_sha},
      {name:"narrow", viewport:"768x900", sha256:$narrow_sha},
      {name:"mobile", viewport:"500x844", sha256:$mobile_sha},
      {name:"phone320", viewport:"320x844", sha256:$phone320_sha, bytes:$phone320_bytes, path:($output_dir + "/phone320.png")}
    ],
    side_effects:{
      telegram_read:false,
      telegram_send:false,
      native_post_real_mutation:false,
      provider_invoked:false
    }
  }')"

printf '%s\n' "$report"

if [[ -n "$REPORT_PATH" ]]; then
  mkdir -p "$(dirname "$REPORT_PATH")"
  printf '%s\n' "$report" >"$REPORT_PATH"
fi

echo "Hepta browser visual smoke passed"
