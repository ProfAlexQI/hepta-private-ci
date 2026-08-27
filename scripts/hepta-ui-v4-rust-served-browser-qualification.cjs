#!/usr/bin/env node
"use strict";

const crypto = require("node:crypto");
const fs = require("node:fs");
const fsPromises = require("node:fs/promises");
const os = require("node:os");
const path = require("node:path");
const { spawnSync } = require("node:child_process");
const { chromium } = require("playwright");

const ROOT = path.resolve(__dirname, "..");
const OUT_DIR = path.resolve(
  process.env.HEPTA_UI_V4_RUST_SERVED_BROWSER_OUT
    || path.join(os.tmpdir(), "hepta-ui-v4-rust-served-browser"),
);
const BROWSER_PATH = process.env.HEPTA_CHROME_BIN || "";
const MAX_ASSET_BYTES = 4 * 1024 * 1024;
const BUNDLE_BOUNDARY = Buffer.from("\n/* hepta-ui-v4-runtime-bundle-boundary */\n");
const BASE_URL = normalizeBaseUrl(process.argv[2] || process.env.HEPTA_LIVE_URL || "");

const SCENARIOS = Object.freeze([
  { id: "chrome-320-thread", width: 320, height: 800, dpr: 1, hash: "#chat-thread" },
  {
    id: "chrome-390-ime-simulated",
    width: 390,
    height: 560,
    dpr: 1,
    hash: "#chat-thread",
    focusComposer: true,
  },
  {
    id: "chrome-412-tools-sheet",
    width: 412,
    height: 915,
    dpr: 1,
    hash: "#chat-thread",
    exerciseTransient: true,
  },
  { id: "chrome-600-chats", width: 600, height: 960, dpr: 1, hash: "#chat-list" },
  {
    id: "chrome-768-font200-simulated",
    width: 768,
    height: 1024,
    dpr: 1,
    hash: "#chat-thread",
    fontScale: 2,
  },
  { id: "chrome-980-compact", width: 980, height: 800, dpr: 1, hash: "#chat-thread" },
  { id: "chrome-1280-desktop", width: 1280, height: 800, dpr: 1, hash: "#chat-thread" },
  {
    id: "chrome-1440-reduced-transparency",
    width: 1440,
    height: 900,
    dpr: 1,
    hash: "#chat-thread",
    mediaFeatures: [{ name: "prefers-reduced-transparency", value: "reduce" }],
  },
  {
    id: "chrome-1440-forced-colors",
    width: 1440,
    height: 900,
    dpr: 1,
    hash: "#chat-thread",
    mediaFeatures: [{ name: "forced-colors", value: "active" }],
  },
  {
    id: "chrome-390-reduced-motion",
    width: 390,
    height: 844,
    dpr: 1,
    hash: "#chat-thread",
    mediaFeatures: [{ name: "prefers-reduced-motion", value: "reduce" }],
  },
]);

function sha256(bytes) {
  return crypto.createHash("sha256").update(bytes).digest("hex");
}

function gitValue(...args) {
  const result = spawnSync("git", args, { cwd: ROOT, encoding: "utf8" });
  return result.status === 0 ? result.stdout.trim() : "";
}

function normalizeBaseUrl(value) {
  if (!value) throw new Error("Rust-served qualification requires a loopback base URL");
  const url = new URL(value);
  if (url.protocol !== "http:") throw new Error("Rust-served qualification requires loopback HTTP");
  if (!["127.0.0.1", "localhost", "[::1]", "::1"].includes(url.hostname)) {
    throw new Error("Rust-served qualification refuses a non-loopback origin");
  }
  url.pathname = "/";
  url.search = "";
  url.hash = "";
  return url.toString().replace(/\/$/, "");
}

async function boundedGet(relativePath) {
  const url = new URL(relativePath, `${BASE_URL}/`);
  if (url.origin !== new URL(BASE_URL).origin) throw new Error(`cross-origin asset URL: ${url}`);
  const response = await fetch(url, {
    method: "GET",
    redirect: "manual",
    headers: { Accept: "*/*" },
  });
  const declared = Number(response.headers.get("content-length") || "0");
  if (Number.isFinite(declared) && declared > MAX_ASSET_BYTES) {
    throw new Error(`asset exceeds declared byte bound: ${relativePath}`);
  }
  const bytes = Buffer.from(await response.arrayBuffer());
  if (bytes.length > MAX_ASSET_BYTES) throw new Error(`asset exceeds byte bound: ${relativePath}`);
  return { response, bytes };
}

