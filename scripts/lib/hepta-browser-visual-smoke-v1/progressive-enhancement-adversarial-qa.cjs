const { chromium } = require("playwright");

const [chromeBin, baseUrl] = process.argv.slice(2);
const sleep = (milliseconds) => new Promise((resolve) => setTimeout(resolve, milliseconds));

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
      "--no-default-browser-check",
      "--no-first-run",
    ],
  });
  const failures = [];

  async function openCase(configure, initScript) {
    const context = await browser.newContext();
    if (initScript) {
      await context.addInitScript(initScript);
    }
    const page = await context.newPage();
    await page.route("**/api/operator-snapshot", (route) =>
      route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify({ status: "ready" }),
      }),
    );
    await configure(page);
    await page.goto(baseUrl, { waitUntil: "domcontentloaded" });
    await page.waitForFunction(
      () => document.documentElement.dataset.controlUiProgressiveEnhancement === "ready",
      null,
      { timeout: 10000 },
    );
    return { context, page };
  }

  async function clickCommand(page, commandId) {
    await page.evaluate((id) => {
      const button = document.querySelector(
        `[data-command-id="${id}"] [data-run-command="read-only"]`,
      );
      if (!(button instanceof HTMLButtonElement) || button.disabled) {
        throw new Error(`read-only command is not clickable: ${id}`);
      }
      button.click();
    }, commandId);
  }

  async function outputState(page) {
    return page.evaluate(() => {
      const output = document.getElementById("command-runner-output");
      let parsed = null;
      try {
        parsed = JSON.parse(output?.textContent || "null");
      } catch (_error) {
        // Error states are intentionally plain text, but must retain their source path.
      }
      return {
        state: output?.dataset.state || "",
        source_path: output?.dataset.sourcePath || "",
        text: output?.textContent || "",
        parsed,
      };
    });
  }

  let race;
  {
    const { context, page } = await openCase(async (casePage) => {
      await casePage.route("**/api/control-ui", async (route) => {
        await sleep(450);
        try {
          await route.fulfill({
            status: 200,
            contentType: "application/json",
            body: JSON.stringify({ marker: "OLDER_SLOW_CONTROL" }),
          });
        } catch (_error) {
          // The superseded fetch is expected to be aborted by the browser.
        }
      });
      await casePage.route("**/api/config", async (route) => {
        await sleep(20);
        await route.fulfill({
          status: 200,
          contentType: "application/json",
          body: JSON.stringify({ marker: "NEWER_FAST_CONFIG" }),
        });
      });
    });
    await clickCommand(page, "control-ui");
    await sleep(15);
    await clickCommand(page, "config-surface");
    await page.waitForFunction(
      () => document.getElementById("command-runner-output")?.dataset.state === "ready",
      null,
      { timeout: 5000 },
    );
    await sleep(550);
    const output = await outputState(page);
    const oldCardState = await page
      .locator('[data-command-id="control-ui"]')
      .getAttribute("data-command-state");
    race = {
      latest_result_retained:
        output.source_path === "/api/config" &&
        output.parsed?.source_path === "/api/config" &&
        output.parsed?.data?.marker === "NEWER_FAST_CONFIG",
      old_card_state: oldCardState,
      output,
    };
    if (!race.latest_result_retained || oldCardState !== "superseded") {
      failures.push("older slow request overwrote the newer command result");
    }
    await context.close();
  }

  let staleTimeout;
  {
    const initScript = () => {
      const nativeAbort = AbortController.prototype.abort;
      AbortController.prototype.abort = function abortWithAdversarialDelay(reason) {
        if (
          reason?.name === "AbortError" &&
          String(reason.message).includes("Superseded by a newer command") &&
          !window.__heptaIgnoredSupersedeAbort
        ) {
          window.__heptaIgnoredSupersedeAbort = 1;
          return;
        }
        return nativeAbort.call(this, reason);
      };
      const nativeSetTimeout = window.setTimeout.bind(window);
      window.setTimeout = (callback, delay, ...args) =>
        nativeSetTimeout(callback, delay === 8000 ? 140 : delay, ...args);
    };
    const { context, page } = await openCase(async (casePage) => {
      await casePage.route("**/api/control-ui", async (route) => {
        await sleep(420);
        try {
          await route.fulfill({
            status: 200,
            contentType: "application/json",
            body: JSON.stringify({ marker: "TOO_LATE" }),
          });
        } catch (_error) {
          // The accelerated old timeout intentionally cancels this response.
        }
      });
      await casePage.route("**/api/config", (route) =>
        route.fulfill({
          status: 200,
          contentType: "application/json",
          body: JSON.stringify({ marker: "NEWER_SUCCESS" }),
        }),
      );
    }, initScript);
    await clickCommand(page, "control-ui");
    await sleep(15);
    await clickCommand(page, "config-surface");
    await sleep(260);
    const output = await outputState(page);
    const ignoredSupersedeAbort = await page.evaluate(
      () => window.__heptaIgnoredSupersedeAbort || 0,
    );
    staleTimeout = {
      stale_timeout_suppressed:
        ignoredSupersedeAbort === 1 &&
        output.state === "ready" &&
        output.source_path === "/api/config" &&
        output.parsed?.data?.marker === "NEWER_SUCCESS",
      ignored_supersede_abort_count: ignoredSupersedeAbort,
      output,
    };
    if (!staleTimeout.stale_timeout_suppressed) {
      failures.push("stale timeout error overwrote the newer successful result");
    }
    await context.close();
  }

  let oversizedUtf8;
  {
    const oversizedBody = JSON.stringify({ payload: "😀".repeat(600000) });
    const { context, page } = await openCase(async (casePage) => {
      await casePage.route("**/api/control-ui", (route) =>
        route.fulfill({
          status: 200,
          headers: {
            "content-type": "application/json",
            "content-length": "1",
          },
          body: oversizedBody,
        }),
      );
    });
    await clickCommand(page, "control-ui");
    await page.waitForFunction(
      () => document.getElementById("command-runner-output")?.dataset.state === "error",
      null,
      { timeout: 10000 },
    );
    const output = await outputState(page);
    oversizedUtf8 = {
      utf8_bytes: Buffer.byteLength(oversizedBody, "utf8"),
      utf16_code_units: oversizedBody.length,
      rejected:
        output.source_path === "/api/control-ui" &&
        output.text.includes("exceeded the local display limit"),
      output,
    };
    if (!oversizedUtf8.rejected || oversizedUtf8.utf8_bytes <= 2 * 1024 * 1024) {
      failures.push("multi-byte response bypassed the UTF-8 byte limit");
    }
    await context.close();
  }

  let oversizedHeader;
  {
    const { context, page } = await openCase(async (casePage) => {
      await casePage.route("**/api/control-ui", (route) =>
        route.fulfill({
          status: 200,
          headers: {
            "content-type": "application/json",
            "content-length": String(2 * 1024 * 1024 + 1),
          },
          body: "{}",
        }),
      );
    });
    await clickCommand(page, "control-ui");
    await page.waitForFunction(
      () => document.getElementById("command-runner-output")?.dataset.state === "error",
      null,
      { timeout: 5000 },
    );
    const output = await outputState(page);
    oversizedHeader = {
      rejected:
        output.source_path === "/api/control-ui" &&
        output.text.includes("exceeded the local display limit"),
      output,
    };
    if (!oversizedHeader.rejected) {
      failures.push("oversized Content-Length was not rejected before rendering");
    }
    await context.close();
  }

  let noContentLengthStream;
  {
    const initScript = () => {
      const nativeFetch = window.fetch.bind(window);
      window.__heptaNoLengthStream = {
        pull_count: 0,
        total_chunk_count: 20,
        cancelled: false,
        content_length: null,
      };
      window.fetch = (input, init) => {
        const requestUrl = new URL(
          input instanceof Request ? input.url : String(input),
          window.location.href,
        );
        if (requestUrl.pathname !== "/api/control-ui") {
          return nativeFetch(input, init);
        }
        const chunk = new Uint8Array(256 * 1024);
        chunk.fill(0x61);
        const stream = new ReadableStream(
          {
            pull(controller) {
              window.__heptaNoLengthStream.pull_count += 1;
              controller.enqueue(chunk);
              if (
                window.__heptaNoLengthStream.pull_count >=
                window.__heptaNoLengthStream.total_chunk_count
              ) {
                controller.close();
              }
            },
            cancel() {
              window.__heptaNoLengthStream.cancelled = true;
            },
          },
          { highWaterMark: 0 },
        );
        const response = new Response(stream, {
          status: 200,
          headers: { "content-type": "application/json" },
        });
        window.__heptaNoLengthStream.content_length = response.headers.get("content-length");
        return Promise.resolve(response);
      };
    };
    const { context, page } = await openCase(async () => {}, initScript);
    await clickCommand(page, "control-ui");
    await page.waitForFunction(
      () => document.getElementById("command-runner-output")?.dataset.state === "error",
      null,
      { timeout: 5000 },
    );
    const output = await outputState(page);
    const streamState = await page.evaluate(() => window.__heptaNoLengthStream);
    noContentLengthStream = {
      rejected_at_bound:
        output.source_path === "/api/control-ui" &&
        output.text.includes("exceeded the local display limit") &&
        streamState.content_length === null &&
        streamState.cancelled === true &&
        streamState.pull_count < streamState.total_chunk_count &&
        streamState.pull_count <= 9,
      stream_state: streamState,
      output,
    };
    if (!noContentLengthStream.rejected_at_bound) {
      failures.push("no-Content-Length stream was not cancelled at the byte limit");
    }
    await context.close();
  }

  let jsonp;
  {
    const { context, page } = await openCase(async (casePage) => {
      await casePage.route("**/api/control-ui", (route) =>
        route.fulfill({
          status: 200,
          headers: { "content-type": "application/jsonp" },
          body: JSON.stringify({ marker: "INVALID_JSONP" }),
        }),
      );
    });
    await clickCommand(page, "control-ui");
    await page.waitForFunction(
      () => document.getElementById("command-runner-output")?.dataset.state === "error",
      null,
      { timeout: 5000 },
    );
    const output = await outputState(page);
    jsonp = {
      rejected:
        output.source_path === "/api/control-ui" && output.text.includes("Response is not JSON"),
      output,
    };
    if (!jsonp.rejected) {
      failures.push("application/jsonp was accepted as JSON");
    }
    await context.close();
  }

  let structuredJson;
  {
    const { context, page } = await openCase(async (casePage) => {
      await casePage.route("**/api/control-ui", (route) =>
        route.fulfill({
          status: 200,
          headers: { "content-type": "application/problem+json; charset=utf-8" },
          body: JSON.stringify({ marker: "VALID_STRUCTURED_JSON" }),
        }),
      );
    });
    await clickCommand(page, "control-ui");
    await page.waitForFunction(
      () => document.getElementById("command-runner-output")?.dataset.state === "ready",
      null,
      { timeout: 5000 },
    );
    const output = await outputState(page);
    structuredJson = {
      accepted:
        output.source_path === "/api/control-ui" &&
        output.parsed?.source_path === "/api/control-ui" &&
        output.parsed?.data?.marker === "VALID_STRUCTURED_JSON",
      output,
    };
    if (!structuredJson.accepted) {
      failures.push("valid application/*+json media type was rejected");
    }
    await context.close();
  }

  let redirect;
  {
    let externalRequestCount = 0;
    const { context, page } = await openCase(async (casePage) => {
      casePage.on("request", (request) => {
        if (new URL(request.url()).origin !== new URL(baseUrl).origin) {
          externalRequestCount += 1;
        }
      });
      await casePage.route("http://127.0.0.1:9/**", (route) => route.abort());
      await casePage.route("**/api/control-ui", (route) =>
        route.fulfill({
          status: 302,
          headers: {
            location: "http://127.0.0.1:9/redirect-target",
            "content-type": "application/json",
          },
          body: "{}",
        }),
      );
    });
    await clickCommand(page, "control-ui");
    await page.waitForFunction(
      () => document.getElementById("command-runner-output")?.dataset.state === "error",
      null,
      { timeout: 5000 },
    );
    const output = await outputState(page);
    redirect = {
      blocked:
        output.source_path === "/api/control-ui" &&
        output.text.includes("Unable to load /api/control-ui") &&
        externalRequestCount === 0,
      external_request_count: externalRequestCount,
      output,
    };
    if (!redirect.blocked) {
      failures.push("redirect escaped the same-origin read-only boundary");
    }
    await context.close();
  }

  let xss;
  {
    const poison =
      '</p><img id="hepta-xss-probe" src=x onerror="window.__heptaXss=1">' +
      "<script>window.__heptaXss=2</script><svg onload=window.__heptaXss=3>";
    const context = await browser.newContext();
    const page = await context.newPage();
    await page.route("**/api/operator-snapshot", (route) =>
      route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify({ status: poison }),
      }),
    );
    await page.route("**/api/control-ui", (route) =>
      route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify({ poison }),
      }),
    );
    await page.goto(baseUrl, { waitUntil: "domcontentloaded" });
    await page.waitForFunction(
      () => document.documentElement.dataset.controlUiProgressiveEnhancement === "ready",
      null,
      { timeout: 10000 },
    );
    await clickCommand(page, "control-ui");
    await page.waitForFunction(
      () => document.getElementById("command-runner-output")?.dataset.state === "ready",
      null,
      { timeout: 5000 },
    );
    xss = await page.evaluate(() => ({
      blocked:
        !window.__heptaXss &&
        document.querySelectorAll('#hepta-xss-probe, script:not([src])').length === 0 &&
        document.getElementById("command-runner-output")?.children.length === 0,
      execution_marker: window.__heptaXss || 0,
      injected_node_count: document.querySelectorAll('#hepta-xss-probe, script:not([src])').length,
      output_child_count: document.getElementById("command-runner-output")?.children.length || 0,
      output_source_path: document.getElementById("command-runner-output")?.dataset.sourcePath || "",
    }));
    if (!xss.blocked || xss.output_source_path !== "/api/control-ui") {
      failures.push("JSON or snapshot content crossed the text-only DOM boundary");
    }
    await context.close();
  }

  let noScriptProductTruth;
  {
    const context = await browser.newContext();
    const page = await context.newPage();
    let blockedScriptRequestCount = 0;
    let apiRequestCount = 0;
    let nonGetRequestCount = 0;
    page.on("request", (request) => {
      const url = new URL(request.url());
      if (url.pathname === "/control-ui.js") {
        blockedScriptRequestCount += 1;
      }
      if (url.pathname.startsWith("/api/")) {
        apiRequestCount += 1;
      }
      if (request.method() !== "GET") {
        nonGetRequestCount += 1;
      }
    });
    await page.route("**/control-ui.js", (route) => route.abort("blockedbyclient"));
    await page.goto(baseUrl, { waitUntil: "domcontentloaded" });
    const staticTruth = await page.evaluate(() => {
      const selector = [
        "[data-plan-action]",
        "[data-chat-add]",
        '[data-chat-row-menu-item="pin"]',
        '[data-chat-row-menu-item="archive"]',
        "[data-chat-folder]",
        "[data-agent-chat-send]",
        "[data-agent-chat-plan]",
        "[data-chat-routing-mode]",
        "[data-chat-autoscroll-mode]",
      ].join(",");
      const controls = [...document.querySelectorAll(selector)];
      let clickCount = 0;
      for (const control of controls) {
        control.addEventListener("click", () => {
          clickCount += 1;
        });
        control.click();
      }
      const conversations = [...document.querySelectorAll("[data-chat-conversation]")];
      const taskSpec = document.querySelector(
        '[data-control-ui-action-control="task-publisher-catalog"]',
      );
      const composer = document.getElementById("chat-message");
      const composerStatus = document.querySelector("[data-chat-send-state]");
      return {
        unavailable_control_count: controls.length,
        all_controls_natively_disabled: controls.every(
          (control) =>
            "disabled" in control &&
            control.disabled === true &&
            control.getAttribute("aria-disabled") === "true" &&
            control.dataset.controlUiUnavailable === "live-adapter",
        ),
        disabled_click_count: clickCount,
        seeded_conversation_count: conversations.length,
        seeded_conversations_static_read_only: conversations.every(
          (conversation) =>
            conversation.getAttribute("aria-disabled") === "true" &&
            conversation.getAttribute("tabindex") === "-1" &&
            conversation.dataset.controlUiConversationMode === "seeded-read-only",
        ),
        task_spec_static_read_only:
          taskSpec instanceof HTMLAnchorElement &&
          taskSpec.textContent.trim() === "Task spec" &&
          taskSpec.getAttribute("aria-label")?.includes("read-only") === true,
        composer_static_local_draft:
          composer instanceof HTMLTextAreaElement &&
          composer.dataset.controlUiComposerMode === "local-draft-only" &&
          composer.getAttribute("aria-label")?.includes("sending unavailable") === true,
        status_static_read_only:
          composerStatus?.dataset.chatSendState === "read-only" &&
          composerStatus.textContent?.includes("live adapter not bound") === true,
      };
    });
    noScriptProductTruth = {
      ...staticTruth,
      blocked_script_request_count: blockedScriptRequestCount,
      api_request_count: apiRequestCount,
      non_get_request_count: nonGetRequestCount,
    };
    noScriptProductTruth.ready =
      blockedScriptRequestCount === 1 &&
      apiRequestCount === 0 &&
      nonGetRequestCount === 0 &&
      staticTruth.unavailable_control_count === 99 &&
      staticTruth.all_controls_natively_disabled &&
      staticTruth.disabled_click_count === 0 &&
      staticTruth.seeded_conversation_count === 3 &&
      staticTruth.seeded_conversations_static_read_only &&
      staticTruth.task_spec_static_read_only &&
      staticTruth.composer_static_local_draft &&
      staticTruth.status_static_read_only;
    if (!noScriptProductTruth.ready) {
      failures.push("blocked control-ui.js exposed non-truthful live controls");
    }
    await context.close();
  }

  await browser.close();
  const report = {
    schema: "hepta_control_ui_progressive_enhancement_adversarial_v1",
    status: failures.length === 0 ? "ready" : "failed",
    race,
    stale_timeout: staleTimeout,
    oversized_utf8: oversizedUtf8,
    oversized_content_length: oversizedHeader,
    no_content_length_stream: noContentLengthStream,
    jsonp,
    structured_json: structuredJson,
    redirect,
    xss,
    no_script_product_truth: noScriptProductTruth,
    failures,
  };
  process.stdout.write(`${JSON.stringify(report)}\n`);
  if (failures.length > 0) {
    process.exit(1);
  }
})().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
