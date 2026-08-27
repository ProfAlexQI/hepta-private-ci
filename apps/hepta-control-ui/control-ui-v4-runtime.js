"use strict";

(() => {
  const MOBILE_QUERY = "(max-width: 700px)";
  const HISTORY_KEY = "__heptaUiV4Transient";
  const READ_STATES = Object.freeze([
    "idle",
    "loading",
    "fresh",
    "stale",
    "partial",
    "offline",
    "error",
  ]);
  const READ_STATE_SET = new Set(READ_STATES);
  const FOCUSABLE_SELECTOR = [
    "a[href]",
    "button:not([disabled])",
    "input:not([disabled])",
    "select:not([disabled])",
    "textarea:not([disabled])",
    "summary",
    "[tabindex]:not([tabindex='-1'])",
  ].join(",");

  const mobileMedia = window.matchMedia(MOBILE_QUERY);
  let activeLayer = null;
  let activeOpener = null;
  let lockedScrollY = 0;
  let historyEntryActive = false;
  let closingFromPopState = false;

  function isVisible(node) {
    return node instanceof HTMLElement
      && !node.hidden
      && node.getAttribute("aria-hidden") !== "true"
      && node.getClientRects().length > 0;
  }

  function focusables(layer) {
    return [...layer.querySelectorAll(FOCUSABLE_SELECTOR)].filter(isVisible);
  }

  function openerFor(layer) {
    if (layer.id) {
      const explicit = document.querySelector(`[popovertarget="${CSS.escape(layer.id)}"]`);
      if (explicit instanceof HTMLElement) return explicit;
    }
    return document.activeElement instanceof HTMLElement ? document.activeElement : null;
  }

  function lockDocumentScroll() {
    if (document.documentElement.dataset.heptaV4TransientOpen === "true") return;
    lockedScrollY = window.scrollY;
    document.documentElement.dataset.heptaV4TransientOpen = "true";
    document.body.dataset.heptaV4ScrollLock = "true";
    document.body.style.position = "fixed";
    document.body.style.inset = `-${lockedScrollY}px 0 auto 0`;
    document.body.style.width = "100%";
  }

  function unlockDocumentScroll() {
    if (document.documentElement.dataset.heptaV4TransientOpen !== "true") return;
    delete document.documentElement.dataset.heptaV4TransientOpen;
    delete document.body.dataset.heptaV4ScrollLock;
    document.body.style.position = "";
    document.body.style.inset = "";
    document.body.style.width = "";
    window.scrollTo({ top: lockedScrollY, left: 0, behavior: "instant" });
  }

  function consumeHistoryEntry() {
    if (!historyEntryActive || closingFromPopState) return;
    historyEntryActive = false;
    history.back();
  }

  function activateLayer(layer) {
    activeLayer = layer;
    activeOpener = openerFor(layer);
    layer.dataset.heptaV4ActiveTransient = "true";
    if (mobileMedia.matches) {
      lockDocumentScroll();
      if (!historyEntryActive) {
        history.pushState(
          { ...history.state, [HISTORY_KEY]: layer.id || "anonymous" },
          "",
          window.location.href,
        );
        historyEntryActive = true;
      }
    }
    window.requestAnimationFrame(() => {
      const target = layer.querySelector("[autofocus]") || focusables(layer)[0] || layer;
      if (target instanceof HTMLElement) {
        if (!target.hasAttribute("tabindex") && target === layer) target.tabIndex = -1;
        target.focus({ preventScroll: true });
      }
    });
  }

  function deactivateLayer(layer, { restoreFocus = true } = {}) {
    delete layer.dataset.heptaV4ActiveTransient;
    if (activeLayer === layer) activeLayer = null;
    unlockDocumentScroll();
    consumeHistoryEntry();
    const opener = activeOpener;
    activeOpener = null;
    if (restoreFocus && isVisible(opener)) {
      window.requestAnimationFrame(() => opener.focus({ preventScroll: true }));
    }
  }

  function hideActiveLayer({ fromPopState = false } = {}) {
    const layer = activeLayer;
    if (!layer) return false;
    closingFromPopState = fromPopState;
    try {
      if (typeof layer.hidePopover === "function" && layer.matches(":popover-open")) {
        layer.hidePopover();
      } else if (layer instanceof HTMLDialogElement && layer.open) {
        layer.close();
      } else {
        deactivateLayer(layer);
      }
    } finally {
      closingFromPopState = false;
    }
    return true;
  }

  function handleLayerToggle(event) {
    const layer = event.currentTarget;
    if (!(layer instanceof HTMLElement)) return;
    const open = event.newState === "open" || layer.matches(":popover-open");
    if (open) activateLayer(layer);
    else deactivateLayer(layer);
  }

  function handleKeydown(event) {
    if (!activeLayer) return;
    if (event.key === "Escape") {
      event.preventDefault();
      hideActiveLayer();
      return;
    }
    if (event.key !== "Tab" || !mobileMedia.matches) return;
    const candidates = focusables(activeLayer);
    if (candidates.length === 0) {
      event.preventDefault();
      activeLayer.focus({ preventScroll: true });
      return;
    }
    const first = candidates[0];
    const last = candidates[candidates.length - 1];
    if (event.shiftKey && document.activeElement === first) {
      event.preventDefault();
      last.focus();
    } else if (!event.shiftKey && document.activeElement === last) {
      event.preventDefault();
      first.focus();
    }
  }

  function attachLayer(layer) {
    if (!(layer instanceof HTMLElement) || layer.dataset.heptaV4RuntimeAttached === "true") return;
    layer.dataset.heptaV4RuntimeAttached = "true";
    layer.addEventListener("toggle", handleLayerToggle);
    if (layer instanceof HTMLDialogElement) {
      layer.addEventListener("close", () => deactivateLayer(layer));
    }
  }

  function attachLayers(root = document) {
    for (const layer of root.querySelectorAll("[popover], dialog")) attachLayer(layer);
  }

  function handleMediaChange() {
    if (!mobileMedia.matches) {
      unlockDocumentScroll();
      if (historyEntryActive) consumeHistoryEntry();
    } else if (activeLayer) {
      lockDocumentScroll();
    }
  }

  function boundedText(value, maximum = 240) {
    return String(value ?? "").trim().slice(0, maximum);
  }

  function validDigest(value) {
    const digest = boundedText(value, 64).toLowerCase();
    return /^[0-9a-f]{64}$/.test(digest) ? digest : "";
  }

  function applyReadState(target, state, metadata = {}) {
    if (!(target instanceof HTMLElement)) throw new TypeError("read-state target must be an HTMLElement");
    if (!READ_STATE_SET.has(state)) throw new TypeError(`unsupported read state: ${state}`);

    target.dataset.controlUiReadState = state;
    target.setAttribute("aria-busy", String(state === "loading"));
    target.setAttribute("aria-live", state === "error" || state === "offline" ? "assertive" : "polite");

    const source = boundedText(metadata.source, 120);
    const freshness = boundedText(metadata.freshness, 80);
    const digest = validDigest(metadata.evidenceDigest);
    if (source) target.dataset.controlUiReadSource = source;
    else delete target.dataset.controlUiReadSource;
    if (freshness) target.dataset.controlUiReadFreshness = freshness;
    else delete target.dataset.controlUiReadFreshness;
    if (digest) target.dataset.controlUiEvidenceDigest = digest;
    else delete target.dataset.controlUiEvidenceDigest;

    const message = boundedText(metadata.message, 240);
    const messageTarget = target.querySelector("[data-control-ui-read-state-message]");
    if (message && messageTarget) messageTarget.textContent = message;

    target.dispatchEvent(new CustomEvent("hepta:ui-v4-read-state", {
      bubbles: true,
      detail: Object.freeze({ state, source, freshness, evidenceDigest: digest }),
    }));
    return Object.freeze({ state, source, freshness, evidenceDigest: digest });
  }

  function currentReadState(target) {
    if (!(target instanceof HTMLElement)) return null;
    const state = target.dataset.controlUiReadState;
    return READ_STATE_SET.has(state) ? state : null;
  }

  const observer = new MutationObserver((records) => {
    for (const record of records) {
      for (const node of record.addedNodes) {
        if (!(node instanceof Element)) continue;
        if (node.matches("[popover], dialog")) attachLayer(node);
        attachLayers(node);
      }
    }
  });

  window.addEventListener("popstate", () => {
    if (!historyEntryActive) return;
    historyEntryActive = false;
    hideActiveLayer({ fromPopState: true });
  });
  document.addEventListener("keydown", handleKeydown, true);
  mobileMedia.addEventListener("change", handleMediaChange);
  attachLayers();
  observer.observe(document.documentElement, { childList: true, subtree: true });

  Object.defineProperty(globalThis, "HeptaUiV4ReadState", {
    configurable: false,
    enumerable: false,
    writable: false,
    value: Object.freeze({
      states: READ_STATES,
      apply: applyReadState,
      current: currentReadState,
    }),
  });

  document.documentElement.dataset.controlUiV4Runtime = "ready";
  document.documentElement.dataset.controlUiV4RuntimeAuthority = "local-ui-only";
})();
