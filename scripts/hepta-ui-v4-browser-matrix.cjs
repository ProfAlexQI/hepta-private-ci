"use strict";

const { spawn, execFileSync } = require("node:child_process");
const crypto = require("node:crypto");
const fs = require("node:fs");
const fsPromises = require("node:fs/promises");
const os = require("node:os");
const path = require("node:path");

const [chromeBin, baseUrl, outputDirectory] = process.argv.slice(2);
if (!chromeBin || !baseUrl || !outputDirectory) {
  console.error(
    "usage: node scripts/hepta-ui-v4-browser-matrix.cjs <chrome-bin> <base-url> <output-dir>",
  );
  process.exit(64);
}

const scenarios = [
  { id: "web-chrome-320-light", width: 320, height: 800, dpr: 1, sheet: "thread" },
  { id: "web-chrome-390-keyboard", width: 390, height: 844, dpr: 1, sheet: "thread" },
  { id: "web-chrome-412-tools-sheet", width: 412, height: 915, dpr: 1, sheet: "composer" },
  { id: "web-chrome-600-tablet", width: 600, height: 960, dpr: 1, sheet: null },
  { id: "web-chrome-768-dpr2", width: 768, height: 1024, dpr: 2, sheet: null },
  { id: "web-chrome-980-compact", width: 980, height: 800, dpr: 1, sheet: null },
  { id: "web-chrome-1280-desktop", width: 1280, height: 800, dpr: 1, sheet: null },
  { id: "web-chrome-1440-wide", width: 1440, height: 900, dpr: 1, sheet: null },
];

const sleep = (milliseconds) => new Promise((resolve) => setTimeout(resolve, milliseconds));

async function waitFor(condition, timeoutMilliseconds, label) {
  const deadline = Date.now() + timeoutMilliseconds;
  while (Date.now() < deadline) {
    const value = condition();
    if (value) return value;
    await sleep(50);
  }
  throw new Error(`Timed out waiting for ${label}`);
}

function gitValue(args) {
  try {
    return execFileSync("git", args, { encoding: "utf8" }).trim();
  } catch (_error) {
    return "UNAVAILABLE";
  }
}

function sha256File(filePath) {
  return crypto.createHash("sha256").update(fs.readFileSync(filePath)).digest("hex");
}

