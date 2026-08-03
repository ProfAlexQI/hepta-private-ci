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

  const chatItemCount = await page.locator(".tg-chat-item").count();
  await page.locator("#chat-search").fill("__hepta_no_matching_chat__");
  const hiddenChatItemCount = await page.locator(".tg-chat-item[hidden]").count();
  await page.locator("#chat-search").fill("");
  if (chatItemCount === 0 || hiddenChatItemCount !== chatItemCount) {
    throw new Error("local chat search did not filter every non-matching item");
  }

  await page.locator("[data-open-command-palette]").click();
  await page.waitForFunction(
    () => document.getElementById("command-palette")?.matches(":popover-open") === true,
    null,
    { timeout: 5000 },
  );
  await page.locator("#command-palette-input").fill("__hepta_no_matching_command__");
  const paletteItemCount = await page.locator("#command-palette-results .command-palette__item").count();
  const hiddenPaletteItemCount = await page
    .locator("#command-palette-results .command-palette__item[hidden]")
    .count();
  await page.locator("#command-palette-input").fill("");
  if (paletteItemCount === 0 || hiddenPaletteItemCount !== paletteItemCount) {
    throw new Error("local command palette search did not filter every non-matching item");
  }
  const paletteInitialFocusReady = await page.evaluate(() => (
    document.activeElement === document.getElementById("command-palette-input")
  ));
  await page.locator("[data-control-ui-command-palette-close]").click();

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
        !node.hasAttribute("aria-disabled")
        && node.getAttribute("data-control-ui-conversation-readonly") === "true"
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
    await page.locator(`[data-chat-composer-popover-toggle="${picker}"]`).click();
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
    await page.locator(itemSelector).click();
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

  await page.locator('[data-chat-row-menu-toggle="ui-chat-agent"]').click();
  await page.locator('[data-chat-row-menu-item="open-evidence"]').click();
  await page.waitForFunction(
    () => (
      document.body.dataset.controlUiActiveView === "evidence"
      && document.activeElement?.id === "screen-card-evidence"
    ),
    null,
    { timeout: 5000 },
  );
  const localRouteAudit = await page.evaluate(() => ({
    hash: window.location.hash,
    popover_open: document.getElementById("row-menu-ui-chat-agent")?.matches(":popover-open") || false,
    toast: document.getElementById("toast")?.textContent || "",
    body_view: document.body.dataset.view || "",
    active_view: document.body.dataset.controlUiActiveView || "",
    target_visible: Boolean(document.getElementById("screen-card-evidence")?.offsetParent),
    target_focused: document.activeElement?.id === "screen-card-evidence",
  }));
  const localRouteNavigationReady = localRouteAudit.hash === "#evidence"
    && localRouteAudit.popover_open === false
    && localRouteAudit.toast === "Opened a local read-only surface."
    && localRouteAudit.body_view === "read-only"
    && localRouteAudit.active_view === "evidence"
    && localRouteAudit.target_visible
    && localRouteAudit.target_focused;
  await page.locator('#hepta-nav [data-screen="chat"]').click();
  await page.waitForFunction(
    () => document.body.dataset.controlUiActiveView === "chat",
    null,
    { timeout: 5000 },
  );

  await page.locator("[data-open-command-palette]").click();
  await page.locator('[data-control-ui-command-palette-item="control-ui"]').click();
  await page.waitForFunction(
    () => (
      document.body.dataset.controlUiActiveView === "commands"
      && document.activeElement?.closest("[data-command-id]")?.dataset.commandId === "control-ui"
    ),
    null,
    { timeout: 5000 },
  );
  const commandPaletteNavigationAudit = await page.evaluate(() => ({
    hash: window.location.hash,
    body_view: document.body.dataset.view || "",
    panel_visible: Boolean(document.getElementById("hepta-command-panel")?.offsetParent),
    focused_command_id: document.activeElement?.closest("[data-command-id]")?.dataset.commandId || "",
    palette_open: document.getElementById("command-palette")?.matches(":popover-open") || false,
    catalog_count: Number(document.documentElement.dataset.controlUiCommandCatalogCount || 0),
    palette_count: Number(document.documentElement.dataset.controlUiCommandPaletteCount || 0),
    command_catalog_source: document.getElementById("commands")?.dataset.controlUiCatalogSource || "",
    palette_catalog_source: document.getElementById("command-palette-results")?.dataset.controlUiCatalogSource || "",
  }));
  const commandPaletteNavigationReady = paletteInitialFocusReady
    && commandPaletteNavigationAudit.hash === "#commands"
    && commandPaletteNavigationAudit.body_view === "commands"
    && commandPaletteNavigationAudit.panel_visible
    && commandPaletteNavigationAudit.focused_command_id === "control-ui"
    && commandPaletteNavigationAudit.palette_open === false
    && commandPaletteNavigationAudit.catalog_count === 51
    && commandPaletteNavigationAudit.palette_count === 18
    && commandPaletteNavigationAudit.command_catalog_source === "typed-command-catalog-v1"
    && commandPaletteNavigationAudit.palette_catalog_source === "typed-command-catalog-v1";

  await page
    .locator('[data-command-id="control-ui"] [data-copy="/control-ui --json"]')
    .click();
  await page.waitForFunction(
    () => document.getElementById("toast")?.textContent === "Copied to clipboard.",
    null,
    { timeout: 5000 },
  );
  const copiedText = await page.evaluate(() => navigator.clipboard.readText());
  if (copiedText !== "/control-ui --json") {
    throw new Error("copy interaction did not preserve the exact command text");
  }

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
    await button.click();
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

  const routeLinkDescriptors = await page.$$eval(
    "[data-hepta-nav-route], [data-control-ui-safety-route]",
    (links) => links.map((link) => ({
      nav_key: link.getAttribute("data-hepta-nav-key") || "",
      screen: link.getAttribute("data-screen") || "",
      href: link.getAttribute("href") || "",
      safety: link.hasAttribute("data-control-ui-safety-route"),
    })),
  );
  await page.setViewportSize({ width: 500, height: 844 });
  const routeLinkAudit = [];
  for (const descriptor of routeLinkDescriptors) {
    await page.locator('#hepta-nav [data-screen="chat"]').click();
    await page.waitForFunction(
      () => document.body.dataset.controlUiActiveView === "chat",
      null,
      { timeout: 5000 },
    );
    await page.locator('[data-chat-mobile-pane-tab="room"]').click();
    await page.waitForFunction(
      () => Boolean(document.getElementById("chat-room")?.offsetParent),
      null,
      { timeout: 5000 },
    );
    const selector = descriptor.safety
      ? `[data-control-ui-safety-route][data-screen="${descriptor.screen}"]`
      : `[data-hepta-nav-key="${descriptor.nav_key}"]`;
    const link = page.locator(selector);
    if ((await link.count()) !== 1) {
      throw new Error(`route link is not unique: ${selector}`);
    }
    const owningDetails = link.locator("xpath=ancestor::details[1]");
    if (
      (await owningDetails.count()) === 1
      && !(await owningDetails.evaluate((details) => details.open))
    ) {
      await owningDetails.locator(":scope > summary").click();
    }
    if (!(await link.isVisible())) {
      throw new Error(`route link is not user-visible: ${selector}`);
    }
    await link.focus();
    await link.press("Enter");
    const targetId = descriptor.screen === "chat"
      ? "chat-thread"
      : `screen-card-${descriptor.screen}`;
    try {
      await page.waitForFunction(
        ({ expectedScreen, expectedTarget }) => (
          document.body.dataset.controlUiActiveView === expectedScreen
          && document.activeElement?.id === expectedTarget
        ),
        { expectedScreen: descriptor.screen, expectedTarget: targetId },
        { timeout: 5000 },
      );
    } catch (error) {
      throw new Error(
        `route link did not activate ${descriptor.href} (${descriptor.screen}): ${error.message}`,
      );
    }
    routeLinkAudit.push(await page.evaluate(({ expectedScreen, expectedTarget, expectedHref }) => {
      const target = document.getElementById(expectedTarget);
      const primaryNav = document.querySelector(`#hepta-nav [data-screen="${expectedScreen}"]`);
      const visibleRouteCards = [...document.querySelectorAll(".route-card")].filter(
        (card) => !card.hidden && Boolean(card.offsetParent),
      );
      return {
        screen: expectedScreen,
        target_id: expectedTarget,
        expected_href: expectedHref,
        hash: window.location.hash,
        body_view: document.body.dataset.view || "",
        active_view: document.body.dataset.controlUiActiveView || "",
        target_visible: Boolean(target?.offsetParent),
        target_focused: document.activeElement === target,
        visible_route_card_count: visibleRouteCards.length,
        visible_route_card_ids: visibleRouteCards.map((card) => card.id),
        primary_nav_current_ready: !primaryNav
          || primaryNav.getAttribute("aria-current") === "page",
      };
    }, {
      expectedScreen: descriptor.screen,
      expectedTarget: targetId,
      expectedHref: descriptor.href,
    }));
  }
  const routeLinkNavigationReady = routeLinkDescriptors.length === 22
    && routeLinkAudit.length === routeLinkDescriptors.length
    && routeLinkAudit.every((item) => (
      item.hash === item.expected_href
      && item.active_view === item.screen
      && item.target_visible
      && item.target_focused
      && item.primary_nav_current_ready
      && (item.screen === "chat"
        ? item.body_view === "chat" && item.visible_route_card_count === 0
        : item.body_view === "read-only"
          && item.visible_route_card_count === 1
          && item.visible_route_card_ids[0] === item.target_id)
    ));

  await page.setViewportSize({ width: 1280, height: 720 });
  const topNavAudit = [];
  for (const [screen, targetId] of [
    ["tasks", "screen-card-tasks"],
    ["ops", "screen-card-ops"],
    ["external-agent-benchmark", "screen-card-external-agent-benchmark"],
  ]) {
    const link = page.locator(`#hepta-nav [data-screen="${screen}"]`);
    await link.focus();
    await link.press("Enter");
    await page.waitForFunction(
      ({ expectedScreen, expectedTarget }) => (
        document.body.dataset.controlUiActiveView === expectedScreen
        && document.activeElement?.id === expectedTarget
      ),
      { expectedScreen: screen, expectedTarget: targetId },
      { timeout: 5000 },
    );
    topNavAudit.push(await page.evaluate(({ expectedScreen, expectedTarget }) => {
      const target = document.getElementById(expectedTarget);
      const activeLink = document.querySelector(`#hepta-nav [data-screen="${expectedScreen}"]`);
      return {
        screen: expectedScreen,
        target_id: expectedTarget,
        body_view: document.body.dataset.view || "",
        active_view: document.body.dataset.controlUiActiveView || "",
        target_visible: Boolean(target?.offsetParent),
        target_focused: document.activeElement === target,
        nav_current: activeLink?.getAttribute("aria-current") || "",
      };
    }, { expectedScreen: screen, expectedTarget: targetId }));
  }
  const topNavNavigationReady = topNavAudit.length === 3 && topNavAudit.every((item) => (
    item.body_view === "read-only"
    && item.active_view === item.screen
    && item.target_visible
    && item.target_focused
    && item.nav_current === "page"
  ));

  await page.goBack();
  await page.waitForFunction(
    () => document.body.dataset.controlUiActiveView === "ops"
      && document.activeElement?.id === "screen-card-ops",
    null,
    { timeout: 5000 },
  );
  const backAudit = await page.evaluate(() => ({
    hash: window.location.hash,
    active_view: document.body.dataset.controlUiActiveView || "",
    target_visible: Boolean(document.getElementById("screen-card-ops")?.offsetParent),
    target_focused: document.activeElement?.id === "screen-card-ops",
  }));
  await page.goForward();
  await page.waitForFunction(
    () => document.body.dataset.controlUiActiveView === "external-agent-benchmark"
      && document.activeElement?.id === "screen-card-external-agent-benchmark",
    null,
    { timeout: 5000 },
  );
  const forwardAudit = await page.evaluate(() => ({
    hash: window.location.hash,
    active_view: document.body.dataset.controlUiActiveView || "",
    target_visible: Boolean(document.getElementById("screen-card-external-agent-benchmark")?.offsetParent),
    target_focused: document.activeElement?.id === "screen-card-external-agent-benchmark",
  }));
  const routeHistoryReady = backAudit.hash === "#screen-card-ops"
    && backAudit.active_view === "ops"
    && backAudit.target_visible
    && backAudit.target_focused
    && forwardAudit.hash === "#screen-card-external-agent-benchmark"
    && forwardAudit.active_view === "external-agent-benchmark"
    && forwardAudit.target_visible
    && forwardAudit.target_focused;

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
    || !commandPaletteNavigationReady
    || !routeLinkNavigationReady
    || !topNavNavigationReady
    || !routeHistoryReady
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

  const report = {
    schema: "hepta_control_ui_progressive_enhancement_browser_v1",
    status: failed ? "failed" : "ready",
    registry_route_count: routes.length,
    successful_route_count: results.length,
    snapshot_state: snapshotState,
    snapshot_request_count: apiRequests.filter(
      (request) => new URL(request.url).pathname === "/api/operator-snapshot",
    ).length,
    copy_interaction_ready: copiedText === "/control-ui --json",
    chat_search_ready: hiddenChatItemCount === chatItemCount,
    command_palette_search_ready: hiddenPaletteItemCount === paletteItemCount,
    command_palette_navigation_ready: commandPaletteNavigationReady,
    command_palette_navigation_audit: commandPaletteNavigationAudit,
    route_link_navigation_ready: routeLinkNavigationReady,
    route_link_navigation_audit: routeLinkAudit,
    top_nav_navigation_ready: topNavNavigationReady,
    top_nav_navigation_audit: topNavAudit,
    route_history_ready: routeHistoryReady,
    route_history_audit: { back: backAudit, forward: forwardAudit },
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
  process.stdout.write(`${JSON.stringify(report)}\n`);
  if (failed) {
    process.exit(1);
  }
})().catch((error) => {
  console.error(error?.stack || error);
  process.exit(1);
});
