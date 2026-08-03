"use strict";

(() => {
  const REQUEST_TIMEOUT_MS = 8000;
  const MAX_RESPONSE_BYTES = 2 * 1024 * 1024;
  const SNAPSHOT_PATH = "/api/operator-snapshot";
  const JSON_MEDIA_TYPE = /^application\/[a-z0-9!#$%&'*+.^_`|~-]+\+json$/;
  const COMMAND_CATALOG = Object.freeze([
    ["control-ui", "Control Ui", "/control-ui --json", "/api/control-ui", true],
    ["config-surface", "Config Surface", "/config-surface --json", "/api/config", true],
    ["local-import", "Local Import", "/local-import --json", null, true],
    ["providers", "Providers", "/providers --json", null, true],
    ["image-models", "Image Models", "/image-models --json", null, true],
    ["optional-configs", "Optional Configs", "/optional-configs --json", "/api/optional-configs", true],
    ["doctor", "Doctor", "/doctor --json", null, true],
    ["native-capabilities", "Native Capabilities", "/native-capabilities --json", null, true],
    ["external-readiness", "External Readiness", "/external-readiness --json", null, true],
    ["production-surface", "Production Surface", "/production-surface --json", null, true],
    ["production-parity", "Production Parity", "/production-parity --json", null, true],
    ["hepta-merge-completion", "Hepta Merge Completion", "/hepta-merge-completion --json", "/api/hepta-merge-completion", true],
    ["external-agent-benchmark", "External Agent Benchmark", "/external-agent-benchmark --json", "/api/external-agent-benchmark", true],
    ["sessions", "Sessions", "/sessions --json", "/api/sessions", true],
    ["session-activity", "Session Activity", "/session-activity --json", "/api/session-activity", true],
    ["tasks", "Tasks", "/tasks --json", null, true],
    ["task", "Task", "/task <task_id> --json", null, true],
    ["spawn-task", "Spawn Task", "/spawn-task <worker_id> <prompt> --json", null, true],
    ["ui-task-publisher-plan", "Ui Task Publisher Plan", "POST /api/tasks/plan", null, false],
    ["ui-task-publisher-publish", "Ui Task Publisher Publish", "POST /api/tasks/publish", null, false],
    ["workers", "Workers", "/workers --json", null, false],
    ["operator-console", "Operator Console", "/operator-console --json", "/api/operator-console", false],
    ["subagent-observatory", "Subagent Observatory", "/subagent-observatory --json", "/api/subagent-observatory", false],
    ["task-supervisor", "Task Supervisor", "/task-supervisor --json", null, false],
    ["handoff-bundle", "Handoff Bundle", "/handoff-bundle <task_id> --json", null, false],
    ["task-patches", "Task Patches", "/task-patches <task_id> --json", null, false],
    ["task-loop", "Task Loop", "/task-loop <task_id> --json", null, false],
    ["task-evidence", "Task Evidence", "/task-evidence <task_id> --json", null, false],
    ["task-replay", "Task Replay", "/task-replay <task_id> --json", null, false],
    ["promotion-ledger", "Promotion Ledger", "/promotion-ledger <task_id> --json", null, false],
    ["ops-status", "Ops Status", "/ops-status --json", null, false],
    ["events", "Events", "/events --json", "/api/events", false],
    ["events-report", "Events Report", "/events-report --json", "/api/events-report", false],
    ["activity", "Activity", "/activity --json", "/api/activity", false],
    ["transcript", "Transcript", "/transcript --json", "/api/transcript", false],
    ["agent-send", "Agent Send", "/agent-send <agent_id> --from <from_agent_id> <message> --json", null, false],
    ["ui-agent-chat-plan", "Ui Agent Chat Plan", "POST /api/chat/plan", null, false],
    ["ui-agent-chat-send", "Ui Agent Chat Send", "POST /api/chat", null, false],
    ["query-transcript", "Query Transcript", "/query-transcript <query> --json", null, false],
    ["approvals", "Approvals", "/approvals --json", "/api/approvals", false],
    ["policy", "Policy", "/policy --json", "/api/policy", false],
    ["exec-approvals-apply", "Exec Approvals Apply", "POST /api/approvals/exec/apply", null, false],
    ["operator-security", "Operator Security", "/operator-security --json", "/api/operator-security", false],
    ["gateway-runtime", "Gateway Runtime", "/gateway-runtime --json", "/api/gateway-runtime", false],
    ["gateway-dispatch", "Gateway Dispatch", "/gateway-dispatch --dry-run --json", "/api/gateway-dispatch", false],
    ["gateway-ledger", "Gateway Ledger", "/gateway-ledger --json", "/api/gateway-ledger", false],
    ["gateway-retry-dead-letter", "Gateway Retry Dead Letter", "/gateway-retry-dead-letter --json", "/api/gateway-retry-dead-letter", false],
    ["multi-agent-runtime", "Multi Agent Runtime", "/multi-agent-runtime --agents 4 --messages 8 --json", "/api/multi-agent-runtime", false],
    ["apply-task-patches", "Apply Task Patches", "/apply-task-patches <task_id> --json", null, false],
    ["rollback-task-patches", "Rollback Task Patches", "/rollback-task-patches <task_id> --json", null, false],
    ["ui-readonly-command-runner", "Ui Readonly Command Runner", "POST /api/commands/<id>", null, false],
  ].map(([id, label, command, route, palette]) => Object.freeze({
    id,
    label,
    command,
    route,
    palette,
  })));
  const READ_ONLY_ROUTES = Object.freeze(Object.fromEntries(
    COMMAND_CATALOG.filter(({ route }) => route !== null).map(({ id, route }) => [id, route]),
  ));
  const UNAVAILABLE_PREVIEW_CONTROLS = Object.freeze([
    {
      selector: "[data-plan-action]",
      label: "Unavailable in read-only preview; live adapter not bound",
    },
    {
      selector: "[data-chat-add]",
      label: "New conversation unavailable in read-only preview",
    },
    {
      selector: '[data-chat-row-menu-item="pin"]',
      label: "Pin unavailable in read-only preview",
    },
    {
      selector: '[data-chat-row-menu-item="archive"]',
      label: "Archive unavailable in read-only preview",
    },
    {
      selector: "[data-chat-folder]",
      label: "Conversation folders require the live adapter",
    },
    {
      selector: "[data-agent-chat-send]",
      label: "Send unavailable: live adapter not bound",
    },
    {
      selector: "[data-agent-chat-plan]",
      label: "Plan unavailable: live adapter not bound",
    },
    {
      selector: "[data-chat-routing-mode]",
      label: "Reply routing requires the live adapter",
    },
    {
      selector: "[data-chat-autoscroll-mode]",
      label: "Auto-scroll mode is unavailable in the static preview",
    },
  ]);
  const LOCAL_ROUTE_ACTIONS = Object.freeze({
    "open-evidence": "evidence",
    "open-approvals": "approvals",
    "open-sources": "evidence",
  });
  const READ_ONLY_VIEW_TARGETS = Object.freeze({
    tasks: "screen-card-tasks",
    ops: "screen-card-ops",
    "external-agent-benchmark": "screen-card-external-agent-benchmark",
    evidence: "screen-card-evidence",
    approvals: "screen-card-approvals",
    sessions: "screen-card-sessions",
    transcript: "screen-card-transcript",
    "task-publisher": "screen-card-task-publisher",
  });
  const LOCAL_ARTIFACT_DRAFTS = Object.freeze({
    "evidence-note": "[Evidence note — local draft only]",
    "decision-log": "[Decision log — local draft only]",
  });
  let commandGeneration = 0;
  let activeCommandRequest = null;
  let pendingRouteFocusId = "";

  function validateCommandCatalog() {
    const ids = new Set();
    const routes = new Set();
    for (const entry of COMMAND_CATALOG) {
      if (
        !/^[a-z0-9][a-z0-9-]*$/.test(entry.id)
        || typeof entry.label !== "string"
        || !entry.label.trim()
        || typeof entry.command !== "string"
        || !entry.command.trim()
        || typeof entry.palette !== "boolean"
        || ids.has(entry.id)
      ) {
        throw new Error("Invalid Control UI command catalog entry");
      }
      ids.add(entry.id);
      if (entry.route !== null) {
        if (
          typeof entry.route !== "string"
          || !entry.route.startsWith("/api/")
          || entry.route.includes("?")
          || entry.route.includes("#")
          || routes.has(entry.route)
        ) {
          throw new Error("Invalid Control UI read-only route entry");
        }
        routes.add(entry.route);
      }
    }
    if (COMMAND_CATALOG.length !== 51 || routes.size !== 21) {
      throw new Error("Control UI command catalog cardinality changed");
    }
    if (COMMAND_CATALOG.filter(({ palette }) => palette).length !== 18) {
      throw new Error("Control UI command palette cardinality changed");
    }
  }

  function textElement(tagName, className, text) {
    const node = document.createElement(tagName);
    if (className) {
      node.className = className;
    }
    node.textContent = text;
    return node;
  }

  function renderCommandCatalog() {
    validateCommandCatalog();
    const commandList = document.getElementById("commands");
    const paletteResults = document.getElementById("command-palette-results");
    if (!commandList || !paletteResults) {
      throw new Error("Control UI command catalog mount is missing");
    }

    const commandFragment = document.createDocumentFragment();
    for (const entry of COMMAND_CATALOG) {
      const article = document.createElement("article");
      article.className = "command-item";
      article.dataset.commandId = entry.id;
      article.append(textElement("strong", "", entry.label));
      article.append(textElement("code", "", entry.command));

      const actions = document.createElement("div");
      actions.className = "action-rail";
      const copyButton = textElement("button", "button small", "Copy");
      copyButton.type = "button";
      copyButton.dataset.copy = entry.command;
      actions.append(copyButton);
      if (entry.route) {
        const runButton = textElement("button", "button small", "Run read-only");
        runButton.type = "button";
        runButton.dataset.controlUiActionControl = "read-only-command";
        runButton.dataset.runCommand = "read-only";
        runButton.setAttribute("aria-label", "Run read-only command");
        runButton.title = "Run read-only command";
        actions.append(runButton);
      } else {
        actions.append(textElement("span", "chip chip--muted", "copy-only / guarded"));
      }
      article.append(actions);
      commandFragment.append(article);
    }
    commandList.replaceChildren(commandFragment);
    commandList.dataset.controlUiCatalogSource = "typed-command-catalog-v1";

    const paletteFragment = document.createDocumentFragment();
    for (const [index, entry] of COMMAND_CATALOG.filter(({ palette }) => palette).entries()) {
      const item = document.createElement("a");
      item.className = `command-palette__item${index === 0 ? " command-palette__item--audit-hover" : ""}`;
      item.dataset.controlUiCommandPaletteItem = entry.id;
      item.dataset.controlUiCommandPaletteResult = "light-glass";
      item.dataset.paletteKind = "command";
      item.dataset.paletteId = entry.id;
      item.href = "#commands";
      const accessibleLabel = `Open command result: ${entry.command}`;
      item.setAttribute("aria-label", accessibleLabel);
      item.title = accessibleLabel;
      const kind = textElement("span", "command-palette__kind", "command");
      const copy = document.createElement("span");
      copy.className = "command-palette__copy";
      copy.append(textElement("strong", "", entry.label));
      copy.append(textElement("small", "", entry.command));
      item.append(kind, copy);
      paletteFragment.append(item);
    }
    paletteResults.replaceChildren(paletteFragment);
    paletteResults.dataset.controlUiCatalogSource = "typed-command-catalog-v1";

    document.documentElement.dataset.controlUiCommandCatalogCount = String(COMMAND_CATALOG.length);
    document.documentElement.dataset.controlUiCommandPaletteCount = String(
      COMMAND_CATALOG.filter(({ palette }) => palette).length,
    );
  }

  function isEmptyPayload(value) {
    if (value === null || value === undefined) {
      return true;
    }
    if (Array.isArray(value)) {
      return value.length === 0;
    }
    return typeof value === "object" && Object.keys(value).length === 0;
  }

  function safeMessage(error) {
    const message = error instanceof Error ? error.message : String(error);
    return message.slice(0, 240);
  }

  function isJsonMediaType(contentType) {
    const mediaType = contentType.split(";", 1)[0].trim().toLowerCase();
    return mediaType === "application/json" || JSON_MEDIA_TYPE.test(mediaType);
  }

  function responseContentLength(response) {
    const value = response.headers.get("content-length");
    if (value === null || !/^\d+$/.test(value.trim())) {
      return null;
    }
    const length = Number(value);
    return Number.isSafeInteger(length) ? length : Number.POSITIVE_INFINITY;
  }

  async function readBoundedResponseBytes(response) {
    const reader = response.body?.getReader();
    if (!reader) {
      throw new Error("Bounded response streaming is unavailable");
    }
    const chunks = [];
    let byteLength = 0;
    let cancelled = false;
    try {
      while (true) {
        const { done, value } = await reader.read();
        if (done) {
          break;
        }
        byteLength += value.byteLength;
        if (byteLength > MAX_RESPONSE_BYTES) {
          cancelled = true;
          try {
            await reader.cancel("Response exceeded the local display limit");
          } catch (_cancelError) {
            // The byte boundary is authoritative even if transport cancellation races.
          }
          throw new Error("Response exceeded the local display limit");
        }
        chunks.push(value);
      }
    } catch (error) {
      if (!cancelled) {
        try {
          await reader.cancel();
        } catch (_cancelError) {
          // Preserve the original read or validation error.
        }
      }
      throw error;
    } finally {
      reader.releaseLock();
    }

    const bodyBytes = new Uint8Array(byteLength);
    let offset = 0;
    for (const chunk of chunks) {
      bodyBytes.set(chunk, offset);
      offset += chunk.byteLength;
    }
    return bodyBytes;
  }

  async function getSameOriginJson(path, requestController = new AbortController()) {
    if (!Object.values(READ_ONLY_ROUTES).includes(path) && path !== SNAPSHOT_PATH) {
      throw new Error("Request is not in the read-only registry");
    }

    const url = new URL(path, window.location.origin);
    if (url.origin !== window.location.origin || url.pathname !== path || url.search || url.hash) {
      throw new Error("Cross-origin or non-canonical request blocked");
    }

    const timeout = window.setTimeout(
      () => requestController.abort(new DOMException("Request timed out", "TimeoutError")),
      REQUEST_TIMEOUT_MS,
    );
    try {
      const response = await fetch(url.href, {
        method: "GET",
        credentials: "same-origin",
        mode: "same-origin",
        cache: "no-store",
        redirect: "error",
        headers: { Accept: "application/json" },
        signal: requestController.signal,
      });
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }
      const contentType = response.headers.get("content-type") || "";
      if (!isJsonMediaType(contentType)) {
        throw new Error("Response is not JSON");
      }
      const contentLength = responseContentLength(response);
      if (contentLength !== null && contentLength > MAX_RESPONSE_BYTES) {
        throw new Error("Response exceeded the local display limit");
      }
      const bodyBytes = await readBoundedResponseBytes(response);
      let body;
      try {
        body = new TextDecoder("utf-8", { fatal: true }).decode(bodyBytes);
      } catch (_error) {
        throw new Error("Response is not valid UTF-8");
      }
      if (!body.trim()) {
        return { data: null, empty: true };
      }
      const data = JSON.parse(body);
      return { data, empty: isEmptyPayload(data) };
    } finally {
      window.clearTimeout(timeout);
    }
  }

  function setStatus(node, state, message) {
    node.dataset.state = state;
    node.textContent = message;
  }

  function ensureSnapshotStatus() {
    const existing = document.getElementById("operator-snapshot-status");
    if (existing) {
      return existing;
    }
    const status = document.createElement("p");
    status.id = "operator-snapshot-status";
    status.className = "muted";
    status.setAttribute("role", "status");
    status.setAttribute("aria-live", "polite");
    status.dataset.controlUiOperatorSnapshot = "read-only";
    const output = document.getElementById("command-runner-output");
    output?.parentNode?.insertBefore(status, output);
    return status;
  }

  async function hydrateOperatorSnapshot() {
    const status = ensureSnapshotStatus();
    setStatus(status, "loading", "Loading local operator snapshot…");
    try {
      const result = await getSameOriginJson(SNAPSHOT_PATH);
      if (result.empty) {
        setStatus(status, "empty", "Operator snapshot returned no data.");
        return;
      }
      const state = typeof result.data?.status === "string" ? result.data.status : "available";
      setStatus(status, "ready", `Operator snapshot: ${state}.`);
    } catch (error) {
      setStatus(status, "error", `Operator snapshot unavailable: ${safeMessage(error)}`);
    }
  }

  function commandIdFor(button) {
    return button.closest("[data-command-id]")?.dataset.commandId || "";
  }

  function configureReadOnlyButtons() {
    const buttons = document.querySelectorAll('[data-run-command="read-only"]');
    for (const button of buttons) {
      const commandId = commandIdFor(button);
      const allowed = Object.prototype.hasOwnProperty.call(READ_ONLY_ROUTES, commandId);
      button.disabled = !allowed;
      button.setAttribute("aria-disabled", String(!allowed));
      button.dataset.readOnlyRegistry = allowed ? "allowed" : "copy-only";
      if (!allowed) {
        button.title = "Copy-only: no canonical read-only route is registered";
      }
    }
    document.documentElement.dataset.controlUiReadOnlyRegistryCount = String(
      Object.keys(READ_ONLY_ROUTES).length,
    );
  }

  function configureUnavailablePreviewControls() {
    for (const control of UNAVAILABLE_PREVIEW_CONTROLS) {
      for (const node of document.querySelectorAll(control.selector)) {
        if (!(node instanceof HTMLButtonElement || node instanceof HTMLSelectElement)) {
          continue;
        }
        node.disabled = true;
        node.setAttribute("aria-disabled", "true");
        node.setAttribute("aria-label", control.label);
        node.title = control.label;
        node.dataset.controlUiUnavailable = "live-adapter";
      }
    }

    for (const conversation of document.querySelectorAll("[data-chat-conversation]")) {
      conversation.removeAttribute("aria-disabled");
      conversation.setAttribute("title", "Seeded read-only conversation preview");
      conversation.tabIndex = -1;
      conversation.dataset.controlUiConversationReadonly = "true";
      conversation.dataset.controlUiConversationMode = "seeded-read-only";
    }

    const composer = document.getElementById("chat-message");
    if (composer instanceof HTMLTextAreaElement) {
      composer.placeholder = "Local draft only";
      composer.setAttribute("aria-label", "Local draft; sending unavailable");
      composer.title = "Local draft only; live adapter not bound";
      composer.dataset.controlUiComposerMode = "local-draft-only";
    }

    const taskPublisherLink = document.querySelector(
      '[data-control-ui-action-control="new-task"]',
    );
    if (taskPublisherLink instanceof HTMLAnchorElement) {
      const label = "View task publisher contract (read-only)";
      taskPublisherLink.setAttribute("aria-label", label);
      taskPublisherLink.title = label;
      taskPublisherLink.dataset.controlUiActionControl = "task-publisher-catalog";
      const text = taskPublisherLink.querySelector("span:last-child");
      if (text) {
        text.textContent = "Task spec";
      }
    }

    const status = document.querySelector("[data-chat-send-state]");
    if (status) {
      status.dataset.chatSendState = "read-only";
      status.textContent = "read-only · live adapter not bound";
    }
  }

  function setComposerStatus(message) {
    const status = document.querySelector("[data-chat-send-state]");
    if (!status) {
      return;
    }
    status.dataset.chatSendState = "local-draft";
    status.textContent = message;
  }

  function insertLocalDraftText(text) {
    const composer = document.getElementById("chat-message");
    if (!(composer instanceof HTMLTextAreaElement)) {
      return;
    }
    const prefix = composer.value && !composer.value.endsWith("\n") ? "\n" : "";
    const insertion = `${prefix}${text}`;
    composer.setRangeText(insertion, composer.selectionStart, composer.selectionEnd, "end");
    composer.dispatchEvent(new Event("input", { bubbles: true }));
    composer.focus();
    setComposerStatus("local draft updated · not sent");
  }

  function closeOwningPopover(node) {
    const popover = node.closest("[popover]");
    if (popover && typeof popover.hidePopover === "function") {
      popover.hidePopover();
    }
  }

  function routeStateFromHash() {
    const key = window.location.hash.replace(/^#/, "");
    if (!key || ["chat", "chat-list", "chat-thread", "chat-room", "command-palette"].includes(key)) {
      return { key: "chat", targetId: "chat-thread" };
    }
    if (key === "commands" || key === "hepta-command-panel") {
      return { key: "commands", targetId: "hepta-command-panel" };
    }
    if (Object.prototype.hasOwnProperty.call(READ_ONLY_VIEW_TARGETS, key)) {
      return { key, targetId: READ_ONLY_VIEW_TARGETS[key] };
    }
    const target = document.getElementById(key);
    const routeCard = target?.matches(".route-card") ? target : target?.closest(".route-card");
    if (routeCard) {
      return {
        key: routeCard.dataset.screen || routeCard.id.replace(/^screen-card-/, ""),
        targetId: routeCard.id,
      };
    }
    if (target?.closest("#hepta-command-panel")) {
      return { key: "commands", targetId: "hepta-command-panel" };
    }
    return { key: "chat", targetId: "chat-thread" };
  }

  function updateNavigationState(viewKey) {
    for (const link of document.querySelectorAll("#hepta-nav [data-screen]")) {
      const active = link.dataset.screen === viewKey;
      link.classList.toggle("active", active);
      if (active) {
        link.setAttribute("aria-current", "page");
      } else {
        link.removeAttribute("aria-current");
      }
    }
  }

  function mountPrimaryNavigation(commandPanel, showingCommandCatalog) {
    const navigation = document.getElementById("hepta-nav");
    const host = showingCommandCatalog
      ? commandPanel?.querySelector(".panel-heading")
      : document.querySelector(".tg-thread-status");
    if (!navigation || !host) {
      return;
    }
    host.prepend(navigation);
    navigation.dataset.controlUiNavMount = showingCommandCatalog ? "commands" : "thread";
  }

  function focusRouteTarget(target) {
    const requested = pendingRouteFocusId && document.getElementById(pendingRouteFocusId);
    pendingRouteFocusId = "";
    const focusTarget = requested && !requested.hidden ? requested : target;
    if (!(focusTarget instanceof HTMLElement)) {
      return;
    }
    if (!focusTarget.hasAttribute("tabindex")) {
      focusTarget.tabIndex = -1;
      focusTarget.dataset.controlUiProgrammaticFocus = "true";
    }
    window.requestAnimationFrame(() => {
      focusTarget.focus({ preventScroll: true });
      focusTarget.scrollIntoView({ block: "nearest", inline: "nearest" });
    });
  }

  function syncRouteView({ focus = true } = {}) {
    const body = document.body;
    const secondaryMap = document.querySelector(".hepta-secondary-map");
    const screenContracts = document.querySelector(".hepta-all-screen-contracts");
    const routeCards = [...document.querySelectorAll(".hepta-all-screen-contracts .route-card")];
    const screenPanel = document.getElementById("hepta-screen-panel");
    const commandPanel = document.getElementById("hepta-command-panel");
    const chatThread = document.getElementById("chat-thread");
    const chatSidePanels = [
      document.getElementById("chat-list"),
      document.getElementById("chat-room"),
    ].filter(Boolean);
    const chatThreadChildren = chatThread ? [...chatThread.children] : [];
    const focusHeader = document.querySelector(".focus-workspace>.focus-header");
    const focusMain = document.querySelector(".focus-workspace>.focus-main");
    const entrySurface = document.querySelector(".hepta-secondary-map .hepta-entry-surface");
    const routeOutlet = screenContracts?.parentElement;
    const entrySurfaceChildren = entrySurface ? [...entrySurface.children] : [];
    const routeOutletChildren = routeOutlet ? [...routeOutlet.children] : [];
    const secondaryPanels = [
      document.querySelector(".dashboard-hero"),
      document.getElementById("hepta-metrics"),
      document.querySelector(".hepta-dashboard-recovery"),
      document.querySelector(".evidence-panel"),
    ].filter(Boolean);
    const route = routeStateFromHash();
    const target = document.getElementById(route.targetId);
    const showingCommands = route.targetId === "hepta-command-panel";

    body.dataset.view = route.key === "chat" ? "chat" : showingCommands ? "commands" : "read-only";
    body.dataset.controlUiActiveView = route.key;
    mountPrimaryNavigation(commandPanel, showingCommands);
    updateNavigationState(route.key);

    if (route.key === "chat") {
      for (const panel of secondaryPanels) {
        panel.hidden = false;
      }
      if (screenPanel) screenPanel.hidden = false;
      if (commandPanel) commandPanel.hidden = false;
      if (focusHeader) focusHeader.hidden = false;
      if (focusMain) focusMain.hidden = false;
      if (entrySurface) entrySurface.hidden = false;
      for (const child of entrySurfaceChildren) child.hidden = false;
      for (const child of routeOutletChildren) child.hidden = false;
      for (const panel of chatSidePanels) panel.hidden = false;
      for (const child of chatThreadChildren) child.hidden = false;
      if (secondaryMap instanceof HTMLDetailsElement) secondaryMap.open = false;
      if (screenContracts instanceof HTMLDetailsElement) screenContracts.open = false;
      for (const card of routeCards) {
        card.hidden = false;
        delete card.dataset.controlUiActiveView;
      }
      if (focus) {
        focusRouteTarget(target);
      }
      return;
    }

    for (const panel of secondaryPanels) {
      panel.hidden = true;
    }
    if (screenPanel) screenPanel.hidden = showingCommands;
    if (commandPanel) commandPanel.hidden = !showingCommands;
    if (focusHeader) focusHeader.hidden = !showingCommands;
    if (focusMain) focusMain.hidden = showingCommands;
    if (entrySurface) entrySurface.hidden = false;
    for (const child of entrySurfaceChildren) {
      child.hidden = child !== routeOutlet;
    }
    for (const child of routeOutletChildren) {
      child.hidden = child !== screenContracts;
    }
    for (const panel of chatSidePanels) panel.hidden = true;
    for (const child of chatThreadChildren) {
      child.hidden = showingCommands || !child.matches(".tg-thread-header,.hepta-secondary-map");
    }

    if (secondaryMap instanceof HTMLDetailsElement) {
      secondaryMap.open = !showingCommands;
    }
    if (screenContracts instanceof HTMLDetailsElement) {
      screenContracts.open = !showingCommands;
    }
    for (const card of routeCards) {
      const active = !showingCommands && card.id === route.targetId;
      card.hidden = !active;
      if (active) {
        card.dataset.controlUiActiveView = "true";
      } else {
        delete card.dataset.controlUiActiveView;
      }
    }
    if (focus) {
      focusRouteTarget(target);
    }
  }

  function navigateToView(viewKey, focusId = "") {
    pendingRouteFocusId = focusId;
    const nextHash = viewKey === "commands" ? "#commands" : `#${viewKey}`;
    if (window.location.hash === nextHash) {
      syncRouteView();
      return;
    }
    window.location.hash = nextHash;
  }

  function configureRouteViews() {
    window.addEventListener("hashchange", () => syncRouteView());
    syncRouteView({ focus: Boolean(window.location.hash) });
  }

  function configureLocalJsonPreview() {
    const input = document.getElementById("json-input");
    const preview = document.getElementById("json-preview");
    if (!(input instanceof HTMLTextAreaElement) || !preview) {
      return;
    }
    input.addEventListener("input", () => {
      const source = input.value.trim();
      if (!source) {
        setStatus(preview, "empty", "Paste JSON for local inspection. Nothing is uploaded.");
        return;
      }
      try {
        preview.textContent = JSON.stringify(JSON.parse(source), null, 2);
        preview.dataset.state = "ready";
      } catch (error) {
        setStatus(preview, "error", `Invalid JSON: ${safeMessage(error)}`);
      }
    });
  }

  function configureComposerPickerSearch() {
    for (const input of document.querySelectorAll("[data-chat-composer-picker-search]")) {
      input.addEventListener("input", () => {
        const query = input.value.trim().toLocaleLowerCase();
        const popover = input.closest("[data-chat-composer-popover]");
        for (const item of popover?.querySelectorAll("[data-chat-composer-picker-item]") || []) {
          item.hidden = Boolean(query) && !item.textContent.toLocaleLowerCase().includes(query);
        }
      });
    }
  }

  function supersedeActiveCommandRequest() {
    const previous = activeCommandRequest;
    if (!previous) {
      return;
    }
    activeCommandRequest = null;
    previous.controller.abort(
      new DOMException("Superseded by a newer command", "AbortError"),
    );
    previous.card?.setAttribute("data-command-state", "superseded");
    if (previous.button.dataset.commandGeneration === String(previous.generation)) {
      previous.button.disabled = false;
      previous.button.removeAttribute("aria-busy");
      delete previous.button.dataset.commandGeneration;
    }
  }

  function commandRequestIsCurrent(generation, controller) {
    return (
      activeCommandRequest?.generation === generation &&
      activeCommandRequest.controller === controller
    );
  }

  async function runReadOnlyCommand(button) {
    const commandId = commandIdFor(button);
    const path = READ_ONLY_ROUTES[commandId];
    if (!path) {
      return;
    }

    const output = document.getElementById("command-runner-output");
    if (!output) {
      return;
    }
    const card = button.closest("[data-command-id]");
    supersedeActiveCommandRequest();
    const generation = ++commandGeneration;
    const controller = new AbortController();
    activeCommandRequest = { generation, controller, button, card, path };
    button.disabled = true;
    button.setAttribute("aria-busy", "true");
    button.dataset.commandGeneration = String(generation);
    card?.setAttribute("data-command-state", "loading");
    output.dataset.sourcePath = path;
    setStatus(output, "loading", `Loading ${path}…`);
    try {
      const result = await getSameOriginJson(path, controller);
      if (!commandRequestIsCurrent(generation, controller)) {
        return;
      }
      output.textContent = JSON.stringify({ source_path: path, data: result.data }, null, 2);
      output.dataset.state = result.empty ? "empty" : "ready";
      card?.setAttribute("data-command-state", result.empty ? "empty" : "ready");
    } catch (error) {
      if (!commandRequestIsCurrent(generation, controller)) {
        return;
      }
      setStatus(output, "error", `Unable to load ${path}: ${safeMessage(error)}`);
      card?.setAttribute("data-command-state", "error");
    } finally {
      if (button.dataset.commandGeneration === String(generation)) {
        button.disabled = false;
        button.removeAttribute("aria-busy");
        delete button.dataset.commandGeneration;
      }
      if (commandRequestIsCurrent(generation, controller)) {
        activeCommandRequest = null;
      }
    }
  }

  function copyStatus(message) {
    const toast = document.getElementById("toast");
    if (toast) {
      toast.textContent = message;
      toast.setAttribute("role", "status");
      toast.dataset.state = "ready";
    }
  }

  async function copyText(text) {
    if (navigator.clipboard?.writeText) {
      try {
        await navigator.clipboard.writeText(text);
        return;
      } catch (_error) {
        // Fall through to the bounded local compatibility path.
      }
    }
    const textarea = document.createElement("textarea");
    textarea.value = text;
    textarea.readOnly = true;
    textarea.style.position = "fixed";
    textarea.style.opacity = "0";
    document.body.appendChild(textarea);
    textarea.select();
    const copied = document.execCommand("copy");
    textarea.remove();
    if (!copied) {
      throw new Error("Clipboard API unavailable");
    }
  }

  function configureLocalInteractions() {
    document.addEventListener("click", (event) => {
      const target = event.target instanceof Element ? event.target : null;
      const localNavigationLink = target?.closest(
        'a[data-screen][href^="#"], a[data-control-ui-menu-item][href^="#"]',
      );
      if (
        localNavigationLink instanceof HTMLAnchorElement
        && localNavigationLink.hash === window.location.hash
      ) {
        window.requestAnimationFrame(() => syncRouteView());
      }
      const copyButton = target?.closest("[data-copy]");
      if (copyButton) {
        event.preventDefault();
        const text = copyButton.dataset.copy || "";
        copyText(text)
          .then(() => copyStatus("Copied to clipboard."))
          .catch((error) => copyStatus(`Copy unavailable: ${safeMessage(error)}`));
        return;
      }

      const paletteItem = target?.closest("[data-control-ui-command-palette-item]");
      if (paletteItem) {
        event.preventDefault();
        const commandId = paletteItem.dataset.controlUiCommandPaletteItem || "";
        const commandCard = document.querySelector(`[data-command-id="${commandId}"]`);
        if (commandCard instanceof HTMLElement && !commandCard.id) {
          commandCard.id = `command-${commandId}`;
        }
        closeOwningPopover(paletteItem);
        navigateToView("commands", commandCard?.id || "");
        copyStatus("Opened the local read-only command catalog.");
        return;
      }

      const localRouteButton = target?.closest("[data-chat-row-menu-item]");
      const localRoute = localRouteButton
        ? LOCAL_ROUTE_ACTIONS[localRouteButton.dataset.chatRowMenuItem]
        : "";
      if (localRoute) {
        event.preventDefault();
        closeOwningPopover(localRouteButton);
        navigateToView(localRoute);
        copyStatus("Opened a local read-only surface.");
        return;
      }

      const artifactButton = target?.closest("[data-chat-artifact-insert]");
      if (artifactButton) {
        event.preventDefault();
        const draft = LOCAL_ARTIFACT_DRAFTS[artifactButton.dataset.chatArtifactInsert];
        if (draft) {
          insertLocalDraftText(draft);
          closeOwningPopover(artifactButton);
        }
        return;
      }

      const commandButton = target?.closest("[data-chat-command-insert]");
      if (commandButton) {
        event.preventDefault();
        insertLocalDraftText(commandButton.dataset.chatCommandInsert || "");
        closeOwningPopover(commandButton);
        return;
      }

      const runButton = target?.closest('[data-run-command="read-only"]');
      if (
        runButton &&
        !runButton.disabled &&
        runButton.dataset.readOnlyRegistry === "allowed"
      ) {
        event.preventDefault();
        void runReadOnlyCommand(runButton);
      }
    });

    const chatSearch = document.getElementById("chat-search");
    chatSearch?.addEventListener("input", () => {
      const query = chatSearch.value.trim().toLocaleLowerCase();
      for (const item of document.querySelectorAll(".tg-chat-item")) {
        item.hidden = Boolean(query) && !item.textContent.toLocaleLowerCase().includes(query);
      }
    });

    const paletteSearch = document.getElementById("command-palette-input");
    paletteSearch?.addEventListener("input", () => {
      const query = paletteSearch.value.trim().toLocaleLowerCase();
      for (const item of document.querySelectorAll("#command-palette-results .command-palette__item")) {
        item.hidden = Boolean(query) && !item.textContent.toLocaleLowerCase().includes(query);
      }
    });
  }

  renderCommandCatalog();
  configureUnavailablePreviewControls();
  configureReadOnlyButtons();
  configureLocalInteractions();
  configureLocalJsonPreview();
  configureComposerPickerSearch();
  configureRouteViews();
  document.documentElement.dataset.controlUiProgressiveEnhancement = "ready";
  document.documentElement.dataset.controlUiCapabilityMode = "local-read-only";
  void hydrateOperatorSnapshot();
})();
