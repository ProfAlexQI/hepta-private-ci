module.exports = `
  const inspectSelector = (selector) => {
    const element = document.querySelector(selector);
    if (!element) {
      return { selector, exists: false, visible: false, rect: null };
    }
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    const visible = style.display !== "none"
      && style.visibility !== "hidden"
      && Number(style.opacity || 1) > 0
      && rect.width > 1
      && rect.height > 1;
    return {
      selector,
      exists: true,
      visible,
      rect: {
        left: Math.round(rect.left),
        top: Math.round(rect.top),
        right: Math.round(rect.right),
        bottom: Math.round(rect.bottom),
        width: Math.round(rect.width),
        height: Math.round(rect.height),
      },
      display: style.display,
      visibility: style.visibility,
      overflowX: style.overflowX,
      overflowY: style.overflowY,
    };
  };
  const selectors = [
    ".shell",
    ".focus-workspace",
    ".telegram-chat-shell .focus-main",
    ".tg-conversation-rail",
    ".tg-thread-panel",
    ".tg-room-panel",
    ".tg-thread-header",
    ".tg-thread",
    ".tg-compose-wrap",
    ".tg-compose-bar",
    ".tg-compose-footer",
    "[data-chat-composer-input]",
    "[data-agent-chat-send]",
    "[data-control-ui-composer-tools-trigger=light-glass]",
  ].map(inspectSelector);
  const bySelector = Object.fromEntries(selectors.map((item) => [item.selector, item]));
  const errors = [];
  const marker = document.querySelector('[data-control-ui-telegram-shell="true"]') !== null;
  const defaultVisible = (element) => {
    if (!element) {
      return false;
    }
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    return style.display !== "none"
      && style.visibility !== "hidden"
      && Number(style.opacity || 1) > 0
      && rect.width > 1
      && rect.height > 1;
  };
  const defaultSubmenuDetails = Array.from(document.querySelectorAll(
    ".tg-composer-popover,.tg-row-action-popover,.tg-thread-command-menu__panel,.command-palette-backdrop,.command-palette",
  )).map((node) => {
    const rect = node.getBoundingClientRect();
    const style = getComputedStyle(node);
    return {
      selector: node.className ? "." + String(node.className).split(/\s+/).filter(Boolean).join(".") : node.tagName.toLowerCase(),
      id: node.id || "",
      visible: defaultVisible(node),
      display: style.display,
      visibility: style.visibility,
      width: Math.round(rect.width),
      height: Math.round(rect.height),
      top: Math.round(rect.top),
      bottom: Math.round(rect.bottom),
    };
  });
  const defaultSubmenusClosedReady = defaultSubmenuDetails.every((item) => item.visible === false);
  if (!defaultSubmenusClosedReady) {
    errors.push("default_submenus_not_closed");
  }
  document.body.setAttribute("data-control-ui-submenu-audit-open", "true");
  document.querySelectorAll(".tg-thread-command-menu").forEach((node) => {
    node.open = true;
  });
  document.querySelectorAll(".tg-chat-item").forEach((row) => {
    row.classList.add("tg-chat-item--menu-open");
    const toggle = row.querySelector("[data-chat-row-menu-toggle]");
    if (toggle) {
      toggle.style.opacity = "1";
      toggle.style.pointerEvents = "auto";
      toggle.style.transform = "translateX(0)";
      toggle.style.transition = "none";
    }
  });
	  if (document.querySelector("#command-palette")) {
	    window.location.hash = "command-palette";
	  }
	  const commandPaletteAuditHoverItem = document.querySelector("[data-control-ui-command-palette-result='light-glass']");
	  if (commandPaletteAuditHoverItem) {
	    commandPaletteAuditHoverItem.classList.add("command-palette__item--audit-hover");
	  }
	  const text = document.body?.innerText || "";
  const htmlOverflow = document.documentElement.scrollWidth - window.innerWidth;
  const bodyOverflow = document.body.scrollWidth - window.innerWidth;
  const visibleKeyRects = selectors.filter((item) => item.visible && !expectedHidden.includes(item.selector));
  if (document.title !== "Hepta Control UI") {
    errors.push("unexpected_title");
  }
  if (!marker) {
    errors.push("missing_control_ui_telegram_shell_marker");
  }
  if (/ERR_CONNECTION_REFUSED|ERR_NAME_NOT_RESOLVED|无法访问此网站|This site can't be reached/i.test(text)) {
    errors.push("browser_error_page_visible");
  }
  if (htmlOverflow > 1 || bodyOverflow > 1) {
    errors.push("document_horizontal_overflow");
  }
  for (const selector of expectedVisible) {
    if (!bySelector[selector]?.visible) {
      errors.push("expected_visible_missing:" + selector);
    }
  }
  for (const selector of expectedHidden) {
    if (bySelector[selector]?.visible) {
      errors.push("expected_hidden_visible:" + selector);
    }
  }
  for (const item of visibleKeyRects) {
    if (item.rect.left < -1 || item.rect.right > window.innerWidth + 1) {
      errors.push("key_element_horizontal_clip:" + item.selector);
    }
  }
  const composer = bySelector["[data-chat-composer-input]"];
  const send = bySelector["[data-agent-chat-send]"];
  const composerBar = bySelector[".tg-compose-bar"];
  const composerWrap = bySelector[".tg-compose-wrap"];
  const defaultComposerToolsTrigger = bySelector["[data-control-ui-composer-tools-trigger=light-glass]"];
  const rectsOverlap = (left, right) => Boolean(left?.visible && right?.visible
    && Math.min(left.rect.right, right.rect.right) - Math.max(left.rect.left, right.rect.left) > 0
    && Math.min(left.rect.bottom, right.rect.bottom) - Math.max(left.rect.top, right.rect.top) > 0);
  const narrowComposerNonOverlapDetails = {
    required: viewportName === "narrow",
    trigger_rect: defaultComposerToolsTrigger?.rect || null,
    composer_bar_rect: composerBar?.rect || null,
    composer_input_rect: composer?.rect || null,
    send_button_rect: send?.rect || null,
    composer_wrap_rect: composerWrap?.rect || null,
    trigger_overlaps_composer_bar: rectsOverlap(defaultComposerToolsTrigger, composerBar),
    trigger_overlaps_composer_input: rectsOverlap(defaultComposerToolsTrigger, composer),
    trigger_overlaps_send_button: rectsOverlap(defaultComposerToolsTrigger, send),
    trigger_below_composer_bar: Boolean(defaultComposerToolsTrigger?.visible && composerBar?.visible
      && defaultComposerToolsTrigger.rect.top >= composerBar.rect.bottom),
    trigger_contained_by_composer_wrap: Boolean(defaultComposerToolsTrigger?.visible && composerWrap?.visible
      && defaultComposerToolsTrigger.rect.left >= composerWrap.rect.left - 1
      && defaultComposerToolsTrigger.rect.right <= composerWrap.rect.right + 1
      && defaultComposerToolsTrigger.rect.top >= composerWrap.rect.top - 1
      && defaultComposerToolsTrigger.rect.bottom <= composerWrap.rect.bottom + 1),
  };
  const narrowComposerNonOverlapReady = viewportName !== "narrow" || Boolean(
    defaultComposerToolsTrigger?.visible
      && composerBar?.visible
      && composer?.visible
      && send?.visible
      && narrowComposerNonOverlapDetails.trigger_overlaps_composer_bar === false
      && narrowComposerNonOverlapDetails.trigger_overlaps_composer_input === false
      && narrowComposerNonOverlapDetails.trigger_overlaps_send_button === false
      && narrowComposerNonOverlapDetails.trigger_below_composer_bar === true
      && narrowComposerNonOverlapDetails.trigger_contained_by_composer_wrap === true
  );
  if (!narrowComposerNonOverlapReady) {
    errors.push("narrow_composer_controls_overlap_or_escape_footer");
  }
  if (composer?.visible && composer.rect.width < (window.innerWidth <= 360 ? 112 : window.innerWidth <= 520 ? 120 : 180)) {
    errors.push("composer_input_too_narrow");
  }
  if (send?.visible && (send.rect.width < 44 || send.rect.height < 44)) {
    errors.push("send_button_preferred_touch_target_too_small");
  }
  const touchTargets = [send, bySelector[".tg-compose-bar"]].filter(Boolean);
  const preferredTouchTargetReady = send?.visible && send.rect.width >= 44 && send.rect.height >= 44;
  const styleProbe = (selector) => {
    const element = document.querySelector(selector);
    if (!element) {
      return { selector, exists: false, visible: false };
    }
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    return {
      selector,
      exists: true,
      visible: style.display !== "none" && style.visibility !== "hidden" && rect.width > 1 && rect.height > 1,
      width: Math.round(rect.width),
      height: Math.round(rect.height),
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      border_color: style.borderTopColor,
      background_image: style.backgroundImage,
      background_color: style.backgroundColor,
      backdrop_filter: backdrop,
      box_shadow: style.boxShadow,
    };
  };
  const composerGlass = styleProbe(".tg-compose-bar");
  const sendGlass = styleProbe("[data-agent-chat-send]");
  const composerGlassReady = composerGlass.visible
    && composerGlass.border_radius >= 16
    && composerGlass.background_image.includes("linear-gradient")
    && /blur\\(/.test(composerGlass.backdrop_filter)
    && composerGlass.box_shadow !== "none";
  const sendGlassReady = sendGlass.visible
    && sendGlass.width >= 44
    && sendGlass.height >= 44
    && sendGlass.border_radius >= 10
    && (sendGlass.background_image !== "none" || sendGlass.background_color !== "rgba(0, 0, 0, 0)")
    && sendGlass.box_shadow !== "none";
  const controlGlassActionReady = composerGlassReady && sendGlassReady;
  if (!controlGlassActionReady) {
    errors.push("control_glass_action_contract_not_ready");
  }
  const elementVisible = (element) => {
    if (!element) {
      return false;
    }
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    return style.display !== "none"
      && style.visibility !== "hidden"
      && Number(style.opacity || 1) > 0
      && rect.width > 1
      && rect.height > 1;
  };
  const richRect = (element) => {
    const rect = element.getBoundingClientRect();
    return {
      left: Math.round(rect.left),
      top: Math.round(rect.top),
      right: Math.round(rect.right),
      bottom: Math.round(rect.bottom),
      width: Math.round(rect.width),
      height: Math.round(rect.height),
    };
  };
  const hasSvgIcon = (element) => Boolean(element?.querySelector("svg use[href^='#hepta-icon-']"));
	  const visibleText = (element) => {
	    const collect = (node) => {
      if (node.nodeType === Node.TEXT_NODE) {
        return node.textContent || "";
      }
      if (node.nodeType !== Node.ELEMENT_NODE) {
        return "";
      }
      const el = node;
      if (el.matches("svg, svg *, .sr-only")) {
        return "";
      }
      return Array.from(el.childNodes).map(collect).join("");
    };
	    return collect(element).replace(/\\s+/g, " ").trim();
	  };
	  const visibleTextIntegrityExpected = "safe status source is";
	  const visibleTextIntegrityProbe = document.createElement("span");
	  visibleTextIntegrityProbe.textContent = visibleTextIntegrityExpected;
	  const visibleTextIntegritySample = visibleText(visibleTextIntegrityProbe);
	  const visibleTextIntegrityReady = visibleTextIntegritySample === visibleTextIntegrityExpected;
	  const parseCssColor = (value) => {
	    const match = String(value || "").match(/rgba?\(([^)]+)\)/);
	    if (!match) {
	      return null;
	    }
	    const parts = (match[1].match(/[0-9.]+/g) || []).map((part) => Number.parseFloat(part));
	    if (parts.length < 3 || parts.slice(0, 3).some((part) => Number.isNaN(part))) {
	      return null;
	    }
	    return { r: parts[0], g: parts[1], b: parts[2], a: parts.length >= 4 && !Number.isNaN(parts[3]) ? parts[3] : 1 };
	  };
	  const blendColor = (fg, bg) => {
	    const alpha = Math.max(0, Math.min(1, fg?.a ?? 1));
	    return {
	      r: (fg.r * alpha) + (bg.r * (1 - alpha)),
	      g: (fg.g * alpha) + (bg.g * (1 - alpha)),
	      b: (fg.b * alpha) + (bg.b * (1 - alpha)),
	      a: 1,
	    };
	  };
	  const effectiveBackground = (node) => {
	    let color = { r: 5, g: 8, b: 11, a: 1 };
	    const stack = [];
	    for (let current = node; current && current.nodeType === Node.ELEMENT_NODE; current = current.parentElement) {
	      const parsed = parseCssColor(getComputedStyle(current).backgroundColor);
	      if (parsed && parsed.a > 0) {
	        stack.push(parsed);
	      }
	    }
	    stack.reverse().forEach((item) => {
	      color = blendColor(item, color);
	    });
	    return color;
	  };
	  const relativeLuminance = (color) => {
	    const channel = (value) => {
	      const normalized = Math.max(0, Math.min(255, value)) / 255;
	      return normalized <= 0.03928 ? normalized / 12.92 : ((normalized + 0.055) / 1.055) ** 2.4;
	    };
	    return (0.2126 * channel(color.r)) + (0.7152 * channel(color.g)) + (0.0722 * channel(color.b));
	  };
	  const contrastRatio = (a, b) => {
	    const la = relativeLuminance(a);
	    const lb = relativeLuminance(b);
	    return (Math.max(la, lb) + 0.05) / (Math.min(la, lb) + 0.05);
	  };
	  const styleNumber = (style, property) => Number.parseFloat(style[property] || "0") || 0;
	  const compactShadow = (value) => value && value !== "none" ? "present" : "none";
	  const directBackgroundAlpha = (style) => {
	    const parsed = parseCssColor(style.backgroundColor);
	    return parsed ? parsed.a : 0;
	  };
	  const colorChannelSpread = (color) => color
	    ? Math.max(color.r, color.g, color.b) - Math.min(color.r, color.g, color.b)
	    : 255;
	  const translucentGlassReady = (style) => {
	    const alpha = directBackgroundAlpha(style);
	    return alpha >= 0.35 && alpha <= 0.88;
	  };
  const railVisible = elementVisible(document.querySelector(".tg-conversation-rail"));
  const submenuAuditSelector = ".tg-composer-popover,.tg-row-action-popover,.tg-thread-command-menu__panel,.command-palette-backdrop,.command-palette";
  const resetComposerPopoverAuditGeometry = (node) => {
    [
      "position",
      "left",
      "right",
      "top",
      "bottom",
      "width",
      "min-width",
      "max-width",
      "box-sizing",
      "margin",
      "transform",
    ].forEach((property) => node.style.removeProperty(property));
  };
  const applyComposerPopoverAuditGeometry = ({ showAll = false } = {}) => {
    const narrow = window.innerWidth <= 980;
    const compact = window.innerWidth <= 700;
    const inset = compact ? 14 : 24;
    const width = Math.max(0, window.innerWidth - (inset * 2));
    document.querySelectorAll(".tg-composer-popover").forEach((node) => {
      if (showAll) {
        node.style.setProperty("display", "grid", "important");
      }
      if (!narrow) {
        return;
      }
      const key = node.getAttribute("data-chat-composer-popover") || "";
      node.style.setProperty("position", "fixed", "important");
      node.style.setProperty("left", inset + "px", "important");
      node.style.setProperty("right", "auto", "important");
      node.style.setProperty("width", width + "px", "important");
      node.style.setProperty("min-width", "0", "important");
      node.style.setProperty("max-width", width + "px", "important");
      node.style.setProperty("box-sizing", "border-box", "important");
      node.style.setProperty("margin", "0", "important");
      node.style.setProperty("transform", "none", "important");
      if (key === "artifact") {
        node.style.setProperty("top", "auto", "important");
        node.style.setProperty("bottom", "300px", "important");
      } else if (key === "command") {
        node.style.setProperty("top", "auto", "important");
        node.style.setProperty("bottom", "84px", "important");
      }
      const rect = node.getBoundingClientRect();
      const delta = rect.left - inset;
      if (Math.abs(delta) > 1) {
        node.style.setProperty("left", (inset - delta) + "px", "important");
      }
    });
  };
  const closeAllSubmenusForSingleAudit = () => {
    document.body.removeAttribute("data-control-ui-submenu-audit-open");
    document.querySelectorAll("[popover]:popover-open").forEach((node) => {
      if (typeof node.hidePopover === "function") node.hidePopover();
    });
    document.querySelectorAll("details[open]").forEach((node) => node.removeAttribute("open"));
    document.querySelectorAll(".tg-chat-item").forEach((row) => {
      row.classList.remove("tg-chat-item--menu-open");
    });
    document.querySelectorAll(".tg-composer-popover").forEach((node) => {
      node.style.display = "";
      resetComposerPopoverAuditGeometry(node);
    });
    if (window.location.hash === "#command-palette") window.location.hash = "chat";
  };
  const restoreFullSubmenuAuditOpen = () => {
    closeAllSubmenusForSingleAudit();
  };
  const inspectSingleSubmenuTarget = (spec) => {
    closeAllSubmenusForSingleAudit();
    const trigger = spec.open();
    const triggerRect = trigger && elementVisible(trigger) ? richRect(trigger) : null;
    const visibleSubmenus = Array.from(document.querySelectorAll(submenuAuditSelector)).filter(elementVisible);
    const targetNodes = spec.targetSelectors
      .flatMap((selector) => Array.from(document.querySelectorAll(selector)))
      .filter(elementVisible);
    const targetNodeSet = new Set(targetNodes);
    const unexpectedVisible = visibleSubmenus.filter((node) => !targetNodeSet.has(node));
    const surfaceNodes = (spec.surfaceSelectors || spec.targetSelectors)
      .flatMap((selector) => Array.from(document.querySelectorAll(selector)))
      .filter(elementVisible);
    const surfaceDetails = surfaceNodes.map((node) => {
      const style = getComputedStyle(node);
      const bgColor = effectiveBackground(node);
      const bgLuminance = relativeLuminance(bgColor);
      const rect = richRect(node);
      return {
        selector: node.id ? ("#" + node.id) : (node.className ? "." + String(node.className).split(/\s+/).filter(Boolean).join(".") : node.tagName.toLowerCase()),
        window_width: window.innerWidth,
        inline_style: node.getAttribute("style") || "",
        role: node.getAttribute("role") || "",
        aria_label: node.getAttribute("aria-label") || "",
        item_count: spec.itemSelector ? node.querySelectorAll(spec.itemSelector).length : 0,
        effective_luminance: Number(bgLuminance.toFixed(3)),
        light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
        backdrop_filter: style.backdropFilter || style.webkitBackdropFilter || "",
        box_shadow: compactShadow(style.boxShadow),
        border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
        in_viewport: rect.left >= -1 && rect.top >= -1 && rect.right <= window.innerWidth + 1 && rect.bottom <= window.innerHeight + 1,
        top_clipped: rect.top < -1,
        bottom_clipped: rect.bottom > window.innerHeight + 1,
        trigger_block_gap: triggerRect ? Math.round(rect.top - triggerRect.bottom) : null,
        trigger_inline_end_delta: triggerRect ? Math.round(rect.right - triggerRect.right) : null,
        trigger_geometry_ready: spec.group !== "row-menu" || Boolean(triggerRect
          && rect.top >= triggerRect.bottom - 1
          && rect.top - triggerRect.bottom <= 18
          && Math.abs(rect.right - triggerRect.right) <= 3),
        ...rect,
      };
    });
    const itemNodes = spec.itemSelector ? surfaceNodes.flatMap((node) => Array.from(node.querySelectorAll(spec.itemSelector))) : [];
    const interactiveNodesForItem = (node) => [
      ...(node.matches("button,input,select,textarea,a[href]") ? [node] : []),
      ...Array.from(node.querySelectorAll("button,input,select,textarea,a[href]")),
    ];
    const itemDetails = itemNodes.filter(elementVisible).map((node) => {
      const style = getComputedStyle(node);
      const textColor = parseCssColor(style.color);
      const bgColor = effectiveBackground(node);
      const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
      const ariaLabel = node.getAttribute("aria-label") || "";
      const title = node.getAttribute("title") || "";
      const interactiveNodes = interactiveNodesForItem(node);
      const interactiveDetails = interactiveNodes.map((control) => {
        const controlAriaLabel = control.getAttribute("aria-label") || "";
        const controlTitle = control.getAttribute("title") || "";
        return {
          native_disabled: "disabled" in control && control.disabled === true,
          aria_disabled: control.getAttribute("aria-disabled") === "true",
          unavailable_marker: control.getAttribute("data-control-ui-unavailable") || "",
          aria_label: controlAriaLabel,
          title: controlTitle,
          truthful_copy: controlAriaLabel.length > 0
            && controlTitle === controlAriaLabel
            && (controlTitle.toLowerCase().includes("unavailable")
              || controlTitle.toLowerCase().includes("requires")
              || controlTitle.toLowerCase().includes("live adapter")
              || controlTitle.toLowerCase().includes("static preview")),
        };
      });
      const disabled = interactiveDetails.length > 0 && interactiveDetails.every((item) => (
        item.native_disabled
        && item.aria_disabled
        && item.unavailable_marker === "live-adapter"
      ));
      const beforeHash = window.location.hash;
      const beforeValues = interactiveNodes.map((control) => "value" in control ? String(control.value) : "");
      const beforePopoverOpen = surfaceNodes.every((surface) => !surface.hasAttribute("popover") || surface.matches(":popover-open"));
      if (disabled) node.click();
      const clickBlocked = !disabled || (
        window.location.hash === beforeHash
        && interactiveNodes.every((control, index) => !("value" in control) || String(control.value) === beforeValues[index])
        && surfaceNodes.every((surface) => !surface.hasAttribute("popover") || surface.matches(":popover-open")) === beforePopoverOpen
      );
      return {
        label: visibleText(node),
        role: node.getAttribute("role") || "",
        aria_label: ariaLabel,
        title,
        title_matches_aria_label: title === ariaLabel,
        svg_icon_present: hasSvgIcon(node),
        contrast_ratio: Number(ratio.toFixed(2)),
        readable: ratio >= 4.5,
        height: Math.round(node.getBoundingClientRect().height),
        label_nowrap_ready: style.whiteSpace === "nowrap" || Boolean(node.querySelector(".tg-menu-item__label,.tg-row-action__label,.tg-composer-popover__item b")),
        disabled,
        interactive_control_count: interactiveDetails.length,
        disabled_control_count: interactiveDetails.filter((item) => item.native_disabled).length,
        disabled_contract_ready: !disabled || interactiveDetails.every((item) => item.truthful_copy),
        disabled_click_blocked: clickBlocked,
        interactive_details: interactiveDetails,
      };
    });
    const supportingControlNodes = spec.supportingControlSelector
      ? Array.from(document.querySelectorAll(spec.supportingControlSelector)).filter(elementVisible)
      : [];
    const supportingControlDetails = supportingControlNodes.map((node) => {
      const rect = richRect(node);
      const ariaLabel = node.getAttribute("aria-label") || "";
      const title = node.getAttribute("title") || "";
      return {
        tag: node.tagName.toLowerCase(),
        type: node.getAttribute("type") || "",
        aria_label: ariaLabel,
        title,
        title_matches_aria_label: title === ariaLabel,
        popover_target: node.getAttribute("popovertarget") || "",
        popover_target_action: node.getAttribute("popovertargetaction") || "",
        ...rect,
      };
    });
    const horizontalOverflowFree = document.documentElement.scrollWidth - window.innerWidth <= 1
      && document.body.scrollWidth - window.innerWidth <= 1;
    const expectedVisibleCount = spec.expectedVisibleCount ?? spec.targetSelectors.length;
    const expectedItemCount = spec.expectedItemCount ?? 0;
    const expectedDisabledItemCount = spec.expectedDisabledItemCount ?? 0;
    const requiresItemSvg = spec.requiresItemSvg !== false;
    const requiresItemNowrap = spec.requiresItemNowrap !== false;
    const expectedSupportingControlCount = spec.expectedSupportingControlCount ?? 0;
    const surfacesReady = surfaceDetails.length > 0 && surfaceDetails.every((item) => (
      item.in_viewport
      && !item.top_clipped
      && !item.bottom_clipped
      && item.light_glass_ready
      && item.effective_luminance >= 0.72
      && item.effective_luminance <= 0.98
      && (item.backdrop_filter || "").includes("blur(")
      && item.box_shadow !== "none"
      && item.border_radius >= 14
      && item.trigger_geometry_ready === true
    ));
    const itemsReady = expectedItemCount === 0 || (
      itemDetails.length === expectedItemCount
      && itemDetails.every((item) => (
        item.height >= 44
        && (!requiresItemSvg || item.svg_icon_present)
        && item.readable
        && item.contrast_ratio >= 4.5
        && item.title_matches_aria_label
        && (!requiresItemNowrap || item.label_nowrap_ready)
        && item.label.length > 0
      ))
    );
    const disabledItemDetails = itemDetails.filter((item) => item.disabled);
    const enabledItemDetails = itemDetails.filter((item) => !item.disabled);
    const unavailableItemsReady = disabledItemDetails.length === expectedDisabledItemCount
      && disabledItemDetails.every((item) => item.disabled_contract_ready && item.disabled_click_blocked)
      && enabledItemDetails.every((item) => item.disabled_control_count === 0);
    const enabledFocusableNodes = itemNodes.flatMap(interactiveNodesForItem).filter((node) => (
      !("disabled" in node && node.disabled === true)
      && node.getAttribute("aria-disabled") !== "true"
    ));
    if (enabledFocusableNodes.length > 0
      && !targetNodes.some((node) => node === document.activeElement || node.contains(document.activeElement))) {
      enabledFocusableNodes[0].focus();
    }
    const focusContained = targetNodes.some((node) => node === document.activeElement || node.contains(document.activeElement));
    const focusPolicyReady = enabledFocusableNodes.length > 0
      ? focusContained
      : unavailableItemsReady && disabledItemDetails.length === itemDetails.length;
    const supportingControlsReady = expectedSupportingControlCount === 0 || (
      supportingControlDetails.length === expectedSupportingControlCount
      && supportingControlDetails.every((item) => (
        item.width >= 44
        && item.height >= 44
        && item.aria_label.length > 0
        && item.title.length > 0
        && item.title_matches_aria_label
      ))
    );
    const ready = targetNodes.length === expectedVisibleCount
      && unexpectedVisible.length === 0
      && horizontalOverflowFree
      && surfacesReady
      && itemsReady
      && unavailableItemsReady
      && supportingControlsReady
      && targetNodes.every((node) => !node.hasAttribute("popover") || node.matches(":popover-open"))
      && Boolean(trigger?.matches("button[popovertarget]"))
      && focusPolicyReady;
    return {
      key: spec.key,
      group: spec.group,
      expected_visible_count: expectedVisibleCount,
      visible_target_count: targetNodes.length,
      unexpected_visible_count: unexpectedVisible.length,
      horizontal_overflow_free: horizontalOverflowFree,
      expected_item_count: expectedItemCount,
      expected_disabled_item_count: expectedDisabledItemCount,
      disabled_item_count: disabledItemDetails.length,
      enabled_item_count: enabledItemDetails.length,
      unavailable_items_ready: unavailableItemsReady,
      expected_supporting_control_count: expectedSupportingControlCount,
      requires_item_svg: requiresItemSvg,
      requires_item_nowrap: requiresItemNowrap,
      visible_item_count: itemDetails.length,
      native_trigger: Boolean(trigger?.matches("button[popovertarget]")),
      trigger_target: trigger?.getAttribute("popovertarget") || "",
      trigger_rect: triggerRect,
      popover_open: targetNodes.every((node) => !node.hasAttribute("popover") || node.matches(":popover-open")),
      focus_contained: focusContained,
      focus_policy_ready: focusPolicyReady,
      surface_details: surfaceDetails,
      item_details: itemDetails,
      supporting_control_details: supportingControlDetails,
      ready,
    };
  };
  const singleSubmenuAuditSpecs = [
    ...(railVisible ? ["ui-chat-agent", "task-queue", "operator-plane"].map((key) => ({
      key: "row-menu:" + key,
      group: "row-menu",
      targetSelectors: ['[data-chat-row-menu-panel="' + key + '"]'],
      itemSelector: "[data-chat-row-menu-item]",
      expectedItemCount: 3,
      expectedDisabledItemCount: 2,
      open: () => {
        const trigger = document.querySelector('[data-chat-conversation="' + key + '"] [data-chat-row-menu-toggle]');
        trigger?.click();
        return trigger;
      },
    })) : []),
    {
      key: "thread-tools",
      group: "thread-tools",
      targetSelectors: ['[data-control-ui-thread-tools-panel="light-glass"]'],
      itemSelector: "[data-control-ui-menu-item]",
      expectedItemCount: 3,
      open: () => {
        const trigger = document.querySelector('[data-control-ui-thread-tools-trigger="light-glass"]');
        trigger?.click();
        return trigger;
      },
    },
    {
      key: "composer-tools",
      group: "composer-tools",
      targetSelectors: ['[data-control-ui-composer-tools-panel="light-glass"]'],
      itemSelector: "[data-control-ui-menu-item]",
      expectedItemCount: 2,
      expectedDisabledItemCount: 2,
      open: () => {
        const trigger = document.querySelector('[data-control-ui-composer-tools-trigger="light-glass"]');
        trigger?.click();
        return trigger;
      },
    },
    ...["artifact", "command"].map((key) => ({
      key: "composer-popover:" + key,
      group: "composer-popover",
      targetSelectors: ['[data-chat-composer-popover="' + key + '"]'],
      itemSelector: ".tg-composer-popover__item",
      expectedItemCount: 2,
      supportingControlSelector: '[data-chat-composer-popover="' + key + '"] [data-chat-composer-picker-search]',
      expectedSupportingControlCount: 1,
      open: () => {
        const trigger = document.querySelector('[data-chat-composer-popover-toggle="' + key + '"]');
        trigger?.click();
        return trigger;
      },
    })),
    {
      key: "command-palette",
      group: "command-palette",
      targetSelectors: ["#command-palette", ".command-palette"],
      surfaceSelectors: [".command-palette"],
      itemSelector: "[data-control-ui-command-palette-result='light-glass']",
      expectedVisibleCount: 2,
      expectedItemCount: 18,
      supportingControlSelector: "#command-palette [data-control-ui-command-palette-input],#command-palette [data-control-ui-command-palette-close]",
      expectedSupportingControlCount: 2,
      requiresItemSvg: false,
      requiresItemNowrap: false,
      open: () => {
        const trigger = document.querySelector('[data-control-ui-command-palette-trigger="light-glass"]');
        trigger?.click();
        return trigger;
      },
    },
  ];
  const singleSubmenuAuditDetails = singleSubmenuAuditSpecs.map(inspectSingleSubmenuTarget);
  const rowMenuAuditDetails = singleSubmenuAuditDetails.filter((item) => item.group === "row-menu");
  const rowMenuDistinctPositionsReady = !railVisible || (rowMenuAuditDetails.length === 3
    && new Set(rowMenuAuditDetails.map((item) => item.surface_details?.[0]?.top)).size === 3);
  const singleSubmenuAuditReady = singleSubmenuAuditDetails.every((item) => item.ready === true)
    && rowMenuDistinctPositionsReady;
  const unavailableSubmenuItemsReady = singleSubmenuAuditDetails.every((item) => item.unavailable_items_ready === true);
  const disabledSubmenuItemCount = singleSubmenuAuditDetails.reduce(
    (count, item) => count + (item.disabled_item_count || 0),
    0,
  );
  restoreFullSubmenuAuditOpen();
  const mobilePaneRouteDetails = [];
  let mobilePaneRowMenuReady = true;
  if (window.innerWidth <= 700) {
    for (const pane of ["chats", "thread", "room"]) {
      const link = document.querySelector('[data-chat-mobile-pane-tab="' + pane + '"]');
      link?.focus();
      const linkKeyboardFocusable = document.activeElement === link;
      link?.click();
      const visiblePanes = Array.from(document.querySelectorAll("[data-chat-mobile-pane]")).filter(elementVisible);
      const target = document.querySelector('[data-chat-mobile-pane="' + pane + '"]');
      const rect = target && elementVisible(target) ? richRect(target) : null;
      const roomContent = pane === "room" ? target?.querySelector(".hepta-right-sidebar") : null;
      const roomContentRect = roomContent && elementVisible(roomContent) ? richRect(roomContent) : null;
      const routeFocusReady = document.activeElement === link || document.activeElement === target;
      const paneContentReady = Boolean(rect && rect.height >= 240) && (pane !== "room" || Boolean(
        roomContent
        && roomContentRect
        && roomContentRect.height >= 240
        && visibleText(roomContent).length >= 120
        && roomContent.querySelectorAll("a").length >= 3
      ));
      mobilePaneRouteDetails.push({
        pane,
        href: link?.getAttribute("href") || "",
        hash: window.location.hash,
        link_focused: document.activeElement === link,
        link_keyboard_focusable: linkKeyboardFocusable,
        target_focused: document.activeElement === target,
        route_focus_ready: routeFocusReady,
        visible_pane_count: visiblePanes.length,
        visible_panes: visiblePanes.map((node) => node.getAttribute("data-chat-mobile-pane") || ""),
        target_visible: Boolean(target && elementVisible(target)),
        target_in_viewport: Boolean(rect && rect.left >= -1 && rect.right <= window.innerWidth + 1 && rect.top >= -1 && rect.bottom <= window.innerHeight + 1),
        target_rect: rect,
        pane_content_ready: paneContentReady,
        room_content_visible: pane !== "room" || Boolean(roomContentRect),
        room_content_rect: roomContentRect,
        room_content_text_length: roomContent ? visibleText(roomContent).length : 0,
        room_content_link_count: roomContent ? roomContent.querySelectorAll("a").length : 0,
      });
      if (pane === "chats") {
        const trigger = target?.querySelector('[data-chat-row-menu-toggle="ui-chat-agent"]');
        trigger?.click();
        const panel = document.querySelector("#row-menu-ui-chat-agent");
        const triggerRect = trigger && elementVisible(trigger) ? richRect(trigger) : null;
        const panelRect = panel && elementVisible(panel) ? richRect(panel) : null;
        mobilePaneRowMenuReady = Boolean(panel?.matches(":popover-open")
          && triggerRect
          && panelRect
          && panelRect.left >= 8
          && panelRect.right <= window.innerWidth - 8
          && panelRect.top >= triggerRect.bottom - 1
          && panelRect.top - triggerRect.bottom <= 18
          && Math.abs(panelRect.right - triggerRect.right) <= 3);
        if (panel?.matches(":popover-open") && typeof panel.hidePopover === "function") panel.hidePopover();
      }
    }
    const threadLink = document.querySelector('[data-chat-mobile-pane-tab="thread"]');
    threadLink?.focus();
    threadLink?.click();
  }
  const mobilePaneNavigationReady = window.innerWidth > 700 || (
    mobilePaneRouteDetails.length === 3
    && mobilePaneRouteDetails.every((item) => item.route_focus_ready
      && item.visible_pane_count === 1
      && item.visible_panes.length === 1
      && item.visible_panes[0] === item.pane
      && item.target_visible
      && item.target_in_viewport
      && item.pane_content_ready)
    && mobilePaneRowMenuReady
  );
  const actualClickAuditForGroup = (group) => singleSubmenuAuditDetails.filter((item) => item.group === group);
  const actualClickRowMenuCompatibilityReady = railVisible
    ? actualClickAuditForGroup("row-menu").length === 3 && actualClickAuditForGroup("row-menu").every((item) => item.ready)
    : mobilePaneRowMenuReady;
  const actualClickThreadToolsCompatibilityReady = actualClickAuditForGroup("thread-tools").length === 1
    && actualClickAuditForGroup("thread-tools").every((item) => item.ready);
  const actualClickComposerToolsCompatibilityReady = actualClickAuditForGroup("composer-tools").length === 1
    && actualClickAuditForGroup("composer-tools").every((item) => item.ready);
  const actualClickComposerPopoverCompatibilityReady = actualClickAuditForGroup("composer-popover").length === 2
    && actualClickAuditForGroup("composer-popover").every((item) => item.ready);
  const actualClickCommandPaletteCompatibilityReady = actualClickAuditForGroup("command-palette").length === 1
    && actualClickAuditForGroup("command-palette").every((item) => item.ready);
  const actualClickMenuCompatibilityReady = singleSubmenuAuditReady
    && actualClickRowMenuCompatibilityReady
    && actualClickThreadToolsCompatibilityReady
    && actualClickComposerToolsCompatibilityReady
    && actualClickComposerPopoverCompatibilityReady
    && actualClickCommandPaletteCompatibilityReady;

`;
