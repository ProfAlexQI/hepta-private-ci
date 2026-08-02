const { chromium } = require("playwright");

const [chromeBin, baseUrl] = process.argv.slice(2);
const routes = [
  ["control-ui", "/api/control-ui"],
  ["config-surface", "/api/config"],
  ["optional-configs", "/api/optional-configs"],
  ["hepta-merge-completion", "/api/hepta-merge-completion"],
  ["external-agent-benchmark", "/api/external-agent-benchmark"],
  ["sessions", "/api/sessions"],
  ["session-activity", "/api/session-activity"],
  ["operator-console", "/api/operator-console"],
  ["subagent-observatory", "/api/subagent-observatory"],
  ["events", "/api/events"],
  ["events-report", "/api/events-report"],
  ["activity", "/api/activity"],
  ["transcript", "/api/transcript"],
  ["approvals", "/api/approvals"],
  ["policy", "/api/policy"],
  ["operator-security", "/api/operator-security"],
  ["gateway-runtime", "/api/gateway-runtime"],
  ["gateway-dispatch", "/api/gateway-dispatch"],
  ["gateway-ledger", "/api/gateway-ledger"],
  ["gateway-retry-dead-letter", "/api/gateway-retry-dead-letter"],
  ["multi-agent-runtime", "/api/multi-agent-runtime"],
];

