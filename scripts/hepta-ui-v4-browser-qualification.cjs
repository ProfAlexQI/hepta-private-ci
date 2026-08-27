#!/usr/bin/env node
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const fsPromises = require("node:fs/promises");
const http = require("node:http");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const { chromium } = require("playwright");

const ROOT = path.resolve(__dirname, "..");
const UI_ROOT = path.join(ROOT, "apps", "hepta-control-ui");
const OUT_DIR = path.resolve(process.env.HEPTA_UI_V4_BROWSER_OUT || path.join(os.tmpdir(), "hepta-ui-v4-browser"));
const BROWSER_PATH = process.env.HEPTA_CHROME_BIN || "";
const MAX_RESPONSE_BYTES = 2 * 1024 * 1024;

const SCENARIOS = Object.freeze([
  { id: "chrome-320-thread", width: 320, height: 800, dpr: 1, hash: "#chat-thread" },
  { id: "chrome-390-ime-simulated", width: 390, height: 560, dpr: 1, hash: "#chat-thread", focusComposer: true },
  { id: "chrome-412-tools-sheet", width: 412, height: 915, dpr: 1, hash: "#chat-thread", openTools: true },
  { id: "chrome-600-chats", width: 600, height: 960, dpr: 1, hash: "#chat-list" },
  { id: "chrome-768-dpr2", width: 768, height: 1024, dpr: 2, hash: "#chat-thread" },
  { id: "chrome-980-compact", width: 980, height: 800, dpr: 1, hash: "#chat-thread" },
  { id: "chrome-1280-desktop", width: 1280, height: 800, dpr: 1, hash: "#chat-thread" },
  { id: "chrome-1440-reduced-transparency", width: 1440, height: 900, dpr: 1, hash: "#chat-thread", reducedTransparency: true },
]);

function sha256(bytes) {
  return crypto.createHash("sha256").update(bytes).digest("hex");
}

function gitValue(...args) {
  const result = spawnSync("git", args, { cwd: ROOT, encoding: "utf8" });
  return result.status === 0 ? result.stdout.trim() : "";
}

function contentType(filePath) {
  const extension = path.extname(filePath).toLowerCase();
  return {
    ".css": "text/css; charset=utf-8",
    ".html": "text/html; charset=utf-8",
    ".js": "text/javascript; charset=utf-8",
    ".json": "application/json; charset=utf-8",
    ".png": "image/png",
    ".svg": "image/svg+xml",
  }[extension] || "application/octet-stream";
}

function safeAssetPath(urlPath) {
  const decoded = decodeURIComponent(urlPath.split("?", 1)[0]);
  const relative = decoded === "/" ? "index.html" : decoded.replace(/^\/+/, "");
  const candidate = path.resolve(UI_ROOT, relative);
  return candidate === UI_ROOT || candidate.startsWith(`${UI_ROOT}${path.sep}`) ? candidate : null;
}

function writeJson(response, status, payload) {
  const bytes = Buffer.from(JSON.stringify(payload));
  response.writeHead(status, {
    "content-type": "application/json; charset=utf-8",
    "content-length": String(bytes.length),
    "cache-control": "no-store",
  });
  response.end(bytes);
}

async function startFixtureServer() {
  const server = http.createServer(async (request, response) => {
    response.setHeader("content-security-policy", "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; connect-src 'self'; object-src 'none'; form-action 'none'; base-uri 'none'");
    response.setHeader("x-content-type-options", "nosniff");
    if (!request.url || !["GET", "HEAD"].includes(request.method || "")) {
      writeJson(response, 405, { status: "denied", reason: "fixture_get_only" });
      return;
    }
    const pathname = new URL(request.url, "http://127.0.0.1").pathname;
    if (pathname === "/api/operator-snapshot") {
      writeJson(response, 200, { status: "ready", source: "ui-v4-fixture", freshness: "fixture", data: {} });
      return;
    }
    if (pathname.startsWith("/api/")) {
      writeJson(response, 200, { status: "ready", source_path: pathname, data: {}, fixture: true });
      return;
    }
    const assetPath = safeAssetPath(pathname);
    if (!assetPath) {
      writeJson(response, 400, { status: "denied", reason: "unsafe_path" });
      return;
    }
    try {
      const stat = await fsPromises.stat(assetPath);
      if (!stat.isFile() || stat.size > MAX_RESPONSE_BYTES) throw new Error("asset_unavailable");
      const bytes = await fsPromises.readFile(assetPath);
      response.writeHead(200, {
        "content-type": contentType(assetPath),
        "content-length": String(bytes.length),
        "cache-control": "no-store",
      });
      if (request.method === "HEAD") response.end();
      else response.end(bytes);
    } catch (_error) {
      writeJson(response, 404, { status: "missing" });
    }
  });
  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", resolve);
  });
  const address = server.address();
  if (!address || typeof address === "string") throw new Error("fixture_server_address_unavailable");
  return { server, baseUrl: `http://127.0.0.1:${address.port}` };
}