async function verifyServedAssets() {
  const baseSource = await fsPromises.readFile(
    path.join(ROOT, "apps", "hepta-control-ui", "control-ui.js"),
  );
  const runtimeSource = await fsPromises.readFile(
    path.join(ROOT, "apps", "hepta-control-ui", "control-ui-v4-runtime.js"),
  );
  const expectedBundle = Buffer.concat([baseSource, BUNDLE_BOUNDARY, runtimeSource]);
  const root = await boundedGet("/");
  const script = await boundedGet("/control-ui.js");
  const sideRoute = await boundedGet("/control-ui-v4-runtime.js");

  const failures = [];
  const rootText = root.bytes.toString("utf8");
  const scriptText = script.bytes.toString("utf8");
  const expectedSha = sha256(expectedBundle);
  const servedSha = sha256(script.bytes);
  const etag = script.response.headers.get("etag") || "";

  if (!root.response.ok) failures.push(`root_status:${root.response.status}`);
  if (!rootText.includes('defer src="./control-ui.js"')) failures.push("root_script_path_missing");
  if (!script.response.ok) failures.push(`script_status:${script.response.status}`);
  if (!script.response.headers.get("content-type")?.startsWith("text/javascript")) {
    failures.push("script_content_type_invalid");
  }
  if (!script.bytes.equals(expectedBundle)) failures.push("served_bundle_bytes_mismatch");
  if (servedSha !== expectedSha) failures.push("served_bundle_digest_mismatch");
  if (etag !== `"sha256-${servedSha}"`) failures.push("served_bundle_etag_mismatch");
  if (sideRoute.response.ok) failures.push("runtime_side_route_must_not_exist");

  for (const marker of [
    "const COMMAND_CATALOG = Object.freeze([",
    "hepta-ui-v4-runtime-bundle-boundary",
    "HeptaUiV4ReadState",
    'controlUiV4Runtime = "ready"',
    'controlUiV4RuntimeAuthority = "local-ui-only"',
  ]) {
    if (!scriptText.includes(marker)) failures.push(`served_bundle_marker_missing:${marker}`);
  }
  for (const forbidden of ["innerHTML", "eval(", "new Function(", "http://", "https://"]) {
    if (scriptText.includes(forbidden)) failures.push(`served_bundle_forbidden_capability:${forbidden}`);
  }

  return {
    failures,
    root: {
      status: root.response.status,
      sha256: sha256(root.bytes),
      bytes: root.bytes.length,
    },
    script: {
      path: "/control-ui.js",
      status: script.response.status,
      contentType: script.response.headers.get("content-type") || "",
      cacheControl: script.response.headers.get("cache-control") || "",
      etag,
      baseSha256: sha256(baseSource),
      runtimeSha256: sha256(runtimeSource),
      expectedBundleSha256: expectedSha,
      servedBundleSha256: servedSha,
      bytes: script.bytes.length,
      exactBytesBound: script.bytes.equals(expectedBundle),
      etagBound: etag === `"sha256-${servedSha}"`,
      runtimeBound: scriptText.includes("HeptaUiV4ReadState"),
      singleServedPath: true,
    },
    runtimeSideRouteStatus: sideRoute.response.status,
  };
}

async function emulateMediaFeatures(page, features = []) {
  if (features.length === 0) return;
  const session = await page.context().newCDPSession(page);
  await session.send("Emulation.setEmulatedMedia", {
    media: "screen",
    features: [{ name: "prefers-color-scheme", value: "light" }, ...features],
  });
}

