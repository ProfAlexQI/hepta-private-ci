const crypto = require("crypto");
const fs = require("fs");
const path = require("path");
const { chromium } = require("playwright");
const { installActualVisibilitySource } = require("./actual-visibility.cjs");

const [chromeBin, baseUrl, outputDir] = process.argv.slice(2);
(async () => {
  if (!outputDir || !path.isAbsolute(outputDir) || !fs.statSync(outputDir).isDirectory()) {
    throw new Error("progressive enhancement QA requires an existing absolute output directory");
  }
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
  await context.addInitScript({ content: installActualVisibilitySource });
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
  const chatFilteredVisibility = await page.locator(".tg-chat-item").evaluateAll((items) => {
    const entries = items.map((item) => ({
      id: item.getAttribute("data-chat-conversation") || "",
      ...window.__heptaActualVisibility(item),
    }));
    return {
      actual_visible_count: entries.filter((item) => item.visible).length,
      entries,
    };
  });
  await page.locator("#chat-search").fill("");
  const chatRestoredVisibility = await page.locator(".tg-chat-item").evaluateAll((items) => ({
    actual_visible_count: items.filter((item) => window.__heptaActualVisibility(item).visible).length,
  }));
  const chatSearchReady = chatItemCount > 0
    && hiddenChatItemCount === chatItemCount
    && chatFilteredVisibility.actual_visible_count === 0
    && chatRestoredVisibility.actual_visible_count === chatItemCount;
  if (!chatSearchReady) {
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
  const paletteFilteredVisibility = await page
    .locator("#command-palette-results .command-palette__item")
    .evaluateAll((items) => {
      const entries = items.map((item) => ({
        command_id: item.getAttribute("data-control-ui-command-palette-item") || "",
        ...window.__heptaActualVisibility(item),
      }));
      return {
        actual_visible_count: entries.filter((item) => item.visible).length,
        entries,
      };
    });
  await page.locator("#command-palette-input").fill("");
  const paletteRestoredVisibility = await page
    .locator("#command-palette-results .command-palette__item")
    .evaluateAll((items) => ({
      actual_visible_count: items.filter((item) => window.__heptaActualVisibility(item).visible).length,
    }));
  const commandPaletteSearchReady = paletteItemCount > 0
    && hiddenPaletteItemCount === paletteItemCount
    && paletteFilteredVisibility.actual_visible_count === 0
    && paletteRestoredVisibility.actual_visible_count === paletteItemCount;
  if (!commandPaletteSearchReady) {
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
      unavailable_controls_ready: controls.length === 14
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
    const filteredVisibility = await page.locator(
      `[data-chat-composer-popover="${picker}"] [data-chat-composer-picker-item]`,
    ).evaluateAll((items) => {
      const entries = items.map((item) => ({
        label: item.textContent.trim(),
        ...window.__heptaActualVisibility(item),
      }));
      return {
        actual_visible_count: entries.filter((item) => item.visible).length,
        entries,
      };
    });
    await search.evaluate((input) => {
      input.value = "";
      input.dispatchEvent(new Event("input", { bubbles: true }));
    });
    const restoredVisibility = await page.locator(
      `[data-chat-composer-popover="${picker}"] [data-chat-composer-picker-item]`,
    ).evaluateAll((items) => ({
      actual_visible_count: items.filter((item) => window.__heptaActualVisibility(item).visible).length,
    }));
    pickerSearchAudit.push({
      picker,
      item_count: itemCount,
      hidden_count: hiddenCount,
      filtered_actual_visible_count: filteredVisibility.actual_visible_count,
      restored_actual_visible_count: restoredVisibility.actual_visible_count,
      visibility: filteredVisibility.entries,
    });
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
    && pickerSearchAudit.every((item) => (
      item.item_count === 2
      && item.hidden_count === 2
      && item.filtered_actual_visible_count === 0
      && item.restored_actual_visible_count === item.item_count
    ));
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
  const localRouteAudit = await page.evaluate(() => {
    const target = document.getElementById("screen-card-evidence");
    const routeCards = [...document.querySelectorAll(".route-card")].map((card) => ({
      id: card.id,
      ...window.__heptaActualVisibility(card),
    }));
    return {
      hash: window.location.hash,
      popover_open: document.getElementById("row-menu-ui-chat-agent")?.matches(":popover-open") || false,
      toast: document.getElementById("toast")?.textContent || "",
      body_view: document.body.dataset.view || "",
      active_view: document.body.dataset.controlUiActiveView || "",
      target_visibility: window.__heptaActualVisibility(target),
      target_focused: document.activeElement === target,
      actual_visible_route_card_count: routeCards.filter((card) => card.visible).length,
      actual_visible_route_card_ids: routeCards.filter((card) => card.visible).map((card) => card.id),
    };
  });
  const localRouteNavigationReady = localRouteAudit.hash === "#evidence"
    && localRouteAudit.popover_open === false
    && localRouteAudit.toast === "Opened a local read-only surface."
    && localRouteAudit.body_view === "read-only"
    && localRouteAudit.active_view === "evidence"
    && localRouteAudit.target_visibility.visible
    && localRouteAudit.target_focused
    && localRouteAudit.actual_visible_route_card_count === 1
    && localRouteAudit.actual_visible_route_card_ids[0] === "screen-card-evidence";
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

  const commandCatalog = await page.locator("#commands [data-command-id]").evaluateAll((items) => (
    items.map((item) => ({
      id: item.dataset.commandId || "",
      label: item.dataset.commandLabel || "",
      command: item.dataset.commandText || "",
      route: item.dataset.commandRoute || null,
      palette: item.dataset.commandPalette === "true",
    }))
  ));
  const routes = commandCatalog
    .filter((entry) => entry.route !== null)
    .map((entry) => [entry.id, entry.route]);
  if (
    commandCatalog.length !== 51
    || commandCatalog.filter((entry) => entry.palette).length !== 18
    || routes.length !== 21
    || commandCatalog.some((entry) => !entry.id || !entry.label || !entry.command)
  ) {
    throw new Error("rendered typed command catalog is incomplete");
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
      const routeCards = [...document.querySelectorAll(".route-card")].map((card) => ({
        id: card.id,
        ...window.__heptaActualVisibility(card),
      }));
      const visibleRouteCards = routeCards.filter((card) => card.visible);
      return {
        screen: expectedScreen,
        target_id: expectedTarget,
        expected_href: expectedHref,
        hash: window.location.hash,
        body_view: document.body.dataset.view || "",
        active_view: document.body.dataset.controlUiActiveView || "",
        target_visibility: window.__heptaActualVisibility(target),
        target_focused: document.activeElement === target,
        actual_visible_route_card_count: visibleRouteCards.length,
        actual_visible_route_card_ids: visibleRouteCards.map((card) => card.id),
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
      && item.target_visibility.visible
      && item.target_focused
      && item.primary_nav_current_ready
      && (item.screen === "chat"
        ? item.body_view === "chat" && item.actual_visible_route_card_count === 0
        : item.body_view === "read-only"
          && item.actual_visible_route_card_count === 1
          && item.actual_visible_route_card_ids[0] === item.target_id)
    ));

  const currentRouteEntryAudit = await page.evaluate(() => {
    const rowTargets = {
      "open-evidence": { hash: "#evidence", screen: "evidence", target_id: "screen-card-evidence" },
      "open-approvals": { hash: "#approvals", screen: "approvals", target_id: "screen-card-approvals" },
      "open-sources": { hash: "#evidence", screen: "evidence", target_id: "screen-card-evidence" },
    };
    const anchorEntries = [...document.querySelectorAll(
      'a[href^="#screen-card-"], a[href="#evidence"], a[href="#task-publisher"]',
    )].map((anchor, index) => {
      const hash = anchor.getAttribute("href") || "";
      const screen = hash === "#evidence"
        ? "evidence"
        : hash === "#task-publisher"
          ? "task-publisher"
          : hash.replace(/^#screen-card-/, "");
      return {
        kind: "anchor",
        index,
        hash,
        screen,
        target_id: `screen-card-${screen}`,
      };
    });
    const directory = document.querySelector('[data-control-ui-route-directory="26-of-26"]');
    const directoryEntries = [...document.querySelectorAll("[data-control-ui-route-entry]")]
      .map((anchor) => ({
        screen: anchor.getAttribute("data-control-ui-route-entry") || "",
        hash: anchor.getAttribute("href") || "",
      }));
    const routeCardScreens = [...document.querySelectorAll(".route-card")]
      .map((card) => card.getAttribute("data-screen") || "");
    const directoryScreens = directoryEntries.map((entry) => entry.screen);
    const directoryReady = directory instanceof HTMLDetailsElement
      && directoryEntries.length === 26
      && new Set(directoryScreens).size === 26
      && routeCardScreens.length === 26
      && [...directoryScreens].sort().join("\u0000") === [...routeCardScreens].sort().join("\u0000")
      && directoryEntries.every((entry) => (
        entry.hash === `#screen-card-${entry.screen}`
        && document.getElementById(entry.hash.slice(1))?.dataset.screen === entry.screen
      ));
    const rowEntries = [...document.querySelectorAll(
      '[data-chat-row-menu-item="open-evidence"],'
        + '[data-chat-row-menu-item="open-approvals"],'
        + '[data-chat-row-menu-item="open-sources"]',
    )].map((item, index) => ({
      kind: "row-action",
      index,
      action: item.getAttribute("data-chat-row-menu-item") || "",
      ...rowTargets[item.getAttribute("data-chat-row-menu-item")],
    }));
    const entries = [...anchorEntries, ...rowEntries].map((entry) => {
      const target = document.getElementById(entry.target_id);
      return {
        ...entry,
        target_exists: Boolean(target),
        target_is_route_card: target?.matches(".route-card") || false,
      };
    });
    return {
      entry_count: entries.length,
      anchor_entry_count: anchorEntries.length,
      row_action_entry_count: rowEntries.length,
      directory_entry_count: directoryEntries.length,
      directory_ready: directoryReady,
      directory_entries: directoryEntries,
      legacy_route_page_count: document.querySelectorAll(".hepta-route-page").length,
      legacy_route_index_count: document.querySelectorAll(".hepta-route-index").length,
      entries,
    };
  });
  const currentRouteHashAudit = [];
  const uniqueCurrentRouteEntries = [...new Map(
    currentRouteEntryAudit.entries.map((entry) => [`${entry.hash}\u0000${entry.target_id}`, entry]),
  ).values()];
  for (const entry of uniqueCurrentRouteEntries) {
    await page.evaluate(() => {
      window.location.hash = "#chat";
    });
    await page.waitForFunction(
      () => document.body.dataset.controlUiActiveView === "chat",
      null,
      { timeout: 5000 },
    );
    await page.evaluate((hash) => {
      window.location.hash = hash;
    }, entry.hash);
    await page.waitForFunction(
      ({ expectedScreen, expectedTarget }) => (
        document.body.dataset.controlUiActiveView === expectedScreen
        && document.activeElement?.id === expectedTarget
      ),
      { expectedScreen: entry.screen, expectedTarget: entry.target_id },
      { timeout: 5000 },
    );
    currentRouteHashAudit.push(await page.evaluate(({ expectedHash, expectedScreen, expectedTarget }) => {
      const target = document.getElementById(expectedTarget);
      const visibleCards = [...document.querySelectorAll(".route-card")]
        .filter((card) => window.__heptaActualVisibility(card).visible);
      return {
        hash: window.location.hash,
        expected_hash: expectedHash,
        active_view: document.body.dataset.controlUiActiveView || "",
        expected_screen: expectedScreen,
        target_id: expectedTarget,
        target_visibility: window.__heptaActualVisibility(target),
        target_focused: document.activeElement === target,
        actual_visible_route_card_count: visibleCards.length,
        actual_visible_route_card_ids: visibleCards.map((card) => card.id),
      };
    }, {
      expectedHash: entry.hash,
      expectedScreen: entry.screen,
      expectedTarget: entry.target_id,
    }));
  }
  const currentRouteEntriesReady = currentRouteEntryAudit.entry_count === 58
    && currentRouteEntryAudit.anchor_entry_count === 55
    && currentRouteEntryAudit.row_action_entry_count === 3
    && currentRouteEntryAudit.directory_entry_count === 26
    && currentRouteEntryAudit.directory_ready
    && currentRouteEntryAudit.legacy_route_page_count === 0
    && currentRouteEntryAudit.legacy_route_index_count === 0
    && currentRouteEntryAudit.entries.every((entry) => (
      entry.target_exists && entry.target_is_route_card
    ))
    && currentRouteHashAudit.length === uniqueCurrentRouteEntries.length
    && currentRouteHashAudit.every((entry) => (
      entry.hash === entry.expected_hash
      && entry.active_view === entry.expected_screen
      && entry.target_visibility.visible
      && entry.target_focused
      && entry.actual_visible_route_card_count === 1
      && entry.actual_visible_route_card_ids[0] === entry.target_id
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
      const routeCards = [...document.querySelectorAll(".route-card")].map((card) => ({
        id: card.id,
        ...window.__heptaActualVisibility(card),
      }));
      const visibleRouteCards = routeCards.filter((card) => card.visible);
      return {
        screen: expectedScreen,
        target_id: expectedTarget,
        body_view: document.body.dataset.view || "",
        active_view: document.body.dataset.controlUiActiveView || "",
        target_visibility: window.__heptaActualVisibility(target),
        target_focused: document.activeElement === target,
        nav_current: activeLink?.getAttribute("aria-current") || "",
        actual_visible_route_card_count: visibleRouteCards.length,
        actual_visible_route_card_ids: visibleRouteCards.map((card) => card.id),
      };
    }, { expectedScreen: screen, expectedTarget: targetId }));
  }
  const topNavNavigationReady = topNavAudit.length === 3 && topNavAudit.every((item) => (
    item.body_view === "read-only"
    && item.active_view === item.screen
    && item.target_visibility.visible
    && item.target_focused
    && item.nav_current === "page"
    && item.actual_visible_route_card_count === 1
    && item.actual_visible_route_card_ids[0] === item.target_id
  ));

  await page.goBack();
  await page.waitForFunction(
    () => document.body.dataset.controlUiActiveView === "ops"
      && document.activeElement?.id === "screen-card-ops",
    null,
    { timeout: 5000 },
  );
  const backAudit = await page.evaluate(() => {
    const target = document.getElementById("screen-card-ops");
    const visibleRouteCards = [...document.querySelectorAll(".route-card")]
      .filter((card) => window.__heptaActualVisibility(card).visible);
    return {
      hash: window.location.hash,
      active_view: document.body.dataset.controlUiActiveView || "",
      target_visibility: window.__heptaActualVisibility(target),
      target_focused: document.activeElement === target,
      actual_visible_route_card_count: visibleRouteCards.length,
      actual_visible_route_card_ids: visibleRouteCards.map((card) => card.id),
    };
  });
  await page.goForward();
  await page.waitForFunction(
    () => document.body.dataset.controlUiActiveView === "external-agent-benchmark"
      && document.activeElement?.id === "screen-card-external-agent-benchmark",
    null,
    { timeout: 5000 },
  );
  const forwardAudit = await page.evaluate(() => {
    const target = document.getElementById("screen-card-external-agent-benchmark");
    const visibleRouteCards = [...document.querySelectorAll(".route-card")]
      .filter((card) => window.__heptaActualVisibility(card).visible);
    return {
      hash: window.location.hash,
      active_view: document.body.dataset.controlUiActiveView || "",
      target_visibility: window.__heptaActualVisibility(target),
      target_focused: document.activeElement === target,
      actual_visible_route_card_count: visibleRouteCards.length,
      actual_visible_route_card_ids: visibleRouteCards.map((card) => card.id),
    };
  });
  const routeHistoryReady = backAudit.hash === "#screen-card-ops"
    && backAudit.active_view === "ops"
    && backAudit.target_visibility.visible
    && backAudit.target_focused
    && backAudit.actual_visible_route_card_count === 1
    && backAudit.actual_visible_route_card_ids[0] === "screen-card-ops"
    && forwardAudit.hash === "#screen-card-external-agent-benchmark"
    && forwardAudit.active_view === "external-agent-benchmark"
    && forwardAudit.target_visibility.visible
    && forwardAudit.target_focused
    && forwardAudit.actual_visible_route_card_count === 1
    && forwardAudit.actual_visible_route_card_ids[0] === "screen-card-external-agent-benchmark";

  const routeViewScreenshots = [];
  for (const capture of [
    {
      name: "route-view-desktop",
      width: 1365,
      height: 900,
      screen: "dashboard",
      target_id: "screen-card-dashboard",
    },
    {
      name: "route-view-phone320",
      width: 320,
      height: 844,
      screen: "tasks",
      target_id: "screen-card-tasks",
    },
  ]) {
    await page.setViewportSize({ width: capture.width, height: capture.height });
    await page.evaluate((targetId) => {
      window.location.hash = `#${targetId}`;
    }, capture.target_id);
    await page.waitForFunction(
      ({ expectedScreen, expectedTarget }) => (
        document.body.dataset.controlUiActiveView === expectedScreen
        && document.activeElement?.id === expectedTarget
      ),
      { expectedScreen: capture.screen, expectedTarget: capture.target_id },
      { timeout: 5000 },
    );
    const visibility = await page.evaluate((targetId) => {
      const target = document.getElementById(targetId);
      const targetEyebrow = target?.querySelector(".eyebrow");
      const topbar = document.querySelector(".topbar");
      const topbarVisibility = window.__heptaActualVisibility(topbar);
      const topbarBottom = topbarVisibility.visible ? topbar.getBoundingClientRect().bottom : 0;
      const cards = [...document.querySelectorAll(".route-card")].map((card) => ({
        id: card.id,
        ...window.__heptaActualVisibility(card),
      }));
      return {
        target_visibility: window.__heptaActualVisibility(target),
        target_eyebrow_visibility: window.__heptaActualVisibility(targetEyebrow),
        top_obstruction_px: Math.max(
          0,
          topbarBottom - (targetEyebrow?.getBoundingClientRect().top || 0),
        ),
        target_focused: document.activeElement === target,
        actual_visible_route_card_count: cards.filter((card) => card.visible).length,
        actual_visible_route_card_ids: cards.filter((card) => card.visible).map((card) => card.id),
        document_width: document.documentElement.scrollWidth,
        viewport_width: window.innerWidth,
        horizontal_overflow_px: Math.max(0, document.documentElement.scrollWidth - window.innerWidth),
      };
    }, capture.target_id);
    const screenshotPath = path.join(outputDir, `${capture.name}.png`);
    await page.screenshot({ path: screenshotPath, fullPage: false });
    const screenshotBytes = fs.readFileSync(screenshotPath);
    routeViewScreenshots.push({
      ...capture,
      path: screenshotPath,
      bytes: screenshotBytes.length,
      sha256: crypto.createHash("sha256").update(screenshotBytes).digest("hex"),
      hash: `#${capture.target_id}`,
      ...visibility,
    });
  }
  const routeViewScreenshotsReady = routeViewScreenshots.length === 2
    && routeViewScreenshots.every((item) => (
      item.bytes > 0
      && /^[a-f0-9]{64}$/.test(item.sha256)
      && item.target_visibility.visible
      && item.target_focused
      && item.actual_visible_route_card_count === 1
      && item.actual_visible_route_card_ids[0] === item.target_id
      && item.horizontal_overflow_px === 0
      && item.target_visibility.rect_width >= (item.width === 320 ? 250 : 600)
      && item.target_eyebrow_visibility.visible
      && item.top_obstruction_px === 0
    ))
    && routeViewScreenshots.some((item) => item.width === 1365 && item.height === 900)
    && routeViewScreenshots.some((item) => item.width === 320 && item.height === 844);

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
    || !chatSearchReady
    || !commandPaletteSearchReady
    || !commandPaletteNavigationReady
    || !routeLinkNavigationReady
    || !currentRouteEntriesReady
    || !topNavNavigationReady
    || !routeHistoryReady
    || !routeViewScreenshotsReady
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
    command_catalog: commandCatalog,
    registry_route_count: routes.length,
    successful_route_count: results.length,
    snapshot_state: snapshotState,
    snapshot_request_count: apiRequests.filter(
      (request) => new URL(request.url).pathname === "/api/operator-snapshot",
    ).length,
    copy_interaction_ready: copiedText === "/control-ui --json",
    chat_search_ready: chatSearchReady,
    chat_search_visibility_audit: {
      item_count: chatItemCount,
      hidden_attribute_count: hiddenChatItemCount,
      filtered: chatFilteredVisibility,
      restored: chatRestoredVisibility,
    },
    command_palette_search_ready: commandPaletteSearchReady,
    command_palette_search_visibility_audit: {
      item_count: paletteItemCount,
      hidden_attribute_count: hiddenPaletteItemCount,
      filtered: paletteFilteredVisibility,
      restored: paletteRestoredVisibility,
    },
    command_palette_navigation_ready: commandPaletteNavigationReady,
    command_palette_navigation_audit: commandPaletteNavigationAudit,
    route_link_navigation_ready: routeLinkNavigationReady,
    route_link_navigation_audit: routeLinkAudit,
    current_route_entries_ready: currentRouteEntriesReady,
    route_directory_ready: currentRouteEntryAudit.directory_ready,
    current_route_entry_audit: currentRouteEntryAudit,
    current_route_hash_audit: currentRouteHashAudit,
    top_nav_navigation_ready: topNavNavigationReady,
    top_nav_navigation_audit: topNavAudit,
    route_history_ready: routeHistoryReady,
    route_history_audit: { back: backAudit, forward: forwardAudit },
    route_view_screenshots_ready: routeViewScreenshotsReady,
    route_view_screenshots: routeViewScreenshots,
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