async function emulateReducedTransparency(page) {
  const session = await page.context().newCDPSession(page);
  await session.send("Emulation.setEmulatedMedia", {
    media: "screen",
    features: [
      { name: "prefers-color-scheme", value: "light" },
      { name: "prefers-reduced-transparency", value: "reduce" },
    ],
  });
}

async function inspectPage(page, scenario, baseUrl) {
  const failures = [];
  const network = { crossOrigin: 0, nonGet: 0 };
  page.on("request", (request) => {
    const url = new URL(request.url());
    if (url.origin !== baseUrl) network.crossOrigin += 1;
    if (![/^GET$/, /^HEAD$/].some((pattern) => pattern.test(request.method()))) network.nonGet += 1;
  });
  page.on("pageerror", (error) => failures.push(`pageerror:${String(error.message).slice(0, 180)}`));
  page.on("console", (message) => {
    if (message.type() === "error") failures.push(`console:${message.text().slice(0, 180)}`);
  });

  if (scenario.reducedTransparency) await emulateReducedTransparency(page);
  await page.goto(`${baseUrl}/${scenario.hash}`, { waitUntil: "networkidle" });
  await page.addScriptTag({ url: `${baseUrl}/control-ui-v4-runtime.js` });
  await page.waitForFunction(() => document.documentElement.dataset.controlUiV4Runtime === "ready");

  if (scenario.focusComposer) await page.focus("#chat-message");

  let runtimeInteraction = null;
  if (scenario.openTools) {
    const trigger = page.locator('[data-control-ui-composer-tools-trigger="light-glass"]');
    await trigger.click();
    await page.waitForFunction(() => document.querySelector("#composer-tools-popover")?.matches(":popover-open"));
    const locked = await page.evaluate(() => document.documentElement.dataset.heptaV4TransientOpen === "true");
    await page.keyboard.press("Escape");
    const closed = await page.evaluate(() => !document.querySelector("#composer-tools-popover")?.matches(":popover-open"));
    const restored = await trigger.evaluate((node) => document.activeElement === node);
    runtimeInteraction = { locked, escapeClosed: closed, focusRestored: restored };
    if (!locked) failures.push("mobile_sheet_scroll_lock_missing");
    if (!closed) failures.push("mobile_sheet_escape_did_not_close");
    if (!restored) failures.push("mobile_sheet_focus_not_restored");
    await trigger.click();
  }

  const audit = await page.evaluate(() => {
    const visible = (node) => node instanceof HTMLElement && node.getClientRects().length > 0 && !node.hidden;
    const stable = [".tg-thread-panel", ".tg-bubble", "input", "textarea", "select"]
      .flatMap((selector) => [...document.querySelectorAll(selector)])
      .filter(visible);
    const stableBlurViolations = stable.filter((node) => {
      const style = getComputedStyle(node);
      return !["none", ""].includes(style.backdropFilter) || !["none", ""].includes(style.webkitBackdropFilter);
    }).length;
    const textNodes = [...document.querySelectorAll("body *")].filter((node) => {
      if (!visible(node) || node.children.length > 0 || !node.textContent.trim()) return false;
      return getComputedStyle(node).visibility !== "hidden";
    });
    const under12 = textNodes.filter((node) => Number.parseFloat(getComputedStyle(node).fontSize) < 12).length;
    const mobileControls = [...document.querySelectorAll("button,a[href],summary,input,textarea,select")].filter(visible);
    const targetViolations = innerWidth <= 700
      ? mobileControls.filter((node) => {
          const box = node.getBoundingClientRect();
          return box.width + 0.01 < 48 || box.height + 0.01 < 48;
        }).length
      : 0;
    const openTransients = [...document.querySelectorAll("[popover]")]
      .filter((node) => node.matches(":popover-open")).length;
    const activePane = document.querySelector('[data-chat-mobile-pane]:not([hidden])');
    const composer = document.querySelector(".tg-compose-wrap")?.getBoundingClientRect();
    return {
      runtimeReady: document.documentElement.dataset.controlUiV4Runtime === "ready",
      horizontalOverflow: document.documentElement.scrollWidth - document.documentElement.clientWidth,
      stableBlurViolations,
      visibleUnder12: under12,
      targetViolations,
      openTransients,
      activePane: activePane?.id || null,
      composerVisible: Boolean(composer && composer.bottom > 0 && composer.top < innerHeight),
      reducedTransparencySolid: matchMedia("(prefers-reduced-transparency: reduce)").matches
        ? [...document.querySelectorAll(".tg-conversation-rail,.tg-compose-bar,[popover]")]
            .filter(visible)
            .every((node) => ["none", ""].includes(getComputedStyle(node).backdropFilter))
        : null,
    };
  });

  if (!audit.runtimeReady) failures.push("runtime_controller_not_ready");
  if (audit.horizontalOverflow > 1) failures.push(`horizontal_overflow:${audit.horizontalOverflow}`);
  if (audit.stableBlurViolations !== 0) failures.push(`stable_blur_violations:${audit.stableBlurViolations}`);
  if (audit.visibleUnder12 !== 0) failures.push(`visible_text_under_12:${audit.visibleUnder12}`);
  if (audit.targetViolations !== 0) failures.push(`mobile_target_violations:${audit.targetViolations}`);
  if (audit.openTransients > 1) failures.push(`transient_layer_budget:${audit.openTransients}`);
  if (!audit.composerVisible && scenario.hash === "#chat-thread") failures.push("composer_obscured");
  if (scenario.reducedTransparency && audit.reducedTransparencySolid !== true) failures.push("reduced_transparency_fallback_failed");
  if (network.crossOrigin !== 0) failures.push(`cross_origin_requests:${network.crossOrigin}`);
  if (network.nonGet !== 0) failures.push(`non_get_requests:${network.nonGet}`);

  const screenshotPath = path.join(OUT_DIR, `${scenario.id}.png`);
  await page.screenshot({ path: screenshotPath, fullPage: false });
  const screenshotBytes = await fsPromises.readFile(screenshotPath);
  return {
    id: scenario.id,
    viewport: `${scenario.width}x${scenario.height}`,
    deviceScaleFactor: scenario.dpr,
    fixtureRuntimeInjected: true,
    rustServedRuntimeBound: false,
    runtimeInteraction,
    audit,
    network,
    screenshot: {
      path: path.relative(ROOT, screenshotPath),
      bytes: screenshotBytes.length,
      sha256: sha256(screenshotBytes),
    },
    status: failures.length === 0 ? "PASS_FIXTURE_VISUAL_CONTRACT_ONLY" : "FAIL_FIXTURE_VISUAL_CONTRACT",
    failures,
  };
}