(async () => {
  const origin = new URL(baseUrl).origin;
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
  const context = await browser.newContext();
  await context.grantPermissions(["clipboard-read", "clipboard-write"], { origin });
  const page = await context.newPage();
  const requests = [];
  const consoleErrors = [];
  page.on("request", (request) => {
    if (/^https?:/i.test(request.url())) {
      requests.push({ url: request.url(), method: request.method() });
    }
  });
  page.on("console", (message) => {
    if (message.type() === "error") {
      consoleErrors.push(message.text());
    }
  });
  page.on("pageerror", (error) => consoleErrors.push(error.message));

  await page.goto(baseUrl, { waitUntil: "networkidle" });
  await page.waitForFunction(
    () => document.documentElement.dataset.controlUiProgressiveEnhancement === "ready",
    null,
    { timeout: 10000 },
  );
  await page.waitForFunction(
    () => {
      const status = document.getElementById("operator-snapshot-status");
      return status && status.dataset.state !== "loading";
    },
    null,
    { timeout: 10000 },
  );
  const snapshotState = await page.locator("#operator-snapshot-status").getAttribute("data-state");
  if (snapshotState !== "ready" && snapshotState !== "empty") {
    throw new Error(`operator snapshot did not reach a usable state: ${snapshotState}`);
  }

  await page.locator("#hepta-command-panel").evaluate((panel) => {
    for (let node = panel; node && node !== document.documentElement; node = node.parentElement) {
      node.hidden = false;
      if (node instanceof HTMLDetailsElement) {
        node.open = true;
      }
      node.style.setProperty("display", node === panel ? "block" : "revert", "important");
      node.style.setProperty("visibility", "visible", "important");
      node.style.setProperty("opacity", "1", "important");
      node.style.setProperty("pointer-events", "auto", "important");
    }
  });

  const chatItemCount = await page.locator(".tg-chat-item").count();
  await page.locator("#chat-search").fill("__hepta_no_matching_chat__");
  const hiddenChatItemCount = await page.locator(".tg-chat-item[hidden]").count();
  await page.locator("#chat-search").fill("");
  if (chatItemCount === 0 || hiddenChatItemCount !== chatItemCount) {
    throw new Error("local chat search did not filter every non-matching item");
  }

  await page.locator("#command-palette-input").evaluate((input) => {
    input.value = "__hepta_no_matching_command__";
    input.dispatchEvent(new Event("input", { bubbles: true }));
  });
  const paletteItemCount = await page.locator("#command-palette-results .command-palette__item").count();
  const hiddenPaletteItemCount = await page
    .locator("#command-palette-results .command-palette__item[hidden]")
    .count();
  await page.locator("#command-palette-input").evaluate((input) => {
    input.value = "";
    input.dispatchEvent(new Event("input", { bubbles: true }));
  });
  if (paletteItemCount === 0 || hiddenPaletteItemCount !== paletteItemCount) {
    throw new Error("local command palette search did not filter every non-matching item");
  }

  await page
    .locator('[data-command-id="control-ui"] [data-copy="/control-ui --json"]')
    .click({ force: true });
  await page.waitForFunction(
    () => document.getElementById("toast")?.textContent === "Copied to clipboard.",
    null,
    { timeout: 5000 },
  );
  const copiedText = await page.evaluate(() => navigator.clipboard.readText());
  if (copiedText !== "/control-ui --json") {
    throw new Error("copy interaction did not preserve the exact command text");
  }

  const requestCountBeforeUnavailableAudit = requests.length;
  const productTruthAudit = await page.evaluate(() => {
    const unavailableControls = Array.from(
      document.querySelectorAll('[data-control-ui-unavailable="live-adapter"]'),
    );
    const conversations = Array.from(document.querySelectorAll("[data-chat-conversation]"));
    const before = {
      hash: window.location.hash,
      composer: document.getElementById("chat-message")?.value || "",
      open_popover_count: document.querySelectorAll("[popover]:popover-open").length,
    };
    const controls = unavailableControls.map((node) => {
      const ariaLabel = node.getAttribute("aria-label") || "";
      const title = node.getAttribute("title") || "";
      return {
        selector: node.getAttribute("data-chat-row-menu-item")
          || node.getAttribute("data-control-ui-icon-button")
          || node.getAttribute("data-plan-action")
          || node.tagName.toLowerCase(),
        native_disabled: "disabled" in node && node.disabled === true,
        aria_disabled: node.getAttribute("aria-disabled") === "true",
        unavailable_marker: node.getAttribute("data-control-ui-unavailable") || "",
        aria_label: ariaLabel,
        title,
        truthful_copy: ariaLabel.length > 0
          && title === ariaLabel
          && (title.toLowerCase().includes("unavailable")
            || title.toLowerCase().includes("requires")
            || title.toLowerCase().includes("live adapter")
            || title.toLowerCase().includes("static preview")),
      };
    });
    unavailableControls.forEach((node) => node.click());
    const after = {
      hash: window.location.hash,
      composer: document.getElementById("chat-message")?.value || "",
      open_popover_count: document.querySelectorAll("[popover]:popover-open").length,
    };
    const seededConversationsReady = conversations.length === 3
      && conversations.every((node) => (
        node.getAttribute("aria-disabled") === "true"
        && node.getAttribute("data-control-ui-conversation-mode") === "seeded-read-only"
        && node.tabIndex === -1
      ));
    return {
      capability_mode: document.documentElement.dataset.controlUiCapabilityMode || "",
      live_adapter_bound: document.documentElement.dataset.controlUiLiveAdapterBound || "",
      unavailable_control_count: controls.length,
      controls,
      unavailable_controls_ready: controls.length > 0
        && controls.every((item) => item.native_disabled
          && item.aria_disabled
          && item.unavailable_marker === "live-adapter"
          && item.truthful_copy),
      unavailable_click_noop_ready: before.hash === after.hash
        && before.composer === after.composer
        && before.open_popover_count === after.open_popover_count,
      seeded_conversation_count: conversations.length,
      seeded_conversations_ready: seededConversationsReady,
    };
  });
  productTruthAudit.unavailable_request_noop_ready = requests.length === requestCountBeforeUnavailableAudit;

  const localInspectorAudit = await page.locator("#json-input").evaluate((input) => {
    const preview = document.getElementById("json-preview");
    input.value = '{"z":1,"nested":{"ready":true}}';
    input.dispatchEvent(new Event("input", { bubbles: true }));
    const valid = {
      state: preview?.dataset.state || "",
      text: preview?.textContent || "",
    };
    input.value = "{";
    input.dispatchEvent(new Event("input", { bubbles: true }));
    const invalid = {
      state: preview?.dataset.state || "",
      text: preview?.textContent || "",
    };
    input.value = "";
    input.dispatchEvent(new Event("input", { bubbles: true }));
    return {
      valid,
      invalid,
      ready: valid.state === "ready"
        && valid.text === '{\n  "z": 1,\n  "nested": {\n    "ready": true\n  }\n}'
        && invalid.state === "error"
        && invalid.text.startsWith("Invalid JSON:")
        && preview?.dataset.state === "empty",
    };
  });

  await page.locator("#chat-message").evaluate((input) => {
    input.value = "";
    input.dispatchEvent(new Event("input", { bubbles: true }));
  });
  const pickerSearchAudit = [];
  for (const picker of ["artifact", "command"]) {
    await page.locator(`[data-chat-composer-popover-toggle="${picker}"]`).click({ force: true });
    const search = page.locator(`[data-chat-composer-picker-search="${picker}"]`);
    await search.evaluate((input) => {
      input.value = "__hepta_no_matching_picker_item__";
      input.dispatchEvent(new Event("input", { bubbles: true }));
    });
    const itemCount = await page.locator(
      `[data-chat-composer-popover="${picker}"] [data-chat-composer-picker-item]`,
    ).count();
    const hiddenCount = await page.locator(
      `[data-chat-composer-popover="${picker}"] [data-chat-composer-picker-item][hidden]`,
    ).count();
    await search.evaluate((input) => {
      input.value = "";
      input.dispatchEvent(new Event("input", { bubbles: true }));
    });
    pickerSearchAudit.push({ picker, item_count: itemCount, hidden_count: hiddenCount });
    const itemSelector = picker === "artifact"
      ? '[data-chat-artifact-insert="evidence-note"]'
      : '[data-chat-command-insert="/control-ui --json"]';
    await page.locator(itemSelector).click({ force: true });
  }
  const localDraftAudit = await page.evaluate(() => ({
    value: document.getElementById("chat-message")?.value || "",
    status: document.querySelector("[data-chat-send-state]")?.textContent || "",
    open_composer_popover_count: document.querySelectorAll(".tg-composer-popover:popover-open").length,
  }));
  const pickerSearchReady = pickerSearchAudit.length === 2
    && pickerSearchAudit.every((item) => item.item_count === 2 && item.hidden_count === 2);
  const localDraftInsertionReady = localDraftAudit.value.includes("[Evidence note — local draft only]")
    && localDraftAudit.value.includes("/control-ui --json")
    && localDraftAudit.status === "local draft updated · not sent"
    && localDraftAudit.open_composer_popover_count === 0;

  await page.locator('[data-chat-row-menu-toggle="ui-chat-agent"]').click({ force: true });
  await page.locator('[data-chat-row-menu-item="open-evidence"]').click({ force: true });
  const localRouteAudit = await page.evaluate(() => ({
    hash: window.location.hash,
    popover_open: document.getElementById("row-menu-ui-chat-agent")?.matches(":popover-open") || false,
    toast: document.getElementById("toast")?.textContent || "",
  }));
  const localRouteNavigationReady = localRouteAudit.hash === "#evidence"
    && localRouteAudit.popover_open === false
    && localRouteAudit.toast === "Opened a local read-only surface.";
  await page.evaluate(() => {
    window.location.hash = "chat";
  });

  const results = [];
  for (const [commandId, path] of routes) {
    const selector = `[data-command-id="${commandId}"] [data-run-command="read-only"]`;
    const button = page.locator(selector);
    if ((await button.count()) !== 1) {
      throw new Error(`missing unique read-only button for ${commandId}`);
    }
    if (await button.isDisabled()) {
      throw new Error(`allowlisted read-only button is disabled: ${commandId}`);
    }
    const responsePromise = page.waitForResponse(
      (response) => new URL(response.url()).pathname === path && response.request().method() === "GET",
      { timeout: 10000 },
    );
    await button.click({ force: true });
    const response = await responsePromise;
    const contentType = response.headers()["content-type"] || "";
    if (response.status() !== 200 || !contentType.toLowerCase().startsWith("application/json")) {
      throw new Error(`${path} returned ${response.status()} ${contentType}`);
    }
    await page.waitForFunction(
      (id) => {
        const state = document.querySelector(`[data-command-id="${id}"]`)?.dataset.commandState;
        return state === "ready" || state === "empty" || state === "error";
      },
      commandId,
      { timeout: 10000 },
    );
    const state = await page
      .locator(`[data-command-id="${commandId}"]`)
      .getAttribute("data-command-state");
    if (state === "error") {
      throw new Error(`${path} rendered an error state`);
    }
    const output = await page.locator("#command-runner-output").textContent();
    const rendered = JSON.parse(output || "null");
    if (rendered?.source_path !== path || !("data" in rendered)) {
      throw new Error(`${path} rendered without an exact source-path envelope`);
    }
    results.push({ command_id: commandId, path, status: response.status(), content_type: contentType, state });
  }

  const buttonAudit = await page.$$eval('[data-run-command="read-only"]', (buttons) =>
    buttons.map((button) => ({
      command_id: button.closest("[data-command-id]")?.dataset.commandId || "",
      disabled: button.disabled,
      registry: button.dataset.readOnlyRegistry || "",
    })),
  );
  const allowedButtonCount = buttonAudit.filter(
    (button) => !button.disabled && button.registry === "allowed",
  ).length;
  const unsafeButtonCount = buttonAudit.filter(
    (button) => button.registry !== "allowed" && !button.disabled,
  ).length;

  const crossOriginRequests = requests.filter((request) => new URL(request.url).origin !== origin);
  const nonGetRequests = requests.filter((request) => request.method !== "GET");
  const expectedApiPaths = ["/api/operator-snapshot", ...routes.map((route) => route[1])];
  const apiRequests = requests.filter((request) => new URL(request.url).pathname.startsWith("/api/"));
  const unexpectedApiRequests = apiRequests.filter(
    (request) => !expectedApiPaths.includes(new URL(request.url).pathname),
  );
  const missingOrDuplicateApiPaths = expectedApiPaths.filter(
    (path) => apiRequests.filter((request) => new URL(request.url).pathname === path).length !== 1,
  );

  const report = {
    schema: "hepta_control_ui_progressive_enhancement_browser_v1",
    status: "ready",
    registry_route_count: routes.length,
    successful_route_count: results.length,
    snapshot_state: snapshotState,
    snapshot_request_count: apiRequests.filter(
      (request) => new URL(request.url).pathname === "/api/operator-snapshot",
    ).length,
    copy_interaction_ready: copiedText === "/control-ui --json",
    chat_search_ready: hiddenChatItemCount === chatItemCount,
    command_palette_search_ready: hiddenPaletteItemCount === paletteItemCount,
    product_truth_audit: productTruthAudit,
    unavailable_controls_ready: productTruthAudit.unavailable_controls_ready,
    unavailable_click_noop_ready: productTruthAudit.unavailable_click_noop_ready
      && productTruthAudit.unavailable_request_noop_ready,
    seeded_conversations_ready: productTruthAudit.seeded_conversations_ready,
    local_json_inspector_ready: localInspectorAudit.ready,
    local_json_inspector_audit: localInspectorAudit,
    composer_picker_search_ready: pickerSearchReady,
    composer_picker_search_audit: pickerSearchAudit,
    local_draft_insertion_ready: localDraftInsertionReady,
    local_draft_audit: localDraftAudit,
    local_route_navigation_ready: localRouteNavigationReady,
    local_route_audit: localRouteAudit,
    allowed_button_count: allowedButtonCount,
    non_allowlisted_button_count: buttonAudit.length - allowedButtonCount,
    unsafe_enabled_button_count: unsafeButtonCount,
    same_origin_request_count: requests.length - crossOriginRequests.length,
    cross_origin_request_count: crossOriginRequests.length,
    non_get_request_count: nonGetRequests.length,
    unexpected_api_request_count: unexpectedApiRequests.length,
    missing_or_duplicate_api_paths: missingOrDuplicateApiPaths,
    console_error_count: consoleErrors.length,
    results,
    live_adapter_bound: false,
    mutation_endpoint_called: false,
  };

  await browser.close();
  const failed = results.length !== routes.length
    || allowedButtonCount !== routes.length
    || unsafeButtonCount !== 0
    || crossOriginRequests.length !== 0
    || nonGetRequests.length !== 0
    || unexpectedApiRequests.length !== 0
    || missingOrDuplicateApiPaths.length !== 0
    || consoleErrors.length !== 0
    || copiedText !== "/control-ui --json"
    || hiddenChatItemCount !== chatItemCount
    || hiddenPaletteItemCount !== paletteItemCount
    || productTruthAudit.capability_mode !== "local-read-only"
    || productTruthAudit.live_adapter_bound !== "false"
    || !productTruthAudit.unavailable_controls_ready
    || !productTruthAudit.unavailable_click_noop_ready
    || !productTruthAudit.unavailable_request_noop_ready
    || !productTruthAudit.seeded_conversations_ready
    || !localInspectorAudit.ready
    || !pickerSearchReady
    || !localDraftInsertionReady
    || !localRouteNavigationReady;
  process.stdout.write(`${JSON.stringify(report)}\n`);
  if (failed) {
    process.exit(1);
  }
})().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