async function exerciseTransient(page) {
  const failures = [];
  const trigger = page.locator('[data-control-ui-composer-tools-trigger="light-glass"]');
  await trigger.click();
  await page.waitForFunction(() =>
    document.querySelector("#composer-tools-popover")?.matches(":popover-open"),
  );
  const scrollLocked = await page.evaluate(() =>
    document.documentElement.dataset.heptaV4TransientOpen === "true"
      && document.body.dataset.heptaV4ScrollLock === "true",
  );
  await page.keyboard.press("Escape");
  await page.waitForFunction(() =>
    !document.querySelector("#composer-tools-popover")?.matches(":popover-open"),
  );
  const escapeFocusRestored = await trigger.evaluate((node) => document.activeElement === node);

  await trigger.click();
  await page.waitForFunction(() =>
    document.querySelector("#composer-tools-popover")?.matches(":popover-open"),
  );
  await page.evaluate(() => history.back());
  await page.waitForFunction(() =>
    !document.querySelector("#composer-tools-popover")?.matches(":popover-open"),
  );
  const backFocusRestored = await trigger.evaluate((node) => document.activeElement === node);

  if (!scrollLocked) failures.push("mobile_sheet_scroll_lock_missing");
  if (!escapeFocusRestored) failures.push("escape_focus_not_restored");
  if (!backFocusRestored) failures.push("back_focus_not_restored");
  return { scrollLocked, escapeFocusRestored, backFocusRestored, failures };
}

