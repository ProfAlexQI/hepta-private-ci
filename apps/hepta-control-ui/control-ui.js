"use strict";

(() => {
  const REQUEST_TIMEOUT_MS = 8000;
  const MAX_RESPONSE_BYTES = 2 * 1024 * 1024;
  const SNAPSHOT_PATH = "/api/operator-snapshot";
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

  async function getSameOriginJson(path) {
    if (!Object.values(READ_ONLY_ROUTES).includes(path) && path !== SNAPSHOT_PATH) {
      throw new Error("Request is not in the read-only registry");
    }

    const url = new URL(path, window.location.origin);
    if (url.origin !== window.location.origin || url.pathname !== path || url.search || url.hash) {
      throw new Error("Cross-origin or non-canonical request blocked");
    }

    const controller = new AbortController();
    const timeout = window.setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);
    try {
      const response = await fetch(url.href, {
        method: "GET",
        credentials: "same-origin",
        mode: "same-origin",
        cache: "no-store",
        redirect: "error",
        headers: { Accept: "application/json" },
        signal: controller.signal,
      });
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }
      const contentType = response.headers.get("content-type") || "";
      if (!contentType.toLowerCase().startsWith("application/json")) {
        throw new Error("Response is not JSON");
      }
      const body = await response.text();
      if (body.length > MAX_RESPONSE_BYTES) {
        throw new Error("Response exceeded the local display limit");
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
    button.disabled = true;
    button.setAttribute("aria-busy", "true");
    card?.setAttribute("data-command-state", "loading");
    setStatus(output, "loading", `Loading ${path}…`);
    try {
      const result = await getSameOriginJson(path);
      output.textContent = JSON.stringify(result.data, null, 2);
      output.dataset.state = result.empty ? "empty" : "ready";
      card?.setAttribute("data-command-state", result.empty ? "empty" : "ready");
    } catch (error) {
      setStatus(output, "error", `Unable to load ${path}: ${safeMessage(error)}`);
      card?.setAttribute("data-command-state", "error");
    } finally {
      button.disabled = false;
      button.removeAttribute("aria-busy");
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

      const runButton = target?.closest('[data-run-command="read-only"]');
      if (runButton && runButton.dataset.readOnlyRegistry === "allowed") {
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

  configureReadOnlyButtons();
  configureLocalInteractions();
  document.documentElement.dataset.controlUiProgressiveEnhancement = "ready";
  void hydrateOperatorSnapshot();
})();
