module.exports = `
		  const menuItems = Array.from(document.querySelectorAll("[data-control-ui-menu-item]"));
  const menuItemDetails = menuItems.map((node) => {
    const labelNode = node.querySelector(".tg-menu-item__label");
    const iconNode = node.querySelector(".tg-menu-item__icon");
    const style = getComputedStyle(node);
    const labelStyle = labelNode ? getComputedStyle(labelNode) : null;
    const label = (labelNode?.textContent || "").replace(/\\s+/g, " ").trim();
    return {
      key: node.getAttribute("data-control-ui-menu-item") || "",
      icon_present: Boolean(iconNode),
      icon_svg_present: hasSvgIcon(iconNode),
      label,
      label_ready: Boolean(labelNode && label.length > 0),
      visible: elementVisible(node),
      min_height: styleNumber(style, "minHeight"),
      height: Math.round(node.getBoundingClientRect().height),
      label_white_space: labelStyle?.whiteSpace || "",
      label_overflow: labelStyle?.overflow || "",
      label_text_overflow: labelStyle?.textOverflow || "",
      label_nowrap_ready: labelStyle?.whiteSpace === "nowrap",
    };
  });
  const menuItemIconReady = menuItemDetails.length >= 5
    && menuItemDetails.every((item) => (
      item.key.length > 0
      && item.icon_present
      && item.icon_svg_present
      && item.label_ready
      && item.visible
      && item.height >= 36
      && item.label_nowrap_ready
    ));
  const menuSurfaces = Array.from(document.querySelectorAll(".tg-thread-command-menu__panel"));
  const menuSurfaceDetails = menuSurfaces.map((node) => {
    const style = getComputedStyle(node);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const rect = richRect(node);
    const horizontalInViewport = rect.left >= -1 && rect.right <= window.innerWidth + 1 && rect.width <= window.innerWidth - 16;
    const verticalInViewport = rect.top >= -1 && rect.bottom <= window.innerHeight + 1 && rect.height <= window.innerHeight - 16;
    return {
      visible: elementVisible(node),
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      background_image: style.backgroundImage && style.backgroundImage !== "none" ? "present" : "none",
      background_color: style.backgroundColor,
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      overflow_x: style.overflowX,
      item_count: node.querySelectorAll("[data-control-ui-menu-item]").length,
      horizontal_in_viewport: horizontalInViewport,
      vertical_in_viewport: verticalInViewport,
      in_viewport: horizontalInViewport && verticalInViewport,
      top_clipped: rect.top < -1,
      bottom_clipped: rect.bottom > window.innerHeight + 1,
      viewport_height: window.innerHeight,
      viewport_width: window.innerWidth,
      ...rect,
    };
  });
  const menuSurfaceReady = menuSurfaceDetails.length >= 2
    && menuSurfaceDetails.every((item) => (
      item.visible
      && item.item_count >= 1
      && item.width >= 180
      && item.height >= 44
      && item.border_radius >= 16
      && /blur\\(/.test(item.backdrop_filter)
      && item.box_shadow !== "none"
      && item.in_viewport
    ));
  const threadToolsTrigger = document.querySelector('[data-thread-command-menu="true"] [data-control-ui-thread-tools-trigger="light-glass"]');
  const threadToolsTriggerDetails = (() => {
    if (!threadToolsTrigger) {
      return { exists: false, visible: false };
    }
    const style = getComputedStyle(threadToolsTrigger);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const bgColor = effectiveBackground(threadToolsTrigger);
    const fgColor = parseCssColor(style.color);
    const bgLuminance = relativeLuminance(bgColor);
    const ratio = fgColor ? contrastRatio(fgColor, bgColor) : 0;
    const ariaLabel = threadToolsTrigger.getAttribute("aria-label") || "";
    const title = threadToolsTrigger.getAttribute("title") || "";
    return {
      exists: true,
      marker: threadToolsTrigger.getAttribute("data-control-ui-thread-tools-trigger") || "",
      visible: elementVisible(threadToolsTrigger),
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      svg_icon_present: hasSvgIcon(threadToolsTrigger),
      visible_icon_text: visibleText(threadToolsTrigger),
      visible_icon_text_absent: visibleText(threadToolsTrigger).length === 0,
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      effective_luminance: Number(bgLuminance.toFixed(3)),
      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      ...richRect(threadToolsTrigger),
    };
  })();
  const threadToolsTriggerLightGlassReady = threadToolsTriggerDetails.exists === true
    && threadToolsTriggerDetails.marker === "light-glass"
    && threadToolsTriggerDetails.visible === true
    && threadToolsTriggerDetails.width >= 44
    && threadToolsTriggerDetails.height >= 44
    && threadToolsTriggerDetails.border_radius >= 20
    && threadToolsTriggerDetails.light_glass_ready === true
    && /blur\\(/.test(threadToolsTriggerDetails.backdrop_filter || "")
    && threadToolsTriggerDetails.box_shadow !== "none"
    && threadToolsTriggerDetails.title_matches_aria_label === true
    && threadToolsTriggerDetails.svg_icon_present === true
    && threadToolsTriggerDetails.visible_icon_text_absent === true
    && threadToolsTriggerDetails.readable === true
    && threadToolsTriggerDetails.contrast_ratio >= 4.5;
  const threadToolsPanel = document.querySelector('[data-thread-command-menu="true"] [data-control-ui-thread-tools-panel="light-glass"]');
  const threadToolsPanelDetails = (() => {
    if (!threadToolsPanel) {
      return { exists: false, visible: false };
    }
    const style = getComputedStyle(threadToolsPanel);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const rect = richRect(threadToolsPanel);
    const bgColor = effectiveBackground(threadToolsPanel);
    const bgLuminance = relativeLuminance(bgColor);
    const horizontalInViewport = rect.left >= -1 && rect.right <= window.innerWidth + 1 && rect.width <= window.innerWidth - 16;
    const verticalInViewport = rect.top >= -1 && rect.bottom <= window.innerHeight + 1 && rect.height <= window.innerHeight - 16;
    return {
      exists: true,
      visible: elementVisible(threadToolsPanel),
      role: threadToolsPanel.getAttribute("role") || "",
      aria_label: threadToolsPanel.getAttribute("aria-label") || "",
      background_color: style.backgroundColor,
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      effective_luminance: Number(bgLuminance.toFixed(3)),
      marker: threadToolsPanel.getAttribute("data-control-ui-thread-tools-panel") || "",
      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      item_count: threadToolsPanel.querySelectorAll("[data-control-ui-menu-item]").length,
      horizontal_in_viewport: horizontalInViewport,
      vertical_in_viewport: verticalInViewport,
      in_viewport: horizontalInViewport && verticalInViewport,
      top_clipped: rect.top < -1,
      bottom_clipped: rect.bottom > window.innerHeight + 1,
      viewport_height: window.innerHeight,
      viewport_width: window.innerWidth,
      ...rect,
    };
  })();
  const threadToolsItemDetails = Array.from(document.querySelectorAll('[data-thread-command-menu="true"] [data-control-ui-menu-item]')).map((node) => {
    const labelNode = node.querySelector(".tg-menu-item__label");
    const iconNode = node.querySelector(".tg-menu-item__icon");
    const style = getComputedStyle(node);
    const labelStyle = labelNode ? getComputedStyle(labelNode) : null;
    const labelTextStyle = labelNode ? getComputedStyle(labelNode) : style;
    const textColor = parseCssColor(labelTextStyle.color);
    const bgColor = effectiveBackground(node);
    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
    const label = (labelNode?.textContent || "").replace(/\\s+/g, " ").trim();
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    return {
      key: node.getAttribute("data-control-ui-menu-item") || "",
      role: node.getAttribute("role") || "",
      text: visibleText(node),
      label,
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      icon_present: Boolean(iconNode),
      icon_svg_present: hasSvgIcon(iconNode),
      label_ready: Boolean(labelNode && label.length > 0),
      visible: elementVisible(node),
      min_height: styleNumber(style, "minHeight"),
      height: Math.round(node.getBoundingClientRect().height),
      label_white_space: labelStyle?.whiteSpace || "",
      label_overflow: labelStyle?.overflow || "",
      label_text_overflow: labelStyle?.textOverflow || "",
      label_nowrap_ready: labelStyle?.whiteSpace === "nowrap",
      color: labelTextStyle.color,
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      ...richRect(node),
    };
  });
  const expectedThreadToolsKeys = ["history", "tasks", "sessions"];
  const threadToolsMenuReady = threadToolsPanelDetails.exists === true
    && threadToolsPanelDetails.visible === true
    && threadToolsPanelDetails.role === "menu"
    && threadToolsPanelDetails.aria_label === "Thread tools"
    && threadToolsPanelDetails.item_count === 3
    && threadToolsPanelDetails.width >= 180
    && threadToolsPanelDetails.height >= 44
    && threadToolsPanelDetails.border_radius >= 16
    && threadToolsTriggerLightGlassReady === true
    && threadToolsPanelDetails.marker === "light-glass"
    && threadToolsPanelDetails.light_glass_ready === true
    && threadToolsPanelDetails.effective_luminance >= 0.72
    && threadToolsPanelDetails.effective_luminance <= 0.98
    && /blur\\(/.test(threadToolsPanelDetails.backdrop_filter || "")
    && threadToolsPanelDetails.box_shadow !== "none"
    && threadToolsPanelDetails.in_viewport === true
    && threadToolsPanelDetails.top_clipped === false
    && threadToolsPanelDetails.bottom_clipped === false
    && threadToolsItemDetails.length === 3
    && expectedThreadToolsKeys.every((key) => threadToolsItemDetails.some((item) => item.key === key))
    && threadToolsItemDetails.every((item) => (
      item.visible
      && item.role === "menuitem"
      && item.key.length > 0
      && item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.icon_present
      && item.icon_svg_present
      && item.label_ready
      && item.height >= 44
      && item.label_nowrap_ready
      && item.readable
      && item.contrast_ratio >= 4.5
    ));
  const composerToolsTrigger = document.querySelector('[data-control-ui-composer-more] [data-control-ui-composer-tools-trigger="light-glass"]');
  const composerToolsTriggerDetails = (() => {
    if (!composerToolsTrigger) {
      return { exists: false, visible: false };
    }
    const style = getComputedStyle(composerToolsTrigger);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const bgColor = effectiveBackground(composerToolsTrigger);
    const fgColor = parseCssColor(style.color);
    const bgLuminance = relativeLuminance(bgColor);
    const ratio = fgColor ? contrastRatio(fgColor, bgColor) : 0;
    const ariaLabel = composerToolsTrigger.getAttribute("aria-label") || "";
    const title = composerToolsTrigger.getAttribute("title") || "";
    return {
      exists: true,
      marker: composerToolsTrigger.getAttribute("data-control-ui-composer-tools-trigger") || "",
      visible: elementVisible(composerToolsTrigger),
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      svg_icon_present: hasSvgIcon(composerToolsTrigger),
      visible_icon_text: visibleText(composerToolsTrigger),
      visible_icon_text_absent: visibleText(composerToolsTrigger).length === 0,
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      effective_luminance: Number(bgLuminance.toFixed(3)),
      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      ...richRect(composerToolsTrigger),
    };
  })();
  const composerToolsTriggerLightGlassReady = composerToolsTriggerDetails.exists === true
    && composerToolsTriggerDetails.marker === "light-glass"
    && composerToolsTriggerDetails.visible === true
    && composerToolsTriggerDetails.width >= 44
    && composerToolsTriggerDetails.height >= 44
    && composerToolsTriggerDetails.border_radius >= 20
    && composerToolsTriggerDetails.light_glass_ready === true
    && /blur\\(/.test(composerToolsTriggerDetails.backdrop_filter || "")
    && composerToolsTriggerDetails.box_shadow !== "none"
    && composerToolsTriggerDetails.title_matches_aria_label === true
    && composerToolsTriggerDetails.svg_icon_present === true
    && composerToolsTriggerDetails.visible_icon_text_absent === true
    && composerToolsTriggerDetails.readable === true
    && composerToolsTriggerDetails.contrast_ratio >= 4.5;
  const composerToolsPanel = document.querySelector('[data-control-ui-composer-more] [data-control-ui-composer-tools-panel="light-glass"]');
  const composerToolsPanelDetails = (() => {
    if (!composerToolsPanel) {
      return { exists: false, visible: false };
    }
    const style = getComputedStyle(composerToolsPanel);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const rect = richRect(composerToolsPanel);
    const bgColor = effectiveBackground(composerToolsPanel);
    const bgLuminance = relativeLuminance(bgColor);
    const horizontalInViewport = rect.left >= -1 && rect.right <= window.innerWidth + 1 && rect.width <= window.innerWidth - 16;
    const verticalInViewport = rect.top >= -1 && rect.bottom <= window.innerHeight + 1 && rect.height <= window.innerHeight - 16;
    return {
      exists: true,
      visible: elementVisible(composerToolsPanel),
      role: composerToolsPanel.getAttribute("role") || "",
      aria_label: composerToolsPanel.getAttribute("aria-label") || "",
      background_color: style.backgroundColor,
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      effective_luminance: Number(bgLuminance.toFixed(3)),
      marker: composerToolsPanel.getAttribute("data-control-ui-composer-tools-panel") || "",
      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      item_count: composerToolsPanel.querySelectorAll("[data-control-ui-composer-tool-item]").length,
      horizontal_in_viewport: horizontalInViewport,
      vertical_in_viewport: verticalInViewport,
      in_viewport: horizontalInViewport && verticalInViewport,
      top_clipped: rect.top < -1,
      bottom_clipped: rect.bottom > window.innerHeight + 1,
      viewport_height: window.innerHeight,
      viewport_width: window.innerWidth,
      ...rect,
    };
  })();
  const composerToolsItemDetails = Array.from(document.querySelectorAll('[data-control-ui-composer-more] [data-control-ui-composer-tool-item]')).map((node) => {
    const labelNode = node.querySelector(".tg-menu-item__label");
    const iconNode = node.querySelector(".tg-menu-item__icon");
    const selectNode = node.querySelector("select");
    const style = getComputedStyle(node);
    const labelStyle = labelNode ? getComputedStyle(labelNode) : null;
    const labelTextStyle = labelNode ? getComputedStyle(labelNode) : style;
    const textColor = parseCssColor(labelTextStyle.color);
    const bgColor = effectiveBackground(node);
    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
    const label = (labelNode?.textContent || "").replace(/\\s+/g, " ").trim();
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const selectAriaLabel = selectNode?.getAttribute("aria-label") || "";
    const selectTitle = selectNode?.getAttribute("title") || "";
    const selectStyle = selectNode ? getComputedStyle(selectNode) : null;
    const selectTextColor = selectStyle ? parseCssColor(selectStyle.color) : null;
    const selectBgColor = selectNode ? effectiveBackground(selectNode) : null;
    const selectRatio = selectTextColor && selectBgColor ? contrastRatio(selectTextColor, selectBgColor) : 0;
    return {
      key: node.getAttribute("data-control-ui-composer-tool-item") || "",
      role: node.getAttribute("role") || "",
      text: visibleText(node),
      label,
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      icon_present: Boolean(iconNode),
      icon_svg_present: hasSvgIcon(iconNode),
      label_ready: Boolean(labelNode && label.length > 0),
      select_present: Boolean(selectNode),
      select_visible: Boolean(selectNode && elementVisible(selectNode)),
      select_aria_label: selectAriaLabel,
      select_title: selectTitle,
      select_title_matches_aria_label: selectTitle === selectAriaLabel,
      select_height: selectNode ? Math.round(selectNode.getBoundingClientRect().height) : 0,
      select_color: selectStyle?.color || "",
      select_effective_background: selectBgColor ? "rgb(" + Math.round(selectBgColor.r) + ", " + Math.round(selectBgColor.g) + ", " + Math.round(selectBgColor.b) + ")" : "",
      select_contrast_ratio: Number(selectRatio.toFixed(2)),
      select_readable: selectRatio >= 4.5,
      visible: elementVisible(node),
      min_height: styleNumber(style, "minHeight"),
      height: Math.round(node.getBoundingClientRect().height),
      label_white_space: labelStyle?.whiteSpace || "",
      label_overflow: labelStyle?.overflow || "",
      label_text_overflow: labelStyle?.textOverflow || "",
      label_nowrap_ready: labelStyle?.whiteSpace === "nowrap",
      color: labelTextStyle.color,
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      ...richRect(node),
    };
  });
  const expectedComposerToolsKeys = ["reply-mode", "scroll-mode"];
  const composerToolsMenuReady = composerToolsPanelDetails.exists === true
    && composerToolsPanelDetails.visible === true
    && composerToolsPanelDetails.role === "menu"
    && composerToolsPanelDetails.aria_label === "Composer tools"
    && composerToolsPanelDetails.item_count === 2
    && composerToolsPanelDetails.width >= 180
    && composerToolsPanelDetails.height >= 44
    && composerToolsPanelDetails.border_radius >= 16
    && composerToolsTriggerLightGlassReady === true
    && composerToolsPanelDetails.marker === "light-glass"
    && composerToolsPanelDetails.light_glass_ready === true
    && composerToolsPanelDetails.effective_luminance >= 0.72
    && composerToolsPanelDetails.effective_luminance <= 0.98
    && /blur\\(/.test(composerToolsPanelDetails.backdrop_filter || "")
    && composerToolsPanelDetails.box_shadow !== "none"
    && composerToolsPanelDetails.in_viewport === true
    && composerToolsPanelDetails.top_clipped === false
    && composerToolsPanelDetails.bottom_clipped === false
    && composerToolsItemDetails.length === 2
    && expectedComposerToolsKeys.every((key) => composerToolsItemDetails.some((item) => item.key === key))
    && composerToolsItemDetails.every((item) => (
      item.visible
      && item.role === "menuitem"
      && item.key.length > 0
      && item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.icon_present
      && item.icon_svg_present
      && item.label_ready
      && item.select_present
      && item.select_visible
      && item.select_aria_label.length > 0
      && item.select_title.length > 0
      && item.select_title_matches_aria_label
      && item.select_height >= 44
      && item.select_readable
      && item.select_contrast_ratio >= 4.5
      && item.height >= 44
      && item.label_nowrap_ready
      && item.readable
      && item.contrast_ratio >= 4.5
    ));
  const composerPopoverToggleDetails = Array.from(document.querySelectorAll("[data-chat-composer-popover-toggle]")).map((node) => {
    const style = getComputedStyle(node);
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    return {
      key: node.getAttribute("data-chat-composer-popover-toggle") || "",
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      aria_haspopup: node.getAttribute("aria-haspopup") || "",
      aria_controls: node.getAttribute("aria-controls") || "",
      visible: elementVisible(node),
      svg_icon_present: hasSvgIcon(node),
      visible_icon_text_absent: visibleText(node).length === 0,
      ...richRect(node),
    };
  });
  const composerPopoverPanelDetails = Array.from(document.querySelectorAll('[data-control-ui-composer-popover-panel="light-glass"]')).map((node) => {
    const style = getComputedStyle(node);
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const rect = richRect(node);
    const bgColor = effectiveBackground(node);
    const bgLuminance = relativeLuminance(bgColor);
    const horizontalInViewport = rect.left >= -1 && rect.right <= window.innerWidth + 1 && rect.width <= window.innerWidth - 16;
    const verticalInViewport = rect.top >= -1 && rect.bottom <= window.innerHeight + 1 && rect.height <= window.innerHeight - 16;
    return {
      key: node.getAttribute("data-chat-composer-popover") || "",
      window_width: window.innerWidth,
      inline_style: node.getAttribute("style") || "",
      role: node.getAttribute("role") || "",
      marker: node.getAttribute("data-control-ui-composer-popover-panel") || "",
      aria_label: node.getAttribute("aria-label") || "",
      visible: elementVisible(node),
      search_count: node.querySelectorAll("[data-chat-composer-picker-search]").length,
      item_count: node.querySelectorAll("[data-chat-composer-picker-item]").length,
      background_color: style.backgroundColor,
      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
      translucent_ready: translucentGlassReady(style),
      effective_background: "rgb(" + Math.round(bgColor.r) + ", " + Math.round(bgColor.g) + ", " + Math.round(bgColor.b) + ")",
      effective_luminance: Number(bgLuminance.toFixed(3)),
      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      horizontal_in_viewport: horizontalInViewport,
      vertical_in_viewport: verticalInViewport,
      in_viewport: horizontalInViewport && verticalInViewport,
      top_clipped: rect.top < -1,
      bottom_clipped: rect.bottom > window.innerHeight + 1,
      ...rect,
    };
  });
  const composerPopoverHeaderDetails = Array.from(document.querySelectorAll('[data-control-ui-composer-popover-panel="light-glass"] .tg-composer-popover__header')).map((node) => {
    const panel = node.closest("[data-control-ui-composer-popover-panel]");
    const labelNode = node.querySelector("strong");
    const statusNode = node.querySelector("span");
    const labelStyle = labelNode ? getComputedStyle(labelNode) : null;
    const statusStyle = statusNode ? getComputedStyle(statusNode) : null;
    const bgColor = effectiveBackground(panel || node);
    const labelColor = parseCssColor((labelStyle || getComputedStyle(node)).color);
    const statusColor = parseCssColor((statusStyle || getComputedStyle(node)).color);
    const labelRatio = labelColor ? contrastRatio(labelColor, bgColor) : 0;
    const statusRatio = statusColor ? contrastRatio(statusColor, bgColor) : 0;
    const labelTextShadow = labelStyle?.textShadow || "";
    const statusTextShadow = statusStyle?.textShadow || "";
    const labelTextShadowCount = labelTextShadow && labelTextShadow !== "none" ? ((labelTextShadow.match(/rgb/g) || []).length || 1) : 0;
    const statusTextShadowCount = statusTextShadow && statusTextShadow !== "none" ? ((statusTextShadow.match(/rgb/g) || []).length || 1) : 0;
    return {
      key: panel?.getAttribute("data-chat-composer-popover") || "",
      label: (labelNode?.textContent || "").replace(/\\s+/g, " ").trim(),
      status: (statusNode?.textContent || "").replace(/\\s+/g, " ").trim(),
      visible: elementVisible(node),
      label_visible: Boolean(labelNode) && elementVisible(labelNode),
      status_visible: Boolean(statusNode) && elementVisible(statusNode),
      label_text_shadow: labelTextShadow && labelTextShadow !== "none" ? "present" : "none",
      label_text_shadow_sample: labelTextShadow.slice(0, 180),
      status_text_shadow: statusTextShadow && statusTextShadow !== "none" ? "present" : "none",
      status_text_shadow_sample: statusTextShadow.slice(0, 180),
      composer_popover_header_label_text_shadow_count: labelTextShadowCount,
      composer_popover_header_status_text_shadow_count: statusTextShadowCount,
      composer_popover_header_prismatic_etch_ready: labelTextShadowCount >= 2 && statusTextShadowCount >= 2,
      label_contrast_ratio: Number(labelRatio.toFixed(2)),
      status_contrast_ratio: Number(statusRatio.toFixed(2)),
      label_readable: labelRatio >= 4.5,
      status_readable: statusRatio >= 4.5,
      ...richRect(node),
    };
  });
  const composerPopoverSearchDetails = Array.from(document.querySelectorAll("[data-chat-composer-picker-search]")).map((node) => {
    const style = getComputedStyle(node);
    const placeholderStyle = getComputedStyle(node, "::placeholder");
    const textColorValue = style.webkitTextFillColor || style.color;
    const textColor = parseCssColor(textColorValue);
    const placeholderColor = parseCssColor(placeholderStyle.color);
    const bgColor = effectiveBackground(node);
    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
    const placeholderRatio = placeholderColor ? contrastRatio(placeholderColor, bgColor) : 0;
    const placeholderTextShadow = placeholderStyle.textShadow || "";
    const placeholderTextShadowCount = placeholderTextShadow && placeholderTextShadow !== "none" ? ((placeholderTextShadow.match(/rgb/g) || []).length || 1) : 0;
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const bgLuminance = relativeLuminance(bgColor);
    const marker = node.getAttribute("data-control-ui-composer-popover-search") || "";
    return {
      key: node.getAttribute("data-chat-composer-picker-search") || "",
      marker,
      placeholder: node.getAttribute("placeholder") || "",
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      visible: elementVisible(node),
      background_color: style.backgroundColor,
      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
      translucent_ready: translucentGlassReady(style),
      color: textColorValue,
      effective_luminance: Number(bgLuminance.toFixed(3)),
      light_glass_ready: marker === "light-glass" && bgLuminance >= 0.72 && bgLuminance <= 0.98,
      border_radius: Number.parseFloat(style.borderTopLeftRadius || "0"),
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      placeholder_text_shadow: placeholderTextShadow && placeholderTextShadow !== "none" ? "present" : "none",
      placeholder_text_shadow_sample: placeholderTextShadow.slice(0, 180),
      composer_popover_search_placeholder_text_shadow_count: placeholderTextShadowCount,
      composer_popover_search_placeholder_prismatic_etch_ready: placeholderTextShadowCount >= 2,
      placeholder_contrast_ratio: Number(placeholderRatio.toFixed(2)),
      placeholder_readable: placeholderRatio >= 4.5,
      ...richRect(node),
    };
  });
  const composerPopoverItemDetails = Array.from(document.querySelectorAll("[data-chat-composer-picker-item]")).map((node) => {
    const labelNode = node.querySelector("b");
    const smallNode = node.querySelector("small");
    const iconNode = node.querySelector(".tg-composer-popover__icon");
    const style = getComputedStyle(node);
    const labelStyle = labelNode ? getComputedStyle(labelNode) : null;
    const detailStyle = smallNode ? getComputedStyle(smallNode) : null;
    const textColor = parseCssColor((labelStyle || style).color);
    const detailColor = parseCssColor((detailStyle || style).color);
    const bgColor = effectiveBackground(node);
    const bgLuminance = relativeLuminance(bgColor);
    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
    const detailRatio = detailColor ? contrastRatio(detailColor, bgColor) : 0;
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const labelTextShadow = labelStyle?.textShadow || "";
    const detailTextShadow = detailStyle?.textShadow || "";
    const labelTextShadowCount = labelTextShadow && labelTextShadow !== "none" ? ((labelTextShadow.match(/rgb/g) || []).length || 1) : 0;
    const detailTextShadowCount = detailTextShadow && detailTextShadow !== "none" ? ((detailTextShadow.match(/rgb/g) || []).length || 1) : 0;
    return {
      key: node.getAttribute("data-chat-composer-picker-item") || "",
      role: node.getAttribute("role") || "",
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      label: (labelNode?.textContent || "").replace(/\\s+/g, " ").trim(),
      detail: (smallNode?.textContent || "").replace(/\\s+/g, " ").trim(),
      visible: elementVisible(node),
      icon_present: Boolean(iconNode),
      icon_svg_present: hasSvgIcon(iconNode),
      label_nowrap_ready: labelStyle?.whiteSpace === "nowrap",
      detail_nowrap_ready: detailStyle?.whiteSpace === "nowrap",
      background_color: style.backgroundColor,
      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
      translucent_ready: translucentGlassReady(style),
      effective_luminance: Number(bgLuminance.toFixed(3)),
      light_glass_ready: bgLuminance >= 0.72 && bgLuminance <= 0.98,
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      detail_contrast_ratio: Number(detailRatio.toFixed(2)),
      detail_readable: detailRatio >= 4.5,
      label_text_shadow: labelTextShadow && labelTextShadow !== "none" ? "present" : "none",
      label_text_shadow_sample: labelTextShadow.slice(0, 180),
      detail_text_shadow: detailTextShadow && detailTextShadow !== "none" ? "present" : "none",
      detail_text_shadow_sample: detailTextShadow.slice(0, 180),
      composer_popover_item_label_text_shadow_count: labelTextShadowCount,
      composer_popover_item_detail_text_shadow_count: detailTextShadowCount,
      composer_popover_item_label_prismatic_etch_ready: labelTextShadowCount >= 2 && detailTextShadowCount >= 2,
      ...richRect(node),
    };
  });
  const expectedComposerPopoverKeys = ["artifact", "command"];
  const composerPopoverReady = composerPopoverToggleDetails.length === 2
    && composerPopoverToggleDetails.every((item) => (
      item.visible
      && expectedComposerPopoverKeys.includes(item.key)
      && item.width >= 44
      && item.height >= 44
      && item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.aria_haspopup === "menu"
      && item.aria_controls.length > 0
      && item.svg_icon_present
      && item.visible_icon_text_absent
    ))
    && composerPopoverPanelDetails.length === 2
    && expectedComposerPopoverKeys.every((key) => composerPopoverPanelDetails.some((item) => item.key === key))
    && composerPopoverPanelDetails.every((item) => (
      item.visible
      && item.role === "menu"
      && item.aria_label.length > 0
      && item.search_count === 1
      && item.item_count === 2
      && item.width >= 180
      && item.height >= 132
      && item.border_radius >= 16
      && item.marker === "light-glass"
      && item.translucent_ready === true
      && item.light_glass_ready
      && item.effective_luminance >= 0.72
      && item.effective_luminance <= 0.98
      && /blur\\(/.test(item.backdrop_filter || "")
      && item.box_shadow !== "none"
      && item.in_viewport
      && item.top_clipped === false
      && item.bottom_clipped === false
    ))
    && composerPopoverSearchDetails.length === 2
    && composerPopoverSearchDetails.every((item) => (
      item.visible
      && expectedComposerPopoverKeys.includes(item.key)
      && item.height >= 44
      && item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.readable
      && item.contrast_ratio >= 4.5
    ))
    && composerPopoverItemDetails.length === 4
    && composerPopoverItemDetails.every((item) => (
      item.visible
      && item.role === "menuitem"
      && item.key.length > 0
      && item.width >= 120
      && item.height >= 44
      && item.aria_label.length > 0
      && item.title.length > 0
      && item.title_matches_aria_label
      && item.label.length > 0
      && item.detail.length > 0
      && item.icon_present
      && item.icon_svg_present
      && item.light_glass_ready
      && item.translucent_ready === true
      && item.label_nowrap_ready
      && item.readable
      && item.contrast_ratio >= 4.5
    ));
  const composerPopoverItemLabelPrismaticEtchLightGlassReady = composerPopoverReady
    && composerPopoverItemDetails.every((item) => (
      item.visible
      && item.label.length > 0
      && item.detail.length > 0
      && item.label_text_shadow === "present"
      && item.detail_text_shadow === "present"
      && item.composer_popover_item_label_prismatic_etch_ready === true
      && item.composer_popover_item_label_text_shadow_count >= 2
      && item.composer_popover_item_detail_text_shadow_count >= 2
      && item.readable
      && item.detail_readable
      && item.contrast_ratio >= 4.5
      && item.detail_contrast_ratio >= 4.5
      && item.label_nowrap_ready
      && item.detail_nowrap_ready
    ));
  const composerPopoverHeaderPrismaticEtchLightGlassReady = composerPopoverReady
    && composerPopoverHeaderDetails.length === 2
    && expectedComposerPopoverKeys.every((key) => composerPopoverHeaderDetails.some((item) => item.key === key))
    && composerPopoverHeaderDetails.every((item) => (
      item.visible
      && item.label_visible
      && item.status_visible
      && item.label.length > 0
      && item.status.length > 0
      && item.label_text_shadow === "present"
      && item.status_text_shadow === "present"
      && item.composer_popover_header_prismatic_etch_ready === true
      && item.composer_popover_header_label_text_shadow_count >= 2
      && item.composer_popover_header_status_text_shadow_count >= 2
      && item.label_readable
      && item.status_readable
      && item.label_contrast_ratio >= 4.5
      && item.status_contrast_ratio >= 4.5
    ));
  const composerPopoverSearchLightGlassReady = composerPopoverSearchDetails.length === 2
    && expectedComposerPopoverKeys.every((key) => composerPopoverSearchDetails.some((item) => item.key === key))
    && composerPopoverSearchDetails.every((item) => (
      item.visible
      && item.marker === "light-glass"
      && item.height >= 44
      && item.border_radius >= 10
      && item.light_glass_ready
      && item.translucent_ready === true
      && item.effective_luminance >= 0.72
      && item.effective_luminance <= 0.98
      && (item.backdrop_filter || "").includes("blur(")
      && item.box_shadow !== "none"
      && item.readable
      && item.contrast_ratio >= 4.5
    ));
  const composerPopoverSearchPlaceholderPrismaticEtchLightGlassReady = composerPopoverSearchLightGlassReady
    && composerPopoverSearchDetails.every((item) => (
      item.placeholder.length > 0
      && item.placeholder_text_shadow === "present"
      && item.composer_popover_search_placeholder_prismatic_etch_ready === true
      && item.composer_popover_search_placeholder_text_shadow_count >= 2
      && item.placeholder_readable === true
      && item.placeholder_contrast_ratio >= 4.5
    ));
  const railSearchNodes = Array.from(document.querySelectorAll("[data-control-ui-rail-search-input]"));
  const railSearchDetails = railSearchNodes.map((node) => {
    const style = getComputedStyle(node);
    const placeholderStyle = getComputedStyle(node, "::placeholder");
    const textColor = parseCssColor(style.webkitTextFillColor || style.color);
    const placeholderColor = parseCssColor(placeholderStyle.color);
    const bgColor = effectiveBackground(node);
    const ratio = textColor ? contrastRatio(textColor, bgColor) : 0;
    const placeholderRatio = placeholderColor ? contrastRatio(placeholderColor, bgColor) : 0;
    const placeholderTextShadow = placeholderStyle.textShadow || "";
    const placeholderTextShadowCount = placeholderTextShadow && placeholderTextShadow !== "none" ? ((placeholderTextShadow.split("rgba(").length - 1) + (placeholderTextShadow.split("rgb(").length - 1) || 1) : 0;
    const backdrop = style.backdropFilter || style.webkitBackdropFilter || "";
    const rect = richRect(node);
    const effectiveLuminance = relativeLuminance(bgColor);
    const ariaLabel = node.getAttribute("aria-label") || "";
    const title = node.getAttribute("title") || "";
    const filterText = style.filter || "";
    const dropShadowCount = (filterText.match(/drop-shadow/g) || []).length;
    return {
      marker: node.getAttribute("data-control-ui-rail-search-input") || "",
      placeholder: node.getAttribute("placeholder") || "",
      aria_label: ariaLabel,
      title,
      title_matches_aria_label: title === ariaLabel,
      visible: elementVisible(node),
      type: node.getAttribute("type") || "",
      border_radius: styleNumber(style, "borderTopLeftRadius"),
      background_color: style.backgroundColor,
      background_alpha: Number(directBackgroundAlpha(style).toFixed(2)),
      translucent_ready: translucentGlassReady(style),
      effective_luminance: Number(effectiveLuminance.toFixed(3)),
      light_glass_ready: effectiveLuminance >= 0.72 && effectiveLuminance <= 0.98,
      backdrop_filter: backdrop,
      box_shadow: compactShadow(style.boxShadow),
      color: style.color,
      text_fill_color: style.webkitTextFillColor || "",
      contrast_ratio: Number(ratio.toFixed(2)),
      readable: ratio >= 4.5,
      placeholder_color: placeholderStyle.color,
      placeholder_text_shadow: placeholderTextShadow && placeholderTextShadow !== "none" ? "present" : "none",
      placeholder_text_shadow_sample: placeholderTextShadow.slice(0, 180),
      rail_search_placeholder_text_shadow_count: placeholderTextShadowCount,
      rail_search_placeholder_prismatic_etch_ready: placeholderTextShadowCount >= 2,
      placeholder_contrast_ratio: Number(placeholderRatio.toFixed(2)),
      placeholder_readable: placeholderRatio >= 4.5,
      filter: filterText && filterText !== "none" ? "present" : "none",
      filter_sample: filterText.slice(0, 180),
      rail_filter_drop_shadow_count: dropShadowCount,
      rail_prismatic_filter_ready: dropShadowCount >= 2,
      ...rect,
    };
  });
  const visibleRailSearchDetails = railSearchDetails.filter((item) => item.visible);
  const railSearchPlaceholderPrismaticEtchDetails = visibleRailSearchDetails.map((item) => ({
    placeholder: item.placeholder,
    visible: item.visible,
    width: item.width,
    height: item.height,
    placeholder_text_shadow: item.placeholder_text_shadow,
    placeholder_text_shadow_sample: item.placeholder_text_shadow_sample,
    rail_search_placeholder_text_shadow_count: item.rail_search_placeholder_text_shadow_count,
    rail_search_placeholder_prismatic_etch_ready: item.rail_search_placeholder_prismatic_etch_ready,
    placeholder_color: item.placeholder_color,
    placeholder_contrast_ratio: item.placeholder_contrast_ratio,
    placeholder_readable: item.placeholder_readable,
  }));
  const railSearchPlaceholderPrismaticEtchLightGlassReady = railVisible
    ? visibleRailSearchDetails.length === 1
      && visibleRailSearchDetails.every((item) => (
        item.placeholder.length > 0
        && item.width >= 180
        && item.height >= 44
        && item.placeholder_text_shadow === "present"
        && item.rail_search_placeholder_prismatic_etch_ready === true
        && item.rail_search_placeholder_text_shadow_count >= 2
        && item.placeholder_readable
        && item.placeholder_contrast_ratio >= 4.5
      ))
    : visibleRailSearchDetails.length === 0;
  const railSearchLightGlassReady = railVisible
    ? (
      railSearchNodes.length === 1
      && visibleRailSearchDetails.length === 1
      && visibleRailSearchDetails.every((item) => (
        item.marker === "light-glass"
        && item.type === "search"
        && item.placeholder.length > 0
        && item.aria_label === "Search chats"
        && item.title === "Search chats"
        && item.title_matches_aria_label
        && item.width >= 180
        && item.height >= 44
        && item.border_radius >= 12
        && item.light_glass_ready
        && item.translucent_ready === true
        && item.effective_luminance >= 0.72
        && item.effective_luminance <= 0.98
        && (item.backdrop_filter || "").includes("blur(")
        && item.box_shadow !== "none"
        && item.readable
        && item.contrast_ratio >= 4.5
        && item.placeholder_readable
        && item.placeholder_contrast_ratio >= 4.5
      ))
    )
    : (railSearchNodes.length === 1 && visibleRailSearchDetails.length === 0);
  const railPrismaticFilterDetails = [
    ...visibleRailSearchDetails.map((item) => ({ kind: "search", ...item })),
    ...folderChipDetails.map((item) => ({ kind: "folder-chip", ...item })),
  ];
  const railPrismaticFilterLightGlassReady = railVisible
    ? (
      railPrismaticFilterDetails.length >= 4
      && railPrismaticFilterDetails.every((item) => (
        item.visible
        && item.width >= 44
        && item.height >= 44
        && item.border_radius >= 12
        && item.box_shadow !== "none"
        && (item.backdrop_filter || "").includes("blur(")
        && item.rail_prismatic_filter_ready === true
        && item.rail_filter_drop_shadow_count >= 2
      ))
    )
    : railPrismaticFilterDetails.length === 0;

`;