async function inspectPage(page, scenario) {
  const failures = [];
  const network = { crossOrigin: 0, nonGet: 0, requests: 0 };
  const expectedOrigin = new URL(BASE_URL).origin;
  page.on("request", (request) => {
    network.requests += 1;
    const url = new URL(request.url());
    if (["http:", "https:"].includes(url.protocol) && url.origin !== expectedOrigin) {
      network.crossOrigin += 1;
    }
    if (!["GET", "HEAD"].includes(request.method())) network.nonGet += 1;
  });
  page.on("pageerror", (error) => failures.push(`pageerror:${String(error.message).slice(0, 180)}`));
  page.on("console", (message) => {
    if (message.type() === "error") failures.push(`console:${message.text().slice(0, 180)}`);
  });

  await emulateMediaFeatures(page, scenario.mediaFeatures || []);
  await page.goto(`${BASE_URL}/${scenario.hash}`, { waitUntil: "networkidle" });
  await page.waitForFunction(() =>
    document.documentElement.dataset.controlUiV4Runtime === "ready"
      && document.documentElement.dataset.controlUiV4RuntimeAuthority === "local-ui-only",
  );

  if (scenario.fontScale) {
    await page.evaluate((scale) => {
      document.documentElement.style.fontSize = `${scale * 100}%`;
      document.documentElement.dataset.heptaQualificationFontScale = String(scale);
    }, scenario.fontScale);
    await page.waitForTimeout(100);
  }
  if (scenario.focusComposer) await page.focus("#chat-message");

  const transient = scenario.exerciseTransient
    ? await exerciseTransient(page)
    : null;
  if (transient) failures.push(...transient.failures);

  const readState = await page.evaluate(() => {
    const target = document.createElement("section");
    target.hidden = true;
    target.innerHTML = '<span data-control-ui-read-state-message></span>';
    document.body.append(target);
    let eventDetail = null;
    target.addEventListener("hepta:ui-v4-read-state", (event) => {
      eventDetail = event.detail;
    }, { once: true });
    const result = globalThis.HeptaUiV4ReadState.apply(target, "stale", {
      source: "qualification",
      freshness: "bounded",
      evidenceDigest: "a".repeat(64),
      message: "Stale qualification state",
    });
    const snapshot = {
      result,
      current: globalThis.HeptaUiV4ReadState.current(target),
      dataState: target.dataset.controlUiReadState,
      evidenceDigest: target.dataset.controlUiEvidenceDigest,
      ariaBusy: target.getAttribute("aria-busy"),
      eventDetail,
    };
    target.remove();
    return snapshot;
  });
  if (readState.current !== "stale" || readState.dataState !== "stale") {
    failures.push("read_state_projection_failed");
  }
  if (readState.evidenceDigest !== "a".repeat(64)) failures.push("read_state_digest_failed");

  const audit = await page.evaluate(() => {
    const visible = (node) => {
      if (!(node instanceof HTMLElement) || node.hidden || node.getClientRects().length === 0) return false;
      const style = getComputedStyle(node);
      return style.visibility !== "hidden" && Number.parseFloat(style.opacity || "1") > 0.01;
    };
    const stable = [".tg-thread-panel", ".tg-bubble", "input", "textarea", "select"]
      .flatMap((selector) => [...document.querySelectorAll(selector)])
      .filter(visible);
    const stableBlurViolations = stable.filter((node) => {
      const style = getComputedStyle(node);
      return !["none", ""].includes(style.backdropFilter)
        || !["none", ""].includes(style.webkitBackdropFilter);
    }).length;
    const textNodes = [...document.querySelectorAll("body *")].filter((node) => {
      if (!visible(node) || node.children.length > 0 || !node.textContent.trim()) return false;
      if (node.matches(".sr-only,[aria-hidden='true']")) return false;
      const style = getComputedStyle(node);
      return style.clip === "auto" && style.clipPath === "none";
    });
    const visibleUnder12 = textNodes.filter((node) =>
      Number.parseFloat(getComputedStyle(node).fontSize) < 12,
    ).length;
    const controls = [...document.querySelectorAll("button,a[href],summary,input,textarea,select")]
      .filter(visible);
    const targetViolations = innerWidth <= 700
      ? controls.filter((node) => {
          const box = node.getBoundingClientRect();
          return box.width + 0.01 < 48 || box.height + 0.01 < 48;
        }).length
      : 0;
    const openTransients = [...document.querySelectorAll("[popover]")]
      .filter((node) => node.matches(":popover-open")).length;
    const composer = document.querySelector(".tg-compose-wrap")?.getBoundingClientRect();
    const active = document.activeElement?.getBoundingClientRect?.();
    const focusNotObscured = !active
      || (active.bottom > 0 && active.top < innerHeight && active.right > 0 && active.left < innerWidth);
    const reducedTransparencyActive = matchMedia("(prefers-reduced-transparency: reduce)").matches;
    const forcedColorsActive = matchMedia("(forced-colors: active)").matches;
    const chrome = [...document.querySelectorAll(
      ".tg-conversation-rail,.tg-compose-bar,.tg-mobile-layer-tabs,[popover]",
    )].filter(visible);
    const transparencyFallback = reducedTransparencyActive || forcedColorsActive
      ? chrome.every((node) => {
          const style = getComputedStyle(node);
          return ["none", ""].includes(style.backdropFilter)
            && ["none", ""].includes(style.webkitBackdropFilter);
        })
      : null;
    return {
      runtimeReady: document.documentElement.dataset.controlUiV4Runtime === "ready",
      runtimeAuthority: document.documentElement.dataset.controlUiV4RuntimeAuthority,
      horizontalOverflow: document.documentElement.scrollWidth - document.documentElement.clientWidth,
      stableBlurViolations,
      visibleUnder12,
      targetViolations,
      openTransients,
      composerVisible: Boolean(composer && composer.bottom > 0 && composer.top < innerHeight),
      focusNotObscured,
      reducedTransparencyActive,
      forcedColorsActive,
      transparencyFallback,
      fontScale: document.documentElement.dataset.heptaQualificationFontScale || "1",
    };
  });

  if (!audit.runtimeReady || audit.runtimeAuthority !== "local-ui-only") {
    failures.push("runtime_controller_not_ready_or_authority_invalid");
  }
  if (audit.horizontalOverflow > 1) failures.push(`horizontal_overflow:${audit.horizontalOverflow}`);
  if (audit.stableBlurViolations !== 0) failures.push(`stable_blur_violations:${audit.stableBlurViolations}`);
  if (audit.visibleUnder12 !== 0) failures.push(`visible_text_under_12:${audit.visibleUnder12}`);
  if (audit.targetViolations !== 0) failures.push(`mobile_target_violations:${audit.targetViolations}`);
  if (audit.openTransients > 1) failures.push(`transient_layer_budget:${audit.openTransients}`);
  if (!audit.composerVisible && scenario.hash === "#chat-thread") failures.push("composer_obscured");
  if (!audit.focusNotObscured) failures.push("focus_obscured");
  if ((audit.reducedTransparencyActive || audit.forcedColorsActive)
      && audit.transparencyFallback !== true) {
    failures.push("solid_fallback_failed");
  }
  if (network.crossOrigin !== 0) failures.push(`cross_origin_requests:${network.crossOrigin}`);
  if (network.nonGet !== 0) failures.push(`non_get_requests:${network.nonGet}`);

  const screenshotPath = path.join(OUT_DIR, `${scenario.id}.png`);
  await page.screenshot({ path: screenshotPath, fullPage: false });
  const screenshotBytes = await fsPromises.readFile(screenshotPath);
  return {
    id: scenario.id,
    viewport: `${scenario.width}x${scenario.height}`,
    deviceScaleFactor: scenario.dpr,
    fontScaleSimulation: scenario.fontScale || 1,
    fixtureRuntimeInjected: false,
    rustServedRuntimeBound: true,
    transient,
    readState,
    audit,
    network,
    screenshot: {
      path: path.relative(ROOT, screenshotPath),
      bytes: screenshotBytes.length,
      sha256: sha256(screenshotBytes),
    },
    status: failures.length === 0
      ? "PASS_RUST_SERVED_BROWSER_CONTRACT"
      : "FAIL_RUST_SERVED_BROWSER_CONTRACT",
    failures,
  };
}

