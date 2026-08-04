module.exports = String.raw`
  const visible = (element) => Boolean(element && actualVisibility(element).visible);
  const elements = Array.from(document.querySelectorAll("body *"));
  const visibleElements = elements.filter(visible);
  const styleOf = (element) => getComputedStyle(element);
  const splitLayers = (value) => {
    if (!value || value === "none") return [];
    const layers = [];
    let depth = 0;
    let start = 0;
    for (let index = 0; index < value.length; index += 1) {
      const character = value[index];
      if (character === "(") depth += 1;
      else if (character === ")") depth = Math.max(0, depth - 1);
      else if (character === "," && depth === 0) {
        layers.push(value.slice(start, index).trim());
        start = index + 1;
      }
    }
    layers.push(value.slice(start).trim());
    return layers.filter(Boolean);
  };
  const countGradients = (value) => (value?.match(/(?:linear|radial|conic)-gradient\(/g) || []).length;
  const hasBorder = (style) => [style.borderTopWidth, style.borderRightWidth, style.borderBottomWidth, style.borderLeftWidth]
    .some((width) => Number.parseFloat(width) > 0);
  const hasBackdrop = (style) => {
    const value = style.backdropFilter || style.webkitBackdropFilter || "none";
    return value !== "none" && /blur\(/.test(value);
  };
  const describe = (element) => {
    const style = styleOf(element);
    const rect = element.getBoundingClientRect();
    return {
      selector: element.id ? "#" + element.id : element.classList.length ? "." + Array.from(element.classList).slice(0, 3).join(".") : element.tagName.toLowerCase(),
      width: Math.round(rect.width),
      height: Math.round(rect.height),
      shadow_layer_count: splitLayers(style.boxShadow).length,
      gradient_layer_count: countGradients(style.backgroundImage),
      border_layer_count: hasBorder(style) ? 1 : 0,
      backdrop_filter: style.backdropFilter || style.webkitBackdropFilter || "none",
    };
  };

  const glassSurfaces = visibleElements.filter((element) => {
    const rect = element.getBoundingClientRect();
    return rect.width >= 32 && rect.height >= 32 && hasBackdrop(styleOf(element));
  });
  const glassSurfaceDetails = glassSurfaces.map(describe);
  const stableContentSelectors = [
    ".tg-chat-row",
    ".tg-message",
    ".tg-message-bubble",
    ".route-card",
    ".tg-thread-intro",
    ".tg-local-json-card",
  ];
  const stableContentDetails = stableContentSelectors.flatMap((selector) =>
    Array.from(document.querySelectorAll(selector)).filter(visible).map((element) => ({ selector, ...describe(element) })),
  );
  const stableContentSurfaceReady = stableContentDetails.every((detail) => !/blur\(/.test(detail.backdrop_filter));

  const visibleLeafText = visibleElements.filter((element) => {
    if (element.matches(".sr-only, [aria-hidden='true']")) return false;
    const ownText = Array.from(element.childNodes)
      .filter((node) => node.nodeType === Node.TEXT_NODE)
      .map((node) => node.textContent || "")
      .join("")
      .trim();
    const placeholder = element instanceof HTMLInputElement || element instanceof HTMLTextAreaElement ? element.placeholder.trim() : "";
    return ownText.length > 0 || placeholder.length > 0;
  });
  const under12Details = visibleLeafText.flatMap((element) => {
    const fontSize = Number.parseFloat(styleOf(element).fontSize);
    if (!Number.isFinite(fontSize) || fontSize >= 12) return [];
    return [{
      selector: element.id ? "#" + element.id : element.classList.length ? "." + Array.from(element.classList).slice(0, 3).join(".") : element.tagName.toLowerCase(),
      font_size: fontSize,
      text: (element.textContent || element.placeholder || "").trim().slice(0, 80),
    }];
  });

  const expectedVisibilityDetails = [
    ...expectedVisible.map((selector) => ({ selector, expected: "visible", actual: visible(document.querySelector(selector)) })),
    ...expectedHidden.map((selector) => ({ selector, expected: "hidden", actual: visible(document.querySelector(selector)) })),
  ];
  const expectedVisibilityReady = expectedVisibilityDetails.every((detail) =>
    detail.expected === "visible" ? detail.actual : !detail.actual,
  );
  const touchSelectors = ["[data-chat-composer-input]", "[data-agent-chat-send]", "[data-chat-attachment-placeholder]"];
  const touchDetails = touchSelectors.flatMap((selector) => {
    const element = document.querySelector(selector);
    if (!visible(element)) return [];
    const rect = element.getBoundingClientRect();
    return [{ selector, width: rect.width, height: rect.height, ready: rect.width >= 40 && rect.height >= 40 }];
  });
  const isMobile = viewportName === "mobile" || viewportName === "phone320";
  const isNarrow = viewportName === "narrow";
  const visibleTopbars = visibleElements.filter((element) => element.matches("[data-mobile-primary-topbar], .tg-mobile-top-tabs"));
  const visibleBottomLayers = visibleElements.filter((element) => element.matches("[data-mobile-bottom-action-layer], .tg-compose-wrap"));
  const mobilePrimaryActions = touchSelectors.map((selector) => visible(document.querySelector(selector)));
  const mobileTopbar = visibleTopbars[0];
  const mobileTopbarBack = mobileTopbar?.querySelector("[data-mobile-topbar-back]");
  const mobileTopbarTitle = mobileTopbar?.querySelector("[data-mobile-topbar-title]");
  const mobileTopbarActions = mobileTopbar?.querySelector("[data-mobile-topbar-actions]");
  const mobileTopbarActionControls = mobileTopbarActions
    ? Array.from(mobileTopbarActions.querySelectorAll("a,button")).filter(visible)
    : [];
  const mobileTopbarControlReady = (element) => {
    if (!visible(element)) return false;
    const rect = element.getBoundingClientRect();
    return rect.width >= 44 && rect.height >= 44;
  };
  const mobileTopbarSemanticsReady = !isMobile || (
    visibleTopbars.length === 1
    && mobileTopbar?.getAttribute("data-chat-mobile-pane-tabs") === "contextual"
    && mobileTopbarControlReady(mobileTopbarBack)
    && visible(mobileTopbarTitle)
    && (mobileTopbarTitle.textContent || "").trim().length > 0
    && mobileTopbarActions
    && mobileTopbarActionControls.length >= 1
    && mobileTopbarActionControls.length <= 2
    && mobileTopbarActionControls.every(mobileTopbarControlReady)
    && mobileTopbar.getBoundingClientRect().height <= 56
  );

  const focusHeader = document.querySelector(".focus-header");
  const narrowRail = document.querySelector(".tg-conversation-rail");
  const narrowThreadHeader = document.querySelector(".tg-thread-header");
  const narrowComposer = document.querySelector(".tg-compose-wrap");
  const narrowComposerFooter = document.querySelector(".tg-compose-footer");
  const narrowComposerMore = document.querySelector("[data-control-ui-composer-more]");
  const narrowMessageViewport = document.querySelector(".tg-thread");
  const rectOrZero = (element) => visible(element) ? element.getBoundingClientRect() : { width: 0, height: 0, top: 0, bottom: 0 };
  const narrowRailRect = rectOrZero(narrowRail);
  const narrowHeaderRect = rectOrZero(narrowThreadHeader);
  const narrowComposerRect = rectOrZero(narrowComposer);
  const narrowMessageRect = rectOrZero(narrowMessageViewport);
  const narrowChromeHeight = narrowRailRect.height + narrowHeaderRect.height + narrowComposerRect.height;
  const narrowChromeRatio = narrowChromeHeight / Math.max(1, window.innerHeight);
  const narrowMessageRatio = narrowMessageRect.height / Math.max(1, window.innerHeight);
  const narrowSingleActionRowReady = !isNarrow || (
    !visible(narrowComposerFooter)
    && visible(narrowComposerMore)
    && narrowComposerMore.closest(".tg-compose-bar") !== null
  );
  const narrowShellDensityReady = !isNarrow || (
    !visible(focusHeader)
    && narrowRailRect.height <= 190
    && narrowHeaderRect.height <= 72
    && narrowComposerRect.height <= 80
    && narrowChromeRatio <= 0.38
    && narrowMessageRect.height >= 360
    && narrowMessageRatio >= 0.40
    && narrowComposerRect.bottom <= window.innerHeight + 1
    && narrowSingleActionRowReady
  );

  const maximumShadowLayerCount = Math.max(0, ...glassSurfaceDetails.map((detail) => detail.shadow_layer_count));
  const maximumGradientLayerCount = Math.max(0, ...glassSurfaceDetails.map((detail) => detail.gradient_layer_count));
  const maximumBorderLayerCount = Math.max(0, ...glassSurfaceDetails.map((detail) => detail.border_layer_count));
  const temperedSurfaceBudgetReady = glassSurfaceDetails.length <= 4
    && maximumShadowLayerCount <= 1
    && maximumGradientLayerCount <= 1
    && maximumBorderLayerCount <= 1;
  const horizontalOverflowFree = document.documentElement.scrollWidth <= window.innerWidth + 1
    && document.body.scrollWidth <= window.innerWidth + 1;
  const browserErrorPageAbsent = !/chrome-error:\/\//.test(location.href)
    && !document.body.innerText.includes("This site can’t be reached");

  const errors = [];
  if (!expectedVisibilityReady) errors.push("expected_visibility_failed");
  if (!browserErrorPageAbsent) errors.push("browser_error_page_present");
  if (!horizontalOverflowFree) errors.push("horizontal_overflow_present");
  if (!stableContentSurfaceReady) errors.push("stable_content_blur_present");
  if (!temperedSurfaceBudgetReady) errors.push("tempered_surface_budget_exceeded");
  if (under12Details.length > 0) errors.push("visible_text_below_12px");
  if (!touchDetails.every((detail) => detail.ready)) errors.push("key_touch_target_too_small");
  if (isMobile && visibleTopbars.length !== 1) errors.push("mobile_topbar_count_mismatch");
  if (!mobileTopbarSemanticsReady) errors.push("mobile_topbar_semantics_failed");
  if (isMobile && visibleBottomLayers.length !== 1) errors.push("mobile_bottom_action_layer_count_mismatch");
  if (isMobile && !mobilePrimaryActions.every(Boolean)) errors.push("mobile_primary_action_missing");
  if (!narrowShellDensityReady) errors.push("narrow_shell_density_failed");
  if (!narrowSingleActionRowReady) errors.push("narrow_composer_action_rows_failed");

  return {
    expected_visibility_ready: expectedVisibilityReady,
    expected_visibility_details: expectedVisibilityDetails,
    browser_error_page_absent: browserErrorPageAbsent,
    horizontal_overflow_free: horizontalOverflowFree,
    stable_content_surface_ready: stableContentSurfaceReady,
    stable_content_details: stableContentDetails,
    tempered_surface_budget_ready: temperedSurfaceBudgetReady,
    visible_glass_surface_count: glassSurfaceDetails.length,
    visible_glass_surface_details: glassSurfaceDetails,
    maximum_shadow_layer_count: maximumShadowLayerCount,
    maximum_gradient_layer_count: maximumGradientLayerCount,
    maximum_border_layer_count: maximumBorderLayerCount,
    visible_text_floor_ready: under12Details.length === 0,
    visible_under_12px_count: under12Details.length,
    visible_under_12px_details: under12Details,
    key_touch_controls_ready: touchDetails.every((detail) => detail.ready),
    key_touch_control_details: touchDetails,
    mobile_single_topbar_ready: !isMobile || visibleTopbars.length === 1,
    mobile_visible_topbar_count: visibleTopbars.length,
    mobile_topbar_semantics_ready: mobileTopbarSemanticsReady,
    mobile_topbar_action_count: mobileTopbarActionControls.length,
    mobile_single_bottom_action_layer_ready: !isMobile || visibleBottomLayers.length === 1,
    mobile_visible_bottom_action_layer_count: visibleBottomLayers.length,
    mobile_primary_actions_ready: !isMobile || mobilePrimaryActions.every(Boolean),
    narrow_shell_density_ready: narrowShellDensityReady,
    narrow_single_action_row_ready: narrowSingleActionRowReady,
    narrow_chrome_height_px: Math.round(narrowChromeHeight),
    narrow_chrome_ratio: Number(narrowChromeRatio.toFixed(4)),
    narrow_message_viewport_height_px: Math.round(narrowMessageRect.height),
    narrow_message_viewport_ratio: Number(narrowMessageRatio.toFixed(4)),
    narrow_layout_details: {
      focus_header_visible: visible(focusHeader),
      rail_height: Math.round(narrowRailRect.height),
      thread_header_height: Math.round(narrowHeaderRect.height),
      composer_height: Math.round(narrowComposerRect.height),
      composer_bottom: Math.round(narrowComposerRect.bottom),
      composer_footer_visible: visible(narrowComposerFooter),
      composer_more_in_bar: narrowComposerMore?.closest(".tg-compose-bar") !== null,
    },
    errors,
  };
`;