async function main() {
  await fsPromises.rm(OUT_DIR, { recursive: true, force: true });
  await fsPromises.mkdir(OUT_DIR, { recursive: true });
  const candidateCommit = process.env.HEPTA_CANDIDATE_COMMIT || gitValue("rev-parse", "HEAD");
  const candidateTree = process.env.HEPTA_CANDIDATE_TREE || gitValue("rev-parse", "HEAD^{tree}");
  const { server, baseUrl } = await startFixtureServer();
  let browser;
  try {
    browser = await chromium.launch({
      headless: true,
      ...(BROWSER_PATH ? { executablePath: BROWSER_PATH } : {}),
      args: ["--disable-background-networking", "--disable-extensions", "--disable-sync", "--no-first-run"],
    });
    const results = [];
    for (const scenario of SCENARIOS) {
      const context = await browser.newContext({
        viewport: { width: scenario.width, height: scenario.height },
        deviceScaleFactor: scenario.dpr,
        colorScheme: "light",
        reducedMotion: "no-preference",
      });
      const page = await context.newPage();
      results.push(await inspectPage(page, scenario, baseUrl));
      await context.close();
    }
    const failures = results.flatMap((result) => result.failures.map((failure) => `${result.id}:${failure}`));
    const sourcePaths = [
      "apps/hepta-control-ui/index.html",
      "apps/hepta-control-ui/styles.css",
      "apps/hepta-control-ui/styles.v4.css",
      "apps/hepta-control-ui/styles.v4.runtime.css",
      "apps/hepta-control-ui/styles.accessibility.css",
      "apps/hepta-control-ui/control-ui.js",
      "apps/hepta-control-ui/control-ui-v4-runtime.js",
    ];
    const source = Object.fromEntries(sourcePaths.map((relative) => {
      const bytes = fs.readFileSync(path.join(ROOT, relative));
      return [relative, { bytes: bytes.length, sha256: sha256(bytes) }];
    }));
    const receipt = {
      schema: "hepta.ui.v4.browser-qualification-receipt.v1",
      status: failures.length === 0 ? "PASS_FIXTURE_VISUAL_CONTRACT_ONLY" : "FAIL_FIXTURE_VISUAL_CONTRACT",
      scope: "LOCAL_STATIC_FIXTURE_WITH_SAME_ORIGIN_GET_STUBS",
      candidate: {
        commit: candidateCommit,
        tree: candidateTree,
        commitBound: /^[0-9a-f]{40}$/.test(candidateCommit),
        treeBound: /^[0-9a-f]{40}$/.test(candidateTree),
      },
      browser: { name: "Chromium", version: await browser.version(), executablePath: BROWSER_PATH || "playwright-managed" },
      fixture: true,
      runtimeAssetInjectedForQualification: true,
      rustServedRuntimeAssetBound: false,
      browserValidation: true,
      rustRuntimeValidation: false,
      deviceValidation: false,
      productionAuthority: false,
      effectAuthority: false,
      operatorAcceptance: false,
      promotion: false,
      source,
      results,
      failures,
    };
    const receiptPath = path.join(OUT_DIR, "HEPTA_UI_V4_BROWSER_QUALIFICATION_RECEIPT.json");
    await fsPromises.writeFile(receiptPath, `${JSON.stringify(receipt, null, 2)}\n`);
    process.stdout.write(`${JSON.stringify(receipt, null, 2)}\n`);
    if (failures.length > 0 || !receipt.candidate.commitBound || !receipt.candidate.treeBound) process.exitCode = 1;
  } finally {
    if (browser) await browser.close();
    await new Promise((resolve) => server.close(resolve));
  }
}

main().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