async function inspectScenario(scenario) {
  const profileDirectory = await fsPromises.mkdtemp(
    path.join(os.tmpdir(), `hepta-ui-v4-${scenario.id}-`),
  );
  const chrome = spawn(
    chromeBin,
    [
      "--headless=new",
      "--disable-background-networking",
      "--disable-component-update",
      "--disable-default-apps",
      "--disable-extensions",
      "--disable-sync",
      "--hide-scrollbars",
      "--no-default-browser-check",
      "--no-first-run",
      "--no-sandbox",
      "--remote-debugging-port=0",
      `--user-data-dir=${path.join(profileDirectory, "profile")}`,
      `--window-size=${scenario.width},${scenario.height}`,
      "about:blank",
    ],
    { stdio: ["ignore", "ignore", "pipe"] },
  );

  let browserWebSocketUrl = "";
  chrome.stderr.setEncoding("utf8");
  chrome.stderr.on("data", (chunk) => {
    const match = chunk.match(/DevTools listening on (ws:\/\/[^\s]+)/);
    if (match) browserWebSocketUrl = match[1];
  });

  const requests = [];
  const consoleErrors = [];
  let webSocket;
  try {
    browserWebSocketUrl = await waitFor(
      () => browserWebSocketUrl,
      10000,
      `${scenario.id} Chrome DevTools endpoint`,
    );
    const browserEndpoint = new URL(browserWebSocketUrl);
    const targets = await (await fetch(`http://${browserEndpoint.host}/json/list`)).json();
    const pageTarget = targets.find((target) => target.type === "page");
    if (!pageTarget?.webSocketDebuggerUrl) {
      throw new Error(`Chrome page target unavailable for ${scenario.id}`);
    }

    webSocket = new WebSocket(pageTarget.webSocketDebuggerUrl);
    await new Promise((resolve, reject) => {
      webSocket.onopen = resolve;
      webSocket.onerror = reject;
    });

    let requestId = 0;
    const pending = new Map();
    webSocket.onmessage = (event) => {
      const message = JSON.parse(event.data);
      if (message.id) {
        const waiter = pending.get(message.id);
        if (!waiter) return;
        pending.delete(message.id);
        if (message.error) waiter.reject(new Error(JSON.stringify(message.error)));
        else waiter.resolve(message.result);
        return;
      }
      if (message.method === "Network.requestWillBeSent") {
        requests.push({
          url: message.params.request.url,
          method: message.params.request.method,
          type: message.params.type,
        });
      }
      if (message.method === "Runtime.exceptionThrown") {
        consoleErrors.push(
          message.params.exceptionDetails.exception?.description ||
            message.params.exceptionDetails.text ||
            "runtime exception",
        );
      }
      if (
        message.method === "Runtime.consoleAPICalled" &&
        message.params.type === "error"
      ) {
        consoleErrors.push(
          message.params.args.map((entry) => entry.value || entry.description || "").join(" "),
        );
      }
    };

    const send = (method, params = {}) => {
      const id = ++requestId;
      webSocket.send(JSON.stringify({ id, method, params }));
      return new Promise((resolve, reject) => pending.set(id, { resolve, reject }));
    };

    await send("Page.enable");
    await send("Runtime.enable");
    await send("Network.enable");
    await send("Emulation.setDeviceMetricsOverride", {
      width: scenario.width,
      height: scenario.height,
      deviceScaleFactor: scenario.dpr,
      mobile: scenario.width <= 700,
      screenWidth: scenario.width,
      screenHeight: scenario.height,
    });
    await send("Emulation.setEmulatedMedia", {
      media: "screen",
      features: [
        { name: "prefers-color-scheme", value: "light" },
        { name: "prefers-reduced-motion", value: "no-preference" },
      ],
    });

    const navigation = await send("Page.navigate", { url: baseUrl });
    if (navigation.errorText) throw new Error(navigation.errorText);
    await sleep(1100);

    const evaluation = await send("Runtime.evaluate", {
      expression: `(async () => {
        const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
        const visible = (node) => {
          if (!(node instanceof HTMLElement) || node.hidden) return false;
          const style = getComputedStyle(node);
          const rect = node.getBoundingClientRect();
          return style.display !== "none" && style.visibility !== "hidden" && rect.width > 0 && rect.height > 0;
        };
        const px = (value) => Number.parseFloat(value || "0") || 0;
        const directText = (node) => [...node.childNodes].some((child) => child.nodeType === Node.TEXT_NODE && child.textContent.trim());
        const bodyStyle = getComputedStyle(document.body);
        const thread = document.querySelector(".tg-thread-panel");
        const bubble = document.querySelector(".tg-bubble");
        const message = document.querySelector(".tg-message");
        const metadata = document.querySelector(".tg-message small");
        const keyControls = [...document.querySelectorAll([
          ".tg-mobile-topbar__back",
          ".tg-mobile-topbar__title",
          ".tg-mobile-topbar__detail",
          ".tg-compose-icon",
          ".tg-send-button",
          "[data-control-ui-menu-trigger=\\"icon\\"]"
        ].join(","))].filter(visible);
        const targetFloor = innerWidth <= 700 ? 48 : 44;
        const controlFailures = keyControls.filter((node) => node.getBoundingClientRect().height + 0.01 < targetFloor).map((node) => ({
          selector: node.className,
          height: node.getBoundingClientRect().height,
          floor: targetFloor,
        }));
        const stableNodes = [thread, bubble].filter(visible);
        const stableBlurFailures = stableNodes.filter((node) => {
          const style = getComputedStyle(node);
          return !["none", "blur(0px)"].includes(style.backdropFilter || style.webkitBackdropFilter || "none");
        }).map((node) => ({ className: node.className, backdropFilter: getComputedStyle(node).backdropFilter }));
        const under12 = [...document.body.querySelectorAll("*")].filter((node) =>
          visible(node) && directText(node) && px(getComputedStyle(node).fontSize) < 12
        ).slice(0, 20).map((node) => ({ tag: node.tagName, className: node.className, fontSize: getComputedStyle(node).fontSize }));

        const runtimeReady = document.documentElement.dataset.heptaUiV4Runtime === "ready";
        const runtimeNetworkNone = document.documentElement.dataset.heptaUiV4RuntimeNetwork === "none";
        const readApiReady = Boolean(window.HeptaUiV4ReadState);
        let readStateReady = false;
        if (readApiReady && thread) {
          window.HeptaUiV4ReadState.set(thread, "stale", { message: "Source snapshot is stale." });
          readStateReady = thread.dataset.heptaV4ReadState === "stale" && Boolean(thread.querySelector("[data-hepta-v4-read-state-status]"));
          window.HeptaUiV4ReadState.clear(thread);
        }

        let sheet = {
          requested: ${JSON.stringify(scenario.sheet)},
          opened: false,
          mobileMarker: false,
          scrollLocked: false,
          focusContained: false,
          tabWrapped: false,
          escapeDismissed: false,
          focusRestored: false,
        };
        if (${JSON.stringify(Boolean(scenario.sheet))}) {
          const id = ${JSON.stringify(scenario.sheet === "composer" ? "composer-tools-popover" : "thread-tools-popover")};
          const trigger = document.querySelector('[popovertarget="' + id + '"]');
          const popover = document.getElementById(id);
          if (trigger && popover && typeof popover.showPopover === "function") {
            trigger.focus();
            trigger.click();
            await sleep(100);
            sheet.opened = popover.matches(":popover-open");
            sheet.mobileMarker = popover.dataset.heptaV4MobileSheet === "true";
            sheet.scrollLocked = document.documentElement.dataset.heptaV4SheetOpen === "true";
            sheet.focusContained = popover.contains(document.activeElement);
            const focusables = [...popover.querySelectorAll("a[href],button:not([disabled]),input:not([disabled]),select:not([disabled]),textarea:not([disabled]),[tabindex]:not([tabindex=\\"-1\\"])")].filter(visible);
            if (focusables.length > 1) {
              const first = focusables[0];
              const last = focusables[focusables.length - 1];
              last.focus();
              document.dispatchEvent(new KeyboardEvent("keydown", { key: "Tab", bubbles: true, cancelable: true }));
              sheet.tabWrapped = document.activeElement === first;
            } else {
              sheet.tabWrapped = focusables.length === 1;
            }
            document.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true, cancelable: true }));
            await sleep(100);
            sheet.escapeDismissed = !popover.matches(":popover-open");
            sheet.focusRestored = document.activeElement === trigger;
          }
        }

        return {
          href: location.href,
          viewport: { width: innerWidth, height: innerHeight, dpr: devicePixelRatio },
          progressiveEnhancementReady: document.documentElement.dataset.controlUiProgressiveEnhancement === "ready",
          capabilityMode: document.documentElement.dataset.controlUiCapabilityMode,
          liveAdapterBound: document.documentElement.dataset.controlUiLiveAdapterBound === "true",
          runtimeReady,
          runtimeNetworkNone,
          readApiReady,
          readStateReady,
          horizontalOverflow: document.documentElement.scrollWidth > innerWidth + 1,
          bodyFontPx: px(bodyStyle.fontSize),
          messageFontPx: message ? px(getComputedStyle(message).fontSize) : 0,
          metadataFontPx: metadata ? px(getComputedStyle(metadata).fontSize) : 0,
          controlFailures,
          stableBlurFailures,
          under12,
          openTransientCount: document.querySelectorAll("[popover]:popover-open").length,
          sheet,
        };
      })()`,
      awaitPromise: true,
      returnByValue: true,
    });

    if (evaluation.exceptionDetails) {
      throw new Error(
        evaluation.exceptionDetails.exception?.description ||
          evaluation.exceptionDetails.text ||
          "runtime evaluation failed",
      );
    }
    const metrics = evaluation.result?.value;
    if (!metrics) throw new Error("runtime evaluation produced no value");

    const screenshotResult = await send("Page.captureScreenshot", {
      format: "png",
      captureBeyondViewport: false,
      fromSurface: true,
    });
    const screenshotPath = path.join(outputDirectory, `${scenario.id}.png`);
    fs.writeFileSync(screenshotPath, Buffer.from(screenshotResult.data, "base64"));

    const origin = new URL(baseUrl).origin;
    const relevantRequests = requests.filter((entry) => /^https?:/.test(entry.url));
    const crossOriginRequests = relevantRequests.filter(
      (entry) => new URL(entry.url).origin !== origin,
    );
    const nonGetRequests = relevantRequests.filter((entry) => entry.method !== "GET");
    const failures = [];
    const requireCheck = (condition, id, detail) => {
      if (!condition) failures.push(`${id}:${detail}`);
    };
    requireCheck(metrics.progressiveEnhancementReady, "progressive_enhancement", "not ready");
    requireCheck(metrics.capabilityMode === "local-read-only", "capability_mode", metrics.capabilityMode);
    requireCheck(!metrics.liveAdapterBound, "live_adapter", "unexpectedly bound");
    requireCheck(metrics.runtimeReady, "v4_runtime", "not bound into served asset");
    requireCheck(metrics.runtimeNetworkNone, "runtime_network", "network ownership is not none");
    requireCheck(metrics.readApiReady && metrics.readStateReady, "read_state", "runtime API not ready");
    requireCheck(!metrics.horizontalOverflow, "horizontal_overflow", "document exceeds viewport");
    requireCheck(metrics.bodyFontPx >= 15, "body_font", metrics.bodyFontPx);
    requireCheck(metrics.messageFontPx >= 15, "message_font", metrics.messageFontPx);
    requireCheck(metrics.metadataFontPx >= 12, "metadata_font", metrics.metadataFontPx);
    requireCheck(metrics.controlFailures.length === 0, "touch_targets", JSON.stringify(metrics.controlFailures));
    requireCheck(metrics.stableBlurFailures.length === 0, "stable_blur", JSON.stringify(metrics.stableBlurFailures));
    requireCheck(metrics.under12.length === 0, "visible_text_floor", JSON.stringify(metrics.under12));
    requireCheck(metrics.openTransientCount <= 1, "transient_budget", metrics.openTransientCount);
    if (scenario.sheet) {
      for (const [key, value] of Object.entries(metrics.sheet)) {
        if (key !== "requested") requireCheck(value === true, `sheet_${key}`, JSON.stringify(metrics.sheet));
      }
    }
    requireCheck(crossOriginRequests.length === 0, "cross_origin", JSON.stringify(crossOriginRequests));
    requireCheck(nonGetRequests.length === 0, "non_get", JSON.stringify(nonGetRequests));
    requireCheck(consoleErrors.length === 0, "console_errors", JSON.stringify(consoleErrors));

    return {
      id: scenario.id,
      status: failures.length === 0 ? "PASS" : "FAIL",
      viewport: `${scenario.width}x${scenario.height}`,
      deviceScaleFactor: scenario.dpr,
      screenshot: {
        path: screenshotPath,
        bytes: fs.statSync(screenshotPath).size,
        sha256: sha256File(screenshotPath),
      },
      metrics,
      requestAudit: {
        totalHttpRequests: relevantRequests.length,
        crossOriginRequests,
        nonGetRequests,
      },
      consoleErrors,
      failures,
    };
  } finally {
    try {
      webSocket?.close();
    } catch (_error) {}
    if (chrome.exitCode === null) chrome.kill("SIGTERM");
    await Promise.race([
      new Promise((resolve) => chrome.once("exit", resolve)),
      sleep(1500).then(() => {
        if (chrome.exitCode === null) chrome.kill("SIGKILL");
      }),
    ]);
    await fsPromises.rm(profileDirectory, {
      recursive: true,
      force: true,
      maxRetries: 5,
      retryDelay: 100,
    });
  }
}

