"use strict";

(() => {
  const REQUEST_TIMEOUT_MS = 8000;
  const MAX_RESPONSE_BYTES = 2 * 1024 * 1024;
  const SNAPSHOT_PATH = "/api/operator-snapshot";
  const JSON_MEDIA_TYPE = /^application\/[a-z0-9!#$%&'*+.^_`|~-]+\+json$/;
  const READ_ONLY_ROUTES = Object.freeze({
    "control-ui": "/api/control-ui",
    "config-surface": "/api/config",
    "optional-configs": "/api/optional-configs",
    "hepta-merge-completion": "/api/hepta-merge-completion",
    "external-agent-benchmark": "/api/external-agent-benchmark",
    sessions: "/api/sessions",
    "session-activity": "/api/session-activity",
    "operator-console": "/api/operator-console",
    "subagent-observatory": "/api/subagent-observatory",
    events: "/api/events",
    "events-report": "/api/events-report",
    activity: "/api/activity",
    transcript: "/api/transcript",
    approvals: "/api/approvals",
    policy: "/api/policy",
    "operator-security": "/api/operator-security",
    "gateway-runtime": "/api/gateway-runtime",
    "gateway-dispatch": "/api/gateway-dispatch",
    "gateway-ledger": "/api/gateway-ledger",
    "gateway-retry-dead-letter": "/api/gateway-retry-dead-letter",
    "multi-agent-runtime": "/api/multi-agent-runtime",
  });
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
    "open-evidence": "#evidence",
    "open-approvals": "#approvals",
    "open-sources": "#evidence",
  });
  const LOCAL_ARTIFACT_DRAFTS = Object.freeze({
    "evidence-note": "[Evidence note — local draft only]",
    "decision-log": "[Decision log — local draft only]",
  });
  let commandGeneration = 0;
  let activeCommandRequest = null;

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
      conversation.setAttribute("aria-disabled", "true");
      conversation.setAttribute("title", "Seeded read-only conversation preview");
      conversation.tabIndex = -1;
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
      const copyButton = target?.closest("[data-copy]");
      if (copyButton) {
        event.preventDefault();
        const text = copyButton.dataset.copy || "";
        copyText(text)
          .then(() => copyStatus("Copied to clipboard."))
          .catch((error) => copyStatus(`Copy unavailable: ${safeMessage(error)}`));
        return;
      }

      const localRouteButton = target?.closest("[data-chat-row-menu-item]");
      const localRoute = localRouteButton
        ? LOCAL_ROUTE_ACTIONS[localRouteButton.dataset.chatRowMenuItem]
        : "";
      if (localRoute) {
        event.preventDefault();
        window.location.hash = localRoute;
        closeOwningPopover(localRouteButton);
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

  configureUnavailablePreviewControls();
  configureReadOnlyButtons();
  configureLocalInteractions();
  configureLocalJsonPreview();
  configureComposerPickerSearch();
  document.documentElement.dataset.controlUiProgressiveEnhancement = "ready";
  document.documentElement.dataset.controlUiCapabilityMode = "local-read-only";
  void hydrateOperatorSnapshot();
})();