async function main() {
  await fsPromises.rm(OUT_DIR, { recursive: true, force: true });
  await fsPromises.mkdir(OUT_DIR, { recursive: true });
  const candidateCommit = process.env.HEPTA_CANDIDATE_COMMIT || gitValue("rev-parse", "HEAD");
  const candidateTree = process.env.HEPTA_CANDIDATE_TREE || gitValue("rev-parse", "HEAD^{tree}");
  const servedAssets = await verifyServedAssets();

  let browser;
  try {
    browser = await chromium.launch({
      headless: true,
      ...(BROWSER_PATH ? { executablePath: BROWSER_PATH } : {}),
      args: [
        "--disable-background-networking",
        "--disable-component-update",
        "--disable-default-apps",
        "--disable-extensions",
        "--disable-sync",
        "--no-default-browser-check",
        "--no-first-run",
      ],
    });
    const results = [];
    for (const scenario of SCENARIOS) {
      const context = await browser.newContext({
        viewport: { width: scenario.width, height: scenario.height },
        deviceScaleFactor: scenario.dpr,
        colorScheme: "light",
        reducedMotion: scenario.mediaFeatures?.some(
          (feature) => feature.name === "prefers-reduced-motion" && feature.value === "reduce",
        ) ? "reduce" : "no-preference",
      });
      const page = await context.newPage();
      results.push(await inspectPage(page, scenario));
      await context.close();
    }

    const failures = [
      ...servedAssets.failures.map((failure) => `served-assets:${failure}`),
      ...results.flatMap((result) => result.failures.map((failure) => `${result.id}:${failure}`)),
    ];
    const receipt = {
      schema: "hepta.ui.v4.rust-served-browser-qualification.v1",
      status: failures.length === 0
        ? "PASS_RUST_SERVED_BROWSER_CONTRACT"
        : "FAIL_RUST_SERVED_BROWSER_CONTRACT",
      scope: "RUST_SERVED_LOOPBACK_LOCAL_READ_ONLY",
      candidate: {
        commit: candidateCommit,
        tree: candidateTree,
        commitBound: /^[0-9a-f]{40}$/.test(candidateCommit),
        treeBound: /^[0-9a-f]{40}$/.test(candidateTree),
      },
      server: {
        baseUrl: BASE_URL,
        loopback: true,
        runtime: "hepta",
        servedAssets,
      },
      browser: {
        name: "Chromium",
        version: await browser.version(),
        executablePath: BROWSER_PATH || "playwright-managed",
      },
      fixture: false,
      runtimeAssetInjectedForQualification: false,
      rustServedRuntimeAssetBound: servedAssets.script.runtimeBound
        && servedAssets.script.exactBytesBound
        && servedAssets.script.etagBound,
      browserValidation: true,
      rustRuntimeValidation: true,
      deviceValidation: false,
      productionAuthority: false,
      effectAuthority: false,
      liveAdapterAuthority: false,
      operatorAcceptance: false,
      promotion: false,
      release: false,
      results,
      failures,
    };
    const receiptPath = path.join(
      OUT_DIR,
      "HEPTA_UI_V4_RUST_SERVED_BROWSER_QUALIFICATION_RECEIPT.json",
    );
    await fsPromises.writeFile(receiptPath, `${JSON.stringify(receipt, null, 2)}\n`);
    process.stdout.write(`${JSON.stringify(receipt)}\n`);
    if (failures.length > 0) process.exitCode = 1;
  } finally {
    if (browser) await browser.close();
  }
}

main().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