(async () => {
  await fsPromises.mkdir(outputDirectory, { recursive: true });
  const results = [];
  for (const scenario of scenarios) {
    try {
      results.push(await inspectScenario(scenario));
    } catch (error) {
      results.push({
        id: scenario.id,
        status: "FAIL",
        viewport: `${scenario.width}x${scenario.height}`,
        deviceScaleFactor: scenario.dpr,
        failures: [String(error?.stack || error).slice(0, 2000)],
      });
    }
  }

  const candidateCommit = process.env.GITHUB_SHA || gitValue(["rev-parse", "HEAD"]);
  const candidateTree = gitValue(["rev-parse", "HEAD^{tree}"]);
  const materialPath = path.resolve("design-tokens/hepta-material-v4.contract.json");
  const runtimePath = path.resolve("apps/hepta-control-ui/control-ui-v4-runtime.js");
  const receipt = {
    schema: "hepta.ui.v4.browser-qualification.v1",
    status: results.every((result) => result.status === "PASS")
      ? "PASS_BROWSER_MATRIX"
      : "FAIL_BROWSER_MATRIX",
    scope: "SOURCE_STATIC_SERVER_BROWSER_ONLY",
    producer: "scripts/hepta-ui-v4-browser-matrix.cjs",
    candidate: {
      commit: candidateCommit,
      tree: candidateTree,
      branch: process.env.GITHUB_REF_NAME || "UNAVAILABLE",
    },
    baseUrl,
    chromeExecutable: chromeBin,
    chromeExecutableSha256: sha256File(chromeBin),
    materialContractSha256: fs.existsSync(materialPath) ? sha256File(materialPath) : "MISSING",
    runtimeSourceSha256: fs.existsSync(runtimePath) ? sha256File(runtimePath) : "MISSING",
    resultCount: results.length,
    passedCount: results.filter((result) => result.status === "PASS").length,
    failedCount: results.filter((result) => result.status !== "PASS").length,
    results,
    authority: {
      mutation: false,
      effect: false,
      production: false,
      operatorAcceptance: false,
      promotion: false,
      release: false,
    },
    claims: {
      rustServedAssetValidated: false,
      nativeCompileValidated: false,
      deviceValidated: false,
      platformSystemMaterialValidated: false,
    },
  };

  const receiptPath = path.join(outputDirectory, "hepta-ui-v4-browser-qualification.json");
  fs.writeFileSync(receiptPath, `${JSON.stringify(receipt, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify(receipt)}\n`);
  if (receipt.status !== "PASS_BROWSER_MATRIX") process.exit(1);
})().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
