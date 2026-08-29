#!/usr/bin/env node
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const fsPromises = require("node:fs/promises");
const path = require("node:path");
const { chromium, firefox, webkit } = require("playwright");

const baseUrl = process.env.HEPTA_UI_BASE_URL;
const engine = process.env.HEPTA_BROWSER_ENGINE;
const engineKey = process.env.HEPTA_BROWSER_ENGINE_KEY;
const outputPath = path.resolve(process.env.HEPTA_BROWSER_RECEIPT);
const screenshotPath = path.resolve(process.env.HEPTA_BROWSER_SCREENSHOT);
const candidateCommit = process.env.HEPTA_CANDIDATE_SHA;
const candidateTree = process.env.HEPTA_CANDIDATE_TREE;
const auditorCommit = process.env.GITHUB_SHA || "";

function sha256(bytes) {
  return crypto.createHash("sha256").update(bytes).digest("hex");
}

function fail(message) {
  throw new Error(message);
}

async function launchBrowser() {
  if (engine === "edge") {
    return chromium.launch({ channel: "msedge", headless: true });
  }
  const type = { chromium, firefox, webkit }[engine];
  if (!type) fail(`unsupported engine: ${engine}`);
  return type.launch({ headless: true });
}

async function main() {
  if (!baseUrl || !engine || !engineKey || !candidateCommit || !candidateTree) {
    fail("missing browser probe environment");
  }
  await fsPromises.mkdir(path.dirname(outputPath), { recursive: true });
  await fsPromises.mkdir(path.dirname(screenshotPath), { recursive: true });

  const failures = [];
  let screenshotSha256 = null;
  let servedJavascriptSha256 = null;
  let consoleErrorCount = 0;
  let pageErrorCount = 0;
  let crossOriginRequestCount = 0;
  let nonGetRequestCount = 0;
  let audit = {};
  let browser;

  try {
    const scriptResponse = await fetch(`${baseUrl}/control-ui.js`, {
      method: "GET",
      headers: { accept: "text/javascript" },
    });
    if (!scriptResponse.ok) fail(`control-ui.js status ${scriptResponse.status}`);
    const scriptBytes = Buffer.from(await scriptResponse.arrayBuffer());
    if (scriptBytes.length < 1024) fail("control-ui.js unexpectedly small");
    servedJavascriptSha256 = sha256(scriptBytes);

    browser = await launchBrowser();
    const context = await browser.newContext({
      viewport: { width: 1280, height: 800 },
      colorScheme: "light",
      reducedMotion: "no-preference",
      locale: "en-CA",
    });
    const page = await context.newPage();
    const origin = new URL(baseUrl).origin;

    page.on("console", (message) => {
      if (message.type() === "error") {
        consoleErrorCount += 1;
        failures.push(`console:${message.text().slice(0, 240)}`);
      }
    });
    page.on("pageerror", (error) => {
      pageErrorCount += 1;
      failures.push(`pageerror:${String(error.message).slice(0, 240)}`);
    });
    page.on("request", (request) => {
      const url = new URL(request.url());
      if (url.origin !== origin) crossOriginRequestCount += 1;
      if (!["GET", "HEAD"].includes(request.method())) nonGetRequestCount += 1;
    });

    await page.goto(`${baseUrl}/#chat-thread`, { waitUntil: "domcontentloaded" });
    await page.waitForFunction(
      () => document.documentElement.dataset.controlUiV4Runtime === "ready",
      undefined,
      { timeout: 20_000 },
    );
    await page.waitForTimeout(750);

    audit = await page.evaluate(async () => {
      const apiResponse = await fetch("/api/ui-contract-audit", { method: "GET" });
      const apiPayload = await apiResponse.json();
      const root = document.documentElement;
      const bodyStyle = getComputedStyle(document.body);
      const openTransients = [...document.querySelectorAll("[popover]")].filter((node) =>
        node.matches(":popover-open"),
      ).length;
      return {
        rustRenderedDocument: root.dataset.rustFrontendRenderer === "hepta-core::control_ui",
        rustRuntimeReady: root.dataset.controlUiV4Runtime === "ready",
        liveAdapterUnbound: root.dataset.controlUiLiveAdapterBound === "false",
        sameOriginReadOnly: root.dataset.progressiveEnhancement === "same-origin-read-only",
        apiReady: apiResponse.ok && apiPayload.status === "ready" && apiPayload.fixture === false,
        stylesheetCount: document.styleSheets.length,
        bodyFontFamily: bodyStyle.fontFamily,
        horizontalOverflow: root.scrollWidth - root.clientWidth,
        openTransients,
        mainVisible: Boolean(document.querySelector("main")?.getClientRects().length),
        composerVisible: Boolean(document.querySelector(".tg-compose-wrap")?.getClientRects().length),
      };
    });

    if (!audit.rustRenderedDocument) failures.push("rust_rendered_document_missing");
    if (!audit.rustRuntimeReady) failures.push("rust_runtime_not_ready");
    if (!audit.liveAdapterUnbound) failures.push("live_adapter_boundary_drift");
    if (!audit.sameOriginReadOnly) failures.push("read_only_boundary_drift");
    if (!audit.apiReady) failures.push("rust_served_api_not_ready");
    if (!(audit.stylesheetCount > 0)) failures.push("stylesheet_not_loaded");
    if (!audit.mainVisible || !audit.composerVisible) failures.push("primary_ui_not_visible");
    if (audit.horizontalOverflow > 1) failures.push(`horizontal_overflow:${audit.horizontalOverflow}`);
    if (audit.openTransients > 1) failures.push(`transient_budget:${audit.openTransients}`);
    if (crossOriginRequestCount !== 0) failures.push(`cross_origin_requests:${crossOriginRequestCount}`);
    if (nonGetRequestCount !== 0) failures.push(`non_get_requests:${nonGetRequestCount}`);

    await page.screenshot({ path: screenshotPath, fullPage: false });
    screenshotSha256 = sha256(await fsPromises.readFile(screenshotPath));
    await context.close();
  } catch (error) {
    failures.push(`probe:${String(error && error.stack ? error.stack : error).slice(0, 1000)}`);
  } finally {
    if (browser) await browser.close().catch(() => {});
  }

  const pass = failures.length === 0;
  const receipt = {
    schema: "hepta.ui.v4.rust-served-browser-engine.v1",
    status: pass ? "PASS_RUST_SERVED_BROWSER_ENGINE" : "FAIL_RUST_SERVED_BROWSER_ENGINE",
    candidate: { commit: candidateCommit, tree: candidateTree },
    auditor: { commit: auditorCommit },
    candidateBound: true,
    synthetic: false,
    fixture: false,
    engine: engineKey,
    playwrightEngine: engine,
    rustServedRuntime: pass,
    screenshotSha256,
    servedJavascriptSha256,
    consoleErrorCount,
    pageErrorCount,
    crossOriginRequestCount,
    nonGetRequestCount,
    networkAuditPassed: pass && crossOriginRequestCount === 0 && nonGetRequestCount === 0,
    audit,
    boundary: {
      productWired: false,
      productCargoFeatureDeclared: false,
      productModuleRegistered: false,
      productLifecycleWired: false,
      automaticBindingAllowed: false,
      productHostMayBind: false,
      productBound: false,
      systemMaterialBound: false,
      nativeProductRuntime: false,
      deviceValidation: false,
    },
    authority: {
      network: false,
      mutation: false,
      effect: false,
      liveAdapter: false,
      production: false,
      operatorAcceptance: false,
      promotion: false,
      release: false,
    },
    failures,
  };
  await fsPromises.writeFile(outputPath, `${JSON.stringify(receipt, null, 2)}\n`);
  process.stdout.write(`${JSON.stringify(receipt)}\n`);
  if (!pass) process.exitCode = 1;
}

main().catch(async (error) => {
  const fallback = {
    schema: "hepta.ui.v4.rust-served-browser-engine.v1",
    status: "FAIL_RUST_SERVED_BROWSER_ENGINE",
    candidate: { commit: candidateCommit || "", tree: candidateTree || "" },
    auditor: { commit: auditorCommit },
    candidateBound: false,
    synthetic: false,
    fixture: false,
    engine: engineKey || engine || "unknown",
    screenshotSha256: null,
    consoleErrorCount: 0,
    pageErrorCount: 0,
    crossOriginRequestCount: 0,
    nonGetRequestCount: 0,
    networkAuditPassed: false,
    failures: [String(error && error.stack ? error.stack : error).slice(0, 1000)],
  };
  await fsPromises.mkdir(path.dirname(outputPath), { recursive: true }).catch(() => {});
  await fsPromises.writeFile(outputPath, `${JSON.stringify(fallback, null, 2)}\n`).catch(() => {});
  console.error(error);
  process.exitCode = 1;
});
